use super::GpuResolveContext;
use crate::{
    i18n::Direction,
    theme::{ResolvedTheme, ThemeMode},
};
use rustify_style::program::GpuCondition;

pub(super) fn condition(
    condition: &GpuCondition,
    theme: &ResolvedTheme,
    context: &GpuResolveContext,
) -> Result<(bool, usize), String> {
    Ok(match condition {
        GpuCondition::Hover => (context.state.hover && context.hover_capable, 1),
        GpuCondition::Active => (context.state.active, 1),
        GpuCondition::Focus => (context.state.focused, 1),
        GpuCondition::FocusVisible => (focus_visible(context), 1),
        GpuCondition::Disabled => (context.state.disabled, 1),
        GpuCondition::Dark => (theme.mode == ThemeMode::Dark, 0),
        GpuCondition::Selector { selector } => {
            let selector = selector
                .to_utf8()
                .map_err(|_| "non-scalar selector is outside the finite GPU projection")?;
            self_selector(&selector, context)?
        }
        GpuCondition::Media { query } => {
            let query = query
                .to_utf8()
                .map_err(|_| "non-scalar media query is outside the finite GPU projection")?;
            (media(&query, context)?, 0)
        }
        GpuCondition::Unsupported { feature } => return Err(feature.clone()),
    })
}
fn focus_visible(context: &GpuResolveContext) -> bool {
    context.state.focused
        && context.state.focus_visible
        && context.state.modality == super::InputModality::Keyboard
}
fn self_selector(selector: &str, context: &GpuResolveContext) -> Result<(bool, usize), String> {
    let rest = selector
        .trim()
        .strip_prefix('&')
        .ok_or("only proven self selectors are supported")?;
    pseudos(rest, context, 0)
}
fn pseudos(
    mut rest: &str,
    context: &GpuResolveContext,
    depth: usize,
) -> Result<(bool, usize), String> {
    if depth > 32 {
        return Err("selector nesting exceeds 32".into());
    }
    let mut active = true;
    let mut specificity = 0;
    while !rest.is_empty() {
        rest = rest
            .strip_prefix(':')
            .ok_or("selector contains a relationship, attribute, class, or pseudo-element")?;
        let end = rest
            .find(|c: char| !c.is_ascii_alphabetic() && c != '-')
            .unwrap_or(rest.len());
        let name = &rest[..end];
        rest = &rest[end..];
        if matches!(name, "is" | "where" | "not" | "dir") {
            let (body, tail) = function(rest)?;
            rest = tail;
            if name == "dir" {
                active &= match body.trim() {
                    "ltr" => context.direction == Direction::LeftToRight,
                    "rtl" => context.direction == Direction::RightToLeft,
                    _ => return Err("unsupported :dir value".into()),
                };
                specificity += 1;
                continue;
            }
            let mut any = false;
            let mut max_specificity = 0;
            for arg in split_commas(body)? {
                let (matched, weight) = pseudos(arg.trim(), context, depth + 1)?;
                any |= matched;
                max_specificity = max_specificity.max(weight);
            }
            active &= if name == "not" { !any } else { any };
            if name != "where" {
                specificity += max_specificity;
            }
        } else {
            active &= match name {
                "hover" => context.state.hover && context.hover_capable,
                "active" => context.state.active,
                "focus" => context.state.focused,
                "focus-visible" => focus_visible(context),
                "disabled" => context.state.disabled,
                _ => return Err(format!("unsupported self pseudo-class :{name}")),
            };
            specificity += 1;
        }
    }
    Ok((active, specificity))
}
fn function(text: &str) -> Result<(&str, &str), String> {
    let text = text
        .strip_prefix('(')
        .ok_or("expected a selector function")?;
    let mut depth = 1;
    for (index, c) in text.char_indices() {
        match c {
            '(' => depth += 1,
            ')' => depth -= 1,
            _ => {}
        }
        if depth == 0 {
            return Ok((&text[..index], &text[index + 1..]));
        }
    }
    Err("unclosed selector function".into())
}
fn split_commas(text: &str) -> Result<Vec<&str>, String> {
    let mut out = Vec::new();
    let mut depth = 0usize;
    let mut start = 0;
    for (index, c) in text.char_indices() {
        match c {
            '(' => depth += 1,
            ')' => {
                depth = depth
                    .checked_sub(1)
                    .ok_or("unexpected closing parenthesis")?
            }
            ',' if depth == 0 => {
                out.push(&text[start..index]);
                start = index + 1;
            }
            _ => {}
        }
    }
    if depth != 0 {
        return Err("unclosed parenthesis".into());
    }
    out.push(&text[start..]);
    if out.iter().any(|s| s.trim().is_empty()) {
        return Err("empty selector/query alternative".into());
    }
    Ok(out)
}
fn media(query: &str, context: &GpuResolveContext) -> Result<bool, String> {
    let mut any = false;
    for arm in split_commas(query)? {
        let arm = arm.trim();
        let (negated, arm) = if let Some(rest) = arm.strip_prefix("not ") {
            (true, rest)
        } else {
            (false, arm.strip_prefix("only ").unwrap_or(arm))
        };
        let mut all = true;
        let mut depth = 0usize;
        let mut start = 0;
        let mut parts = Vec::new();
        for (index, c) in arm.char_indices() {
            match c {
                '(' => depth += 1,
                ')' => depth = depth.checked_sub(1).ok_or("unexpected media parenthesis")?,
                _ => {}
            }
            if depth == 0 && arm[index..].starts_with(" and ") {
                parts.push(&arm[start..index]);
                start = index + 5;
            }
        }
        parts.push(&arm[start..]);
        for part in parts {
            let part = part.trim();
            if matches!(part, "all" | "screen") {
                continue;
            }
            let body = part
                .strip_prefix('(')
                .and_then(|s| s.strip_suffix(')'))
                .ok_or_else(|| format!("unsupported media query {part}"))?;
            all &= media_feature(body.trim(), context)?;
        }
        any |= if negated { !all } else { all };
    }
    Ok(any)
}
fn media_feature(body: &str, context: &GpuResolveContext) -> Result<bool, String> {
    if let Some((name, value)) = body.split_once(':') {
        let name = name.trim();
        let value = value.trim();
        match name {
            "hover" => {
                return match value {
                    "hover" => Ok(context.hover_capable),
                    "none" => Ok(!context.hover_capable),
                    _ => Err("unsupported hover media feature".into()),
                }
            }
            "orientation" => {
                return match value {
                    "portrait" => Ok(context.viewport[1] >= context.viewport[0]),
                    "landscape" => Ok(context.viewport[0] > context.viewport[1]),
                    _ => Err("unsupported orientation".into()),
                }
            }
            _ => {}
        }
        let (name, operator) = if let Some(name) = name.strip_prefix("min-") {
            (name, ">=")
        } else if let Some(name) = name.strip_prefix("max-") {
            (name, "<=")
        } else {
            (name, "=")
        };
        let actual = viewport(name, context)?;
        return Ok(compare(actual, operator, media_length(value, context)?));
    }
    let mut tokens = Vec::new();
    let mut rest = body;
    while !rest.trim().is_empty() {
        rest = rest.trim_start();
        if rest.starts_with(['<', '>', '=']) {
            let len = if rest.as_bytes().get(1) == Some(&b'=') {
                2
            } else {
                1
            };
            tokens.push(&rest[..len]);
            rest = &rest[len..];
        } else {
            let end = rest
                .find(|c: char| c.is_whitespace() || "<>=".contains(c))
                .unwrap_or(rest.len());
            tokens.push(&rest[..end]);
            rest = &rest[end..];
        }
    }
    if tokens
        .iter()
        .any(|token| matches!(*token, "==" | "=>" | "=<"))
    {
        return Err("invalid media comparison operator".into());
    }
    if tokens.len() == 3 {
        if let Ok(actual) = viewport(tokens[0], context) {
            return Ok(compare(
                actual,
                tokens[1],
                media_length(tokens[2], context)?,
            ));
        }
        return Ok(compare(
            media_length(tokens[0], context)?,
            tokens[1],
            viewport(tokens[2], context)?,
        ));
    }
    if tokens.len() == 5 {
        if !((matches!(tokens[1], "<" | "<=") && matches!(tokens[3], "<" | "<="))
            || (matches!(tokens[1], ">" | ">=") && matches!(tokens[3], ">" | ">=")))
        {
            return Err("media range comparisons must have the same direction".into());
        }
        let actual = viewport(tokens[2], context)?;
        return Ok(
            compare(media_length(tokens[0], context)?, tokens[1], actual)
                && compare(actual, tokens[3], media_length(tokens[4], context)?),
        );
    }
    Err(format!("unsupported viewport feature {body}"))
}
fn viewport(name: &str, context: &GpuResolveContext) -> Result<f64, String> {
    match name {
        "width" => Ok(context.viewport[0]),
        "height" => Ok(context.viewport[1]),
        _ => Err(format!("unsupported media feature {name}")),
    }
}
fn media_length(value: &str, context: &GpuResolveContext) -> Result<f64, String> {
    let (number, multiplier) = if let Some(number) = value
        .strip_suffix("rem")
        .or_else(|| value.strip_suffix("em"))
    {
        (number, context.media_rem_px)
    } else if let Some(number) = value.strip_suffix("px") {
        (number, 1.)
    } else if value == "0" {
        (value, 1.)
    } else {
        return Err(format!("unsupported viewport query unit {value}"));
    };
    let number = number
        .trim()
        .parse::<f64>()
        .map_err(|_| format!("invalid viewport threshold {value}"))?;
    let pixels = number * multiplier;
    if !pixels.is_finite() || pixels < 0. {
        return Err("viewport threshold must be finite and nonnegative".into());
    }
    Ok(pixels)
}
fn compare(left: f64, op: &str, right: f64) -> bool {
    match op {
        "<" => left < right,
        ">" => left > right,
        "<=" => left <= right,
        ">=" => left >= right,
        "=" => left == right,
        _ => false,
    }
}
