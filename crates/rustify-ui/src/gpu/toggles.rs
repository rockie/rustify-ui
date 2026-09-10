//! The two-state controls: a box, a circle in a group, and a track a knob
//! slides along. All three ask; none of them decides.

use crate::makepad_widgets::widget::*;
use crate::makepad_widgets::*;

use super::colour;

/// A two-state control drawn by a region.
///
/// It never flips itself: a click asks, and the box shows whatever the
/// application projected back. That is what keeps it in step with the DOM
/// checkbox bound to the same value.
#[derive(Script, ScriptHook, Widget)]
pub struct RustifyCheckBox {
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
    draw_mark: DrawColor,
    #[live]
    draw_border: DrawColor,
    #[rust]
    active: bool,
    /// Both default to false, which is the ordinary control: usable.
    #[rust]
    disabled: bool,
    #[rust]
    read_only: bool,
    /// Box, mark and border, from the scope's theme.
    #[rust]
    palette: Option<(u32, u32, u32)>,
    /// The value the user asked for, taken by the region app. Not the value:
    /// the application decides whether it becomes one.
    #[rust]
    change: Option<bool>,
}

impl RustifyCheckBox {
    pub fn set_state(&mut self, cx: &mut Cx, active: bool, disabled: bool, read_only: bool) {
        if self.active != active || self.disabled != disabled || self.read_only != read_only {
            self.active = active;
            self.disabled = disabled;
            self.read_only = read_only;
            self.redraw(cx);
        }
    }

    pub fn set_palette(&mut self, cx: &mut Cx, box_colour: u32, mark: u32, border: u32) {
        let next = Some((box_colour, mark, border));
        if self.palette != next {
            self.palette = next;
            self.redraw(cx);
        }
    }

    pub fn take_change(&mut self) -> Option<bool> {
        self.change.take()
    }

    /// Where it ended up, in the region's own local pixels.
    ///
    /// The area rather than the rectangle the layout walked: a parent that
    /// aligns its children moves them after the walk, and what a pointer has
    /// to be aimed at is where they ended up.
    pub fn drawn(&self, cx: &Cx) -> Option<Rect> {
        let area = self.draw_bg.area();
        (!area.is_empty()).then(|| area.rect(cx))
    }
}

impl Widget for RustifyCheckBox {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        if let Hit::FingerUp(fe) = event.hits(cx, self.draw_bg.area()) {
            if !fe.is_over || !fe.is_primary_hit() {
                return;
            }
            // A control the application has closed makes no request; there is
            // nothing to refuse later because nothing was asked.
            if self.disabled || self.read_only {
                return;
            }
            self.change = Some(!self.active);
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let pane = cx.walk_turtle(walk);
        if let Some((box_colour, mark, border)) = self.palette {
            self.draw_bg.color = colour(box_colour);
            self.draw_mark.color = colour(mark);
            self.draw_border.color = colour(border);
        }
        // The border is drawn first and covered by the box, so what is left is
        // an outline rather than a second quad over the mark.
        self.draw_border.draw_abs(cx, pane);
        let inset = 2.0f64.min(pane.size.x * 0.2).min(pane.size.y * 0.2);
        self.draw_bg.draw_abs(
            cx,
            Rect {
                pos: dvec2(pane.pos.x + inset, pane.pos.y + inset),
                size: dvec2(
                    (pane.size.x - inset * 2.0).max(0.0),
                    (pane.size.y - inset * 2.0).max(0.0),
                ),
            },
        );
        if self.active {
            let mark = (pane.size.x.min(pane.size.y) * 0.5).max(1.0);
            self.draw_mark.draw_abs(
                cx,
                Rect {
                    pos: dvec2(
                        pane.pos.x + (pane.size.x - mark) * 0.5,
                        pane.pos.y + (pane.size.y - mark) * 0.5,
                    ),
                    size: dvec2(mark, mark),
                },
            );
        }
        DrawStep::done()
    }
}

/// One choice of several, drawn by a region.
///
/// A radio is not a checkbox that happens to be round: it can only be turned
/// on, and turning one on is the group's business, not this widget's. So the
/// request it makes carries nothing - "this one, please" - and the application
/// decides what that does to the rest.
#[derive(Script, ScriptHook, Widget)]
pub struct RustifyRadio {
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
    draw_mark: DrawColor,
    #[live]
    draw_border: DrawColor,
    #[rust]
    active: bool,
    #[rust]
    disabled: bool,
    #[rust]
    read_only: bool,
    /// Disc, dot and ring, from the scope's theme.
    #[rust]
    palette: Option<(u32, u32, u32)>,
    /// Set when the user asked for this one. Not a value: the application
    /// decides whether it becomes one.
    #[rust]
    change: Option<()>,
}

impl RustifyRadio {
    pub fn set_state(&mut self, cx: &mut Cx, active: bool, disabled: bool, read_only: bool) {
        if self.active != active || self.disabled != disabled || self.read_only != read_only {
            self.active = active;
            self.disabled = disabled;
            self.read_only = read_only;
            self.redraw(cx);
        }
    }

    pub fn set_palette(&mut self, cx: &mut Cx, disc: u32, dot: u32, ring: u32) {
        let next = Some((disc, dot, ring));
        if self.palette != next {
            self.palette = next;
            self.redraw(cx);
        }
    }

    pub fn take_change(&mut self) -> Option<()> {
        self.change.take()
    }

    /// Where it ended up, in the region's own local pixels.
    pub fn drawn(&self, cx: &Cx) -> Option<Rect> {
        let area = self.draw_bg.area();
        (!area.is_empty()).then(|| area.rect(cx))
    }
}

impl Widget for RustifyRadio {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        if let Hit::FingerUp(fe) = event.hits(cx, self.draw_bg.area()) {
            if !fe.is_over || !fe.is_primary_hit() {
                return;
            }
            if self.disabled || self.read_only {
                return;
            }
            // Choosing what is already chosen is not a change, and an
            // application that hears one has to work out that it is not.
            if !self.active {
                self.change = Some(());
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let pane = cx.walk_turtle(walk);
        if let Some((disc, dot, ring)) = self.palette {
            self.draw_bg.color = colour(disc);
            self.draw_mark.color = colour(dot);
            self.draw_border.color = colour(ring);
        }
        self.draw_border.draw_abs(cx, pane);
        let inset = 2.0f64.min(pane.size.x * 0.2).min(pane.size.y * 0.2);
        self.draw_bg.draw_abs(cx, inset_rect(pane, inset));
        if self.active {
            self.draw_mark
                .draw_abs(cx, inset_rect(pane, pane.size.x.min(pane.size.y) * 0.3));
        }
        DrawStep::done()
    }
}

/// The same rectangle, pulled in on every side.
pub(super) fn inset_rect(rect: Rect, by: f64) -> Rect {
    Rect {
        pos: dvec2(rect.pos.x + by, rect.pos.y + by),
        size: dvec2(
            (rect.size.x - by * 2.0).max(0.0),
            (rect.size.y - by * 2.0).max(0.0),
        ),
    }
}

/// A switch drawn by a region: two states, presented as a thing that moves.
#[derive(Script, ScriptHook, Widget)]
pub struct RustifyToggle {
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
    draw_track: DrawColor,
    #[live]
    draw_knob: DrawColor,
    #[rust]
    active: bool,
    #[rust]
    disabled: bool,
    #[rust]
    read_only: bool,
    /// The track when off, the track when on, and the knob.
    #[rust]
    palette: Option<(u32, u32, u32)>,
    #[rust]
    change: Option<bool>,
}

impl RustifyToggle {
    pub fn set_state(&mut self, cx: &mut Cx, active: bool, disabled: bool, read_only: bool) {
        if self.active != active || self.disabled != disabled || self.read_only != read_only {
            self.active = active;
            self.disabled = disabled;
            self.read_only = read_only;
            self.redraw(cx);
        }
    }

    pub fn set_palette(&mut self, cx: &mut Cx, off: u32, on: u32, knob: u32) {
        let next = Some((off, on, knob));
        if self.palette != next {
            self.palette = next;
            self.redraw(cx);
        }
    }

    pub fn take_change(&mut self) -> Option<bool> {
        self.change.take()
    }

    /// Where it ended up, in the region's own local pixels.
    pub fn drawn(&self, cx: &Cx) -> Option<Rect> {
        let area = self.draw_track.area();
        (!area.is_empty()).then(|| area.rect(cx))
    }
}

impl Widget for RustifyToggle {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        if let Hit::FingerUp(fe) = event.hits(cx, self.draw_track.area()) {
            if !fe.is_over || !fe.is_primary_hit() {
                return;
            }
            if self.disabled || self.read_only {
                return;
            }
            self.change = Some(!self.active);
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let pane = cx.walk_turtle(walk);
        if let Some((off, on, knob)) = self.palette {
            self.draw_track.color = colour(if self.active { on } else { off });
            self.draw_knob.color = colour(knob);
        }
        self.draw_track.draw_abs(cx, pane);
        let inset = (pane.size.y * 0.1).min(3.0);
        let diameter = (pane.size.y - inset * 2.0).max(0.0);
        // Right when on, left when off, and never past either end.
        let travel = (pane.size.x - diameter - inset * 2.0).max(0.0);
        let left = pane.pos.x + inset + if self.active { travel } else { 0.0 };
        self.draw_knob.draw_abs(
            cx,
            Rect {
                pos: dvec2(left, pane.pos.y + inset),
                size: dvec2(diameter, diameter),
            },
        );
        DrawStep::done()
    }
}
