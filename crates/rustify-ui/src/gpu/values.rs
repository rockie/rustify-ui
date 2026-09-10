//! The controls that carry a number: one the user moves, one they only read.

use crate::components::snap;
use crate::makepad_widgets::widget::*;
use crate::makepad_widgets::*;

use super::colour;

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

/// How far along something is, drawn by a region.
///
/// It takes no input, so there is nothing here about disabled or read-only: a
/// bar is a reading. What it does share with the rest is where the number
/// comes from, which is the application and nowhere else.
#[derive(Script, ScriptHook, Widget)]
pub struct RustifyProgress {
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
    /// 0..=1. A bar with no fraction yet draws an empty track rather than a
    /// full one: the DOM half animates an unknown quantity, and a region that
    /// did the same would have to keep the page's GPU awake to do it.
    #[rust]
    fraction: f64,
    /// Track and fill, from the scope's theme.
    #[rust]
    palette: Option<(u32, u32)>,
}

impl RustifyProgress {
    pub fn set_fraction(&mut self, cx: &mut Cx, fraction: f64) {
        let fraction = fraction.clamp(0.0, 1.0);
        if self.fraction != fraction {
            self.fraction = fraction;
            self.redraw(cx);
        }
    }

    pub fn set_palette(&mut self, cx: &mut Cx, track: u32, fill: u32) {
        let next = Some((track, fill));
        if self.palette != next {
            self.palette = next;
            self.redraw(cx);
        }
    }

    /// Where it ended up, in the region's own local pixels.
    pub fn drawn(&self, cx: &Cx) -> Option<Rect> {
        let area = self.draw_bg.area();
        (!area.is_empty()).then(|| area.rect(cx))
    }
}

impl Widget for RustifyProgress {
    fn handle_event(&mut self, _cx: &mut Cx, _event: &Event, _scope: &mut Scope) {}

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let pane = cx.walk_turtle(walk);
        if let Some((track, fill)) = self.palette {
            self.draw_bg.color = colour(track);
            self.draw_fill.color = colour(fill);
        }
        self.draw_bg.draw_abs(cx, pane);
        if self.fraction > 0.0 {
            // Never narrower than it is tall: a rounded end needs the room, and
            // a one-percent bar drawn as a sliver reads as nothing at all.
            let width = (pane.size.x * self.fraction).max(pane.size.y.min(pane.size.x));
            self.draw_fill.draw_abs(
                cx,
                Rect {
                    pos: pane.pos,
                    size: dvec2(width, pane.size.y),
                },
            );
        }
        DrawStep::done()
    }
}
