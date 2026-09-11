//! A thousand-bucket picture of where the selection is.
//!
//! A hundred thousand rows do not fit on a scrollbar, and a person who has
//! selected two hundred of them has no way to see where they are. The strip is
//! the whole table squeezed into its own width: one bucket per pixel column,
//! drawn as tall as the share of its rows that are selected, with the part the
//! table is showing marked on it.

use rustify_ui::makepad_widgets::widget::*;
use rustify_ui::makepad_widgets::*;
use std::sync::Arc;

/// Buckets across the strip. One per pixel column at the widths this view uses.
pub const BUCKETS: usize = 1_000;

script_mod! {
    use mod.prelude.widgets_internal.*

    mod.widgets.SelectionStripBase = #(SelectionStrip::register_widget(vm))

    mod.widgets.SelectionStrip = set_type_default() do mod.widgets.SelectionStripBase{
        width: Fill
        height: Fill
        draw_bg +: {
            color: #x101828
        }
        draw_bar +: {
            color: #x2e90fa
        }
        draw_viewport +: {
            color: #x475467
        }
    }
}

#[derive(Script, ScriptHook, Widget)]
pub struct SelectionStrip {
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
    draw_bar: DrawColor,
    #[live]
    draw_viewport: DrawColor,
    #[rust]
    buckets: Arc<Vec<u16>>,
    #[rust]
    peak: u16,
    #[rust]
    viewport: (usize, usize),
    /// The pane of the last draw, so a click is resolved against what the
    /// person actually saw rather than against the current layout.
    #[rust]
    pane: Option<Rect>,
    #[rust]
    picked: Option<usize>,
}

impl SelectionStrip {
    pub fn show(
        &mut self,
        cx: &mut Cx,
        buckets: Arc<Vec<u16>>,
        peak: u16,
        viewport: (usize, usize),
    ) {
        if self.peak == peak && self.viewport == viewport && Arc::ptr_eq(&self.buckets, &buckets) {
            return;
        }
        self.buckets = buckets;
        self.peak = peak;
        self.viewport = viewport;
        self.redraw(cx);
    }

    pub fn take_picked(&mut self) -> Option<usize> {
        self.picked.take()
    }
}

impl Widget for SelectionStrip {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        let Some(pane) = self.pane else {
            return;
        };
        if let Hit::FingerUp(fe) = event.hits(cx, self.draw_bg.area()) {
            if !fe.is_over || !fe.is_primary_hit() || pane.size.x <= 0.0 {
                return;
            }
            let across = ((fe.abs.x - pane.pos.x) / pane.size.x).clamp(0.0, 1.0);
            self.picked = Some(((across * BUCKETS as f64) as usize).min(BUCKETS - 1));
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let pane = cx.walk_turtle(walk);
        self.draw_bg.draw_abs(cx, pane);
        self.pane = Some(pane);
        if pane.size.x < 4.0 || pane.size.y < 4.0 {
            return DrawStep::done();
        }
        let width = pane.size.x / BUCKETS as f64;
        // The part of the table on screen, drawn under the bars so a selected
        // row inside it is still the thing you see.
        let (start, end) = self.viewport;
        if end > start {
            let left = pane.pos.x + start as f64 * width;
            self.draw_viewport.draw_abs(
                cx,
                Rect {
                    pos: dvec2(left, pane.pos.y),
                    size: dvec2(((end - start) as f64 * width).max(1.0), pane.size.y),
                },
            );
        }
        let peak = self.peak.max(1) as f64;
        for (index, count) in self.buckets.iter().enumerate() {
            if *count == 0 {
                continue;
            }
            // At least a pixel: a bucket with one selected row in a thousand
            // has to be visible, or the strip lies about where the selection
            // is.
            let height = (pane.size.y * (*count as f64 / peak)).max(1.0);
            self.draw_bar.draw_abs(
                cx,
                Rect {
                    pos: dvec2(
                        pane.pos.x + index as f64 * width,
                        pane.pos.y + pane.size.y - height,
                    ),
                    size: dvec2(width.max(1.0), height),
                },
            );
        }
        DrawStep::done()
    }
}
