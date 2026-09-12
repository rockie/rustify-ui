//! Ten thousand objects, of which the few hundred a camera can see are drawn.
//!
//! Nothing about the objects is stored here. Where they are is arithmetic
//! (`scene_layout`); what this widget owns is the camera it looks through, the
//! pointer that moves it, and the decision not to draw what is off screen.
//! That decision is the whole of load B3: ten thousand quads and ten thousand
//! strings a frame is not a budget anyone could meet, and the text shaper's
//! cache holds four thousand entries, so the labels it is asked for have to be
//! the ones on screen.

use crate::scene_layout;
use rustify_ui::makepad_widgets::widget::*;
use rustify_ui::makepad_widgets::*;
use rustify_ui::Selection;
use std::collections::BTreeMap;
use std::sync::Arc;

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
        draw_chosen +: {
            color: #x2e90fa
        }
        draw_band +: {
            color: #x84caff
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

/// What the pointer is in the middle of doing.
///
/// Which of the two it is was decided when the pointer went down, by what was
/// under it: empty space moves the camera, an object starts a box. Deciding it
/// later - on the first movement, say - would make the same gesture mean
/// different things depending on how fast it began.
#[derive(Clone, Copy, Debug)]
enum Drag {
    /// Started on empty space. The scene follows the pointer, so the camera
    /// moves against it, and both ends are held in window coordinates: the
    /// camera is what is changing, and measuring the movement in scene
    /// coordinates would measure it against itself.
    Pan { from: Vec2d, camera: (f64, f64) },
    /// Started on an object. The corners are in scene coordinates, so the box
    /// keeps hold of the same objects if the camera moves under it.
    Marquee {
        origin: Vec2d,
        to: Vec2d,
        additive: bool,
    },
}

/// A pointer that travelled less than this before it came up was pointing at
/// something rather than drawing a box around it.
const CLICK_SLOP: f64 = 3.0;

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
    draw_chosen: DrawColor,
    #[live]
    draw_band: DrawColor,
    #[live]
    draw_label: DrawText,
    #[rust]
    camera: (f64, f64),
    #[rust]
    highlight: Option<u32>,
    #[rust]
    chosen: Arc<Selection>,
    #[rust]
    labels: Arc<BTreeMap<u32, String>>,
    #[rust]
    drawn: Option<Drawn>,
    #[rust]
    drag: Option<Drag>,
    #[rust]
    hovering: Option<u32>,
    /// What the region has to tell the application, each waiting for the
    /// handler above it to collect it.
    #[rust]
    panned: Option<(f64, f64)>,
    #[rust]
    marqueed: Option<(f64, f64, f64, f64, bool)>,
    #[rust]
    picked: Option<(u32, bool)>,
    #[rust]
    hovered: Option<Option<u32>>,
}

impl SceneView {
    /// Shows what the application says. `camera` is `None` when the
    /// application is only echoing back what this region sent it, which is the
    /// common case and must not move a camera the pointer has since moved on.
    pub fn show(
        &mut self,
        cx: &mut Cx,
        camera: Option<(f64, f64)>,
        highlight: Option<u32>,
        chosen: Arc<Selection>,
        labels: Arc<BTreeMap<u32, String>>,
    ) {
        let moved = camera.is_some_and(|camera| camera != self.camera);
        let changed = highlight != self.highlight
            || !Arc::ptr_eq(&self.chosen, &chosen)
            || !Arc::ptr_eq(&self.labels, &labels);
        if let Some(camera) = camera {
            self.camera = camera;
        }
        self.highlight = highlight;
        self.chosen = chosen;
        self.labels = labels;
        if moved || changed {
            self.redraw(cx);
        }
    }

    pub fn drawn(&self) -> Option<Drawn> {
        self.drawn
    }

    pub fn take_panned(&mut self) -> Option<(f64, f64)> {
        self.panned.take()
    }

    pub fn take_marqueed(&mut self) -> Option<(f64, f64, f64, f64, bool)> {
        self.marqueed.take()
    }

    pub fn take_picked(&mut self) -> Option<(u32, bool)> {
        self.picked.take()
    }

    pub fn take_hovered(&mut self) -> Option<Option<u32>> {
        self.hovered.take()
    }

    /// Where a point of the window is in the scene, by the camera and the pane
    /// of the last draw - which is what the person was looking at when they
    /// put the pointer there.
    fn at(&self, abs: Vec2d) -> Option<Vec2d> {
        let drawn = self.drawn?;
        Some(dvec2(
            drawn.camera.0 + abs.x - drawn.pane.pos.x,
            drawn.camera.1 + abs.y - drawn.pane.pos.y,
        ))
    }

    /// Moves the camera and says so. The region draws the new position at
    /// once rather than waiting for the application to send it back: a stream
    /// that has to make a round trip before it can be seen is a stream a
    /// person watches lag behind their hand.
    fn pan_to(&mut self, cx: &mut Cx, camera: (f64, f64)) {
        if self.camera == camera {
            return;
        }
        self.camera = camera;
        self.panned = Some(camera);
        self.redraw(cx);
    }

    fn note_hover(&mut self, over: Option<u32>) {
        if self.hovering == over {
            return;
        }
        self.hovering = over;
        self.hovered = Some(over);
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
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        match event.hits(cx, self.draw_bg.area()) {
            Hit::FingerScroll(fe) => {
                let camera = (self.camera.0 + fe.scroll.x, self.camera.1 + fe.scroll.y);
                self.pan_to(cx, camera);
            }
            Hit::FingerDown(fe) => {
                if !fe.is_primary_hit() {
                    return;
                }
                let Some(at) = self.at(fe.abs) else {
                    return;
                };
                self.drag = Some(match scene_layout::pick(at.x, at.y) {
                    Some(_) => Drag::Marquee {
                        origin: at,
                        to: at,
                        additive: fe.modifiers.shift,
                    },
                    None => Drag::Pan {
                        from: fe.abs,
                        camera: self.camera,
                    },
                });
            }
            Hit::FingerMove(fe) => match self.drag {
                Some(Drag::Pan { from, camera }) => {
                    let moved = (
                        camera.0 - (fe.abs.x - from.x),
                        camera.1 - (fe.abs.y - from.y),
                    );
                    self.pan_to(cx, moved);
                }
                Some(Drag::Marquee {
                    origin, additive, ..
                }) => {
                    if let Some(to) = self.at(fe.abs) {
                        self.drag = Some(Drag::Marquee {
                            origin,
                            to,
                            additive,
                        });
                        self.redraw(cx);
                    }
                }
                None => {}
            },
            Hit::FingerUp(fe) => {
                if let Some(Drag::Marquee {
                    origin,
                    to,
                    additive,
                }) = self.drag
                {
                    let travelled = (fe.abs - fe.abs_start).length();
                    if travelled <= CLICK_SLOP {
                        // A press and a release in the same place is a person
                        // pointing at one object, and the one they see is the
                        // one on top.
                        if let Some(index) = scene_layout::pick(origin.x, origin.y) {
                            self.picked = Some((index as u32, additive));
                        }
                    } else {
                        self.marqueed = Some((
                            origin.x.min(to.x),
                            origin.y.min(to.y),
                            (to.x - origin.x).abs(),
                            (to.y - origin.y).abs(),
                            additive,
                        ));
                    }
                }
                if self.drag.take().is_some() {
                    self.redraw(cx);
                }
            }
            Hit::FingerHoverIn(fe) | Hit::FingerHoverOver(fe) => {
                let over = self
                    .at(fe.abs)
                    .and_then(|at| scene_layout::pick(at.x, at.y))
                    .map(|index| index as u32);
                self.note_hover(over);
            }
            Hit::FingerHoverOut(_) => self.note_hover(None),
            _ => {}
        }
    }

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
                let renamed = self.labels.get(&(index as u32)).cloned();
                let label = renamed.unwrap_or_else(|| scene_layout::label(index));
                self.draw_label
                    .draw_abs(cx, dvec2(rect.pos.x + 6.0, rect.pos.y + 5.0), &label);
                objects += 1;
            }
        }
        // The selection is drawn over both layers: an object that is chosen
        // has to look chosen even where the layer above it covers its edge.
        for index in window.indices() {
            if self.chosen.contains(index as u32) {
                for edge in Self::ring(place(index), 2.0) {
                    self.draw_chosen.draw_abs(cx, edge);
                }
            }
        }
        if let Some(id) = self.highlight.filter(|id| window.contains(*id as usize)) {
            for edge in Self::ring(place(id as usize), 2.0) {
                self.draw_mark.draw_abs(cx, edge);
            }
        }
        if let Some(Drag::Marquee { origin, to, .. }) = self.drag {
            let box_rect = Rect {
                pos: dvec2(
                    pane.pos.x + origin.x.min(to.x) - camera.0,
                    pane.pos.y + origin.y.min(to.y) - camera.1,
                ),
                size: dvec2((to.x - origin.x).abs(), (to.y - origin.y).abs()),
            };
            for edge in Self::ring(box_rect, 1.0) {
                self.draw_band.draw_abs(cx, edge);
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
