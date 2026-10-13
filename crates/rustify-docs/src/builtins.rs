//! Typed static content blocks, rendered through the same site registry.

use crate::site::{SiteError, SiteRegistry};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TabSpec {
    pub label: String,
    pub content: String,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CardSpec {
    pub title: String,
    pub link: String,
    pub description: String,
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StepSpec {
    pub title: String,
    pub description: String,
}

/// The declaration is the props schema and the typed renderer registration.
pub fn registry() -> SiteRegistry {
    crate::site! {
        components {
            Tabs { tabs: Vec<TabSpec> } => |props, children| tabs(props.tabs, children),
            CardGrid { cards: Vec<CardSpec>, columns: u8 = 2 } =>
                |props, children| card_grid(props.cards, props.columns, children),
            Steps { items: Vec<StepSpec> } => |props, children| steps(props.items, children),
        }
    }
}

pub fn is_builtin(name: &str) -> bool {
    matches!(name, "Tabs" | "CardGrid" | "Steps")
}

/// Produces static HTML without an interactive scope or author Rust build.
pub fn render(name: &str, props: Value, children_html: String) -> Result<String, SiteError> {
    if name == "CardGrid" {
        if let Some(columns) = props.get("columns").and_then(Value::as_u64) {
            if !(1..=3).contains(&columns) {
                return Err(
                    SiteError::new("invalid_props", "CardGrid.columns must be 1, 2 or 3")
                        .with_property("columns"),
                );
            }
        }
    }
    registry().render_component_static(name, props, children_html)
}

fn tabs(items: Vec<TabSpec>, children: AnyView) -> impl IntoView {
    let buttons = items
        .iter()
        .enumerate()
        .map(|(index, item)| {
            let label = item.label.clone();
            view! { <button type="button" data-rd-static-tab=index.to_string()>{label}</button> }
        })
        .collect_view();
    let panels = items.into_iter().enumerate().map(|(index, item)| {
        let label = item.label.clone();
        view! {
            <section class="rd-static-panel" data-rd-static-panel=index.to_string() aria-label=label>
                <p class="rd-static-tab-title">{item.label}</p>
                <p class="rd-static-text">{item.content}</p>
            </section>
        }
    }).collect_view();
    view! {
        <div class="rd-static-tabs" data-rd-static-tabs="">
            <div class="rd-static-tabbar" data-rd-static-tabbar="" hidden>{buttons}</div>
            {panels}
            {children}
        </div>
    }
}

fn card_grid(cards: Vec<CardSpec>, columns: u8, children: AnyView) -> impl IntoView {
    let cards = cards
        .into_iter()
        .map(|card| {
            view! {
                <li>
                    <a class="rd-static-card-title" href=card.link>{card.title}</a>
                    <p class="rd-static-text">{card.description}</p>
                </li>
            }
        })
        .collect_view();
    view! {
        <div class="rd-static-card-grid" data-columns=columns.to_string()>
            <ul>{cards}</ul>
            {children}
        </div>
    }
}

fn steps(items: Vec<StepSpec>, children: AnyView) -> impl IntoView {
    let items = items.into_iter().map(|item| {
        view! {
            <li><p class="rd-static-step-title">{item.title}</p><p class="rd-static-text">{item.description}</p></li>
        }
    }).collect_view();
    view! { <div class="rd-static-steps"><ol>{items}</ol>{children}</div> }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn static_tabs_keep_every_panel_and_one_unchanged_children_marker() {
        let children = "<h2 id=\"details\">Details</h2><a href=\"guide.md#next\">Next</a>";
        let html = render("Tabs", json!({"tabs":[{"label":"Install <&>","content":"cargo <add> & run"},{"label":"Usage","content":"More"}]}), children.to_string()).unwrap();
        let dom = dom_query::Document::from(html.as_str());
        assert_eq!(
            dom.select(".rd-static-tabs[data-rd-static-tabs] > .rd-static-tabbar[data-rd-static-tabbar][hidden]").length(),
            1
        );
        assert_eq!(
            dom.select("button[type=button][data-rd-static-tab]")
                .length(),
            2
        );
        assert_eq!(
            dom.select(".rd-static-panel[data-rd-static-panel]")
                .length(),
            2
        );
        assert_eq!(dom.select(".rd-static-panel[hidden]").length(), 0);
        assert_eq!(
            dom.select("[data-rd-static-panel=\"0\"]")
                .attr("aria-label")
                .as_deref(),
            Some("Install <&>")
        );
        assert_eq!(
            dom.select("[data-rd-static-panel=\"0\"] .rd-static-text")
                .text()
                .to_string(),
            "cargo <add> & run"
        );
        assert!(html.contains("cargo &lt;add&gt; &amp; run"), "{html}");
        assert_eq!(
            dom.select(".rd-static-tabs > span[data-rustify-children]")
                .length(),
            1
        );
        assert_eq!(
            dom.select("span[data-rustify-children] > h2#details")
                .text()
                .to_string(),
            "Details"
        );
        assert_eq!(
            dom.select("span[data-rustify-children] > a")
                .attr("href")
                .as_deref(),
            Some("guide.md#next")
        );
        assert!(html.contains(children));
        assert_eq!(
            dom.select("[data-rustify-scope], [data-rustify-island]")
                .length(),
            0
        );
    }

    #[test]
    fn card_grid_defaults_and_steps_use_plaintext_and_native_links() {
        let html = render("CardGrid", json!({"cards":[{"title":"Guide <&>","link":"guide.md?one=1&two=2","description":"Read <this>"}]}), String::new()).unwrap();
        assert!(html.contains("data-columns=\"2\""), "{html}");
        assert!(html.contains("href=\"guide.md?one=1&amp;two=2\""), "{html}");
        assert!(html.contains("Guide &lt;&amp;&gt;"), "{html}");
        assert!(html.contains("Read &lt;this&gt;"), "{html}");
        let html = render(
            "Steps",
            json!({"items":[{"title":"First","description":"A < B"}]}),
            String::new(),
        )
        .unwrap();
        assert!(html.contains("<ol>"), "{html}");
        assert!(html.contains("A &lt; B"), "{html}");
        assert!(!html.contains("<h"), "{html}");
    }

    #[test]
    fn generated_props_reject_unknown_nested_fields_wrong_types_and_columns() {
        for (name, props) in [
            (
                "Tabs",
                json!({"tabs":[{"label":"One","content":"Text","extra":true}]}),
            ),
            (
                "CardGrid",
                json!({"cards":[{"title":"One","link":"x","description":"Text","extra":true}]}),
            ),
            (
                "Steps",
                json!({"items":[{"title":"One","description":"Text","extra":true}]}),
            ),
            ("Tabs", json!({"tabs":"wrong"})),
            ("CardGrid", json!({"cards":[],"columns":0})),
            ("CardGrid", json!({"cards":[],"columns":4})),
            ("CardGrid", json!({"cards":[],"columns":"2"})),
            ("Steps", json!({"items":[],"unexpected":true})),
        ] {
            let invalid_columns =
                name == "CardGrid" && props.get("columns").is_some_and(Value::is_u64);
            let error = render(name, props, String::new()).unwrap_err();
            assert_eq!(error.code, "invalid_props", "{error}");
            if invalid_columns {
                assert_eq!(error.property.as_deref(), Some("columns"));
            }
        }
        assert_eq!(
            render("Unknown", json!({}), String::new())
                .unwrap_err()
                .code,
            "unknown_component"
        );
    }

    #[test]
    fn builtin_names_and_defaults_agree_with_the_one_registry_manifest() {
        let registry = registry();
        let components = registry.manifest()["components"].as_array().unwrap();
        assert_eq!(components.len(), 3);
        for component in components {
            assert!(is_builtin(component["name"].as_str().unwrap()));
        }
        assert_eq!(components[1]["fields"][1]["default"], "2");
    }
}
