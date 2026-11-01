//! Native definition lists with responsive presentation and component-local label color.

use crate::geometry::{PropError, StyleSet};
use crate::{Align, Margin, Responsive, TextTrim};
use leptos::prelude::*;
use rustify_ui::AccentColor;

/// Definition-list text size and spacing.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DataListSize {
    One,
    #[default]
    Two,
    Three,
}
impl DataListSize {
    pub const ALL: &'static [Self] = &[Self::One, Self::Two, Self::Three];
}
impl From<DataListSize> for Signal<Responsive<DataListSize>> {
    fn from(value: DataListSize) -> Self {
        Signal::stored(value.into())
    }
}

/// Places the label beside or above its values without changing their native association.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DataListOrientation {
    #[default]
    Horizontal,
    Vertical,
}
impl DataListOrientation {
    pub const ALL: &'static [Self] = &[Self::Horizontal, Self::Vertical];
}
impl From<DataListOrientation> for Signal<Responsive<DataListOrientation>> {
    fn from(value: DataListOrientation) -> Self {
        Signal::stored(value.into())
    }
}

fn root_styles(
    size: &Responsive<DataListSize>,
    orientation: &Responsive<DataListOrientation>,
    trim: &Responsive<TextTrim>,
    margin: &Margin,
) -> Result<StyleSet, PropError> {
    let mut styles = StyleSet::default();
    for (width, size) in size.iter() {
        let (font, gap) = match size {
            DataListSize::One => (1, "var(--rustify-space-3, 12px)"),
            DataListSize::Two => (2, "var(--rustify-space-4, 16px)"),
            DataListSize::Three => (3, "calc(var(--rustify-space-4, 16px) * 1.25)"),
        };
        styles.insert(
            width,
            "font-size",
            format!("var(--rustify-font-size-{font})"),
        )?;
        styles.insert(
            width,
            "--rustify-data-list-line-height",
            format!("var(--rustify-line-height-{font})"),
        )?;
        styles.insert(width, "--rustify-data-list-gap", gap)?;
    }
    for (width, orientation) in orientation.iter() {
        let (display, item_display, columns, label_min_width, value_column, align) =
            match orientation {
                DataListOrientation::Horizontal => (
                    "grid",
                    "grid",
                    "auto minmax(0, 1fr)",
                    "120px",
                    "2",
                    "baseline",
                ),
                DataListOrientation::Vertical => ("flex", "flex", "none", "0px", "auto", "stretch"),
            };
        for (property, value) in [
            ("--rustify-data-list-display", display),
            ("--rustify-data-list-item-display", item_display),
            ("--rustify-data-list-columns", columns),
            ("--rustify-data-list-label-min-width", label_min_width),
            ("--rustify-data-list-value-column", value_column),
            ("--rustify-data-list-default-align", align),
            (
                "--rustify-data-list-value-trim-factor",
                if *orientation == DataListOrientation::Horizontal {
                    "1"
                } else {
                    "0"
                },
            ),
        ] {
            styles.insert(width, property, value)?;
        }
    }
    styles.set("--rustify-data-list-trim-start", trim, |trim| {
        if matches!(trim, TextTrim::Start | TextTrim::Both) {
            "calc(0.42em - var(--rustify-data-list-line-height, 20px) / 2)"
        } else {
            "0px"
        }
        .into()
    })?;
    styles.set("--rustify-data-list-trim-end", trim, |trim| {
        if matches!(trim, TextTrim::End | TextTrim::Both) {
            "calc(0.36em - var(--rustify-data-list-line-height, 20px) / 2)"
        } else {
            "0px"
        }
        .into()
    })?;
    margin.write(&mut styles)?;
    Ok(styles)
}

/// A native definition list. Each Item can contain one label and multiple values.
#[component]
pub fn Root(
    #[prop(optional, into)] size: Signal<Responsive<DataListSize>>,
    #[prop(optional, into)] orientation: Signal<Responsive<DataListOrientation>>,
    #[prop(optional, into)] trim: Signal<Responsive<TextTrim>>,
    #[prop(optional, into)] margin: Signal<Margin>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let geometry = crate::geometry::style_class(move || {
        root_styles(&size.get(), &orientation.get(), &trim.get(), &margin.get())
    });
    view! { <dl class=crate::macros::reactive_merge(format!("rustify-data-list {geometry}"), &class) data-name="DataList" data-testid=test_id>{children()}</dl> }
}

/// A native grouping div that keeps its dt and dd children together.
#[component]
pub fn Item(
    #[prop(optional, into)] align: Signal<Responsive<Align>>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let geometry = crate::geometry::style_class(move || {
        let mut styles = StyleSet::default();
        let align = align.get();
        styles.set("--rustify-data-list-item-align", &align, |value| {
            value.css().into()
        })?;
        for (width, align) in align.iter() {
            let (start, end, first, last) = match align {
                Align::Start => ("0px", "-0.25em", "0px", "0px"),
                Align::Center => ("-0.25em", "-0.25em", "-0.25em", "-0.25em"),
                Align::End => ("-0.25em", "0px", "0px", "0px"),
                Align::Baseline => ("-0.25em", "-0.25em", "0px", "0px"),
                Align::Stretch => ("0px", "0px", "0px", "0px"),
            };
            for (property, value) in [
                ("--rustify-data-list-value-trim-start", start),
                ("--rustify-data-list-value-trim-end", end),
                ("--rustify-data-list-first-value-trim-start", first),
                ("--rustify-data-list-last-value-trim-end", last),
            ] {
                styles.insert(width, property, value)?;
            }
        }
        Ok(styles)
    });
    view! { <div class=crate::macros::reactive_merge(format!("rustify-data-list-item {geometry}"), &class) data-testid=test_id>{children()}</div> }
}

/// A native term, with optional label-only dimensions and palette.
#[component]
pub fn Label(
    #[prop(optional, into)] width: Signal<Responsive<String>>,
    #[prop(optional, into)] min_width: Signal<Responsive<String>>,
    #[prop(optional, into)] max_width: Signal<Responsive<String>>,
    #[prop(optional, into)] color: Signal<Option<AccentColor>>,
    #[prop(optional, into)] high_contrast: Signal<bool>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let mode = crate::component_color::use_mode();
    let geometry = crate::geometry::style_class(move || {
        let mut styles = StyleSet::default();
        styles.set("width", &width.get(), Clone::clone)?;
        styles.set("min-width", &min_width.get(), Clone::clone)?;
        styles.set("max-width", &max_width.get(), Clone::clone)?;
        crate::component_color::set_color(&mut styles, color.get(), mode.get())?;
        Ok(styles)
    });
    view! { <dt class=crate::macros::reactive_merge(format!("rustify-data-list-label rustify-color {geometry}"), &class)
    data-color=move || color.get().map(AccentColor::as_str)
    data-high-contrast=move || high_contrast.get().then_some("true") data-testid=test_id>{children()}</dt> }
}

/// One native definition. Multiple sibling values retain their full text and order.
#[component]
pub fn Value(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    view! { <dd class=crate::macros::merge("rustify-data-list-value", &class) data-testid=test_id>{children()}</dd> }
}
