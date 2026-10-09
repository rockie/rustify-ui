use rustify_style::{program::GpuValue, text::Text};
use std::collections::{BTreeMap, BTreeSet};

pub(super) fn raw(value: &GpuValue) -> String {
    match value {
        GpuValue::Pixels { value } => format!("{value}px"),
        GpuValue::CssLength { value }
        | GpuValue::Color { value }
        | GpuValue::Number { value }
        | GpuValue::Keyword { value }
        | GpuValue::CssExpression { value } => value.clone(),
        GpuValue::ThemeVariable { name, fallback } => format!(
            "var({name}{})",
            fallback
                .as_ref()
                .map(|v| format!(",{v}"))
                .unwrap_or_default()
        ),
    }
}
struct ValueError {
    message: String,
    fatal: bool,
}
fn invalid(message: impl Into<String>) -> ValueError {
    ValueError {
        message: message.into(),
        fatal: false,
    }
}
/// Resolve custom properties before interpreting the destination's value type.
pub(super) struct Values {
    variables: BTreeMap<Text, Text>,
    cyclic: BTreeSet<Text>,
}
impl Values {
    pub fn new(variables: BTreeMap<Text, Text>) -> Self {
        let graph: BTreeMap<_, _> = variables
            .iter()
            .map(|(name, value)| {
                (
                    name.clone(),
                    value
                        .to_utf8()
                        .ok()
                        .and_then(|value| references(&value).ok())
                        .unwrap_or_default()
                        .into_iter()
                        .map(Text::from)
                        .collect::<Vec<_>>(),
                )
            })
            .collect();
        let mut marks = BTreeMap::<Text, u8>::new();
        let mut cyclic = BTreeSet::new();
        for root in graph.keys() {
            if marks.contains_key(root) {
                continue;
            }
            let mut path = vec![root.clone()];
            let mut stack = vec![(root.clone(), 0usize)];
            marks.insert(root.clone(), 1);
            while let Some((name, index)) = stack.last_mut() {
                let dependencies = &graph[name];
                if *index >= dependencies.len() {
                    marks.insert(name.clone(), 2);
                    stack.pop();
                    path.pop();
                    continue;
                }
                let dependency = dependencies[*index].clone();
                *index += 1;
                if !graph.contains_key(&dependency) {
                    continue;
                }
                match marks.get(&dependency) {
                    Some(1) => {
                        if let Some(start) = path.iter().position(|n| n == &dependency) {
                            cyclic.extend(path[start..].iter().cloned());
                        }
                    }
                    Some(_) => {}
                    None => {
                        marks.insert(dependency.clone(), 1);
                        path.push(dependency.clone());
                        stack.push((dependency, 0));
                    }
                }
            }
        }
        Self { variables, cyclic }
    }
    pub fn resolve(&self, value: &GpuValue) -> Result<String, String> {
        self.css(&raw(value))
    }
    pub fn css(&self, value: &str) -> Result<String, String> {
        self.substitute(value, 0, &mut 4096)
            .map_err(|err| err.message)
    }
    fn substitute(
        &self,
        value: &str,
        depth: usize,
        remaining: &mut usize,
    ) -> Result<String, ValueError> {
        if *remaining == 0 {
            return Err(ValueError {
                message: "variable expansion exceeds 4096 substitutions".into(),
                fatal: true,
            });
        }
        *remaining -= 1;
        if depth > 64 {
            return Err(ValueError {
                message: "variable expansion exceeds 64".into(),
                fatal: true,
            });
        }
        let mut out = String::new();
        let mut copied = 0;
        let mut cursor = 0;
        while let Some(start) = next_var(value, cursor) {
            let (end, name, fallback) = var_body(value, start)?;
            out.push_str(&value[copied..start]);
            let resolved = if self.cyclic.contains(&Text::from(name)) {
                Err(invalid(format!("cyclic custom property {name}")))
            } else if let Some(value) = self.variables.get(&Text::from(name)) {
                match value.to_utf8() {
                    Ok(value) => self.substitute(&value, depth + 1, remaining),
                    Err(_) => Err(ValueError {
                        message: format!("non-scalar custom property {name} is unsupported on GPU"),
                        fatal: true,
                    }),
                }
            } else {
                Err(invalid(format!("unbound custom property {name}")))
            };
            match resolved {
                Ok(value) => out.push_str(&value),
                Err(err) => {
                    if err.fatal {
                        return Err(err);
                    }
                    if let Some(fallback) = fallback {
                        out.push_str(&self.substitute(fallback, depth + 1, remaining)?);
                    } else {
                        return Err(err);
                    }
                }
            }
            if out.len() > 1_048_576 {
                return Err(ValueError {
                    message: "expanded value exceeds 1 MiB".into(),
                    fatal: true,
                });
            }
            cursor = end;
            copied = end;
        }
        if out.len().saturating_add(value.len() - copied) > 1_048_576 {
            return Err(ValueError {
                message: "expanded value exceeds 1 MiB".into(),
                fatal: true,
            });
        }
        out.push_str(&value[copied..]);
        Ok(out)
    }
}
fn next_var(value: &str, start: usize) -> Option<usize> {
    let mut quote = None;
    let mut escaped = false;
    for (index, c) in value[start..].char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        if c == '\\' {
            escaped = true;
            continue;
        }
        if let Some(open) = quote {
            if c == open {
                quote = None;
            }
            continue;
        }
        if c == '\'' || c == '"' {
            quote = Some(c);
            continue;
        }
        let at = start + index;
        if value[at..].starts_with("var(") {
            return Some(at);
        }
    }
    None
}
fn var_body(value: &str, start: usize) -> Result<(usize, &str, Option<&str>), ValueError> {
    let mut depth = 1;
    let mut quote = None;
    let mut escaped = false;
    let mut comma = None;
    for (index, c) in value[start + 4..].char_indices() {
        let at = start + 4 + index;
        if escaped {
            escaped = false;
            continue;
        }
        if c == '\\' {
            escaped = true;
            continue;
        }
        if let Some(open) = quote {
            if c == open {
                quote = None;
            }
            continue;
        }
        match c {
            '\'' | '"' => quote = Some(c),
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    let end = comma.unwrap_or(at);
                    let name = value[start + 4..end].trim();
                    if !name.starts_with("--")
                        || name.len() == 2
                        || name
                            .chars()
                            .any(|c| c.is_whitespace() || "(){};,:\\\"'".contains(c))
                    {
                        return Err(invalid("invalid custom property name"));
                    }
                    return Ok((at + 1, name, comma.map(|comma| value[comma + 1..at].trim())));
                }
            }
            ',' if depth == 1 && comma.is_none() => comma = Some(at),
            _ => {}
        }
    }
    Err(invalid("unclosed var()"))
}
fn references(value: &str) -> Result<Vec<String>, ValueError> {
    let mut out = Vec::new();
    let mut cursor = 0;
    while let Some(start) = next_var(value, cursor) {
        let (_, name, _) = var_body(value, start)?;
        out.push(name.into());
        cursor = start + 4;
    }
    Ok(out)
}

#[derive(Clone, Copy, PartialEq)]
enum Dimension {
    Scalar,
    Length,
}
#[derive(Clone, Copy)]
struct Quantity {
    value: f64,
    dimension: Dimension,
}
/// The finite arithmetic subset needed for emitted spacing/radius/ring values.
/// Other CSS functions, derived dimensions, and unresolved percentages reject.
pub(super) fn length(value: &str, rem: f64, em: f64, percent: Option<f64>) -> Result<f64, String> {
    let quantity = Parser {
        input: value.trim(),
        offset: 0,
        rem,
        em,
        percent,
    }
    .parse()?;
    if quantity.dimension != Dimension::Length && quantity.value != 0. {
        return Err("length requires px/rem/em or unitless zero".into());
    }
    finite(quantity.value)
}
pub(super) fn number(value: &str) -> Result<f64, String> {
    let value = value.trim();
    let (number, factor) = value
        .strip_suffix('%')
        .map_or((value, 1.), |raw| (raw, 0.01));
    finite(
        number
            .trim()
            .parse::<f64>()
            .map_err(|_| "expected a finite number")?
            * factor,
    )
}
pub(super) fn finite(value: f64) -> Result<f64, String> {
    if value.is_finite() && (value as f32).is_finite() {
        Ok(value)
    } else {
        Err("value is not finite in the GPU numeric range".into())
    }
}
struct Parser<'a> {
    input: &'a str,
    offset: usize,
    rem: f64,
    em: f64,
    percent: Option<f64>,
}
impl Parser<'_> {
    fn parse(mut self) -> Result<Quantity, String> {
        let quantity = self.atom(0)?;
        self.space();
        if self.offset != self.input.len() {
            return Err("unsupported CSS value/function".into());
        }
        finite(quantity.value)?;
        Ok(quantity)
    }
    fn space(&mut self) {
        while self
            .input
            .as_bytes()
            .get(self.offset)
            .is_some_and(u8::is_ascii_whitespace)
        {
            self.offset += 1;
        }
    }
    fn consume(&mut self, token: &str) -> bool {
        self.space();
        if self.input[self.offset..].starts_with(token) {
            self.offset += token.len();
            true
        } else {
            false
        }
    }
    fn sum(&mut self, depth: usize) -> Result<Quantity, String> {
        let mut left = self.product(depth)?;
        loop {
            self.space();
            let op = self.input.as_bytes().get(self.offset).copied();
            if !matches!(op, Some(b'+' | b'-')) {
                break;
            }
            if self.offset == 0
                || !self.input.as_bytes()[self.offset - 1].is_ascii_whitespace()
                || !self
                    .input
                    .as_bytes()
                    .get(self.offset + 1)
                    .is_some_and(u8::is_ascii_whitespace)
            {
                return Err("calc addition/subtraction requires surrounding whitespace".into());
            }
            self.offset += 1;
            let right = self.product(depth)?;
            if left.dimension != right.dimension {
                return Err("calc addition requires compatible dimensions".into());
            }
            left.value = if op == Some(b'+') {
                left.value + right.value
            } else {
                left.value - right.value
            };
            finite(left.value)?;
        }
        Ok(left)
    }
    fn product(&mut self, depth: usize) -> Result<Quantity, String> {
        let mut left = self.atom(depth)?;
        loop {
            self.space();
            let op = self.input.as_bytes().get(self.offset).copied();
            if !matches!(op, Some(b'*' | b'/')) {
                break;
            }
            self.offset += 1;
            let right = self.atom(depth)?;
            if op == Some(b'*') {
                if left.dimension == Dimension::Length && right.dimension == Dimension::Length {
                    return Err("calc multiplication produces a derived dimension".into());
                }
                left = Quantity {
                    value: left.value * right.value,
                    dimension: if right.dimension == Dimension::Length {
                        right.dimension
                    } else {
                        left.dimension
                    },
                };
            } else {
                if right.dimension != Dimension::Scalar || right.value == 0. {
                    return Err("calc division requires a nonzero scalar".into());
                }
                left.value /= right.value;
            }
            finite(left.value)?;
        }
        Ok(left)
    }
    fn atom(&mut self, depth: usize) -> Result<Quantity, String> {
        if depth > 32 {
            return Err("numeric expression nesting exceeds 32".into());
        }
        self.space();
        let negative = self.consume("-");
        let positive = if negative { false } else { self.consume("+") };
        if negative || positive {
            let mut quantity = self.atom(depth + 1)?;
            if negative {
                quantity.value = -quantity.value;
            }
            return Ok(quantity);
        }
        if self.consume("calc(") || (depth > 0 && self.consume("(")) {
            let quantity = self.sum(depth + 1)?;
            if !self.consume(")") {
                return Err("unclosed numeric expression".into());
            }
            return Ok(quantity);
        }
        self.space();
        let start = self.offset;
        let mut digits = false;
        while self
            .input
            .as_bytes()
            .get(self.offset)
            .is_some_and(u8::is_ascii_digit)
        {
            digits = true;
            self.offset += 1;
        }
        if self.input.as_bytes().get(self.offset) == Some(&b'.') {
            self.offset += 1;
            while self
                .input
                .as_bytes()
                .get(self.offset)
                .is_some_and(u8::is_ascii_digit)
            {
                digits = true;
                self.offset += 1;
            }
        }
        if !digits {
            return Err("unsupported numeric CSS value/function".into());
        }
        if self
            .input
            .as_bytes()
            .get(self.offset)
            .is_some_and(|b| matches!(b, b'e' | b'E'))
            && self
                .input
                .as_bytes()
                .get(self.offset + 1)
                .is_some_and(|b| b.is_ascii_digit() || matches!(b, b'+' | b'-'))
        {
            self.offset += 1;
            if self
                .input
                .as_bytes()
                .get(self.offset)
                .is_some_and(|b| matches!(b, b'+' | b'-'))
            {
                self.offset += 1;
            }
            let exponent = self.offset;
            while self
                .input
                .as_bytes()
                .get(self.offset)
                .is_some_and(u8::is_ascii_digit)
            {
                self.offset += 1;
            }
            if self.offset == exponent {
                return Err("invalid numeric exponent".into());
            }
        }
        let value = self.input[start..self.offset]
            .parse::<f64>()
            .map_err(|_| "invalid numeric value")?;
        let start = self.offset;
        while self
            .input
            .as_bytes()
            .get(self.offset)
            .is_some_and(|b| b.is_ascii_alphabetic() || *b == b'%')
        {
            self.offset += 1;
        }
        let unit = self.input[start..self.offset].to_ascii_lowercase();
        let (multiplier, dimension) = match unit.as_str() {
            "" => (1., Dimension::Scalar),
            "px" => (1., Dimension::Length),
            "rem" => (self.rem, Dimension::Length),
            "em" => (self.em, Dimension::Length),
            "%" => (
                self.percent
                    .ok_or("percentage needs an explicit reference size")?
                    / 100.,
                Dimension::Length,
            ),
            _ => return Err(format!("unsupported length unit {unit}")),
        };
        Ok(Quantity {
            value: finite(value * multiplier)?,
            dimension,
        })
    }
}
/// Split shorthand/list tokens while preserving function arguments and quotes.
pub(super) fn tokens(value: &str, delimiter: Option<char>) -> Result<Vec<&str>, String> {
    let mut out = Vec::new();
    let mut depth = 0usize;
    let mut quote = None;
    let mut escaped = false;
    let mut start = 0;
    for (index, c) in value.char_indices() {
        if escaped {
            escaped = false;
            continue;
        }
        if c == '\\' {
            escaped = true;
            continue;
        }
        if let Some(open) = quote {
            if c == open {
                quote = None;
            }
            continue;
        }
        match c {
            '\'' | '"' => quote = Some(c),
            '(' => depth += 1,
            ')' => depth = depth.checked_sub(1).ok_or("unbalanced value parentheses")?,
            _ => {}
        }
        if depth == 0 && (delimiter == Some(c) || (delimiter.is_none() && c.is_whitespace())) {
            let part = value[start..index].trim();
            if !part.is_empty() {
                out.push(part);
            } else if delimiter.is_some() {
                return Err("empty CSS list entry".into());
            }
            start = index + c.len_utf8();
        }
    }
    if depth != 0 || quote.is_some() {
        return Err("unclosed CSS value/function".into());
    }
    let last = value[start..].trim();
    if !last.is_empty() {
        out.push(last);
    } else if delimiter.is_some() {
        return Err("empty CSS list entry".into());
    }
    Ok(out)
}
