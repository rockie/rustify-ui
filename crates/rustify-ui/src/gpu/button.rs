//! A button a region draws, in the scope's own colours.
//!
//! Makepad has its own, and it is a fine button - but it carries makepad's
//! theme, which in a light scope is pale text on a pale face. A catalogue that
//! claims both halves draw from one token table has to mean it, so this one
//! takes the same table every other control here takes.

use crate::makepad_widgets::widget::*;
use crate::makepad_widgets::*;

use super::colour;
use super::theme::{disabled_color, draw_focus, focus_hit, set_box_radius, token, ThemeMetrics};
use super::{apply_text_theme, DrawRustifyShadow, FontSlot};
use crate::theme::ResolvedTheme;

#[derive(Script, ScriptHook, Widget)]
pub struct RustifyButton {
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
    draw_border: DrawColor,
    #[live]
    draw_text: DrawText,
    #[live]
    draw_shadow: DrawRustifyShadow,
    #[rust]
    label: String,
    #[rust]
    disabled: bool,
    /// Held down, and under the pointer. Drawn, because a state nobody can see
    /// is a state the control does not have.
    #[rust]
    pressed: bool,
    #[rust]
    hovered: bool,
    /// Face, face under the pointer, border and ink.
    #[rust]
    palette: Option<(u32, u32, u32, u32)>,
    #[rust]
    theme_palette: Option<[Vec4f; 4]>,
    #[rust]
    theme_metrics: Option<ThemeMetrics>,
    #[rust]
    click: Option<()>,
}

impl RustifyButton {
    pub fn set_state(&mut self, cx: &mut Cx, label: &str, disabled: bool) {
        if self.label != label || self.disabled != disabled {
            self.label = label.to_string();
            self.disabled = disabled;
            self.redraw(cx);
        }
    }

    pub fn set_palette(&mut self, cx: &mut Cx, face: u32, hover: u32, border: u32, ink: u32) {
        let next = Some((face, hover, border, ink));
        if self.palette != next || self.theme_palette.is_some() {
            self.palette = next;
            self.theme_palette = None;
            self.redraw(cx);
        }
    }

    /// Applies the same resolved RGBA, typography and geometry as the DOM theme.
    pub fn apply_theme(&mut self, cx: &mut Cx, theme: &ResolvedTheme) {
        let face = token(theme, "primary");
        let mut hover = face;
        hover.w *= 0.9;
        let palette = [
            face,
            hover,
            vec4(0.0, 0.0, 0.0, 0.0),
            token(theme, "primary-foreground"),
        ];
        let metrics = ThemeMetrics::from(theme);
        let changed = self.theme_palette != Some(palette) || self.theme_metrics != Some(metrics);
        let text_changed = apply_text_theme(&mut self.draw_text, cx, theme, FontSlot::Sans);
        self.palette = None;
        self.theme_palette = Some(palette);
        self.theme_metrics = Some(metrics);
        self.walk.height = Size::Fixed(metrics.spacing * 9.0);
        if changed || text_changed {
            self.redraw(cx);
        }
    }

    pub fn take_click(&mut self) -> Option<()> {
        self.click.take()
    }

    /// Where it ended up, in the region's own local pixels.
    pub fn drawn(&self, cx: &Cx) -> Option<Rect> {
        let area = self.draw_bg.area();
        (!area.is_empty()).then(|| area.rect(cx))
    }
}

impl Widget for RustifyButton {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        let hit = event.hits(cx, self.draw_bg.area());
        if focus_hit(&hit, cx, self.draw_bg.area(), self.disabled) {
            self.redraw(cx);
        }
        match hit {
            Hit::FingerDown(fe) => {
                if fe.is_primary_hit() && !self.disabled {
                    self.pressed = true;
                    self.redraw(cx);
                }
            }
            Hit::FingerUp(fe) => {
                let was = self.pressed;
                self.pressed = false;
                self.redraw(cx);
                if was && fe.is_over && fe.is_primary_hit() && !self.disabled {
                    self.click = Some(());
                }
            }
            Hit::FingerHoverIn(_) => {
                self.hovered = true;
                self.redraw(cx);
            }
            Hit::FingerHoverOut(_) => {
                self.hovered = false;
                self.redraw(cx);
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let pane = cx.walk_turtle(walk);
        if let Some([face, hover, border, ink]) = self.theme_palette {
            self.draw_bg.color = disabled_color(
                if self.pressed || self.hovered {
                    hover
                } else {
                    face
                },
                self.disabled,
            );
            self.draw_border.color = disabled_color(border, self.disabled);
            self.draw_text.color = disabled_color(ink, self.disabled);
        }
        if let Some((face, hover, border, ink)) = self.palette {
            let face = if self.disabled {
                border
            } else if self.pressed || self.hovered {
                hover
            } else {
                face
            };
            self.draw_bg.color = colour(face);
            self.draw_border.color = colour(border);
            self.draw_text.color = colour(ink);
        }
        if let Some(metrics) = self.theme_metrics {
            let focused = cx.has_key_focus(self.draw_bg.area()) && !self.disabled;
            draw_focus(
                &mut self.draw_shadow,
                cx,
                pane,
                metrics.radii[1],
                metrics,
                focused,
            );
            set_box_radius(&mut self.draw_bg, cx, metrics.radii[1], 0.0);
            set_box_radius(&mut self.draw_border, cx, metrics.radii[1], 1.0);
            self.draw_bg.draw_abs(cx, pane);
            self.draw_border.draw_abs(cx, pane);
        } else {
            self.draw_border.draw_abs(cx, pane);
            self.draw_bg
                .draw_abs(cx, super::toggles::inset_rect(pane, 1.0));
        }
        // The line box, centred in the face. `draw_abs` places a line by its
        // top-left, so a text drawn at the middle of the box sits below it.
        let line = self
            .theme_metrics
            .map(|metrics| metrics.font_size * 1.4)
            .unwrap_or(self.draw_text.text_style.font_size as f64 * 1.4);
        let padding = self
            .theme_metrics
            .map(|metrics| metrics.spacing * 4.0)
            .unwrap_or(10.0);
        self.draw_text.draw_abs(
            cx,
            dvec2(
                pane.pos.x + padding,
                pane.pos.y + (pane.size.y - line).max(0.0) * 0.5,
            ),
            if self.label.is_empty() {
                "EMPTY"
            } else {
                &self.label
            },
        );
        DrawStep::done()
    }
}
