//! Diagnostic prose keeps author values intact across the real command exits.

mod support;

use rustify_docs::embed::{self, Field, Owner};
use serde_json::Value;
use support::{run_in, stdout, Temp};

#[test]
fn both_writers_match_the_handwritten_verbatim_table() {
    for (value, encoded) in [
        ("", "``  ``"),
        (" ", "` `"),
        ("  ", "`  `"),
        (" a ", "` a `"),
        ("a\nb", "`a\nb`"),
        ("a\r\nb", "`a\r\nb`"),
        ("a\\b", "`a\\b`"),
        ("Counter", "`Counter`"),
        ("a`b", "`` a`b ``"),
        ("a``b", "``` a``b ```"),
        ("`a`", "`` `a` ``"),
        ("`", "`` ` ``"),
        ("``", "``` `` ```"),
        (" `\r\n ", "``  `\r\n  ``"),
    ] {
        assert_eq!(rustify_docs::diagnostic_text::code_span(value), encoded);
        assert_eq!(rustify_docs_syntax::code_span(value), encoded);
        let prose = if value.contains('`') { encoded } else { value };
        assert_eq!(rustify_docs::diagnostic_text::literal(value), prose);
        assert_eq!(rustify_docs_syntax::literal(value), prose);
    }
}

#[test]
fn duplicate_site_routes_quote_the_whole_route() {
    let error = rustify_docs_syntax::parse_registry(
        "fn site() { site! { pages { One { route: \"/a`b/\", title: \"One\" } => one, Two { route: \"/a`b/\", title: \"Two\" } => two } } }",
    ).err().unwrap();
    assert_eq!(error.to_string(), "duplicate page route `` /a`b/ ``");
}

#[test]
fn repairs_encode_the_entire_default_and_author_key() {
    let fields = [Field {
        name: "label".into(),
        ty: "String".into(),
        default: Some("\"a`b\".to_string()".into()),
    }];
    let owner = Owner::Registered {
        registry: "src/lib.rs",
    };
    assert_eq!(
        embed::unknown_property("Counter", &fields, owner, "un`known"),
        "Use one of `label` (`String`; default expression `` \"a`b\".to_string() ``) for `Counter`; rename or remove `` un`known ``."
    );
    assert_eq!(
        embed::invalid_props(&fields, owner, Some("label")),
        "Write `label: \"text\"` for declared type `String` (default expression `` \"a`b\".to_string() ``). Reported props path: `label`."
    );
    assert_eq!(
        embed::invalid_props(&fields, owner, None),
        "Add or correct `props` using the declared fields `label` (`String`; default expression `` \"a`b\".to_string() ``); check the registration in `src/lib.rs`."
    );
}

#[test]
fn check_and_build_encode_plain_paths_prefixed_anchors_yaml_keys_and_empty_language() {
    let temp = Temp::new("diagnostic-text");
    let config = "schema_version=1\n[site]\nname='D'\nurl='https://example.com/'\n";
    temp.write("rustify-docs.toml", config);
    temp.write(
        "content/index.md",
        "---\ndescription: Home\n---\n# Home\n\n[Target](a%60b%60c.md#a%60b)\n",
    );
    temp.write(
        "content/a`b`c.md",
        "---\ndescription: Target\n---\n# Target\n",
    );
    let check = run_in(&temp.0, &["check", "--format", "json"]);
    assert_eq!(check.status.code(), Some(1));
    let checked: Value = serde_json::from_slice(&check.stdout).unwrap();
    let anchor = checked["diagnostics"]
        .as_array()
        .unwrap()
        .iter()
        .find(|d| d["rule"] == "broken-anchor")
        .unwrap();
    assert_eq!(
        anchor["message"],
        "`` content/a`b`c.md `` has no heading `` #a`b ``"
    );
    let built = run_in(&temp.0, &["build", "--format", "json"]);
    assert_eq!(built.status.code(), Some(1));
    let built: Value = serde_json::from_str(&stdout(&built)).unwrap();
    assert_eq!(built["diagnostics"], checked["diagnostics"]);

    temp.write("content/index.md", "---\ndescription: Home\n---\n# Home\n\n```rustify-component\nname: Steps\nun`known: 1\n```\n\n```rustify-component\nname: Miss`ing\n```\n\n```rustify-component\nname: MissingCounter\n```\n");
    let output = run_in(&temp.0, &["check", "--format", "json"]);
    assert_eq!(output.status.code(), Some(1));
    let report: Value = serde_json::from_slice(&output.stdout).unwrap();
    let diagnostics = report["diagnostics"].as_array().unwrap();
    assert!(diagnostics.iter().any(|d| d["rule"] == "embed-payload" && d["message"] == "unknown field `` un`known ``; a demo takes only `name`, a component takes `name`, `props` and `children`"));
    assert!(diagnostics.iter().any(|d| d["rule"] == "embed-payload"
        && d["message"] == "name must be a registered Rust identifier"));
    assert_eq!(
        diagnostics
            .iter()
            .filter(|d| d["rule"] == "unregistered-component")
            .count(),
        1
    );

    temp.write("rustify-docs.toml", &format!("{config}lang=''\n"));
    let output = run_in(&temp.0, &["check", "--format", "json"]);
    assert_eq!(output.status.code(), Some(1));
    let report: Value = serde_json::from_slice(&output.stdout).unwrap();
    let language = report["diagnostics"]
        .as_array()
        .unwrap()
        .iter()
        .find(|d| d["rule"] == "config-lang")
        .unwrap();
    assert_eq!(language["message"], "``  `` is not a language tag");
    assert_eq!(language["suggestion"], "use a tag like `en` or `zh-CN`");
}

#[test]
fn region_repairs_encode_the_fixed_marker_prefixes_and_configuration_values() {
    let temp = Temp::new("diagnostic-region");
    let config = "schema_version=1\n[site]\nname='D'\nurl='https://example.com/'\n";
    temp.write("rustify-docs.toml", config);
    temp.write("content/index.md", "---\ndescription: Home\n---\n# Home\n\n<!-- rustify:include path=\"snippets/a.rs\" region=\"a`b\" -->\n");
    temp.write("snippets/a.rs", "fn example() {}\n");
    let output = run_in(&temp.0, &["check", "--format", "json"]);
    assert_eq!(output.status.code(), Some(1));
    let report: Value = serde_json::from_slice(&output.stdout).unwrap();
    let region = report["diagnostics"]
        .as_array()
        .unwrap()
        .iter()
        .find(|d| d["rule"] == "include-region")
        .unwrap();
    assert_eq!(
        region["message"],
        "`snippets/a.rs`: there is no region `` a`b ``"
    );
    assert_eq!(
        region["suggestion"],
        "mark the lines with `` rustify-region:a`b `` and `` rustify-endregion:a`b `` comments"
    );
    temp.write("rustify-docs.toml", &format!("{config}lang='a`b'\n"));
    let output = run_in(&temp.0, &["check", "--format", "json"]);
    let report: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert!(report["diagnostics"]
        .as_array()
        .unwrap()
        .iter()
        .any(|d| d["rule"] == "config-lang" && d["message"] == "`` a`b `` is not a language tag"));
}
