//! Static native tables. Put the table inside a local scrollport when it can overflow.

use crate::geometry::{PropError, StyleSet};
use crate::{Margin, Padding, Responsive};
use leptos::prelude::*;

/// Table density, independent of its native semantics.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TableSize {
    One,
    #[default]
    Two,
    Three,
}
impl TableSize {
    pub const ALL: &'static [Self] = &[Self::One, Self::Two, Self::Three];
}
impl From<TableSize> for Signal<Responsive<TableSize>> {
    fn from(value: TableSize) -> Self {
        Signal::stored(value.into())
    }
}

/// The outer table surface.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TableVariant {
    Surface,
    #[default]
    Ghost,
}
impl TableVariant {
    pub const ALL: &'static [Self] = &[Self::Surface, Self::Ghost];
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Surface => "surface",
            Self::Ghost => "ghost",
        }
    }
}

/// The browser's native table layout algorithm.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TableLayout {
    #[default]
    Auto,
    Fixed,
}
impl TableLayout {
    pub const ALL: &'static [Self] = &[Self::Auto, Self::Fixed];
    pub const fn css(self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::Fixed => "fixed",
        }
    }
}
impl From<TableLayout> for Signal<Responsive<TableLayout>> {
    fn from(value: TableLayout) -> Self {
        Signal::stored(value.into())
    }
}

/// Vertical alignment of cells in a row.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TableRowAlign {
    Start,
    Center,
    End,
    Baseline,
}
impl TableRowAlign {
    pub const ALL: &'static [Self] = &[Self::Start, Self::Center, Self::End, Self::Baseline];
    pub const fn css(self) -> &'static str {
        match self {
            Self::Start => "top",
            Self::Center => "middle",
            Self::End => "bottom",
            Self::Baseline => "baseline",
        }
    }
}
impl From<TableRowAlign> for Signal<Responsive<TableRowAlign>> {
    fn from(value: TableRowAlign) -> Self {
        Signal::stored(value.into())
    }
}

/// Logical horizontal alignment of cell content.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TableCellJustify {
    Start,
    Center,
    End,
}
impl TableCellJustify {
    pub const ALL: &'static [Self] = &[Self::Start, Self::Center, Self::End];
    pub const fn css(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::Center => "center",
            Self::End => "end",
        }
    }
}
impl From<TableCellJustify> for Signal<Responsive<TableCellJustify>> {
    fn from(value: TableCellJustify) -> Self {
        Signal::stored(value.into())
    }
}

fn size_styles(styles: &mut StyleSet, size: &Responsive<TableSize>) -> Result<(), PropError> {
    for (width, size) in size.iter() {
        let (padding, height, radius, font) = match size {
            TableSize::One => (
                "var(--rustify-space-2, 8px)",
                "calc(var(--rustify-space-6, 32px) + var(--rustify-space-1, 4px))",
                "var(--rustify-radius-3, 6px)",
                2,
            ),
            TableSize::Two => (
                "var(--rustify-space-3, 12px)",
                "calc(var(--rustify-space-7, 40px) + var(--rustify-space-1, 4px))",
                "var(--rustify-radius-4, 8px)",
                2,
            ),
            TableSize::Three => (
                "var(--rustify-space-3, 12px) var(--rustify-space-4, 16px)",
                "var(--rustify-space-8, 48px)",
                "var(--rustify-radius-4, 8px)",
                3,
            ),
        };
        styles.insert(width, "--rustify-table-cell-padding", padding)?;
        styles.insert(width, "--rustify-table-cell-min-height", height)?;
        styles.insert(width, "--rustify-table-radius", radius)?;
        styles.insert(
            width,
            "font-size",
            format!("var(--rustify-font-size-{font})"),
        )?;
        styles.insert(
            width,
            "line-height",
            format!("var(--rustify-line-height-{font})"),
        )?;
    }
    Ok(())
}

/// A native table; horizontal overflow belongs to the caller's surrounding scrollport.
#[component]
pub fn Root(
    #[prop(optional, into)] size: Signal<Responsive<TableSize>>,
    #[prop(optional, into)] variant: Signal<TableVariant>,
    #[prop(optional, into)] layout: Signal<Responsive<TableLayout>>,
    #[prop(optional, into)] margin: Signal<Margin>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let geometry = crate::geometry::style_class(move || {
        let mut styles = StyleSet::default();
        size_styles(&mut styles, &size.get())?;
        styles.set("table-layout", &layout.get(), |value| value.css().into())?;
        margin.get().write(&mut styles)?;
        Ok(styles)
    });
    view! {
        <table class=crate::macros::reactive_merge(format!("rustify-table {geometry}"), &class)
            data-name="Table" data-testid=test_id data-variant=move || variant.get().as_str()>
            {children()}
        </table>
    }
}

/// The table's native column-header section.
#[component]
pub fn Header(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    view! { <thead class=crate::macros::merge("rustify-table-header", &class) data-testid=test_id>{children()}</thead> }
}

/// The table's native body section.
#[component]
pub fn Body(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    view! { <tbody class=crate::macros::merge("rustify-table-body", &class) data-testid=test_id>{children()}</tbody> }
}

/// A native row with optional responsive vertical cell alignment.
#[component]
pub fn Row(
    #[prop(optional, into)] align: Signal<Responsive<TableRowAlign>>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let geometry = crate::geometry::style_class(move || {
        let mut styles = StyleSet::default();
        styles.set("vertical-align", &align.get(), |value| value.css().into())?;
        Ok(styles)
    });
    view! { <tr class=crate::macros::reactive_merge(format!("rustify-table-row {geometry}"), &class) data-testid=test_id>{children()}</tr> }
}

#[derive(Clone, Copy)]
struct CellStyle {
    justify: Signal<Responsive<TableCellJustify>>,
    width: Signal<Responsive<String>>,
    min_width: Signal<Responsive<String>>,
    max_width: Signal<Responsive<String>>,
    padding: Signal<Padding>,
}
impl CellStyle {
    fn class(self) -> String {
        crate::geometry::style_class(move || {
            let mut styles = StyleSet::default();
            styles.set("text-align", &self.justify.get(), |value| {
                value.css().into()
            })?;
            styles.set("width", &self.width.get(), Clone::clone)?;
            styles.set("min-width", &self.min_width.get(), Clone::clone)?;
            styles.set("max-width", &self.max_width.get(), Clone::clone)?;
            self.padding.get().write(&mut styles)?;
            Ok(styles)
        })
    }
}

/// A native caption that supplies the table's visible description.
#[component]
pub fn Caption(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    view! { <caption class=crate::macros::merge("rustify-table-caption", &class) data-testid=test_id>{children()}</caption> }
}

/// A native data cell with optional spans and explicit header associations.
#[component]
pub fn Cell(
    #[prop(optional, into)] justify: Signal<Responsive<TableCellJustify>>,
    #[prop(optional, into)] width: Signal<Responsive<String>>,
    #[prop(optional, into)] min_width: Signal<Responsive<String>>,
    #[prop(optional, into)] max_width: Signal<Responsive<String>>,
    #[prop(optional, into)] padding: Signal<Padding>,
    #[prop(optional, into)] colspan: Signal<Option<u32>>,
    #[prop(optional, into)] rowspan: Signal<Option<u32>>,
    #[prop(optional, into)] headers: Signal<String>,
    #[prop(optional, into)] id: Signal<String>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let geometry = CellStyle {
        justify,
        width,
        min_width,
        max_width,
        padding,
    }
    .class();
    view! { <td class=crate::macros::reactive_merge(format!("rustify-table-cell {geometry}"), &class)
    colspan=move || colspan.get() rowspan=move || rowspan.get()
    headers=move || { let value = headers.get(); (!value.is_empty()).then_some(value) }
    id=move || { let value = id.get(); (!value.is_empty()).then_some(value) } data-testid=test_id>{children()}</td> }
}

/// A native column header with scope="col".
#[component]
pub fn ColumnHeaderCell(
    #[prop(optional, into)] justify: Signal<Responsive<TableCellJustify>>,
    #[prop(optional, into)] width: Signal<Responsive<String>>,
    #[prop(optional, into)] min_width: Signal<Responsive<String>>,
    #[prop(optional, into)] max_width: Signal<Responsive<String>>,
    #[prop(optional, into)] padding: Signal<Padding>,
    #[prop(optional, into)] colspan: Signal<Option<u32>>,
    #[prop(optional, into)] rowspan: Signal<Option<u32>>,
    #[prop(optional, into)] headers: Signal<String>,
    #[prop(optional, into)] id: Signal<String>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let geometry = CellStyle {
        justify,
        width,
        min_width,
        max_width,
        padding,
    }
    .class();
    view! { <th class=crate::macros::reactive_merge(format!("rustify-table-cell rustify-table-column-header-cell {geometry}"), &class)
    scope="col"
    colspan=move || colspan.get() rowspan=move || rowspan.get()
    headers=move || { let value = headers.get(); (!value.is_empty()).then_some(value) }
    id=move || { let value = id.get(); (!value.is_empty()).then_some(value) } data-testid=test_id>{children()}</th> }
}

/// A native row header with scope="row".
#[component]
pub fn RowHeaderCell(
    #[prop(optional, into)] justify: Signal<Responsive<TableCellJustify>>,
    #[prop(optional, into)] width: Signal<Responsive<String>>,
    #[prop(optional, into)] min_width: Signal<Responsive<String>>,
    #[prop(optional, into)] max_width: Signal<Responsive<String>>,
    #[prop(optional, into)] padding: Signal<Padding>,
    #[prop(optional, into)] colspan: Signal<Option<u32>>,
    #[prop(optional, into)] rowspan: Signal<Option<u32>>,
    #[prop(optional, into)] headers: Signal<String>,
    #[prop(optional, into)] id: Signal<String>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let geometry = CellStyle {
        justify,
        width,
        min_width,
        max_width,
        padding,
    }
    .class();
    view! { <th class=crate::macros::reactive_merge(format!("rustify-table-cell rustify-table-row-header-cell {geometry}"), &class)
    scope="row"
    colspan=move || colspan.get() rowspan=move || rowspan.get()
    headers=move || { let value = headers.get(); (!value.is_empty()).then_some(value) }
    id=move || { let value = id.get(); (!value.is_empty()).then_some(value) } data-testid=test_id>{children()}</th> }
}
