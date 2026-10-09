//! Fixed Tailwind public compiler options, independent of the runtime session protocol.

use super::{CompileError, Compiler, Limits, SourceAsset, SourceMetadata};
use crate::text::Text;
use serde::{Deserialize, Serialize};

const BANNER: &str = "/*! tailwindcss v4.1.13 | MIT License | https://tailwindcss.com */\n";

pub(super) mod design_system;
pub use design_system::{
    load_design_system, load_design_system_text, load_design_system_with_loader, CacheCheckpoint,
    DesignSystem, PublicObject, PublicValue, ThemeEntry,
};
mod compile_ast;
pub use compile_ast::compile_ast as compile_ast_text;
pub(super) use compile_ast::{
    caller_source, invalid_theme_child, register_native, undefined, variant_body_current,
};
pub use compile_ast::{
    compile_ast, compile_ast_setup, prepare_ast_setup, prepare_ast_setup_async, AstCompiler,
    AstSetupFailure, AstSetupFault, AstSetupJob, AstSetupOutcome, InputMutation, PreparedAstSetup,
    PublicAst,
};
mod source_map;
pub(in crate::compiler) use source_map::destinations;
pub use source_map::{
    create_source_map, create_translation_map, DecodedMapping, DecodedSource, DecodedSourceMap,
    LineTable, OriginalPosition, Position, Source, SourceLocation, Translation,
};

/// Features used while interpreting the input stylesheet.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Features(pub u32);

impl Features {
    pub const NONE: Self = Self(0);
    pub const AT_APPLY: Self = Self(1);
    pub const AT_IMPORT: Self = Self(2);
    pub const JS_PLUGIN_COMPAT: Self = Self(4);
    pub const THEME_FUNCTION: Self = Self(8);
    pub const UTILITIES: Self = Self(16);
    pub const VARIANTS: Self = Self(32);
    pub fn contains(self, flag: Self) -> bool {
        self.0 & flag.0 == flag.0
    }
}

/// CSS compatibility fallbacks enabled during AST optimization.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Polyfills(pub u32);

impl Polyfills {
    pub const NONE: Self = Self(0);
    pub const AT_PROPERTY: Self = Self(1);
    pub const COLOR_MIX: Self = Self(2);
    pub const ALL: Self = Self(3);
    pub fn contains(self, flag: Self) -> bool {
        self.0 & flag.0 == flag.0
    }
}

impl Default for Polyfills {
    fn default() -> Self {
        Self::ALL
    }
}

/// `base` controls discovery and resource loading; `from` identifies the source.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CompileOptions {
    #[serde(default)]
    pub base: Option<String>,
    #[serde(default)]
    pub from: Option<String>,
    #[serde(default)]
    pub polyfills: Polyfills,
}

/// A virtual loader response. Its resolution id, source path, and discovery base are distinct.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Stylesheet {
    pub id: String,
    #[serde(default)]
    pub path: Option<String>,
    #[serde(default)]
    pub base: Option<String>,
    pub content: Text,
}

impl From<SourceAsset> for Stylesheet {
    fn from(asset: SourceAsset) -> Self {
        Self {
            id: asset.id,
            content: asset.content,
            path: None,
            base: None,
        }
    }
}

/// Arguments passed to the virtual stylesheet loader, in resolution order.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct StylesheetLoad {
    pub id: String,
    pub base: String,
}

/// The string compiler accumulates candidates across `build` calls.
pub struct CssCompiler {
    compiler: Compiler,
    mapped_ast: Vec<super::ast::AstNode>,
    printed: bool,
    track: bool,
    retained_ast_bytes: usize,
    license_banner: bool,
    /// A build has committed; until then the committed CSS is not a result.
    built: bool,
}

pub fn compile(
    css: &str,
    options: CompileOptions,
    assets: Vec<Stylesheet>,
    limits: Limits,
) -> Result<CssCompiler, CompileError> {
    compile_text(&css.into(), options, assets, limits)
}

/// `loader` reports that the caller owns a stylesheet loader: missing imports
/// then surface as [`CompileError::MissingAsset`] for the caller to resolve.
pub fn compile_text_with_loader(
    css: &Text,
    options: CompileOptions,
    assets: Vec<Stylesheet>,
    limits: Limits,
    loader: bool,
) -> Result<CssCompiler, CompileError> {
    compile_text_impl(css, options, assets, limits, true, loader)
}

pub fn compile_text(
    css: &Text,
    options: CompileOptions,
    assets: Vec<Stylesheet>,
    limits: Limits,
) -> Result<CssCompiler, CompileError> {
    compile_text_impl(css, options, assets, limits, true, false)
}

pub(super) fn compile_text_impl(
    css: &Text,
    options: CompileOptions,
    assets: Vec<Stylesheet>,
    limits: Limits,
    track_source_maps: bool,
    loader: bool,
) -> Result<CssCompiler, CompileError> {
    let has_stylesheet_loader = loader || !assets.is_empty();
    super::validate_limits(&limits)?;
    super::check_limit(
        css.len()
            .saturating_add(options.from.as_ref().map_or(0, String::len)),
        limits.input_bytes,
        "input_bytes",
    )?;
    super::validate_assets(&assets, &limits)?;
    let track = track_source_maps && options.from.is_some();
    let source = options
        .from
        .as_ref()
        .filter(|from| track_source_maps && !from.is_empty())
        .map(|from| {
            std::sync::Arc::new(Source {
                file: Some(from.clone()),
                code: css.clone(),
            })
        });
    let input_ast = super::parser::parse_with_source(css, limits.ast_depth, source)?;
    let mut compiler =
        Compiler::new_with_ast(input_ast, &assets, limits, options, Some(css.clone()), None)
            .map_err(|error| {
                if !has_stylesheet_loader && matches!(error, CompileError::MissingAsset { .. }) {
                    CompileError::Syntax {
                        offset: 0,
                        message: "No `loadStylesheet` function provided to `compile`".into(),
                    }
                } else {
                    error
                }
            })?;
    let retained_ast_bytes = if track {
        source_map::retained_ast_bytes(&compiler.input_ast)
    } else {
        0
    };
    super::check_limit(
        compiler
            .committed
            .metrics
            .cache_bytes
            .saturating_add(retained_ast_bytes),
        compiler.limits.cache_bytes,
        "cache_bytes",
    )?;
    let mapped_ast = if track {
        compiler.input_ast.clone()
    } else {
        Vec::new()
    };
    compiler.sources_cache_bytes = compiler
        .sources_cache_bytes
        .saturating_add(retained_ast_bytes);
    compiler.committed.metrics.cache_bytes = compiler
        .committed
        .metrics
        .cache_bytes
        .saturating_add(retained_ast_bytes);
    Ok(CssCompiler {
        compiler,
        mapped_ast,
        printed: false,
        track,
        retained_ast_bytes,
        license_banner: true,
        built: false,
    })
}

impl CssCompiler {
    /// Upstream omits the license comment in its own test environment.
    pub fn set_license_banner(&mut self, emit: bool) {
        self.license_banner = emit;
    }

    /// Evaluates CSS functions while retaining the caller's UTF-16 text units.
    pub fn substitute_functions_in_value(
        &self,
        value: &crate::text::Text,
        at_rule: bool,
    ) -> Result<crate::text::Text, CompileError> {
        self.compiler.substitute_functions_in_value(value, at_rule)
    }

    /// Decoded map of the last successful output, initially the initialized input AST.
    pub fn build_source_map(&self) -> Result<DecodedSourceMap, CompileError> {
        let generated = (self.printed && self.compiler.source_tracking).then(|| {
            (
                self.compiler
                    .report()
                    .css
                    .units()
                    .len()
                    .saturating_add(BANNER.encode_utf16().count()),
                self.compiler
                    .report()
                    .css
                    .units()
                    .iter()
                    .filter(|unit| **unit == 10)
                    .count()
                    .saturating_add(2),
            )
        });
        let structure = source_map::structure_budget(&self.mapped_ast, generated.is_some());
        super::check_limit(
            self.compiler
                .committed
                .metrics
                .cache_bytes
                .saturating_add(structure),
            self.compiler.limits.cache_bytes,
            "cache_bytes",
        )?;
        let extra = source_map::allocation_budget(&self.mapped_ast, generated);
        super::check_limit(
            self.compiler
                .committed
                .metrics
                .cache_bytes
                .saturating_add(extra),
            self.compiler.limits.cache_bytes,
            "cache_bytes",
        )?;
        Ok(if self.printed && self.compiler.source_tracking {
            source_map::printed_map(&self.mapped_ast)
        } else {
            source_map::initial_map(&self.mapped_ast)
        })
    }
    pub fn stylesheet_loads(&self) -> &[StylesheetLoad] {
        self.compiler.stylesheet_loads()
    }
    pub fn features(&self) -> Features {
        self.compiler.features()
    }
    pub fn sources(&self) -> &SourceMetadata {
        self.compiler.source_metadata()
    }
    pub fn warnings(&self) -> &[Text] {
        self.compiler.warnings()
    }
    pub fn build(&mut self, candidates: &[String]) -> Result<crate::text::Text, CompileError> {
        let candidates = candidates.iter().map(Text::from).collect::<Vec<_>>();
        self.build_text(&candidates)
    }
    pub fn build_text(&mut self, candidates: &[Text]) -> Result<Text, CompileError> {
        let candidates = if self.features().contains(Features::UTILITIES) {
            candidates
        } else {
            &[]
        };
        let banner = if self.features() == Features::NONE || !self.license_banner {
            ""
        } else {
            BANNER
        };
        // Upstream returns its last CSS when every candidate already has an
        // outcome: nothing new can change the sorted output.
        if self.built && self.compiler.knows_all(candidates) {
            let mut output = crate::text::Text::from(banner);
            output.push_text(&self.compiler.committed.css);
            return Ok(output);
        }
        let mut prepared = self.compiler.prepare_add_text(candidates)?;
        if let Err(error) = super::check_limit(
            banner
                .len()
                .saturating_add(prepared.report().css.byte_len()),
            self.compiler.limits.css_bytes,
            "css_bytes",
        ) {
            self.compiler.discard(prepared)?;
            return Err(error);
        }
        let retained_ast_bytes = if !self.track || self.features() == Features::NONE {
            self.retained_ast_bytes
        } else {
            source_map::retained_ast_bytes(&prepared.ast)
                .saturating_add(std::mem::size_of::<super::ast::AstNode>())
                .saturating_add(BANNER.len())
        };
        let cache_bytes = prepared
            .report
            .metrics
            .cache_bytes
            .saturating_sub(self.retained_ast_bytes)
            .saturating_add(retained_ast_bytes);
        if let Err(error) =
            super::check_limit(cache_bytes, self.compiler.limits.cache_bytes, "cache_bytes")
        {
            self.compiler.discard(prepared)?;
            return Err(error);
        }
        let mapped_ast = if !self.track || self.features() == Features::NONE {
            None
        } else {
            let mut nodes = Vec::with_capacity(prepared.ast.len().saturating_add(1));
            if self.license_banner {
                nodes.push(super::ast::AstNode::Comment {
                    origin: None,
                    source: None,
                    destination: None,
                    value: "! tailwindcss v4.1.13 | MIT License | https://tailwindcss.com ".into(),
                });
            }
            nodes.extend(prepared.ast.iter().cloned());
            Some(nodes)
        };
        prepared.report.metrics.cache_bytes = cache_bytes;
        let css = self.compiler.commit(prepared)?.css;
        self.built = true;
        self.compiler.sources_cache_bytes = self
            .compiler
            .sources_cache_bytes
            .saturating_sub(self.retained_ast_bytes)
            .saturating_add(retained_ast_bytes);
        self.retained_ast_bytes = retained_ast_bytes;
        if let Some(mapped_ast) = mapped_ast {
            self.mapped_ast = mapped_ast;
            self.printed = true;
        }
        if banner.is_empty() {
            Ok(css)
        } else {
            let mut output = crate::text::Text::from(BANNER);
            output.push_text(&css);
            Ok(output)
        }
    }
}

impl Compiler {
    /// Features detected during stylesheet initialization.
    pub fn features(&self) -> Features {
        self.features
    }

    /// CSS-native custom utility warnings, in upstream reporting order.
    pub fn warnings(&self) -> &[Text] {
        self.custom.warnings()
    }

    /// Virtual loader calls performed during stylesheet initialization.
    pub fn stylesheet_loads(&self) -> &[StylesheetLoad] {
        &self.stylesheet_loads
    }

    /// Evaluates CSS functions while retaining the caller's UTF-16 text units.
    pub fn substitute_functions_in_value(
        &self,
        value: &crate::text::Text,
        at_rule: bool,
    ) -> Result<crate::text::Text, CompileError> {
        super::check_limit(value.byte_len(), self.limits.input_bytes, "input_bytes")?;
        let output = super::functions::in_text_value(
            value,
            &self.theme,
            at_rule,
            self.limits.ast_depth,
            self.limits.css_bytes,
        )?;
        super::check_limit(output.byte_len(), self.limits.css_bytes, "css_bytes")?;
        Ok(output)
    }
}

#[cfg(test)]
#[path = "../../../../tests/utility/compiler/public-api-native.rs"]
mod tests;
