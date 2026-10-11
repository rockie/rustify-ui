//! What a production build publishes, read back from its output directory:
//! every address under the base and resolvable, the manifest true to the
//! files, the site files listing what may be indexed, file names that change
//! with their content and nothing else, and nothing published that the
//! author did not mean to publish.

mod support;

use std::collections::BTreeMap;

use support::*;

type Files = BTreeMap<String, Vec<u8>>;

/// The content fixture under `base`, with a page kept out of search engines.
fn fixture_at(base: &str) -> Temp {
    let temp = copy_fixture("content");
    temp.edit(
        "site/rustify-docs.toml",
        "url = \"https://example.com/docs/\"\nbase = \"/docs/\"",
        &format!("url = \"https://docs.example.net{base}\"\nbase = \"{base}\""),
    );
    temp.write(
        "site/content/reference/internal.md",
        "---\ndescription: Not for search engines.\nindex: false\n---\n\n# Internal\n\nSee the [API](api.md).\n",
    );
    temp.edit(
        "site/rustify-docs.toml",
        "items = [\"reference/api.md\"]",
        "items = [\"reference/api.md\", \"reference/internal.md\"]",
    );
    temp
}

/// Builds the copy; its only warning is the history a temporary directory
/// lacks.
fn build(temp: &Temp, extra: &[&str]) -> Files {
    let mut args = vec!["build", "--format", "json"];
    args.extend_from_slice(extra);
    let output = run_in(&temp.path("site"), &args);
    succeeded(&output);
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    let rules: Vec<&str> = report["diagnostics"]
        .as_array()
        .unwrap()
        .iter()
        .map(|d| d["rule"].as_str().unwrap())
        .collect();
    assert_eq!(rules, ["no-git-history"], "{report:#}");
    tree(&temp.path("site/dist"))
}

/// Every `href="…"`/`src="…"` value in `html`.
fn urls(html: &str) -> Vec<String> {
    let mut found = Vec::new();
    for attribute in [" href=\"", " src=\"", " content=\""] {
        for rest in html.split(attribute).skip(1) {
            found.push(rest.split('"').next().unwrap().replace("&amp;", "&"));
        }
    }
    found
}

fn decode(path: &str) -> String {
    let bytes = path.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            out.push(u8::from_str_radix(&path[i + 1..i + 3], 16).unwrap());
            i += 3;
        } else {
            out.push(bytes[i]);
            i += 1;
        }
    }
    String::from_utf8(out).unwrap()
}

/// Checks that every same-site address in every page starts with `base` and
/// names a file in the output.
fn addresses_resolve(files: &Files, base: &str, origin: &str) {
    let mut checked = 0;
    for (path, bytes) in files.iter().filter(|(path, _)| path.ends_with(".html")) {
        let html = String::from_utf8(bytes.clone()).unwrap();
        for url in urls(&html) {
            let url = url.strip_prefix(origin).map_or(url.clone(), str::to_owned);
            if !url.starts_with('/') {
                continue;
            }
            assert!(url.starts_with(base), "{path}: {url} is not under {base}");
            let target = decode(url[base.len()..].split('#').next().unwrap());
            let file = if target.is_empty() || target.ends_with('/') {
                format!("{target}index.html")
            } else {
                target
            };
            assert!(
                files.contains_key(&file),
                "{path}: {url} names no file ({file})"
            );
            checked += 1;
        }
    }
    assert!(checked > 50, "only {checked} addresses checked");
}

#[test]
fn a_subpath_build_prefixes_every_address_and_every_address_resolves() {
    let temp = fixture_at("/rustify-ui/");
    let files = build(&temp, &[]);
    for page in [
        "index.html",
        "guide/index.html",
        "guide/setup/index.html",
        "reference/api/index.html",
        "reference/internal/index.html",
        "404.html",
    ] {
        assert!(files.contains_key(page), "{page}");
    }
    addresses_resolve(&files, "/rustify-ui/", "https://docs.example.net");
    let setup = text_of(&files, "guide/setup/index.html");
    assert!(setup.contains(
        "<link rel=\"canonical\" href=\"https://docs.example.net/rustify-ui/guide/setup/\">"
    ));
}

#[test]
fn a_root_build_and_a_base_override_resolve_the_same_way() {
    let temp = fixture_at("/");
    let files = build(&temp, &[]);
    addresses_resolve(&files, "/", "https://docs.example.net");

    // `--base` must agree with site.url.
    let refused = run_in(&temp.path("site"), &["build", "--base", "/other/"]);
    assert_eq!(refused.status.code(), Some(1));
    assert!(
        stderr(&refused).contains("does not match site.url"),
        "{}",
        stderr(&refused)
    );
    temp.edit(
        "site/rustify-docs.toml",
        "url = \"https://docs.example.net/\"\n",
        "",
    );
    let output = run_in(
        &temp.path("site"),
        &["build", "--base", "/other/", "--out", "../other"],
    );
    succeeded(&output);
    let other = tree(&temp.path("other"));
    let home = text_of(&other, "index.html");
    assert!(home.contains("href=\"/other/guide/\""), "{home}");
    assert!(
        !home.contains("rel=\"canonical\""),
        "no URL, no canonical address"
    );
}

#[test]
fn the_manifest_is_true_to_the_files() {
    let temp = fixture_at("/docs/");
    let files = build(&temp, &[]);
    let manifest: serde_json::Value =
        serde_json::from_slice(&files["rustify-docs-manifest.json"]).unwrap();
    assert_eq!(manifest["schema_version"], 1);
    assert_eq!(manifest["mode"], "production");
    assert_eq!(manifest["base"], "/docs/");
    assert_eq!(manifest["cli_version"], env!("CARGO_PKG_VERSION"));
    let listed: BTreeMap<String, (String, u64)> = manifest["files"]
        .as_array()
        .unwrap()
        .iter()
        .map(|f| {
            (
                f["path"].as_str().unwrap().to_owned(),
                (
                    f["sha256"].as_str().unwrap().to_owned(),
                    f["bytes"].as_u64().unwrap(),
                ),
            )
        })
        .collect();
    let mut actual = files.clone();
    actual.remove("rustify-docs-manifest.json");
    assert_eq!(
        listed.keys().collect::<Vec<_>>(),
        actual.keys().collect::<Vec<_>>()
    );
    for (path, bytes) in &actual {
        assert_eq!(listed[path], (sha256(bytes), bytes.len() as u64), "{path}");
    }
    let text = String::from_utf8(files["rustify-docs-manifest.json"].clone()).unwrap();
    let root = temp.0.to_string_lossy().into_owned();
    assert!(
        !text.contains(&root) && !text.contains("/Users/") && !text.contains("/tmp"),
        "{text}"
    );
}

#[test]
fn site_files_list_what_may_be_indexed_and_nothing_private_ships() {
    let temp = fixture_at("/docs/");
    temp.write("site/static/.env", "SECRET=1");
    temp.write("site/static/notes/.env.local", "SECRET=2");
    let files = build(&temp, &[]);
    let sitemap = text_of(&files, "sitemap.xml");
    let locations: Vec<&str> = sitemap
        .split("<loc>")
        .skip(1)
        .map(|rest| rest.split("</loc>").next().unwrap())
        .collect();
    assert_eq!(
        locations,
        [
            "https://docs.example.net/docs/",
            "https://docs.example.net/docs/guide/",
            "https://docs.example.net/docs/guide/setup/",
            "https://docs.example.net/docs/reference/api/",
        ],
        "no draft, no index: false page, no 404"
    );
    assert!(text_of(&files, "robots.txt")
        .ends_with("\nSitemap: https://docs.example.net/docs/sitemap.xml\n"));
    let internal = text_of(&files, "reference/internal/index.html");
    assert!(internal.contains("<meta name=\"robots\" content=\"noindex\">"));
    assert!(!text_of(&files, "guide/index.html").contains("noindex"));

    for path in files.keys() {
        assert!(
            !path.ends_with(".md") && !path.ends_with(".rs") && !path.contains(".env"),
            "{path} published"
        );
        assert!(!path.contains("later"), "the draft {path} published");
    }
    for bytes in files.values() {
        let text = String::from_utf8_lossy(bytes);
        assert!(!text.contains("SECRET") && !text.contains("rustify-region"));
    }
}

#[test]
fn a_file_only_a_home_action_names_is_published_and_its_link_resolves() {
    let temp = fixture_at("/rustify-ui/");
    temp.write("site/content/manual.pdf", "%PDF-1.4 manual");
    temp.edit(
        "site/content/index.md",
        "  - text: Reference\n    link: reference/api.md\n",
        "  - text: Reference\n    link: manual.pdf#page=2\n",
    );
    let files = build(&temp, &[]);
    let home = text_of(&files, "index.html");
    let manual = files
        .keys()
        .find(|path| path.starts_with("assets/files/manual."))
        .expect("the manual is published");
    assert!(
        home.contains(&format!("href=\"/rustify-ui/{manual}#page=2\"")),
        "{home}"
    );
    addresses_resolve(&files, "/rustify-ui/", "https://docs.example.net");
}

#[cfg(unix)]
#[test]
fn nothing_private_or_outside_the_project_ships_under_another_name() {
    use std::os::unix::fs::symlink;
    /// Builds, expecting the build to be refused for `rule` with the
    /// previous output left as it was.
    fn refused(temp: &Temp, rule: &str, before: &Files) {
        let output = run_in(&temp.path("site"), &["build", "--format", "json"]);
        assert_eq!(output.status.code(), Some(1), "{}", stdout(&output));
        let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
        assert!(
            report["diagnostics"]
                .as_array()
                .unwrap()
                .iter()
                .any(|d| d["rule"] == rule && d["severity"] == "error"),
            "{rule} in {report:#}"
        );
        assert_eq!(&tree(&temp.path("site/dist")), before);
    }
    let fresh = || {
        let temp = fixture_at("/docs/");
        temp.write("site/.env", "SECRET=PRIVATE_MARKER");
        temp.write("outside/static/x.txt", "OUTSIDE_MARKER");
        temp.write("outside/far.svg", "<svg>OUTSIDE_MARKER</svg>");
        let before = build(&temp, &[]);
        (temp, before)
    };

    // The asset directory, or a directory on an image's path, leading out.
    let (temp, before) = fresh();
    std::fs::rename(temp.path("site/static"), temp.path("site/static-real")).unwrap();
    symlink(temp.path("outside/static"), temp.path("site/static")).unwrap();
    refused(&temp, "symlink-outside", &before);
    let (temp, before) = fresh();
    symlink(temp.path("outside"), temp.path("site/images")).unwrap();
    temp.edit(
        "site/content/guide/index.md",
        "Read the setup page first.",
        "Read the setup page first. ![Far](../../images/far.svg)",
    );
    refused(&temp, "symlink-outside", &before);

    // A private file under a public name: a static file, an include, the logo.
    let (temp, before) = fresh();
    symlink(temp.path("site/.env"), temp.path("site/static/public.txt")).unwrap();
    refused(&temp, "private-file", &before);
    let (temp, before) = fresh();
    symlink(temp.path("site/.env"), temp.path("site/notes.txt")).unwrap();
    temp.edit(
        "site/content/guide/index.md",
        "Read the setup page first.",
        "Read the setup page first.\n\n<!-- rustify:include path=\"notes.txt\" -->\n",
    );
    refused(&temp, "include-missing", &before);
    let (temp, before) = fresh();
    temp.edit(
        "site/rustify-docs.toml",
        "logo = \"static/logo.svg\"",
        "logo = \".env\"",
    );
    refused(&temp, "private-file", &before);

    // A private page is no page; a link that stays inside the project is
    // followed.
    let (temp, _) = fresh();
    temp.write(
        "site/content/.env.md",
        "---\ndescription: d\n---\n# PRIVATE_MARKER\n",
    );
    temp.write(
        "site/content/.env/notes.md",
        "---\ndescription: d\n---\n# PRIVATE_MARKER\n",
    );
    temp.write("site/brand/mark.svg", "<svg>INSIDE_MARKER</svg>");
    symlink(
        temp.path("site/brand/mark.svg"),
        temp.path("site/static/mark.svg"),
    )
    .unwrap();
    let files = build(&temp, &[]);
    assert!(files.contains_key("mark.svg"));
    for (path, bytes) in &files {
        let text = String::from_utf8_lossy(bytes);
        assert!(!path.contains(".env"), "{path} published");
        assert!(
            !text.contains("PRIVATE_MARKER") && !text.contains("OUTSIDE_MARKER"),
            "{path} holds a marker"
        );
    }
}

#[test]
fn without_a_url_there_is_no_canonical_address_or_sitemap_and_a_warning() {
    let temp = fixture_at("/docs/");
    temp.edit(
        "site/rustify-docs.toml",
        "url = \"https://docs.example.net/docs/\"\n",
        "",
    );
    let output = run_in(&temp.path("site"), &["build", "--format", "json"]);
    succeeded(&output);
    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap();
    let warning = report["diagnostics"]
        .as_array()
        .unwrap()
        .iter()
        .find(|d| d["rule"] == "missing-site-url")
        .expect("the missing URL is a warning");
    assert!(warning["suggestion"].as_str().unwrap().contains("url = "));
    let files = tree(&temp.path("site/dist"));
    assert!(!files.contains_key("sitemap.xml"));
    assert_eq!(text_of(&files, "robots.txt"), "User-agent: *\nAllow: /\n");
    for (path, bytes) in files.iter().filter(|(path, _)| path.ends_with(".html")) {
        let html = String::from_utf8_lossy(bytes);
        assert!(
            !html.contains("rel=\"canonical\"")
                && !html.contains("og:url")
                && !html.contains("og:image"),
            "{path}"
        );
        assert!(
            html.contains("<meta property=\"og:title\""),
            "{path}: a card without an image still has a title"
        );
    }
}

/// Names that are fingerprints, with the fingerprint taken out.
fn shapes(files: &Files) -> BTreeMap<String, String> {
    files
        .keys()
        .map(|path| {
            let parts: Vec<&str> = path.split('.').collect();
            let shape = if parts.len() >= 3
                && parts[parts.len() - 2].len() == 10
                && parts[parts.len() - 2]
                    .bytes()
                    .all(|b| b.is_ascii_hexdigit())
            {
                let mut parts = parts.clone();
                let n = parts.len();
                parts[n - 2] = "#";
                parts.join(".")
            } else {
                path.clone()
            };
            (shape, path.clone())
        })
        .collect()
}

/// Which outputs differ between two builds: (renamed, changed in place).
fn differences(before: &Files, after: &Files) -> (Vec<String>, Vec<String>) {
    let (a, b) = (shapes(before), shapes(after));
    assert_eq!(a.keys().collect::<Vec<_>>(), b.keys().collect::<Vec<_>>());
    let mut renamed = Vec::new();
    let mut changed = Vec::new();
    for (shape, old) in &a {
        let new = &b[shape];
        if old != new {
            renamed.push(shape.clone());
        } else if before[old] != after[new] {
            changed.push(old.clone());
        }
    }
    (renamed, changed)
}

#[test]
fn a_changed_file_renames_itself_and_changes_only_what_references_it() {
    let temp = fixture_at("/docs/");
    let first = build(&temp, &[]);
    assert_eq!(
        build(&temp, &[]),
        first,
        "an unchanged rebuild writes the same bytes"
    );

    temp.edit(
        "site/content/guide/pipeline.svg",
        "pipeline</text>",
        "pipeline v2</text>",
    );
    let second = build(&temp, &[]);
    let (renamed, changed) = differences(&first, &second);
    assert_eq!(renamed, ["assets/files/pipeline.#.svg"]);
    assert_eq!(
        changed,
        [
            "guide/index.html",
            "guide/setup/index.html",
            "rustify-docs-manifest.json"
        ],
        "the pages showing the image, and the manifest"
    );

    // The logo is in every header: every page changes, the stylesheet and
    // scripts do not.
    temp.edit("site/static/logo.svg", "#175cd3", "#0b4fc2");
    let third = build(&temp, &[]);
    let (renamed, changed) = differences(&second, &third);
    assert_eq!(renamed, ["assets/files/logo.#.svg"]);
    let mut expected: Vec<String> = third
        .keys()
        .filter(|path| path.ends_with(".html"))
        .cloned()
        .collect();
    expected.extend([
        "logo.svg".to_owned(),
        "rustify-docs-manifest.json".to_owned(),
    ]);
    expected.sort();
    assert_eq!(changed, expected);

    // A class's background image is the stylesheet's: it renames, so does
    // the stylesheet, and every page that loads it changes; the scripts do
    // not.
    temp.edit("site/static/banner.svg", "#53b1fd", "#2e90fa");
    let fourth = build(&temp, &[]);
    let (renamed, changed) = differences(&third, &fourth);
    assert_eq!(renamed, ["assets/files/banner.#.svg", "assets/site.#.css"]);
    let mut expected: Vec<String> = fourth
        .keys()
        .filter(|path| path.ends_with(".html"))
        .cloned()
        .collect();
    expected.extend([
        "banner.svg".to_owned(),
        "rustify-docs-manifest.json".to_owned(),
    ]);
    expected.sort();
    assert_eq!(changed, expected);

    // The stylesheet loads nothing from elsewhere, and nothing it names is
    // missing.
    let css_path = fourth
        .keys()
        .find(|p| p.starts_with("assets/site.") && p.ends_with(".css"))
        .unwrap();
    let css = text_of(&fourth, css_path);
    let mut published = 0;
    for rest in css.split("url(").skip(1) {
        let url = rest.split(')').next().unwrap().trim_matches(['"', '\'']);
        if let Some(file) = url.strip_prefix("/docs/") {
            assert!(
                fourth.contains_key(file),
                "the stylesheet loads {url}, not published"
            );
            published += 1;
        } else {
            assert!(
                url.starts_with("data:") || url.starts_with('#'),
                "the stylesheet loads {url}"
            );
        }
    }
    assert_eq!(published, 1, "the banner");
}

#[test]
fn a_logo_pair_and_the_share_image_are_published_by_content() {
    let temp = fixture_at("/docs/");
    temp.write(
        "site/static/logo-dark.svg",
        &temp
            .read("site/static/logo.svg")
            .replace("#175cd3", "#53b1fd"),
    );
    temp.edit(
        "site/rustify-docs.toml",
        "logo = \"static/logo.svg\"",
        "logo = { light = \"static/logo.svg\", dark = \"static/logo-dark.svg\" }",
    );
    let files = build(&temp, &[]);
    let named = |prefix: &str| -> String {
        files
            .keys()
            .find(|name| name.starts_with(prefix))
            .unwrap_or_else(|| panic!("no {prefix}*"))
            .clone()
    };
    let light = named("assets/files/logo.");
    let dark = named("assets/files/logo-dark.");
    let share = named("assets/share.");
    for page in ["index.html", "guide/setup/index.html", "404.html"] {
        let html = text_of(&files, page);
        assert!(
            html.contains(&format!(r#"<img src="/docs/{light}" alt="" width="24" height="24" class="h-6 w-auto dark:hidden">"#)),
            "{page}"
        );
        assert!(
            html.contains(&format!(r#"<img src="/docs/{dark}" alt="" width="24" height="24" class="hidden h-6 w-auto dark:block">"#)),
            "{page}"
        );
    }
    // Without `site.image`, link previews show the framework's own card,
    // at its absolute address.
    assert!(text_of(&files, "index.html").contains(&format!(
        r#"<meta property="og:image" content="https://docs.example.net/docs/{share}">"#
    )));

    // A change to the dark logo renames it alone.
    temp.edit("site/static/logo-dark.svg", "#53b1fd", "#84caff");
    let after = build(&temp, &[]);
    assert!(after.contains_key(&light));
    assert!(!after.contains_key(&dark));
    assert!(after.contains_key(&share));

    // With `site.image`, that file is the card and the framework's is not
    // published.
    std::fs::write(temp.path("site/static/card.png"), SHARE_PNG).unwrap();
    temp.edit(
        "site/rustify-docs.toml",
        "base = \"/docs/\"",
        "base = \"/docs/\"\nimage = \"static/card.png\"",
    );
    let with_card = build(&temp, &[]);
    let card = with_card
        .keys()
        .find(|name| name.starts_with("assets/files/card."))
        .expect("the card, by content");
    assert!(!with_card
        .keys()
        .any(|name| name.starts_with("assets/share.")));
    assert!(text_of(&with_card, "index.html").contains(&format!(
        r#"<meta property="og:image" content="https://docs.example.net/docs/{card}">"#
    )));
}

/// A 1×1 PNG.
const SHARE_PNG: &[u8] = &[
    0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x48, 0x44, 0x52,
    0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x06, 0x00, 0x00, 0x00, 0x1f, 0x15, 0xc4,
    0x89, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x44, 0x41, 0x54, 0x78, 0x9c, 0x63, 0xf8, 0xff, 0xff, 0x3f,
    0x00, 0x05, 0xfe, 0x02, 0xfe, 0xa7, 0x35, 0x81, 0x84, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4e,
    0x44, 0xae, 0x42, 0x60, 0x82,
];

#[test]
fn a_built_site_carries_no_gpu_runtime() {
    let files = build(&fixture_at("/"), &[]);
    for (path, bytes) in &files {
        assert!(!path.ends_with(".wasm"), "{path}");
        let text = String::from_utf8_lossy(bytes).to_ascii_lowercase();
        for needle in [
            "makepad",
            "webgl",
            "wasm-bindgen",
            "webassembly.instantiate",
        ] {
            assert!(!text.contains(needle), "{path} mentions {needle}");
        }
    }
}

#[test]
fn encoded_question_marks_in_file_names_keep_real_url_suffixes() {
    let temp = Temp::new("encoded-files");
    temp.write("rustify-docs.toml", "schema_version = 1\n[site]\nname = 'Files'\nurl = 'https://example.com/docs/'\nbase = '/docs/'\n[[nav.top]]\nlabel = 'Content'\nlink = 'manual%3Fv.pdf?download=1#page=2'\n[[nav.social]]\nlabel = 'Static'\nlink = '/manual%3Fv.pdf?download=1#page=2'\n");
    temp.write("content/index.md", "---\ndescription: Files.\nlayout: home\nactions:\n  - text: Content\n    link: manual%3Fv.pdf?download=1#page=2\n  - text: Static\n    link: /manual%3Fv.pdf?download=1#page=2\n---\n# Files\n\n[Content](manual%3Fv.pdf) [Query](manual%3Fv.pdf?download=1#page=2) [Static](/manual%3Fv.pdf?download=1#page=2) [Normal](plain.pdf?download=1#page=2).\n");
    temp.write("content/manual?v.pdf", "content attachment");
    temp.write("content/plain.pdf", "ordinary attachment");
    temp.write("static/manual?v.pdf", "static attachment");
    succeeded(&run_in(&temp.0, &["build", "--format", "json"]));
    let files = tree(&temp.path("dist"));
    let content = files
        .iter()
        .find(|(path, bytes)| {
            path.starts_with("assets/files/manual?v.") && **bytes == b"content attachment"
        })
        .unwrap()
        .0;
    let static_file = files
        .iter()
        .find(|(path, bytes)| {
            path.starts_with("assets/files/manual?v.") && **bytes == b"static attachment"
        })
        .unwrap()
        .0;
    assert_eq!(files[content], b"content attachment");
    assert_eq!(files["manual?v.pdf"], b"static attachment");
    let html = text_of(&files, "index.html");
    let encoded = content.replace('?', "%3F");
    assert!(
        html.contains(&format!("href=\"/docs/{encoded}\"")),
        "{html}"
    );
    assert_eq!(
        html.matches(&format!("href=\"/docs/{encoded}?download=1#page=2\""))
            .count(),
        4
    );
    assert_eq!(
        html.matches(&format!(
            "href=\"/docs/{}?download=1#page=2\"",
            static_file.replace('?', "%3F")
        ))
        .count(),
        4
    );
    let plain = files
        .keys()
        .find(|path| path.starts_with("assets/files/plain."))
        .unwrap();
    assert!(html.contains(&format!("href=\"/docs/{plain}?download=1#page=2\"")));
}

#[test]
fn css_url_tokens_preserve_text_entities_and_document_fragments() {
    let temp = Temp::new("css-tokens");
    temp.write("rustify-docs.toml", "schema_version = 1\n[site]\nname = 'CSS'\nurl = 'https://example.com/docs/'\nbase = '/docs/'\n");
    temp.write("static/bg.svg", "<svg xmlns='http://www.w3.org/2000/svg'/>");
    temp.write(
        "content/sprite.svg",
        "<svg xmlns='http://www.w3.org/2000/svg'><mask id='icon'/></svg>",
    );
    temp.write("content/index.md", "---\ndescription: CSS.\n---\n# CSS\n\n<div class=\"before:content-['url(foo)'] h-10 [background-image:URL(/bg.svg)]\">Text</div>\n<div class=\"bg-[url(&#47;bg.svg)]\">Entity</div>\n<div class=\"[background-image:url(&quot;/bg.svg&quot;),url('sprite.svg#icon')]\">Multiple</div>\n<svg><defs><mask id=\"m\"></mask><filter id=\"f\"></filter></defs><rect class=\"[mask:url(#m)] [filter:url(#f)]\"></rect></svg>\n");
    succeeded(&run_in(&temp.0, &["build", "--format", "json"]));
    let files = tree(&temp.path("dist"));
    let css = files
        .iter()
        .find(|(path, _)| path.starts_with("assets/site.") && path.ends_with(".css"))
        .unwrap()
        .1;
    let css = String::from_utf8_lossy(css);
    let urls = rustify_docs::style::urls(&css);
    assert!(
        css.contains("url(foo)"),
        "visible content text survives: {css}"
    );
    assert!(!urls.contains(&"foo".into()), "{urls:?}");
    assert!(
        urls.contains(&"#m".into()) && urls.contains(&"#f".into()),
        "{urls:?}"
    );
    assert!(
        urls.iter()
            .any(|url| url.starts_with("/docs/assets/files/bg.") && url.ends_with(".svg")),
        "{urls:?}"
    );
    assert!(
        urls.iter()
            .any(|url| url.starts_with("/docs/assets/files/sprite.") && url.ends_with("#icon")),
        "{urls:?}"
    );
    for fragment in ["m", "f"] {
        assert!(text_of(&files, "index.html").contains(&format!("id=\"{fragment}\"")));
    }
    temp.edit("content/index.md", "url(#m)", "url(#absent)");
    let old = tree(&temp.path("dist"));
    let refused = run_in(&temp.0, &["build", "--format", "json"]);
    assert_eq!(refused.status.code(), Some(1));
    let report: serde_json::Value = serde_json::from_slice(&refused.stdout).unwrap();
    assert!(
        report["diagnostics"]
            .as_array()
            .unwrap()
            .iter()
            .any(|d| d["rule"] == "broken-anchor"),
        "{report}"
    );
    assert_eq!(tree(&temp.path("dist")), old);
}

#[test]
fn a_css_url_with_a_generated_prefix_still_needs_a_published_file() {
    let temp = fixture_at("/docs/");
    let old = build(&temp, &[]);
    temp.write("site/content/index.md", "---\ndescription: Missing CSS file.\n---\n# Home\n\n<div class=\"[background-image:URL(/docs/assets/missing.svg)]\">Missing</div>\n");
    let refused = run_in(&temp.path("site"), &["build", "--format", "json"]);
    assert_eq!(refused.status.code(), Some(1), "{}", stdout(&refused));
    let report: serde_json::Value = serde_json::from_slice(&refused.stdout).unwrap();
    assert!(
        report["diagnostics"]
            .as_array()
            .unwrap()
            .iter()
            .any(|d| d["severity"] == "error" && d["file"] == "content/index.md"),
        "{report}"
    );
    assert_eq!(tree(&temp.path("site/dist")), old);
}
