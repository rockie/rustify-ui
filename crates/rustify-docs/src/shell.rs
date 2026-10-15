//! The default theme's page, rendered to HTML on the host with Leptos.
//!
//! A page is complete before any script runs: the navigation, the table of
//! contents and the article are links and text, the theme follows the
//! system through CSS, and the narrow-screen menu is a native `<details>`.
//! The framework's small script only adds what needs one (the mode switch,
//! copying, code groups, the active heading). No wasm is ever loaded.

use leptos::prelude::*;
use leptos::tachys::html::attribute::custom::CustomAttribute;
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Arc;

use crate::escape::escape_html_into;
use crate::labels::{Catalog, LabelText};
use crate::markdown::Heading;
use crate::site::Slot;

/// Registered layout content for one final browser document.
pub type LayoutSlots = BTreeMap<Slot, String>;

fn slot_marker(slot: Slot) -> &'static str {
    match slot {
        Slot::Header => "<!--rustify-docs:slot:header-->",
        Slot::Footer => "<!--rustify-docs:slot:footer-->",
        Slot::SidebarTop => "<!--rustify-docs:slot:sidebar-top-->",
        Slot::SidebarBottom => "<!--rustify-docs:slot:sidebar-bottom-->",
        Slot::BeforeContent => "<!--rustify-docs:slot:before-content-->",
        Slot::AfterContent => "<!--rustify-docs:slot:after-content-->",
        Slot::NotFound => "<!--rustify-docs:slot:not-found-->",
    }
}

fn slot_view(slot: Slot, slots: &LayoutSlots) -> Option<impl IntoView> {
    slots.contains_key(&slot).then(|| {
        view! { <div class="rd-layout-slot" inner_html=slot_marker(slot)></div> }
    })
}

/// The strict policy every page is served under, as a header. The `meta`
/// copy in each page omits `frame-ancestors`, which only a header can carry.
pub const CSP: &str = "default-src 'none'; script-src 'self' 'wasm-unsafe-eval'; style-src 'self'; img-src 'self' data:; font-src 'self'; connect-src 'self'; media-src 'self'; worker-src 'self'; object-src 'none'; base-uri 'none'; form-action 'none'; frame-ancestors 'none'";

/// [`CSP`] without the one directive a `meta` element cannot deliver.
pub fn meta_csp() -> String {
    CSP.split("; ")
        .filter(|directive| !directive.starts_with("frame-ancestors"))
        .collect::<Vec<_>>()
        .join("; ")
}

/// One sidebar entry.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NavItem {
    pub title: String,
    /// The page's route, relative to the site's base: `/guide/setup/`.
    pub route: String,
    pub draft: bool,
    pub lang: String,
}

/// A titled group of sidebar entries, shown on the pages under `prefix`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NavGroup {
    pub label: String,
    pub prefix: String,
    pub items: Vec<NavItem>,
    pub lang: String,
}

/// Where `dev` serves what it adds to every page: the problem panel's
/// script and style, and the stream of build results. Outside the site's
/// base, and never part of a production build.
pub const DEV_PREFIX: &str = "/__rustify-docs/";
pub const DEV_SCRIPT_URL: &str = "/__rustify-docs/dev.js";
pub const DEV_STYLE_URL: &str = "/__rustify-docs/dev.css";

/// A link the configuration adds to the header: one of `nav.top`, or of
/// `nav.social` after them.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HeaderLink {
    pub label: String,
    /// Resolved: under the site's base, or another site's.
    pub url: String,
    /// The route of the page it links to, if it links to one. A reader on
    /// that page, or on a page under it, is shown that they are there.
    pub route: Option<String>,
    /// Another site's address, which gets `rel="noopener"`.
    pub external: bool,
    pub lang: String,
}

/// Where a build put the shared files, already joined with the base.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AssetUrls {
    pub stylesheet: String,
    /// Runs in `<head>`, before first paint, to apply a stored mode.
    pub mode_script: String,
    pub site_script: String,
    /// Pages served by `dev`: they load its panel and follow its builds.
    pub dev: bool,
}

/// What every page of a site shares.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Site {
    pub name: String,
    pub lang: String,
    pub home_route: String,
    pub catalog: Arc<Catalog>,
    /// `/` or `/segments/`.
    pub base: String,
    pub nav: Vec<NavGroup>,
    /// The header's links (`nav.top`) and the social ones (`nav.social`).
    pub top: Vec<HeaderLink>,
    pub social: Vec<HeaderLink>,
    pub assets: AssetUrls,
    pub logo: Option<Logo>,
}

/// The site's logo: its URL in light and in dark mode (the same twice for
/// one image), and its width at the header's 24 px height when known, so
/// the header keeps its layout before the image loads.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Logo {
    pub light: String,
    pub dark: String,
    pub width: Option<u32>,
}

impl Site {
    /// `route` under the site's base, as a root-relative URL with its path
    /// percent-encoded.
    pub fn url(&self, route: &str) -> String {
        let base = self.base.trim_end_matches('/');
        crate::escape::encode_path(&format!("{base}{route}"))
    }

    /// The groups shown beside `route`.
    pub fn groups_for<'a>(&'a self, route: &'a str) -> impl Iterator<Item = &'a NavGroup> + 'a {
        self.nav
            .iter()
            .filter(move |group| route.starts_with(&group.prefix))
    }

    /// The pages before and after `route` in sidebar order.
    pub fn neighbours<'a>(&'a self, route: &'a str) -> (Option<&'a NavItem>, Option<&'a NavItem>) {
        let items: Vec<&NavItem> = self
            .groups_for(route)
            .flat_map(|group| &group.items)
            .collect();
        match items.iter().position(|item| item.route == route) {
            Some(index) => (
                index.checked_sub(1).map(|i| items[i]),
                items.get(index + 1).copied(),
            ),
            None => (None, None),
        }
    }
}

/// What a page tells search engines and link previews.
#[derive(Clone, Copy, Debug, Default)]
pub struct Meta<'a> {
    /// The page's absolute address, when the site's URL is known.
    pub canonical: Option<&'a str>,
    /// The absolute address of the image a link preview shows.
    pub image: Option<&'a str>,
    /// Ask search engines not to index the page.
    pub noindex: bool,
    /// The site's home page, a `website` rather than an `article`.
    pub home: bool,
    /// When the page's file last changed, as `YYYY-MM-DD`.
    pub updated: Option<&'a str>,
    /// Where the page's source can be edited.
    pub edit_url: Option<&'a str>,
    pub alternates: &'a [AlternateLink],
}

#[derive(Clone, Debug)]
pub struct AlternateLink {
    pub lang: String,
    pub href: String,
}

#[derive(Clone, Debug)]
pub struct LanguageLink {
    pub lang: String,
    pub label: String,
    pub href: String,
    pub is_fallback: bool,
    pub fallback_label: LabelText,
    pub anchors: Vec<String>,
}

/// One page to render.
pub struct Page<'a> {
    pub route: &'a str,
    /// The document's `<title>`, unique within its configured language.
    pub document_title: &'a str,
    pub title: &'a str,
    pub description: Option<&'a str>,
    pub lang: &'a str,
    pub source_lang: &'a str,
    pub language_links: &'a [LanguageLink],
    pub draft: bool,
    /// The article's HTML, with every URL already resolved.
    pub article: &'a str,
    pub headings: &'a [Heading],
    /// The article has a level-one heading of its own; otherwise the title
    /// becomes the page's one.
    pub article_has_h1: bool,
    /// The heading levels the table of contents lists.
    pub toc_levels: (u8, u8),
    pub meta: Meta<'a>,
    /// A page with `layout: home`.
    pub home: Option<Home<'a>>,
}

/// What a home page shows above its own Markdown.
#[derive(Clone, Debug, Default)]
pub struct Home<'a> {
    /// A registered root renderer supplies the complete home composition.
    pub custom: bool,
    /// The existing anchor of the leading author H1 merged into the hero.
    pub heading_id: Option<&'a str>,
    pub tagline: Option<&'a str>,
    /// Each action's text and its resolved URL; the first is the primary.
    pub actions: Vec<(&'a str, String)>,
    /// Each feature's title and one-sentence details.
    pub features: Vec<(&'a str, &'a str)>,
}

/// The page served for an address with nothing at it: the site's shell, one
/// heading and a way home. Its links and files are root-relative under the
/// base, so it works from any depth.
pub fn render_not_found(site: &Site) -> String {
    render_not_found_with_slots(site, &LayoutSlots::new())
}

/// Renders a not-found page with the same overrides as ordinary pages.
pub fn render_not_found_with_slots(site: &Site, slots: &LayoutSlots) -> String {
    render_not_found_with_languages(site, slots, &[])
}

/// An error document with ordinary links to each configured language's home.
pub fn render_not_found_with_languages(
    site: &Site,
    slots: &LayoutSlots,
    homes: &[LanguageLink],
) -> String {
    let labels = &site.catalog;
    let mut article = format!(
        "<p lang=\"{}\">",
        crate::escape::escape_html(&labels.get("not_found_body").lang)
    );
    crate::escape::escape_html_into(&mut article, &labels.get("not_found_body").text);
    article.push_str("</p>\n<p><a href=\"");
    crate::escape::escape_html_into(&mut article, &site.url(&site.home_route));
    article.push_str("\" lang=\"");
    crate::escape::escape_html_into(&mut article, &labels.get("back_home").lang);
    article.push_str("\">");
    crate::escape::escape_html_into(&mut article, &labels.get("back_home").text);
    article.push_str("</a></p>\n");
    let document_title = format!(
        "{} | {}",
        labels.get("not_found_title").text.clone(),
        site.name
    );
    let article = slots
        .get(&Slot::NotFound)
        .map_or(article.as_str(), String::as_str);
    let mut article = article.to_owned();
    if !homes.is_empty() {
        let label = labels.get("available_languages");
        article.push_str("<nav class=\"rd-available-languages\" aria-label=\"");
        escape_html_into(&mut article, &label.text);
        article.push_str("\" lang=\"");
        escape_html_into(&mut article, &label.lang);
        article.push_str("\"><p>");
        escape_html_into(&mut article, &label.text);
        article.push_str("</p><ul>");
        for home in homes {
            article.push_str("<li><a href=\"");
            escape_html_into(&mut article, &home.href);
            article.push_str("\" hreflang=\"");
            escape_html_into(&mut article, &home.lang);
            article.push_str("\" lang=\"");
            escape_html_into(&mut article, &home.lang);
            article.push_str("\">");
            escape_html_into(&mut article, &home.label);
            article.push_str("</a></li>");
        }
        article.push_str("</ul></nav>");
    }
    let route = format!("{}404/", site.home_route);
    render_page_with_slots(
        site,
        &Page {
            route: &route,
            document_title: &document_title,
            title: &labels.get("not_found_title").text,
            description: Some(&labels.get("not_found_body").text),
            lang: &site.lang,
            source_lang: &labels.get("not_found_title").lang,
            language_links: &[],
            draft: false,
            article: &article,
            headings: &[],
            article_has_h1: false,
            toc_levels: (2, 3),
            meta: Meta {
                noindex: true,
                ..Meta::default()
            },
            home: None,
        },
        slots,
    )
}

/// What `dev` serves while no version of the site has built: the site's
/// shell with one heading saying so. The problems themselves are listed by
/// the panel the page loads.
pub fn render_dev_shell(site: &Site) -> String {
    render_dev_shell_with_status(site, false)
}

/// The same static shell while the first build is still running.
pub fn render_dev_shell_with_status(site: &Site, building: bool) -> String {
    let labels = &site.catalog;
    let (title, body) = if building {
        (
            labels.get("dev.building_title").text.as_str(),
            labels.get("dev.building_body").text.as_str(),
        )
    } else {
        (
            labels.get("dev.shell_title").text.as_str(),
            labels.get("dev.shell_body").text.as_str(),
        )
    };
    let body_lang = &labels
        .get(if building {
            "dev.building_body"
        } else {
            "dev.shell_body"
        })
        .lang;
    let mut article = format!("<p lang=\"{}\">", crate::escape::escape_html(body_lang));
    crate::escape::escape_html_into(&mut article, body);
    article.push_str("</p>\n");
    let document_title = format!("{} | {}", title, site.name);
    render_page(
        site,
        &Page {
            route: DEV_PREFIX,
            document_title: &document_title,
            title,
            description: Some(body),
            lang: &site.lang,
            source_lang: &labels
                .get(if building {
                    "dev.building_title"
                } else {
                    "dev.shell_title"
                })
                .lang,
            language_links: &[],
            draft: false,
            article: &article,
            headings: &[],
            article_has_h1: false,
            toc_levels: (2, 3),
            meta: Meta {
                noindex: true,
                ..Meta::default()
            },
            home: None,
        },
    )
}

/// Where the article goes in the rendered shell. Leptos writes the shell;
/// the article is spliced in afterwards, so it is not copied through the
/// renderer and the clean-up below never touches an author's markup.
const ARTICLE: &str = "<!--rustify-docs:article-->";

/// Renders `page` as a complete document.
pub fn render_page(site: &Site, page: &Page) -> String {
    render_page_with_slots(site, page, &LayoutSlots::new())
}

/// Inserts registered content after the framework shell's marker cleanup.
pub fn render_page_with_slots(site: &Site, page: &Page, slots: &LayoutSlots) -> String {
    let owner = Owner::new();
    let shell = owner.with(|| page_view(site, page, slots).to_html());
    drop(owner);
    // `<!>` is a hydration marker between dynamic pieces. Nothing hydrates a
    // static page, and author text cannot produce one here: the renderer
    // escapes every `<` it writes as text.
    let shell = shell.replace("<!>", "");
    let mut article = std::borrow::Cow::Borrowed(page.article);
    if is_fallback(page) && page.article_has_h1 {
        if let Some(cut) = crate::html_check::leading_h1_end(page.article) {
            let note = fallback_notice(&site.catalog).to_html().replace("<!>", "");
            article.to_mut().insert_str(cut, &note);
        }
    }
    let mut replacements = vec![(ARTICLE, article.as_ref())];
    replacements.extend(
        slots
            .iter()
            .map(|(slot, html)| (slot_marker(*slot), html.as_str())),
    );
    // Locate every marker before inserting author content. Neither literal
    // comments in code samples nor another slot can become replacement input.
    let mut ranges: Vec<_> = replacements
        .iter()
        .flat_map(|(marker, html)| {
            shell
                .match_indices(*marker)
                .map(move |(start, _)| (start, start + marker.len(), *html))
        })
        .collect();
    ranges.sort_by_key(|(start, _, _)| *start);
    let mut document = String::with_capacity(16 + shell.len() + page.article.len());
    document.push_str("<!doctype html>");
    let mut at = 0;
    for (start, end, html) in ranges {
        document.push_str(&shell[at..start]);
        document.push_str(html);
        at = end;
    }
    document.push_str(&shell[at..]);
    document
}

fn main_id(page: &Page, slots: &LayoutSlots) -> String {
    let mut occupied: BTreeSet<String> = crate::html_check::ids(page.article).into_iter().collect();
    for (slot, html) in slots {
        // NotFound is already the article; home layouts have no sidebar.
        if *slot == Slot::NotFound
            || (page.home.is_some() && matches!(slot, Slot::SidebarTop | Slot::SidebarBottom))
        {
            continue;
        }
        occupied.extend(crate::html_check::ids(html));
    }
    if let Some(id) = page.home.as_ref().and_then(|home| home.heading_id) {
        occupied.insert(id.to_owned());
    }
    let mut id = "content".to_owned();
    let mut suffix = 0;
    while occupied.contains(&id) {
        suffix += 1;
        id = format!("content-{suffix}");
    }
    id
}

fn page_view(site: &Site, page: &Page, slots: &LayoutSlots) -> impl IntoView {
    let labels = &site.catalog;
    let document_title = page.document_title.to_owned();
    let description = page.description.unwrap_or_default().to_owned();
    let meta = page.meta;
    let card = if meta.image.is_some() {
        "summary_large_image"
    } else {
        "summary"
    };
    let toc: Vec<&Heading> = page
        .headings
        .iter()
        .filter(|heading| {
            !heading.id.is_empty()
                && heading.level >= page.toc_levels.0
                && heading.level <= page.toc_levels.1
        })
        .collect();
    let main_id = main_id(page, slots);
    let body = match &page.home {
        Some(home) => home_layout(page, home, labels, slots, &main_id).into_any(),
        None => doc_layout(site, page, &toc, labels, slots, &main_id).into_any(),
    };
    view! {
        <html lang=page.lang.to_owned() data-rustify-scope="" data-theme="system" data-rd-base=site.base.clone()>
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <meta http-equiv="Content-Security-Policy" content=meta_csp() />
                <link rel="icon" href="data:image/svg+xml,%3Csvg%20xmlns=%22http://www.w3.org/2000/svg%22/%3E" />
                <title>{document_title}</title>
                <meta name="description" content=description.clone() />
                {meta.noindex.then(|| view! { <meta name="robots" content="noindex" /> })}
                {meta.canonical.map(|url| view! { <link rel="canonical" href=url.to_owned() /> })}
                {meta.alternates.iter().map(|link| view! {
                    <link rel="alternate" hreflang=link.lang.clone() href=link.href.clone() />
                }).collect_view()}
                {open_graph("og:type", if meta.home { "website" } else { "article" }.to_owned())}
                {open_graph("og:site_name", site.name.clone())}
                {open_graph("og:title", page.title.to_owned())}
                {open_graph("og:description", description.clone())}
                {meta.canonical.map(|url| open_graph("og:url", url.to_owned()))}
                {meta.image.map(|url| open_graph("og:image", url.to_owned()))}
                <meta name="twitter:card" content=card />
                <meta name="twitter:title" content=page.title.to_owned() />
                <meta name="twitter:description" content=description />
                {meta.image.map(|url| view! { <meta name="twitter:image" content=url.to_owned() /> })}
                <link rel="stylesheet" href=site.assets.stylesheet.clone() />
                {site.assets.dev.then(|| view! { <link rel="stylesheet" href=DEV_STYLE_URL /> })}
                <script src=site.assets.mode_script.clone()></script>
                <script src=site.assets.site_script.clone() defer=""></script>
                {site.assets.dev.then(|| {
                    let words: BTreeMap<_, _> = labels.iter().filter_map(|(key, value)| key.strip_prefix("dev.").map(|key| (key, &value.text))).collect();
                    let languages: BTreeMap<_, _> = labels.iter().filter_map(|(key, value)| key.strip_prefix("dev.").map(|key| (key, &value.lang))).collect();
                    let words = serde_json::to_string(&words).unwrap_or_default();
                    let languages = serde_json::to_string(&languages).unwrap_or_default();
                    view! {
                        <script src=DEV_SCRIPT_URL defer="" data-labels=words data-lang=page.lang.to_owned() data-label-langs=languages></script>
                    }
                })}
            </head>
            <body class="min-h-screen bg-background text-foreground antialiased">
                <a
                    href=format!("#{main_id}")
                    class="sr-only focus:not-sr-only focus:fixed focus:top-2 focus:left-2 focus:z-50 focus:rounded-md focus:bg-background focus:px-3 focus:py-2"
                    lang=labels.get("skip_to_content").lang.clone()
                >
                    {labels.get("skip_to_content").text.clone()}
                </a>
                {header(site, page, labels, slots)}
                {body}
                {search_dialog(site, labels)}
                <footer class="rd-site-footer mx-auto max-w-[76rem] px-4 md:px-6">
                    {if slots.contains_key(&Slot::Footer) {
                        view! { <div inner_html=slot_marker(Slot::Footer)></div> }.into_any()
                    } else {
                        view! { <nav aria-label=labels.get("footer_links").text.clone() lang=labels.get("footer_links").lang.clone()>
                            <a href=site.url(&site.home_route) lang=labels.get("back_home").lang.clone()>{labels.get("back_home").text.clone()}</a>
                        </nav> }.into_any()
                    }}
                </footer>
                <p
                    id="rd-status"
                    class="sr-only"
                    aria-live="polite"
                    data-copied=labels.get("copied").text.clone()
                    data-copy-failed=labels.get("copy_failed").text.clone()
                    data-copy-failed-short=labels.get("copy_failed_short").text.clone()
                    data-copied-lang=labels.get("copied").lang.clone()
                    data-copy-failed-lang=labels.get("copy_failed").lang.clone()
                    data-copy-failed-short-lang=labels.get("copy_failed_short").lang.clone()
                    lang=page.lang.to_owned()
                ></p>
            </body>
        </html>
    }
}

fn fallback_note(page: &Page, labels: &Catalog) -> Option<impl IntoView> {
    (is_fallback(page)
        && !(page.article_has_h1 && crate::html_check::leading_h1_end(page.article).is_some()))
    .then(|| fallback_notice(labels))
}

fn is_fallback(page: &Page) -> bool {
    page.language_links
        .iter()
        .any(|link| link.lang == page.lang && link.is_fallback)
}

fn fallback_notice(labels: &Catalog) -> impl IntoView {
    view! {
        <p class="rd-untranslated" role="note" lang=labels.get("untranslated").lang.clone()>
            {labels.get("untranslated").text.clone()}
        </p>
    }
}

fn language_switcher(page: &Page, labels: &Catalog) -> Option<impl IntoView> {
    let current = page
        .language_links
        .iter()
        .find(|link| link.lang == page.lang)?;
    (page.language_links.len() > 1).then(|| view! {
        <details class="rd-language-switch" data-rd-language-switch="">
            <summary lang=current.lang.clone()>{current.label.clone()}</summary>
            <nav aria-label=labels.get("language").text.clone() lang=labels.get("language").lang.clone()><ul>
                {page.language_links.iter().map(|link| view! {
                    <li><a href=link.href.clone() hreflang=link.lang.clone() lang=link.lang.clone()
                        aria-current=(link.lang == page.lang).then_some("page")
                        data-rd-language-anchors=serde_json::to_string(&link.anchors).unwrap_or_default()>
                        {link.label.clone()}
                        {link.is_fallback.then(|| view! {
                            <span class="rd-language-fallback" lang=link.fallback_label.lang.clone()>
                                " · "{link.fallback_label.text.clone()}
                            </span>
                        })}
                    </a></li>
                }).collect_view()}
            </ul></nav>
        </details>
    })
}

/// A content page: the sidebar, the table of contents and the article with
/// its footer and the previous/next links.
fn doc_layout(
    site: &Site,
    page: &Page,
    toc: &[&Heading],
    labels: &Catalog,
    slots: &LayoutSlots,
    main_id: &str,
) -> impl IntoView {
    let (previous, next) = site.neighbours(page.route);
    // The article's one H1 is the content's own when it has one.
    let heading = (!page.article_has_h1).then(|| page.title.to_owned());
    view! {
        <div class="rd-layout mx-auto grid max-w-[76rem] gap-x-10 px-4 md:px-6">
            {sidebar(site, page.route, labels, slots)}
            {table_of_contents(toc, page.source_lang, labels)}
            <main id=main_id.to_owned() class="rd-main min-w-0 py-8 md:py-12">
                <div class="rd-column">
                    {page.draft.then(|| view! {
                        <p class="rd-draft" lang=labels.get("draft").lang.clone()>{labels.get("draft").text.clone()}</p>
                    })}
                    {heading.map(|title| view! { <h1 class="rd-title" lang=page.source_lang.to_owned()>{title}</h1> })}
                    {fallback_note(page, labels)}
                    {slot_view(Slot::BeforeContent, slots)}
                    <article class="rd-prose" lang=page.source_lang.to_owned() inner_html=ARTICLE></article>
                    {slot_view(Slot::AfterContent, slots)}
                    {page_footer(page.meta, labels)}
                    {pager(site, previous, next, labels)}
                </div>
            </main>
        </div>
    }
}

/// The home page: who the site is for, where to start, what it offers, then
/// whatever the page's Markdown adds. No sidebar: it is the way in.
fn home_layout(
    page: &Page,
    home: &Home,
    labels: &Catalog,
    slots: &LayoutSlots,
    main_id: &str,
) -> impl IntoView {
    if home.custom {
        return view! {
            <main id=main_id.to_owned() class="rd-home mx-auto max-w-[76rem] px-4 md:px-6">
                {slot_view(Slot::BeforeContent, slots)}
                {(!page.article_has_h1).then(|| view! { <h1 class="rd-home-title" lang=page.source_lang.to_owned()>{page.title.to_owned()}</h1> })}
                {fallback_note(page, labels)}
                <div lang=page.source_lang.to_owned() inner_html=ARTICLE></div>
                {slot_view(Slot::AfterContent, slots)}
            </main>
        }.into_any();
    }
    let actions = home
        .actions
        .iter()
        .enumerate()
        .map(|(i, (text, url))| {
            let class = if i == 0 {
                "rd-action rd-action-primary"
            } else {
                "rd-action rd-action-secondary"
            };
            view! { <a href=url.clone() class=class lang=page.source_lang.to_owned()>{text.to_string()}</a> }
        })
        .collect_view();
    let features = (!home.features.is_empty()).then(|| {
        let items = home
            .features
            .iter()
            .map(|(title, details)| {
                view! {
                    <li>
                        <h2>{title.to_string()}</h2>
                        <p>{details.to_string()}</p>
                    </li>
                }
            })
            .collect_view();
        // Two or four go in pairs, any other count in threes, so the
        // hairline rows never end in an empty column.
        let columns = match home.features.len() {
            1 => "1",
            2 | 4 => "2",
            _ => "3",
        };
        view! { <ul class="rd-features" data-columns=columns lang=page.source_lang.to_owned()>{items}</ul> }
    });
    view! {
        <main id=main_id.to_owned() class="rd-home mx-auto max-w-[76rem] px-4 md:px-6">
            <div class="rd-hero" lang=page.source_lang.to_owned()>
                <h1 class="rd-home-title" lang=page.source_lang.to_owned() id=home.heading_id.map(str::to_owned)>{page.title.to_owned()}</h1>
                {home.tagline.map(|tagline| view! { <p class="rd-tagline">{tagline.to_owned()}</p> })}
                {(!home.actions.is_empty()).then(|| view! { <nav class="rd-actions" aria-label=labels.get("start_reading").text.clone() lang=labels.get("start_reading").lang.clone()>{actions}</nav> })}
            </div>
            {fallback_note(page, labels)}
            {features}
            {slot_view(Slot::BeforeContent, slots)}
            <article class="rd-prose rd-home-prose" lang=page.source_lang.to_owned() inner_html=ARTICLE></article>
            {slot_view(Slot::AfterContent, slots)}
        </main>
    }.into_any()
}

/// An Open Graph `<meta property=… content=…>`; `property` is not an
/// attribute the view macro knows.
fn open_graph(property: &'static str, content: String) -> impl IntoView {
    leptos::html::meta()
        .attr("property", property)
        .attr("content", content)
}

/// The configuration's header links as HTML, the current one marked: the
/// page it links to (`page`), or the section a page under it belongs to.
fn header_entries(site: &Site, route: &str) -> String {
    let mut html = String::new();
    for (class, links) in [
        ("rd-header-links", &site.top),
        ("rd-header-social", &site.social),
    ] {
        if links.is_empty() {
            continue;
        }
        html.push_str(&format!("<ul class=\"{class}\">"));
        for link in links {
            html.push_str("<li><a href=\"");
            escape_html_into(&mut html, &link.url);
            html.push_str("\" lang=\"");
            escape_html_into(&mut html, &link.lang);
            html.push('"');
            match link.route.as_deref() {
                Some(target) if target == route => html.push_str(" aria-current=\"page\""),
                Some(target) if target != site.home_route && route.starts_with(target) => {
                    html.push_str(" aria-current=\"true\"")
                }
                _ => {}
            }
            if link.external {
                html.push_str(" rel=\"noopener\"");
            }
            html.push('>');
            escape_html_into(&mut html, &link.label);
            html.push_str("</a></li>");
        }
        html.push_str("</ul>");
    }
    html
}

/// The header bar, and the configuration's links: in the bar from 768 px,
/// and below that in a row of their own under it (one of the two is always
/// `display: none`), so a narrow screen never scrolls sideways for them.
fn header(site: &Site, page: &Page, labels: &Catalog, slots: &LayoutSlots) -> impl IntoView {
    let route = page.route;
    let at_home = route == site.home_route;
    let name = site.name.clone();
    let entries = header_entries(site, route);
    let links = (!entries.is_empty()).then(|| {
        view! { <nav class="rd-header-nav rd-header-nav-wide" aria-label=labels.get("site_links").text.clone() lang=labels.get("site_links").lang.clone() inner_html=entries.clone()></nav> }
    });
    let narrow = (!entries.is_empty()).then(|| {
        view! { <nav class="rd-header-nav rd-header-nav-narrow" aria-label=labels.get("site_links").text.clone() lang=labels.get("site_links").lang.clone() inner_html=entries></nav> }
    });
    // The name says what the logo shows, so the logo is decoration. A pair
    // switches with the mode through CSS, with or without script.
    let logo = site.logo.clone().map(|logo| {
        let width = logo.width.map(|width| width.to_string());
        if logo.light == logo.dark {
            view! { <img src=logo.light alt="" width=width height="24" class="h-6 w-auto" /> }.into_any()
        } else {
            view! {
                <img src=logo.light alt="" width=width.clone() height="24" class="h-6 w-auto dark:hidden" />
                <img src=logo.dark alt="" width=width height="24" class="hidden h-6 w-auto dark:block" />
            }
            .into_any()
        }
    });
    view! {
        <header class="sticky top-0 z-40 border-b border-secondary bg-background">
            <div class="rd-header-bar mx-auto flex max-w-[76rem] items-center gap-4 px-4 md:px-6">
                {if slots.contains_key(&Slot::Header) {
                    view! { <div class="rd-brand" inner_html=slot_marker(Slot::Header)></div> }.into_any()
                } else {
                    view! { <a href=site.url(&site.home_route) class="rd-brand" aria-current=at_home.then_some("page")>{logo}<span>{name}</span></a> }.into_any()
                }}
                {links}
                <button type="button" class="rd-search-open" data-rd-search-open="" disabled aria-haspopup="dialog" aria-controls="rd-search-dialog" aria-keyshortcuts="Control+k Meta+k" lang=labels.get("search.open").lang.clone()>
                    {labels.get("search.open").text.clone()}
                    <kbd class="rd-search-shortcut">"Ctrl / Cmd K"</kbd>
                </button>
                {language_switcher(page, labels)}
                <div class="rd-mode-switch" data-rd-mode-switch="" hidden="" role="group" aria-label=labels.get("theme").text.clone() lang=labels.get("theme").lang.clone()>
                    {mode_button("system", labels.get("theme_system"))}
                    {mode_button("light", labels.get("theme_light"))}
                    {mode_button("dark", labels.get("theme_dark"))}
                </div>
            </div>
        </header>
        {narrow}
    }
}

fn search_dialog(site: &Site, labels: &Catalog) -> impl IntoView {
    let words: BTreeMap<_, _> = labels
        .iter()
        .filter_map(|(key, value)| key.strip_prefix("search.").map(|key| (key, value)))
        .collect();
    view! {
        <noscript><p class="rd-search-noscript" lang=labels.get("search.no_script").lang.clone()>{labels.get("search.no_script").text.clone()}</p></noscript>
        <dialog id="rd-search-dialog" class="rd-search-dialog" data-rd-search="" data-state="closed"
            aria-label=labels.get("search.open").text.clone()
            lang=labels.get("search.open").lang.clone()
            data-rd-search-lang=site.lang.clone()
            data-rd-search-labels=serde_json::to_string(&words).unwrap_or_default()
            data-rd-search-index=crate::search::index_placeholder(&site.lang)
            data-rd-search-module=crate::search::MODULE_PLACEHOLDER
            data-rd-search-worker=crate::search::WORKER_PLACEHOLDER>
            <div class="rd-search-heading">
                <label for="rd-search-input" lang=labels.get("search.input").lang.clone()>{labels.get("search.input").text.clone()}</label>
                <button type="button" data-rd-search-close="" lang=labels.get("search.close").lang.clone()>{labels.get("search.close").text.clone()}</button>
            </div>
            <input id="rd-search-input" data-rd-search-input="" role="combobox" aria-controls="rd-search-results" aria-expanded="false" aria-autocomplete="list" autocomplete="off" autocapitalize="off" spellcheck="false" lang=labels.get("search.input").lang.clone() />
            <p class="rd-search-status" data-rd-search-status="" lang=labels.get("search.hint").lang.clone()>{labels.get("search.hint").text.clone()}</p>
            <p class="rd-search-count" data-rd-search-count="" aria-live="polite" aria-atomic="true"></p>
            <div id="rd-search-results" class="rd-search-results" data-rd-search-results="" role="listbox" aria-label=labels.get("search.results").text.clone() lang=labels.get("search.results").lang.clone()></div>
            <button type="button" class="rd-search-action" data-rd-search-retry="" hidden lang=labels.get("search.retry").lang.clone()>{labels.get("search.retry").text.clone()}</button>
            <button type="button" class="rd-search-action" data-rd-search-more="" hidden lang=labels.get("search.more").lang.clone()>{labels.get("search.more").text.clone()}</button>
        </dialog>
    }
}

fn mode_button(mode: &'static str, label: &LabelText) -> impl IntoView {
    view! {
        <button type="button" class="rd-mode-option" data-rd-mode=mode aria-pressed="false" lang=label.lang.clone()>
            <span class="rd-mode-icon" inner_html=crate::icons::mode(mode)></span>
            <span class="rd-mode-label">{label.text.clone()}</span>
        </button>
    }
}

/// The sidebar's entries as HTML: hundreds on every page of a large site,
/// each a bare link styled by one `.rd-nav` rule instead of a class list
/// repeated per entry, written once and placed in both navigations.
fn sidebar_entries(site: &Site, route: &str, labels: &Catalog) -> String {
    let mut html = String::new();
    for group in site.groups_for(route) {
        html.push_str("<section><h2 lang=\"");
        escape_html_into(&mut html, &group.lang);
        html.push_str("\">");
        escape_html_into(&mut html, &group.label);
        html.push_str("</h2><ul>");
        for item in &group.items {
            html.push_str("<li><a href=\"");
            escape_html_into(&mut html, &site.url(&item.route));
            html.push_str("\" lang=\"");
            escape_html_into(&mut html, &item.lang);
            html.push('"');
            if item.route == route {
                html.push_str(" aria-current=\"page\"");
            }
            html.push('>');
            escape_html_into(&mut html, &item.title);
            if item.draft {
                html.push_str("<span lang=\"");
                escape_html_into(&mut html, &labels.get("draft").lang);
                html.push_str("\"> · ");
                escape_html_into(&mut html, &labels.get("draft").text);
                html.push_str("</span>");
            }
            html.push_str("</a></li>");
        }
        html.push_str("</ul></section>");
    }
    html
}

/// The site's navigation, twice: a plain column from 768 px, and on a
/// narrower screen a native <details> menu, closed until opened, that works
/// without script. Only one is displayed at a width; the other is
/// `display: none`, so focus and assistive technology meet one navigation.
/// (A closed <details> shown by CSS alone is not in WebKit's accessibility
/// tree, so the wide column cannot be one.)
fn sidebar(site: &Site, route: &str, labels: &Catalog, slots: &LayoutSlots) -> impl IntoView {
    let entries = sidebar_entries(site, route, labels);
    view! {
        <div class="rd-sidebar">
            {slot_view(Slot::SidebarTop, slots)}
            <nav class="rd-nav rd-sidebar-wide" aria-label=labels.get("menu").text.clone() lang=labels.get("menu").lang.clone() inner_html=entries.clone()></nav>
            <details class="rd-sidebar-narrow">
                <summary lang=labels.get("menu").lang.clone()>{labels.get("menu").text.clone()}</summary>
                <nav class="rd-nav" aria-label=labels.get("menu").text.clone() lang=labels.get("menu").lang.clone() inner_html=entries></nav>
            </details>
            {slot_view(Slot::SidebarBottom, slots)}
        </div>
    }
}

/// The page's headings as list items for the table of contents.
fn toc_entries(toc: &[&Heading]) -> String {
    let mut html = String::new();
    for heading in toc {
        html.push_str("<li");
        if heading.level > 2 {
            html.push_str(" class=\"rd-toc-sub\"");
        }
        html.push_str("><a href=\"#");
        escape_html_into(&mut html, &crate::escape::encode_fragment(&heading.id));
        html.push_str("\" data-rd-toc-link=\"\">");
        escape_html_into(&mut html, &heading.text);
        html.push_str("</a></li>");
    }
    html
}

/// The table of contents, twice for the same reason as the sidebar: a
/// column from 1280 px, a <details> panel above the article below that.
fn table_of_contents(toc: &[&Heading], source_lang: &str, labels: &Catalog) -> impl IntoView {
    (!toc.is_empty()).then(|| {
        let entries = toc_entries(toc);
        view! {
            <div class="rd-toc">
                <nav class="rd-toc-wide" aria-label=labels.get("on_this_page").text.clone() lang=labels.get("on_this_page").lang.clone()>
                    <p>{labels.get("on_this_page").text.clone()}</p>
                    <ul lang=source_lang.to_owned() inner_html=entries.clone()></ul>
                </nav>
                <details class="rd-toc-narrow">
                    <summary lang=labels.get("on_this_page").lang.clone()>{labels.get("on_this_page").text.clone()}</summary>
                    <nav aria-label=labels.get("on_this_page").text.clone() lang=labels.get("on_this_page").lang.clone()>
                        <ul lang=source_lang.to_owned() inner_html=entries></ul>
                    </nav>
                </details>
            </div>
        }
    })
}

fn page_footer(meta: Meta, labels: &Catalog) -> impl IntoView {
    let edit = meta.edit_url.map(|url| {
        view! { <a href=url.to_owned() rel="noopener" lang=labels.get("edit_page").lang.clone()>{labels.get("edit_page").text.clone()}</a> }
    });
    let updated = meta.updated.map(|date| {
        view! {
            <p>
                <span lang=labels.get("last_updated").lang.clone()>{labels.get("last_updated").text.clone()}</span>
                " "
                <time datetime=date.to_owned()>{date.to_owned()}</time>
            </p>
        }
    });
    (edit.is_some() || updated.is_some()).then(|| {
        view! {
            <footer class="rd-page-meta">
                {edit}
                {updated}
            </footer>
        }
    })
}

fn pager(
    site: &Site,
    previous: Option<&NavItem>,
    next: Option<&NavItem>,
    labels: &Catalog,
) -> impl IntoView {
    let link = |item: &NavItem, label: &LabelText, is_next: bool| {
        view! {
            <a href=site.url(&item.route) rel=if is_next { "next" } else { "prev" } data-rd-next=is_next.then_some("")>
                <span class="rd-pager-label" lang=label.lang.clone()>{label.text.clone()}</span>
                <span class="rd-pager-title" lang=item.lang.clone()>{item.title.clone()}</span>
            </a>
        }
    };
    (previous.is_some() || next.is_some()).then(|| {
        view! {
            <nav class="rd-pager" aria-label=format!("{} / {}", labels.get("previous").text.clone(), labels.get("next").text.clone())>
                {previous.map(|item| link(item, labels.get("previous"), false))}
                {next.map(|item| link(item, labels.get("next"), true))}
            </nav>
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::highlight::Highlighter;
    use crate::labels;
    use crate::markdown;

    fn site() -> Site {
        Site {
            name: "Docs".into(),
            lang: "en".into(),
            home_route: "/".into(),
            catalog: Arc::new(labels::resolve("en", &BTreeMap::new())),
            base: "/rustify-ui/".into(),
            nav: vec![NavGroup {
                lang: "en".into(),
                label: "Guide".into(),
                prefix: "/".into(),
                items: vec![
                    NavItem {
                        lang: "en".into(),
                        title: "Intro".into(),
                        route: "/".into(),
                        draft: false,
                    },
                    NavItem {
                        lang: "en".into(),
                        title: "Setup".into(),
                        route: "/setup/".into(),
                        draft: false,
                    },
                    NavItem {
                        lang: "en".into(),
                        title: "Next steps".into(),
                        route: "/next/".into(),
                        draft: true,
                    },
                ],
            }],
            top: Vec::new(),
            social: Vec::new(),
            assets: AssetUrls {
                stylesheet: "/rustify-ui/assets/site.css".into(),
                mode_script: "/rustify-ui/assets/mode.js".into(),
                site_script: "/rustify-ui/assets/site.js".into(),
                dev: false,
            },
            logo: None,
        }
    }

    #[test]
    fn localized_home_is_not_a_current_section() {
        let mut site = site();
        site.home_route = "/fr/".into();
        for (label, route) in [("Accueil", "/fr/"), ("Guide", "/fr/guide/")] {
            site.top.push(HeaderLink {
                lang: "fr".into(),
                label: label.into(),
                url: site.url(route),
                route: Some(route.into()),
                external: false,
            });
        }
        let html = header_entries(&site, "/fr/guide/setup/");
        let document = dom_query::Document::from(html.as_str());
        assert!(document
            .select("a[href='/rustify-ui/fr/']")
            .attr("aria-current")
            .is_none());
        assert_eq!(
            document
                .select("a[href='/rustify-ui/fr/guide/']")
                .attr("aria-current")
                .as_deref(),
            Some("true")
        );
    }

    #[test]
    fn language_switcher_and_fallback_body_keep_each_words_language() {
        let mut site = site();
        site.lang = "fr".into();
        site.home_route = "/fr/".into();
        site.catalog = std::sync::Arc::new(labels::resolve(
            "fr",
            &BTreeMap::from([
                ("menu".into(), "Parcourir".into()),
                ("language".into(), "Langue".into()),
                ("untranslated".into(), "Traduction indisponible".into()),
            ]),
        ));
        let links = [
            LanguageLink {
                lang: "en".into(),
                label: "English".into(),
                href: site.url("/setup/"),
                is_fallback: false,
                anchors: vec!["setup".into()],
                fallback_label: labels::resolve("en", &BTreeMap::new())
                    .get("untranslated")
                    .clone(),
            },
            LanguageLink {
                lang: "fr".into(),
                label: "Français".into(),
                href: site.url("/fr/setup/"),
                is_fallback: true,
                anchors: vec!["setup".into()],
                fallback_label: site.catalog.get("untranslated").clone(),
            },
        ];
        let mut page = Page {
            route: "/fr/setup/",
            document_title: "Setup | Docs",
            title: "Setup",
            description: None,
            lang: "fr",
            source_lang: "en",
            language_links: &links,
            draft: false,
            article: "<p>English source</p>",
            headings: &[],
            article_has_h1: false,
            toc_levels: (2, 3),
            meta: Meta::default(),
            home: None,
        };
        let html = render_page(&site, &page);
        let doc = dom_query::Document::from(html.as_str());
        assert_eq!(doc.select("html").attr("lang").as_deref(), Some("fr"));
        assert_eq!(doc.select("article").attr("lang").as_deref(), Some("en"));
        assert_eq!(doc.select("h1").attr("lang").as_deref(), Some("en"));
        assert_eq!(
            doc.select(".rd-language-switch summary").text().as_ref(),
            "Français"
        );
        assert_eq!(
            doc.select("a[hreflang=fr]").attr("aria-current").as_deref(),
            Some("page")
        );
        assert_eq!(
            doc.select(".rd-untranslated").attr("lang").as_deref(),
            Some("fr")
        );
        assert_eq!(
            doc.select(".rd-sidebar-wide").attr("lang").as_deref(),
            Some("fr")
        );
        assert_eq!(
            doc.select(".rd-mode-switch").attr("lang").as_deref(),
            Some("en")
        );
        assert!(html.contains("href=\"/rustify-ui/fr/\""));
        page.article = "<h1>English title</h1><p>English body</p>";
        page.article_has_h1 = true;
        let html = render_page(&site, &page);
        assert!(html.find("</h1>").unwrap() < html.find("rd-untranslated").unwrap());
        assert!(html.find("rd-untranslated").unwrap() < html.find("<p>English body").unwrap());
    }

    fn render(markdown_source: &str, route: &str, lang: &str) -> String {
        let catalog = labels::resolve(lang, &std::collections::BTreeMap::new());
        let includes = markdown::IncludeFiles::default();
        let context = markdown::Context {
            highlighter: &Highlighter::new(),
            catalog: &catalog,
            source_lang: lang,
            includes: &includes,
        };
        let fragment = markdown::render(markdown_source, 0, &context);
        let mut site = site();
        site.lang = lang.into();
        site.catalog = Arc::new(catalog);
        render_page(
            &site,
            &Page {
                route,
                document_title: "Setup | Docs",
                title: "Setup",
                description: fragment.summary.as_deref(),
                lang,
                source_lang: lang,
                language_links: &[],
                draft: false,
                article: &fragment.html,
                headings: &fragment.headings,
                article_has_h1: fragment.h1_count > 0,
                toc_levels: (2, 3),
                meta: Meta::default(),
                home: None,
            },
        )
    }

    fn render_slots(article: &str, slots: &LayoutSlots) -> String {
        let mut site = site();
        site.top.push(HeaderLink {
            lang: "en".into(),
            label: "Examples".into(),
            url: site.url("/next/"),
            route: Some("/next/".into()),
            external: false,
        });
        render_page_with_slots(
            &site,
            &Page {
                route: "/setup/",
                document_title: "Setup | Docs",
                title: "Setup",
                description: None,
                lang: "en",
                source_lang: "en",
                language_links: &[],
                draft: false,
                article,
                headings: &[],
                article_has_h1: false,
                toc_levels: (2, 3),
                meta: Meta::default(),
                home: None,
            },
            slots,
        )
    }

    #[test]
    fn layout_slots_are_inserted_once_after_shell_cleanup() {
        let slots = LayoutSlots::from([
            (
                Slot::Header,
                "<a id=\"custom-brand\" href=\"/rustify-ui/\">Custom</a>".into(),
            ),
            (
                Slot::Footer,
                "<p id=\"custom-footer\">Footer<!>end</p>".into(),
            ),
            (
                Slot::SidebarTop,
                "<div id=\"sidebar-island\"><!>Island</div>".into(),
            ),
            (
                Slot::SidebarBottom,
                "<p id=\"sidebar-bottom\">More</p>".into(),
            ),
            (Slot::BeforeContent, "<p id=\"before\">Intro</p>".into()),
            (Slot::AfterContent, "<p id=\"after\">After</p>".into()),
        ]);
        let html = render_slots("<p id=\"body\">Body<!>end</p>", &slots);
        for id in [
            "custom-brand",
            "custom-footer",
            "sidebar-island",
            "sidebar-bottom",
            "before",
            "after",
            "body",
        ] {
            assert_eq!(
                html.matches(&format!("id=\"{id}\"")).count(),
                1,
                "{id}: {html}"
            );
        }
        assert!(html.contains("Footer<!>end"));
        assert!(html.contains("Body<!>end"));
        assert!(html.contains("<!>Island"));
        assert!(html.contains("data-rd-mode=\"system\""));
        assert!(html.contains("data-rd-mode=\"dark\""));
        assert!(html.contains("Examples</a>"));
        assert!(html.contains("Content-Security-Policy"));
    }

    #[test]
    fn author_slot_markers_cannot_trigger_another_slot_replacement() {
        let slots = LayoutSlots::from([
            (
                Slot::BeforeContent,
                "<div><!--rustify-docs:slot:footer--></div>".into(),
            ),
            (Slot::Footer, "<p id=\"real-footer\">Only one</p>".into()),
        ]);
        let html = render_slots("<!--rustify-docs:slot:footer--><p>Body</p>", &slots);
        assert_eq!(html.matches("id=\"real-footer\"").count(), 1);
        assert_eq!(html.matches(slot_marker(Slot::Footer)).count(), 2);
    }

    #[test]
    fn not_found_body_and_shared_layout_use_the_same_slots() {
        let slots = LayoutSlots::from([
            (
                Slot::NotFound,
                "<p id=\"custom-error\">Custom error<!>body</p>".into(),
            ),
            (Slot::Header, "<p id=\"error-header\">Header</p>".into()),
            (Slot::Footer, "<p id=\"error-footer\">Footer</p>".into()),
            (
                Slot::SidebarTop,
                "<p id=\"error-sidebar\">Sidebar</p>".into(),
            ),
            (
                Slot::BeforeContent,
                "<p id=\"error-before\">Before</p>".into(),
            ),
            (Slot::AfterContent, "<p id=\"error-after\">After</p>".into()),
        ]);
        let html = render_not_found_with_slots(&site(), &slots);
        for id in [
            "custom-error",
            "error-header",
            "error-footer",
            "error-sidebar",
            "error-before",
            "error-after",
        ] {
            assert_eq!(html.matches(&format!("id=\"{id}\"")).count(), 1);
        }
        assert!(html.contains("Custom error<!>body"));
        assert!(html.contains("<h1 lang=\"en\" class=\"rd-title\">Page not found</h1>"));
        assert!(html.contains("name=\"robots\" content=\"noindex\""));
    }

    #[test]
    fn a_page_is_a_complete_static_document() {
        let html = render(
            "## Install\n\nRun `cargo`.\n\n### Check\n\nDone.\n",
            "/setup/",
            "en",
        );
        assert!(html.starts_with("<!doctype html><html lang=\"en\" data-rustify-scope=\"\" data-theme=\"system\" data-rd-base=\"/rustify-ui/\"><head><meta charset=\"utf-8\">"), "{html}");
        assert!(html.contains("<meta http-equiv=\"Content-Security-Policy\" content=\"default-src 'none'; script-src 'self' 'wasm-unsafe-eval'; style-src 'self'; img-src 'self' data:; font-src 'self'; connect-src 'self'; media-src 'self'; worker-src 'self'; object-src 'none'; base-uri 'none'; form-action 'none'\">"));
        assert!(html.contains("<title>Setup | Docs</title>"));
        assert!(html.contains("<link rel=\"icon\" href=\"data:image/svg+xml,"));
        assert!(html.contains("<meta name=\"description\" content=\"Run cargo.\">"));
        // One H1, from the title, because the content has none.
        assert_eq!(html.matches("<h1").count(), 1);
        assert!(html.contains("<h2 id=\"install\">Install</h2>"));
        // The current page is marked; the others link under the base.
        assert!(
            html.contains(
                "<li><a href=\"/rustify-ui/setup/\" lang=\"en\" aria-current=\"page\">Setup</a></li>"
            ),
            "{html}"
        );
        assert!(html.contains("href=\"/rustify-ui/\""));
        assert!(html.contains("<a href=\"#install\""));
        assert!(html.contains("<a href=\"#check\""));
        // Neighbours in sidebar order.
        assert!(
            html.contains("<a href=\"/rustify-ui/\" rel=\"prev\">"),
            "{html}"
        );
        assert!(html.contains("<a href=\"/rustify-ui/next/\" rel=\"next\" data-rd-next=\"\">"));
        assert!(
            html.contains("Next steps<span lang=\"en\"> · Draft</span></a>"),
            "draft marker in the sidebar"
        );
        assert!(!html.contains("<!>"));
        assert!(!html.contains("style="));
        assert!(!html.contains("<script>"));
        assert!(!html.contains(".wasm"));
    }

    #[test]
    fn header_links_are_in_the_header_and_mark_where_the_reader_is() {
        let mut site = site();
        site.top = vec![
            HeaderLink {
                lang: "en".into(),
                label: "Setup".into(),
                url: "/rustify-ui/setup/".into(),
                route: Some("/setup/".into()),
                external: false,
            },
            HeaderLink {
                lang: "en".into(),
                label: "Home".into(),
                url: "/rustify-ui/".into(),
                route: Some("/".into()),
                external: false,
            },
        ];
        site.social = vec![HeaderLink {
            lang: "en".into(),
            label: "GitHub".into(),
            url: "https://github.com/example".into(),
            route: None,
            external: true,
        }];
        let page = |route: &str| {
            render_page(
                &site,
                &Page {
                    route,
                    document_title: "Setup | Docs",
                    title: "Setup",
                    description: None,
                    lang: "en",
                    source_lang: "en",
                    language_links: &[],
                    draft: false,
                    article: "<p>x</p>",
                    headings: &[],
                    article_has_h1: false,
                    toc_levels: (2, 3),
                    meta: Meta::default(),
                    home: None,
                },
            )
        };
        let html = page("/setup/install/");
        let links = "<ul class=\"rd-header-links\"><li><a href=\"/rustify-ui/setup/\" lang=\"en\" aria-current=\"true\">Setup</a></li><li><a href=\"/rustify-ui/\" lang=\"en\">Home</a></li></ul><ul class=\"rd-header-social\"><li><a href=\"https://github.com/example\" lang=\"en\" rel=\"noopener\">GitHub</a></li></ul>";
        // Once in the header bar, once in a row of its own under it.
        assert_eq!(html.matches(links).count(), 2, "{html}");
        assert!(
            html.contains(
                "<nav aria-label=\"Site links\" lang=\"en\" class=\"rd-header-nav rd-header-nav-wide\">"
            ),
            "{html}"
        );
        assert!(html.contains(
            "</header><nav aria-label=\"Site links\" lang=\"en\" class=\"rd-header-nav rd-header-nav-narrow\">"
        ));
        let html = page("/setup/");
        assert!(html.contains(
            "<a href=\"/rustify-ui/setup/\" lang=\"en\" aria-current=\"page\">Setup</a>"
        ));
        let html = page("/");
        assert!(
            html.contains("<a href=\"/rustify-ui/\" lang=\"en\" aria-current=\"page\">Home</a>")
        );
        assert!(html.contains("<a href=\"/rustify-ui/setup/\" lang=\"en\">Setup</a>"));
        // A site without them has neither navigation.
        assert!(!render("x\n", "/setup/", "en").contains("rd-header-nav"));
    }

    #[test]
    fn contents_links_encode_their_fragment() {
        let html = render(
            "## Done {#100%}\n\n## Space {#a%20b}\n\n## 安装\n",
            "/setup/",
            "en",
        );
        assert!(
            html.contains("<a href=\"#100%25\" data-rd-toc-link=\"\">Done</a>"),
            "{html}"
        );
        assert!(html.contains("<a href=\"#a%2520b\" data-rd-toc-link=\"\">Space</a>"));
        assert!(html.contains("<a href=\"#安装\" data-rd-toc-link=\"\">安装</a>"));
        assert!(html.contains("<h2 id=\"100%\">Done</h2>"));
    }

    #[test]
    fn content_h1_is_not_doubled_and_chinese_labels_are_used() {
        let html = render("# 安装\n\n正文。\n", "/setup/", "zh-CN");
        assert_eq!(html.matches("<h1").count(), 1);
        assert!(html.contains("<html lang=\"zh-CN\""));
        assert!(html.contains("跳到正文"));
        let doc = dom_query::Document::from(html.as_str());
        assert_eq!(
            doc.select(".rd-mode-switch").attr("lang").as_deref(),
            Some("zh")
        );
        assert_eq!(doc.select("article").attr("lang").as_deref(), Some("zh-CN"));
    }

    #[test]
    fn a_third_language_keeps_its_lang_and_marks_the_interface_english() {
        let html = render("Bonjour.\n", "/setup/", "fr");
        assert!(html.contains("<html lang=\"fr\""));
        assert!(
            html.contains("<a href=\"#content\" lang=\"en\" class=\"sr-only"),
            "{html}"
        );
    }

    /// The policy is the repository's strict one, character for character:
    /// read from the serve command's source, where its string continues
    /// across lines with `\` and leading spaces.
    #[test]
    fn csp_is_the_strict_serve_policy() {
        let source = include_str!("../../../xtask/src/serve.rs");
        let start = source
            .find("Self::Strict => Some(")
            .expect("the strict arm")
            + "Self::Strict => Some(".len();
        let literal = &source[start..];
        let open = literal.find('"').unwrap() + 1;
        let close = open
            + literal[open..]
                .find("\",")
                .or_else(|| literal[open..].find('"'))
                .unwrap();
        let mut policy = String::new();
        for (i, line) in literal[open..close].split("\\\n").enumerate() {
            policy.push_str(if i == 0 { line } else { line.trim_start() });
        }
        assert_eq!(policy, CSP);
    }

    #[test]
    fn csp_meta_drops_only_frame_ancestors() {
        assert_eq!(
            meta_csp().matches("; ").count(),
            CSP.matches("; ").count() - 1
        );
        assert!(!meta_csp().contains("frame-ancestors"));
    }
}
