//! DOM layout primitives and their shared responsive property groups.

use crate::geometry::{PropError, StyleSet};
use crate::responsive::Responsive;
use leptos::prelude::*;

/// A theme space step, or an absolute CSS value that is never scaled.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Spacing {
    /// A signed theme space step, from -9 through 9. Padding and gap reject negatives.
    Scale(i8),
    /// One CSS property value, validated identically on the host and browser.
    Css(String),
}
impl From<i8> for Spacing {
    fn from(value: i8) -> Self {
        Self::Scale(value)
    }
}
impl From<&str> for Spacing {
    fn from(value: &str) -> Self {
        Self::Css(value.into())
    }
}
impl From<String> for Spacing {
    fn from(value: String) -> Self {
        Self::Css(value)
    }
}

/// Responsive margins. More specific axes and edges override the shorthand.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Margin {
    pub m: Responsive<Spacing>,
    pub mx: Responsive<Spacing>,
    pub my: Responsive<Spacing>,
    pub mt: Responsive<Spacing>,
    pub mr: Responsive<Spacing>,
    pub mb: Responsive<Spacing>,
    pub ml: Responsive<Spacing>,
}

/// Responsive padding. Theme steps are restricted to 0 through 9.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Padding {
    pub p: Responsive<Spacing>,
    pub px: Responsive<Spacing>,
    pub py: Responsive<Spacing>,
    pub pt: Responsive<Spacing>,
    pub pr: Responsive<Spacing>,
    pub pb: Responsive<Spacing>,
    pub pl: Responsive<Spacing>,
}

/// Responsive grid and flex gaps. Theme steps are restricted to 0 through 9.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Gap {
    pub gap: Responsive<Spacing>,
    pub gap_x: Responsive<Spacing>,
    pub gap_y: Responsive<Spacing>,
}

macro_rules! css_enum {
    ($(#[$meta:meta])* pub enum $name:ident { $($variant:ident => $css:literal),+ $(,)? }) => {
        $(#[$meta])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub enum $name { $($variant),+ }
        impl $name { pub fn css(self) -> &'static str { match self { $(Self::$variant => $css),+ } } }
    };
}

css_enum! { /// CSS positioning mode.
pub enum Position { Static => "static", Relative => "relative", Absolute => "absolute", Fixed => "fixed", Sticky => "sticky" } }
css_enum! { /// Overflow on one or both axes.
pub enum Overflow { Visible => "visible", Hidden => "hidden", Clip => "clip", Scroll => "scroll", Auto => "auto" } }
css_enum! { /// Alignment of an item within its layout area.
pub enum Align { Start => "start", Center => "center", End => "end", Baseline => "baseline", Stretch => "stretch" } }
css_enum! { /// Alignment of the items on the main axis.
pub enum Justify { Start => "flex-start", Center => "center", End => "flex-end", Between => "space-between" } }
css_enum! { /// Alignment of grid tracks within their container.
pub enum AlignContent { Start => "start", Center => "center", End => "end", Baseline => "baseline", Between => "space-between", Around => "space-around", Evenly => "space-evenly", Stretch => "stretch" } }

/// Shared layout fields. Unset values emit no CSS declaration.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Layout {
    pub padding: Padding,
    pub width: Responsive<String>,
    pub min_width: Responsive<String>,
    pub max_width: Responsive<String>,
    pub height: Responsive<String>,
    pub min_height: Responsive<String>,
    pub max_height: Responsive<String>,
    pub position: Responsive<Position>,
    pub inset: Responsive<Spacing>,
    pub top: Responsive<Spacing>,
    pub right: Responsive<Spacing>,
    pub bottom: Responsive<Spacing>,
    pub left: Responsive<Spacing>,
    pub overflow: Responsive<Overflow>,
    pub overflow_x: Responsive<Overflow>,
    pub overflow_y: Responsive<Overflow>,
    pub flex_basis: Responsive<String>,
    pub flex_grow: Responsive<f64>,
    pub flex_shrink: Responsive<f64>,
    pub grid_area: Responsive<String>,
    pub grid_column: Responsive<String>,
    pub grid_column_start: Responsive<String>,
    pub grid_column_end: Responsive<String>,
    pub grid_row: Responsive<String>,
    pub grid_row_start: Responsive<String>,
    pub grid_row_end: Responsive<String>,
    pub align_self: Responsive<Align>,
    pub justify_self: Responsive<Align>,
}

impl Spacing {
    fn css(&self, field: &'static str, signed: bool) -> Result<String, PropError> {
        match self {
            Self::Scale(step) if (-9..=9).contains(step) && (signed || *step >= 0) => {
                if *step < 0 {
                    Ok(format!("calc(var(--rustify-space-{}) * -1)", -step))
                } else {
                    Ok(format!("var(--rustify-space-{step})"))
                }
            }
            Self::Scale(_) => Err(PropError {
                field,
                message: if signed {
                    "expected a theme space step from -9 through 9"
                } else {
                    "expected a theme space step from 0 through 9"
                }
                .into(),
            }),
            Self::Css(value) => Ok(value.clone()),
        }
    }
}

fn insert(
    styles: &mut StyleSet,
    width: u16,
    field: &'static str,
    property: &'static str,
    value: String,
) -> Result<(), PropError> {
    styles
        .insert(width, property, value)
        .map_err(|error| PropError { field, ..error })
}

fn shorthand(value: &str) -> Vec<String> {
    let mut start = 0;
    let mut depth = 0;
    let mut parts = Vec::new();
    for (index, character) in value.char_indices() {
        match character {
            '(' | '[' => depth += 1,
            ')' | ']' => depth -= 1,
            character if character.is_whitespace() && depth == 0 => {
                if start != index {
                    parts.push(value[start..index].into());
                }
                start = index + character.len_utf8();
            }
            _ => {}
        }
    }
    if start < value.len() {
        parts.push(value[start..].into());
    }
    parts
}

fn edges(
    styles: &mut StyleSet,
    fields: [(&'static str, &Responsive<Spacing>); 7],
    properties: [&'static str; 4],
    signed: bool,
    padding: bool,
) -> Result<(), PropError> {
    for width in crate::responsive::BREAKPOINTS {
        if !fields
            .iter()
            .any(|(_, value)| value.iter().any(|(minimum, _)| minimum == width))
        {
            continue;
        }
        let mut values: [Option<(&'static str, String)>; 4] = std::array::from_fn(|_| None);
        for (index, (field, responsive)) in fields.iter().enumerate() {
            let Some(value) = responsive.at(width) else {
                continue;
            };
            let value = value.css(field, signed)?;
            let mut check = StyleSet::default();
            check
                .insert(0, if padding { "padding" } else { "margin" }, value.clone())
                .map_err(|error| PropError { field, ..error })?;
            let parts = shorthand(&value);
            let selected: &[usize] = match index {
                0 => &[0, 1, 2, 3],
                1 => &[1, 3],
                2 => &[0, 2],
                3 => &[0],
                4 => &[1],
                5 => &[2],
                _ => &[3],
            };
            if index != 0 && parts.len() != 1 {
                return Err(PropError {
                    field,
                    message: "expected one length for an axis or edge".into(),
                });
            }
            for &side in selected {
                let part = match (parts.len(), side) {
                    (1, _) => 0,
                    (2, 0 | 2) => 0,
                    (2, _) => 1,
                    (3, 0) => 0,
                    (3, 1 | 3) => 1,
                    (3, _) => 2,
                    (_, side) => side,
                };
                values[side] = Some((field, parts[part].clone()));
            }
        }
        for (side, value) in values.into_iter().enumerate() {
            if let Some((field, value)) = value {
                insert(styles, width, field, properties[side], value.clone())?;
                if padding {
                    styles.insert(width, INSET_PROPERTIES[side], value)?;
                }
            }
        }
    }
    Ok(())
}

const INSET_PROPERTIES: [&str; 4] = [
    "--rustify-inset-top",
    "--rustify-inset-right",
    "--rustify-inset-bottom",
    "--rustify-inset-left",
];
const PADDING_PROPERTIES: [&str; 4] = [
    "padding-top",
    "padding-right",
    "padding-bottom",
    "padding-left",
];
const MARGIN_PROPERTIES: [&str; 4] = ["margin-top", "margin-right", "margin-bottom", "margin-left"];

impl Margin {
    /// Appends the complete margin group to a validated style set.
    pub fn write(&self, styles: &mut StyleSet) -> Result<(), PropError> {
        edges(
            styles,
            [
                ("m", &self.m),
                ("mx", &self.mx),
                ("my", &self.my),
                ("mt", &self.mt),
                ("mr", &self.mr),
                ("mb", &self.mb),
                ("ml", &self.ml),
            ],
            MARGIN_PROPERTIES,
            true,
            false,
        )
    }
}
impl Padding {
    /// Appends padding and the nearest-container inset values.
    pub fn write(&self, styles: &mut StyleSet) -> Result<(), PropError> {
        edges(
            styles,
            [
                ("p", &self.p),
                ("px", &self.px),
                ("py", &self.py),
                ("pt", &self.pt),
                ("pr", &self.pr),
                ("pb", &self.pb),
                ("pl", &self.pl),
            ],
            PADDING_PROPERTIES,
            false,
            true,
        )
    }
}
impl Gap {
    /// Appends the gap group, with axis values taking precedence.
    pub fn write(&self, styles: &mut StyleSet) -> Result<(), PropError> {
        for (field, property, responsive) in [
            ("gap", "gap", &self.gap),
            ("gap_x", "column-gap", &self.gap_x),
            ("gap_y", "row-gap", &self.gap_y),
        ] {
            for (width, value) in responsive.iter() {
                insert(styles, width, field, property, value.css(field, false)?)?;
            }
        }
        Ok(())
    }
}

impl Layout {
    /// Appends every shared layout field, including dimensions and padding.
    pub fn write(&self, styles: &mut StyleSet) -> Result<(), PropError> {
        self.write_outer(styles)?;
        self.write_dimensions(styles)
    }

    fn write_dimensions(&self, styles: &mut StyleSet) -> Result<(), PropError> {
        for (property, value) in [
            ("width", &self.width),
            ("min-width", &self.min_width),
            ("max-width", &self.max_width),
            ("height", &self.height),
            ("min-height", &self.min_height),
            ("max-height", &self.max_height),
        ] {
            styles.set(property, value, Clone::clone)?;
        }
        Ok(())
    }

    fn write_outer(&self, styles: &mut StyleSet) -> Result<(), PropError> {
        self.padding.write(styles)?;
        styles.set("position", &self.position, |value| value.css().into())?;
        edges(
            styles,
            [
                ("inset", &self.inset),
                ("inset", &Responsive::default()),
                ("inset", &Responsive::default()),
                ("top", &self.top),
                ("right", &self.right),
                ("bottom", &self.bottom),
                ("left", &self.left),
            ],
            ["top", "right", "bottom", "left"],
            true,
            false,
        )?;
        for (property, value) in [
            ("overflow", &self.overflow),
            ("overflow-x", &self.overflow_x),
            ("overflow-y", &self.overflow_y),
        ] {
            styles.set(property, value, |value| value.css().into())?;
        }
        for (property, value) in [
            ("flex-basis", &self.flex_basis),
            ("grid-area", &self.grid_area),
            ("grid-column", &self.grid_column),
            ("grid-column-start", &self.grid_column_start),
            ("grid-column-end", &self.grid_column_end),
            ("grid-row", &self.grid_row),
            ("grid-row-start", &self.grid_row_start),
            ("grid-row-end", &self.grid_row_end),
        ] {
            styles.set(property, value, Clone::clone)?;
        }
        styles.set("flex-grow", &self.flex_grow, ToString::to_string)?;
        styles.set("flex-shrink", &self.flex_shrink, ToString::to_string)?;
        styles.set("align-self", &self.align_self, |value| value.css().into())?;
        styles.set("justify-self", &self.justify_self, |value| {
            value.css().into()
        })?;
        Ok(())
    }
}

/// Native element for Box, Flex and Grid. Display is independent of the tag.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LayoutElement {
    #[default]
    Div,
    Span,
}
css_enum! { /// Box display mode.
pub enum BoxDisplay { None => "none", Inline => "inline", InlineBlock => "inline-block", Block => "block", Contents => "contents" } }
css_enum! { /// Flex display mode.
pub enum FlexDisplay { None => "none", InlineFlex => "inline-flex", Flex => "flex" } }
css_enum! { /// Grid display mode.
pub enum GridDisplay { None => "none", InlineGrid => "inline-grid", Grid => "grid" } }
css_enum! { /// Direction of the flex main axis.
pub enum FlexDirection { Row => "row", Column => "column", RowReverse => "row-reverse", ColumnReverse => "column-reverse" } }
css_enum! { /// Flex line wrapping.
pub enum FlexWrap { Nowrap => "nowrap", Wrap => "wrap", WrapReverse => "wrap-reverse" } }
css_enum! { /// Placement of automatically placed grid items.
pub enum GridFlow { Row => "row", Column => "column", Dense => "dense", RowDense => "row dense", ColumnDense => "column dense" } }
css_enum! { /// Visibility for Container and Section; Initial restores component display.
pub enum ContentDisplay { None => "none", Initial => "initial" } }
css_enum! { /// Horizontal alignment of the Container's content layer.
pub enum ContainerAlign { Left => "start", Center => "center", Right => "end" } }
css_enum! { /// Content width scale, independent of arbitrary width/height props.
pub enum ContainerSize { One => "448px", Two => "688px", Three => "880px", Four => "1136px" } }
css_enum! { /// Section vertical padding from the already resolved space snapshot.
pub enum SectionSize { One => "var(--rustify-space-5)", Two => "var(--rustify-space-7)", Three => "var(--rustify-space-9)", Four => "calc(var(--rustify-space-9) * 1.25)" } }

/// Equal grid tracks, or one validated CSS track list.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum GridTracks {
    Count(u8),
    Css(String),
}
impl From<u8> for GridTracks {
    fn from(value: u8) -> Self {
        Self::Count(value)
    }
}
impl From<&str> for GridTracks {
    fn from(value: &str) -> Self {
        Self::Css(value.into())
    }
}
impl From<String> for GridTracks {
    fn from(value: String) -> Self {
        Self::Css(value)
    }
}
impl GridTracks {
    fn css(&self, field: &'static str) -> Result<String, PropError> {
        match self {
            Self::Count(count) if (1..=9).contains(count) => {
                Ok(format!("repeat({count}, minmax(0, 1fr))"))
            }
            Self::Count(_) => Err(PropError {
                field,
                message: "expected an equal-track count from 1 through 9".into(),
            }),
            Self::Css(value) => Ok(value.clone()),
        }
    }
}

fn base_styles(layout: Signal<Layout>, margin: Signal<Margin>) -> Result<StyleSet, PropError> {
    let mut styles = StyleSet::default();
    layout.get().write(&mut styles)?;
    margin.get().write(&mut styles)?;
    Ok(styles)
}

fn layout_node(
    as_: LayoutElement,
    classes: impl Fn() -> String + Send + Sync + Copy + 'static,
    name: &'static str,
    test_id: String,
    children: Children,
) -> AnyView {
    match as_ {
        LayoutElement::Div => {
            view! { <div class=classes data-name=name data-testid=test_id>{children()}</div> }
                .into_any()
        }
        LayoutElement::Span => {
            view! { <span class=classes data-name=name data-testid=test_id>{children()}</span> }
                .into_any()
        }
    }
}

/// A block by default, with optional native span semantics.
#[component]
pub fn Box(
    #[prop(optional)] as_: LayoutElement,
    #[prop(optional, into)] display: Signal<Responsive<BoxDisplay>>,
    #[prop(optional, into)] layout: Signal<Layout>,
    #[prop(optional, into)] margin: Signal<Margin>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let geometry = crate::geometry::style_class(move || {
        let mut styles = base_styles(layout, margin)?;
        styles.set("display", &display.get(), |value| value.css().into())?;
        Ok(styles)
    });
    layout_node(
        as_,
        crate::macros::reactive_merge(format!("rustify-box rustify-layout {geometry}"), &class),
        "Box",
        test_id,
        children,
    )
}

/// A flex formatting context with sparse responsive layout properties.
#[component]
pub fn Flex(
    #[prop(optional)] as_: LayoutElement,
    #[prop(optional, into)] display: Signal<Responsive<FlexDisplay>>,
    #[prop(optional, into)] direction: Signal<Responsive<FlexDirection>>,
    #[prop(optional, into)] align: Signal<Responsive<Align>>,
    #[prop(optional, into)] justify: Signal<Responsive<Justify>>,
    #[prop(optional, into)] wrap: Signal<Responsive<FlexWrap>>,
    #[prop(optional, into)] gap: Signal<Gap>,
    #[prop(optional, into)] layout: Signal<Layout>,
    #[prop(optional, into)] margin: Signal<Margin>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let geometry = crate::geometry::style_class(move || {
        let mut styles = base_styles(layout, margin)?;
        styles.set("display", &display.get(), |value| value.css().into())?;
        styles.set("flex-direction", &direction.get(), |value| {
            value.css().into()
        })?;
        styles.set("align-items", &align.get(), |value| value.css().into())?;
        styles.set("justify-content", &justify.get(), |value| {
            value.css().into()
        })?;
        styles.set("flex-wrap", &wrap.get(), |value| value.css().into())?;
        gap.get().write(&mut styles)?;
        Ok(styles)
    });
    layout_node(
        as_,
        crate::macros::reactive_merge(format!("rustify-flex rustify-layout {geometry}"), &class),
        "Flex",
        test_id,
        children,
    )
}

/// A grid formatting context, one flexible column by default.
#[component]
pub fn Grid(
    #[prop(optional)] as_: LayoutElement,
    #[prop(optional, into)] display: Signal<Responsive<GridDisplay>>,
    #[prop(optional, into)] areas: Signal<Responsive<String>>,
    #[prop(optional, into)] columns: Signal<Responsive<GridTracks>>,
    #[prop(optional, into)] rows: Signal<Responsive<GridTracks>>,
    #[prop(optional, into)] flow: Signal<Responsive<GridFlow>>,
    #[prop(optional, into)] align: Signal<Responsive<Align>>,
    #[prop(optional, into)] justify: Signal<Responsive<Justify>>,
    #[prop(optional, into)] align_content: Signal<Responsive<AlignContent>>,
    #[prop(optional, into)] justify_items: Signal<Responsive<Align>>,
    #[prop(optional, into)] gap: Signal<Gap>,
    #[prop(optional, into)] layout: Signal<Layout>,
    #[prop(optional, into)] margin: Signal<Margin>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let geometry = crate::geometry::style_class(move || {
        let mut styles = base_styles(layout, margin)?;
        styles.set("display", &display.get(), |value| value.css().into())?;
        styles.set("grid-template-areas", &areas.get(), Clone::clone)?;
        for (field, property, value) in [
            ("columns", "grid-template-columns", columns.get()),
            ("rows", "grid-template-rows", rows.get()),
        ] {
            for (width, value) in value.iter() {
                insert(&mut styles, width, field, property, value.css(field)?)?;
            }
        }
        styles.set("grid-auto-flow", &flow.get(), |value| value.css().into())?;
        styles.set("align-items", &align.get(), |value| value.css().into())?;
        styles.set("justify-content", &justify.get(), |value| {
            value.css().into()
        })?;
        styles.set("align-content", &align_content.get(), |value| {
            value.css().into()
        })?;
        styles.set("justify-items", &justify_items.get(), |value| {
            value.css().into()
        })?;
        gap.get().write(&mut styles)?;
        Ok(styles)
    });
    layout_node(
        as_,
        crate::macros::reactive_merge(format!("rustify-grid rustify-layout {geometry}"), &class),
        "Grid",
        test_id,
        children,
    )
}

/// A two-layer content container. Width/height props describe the inner layer.
#[component]
pub fn Container(
    #[prop(default = Responsive::from(ContainerSize::Four).into(), into)] size: Signal<
        Responsive<ContainerSize>,
    >,
    #[prop(optional, into)] align: Signal<Responsive<ContainerAlign>>,
    #[prop(optional, into)] display: Signal<Responsive<ContentDisplay>>,
    #[prop(optional, into)] layout: Signal<Layout>,
    #[prop(optional, into)] margin: Signal<Margin>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let outer = crate::geometry::style_class(move || {
        let mut styles = StyleSet::default();
        layout.get().write_outer(&mut styles)?;
        margin.get().write(&mut styles)?;
        styles.set("align-items", &align.get(), |value| value.css().into())?;
        styles.set("display", &display.get(), |value| {
            if *value == ContentDisplay::Initial {
                "flex"
            } else {
                "none"
            }
            .into()
        })?;
        Ok(styles)
    });
    let inner = crate::geometry::style_class(move || {
        let mut styles = StyleSet::default();
        styles.set("--rustify-container-width", &size.get(), |value| {
            value.css().into()
        })?;
        layout.get().write_dimensions(&mut styles)?;
        Ok(styles)
    });
    view! {
        <div class=crate::macros::reactive_merge(format!("rustify-container rustify-layout {outer}"), &class) data-name="Container" data-testid=test_id>
            <div class=format!("rustify-container-inner {inner}")>{children()}</div>
        </div>
    }
}

/// A semantic section with theme-scaled vertical spacing.
#[component]
pub fn Section(
    #[prop(default = Responsive::from(SectionSize::Three).into(), into)] size: Signal<
        Responsive<SectionSize>,
    >,
    #[prop(optional, into)] display: Signal<Responsive<ContentDisplay>>,
    #[prop(optional, into)] layout: Signal<Layout>,
    #[prop(optional, into)] margin: Signal<Margin>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let geometry = crate::geometry::style_class(move || {
        let mut styles = StyleSet::default();
        styles.set("--rustify-section-padding", &size.get(), |value| {
            value.css().into()
        })?;
        layout.get().write(&mut styles)?;
        margin.get().write(&mut styles)?;
        styles.set("display", &display.get(), |value| {
            if *value == ContentDisplay::Initial {
                "block"
            } else {
                "none"
            }
            .into()
        })?;
        Ok(styles)
    });
    view! { <section class=crate::macros::reactive_merge(format!("rustify-section rustify-layout {geometry}"), &class) data-name="Section" data-testid=test_id>{children()}</section> }
}

/// Which edges of the nearest layout container the inset reaches.
///
/// A named side also reaches both adjacent edges, rounding only the two
/// outer corners on that side.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InsetSide {
    All,
    X,
    Y,
    Top,
    Bottom,
    Left,
    Right,
}
impl InsetSide {
    fn edges(self) -> [bool; 4] {
        match self {
            Self::All => [true; 4],
            Self::X => [false, true, false, true],
            Self::Y => [true, false, true, false],
            Self::Top => [true, true, false, true],
            Self::Bottom => [false, true, true, true],
            Self::Left => [true, false, true, true],
            Self::Right => [true, true, true, false],
        }
    }
}
/// Whether the inset stops at the container border or includes its width.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InsetClip {
    BorderBox,
    PaddingBox,
}
/// Inset padding can reuse its nearest container's padding or explicitly clear it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InsetPaddingValue {
    Current,
    Zero,
}
/// Restricted inset padding, with no arbitrary negative-margin escape hatch.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct InsetPadding {
    pub p: Responsive<InsetPaddingValue>,
    pub px: Responsive<InsetPaddingValue>,
    pub py: Responsive<InsetPaddingValue>,
    pub pt: Responsive<InsetPaddingValue>,
    pub pr: Responsive<InsetPaddingValue>,
    pub pb: Responsive<InsetPaddingValue>,
    pub pl: Responsive<InsetPaddingValue>,
}
impl InsetPadding {
    fn at(&self, width: u16, edge: usize) -> Option<InsetPaddingValue> {
        let axis = if edge == 0 || edge == 2 {
            &self.py
        } else {
            &self.px
        };
        let individual = [&self.pt, &self.pr, &self.pb, &self.pl][edge];
        individual
            .at(width)
            .or_else(|| axis.at(width))
            .or_else(|| self.p.at(width))
            .copied()
    }
}

/// Cancels only the padding of the nearest layout container.
#[component]
pub fn Inset(
    #[prop(default = Responsive::from(InsetSide::All).into(), into)] side: Signal<
        Responsive<InsetSide>,
    >,
    #[prop(default = Responsive::from(InsetClip::BorderBox).into(), into)] clip: Signal<
        Responsive<InsetClip>,
    >,
    #[prop(optional, into)] padding: Signal<InsetPadding>,
    #[prop(optional, into)] margin: Signal<Margin>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let geometry = crate::geometry::style_class(move || {
        let mut margins = StyleSet::default();
        margin.get().write(&mut margins)?;
        let side = side.get();
        let clip = clip.get();
        let padding = padding.get();
        let mut styles = StyleSet::default();
        for width in crate::responsive::BREAKPOINTS {
            let active = side.at(width).copied().unwrap_or(InsetSide::All).edges();
            let clip = clip.at(width).copied().unwrap_or(InsetClip::BorderBox);
            for edge in 0..4 {
                let margin = margins
                    .value_at(width, MARGIN_PROPERTIES[edge])
                    .unwrap_or("0px");
                let value = if active[edge] && margin != "auto" {
                    let border = if clip == InsetClip::PaddingBox {
                        " + var(--rustify-inset-border-width, 0px)"
                    } else {
                        ""
                    };
                    format!(
                        "calc({margin} - (var({}, 0px){border}))",
                        INSET_PROPERTIES[edge]
                    )
                } else {
                    margin.into()
                };
                styles.insert(width, MARGIN_PROPERTIES[edge], value)?;
                if let Some(value) = padding.at(width, edge) {
                    styles.insert(
                        width,
                        PADDING_PROPERTIES[edge],
                        match value {
                            InsetPaddingValue::Current => {
                                format!("var({}, 0px)", INSET_PROPERTIES[edge])
                            }
                            InsetPaddingValue::Zero => "0px".into(),
                        },
                    )?;
                }
            }
            let radius = if clip == InsetClip::PaddingBox {
                "var(--rustify-inset-radius, 0px)"
            } else {
                "max(0px, calc(var(--rustify-inset-radius, 0px) - var(--rustify-inset-border-width, 0px)))"
            };
            for (property, selected) in [
                ("border-top-left-radius", active[0] && active[3]),
                ("border-top-right-radius", active[0] && active[1]),
                ("border-bottom-right-radius", active[2] && active[1]),
                ("border-bottom-left-radius", active[2] && active[3]),
            ] {
                styles.insert(width, property, if selected { radius } else { "0px" })?;
            }
        }
        Ok(styles)
    });
    view! { <div class=crate::macros::reactive_merge(format!("rustify-inset {geometry}"), &class) data-name="Inset" data-testid=test_id>{children()}</div> }
}

/// Decorates exactly one native Leptos element, preserving its tag and events.
///
/// Pass the element itself, for example `Reset(leptos::html::button().on(leptos::ev::click, handler).child("Save"))`.
/// Tuples, fragments, component views and lists do not satisfy this signature.
///
/// ```
/// use leptos::prelude::*;
/// let _ = rustify_components::Reset(leptos::html::button().attr("type", "submit").child("Save"));
/// ```
///
/// ```compile_fail
/// use leptos::prelude::*;
/// rustify_components::Reset((leptos::html::div().child("one"), leptos::html::span().child("two")));
/// ```
#[allow(non_snake_case)]
pub fn Reset<E, At, Ch>(element: leptos::html::HtmlElement<E, At, Ch>) -> impl IntoView
where
    E: leptos::html::ElementType + Send,
    At: leptos::attr::Attribute + Send,
    Ch: RenderHtml + Send,
{
    element.class(("rustify-reset", true))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn responsive_shorthands_preserve_more_specific_edges() {
        let padding = Padding {
            p: Responsive {
                initial: Some(Spacing::Scale(4)),
                md: Some(Spacing::Scale(6)),
                ..Responsive::default()
            },
            px: Responsive::from(Spacing::Css("37px".into())),
            pt: Responsive::from(Spacing::Scale(1)),
            ..Padding::default()
        };
        let mut styles = StyleSet::default();
        padding.write(&mut styles).unwrap();
        assert_eq!(
            styles.value_at(1024, "padding-top"),
            Some("var(--rustify-space-1)")
        );
        assert_eq!(styles.value_at(1024, "padding-right"), Some("37px"));
        assert_eq!(
            styles.value_at(1024, "padding-bottom"),
            Some("var(--rustify-space-6)")
        );
        assert_eq!(styles.value_at(1024, "--rustify-inset-left"), Some("37px"));
    }

    #[test]
    fn negative_spacing_is_confined_to_margin_and_position_edges() {
        let mut styles = StyleSet::default();
        Margin {
            m: Spacing::Scale(-3).into(),
            ..Margin::default()
        }
        .write(&mut styles)
        .unwrap();
        assert_eq!(
            styles.value_at(0, "margin-top"),
            Some("calc(var(--rustify-space-3) * -1)")
        );
        assert_eq!(
            Padding {
                px: Spacing::Scale(-1).into(),
                ..Padding::default()
            }
            .write(&mut StyleSet::default())
            .unwrap_err()
            .field,
            "px"
        );
        assert_eq!(
            Gap {
                gap_y: Spacing::Scale(10).into(),
                ..Gap::default()
            }
            .write(&mut StyleSet::default())
            .unwrap_err()
            .field,
            "gap_y"
        );
    }

    #[test]
    fn every_margin_and_padding_field_has_the_documented_edge_mapping() {
        type MarginCase = (&'static str, fn(&mut Margin), [bool; 4]);
        let margin_cases: [MarginCase; 7] = [
            (
                "m",
                |value| value.m = Spacing::Scale(3).into(),
                [true, true, true, true],
            ),
            (
                "mx",
                |value| value.mx = Spacing::Scale(3).into(),
                [false, true, false, true],
            ),
            (
                "my",
                |value| value.my = Spacing::Scale(3).into(),
                [true, false, true, false],
            ),
            (
                "mt",
                |value| value.mt = Spacing::Scale(3).into(),
                [true, false, false, false],
            ),
            (
                "mr",
                |value| value.mr = Spacing::Scale(3).into(),
                [false, true, false, false],
            ),
            (
                "mb",
                |value| value.mb = Spacing::Scale(3).into(),
                [false, false, true, false],
            ),
            (
                "ml",
                |value| value.ml = Spacing::Scale(3).into(),
                [false, false, false, true],
            ),
        ];
        for (field, assign, expected) in margin_cases {
            let mut value = Margin::default();
            assign(&mut value);
            let mut styles = StyleSet::default();
            value.write(&mut styles).unwrap();
            for (edge, property) in MARGIN_PROPERTIES.iter().enumerate() {
                assert_eq!(
                    styles.value_at(0, property),
                    expected[edge].then_some("var(--rustify-space-3)"),
                    "{field}: {property}"
                );
            }
        }
        type PaddingCase = (&'static str, fn(&mut Padding), [bool; 4]);
        let padding_cases: [PaddingCase; 7] = [
            (
                "p",
                |value| value.p = Spacing::Scale(3).into(),
                [true, true, true, true],
            ),
            (
                "px",
                |value| value.px = Spacing::Scale(3).into(),
                [false, true, false, true],
            ),
            (
                "py",
                |value| value.py = Spacing::Scale(3).into(),
                [true, false, true, false],
            ),
            (
                "pt",
                |value| value.pt = Spacing::Scale(3).into(),
                [true, false, false, false],
            ),
            (
                "pr",
                |value| value.pr = Spacing::Scale(3).into(),
                [false, true, false, false],
            ),
            (
                "pb",
                |value| value.pb = Spacing::Scale(3).into(),
                [false, false, true, false],
            ),
            (
                "pl",
                |value| value.pl = Spacing::Scale(3).into(),
                [false, false, false, true],
            ),
        ];
        for (field, assign, expected) in padding_cases {
            let mut value = Padding::default();
            assign(&mut value);
            let mut styles = StyleSet::default();
            value.write(&mut styles).unwrap();
            for (edge, property) in PADDING_PROPERTIES.iter().enumerate() {
                assert_eq!(
                    styles.value_at(0, property),
                    expected[edge].then_some("var(--rustify-space-3)"),
                    "{field}: {property}"
                );
            }
        }
    }

    #[test]
    fn every_shared_layout_field_emits_its_corresponding_property() {
        let layout = Layout {
            padding: Padding {
                p: Spacing::Scale(2).into(),
                ..Padding::default()
            },
            width: "37px".to_string().into(),
            min_width: "10px".to_string().into(),
            max_width: "100%".to_string().into(),
            height: "48px".to_string().into(),
            min_height: "20px".to_string().into(),
            max_height: "100vh".to_string().into(),
            position: Position::Relative.into(),
            inset: Spacing::Scale(2).into(),
            top: Spacing::Scale(-1).into(),
            right: Spacing::Css("3px".into()).into(),
            bottom: Spacing::Scale(4).into(),
            left: Spacing::Scale(5).into(),
            overflow: Overflow::Auto.into(),
            overflow_x: Overflow::Hidden.into(),
            overflow_y: Overflow::Scroll.into(),
            flex_basis: "40%".to_string().into(),
            flex_grow: 2.5.into(),
            flex_shrink: 0.5.into(),
            grid_area: "main".to_string().into(),
            grid_column: "1 / 3".to_string().into(),
            grid_column_start: "1".to_string().into(),
            grid_column_end: "3".to_string().into(),
            grid_row: "2 / 4".to_string().into(),
            grid_row_start: "2".to_string().into(),
            grid_row_end: "4".to_string().into(),
            align_self: Align::Center.into(),
            justify_self: Align::End.into(),
        };
        let mut styles = StyleSet::default();
        layout.write(&mut styles).unwrap();
        for property in [
            "padding-top",
            "padding-right",
            "padding-bottom",
            "padding-left",
            "width",
            "min-width",
            "max-width",
            "height",
            "min-height",
            "max-height",
            "position",
            "top",
            "right",
            "bottom",
            "left",
            "overflow",
            "overflow-x",
            "overflow-y",
            "flex-basis",
            "flex-grow",
            "flex-shrink",
            "grid-area",
            "grid-column",
            "grid-column-start",
            "grid-column-end",
            "grid-row",
            "grid-row-start",
            "grid-row-end",
            "align-self",
            "justify-self",
        ] {
            assert!(styles.value_at(0, property).is_some(), "{property}");
        }
    }

    #[test]
    fn empty_groups_do_not_override_defaults_and_css_lengths_are_not_scaled() {
        let mut styles = StyleSet::default();
        Layout::default().write(&mut styles).unwrap();
        Margin::default().write(&mut styles).unwrap();
        Gap::default().write(&mut styles).unwrap();
        assert_eq!(styles, StyleSet::default());
        Layout {
            width: "37px".to_string().into(),
            ..Layout::default()
        }
        .write(&mut styles)
        .unwrap();
        assert_eq!(styles.value_at(0, "width"), Some("37px"));
    }

    #[cfg(all(feature = "ssr", not(target_arch = "wasm32")))]
    fn render(view: impl FnOnce() -> AnyView) -> (String, String) {
        let styles = crate::GeometryStyles::default();
        let owner = Owner::new();
        let html = owner.with(|| {
            rustify_ui::id::provide("layout-test");
            crate::provide_geometry_styles(styles.clone());
            view().to_html()
        });
        drop(owner);
        (html, styles.css())
    }

    #[cfg(all(feature = "ssr", not(target_arch = "wasm32")))]
    #[test]
    fn ssr_preserves_tags_and_projects_container_dimensions_to_inner_layer() {
        let (html, css) = render(|| {
            view! {
            <Box as_=LayoutElement::Span>"inline semantics"</Box>
            <Container layout=Layout { width: "37px".to_string().into(), height: "48px".to_string().into(), ..Layout::default() }>"content"</Container>
            <Section>"section"</Section>
        }.into_any()
        });
        assert!(html.contains("<span"), "{html}");
        assert!(html.contains("<section"), "{html}");
        assert!(
            html.contains("rustify-container-inner rui-layout-test-geometry-3"),
            "{html}"
        );
        assert!(css.contains(":where(.rui-layout-test-geometry-3){--rustify-container-width:1136px;height:48px;width:37px;}"), "{css}");
        assert!(!css.contains("style="));
        assert!(!html.contains("style="));
    }

    #[cfg(all(feature = "ssr", not(target_arch = "wasm32")))]
    #[test]
    fn reset_accepts_one_native_element_without_wrapping_or_dropping_its_attributes() {
        let (html, _) = render(|| {
            Reset(
                leptos::html::button()
                    .attr("type", "submit")
                    .attr("aria-label", "Save")
                    .class("original")
                    .child("Save"),
            )
            .into_any()
        });
        assert!(html.starts_with("<button"), "{html}");
        assert!(html.contains("type=\"submit\""), "{html}");
        assert!(html.contains("aria-label=\"Save\""), "{html}");
        assert!(html.contains("original rustify-reset"), "{html}");
        assert!(!html.contains("<div"));
    }

    #[cfg(all(feature = "ssr", not(target_arch = "wasm32")))]
    #[test]
    fn inset_sides_preserve_unselected_margins_and_select_only_joined_corners() {
        // Themes inset.css defines edges clockwise from top, corners from top-left.
        for (side, edges, corners) in [
            (InsetSide::All, [true; 4], [true; 4]),
            (InsetSide::X, [false, true, false, true], [false; 4]),
            (InsetSide::Y, [true, false, true, false], [false; 4]),
            (
                InsetSide::Top,
                [true, true, false, true],
                [true, true, false, false],
            ),
            (
                InsetSide::Bottom,
                [false, true, true, true],
                [false, false, true, true],
            ),
            (
                InsetSide::Left,
                [true, false, true, true],
                [true, false, false, true],
            ),
            (
                InsetSide::Right,
                [true, true, true, false],
                [false, true, true, false],
            ),
        ] {
            for clip in [InsetClip::BorderBox, InsetClip::PaddingBox] {
                let (_, css) = render(|| {
                    view! {
                    <Inset side=Responsive::from(side) clip=Responsive::from(clip)
                        margin=Margin { m: Spacing::Css("3px".into()).into(), ..Margin::default() }>
                        "inset"
                    </Inset>
                }.into_any()
                });
                for edge in 0..4 {
                    let value = if edges[edge] {
                        let border = if clip == InsetClip::PaddingBox {
                            " + var(--rustify-inset-border-width, 0px)"
                        } else {
                            ""
                        };
                        format!("calc(3px - (var({}, 0px){border}))", INSET_PROPERTIES[edge])
                    } else {
                        "3px".into()
                    };
                    assert!(
                        css.contains(&format!("{}:{value};", MARGIN_PROPERTIES[edge])),
                        "{side:?}/{clip:?}: {css}"
                    );
                }
                for (corner, selected) in [
                    ("top-left", corners[0]),
                    ("top-right", corners[1]),
                    ("bottom-right", corners[2]),
                    ("bottom-left", corners[3]),
                ] {
                    let radius = if !selected {
                        "0px"
                    } else if clip == InsetClip::PaddingBox {
                        "var(--rustify-inset-radius, 0px)"
                    } else {
                        "max(0px, calc(var(--rustify-inset-radius, 0px) - var(--rustify-inset-border-width, 0px)))"
                    };
                    assert!(
                        css.contains(&format!("border-{corner}-radius:{radius};")),
                        "{side:?}/{clip:?}: {css}"
                    );
                }
            }
        }
    }

    #[cfg(all(feature = "ssr", not(target_arch = "wasm32")))]
    #[test]
    fn rendering_with_the_same_seed_produces_identical_responsive_css() {
        let render_view = || {
            view! {
            <Flex direction=Responsive { sm: Some(FlexDirection::Column), lg: Some(FlexDirection::Row), ..Responsive::default() }>
                <Grid columns=Responsive::from(GridTracks::Count(3))>"grid"</Grid>
                <Inset side=Responsive::from(InsetSide::X)>"inset"</Inset>
            </Flex>
        }.into_any()
        };
        assert_eq!(render(render_view), render(render_view));
    }
}
