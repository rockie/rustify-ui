//! Controlled utility state and the existing Button's numerical defaults.
#[cfg(target_arch = "wasm32")]
use crate::gpu::utility::model::{NodeModel, UtilityNodeKind, UtilityNodeProps, UtilityNodeReport};
#[cfg(not(target_arch = "wasm32"))]
use crate::utility::gpu::tests::widget_model::{
    NodeModel, UtilityNodeKind, UtilityNodeProps, UtilityNodeReport,
};
use crate::{
    theme::{ResolvedFont, ResolvedTheme, Rgba, ShadowLayer},
    utility::gpu::*,
};
use rustify_style::program::GpuProgram;
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq)]
pub struct ButtonUtilityProps {
    pub revision: u64,
    pub programs: Arc<[GpuProgram]>,
    pub theme: Arc<ResolvedTheme>,
    pub context: GpuResolveContext,
}
#[derive(Clone)]
pub(crate) enum ButtonPalette {
    Theme([Rgba; 4]),
    Packed([Rgba; 4]),
    Script([Rgba; 3]),
}
#[derive(Clone)]
pub(crate) struct ButtonDefaults {
    pub palette: ButtonPalette,
    pub font: ResolvedFont,
    pub font_size_px: f64,
    pub letter_spacing_em: f64,
    pub line_height_px: f64,
    pub padding_px: f64,
    pub radius_px: f64,
    pub ring: Option<Rgba>,
    pub width: GpuLength,
    pub height: GpuLength,
}
impl ButtonDefaults {
    pub fn compute(&self, state: GpuControlState) -> GpuComputedStyle {
        let colors = match self.palette {
            ButtonPalette::Theme([face, hover, border, ink]) => {
                let mut colors = [
                    if state.active || state.hover {
                        hover
                    } else {
                        face
                    },
                    border,
                    ink,
                ];
                if state.disabled {
                    for color in &mut colors {
                        color.a *= 0.5;
                    }
                }
                colors
            }
            ButtonPalette::Packed([face, hover, border, ink]) => [
                if state.disabled {
                    border
                } else if state.active || state.hover {
                    hover
                } else {
                    face
                },
                border,
                ink,
            ],
            ButtonPalette::Script(colors) => colors,
        };
        GpuComputedStyle {
            width: self.width,
            height: self.height,
            background: colors[0],
            color: colors[2],
            border_colors: [colors[1]; 4],
            border_widths: [1.; 4],
            padding: [0., self.padding_px, 0., self.padding_px],
            flow: GpuFlow::Row,
            items: GpuAlignment::Center,
            justify: GpuJustification::Start,
            radii: [self.radius_px; 4],
            font: self.font.clone(),
            font_size_px: self.font_size_px,
            font_size_points: crate::theme::css_px_to_points(self.font_size_px),
            font_weight: 400,
            line_height: GpuLineHeight::Pixels(self.line_height_px),
            letter_spacing_em: self.letter_spacing_em,
            shadows: self
                .ring
                .filter(|_| state.focused && !state.disabled)
                .map_or_else(Vec::new, |ring| {
                    vec![ShadowLayer {
                        color: Rgba {
                            a: ring.a * 0.5,
                            ..ring
                        },
                        offset_x: 0.,
                        offset_y: 0.,
                        blur: 0.,
                        spread: 3.,
                    }]
                }),
            ..GpuComputedStyle::default()
        }
    }
    #[cfg(test)]
    fn themed(theme: &ResolvedTheme) -> Self {
        let face = theme.colors["primary"];
        Self {
            palette: ButtonPalette::Theme([
                face,
                Rgba {
                    a: face.a * 0.9,
                    ..face
                },
                Rgba {
                    r: 0.,
                    g: 0.,
                    b: 0.,
                    a: 0.,
                },
                theme.colors["primary-foreground"],
            ]),
            font: theme.fonts[0].clone(),
            font_size_px: theme.font_size_px,
            letter_spacing_em: theme.letter_spacing_em,
            line_height_px: theme.font_size_px * 1.4,
            padding_px: theme.spacing_px * 4.,
            radius_px: theme.radii_px[1],
            ring: Some(theme.colors["ring"]),
            width: GpuLength::Pixels(170.),
            height: GpuLength::Pixels(theme.spacing_px * 9.),
        }
    }
}
#[derive(Default)]
pub(crate) struct ButtonUtilityModel {
    pub model: NodeModel,
    #[cfg(target_arch = "wasm32")]
    draw_defaults: Option<DrawDefaults>,
    #[cfg(target_arch = "wasm32")]
    font_binding: Option<(String, Option<String>)>,
    #[cfg(target_arch = "wasm32")]
    pressed_key: bool,
    #[cfg(target_arch = "wasm32")]
    pointer_pressed: bool,
}
impl ButtonUtilityModel {
    pub fn apply(
        &mut self,
        props: &ButtonUtilityProps,
        defaults: &ButtonDefaults,
        state: GpuControlState,
        label: &str,
    ) -> bool {
        let mut context = props.context.clone();
        context.base_style = defaults.compute(state);
        context.state = state;
        self.model.apply(UtilityNodeProps {
            revision: props.revision,
            programs: props.programs.clone(),
            theme: props.theme.clone(),
            context,
            label: label.into(),
            kind: UtilityNodeKind::Button,
        })
    }
    pub fn report(&self) -> UtilityNodeReport {
        self.model.report()
    }
    pub fn refresh(
        &mut self,
        defaults: &ButtonDefaults,
        state: GpuControlState,
        theme: Option<Arc<ResolvedTheme>>,
        viewport: Option<[f64; 2]>,
    ) -> bool {
        let previous = self.model.style().cloned();
        self.model.update_state(state);
        if let Some(props) = self.model.props() {
            let theme = theme.unwrap_or_else(|| props.theme.clone());
            self.model
                .update_baseline(theme, defaults.compute(self.model.state()));
        }
        if let Some(viewport) = viewport {
            self.model.update_viewport(viewport);
        }
        self.model
            .style()
            .is_some_and(|style| previous.as_ref().is_none_or(|old| !old.rendered_eq(style)))
    }
}

#[cfg(target_arch = "wasm32")]
use crate::makepad_widgets::{widget::*, *};
#[cfg(target_arch = "wasm32")]
struct DrawDefaults {
    text: TextStyle,
    ink_centered: bool,
    colors: [Vec4f; 3],
    shape_uniforms: [[f32; 3]; 2],
}
#[cfg(target_arch = "wasm32")]
impl super::RustifyButton {
    /// Programs are evaluated over this Button's actual theme/palette defaults.
    /// Label/disabled remain controlled by set_state; input state can also be
    /// supplied explicitly with set_utility_state.
    pub fn apply_utility_programs(
        &mut self,
        cx: &mut Cx,
        props: &ButtonUtilityProps,
    ) -> GpuResolutionStatus {
        if self.utility.is_none() {
            let mut shape_uniforms = [[6., 0., 0.]; 2];
            for (draw, uniforms) in [&self.draw_bg, &self.draw_border]
                .into_iter()
                .zip(&mut shape_uniforms)
            {
                for (index, uniform) in [id!(corner), id!(stroke_width), id!(flat)]
                    .into_iter()
                    .enumerate()
                {
                    draw.get_uniform(cx, uniform, &mut uniforms[index..index + 1]);
                }
            }
            self.utility = Some(ButtonUtilityModel {
                draw_defaults: Some(DrawDefaults {
                    text: self.draw_text.text_style.clone(),
                    ink_centered: self.draw_text.ink_centered,
                    colors: [
                        self.draw_bg.color,
                        self.draw_border.color,
                        self.draw_text.color,
                    ],
                    shape_uniforms,
                }),
                ..ButtonUtilityModel::default()
            });
        }
        let state = self.button_utility_state(cx);
        let defaults = self.button_utility_defaults(&props.theme);
        let utility = self.utility.as_mut().unwrap();
        if utility.apply(props, &defaults, state, &self.label) {
            self.redraw(cx);
        }
        self.utility_report().unwrap().status
    }
    /// Removes utility ownership and restores the latest ordinary Button defaults.
    pub fn clear_utility(&mut self, cx: &mut Cx) {
        if let Some(utility) = self.utility.take() {
            self.pressed = false;
            if let Some(defaults) = utility.draw_defaults {
                self.draw_text.text_style = defaults.text;
                self.draw_text.ink_centered = defaults.ink_centered;
                self.draw_bg.color = defaults.colors[0];
                self.draw_border.color = defaults.colors[1];
                self.draw_text.color = defaults.colors[2];
                for (draw, uniforms) in [&mut self.draw_bg, &mut self.draw_border]
                    .into_iter()
                    .zip(defaults.shape_uniforms)
                {
                    for (index, uniform) in [id!(corner), id!(stroke_width), id!(flat)]
                        .into_iter()
                        .enumerate()
                    {
                        draw.set_uniform(cx, uniform, &uniforms[index..index + 1]);
                    }
                }
            }
            self.redraw(cx);
        }
    }
    pub fn utility_report(&self) -> Option<UtilityNodeReport> {
        self.utility.as_ref().map(ButtonUtilityModel::report)
    }
    pub fn set_utility_state(&mut self, cx: &mut Cx, mut state: GpuControlState) {
        if state.disabled {
            state.active = false;
            state.focused = false;
            state.focus_visible = false;
        }
        self.disabled = state.disabled;
        self.hovered = state.hover;
        self.pressed = state.active;
        let Some(theme) = self
            .utility
            .as_ref()
            .and_then(|u| u.model.props())
            .map(|p| p.theme.clone())
        else {
            return;
        };
        let defaults = self.button_utility_defaults(&theme);
        let utility = self.utility.as_mut().unwrap();
        if state.disabled {
            utility.pressed_key = false;
            utility.pointer_pressed = false;
        }
        if utility.refresh(&defaults, state, None, None) {
            self.redraw(cx);
        }
    }
    pub fn update_utility_viewport(&mut self, cx: &mut Cx, viewport: [f64; 2]) {
        let Some(theme) = self
            .utility
            .as_ref()
            .and_then(|u| u.model.props())
            .map(|p| p.theme.clone())
        else {
            return;
        };
        let defaults = self.button_utility_defaults(&theme);
        let state = self.button_utility_state(cx);
        if self
            .utility
            .as_mut()
            .unwrap()
            .refresh(&defaults, state, None, Some(viewport))
        {
            self.redraw(cx);
        }
    }
    pub(super) fn restore_utility_text_defaults(&mut self) {
        if let Some(defaults) = self.utility.as_ref().and_then(|u| u.draw_defaults.as_ref()) {
            self.draw_text.text_style = defaults.text.clone();
            self.draw_text.ink_centered = defaults.ink_centered;
        }
    }
    pub(super) fn refresh_utility_theme(&mut self, cx: &mut Cx, theme: &ResolvedTheme) {
        if self.utility.is_none() {
            return;
        }
        if let Some(utility) = &mut self.utility {
            if let Some(defaults) = &mut utility.draw_defaults {
                defaults.text = self.draw_text.text_style.clone();
            }
            utility.font_binding = None;
        }
        self.refresh_utility_defaults(cx, Some(Arc::new(theme.clone())));
    }
    pub(super) fn refresh_utility_defaults(
        &mut self,
        cx: &mut Cx,
        theme: Option<Arc<ResolvedTheme>>,
    ) {
        if self.utility.is_none() {
            return;
        }
        let Some(theme) = theme.or_else(|| {
            self.utility
                .as_ref()
                .and_then(|u| u.model.props())
                .map(|p| p.theme.clone())
        }) else {
            return;
        };
        let defaults = self.button_utility_defaults(&theme);
        let state = self.button_utility_state(cx);
        let utility = self.utility.as_mut().unwrap();
        if state.disabled {
            utility.pressed_key = false;
            utility.pointer_pressed = false;
            self.pressed = false;
        }
        let previous = utility.model.style().cloned();
        utility.refresh(&defaults, state, Some(theme), None);
        // Label is owned by the existing Button setter, including after a
        // rejected class attempt. It never comes from a second label API.
        if utility
            .model
            .style()
            .is_some_and(|s| previous.as_ref().is_none_or(|old| !old.rendered_eq(s)))
        {
            self.redraw(cx);
        }
    }
    fn button_utility_state(&self, cx: &Cx) -> GpuControlState {
        let mut state = self.utility.as_ref().map_or_else(
            || GpuControlState {
                hover: self.hovered,
                active: self.pressed,
                focused: cx.has_key_focus(self.draw_utility_body.area()),
                ..GpuControlState::default()
            },
            |u| u.model.state(),
        );
        state.disabled = self.disabled;
        if state.disabled {
            state.active = false;
            state.focused = false;
            state.focus_visible = false;
        }
        state
    }
    fn button_utility_defaults(&self, theme: &ResolvedTheme) -> ButtonDefaults {
        let rgba = |c: Vec4f| Rgba {
            r: c.x as f64,
            g: c.y as f64,
            b: c.z as f64,
            a: c.w as f64,
        };
        let defaults = self.utility.as_ref().and_then(|u| u.draw_defaults.as_ref());
        let palette = if let Some(colors) = self.theme_palette {
            ButtonPalette::Theme(colors.map(rgba))
        } else if let Some((face, hover, border, ink)) = self.palette {
            ButtonPalette::Packed(
                [face, hover, border, ink].map(|color| rgba(crate::gpu::colour(color))),
            )
        } else {
            ButtonPalette::Script(
                defaults
                    .map_or(
                        [
                            self.draw_bg.color,
                            self.draw_border.color,
                            self.draw_text.color,
                        ],
                        |d| d.colors,
                    )
                    .map(rgba),
            )
        };
        let text = defaults.map_or(&self.draw_text.text_style, |d| &d.text);
        ButtonDefaults {
            palette,
            font: theme.fonts[0].clone(),
            font_size_px: self
                .theme_metrics
                .map_or(text.font_size as f64 * 4. / 3., |m| m.font_size),
            letter_spacing_em: text.letter_spacing,
            line_height_px: self
                .theme_metrics
                .map_or(text.font_size as f64 * 1.4, |m| m.font_size * 1.4),
            padding_px: self.theme_metrics.map_or(10., |m| m.spacing * 4.),
            radius_px: self.theme_metrics.map_or_else(
                || defaults.map_or(12., |defaults| defaults.shape_uniforms[0][0] as f64 * 2.),
                |m| m.radii[1],
            ),
            ring: self.theme_metrics.map(|m| m.ring),
            width: if let Size::Fixed(px) = self.walk.width {
                GpuLength::Pixels(px)
            } else {
                GpuLength::Auto
            },
            height: if let Size::Fixed(px) = self.walk.height {
                GpuLength::Pixels(px)
            } else {
                GpuLength::Auto
            },
        }
    }
    pub(super) fn handle_utility_event(&mut self, cx: &mut Cx, event: &Event) {
        let mut state = self.button_utility_state(cx);
        let previous = state;
        if let Event::KeyDown(key) = event {
            state.modality = InputModality::Keyboard;
            if key.key_code == KeyCode::Tab && !state.disabled {
                if state.focused {
                    cx.set_key_focus(Area::Empty);
                    state.focused = false;
                    state.focus_visible = false;
                } else if !key.modifiers.shift {
                    cx.set_key_focus(self.draw_utility_body.area());
                    state.focused = true;
                    state.focus_visible = true;
                }
            } else if state.focused {
                state.focus_visible = true;
            }
        }
        let utility = self.utility.as_mut().unwrap();
        match event.hits(cx, self.draw_utility_body.area()) {
            Hit::FingerDown(fe) if fe.is_primary_hit() && !state.disabled => {
                state.modality = InputModality::Pointer;
                state.active = true;
                state.focused = true;
                state.focus_visible = false;
                utility.pointer_pressed = true;
                cx.set_key_focus(self.draw_utility_body.area());
            }
            Hit::FingerUp(fe) => {
                let was = utility.pointer_pressed;
                utility.pointer_pressed = false;
                state.active = false;
                if was && fe.is_primary_hit() && fe.is_over && !state.disabled {
                    self.click = Some(());
                }
            }
            Hit::FingerMove(fe) => {
                if fe.has_hovers() {
                    state.hover = fe.is_over;
                }
                if utility.pointer_pressed {
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
                utility.pressed_key = false;
            }
            Hit::KeyDown(key)
                if !state.disabled
                    && matches!(key.key_code, KeyCode::Space | KeyCode::ReturnKey) =>
            {
                if !utility.pressed_key {
                    utility.pressed_key = true;
                    state.active = true;
                }
            }
            Hit::KeyUp(key) if matches!(key.key_code, KeyCode::Space | KeyCode::ReturnKey) => {
                if utility.pressed_key && !state.disabled {
                    self.click = Some(());
                }
                utility.pressed_key = false;
                state.active = false;
            }
            _ => {}
        }
        if state != previous {
            self.set_utility_state(cx, state);
        }
    }
    pub(super) fn draw_utility(&mut self, cx: &mut Cx2d, walk: Walk) -> DrawStep {
        self.draw_utility_inner(cx, walk, None)
    }
    pub(crate) fn utility_style(&self) -> Option<&GpuComputedStyle> {
        self.utility
            .as_ref()
            .and_then(|utility| utility.model.style())
    }
    pub(crate) fn utility_natural_size(&mut self, cx: &mut Cx) -> [f64; 2] {
        use crate::gpu::utility::{
            model::{box_minimum, constrained},
            text_available_width, utility_text_layout,
        };
        let Some(style) = self.utility_style().cloned() else {
            return [0.; 2];
        };
        let label = if self.label.is_empty() {
            "EMPTY"
        } else {
            &self.label
        };
        let utility = self.utility.as_mut().unwrap();
        let parent = utility
            .model
            .layout_context()
            .and_then(|context| context.parent_size[0]);
        let layout = utility_text_layout(
            &mut self.draw_text,
            cx,
            &style,
            label,
            &mut utility.font_binding,
            text_available_width(&style, parent),
        );
        let intrinsic = [
            layout.size[0]
                + style.padding[1]
                + style.padding[3]
                + style.border_widths[1]
                + style.border_widths[3],
            layout.size[1]
                + style.padding[0]
                + style.padding[2]
                + style.border_widths[0]
                + style.border_widths[2],
        ];
        let minimum = box_minimum(&style);
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
    pub(crate) fn draw_utility_at(&mut self, cx: &mut Cx2d, pane: Rect) {
        let _ = self.draw_utility_inner(cx, Walk::default(), Some(pane));
    }
    pub(crate) fn update_utility_parent(&mut self, parent_size: [Option<f64>; 2]) {
        if let Some(utility) = self.utility.as_mut() {
            let control_size = utility
                .model
                .layout_context()
                .map_or([None; 2], |context| context.control_size);
            utility.model.update_layout(parent_size, control_size);
        }
    }
    fn draw_utility_inner(
        &mut self,
        cx: &mut Cx2d,
        walk: Walk,
        placement: Option<Rect>,
    ) -> DrawStep {
        use crate::gpu::utility::{
            dimension_walk, model::box_minimum, text_available_width, utility_text_layout,
        };
        if placement.is_none() {
            self.update_utility_parent(crate::gpu::utility::parent_content_size(cx));
        }
        let Some(style) = self.utility.as_ref().and_then(|u| u.model.style()).cloned() else {
            return DrawStep::done();
        };
        let label = if self.label.is_empty() {
            "EMPTY"
        } else {
            &self.label
        };
        let utility = self.utility.as_mut().unwrap();
        let parent = utility
            .model
            .layout_context()
            .and_then(|context| context.parent_size[0]);
        let size = utility_text_layout(
            &mut self.draw_text,
            cx,
            &style,
            label,
            &mut utility.font_binding,
            text_available_width(&style, parent),
        )
        .size;
        let intrinsic = [
            size[0]
                + style.padding[1]
                + style.padding[3]
                + style.border_widths[1]
                + style.border_widths[3],
            size[1]
                + style.padding[0]
                + style.padding[2]
                + style.border_widths[0]
                + style.border_widths[2],
        ];
        let minimum = box_minimum(&style);
        let mut pane = placement.unwrap_or_else(|| {
            cx.walk_turtle(Walk {
                width: dimension_walk(
                    style.width,
                    GpuLength::Pixels(minimum[0]),
                    style.max_width,
                    if style.width == GpuLength::Auto {
                        Size::fit()
                    } else {
                        walk.width
                    },
                    intrinsic[0],
                ),
                height: dimension_walk(
                    style.height,
                    GpuLength::Pixels(minimum[1]),
                    style.max_height,
                    if style.height == GpuLength::Auto {
                        Size::fit()
                    } else {
                        walk.height
                    },
                    intrinsic[1],
                ),
                ..walk
            })
        });
        pane.size.x = pane.size.x.max(minimum[0]);
        pane.size.y = pane.size.y.max(minimum[1]);
        let parent = utility
            .model
            .layout_context()
            .map_or([None; 2], |context| context.parent_size);
        utility
            .model
            .update_layout(parent, [Some(pane.size.x), Some(pane.size.y)]);
        let style = utility.model.style().cloned().unwrap_or(style);
        // The box is final now; its content width decides where lines break.
        let text = utility_text_layout(
            &mut self.draw_text,
            cx,
            &style,
            label,
            &mut utility.font_binding,
            text_available_width(
                &GpuComputedStyle {
                    width: GpuLength::Pixels(pane.size.x),
                    ..style.clone()
                },
                None,
            ),
        );
        let size = text.size;
        // Sdf2d's stroke coverage never reaches one for a two-pixel border.
        // The utility shader composites a full-width CSS border with its fill.
        crate::gpu::utility::body(
            &mut self.draw_utility_body,
            &mut self.draw_shadow,
            cx,
            pane,
            &style,
        );
        // Widget's generated area/redraw methods use draw_bg. Keep that public
        // area attached to the utility body while the ordinary shader is idle.
        self.draw_bg.draw_vars.area = self.draw_utility_body.area();
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
            _ => (inner[main] - size[main]) * 0.5,
        };
        offset[cross] = match style.items {
            GpuAlignment::Start | GpuAlignment::Stretch => 0.,
            GpuAlignment::Center => (inner[cross] - size[cross]) * 0.5,
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
        utility.model.drawn_text(text.rows(origin));
        use crate::makepad_widgets::makepad_draw::text::fonts::Fonts;
        use std::{cell::RefCell, rc::Rc};
        let ready = utility.font_binding.as_ref().is_some_and(|binding| {
            binding.0 == style.font.family && binding.1 == style.font.resource_path
        }) && cx
            .get_global::<Rc<RefCell<Fonts>>>()
            .borrow()
            .is_font_family_complete(self.draw_text.text_style.font_family_id());
        utility
            .model
            .drawn([pane.pos.x, pane.pos.y, pane.size.x, pane.size.y], ready);
        if !ready {
            self.redraw(cx);
        }
        DrawStep::done()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::theme::{font_context, resolve, Theme, ThemeDocument, ThemeMode};
    use rustify_style::program::{GpuDeclaration, GpuValue};
    fn theme() -> Arc<ResolvedTheme> {
        Arc::new(
            resolve(
                &ThemeDocument::from_legacy(Theme::light(), Theme::dark()),
                ThemeMode::Light,
                &font_context(16., false),
            )
            .unwrap(),
        )
    }
    #[test]
    fn existing_button_theme_defaults_match_actual_primary_geometry_and_typography() {
        let theme = theme();
        let defaults = ButtonDefaults::themed(&theme);
        let normal = defaults.compute(GpuControlState::default());
        assert_eq!(normal.background, theme.colors["primary"]);
        assert_eq!(normal.color, theme.colors["primary-foreground"]);
        assert_eq!(
            normal.padding,
            [0., theme.spacing_px * 4., 0., theme.spacing_px * 4.]
        );
        assert_eq!(normal.radii, [theme.radii_px[1]; 4]);
        assert_eq!(normal.width, GpuLength::Pixels(170.));
        assert_eq!(normal.height, GpuLength::Pixels(theme.spacing_px * 9.));
        assert_eq!(normal.font, theme.fonts[0]);
        assert_eq!(
            normal.line_height,
            GpuLineHeight::Pixels(theme.font_size_px * 1.4)
        );
    }
    #[test]
    fn complete_program_update_and_class_deletion_restore_existing_button_defaults() {
        let theme = theme();
        let defaults = ButtonDefaults::themed(&theme);
        let state = GpuControlState::default();
        let mut props = ButtonUtilityProps {
            revision: 1,
            programs: Arc::from([GpuProgram {
                candidate: "padding".into(),
                conditions: vec![],
                cascade_order: 0,
                declarations: vec![GpuDeclaration {
                    property: "padding".into(),
                    value: GpuValue::CssExpression {
                        value: "8px".into(),
                    },
                    important: false,
                }],
            }]),
            theme: theme.clone(),
            context: GpuResolveContext::new(GpuComputedStyle::default()),
        };
        let mut model = ButtonUtilityModel::default();
        model.apply(&props, &defaults, state, "Existing Button");
        assert_eq!(model.report().computed.padding, [8.; 4]);
        props.revision = 2;
        props.programs = Arc::from([]);
        model.apply(&props, &defaults, state, "Existing Button");
        assert_eq!(model.report().computed, defaults.compute(state));
    }
    #[test]
    fn palette_state_and_theme_refresh_keep_current_button_defaults() {
        let theme = theme();
        let mut defaults = ButtonDefaults::themed(&theme);
        let hover = GpuControlState {
            hover: true,
            ..GpuControlState::default()
        };
        assert_eq!(
            defaults.compute(hover).background.a,
            theme.colors["primary"].a * 0.9
        );
        let disabled = GpuControlState {
            disabled: true,
            focused: true,
            ..hover
        };
        assert_eq!(
            defaults.compute(disabled).color.a,
            theme.colors["primary-foreground"].a * 0.5
        );
        assert!(defaults.compute(disabled).shadows.is_empty());
        let colors = [
            theme.colors["primary"],
            theme.colors["secondary"],
            theme.colors["border"],
            theme.colors["foreground"],
        ];
        defaults.palette = ButtonPalette::Packed(colors);
        assert_eq!(defaults.compute(disabled).background, colors[2]);
        assert_eq!(defaults.compute(disabled).color, colors[3]);
        defaults.palette = ButtonPalette::Script([colors[0], colors[2], colors[3]]);
        assert_eq!(defaults.compute(hover).background, colors[0]);
        let mut model = ButtonUtilityModel::default();
        let props = ButtonUtilityProps {
            revision: 1,
            programs: Arc::from([]),
            theme: theme.clone(),
            context: GpuResolveContext::new(GpuComputedStyle::default()),
        };
        model.apply(
            &props,
            &defaults,
            GpuControlState::default(),
            "Actual Button",
        );
        defaults.padding_px = 17.;
        assert!(model.refresh(&defaults, hover, Some(theme), Some([1280., 720.])));
        assert_eq!(model.report().computed.padding, [0., 17., 0., 17.]);
        assert!(model.report().state.hover);
        assert!(!model.refresh(&defaults, hover, None, None));
    }
    #[test]
    fn scoped_theme_refresh_recomputes_rem_dark_and_boundary_tokens_without_recompile() {
        use crate::theme::ThemeValuePatch;
        use rustify_style::program::GpuCondition;
        let outer = theme();
        let mut props = ButtonUtilityProps {
            revision: 7,
            programs: Arc::from([
                GpuProgram {
                    candidate: "scope-width".into(),
                    conditions: vec![],
                    cascade_order: 0,
                    declarations: vec![
                        GpuDeclaration {
                            property: "width".into(),
                            value: GpuValue::CssExpression {
                                value: "1rem".into(),
                            },
                            important: false,
                        },
                        GpuDeclaration {
                            property: "background-color".into(),
                            value: GpuValue::CssExpression {
                                value: "var(--primary)".into(),
                            },
                            important: false,
                        },
                        GpuDeclaration {
                            property: "padding".into(),
                            value: GpuValue::CssExpression {
                                value: "calc(var(--spacing) * 2)".into(),
                            },
                            important: false,
                        },
                    ],
                },
                GpuProgram {
                    candidate: "dark-ink".into(),
                    conditions: vec![GpuCondition::Dark],
                    cascade_order: 1,
                    declarations: vec![GpuDeclaration {
                        property: "color".into(),
                        value: GpuValue::CssExpression {
                            value: "#abcdef".into(),
                        },
                        important: false,
                    }],
                },
            ]),
            theme: outer.clone(),
            context: GpuResolveContext::new(GpuComputedStyle::default()),
        };
        props
            .context
            .variables
            .insert("--spacing".into(), "3px".into());
        let state = GpuControlState::default();
        let mut model = ButtonUtilityModel::default();
        model.apply(
            &props,
            &ButtonDefaults::themed(&outer),
            state,
            "Actual Button",
        );
        model.model.drawn([0., 0., 170., 36.], true);
        assert_eq!(model.report().status, GpuResolutionStatus::Applied);
        assert_eq!(model.report().computed.width, GpuLength::Pixels(16.));
        let dark = Arc::new(
            resolve(
                &ThemeDocument::from_legacy(Theme::light(), Theme::dark()),
                ThemeMode::Dark,
                &font_context(20., false),
            )
            .unwrap(),
        );
        assert!(model.refresh(
            &ButtonDefaults::themed(&dark),
            state,
            Some(dark.clone()),
            None
        ));
        let snapshot = model.report();
        assert_eq!(snapshot.computed.width, GpuLength::Pixels(20.));
        assert_eq!(snapshot.computed.background, dark.colors["primary"]);
        assert_eq!(
            snapshot.computed.color,
            crate::theme::parse_color("#abcdef").unwrap().rgba
        );
        assert_eq!(snapshot.computed.padding, [6.; 4]);
        assert_eq!(model.model.props().unwrap().context.media_rem_px, 16.);
        assert_eq!(snapshot.applied_revision, Some(7));
        assert_eq!(snapshot.drawn_revision, Some(7));
        let boundary = Arc::new(
            dark.patched(&ThemeValuePatch(std::collections::BTreeMap::from([(
                "primary".into(),
                "#123456".into(),
            )])))
            .unwrap(),
        );
        assert!(model.refresh(
            &ButtonDefaults::themed(&boundary),
            state,
            Some(boundary.clone()),
            None
        ));
        assert_eq!(
            model.report().computed.background,
            boundary.colors["primary"]
        );
        assert!(model.refresh(
            &ButtonDefaults::themed(&outer),
            state,
            Some(outer.clone()),
            None
        ));
        assert_eq!(model.report().computed.width, GpuLength::Pixels(16.));
        assert_eq!(model.report().computed.background, outer.colors["primary"]);
        assert!(!model.refresh(&ButtonDefaults::themed(&outer), state, Some(outer), None));
    }
    #[test]
    fn initial_invalid_utility_context_retains_actual_button_baseline() {
        let theme = theme();
        let defaults = ButtonDefaults::themed(&theme);
        let state = GpuControlState::default();
        let mut context = GpuResolveContext::new(GpuComputedStyle::default());
        context.viewport = [f64::NAN, 600.];
        let props = ButtonUtilityProps {
            revision: 1,
            programs: Arc::from([]),
            theme,
            context,
        };
        let mut model = ButtonUtilityModel::default();
        model.apply(&props, &defaults, state, "Existing Button");
        assert_eq!(model.report().status, GpuResolutionStatus::Rejected);
        assert_eq!(model.report().computed, defaults.compute(state));
        assert_eq!(model.report().applied_revision, None);
    }
    #[test]
    fn button_strict_retains_snapshot_partial_is_visible_and_group_opacity_rejects() {
        let theme = theme();
        let defaults = ButtonDefaults::themed(&theme);
        let state = GpuControlState::default();
        let mut props = ButtonUtilityProps {
            revision: 1,
            programs: Arc::from([]),
            theme,
            context: GpuResolveContext::new(GpuComputedStyle::default()),
        };
        let mut model = ButtonUtilityModel::default();
        model.apply(&props, &defaults, state, "Existing Button");
        model.model.drawn([0., 0., 170., 36.], true);
        let old = model.report().computed;
        props.revision = 2;
        props.programs = Arc::from([GpuProgram {
            candidate: "opacity".into(),
            conditions: vec![],
            cascade_order: 0,
            declarations: vec![GpuDeclaration {
                property: "opacity".into(),
                value: GpuValue::CssExpression {
                    value: "0.5".into(),
                },
                important: false,
            }],
        }]);
        assert!(!model.apply(&props, &defaults, state, "Existing Button"));
        let rejected = model.report();
        assert_eq!(rejected.status, GpuResolutionStatus::Rejected);
        assert_eq!(rejected.computed, old);
        assert_eq!(rejected.attempted_revision, Some(2));
        assert_eq!(rejected.applied_revision, Some(1));
        assert_eq!(rejected.drawn_revision, Some(1));
        assert!(!rejected.diagnostics.is_empty());
        props.context.policy = GpuApplyPolicy::Partial;
        model.apply(&props, &defaults, state, "Existing Button");
        assert_eq!(model.report().status, GpuResolutionStatus::Partial);
        assert_eq!(model.report().computed.opacity, 1.);
        let focused = GpuControlState {
            focused: true,
            ..state
        };
        assert_eq!(defaults.compute(focused).shadows[0].spread, 3.);
    }
}
