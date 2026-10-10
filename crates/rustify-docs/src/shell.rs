//! The default theme's page, rendered to HTML on the host with Leptos.
//!
//! A page is complete before any script runs: the navigation, the table of
//! contents and the article are links and text, the theme follows the
//! system through CSS, and the narrow-screen menu is a native `<details>`.
//! The framework's small script only adds what needs one (the mode switch,
//! copying, code groups, the active heading). No wasm is ever loaded.

use leptos::prelude::*;
use leptos::tachys::html::attribute::custom::CustomAttribute;
use rustify_components::{provide_current_path, Link, LinkMatch};

use crate::labels::{self, Labels};
use crate::markdown::Heading;

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

/// Where a build put the shared files, already joined with the base.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AssetUrls {
    pub stylesheet: String,
    /// Runs in `<head>`, before first paint, to apply a stored mode.
    pub mode_script: String,
    pub site_script: String,
}

/// What every page of a site shares.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Site {
    pub name: String,
    pub lang: String,
    /// `/` or `/segments/`.
    pub base: String,
    pub nav: Vec<NavGroup>,
    pub assets: AssetUrls,
    /// The logo's URL in light and in dark mode; the same URL twice for one
    /// image.
    pub logo: Option<(String, String)>,
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
}

/// The page served for an address with nothing at it: the site's shell, one
/// heading and a way home. Its links and files are root-relative under the
/// base, so it works from any depth.
pub fn render_not_found(site: &Site) -> String {
    let (labels, _) = labels::for_lang(&site.lang);
    let mut article = String::from("<p>");
    crate::escape::escape_html_into(&mut article, labels.not_found_body);
    article.push_str("</p>\n<p><a href=\"");
    crate::escape::escape_html_into(&mut article, &site.url("/"));
    article.push_str("\">");
    crate::escape::escape_html_into(&mut article, labels.back_home);
    article.push_str("</a></p>\n");
    let document_title = format!("{} | {}", labels.not_found_title, site.name);
    render_page(
        site,
        &Page {
            route: "/404/",
            document_title: &document_title,
            title: labels.not_found_title,
            description: Some(labels.not_found_body),
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
        },
    )
}

/// Where the article goes in the rendered shell. Leptos writes the shell;
/// the article is spliced in afterwards, so it is not copied through the
/// renderer and the clean-up below never touches an author's markup.
const ARTICLE: &str = "<!--rustify-docs:article-->";

/// Renders `page` as a complete document.
pub fn render_page(site: &Site, page: &Page) -> String {
    let owner = Owner::new();
    let shell = owner.with(|| page_view(site, page).to_html());
    drop(owner);
    // `<!>` is a hydration marker between dynamic pieces. Nothing hydrates a
    // static page, and author text cannot produce one here: the renderer
    // escapes every `<` it writes as text.
    let shell = shell.replace("<!>", "");
    let (before, after) = shell
        .split_once(ARTICLE)
        .expect("the shell has an article placeholder");
    let mut document = String::with_capacity(16 + shell.len() + page.article.len());
    document.push_str("<!doctype html>");
    document.push_str(before);
    document.push_str(page.article);
    document.push_str(after);
    document
}

fn page_view(site: &Site, page: &Page) -> impl IntoView {
    provide_current_path(Signal::stored(site.url(page.route)));
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
        .filter(|heading| heading.level >= page.toc_levels.0 && heading.level <= page.toc_levels.1)
        .collect();
    let (previous, next) = site.neighbours(page.route);
    // The article's one H1 is the content's own when it has one.
    let heading = (!page.article_has_h1).then(|| page.title.to_owned());
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
                <script src=site.assets.mode_script.clone()></script>
                <script src=site.assets.site_script.clone() defer=""></script>
            </head>
            <body class="min-h-screen bg-background text-foreground antialiased">
                <a
                    href="#content"
                    class="sr-only focus:not-sr-only focus:fixed focus:top-2 focus:left-2 focus:z-50 focus:rounded-md focus:bg-background focus:px-3 focus:py-2"
                    lang=ui_lang
                >
                    {labels.skip_to_content}
                </a>
                {header(site, labels, ui_lang)}
                <div class="rd-layout mx-auto grid max-w-[90rem] gap-x-8 px-4 md:px-6">
                    {sidebar(site, page.route, labels, ui_lang)}
                    {table_of_contents(&toc, labels, ui_lang)}
                    <main id="content" class="rd-main min-w-0 py-8 md:py-10">
                        {page.draft.then(|| view! {
                            <p class="mb-4 w-fit rounded-md border border-border px-2 py-0.5 text-sm text-muted-foreground" lang=ui_lang>
                                {labels.draft}
                            </p>
                        })}
                        {heading.map(|title| view! { <h1 class="mb-6 text-3xl font-bold tracking-tight">{title}</h1> })}
                        <article class="rd-prose max-w-3xl" inner_html=ARTICLE></article>
                        {page_footer(meta, labels, ui_lang)}
                        {pager(site, previous, next, labels, ui_lang)}
                    </main>
                </div>
                <p
                    id="rd-status"
                    class="sr-only"
                    aria-live="polite"
                    data-copied=labels.copied
                    data-copy-failed=labels.copy_failed
                    lang=ui_lang
                ></p>
            </body>
        </html>
    }
}

/// An Open Graph `<meta property=… content=…>`; `property` is not an
/// attribute the view macro knows.
fn open_graph(property: &'static str, content: String) -> impl IntoView {
    leptos::html::meta()
        .attr("property", property)
        .attr("content", content)
}

fn header(site: &Site, labels: &'static Labels, ui_lang: Option<&'static str>) -> impl IntoView {
    let name = site.name.clone();
    // The name says what the logo shows, so the logo is decoration. A pair
    // switches with the mode through CSS, with or without script.
    let logo = site.logo.clone().map(|(light, dark)| {
        if light == dark {
            view! { <img src=light alt="" class="h-6 w-auto" /> }.into_any()
        } else {
            view! {
                <img src=light alt="" class="h-6 w-auto dark:hidden" />
                <img src=dark alt="" class="hidden h-6 w-auto dark:block" />
            }
            .into_any()
        }
    });
    view! {
        <header class="sticky top-0 z-40 border-b border-secondary bg-background">
            <div class="mx-auto flex h-14 max-w-[90rem] items-center gap-4 px-4 md:px-6">
                <Link href=site.url("/") match_=LinkMatch::Exact class="inline-flex items-center gap-2 font-semibold text-foreground no-underline">{logo}{name}</Link>
                <div class="ml-auto flex items-center gap-1" data-rd-mode-switch="" hidden="" role="group" aria-label=labels.theme lang=ui_lang>
                    {mode_button("system", labels.theme_system)}
                    {mode_button("light", labels.theme_light)}
                    {mode_button("dark", labels.theme_dark)}
                </div>
            </div>
        </header>
    }
}

fn mode_button(mode: &'static str, label: &'static str) -> impl IntoView {
    view! {
        <button
            type="button"
            class="rounded-md px-2 py-1 text-sm text-muted-foreground hover:bg-muted aria-pressed:bg-secondary aria-pressed:text-foreground"
            data-rd-mode=mode
            aria-pressed="false"
        >
            {label}
        </button>
    }
}

fn sidebar(
    site: &Site,
    route: &str,
    labels: &'static Labels,
    ui_lang: Option<&'static str>,
) -> impl IntoView {
    let groups: Vec<&NavGroup> = site.groups_for(route).collect();
    // Hundreds of entries on every page: each is a bare link, styled by one
    // `.rd-nav` rule instead of a class list repeated per entry.
    let entries = groups
        .into_iter()
        .map(|group| {
            let items = group
                .items
                .iter()
                .map(|item| {
                    let current = (item.route == route).then_some("page");
                    let title = if item.draft {
                        format!("{} · {}", item.title, labels.draft)
                    } else {
                        item.title.clone()
                    };
                    view! { <li><a href=site.url(&item.route) aria-current=current>{title}</a></li> }
                })
                .collect_view();
            view! {
                <section>
                    <h2>{group.label.clone()}</h2>
                    <ul>{items}</ul>
                </section>
            }
        })
        .collect_view();
    view! {
        <details class="rd-sidebar md:sticky md:top-14 md:max-h-[calc(100vh-3.5rem)] md:overflow-y-auto md:py-8">
            <summary class="cursor-pointer py-3 text-sm font-medium md:hidden" lang=ui_lang>{labels.menu}</summary>
            <nav class="rd-nav" aria-label=labels.menu lang=ui_lang>{entries}</nav>
        </details>
    }
}

fn table_of_contents(
    toc: &[&Heading],
    labels: &'static Labels,
    ui_lang: Option<&'static str>,
) -> impl IntoView {
    (!toc.is_empty()).then(|| {
        let entries = toc
            .iter()
            .map(|heading| {
                let indent = if heading.level > 2 { "pl-3" } else { "" };
                view! {
                    <li class=indent>
                        <a href=format!("#{}", heading.id) class="block py-0.5 text-muted-foreground hover:text-foreground aria-[current=location]:font-medium aria-[current=location]:text-primary" data-rd-toc-link="">
                            {heading.text.clone()}
                        </a>
                    </li>
                }
            })
            .collect_view();
        view! {
            <details class="rd-toc text-sm xl:sticky xl:top-14 xl:max-h-[calc(100vh-3.5rem)] xl:overflow-y-auto xl:py-10">
                <summary class="cursor-pointer py-2 font-medium xl:hidden" lang=ui_lang>{labels.on_this_page}</summary>
                <nav aria-label=labels.on_this_page lang=ui_lang>
                    <p class="mb-2 hidden font-semibold xl:block">{labels.on_this_page}</p>
                    <ul class="space-y-1">{entries}</ul>
                </nav>
            </details>
        }
    })
}

fn page_footer(
    meta: Meta,
    labels: &'static Labels,
    ui_lang: Option<&'static str>,
) -> impl IntoView {
    let edit = meta.edit_url.map(|url| {
        view! { <a href=url.to_owned() rel="noopener" class="text-primary hover:underline" lang=ui_lang>{labels.edit_page}</a> }
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
            <footer class="mt-10 flex flex-wrap items-center gap-x-6 gap-y-2 border-t border-secondary pt-4 text-sm text-muted-foreground">
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
    let link = |item: &NavItem, label: &'static str, align: &'static str| {
        view! {
            <a href=site.url(&item.route) class=format!("flex flex-col rounded-md border border-border px-4 py-3 hover:border-primary {align}")>
                <span class="text-xs text-muted-foreground" lang=ui_lang>{label}</span>
                <span class="font-medium text-primary">{item.title.clone()}</span>
            </a>
        }
    };
    (previous.is_some() || next.is_some()).then(|| {
        view! {
            <nav class="mt-12 grid gap-4 border-t border-secondary pt-6 sm:grid-cols-2" aria-label=format!("{} / {}", labels.previous, labels.next)>
                <div>{previous.map(|item| link(item, labels.previous, ""))}</div>
                <div>{next.map(|item| link(item, labels.next, "sm:text-right"))}</div>
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
            assets: AssetUrls {
                stylesheet: "/rustify-ui/assets/site.css".into(),
                mode_script: "/rustify-ui/assets/mode.js".into(),
                site_script: "/rustify-ui/assets/site.js".into(),
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
            },
        )
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
        assert!(html.contains("<a href=\"/rustify-ui/\" class=\"flex flex-col"));
        assert!(html.contains("<a href=\"/rustify-ui/next/\" class=\"flex flex-col"));
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
