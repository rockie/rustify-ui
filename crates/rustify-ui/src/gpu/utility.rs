//! SDK-owned utility preview widgets consume precompiled complete snapshots.
pub(crate) mod model;
pub use super::button::utility::ButtonUtilityProps;
use super::{rgba_color, DrawRustifyShadow, RustifyButton};
use crate::{makepad_widgets::widget::*, makepad_widgets::*, utility::gpu::*};
pub(crate) use model::text_available_width;
use model::{arrange, box_minimum, constrained, content_size, NodeModel};
pub use model::{
    UtilityNodeKind, UtilityNodeProps, UtilityNodeReport, UtilityPreviewProps,
    UtilityPreviewReport, UtilityTextRow,
};

script_mod! {
    use mod.prelude.widgets_internal.*
    mod.widgets.DrawRustifyUtilityBox = mod.draw.DrawColor{
        border_color: uniform(#x858f9e)
        border_width: uniform(0.0)
        radius: uniform(4.0)
        opacity: uniform(1.0)
        box_distance: fn(point: vec2, origin: vec2, size: vec2, corner: float) {
            let half = size * 0.5
            let r = clamp(corner, 0.0, min(half.x, half.y))
            let q = abs(point - origin - half) - half + vec2(r)
            return length(max(q, vec2(0.0))) + min(max(q.x,q.y),0.0) - r
        }
        pixel: fn() {
            let point = self.pos * self.rect_size
            let outer = 1.0 - smoothstep(-0.5,0.5,self.box_distance(point,vec2(0.0),self.rect_size,self.radius))
            let bw = clamp(self.border_width,0.0,min(self.rect_size.x,self.rect_size.y)*0.5)
            let mut inner = outer
            if bw > 0.0 {
                let inside = max(self.rect_size - vec2(bw*2.0),vec2(0.0))
                inner = 0.0
                if min(inside.x,inside.y) > 0.0 {
                    inner = 1.0 - smoothstep(-0.5,0.5,self.box_distance(point,vec2(bw),inside,max(self.radius-bw,0.0)))
                }
            }
            let border_a = self.border_color.a * max(outer-inner,0.0)
            let fill_a = self.color.a * outer
            let a = border_a + fill_a * (1.0-border_a)
            let rgb = self.border_color.rgb * border_a + self.color.rgb * fill_a * (1.0-border_a)
            return vec4(rgb,a) * self.opacity
        }
    }
    mod.widgets.RustifyButton.draw_utility_body = mod.widgets.DrawRustifyUtilityBox{}
    mod.widgets.RustifyUtilityLeafBase = #(RustifyUtilityLeaf::register_widget(vm))
    mod.widgets.RustifyUtilityLeaf = set_type_default() do mod.widgets.RustifyUtilityLeafBase{
        width: Fit height: Fit
        draw_bg: mod.widgets.DrawRustifyUtilityBox{color: #xffffff}
        draw_shadow: mod.widgets.DrawRustifyShadow{}
        draw_text +: {color: #x1d2939 text_style: theme.font_regular_i18n{font_size: 11.25}}
    }
    mod.widgets.RustifyUtilityPreviewBase = #(RustifyUtilityPreview::register_widget(vm))
    mod.widgets.RustifyUtilityPreview = set_type_default() do mod.widgets.RustifyUtilityPreviewBase{
        width: Fill height: 180
        draw_bg: mod.widgets.DrawRustifyUtilityBox{color: #xffffff}
        draw_shadow: mod.widgets.DrawRustifyShadow{}
        button: mod.widgets.RustifyButton{}
        text: mod.widgets.RustifyUtilityLeaf{}
        swatch: mod.widgets.RustifyUtilityLeaf{}
    }
}
#[derive(Script, ScriptHook, Widget)]
pub struct RustifyUtilityLeaf {
    #[uid]
    uid: WidgetUid,
    #[source]
    source: ScriptObjectRef,
    #[walk]
    walk: Walk,
    #[layout]
    layout: Layout,
    #[redraw]
    #[live]
    draw_bg: DrawColor,
    #[live]
    draw_shadow: DrawRustifyShadow,
    #[live]
    draw_text: DrawText,
    #[rust]
    model: NodeModel,
    #[rust]
    click: bool,
    #[rust]
    pressed_key: bool,
    #[rust]
    pointer_pressed: bool,
    #[rust]
    font_binding: Option<(String, Option<String>)>,
}
#[derive(Script, ScriptHook, Widget)]
pub struct RustifyUtilityPreview {
    #[uid]
    uid: WidgetUid,
    #[source]
    source: ScriptObjectRef,
    #[walk]
    walk: Walk,
    #[layout]
    layout: Layout,
    #[redraw]
    #[live]
    draw_bg: DrawColor,
    #[live]
    draw_shadow: DrawRustifyShadow,
    #[live]
    button: RustifyButton,
    #[live]
    text: RustifyUtilityLeaf,
    #[live]
    swatch: RustifyUtilityLeaf,
    #[rust]
    model: NodeModel,
}
impl RustifyUtilityLeaf {
    pub fn apply_props(&mut self, cx: &mut Cx, props: &UtilityNodeProps) {
        if props.context.state.disabled {
            self.pressed_key = false;
            self.pointer_pressed = false;
        }
        if self.model.apply(props.clone()) {
            self.redraw(cx);
        }
    }
    pub fn report(&self) -> UtilityNodeReport {
        self.model.report()
    }
    pub fn take_click(&mut self) -> bool {
        std::mem::take(&mut self.click)
    }
    pub fn set_control_state(&mut self, cx: &mut Cx, state: GpuControlState) {
        if state.disabled {
            self.pressed_key = false;
            self.pointer_pressed = false;
        }
        if self.model.update_state(state) {
            self.redraw(cx);
        }
    }
    pub fn update_viewport(&mut self, cx: &mut Cx, viewport: [f64; 2]) {
        if self.model.update_viewport(viewport) {
            self.redraw(cx);
        }
    }
}
impl RustifyUtilityPreview {
    pub fn apply_props(&mut self, cx: &mut Cx, props: &UtilityPreviewProps) {
        let mut container = props.container.clone();
        container.kind = UtilityNodeKind::Container;
        if self.model.apply(container) {
            self.redraw(cx);
        }
        self.button.apply_theme(cx, &props.button.theme);
        self.button
            .set_state(cx, &props.button.label, props.button.context.state.disabled);
        self.button.apply_utility_programs(
            cx,
            &ButtonUtilityProps {
                revision: props.button.revision,
                programs: props.button.programs.clone(),
                theme: props.button.theme.clone(),
                context: props.button.context.clone(),
            },
        );
        let mut text = props.text.clone();
        text.kind = UtilityNodeKind::Text;
        self.text.apply_props(cx, &text);
        let mut swatch = props.swatch.clone();
        swatch.kind = UtilityNodeKind::ColorBlock;
        self.swatch.apply_props(cx, &swatch);
    }
    pub fn report(&self) -> UtilityPreviewReport {
        UtilityPreviewReport {
            container: self.model.report(),
            button: self
                .button
                .utility_report()
                .unwrap_or_else(|| NodeModel::default().report()),
            text: self.text.report(),
            swatch: self.swatch.report(),
        }
    }
    pub fn take_click(&mut self) -> bool {
        self.button.take_click().is_some()
    }
    pub fn set_button_state(&mut self, cx: &mut Cx, state: GpuControlState) {
        self.button.set_utility_state(cx, state);
    }
    pub fn set_node_state(&mut self, cx: &mut Cx, kind: UtilityNodeKind, state: GpuControlState) {
        match kind {
            UtilityNodeKind::Container => {
                if self.model.update_state(state) {
                    self.redraw(cx);
                }
            }
            UtilityNodeKind::Button => self.button.set_utility_state(cx, state),
            UtilityNodeKind::Text => self.text.set_control_state(cx, state),
            UtilityNodeKind::ColorBlock => self.swatch.set_control_state(cx, state),
        }
    }
    pub fn update_viewport(&mut self, cx: &mut Cx, viewport: [f64; 2]) {
        if self.model.update_viewport(viewport) {
            self.redraw(cx);
        }
        self.button.update_utility_viewport(cx, viewport);
        self.text.update_viewport(cx, viewport);
        self.swatch.update_viewport(cx, viewport);
    }
}
pub(crate) fn body(
    draw: &mut DrawColor,
    shadow: &mut DrawRustifyShadow,
    cx: &mut Cx2d,
    pane: Rect,
    style: &GpuComputedStyle,
) {
    shadow.draw_layers(cx, pane, style.radii[0], &style.shadows);
    draw.color = rgba_color(style.background);
    let border = rgba_color(style.border_colors[0]);
    draw.set_uniform(
        cx,
        id!(border_color),
        &[border.x, border.y, border.z, border.w],
    );
    draw.set_uniform(cx, id!(border_width), &[style.border_widths[0] as f32]);
    draw.set_uniform(cx, id!(radius), &[style.radii[0] as f32]);
    draw.set_uniform(cx, id!(opacity), &[style.opacity as f32]);
    draw.draw_abs(cx, pane);
}
fn box_walk(style: &GpuComputedStyle, walk: Walk, intrinsic: [f64; 2]) -> Walk {
    let minimum = box_minimum(style);
    Walk {
        width: Size::Fixed(constrained(
            style.width,
            GpuLength::Pixels(minimum[0]),
            style.max_width,
            intrinsic[0],
        )),
        height: Size::Fixed(constrained(
            style.height,
            GpuLength::Pixels(minimum[1]),
            style.max_height,
            intrinsic[1],
        )),
        ..walk
    }
}
pub(crate) fn dimension_walk(
    value: GpuLength,
    min: GpuLength,
    max: GpuLength,
    base: Size,
    intrinsic: f64,
) -> Size {
    if let GpuLength::Pixels(_) = value {
        return Size::Fixed(constrained(value, min, max, intrinsic));
    }
    let minimum = if let GpuLength::Pixels(px) = min {
        Some(px)
    } else {
        None
    };
    let maximum = if let GpuLength::Pixels(px) = max {
        Some(px.max(minimum.unwrap_or(0.)))
    } else {
        None
    };
    match base {
        Size::Fixed(px) => Size::Fixed(constrained(value, min, max, px)),
        Size::Fill { weight, .. } => Size::Fill {
            weight,
            min: minimum,
            max: maximum,
        },
        Size::Fit { .. } => Size::Fit {
            min: minimum.map(FitBound::Abs),
            max: maximum.map(FitBound::Abs),
        },
    }
}
pub(crate) fn parent_content_size(cx: &Cx2d) -> [Option<f64>; 2] {
    let size = cx.turtle().inner_size();
    [size.x, size.y].map(|value| (value.is_finite() && value >= 0.).then_some(value))
}
fn definite_walk_size(cx: &Cx2d, walk: Walk) -> [Option<f64>; 2] {
    let rect = cx.peek_walk_turtle(walk);
    let parent = parent_content_size(cx);
    let sizes = [walk.width, walk.height];
    std::array::from_fn(|axis| {
        let size = sizes[axis];
        match size {
            Size::Fixed(value) => Some(value),
            Size::Fill { .. } if parent[axis].is_some() => Some([rect.size.x, rect.size.y][axis]),
            _ => None,
        }
    })
}
/// A label laid out the way CSS lays out a box's anonymous text: on one line
/// while it fits the available content width, otherwise wrapped between words.
pub(crate) struct UtilityText {
    pub laidout: crate::makepad_widgets::makepad_draw::text::layouter::LaidoutText,
    /// The item box CSS gives that text. A wrapped label shrinks to the
    /// available width, not to its longest line.
    pub size: [f64; 2],
    line_height: Option<f64>,
}
impl UtilityText {
    /// Each painted line, offset by the text box origin.
    pub fn rows(&self, origin: [f64; 2]) -> Vec<UtilityTextRow> {
        self.laidout
            .rows
            .iter()
            .enumerate()
            .map(|(index, row)| {
                let visible: Vec<_> = row
                    .glyphs
                    .iter()
                    .filter(|glyph| {
                        row.text[glyph.cluster.min(row.text.len())..]
                            .chars()
                            .next()
                            .is_some_and(|char| !char.is_whitespace())
                    })
                    .collect();
                let start = visible.first().map_or(0., |glyph| glyph.origin_in_lpxs.x);
                let end = visible
                    .last()
                    .map_or(0., |glyph| glyph.origin_in_lpxs.x + glyph.advance_in_lpxs());
                let glyph_height = row.ascender_in_lpxs - row.descender_in_lpxs;
                let (top, height) = match self.line_height {
                    Some(leading) => (index as f64 * leading, leading),
                    None => (
                        (row.origin_in_lpxs.y - row.ascender_in_lpxs) as f64,
                        glyph_height as f64,
                    ),
                };
                UtilityTextRow {
                    text: row.text.trim().to_string(),
                    x: origin[0] + (row.origin_in_lpxs.x + start) as f64,
                    width: (end - start).max(0.) as f64,
                    top: origin[1] + top,
                    height,
                }
            })
            .collect()
    }
}
pub(crate) fn utility_text_layout(
    draw: &mut DrawText,
    cx: &mut Cx,
    style: &GpuComputedStyle,
    label: &str,
    font_binding: &mut Option<(String, Option<String>)>,
    available_width: Option<f64>,
) -> UtilityText {
    let binding = (style.font.family.clone(), style.font.resource_path.clone());
    if font_binding.as_ref() != Some(&binding) {
        if let Some(family) = crate::theme::gpu_font_family(cx, &style.font) {
            draw.text_style.font_family = family;
            *font_binding = Some(binding);
        }
    }
    draw.text_style.font_size = style.font_size_points;
    draw.text_style.letter_spacing = style.letter_spacing_em;
    draw.text_style.line_spacing = 1.;
    draw.ink_centered = false;
    draw.color = rgba_color(style.color);
    draw.color.w *= style.opacity as f32;
    let mut laidout = (*draw.layout(cx, 0., 0., None, false, Align::default(), label)).clone();
    let mut width = laidout.size_in_lpxs.width as f64;
    // Rounding noise must not wrap a label that measures exactly its box.
    if let Some(available) = available_width.filter(|available| width > available + 0.01) {
        laidout = (*draw.layout(
            cx,
            0.,
            0.,
            Some(available as f32),
            true,
            Align::default(),
            label,
        ))
        .clone();
        width = available.max(laidout.size_in_lpxs.width as f64);
    }
    let line_height = match style.line_height {
        GpuLineHeight::Pixels(leading) => Some(leading),
        GpuLineHeight::Normal => None,
    };
    if let Some(leading) = line_height {
        for (index, row) in laidout.rows.iter_mut().enumerate() {
            let glyph_height = row.ascender_in_lpxs - row.descender_in_lpxs;
            row.origin_in_lpxs.y = index as f32 * leading as f32
                + (leading as f32 - glyph_height) * 0.5
                + row.ascender_in_lpxs;
        }
        laidout.size_in_lpxs.height = laidout.rows.len() as f32 * leading as f32;
    }
    UtilityText {
        size: [width, laidout.size_in_lpxs.height as f64],
        laidout,
        line_height,
    }
}
impl RustifyUtilityLeaf {
    fn text_layout(&mut self, cx: &mut Cx, available_width: Option<f64>) -> Option<UtilityText> {
        let style = self.model.style()?;
        let props = self.model.props()?;
        if props.kind == UtilityNodeKind::ColorBlock {
            return None;
        }
        Some(utility_text_layout(
            &mut self.draw_text,
            cx,
            style,
            &props.label,
            &mut self.font_binding,
            available_width,
        ))
    }
    fn intrinsic(&mut self, cx: &mut Cx) -> [f64; 2] {
        let available = self.model.style().and_then(|style| {
            let parent = self
                .model
                .layout_context()
                .and_then(|context| context.parent_size[0]);
            text_available_width(style, parent)
        });
        let measured = self.text_layout(cx, available).map(|layout| layout.size);
        let Some(style) = self.model.style() else {
            return [0.; 2];
        };
        let content = measured.unwrap_or([64., 40.]);
        [
            content[0]
                + style.padding[1]
                + style.padding[3]
                + style.border_widths[1]
                + style.border_widths[3],
            content[1]
                + style.padding[0]
                + style.padding[2]
                + style.border_widths[0]
                + style.border_widths[2],
        ]
    }
    fn natural_size(&mut self, cx: &mut Cx) -> [f64; 2] {
        let intrinsic = self.intrinsic(cx);
        let Some(style) = self.model.style() else {
            return intrinsic;
        };
        let minimum = box_minimum(style);
        [
            constrained(
                style.width,
                GpuLength::Pixels(minimum[0]),
                style.max_width,
                intrinsic[0],
            ),
            constrained(
                style.height,
                GpuLength::Pixels(minimum[1]),
                style.max_height,
                intrinsic[1],
            ),
        ]
    }
    fn paint(&mut self, cx: &mut Cx2d, mut pane: Rect) {
        let parent = self
            .model
            .layout_context()
            .map_or([None; 2], |context| context.parent_size);
        self.model
            .update_layout(parent, [Some(pane.size.x), Some(pane.size.y)]);
        let Some(style) = self.model.style().cloned() else {
            return;
        };
        let minimum = box_minimum(&style);
        pane.size.x = pane.size.x.max(minimum[0]);
        pane.size.y = pane.size.y.max(minimum[1]);
        body(&mut self.draw_bg, &mut self.draw_shadow, cx, pane, &style);
        let mut ready = true;
        let content_width = text_available_width(
            &GpuComputedStyle {
                width: GpuLength::Pixels(pane.size.x),
                ..style.clone()
            },
            None,
        );
        if let Some(text) = self.text_layout(cx, content_width) {
            let size = text.size;
            let inner = [
                (pane.size.x
                    - style.padding[1]
                    - style.padding[3]
                    - style.border_widths[1]
                    - style.border_widths[3])
                    .max(0.),
                (pane.size.y
                    - style.padding[0]
                    - style.padding[2]
                    - style.border_widths[0]
                    - style.border_widths[2])
                    .max(0.),
            ];
            let main = if style.flow == GpuFlow::Row { 0 } else { 1 };
            let cross = 1 - main;
            let mut offset = [0.; 2];
            offset[main] = match style.justify {
                GpuJustification::Start | GpuJustification::SpaceBetween => 0.,
                GpuJustification::End => inner[main] - size[main],
                _ => (inner[main] - size[main]) / 2.,
            };
            offset[cross] = match style.items {
                GpuAlignment::Start | GpuAlignment::Stretch => 0.,
                GpuAlignment::Center => (inner[cross] - size[cross]) / 2.,
                GpuAlignment::End => inner[cross] - size[cross],
            };
            let origin = [
                pane.pos.x + style.padding[3] + style.border_widths[3] + offset[0],
                pane.pos.y + style.padding[0] + style.border_widths[0] + offset[1],
            ];
            if !text.laidout.rows.is_empty() {
                self.draw_text.draw_walk_laidout(
                    cx,
                    Walk {
                        abs_pos: Some(dvec2(origin[0], origin[1])),
                        width: Size::Fixed(size[0]),
                        height: Size::Fixed(size[1]),
                        ..Walk::default()
                    },
                    &text.laidout,
                );
            }
            self.model.drawn_text(text.rows(origin));
            use crate::makepad_widgets::makepad_draw::text::fonts::Fonts;
            use std::{cell::RefCell, rc::Rc};
            ready = self.font_binding.as_ref().is_some_and(|binding| {
                binding.0 == style.font.family && binding.1 == style.font.resource_path
            }) && cx
                .get_global::<Rc<RefCell<Fonts>>>()
                .borrow()
                .is_font_family_complete(self.draw_text.text_style.font_family_id());
            if !ready {
                self.redraw(cx);
            }
        }
        self.model
            .drawn([pane.pos.x, pane.pos.y, pane.size.x, pane.size.y], ready);
    }
}
impl Widget for RustifyUtilityLeaf {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        if self
            .model
            .props()
            .is_none_or(|props| props.kind != UtilityNodeKind::Button)
        {
            return;
        }
        let mut state = self.model.state();
        let previous = state;
        if let Event::KeyDown(key) = event {
            state.modality = InputModality::Keyboard;
            if key.key_code == KeyCode::Tab && !state.disabled {
                if state.focused {
                    cx.set_key_focus(Area::Empty);
                    state.focused = false;
                    state.focus_visible = false;
                } else if !key.modifiers.shift {
                    cx.set_key_focus(self.draw_bg.area());
                    state.focused = true;
                    state.focus_visible = true;
                }
            } else if state.focused {
                state.focus_visible = true;
            }
        }
        let hit = event.hits(cx, self.draw_bg.area());
        match hit {
            Hit::FingerDown(fe) if fe.is_primary_hit() && !state.disabled => {
                state.modality = InputModality::Pointer;
                state.active = true;
                self.pointer_pressed = true;
                state.focus_visible = false;
                state.focused = true;
                cx.set_key_focus(self.draw_bg.area());
            }
            Hit::FingerUp(fe) => {
                let was = self.pointer_pressed;
                self.pointer_pressed = false;
                state.active = false;
                if was && fe.is_over && fe.is_primary_hit() && !state.disabled {
                    self.click = true;
                }
            }
            Hit::FingerMove(fe) => {
                if fe.has_hovers() {
                    state.hover = fe.is_over;
                }
                if self.pointer_pressed {
                    state.active = fe.is_over;
                }
            }
            Hit::FingerHoverIn(_) => state.hover = true,
            Hit::FingerHoverOut(_) => state.hover = false,
            Hit::KeyFocus(_) => {
                state.focused = !state.disabled;
                state.focus_visible = state.focused && state.modality == InputModality::Keyboard;
            }
            Hit::KeyFocusLost(_) => {
                state.focused = false;
                state.focus_visible = false;
                state.active = false;
                self.pressed_key = false;
            }
            Hit::KeyDown(key)
                if !state.disabled
                    && matches!(key.key_code, KeyCode::Space | KeyCode::ReturnKey) =>
            {
                if !self.pressed_key {
                    self.pressed_key = true;
                    state.active = true;
                }
            }
            Hit::KeyUp(key) if matches!(key.key_code, KeyCode::Space | KeyCode::ReturnKey) => {
                if self.pressed_key && !state.disabled {
                    self.click = true;
                }
                self.pressed_key = false;
                state.active = false;
            }
            _ => {}
        }
        if state != previous {
            self.set_control_state(cx, state);
        }
    }
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let control = self
            .model
            .layout_context()
            .map_or([None; 2], |context| context.control_size);
        self.model.update_layout(parent_content_size(cx), control);
        let intrinsic = self.intrinsic(cx);
        let Some(style) = self.model.style() else {
            return DrawStep::done();
        };
        let pane = cx.walk_turtle(box_walk(style, walk, intrinsic));
        self.paint(cx, pane);
        DrawStep::done()
    }
}
impl Widget for RustifyUtilityPreview {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.button.handle_event(cx, event, scope);
    }
    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let parent = parent_content_size(cx);
        let control = self
            .model
            .layout_context()
            .map_or([None; 2], |context| context.control_size);
        self.model.update_layout(parent, control);
        let Some(style) = self.model.style().cloned() else {
            return DrawStep::done();
        };
        let measured_walk = Walk {
            width: dimension_walk(
                style.width,
                style.min_width,
                style.max_width,
                walk.width,
                0.,
            ),
            height: dimension_walk(
                style.height,
                style.min_height,
                style.max_height,
                walk.height,
                0.,
            ),
            ..walk
        };
        let border_box = definite_walk_size(cx, measured_walk);
        self.model.update_layout(parent, border_box);
        let style = self.model.style().unwrap();
        let child_parent = content_size(style, border_box);
        self.button.update_utility_parent(child_parent);
        for leaf in [&mut self.text, &mut self.swatch] {
            let control = leaf
                .model
                .layout_context()
                .map_or([None; 2], |context| context.control_size);
            leaf.model.update_layout(child_parent, control);
        }
        let sizes = [
            self.button.utility_natural_size(cx),
            self.text.natural_size(cx),
            self.swatch.natural_size(cx),
        ];
        let Some(style) = self.model.style().cloned() else {
            return DrawStep::done();
        };
        let intrinsic = if style.flow == GpuFlow::Row {
            [
                sizes.iter().map(|size| size[0]).sum::<f64>() + style.gap[1] * 2.,
                sizes.iter().map(|size| size[1]).fold(0., f64::max),
            ]
        } else {
            [
                sizes.iter().map(|size| size[0]).fold(0., f64::max),
                sizes.iter().map(|size| size[1]).sum::<f64>() + style.gap[0] * 2.,
            ]
        };
        let intrinsic = [
            intrinsic[0]
                + style.padding[1]
                + style.padding[3]
                + style.border_widths[1]
                + style.border_widths[3],
            intrinsic[1]
                + style.padding[0]
                + style.padding[2]
                + style.border_widths[0]
                + style.border_widths[2],
        ];
        let walk = Walk {
            width: dimension_walk(
                style.width,
                style.min_width,
                style.max_width,
                walk.width,
                intrinsic[0],
            ),
            height: dimension_walk(
                style.height,
                style.min_height,
                style.max_height,
                walk.height,
                intrinsic[1],
            ),
            ..walk
        };
        let pane = cx.walk_turtle(walk);
        body(&mut self.draw_bg, &mut self.draw_shadow, cx, pane, &style);
        let auto = |leaf: &RustifyUtilityLeaf| {
            leaf.model.style().map_or([false; 2], |style| {
                [
                    style.width == GpuLength::Auto,
                    style.height == GpuLength::Auto,
                ]
            })
        };
        let boxes = arrange(
            [pane.pos.x, pane.pos.y, pane.size.x, pane.size.y],
            &style,
            sizes,
            [
                self.button.utility_style().map_or([false; 2], |style| {
                    [
                        style.width == GpuLength::Auto,
                        style.height == GpuLength::Auto,
                    ]
                }),
                auto(&self.text),
                auto(&self.swatch),
            ],
        );
        let mut button_box = boxes[0];
        if let Some(style) = self.button.utility_style() {
            button_box[2] = constrained(
                GpuLength::Auto,
                style.min_width,
                style.max_width,
                button_box[2],
            );
            button_box[3] = constrained(
                GpuLength::Auto,
                style.min_height,
                style.max_height,
                button_box[3],
            );
        }
        self.button.draw_utility_at(
            cx,
            Rect {
                pos: dvec2(button_box[0], button_box[1]),
                size: dvec2(button_box[2], button_box[3]),
            },
        );
        for (leaf, mut box_) in [&mut self.text, &mut self.swatch]
            .into_iter()
            .zip(boxes.into_iter().skip(1))
        {
            if let Some(style) = leaf.model.style() {
                box_[2] = constrained(GpuLength::Auto, style.min_width, style.max_width, box_[2]);
                box_[3] = constrained(GpuLength::Auto, style.min_height, style.max_height, box_[3]);
            }
            leaf.paint(
                cx,
                Rect {
                    pos: dvec2(box_[0], box_[1]),
                    size: dvec2(box_[2], box_[3]),
                },
            );
        }
        self.model
            .drawn([pane.pos.x, pane.pos.y, pane.size.x, pane.size.y], true);
        DrawStep::done()
    }
}
