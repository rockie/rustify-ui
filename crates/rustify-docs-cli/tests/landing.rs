//! Landing pages served the way their build says: each under the policy the
//! manifest records for it, every other response under the strict one, by
//! `preview` and by `dev` alike.

mod support;

use rustify_docs::shell::CSP;
use support::{copy_fixture, get, run_in, succeeded, Dev, Preview};

/// The fixture built, and the policies its manifest records.
fn built() -> (support::Temp, serde_json::Map<String, serde_json::Value>) {
    let temp = copy_fixture("landing");
    succeeded(&run_in(&temp.path("site"), &["build"]));
    let manifest: serde_json::Value =
        serde_json::from_str(&temp.read("site/dist/rustify-docs-manifest.json")).unwrap();
    let policies = manifest["page_policies"].as_object().unwrap().clone();
    assert_eq!(
        policies.keys().map(String::as_str).collect::<Vec<_>>(),
        ["lp/index.html", "thanks/index.html"]
    );
    (temp, policies)
}

fn header(url: &str) -> (u16, String) {
    let (status, headers, _) = get(url);
    (status, headers["content-security-policy"].clone())
}

#[test]
fn preview_sends_each_landing_page_its_recorded_policy() {
    let (temp, policies) = built();
    let landing = policies["lp/index.html"].as_str().unwrap();
    assert_ne!(landing, CSP);
    let preview = Preview::start(&temp.path("site"), &[]).unwrap();
    let url = |path: &str| format!("{}{path}", preview.url);
    assert_eq!(header(&url("lp/")), (200, landing.to_owned()));
    assert_eq!(header(&url("lp/index.html")), (200, landing.to_owned()));
    assert_eq!(header(&url("lp/?gclid=x")), (200, landing.to_owned()));
    assert_eq!(
        header(&url("thanks/")),
        (
            200,
            policies["thanks/index.html"].as_str().unwrap().to_owned()
        )
    );
    for (path, status) in [
        ("", 200),
        ("guide/", 200),
        ("offer/", 200),
        ("landing/init.js", 200),
        ("lp", 301),
        ("missing/", 404),
    ] {
        assert_eq!(header(&url(path)), (status, CSP.to_owned()), "{path}");
    }
}

#[test]
fn preview_refuses_an_output_whose_policies_are_damaged() {
    let (temp, policies) = built();
    let landing = policies["lp/index.html"].as_str().unwrap();
    let original = temp.read("site/dist/rustify-docs-manifest.json");
    for (path, policy) in [
        ("lp/index.html", format!("{landing}\r\nSet-Cookie: a=b")),
        ("lp/index.html", "default-src *".to_owned()),
        ("landing/init.js", landing.to_owned()),
        ("nowhere/index.html", landing.to_owned()),
    ] {
        let mut manifest: serde_json::Value = serde_json::from_str(&original).unwrap();
        manifest["page_policies"] = serde_json::json!({ path: policy });
        temp.write(
            "site/dist/rustify-docs-manifest.json",
            &serde_json::to_string_pretty(&manifest).unwrap(),
        );
        let refused = Preview::start(&temp.path("site"), &[]).err().unwrap();
        assert!(
            refused.contains("invalid page_policies"),
            "{path}: {refused}"
        );
    }
}

#[test]
fn dev_sends_each_landing_page_its_own_policy() {
    let (temp, policies) = built();
    let dev = Dev::start(&temp.path("site"), &[], &[]).unwrap();
    let url = |path: &str| format!("{}{path}", dev.url);
    assert_eq!(
        header(&url("lp/")),
        (200, policies["lp/index.html"].as_str().unwrap().to_owned())
    );
    for path in ["", "guide/", "offer/", "missing/"] {
        assert_eq!(header(&url(path)).1, CSP, "{path}");
    }
}
