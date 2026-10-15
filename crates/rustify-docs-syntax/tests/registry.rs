use rustify_docs_syntax::{parse_registry, Manifest, SiteDeclaration, Slot};

fn registry(body: &str) -> String {
    format!("pub fn site() {{ rustify_docs::site! {{ {body} }} }}")
}

fn parse(body: &str) -> SiteDeclaration {
    parse_registry(&registry(body)).expect("valid site declaration")
}

fn error(body: &str) -> syn::Error {
    match parse_registry(&registry(body)) {
        Ok(_) => panic!("accepted invalid site declaration: {body}"),
        Err(error) => error,
    }
}

#[test]
fn a_complete_declaration_retains_rust_ast_and_normalized_metadata() {
    let site = parse(
        r#"
        components {
            Counter {
                initial: i32 = -2,
                labels: Vec<Option<String>> = Vec::new(),
                required: std::collections::BTreeMap<String, u64>,
            } => |props, children| views::counter(props.initial, children),
        }
        demos { CounterDemo => crate::demos::counter_demo, }
        pages {
            Home { route: "/", title: "Home", description: "Introduction" } => crate::pages::home,
        }
        slots { footer => crate::layout::footer, }
        "#,
    );
    let manifest = site.manifest();
    assert_eq!(manifest.components[0].name, "Counter");
    assert_eq!(manifest.components[0].fields[0].ty, "i32");
    assert_eq!(
        manifest.components[0].fields[0].default.as_deref(),
        Some("- 2")
    );
    assert_eq!(
        manifest.components[0].fields[1].ty,
        "Vec < Option < String > >"
    );
    assert_eq!(manifest.components[0].fields[2].default, None);
    assert_eq!(manifest.demos[0].function, "crate :: demos :: counter_demo");
    assert_eq!(manifest.pages[0].route, "/");
    assert_eq!(
        manifest.pages[0].description.as_deref(),
        Some("Introduction")
    );
    assert_eq!(manifest.slots[0].slot, Slot::Footer);
    assert!(matches!(&site.components[0].adapter, syn::Expr::Closure(_)));
    assert!(matches!(
        &site.components[0].fields[1].ty,
        syn::Type::Path(_)
    ));
    assert!(matches!(
        &site.components[0].fields[1].default,
        Some(syn::Expr::Call(_))
    ));
}

#[test]
fn source_and_macro_token_parsing_produce_the_same_manifest() {
    let body = "components { Counter { initial: i32 = 0 } => |props, children| views::counter(props, children), }";
    let from_tokens = syn::parse_str::<SiteDeclaration>(body).expect("macro tokens");
    assert_eq!(parse(body).manifest(), from_tokens.manifest());
}

#[test]
fn page_languages_are_shared_by_cold_and_macro_token_parsing() {
    let body = r#"pages {
        Home { route: "/", title: "Home" } => crate::pages::home,
        HomeZh { route: "/", title: "首页", lang: "zh" } => crate::pages::home_zh,
    }"#;
    let from_tokens = syn::parse_str::<SiteDeclaration>(body).expect("macro tokens");
    let site = parse(body);
    assert_eq!(site.manifest(), from_tokens.manifest());
    assert!(site.pages[0].lang.is_none());
    assert_eq!(site.pages[1].lang.as_ref().unwrap().value(), "zh");
    assert_eq!(site.manifest().pages[0].lang, None);
    assert_eq!(site.manifest().pages[1].lang.as_deref(), Some("zh"));
}

#[test]
fn page_publication_switches_are_literal_optional_booleans() {
    let site = rustify_docs_syntax::parse_registry(r#"site! { pages {
        Home { route: "/", title: "Home", draft: true, index: false, search: false, ai: true } => home,
    }}"#).unwrap();
    let value = format!("{:?}", site.manifest().pages[0]);
    for field in [
        "draft: Some(true)",
        "index: Some(false)",
        "search: Some(false)",
        "ai: Some(true)",
    ] {
        assert!(value.contains(field), "{value}");
    }
    for metadata in [
        "draft: \"true\"",
        "ai: SOME_FLAG",
        "index: true, index: false",
    ] {
        assert!(rustify_docs_syntax::parse_registry(&format!(
            "site! {{ pages {{ Home {{ route: \"/\", title: \"Home\", {metadata} }} => home, }} }}"
        ))
        .is_err());
    }
}

#[test]
fn one_logical_route_can_register_different_declared_languages() {
    let site = parse(
        r#"pages {
        HomeEn { route: "/", title: "Home", lang: "en" } => home,
        HomeZh { route: "/", title: "首页", lang: "zh" } => home_zh,
    }"#,
    );
    assert_eq!(site.pages.len(), 2);
}

#[test]
fn whitespace_comments_and_crlf_do_not_change_the_manifest() {
    let compact = parse("demos { CounterDemo => crate::demos::counter_demo, }").manifest();
    let expanded = parse(
        "\r\n demos {\r\n // same function\r\n CounterDemo\r\n => crate :: demos :: counter_demo\r\n }\r\n",
    )
    .manifest();
    assert_eq!(compact, expanded);
}

#[test]
fn sections_can_be_omitted_or_empty() {
    let manifest =
        parse("components {} demos { CounterDemo => crate::demos::counter_demo }").manifest();
    assert_eq!(
        (
            manifest.components.len(),
            manifest.demos.len(),
            manifest.pages.len(),
            manifest.slots.len()
        ),
        (0, 1, 0, 0)
    );
}

#[test]
fn all_seven_slots_have_the_exact_declared_names() {
    let site = parse(
        "slots { header => a, footer => b, sidebar_top => c, sidebar_bottom => d, before_content => e, after_content => f, not_found => g }",
    );
    assert_eq!(
        site.manifest()
            .slots
            .iter()
            .map(|slot| slot.slot.key())
            .collect::<Vec<_>>(),
        [
            "header",
            "footer",
            "sidebar_top",
            "sidebar_bottom",
            "before_content",
            "after_content",
            "not_found"
        ]
    );
}

#[test]
fn explicit_client_only_metadata_uses_decoded_literals() {
    let site = parse(
        r##"components {
            #[client_only(class = r#"min-h-48"#, message = "Requires browser")]
            Editor {} => |props, children| views::editor(props, children),
        }"##,
    );
    let manifest = site.manifest();
    let client_only = manifest.components[0]
        .client_only
        .as_ref()
        .expect("client-only metadata");
    assert_eq!(
        (&*client_only.class, &*client_only.message),
        ("min-h-48", "Requires browser")
    );
}

#[test]
fn generic_reference_tuple_and_function_property_types_are_accepted() {
    let site = parse(
        "components { Typed { data: Vec<(String, Option<u64>)>, borrowed: &'static str, callback: fn(String) -> String } => render, }",
    );
    assert_eq!(site.components[0].fields.len(), 3);
}

#[test]
fn raw_property_identifiers_use_their_serde_field_name() {
    let manifest = parse("components { Typed { r#type: String } => render }").manifest();
    assert_eq!(manifest.components[0].fields[0].name, "type");
}

#[test]
fn the_manifest_is_serializable_without_serializing_syn_ast() {
    fn assert_serializable<T: serde::Serialize + serde::de::DeserializeOwned>() {}
    assert_serializable::<Manifest>();
}

#[test]
fn source_extraction_ignores_comments_strings_and_macro_rules_bodies() {
    let source = r#"
        // site! { demos { FalseDemo => false_demo } }
        const SAMPLE: &str = "site! { demos { Another => another } }";
        macro_rules! wrapped { () => { site! { demos { Hidden => hidden } } }; }
        pub fn site() {
            rustify_docs::site! { demos { RealDemo => crate::demos::real_demo } }
        }
    "#;
    let site = parse_registry(source).expect("one concrete declaration");
    assert_eq!(site.manifest().demos[0].name, "RealDemo");
}

#[test]
fn unknown_sections_are_rejected() {
    assert!(error("widgets {}")
        .to_string()
        .contains("unknown site section `widgets`"));
}

#[test]
fn repeated_sections_are_rejected() {
    assert!(error("demos { A => a } demos { B => b }")
        .to_string()
        .contains("duplicate site section `demos`"));
}

#[test]
fn repeated_names_in_one_section_are_rejected() {
    assert!(error("demos { Counter => a, Counter => b }")
        .to_string()
        .contains("duplicate registration name `Counter`"));
}

#[test]
fn names_are_unique_across_components_demos_and_pages() {
    assert!(
        error("components { Counter {} => render } demos { Counter => demo }")
            .to_string()
            .contains("duplicate registration name `Counter`")
    );
}

#[test]
fn repeated_property_names_are_rejected() {
    assert!(
        error("components { Counter { initial: i32, initial: u64 } => render }")
            .to_string()
            .contains("duplicate property `initial`")
    );
}

#[test]
fn repeated_page_routes_are_rejected() {
    assert!(error("pages { Home { route: \"/\", title: \"Home\" } => home, Index { route: \"/\", title: \"Index\" } => index }").to_string().contains("duplicate page route `/`"));
}

#[test]
fn repeated_page_routes_in_the_same_declared_language_are_rejected() {
    assert!(error(
        r#"pages {
        Home { route: "/", title: "Home", lang: "zh-CN" } => home,
        Index { route: "/", title: "Index", lang: "ZH-cn" } => index,
    }"#
    )
    .to_string()
    .contains("duplicate page route `/`"));
}

#[test]
fn page_names_remain_unique_across_declared_languages() {
    assert!(error(
        r#"pages {
        Home { route: "/", title: "Home", lang: "en" } => home,
        Home { route: "/", title: "首页", lang: "zh" } => home_zh,
    }"#
    )
    .to_string()
    .contains("duplicate registration name `Home`"));
}

#[test]
fn page_language_metadata_is_literal_and_has_no_repeated_key() {
    assert!(error(
        r#"pages {
        Home { route: "/", title: "Home", lang: LOCALE } => home,
    }"#
    )
    .to_string()
    .contains("string literal"));
    assert!(error(
        r#"pages {
        Home { route: "/", title: "Home", lang: "en", lang: "zh" } => home,
    }"#
    )
    .to_string()
    .contains("duplicate page metadata `lang`"));
}

#[test]
fn repeated_slots_are_rejected() {
    assert!(error("slots { footer => a, footer => b }")
        .to_string()
        .contains("duplicate slot `footer`"));
}

#[test]
fn unknown_slots_are_rejected() {
    assert!(error("slots { hero => render }")
        .to_string()
        .contains("unknown slot `hero`"));
}

#[test]
fn page_route_and_title_are_required() {
    assert!(error("pages { Home { title: \"Home\" } => home }")
        .to_string()
        .contains("missing page metadata `route`"));
    assert!(error("pages { Home { route: \"/\" } => home }")
        .to_string()
        .contains("missing page metadata `title`"));
}

#[test]
fn page_metadata_is_literal_and_has_no_unknown_or_repeated_keys() {
    assert!(
        error("pages { Home { route: ROOT, title: \"Home\" } => home }")
            .to_string()
            .contains("string literal")
    );
    assert!(
        error("pages { Home { route: \"/\", title: \"Home\", slug: \"home\" } => home }")
            .to_string()
            .contains("unknown page metadata `slug`")
    );
    assert!(
        error("pages { Home { route: \"/\", route: \"/home\", title: \"Home\" } => home }")
            .to_string()
            .contains("duplicate page metadata `route`")
    );
}

#[test]
fn demo_closures_and_calls_are_rejected_as_function_identities() {
    assert!(error("demos { CounterDemo => || views::counter() }")
        .to_string()
        .contains("demo render function must be a direct Rust path"));
    assert!(error("demos { CounterDemo => views::counter() }")
        .to_string()
        .contains("demo render function must be a direct Rust path"));
}

#[test]
fn page_and_slot_render_functions_must_be_paths() {
    assert!(
        error("pages { Home { route: \"/\", title: \"Home\" } => |context| page(context) }")
            .to_string()
            .contains("page render function must be a direct Rust path")
    );
    assert!(error("slots { footer => |context| footer(context) }")
        .to_string()
        .contains("slot render function must be a direct Rust path"));
}

#[test]
fn client_only_requires_a_nonempty_literal_class_and_message() {
    assert!(error(
        "components { #[client_only(message = \"Requires browser\")] Editor {} => render }"
    )
    .to_string()
    .contains("missing client_only metadata `class`"));
    assert!(
        error("components { #[client_only(class = \"min-h-48\")] Editor {} => render }")
            .to_string()
            .contains("missing client_only metadata `message`")
    );
    assert!(error("components { #[client_only(class = \"\", message = \"Requires browser\")] Editor {} => render }").to_string().contains("client_only `class` must not be empty"));
}

#[test]
fn client_only_rejects_unknown_repeated_arguments_and_other_attributes() {
    assert!(error("components { #[client_only(class = \"min-h-48\", message = \"Requires browser\", extra = true)] Editor {} => render }").to_string().contains("unknown client_only metadata `extra`"));
    assert!(error("components { #[client_only(class = \"min-h-48\", class = \"min-h-64\", message = \"Requires browser\")] Editor {} => render }").to_string().contains("duplicate client_only metadata `class`"));
    assert!(
        error("components { #[cfg(feature = \"browser\")] Editor {} => render }")
            .to_string()
            .contains("unsupported component attribute")
    );
}

#[test]
fn a_whole_site_declaration_cannot_be_empty() {
    assert!(error("components {} demos {} pages {} slots {}")
        .to_string()
        .contains("site! must register at least one"));
}

#[test]
fn a_registry_source_must_have_exactly_one_site_invocation() {
    let absent = match parse_registry("pub fn site() {}") {
        Ok(_) => panic!("accepted missing declaration"),
        Err(error) => error,
    };
    assert!(absent.to_string().contains("no site! declaration"));
    let ambiguous = match parse_registry(
        "fn a() { site! { demos { A => a } } } fn b() { site! { demos { B => b } } }",
    ) {
        Ok(_) => panic!("accepted ambiguous declarations"),
        Err(error) => error,
    };
    assert!(ambiguous
        .to_string()
        .contains("multiple site! declarations"));
}

#[test]
fn registry_errors_preserve_exact_source_line_and_unicode_column() {
    let source = "// 原始 source\r\nfn site() {\r\n    rustify_docs::site! {\r\n        demos { 演示 => demo, 演示 => another }\r\n    }\r\n}\r\n";
    let error = match parse_registry(source) {
        Ok(_) => panic!("accepted duplicate Unicode name"),
        Err(error) => error,
    };
    let at = error.span().start();
    let line = source.lines().nth(3).expect("declaration line");
    let second = line.rfind("演示").expect("second name");
    assert_eq!((at.line, at.column), (4, line[..second].chars().count()));
}

#[test]
fn a_leading_bom_does_not_shift_first_line_diagnostics() {
    let source = "\u{feff}fn site() { site! { demos { 演示 => demo, 演示 => another } } }";
    let error = match parse_registry(source) {
        Ok(_) => panic!("accepted duplicate Unicode name"),
        Err(error) => error,
    };
    let at = error.span().start();
    let second = source.rfind("演示").expect("second name");
    assert_eq!((at.line, at.column), (1, source[..second].chars().count()));
}

#[test]
fn a_bom_and_shebang_preserve_following_line_diagnostics() {
    let source = "\u{feff}#!/usr/bin/env rust-script\r\nfn site() { site! { demos { 演示 => demo, 演示 => another } } }";
    let error = match parse_registry(source) {
        Ok(_) => panic!("accepted duplicate Unicode name"),
        Err(error) => error,
    };
    let at = error.span().start();
    let line = source.lines().nth(1).expect("declaration line");
    let second = line.rfind("演示").expect("second name");
    assert_eq!((at.line, at.column), (2, line[..second].chars().count()));
}

#[test]
fn diagnostic_spans_keep_empty_values_and_literal_tick_runs() {
    for (value, encoded) in [
        ("", "``  ``"),
        (" a \r\nb ", "` a \r\nb `"),
        ("`", "`` ` ``"),
        ("a``b", "``` a``b ```"),
        (" `\r\n ", "``  `\r\n  ``"),
    ] {
        assert_eq!(rustify_docs_syntax::code_span(value), encoded);
        assert_eq!(
            rustify_docs_syntax::literal(value),
            if value.contains('`') { encoded } else { value }
        );
    }
}
