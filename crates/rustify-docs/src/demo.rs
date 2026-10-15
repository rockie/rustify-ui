//! The static example, status and source shell around one SDK island.

use crate::escape::{escape_html, escape_html_into};
use crate::highlight::Highlighter;
use crate::labels::Catalog;
use crate::markdown::{code_group_html, Context, IncludeFiles};
use crate::site::IslandMetadata;

/// Project-relative provenance and the exact source slices validated by the CLI.
pub struct Source<'a> {
    pub path: &'a str,
    pub function: &'a str,
    pub file: &'a str,
}

/// Keeps the server-rendered view separate from source and failure controls.
/// `html` is the real SDK scope; source slices have already passed validation.
pub fn render(
    html: &str,
    metadata: &IslandMetadata,
    source: Option<Source<'_>>,
    highlighter: &Highlighter,
    catalog: &Catalog,
    source_lang: &str,
    client_only: bool,
) -> String {
    let json = serde_json::to_string(metadata).expect("island metadata has JSON-compatible fields");
    let key = escape_html(&metadata.key);
    let mut out = format!(
        "<section class=\"rd-demo\" data-rustify-island=\"{key}\" data-rustify-island-metadata=\"{}\" data-rd-island-state=\"no-script\"{}>\n\
         <fieldset class=\"rd-demo-canvas\" data-rd-island-controls disabled aria-labelledby=\"rd-demo-label-{key}\">",
        escape_html(&json),
        if client_only { " data-rd-island-client-only=\"true\"" } else { "" },
    );
    out.push_str(html);
    out.push_str("</fieldset>\n");
    out.push_str("<div class=\"rd-demo-status\" data-rd-island-status role=\"status\" aria-live=\"polite\">\n");
    for (key, attributes) in [
        (
            "demo.example",
            format!("class=\"sr-only\" id=\"rd-demo-label-{key}\""),
        ),
        ("demo.loading", "data-rd-island-loading hidden".into()),
        (
            "demo.failed",
            "data-rd-island-error aria-hidden=\"true\"".into(),
        ),
    ] {
        let label = catalog.get(key);
        out.push_str(&format!(
            "<span {attributes} lang=\"{}\">{}</span>",
            escape_html(&label.lang),
            escape_html(&label.text)
        ));
    }
    let reload = catalog.get("demo.reload");
    let no_script = catalog.get("demo.no_script");
    out.push_str(&format!(
        "<button type=\"button\" class=\"rd-demo-reload\" data-rd-island-reload aria-hidden=\"true\" lang=\"{}\">{}</button>\
         <noscript><span lang=\"{}\">{}</span></noscript></div>\n",
        escape_html(&reload.lang), escape_html(&reload.text), escape_html(&no_script.lang), escape_html(&no_script.text),
    ));
    if let Some(source) = source {
        out.push_str(
            "<div class=\"rd-demo-source\"><p class=\"rd-demo-path\" data-rd-source-path=\"",
        );
        escape_html_into(&mut out, source.path);
        out.push_str("\">");
        escape_html_into(&mut out, source.path);
        out.push_str("</p>\n");
        let includes = IncludeFiles::default();
        out.push_str(&code_group_html(
            &[
                (catalog.get("demo.function"), source.function),
                (catalog.get("demo.full_file"), source.file),
            ],
            &format!("rd-demo-source-{}", metadata.key),
            &Context {
                highlighter,
                catalog,
                source_lang,
                includes: &includes,
            },
        ));
        out.push_str("</div>\n");
    }
    out.push_str("</section>\n");
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::site::{RenderTarget, SiteEnvironment, PROTOCOL_VERSION};

    fn metadata(key: &str) -> IslandMetadata {
        IslandMetadata {
            protocol_version: PROTOCOL_VERSION,
            key: key.to_string(),
            seed: format!("seed-{key}"),
            target: RenderTarget::Demo {
                name: "Counter".to_string(),
            },
            environment: SiteEnvironment::default(),
        }
    }

    #[test]
    fn source_and_status_stay_outside_the_disabled_original_view() {
        let original = "<div data-rustify-scope=\"seed-counter\"><input value=\"7\" /><!></div>";
        let function = "pub fn 示例() {\r\n    let text = \"<&>\";\r\n}\r\n";
        let file = format!("// Unicode source\r\n{function}");
        let rendered = render(
            original,
            &metadata("counter"),
            Some(Source {
                path: "src/example.rs",
                function,
                file: &file,
            }),
            &Highlighter::new(),
            &crate::labels::resolve("en", &Default::default()),
            "en",
            false,
        );
        let canvas_end = rendered.find("</fieldset>").unwrap();
        assert!(rendered[..canvas_end].ends_with(original));
        assert!(!rendered[..canvas_end].contains("rd-code"));
        assert!(rendered[canvas_end..].contains("data-rd-island-status"));
        assert!(rendered.contains("data-rd-island-error aria-hidden=\"true\""));
        assert!(rendered.contains("data-rd-island-reload aria-hidden=\"true\""));
        assert!(rendered[canvas_end..].contains("data-rd-source-path=\"src/example.rs\""));
        assert!(rendered.contains("data-rd-group=\"rd-demo-source-counter\""));
        assert!(rendered.contains(">Function</button>"));
        assert!(rendered.contains(">Full file</button>"));
        assert!(rendered.contains("<template class=\"rd-code-raw\">pub fn 示例() {&#13;\n"));
        assert!(rendered.contains("&quot;&lt;&amp;&gt;&quot;"));
        assert!(rendered.contains("disabled aria-labelledby="));
        assert!(!rendered.contains(" inert"));
    }

    #[test]
    fn metadata_and_foreign_interface_are_escaped_without_changing_view_html() {
        let original = "<div data-rustify-scope=\"scope\"><!></div>";
        let rendered = render(
            original,
            &metadata("key\"<&"),
            None,
            &Highlighter::new(),
            &crate::labels::resolve("zh", &Default::default()),
            "en",
            true,
        );
        assert!(rendered.contains("data-rustify-island=\"key&quot;&lt;&amp;\""));
        assert!(rendered.contains("data-rustify-island-metadata=\"{&quot;protocol_version&quot;:1"));
        assert!(rendered.contains(original));
        assert!(rendered.contains("data-rd-island-client-only=\"true\""));
        assert!(rendered.contains("data-rd-island-loading hidden lang=\"zh\""));
        assert!(rendered.contains(
            "<noscript><span lang=\"zh\">启用 JavaScript 后可操作此示例</span></noscript>"
        ));
        assert!(!rendered.contains("rd-demo-source\""));
        assert!(!rendered.contains("<script"));
    }
}
