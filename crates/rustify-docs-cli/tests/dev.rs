//! `rustify-docs dev` as a process: it serves from memory, writes nothing,
//! follows saves, keeps the last good build through failures and panics,
//! and tells each open page whether the newest build changed it.

mod support;

use std::time::{Duration, Instant};

use serde_json::Value;
use support::*;

/// The build a served page says it is a copy of.
fn good_of(html: &str) -> u64 {
    let at = html
        .find("&amp;good=")
        .unwrap_or_else(|| panic!("no dev script in {html}"));
    html[at + "&amp;good=".len()..]
        .split('"')
        .next()
        .unwrap()
        .parse()
        .unwrap()
}

fn revision(state: &Value) -> u64 {
    state["revision"].as_u64().unwrap()
}

#[track_caller]
fn wait_for_log(dev: &Dev, wanted: &str) {
    let start = Instant::now();
    while !dev.log().contains(wanted) {
        assert!(
            start.elapsed() < Duration::from_secs(20),
            "{wanted:?} not in:\n{}",
            dev.log()
        );
        std::thread::sleep(Duration::from_millis(20));
    }
}

#[test]
fn a_new_site_is_served_from_memory_and_nothing_is_written() {
    let temp = Temp::new("dev-new");
    succeeded(&run_in(&temp.0, &["new", "docs"]));
    let site = temp.path("docs");
    let before = tree(&site);
    let dev = Dev::start(&site, &[], &[]).unwrap();
    assert!(dev.url.starts_with("http://127.0.0.1:") && dev.url.ends_with('/'));
    let (status, home) = get_text(&dev.url);
    assert_eq!(status, 200);
    assert!(home.contains("/__rustify-docs/dev.js?session="), "{home}");
    for page in ["guide/getting-started/", "guide/writing-pages/"] {
        let (status, html) = get_text(&format!("{}{page}", dev.url));
        assert_eq!(status, 200, "{page}");
        assert!(html.contains("<h1"), "{page}");
    }
    assert_eq!(get_text(&format!("{}no/such/page/", dev.url)).0, 404);
    let (status, headers, script) = get(&format!("{}/__rustify-docs/dev.js", dev.origin()));
    assert_eq!(status, 200);
    assert!(headers["content-security-policy"].contains("script-src 'self'"));
    assert!(String::from_utf8_lossy(&script).contains("EventSource"));
    // Serving wrote nothing, and took no lock: a build runs alongside.
    assert_eq!(tree(&site), before);
    succeeded(&run_in(&site, &["build"]));
    let (status, _) = get_text(&dev.url);
    assert_eq!(status, 200);
    let built = tree(&site.join("dist"));
    assert!(built.contains_key("index.html"));
    for (path, bytes) in &built {
        assert!(
            !String::from_utf8_lossy(bytes).contains("__rustify-docs"),
            "{path} carries dev's client"
        );
    }
}

#[test]
fn without_rust_dev_stops_before_watching_or_listening() {
    let temp = copy_fixture("content");
    let tools = Toolchain::new(&temp, false);
    let refused = tools.run_in(&temp.path("site"), &["dev", "--port", "0"]);
    assert_eq!(refused.status.code(), Some(1));
    let text = stderr(&refused);
    assert!(
        text.contains("https://rustup.rs") && text.contains("rustup toolchain install stable"),
        "{text}"
    );
    assert!(stdout(&refused).is_empty(), "it printed an address");
    assert!(tools.calls().iter().all(|call| !call.starts_with("cargo")));
}

#[test]
fn a_busy_default_port_falls_back_and_a_busy_explicit_port_is_refused() {
    let temp = copy_fixture("content");
    let busy = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    let port = busy.local_addr().unwrap().port();
    let config = temp.read("site/rustify-docs.toml");
    temp.write(
        "site/rustify-docs.toml",
        &format!("{config}\n[dev]\nport = {port}\n"),
    );
    let dev = Dev::start_with(&temp.path("site"), &[], &[]).unwrap();
    assert!(!dev.url.contains(&format!(":{port}/")), "{}", dev.url);
    wait_for_log(&dev, &format!("port {port} is in use"));
    assert_eq!(get_text(&dev.url).0, 200);
    drop(dev);
    let refused = Dev::start_with(&temp.path("site"), &["--port", &port.to_string()], &[]);
    let text = refused.err().expect("a busy explicit port is an error");
    assert!(text.contains("cannot listen"), "{text}");
}

#[test]
fn a_save_reloads_the_pages_it_changed_and_no_others() {
    let temp = copy_fixture("content");
    let dev = Dev::start(&temp.path("site"), &[], &[]).unwrap();
    let (_, html) = get_text(&format!("{}guide/", dev.url));
    let good = good_of(&html);
    let mut guide = dev.events("/docs/guide/", good);
    let mut api = dev.events("/docs/reference/api/", good);
    let mut setup = dev.events("/docs/guide/setup/", good);
    let hello = guide.next();
    assert_eq!(hello["reload"], false);
    assert_eq!(hello["ok"], true);
    api.next();
    setup.next();

    // A paragraph: its own page.
    temp.edit(
        "site/content/guide/index.md",
        "Read the setup page first.",
        "Read the setup page first. Edited once.",
    );
    let changed = guide.until(|state| revision(state) > revision(&hello));
    assert_eq!(changed["reload"], true);
    let other = api.until(|state| revision(state) == revision(&changed));
    assert_eq!(other["reload"], false);
    setup.until(|state| revision(state) == revision(&changed));
    assert!(get_text(&format!("{}guide/", dev.url))
        .1
        .contains("Edited once."));

    // An included source file: the page that includes it.
    temp.edit("site/src/example.rs", "hello", "hello again");
    let changed = setup.until(|state| state["reload"] == true);
    let other = api.until(|state| revision(state) == revision(&changed));
    assert_eq!(other["reload"], false);
    assert!(get_text(&format!("{}guide/setup/", dev.url))
        .1
        .contains("hello again"));

    // An image: the pages that show it, under its new name.
    let (_, before) = get_text(&format!("{}guide/setup/", dev.url));
    temp.edit(
        "site/content/guide/pipeline.svg",
        "</svg>",
        "<!-- changed --></svg>",
    );
    let changed = setup.until(|state| state["reload"] == true);
    let other = api.until(|state| revision(state) == revision(&changed));
    assert_eq!(other["reload"], false);
    let (_, after) = get_text(&format!("{}guide/setup/", dev.url));
    assert_ne!(before, after);

    // The configuration: every page.
    temp.edit("site/rustify-docs.toml", "Fixture Docs", "Fixture Handbook");
    api.until(|state| state["reload"] == true);
    assert!(get_text(&format!("{}reference/api/", dev.url))
        .1
        .contains("Fixture Handbook"));
}

#[test]
fn an_image_outside_the_content_and_asset_directories_refreshes_its_page() {
    let temp = copy_fixture("content");
    temp.write(
        "site/images/mark.svg",
        "<svg xmlns=\"http://www.w3.org/2000/svg\"/>",
    );
    temp.edit(
        "site/content/guide/index.md",
        "Read the setup page first.",
        "Read the setup page first. ![Mark](../../images/mark.svg)",
    );
    let dev = Dev::start(&temp.path("site"), &[], &[]).unwrap();
    let (_, before) = get_text(&format!("{}guide/", dev.url));
    assert!(before.contains("/assets/files/mark."), "{before}");
    let mut guide = dev.events("/docs/guide/", good_of(&before));
    let mut api = dev.events("/docs/reference/api/", good_of(&before));
    let hello = guide.next();
    api.next();
    // Saved alone, the image is a new build that reloads its page only.
    temp.edit("site/images/mark.svg", "/>", "><!-- changed --></svg>");
    let changed = guide.until(|state| revision(state) > revision(&hello));
    assert_eq!(changed["reload"], true);
    let other = api.until(|state| revision(state) == revision(&changed));
    assert_eq!(other["reload"], false);
    let (_, after) = get_text(&format!("{}guide/", dev.url));
    assert_ne!(before, after);
}

#[test]
fn dev_reads_the_configuration_it_is_given() {
    let temp = copy_fixture("content");
    let custom = temp
        .read("site/rustify-docs.toml")
        .replace("Fixture Docs", "Custom Docs");
    temp.write("site/custom.toml", &custom);
    temp.write("site/rustify-docs.toml", "NOT TOML");
    let dev = Dev::start(&temp.path("site"), &["--config", "custom.toml"], &[]).unwrap();
    let (_, html) = get_text(&dev.url);
    assert!(html.contains("Custom Docs"), "{html}");
    let mut home = dev.events("/docs/", good_of(&html));
    let hello = home.next();
    // A save to the file it read is a new build; one to the default is not.
    temp.edit("site/custom.toml", "Custom Docs", "Custom Handbook");
    let changed = home.until(|state| revision(state) > revision(&hello));
    assert_eq!(changed["reload"], true);
    assert!(get_text(&dev.url).1.contains("Custom Handbook"));
}

#[test]
fn a_failed_build_keeps_the_last_good_one_until_it_is_fixed() {
    let temp = copy_fixture("content");
    let dev = Dev::start(&temp.path("site"), &[], &[]).unwrap();
    let page = format!("{}guide/", dev.url);
    let (_, html) = get_text(&page);
    let mut api = dev.events("/docs/reference/api/", good_of(&html));
    let hello = api.next();

    temp.edit(
        "site/content/guide/index.md",
        "Then write a page.",
        "Then write a page.\n\n[Broken](missing.md)",
    );
    let failed = api.until(|state| state["ok"] == false);
    assert_eq!(failed["reload"], false);
    assert_eq!(failed["errors"], 1);
    assert_eq!(failed["good"], hello["good"]);
    let problem = &failed["diagnostics"][0];
    assert_eq!(problem["rule"], "broken-link");
    assert_eq!(problem["file"], "content/guide/index.md");
    assert!(problem["line"].as_u64().is_some() && problem["column"].as_u64().is_some());
    // The page served is the last good one.
    let (status, served) = get_text(&page);
    assert_eq!(status, 200);
    assert!(!served.contains("Broken"));
    wait_for_log(&dev, "error[broken-link]");
    wait_for_log(&dev, "still serving revision");

    // The same failure saved again is a new revision, still failed.
    let text = temp.read("site/content/guide/index.md");
    temp.write("site/content/guide/index.md", &format!("{text}\n"));
    let again = api.until(|state| revision(state) > revision(&failed));
    assert_eq!(again["ok"], false);
    assert_eq!(again["errors"], 1);

    temp.edit("site/content/guide/index.md", "[Broken](missing.md)", "");
    let fixed = api.until(|state| state["ok"] == true);
    assert_eq!(fixed["errors"], 0);
    assert_eq!(fixed["reload"], false);
}

#[test]
fn located_semantic_failures_keep_last_good_and_clear_after_repair() {
    let temp = copy_fixture("content");
    let original = temp.read("site/content/index.md");
    let dev = Dev::start(&temp.path("site"), &[], &[]).unwrap();
    let (_, html) = get_text(&dev.url);
    let mut home = dev.events("/docs/", good_of(&html));
    home.next();
    for (bad, rule, line, column) in [
        (
            "---\ntitle: Home\nlayout: home\nlang: english\n---\n# Home\n",
            "lang",
            4,
            7,
        ),
        (
            "---\ntitle: Home\nlayout: home\n---\nIntro\n\n# Home\n",
            "multiple-h1",
            7,
            1,
        ),
        (
            "---\ntitle: Home\nlayout: home\n---\n<!-- comment -->\n\n# Home\n",
            "multiple-h1",
            7,
            1,
        ),
        (
            "---\ntitle: Home\nlayout: home\n---\n<div><h1>Home</h1></div>\n",
            "multiple-h1",
            5,
            6,
        ),
        (
            "---\ntitle: Home\nlayout: home\n---\n<H1>Home</H1>\n",
            "multiple-h1",
            5,
            1,
        ),
    ] {
        temp.write("site/content/index.md", bad);
        let failed = home.until(|state| state["ok"] == false);
        let problem = failed["diagnostics"]
            .as_array()
            .unwrap()
            .iter()
            .find(|d| d["rule"] == rule)
            .unwrap();
        assert_eq!(
            (problem["line"].as_u64(), problem["column"].as_u64()),
            (Some(line), Some(column))
        );
        assert!(problem["suggestion"]
            .as_str()
            .is_some_and(|s| !s.is_empty()));
        assert_eq!(failed["reload"], false);
        assert!(get_text(&dev.url).1.contains("Fixture Docs"));
        temp.write("site/content/index.md", &original);
        let fixed = home.until(|state| state["ok"] == true);
        assert_eq!(fixed["errors"], 0);
    }
}

#[test]
fn a_first_build_that_fails_shows_the_problems_until_one_builds() {
    let temp = copy_fixture("content");
    temp.edit(
        "site/content/guide/setup.md",
        "Run `rustify-docs --version`",
        "[Broken](missing.md) Run `rustify-docs --version`",
    );
    let dev = Dev::start(&temp.path("site"), &[], &[]).unwrap();
    for page in ["", "guide/setup/", "no/such/"] {
        let (status, html) = get_text(&format!("{}{page}", dev.url));
        assert_eq!(status, 500, "{page}");
        assert!(html.contains("The site does not build yet"), "{page}");
        assert_eq!(good_of(&html), 0);
    }
    // Its stylesheet and scripts are served.
    let (_, html) = get_text(&dev.url);
    let dom = dom_query::Document::from(html.as_str());
    let css = dom
        .select("head link[rel=stylesheet]")
        .attr("href")
        .expect("the head contains a stylesheet");
    assert!(css.contains("/assets/site."), "{css}");
    assert_eq!(get(&format!("{}{css}", dev.origin())).0, 200);

    let mut events = dev.events("/docs/guide/setup/", 0);
    let first = events.next();
    assert_eq!(first["ok"], false);
    assert!(first["good"].is_null());
    temp.edit("site/content/guide/setup.md", "[Broken](missing.md) ", "");
    let built = events.until(|state| state["ok"] == true);
    assert_eq!(built["reload"], true);
    assert_eq!(get_text(&format!("{}guide/setup/", dev.url)).0, 200);
}

#[test]
fn a_panic_in_a_build_is_a_problem_and_the_session_goes_on() {
    let temp = copy_fixture("content");
    let dev = Dev::start(
        &temp.path("site"),
        &[],
        &[("RUSTIFY_DOCS_FAULT", "panic:PANIC-HERE")],
    )
    .unwrap();
    let page = format!("{}guide/", dev.url);
    let (_, html) = get_text(&page);
    let mut events = dev.events("/docs/guide/", good_of(&html));
    events.next();
    temp.edit(
        "site/content/guide/index.md",
        "Then write a page.",
        "Then write a page. PANIC-HERE",
    );
    let failed = events.until(|state| state["ok"] == false);
    assert_eq!(failed["diagnostics"][0]["rule"], "internal");
    assert_eq!(failed["reload"], false);
    let (status, served) = get_text(&page);
    assert_eq!(status, 200);
    assert!(!served.contains("PANIC-HERE"));
    // The next build starts clean and is served.
    temp.edit("site/content/guide/index.md", " PANIC-HERE", " Recovered.");
    let fixed = events.until(|state| state["ok"] == true);
    assert_eq!(fixed["reload"], true);
    assert!(get_text(&page).1.contains("Recovered."));
}

#[test]
fn drafts_are_shown_and_marked_and_a_link_to_one_is_a_warning() {
    let temp = copy_fixture("content");
    let dev = Dev::start(&temp.path("site"), &[], &[]).unwrap();
    let (status, draft) = get_text(&format!("{}guide/later/", dev.url));
    assert_eq!(status, 200);
    assert!(
        draft.contains("class=\"rd-draft\""),
        "the page says it is a draft"
    );
    let (_, guide) = get_text(&format!("{}guide/", dev.url));
    assert!(
        guide.contains("Later · Draft"),
        "the sidebar says it is a draft"
    );
    let mut events = dev.events("/docs/guide/", good_of(&guide));
    events.next();
    temp.edit(
        "site/content/guide/index.md",
        "Then write a page.",
        "Then write a page, [later](later.md).",
    );
    let state = events.until(|state| state["warnings"] == 1);
    assert_eq!(state["ok"], true);
    assert_eq!(state["diagnostics"][0]["rule"], "draft-link");
    assert_eq!(state["diagnostics"][0]["severity"], "warning");
}

#[test]
fn a_burst_of_saves_ends_on_the_last_one() {
    let temp = copy_fixture("content");
    let dev = Dev::start(&temp.path("site"), &[], &[]).unwrap();
    let page = format!("{}guide/", dev.url);
    let original = temp.read("site/content/guide/index.md");
    for i in 0..30 {
        temp.write(
            "site/content/guide/index.md",
            &original.replace("Then write a page.", &format!("Then write page {i}.")),
        );
        std::thread::sleep(Duration::from_millis(7));
    }
    let start = Instant::now();
    while !get_text(&page).1.contains("Then write page 29.") {
        assert!(
            start.elapsed() < Duration::from_secs(20),
            "the last save never showed"
        );
        std::thread::sleep(Duration::from_millis(50));
    }
}

#[cfg(unix)]
#[test]
fn snapshot_attachment_symlinks_recover_without_a_markdown_save() {
    for (alias, url) in [
        ("content/manual.pdf", "../manual.pdf"),
        ("static/manual.pdf", "/manual.pdf"),
    ] {
        let temp = copy_fixture("content");
        temp.write("site/brand/real.pdf", "original attachment");
        std::os::unix::fs::symlink("../brand/real.pdf", temp.path(&format!("site/{alias}")))
            .unwrap();
        temp.edit(
            "site/content/guide/index.md",
            "Then write a page.",
            &format!("Then write a page.\n\n[Download]({url})"),
        );
        let dev = Dev::start(&temp.path("site"), &[], &[]).unwrap();
        let page = format!("{}guide/", dev.url);
        let (_, original) = get_text(&page);
        let mut changed = dev.events("/docs/guide/", good_of(&original));
        changed.next();
        let (_, other) = get_text(&format!("{}reference/api/", dev.url));
        let mut stable = dev.events("/docs/reference/api/", good_of(&other));
        stable.next();
        std::fs::remove_file(temp.path("site/brand/real.pdf")).unwrap();
        let failed = changed.until(|state| state["ok"] == false);
        assert_eq!(failed["reload"], false);
        assert!(failed["diagnostics"]
            .as_array()
            .unwrap()
            .iter()
            .any(|d| d["rule"] == "broken-link"));
        assert_eq!(
            get_text(&page).1,
            original,
            "last good page after deleting {alias}"
        );
        temp.write("site/brand/real.pdf", "restored attachment");
        let started = Instant::now();
        while get_text(&page).1 == original {
            assert!(
                started.elapsed() < Duration::from_secs(10),
                "restoring {alias}'s target never updated its page"
            );
            std::thread::sleep(Duration::from_millis(50));
        }
        let restored = changed.until(|state| state["ok"] == true);
        assert_eq!(restored["reload"], true, "{alias}");
        assert!(restored["diagnostics"].as_array().unwrap().is_empty());
        assert_eq!(
            stable.until(|state| revision(state) >= revision(&restored))["reload"],
            false
        );
        let (_, updated) = get_text(&page);
        assert_ne!(updated, original, "{alias}");
        let href = updated
            .split("Download</a>")
            .next()
            .unwrap()
            .rsplit("href=\"")
            .next()
            .unwrap()
            .split('"')
            .next()
            .unwrap();
        assert_eq!(
            get_text(&format!("{}{href}", dev.origin())).1,
            "restored attachment"
        );
    }
}

#[cfg(unix)]
#[test]
fn a_symlink_target_outside_the_snapshot_refreshes_and_recovers() {
    let temp = copy_fixture("content");
    temp.write(
        "site/brand/real.svg",
        "<svg xmlns='http://www.w3.org/2000/svg'><rect fill='red'/></svg>",
    );
    std::fs::create_dir_all(temp.path("site/images")).unwrap();
    std::os::unix::fs::symlink("../brand/real.svg", temp.path("site/images/mark.svg")).unwrap();
    temp.edit(
        "site/content/guide/index.md",
        "Then write a page.",
        "Then write a page.\n\n![Mark](../../images/mark.svg)",
    );
    let dev = Dev::start(&temp.path("site"), &[], &[]).unwrap();
    let page = format!("{}guide/", dev.url);
    let (_, original) = get_text(&page);
    let mut changed = dev.events("/docs/guide/", good_of(&original));
    changed.next();
    let (_, other) = get_text(&format!("{}reference/api/", dev.url));
    let mut stable = dev.events("/docs/reference/api/", good_of(&other));
    stable.next();
    // An editor replaces the real file atomically, without touching its alias.
    temp.write(
        "site/brand/replacement.svg",
        "<svg xmlns='http://www.w3.org/2000/svg'><rect fill='blue'/></svg>",
    );
    std::fs::rename(
        temp.path("site/brand/replacement.svg"),
        temp.path("site/brand/real.svg"),
    )
    .unwrap();
    let saved = changed.until(|state| state["ok"] == true && state["reload"] == true);
    assert_eq!(
        stable.until(|state| revision(state) >= revision(&saved))["reload"],
        false
    );
    let (_, updated) = get_text(&page);
    assert_ne!(original, updated);
    drop(changed);
    let mut changed = dev.events("/docs/guide/", good_of(&updated));
    changed.next();
    std::fs::remove_file(temp.path("site/brand/real.svg")).unwrap();
    let failed = changed.until(|state| state["ok"] == false);
    assert_eq!(failed["reload"], false);
    assert!(failed["diagnostics"]
        .as_array()
        .unwrap()
        .iter()
        .any(|d| d["rule"] == "missing-image"));
    temp.write(
        "site/brand/real.svg",
        "<svg xmlns='http://www.w3.org/2000/svg'><rect fill='green'/></svg>",
    );
    assert_eq!(changed.until(|state| state["ok"] == true)["reload"], true);
    assert_eq!(get_text(&page).0, 200);
}
