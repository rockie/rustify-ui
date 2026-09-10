//! A button a region draws, in the scope's own colours.
//!
//! Makepad has its own, and it is a fine button - but it carries makepad's
//! theme, which in a light scope is pale text on a pale face. A catalogue that
//! claims both halves draw from one token table has to mean it, so this one
//! takes the same table every other control here takes.

use crate::makepad_widgets::widget::*;
use crate::makepad_widgets::*;

use super::colour;

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
        if self.palette != next {
            self.palette = next;
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
        match event.hits(cx, self.draw_bg.area()) {
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
        self.draw_border.draw_abs(cx, pane);
        self.draw_bg
            .draw_abs(cx, super::toggles::inset_rect(pane, 1.0));
        // The line box, centred in the face. `draw_abs` places a line by its
        // top-left, so a text drawn at the middle of the box sits below it.
        let line = self.draw_text.text_style.font_size as f64 * 1.4;
        self.draw_text.draw_abs(
            cx,
            dvec2(
                pane.pos.x + 10.0,
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
