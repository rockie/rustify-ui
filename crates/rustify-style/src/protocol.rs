//! Lightweight compiler requests, reports, and errors shared across WASM boundaries.

use crate::program::GpuProgram;
use crate::text::Text;
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompilerInput {
    pub css: Text,
    pub entry: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceAsset {
    pub id: String,
    pub content: Text,
}

#[cfg(test)]
mod css_input_text_tests {
    use super::{CompilerInput, SourceAsset};

    #[test]
    fn css_input_and_asset_decode_original_non_scalar_json_units() {
        let input: CompilerInput =
            serde_json::from_str(r#"{"entry":"","css":".\ud800 { value: \udc00; }"}"#).unwrap();
        let asset: SourceAsset =
            serde_json::from_str(r#"{"content":"/*! \udc00 */","id":"a.css"}"#).unwrap();
        assert!(serde_json::to_string(&input).unwrap().contains(r"\ud800"));
        assert!(serde_json::to_string(&asset).unwrap().contains(r"\udc00"));
    }
}

/// Caller-supplied raw source text; extensions use spelling without a leading dot.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceContent {
    pub content: String,
    pub extension: String,
}

/// A complete virtual source snapshot, including applicable ignore files and symlinks.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceSnapshot {
    pub root: String,
    pub base: String,
    pub entries: Vec<VirtualSourceEntry>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum VirtualSourceEntry {
    Directory {
        path: String,
    },
    File {
        path: String,
        content: String,
        revision: u64,
    },
    Symlink {
        path: String,
        target: String,
    },
    Unavailable {
        path: String,
    },
}

/// Source discovery requested by CSS; the core never traverses these paths.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize)]
pub struct SourceMetadata {
    pub sources: Vec<SourcePattern>,
    pub root: SourceRoot,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourcePattern {
    pub base: String,
    pub pattern: String,
    pub negated: bool,
}

/// Automatic discovery, explicit opt-out, or a CSS-authored discovery root.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum SourceRoot {
    #[default]
    Automatic,
    None,
    Path {
        base: String,
        pattern: String,
    },
}

impl Serialize for SourceRoot {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Automatic => serializer.serialize_none(),
            Self::None => serializer.serialize_str("none"),
            Self::Path { base, pattern } => {
                #[derive(Serialize)]
                struct Path<'a> {
                    base: &'a str,
                    pattern: &'a str,
                }
                Path { base, pattern }.serialize(serializer)
            }
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Limits {
    pub input_bytes: usize,
    pub asset_bytes: usize,
    pub candidate_count: usize,
    pub ast_depth: usize,
    pub css_bytes: usize,
    pub queued_bytes: usize,
    pub cache_bytes: usize,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum CompileError {
    Syntax { offset: usize, message: Text },
    RangeError { message: Text },
    Limit { resource: String, limit: usize },
    InvalidLimit { resource: String },
    UnsupportedFeature { feature: String },
    PendingFeature { feature: String },
    PendingPrepared,
    StalePrepared,
    RevisionExhausted,
    DuplicateAsset { id: String },
    MissingAsset { id: String, base: String },
}

impl<'de> Deserialize<'de> for CompileError {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        #[derive(Deserialize)]
        #[serde(rename_all = "snake_case")]
        enum Kind {
            Syntax,
            RangeError,
            Limit,
            InvalidLimit,
            UnsupportedFeature,
            PendingFeature,
            PendingPrepared,
            StalePrepared,
            RevisionExhausted,
            DuplicateAsset,
            MissingAsset,
        }
        // Internally tagged enum decoding buffers strings before Text can read
        // their UTF-16 units. Decode the fields directly to retain lone surrogates.
        #[derive(Deserialize)]
        struct Fields {
            kind: Kind,
            offset: Option<usize>,
            message: Option<Text>,
            resource: Option<String>,
            limit: Option<usize>,
            feature: Option<String>,
            id: Option<String>,
            base: Option<String>,
        }
        let fields = Fields::deserialize(deserializer)?;
        Ok(match fields.kind {
            Kind::Syntax => Self::Syntax {
                offset: fields
                    .offset
                    .ok_or_else(|| serde::de::Error::missing_field("offset"))?,
                message: fields
                    .message
                    .ok_or_else(|| serde::de::Error::missing_field("message"))?,
            },
            Kind::RangeError => Self::RangeError {
                message: fields
                    .message
                    .ok_or_else(|| serde::de::Error::missing_field("message"))?,
            },
            Kind::Limit => Self::Limit {
                resource: fields
                    .resource
                    .ok_or_else(|| serde::de::Error::missing_field("resource"))?,
                limit: fields
                    .limit
                    .ok_or_else(|| serde::de::Error::missing_field("limit"))?,
            },
            Kind::InvalidLimit => Self::InvalidLimit {
                resource: fields
                    .resource
                    .ok_or_else(|| serde::de::Error::missing_field("resource"))?,
            },
            Kind::UnsupportedFeature => Self::UnsupportedFeature {
                feature: fields
                    .feature
                    .ok_or_else(|| serde::de::Error::missing_field("feature"))?,
            },
            Kind::PendingFeature => Self::PendingFeature {
                feature: fields
                    .feature
                    .ok_or_else(|| serde::de::Error::missing_field("feature"))?,
            },
            Kind::PendingPrepared => Self::PendingPrepared,
            Kind::StalePrepared => Self::StalePrepared,
            Kind::RevisionExhausted => Self::RevisionExhausted,
            Kind::DuplicateAsset => Self::DuplicateAsset {
                id: fields
                    .id
                    .ok_or_else(|| serde::de::Error::missing_field("id"))?,
            },
            Kind::MissingAsset => Self::MissingAsset {
                id: fields
                    .id
                    .ok_or_else(|| serde::de::Error::missing_field("id"))?,
                base: fields
                    .base
                    .ok_or_else(|| serde::de::Error::missing_field("base"))?,
            },
        })
    }
}

impl fmt::Display for CompileError {
    fn fmt(&self, output: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn write_message(output: &mut fmt::Formatter<'_>, message: &Text) -> fmt::Result {
            for character in char::decode_utf16(message.units().iter().copied()) {
                match character {
                    Ok(character) => write!(output, "{character}")?,
                    Err(error) => write!(output, "\\u{:04x}", error.unpaired_surrogate())?,
                }
            }
            Ok(())
        }
        match self {
            Self::Syntax { offset, message } => {
                write!(output, "CSS syntax at {offset}: ")?;
                write_message(output, message)
            }
            Self::RangeError { message } => {
                write!(output, "RangeError: ")?;
                write_message(output, message)
            }
            Self::Limit { resource, limit } => write!(output, "{resource} exceeds {limit}"),
            Self::InvalidLimit { resource } => write!(output, "{resource} must be positive"),
            Self::UnsupportedFeature { feature } => {
                write!(output, "unsupported feature: {feature}")
            }
            Self::PendingFeature { feature } => {
                write!(output, "compiler compatibility pending: {feature}")
            }
            Self::PendingPrepared => write!(
                output,
                "a prepared result must be committed or discarded first"
            ),
            Self::StalePrepared => write!(
                output,
                "prepared result belongs to a different session or revision"
            ),
            Self::RevisionExhausted => write!(output, "compiler revision exhausted"),
            Self::DuplicateAsset { id } => write!(output, "duplicate virtual asset: {id}"),
            Self::MissingAsset { id, base } => {
                write!(output, "missing virtual asset: {id} from {base}")
            }
        }
    }
}
impl std::error::Error for CompileError {}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DiagnosticKind {
    InvalidSyntax,
    UnknownUtility,
    InvalidValue,
    PendingFamily,
    PendingVariant,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CandidateDiagnostic {
    pub candidate: Text,
    pub kind: DiagnosticKind,
    pub family: Option<Text>,
}
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompileMetrics {
    pub parsed_candidates: usize,
    pub generated_candidates: usize,
    pub cache_entries: usize,
    pub cache_bytes: usize,
    pub output_bytes: usize,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompileReport {
    pub revision: u64,
    pub changed: bool,
    pub css: crate::text::Text,
    pub admitted: Vec<Text>,
    pub rejected: Vec<CandidateDiagnostic>,
    pub unsupported: Vec<CandidateDiagnostic>,
    pub gpu_programs: Vec<GpuProgram>,
    /// Raw bindings for renderer evaluation; live host values take precedence.
    #[serde(default, with = "crate::text::map")]
    pub gpu_variables: std::collections::BTreeMap<crate::text::Text, crate::text::Text>,
    pub metrics: CompileMetrics,
}
