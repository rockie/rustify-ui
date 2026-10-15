//! Current-language search data, extracted from the candidate's actual HTML.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use dom_query::{Document, NodeRef};
use serde::Serialize;

use crate::content::{Analysis, Diagnostic, Project};
use crate::diagnostic::Severity;
use crate::generation::{fingerprint, page_path};

/// Generated search files and the index URL for each interface language.
pub struct SearchBundle {
    pub files: BTreeMap<String, Arc<[u8]>>,
    pub indexes: BTreeMap<String, String>,
    pub module_url: String,
    pub worker_url: String,
}

pub(crate) const MODULE_PLACEHOLDER: &str = "/__rustify_docs_search_module__.js";
pub(crate) const WORKER_PLACEHOLDER: &str = "/__rustify_docs_search_worker__.js";

pub(crate) fn index_placeholder(lang: &str) -> String {
    format!("/__rustify_docs_search_index_{lang}__.json")
}

#[derive(Serialize)]
struct SearchIndex<'a> {
    schema_version: u8,
    ui_lang: &'a str,
    pages: Vec<SearchPage<'a>>,
}

#[derive(Serialize)]
struct SearchPage<'a> {
    route: &'a str,
    title: &'a str,
    source_lang: &'a str,
    is_fallback: bool,
    nav_order: usize,
    sections: Vec<Section>,
}

#[derive(Serialize)]
struct Section {
    anchor: Option<String>,
    path: Vec<String>,
    text: String,
}

/// Uses only published, search-enabled pages and this generation's documents.
pub fn build(
    project: &Project,
    analysis: &Analysis,
    documents: &BTreeMap<String, Arc<[u8]>>,
) -> Result<SearchBundle, Vec<Diagnostic>> {
    let mut bundle = SearchBundle {
        files: BTreeMap::new(),
        indexes: BTreeMap::new(),
        module_url: String::new(),
        worker_url: String::new(),
    };
    let worker = include_bytes!("../assets/search-worker.js");
    let worker_path = format!("assets/search-worker.{}.js", fingerprint(worker));
    bundle.worker_url = format!("{}{worker_path}", project.config.site.base);
    bundle
        .files
        .insert(worker_path, Arc::from(worker.as_slice()));
    let module = include_bytes!("../assets/search.js");
    let module_path = format!("assets/search.{}.js", fingerprint(module));
    bundle.module_url = format!("{}{module_path}", project.config.site.base);
    bundle
        .files
        .insert(module_path, Arc::from(module.as_slice()));
    let languages: Vec<_> = analysis.catalogs.keys().map(String::as_str).collect();
    let default_drafts: BTreeSet<_> = analysis
        .pages
        .iter()
        .filter(|page| {
            page.locale
                .ui_lang
                .eq_ignore_ascii_case(&project.config.site.lang)
                && page.draft
        })
        .map(|page| page.locale.logical_route.as_str())
        .collect();
    for lang in languages {
        let nav = analysis
            .locales
            .get(lang)
            .map_or(&analysis.nav, |view| &view.nav);
        let mut routes = Vec::new();
        let mut seen = BTreeSet::new();
        for item in nav.iter().flat_map(|group| &group.items) {
            if seen.insert(item.route.as_str()) {
                routes.push(item.route.as_str());
            }
        }
        for page in analysis.routes.keys() {
            if seen.insert(page) {
                routes.push(page);
            }
        }
        let mut pages = Vec::new();
        for route in routes {
            let Some(&i) = analysis.routes.get(route) else {
                continue;
            };
            let page = &analysis.pages[i];
            if page.draft
                || default_drafts.contains(page.locale.logical_route.as_str())
                || page.front.search == Some(false)
                || page.locale.ui_lang != lang
            {
                continue;
            }
            let diagnostic = |message| {
                vec![Diagnostic::in_file(
                    &page.file,
                    Severity::Error,
                    "search-extraction",
                    message,
                )]
            };
            let document = documents
                .get(&page_path(&page.route))
                .ok_or_else(|| diagnostic("the search page has no candidate document"))?;
            let html = std::str::from_utf8(document)
                .map_err(|_| diagnostic("the search page is not UTF-8 HTML"))?;
            let dom = Document::from(html);
            let main = dom.select("main");
            if main.length() != 1 {
                return Err(diagnostic(
                    "search extraction requires exactly one main element",
                ));
            }
            let mut sections = vec![Section {
                anchor: None,
                path: vec![page.title.clone()],
                text: String::new(),
            }];
            if let Some(main) = main.nodes().first() {
                walk(*main, &mut sections, &mut None, false);
            }
            for section in &mut sections {
                section.text = section
                    .text
                    .split_whitespace()
                    .collect::<Vec<_>>()
                    .join(" ");
            }
            pages.push(SearchPage {
                route: &page.route,
                title: &page.title,
                source_lang: &page.locale.source_lang,
                is_fallback: page.locale.is_fallback,
                nav_order: pages.len(),
                sections,
            });
        }
        let bytes = serde_json::to_vec(&SearchIndex {
            schema_version: 1,
            ui_lang: lang,
            pages,
        })
        .map_err(|error| {
            vec![Diagnostic::general(
                Severity::Error,
                "search-index",
                format!("cannot serialize search index: {error}"),
            )]
        })?;
        let path = format!("assets/search.{lang}.{}.json", fingerprint(&bytes));
        bundle.indexes.insert(
            lang.to_owned(),
            format!("{}{path}", project.config.site.base),
        );
        bundle.files.insert(path, Arc::from(bytes));
    }
    Ok(bundle)
}

fn append(sections: &mut [Section], text: &str) {
    if let Some(section) = sections.last_mut() {
        section.text.push_str(text);
    }
}

fn walk(
    node: NodeRef<'_>,
    sections: &mut Vec<Section>,
    parent: &mut Option<String>,
    in_source: bool,
) {
    if node.is_text() {
        append(sections, &node.text());
        return;
    }
    let name = node.node_name();
    let name = name.as_deref().unwrap_or_default();
    let in_source = in_source || node.has_class("rd-demo-source");
    if matches!(
        name,
        "nav" | "footer" | "script" | "style" | "template" | "noscript" | "figcaption"
    ) || node
        .attr("role")
        .is_some_and(|role| role.as_ref() == "tablist")
        || [
            "rd-layout-slot",
            "rd-demo-status",
            "rd-demo-path",
            "rd-code-tabs",
            "rd-copy",
            "rd-untranslated",
            "rd-draft",
        ]
        .iter()
        .any(|class| node.has_class(class))
        || (in_source
            && (node
                .attr("data-rd-tab-panel")
                .is_some_and(|tab| tab.as_ref() != "0")
                || node.attr("hidden").is_some()))
    {
        append(sections, " ");
        return;
    }
    if matches!(name, "h2" | "h3") {
        let heading = node.text().split_whitespace().collect::<Vec<_>>().join(" ");
        let mut path = vec![sections[0].path[0].clone()];
        if name == "h2" {
            *parent = Some(heading.clone());
        } else if let Some(parent) = parent.as_ref() {
            path.push(parent.clone());
        }
        path.push(heading.clone());
        if let Some(id) = node.id_attr().filter(|id| !id.is_empty()) {
            sections.push(Section {
                anchor: Some(id.to_string()),
                path,
                text: String::new(),
            });
        } else {
            append(sections, " ");
            append(sections, &heading);
            append(sections, " ");
        }
        return;
    }
    let block = matches!(
        name,
        "main"
            | "article"
            | "section"
            | "div"
            | "p"
            | "h1"
            | "h4"
            | "h5"
            | "h6"
            | "pre"
            | "li"
            | "ul"
            | "ol"
            | "table"
            | "tr"
            | "td"
            | "th"
            | "dl"
            | "dt"
            | "dd"
            | "blockquote"
            | "br"
            | "hr"
            | "fieldset"
            | "button"
            | "label"
            | "select"
            | "option"
            | "textarea"
    );
    if block {
        append(sections, " ");
    }
    for child in node.children_it(false) {
        walk(child, sections, parent, in_source);
    }
    if block {
        append(sections, " ");
    }
}
