//! A fixed grid of anchors, drawn and hit-tested in the same local space.
//!
//! The region reports where it put each anchor and which one a click landed
//! on. Nothing else knows that geometry, so display and hit can be checked
//! against the same claim rather than against a second copy of the layout.

use rustify_ui::makepad_widgets::widget::*;
use rustify_ui::makepad_widgets::*;

pub const COLUMNS: usize = 5;
pub const ROWS: usize = 4;
/// Gap between two anchors, in local CSS pixels.
const GAP: f64 = 4.0;
/// Side of the marker drawn at an anchor's centre, in local CSS pixels.
const MARKER: f64 = 4.0;

script_mod! {
    use mod.prelude.widgets_internal.*

    mod.widgets.AnchorGridBase = #(AnchorGrid::register_widget(vm))

    mod.widgets.AnchorGrid = set_type_default() do mod.widgets.AnchorGridBase{
        width: Fill
        height: Fill
        draw_bg +: {
            color: #x0c111d
        }
        draw_cell +: {
            color: #x475467
        }
        draw_marker +: {
            color: #xffffff
        }
        draw_ring +: {
            color: #x12b76a
        }
    }
}

/// The colour an anchor has until the application gives it another.
pub const DEFAULT_COLOR: u32 = 0x475467;

/// One anchor as the region drew it, in local CSS pixels.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Anchor {
    pub id: usize,
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AnchorHit {
    pub anchor: usize,
    pub x: f64,
    pub y: f64,
}

#[derive(Script, ScriptHook, Widget)]
pub struct AnchorGrid {
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
    draw_cell: DrawColor,
    #[live]
    draw_marker: DrawColor,
    #[live]
    draw_ring: DrawColor,
    /// One colour per anchor, as the application projected them. Empty until
    /// it does, which is what the default in the script is for.
    #[rust]
    colors: Vec<u32>,
    /// The anchor the application says is selected. The grid does not decide
    /// this: it reports a click and draws what comes back.
    #[rust]
    selected: Option<usize>,
    /// The cell grid of the last draw: a click is resolved against what the
    /// user actually saw.
    #[rust]
    cells: Option<Cells>,
    #[rust]
    hit: Option<AnchorHit>,
    /// Set when the drawn geometry changed, taken by the region.
    #[rust]
    layout_report: Option<Vec<Anchor>>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Cells {
    origin: DVec2,
    cell: DVec2,
}

impl Cells {
    fn anchor(&self, id: usize) -> Anchor {
        let column = id % COLUMNS;
        let row = id / COLUMNS;
        Anchor {
            id,
            x: self.origin.x + column as f64 * self.cell.x + GAP * 0.5,
            y: self.origin.y + row as f64 * self.cell.y + GAP * 0.5,
            width: self.cell.x - GAP,
            height: self.cell.y - GAP,
        }
    }

    fn id_at(&self, point: DVec2) -> Option<usize> {
        let column = ((point.x - self.origin.x) / self.cell.x).floor();
        let row = ((point.y - self.origin.y) / self.cell.y).floor();
        if column < 0.0 || row < 0.0 || column >= COLUMNS as f64 || row >= ROWS as f64 {
            return None;
        }
        Some(row as usize * COLUMNS + column as usize)
    }
}

impl AnchorGrid {
    pub fn take_hit(&mut self) -> Option<AnchorHit> {
        self.hit.take()
    }

    pub fn set_projection(&mut self, cx: &mut Cx, colors: &[u32], selected: Option<usize>) {
        if self.colors != colors || self.selected != selected {
            self.colors = colors.to_vec();
            self.selected = selected;
            self.redraw(cx);
        }
    }

    fn color_of(&self, id: usize) -> Vec4f {
        let rgb = self.colors.get(id).copied().unwrap_or(DEFAULT_COLOR);
        Vec4f::from_u32(rgb << 8 | 0xff)
    }

    pub fn take_layout(&mut self) -> Option<Vec<Anchor>> {
        self.layout_report.take()
    }
}

fn rect_of(anchor: &Anchor) -> Rect {
    Rect {
        pos: dvec2(anchor.x, anchor.y),
        size: dvec2(anchor.width, anchor.height),
    }
}

impl Widget for AnchorGrid {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        let Some(cells) = self.cells else {
            return;
        };
        if let Hit::FingerUp(fe) = event.hits(cx, self.draw_bg.area()) {
            if !fe.is_over || !fe.is_primary_hit() {
                return;
            }
            if let Some(anchor) = cells.id_at(fe.abs) {
                self.hit = Some(AnchorHit {
                    anchor,
                    x: fe.abs.x - cells.origin.x,
                    y: fe.abs.y - cells.origin.y,
                });
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let pane = cx.walk_turtle(walk);
        self.draw_bg.draw_abs(cx, pane);
        if pane.size.x < COLUMNS as f64 || pane.size.y < ROWS as f64 {
            // Nothing that could carry an anchor: report the absence rather
            // than a grid of degenerate rectangles.
            if self.cells.take().is_some() {
                self.layout_report = Some(Vec::new());
            }
            return DrawStep::done();
        }

        let cells = Cells {
            origin: pane.pos,
            cell: dvec2(pane.size.x / COLUMNS as f64, pane.size.y / ROWS as f64),
        };
        let anchors: Vec<Anchor> = (0..COLUMNS * ROWS).map(|id| cells.anchor(id)).collect();
        for anchor in &anchors {
            self.draw_cell.color = self.color_of(anchor.id);
            self.draw_cell.draw_abs(cx, rect_of(anchor));
            self.draw_marker.draw_abs(
                cx,
                Rect {
                    pos: dvec2(
                        anchor.x + anchor.width * 0.5 - MARKER * 0.5,
                        anchor.y + anchor.height * 0.5 - MARKER * 0.5,
                    ),
                    size: dvec2(MARKER, MARKER),
                },
            );
        }
        // The selection is drawn last: it is the one thing on the grid that
        // says which anchor the next command will act on.
        if let Some(anchor) = self.selected.and_then(|id| anchors.get(id)) {
            let rect = rect_of(anchor);
            let thickness = 2.0;
            for edge in [
                Rect {
                    pos: rect.pos,
                    size: dvec2(rect.size.x, thickness),
                },
                Rect {
                    pos: dvec2(rect.pos.x, rect.pos.y + rect.size.y - thickness),
                    size: dvec2(rect.size.x, thickness),
                },
                Rect {
                    pos: rect.pos,
                    size: dvec2(thickness, rect.size.y),
                },
                Rect {
                    pos: dvec2(rect.pos.x + rect.size.x - thickness, rect.pos.y),
                    size: dvec2(thickness, rect.size.y),
                },
            ] {
                self.draw_ring.draw_abs(cx, edge);
            }
        }
        if self.cells != Some(cells) {
            self.cells = Some(cells);
            self.layout_report = Some(anchors);
        }
        DrawStep::done()
    }
}
