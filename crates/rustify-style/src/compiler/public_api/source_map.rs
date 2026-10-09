//! Decoded maps preserve the identity of both authored and generated sources.

use crate::text::Text;
use serde::{Serialize, Serializer};
use std::{collections::BTreeMap, sync::Arc};

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct Source {
    pub file: Option<String>,
    pub code: Text,
}

/// Offsets count UTF-16 units, as in the fixed upstream compiler.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceLocation {
    pub source: Arc<Source>,
    pub start: usize,
    pub end: usize,
    /// Canonical metadata supplied through the public AST, including shared
    /// Source objects and JavaScript number offsets.
    pub public: Option<Arc<super::PublicValue>>,
}
impl SourceLocation {
    pub(in crate::compiler) fn undefined() -> Self {
        use super::PublicValue;
        Self {
            source: Arc::new(Source {
                file: None,
                code: Text::default(),
            }),
            start: 0,
            end: 0,
            public: Some(Arc::new(
                super::design_system::public_json!({"undefined": true}),
            )),
        }
    }

    pub(in crate::compiler) fn offset_public(&mut self, offset: usize, length: usize) {
        use super::PublicValue as Value;
        fn add(value: &Value, offset: usize) -> Option<Value> {
            if let Some(value) = value.as_u64() {
                return Some(Value::from(value.saturating_add(offset as u64)));
            }
            match value {
                Value::Number(value) => {
                    serde_json::Number::from_f64(value.as_f64()? + offset as f64).map(Value::Number)
                }
                _ => None,
            }
        }
        if let Some(public) = &self.public {
            let mut value = (**public).clone();
            if let Some(tuple) = value.as_array_mut().filter(|tuple| tuple.len() == 3) {
                if let Some(start) = add(&tuple[1], offset) {
                    if let Some(end) = add(&start, length) {
                        tuple[1] = start;
                        tuple[2] = end;
                    }
                }
            }
            self.public = Some(Arc::new(value));
        }
    }
}
impl Serialize for SourceLocation {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        if let Some(public) = &self.public {
            return public.serialize(serializer);
        }
        (&*self.source, self.start, self.end).serialize(serializer)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

pub struct LineTable {
    starts: Vec<usize>,
}
impl LineTable {
    pub fn new(source: &Text) -> Self {
        let mut starts = vec![0];
        starts.extend(
            source
                .units()
                .iter()
                .enumerate()
                .filter_map(|(index, unit)| (*unit == 10).then_some(index + 1)),
        );
        Self { starts }
    }
    pub fn find(&self, offset: usize) -> Position {
        let line = self
            .starts
            .partition_point(|start| *start <= offset)
            .saturating_sub(1);
        Position {
            line: line + 1,
            column: offset.saturating_sub(self.starts[line]),
        }
    }
    pub fn find_offset(&self, position: Position) -> usize {
        let line = position.line.saturating_sub(1).min(self.starts.len() - 1);
        let start = self.starts[line];
        start
            .saturating_add(position.column)
            .min(self.starts.get(line + 1).copied().unwrap_or(start))
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
pub struct DecodedSource {
    pub url: Option<String>,
    pub content: Option<Text>,
    pub ignore: bool,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct OriginalPosition {
    pub source: Arc<DecodedSource>,
    pub position: Position,
}
impl Serialize for OriginalPosition {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct Fields<'a> {
            source: &'a DecodedSource,
            #[serde(flatten)]
            position: Position,
        }
        Fields {
            source: &self.source,
            position: self.position,
        }
        .serialize(serializer)
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DecodedMapping {
    pub name: Option<String>,
    pub original_position: Option<OriginalPosition>,
    pub generated_position: Position,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DecodedSourceMap {
    pub file: Option<String>,
    pub sources: Vec<Arc<DecodedSource>>,
    pub mappings: Vec<DecodedMapping>,
}
impl Serialize for DecodedSourceMap {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        #[derive(Serialize)]
        struct Fields<'a> {
            file: &'a Option<String>,
            sources: Vec<&'a DecodedSource>,
            mappings: &'a [DecodedMapping],
        }
        Fields {
            file: &self.file,
            sources: self.sources.iter().map(AsRef::as_ref).collect(),
            mappings: &self.mappings,
        }
        .serialize(serializer)
    }
}

/// The iterator follows AST traversal order. Equal source text in distinct
/// handles remains distinct; every mapping references its table entry's handle.
pub fn create_source_map<'a>(
    locations: impl IntoIterator<Item = (Option<&'a SourceLocation>, Option<&'a SourceLocation>)>,
) -> DecodedSourceMap {
    struct Entry {
        table: LineTable,
        decoded: Arc<DecodedSource>,
    }
    let mut indexes = BTreeMap::new();
    let mut entries = Vec::<Entry>::new();
    let mut mappings = Vec::new();
    for (src, dst) in locations {
        let (Some(src), Some(dst)) = (src, dst) else {
            continue;
        };
        if src.source.code.is_empty() {
            continue;
        }
        for location in [src, dst] {
            let identity = Arc::as_ptr(&location.source) as usize;
            if let std::collections::btree_map::Entry::Vacant(index) = indexes.entry(identity) {
                index.insert(entries.len());
                entries.push(Entry {
                    table: LineTable::new(&location.source.code),
                    decoded: Arc::new(DecodedSource {
                        url: location.source.file.clone(),
                        content: Some(location.source.code.clone()),
                        ignore: false,
                    }),
                });
            }
        }
        let original = &entries[indexes[&(Arc::as_ptr(&src.source) as usize)]];
        let generated = &entries[indexes[&(Arc::as_ptr(&dst.source) as usize)]];
        let units = src.source.code.units();
        let start = src.start.min(units.len());
        let end = src.end.min(units.len()).max(start);
        let mut offset = 0;
        for line in units[start..end].split(|unit| *unit == 10) {
            if line.iter().any(|unit| !matches!(*unit, 9..=13 | 0x20 | 0xa0 | 0x1680 | 0x2000..=0x200a | 0x2028 | 0x2029 | 0x202f | 0x205f | 0x3000 | 0xfeff)) {
                mappings.push(DecodedMapping { name: None, original_position: Some(OriginalPosition { source: original.decoded.clone(), position: original.table.find(src.start + offset) }), generated_position: generated.table.find(dst.start) });
            }
            offset += line.len() + 1;
        }
        mappings.push(DecodedMapping {
            name: None,
            original_position: Some(OriginalPosition {
                source: original.decoded.clone(),
                position: original.table.find(src.end),
            }),
            generated_position: generated.table.find(dst.end),
        });
    }
    mappings.sort_by_key(|mapping| {
        (
            mapping.generated_position.line,
            mapping.generated_position.column,
            mapping
                .original_position
                .as_ref()
                .map_or(0, |original| original.position.line),
            mapping
                .original_position
                .as_ref()
                .map_or(0, |original| original.position.column),
        )
    });
    DecodedSourceMap {
        file: None,
        sources: entries.into_iter().map(|entry| entry.decoded).collect(),
        mappings,
    }
}

pub type Translation = (Position, Position, Option<Position>, Option<Position>);
/// Returns the same one-range translation callback as the fixed upstream helper.
pub fn create_translation_map(
    original: &Text,
    generated: &Text,
) -> impl Fn(Option<&SourceLocation>, Option<&SourceLocation>) -> Vec<Translation> {
    let original = LineTable::new(original);
    let generated = LineTable::new(generated);
    move |src, dst| {
        src.map(|src| {
            vec![(
                original.find(src.start),
                original.find(src.end),
                dst.map(|dst| generated.find(dst.start)),
                dst.map(|dst| generated.find(dst.end)),
            )]
        })
        .unwrap_or_default()
    }
}

pub(super) fn initial_map(nodes: &[crate::compiler::ast::AstNode]) -> DecodedSourceMap {
    fn collect<'a>(
        nodes: &'a [crate::compiler::ast::AstNode],
        locations: &mut Vec<(Option<&'a SourceLocation>, Option<&'a SourceLocation>)>,
    ) {
        for node in nodes {
            locations.push((node.source(), node.destination()));
            if let Some(children) = node.children() {
                collect(children, locations);
            }
        }
    }
    let mut locations = Vec::new();
    collect(nodes, &mut locations);
    create_source_map(locations)
}

pub(super) fn printed_map(nodes: &[crate::compiler::ast::AstNode]) -> DecodedSourceMap {
    let generated = Arc::new(Source {
        file: None,
        code: crate::compiler::ast::to_css(nodes),
    });
    let locations: Vec<_> = destinations(nodes)
        .into_iter()
        .map(|(node, start, end)| {
            (
                node.source().cloned(),
                SourceLocation {
                    public: None,
                    source: generated.clone(),
                    start,
                    end,
                },
            )
        })
        .collect();
    create_source_map(locations.iter().map(|(src, dst)| (src.as_ref(), Some(dst))))
}

/// Printed UTF-16 span of each node `to_css` emits, in print order. Context
/// and at-root nodes are already flattened before printing and are skipped.
pub(in crate::compiler) fn destinations(
    nodes: &[crate::compiler::ast::AstNode],
) -> Vec<(&crate::compiler::ast::AstNode, usize, usize)> {
    use crate::compiler::ast::AstNode;
    fn collect<'a>(
        nodes: &'a [AstNode],
        depth: usize,
        pos: &mut usize,
        output: &mut Vec<(&'a AstNode, usize, usize)>,
    ) {
        for node in nodes {
            if matches!(node, AstNode::Context { .. } | AstNode::AtRoot { .. }) {
                continue;
            }
            let indent = depth * 2;
            *pos += indent;
            let start = *pos;
            match node {
                AstNode::Declaration {
                    property,
                    value,
                    important,
                    ..
                } => {
                    *pos += property.units().len()
                        + 2
                        + value.units().len()
                        + usize::from(*important) * 11
                }
                AstNode::Rule { selector, .. } => *pos += selector.units().len() + 1,
                AstNode::AtRule {
                    name,
                    params,
                    nodes,
                    ..
                } => {
                    *pos += name.units().len()
                        + if nodes.as_ref().is_none_or(Vec::is_empty) {
                            1 + params.units().len()
                        } else {
                            1 + usize::from(!params.is_empty()) + params.units().len()
                        }
                }
                AstNode::Comment { value, .. } => *pos += 4 + value.units().len(),
                _ => {}
            }
            output.push((node, start, *pos));
            *pos += if matches!(node, AstNode::Comment { .. }) {
                1
            } else {
                2
            };
            if let Some(children) = node.children().filter(|children| !children.is_empty()) {
                collect(children, depth + 1, pos, output);
                *pos += indent + 2;
            } else if matches!(node, AstNode::Rule { .. }) {
                *pos += indent + 2;
            }
        }
    }
    let mut output = Vec::new();
    collect(nodes, 0, &mut 0, &mut output);
    output
}

pub(super) fn retained_ast_bytes(nodes: &[crate::compiler::ast::AstNode]) -> usize {
    use crate::compiler::ast::AstNode;
    std::mem::size_of_val(nodes).saturating_add(
        nodes
            .iter()
            .map(|node| {
                let own = match node {
                    AstNode::Rule { selector, .. } => selector.owned_bytes(),
                    AstNode::AtRule { name, params, .. } => {
                        name.owned_bytes().saturating_add(params.owned_bytes())
                    }
                    AstNode::Declaration {
                        property, value, ..
                    } => property.owned_bytes().saturating_add(value.owned_bytes()),
                    AstNode::Comment { value, .. } => value.owned_bytes(),
                    AstNode::Context {
                        base, source_base, ..
                    } => base
                        .owned_bytes()
                        .saturating_add(source_base.as_ref().map_or(0, Text::owned_bytes)),
                    AstNode::AtRoot { .. } => 0,
                };
                own.saturating_add(node.children().map_or(0, retained_ast_bytes))
            })
            .fold(0usize, usize::saturating_add),
    )
}

/// Conservative peak: locations, cloned decoded text, line tables, source
/// indexes, vector spare capacity and the stable sort's scratch storage.
pub(super) fn allocation_budget(
    nodes: &[crate::compiler::ast::AstNode],
    generated: Option<(usize, usize)>,
) -> usize {
    use crate::compiler::ast::AstNode;
    struct Usage {
        nodes: usize,
        mappings: usize,
        sources: BTreeMap<usize, Arc<Source>>,
    }
    fn count(nodes: &[AstNode], generated: bool, usage: &mut Usage) {
        for node in nodes {
            usage.nodes = usage.nodes.saturating_add(1);
            if let Some(src) = node.source().filter(|src| {
                !src.source.code.is_empty() && (generated || node.destination().is_some())
            }) {
                usage
                    .sources
                    .entry(Arc::as_ptr(&src.source) as usize)
                    .or_insert_with(|| src.source.clone());
                if !generated {
                    if let Some(dst) = node.destination() {
                        usage
                            .sources
                            .entry(Arc::as_ptr(&dst.source) as usize)
                            .or_insert_with(|| dst.source.clone());
                    }
                }
                let units = src.source.code.units();
                let start = src.start.min(units.len());
                let end = src.end.min(units.len()).max(start);
                let lines=units[start..end].split(|unit|*unit==10).filter(|line|line.iter().any(|unit| !matches!(*unit, 9..=13 | 0x20 | 0xa0 | 0x1680 | 0x2000..=0x200a | 0x2028 | 0x2029 | 0x202f | 0x205f | 0x3000 | 0xfeff))).count();
                usage.mappings = usage.mappings.saturating_add(lines.saturating_add(1));
            }
            if let Some(children) = node.children() {
                count(children, generated, usage);
            }
        }
    }
    let mut usage = Usage {
        nodes: 0,
        mappings: 0,
        sources: BTreeMap::new(),
    };
    count(nodes, generated.is_some(), &mut usage);
    let source_bytes = usage
        .sources
        .values()
        .map(|source| {
            std::mem::size_of::<DecodedSource>()
                .saturating_add(2 * std::mem::size_of::<usize>())
                .saturating_add(source.file.as_ref().map_or(0, String::capacity))
                .saturating_add(source.code.owned_bytes())
                .saturating_add(
                    source
                        .code
                        .units()
                        .iter()
                        .filter(|unit| **unit == 10)
                        .count()
                        .saturating_add(1)
                        .saturating_mul(2 * std::mem::size_of::<usize>()),
                )
        })
        .fold(0usize, usize::saturating_add);
    let generated_bytes = generated.map_or(0, |(units, lines)| {
        units
            .saturating_mul(8)
            .saturating_add(lines.saturating_mul(2 * std::mem::size_of::<usize>()))
            .saturating_add(std::mem::size_of::<Source>())
            .saturating_add(std::mem::size_of::<DecodedSource>())
            .saturating_add(4 * std::mem::size_of::<usize>())
    });
    source_bytes
        .saturating_add(generated_bytes)
        .saturating_add(
            usage
                .sources
                .len()
                .saturating_add(usize::from(generated.is_some()))
                .saturating_mul(
                    16 * std::mem::size_of::<usize>() + 2 * std::mem::size_of::<LineTable>(),
                ),
        )
        .saturating_add(usage.nodes.saturating_mul(if generated.is_some() {
            2 * std::mem::size_of::<(Option<SourceLocation>, SourceLocation)>()
        } else {
            2 * std::mem::size_of::<(Option<&SourceLocation>, Option<&SourceLocation>)>()
        }))
        .saturating_add(
            usage
                .mappings
                .saturating_mul(3 * std::mem::size_of::<DecodedMapping>()),
        )
}

pub(super) fn structure_budget(nodes: &[crate::compiler::ast::AstNode], generated: bool) -> usize {
    fn count(nodes: &[crate::compiler::ast::AstNode]) -> usize {
        nodes.len().saturating_add(
            nodes
                .iter()
                .map(|node| node.children().map_or(0, count))
                .fold(0usize, usize::saturating_add),
        )
    }
    // Preflight even the auxiliary source-index collection before allocating it.
    count(nodes).saturating_mul(
        16 * std::mem::size_of::<usize>()
            + if generated {
                2 * std::mem::size_of::<(Option<SourceLocation>, SourceLocation)>()
            } else {
                2 * std::mem::size_of::<(Option<&SourceLocation>, Option<&SourceLocation>)>()
            },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn printed_map_tracks_raw_ast_units_and_emitted_line_columns() {
        use crate::compiler::ast::AstNode;

        let source = Arc::new(Source {
            file: Some("authored.css".into()),
            code: Text::from("R\nA\nC\nL"),
        });
        let location = |start| {
            Some(SourceLocation {
                public: None,
                source: source.clone(),
                start,
                end: start + 1,
            })
        };
        let nodes = vec![
            AstNode::Rule {
                origin: None,
                source: location(0),
                destination: None,
                selector: Text::from_units(vec![46, 0xd800, 10, 0xd83d, 0xde00]),
                nodes: vec![AstNode::AtRule {
                    origin: None,
                    source: location(2),
                    destination: None,
                    name: Text::from_units(vec![64, 0xd800]),
                    params: Text::from_units(vec![0xd83d, 0xde00, 10, 0xdc00]),
                    nodes: Some(vec![AstNode::Comment {
                        origin: None,
                        source: location(4),
                        destination: None,
                        value: Text::from_units(vec![0xdc00, 10, 0xd83d, 0xde00]),
                    }]),
                }],
            },
            AstNode::AtRule {
                origin: None,
                source: location(6),
                destination: None,
                name: Text::from_units(vec![64, 0xdc00]),
                params: Text::from_units(vec![0xd83d, 0xde00, 10, 0xd800]),
                nodes: None,
            },
        ];
        let map = printed_map(&nodes);
        assert_eq!(map.sources.len(), 2);
        assert_eq!(map.sources[0].url.as_deref(), Some("authored.css"));
        assert_eq!(
            map.sources[0].content.as_ref().unwrap().units(),
            source.code.units()
        );
        assert_eq!(map.sources[1].url, None);
        // Explicit emitted CSS: selector/header, nested header/comment, both
        // closing braces, then the leaf at-rule. Newlines inside fields remain.
        assert_eq!(
            map.sources[1].content.as_ref().unwrap().units(),
            &[
                46, 0xd800, 10, 0xd83d, 0xde00, 32, 123, 10, 32, 32, 64, 0xd800, 32, 0xd83d,
                0xde00, 10, 0xdc00, 32, 123, 10, 32, 32, 32, 32, 47, 42, 0xdc00, 10, 0xd83d,
                0xde00, 42, 47, 10, 32, 32, 125, 10, 125, 10, 64, 0xdc00, 32, 0xd83d, 0xde00, 10,
                0xd800, 59, 10,
            ]
        );
        let positions: Vec<_> = map
            .mappings
            .iter()
            .map(|mapping| {
                let original = mapping.original_position.as_ref().unwrap();
                assert!(Arc::ptr_eq(&original.source, &map.sources[0]));
                assert_eq!(mapping.name, None);
                (
                    (original.position.line, original.position.column),
                    (
                        mapping.generated_position.line,
                        mapping.generated_position.column,
                    ),
                )
            })
            .collect();
        assert_eq!(
            positions,
            vec![
                ((1, 0), (1, 0)),
                ((1, 1), (2, 3)),
                ((2, 0), (3, 2)),
                ((2, 1), (4, 2)),
                ((3, 0), (5, 4)),
                ((3, 1), (6, 4)),
                ((4, 0), (9, 0)),
                ((4, 1), (10, 1)),
            ]
        );
    }

    #[test]
    fn retained_ast_bytes_charges_each_text_field_spare_capacity() {
        use crate::compiler::ast::AstNode;

        let mut source_units = Vec::with_capacity(4096);
        source_units.push(0xd800);
        let source = Arc::new(Source {
            file: Some("shared.css".into()),
            code: Text::from_units(source_units),
        });
        let location = Some(SourceLocation {
            public: None,
            source: source.clone(),
            start: 0,
            end: 1,
        });
        let mut nodes = vec![AstNode::Context {
            origin: None,
            base: Text::default(),
            source_base: Some(Text::default()),
            reference: false,
            nodes: vec![
                AstNode::Rule {
                    origin: None,
                    source: location.clone(),
                    destination: None,
                    selector: Text::default(),
                    nodes: Vec::new(),
                },
                AstNode::AtRule {
                    origin: None,
                    source: location.clone(),
                    destination: None,
                    name: Text::default(),
                    params: Text::default(),
                    nodes: None,
                },
                AstNode::Comment {
                    origin: None,
                    source: location,
                    destination: None,
                    value: Text::default(),
                },
            ],
        }];
        // This helper accounts for AST storage; shared Source allocations are
        // accounted for separately, rather than once for each location.
        let baseline = 4 * std::mem::size_of::<AstNode>();
        assert_eq!(retained_ast_bytes(&nodes), baseline);
        let mut expected = baseline;
        for (index, requested_capacity) in [31, 37, 41, 43, 47, 53].into_iter().enumerate() {
            let mut units = Vec::with_capacity(requested_capacity);
            units.extend_from_slice(&[0xd800, 10, 0xd83d, 0xde00]);
            assert!(units.capacity() > units.len());
            let allocation = units.capacity() * std::mem::size_of::<u16>();
            let text = Text::from_units(units);
            assert_eq!(text.owned_bytes(), allocation);
            let AstNode::Context {
                base,
                source_base,
                nodes: children,
                ..
            } = &mut nodes[0]
            else {
                panic!("expected context");
            };
            let field = match index {
                0 => base,
                1 => source_base.as_mut().unwrap(),
                2 => {
                    let AstNode::Rule { selector, .. } = &mut children[0] else {
                        panic!("expected rule");
                    };
                    selector
                }
                3 | 4 => {
                    let AstNode::AtRule { name, params, .. } = &mut children[1] else {
                        panic!("expected at-rule");
                    };
                    if index == 3 {
                        name
                    } else {
                        params
                    }
                }
                5 => {
                    let AstNode::Comment { value, .. } = &mut children[2] else {
                        panic!("expected comment");
                    };
                    value
                }
                _ => unreachable!(),
            };
            *field = text;
            expected += allocation;
            assert_eq!(retained_ast_bytes(&nodes), expected, "field {index}");
        }
        assert!(source.code.owned_bytes() > expected - baseline);
    }

    #[test]
    fn equal_text_sources_remain_distinct_and_mappings_share_decoded_handles() {
        let source = || {
            Arc::new(Source {
                file: Some("same.css".into()),
                code: Text::from("a😀\nb"),
            })
        };
        let a = SourceLocation {
            public: None,
            source: source(),
            start: 0,
            end: 3,
        };
        let b = SourceLocation {
            public: None,
            source: source(),
            start: 4,
            end: 5,
        };
        let generated = SourceLocation {
            public: None,
            source: Arc::new(Source {
                file: None,
                code: Text::from("abc\nd"),
            }),
            start: 0,
            end: 3,
        };
        let map = create_source_map([(Some(&a), Some(&generated)), (Some(&b), Some(&generated))]);
        assert_eq!(map.sources.len(), 3);
        for mapping in &map.mappings {
            assert!(map.sources.iter().any(|source| Arc::ptr_eq(
                source,
                &mapping.original_position.as_ref().unwrap().source
            )));
        }
        assert!(!Arc::ptr_eq(&map.sources[0], &map.sources[2]));
    }
    #[test]
    fn line_table_retains_non_scalar_units_and_upstream_offset_clamping() {
        let table = LineTable::new(&Text::from_units(vec![0xd800, 10, 0xdc00, 10]));
        assert_eq!(table.find(2), Position { line: 2, column: 0 });
        assert_eq!(table.find(6), Position { line: 3, column: 2 });
        assert_eq!(
            table.find_offset(Position {
                line: 99,
                column: 99
            }),
            4
        );
        assert_eq!(
            table.find_offset(Position {
                line: 1,
                column: 99
            }),
            2
        );
    }
    #[test]
    fn translation_preserves_absent_destination_and_ignores_absent_source() {
        let source = Arc::new(Source {
            file: None,
            code: Text::from("a\nb"),
        });
        let location = SourceLocation {
            public: None,
            source,
            start: 2,
            end: 3,
        };
        let translate = create_translation_map(&Text::from("a\nb"), &Text::from("abc"));
        assert!(translate(None, Some(&location)).is_empty());
        assert_eq!(
            translate(Some(&location), None),
            vec![(
                Position { line: 2, column: 0 },
                Position { line: 2, column: 1 },
                None,
                None
            )]
        );
    }
}
