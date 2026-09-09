//! A grid of every object, drawn as one quad each and picked by clicking.
//!
//! The grid holds no business data: it is handed a projection of the objects
//! before each draw and reports which cell the pointer chose. The application
//! decides what a pick means.

use rustify_ui::makepad_widgets::widget::*;
use rustify_ui::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets_internal.*

    mod.widgets.ObjectGridBase = #(ObjectGrid::register_widget(vm))

    mod.widgets.ObjectGrid = set_type_default() do mod.widgets.ObjectGridBase{
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
        draw_hover +: {
            color: #x98a2b3
        }
    }
}

/// One object as the grid needs to draw it.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GridCell {
    pub id: u32,
    /// `0xRRGGBB`.
    pub color: u32,
}

#[derive(Script, ScriptHook, Widget)]
pub struct ObjectGrid {
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
    draw_hover: DrawColor,
    #[rust]
    cells: Vec<GridCell>,
    #[rust]
    selected: Option<u32>,
    #[rust]
    hovered: Option<u32>,
    /// Where the cells ended up in the last draw, so a click can be resolved
    /// against what the user actually saw.
    #[rust]
    layout_grid: Option<Grid>,
    /// The last cell the pointer chose, taken by the application.
    #[rust]
    picked: Option<u32>,
    /// The cell the pointer last moved over, or `None` once it left the grid.
    /// Set on every move and taken by the application, which decides whether
    /// it is worth acting on.
    #[rust]
    hover_report: Option<Option<u32>>,
}

#[derive(Clone, Copy, Debug)]
struct Grid {
    origin: DVec2,
    cell: DVec2,
    columns: usize,
}

impl Grid {
    fn rect(&self, index: usize) -> Rect {
        let column = index % self.columns;
        let row = index / self.columns;
        Rect {
            pos: dvec2(
                self.origin.x + column as f64 * self.cell.x,
                self.origin.y + row as f64 * self.cell.y,
            ),
            size: dvec2(self.cell.x - 1.0, self.cell.y - 1.0),
        }
    }

    /// The four thin quads that outline `index` without covering the cell's
    /// own colour.
    fn ring(&self, index: usize, thickness: f64) -> [Rect; 4] {
        let rect = self.rect(index);
        [
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
        ]
    }

    fn index_at(&self, point: DVec2, len: usize) -> Option<usize> {
        let column = ((point.x - self.origin.x) / self.cell.x).floor();
        let row = ((point.y - self.origin.y) / self.cell.y).floor();
        if column < 0.0 || row < 0.0 || column >= self.columns as f64 {
            return None;
        }
        let index = row as usize * self.columns + column as usize;
        (index < len).then_some(index)
    }
}

impl ObjectGrid {
    /// Refuses a list that names the same object twice: a click could not say
    /// which of them it meant, so the grid keeps what it already had and
    /// reports the ambiguous id instead of picking a winner.
    pub fn set_cells(
        &mut self,
        cx: &mut Cx,
        cells: Vec<GridCell>,
        selected: Option<u32>,
        hovered: Option<u32>,
    ) -> Result<(), u32> {
        if let Some(duplicate) = rustify_ui::duplicate_key(cells.iter().map(|cell| cell.id)) {
            return Err(duplicate);
        }
        if self.cells != cells || self.selected != selected || self.hovered != hovered {
            self.cells = cells;
            self.selected = selected;
            self.hovered = hovered;
            self.redraw(cx);
        }
        Ok(())
    }

    pub fn take_picked(&mut self) -> Option<u32> {
        self.picked.take()
    }

    pub fn take_hover_report(&mut self) -> Option<Option<u32>> {
        self.hover_report.take()
    }

    fn cell_at(&self, grid: &Grid, point: DVec2) -> Option<u32> {
        grid.index_at(point, self.cells.len())
            .map(|index| self.cells[index].id)
    }
}

fn cell_color(rgb: u32) -> Vec4f {
    Vec4f::from_u32(rgb << 8 | 0xff)
}

impl Widget for ObjectGrid {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        let Some(grid) = self.layout_grid else {
            return;
        };
        match event.hits(cx, self.draw_bg.area()) {
            Hit::FingerUp(fe) => {
                if !fe.is_over || !fe.is_primary_hit() {
                    return;
                }
                if let Some(id) = self.cell_at(&grid, fe.abs) {
                    self.picked = Some(id);
                }
            }
            Hit::FingerHoverIn(fe) | Hit::FingerHoverOver(fe) => {
                self.hover_report = Some(self.cell_at(&grid, fe.abs));
            }
            Hit::FingerHoverOut(_) => {
                self.hover_report = Some(None);
            }
            _ => {}
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let pane = cx.walk_turtle(walk);
        self.draw_bg.draw_abs(cx, pane);
        self.layout_grid = None;
        if self.cells.is_empty() || pane.size.x < 8.0 || pane.size.y < 8.0 {
            return DrawStep::done();
        }

        // Square-ish cells: enough columns that every object fits the pane.
        let count = self.cells.len() as f64;
        let ratio = pane.size.x / pane.size.y;
        let columns = (count * ratio).sqrt().ceil().max(1.0);
        let rows = (count / columns).ceil().max(1.0);
        let cell = dvec2(pane.size.x / columns, pane.size.y / rows);
        let grid = Grid {
            origin: pane.pos,
            cell,
            columns: columns as usize,
        };

        for (index, entry) in self.cells.iter().enumerate() {
            let rect = grid.rect(index);
            self.draw_cell.color = cell_color(entry.color);
            self.draw_cell.draw_abs(cx, rect);
        }
        let thickness = (cell.x.min(cell.y) * 0.25).clamp(1.0, 4.0);
        let index_of =
            |id: Option<u32>| id.and_then(|id| self.cells.iter().position(|cell| cell.id == id));
        let hovered = index_of(self.hovered);
        let selected = index_of(self.selected);
        // The pointer's cell first, so the selection still reads as the
        // stronger of the two when they are the same cell.
        if let Some(index) = hovered {
            for edge in grid.ring(index, thickness) {
                self.draw_hover.draw_abs(cx, edge);
            }
        }
        if let Some(index) = selected {
            for edge in grid.ring(index, thickness) {
                self.draw_marker.draw_abs(cx, edge);
            }
        }
        self.layout_grid = Some(grid);
        DrawStep::done()
    }
}
