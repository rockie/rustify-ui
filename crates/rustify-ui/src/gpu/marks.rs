//! The two that say something without taking anything: a glyph and a spinner.

use crate::makepad_widgets::widget::*;
use crate::makepad_widgets::*;

use super::colour;

/// The drawings a region can put beside its own text.
///
/// The same six the DOM `Icon` has, on the same twenty-four unit grid, so a
/// catalogue can put the two halves side by side and they are the same shape.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Glyph {
    #[default]
    Check,
    ChevronDown,
    ChevronUp,
    ChevronRight,
    Close,
    Dot,
}

impl Glyph {
    pub fn name(self) -> &'static str {
        match self {
            Self::Check => "check",
            Self::ChevronDown => "chevron-down",
            Self::ChevronUp => "chevron-up",
            Self::ChevronRight => "chevron-right",
            Self::Close => "close",
            Self::Dot => "dot",
        }
    }
}

/// One glyph, drawn by a region.
///
/// Six drawings live in one widget rather than in six, because which one is
/// showing is a value the application changes and a widget it swaps for
/// another is a widget that loses its place in the layout.
#[derive(Script, ScriptHook, Widget)]
pub struct RustifyIcon {
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
    draw_check: DrawColor,
    #[live]
    draw_chevron_down: DrawColor,
    #[live]
    draw_chevron_up: DrawColor,
    #[live]
    draw_chevron_right: DrawColor,
    #[live]
    draw_close: DrawColor,
    #[live]
    draw_dot: DrawColor,
    #[rust]
    glyph: Glyph,
    #[rust]
    ink: Option<u32>,
}

impl RustifyIcon {
    pub fn set_glyph(&mut self, cx: &mut Cx, glyph: Glyph) {
        if self.glyph != glyph {
            self.glyph = glyph;
            self.redraw(cx);
        }
    }

    pub fn set_ink(&mut self, cx: &mut Cx, ink: u32) {
        if self.ink != Some(ink) {
            self.ink = Some(ink);
            self.redraw(cx);
        }
    }

    fn drawing(&mut self) -> &mut DrawColor {
        match self.glyph {
            Glyph::Check => &mut self.draw_check,
            Glyph::ChevronDown => &mut self.draw_chevron_down,
            Glyph::ChevronUp => &mut self.draw_chevron_up,
            Glyph::ChevronRight => &mut self.draw_chevron_right,
            Glyph::Close => &mut self.draw_close,
            Glyph::Dot => &mut self.draw_dot,
        }
    }
}

impl Widget for RustifyIcon {
    fn handle_event(&mut self, _cx: &mut Cx, _event: &Event, _scope: &mut Scope) {}

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let pane = cx.walk_turtle(walk);
        let ink = self.ink;
        let drawing = self.drawing();
        if let Some(ink) = ink {
            drawing.color = colour(ink);
        }
        // A dot fills its box; the strokes are drawn on the grid the shader
        // scales, so both want the whole rectangle.
        drawing.draw_abs(cx, pane);
        DrawStep::done()
    }
}

/// Something is happening and there is no fraction to show.
///
/// The arc turns on the pass clock, and the pass keeps arriving because the
/// widget asks for another frame each time it draws - but only while it is
/// spinning. A region with a stopped spinner asks for nothing, which is what
/// keeps an idle page's GPU idle.
#[derive(Script, ScriptHook, Widget)]
pub struct RustifySpinner {
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
    /// Whether it is spinning. Off by default: a spinner that starts turning
    /// the moment it exists says something is happening before anything is.
    #[rust]
    spinning: bool,
    /// The scope asked for less movement. The arc is still drawn - it is what
    /// says "busy" - it just does not turn.
    #[rust]
    still: bool,
    #[rust]
    ink: Option<u32>,
    #[rust]
    next_frame: Option<NextFrame>,
}

impl RustifySpinner {
    pub fn set_state(&mut self, cx: &mut Cx, spinning: bool, still: bool) {
        if self.spinning != spinning || self.still != still {
            self.spinning = spinning;
            self.still = still;
            self.redraw(cx);
        }
    }

    pub fn set_ink(&mut self, cx: &mut Cx, ink: u32) {
        if self.ink != Some(ink) {
            self.ink = Some(ink);
            self.redraw(cx);
        }
    }

    fn turning(&self) -> bool {
        self.spinning && !self.still
    }
}

impl Widget for RustifySpinner {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        let Some(next_frame) = self.next_frame else {
            return;
        };
        if next_frame.is_event(event).is_some() {
            self.next_frame = None;
            if self.turning() {
                self.redraw(cx);
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let pane = cx.walk_turtle(walk);
        if let Some(ink) = self.ink {
            self.draw_bg.color = colour(ink);
        }
        // The shader reads the clock and multiplies by this: zero is the same
        // arc, held still.
        let turning = if self.turning() { 1.0 } else { 0.0 };
        self.draw_bg.set_uniform(cx, id!(turning), &[turning]);
        self.draw_bg.draw_abs(cx, pane);
        if self.turning() && self.next_frame.is_none() {
            self.next_frame = Some(cx.new_next_frame());
        }
        DrawStep::done()
    }
}
