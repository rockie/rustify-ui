//! The GPU half of the controls that have one on both sides of the boundary.
//!
//! Neither of these widgets holds the value it shows. A click is a request the
//! application answers by handing back a new projection, exactly as the DOM
//! half works, so one value cannot mean two things on one screen. A disabled
//! or read-only control makes no request at all.

use crate::components::snap;
use crate::makepad_widgets::widget::*;
use crate::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets_internal.*

    mod.widgets.RustifyCheckBoxBase = #(RustifyCheckBox::register_widget(vm))

    mod.widgets.RustifyCheckBox = set_type_default() do mod.widgets.RustifyCheckBoxBase{
        width: 24
        height: 24
        draw_bg +: {
            color: #x101828
        }
        draw_mark +: {
            color: #x2e90fa
        }
        draw_border +: {
            color: #xd0d5dd
        }
    }

    mod.widgets.RustifySliderBase = #(RustifySlider::register_widget(vm))

    mod.widgets.RustifySlider = set_type_default() do mod.widgets.RustifySliderBase{
        width: 140
        height: 24
        draw_bg +: {
            color: #x101828
        }
        draw_fill +: {
            color: #x2e90fa
        }
        draw_knob +: {
            color: #xffffff
        }
    }
}

fn colour(rgb: u32) -> Vec4f {
    Vec4f::from_u32(rgb << 8 | 0xff)
}

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

/// A value on a range drawn by a region.
///
/// The value is snapped by the same rule the DOM half uses, so dragging the
/// GPU knob and dragging the DOM range cannot produce values from two
/// different sets.
#[derive(Script, ScriptHook, Widget)]
pub struct RustifySlider {
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
    draw_fill: DrawColor,
    #[live]
    draw_knob: DrawColor,
    #[rust]
    value: f64,
    #[rust]
    disabled: bool,
    #[rust]
    read_only: bool,
    /// Minimum, maximum and step. `None` until the application says; a slider
    /// with no range would answer every drag with the same number.
    #[rust]
    range: Option<(f64, f64, f64)>,
    /// Track, fill and knob, from the scope's theme.
    #[rust]
    palette: Option<(u32, u32, u32)>,
    #[rust]
    change: Option<f64>,
}

/// The default range, used until an application gives its own.
const RANGE: (f64, f64, f64) = (0.0, 100.0, 1.0);

impl RustifySlider {
    pub fn set_state(&mut self, cx: &mut Cx, value: f64, disabled: bool, read_only: bool) {
        if self.value != value || self.disabled != disabled || self.read_only != read_only {
            self.value = value;
            self.disabled = disabled;
            self.read_only = read_only;
            self.redraw(cx);
        }
    }

    pub fn set_range(&mut self, cx: &mut Cx, min: f64, max: f64, step: f64) {
        let next = Some((min, max, step));
        if self.range != next {
            self.range = next;
            self.redraw(cx);
        }
    }

    pub fn set_palette(&mut self, cx: &mut Cx, track: u32, fill: u32, knob: u32) {
        let next = Some((track, fill, knob));
        if self.palette != next {
            self.palette = next;
            self.redraw(cx);
        }
    }

    pub fn take_change(&mut self) -> Option<f64> {
        self.change.take()
    }

    /// The track where it ended up, in the region's own local pixels. See
    /// `RustifyCheckBox::drawn`.
    pub fn drawn(&self, cx: &Cx) -> Option<Rect> {
        let area = self.draw_bg.area();
        (!area.is_empty()).then(|| area.rect(cx))
    }

    fn range(&self) -> (f64, f64, f64) {
        self.range.unwrap_or(RANGE)
    }

    /// Where along the track the value sits, as 0..=1.
    fn fraction(&self) -> f64 {
        let (min, max, _) = self.range();
        if max <= min {
            return 0.0;
        }
        ((self.value - min) / (max - min)).clamp(0.0, 1.0)
    }

    /// The value the pointer is asking for at `x`.
    fn value_at(&self, track: Rect, x: f64) -> f64 {
        let (min, max, step) = self.range();
        let span = track.size.x.max(1.0);
        let fraction = ((x - track.pos.x) / span).clamp(0.0, 1.0);
        snap(min + fraction * (max - min), min, max, step)
    }

    fn ask(&mut self, cx: &Cx, x: f64) {
        let Some(track) = self.drawn(cx) else {
            return;
        };
        if self.disabled || self.read_only {
            return;
        }
        let asked = self.value_at(track, x);
        if asked != self.value {
            self.change = Some(asked);
        }
    }
}

impl Widget for RustifySlider {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        match event.hits(cx, self.draw_bg.area()) {
            Hit::FingerDown(fe) => {
                if fe.is_primary_hit() {
                    self.ask(cx, fe.abs.x);
                }
            }
            Hit::FingerMove(fe) => self.ask(cx, fe.abs.x),
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let pane = cx.walk_turtle(walk);
        if let Some((track, fill, knob)) = self.palette {
            self.draw_bg.color = colour(track);
            self.draw_fill.color = colour(fill);
            self.draw_knob.color = colour(knob);
        }
        self.draw_bg.draw_abs(cx, pane);
        let fraction = self.fraction();
        if fraction > 0.0 {
            self.draw_fill.draw_abs(
                cx,
                Rect {
                    pos: pane.pos,
                    size: dvec2(pane.size.x * fraction, pane.size.y),
                },
            );
        }
        let knob = pane.size.y.min(pane.size.x).max(2.0);
        // Kept inside the track: a knob half off the end would report a value
        // the pointer never asked for.
        let left = pane.pos.x + (pane.size.x - knob) * fraction;
        self.draw_knob.draw_abs(
            cx,
            Rect {
                pos: dvec2(left, pane.pos.y),
                size: dvec2(knob, pane.size.y),
            },
        );
        DrawStep::done()
    }
}
