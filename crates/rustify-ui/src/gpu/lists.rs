//! The two experimental ones: a closed chooser, and a strip of tabs.
//!
//! Both are marked experimental in the catalogue for the same reason. What a
//! region can draw is the control; what it cannot draw is the list that opens
//! out of it, because a popup that has to escape the canvas is a DOM layer
//! (P1 D6, and F16's note that menus and modals inside a region are not
//! applicable). So each of these draws the closed state and asks the
//! application to open the DOM half against the rectangle it reports.

use crate::makepad_widgets::widget::*;
use crate::makepad_widgets::*;

use super::colour;

/// A chooser in its closed state, drawn by a region.
#[derive(Script, ScriptHook, Widget)]
pub struct RustifyDropDown {
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
    draw_chevron: DrawColor,
    #[live]
    draw_text: DrawText,
    /// What the application holds, spelled the way it wants it read. The
    /// widget never edits it.
    #[rust]
    label: String,
    #[rust]
    disabled: bool,
    #[rust]
    read_only: bool,
    /// Face, border, text and chevron, from the scope's theme.
    #[rust]
    palette: Option<(u32, u32, u32, u32)>,
    /// The user asked for the list. The application opens the DOM one.
    #[rust]
    open: Option<()>,
}

impl RustifyDropDown {
    pub fn set_state(&mut self, cx: &mut Cx, label: &str, disabled: bool, read_only: bool) {
        if self.label != label || self.disabled != disabled || self.read_only != read_only {
            self.label = label.to_string();
            self.disabled = disabled;
            self.read_only = read_only;
            self.redraw(cx);
        }
    }

    pub fn set_palette(&mut self, cx: &mut Cx, face: u32, border: u32, text: u32, chevron: u32) {
        let next = Some((face, border, text, chevron));
        if self.palette != next {
            self.palette = next;
            self.redraw(cx);
        }
    }

    pub fn take_open(&mut self) -> Option<()> {
        self.open.take()
    }

    /// Where it ended up, in the region's own local pixels. This is what the
    /// application anchors the DOM list to.
    pub fn drawn(&self, cx: &Cx) -> Option<Rect> {
        let area = self.draw_bg.area();
        (!area.is_empty()).then(|| area.rect(cx))
    }
}

impl Widget for RustifyDropDown {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        if let Hit::FingerUp(fe) = event.hits(cx, self.draw_bg.area()) {
            if !fe.is_over || !fe.is_primary_hit() {
                return;
            }
            // A read-only chooser still opens: reading the options is not
            // changing the value, and closing the list is the only way to
            // find out what the value means.
            if self.disabled {
                return;
            }
            self.open = Some(());
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let pane = cx.walk_turtle(walk);
        if let Some((face, border, text, chevron)) = self.palette {
            self.draw_bg.color = colour(face);
            self.draw_border.color = colour(border);
            self.draw_text.color = colour(text);
            self.draw_chevron.color = colour(chevron);
        }
        self.draw_border.draw_abs(cx, pane);
        self.draw_bg
            .draw_abs(cx, super::toggles::inset_rect(pane, 1.0));
        let chevron = pane.size.y * 0.6;
        self.draw_chevron.draw_abs(
            cx,
            Rect {
                pos: dvec2(
                    pane.pos.x + pane.size.x - chevron - 6.0,
                    pane.pos.y + (pane.size.y - chevron) * 0.5,
                ),
                size: dvec2(chevron, chevron),
            },
        );
        self.draw_text.draw_abs(
            cx,
            dvec2(pane.pos.x + 10.0, pane.pos.y + pane.size.y * 0.5),
            &self.label,
        );
        DrawStep::done()
    }
}

/// A strip of tabs, drawn by a region.
///
/// One widget rather than one per tab: which tab is where has to be one
/// answer, and a strip whose tabs each decided their own width would report
/// rectangles that do not add up to the strip.
#[derive(Script, ScriptHook, Widget)]
pub struct RustifyTabBar {
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
    draw_tab: DrawColor,
    #[live]
    draw_text: DrawText,
    #[rust]
    tabs: Vec<String>,
    /// Which one the application holds as active. Out of range means none,
    /// which is what an empty strip and a strip whose active tab just closed
    /// both look like.
    #[rust]
    active: usize,
    #[rust]
    disabled: bool,
    /// Strip, active tab and text, from the scope's theme.
    #[rust]
    palette: Option<(u32, u32, u32)>,
    /// Where each tab was last drawn, so a click can be turned into an index
    /// without a second copy of the layout.
    #[rust]
    rects: Vec<Rect>,
    #[rust]
    change: Option<usize>,
}

impl RustifyTabBar {
    pub fn set_state(&mut self, cx: &mut Cx, tabs: &[String], active: usize, disabled: bool) {
        if self.tabs != tabs || self.active != active || self.disabled != disabled {
            self.tabs = tabs.to_vec();
            self.active = active;
            self.disabled = disabled;
            self.redraw(cx);
        }
    }

    pub fn set_palette(&mut self, cx: &mut Cx, strip: u32, active: u32, text: u32) {
        let next = Some((strip, active, text));
        if self.palette != next {
            self.palette = next;
            self.redraw(cx);
        }
    }

    pub fn take_change(&mut self) -> Option<usize> {
        self.change.take()
    }

    /// Where one tab ended up, in the region's own local pixels.
    pub fn tab_drawn(&self, index: usize) -> Option<Rect> {
        self.rects.get(index).copied()
    }

    /// Where the whole strip ended up.
    pub fn drawn(&self, cx: &Cx) -> Option<Rect> {
        let area = self.draw_bg.area();
        (!area.is_empty()).then(|| area.rect(cx))
    }
}

impl Widget for RustifyTabBar {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        if let Hit::FingerUp(fe) = event.hits(cx, self.draw_bg.area()) {
            if !fe.is_over || !fe.is_primary_hit() || self.disabled {
                return;
            }
            let hit = self
                .rects
                .iter()
                .position(|rect| rect.contains(fe.abs.into()));
            // Activating the tab already showing is not a change; the strip
            // reports what the application does not already know.
            if let Some(hit) = hit.filter(|hit| *hit != self.active) {
                self.change = Some(hit);
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let pane = cx.walk_turtle(walk);
        if let Some((strip, active, text)) = self.palette {
            self.draw_bg.color = colour(strip);
            self.draw_tab.color = colour(active);
            self.draw_text.color = colour(text);
        }
        self.draw_bg.draw_abs(cx, pane);
        self.rects.clear();
        if self.tabs.is_empty() {
            return DrawStep::done();
        }
        let pad = 3.0;
        let width = ((pane.size.x - pad * 2.0) / self.tabs.len() as f64).max(0.0);
        let height = (pane.size.y - pad * 2.0).max(0.0);
        for index in 0..self.tabs.len() {
            let rect = Rect {
                pos: dvec2(pane.pos.x + pad + width * index as f64, pane.pos.y + pad),
                size: dvec2(width, height),
            };
            self.rects.push(rect);
            if index == self.active {
                self.draw_tab.draw_abs(cx, rect);
            }
        }
        // The faces are drawn before any of the text so that one draw call
        // carries the strip and one carries the words.
        for (index, label) in self.tabs.iter().enumerate() {
            let rect = self.rects[index];
            self.draw_text.draw_abs(
                cx,
                dvec2(rect.pos.x + 8.0, rect.pos.y + rect.size.y * 0.5),
                label,
            );
        }
        DrawStep::done()
    }
}
