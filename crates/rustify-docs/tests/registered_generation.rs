#![cfg(feature = "pipeline")]

use std::collections::{BTreeMap, BTreeSet};
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use rustify_docs::content::{
    self, Diagnostic, FragmentCache, Mode, Project, RegisteredPage, Surfaces,
};
use rustify_docs::diagnostic::Severity;
use rustify_docs::generation::{self, Extensions, Generation};
use rustify_docs::highlight::Highlighter;
use rustify_docs::site::Slot;

const CONFIG: &str =
    "schema_version=1\n[site]\nname='Docs'\nbase='/docs/'\nurl='https://example.com/docs/'\n";
const HOME: &str = "---\ndescription: Home page\n---\n# Home\n";

struct Fixture(PathBuf);

impl Fixture {
    fn new(files: &[(&str, &str)]) -> Self {
        static NEXT: AtomicUsize = AtomicUsize::new(0);
        let time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let root = std::env::temp_dir().join(format!(
            "rustify-docs-registered-generation-{}-{time}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        for (path, text) in files {
            let path = root.join(path);
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(path, text).unwrap();
        }
        Self(root)
    }

    fn generate(
        &self,
        surfaces: &Surfaces,
        extensions: &Extensions,
    ) -> Result<Generation, Vec<Diagnostic>> {
        let (project, warnings) =
            Project::load(&self.0).unwrap_or_else(|errors| panic!("{errors:?}"));
        assert!(warnings.is_empty(), "{warnings:?}");
        let (snapshot, problems) = content::snapshot(&project);
        assert!(problems.is_empty(), "{problems:?}");
        let analysis = content::analyze_with_surfaces(
            &project,
            &snapshot,
            Mode::Production,
            &mut FragmentCache::default(),
            &Highlighter::new(),
            1,
            surfaces,
        );
        if analysis.errors() != 0 {
            return Err(analysis.diagnostics);
        }
        generation::generate_extended(
            &project,
            &snapshot,
            &analysis,
            &Default::default(),
            1,
            extensions,
        )
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn page(name: &str, route: &str, title: &str, html: &str) -> RegisteredPage {
    RegisteredPage {
        name: name.into(),
        route: route.into(),
        title: title.into(),
        description: None,
        html: html.into(),
        file: "src/pages.rs".into(),
    }
}

fn html(generation: &Generation, path: &str) -> String {
    String::from_utf8(generation.files.get(path).unwrap().to_vec()).unwrap()
}

#[track_caller]
fn rejects(result: Result<Generation, Vec<Diagnostic>>, rule: &str) {
    let errors = match result {
        Ok(generation) => panic!(
            "expected {rule}, but generated {:?}",
            generation.files.keys().collect::<Vec<_>>()
        ),
        Err(errors) => errors,
    };
    assert!(
        errors
            .iter()
            .any(|error| error.severity == Severity::Error && error.rule == rule),
        "expected {rule}: {errors:?}"
    );
}

fn skip_target(output: &str) -> String {
    let dom = dom_query::Document::from(output);
    assert_eq!(dom.select("main").length(), 1);
    let main = dom.select("main").attr("id").unwrap().to_string();
    assert_eq!(
        dom.select("body > a").attr("href").unwrap().to_string(),
        format!("#{main}")
    );
    assert_eq!(dom.select(&format!("[id='{main}']")).length(), 1);
    main
}

#[test]
fn markdown_content_heading_keeps_its_body_anchor_and_a_unique_main_target() {
    let fixture = Fixture::new(&[
        ("rustify-docs.toml", CONFIG),
        ("content/index.md", HOME),
        (
            "content/guide.md",
            "---\ndescription: Guide page\n---\n# Guide\n\n## Content\n\n[Return to content](#content)\n",
        ),
    ]);
    let generation = fixture
        .generate(&Surfaces::default(), &Extensions::default())
        .unwrap();
    let output = html(&generation, "guide/index.html");
    let dom = dom_query::Document::from(output.as_str());
    assert_eq!(dom.select("h2#content").length(), 1);
    assert_eq!(dom.select("article a[href='#content']").length(), 1);
    assert_ne!(skip_target(&output), "content");
}

#[test]
fn markdown_home_content_heading_keeps_the_hero_anchor_and_a_unique_main_target() {
    let fixture = Fixture::new(&[
        ("rustify-docs.toml", CONFIG),
        (
            "content/index.md",
            "---\nlayout: home\ndescription: Home page\n---\n# Content\n\n[Return to content](#content)\n",
        ),
    ]);
    let generation = fixture
        .generate(&Surfaces::default(), &Extensions::default())
        .unwrap();
    let output = html(&generation, "index.html");
    let dom = dom_query::Document::from(output.as_str());
    assert_eq!(dom.select("h1#content").length(), 1);
    assert_eq!(dom.select("article a[href='#content']").length(), 1);
    assert_ne!(skip_target(&output), "content");
}

#[test]
fn registered_home_custom_and_not_found_skip_targets_avoid_live_body_and_slot_ids() {
    let fixture = Fixture::new(&[
        ("rustify-docs.toml", CONFIG),
        (
            "content/guide.md",
            "---\ndescription: Guide page\n---\n# Guide\n",
        ),
    ]);
    let surfaces = Surfaces {
        pages: vec![
            page(
                "Home",
                "/",
                "Registered home",
                "<h1 id='content'>Home content</h1><a href='#content'>Home anchor</a>",
            ),
            page(
                "Custom",
                "/custom/",
                "Custom",
                "<h2 id='content'>Custom content</h2><a href='#content'>Custom anchor</a>",
            ),
        ],
        slots: BTreeMap::from([
            (
                "/".into(),
                BTreeMap::from([
                    (Slot::Header, "<span id='content-1'>Header</span>".into()),
                    (Slot::Footer, "<p id='content-2'>Footer</p>".into()),
                ]),
            ),
            (
                "/custom/".into(),
                BTreeMap::from([
                    (
                        Slot::SidebarTop,
                        "<span id='content-1'>Sidebar</span>".into(),
                    ),
                    (Slot::AfterContent, "<p id='content-2'>After</p>".into()),
                ]),
            ),
            (
                "/404/".into(),
                BTreeMap::from([
                    (
                        Slot::NotFound,
                        "<p id='content'>Missing content</p><a href='#content'>Missing anchor</a>"
                            .into(),
                    ),
                    (Slot::Header, "<span id='content-1'>Header</span>".into()),
                    (Slot::Footer, "<p id='content-2'>Footer</p>".into()),
                ]),
            ),
        ]),
        ..Default::default()
    };
    let generation = fixture.generate(&surfaces, &Extensions::default()).unwrap();
    for path in ["index.html", "custom/index.html", "404.html"] {
        let output = html(&generation, path);
        let dom = dom_query::Document::from(output.as_str());
        for id in ["content", "content-1", "content-2"] {
            assert_eq!(dom.select(&format!("[id='{id}']")).length(), 1);
        }
        let target = skip_target(&output);
        assert!(!["content", "content-1", "content-2"].contains(&target.as_str()));
        assert!(dom.select("a[href='#content']").length() > 0);
    }
}

#[test]
fn stored_template_and_textarea_content_do_not_reserve_a_live_main_target() {
    let fixture = Fixture::new(&[("rustify-docs.toml", CONFIG), ("content/index.md", HOME)]);
    let surfaces = Surfaces {
        pages: vec![page("Custom", "/custom/", "Custom", "<template><p id='content'>Stored content</p></template><textarea>&lt;p id='content'&gt;Literal content&lt;/p&gt;</textarea>")],
        ..Default::default()
    };
    let generation = fixture.generate(&surfaces, &Extensions::default()).unwrap();
    assert_eq!(
        skip_target(&html(&generation, "custom/index.html")),
        "content"
    );
}

#[test]
fn seven_slots_keep_live_ids_and_references_in_the_final_documents() {
    let fixture = Fixture::new(&[
        ("rustify-docs.toml", CONFIG),
        ("content/index.md", "---\ndescription: Home page\n---\n# Home\n\n[Body](/docs/custom/#body-anchor) [Header](/docs/custom/#slot-header) [Footer](/docs/custom/#slot-footer) [Top](/docs/custom/#slot-top) [Bottom](/docs/custom/#slot-bottom) [Before](/docs/custom/#slot-before) [After](/docs/custom/#slot-after)\n"),
        ("content/guide.md", "---\ndescription: Guide page\n---\n# Guide\n\n## Markdown target\n\n[Custom footer](/docs/custom/#slot-footer)\n"),
    ]);
    let surfaces = Surfaces {
        pages: vec![page("Custom", "/custom/", "Custom", "<!><h2 id='body-anchor'>Body</h2><a href='#slot-header'>Header</a><a href='#slot-footer'>Footer</a><a href='/docs/guide/#markdown-target'>Guide</a>")],
        slots: BTreeMap::from([
            ("/custom/".into(), BTreeMap::from([
                (Slot::Header, "<span id='slot-header'><a href='#body-anchor'>Body</a></span>".into()),
                (Slot::Footer, "<p id='slot-footer'><a href='/docs/guide/#markdown-target'>Guide</a></p>".into()),
                (Slot::SidebarTop, "<p id='slot-top'><a href='#slot-bottom'>Bottom</a></p>".into()),
                (Slot::SidebarBottom, "<p id='slot-bottom'><a href='#slot-top'>Top</a></p>".into()),
                (Slot::BeforeContent, "<p id='slot-before'><a href='#slot-after'>After</a></p>".into()),
                (Slot::AfterContent, "<p id='slot-after'><a href='#slot-before'>Before</a></p>".into()),
            ])),
            ("/404/".into(), BTreeMap::from([
                (Slot::NotFound, "<p id='not-found-body'><a href='#not-found-footer'>Footer</a><a href='/docs/custom/#slot-header'>Custom header</a></p>".into()),
                (Slot::Footer, "<p id='not-found-footer'><a href='#not-found-body'>Message</a></p>".into()),
            ])),
        ]),
        ..Default::default()
    };
    let generation = fixture.generate(&surfaces, &Extensions::default()).unwrap();
    let custom = html(&generation, "custom/index.html");
    let dom = dom_query::Document::from(custom.as_str());
    for id in [
        "body-anchor",
        "slot-header",
        "slot-footer",
        "slot-top",
        "slot-bottom",
        "slot-before",
        "slot-after",
    ] {
        assert_eq!(
            dom.select(&format!("[id='{id}']")).length(),
            1,
            "{id}: {custom}"
        );
    }
    assert_eq!(
        dom.select("a[href='/docs/guide/#markdown-target']")
            .length(),
        2
    );
    assert!(html(&generation, "index.html").contains("/docs/custom/#slot-footer"));
    let missing = html(&generation, "404.html");
    let dom = dom_query::Document::from(missing.as_str());
    assert_eq!(dom.select("#not-found-body").length(), 1);
    assert_eq!(dom.select("#not-found-footer").length(), 1);
    assert_eq!(dom.select("a[href='#not-found-body']").length(), 1);
    assert_eq!(dom.select("a[href='#not-found-footer']").length(), 1);
}

#[test]
fn final_tree_ignores_template_and_textarea_ghosts_but_keeps_foreign_integration_links() {
    let fixture = Fixture::new(&[("rustify-docs.toml", CONFIG), ("content/index.md", HOME)]);
    let surfaces = Surfaces {
        pages: vec![page("Custom", "/custom/", "Custom", "<h2 id='actual'>Actual</h2><template id='raw'><a id='actual' href='/docs/not-published/#ghost'>Stored</a><img src='missing.svg' alt=''></template><textarea><a id='ghost' href='/docs/not-published/'>Literal link text</a></textarea><svg><title><a href='#actual'>Integrated link</a></title><defs><rect id='shape' width='2' height='2'/></defs><use href='#shape'/></svg>")],
        ..Default::default()
    };
    let generation = fixture.generate(&surfaces, &Extensions::default()).unwrap();
    let output = html(&generation, "custom/index.html");
    let dom = dom_query::Document::from(output.as_str());
    assert_eq!(dom.select("#actual").length(), 1);
    assert_eq!(dom.select("#ghost").length(), 0);
    assert_eq!(dom.select("svg title a[href='#actual']").length(), 1);
    assert_eq!(dom.select("svg use[href='#shape']").length(), 1);
    assert!(dom.select("textarea").text().contains("Literal link text"));
}

#[test]
fn slot_ids_cannot_duplicate_fixed_framework_or_body_ids() {
    let fixture = Fixture::new(&[("rustify-docs.toml", CONFIG), ("content/index.md", HOME)]);
    for id in ["rd-status", "body-anchor"] {
        let surfaces = Surfaces {
            pages: vec![page(
                "Custom",
                "/custom/",
                "Custom",
                "<h2 id='body-anchor'>Body</h2>",
            )],
            slots: BTreeMap::from([(
                "/custom/".into(),
                BTreeMap::from([(Slot::Footer, format!("<p id='{id}'>Footer</p>"))]),
            )]),
            ..Default::default()
        };
        rejects(
            fixture.generate(&surfaces, &Extensions::default()),
            "output-check",
        );
    }
}

#[test]
fn final_document_rejects_an_id_discarded_by_body_attribute_merging() {
    let fixture = Fixture::new(&[("rustify-docs.toml", CONFIG), ("content/index.md", HOME)]);
    let surfaces = Surfaces {
        pages: vec![page("Custom", "/custom/", "Custom", "<body id='discarded'><h2 id='body-anchor'>Body</h2><a href='#discarded'>Discarded body attribute</a>")],
        slots: BTreeMap::from([("/custom/".into(), BTreeMap::from([(Slot::Header, "<body id='winner'><span>Header</span>".into())]))]),
        ..Default::default()
    };
    rejects(
        fixture.generate(&surfaces, &Extensions::default()),
        "output-check",
    );
}

#[test]
fn analyzed_svg_use_rejects_an_absent_target_before_generation() {
    let fixture = Fixture::new(&[("rustify-docs.toml", CONFIG), ("content/index.md", HOME)]);
    let surfaces = Surfaces {
        pages: vec![page(
            "Custom",
            "/custom/",
            "Custom",
            "<h2>Custom</h2><svg><use href='#absent'/></svg>",
        )],
        ..Default::default()
    };
    rejects(
        fixture.generate(&surfaces, &Extensions::default()),
        "broken-anchor",
    );
}

#[test]
fn registered_document_titles_cannot_collide_with_markdown_or_its_qualified_title() {
    let fixture = Fixture::new(&[
        ("rustify-docs.toml", CONFIG),
        ("content/index.md", HOME),
        (
            "content/one/same.md",
            "---\ndescription: First\n---\n# Repeated\n",
        ),
        (
            "content/two/same.md",
            "---\ndescription: Second\n---\n# Repeated\n",
        ),
    ]);
    for title in ["Repeated", "Repeated · one"] {
        let surfaces = Surfaces {
            pages: vec![page("Custom", "/custom/", title, "<p>Custom content</p>")],
            ..Default::default()
        };
        rejects(
            fixture.generate(&surfaces, &Extensions::default()),
            "document-title",
        );
    }
}

#[test]
fn markdown_only_repeated_titles_keep_the_existing_disambiguation() {
    let fixture = Fixture::new(&[
        ("rustify-docs.toml", CONFIG),
        ("content/index.md", HOME),
        (
            "content/one/same.md",
            "---\ndescription: First\n---\n# Repeated\n",
        ),
        (
            "content/two/same.md",
            "---\ndescription: Second\n---\n# Repeated\n",
        ),
    ]);
    let generation = fixture
        .generate(&Surfaces::default(), &Extensions::default())
        .unwrap();
    for (path, title) in [
        ("one/same/index.html", "Repeated · one | Docs"),
        ("two/same/index.html", "Repeated · two | Docs"),
    ] {
        let output = html(&generation, path);
        let dom = dom_query::Document::from(output.as_str());
        assert_eq!(dom.select("head title").text().to_string(), title);
    }
}

#[test]
fn output_file_and_directory_segments_cannot_overlap_across_origins() {
    let fixture = Fixture::new(&[
        ("rustify-docs.toml", CONFIG),
        ("content/index.md", HOME),
        (
            "content/guide.md",
            "---\ndescription: Guide\n---\n# Guide\n",
        ),
    ]);
    let surfaces = Surfaces {
        pages: vec![page(
            "Nested",
            "/guide/index.html/",
            "Nested",
            "<p>Nested</p>",
        )],
        ..Default::default()
    };
    rejects(
        fixture.generate(&surfaces, &Extensions::default()),
        "output-conflict",
    );

    let fixture = Fixture::new(&[
        ("rustify-docs.toml", CONFIG),
        ("content/index.md", HOME),
        ("static/guide", "static file"),
    ]);
    let surfaces = Surfaces {
        pages: vec![page("Guide", "/guide/", "Registered guide", "<p>Guide</p>")],
        ..Default::default()
    };
    rejects(
        fixture.generate(&surfaces, &Extensions::default()),
        "output-conflict",
    );
}

#[test]
fn island_and_font_assets_cannot_publish_a_parent_file_and_child_path() {
    let fixture = Fixture::new(&[("rustify-docs.toml", CONFIG), ("content/index.md", HOME)]);
    for (parent, child) in [
        ("assets/islands", "assets/islands/group/app.js"),
        ("assets/fonts", "assets/fonts/digest.ttf"),
    ] {
        let extensions = Extensions {
            files: BTreeMap::from([
                (
                    parent.to_owned(),
                    Arc::<[u8]>::from(b"parent file".as_slice()),
                ),
                (
                    child.to_owned(),
                    Arc::<[u8]>::from(b"child file".as_slice()),
                ),
            ]),
            ..Default::default()
        };
        rejects(
            fixture.generate(&Surfaces::default(), &extensions),
            "output-conflict",
        );
    }
}

#[test]
fn a_theme_document_inside_static_is_input_only_and_never_published() {
    let config = format!("{CONFIG}[theme]\ndocument='static/theme.json'\n");
    let theme = serde_json::to_string(&rustify_ui::ThemeDocument::from_legacy(
        rustify_ui::Theme::light(),
        rustify_ui::Theme::dark(),
    ))
    .unwrap();
    let fixture = Fixture::new(&[
        ("rustify-docs.toml", &config),
        ("content/index.md", HOME),
        ("static/theme.json", &theme),
        ("static/visible.txt", "published"),
    ]);
    let generation = fixture
        .generate(&Surfaces::default(), &Extensions::default())
        .unwrap();
    assert!(!generation.files.contains_key("theme.json"));
    assert!(!generation
        .files
        .values()
        .any(|bytes| bytes.as_ref() == theme.as_bytes()));
    assert_eq!(generation.files["visible.txt"].as_ref(), b"published");
}

#[test]
fn a_hidden_only_source_image_candidate_hashes_its_url_without_renaming_the_selector() {
    let svg = "<svg xmlns='http://www.w3.org/2000/svg'><path id='shape' d='M0 0h1v1z'/></svg>";
    let fixture = Fixture::new(&[
        ("rustify-docs.toml", CONFIG),
        ("content/index.md", HOME),
        ("static/hidden.svg", svg),
    ]);
    let candidate = "bg-[url(static/hidden.svg)]";
    let candidates = BTreeSet::from([candidate.to_owned()]);
    let surfaces = Surfaces {
        candidates: candidates.clone(),
        ..Default::default()
    };
    let extensions = Extensions {
        candidates,
        ..Default::default()
    };
    let generation = fixture.generate(&surfaces, &extensions).unwrap();
    let output = generation::resource_output("static/hidden.svg", svg.as_bytes());
    assert_eq!(generation.files[&output].as_ref(), svg.as_bytes());
    assert!(!html(&generation, "index.html").contains(candidate));
    let css = html(&generation, &generation.stylesheet_path);
    let urls: Vec<_> = rustify_docs::style::urls(&css)
        .into_iter()
        .filter(|url| !url.starts_with("data:"))
        .collect();
    assert_eq!(urls, [format!("/docs/{output}")]);
    assert!(css.contains(r".bg-\[url\(static\/hidden\.svg\)\]"), "{css}");
}

#[test]
fn hidden_candidate_urls_keep_query_fragments_quoted_escapes_and_arbitrary_spaces() {
    let svg = "<svg xmlns='http://www.w3.org/2000/svg'><path id='shape' d='M0 0h1v1z'/></svg>";
    for (candidate, file, selector_path) in [
        (
            r"bg-[url('static/hidden\_figure.svg?size=2#shape')]",
            "static/hidden_figure.svg",
            r"static\/hidden\\_figure\.svg\?size\=2\#shape",
        ),
        (
            "[background-image:URL('static/hidden_figure.svg?size=2#shape')]",
            "static/hidden figure.svg",
            r"static\/hidden_figure\.svg\?size\=2\#shape",
        ),
    ] {
        let fixture = Fixture::new(&[
            ("rustify-docs.toml", CONFIG),
            ("content/index.md", HOME),
            (file, svg),
        ]);
        let candidates = BTreeSet::from([candidate.to_owned()]);
        let surfaces = Surfaces {
            candidates: candidates.clone(),
            ..Default::default()
        };
        let extensions = Extensions {
            candidates,
            ..Default::default()
        };
        let generation = fixture.generate(&surfaces, &extensions).unwrap();
        let output = generation::resource_output(file, svg.as_bytes());
        assert_eq!(generation.files[&output].as_ref(), svg.as_bytes());
        assert!(!html(&generation, "index.html").contains(candidate));
        let css = html(&generation, &generation.stylesheet_path);
        let expected = format!(
            "/docs/{}?size=2#shape",
            rustify_docs::escape::encode_path(&output)
        );
        let urls: Vec<_> = rustify_docs::style::urls(&css)
            .into_iter()
            .filter(|url| !url.starts_with("data:"))
            .collect();
        assert_eq!(urls, [expected], "{candidate}");
        assert!(css.contains(selector_path), "{candidate}: {css}");
    }
}
