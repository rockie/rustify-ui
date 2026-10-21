//! Language-specific AI discovery, from the same public page graph as HTML.

use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use crate::content::{Analysis, Diagnostic, Page, PageOrigin, Project};
use crate::diagnostic::Severity;
use crate::labels::Catalog;

fn clean(value: &str) -> Option<String> {
    let value = value.split_whitespace().collect::<Vec<_>>().join(" ");
    (!value.is_empty()).then_some(value)
}

fn markdown(value: &str) -> String {
    let mut output = String::new();
    for ch in value.chars() {
        if matches!(ch, '\\' | '[' | ']' | '<' | '>' | '*' | '_' | '`') {
            output.push('\\');
        }
        output.push(ch);
    }
    output
}

fn description(project: &Project, analysis: &Analysis, page: &Page) -> Option<String> {
    let explicit = page.front.description.as_deref().and_then(clean);
    if matches!(page.origin, PageOrigin::Registered(_)) {
        return explicit;
    }
    if let Some(description) = explicit.or_else(|| page.fragment.summary.as_deref().and_then(clean))
    {
        return Some(description);
    }
    if page.front.layout.has_hero() {
        if let Some(tagline) = page.front.tagline.as_deref().and_then(clean) {
            return Some(tagline);
        }
        let builtin;
        let catalog = if let Some(catalog) = analysis.catalogs.get(&page.locale.source_lang) {
            catalog
        } else {
            builtin = crate::labels::resolve(&page.locale.source_lang, &BTreeMap::new());
            &builtin
        };
        return clean(
            &catalog
                .get("ai.home_description")
                .text
                .replace("{site}", &project.config.site.name),
        );
    }
    None
}

struct Entry<'a> {
    page: &'a Page,
    description: String,
}

fn item(output: &mut String, project: &Project, catalog: &Catalog, entry: &Entry<'_>) {
    let page = entry.page;
    let mut title = clean(&page.title).unwrap_or_default();
    if page.locale.is_fallback {
        let note = catalog
            .get("ai.fallback")
            .text
            .replace("{lang}", &page.locale.source_lang);
        title.push_str(&format!(" ({})", clean(&note).unwrap_or_default()));
    }
    let route = if page.locale.is_fallback {
        &page.locale.source_route
    } else {
        &page.route
    };
    let root_relative = crate::escape::encode_path(&format!(
        "{}{route}",
        project.config.site.base.trim_end_matches('/')
    ));
    let url = crate::generation::site_origin(project)
        .map_or(root_relative.clone(), |origin| {
            format!("{origin}{root_relative}")
        })
        .replace('(', "%28")
        .replace(')', "%29")
        .replace('[', "%5B")
        .replace(']', "%5D");
    output.push_str(&format!(
        "- [{}]({url}): {}\n",
        markdown(&title),
        markdown(&entry.description)
    ));
}

/// Builds one llms.txt per interface language. A fallback links to its real
/// source page; a registered page is listed only with an explicit description.
pub fn build(
    project: &Project,
    analysis: &Analysis,
) -> Result<BTreeMap<String, Arc<[u8]>>, Vec<Diagnostic>> {
    let mut files = BTreeMap::new();
    let mut problems = Vec::new();
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
    let languages: Vec<_> = if analysis.locales.is_empty() {
        vec![project.config.site.lang.as_str()]
    } else {
        analysis.locales.keys().map(String::as_str).collect()
    };
    for lang in languages {
        let mut entries = BTreeMap::new();
        for page in &analysis.pages {
            if page.draft
                || default_drafts.contains(page.locale.logical_route.as_str())
                || !page.front.discoverable()
                || (!analysis.locales.is_empty() && page.locale.ui_lang != lang)
            {
                continue;
            }
            if let Some(description) = description(project, analysis, page) {
                entries.insert(page.route.as_str(), Entry { page, description });
            } else if page.origin == PageOrigin::Markdown {
                let mut error = Diagnostic::in_file(
                    &page.file,
                    Severity::Error,
                    "missing-description",
                    "an AI-indexed page needs a nonempty description or first paragraph",
                );
                error.suggestion =
                    Some("add `description:` or a first paragraph, or set `ai: false`".into());
                problems.push(error);
            }
        }
        let catalog = &analysis.catalogs[lang];
        let mut output = format!(
            "# {}\n\n",
            markdown(&clean(&project.config.site.name).unwrap_or_default())
        );
        if let Some(brief) = project.config.site.description.as_deref().and_then(clean) {
            output.push_str(&format!("> {}\n\n", markdown(&brief)));
        }
        let nav = analysis
            .locales
            .get(lang)
            .map_or(&analysis.nav, |view| &view.nav);
        for group in nav {
            let selected: Vec<_> = group
                .items
                .iter()
                .filter_map(|page| entries.remove(page.route.as_str()))
                .collect();
            if selected.is_empty() {
                continue;
            }
            output.push_str(&format!(
                "## {}\n\n",
                markdown(&clean(&group.label).unwrap_or_default())
            ));
            for entry in &selected {
                item(&mut output, project, catalog, entry);
            }
            output.push('\n');
        }
        if !entries.is_empty() {
            output.push_str(&format!(
                "## {}\n\n",
                markdown(&clean(&catalog.get("ai.other_pages").text).unwrap_or_default())
            ));
            for entry in entries.values() {
                item(&mut output, project, catalog, entry);
            }
        }
        let prefix = analysis
            .locales
            .get(lang)
            .map_or("", |view| view.root_route.trim_matches('/'));
        let path = if prefix.is_empty() {
            "llms.txt".into()
        } else {
            format!("{prefix}/llms.txt")
        };
        files.insert(path, Arc::from(output.into_bytes()));
    }
    if problems.is_empty() {
        Ok(files)
    } else {
        problems.sort();
        problems.dedup();
        Err(problems)
    }
}
