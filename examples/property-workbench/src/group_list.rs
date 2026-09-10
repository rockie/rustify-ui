//! The groups an object can be dropped into, in a column that scrolls.
//!
//! Two jobs that belong together because they are the same fact drawn once:
//! the rows are the drop targets a cross-region drag aims at, and they are
//! also the thing that runs out of room and so has to tell the host where its
//! scrolling ends. A caller asking "what is at this point?" and the host
//! asking "is this wheel mine?" both get answered from the layout of the last
//! draw, which is the only version the user has actually seen.

use rustify_ui::makepad_widgets::makepad_platform::ScrollBoundary;
use rustify_ui::makepad_widgets::widget::*;
use rustify_ui::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets_internal.*

    mod.widgets.GroupListBase = #(GroupList::register_widget(vm))

    mod.widgets.GroupList = set_type_default() do mod.widgets.GroupListBase{
        width: 150
        height: Fill
        draw_bg +: {
            color: #x0c111d
        }
        draw_row +: {
            color: #x1d2939
        }
        draw_target +: {
            color: #x2e90fa
        }
        draw_text +: {
            color: #xd0d5dd
            text_style: theme.font_regular{ font_size: 11.0 }
        }
    }
}

/// One group as the list needs to draw it. The list holds no membership: the
/// count is a number the application computed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Group {
    pub id: u32,
    pub name: String,
    pub members: usize,
}

/// The height of one row, and so the unit everything here is measured in.
const ROW: f64 = 30.0;

#[derive(Script, ScriptHook, Widget)]
pub struct GroupList {
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
    draw_row: DrawColor,
    #[live]
    draw_target: DrawColor,
    #[live]
    draw_text: DrawText,
    #[rust]
    groups: Vec<Group>,
    /// How far down the list is, in pixels, and how far down it can go. The
    /// second is a fact of the last draw: it depends on how tall the pane
    /// turned out to be.
    #[rust]
    scroll: f64,
    #[rust]
    max_scroll: f64,
    /// The row a drag is currently over, drawn differently so the user can see
    /// where a release would land. A projection, like everything else here.
    #[rust]
    target: Option<u32>,
    /// Where the rows ended up, so a point can be resolved against what the
    /// user saw rather than against what the next draw will show.
    #[rust]
    pane: Option<Rect>,
    /// What the region wants done with a wheel that arrives at the end of this
    /// list: hand it to the page, or keep it.
    #[rust]
    propagate: bool,
    /// The boundary last reported, so an unchanged one is not resent on every
    /// draw. The host keeps the last report; repeating it says nothing.
    #[rust]
    reported: Option<ScrollBoundary>,
}

impl GroupList {
    pub fn set_groups(&mut self, cx: &mut Cx, groups: Vec<Group>) {
        if self.groups != groups {
            self.groups = groups;
            self.redraw(cx);
        }
    }

    pub fn set_target(&mut self, cx: &mut Cx, target: Option<u32>) {
        if self.target != target {
            self.target = target;
            self.redraw(cx);
        }
    }

    /// Sets what a wheel at the end of the list is for.
    ///
    /// Changing it redraws, because the report the host reads is only sent
    /// from a draw: a policy the host has not been told about is not a policy.
    pub fn set_propagate(&mut self, cx: &mut Cx, propagate: bool) {
        if self.propagate != propagate {
            self.propagate = propagate;
            self.redraw(cx);
        }
    }

    /// Which group is at `point`, in the region's own coordinates.
    ///
    /// `None` for a point outside the list or on empty space below the last
    /// row - both mean the same thing to a drag, which is that releasing here
    /// puts the object nowhere.
    pub fn group_at(&self, point: DVec2) -> Option<u32> {
        let pane = self.pane?;
        if !pane.contains(point) {
            return None;
        }
        let index = ((point.y - pane.pos.y + self.scroll) / ROW).floor();
        if index < 0.0 {
            return None;
        }
        self.groups.get(index as usize).map(|group| group.id)
    }

    /// The scroll position, for a test or a caller that wants to know how far
    /// down the list is without reading pixels off the screen.
    pub fn scroll(&self) -> (f64, f64) {
        (self.scroll, self.max_scroll)
    }

    /// Where the list was drawn, in the region's own coordinates.
    ///
    /// Reported outwards for the same reason the second row's controls are: a
    /// caller that has to put a pointer on a group cannot guess at the layout,
    /// and every time one has tried in this project it broke the first time a
    /// panel changed width.
    pub fn pane(&self) -> Option<Rect> {
        self.pane
    }

    /// How tall one row is. A caller aiming at the first group needs it, and
    /// it is the list's number rather than anybody else's.
    pub fn row_height(&self) -> f64 {
        ROW
    }

    fn boundary(&self) -> ScrollBoundary {
        ScrollBoundary::vertical(self.scroll, self.max_scroll, self.propagate)
    }
}

impl Widget for GroupList {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        let Event::Scroll(wheel) = event else {
            return;
        };
        // Tested against the rectangle this list drew, rather than through the
        // widget's draw area. An area reads as an empty rectangle whenever its
        // draw list has been redrawn since the instance was written, and this
        // region asks for a redraw on every projection - so the ordinary hit
        // test answers "nowhere" for a list that is plainly on the screen. The
        // rectangle it drew is the one the user saw, which is the one a wheel
        // over it should scroll.
        let Some(pane) = self.pane else {
            return;
        };
        if !pane.contains(wheel.abs) || self.max_scroll <= 0.0 {
            return;
        }
        let next = (self.scroll + wheel.scroll.y).clamp(0.0, self.max_scroll);
        if next != self.scroll {
            self.scroll = next;
            self.redraw(cx);
        }
        // Taken here, so an ancestor scroller does not move as well.
        wheel.handled_y.set(true);
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        // Clipped, because a row scrolled past the end of the pane would
        // otherwise draw over the column beside it.
        cx.begin_turtle(walk, self.layout.with_clip(true, true));
        let pane = cx.turtle().rect();
        self.draw_bg.draw_abs(cx, pane);

        let content = self.groups.len() as f64 * ROW;
        self.max_scroll = (content - pane.size.y).max(0.0);
        self.scroll = self.scroll.clamp(0.0, self.max_scroll);

        for (index, group) in self.groups.iter().enumerate() {
            let top = pane.pos.y + index as f64 * ROW - self.scroll;
            // A row entirely outside the pane is not drawn. Clipping would
            // hide it anyway; not drawing it keeps a long list from filling
            // the draw call with invisible quads.
            if top + ROW < pane.pos.y || top > pane.pos.y + pane.size.y {
                continue;
            }
            let rect = Rect {
                pos: dvec2(pane.pos.x + 4.0, top + 2.0),
                size: dvec2((pane.size.x - 8.0).max(0.0), ROW - 4.0),
            };
            if self.target == Some(group.id) {
                self.draw_target.draw_abs(cx, rect);
            } else {
                self.draw_row.draw_abs(cx, rect);
            }
            self.draw_text.draw_abs(
                cx,
                dvec2(rect.pos.x + 8.0, rect.pos.y + 6.0),
                &format!("{} ({})", group.name, group.members),
            );
        }
        self.pane = Some(pane);
        cx.end_turtle();

        // After the draw, because only now is the answer a fact: the pane's
        // height is what the layout just decided, not what it was last frame.
        let boundary = self.boundary();
        if self.reported != Some(boundary) {
            self.reported = Some(boundary);
            cx.report_scroll_boundary(boundary);
        }
        DrawStep::done()
    }
}
