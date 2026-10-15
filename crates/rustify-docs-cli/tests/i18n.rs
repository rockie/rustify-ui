mod support;

use rustify_docs::content::{self, FragmentCache, Mode, Project};
use rustify_docs::highlight::Highlighter;
use serde_json::Value;
use support::{run_in, Temp};

fn project() -> Temp {
    let project = Temp::new("i18n-inputs");
    project.write("rustify-docs.toml", "schema_version=1\n[site]\nname='Language inputs'\nurl='https://example.com/docs/'\nbase='/docs/'\nlang='en'\n[content]\ninclude=['index.md','zh/index.md']\n[[i18n.locales]]\nlang='en'\nlabel='English'\nprefix=''\n[[i18n.locales]]\nlang='zh'\nlabel='中文'\nprefix='zh'\nmessages='locales/zh.toml'\n");
    project.write(
        "content/index.md",
        "---\ndescription: Home page\n---\n# Home\n",
    );
    project.write(
        "content/zh/index.md",
        "---\ndescription: 首页\n---\n# 首页\n",
    );
    project.write("locales/zh.toml", "copy='拷贝代码'\n");
    project
}

fn check(project: &Temp) -> (Option<i32>, Value) {
    let output = run_in(&project.0, &["check", "--format", "json"]);
    let report = serde_json::from_slice(&output.stdout).unwrap_or_else(|error| {
        panic!(
            "invalid JSON ({error}):\n{}\n{}",
            support::stdout(&output),
            support::stderr(&output)
        )
    });
    (output.status.code(), report)
}

#[test]
fn inputs_message_errors_keep_the_message_source_location_and_recover() {
    let project = project();
    for text in [
        "# Locale overrides\ncopy = [\n",
        "# Locale overrides\nunknown = 'value'\n",
        "# Locale overrides\ncopy = 3\n",
        "# Locale overrides\ncopy = 'Copy {extra}'\n",
    ] {
        project.write("locales/zh.toml", text);
        let (code, report) = check(&project);
        assert_eq!(code, Some(1), "{text}: {report}");
        let issues: Vec<_> = report["diagnostics"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|issue| issue["rule"] == "i18n-messages")
            .collect();
        assert!(!issues.is_empty(), "{text}: {report}");
        for issue in issues {
            assert_eq!(issue["file"], "locales/zh.toml", "{issue}");
            assert!(
                issue["line"].as_u64().is_some_and(|line| line >= 2),
                "{issue}"
            );
            assert!(
                issue["column"].as_u64().is_some_and(|column| column > 0),
                "{issue}"
            );
        }
    }
    project.write("locales/zh.toml", "copy='拷贝代码'\n");
    let (code, report) = check(&project);
    assert_eq!(code, Some(0), "{report}");
    assert_eq!(report["summary"]["errors"], 0);
}

#[test]
fn inputs_missing_message_files_report_the_configuration_and_recover() {
    let project = project();
    std::fs::remove_file(project.path("locales/zh.toml")).unwrap();
    let (code, report) = check(&project);
    assert_eq!(code, Some(1), "{report}");
    assert!(
        report["diagnostics"]
            .as_array()
            .unwrap()
            .iter()
            .any(|issue| {
                issue["rule"] == "i18n-messages"
                    && issue["file"] == "rustify-docs.toml"
                    && issue["line"].as_u64().is_some_and(|line| line > 0)
                    && issue["column"].as_u64().is_some_and(|column| column > 0)
                    && issue["message"]
                        .as_str()
                        .unwrap()
                        .contains("locales/zh.toml")
            }),
        "{report}"
    );
    project.write("locales/zh.toml", "copy='拷贝代码'\n");
    let (code, report) = check(&project);
    assert_eq!(code, Some(0), "{report}");
}

#[test]
fn inputs_messages_are_read_as_inputs_without_expanding_the_content_allowlist() {
    let fixture = project();
    fixture.write(
        "content/unselected.md",
        "---\ntitle: [\n---\n# Invalid excluded content\n",
    );
    fixture.write(
        "content/zh/unselected.md",
        "# Excluded translation\n\n[Missing](absent.md)\n",
    );
    let (project, _) = Project::load(&fixture.0).unwrap();
    let (snapshot, issues) = content::snapshot(&project);
    assert!(issues.is_empty(), "{issues:?}");
    assert_eq!(&*snapshot.messages["locales/zh.toml"], "copy='拷贝代码'\n");
    assert!(!snapshot.files.contains_key("locales/zh.toml"));
    assert!(snapshot.inputs.byte_paths().any(|(alias, real)| {
        alias == "locales/zh.toml" && real == fixture.path("locales/zh.toml")
    }));
    let analysis = content::analyze(
        &project,
        &snapshot,
        Mode::Production,
        &mut FragmentCache::default(),
        &Highlighter::new(),
        1,
    );
    assert_eq!(analysis.errors(), 0, "{:?}", analysis.diagnostics);
    assert!(!analysis.resources.contains("locales/zh.toml"));
    assert!(!analysis.outside.contains_key("locales/zh.toml"));
    assert!(analysis.referenced.contains("locales/zh.toml"));
    let (code, report) = check(&fixture);
    assert_eq!(code, Some(0), "{report}");
}

#[test]
fn inputs_message_sources_are_refused_as_markdown_resources() {
    let project = project();
    project.write(
        "content/index.md",
        "---\ndescription: Home page\n---\n# Home\n\n[Message source](../locales/zh.toml)\n",
    );
    let (code, report) = check(&project);
    assert_eq!(code, Some(1), "{report}");
    assert!(
        report["diagnostics"]
            .as_array()
            .unwrap()
            .iter()
            .any(|issue| {
                issue["rule"] == "i18n-messages" && issue["file"] == "content/index.md"
            }),
        "{report}"
    );
}

#[cfg(unix)]
#[test]
fn inputs_message_receipts_detect_same_byte_symlink_retargets() {
    let fixture = project();
    for target in ["first.toml", "second.toml"] {
        fixture.write(&format!("locales/{target}"), "copy='拷贝代码'\n");
    }
    let alias = fixture.path("locales/zh.toml");
    std::fs::remove_file(&alias).unwrap();
    std::os::unix::fs::symlink("first.toml", &alias).unwrap();
    let (project, _) = Project::load(&fixture.0).unwrap();
    let (snapshot, issues) = content::snapshot(&project);
    assert!(issues.is_empty(), "{issues:?}");
    assert!(snapshot.linked.contains("locales/first.toml"));
    let analysis = content::analyze(
        &project,
        &snapshot,
        Mode::Production,
        &mut FragmentCache::default(),
        &Highlighter::new(),
        1,
    );
    assert!(analysis.referenced.contains("locales/first.toml"));
    assert!(content::validate_inputs(&project, &snapshot, &analysis).is_empty());
    std::fs::remove_file(&alias).unwrap();
    std::os::unix::fs::symlink("second.toml", &alias).unwrap();
    let changed = content::validate_inputs(&project, &snapshot, &analysis);
    assert!(
        changed.iter().any(|issue| {
            issue.rule == "render-generation-changed"
                && issue.file.as_deref() == Some("locales/zh.toml")
        }),
        "{changed:?}"
    );
}

#[test]
fn inputs_messages_cannot_be_replaced_by_an_output_override() {
    let project = project();
    support::succeeded(&run_in(&project.0, &["build", "--format", "json"]));
    support::copy_tree(&project.path("dist"), &project.path("locales"));
    let before = support::tree(&project.path("locales"));
    let refused = run_in(
        &project.0,
        &["build", "--out", "locales", "--format", "json"],
    );
    assert_eq!(
        refused.status.code(),
        Some(1),
        "{}",
        support::stdout(&refused)
    );
    assert_eq!(support::tree(&project.path("locales")), before);
}

#[cfg(unix)]
#[test]
fn inputs_message_symlinks_inside_an_owned_output_are_never_deleted() {
    let project = project();
    support::succeeded(&run_in(&project.0, &["build", "--format", "json"]));
    project.write("dist/zh.toml", "copy='拷贝代码'\n");
    std::fs::remove_file(project.path("locales/zh.toml")).unwrap();
    std::os::unix::fs::symlink("../dist/zh.toml", project.path("locales/zh.toml")).unwrap();
    let before = support::tree(&project.path("dist"));
    let refused = run_in(&project.0, &["build", "--format", "json"]);
    assert_eq!(
        refused.status.code(),
        Some(1),
        "{}",
        support::stdout(&refused)
    );
    assert_eq!(support::tree(&project.path("dist")), before);
}

#[test]
fn render_registered_pages_slots_and_demos_share_each_target_language() {
    let fixture = Temp::new("i18n-render");
    let repository = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let prepared = std::process::Command::new("python3")
        .arg(repository.join("tests/docs-ssg/i18n-fixture.py"))
        .arg("--out")
        .arg(fixture.path("site"))
        .arg("--cli")
        .arg(support::BIN)
        .output()
        .unwrap();
    support::succeeded(&prepared);
    let output = std::process::Command::new(support::BIN)
        .args(["build", "--format", "json"])
        .env("RUSTIFY_DOCS_CARGO", "mbx")
        .current_dir(fixture.path("site"))
        .output()
        .unwrap();
    support::succeeded(&output);
    let mut all_keys = std::collections::BTreeSet::new();
    for (lang, prefix, marker, source_lang) in [
        ("en", "", "English locale page", "en"),
        ("zh", "zh/", "中文语言页面", "zh"),
        ("fr", "fr/", "English locale page", "en"),
    ] {
        let html = fixture.read(&format!("site/dist/{prefix}locale/index.html"));
        let document = dom_query::Document::from(html.as_str());
        assert_eq!(document.select("html").attr("lang").as_deref(), Some(lang));
        assert_eq!(
            document.select("article").attr("lang").as_deref(),
            Some(source_lang)
        );
        assert!(html.contains(marker), "{lang}");
        for node in document.select("[data-context-route]").nodes() {
            assert_eq!(node.attr("data-context-route").as_deref(), Some("/locale/"));
            assert_eq!(node.attr("data-context-lang").as_deref(), Some(lang));
        }
        let islands = document.select("[data-rustify-island-metadata]");
        assert_eq!(islands.length(), 2, "{lang}");
        for node in islands.nodes() {
            let metadata: Value =
                serde_json::from_str(&node.attr("data-rustify-island-metadata").unwrap()).unwrap();
            assert_eq!(metadata["environment"]["lang"], lang);
            assert!(all_keys.insert(metadata["key"].as_str().unwrap().to_owned()));
            let seed = metadata["seed"].as_str().unwrap();
            assert_eq!(
                document
                    .select(&format!("[data-rustify-scope='{seed}']"))
                    .length(),
                1
            );
        }
        assert_eq!(document.select(".rd-language-switch a").length(), 3);
    }
    let fallback = fixture.read("site/dist/fr/locale/index.html");
    let document = dom_query::Document::from(fallback.as_str());
    assert_eq!(
        document.select(".rd-untranslated").attr("lang").as_deref(),
        Some("fr")
    );
    assert_eq!(
        document
            .select("[data-rd-island-loading]")
            .first()
            .attr("lang")
            .as_deref(),
        Some("fr")
    );
    assert_eq!(
        document
            .select("[data-rd-island-reload]")
            .first()
            .attr("lang")
            .as_deref(),
        Some("en")
    );
    let plain = fixture.read("site/dist/zh/plain/index.html");
    assert_eq!(
        dom_query::Document::from(plain.as_str())
            .select("article")
            .attr("lang")
            .as_deref(),
        Some("en")
    );
}

fn content_project() -> Temp {
    let project = Temp::new("i18n-content");
    project.write("rustify-docs.toml", "schema_version=1\n[site]\nname='Docs'\nlang='en'\nbase='/docs/'\nurl='https://example.com/docs/'\n[[i18n.locales]]\nlang='en'\nlabel='English'\nprefix=''\n[[i18n.locales]]\nlang='zh'\nlabel='中文'\nprefix='zh'\n[[i18n.locales]]\nlang='fr'\nlabel='Français'\nprefix='fr'\n");
    for (path, title) in [
        ("index.md", "Home"),
        ("guide.md", "Guide"),
        ("zh/index.md", "首页"),
    ] {
        project.write(
            &format!("content/{path}"),
            &format!("---\ndescription: {title} description\n---\n# {title}\n"),
        );
    }
    project.write("static/foo/404.html", "<p>ordinary asset</p>");
    project
}

#[test]
fn serving_reads_language_errors_from_a_detached_output_and_ignores_configuration_drift() {
    let project = content_project();
    support::succeeded(&run_in(&project.0, &["build", "--format", "json"]));
    let detached = Temp::new("detached-i18n-output");
    support::copy_tree(&project.path("dist"), &detached.path("dist"));
    let preview = support::Preview::start(&detached.0, &["--dir", "dist"]).unwrap();
    for (path, lang) in [
        ("zh/missing/a/b/", "zh"),
        ("fr/missing/a/b/", "fr"),
        ("zh-other/missing/", "en"),
        ("foo/missing/", "en"),
    ] {
        let (status, headers, body) = support::get(&format!("{}{path}", preview.url));
        assert_eq!(status, 404, "{path}");
        assert!(headers["content-security-policy"].contains("default-src 'none'"));
        let html = String::from_utf8(body).unwrap();
        assert_eq!(
            dom_query::Document::from(html.as_str())
                .select("html")
                .attr("lang")
                .as_deref(),
            Some(lang),
            "{path}"
        );
    }
    let (status, _, body) = support::get(&format!("{}foo/404.html", preview.url));
    assert_eq!(status, 200);
    assert_eq!(body, b"<p>ordinary asset</p>");
    drop(preview);
    project.edit("rustify-docs.toml", "prefix='zh'", "prefix='cn'");
    let preview = support::Preview::start(&project.0, &[]).unwrap();
    for (path, lang) in [("zh/missing/", "zh"), ("cn/missing/", "en")] {
        let (status, _, body) = support::get(&format!("{}{path}", preview.url));
        assert_eq!(status, 404);
        let html = String::from_utf8(body).unwrap();
        assert_eq!(
            dom_query::Document::from(html.as_str())
                .select("html")
                .attr("lang")
                .as_deref(),
            Some(lang)
        );
    }
}

#[test]
fn serving_refuses_damaged_mappings_but_does_not_infer_languages_from_legacy_files() {
    let project = content_project();
    support::succeeded(&run_in(&project.0, &["build", "--format", "json"]));
    let manifest = project.read("dist/rustify-docs-manifest.json");
    let original: Value = serde_json::from_str(&manifest).unwrap();
    for bad in [
        serde_json::json!(null),
        serde_json::json!([]),
        serde_json::json!({"zh":"zh/404.html"}),
        serde_json::json!({"":"404.html","zh":"../404.html"}),
        serde_json::json!({"":"404.html","de":"de/404.html"}),
    ] {
        let mut candidate = original.clone();
        candidate["locale_not_found"] = bad;
        project.write("dist/rustify-docs-manifest.json", &candidate.to_string());
        let error = support::Preview::start(&project.0, &["--dir", "dist"])
            .err()
            .expect("damaged output is refused");
        assert!(error.contains("build"), "{error}");
    }
    project.write("dist/rustify-docs-manifest.json", &manifest);
    let translated = project.read("dist/zh/404.html");
    std::fs::remove_file(project.path("dist/zh/404.html")).unwrap();
    assert!(support::Preview::start(&project.0, &["--dir", "dist"]).is_err());
    project.write("dist/zh/404.html", &translated);
    let mut legacy = original;
    legacy.as_object_mut().unwrap().remove("locale_not_found");
    project.write("dist/rustify-docs-manifest.json", &legacy.to_string());
    let preview = support::Preview::start(&project.0, &["--dir", "dist"]).unwrap();
    let (status, _, body) = support::get(&format!("{}zh/missing/", preview.url));
    assert_eq!(status, 404);
    let html = String::from_utf8(body).unwrap();
    assert_eq!(
        dom_query::Document::from(html.as_str())
            .select("html")
            .attr("lang")
            .as_deref(),
        Some("en")
    );
}

fn good_revision(html: &str) -> u64 {
    html.split("&amp;good=")
        .nth(1)
        .unwrap()
        .split('"')
        .next()
        .unwrap()
        .parse()
        .unwrap()
}

#[test]
fn serving_dev_first_failure_uses_the_requested_language_and_recovers() {
    let project = content_project();
    let config = project.read("rustify-docs.toml");
    project.write(
        "rustify-docs.toml",
        &format!("{config}messages='locales/fr.toml'\n"),
    );
    project.write(
        "locales/fr.toml",
        "[dev]\nshell_title='Site indisponible'\nshell_body='Corrigez les erreurs.'\n",
    );
    project.write("content/guide.md", "---\nbroken: [\n");
    let dev = support::Dev::start(&project.0, &[], &[]).unwrap();
    let (status, html) = support::get_text(&format!("{}fr/missing/a/b/", dev.url));
    assert_eq!(status, 500);
    let document = dom_query::Document::from(html.as_str());
    assert_eq!(document.select("html").attr("lang").as_deref(), Some("fr"));
    assert_eq!(document.select("h1").text().as_ref(), "Site indisponible");
    let mut events = dev.events("/docs/fr/missing/a/b/", 0);
    let first = events.next();
    assert_eq!(first["ok"], false);
    project.write(
        "content/guide.md",
        "---\ndescription: Recovered guide\n---\n# Recovered guide\n",
    );
    events.until(|state| state["building"] == false && state["ok"] == true);
    let (status, html) = support::get_text(&format!("{}fr/missing/a/b/", dev.url));
    assert_eq!(status, 404);
    assert_eq!(
        dom_query::Document::from(html.as_str())
            .select("html")
            .attr("lang")
            .as_deref(),
        Some("fr")
    );
}

#[test]
fn serving_dev_failed_prefix_candidate_keeps_the_previous_mapping_and_then_reloads() {
    let project = content_project();
    std::fs::remove_file(project.path("content/zh/index.md")).unwrap();
    let dev = support::Dev::start(&project.0, &[], &[]).unwrap();
    let (_, html) = support::get_text(&format!("{}zh/missing/a/b/", dev.url));
    let good = good_revision(&html);
    let mut events = dev.events("/docs/zh/missing/a/b/", good);
    assert_eq!(events.next()["ok"], true);
    project.write("content/guide.md", "---\nbroken: [\n");
    let failed = events.until(|state| state["building"] == false && state["ok"] == false);
    project.edit("rustify-docs.toml", "prefix='zh'", "prefix='cn'");
    let failed_again = events.until(|state| {
        state["building"] == false
            && state["ok"] == false
            && state["revision"].as_u64() > failed["revision"].as_u64()
    });
    assert_eq!(failed_again["reload"], false);
    for (path, lang) in [("zh/missing/", "zh"), ("cn/missing/", "en")] {
        let (status, html) = support::get_text(&format!("{}{path}", dev.url));
        assert_eq!(status, 404);
        assert_eq!(
            dom_query::Document::from(html.as_str())
                .select("html")
                .attr("lang")
                .as_deref(),
            Some(lang)
        );
    }
    project.write(
        "content/guide.md",
        "---\ndescription: Recovered guide\n---\n# Recovered guide\n",
    );
    let recovered = events.until(|state| state["building"] == false && state["ok"] == true);
    assert_eq!(recovered["reload"], true);
    for (path, lang) in [("cn/missing/", "zh"), ("zh/missing/", "en")] {
        let (status, html) = support::get_text(&format!("{}{path}", dev.url));
        assert_eq!(status, 404);
        assert_eq!(
            dom_query::Document::from(html.as_str())
                .select("html")
                .attr("lang")
                .as_deref(),
            Some(lang)
        );
    }
}

#[test]
fn diagnostics_report_exactly_three_missing_pages_and_two_keys_with_locations_and_deny_warnings() {
    let project = Temp::new("i18n-counts");
    project.write("rustify-docs.toml", "schema_version=1\n[site]\nname='Docs'\nlang='en'\nbase='/docs/'\nurl='https://example.com/docs/'\n[[i18n.locales]]\nlang='en'\nlabel='English'\nprefix=''\n[[i18n.locales]]\nlang='fr'\nlabel='Français'\nprefix='fr'\nmessages='locales/fr.toml'\n");
    for (path, title) in [
        ("index.md", "Home"),
        ("guide/a.md", "First guide"),
        ("guide/b.md", "Second guide"),
        ("faq.md", "FAQ"),
        ("fr/index.md", "Accueil"),
    ] {
        project.write(
            &format!("content/{path}"),
            &format!("---\ndescription: {title} description\n---\n# {title}\n"),
        );
    }
    let catalog = rustify_docs::labels::resolve("en", &Default::default());
    let messages = catalog
        .iter()
        .filter(|(key, _)| !matches!(*key, "copy" | "theme_light"))
        .map(|(key, value)| {
            format!(
                "{} = {}\n",
                serde_json::to_string(key).unwrap(),
                serde_json::to_string(&value.text).unwrap()
            )
        })
        .collect::<String>();
    project.write("locales/fr.toml", &messages);
    for args in [
        vec!["init", "-q"],
        vec!["add", "."],
        vec![
            "-c",
            "user.name=Docs fixture",
            "-c",
            "user.email=docs@example.invalid",
            "commit",
            "-qm",
            "Fixture inputs",
        ],
    ] {
        assert!(std::process::Command::new("git")
            .args(args)
            .current_dir(&project.0)
            .output()
            .unwrap()
            .status
            .success());
    }
    let (code, report) = check(&project);
    assert_eq!(code, Some(0), "{report}");
    assert_eq!(report["schema_version"], 1);
    assert_eq!(report["summary"]["warnings"], 5, "{report}");
    let diagnostics = report["diagnostics"].as_array().unwrap();
    assert_eq!(diagnostics.len(), 5, "{report}");
    assert_eq!(
        diagnostics
            .iter()
            .filter(|issue| issue["rule"] == "i18n-missing-page")
            .count(),
        3
    );
    assert_eq!(
        diagnostics
            .iter()
            .filter(|issue| issue["rule"] == "i18n-missing-label")
            .count(),
        2
    );
    for issue in diagnostics {
        assert_eq!(issue["severity"], "warning");
        assert_eq!(issue["file"], "rustify-docs.toml");
        assert!(issue["line"].as_u64().is_some_and(|line| line > 0));
        assert!(issue["column"].as_u64().is_some_and(|column| column > 0));
        assert!(!issue["suggestion"].as_str().unwrap().is_empty());
    }
    let denied = run_in(
        &project.0,
        &["check", "--format", "json", "--deny-warnings"],
    );
    assert_eq!(denied.status.code(), Some(1));
    support::succeeded(&run_in(&project.0, &["build", "--format", "json"]));
    let before = support::tree(&project.path("dist"));
    project.write("locales/fr.toml", "copy=42\n");
    let (code, report) = check(&project);
    assert_eq!(code, Some(1));
    assert!(report["diagnostics"]
        .as_array()
        .unwrap()
        .iter()
        .all(|issue| issue["rule"] != "i18n-missing-label"));
    assert_eq!(
        run_in(&project.0, &["build", "--format", "json"])
            .status
            .code(),
        Some(1)
    );
    assert_eq!(support::tree(&project.path("dist")), before);
    let full_messages = catalog
        .iter()
        .map(|(key, value)| {
            format!(
                "{} = {}\n",
                serde_json::to_string(key).unwrap(),
                serde_json::to_string(&value.text).unwrap()
            )
        })
        .collect::<String>();
    project.write("locales/fr.toml", &full_messages);
    for (path, title) in [
        ("guide/a.md", "Premier guide"),
        ("guide/b.md", "Deuxième guide"),
        ("faq.md", "Questions"),
    ] {
        project.write(
            &format!("content/fr/{path}"),
            &format!("---\ndescription: {title}\n---\n# {title}\n"),
        );
    }
    let dev = support::Dev::start(&project.0, &[], &[]).unwrap();
    let (_, html) = support::get_text(&format!("{}fr/", dev.url));
    let mut events = dev.events("/docs/fr/", good_revision(&html));
    let initial = events.next();
    assert_eq!(initial["errors"], 0);
    assert_eq!(initial["warnings"], 0, "{initial}");
    project.write("locales/fr.toml", "copy=42\n");
    let failed = events.until(|state| state["building"] == false && state["ok"] == false);
    assert!(failed["errors"].as_u64().unwrap() > 0);
    assert_eq!(failed["reload"], false);
    project.write("locales/fr.toml", &full_messages);
    let recovered = events.until(|state| state["building"] == false && state["ok"] == true);
    assert_eq!(recovered["errors"], 0);
    assert_eq!(recovered["warnings"], 0);
    assert!(recovered["diagnostics"].as_array().unwrap().is_empty());
}
