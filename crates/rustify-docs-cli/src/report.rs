//! How a command reports its diagnostics: readable text on stderr, or one
//! JSON object on stdout and nothing else there.

use rustify_docs::content::Diagnostic;
use rustify_docs::diagnostic::Severity;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Format {
    Text,
    Json,
}

/// What the diagnostics add up to.
pub struct Summary {
    pub errors: usize,
    pub warnings: usize,
    /// Warnings count as failures (`--deny-warnings`).
    pub deny_warnings: bool,
}

impl Summary {
    pub fn of(diagnostics: &[Diagnostic], deny_warnings: bool) -> Self {
        Self {
            errors: diagnostics
                .iter()
                .filter(|d| d.severity == Severity::Error)
                .count(),
            warnings: diagnostics
                .iter()
                .filter(|d| d.severity == Severity::Warning)
                .count(),
            deny_warnings,
        }
    }

    pub fn success(&self) -> bool {
        self.errors == 0 && !(self.deny_warnings && self.warnings > 0)
    }
}

/// One diagnostic as an author reads it.
pub fn text(diagnostic: &Diagnostic) -> String {
    let mut out = format!(
        "{}[{}]: {}\n",
        diagnostic.severity.as_str(),
        diagnostic.rule,
        diagnostic.message
    );
    match (&diagnostic.file, diagnostic.line, diagnostic.column) {
        (Some(file), Some(line), Some(column)) => {
            out.push_str(&format!("  --> {file}:{line}:{column}\n"))
        }
        (Some(file), _, _) => out.push_str(&format!("  --> {file}\n")),
        _ => {}
    }
    if let Some(suggestion) = &diagnostic.suggestion {
        out.push_str(&format!("  = help: {suggestion}\n"));
    }
    out
}

/// One diagnostic as `--format json` and `dev`'s pages receive it.
pub fn diagnostic_json(d: &Diagnostic) -> serde_json::Value {
    serde_json::json!({
        "severity": d.severity.as_str(),
        "rule": d.rule,
        "file": d.file,
        "line": d.line,
        "column": d.column,
        "message": d.message,
        "suggestion": d.suggestion,
    })
}

pub fn json(diagnostics: &[Diagnostic], summary: &Summary, extra: serde_json::Value) -> String {
    let list: Vec<serde_json::Value> = diagnostics.iter().map(diagnostic_json).collect();
    let mut summary_json =
        serde_json::json!({ "errors": summary.errors, "warnings": summary.warnings });
    if let (Some(target), serde_json::Value::Object(extra)) = (summary_json.as_object_mut(), extra)
    {
        target.extend(extra);
    }
    serde_json::json!({
        "schema_version": 1,
        "success": summary.success(),
        "diagnostics": list,
        "summary": summary_json,
    })
    .to_string()
}
