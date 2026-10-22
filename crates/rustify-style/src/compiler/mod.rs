//! Tailwind candidate compilation without a content scanner or JavaScript runtime.

mod apply;
pub mod ast;
mod backgrounds;
mod borders;
pub mod candidate;
mod colors;
mod custom_utilities;
mod custom_variants;
mod dependencies;
mod effects;
mod filters;
mod functional;
mod functions;
mod gradients;
mod grid;
mod imports;
mod layout;
mod legacy_theme;
mod masks;
mod miscellaneous;
mod motion;
mod origin;
pub mod parser;
mod polyfills;
pub mod program;
pub mod public_api;
mod registration;
#[path = "../../upstream/tailwind/registry.rs"]
mod registry;
mod shadows;
mod sources;
mod static_utilities;
mod structural_spacing;
mod theme;
mod transforms;
mod typography;
pub mod units;
mod utilities;
pub mod value;
mod variants;

use crate::text::Text;
use ast::AstNode;
pub use program::{GpuCondition, GpuDeclaration, GpuProgram, GpuValue};
pub use public_api::{
    compile, compile_text, CompileOptions, CssCompiler, Features, Polyfills, Stylesheet,
    StylesheetLoad,
};
use std::{
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};
use utilities::CompiledCandidate;

/// Fixed upstream default CSS theme, provided as data rather than executed code.
pub const DEFAULT_THEME_CSS: &str = include_str!("../../upstream/tailwind/theme.css");

pub use crate::protocol::{
    CandidateDiagnostic, CompileError, CompileMetrics, CompileReport, CompilerInput,
    DiagnosticKind, Limits, SourceAsset, SourceMetadata, SourcePattern, SourceRoot,
};

#[derive(Clone)]
enum Outcome {
    Accepted(Vec<Arc<CompiledCandidate>>),
    ExternalVariable(Option<crate::text::Text>),
    Rejected(CandidateDiagnostic),
    Pending(CandidateDiagnostic),
}
#[derive(Clone)]
struct CacheEntry {
    outcome: Outcome,
    bytes: usize,
}

/// A session-owned staging token. Only commit publishes its candidate cache.
pub struct Prepared {
    owner: Arc<()>,
    nonce: u64,
    base_revision: u64,
    report: CompileReport,
    ast: Vec<AstNode>,
    cache: BTreeMap<Text, CacheEntry>,
}
impl Prepared {
    pub fn report(&self) -> &CompileReport {
        &self.report
    }
    pub fn ast(&self) -> &[AstNode] {
        &self.ast
    }
}

pub struct Compiler {
    owner: Arc<()>,
    nonce: u64,
    pending: Option<u64>,
    input_ast: Vec<AstNode>,
    apply_replacements: Vec<apply::Replacement>,
    theme: theme::Theme,
    custom: custom_utilities::CustomUtilities,
    limits: Limits,
    inline: Vec<Text>,
    source_metadata: SourceMetadata,
    important: bool,
    features: public_api::Features,
    source_tracking: bool,
    stylesheet_loads: Vec<public_api::StylesheetLoad>,
    polyfills: public_api::Polyfills,
    sources_cache_bytes: usize,
    passthrough_css: Option<crate::text::Text>,
    cache: BTreeMap<Text, CacheEntry>,
    committed: CompileReport,
}

impl Compiler {
    /// Creates a source scanner using CSS metadata and the caller's discovery base.
    #[cfg(feature = "scanner")]
    pub fn scanner(
        &self,
        base: &str,
        tree: crate::scanner::SourceTree,
    ) -> Result<crate::scanner::Scanner, crate::scanner::ScanError> {
        use crate::scanner::{PublicSourceEntry, Scanner};
        let mut sources = match &self.source_metadata.root {
            SourceRoot::None => Vec::new(),
            SourceRoot::Automatic => vec![PublicSourceEntry {
                base: base.into(),
                pattern: "**/*".into(),
                negated: false,
            }],
            SourceRoot::Path { base, pattern } => vec![PublicSourceEntry {
                base: base.clone(),
                pattern: pattern.clone(),
                negated: false,
            }],
        };
        sources.extend(
            self.source_metadata
                .sources
                .iter()
                .map(|source| PublicSourceEntry {
                    base: source.base.clone(),
                    pattern: source.pattern.clone(),
                    negated: source.negated,
                }),
        );
        Scanner::new(sources, tree)
    }
    /// CSS-authored source roots and patterns, independent of candidate revisions.
    pub fn source_metadata(&self) -> &SourceMetadata {
        &self.source_metadata
    }
    /// Parses a variant using this configuration's registered roots and theme.
    pub fn parse_variant_json(&self, raw: &str) -> Result<serde_json::Value, CompileError> {
        check_limit(raw.len(), self.limits.queued_bytes, "queued_bytes")?;
        candidate::parse_variant_json_with_theme(raw, self.limits.ast_depth, &self.theme)
    }

    pub fn new(
        input: CompilerInput,
        assets: Vec<SourceAsset>,
        limits: Limits,
    ) -> Result<Self, CompileError> {
        let options = public_api::CompileOptions {
            base: Some(imports::directory(&input.entry).into()),
            ..public_api::CompileOptions::default()
        };
        Self::new_with_options(
            input,
            assets.into_iter().map(Into::into).collect(),
            limits,
            options,
        )
    }

    /// Creates a compiler with independent discovery and source identity options.
    pub fn new_with_options(
        input: CompilerInput,
        assets: Vec<public_api::Stylesheet>,
        limits: Limits,
        options: public_api::CompileOptions,
    ) -> Result<Self, CompileError> {
        validate_limits(&limits)?;
        check_limit(
            input.css.len() + input.entry.len(),
            limits.input_bytes,
            "input_bytes",
        )?;
        validate_assets(&assets, &limits)?;
        let source = options
            .from
            .as_ref()
            .filter(|from| !from.is_empty())
            .map(|from| {
                Arc::new(public_api::Source {
                    file: Some(from.clone()),
                    code: input.css.clone(),
                })
            });
        let input_ast = parser::parse_with_source(&input.css, limits.ast_depth, source)?;
        Self::new_with_ast(input_ast, &assets, limits, options, Some(input.css), None)
    }

    fn new_with_ast(
        mut input_ast: Vec<AstNode>,
        assets: &[public_api::Stylesheet],
        limits: Limits,
        options: public_api::CompileOptions,
        original_css: Option<crate::text::Text>,
        mut public_setup: Option<&mut dyn imports::PublicSetup>,
    ) -> Result<Self, CompileError> {
        validate_limits(&limits)?;
        ast::validate_depth(&input_ast, limits.ast_depth)?;
        validate_assets(assets, &limits)?;
        let base = options.base.as_deref().unwrap_or("");
        let expanded = imports::expand_with_base(
            &mut input_ast,
            base,
            assets,
            &limits,
            options.from.is_some(),
            public_setup
                .as_mut()
                .map(|setup| &mut **setup as &mut dyn imports::PublicSetup),
        )?;
        let has_import = expanded.has_import;
        let registration::Initial {
            nodes,
            sources,
            important,
            utilities,
            variants: registered_variants,
            mut theme,
            definitions,
            warnings,
            has_authored_variants,
            has_functions,
        } = if let Some(setup) = &mut public_setup {
            setup.initial(&limits)?
        } else {
            public_api::register_native(&input_ast, base, &limits)?
        };
        input_ast = nodes;
        ast::validate_depth(&input_ast, limits.ast_depth)?;
        let source_locations_bytes = ast::source_owned_bytes(&input_ast);
        let sources_cache_bytes = sources
            .owned_bytes()
            .saturating_add(source_locations_bytes)
            .saturating_add(
                expanded
                    .calls
                    .capacity()
                    .saturating_mul(std::mem::size_of::<public_api::StylesheetLoad>()),
            )
            .saturating_add(
                expanded
                    .calls
                    .iter()
                    .map(|call| call.id.capacity().saturating_add(call.base.capacity()))
                    .sum::<usize>(),
            );
        check_limit(sources_cache_bytes, limits.cache_bytes, "cache_bytes")?;
        let mut custom_variants = custom_variants::CustomVariants::default();
        for (definition, nodes) in registered_variants {
            custom_variants.install(definition, nodes);
        }
        custom_variants.finish(&limits)?;
        let mut custom =
            custom_utilities::CustomUtilities::from_definitions(definitions, warnings, &limits)?;
        let has_directive = ast::contains_at_rule(&input_ast, "@tailwind")
            || ast::contains_at_rule(&input_ast, "@apply")
            || custom.contains_apply();
        let features = public_api::Features(
            (u32::from(has_import) * public_api::Features::AT_IMPORT.0)
                | (u32::from(utilities) * public_api::Features::UTILITIES.0)
                | (u32::from(
                    ast::contains_at_rule(&input_ast, "@apply") || custom.contains_apply(),
                ) * public_api::Features::AT_APPLY.0)
                | (u32::from(has_functions) * public_api::Features::THEME_FUNCTION.0)
                | (u32::from(has_authored_variants) * public_api::Features::VARIANTS.0),
        );
        let passthrough_css =
            (!has_import && !has_functions && !has_directive && !has_authored_variants)
                .then_some(original_css)
                .flatten();
        let sources_cache_bytes = sources_cache_bytes.saturating_add(
            passthrough_css
                .as_ref()
                .map_or(0, crate::text::Text::owned_bytes),
        );
        let theme_budget = limits
            .cache_bytes
            .saturating_sub(custom.owned_bytes())
            .saturating_sub(custom_variants.owned_bytes())
            .saturating_sub(sources_cache_bytes);
        theme.custom_variants = custom_variants;
        theme.blocked = sources.blocked;
        theme.legacy_budget = Some(theme_budget.saturating_sub(theme.owned_bytes()));
        validate_directives(&input_ast)?;
        theme.prepare_functions(limits.ast_depth, limits.css_bytes, theme_budget)?;
        let mut apply_replacements = Vec::new();
        if let Err(error) = apply::substitute(
            &mut input_ast,
            &theme,
            &mut custom,
            &limits,
            &mut apply_replacements,
        ) {
            if let Some(setup) = &mut public_setup {
                setup.failed_apply(apply_replacements)?;
            }
            return Err(error);
        }
        let sources_cache_bytes = sources_cache_bytes.saturating_add(
            ast::source_owned_bytes(&input_ast).saturating_sub(source_locations_bytes),
        );
        let empty = render(
            &input_ast,
            &[],
            &theme,
            RenderPhase::Setup,
            Vec::new(),
            limits.ast_depth,
            options.polyfills,
        )?;
        ast::validate_depth(&empty, limits.ast_depth)?;
        let css = passthrough_css
            .as_ref()
            .cloned()
            .unwrap_or_else(|| ast::to_css(&empty));
        let gpu_variables = gpu_variables(&theme, &empty);
        let initial_metrics = CompileMetrics {
            cache_bytes: theme
                .owned_bytes()
                .saturating_add(legacy_theme::cache_usage(&theme).owned_bytes)
                .saturating_add(theme.custom_variants.owned_bytes())
                .saturating_add(custom.owned_bytes())
                .saturating_add(sources_cache_bytes),
            ..CompileMetrics::default()
        };
        check_limit(css.len(), limits.css_bytes, "css_bytes")?;
        check_limit(
            initial_metrics.cache_bytes,
            limits.cache_bytes,
            "cache_bytes",
        )?;
        Ok(Self {
            owner: Arc::new(()),
            nonce: 0,
            pending: None,
            input_ast,
            apply_replacements,
            theme,
            custom,
            limits,
            inline: sources.inline,
            source_metadata: sources.metadata,
            important,
            features,
            source_tracking: options.from.as_ref().is_some_and(|from| !from.is_empty()),
            stylesheet_loads: expanded.calls,
            polyfills: options.polyfills,
            sources_cache_bytes,
            passthrough_css,
            cache: BTreeMap::new(),
            committed: CompileReport {
                revision: 0,
                changed: false,
                css,
                admitted: Vec::new(),
                rejected: Vec::new(),
                unsupported: Vec::new(),
                gpu_programs: Vec::new(),
                gpu_variables,
                metrics: initial_metrics,
            },
        })
    }

    pub fn revision(&self) -> u64 {
        self.committed.revision
    }
    pub fn report(&self) -> &CompileReport {
        &self.committed
    }

    /// Configuration output before any candidate build, retaining every text unit.
    pub fn initial_ast(&self) -> Result<Vec<AstNode>, CompileError> {
        if let Some(css) = &self.passthrough_css {
            return parser::parse_text(css, self.limits.ast_depth);
        }
        render(
            &self.input_ast,
            &[],
            &self.theme,
            RenderPhase::Setup,
            Vec::new(),
            self.limits.ast_depth,
            self.polyfills,
        )
    }

    pub fn prepare_add(&mut self, batch: &[String]) -> Result<Prepared, CompileError> {
        if self.pending.is_some() {
            return Err(CompileError::PendingPrepared);
        }
        check_limit(
            batch
                .iter()
                .map(String::len)
                .sum::<usize>()
                .saturating_add(self.inline.iter().map(Text::byte_len).sum()),
            self.limits.queued_bytes,
            "queued_bytes",
        )?;
        self.prepare_add_text(&batch.iter().map(Text::from).collect::<Vec<_>>())
    }

    /// Whether every candidate and inline source already has a cached outcome,
    /// so preparing them cannot change the committed output.
    pub(crate) fn knows_all(&self, batch: &[Text]) -> bool {
        self.pending.is_none()
            && (self.passthrough_css.is_some()
                || batch
                    .iter()
                    .chain(&self.inline)
                    .all(|raw| self.cache.contains_key(raw)))
    }

    pub fn prepare_add_text(&mut self, batch: &[Text]) -> Result<Prepared, CompileError> {
        if self.pending.is_some() {
            return Err(CompileError::PendingPrepared);
        }
        check_limit(
            batch
                .iter()
                .chain(&self.inline)
                .fold(0usize, |size, raw| size.saturating_add(raw.byte_len())),
            self.limits.queued_bytes,
            "queued_bytes",
        )?;
        let new_legacy_cache = self.theme.legacy.get().is_none();
        if new_legacy_cache {
            self.theme.legacy_budget = Some(
                self.limits
                    .cache_bytes
                    .saturating_sub(self.committed.metrics.cache_bytes),
            );
        }
        let external_variables: BTreeSet<_> =
            batch.iter().filter(|raw| raw.starts_with("--")).collect();
        let batch = batch
            .iter()
            .chain(&self.inline)
            .cloned()
            .collect::<Vec<_>>();
        let result = self.prepare_add_inner(&batch, &external_variables);
        if new_legacy_cache && result.is_err() {
            self.theme.legacy.take();
        }
        result
    }

    fn prepare_add_inner(
        &mut self,
        batch: &[Text],
        external_variables: &BTreeSet<&Text>,
    ) -> Result<Prepared, CompileError> {
        if self.pending.is_some() {
            return Err(CompileError::PendingPrepared);
        }
        let batch_bytes = batch
            .iter()
            .fold(0usize, |size, item| size.saturating_add(item.len()));
        check_limit(batch_bytes, self.limits.queued_bytes, "queued_bytes")?;
        let unique: BTreeSet<_> = batch.iter().collect();
        let unique = if self.passthrough_css.is_some() {
            BTreeSet::new()
        } else {
            unique
        };
        let new_count = unique
            .iter()
            .filter(|item| !self.cache.contains_key(*item))
            .count();
        check_limit(
            self.cache.len().saturating_add(new_count),
            self.limits.candidate_count,
            "candidate_count",
        )?;
        let mut staged_cache_bytes = self
            .cache
            .values()
            .fold(self.sources_cache_bytes, |bytes, entry| {
                bytes.saturating_add(entry.bytes)
            });
        let mut cache = self.cache.clone();
        let mut metrics = CompileMetrics::default();
        let utilities_enabled = ast::contains_at_rule(&self.input_ast, "@tailwind");
        for raw in unique {
            if cache.contains_key(raw) {
                continue;
            }
            metrics.parsed_candidates += 1;
            let outcome = if raw.starts_with("--") {
                let variable = if utilities_enabled
                    && !self.theme.blocked.contains(raw)
                    && external_variables.contains(raw)
                {
                    self.theme.external_variable(raw)?
                } else {
                    None
                };
                Outcome::ExternalVariable(variable)
            } else if self.theme.blocked.contains(raw) {
                Outcome::Rejected(CandidateDiagnostic {
                    candidate: raw.clone(),
                    kind: DiagnosticKind::UnknownUtility,
                    family: None,
                })
            } else {
                match candidate::parse_with_utilities(
                    raw,
                    self.limits.ast_depth,
                    &self.theme,
                    &self.custom,
                ) {
                    Err(CompileError::Syntax { .. }) => Outcome::Rejected(CandidateDiagnostic {
                        candidate: raw.clone(),
                        kind: DiagnosticKind::InvalidSyntax,
                        family: None,
                    }),
                    Err(error) => return Err(error),
                    Ok(parsed) if parsed.is_empty() => Outcome::Rejected(CandidateDiagnostic {
                        candidate: raw.clone(),
                        kind: DiagnosticKind::UnknownUtility,
                        family: None,
                    }),
                    Ok(parsed) => {
                        let mut last = None;
                        let mut compiled = Vec::new();
                        for mut interpretation in parsed {
                            interpretation.important |= self.important;
                            match utilities::compile_all(
                                &interpretation,
                                &self.theme,
                                &self.limits,
                                &self.custom,
                            ) {
                                Ok(values) => {
                                    for value in values {
                                        ast::validate_depth(
                                            std::slice::from_ref(&value.node),
                                            self.limits.ast_depth,
                                        )?;
                                        compiled.push(Arc::new(value));
                                    }
                                }
                                Err(utilities::CompileFailure::Resource(error)) => {
                                    return Err(error)
                                }
                                Err(utilities::CompileFailure::Candidate(diagnostic)) => {
                                    let pending = |kind: &DiagnosticKind| {
                                        matches!(
                                            kind,
                                            DiagnosticKind::PendingFamily
                                                | DiagnosticKind::PendingVariant
                                        )
                                    };
                                    if pending(&diagnostic.kind)
                                        || last.as_ref().is_none_or(
                                            |previous: &CandidateDiagnostic| {
                                                !pending(&previous.kind)
                                            },
                                        )
                                    {
                                        last = Some(diagnostic);
                                    }
                                }
                            }
                        }
                        if !compiled.is_empty() {
                            metrics.generated_candidates += 1;
                            Outcome::Accepted(compiled)
                        } else {
                            let diagnostic = last.unwrap_or(CandidateDiagnostic {
                                candidate: raw.clone(),
                                kind: DiagnosticKind::InvalidValue,
                                family: None,
                            });
                            if matches!(
                                diagnostic.kind,
                                DiagnosticKind::PendingFamily | DiagnosticKind::PendingVariant
                            ) {
                                Outcome::Pending(diagnostic)
                            } else {
                                Outcome::Rejected(diagnostic)
                            }
                        }
                    }
                }
            };
            let bytes = raw
                .owned_bytes()
                .saturating_add(std::mem::size_of::<Text>())
                .saturating_add(match &outcome {
                    Outcome::ExternalVariable(variable) => std::mem::size_of::<Option<Text>>()
                        .saturating_add(
                            variable.as_ref().map_or(0, crate::text::Text::owned_bytes),
                        ),
                    Outcome::Accepted(compiled) => compiled.iter().fold(
                        std::mem::size_of::<Vec<Arc<CompiledCandidate>>>(),
                        |bytes, compiled| {
                            bytes.saturating_add(
                                ast::to_css(std::slice::from_ref(&compiled.node))
                                    .len()
                                    .saturating_add(ast::to_css(&compiled.globals).len())
                                    .saturating_mul(2)
                                    .saturating_add(std::mem::size_of::<CompiledCandidate>())
                                    .saturating_add(ast::declaration_identity_bytes(
                                        std::slice::from_ref(&compiled.node),
                                    ))
                                    .saturating_add(ast::declaration_identity_bytes(
                                        &compiled.globals,
                                    ))
                                    .saturating_add(ast::source_owned_bytes_many([
                                        std::slice::from_ref(&compiled.node),
                                        compiled.globals.as_slice(),
                                    ])),
                            )
                        },
                    ),
                    Outcome::Rejected(value) | Outcome::Pending(value) => value
                        .candidate
                        .owned_bytes()
                        .saturating_add(value.family.as_ref().map_or(0, Text::owned_bytes))
                        .saturating_add(std::mem::size_of::<CandidateDiagnostic>()),
                });
            staged_cache_bytes = staged_cache_bytes.saturating_add(bytes);
            check_limit(staged_cache_bytes, self.limits.cache_bytes, "cache_bytes")?;
            cache.insert(raw.clone(), CacheEntry { outcome, bytes });
        }
        metrics.cache_entries = cache.len();
        metrics.cache_bytes = cache.values().fold(
            self.theme
                .owned_bytes()
                .saturating_add(legacy_theme::cache_usage(&self.theme).owned_bytes)
                .saturating_add(self.theme.custom_variants.owned_bytes())
                .saturating_add(self.custom.owned_bytes())
                .saturating_add(self.sources_cache_bytes),
            |bytes, entry| bytes.saturating_add(entry.bytes),
        );
        check_limit(metrics.cache_bytes, self.limits.cache_bytes, "cache_bytes")?;
        let mut accepted = Vec::new();
        let mut rejected = Vec::new();
        let mut unsupported = Vec::new();
        let mut used = BTreeSet::new();
        for entry in cache.values() {
            match &entry.outcome {
                Outcome::Accepted(compiled) => accepted.extend(compiled.iter().cloned()),
                Outcome::ExternalVariable(variable) => used.extend(variable.iter().cloned()),
                Outcome::Rejected(value) => rejected.push(value.clone()),
                Outcome::Pending(value) => unsupported.push(value.clone()),
            }
        }
        accepted.sort_by(|left, right| compare_rules(left, right));
        let mut nodes = Vec::new();
        let mut globals = Vec::new();
        let mut gpu_programs = Vec::new();
        let mut admitted = Vec::new();
        let mut seen_candidates = BTreeSet::new();
        for (order, compiled) in accepted.into_iter().enumerate() {
            nodes.push(compiled.node.clone());
            globals.extend(compiled.globals.iter().cloned());
            used.extend(compiled.used_variables.iter().cloned());
            if seen_candidates.insert(compiled.program.candidate.clone()) {
                admitted.push(compiled.program.candidate.clone());
            }
            let mut program = compiled.program.clone();
            program.cascade_order = order;
            gpu_programs.push(program);
        }
        if self.source_tracking {
            if let Some(source) = ast::find_at_rule_source(&self.input_ast, "@tailwind") {
                ast::fill_sources(&mut nodes, Some(source));
                ast::fill_sources(&mut globals, Some(source));
            }
        }
        let output_ast = if self.features == public_api::Features::NONE {
            self.input_ast.clone()
        } else {
            render(
                &self.input_ast,
                &nodes,
                &self.theme,
                RenderPhase::Build(&used),
                globals,
                self.limits.ast_depth,
                self.polyfills,
            )?
        };
        ast::validate_depth(&output_ast, self.limits.ast_depth)?;
        let css = self
            .passthrough_css
            .as_ref()
            .cloned()
            .unwrap_or_else(|| ast::to_css(&output_ast));
        let gpu_variables = gpu_variables(&self.theme, &output_ast);
        check_limit(css.len(), self.limits.css_bytes, "css_bytes")?;
        metrics.output_bytes = css.len();
        let changed = css != self.committed.css;
        let revision = if changed {
            self.committed
                .revision
                .checked_add(1)
                .ok_or(CompileError::RevisionExhausted)?
        } else {
            self.committed.revision
        };
        let nonce = self
            .nonce
            .checked_add(1)
            .ok_or(CompileError::RevisionExhausted)?;
        let report = CompileReport {
            revision,
            changed,
            css,
            admitted,
            rejected,
            unsupported,
            gpu_programs,
            gpu_variables,
            metrics,
        };
        self.nonce = nonce;
        self.pending = Some(nonce);
        Ok(Prepared {
            owner: self.owner.clone(),
            nonce,
            base_revision: self.revision(),
            report,
            ast: output_ast,
            cache,
        })
    }

    fn validate_prepared(&self, prepared: &Prepared) -> Result<(), CompileError> {
        if !Arc::ptr_eq(&self.owner, &prepared.owner)
            || self.pending != Some(prepared.nonce)
            || self.revision() != prepared.base_revision
        {
            return Err(CompileError::StalePrepared);
        }
        Ok(())
    }

    pub fn commit(&mut self, prepared: Prepared) -> Result<CompileReport, CompileError> {
        self.validate_prepared(&prepared)?;
        self.pending = None;
        self.cache = prepared.cache;
        self.committed = prepared.report;
        Ok(self.committed.clone())
    }

    pub fn discard(&mut self, prepared: Prepared) -> Result<(), CompileError> {
        self.validate_prepared(&prepared)?;
        self.pending = None;
        Ok(())
    }

    pub fn parse_variant_object_text(
        &self,
        raw: &Text,
    ) -> Result<Option<candidate::ParsedVariant>, CompileError> {
        check_limit(raw.byte_len(), self.limits.queued_bytes, "queued_bytes")?;
        candidate::parse_variant_object(raw, self.limits.ast_depth, &self.theme)
    }

    pub fn rebuild_text(
        input: CompilerInput,
        assets: Vec<SourceAsset>,
        limits: Limits,
        active: &[Text],
    ) -> Result<(Self, Prepared), CompileError> {
        let mut replacement = Self::new(input, assets, limits)?;
        let prepared = replacement.prepare_add_text(active)?;
        Ok((replacement, prepared))
    }

    /// Builds a replacement session; the caller swaps it only after installation succeeds.
    pub fn rebuild(
        input: CompilerInput,
        assets: Vec<SourceAsset>,
        limits: Limits,
        active: &[String],
    ) -> Result<(Self, Prepared), CompileError> {
        let mut replacement = Self::new(input, assets, limits)?;
        let prepared = replacement.prepare_add(active)?;
        Ok((replacement, prepared))
    }
}

fn validate_assets(assets: &[public_api::Stylesheet], limits: &Limits) -> Result<(), CompileError> {
    let mut asset_size = 0usize;
    let mut ids = BTreeSet::new();
    for asset in assets {
        asset_size = asset_size
            .saturating_add(asset.id.len())
            .saturating_add(asset.content.len());
        if !ids.insert(imports::normalize(&asset.id)) {
            return Err(CompileError::DuplicateAsset {
                id: asset.id.clone(),
            });
        }
    }
    check_limit(asset_size, limits.asset_bytes, "asset_bytes")?;
    Ok(())
}

pub(super) fn validate_limits(limits: &Limits) -> Result<(), CompileError> {
    for (resource, amount) in [
        ("input_bytes", limits.input_bytes),
        ("asset_bytes", limits.asset_bytes),
        ("candidate_count", limits.candidate_count),
        ("ast_depth", limits.ast_depth),
        ("css_bytes", limits.css_bytes),
        ("queued_bytes", limits.queued_bytes),
        ("cache_bytes", limits.cache_bytes),
    ] {
        if amount == 0 {
            return Err(CompileError::InvalidLimit {
                resource: resource.into(),
            });
        }
    }
    Ok(())
}

fn check_limit(amount: usize, limit: usize, resource: &str) -> Result<(), CompileError> {
    if amount > limit {
        Err(CompileError::Limit {
            resource: resource.into(),
            limit,
        })
    } else {
        Ok(())
    }
}

fn gpu_variables(
    theme: &theme::Theme,
    nodes: &[AstNode],
) -> BTreeMap<crate::text::Text, crate::text::Text> {
    fn collect(nodes: &[AstNode], variables: &mut BTreeMap<crate::text::Text, crate::text::Text>) {
        for node in nodes {
            if let AstNode::AtRule {
                name,
                params,
                nodes: Some(children),
                ..
            } = node
            {
                if name == "@property" {
                    for child in children {
                        if let AstNode::Declaration {
                            property, value, ..
                        } = child
                        {
                            if property == "initial-value" {
                                variables
                                    .entry(params.clone())
                                    .or_insert_with(|| value.clone());
                            }
                        }
                    }
                }
            }
            if let Some(children) = node.children() {
                collect(children, variables);
            }
        }
    }
    let mut variables: BTreeMap<crate::text::Text, crate::text::Text> = theme
        .tokens
        .iter()
        .map(|(name, token)| {
            (
                candidate::escape_text(&theme.prefix_key_text(name)),
                token.output_value().clone(),
            )
        })
        .collect();
    collect(nodes, &mut variables);
    variables
}

fn validate_directives(nodes: &[AstNode]) -> Result<(), CompileError> {
    for node in nodes {
        if let AstNode::AtRule { name, params, .. } = node {
            if name == "@tailwind"
                && !["utilities", "utilities source(none)", "utilities important"]
                    .iter()
                    .any(|label| params == label)
            {
                return Err(CompileError::PendingFeature {
                    feature: format!(
                        "{} {}",
                        value::scalar_text(name)?,
                        value::scalar_text(params)?
                    ),
                });
            }
        }
        if let Some(children) = node.children() {
            validate_directives(children)?;
        }
    }
    Ok(())
}

enum RenderPhase<'a> {
    Setup,
    Build(&'a BTreeSet<crate::text::Text>),
}

fn render(
    input: &[AstNode],
    utilities: &[AstNode],
    theme: &theme::Theme,
    phase: RenderPhase<'_>,
    mut globals: Vec<AstNode>,
    depth_limit: usize,
    polyfills: public_api::Polyfills,
) -> Result<Vec<AstNode>, CompileError> {
    fn substitute(input: &[AstNode], utilities: &[AstNode], tokens: &[AstNode]) -> Vec<AstNode> {
        let mut output = Vec::new();
        for node in input {
            if matches!(node, AstNode::Declaration { property, .. } if property == "--tw-sort") {
                continue;
            }
            if let AstNode::Context {
                reference, nodes, ..
            } = node
            {
                if !reference {
                    output.extend(substitute(nodes, utilities, tokens));
                }
                continue;
            }
            if matches!(node, AstNode::AtRoot { .. }) {
                output.push(node.clone());
                continue;
            }
            if matches!(node, AstNode::AtRule { name, .. } if name == "@rustify-theme-anchor") {
                if !tokens.is_empty() {
                    output.push(AstNode::Rule {
                        origin: None,
                        source: node.source().cloned(),
                        destination: node.destination().cloned(),
                        selector: ":root, :host".into(),
                        nodes: tokens.to_vec(),
                    });
                }
                continue;
            }
            if matches!(node, AstNode::AtRule { name, .. } if name == "@tailwind") {
                output.extend(substitute(utilities, &[], tokens));
                continue;
            }
            if matches!(node, AstNode::AtRule { name, nodes: None, .. } if !["@layer", "@charset", "@custom-media", "@namespace", "@import"].iter().any(|label| name == label))
            {
                continue;
            }
            let mut node = node.clone();
            let preserve_empty = matches!(&node, AstNode::AtRule { name, .. } if ["@layer", "@charset", "@custom-media", "@namespace", "@import"].iter().any(|label| name == label));
            if let Some(children) = node.children_mut() {
                *children = substitute(children, utilities, tokens);
                if children.is_empty() && !preserve_empty {
                    continue;
                }
            }
            if matches!(&node, AstNode::Rule { selector, .. } if selector == "&") {
                output.extend(std::mem::take(node.children_mut().unwrap()));
            } else {
                output.push(node);
            }
        }
        output
    }
    fn prune_theme(nodes: &mut Vec<AstNode>, unused: &BTreeSet<usize>, style_parent: bool) -> bool {
        let mut removed_declaration = false;
        let mut removed_ancestor = false;
        let mut output = Vec::new();
        for mut node in std::mem::take(nodes) {
            if matches!(&node, AstNode::Declaration { identity, .. } if unused.contains(&(Arc::as_ptr(identity) as usize)))
            {
                removed_declaration = true;
                continue;
            }
            let style = matches!(&node, AstNode::Rule { .. });
            if let Some(children) = node.children_mut() {
                if prune_theme(children, unused, style) && !style_parent {
                    removed_ancestor = true;
                    continue;
                }
            }
            output.push(node);
        }
        // The upstream removal walk stops at a style-rule parent, retaining
        // its empty child. At-rule ancestors are removed even with siblings.
        let removed = removed_ancestor || removed_declaration && output.is_empty();
        *nodes = output;
        removed
    }
    fn track(
        nodes: &[AstNode],
        used: &mut BTreeSet<crate::text::Text>,
        animations: &mut BTreeSet<crate::text::Text>,
        depth: usize,
        theme_tokens: bool,
        animation_prefix: &str,
    ) -> Result<(), CompileError> {
        for node in nodes {
            // Published theme values participate in setup substitutions; usage
            // tracking visits only the selected render tokens below.
            if matches!(node, AstNode::AtRule { name, .. } if name == "@rustify-theme-anchor") {
                continue;
            }
            if matches!(
                node,
                AstNode::Context {
                    reference: true,
                    ..
                }
            ) {
                continue;
            }
            if let AstNode::Declaration {
                property, value, ..
            } = node
            {
                if property == "--tw-sort" {
                    continue;
                }
                if value.contains("var(") {
                    used.extend(theme::used_variables(&value::parse_text(value, depth)?));
                }
                if property == "animation" || theme_tokens && property.starts_with(animation_prefix)
                {
                    animations.extend(
                        value
                            .units()
                            .split(|unit| {
                                *unit == 44
                                    || char::from_u32(u32::from(*unit))
                                        .is_some_and(value::js_whitespace)
                            })
                            .map(|units| crate::text::Text::from_units(units.to_vec())),
                    );
                }
            }
            if let Some(children) = node.children() {
                track(
                    children,
                    used,
                    animations,
                    depth,
                    theme_tokens,
                    animation_prefix,
                )?;
            }
        }
        Ok(())
    }
    let (mut used, track_usage) = match phase {
        RenderPhase::Setup => (BTreeSet::new(), false),
        RenderPhase::Build(used) => (used.clone(), true),
    };
    let mut animations = BTreeSet::new();
    let animation_prefix = theme.prefix_key("--animate-");
    // Upstream tracks ordinary CSS variable usage during optimizeAst/build,
    // after setup has extracted theme keys and substituted functions/@apply.
    if track_usage {
        track(
            input,
            &mut used,
            &mut animations,
            depth_limit,
            false,
            &animation_prefix,
        )?;
        track(
            utilities,
            &mut used,
            &mut animations,
            depth_limit,
            false,
            &animation_prefix,
        )?;
        track(
            &theme.keyframes,
            &mut used,
            &mut animations,
            depth_limit,
            false,
            &animation_prefix,
        )?;
    }
    let mut tokens = theme.declarations(&used)?;
    tokens.retain(
        |node| !matches!(node, AstNode::Declaration { property, .. } if property == "--tw-sort"),
    );
    if track_usage {
        track(
            &tokens,
            &mut used,
            &mut animations,
            depth_limit,
            true,
            &animation_prefix,
        )?;
    }
    // Unused theme declarations only affect the optimizer's nonempty-rule
    // decision. If every declaration is unused, one real declaration preserves
    // that decision; prune it afterwards instead of rebuilding the entire tree.
    let mut unused = BTreeSet::new();
    if tokens.is_empty() {
        if let Some((key, token)) = theme.ordered_tokens().find(|(key, token)| {
            !token.reference
                && token.output_value() != "initial"
                && (theme.prefix.is_some() || *key != "--tw-sort")
        }) {
            let mut node = AstNode::declaration(
                candidate::escape_text(&theme.prefix_key_text(key)),
                token.output_value().clone(),
            );
            node.set_source(token.source.clone());
            if let AstNode::Declaration { identity, .. } = &node {
                unused.insert(Arc::as_ptr(identity) as usize);
            }
            tokens.push(node);
        }
    }
    let mut output = substitute(input, utilities, &tokens);
    ast::hoist_at_roots(&mut output, &mut globals);
    for keyframe in &theme.keyframes {
        if matches!(keyframe, AstNode::AtRule { params, .. } if animations.iter().any(|animation| params == animation))
        {
            globals.extend(substitute(
                std::slice::from_ref(keyframe),
                utilities,
                &tokens,
            ));
        }
    }
    ast::optimize_rules(&mut output);
    ast::optimize_rules(&mut globals);
    prune_theme(&mut output, &unused, false);
    prune_theme(&mut globals, &unused, false);
    if polyfills.contains(public_api::Polyfills::COLOR_MIX) {
        polyfills::color_mix(&mut output, theme, depth_limit)?;
        polyfills::color_mix(&mut globals, theme, depth_limit)?;
    }
    Ok(dependencies::hoist_with_polyfills(
        output, globals, polyfills,
    ))
}

fn compare_rules(left: &CompiledCandidate, right: &CompiledCandidate) -> Ordering {
    let variant = left
        .variant_order
        .iter()
        .rev()
        .cmp(right.variant_order.iter().rev());
    if variant != Ordering::Equal {
        return variant;
    }
    for (left, right) in left.property_order.iter().zip(&right.property_order) {
        let order = left.cmp(right);
        if order != Ordering::Equal {
            return order;
        }
    }
    right
        .property_order
        .len()
        .cmp(&left.property_order.len())
        .then_with(|| right.declaration_count.cmp(&left.declaration_count))
        .then_with(|| compare_classes(&left.program.candidate, &right.program.candidate))
}

// Tailwind compares digit runs numerically, then uses UTF-16 string order.
pub(super) fn compare_classes(left: &Text, right: &Text) -> Ordering {
    fn run(input: &[u16], index: usize) -> &[u16] {
        &input[index
            ..index
                + input[index..]
                    .iter()
                    .take_while(|v| (48..=57).contains(*v))
                    .count()]
    }
    let left = left.units();
    let right = right.units();
    let digit = |value: &u16| (48..=57).contains(value);
    for index in 0..left.len().min(right.len()) {
        if digit(&left[index]) && digit(&right[index]) {
            let a = run(left, index);
            let b = run(right, index);
            let number = |input: &[u16]| {
                // The run is proven ASCII decimal digits. Parse as Number with one rounding.
                input
                    .iter()
                    .map(|unit| char::from(*unit as u8))
                    .collect::<String>()
                    .parse::<f64>()
                    .unwrap_or(f64::INFINITY)
            };
            let diff = number(a) - number(b);
            let order = if diff < 0.0 {
                Ordering::Less
            } else if diff > 0.0 {
                Ordering::Greater
            } else {
                a.cmp(b)
            };
            if order != Ordering::Equal {
                return order;
            }
        } else if left[index] != right[index] {
            return left[index].cmp(&right[index]);
        }
    }
    left.len().cmp(&right.len())
}

#[cfg(test)]
mod tests;

#[cfg(all(rustify_repo_tests, test))]
#[path = "../../../../tests/utility/compiler/utf16-value-native.rs"]
mod utf16_value_tests;

#[cfg(all(rustify_repo_tests, test))]
#[path = "../../../../tests/utility/compiler/utf16-declaration-native.rs"]
mod utf16_declaration_tests;

#[cfg(all(rustify_repo_tests, test))]
#[path = "../../../../tests/utility/compiler/utf16-theme-native.rs"]
mod utf16_theme_tests;

#[cfg(all(rustify_repo_tests, test))]
#[path = "../../../../tests/utility/compiler/utf16-ast-fields-native.rs"]
mod utf16_ast_fields_tests;

#[cfg(all(rustify_repo_tests, test))]
#[path = "../../../../tests/utility/compiler/utf16-parser-native.rs"]
mod utf16_parser_tests;

#[cfg(all(rustify_repo_tests, test))]
#[path = "../../../../tests/utility/compiler/utf16-error-native.rs"]
mod utf16_error_tests;

#[cfg(all(rustify_repo_tests, test))]
#[path = "../../../../tests/utility/compiler/utf16-family-native.rs"]
mod utf16_family_tests;

#[cfg(all(rustify_repo_tests, all(test, feature = "scanner")))]
#[path = "../../../../tests/utility/compiler/source-pipeline.rs"]
mod source_pipeline_tests;
