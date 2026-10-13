use leptos::prelude::*;

fn declaration() -> rustify_docs::SiteRegistry {
    rustify_docs::site! {
        components {
            Example {
                label: String = "Counter".to_string(),
                values: Vec<Option<u64>> = vec![Some(1), None],
                character: char = 'λ',
            } => |props, children| view! {
                <div>{props.label}{props.values.len()}{props.character}{children}</div>
            },
        }
        demos { ExampleDemo => crate::example_demo, }
    }
}

fn example_demo() -> impl IntoView {
    view! { <p>"The same source function"</p> }
}

#[test]
fn native_macro_and_cold_file_reader_have_identical_manifest_identity() {
    let parsed = rustify_docs_syntax::parse_registry(include_str!("site_metadata.rs")).unwrap();
    assert_eq!(
        declaration().manifest(),
        &serde_json::to_value(parsed.manifest()).unwrap(),
    );
}
