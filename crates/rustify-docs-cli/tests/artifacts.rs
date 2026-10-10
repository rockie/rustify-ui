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

    // The stylesheet loads nothing from elsewhere, and nothing it names is
    // missing.
    let css_path = third
        .keys()
        .find(|p| p.starts_with("assets/site.") && p.ends_with(".css"))
        .unwrap();
    let css = text_of(&third, css_path);
    for rest in css.split("url(").skip(1) {
        let url = rest.split(')').next().unwrap().trim_matches(['"', '\'']);
        assert!(
            url.starts_with("data:") || url.starts_with('#'),
            "the stylesheet loads {url}"
        );
    }
}
