//! CSS scanning follows the fixed upstream grammar with explicit depth limits.

use super::{
    ast::AstNode,
    public_api::{Source, SourceLocation},
    CompileError,
};
use crate::text::Text;
use std::sync::Arc;

pub fn parse(css: &str, max_depth: usize) -> Result<Vec<AstNode>, CompileError> {
    parse_text(&css.into(), max_depth)
}

pub fn parse_text(css: &Text, max_depth: usize) -> Result<Vec<AstNode>, CompileError> {
    parse_with_source(css, max_depth, None)
}

pub(super) fn parse_with_source(
    css: &Text,
    max_depth: usize,
    source: Option<Arc<Source>>,
) -> Result<Vec<AstNode>, CompileError> {
    Parser {
        css: css.units(),
        offset: 0,
        max_depth,
        licenses: Vec::new(),
        source,
    }
    .parse()
}

struct Parser<'a> {
    css: &'a [u16],
    offset: usize,
    max_depth: usize,
    licenses: Vec<AstNode>,
    source: Option<Arc<Source>>,
}

struct Frame {
    header: Vec<u16>,
    source: Option<SourceLocation>,
    nodes: Vec<AstNode>,
}

impl Parser<'_> {
    fn location(&self, start: usize, end: usize) -> Option<SourceLocation> {
        self.source.as_ref().map(|source| SourceLocation {
            public: None,
            source: source.clone(),
            start,
            end,
        })
    }
    fn located(&self, mut node: AstNode, start: usize, end: usize) -> AstNode {
        node.set_source(self.location(start, end));
        node.set_destination(self.location(start, end));
        node
    }
    fn error(&self, offset: usize, message: impl Into<Text>) -> CompileError {
        CompileError::Syntax {
            offset,
            message: message.into(),
        }
    }

    fn character(&self, offset: usize) -> Option<u16> {
        self.css.get(offset).copied()
    }

    fn next(&self, offset: usize) -> usize {
        offset.saturating_add(1).min(self.css.len())
    }

    fn check_depth(&self, depth: usize) -> Result<(), CompileError> {
        if depth > self.max_depth {
            return Err(CompileError::Limit {
                resource: "ast_depth".into(),
                limit: self.max_depth,
            });
        }
        Ok(())
    }

    fn comment_end(&self, start: usize) -> Option<usize> {
        let mut offset = start + 2;
        while let Some(ch) = self.character(offset) {
            if ch == 92 {
                offset = self.next(self.next(offset));
            } else if ch == 42 && self.character(offset + 1) == Some(47) {
                return Some(offset + 2);
            } else {
                offset = self.next(offset);
            }
        }
        None
    }

    fn string_end(&self, start: usize, quote: u16) -> Result<usize, CompileError> {
        let mut offset = self.next(start);
        while let Some(ch) = self.character(offset) {
            if ch == 92 {
                offset = self.next(self.next(offset));
                continue;
            }
            if ch == quote {
                return Ok(offset);
            }
            let next = self.next(offset);
            if ch == 59
                && (self.character(next) == Some(10)
                    || self.character(next) == Some(13) && self.character(next + 1) == Some(10))
            {
                return Err(self.error(
                    offset,
                    message("Unterminated string: ", &self.css[start..next], &[quote]),
                ));
            }
            if ch == 10 || ch == 13 && self.character(next) == Some(10) {
                return Err(self.error(
                    offset,
                    message("Unterminated string: ", &self.css[start..offset], &[quote]),
                ));
            }
            offset = next;
        }
        Ok(start)
    }

    fn custom_property(&mut self, depth: usize) -> Result<AstNode, CompileError> {
        let start = self.offset;
        let mut offset = start + 2;
        let mut resume = self.next(start);
        let mut brackets = Vec::new();
        let mut buffer: &[u16] = &[];
        let mut colon = None;
        while let Some(ch) = self.character(offset) {
            let next = self.next(offset);
            match ch {
                92 => {
                    offset = self.next(next);
                    continue;
                }
                39 | 34 => {
                    offset = self.next(self.string_end(offset, ch)?);
                    continue;
                }
                47 if self.character(next) == Some(42) => {
                    offset = self.comment_end(offset).unwrap_or(next);
                    continue;
                }
                58 if colon.is_none() => colon = Some(offset - start),
                59 if brackets.is_empty() => {
                    buffer = &self.css[start..offset];
                    resume = next;
                    break;
                }
                40 | 91 | 123 => {
                    brackets.push(match ch {
                        40 => 41,
                        91 => 93,
                        _ => 125,
                    });
                    self.check_depth(depth.saturating_add(brackets.len()))?;
                }
                _ if (ch == 125 || next == self.css.len()) && brackets.is_empty() => {
                    // The upstream EOF scan leaves the final character for the outer loop.
                    buffer = &self.css[start..offset];
                    resume = offset;
                    break;
                }
                41 | 93 | 125 if brackets.last() == Some(&ch) => {
                    brackets.pop();
                }
                _ => {}
            }
            offset = next;
        }
        self.offset = resume;
        declaration(buffer, colon)
            .map(|node| self.located(node, start, offset))
            .ok_or_else(|| self.error(start, "Invalid custom property, expected a value"))
    }

    fn parse(mut self) -> Result<Vec<AstNode>, CompileError> {
        fn append(nodes: &mut Vec<AstNode>, parents: &mut [Frame], node: AstNode) {
            if let Some(parent) = parents.last_mut() {
                parent.nodes.push(node);
            } else {
                nodes.push(node);
            }
        }
        let mut nodes = Vec::new();
        let mut parents: Vec<Frame> = Vec::new();
        let mut brackets = Vec::new();
        let mut buffer = Vec::new();
        let mut buffer_start = 0;
        while let Some(ch) = self.character(self.offset) {
            let start = self.offset;
            let next = self.next(start);
            self.offset = next;
            if start == 0 && ch == 65279 {
                continue;
            }
            if ch == 13 && self.character(next) == Some(10) {
                continue;
            }
            if ch == 92 {
                if buffer.is_empty() {
                    buffer_start = start;
                }
                self.offset = self.next(next);
                buffer.extend_from_slice(&self.css[start..self.offset]);
            } else if ch == 47 && self.character(next) == Some(42) {
                if let Some(end) = self.comment_end(start) {
                    if self.character(start + 2) == Some(33) {
                        self.licenses.push(AstNode::Comment {
                            origin: None,
                            source: self.location(start, end),
                            destination: self.location(start, end),
                            value: Text::from_units(self.css[start + 2..end - 2].to_vec()),
                        });
                    }
                    self.offset = end;
                }
            } else if matches!(ch, 39 | 34) {
                let end = self.string_end(start, ch)?;
                self.offset = self.next(end);
                buffer.extend_from_slice(&self.css[start..self.offset]);
            } else if matches!(ch, 32 | 10 | 9)
                && (matches!(self.character(next), Some(32 | 10 | 9))
                    || self.character(next) == Some(13) && self.character(next + 1) == Some(10))
            {
                continue;
            } else if ch == 10 {
                if !buffer.is_empty() && !matches!(buffer.last(), Some(32 | 10 | 9)) {
                    buffer.push(32);
                }
            } else if ch == 45 && self.character(next) == Some(45) && buffer.is_empty() {
                self.offset = start;
                let node = self.custom_property(brackets.len())?;
                append(&mut nodes, &mut parents, node);
            } else if ch == 59 && buffer.first() == Some(&64) {
                append(
                    &mut nodes,
                    &mut parents,
                    self.located(at_rule_units(&buffer, None), buffer_start, start),
                );
                buffer.clear();
            } else if ch == 59 && brackets.last() != Some(&41) {
                if let Some(node) = declaration(&buffer, buffer.iter().position(|unit| *unit == 58))
                {
                    append(
                        &mut nodes,
                        &mut parents,
                        self.located(node, buffer_start, start),
                    );
                } else if !buffer.is_empty() {
                    return Err(self.error(
                        start,
                        message("Invalid declaration: `", trim(&buffer), &[96]),
                    ));
                }
                buffer.clear();
            } else if ch == 123 && brackets.last() != Some(&41) {
                brackets.push(125);
                self.check_depth(brackets.len())?;
                parents.push(Frame {
                    header: trim(&buffer).to_vec(),
                    source: self.location(buffer_start, start),
                    nodes: Vec::new(),
                });
                buffer.clear();
            } else if ch == 125 && brackets.last() != Some(&41) {
                if brackets.pop().is_none() {
                    return Err(self.error(start, "Missing opening {"));
                }
                if !buffer.is_empty() {
                    if buffer.first() == Some(&64) {
                        append(
                            &mut nodes,
                            &mut parents,
                            self.located(at_rule_units(&buffer, None), buffer_start, start),
                        );
                    } else if !parents.is_empty() {
                        let node = declaration(&buffer, buffer.iter().position(|unit| *unit == 58))
                            .ok_or_else(|| {
                                self.error(
                                    start,
                                    message("Invalid declaration: `", trim(&buffer), &[96]),
                                )
                            })?;
                        append(
                            &mut nodes,
                            &mut parents,
                            self.located(node, buffer_start, start),
                        );
                    }
                }
                if let Some(parent) = parents.pop() {
                    let mut node = if parent.header.first() == Some(&64) {
                        at_rule_units(&parent.header, Some(parent.nodes))
                    } else {
                        AstNode::Rule {
                            origin: None,
                            source: None,
                            destination: None,
                            selector: Text::from_units(parent.header),
                            nodes: parent.nodes,
                        }
                    };
                    node.set_source(parent.source.clone());
                    node.set_destination(parent.source);
                    append(&mut nodes, &mut parents, node);
                }
                buffer.clear();
            } else if ch == 40 {
                brackets.push(41);
                self.check_depth(brackets.len())?;
                buffer.push(ch);
            } else if ch == 41 {
                if brackets.last() != Some(&41) {
                    return Err(self.error(start, "Missing opening ("));
                }
                brackets.pop();
                buffer.push(ch);
            } else {
                if buffer.is_empty() && matches!(ch, 32 | 10 | 9) {
                    continue;
                }
                if buffer.is_empty() {
                    buffer_start = start;
                }
                buffer.push(ch);
            }
        }
        if buffer.first() == Some(&64) {
            nodes.push(self.located(at_rule_units(&buffer, None), buffer_start, self.css.len()));
        }
        if let Some(parent) = parents.last().filter(|_| !brackets.is_empty()) {
            let mut name = parent.header.clone();
            if parent.header.first() == Some(&64) {
                if let AstNode::AtRule {
                    name: rule_name,
                    params,
                    ..
                } = at_rule_units(&parent.header, None)
                {
                    name = rule_name.into_units();
                    name.push(32);
                    name.extend(params.into_units());
                }
            }
            return Err(self.error(self.offset, message("Missing closing } at ", &name, &[])));
        }
        self.licenses.append(&mut nodes);
        Ok(self.licenses)
    }
}

fn message(prefix: &str, units: &[u16], suffix: &[u16]) -> Text {
    let mut output: Vec<_> = prefix.encode_utf16().collect();
    output.extend_from_slice(units);
    output.extend_from_slice(suffix);
    Text::from_units(output)
}

fn trim(value: &[u16]) -> &[u16] {
    let whitespace =
        |unit: &u16| char::from_u32(u32::from(*unit)).is_some_and(super::value::js_whitespace);
    let start = value.iter().take_while(|unit| whitespace(unit)).count();
    let end = value.len()
        - value[start..]
            .iter()
            .rev()
            .take_while(|unit| whitespace(unit))
            .count();
    &value[start..end]
}

#[cfg(test)]
pub(super) fn at_rule(header: &str, nodes: Option<Vec<AstNode>>) -> AstNode {
    at_rule_units(&header.encode_utf16().collect::<Vec<_>>(), nodes)
}

pub(super) fn at_rule_text(header: &Text, nodes: Option<Vec<AstNode>>) -> AstNode {
    at_rule_units(header.units(), nodes)
}

fn at_rule_units(header: &[u16], nodes: Option<Vec<AstNode>>) -> AstNode {
    let boundary = header
        .iter()
        .enumerate()
        .skip(5)
        .find_map(|(index, unit)| matches!(*unit, 32 | 40).then_some(index))
        .unwrap_or(header.len());
    AstNode::AtRule {
        origin: None,
        source: None,
        destination: None,
        name: Text::from_units(trim(&header[..boundary]).to_vec()),
        params: Text::from_units(trim(&header[boundary..]).to_vec()),
        nodes,
    }
}

fn declaration(buffer: &[u16], colon: Option<usize>) -> Option<AstNode> {
    let colon = colon?;
    let property = trim(&buffer[..colon.min(buffer.len())]);
    let raw = buffer.get(colon.saturating_add(1)..).unwrap_or_default();
    let important = raw
        .windows(10)
        .position(|part| part == [33, 105, 109, 112, 111, 114, 116, 97, 110, 116]);
    let mut node = AstNode::declaration(
        Text::from_units(property.to_vec()),
        Text::from_units(trim(&raw[..important.unwrap_or(raw.len())]).to_vec()),
    );
    if let AstNode::Declaration {
        important: flag, ..
    } = &mut node
    {
        *flag = important.is_some();
    }
    Some(node)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_declarations_match_the_official_parser() {
        let nodes = parse("--radius-empty: ; color: ; content: !important; : red;", 32)
            .expect("the official parser accepts empty declaration fields");
        assert_eq!(
            nodes,
            vec![
                AstNode::Declaration {
                    origin: None,
                    source: None,
                    destination: None,
                    property: "--radius-empty".into(),
                    value: "".into(),
                    important: false,
                    identity: Default::default(),
                },
                AstNode::Declaration {
                    origin: None,
                    source: None,
                    destination: None,
                    property: "color".into(),
                    value: "".into(),
                    important: false,
                    identity: Default::default(),
                },
                AstNode::Declaration {
                    origin: None,
                    source: None,
                    destination: None,
                    property: "content".into(),
                    value: "".into(),
                    important: true,
                    identity: Default::default(),
                },
                AstNode::Declaration {
                    origin: None,
                    source: None,
                    destination: None,
                    property: "".into(),
                    value: "red".into(),
                    important: false,
                    identity: Default::default(),
                },
            ]
        );
    }

    #[test]
    fn short_at_rule_boundary_uses_utf16_positions() {
        let actual: Vec<_> = ["@é é x foo", "@🙂 a b", "@x a"]
            .map(|header| at_rule(header, None))
            .into();
        let expected: Vec<_> = [("@é é x", "foo"), ("@🙂 a", "b"), ("@x a", "")]
            .map(|(name, params)| AstNode::AtRule {
                origin: None,
                source: None,
                destination: None,
                name: name.into(),
                params: params.into(),
                nodes: None,
            })
            .into();
        assert_eq!(actual, expected);
    }

    #[test]
    fn custom_property_eof_retains_the_upstream_final_character_quirk() {
        assert_eq!(
            parse("--raw:13px", 32).unwrap(),
            vec![AstNode::declaration("--raw", "13p")]
        );
    }

    #[test]
    fn custom_property_nested_values_still_enforce_depth_limits() {
        assert_eq!(
            parse("--raw:foo(bar(baz));", 1).unwrap_err(),
            CompileError::Limit {
                resource: "ast_depth".into(),
                limit: 1
            }
        );
    }
}
