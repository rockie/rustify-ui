use rustify_style::compiler::{
    CompileError, CompileReport, Compiler, CompilerInput, Limits, Prepared, SourceAsset,
};
use rustify_style::text::Text;
use serde::{Deserialize, Serialize};
use wasm_bindgen::{prelude::*, JsCast};

#[cfg(feature = "scanner")]
mod scanning;

#[derive(Deserialize)]
struct Options {
    input: CompilerInput,
    assets: Vec<SourceAsset>,
    limits: Limits,
    owner: Option<String>,
    #[serde(default)]
    revision_base: u64,
}

#[derive(Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum Error {
    Protocol {
        message: String,
    },
    Compile {
        error: CompileError,
    },
    #[cfg(feature = "scanner")]
    Scan {
        message: String,
    },
}
impl From<CompileError> for Error {
    fn from(error: CompileError) -> Self {
        Self::Compile { error }
    }
}
impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self::Protocol {
            message: error.to_string(),
        }
    }
}

#[derive(Serialize)]
struct PreparedReply<'a> {
    token: u32,
    report: &'a CompileReport,
    candidates: &'a [Text],
}

struct Session {
    compiler: Compiler,
    pending: Option<(u32, Prepared, CompileReport)>,
    last_token: u32,
    owner: Option<String>,
    limits: Limits,
    committed: CompileReport,
    replacement: Option<Box<Session>>,
    revision_base: u64,
}
impl Session {
    fn new(input: &str) -> Result<Self, Error> {
        let options: Options = serde_json::from_str(input)?;
        if let Some(owner) = &options.owner {
            rustify_style::scope::stylesheet(&[], owner, options.limits.ast_depth)?;
        }
        let compiler = Compiler::new(options.input, options.assets, options.limits.clone())?;
        let mut committed = compiler.report().clone();
        committed.revision = options.revision_base;
        if let Some(owner) = &options.owner {
            let ast = compiler.initial_ast()?;
            let scoped = rustify_style::scope::stylesheet(&ast, owner, options.limits.ast_depth)?;
            committed.css = rustify_style::compiler::ast::to_css(&scoped);
            if committed.css.len() > options.limits.css_bytes {
                return Err(CompileError::Limit {
                    resource: "css_bytes".into(),
                    limit: options.limits.css_bytes,
                }
                .into());
            }
            committed.metrics.output_bytes = committed.css.len();
        }
        Ok(Self {
            compiler,
            pending: None,
            last_token: 0,
            owner: options.owner,
            limits: options.limits,
            committed,
            replacement: None,
            revision_base: options.revision_base,
        })
    }
    fn prepare(&mut self, input: &str) -> Result<String, Error> {
        if input.len() > self.limits.queued_bytes {
            return Err(CompileError::Limit {
                resource: "queued_bytes".into(),
                limit: self.limits.queued_bytes,
            }
            .into());
        }
        let candidates: Vec<Text> = serde_json::from_str(input)?;
        self.prepare_candidates(&candidates)
    }
    fn prepare_replacement(&mut self, config: &str, candidates: &str) -> Result<String, Error> {
        if self.pending.is_some() || self.replacement.is_some() {
            return Err(CompileError::PendingPrepared.into());
        }
        let mut staged = Session::new(config)?;
        staged.last_token = self.last_token;
        staged.revision_base = self.committed.revision;
        let output = staged.prepare(candidates)?;
        self.last_token = staged.last_token;
        self.replacement = Some(Box::new(staged));
        Ok(output)
    }
    fn prepare_candidates(&mut self, candidates: &[Text]) -> Result<String, Error> {
        if self.replacement.is_some() {
            return Err(CompileError::PendingPrepared.into());
        }
        let token = self
            .last_token
            .checked_add(1)
            .ok_or(CompileError::RevisionExhausted)?;
        let prepared = self.compiler.prepare_add_text(candidates)?;
        let mut report = prepared.report().clone();
        report.revision = report
            .revision
            .checked_add(self.revision_base)
            .ok_or(CompileError::RevisionExhausted)?;
        if let Some(owner) = &self.owner {
            match rustify_style::scope::stylesheet(prepared.ast(), owner, self.limits.ast_depth) {
                Ok(nodes) => report.css = rustify_style::compiler::ast::to_css(&nodes),
                Err(error) => {
                    self.compiler.discard(prepared)?;
                    return Err(error.into());
                }
            }
        }
        if report.css.len() > self.limits.css_bytes {
            self.compiler.discard(prepared)?;
            return Err(CompileError::Limit {
                resource: "css_bytes".into(),
                limit: self.limits.css_bytes,
            }
            .into());
        }
        report.metrics.output_bytes = report.css.len();
        let output = serde_json::to_string(&PreparedReply {
            token,
            report: &report,
            candidates,
        })?;
        self.pending = Some((token, prepared, report));
        self.last_token = token;
        Ok(output)
    }
    #[cfg(not(feature = "scanner"))]
    fn prepare_content(&mut self, _input: &str) -> Result<String, Error> {
        Err(CompileError::UnsupportedFeature {
            feature: "source scanning".into(),
        }
        .into())
    }
    #[cfg(not(feature = "scanner"))]
    fn prepare_sources(&mut self, _input: &str) -> Result<String, Error> {
        Err(CompileError::UnsupportedFeature {
            feature: "source scanning".into(),
        }
        .into())
    }
    fn finish(&mut self, token: u32, commit: bool) -> Result<String, Error> {
        if let Some(staged) = self.replacement.as_mut() {
            let output = staged.finish(token, commit)?;
            if commit {
                if let Some(staged) = self.replacement.take() {
                    *self = *staged;
                }
                return Ok(output);
            }
            self.replacement = None;
            return Ok(serde_json::to_string(&self.committed)?);
        }
        if self.pending.as_ref().map(|entry| entry.0) != Some(token) {
            return Err(CompileError::StalePrepared.into());
        }
        let (_, prepared, report) = self.pending.take().ok_or(CompileError::StalePrepared)?;
        if commit {
            self.compiler.commit(prepared)?;
            self.committed = report;
        } else {
            self.compiler.discard(prepared)?;
        }
        Ok(serde_json::to_string(&self.committed)?)
    }
}

fn variant_text_units(units: impl Iterator<Item = u16>, limits: &Limits) -> Result<Text, Error> {
    let (resource, limit) = if limits.queued_bytes <= limits.input_bytes {
        ("queued_bytes", limits.queued_bytes)
    } else {
        ("input_bytes", limits.input_bytes)
    };
    let mut units = units.peekable();
    let mut output = Vec::new();
    let mut bytes = 0usize;
    while let Some(unit) = units.next() {
        let pair = matches!(unit, 0xd800..=0xdbff)
            && units
                .peek()
                .is_some_and(|unit| matches!(*unit, 0xdc00..=0xdfff));
        bytes = bytes.saturating_add(if pair {
            4
        } else {
            match unit {
                0..=0x7f => 1,
                0x80..=0x7ff => 2,
                _ => 3,
            }
        });
        if bytes > limit {
            return Err(CompileError::Limit {
                resource: resource.into(),
                limit,
            }
            .into());
        }
        output.push(unit);
        if pair {
            if let Some(low) = units.next() {
                output.push(low);
            }
        }
    }
    Ok(Text::from_units(output))
}

fn javascript_error(error: Error) -> JsValue {
    // All variants contain JSON-compatible data; no input values are discarded.
    JsValue::from_str(&serde_json::to_string(&error).expect("serializable facade error"))
}

/// Owns compiler state and staging tokens in the Worker WASM module.
#[wasm_bindgen]
pub struct CompilerSession {
    inner: Session,
}
#[wasm_bindgen]
impl CompilerSession {
    #[wasm_bindgen(constructor)]
    pub fn new(input: &str) -> Result<CompilerSession, JsValue> {
        Session::new(input)
            .map(|inner| Self { inner })
            .map_err(javascript_error)
    }
    pub fn prepare(&mut self, candidates: &str) -> Result<String, JsValue> {
        self.inner.prepare(candidates).map_err(javascript_error)
    }
    /// Stages a fresh compiler configuration while retaining the committed session.
    pub fn replace_config(&mut self, config: &str, candidates: &str) -> Result<String, JsValue> {
        self.inner
            .prepare_replacement(config, candidates)
            .map_err(javascript_error)
    }
    /// Scans raw source text and stages its candidates and variable dependencies together.
    pub fn prepare_content(&mut self, content: &str) -> Result<String, JsValue> {
        self.inner
            .prepare_content(content)
            .map_err(javascript_error)
    }
    /// Selects files from a complete virtual snapshot using this compiler's CSS sources.
    pub fn prepare_sources(&mut self, sources: &str) -> Result<String, JsValue> {
        self.inner
            .prepare_sources(sources)
            .map_err(javascript_error)
    }
    pub fn commit(&mut self, token: u32) -> Result<String, JsValue> {
        self.inner.finish(token, true).map_err(javascript_error)
    }
    pub fn discard(&mut self, token: u32) -> Result<String, JsValue> {
        self.inner.finish(token, false).map_err(javascript_error)
    }
    pub fn report(&self) -> String {
        serde_json::to_string(&self.inner.committed).expect("serializable compile report")
    }
    pub fn source_metadata(&self) -> String {
        serde_json::to_string(self.inner.compiler.source_metadata())
            .expect("serializable source metadata")
    }
    pub fn parse_variant(&self, raw: &JsValue) -> Result<String, JsValue> {
        if !raw.is_string() {
            return Err(javascript_error(Error::Protocol {
                message: "expected a string".into(),
            }));
        }
        let string: &js_sys::JsString = raw.unchecked_ref();
        let raw =
            variant_text_units(string.iter(), &self.inner.limits).map_err(javascript_error)?;
        let variant = self
            .inner
            .compiler
            .parse_variant_object_text(&raw)
            .map_err(|error| javascript_error(error.into()))?;
        serde_json::to_string(&variant).map_err(|error| javascript_error(error.into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    pub(super) fn options(owner: Option<&str>) -> String {
        serde_json::json!({
            "input": { "css": "@tailwind utilities;", "entry": "memory:input.css" },
            "assets": [], "owner": owner,
            "limits": { "input_bytes": 65536, "asset_bytes": 65536, "candidate_count": 1024,
                "ast_depth": 32, "css_bytes": 262144, "queued_bytes": 65536, "cache_bytes": 1048576 },
        }).to_string()
    }
    #[derive(Deserialize)]
    struct TextPreparedReply {
        token: u32,
        report: CompileReport,
        candidates: Vec<Text>,
    }

    #[test]
    fn candidate_json_retains_raw_units_through_prepare_commit_discard_and_replacement() {
        for raw in [
            &[0xd800][..],
            &[0xdc00],
            &[0xd83d, 0xde00],
            &[0xfffd],
            &[0xe000],
        ] {
            let candidate = Text::from_units(
                "[--unit:"
                    .encode_utf16()
                    .chain(raw.iter().copied())
                    .chain("]".encode_utf16())
                    .collect(),
            );
            let input = serde_json::to_string(std::slice::from_ref(&candidate)).unwrap();
            let mut session = Session::new(&options(None)).unwrap_or_else(|_| panic!("options"));
            let prepared: TextPreparedReply = serde_json::from_str(
                &session
                    .prepare(&input)
                    .unwrap_or_else(|_| panic!("prepare")),
            )
            .unwrap();
            assert_eq!(prepared.candidates[0].units(), candidate.units());
            assert_eq!(prepared.report.admitted[0].units(), candidate.units());
            let declaration: Vec<u16> = "--unit: "
                .encode_utf16()
                .chain(raw.iter().copied())
                .chain(";".encode_utf16())
                .collect();
            assert!(prepared
                .report
                .css
                .units()
                .windows(declaration.len())
                .any(|part| part == declaration));
            session
                .finish(prepared.token, false)
                .unwrap_or_else(|_| panic!("discard"));
            assert!(session.committed.admitted.is_empty());
            let prepared: TextPreparedReply =
                serde_json::from_str(&session.prepare(&input).unwrap_or_else(|_| panic!("retry")))
                    .unwrap();
            session
                .finish(prepared.token, true)
                .unwrap_or_else(|_| panic!("commit"));
            assert_eq!(session.committed.admitted[0].units(), candidate.units());
            let repeat: TextPreparedReply =
                serde_json::from_str(&session.prepare(&input).unwrap_or_else(|_| panic!("repeat")))
                    .unwrap();
            assert_eq!(repeat.report.metrics.parsed_candidates, 0);
            session
                .finish(repeat.token, false)
                .unwrap_or_else(|_| panic!("discard repeat"));
            let staged: TextPreparedReply = serde_json::from_str(
                &session
                    .prepare_replacement(&options(None), &input)
                    .unwrap_or_else(|_| panic!("replacement")),
            )
            .unwrap();
            assert_eq!(staged.candidates[0].units(), candidate.units());
            session
                .finish(staged.token, true)
                .unwrap_or_else(|_| panic!("replace commit"));
            assert_eq!(session.committed.admitted[0].units(), candidate.units());
        }
    }

    #[test]
    fn variant_metadata_reader_counts_pair_and_lone_logical_input_bytes() {
        let options: Options = serde_json::from_str(&options(None)).unwrap();
        let mut limits = options.limits;
        for raw in [&[0xd800, 120][..], &[0xdc00, 120], &[0xd83d, 0xde00]] {
            limits.input_bytes = 4;
            assert_eq!(
                variant_text_units(raw.iter().copied(), &limits)
                    .unwrap_or_else(|_| panic!("four bytes"))
                    .units(),
                raw
            );
            limits.input_bytes = 3;
            assert!(
                matches!(variant_text_units(raw.iter().copied(), &limits), Err(Error::Compile { error: CompileError::Limit { resource, limit: 3 } }) if resource == "input_bytes")
            );
        }
        limits.input_bytes = 196608;
        limits.queued_bytes = 196608;
        let every_unit =
            variant_text_units(0..=u16::MAX, &limits).unwrap_or_else(|_| panic!("all units"));
        assert_eq!(every_unit.units(), &(0..=u16::MAX).collect::<Vec<_>>());
    }

    #[test]
    fn variant_metadata_reader_keeps_queued_limit_priority() {
        let options: Options = serde_json::from_str(&options(None)).unwrap();
        let mut limits = options.limits;
        limits.input_bytes = 4;
        limits.queued_bytes = 4;
        assert!(
            matches!(variant_text_units([0xd800, 120, 120].into_iter(), &limits),
            Err(Error::Compile { error: CompileError::Limit { resource, limit: 4 } })
                if resource == "queued_bytes")
        );
        limits.input_bytes = 5;
        assert!(
            matches!(variant_text_units([0xd800, 120, 120].into_iter(), &limits),
            Err(Error::Compile { error: CompileError::Limit { resource, limit: 4 } })
                if resource == "queued_bytes")
        );
    }

    #[test]
    fn invalid_candidate_json_recovers_without_creating_pending_text_cache() {
        let mut session = Session::new(&options(None)).unwrap_or_else(|_| panic!("options"));
        assert!(session.prepare(r#"["\ud800",]"#).is_err());
        assert!(session.pending.is_none());
        let candidate = Text::from_units(vec![0xd800]);
        let input = serde_json::to_string(std::slice::from_ref(&candidate)).unwrap();
        let prepared: TextPreparedReply =
            serde_json::from_str(&session.prepare(&input).unwrap_or_else(|_| panic!("retry")))
                .unwrap();
        assert_eq!(prepared.candidates[0].units(), candidate.units());
        // Unknown roots remain a diagnostic carrying the original candidate units.
        let diagnostic = prepared
            .report
            .rejected
            .iter()
            .chain(&prepared.report.unsupported)
            .next()
            .expect("candidate diagnostic");
        assert_eq!(diagnostic.candidate.units(), candidate.units());
        session
            .finish(prepared.token, false)
            .unwrap_or_else(|_| panic!("discard"));
        assert!(session.committed.admitted.is_empty());
    }

    #[test]
    fn replacement_configuration_is_staged_until_commit_and_discard_keeps_old_cache() {
        let mut session = Session::new(&options(None)).unwrap_or_else(|_| panic!("options"));
        let first: serde_json::Value = serde_json::from_str(
            &session
                .prepare("[\"w-[137px]\"]")
                .unwrap_or_else(|_| panic!("prepare")),
        )
        .unwrap();
        session
            .finish(first["token"].as_u64().unwrap() as u32, true)
            .unwrap_or_else(|_| panic!("commit"));
        let previous = session.committed.clone();
        let mut config: serde_json::Value = serde_json::from_str(&options(None)).unwrap();
        config["input"]["css"] =
            serde_json::json!("@theme { --color-brand: #123456; } @tailwind utilities;");
        let replacement: serde_json::Value = serde_json::from_str(
            &session
                .prepare_replacement(&config.to_string(), "[\"bg-brand\"]")
                .unwrap_or_else(|_| panic!("replace")),
        )
        .unwrap();
        assert_eq!(session.committed, previous);
        assert!(session.prepare("[]").is_err());
        session
            .finish(replacement["token"].as_u64().unwrap() as u32, false)
            .unwrap_or_else(|_| panic!("discard"));
        assert_eq!(session.committed, previous);
        let replacement: serde_json::Value = serde_json::from_str(
            &session
                .prepare_replacement(&config.to_string(), "[\"bg-brand\"]")
                .unwrap_or_else(|_| panic!("replace")),
        )
        .unwrap();
        session
            .finish(replacement["token"].as_u64().unwrap() as u32, true)
            .unwrap_or_else(|_| panic!("commit"));
        assert_eq!(session.committed.admitted, ["bg-brand"]);
        assert!(session.committed.css.contains("#123456"));
        assert!(!session.committed.css.contains("137px"));
    }

    #[test]
    fn prepared_reply_preserves_variable_candidates_for_recovery() {
        let mut session = Session::new(&options(None)).unwrap_or_else(|_| panic!("valid options"));
        let candidates = vec!["w-[137px]".into(), "--application-color".into()];
        let output: serde_json::Value = serde_json::from_str(
            &session
                .prepare_candidates(&candidates)
                .unwrap_or_else(|_| panic!("valid prepare")),
        )
        .unwrap();
        assert_eq!(output["candidates"], serde_json::json!(candidates));
    }
    #[test]
    fn replacing_configuration_stages_and_discards_without_mutating_committed_state() {
        let mut session = Session::new(&options(None)).unwrap_or_else(|_| panic!("valid options"));
        let prepared: serde_json::Value = serde_json::from_str(
            &session
                .prepare("[\"w-[137px]\"]")
                .unwrap_or_else(|_| panic!("prepare")),
        )
        .unwrap();
        session
            .finish(prepared["token"].as_u64().unwrap() as u32, true)
            .unwrap_or_else(|_| panic!("commit"));
        let old = session.committed.clone();
        let mut config: serde_json::Value = serde_json::from_str(&options(None)).unwrap();
        config["input"]["css"] =
            serde_json::json!("@theme { --spacing: 0.5rem; } @tailwind utilities;");
        let staged: serde_json::Value = serde_json::from_str(
            &session
                .prepare_replacement(&config.to_string(), "[\"p-4\"]")
                .unwrap_or_else(|_| panic!("replacement")),
        )
        .unwrap();
        assert_eq!(session.committed.css, old.css);
        assert!(staged["report"]["css"]
            .as_str()
            .unwrap()
            .contains("--spacing: 0.5rem"));
        assert!(session.prepare("[]").is_err());
        session
            .finish(staged["token"].as_u64().unwrap() as u32, false)
            .unwrap_or_else(|_| panic!("discard replacement"));
        assert_eq!(session.committed.css, old.css);
        let staged: serde_json::Value = serde_json::from_str(
            &session
                .prepare_replacement(&config.to_string(), "[\"p-4\"]")
                .unwrap_or_else(|_| panic!("replacement")),
        )
        .unwrap();
        session
            .finish(staged["token"].as_u64().unwrap() as u32, true)
            .unwrap_or_else(|_| panic!("commit replacement"));
        assert!(session.committed.css.contains("--spacing: 0.5rem"));
        assert_eq!(session.committed.revision, old.revision + 1);
    }

    #[test]
    #[cfg(rustify_repo_tests)]
    fn prepare_commit_discard_keep_a_single_owned_snapshot() {
        let mut session = Session::new(&options(None)).unwrap_or_else(|_| panic!("valid options"));
        let output: serde_json::Value = serde_json::from_str(
            &session
                .prepare("[\"w-[137px]\",\"bg-[#123456]\"]")
                .unwrap_or_else(|_| panic!("valid prepare")),
        )
        .unwrap();
        assert_eq!(session.committed.revision, 0);
        assert!(session.prepare("[]").is_err());
        assert!(session.finish(99, true).is_err());
        session
            .finish(output["token"].as_u64().unwrap() as u32, false)
            .unwrap_or_else(|_| panic!("valid discard"));
        assert_eq!(session.committed.revision, 0);
        let output: serde_json::Value = serde_json::from_str(
            &session
                .prepare("[\"w-[137px]\",\"bg-[#123456]\"]")
                .unwrap_or_else(|_| panic!("valid prepare")),
        )
        .unwrap();
        session
            .finish(output["token"].as_u64().unwrap() as u32, true)
            .unwrap_or_else(|_| panic!("valid commit"));
        assert_eq!(
            session.committed.css,
            include_str!("../../../tests/utility/compiler/compiler-smoke.css")
        );
        let output: serde_json::Value = serde_json::from_str(
            &session
                .prepare("[\"w-[137px]\"]")
                .unwrap_or_else(|_| panic!("valid repeat")),
        )
        .unwrap();
        assert_eq!(output["report"]["metrics"]["parsed_candidates"], 0);
        assert_eq!(output["report"]["changed"], false);
    }
    #[test]
    fn scoped_facade_does_not_publish_uninstalled_preparation() {
        let mut session =
            Session::new(&options(Some("alpha"))).unwrap_or_else(|_| panic!("valid options"));
        let output: serde_json::Value = serde_json::from_str(
            &session
                .prepare("[\"w-[137px]\"]")
                .unwrap_or_else(|_| panic!("valid prepare")),
        )
        .unwrap();
        assert!(output["report"]["css"]
            .as_str()
            .unwrap()
            .contains("data-rustify-style-owner=\"alpha\""));
        assert!(session.committed.admitted.is_empty());
        assert!(Session::new(&options(Some("bad\"owner"))).is_err());
    }

    #[test]
    fn initial_css_report_uses_the_same_scope_as_prepared_reports() {
        let mut input: serde_json::Value = serde_json::from_str(&options(Some("alpha"))).unwrap();
        input["input"]["css"] =
            serde_json::json!(":root { --color-brand: red; } @tailwind utilities;");
        let mut session =
            Session::new(&input.to_string()).unwrap_or_else(|_| panic!("valid initial CSS"));
        assert!(session
            .committed
            .css
            .contains("data-rustify-style-owner=\"alpha\""));
        assert!(session.committed.css.contains(":scope {"));
        assert!(!session.committed.css.contains(":root {"));
        let prepared: serde_json::Value = serde_json::from_str(
            &session
                .prepare("[]")
                .unwrap_or_else(|_| panic!("valid empty preparation")),
        )
        .unwrap();
        assert_eq!(
            serde_json::from_value::<rustify_style::text::Text>(prepared["report"]["css"].clone())
                .unwrap(),
            session.committed.css
        );
        assert_eq!(prepared["report"]["changed"], false);
    }
}

#[cfg(all(test, feature = "scanner"))]
mod scanning_tests {
    use super::*;
    fn session() -> Session {
        let mut options: serde_json::Value =
            serde_json::from_str(&super::tests::options(None)).unwrap();
        options["input"]["entry"] = serde_json::json!("/app/input.css");
        options["input"]["css"] = serde_json::json!("@theme { --color-brand: #123456; } @tailwind utilities source(none); @source \"./src/**\";");
        Session::new(&options.to_string()).unwrap_or_else(|_| panic!("valid options"))
    }
    #[test]
    fn scanning_stages_variables_and_preserves_discarded_candidates_for_retry() {
        let mut session = session();
        let content = r#"[{"content":"w-[137px] var(--color-brand)","extension":"html"}]"#;
        let before = serde_json::to_string(&session.committed).unwrap();
        let prepared: serde_json::Value = serde_json::from_str(
            &session
                .prepare_content(content)
                .unwrap_or_else(|_| panic!("valid content")),
        )
        .unwrap();
        assert!(prepared["report"]["css"]
            .as_str()
            .unwrap()
            .contains("--color-brand: #123456"));
        session
            .finish(prepared["token"].as_u64().unwrap() as u32, false)
            .unwrap_or_else(|_| panic!("discard"));
        assert_eq!(serde_json::to_string(&session.committed).unwrap(), before);
        let retry: serde_json::Value = serde_json::from_str(
            &session
                .prepare_content(content)
                .unwrap_or_else(|_| panic!("retry")),
        )
        .unwrap();
        assert_eq!(
            retry["report"]["metrics"]["parsed_candidates"],
            prepared["report"]["metrics"]["parsed_candidates"]
        );
        session
            .finish(retry["token"].as_u64().unwrap() as u32, true)
            .unwrap_or_else(|_| panic!("commit"));
        let repeat: serde_json::Value = serde_json::from_str(
            &session
                .prepare_content(content)
                .unwrap_or_else(|_| panic!("repeat")),
        )
        .unwrap();
        assert_eq!(repeat["report"]["changed"], false);
        assert_eq!(repeat["report"]["metrics"]["parsed_candidates"], 0);
    }
    #[test]
    fn source_selection_and_missing_information_are_explicit() {
        let mut session = session();
        let snapshot = serde_json::json!({"root":"/", "base":"/app", "entries":[
            {"kind":"directory","path":"/app/src"},
            {"kind":"file","path":"/app/src/view.html","content":"w-[137px] var(--color-brand)","revision":1},
            {"kind":"file","path":"/app/other.html","content":"w-[999px]","revision":1}
        ]});
        let prepared: serde_json::Value = serde_json::from_str(
            &session
                .prepare_sources(&snapshot.to_string())
                .unwrap_or_else(|_| panic!("valid snapshot")),
        )
        .unwrap();
        assert!(prepared["report"]["css"]
            .as_str()
            .unwrap()
            .contains("width: 137px"));
        assert!(!prepared["report"]["css"]
            .as_str()
            .unwrap()
            .contains("999px"));
        session
            .finish(prepared["token"].as_u64().unwrap() as u32, true)
            .unwrap_or_else(|_| panic!("commit"));
        let before = serde_json::to_string(&session.committed).unwrap();
        let missing = serde_json::json!({"root":"/", "base":"/app", "entries":[{"kind":"unavailable", "path":"/app/src"}]});
        assert!(session.prepare_sources(&missing.to_string()).is_err());
        assert_eq!(serde_json::to_string(&session.committed).unwrap(), before);
    }
}
