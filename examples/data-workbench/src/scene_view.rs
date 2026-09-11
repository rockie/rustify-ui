//! Ten thousand objects, of which the few hundred a camera can see are drawn.
//!
//! Nothing about the objects is stored here. Where they are is arithmetic
//! (`scene_layout`); what this widget owns is the camera it looks through and
//! the decision not to draw what is off screen. That decision is the whole of
//! load B3: ten thousand quads and ten thousand strings a frame is not a
//! budget anyone could meet, and the text shaper's cache holds four thousand
//! entries, so the labels it is asked for have to be the ones on screen.

use crate::scene_layout;
use rustify_ui::makepad_widgets::widget::*;
use rustify_ui::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets_internal.*

    mod.widgets.SceneViewBase = #(SceneView::register_widget(vm))

    mod.widgets.SceneView = set_type_default() do mod.widgets.SceneViewBase{
        width: Fill
        height: Fill
        draw_bg +: {
            color: #x0c111d
        }
        draw_rect +: {
            color: #x475467
        }
        draw_overlay +: {
            color: #x667085
        }
        draw_mark +: {
            color: #x12b76a
        }
        draw_label +: {
            color: #xf2f4f7
            text_style: theme.font_regular{ font_size: 8.0 }
        }
    }
}

/// What the last draw actually put on screen.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Drawn {
    pub camera: (f64, f64),
    pub pane: Rect,
    pub objects: usize,
}

#[derive(Script, ScriptHook, Widget)]
pub struct SceneView {
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
    draw_rect: DrawColor,
    #[live]
    draw_overlay: DrawColor,
    #[live]
    draw_mark: DrawColor,
    #[live]
    draw_label: DrawText,
    #[rust]
    camera: (f64, f64),
    #[rust]
    highlight: Option<u32>,
    #[rust]
    drawn: Option<Drawn>,
}

impl SceneView {
    pub fn look_at(&mut self, cx: &mut Cx, camera: (f64, f64), highlight: Option<u32>) {
        if self.camera == camera && self.highlight == highlight {
            return;
        }
        self.camera = camera;
        self.highlight = highlight;
        self.redraw(cx);
    }

    pub fn drawn(&self) -> Option<Drawn> {
        self.drawn
    }

    /// The four thin quads that outline an object without covering it.
    fn ring(rect: Rect, thickness: f64) -> [Rect; 4] {
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
}

impl Widget for SceneView {
    fn handle_event(&mut self, _cx: &mut Cx, _event: &Event, _scope: &mut Scope) {}

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let pane = cx.walk_turtle(walk);
        self.draw_bg.draw_abs(cx, pane);
        let camera = self.camera;
        if pane.size.x < 8.0 || pane.size.y < 8.0 {
            self.drawn = Some(Drawn {
                camera,
                pane,
                objects: 0,
            });
            return DrawStep::done();
        }
        let window = scene_layout::visible(camera.0, camera.1, pane.size.x, pane.size.y);
        let place = |index: usize| {
            let object = scene_layout::rect(index);
            Rect {
                pos: dvec2(
                    pane.pos.x + object.x - camera.0,
                    pane.pos.y + object.y - camera.1,
                ),
                size: dvec2(scene_layout::WIDTH, scene_layout::HEIGHT),
            }
        };
        let mut objects = 0;
        // Two passes, because the shifted objects are the upper layer and a
        // layer is an order of drawing.
        for upper in [false, true] {
            for index in window.indices() {
                if scene_layout::is_overlay(index) != upper {
                    continue;
                }
                let rect = place(index);
                if upper {
                    self.draw_overlay.draw_abs(cx, rect);
                } else {
                    self.draw_rect.draw_abs(cx, rect);
                }
                self.draw_label.draw_abs(
                    cx,
                    dvec2(rect.pos.x + 6.0, rect.pos.y + 5.0),
                    &scene_layout::label(index),
                );
                objects += 1;
            }
        }
        if let Some(id) = self.highlight.filter(|id| window.contains(*id as usize)) {
            for edge in Self::ring(place(id as usize), 2.0) {
                self.draw_mark.draw_abs(cx, edge);
            }
        }
        self.drawn = Some(Drawn {
            camera,
            pane,
            objects,
        });
        DrawStep::done()
    }
}
