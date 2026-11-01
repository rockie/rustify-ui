#![cfg(all(feature = "ssr", not(target_arch = "wasm32")))]

use leptos::prelude::*;
use rustify_components::{data_list as dl, table as t, GeometryStyles, Responsive};

fn render_tables(view: impl FnOnce() -> AnyView + Send + 'static) -> (String, GeometryStyles) {
    let styles = GeometryStyles::default();
    let output = styles.clone();
    let html = Owner::new().with(|| {
        rustify_ui::ssr_scope(
            rustify_ui::MountConfig {
                scope: "content-tables".into(),
                ..Default::default()
            },
            move || {
                rustify_components::provide_geometry_styles(output);
                view()
            },
        )
        .to_html()
    });
    (html, styles)
}

#[test]
fn invalid_table_and_label_dimensions_are_diagnosed_without_unsafe_rules() {
    let (html, styles) = render_tables(|| {
        view! {
        <t::Root><t::Body><t::Row>
            <t::Cell width=Responsive::from("10px; color: red".to_string())>"Retained cell"</t::Cell>
            <t::Cell padding=rustify_components::Padding { p: Responsive::from(rustify_components::Spacing::Scale(-1)), ..Default::default() }>"Retained padding"</t::Cell>
        </t::Row></t::Body></t::Root>
        <dl::Root><dl::Item><dl::Label max_width=Responsive::from("not-a-length".to_string())>"Retained label"</dl::Label><dl::Value>"Value"</dl::Value></dl::Item></dl::Root>
    }.into_any()
    });
    assert!(
        html.contains("Retained cell")
            && html.contains("Retained padding")
            && html.contains("Retained label"),
        "{html}"
    );
    let fields: Vec<_> = styles
        .diagnostics()
        .into_iter()
        .map(|error| error.field)
        .collect();
    assert!(
        fields.contains(&"width") && fields.contains(&"p") && fields.contains(&"max-width"),
        "{fields:?}"
    );
    assert!(
        !styles.css().contains("color: red") && !styles.css().contains("not-a-length"),
        "{}",
        styles.css()
    );
    assert!(!html.contains(" style="), "{html}");
}

#[test]
fn tables_render_native_headers_spans_and_caption_without_a_grid_role() {
    let (html, _) = render_tables(|| {
        view! {
        <t::Root attr:aria-label="Local entries">
            <t::Caption>"Local entries by owner"</t::Caption>
            <t::Header><t::Row><t::ColumnHeaderCell id="owner" rowspan=Some(2)>"Owner"</t::ColumnHeaderCell><t::ColumnHeaderCell id="total" colspan=Some(2)>"Total"</t::ColumnHeaderCell></t::Row></t::Header>
            <t::Body><t::Row><t::RowHeaderCell id="row-a" headers="owner">"Ada"</t::RowHeaderCell><t::Cell headers="row-a total">"12"</t::Cell><t::Cell>"items"</t::Cell></t::Row></t::Body>
        </t::Root>
    }.into_any()
    });
    for tag in [
        "<table", "<caption", "<thead", "<tbody", "<tr", "<th", "<td",
    ] {
        assert!(html.contains(tag), "missing {tag}: {html}");
    }
    assert_eq!(html.matches("scope=\"col\"").count(), 2, "{html}");
    assert_eq!(html.matches("scope=\"row\"").count(), 1, "{html}");
    for attribute in [
        "rowspan=\"2\"",
        "colspan=\"2\"",
        "headers=\"row-a total\"",
        "id=\"row-a\"",
        "aria-label=\"Local entries\"",
    ] {
        assert!(html.contains(attribute), "missing {attribute}: {html}");
    }
    let table = html
        .split("<table")
        .nth(1)
        .unwrap()
        .split("</table>")
        .next()
        .unwrap();
    assert!(
        !table.contains("role=\"grid\"") && !table.contains("tabindex="),
        "{html}"
    );
}

#[test]
fn definition_lists_keep_multiple_values_with_their_native_label() {
    let long_value = "A deliberately long definition that stays readable without truncating or removing any of its text.";
    let (html, _) = render_tables(move || {
        view! {
        <dl::Root><dl::Item><dl::Label>"Maintainers"</dl::Label><dl::Value>"Ada"</dl::Value><dl::Value>{long_value}</dl::Value></dl::Item></dl::Root>
    }.into_any()
    });
    assert!(
        html.contains("<dl") && html.contains("<dt") && html.contains("<dd"),
        "{html}"
    );
    assert_eq!(html.matches("<dt").count(), 1, "{html}");
    assert_eq!(html.matches("<dd").count(), 2, "{html}");
    let label = html.find("Maintainers").unwrap();
    let first = html.find("Ada").unwrap();
    let second = html.find(long_value).unwrap();
    assert!(label < first && first < second, "{html}");
    assert!(!html.contains("role="), "{html}");
}

#[test]
fn table_and_list_geometry_is_collected_at_the_declared_breakpoints() {
    let (html, styles) = render_tables(|| {
        view! {
        <t::Root size=Responsive { initial: Some(t::TableSize::One), md: Some(t::TableSize::Three), ..Default::default() }
            variant=t::TableVariant::Surface layout=Responsive { initial: Some(t::TableLayout::Auto), sm: Some(t::TableLayout::Fixed), ..Default::default() }>
            <t::Body><t::Row align=Responsive::from(t::TableRowAlign::Center)><t::Cell justify=Responsive::from(t::TableCellJustify::End)
                width=Responsive::from("12rem".to_string()) min_width=Responsive::from("8rem".to_string()) max_width=Responsive::from("20rem".to_string())
                padding=rustify_components::Padding { px: Responsive::from(rustify_components::Spacing::Scale(4)), ..Default::default() }>"Cell"</t::Cell></t::Row></t::Body>
        </t::Root>
        <dl::Root size=dl::DataListSize::Three orientation=Responsive { initial: Some(dl::DataListOrientation::Vertical), sm: Some(dl::DataListOrientation::Horizontal), ..Default::default() }
            trim=Responsive::from(rustify_components::TextTrim::Both)>
            <dl::Item align=Responsive::from(rustify_components::Align::Center)><dl::Label width=Responsive::from("10rem".to_string()) color=Some(rustify_ui::AccentColor::Indigo) high_contrast=true>"Label"</dl::Label><dl::Value>"Value"</dl::Value></dl::Item>
        </dl::Root>
    }.into_any()
    });
    let css = styles.css();
    for rule in [
        "@media (min-width:768px)",
        "@media (min-width:1024px)",
        "table-layout:fixed",
        "vertical-align:middle",
        "text-align:end",
        "width:12rem",
        "min-width:8rem",
        "max-width:20rem",
        "padding-left:var(--rustify-space-4)",
        "--rustify-data-list-display:flex",
        "--rustify-data-list-display:grid",
        "--rustify-data-list-item-align:center",
        "--rustify-data-list-trim-start:calc(",
        "--rustify-component-color-12:",
    ] {
        assert!(css.contains(rule), "missing {rule}: {css}");
    }
    assert!(
        html.contains("data-variant=\"surface\"") && html.contains("data-high-contrast=\"true\""),
        "{html}"
    );
    assert!(
        styles.diagnostics().is_empty(),
        "{:?}",
        styles.diagnostics()
    );
    assert!(!html.contains(" style="), "{html}");
}

#[test]
fn every_table_and_list_size_emits_its_actual_density() {
    for (size, padding, height) in [
        (
            t::TableSize::One,
            "var(--rustify-space-2, 8px)",
            "calc(var(--rustify-space-6, 32px) + var(--rustify-space-1, 4px))",
        ),
        (
            t::TableSize::Two,
            "var(--rustify-space-3, 12px)",
            "calc(var(--rustify-space-7, 40px) + var(--rustify-space-1, 4px))",
        ),
        (
            t::TableSize::Three,
            "var(--rustify-space-3, 12px) var(--rustify-space-4, 16px)",
            "var(--rustify-space-8, 48px)",
        ),
    ] {
        let (html, styles) = render_tables(move || {
            view! {
            <t::Root size=size><t::Body><t::Row><t::Cell>"Density"</t::Cell></t::Row></t::Body></t::Root>
        }.into_any()
        });
        assert!(
            styles
                .css()
                .contains(&format!("--rustify-table-cell-padding:{padding};")),
            "{}",
            styles.css()
        );
        assert!(
            styles
                .css()
                .contains(&format!("--rustify-table-cell-min-height:{height};")),
            "{}",
            styles.css()
        );
        assert!(
            html.contains("data-variant=\"ghost\"") && html.contains("Density"),
            "{html}"
        );
        assert!(
            styles.diagnostics().is_empty(),
            "{:?}",
            styles.diagnostics()
        );
    }
    for (size, font, gap) in [
        (dl::DataListSize::One, 1, "var(--rustify-space-3, 12px)"),
        (dl::DataListSize::Two, 2, "var(--rustify-space-4, 16px)"),
        (
            dl::DataListSize::Three,
            3,
            "calc(var(--rustify-space-4, 16px) * 1.25)",
        ),
    ] {
        let (html, styles) = render_tables(move || {
            view! {
            <dl::Root size=size><dl::Item><dl::Label>"Label"</dl::Label><dl::Value>"Density"</dl::Value></dl::Item></dl::Root>
        }.into_any()
        });
        assert!(
            styles
                .css()
                .contains(&format!("font-size:var(--rustify-font-size-{font});")),
            "{}",
            styles.css()
        );
        assert!(
            styles
                .css()
                .contains(&format!("--rustify-data-list-gap:{gap};")),
            "{}",
            styles.css()
        );
        assert!(html.contains("Density"), "{html}");
        assert!(
            styles.diagnostics().is_empty(),
            "{:?}",
            styles.diagnostics()
        );
    }
}
