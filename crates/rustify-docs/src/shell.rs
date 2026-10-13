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

use crate::escape::escape_html_into;
use crate::labels::{self, Labels};
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
}

/// A titled group of sidebar entries, shown on the pages under `prefix`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NavGroup {
    pub label: String,
    pub prefix: String,
    pub items: Vec<NavItem>,
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
}

/// One page to render.
pub struct Page<'a> {
    pub route: &'a str,
    /// The document's `<title>`, unique across the site.
    pub document_title: &'a str,
    pub title: &'a str,
    pub description: Option<&'a str>,
    pub lang: &'a str,
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
    let (labels, _) = labels::for_lang(&site.lang);
    let mut article = String::from("<p>");
    crate::escape::escape_html_into(&mut article, labels.not_found_body);
    article.push_str("</p>\n<p><a href=\"");
    crate::escape::escape_html_into(&mut article, &site.url("/"));
    article.push_str("\">");
    crate::escape::escape_html_into(&mut article, labels.back_home);
    article.push_str("</a></p>\n");
    let document_title = format!("{} | {}", labels.not_found_title, site.name);
    let article = slots
        .get(&Slot::NotFound)
        .map_or(article.as_str(), String::as_str);
    render_page_with_slots(
        site,
        &Page {
            route: "/404/",
            document_title: &document_title,
            title: labels.not_found_title,
            description: Some(labels.not_found_body),
            lang: &site.lang,
            draft: false,
            article,
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
    let (labels, _) = labels::for_lang(&site.lang);
    let (title, body) = if building {
        (labels.dev.building_title, labels.dev.building_body)
    } else {
        (labels.dev.shell_title, labels.dev.shell_body)
    };
    let mut article = String::from("<p>");
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
    let mut replacements = vec![(ARTICLE, page.article)];
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
    let (labels, foreign) = labels::for_lang(page.lang);
    let ui_lang = foreign.then_some(labels.lang);
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
        Some(home) => home_layout(page, home, labels, ui_lang, slots, &main_id).into_any(),
        None => doc_layout(site, page, &toc, labels, ui_lang, slots, &main_id).into_any(),
    };
    view! {
        <html lang=page.lang.to_owned() data-rustify-scope="" data-theme="system" data-rd-base=site.base.clone()>
            <head>
                <meta charset="utf-8" />
                <meta name="viewport" content="width=device-width, initial-scale=1" />
                <meta http-equiv="Content-Security-Policy" content=meta_csp() />
                <title>{document_title}</title>
                <meta name="description" content=description.clone() />
                {meta.noindex.then(|| view! { <meta name="robots" content="noindex" /> })}
                {meta.canonical.map(|url| view! { <link rel="canonical" href=url.to_owned() /> })}
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
                    let words = serde_json::to_string(&labels.dev).unwrap_or_default();
                    view! {
                        <script src=DEV_SCRIPT_URL defer="" data-labels=words data-lang=ui_lang.unwrap_or(page.lang).to_owned()></script>
                    }
                })}
            </head>
            <body class="min-h-screen bg-background text-foreground antialiased">
                <a
                    href=format!("#{main_id}")
                    class="sr-only focus:not-sr-only focus:fixed focus:top-2 focus:left-2 focus:z-50 focus:rounded-md focus:bg-background focus:px-3 focus:py-2"
                    lang=ui_lang
                >
                    {labels.skip_to_content}
                </a>
                {header(site, page.route, labels, ui_lang, slots)}
                {body}
                <footer class="rd-site-footer mx-auto max-w-[76rem] px-4 md:px-6">
                    {if slots.contains_key(&Slot::Footer) {
                        view! { <div inner_html=slot_marker(Slot::Footer)></div> }.into_any()
                    } else {
                        view! { <nav aria-label=labels.footer_links lang=ui_lang>
                            <a href=site.url("/")>{labels.back_home}</a>
                        </nav> }.into_any()
                    }}
                </footer>
                <p
                    id="rd-status"
                    class="sr-only"
                    aria-live="polite"
                    data-copied=labels.copied
                    data-copy-failed=labels.copy_failed
                    data-copy-failed-short=labels.copy_failed_short
                    lang=ui_lang
                ></p>
            </body>
        </html>
    }
}

/// A content page: the sidebar, the table of contents and the article with
/// its footer and the previous/next links.
fn doc_layout(
    site: &Site,
    page: &Page,
    toc: &[&Heading],
    labels: &'static Labels,
    ui_lang: Option<&'static str>,
    slots: &LayoutSlots,
    main_id: &str,
) -> impl IntoView {
    let (previous, next) = site.neighbours(page.route);
    // The article's one H1 is the content's own when it has one.
    let heading = (!page.article_has_h1).then(|| page.title.to_owned());
    view! {
        <div class="rd-layout mx-auto grid max-w-[76rem] gap-x-10 px-4 md:px-6">
            {sidebar(site, page.route, labels, ui_lang, slots)}
            {table_of_contents(toc, labels, ui_lang)}
            <main id=main_id.to_owned() class="rd-main min-w-0 py-8 md:py-12">
                <div class="rd-column">
                    {page.draft.then(|| view! {
                        <p class="rd-draft" lang=ui_lang>{labels.draft}</p>
                    })}
                    {heading.map(|title| view! { <h1 class="rd-title">{title}</h1> })}
                    {slot_view(Slot::BeforeContent, slots)}
                    <article class="rd-prose" inner_html=ARTICLE></article>
                    {slot_view(Slot::AfterContent, slots)}
                    {page_footer(page.meta, labels, ui_lang)}
                    {pager(site, previous, next, labels, ui_lang)}
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
    labels: &'static Labels,
    ui_lang: Option<&'static str>,
    slots: &LayoutSlots,
    main_id: &str,
) -> impl IntoView {
    if home.custom {
        return view! {
            <main id=main_id.to_owned() class="rd-home mx-auto max-w-[76rem] px-4 md:px-6">
                {slot_view(Slot::BeforeContent, slots)}
                {(!page.article_has_h1).then(|| view! { <h1 class="rd-home-title">{page.title.to_owned()}</h1> })}
                <div inner_html=ARTICLE></div>
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
            view! { <a href=url.clone() class=class lang=ui_lang.map(|_| page.lang.to_owned())>{text.to_string()}</a> }
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
        view! { <ul class="rd-features" data-columns=columns>{items}</ul> }
    });
    view! {
        <main id=main_id.to_owned() class="rd-home mx-auto max-w-[76rem] px-4 md:px-6">
            <div class="rd-hero">
                <h1 class="rd-home-title" id=home.heading_id.map(str::to_owned)>{page.title.to_owned()}</h1>
                {home.tagline.map(|tagline| view! { <p class="rd-tagline">{tagline.to_owned()}</p> })}
                {(!home.actions.is_empty()).then(|| view! { <nav class="rd-actions" aria-label=labels.start_reading lang=ui_lang>{actions}</nav> })}
            </div>
            {features}
            {slot_view(Slot::BeforeContent, slots)}
            <article class="rd-prose rd-home-prose" inner_html=ARTICLE></article>
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
            html.push('"');
            match link.route.as_deref() {
                Some(target) if target == route => html.push_str(" aria-current=\"page\""),
                Some(target) if target != "/" && route.starts_with(target) => {
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
fn header(
    site: &Site,
    route: &str,
    labels: &'static Labels,
    ui_lang: Option<&'static str>,
    slots: &LayoutSlots,
) -> impl IntoView {
    let at_home = route == "/";
    let name = site.name.clone();
    let entries = header_entries(site, route);
    let links = (!entries.is_empty()).then(|| {
        view! { <nav class="rd-header-nav rd-header-nav-wide" aria-label=labels.site_links lang=ui_lang inner_html=entries.clone()></nav> }
    });
    let narrow = (!entries.is_empty()).then(|| {
        view! { <nav class="rd-header-nav rd-header-nav-narrow" aria-label=labels.site_links lang=ui_lang inner_html=entries></nav> }
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
            <div class="mx-auto flex h-14 max-w-[76rem] items-center gap-4 px-4 md:px-6">
                {if slots.contains_key(&Slot::Header) {
                    view! { <div class="rd-brand" inner_html=slot_marker(Slot::Header)></div> }.into_any()
                } else {
                    view! { <a href=site.url("/") class="rd-brand" aria-current=at_home.then_some("page")>{logo}<span>{name}</span></a> }.into_any()
                }}
                {links}
                <div class="rd-mode-switch" data-rd-mode-switch="" hidden="" role="group" aria-label=labels.theme lang=ui_lang>
                    {mode_button("system", labels.theme_system)}
                    {mode_button("light", labels.theme_light)}
                    {mode_button("dark", labels.theme_dark)}
                </div>
            </div>
        </header>
        {narrow}
    }
}

fn mode_button(mode: &'static str, label: &'static str) -> impl IntoView {
    view! {
        <button type="button" class="rd-mode-option" data-rd-mode=mode aria-pressed="false">
            <span class="rd-mode-icon" inner_html=crate::icons::mode(mode)></span>
            <span class="rd-mode-label">{label}</span>
        </button>
    }
}

/// The sidebar's entries as HTML: hundreds on every page of a large site,
/// each a bare link styled by one `.rd-nav` rule instead of a class list
/// repeated per entry, written once and placed in both navigations.
fn sidebar_entries(site: &Site, route: &str, labels: &'static Labels) -> String {
    let mut html = String::new();
    for group in site.groups_for(route) {
        html.push_str("<section><h2>");
        escape_html_into(&mut html, &group.label);
        html.push_str("</h2><ul>");
        for item in &group.items {
            html.push_str("<li><a href=\"");
            escape_html_into(&mut html, &site.url(&item.route));
            html.push('"');
            if item.route == route {
                html.push_str(" aria-current=\"page\"");
            }
            html.push('>');
            escape_html_into(&mut html, &item.title);
            if item.draft {
                html.push_str(" · ");
                html.push_str(labels.draft);
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
fn sidebar(
    site: &Site,
    route: &str,
    labels: &'static Labels,
    ui_lang: Option<&'static str>,
    slots: &LayoutSlots,
) -> impl IntoView {
    let entries = sidebar_entries(site, route, labels);
    view! {
        <div class="rd-sidebar">
            {slot_view(Slot::SidebarTop, slots)}
            <nav class="rd-nav rd-sidebar-wide" aria-label=labels.menu lang=ui_lang inner_html=entries.clone()></nav>
            <details class="rd-sidebar-narrow">
                <summary lang=ui_lang>{labels.menu}</summary>
                <nav class="rd-nav" aria-label=labels.menu lang=ui_lang inner_html=entries></nav>
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
fn table_of_contents(
    toc: &[&Heading],
    labels: &'static Labels,
    ui_lang: Option<&'static str>,
) -> impl IntoView {
    (!toc.is_empty()).then(|| {
        let entries = toc_entries(toc);
        view! {
            <div class="rd-toc">
                <nav class="rd-toc-wide" aria-label=labels.on_this_page lang=ui_lang>
                    <p>{labels.on_this_page}</p>
                    <ul inner_html=entries.clone()></ul>
                </nav>
                <details class="rd-toc-narrow">
                    <summary lang=ui_lang>{labels.on_this_page}</summary>
                    <nav aria-label=labels.on_this_page lang=ui_lang>
                        <ul inner_html=entries></ul>
                    </nav>
                </details>
            </div>
        }
    })
}

fn page_footer(
    meta: Meta,
    labels: &'static Labels,
    ui_lang: Option<&'static str>,
) -> impl IntoView {
    let edit = meta.edit_url.map(|url| {
        view! { <a href=url.to_owned() rel="noopener" lang=ui_lang>{labels.edit_page}</a> }
    });
    let updated = meta.updated.map(|date| {
        view! {
            <p>
                <span lang=ui_lang>{labels.last_updated}</span>
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
    labels: &'static Labels,
    ui_lang: Option<&'static str>,
) -> impl IntoView {
    let link = |item: &NavItem, label: &'static str, is_next: bool| {
        view! {
            <a href=site.url(&item.route) rel=if is_next { "next" } else { "prev" } data-rd-next=is_next.then_some("")>
                <span class="rd-pager-label" lang=ui_lang>{label}</span>
                <span class="rd-pager-title">{item.title.clone()}</span>
            </a>
        }
    };
    (previous.is_some() || next.is_some()).then(|| {
        view! {
            <nav class="rd-pager" aria-label=format!("{} / {}", labels.previous, labels.next)>
                {previous.map(|item| link(item, labels.previous, false))}
                {next.map(|item| link(item, labels.next, true))}
            </nav>
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::highlight::Highlighter;
    use crate::markdown;

    fn site() -> Site {
        Site {
            name: "Docs".into(),
            lang: "en".into(),
            base: "/rustify-ui/".into(),
            nav: vec![NavGroup {
                label: "Guide".into(),
                prefix: "/".into(),
                items: vec![
                    NavItem {
                        title: "Intro".into(),
                        route: "/".into(),
                        draft: false,
                    },
                    NavItem {
                        title: "Setup".into(),
                        route: "/setup/".into(),
                        draft: false,
                    },
                    NavItem {
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

    fn render(markdown_source: &str, route: &str, lang: &str) -> String {
        let (labels, foreign) = labels::for_lang(lang);
        let includes = markdown::IncludeFiles::default();
        let context = markdown::Context {
            highlighter: &Highlighter::new(),
            labels,
            foreign,
            includes: &includes,
        };
        let fragment = markdown::render(markdown_source, 0, &context);
        render_page(
            &site(),
            &Page {
                route,
                document_title: "Setup | Docs",
                title: "Setup",
                description: fragment.summary.as_deref(),
                lang,
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
        assert!(html.contains("<h1 class=\"rd-title\">Page not found</h1>"));
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
        assert!(html.contains("<meta name=\"description\" content=\"Run cargo.\">"));
        // One H1, from the title, because the content has none.
        assert_eq!(html.matches("<h1").count(), 1);
        assert!(html.contains("<h2 id=\"install\">Install</h2>"));
        // The current page is marked; the others link under the base.
        assert!(
            html.contains(
                "<li><a href=\"/rustify-ui/setup/\" aria-current=\"page\">Setup</a></li>"
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
            html.contains("Next steps · Draft</a>"),
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
                label: "Setup".into(),
                url: "/rustify-ui/setup/".into(),
                route: Some("/setup/".into()),
                external: false,
            },
            HeaderLink {
                label: "Home".into(),
                url: "/rustify-ui/".into(),
                route: Some("/".into()),
                external: false,
            },
        ];
        site.social = vec![HeaderLink {
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
        let links = "<ul class=\"rd-header-links\"><li><a href=\"/rustify-ui/setup/\" aria-current=\"true\">Setup</a></li><li><a href=\"/rustify-ui/\">Home</a></li></ul><ul class=\"rd-header-social\"><li><a href=\"https://github.com/example\" rel=\"noopener\">GitHub</a></li></ul>";
        // Once in the header bar, once in a row of its own under it.
        assert_eq!(html.matches(links).count(), 2, "{html}");
        assert!(
            html.contains(
                "<nav aria-label=\"Site links\" class=\"rd-header-nav rd-header-nav-wide\">"
            ),
            "{html}"
        );
        assert!(html.contains(
            "</header><nav aria-label=\"Site links\" class=\"rd-header-nav rd-header-nav-narrow\">"
        ));
        let html = page("/setup/");
        assert!(html.contains("<a href=\"/rustify-ui/setup/\" aria-current=\"page\">Setup</a>"));
        let html = page("/");
        assert!(html.contains("<a href=\"/rustify-ui/\" aria-current=\"page\">Home</a>"));
        assert!(html.contains("<a href=\"/rustify-ui/setup/\">Setup</a>"));
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
        assert!(!html.contains("lang=\"en\""));
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
