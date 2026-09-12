//! The scene as a region: one camera, one pointer, and a report of what it drew.

use crate::scene_view::SceneView;
use rustify_ui::makepad_widgets::*;
use rustify_ui::{Pace, RegionApp, Selection};
use std::collections::BTreeMap;
use std::sync::Arc;

/// What the region is looking at.
///
/// The camera is an absolute position rather than a movement: a continuous
/// stream keeps only its newest value, so a stream of movements would lose
/// whatever was overwritten, while the newest position is always the right
/// answer.
#[derive(Clone, Debug, PartialEq)]
pub struct SceneProps {
    pub camera: (f64, f64),
    pub highlight: Option<u32>,
    /// What is selected, and what has been renamed. Shared rather than
    /// copied: a marquee can take thousands of objects, and a camera that
    /// moves sixty times a second must not copy the set each time.
    pub chosen: Arc<Selection>,
    pub labels: Arc<BTreeMap<u32, String>>,
    /// Stops the region changing what it presents, while everything else
    /// about it goes on as before.
    ///
    /// It is here for the frame budget to prove itself with: a budget that
    /// samples animation frames rather than presentations passes at a steady
    /// 16.7 ms while the picture stands still, so the gate runs against a
    /// frozen scene first and has to fail.
    pub frozen: bool,
}

#[derive(Debug)]
pub enum SceneAction {
    /// Where the region ended up looking and how many objects that came to.
    /// Reported after a draw, because until then it is a guess.
    Viewport {
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        drawn: usize,
    },
    /// Where the pointer moved the camera to, in scene coordinates.
    Camera { x: f64, y: f64 },
    /// A box was drawn over the scene. Scene coordinates, so it means the same
    /// thing however the camera moved while it was being drawn.
    Marquee {
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        additive: bool,
    },
    /// One object was pointed at.
    Pick { id: u32, additive: bool },
    /// What the pointer is over now, or nothing.
    Hover(Option<u32>),
}

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    startup() do #(SceneRegion::script_component(vm)){
        ui: Root{
            main_window := Window{
                body +: {
                    scene := SceneView{
                        width: Fill
                        height: Fill
                    }
                }
            }
        }
    }
}

#[derive(Script, ScriptHook)]
pub struct SceneRegion {
    #[live]
    ui: WidgetRef,
    /// The last viewport reported, so a redraw that changed nothing reports
    /// nothing.
    #[rust]
    reported: Option<(f64, f64, f64, f64, usize)>,
    /// The camera position this region last sent. A projection carrying that
    /// same position is the application echoing the region back to itself and
    /// must not move a camera the pointer has since moved on; anything else -
    /// a clamp, a find - is the application's answer and is taken.
    #[rust]
    sent: Option<(f64, f64)>,
}

impl RegionApp for SceneRegion {
    type Props = SceneProps;
    type Action = SceneAction;

    fn pace(action: &SceneAction) -> Pace {
        match action {
            // Streams about where things are: only the newest of each can
            // still be true, and each supersedes only itself.
            SceneAction::Viewport { .. } => Pace::Continuous("viewport"),
            SceneAction::Camera { .. } => Pace::Continuous("pan"),
            SceneAction::Hover(_) => Pace::Continuous("hover"),
            // Each of these is a decision a person made, and dropping one
            // changes what they selected.
            SceneAction::Marquee { .. } | SceneAction::Pick { .. } => Pace::Discrete,
        }
    }

    fn script_mod(vm: &mut ScriptVm) -> ScriptValue {
        rustify_ui::makepad_widgets::script_mod(vm);
        crate::scene_view::script_mod(vm);
        self::script_mod(vm)
    }

    fn apply_props(&mut self, cx: &mut Cx, props: &SceneProps) {
        if props.frozen {
            return;
        }
        let echo = self.sent == Some(props.camera);
        self.sent = Some(props.camera);
        if let Some(mut scene) = self.ui.widget(cx, ids!(scene)).borrow_mut::<SceneView>() {
            scene.show(
                cx,
                (!echo).then_some(props.camera),
                props.highlight,
                props.chosen.clone(),
                props.labels.clone(),
            );
        }
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, outbox: &mut Vec<SceneAction>) {
        self.ui.handle_event(cx, event, &mut Scope::empty());
        let widget = self.ui.widget(cx, ids!(scene));
        let Some(mut scene) = widget.borrow_mut::<SceneView>() else {
            return;
        };
        if let Some((x, y)) = scene.take_panned() {
            self.sent = Some((x, y));
            outbox.push(SceneAction::Camera { x, y });
        }
        if let Some((x, y, width, height, additive)) = scene.take_marqueed() {
            outbox.push(SceneAction::Marquee {
                x,
                y,
                width,
                height,
                additive,
            });
        }
        if let Some((id, additive)) = scene.take_picked() {
            outbox.push(SceneAction::Pick { id, additive });
        }
        if let Some(over) = scene.take_hovered() {
            outbox.push(SceneAction::Hover(over));
        }
        let Some(drawn) = scene.drawn() else {
            return;
        };
        let next = (
            drawn.camera.0,
            drawn.camera.1,
            drawn.pane.size.x,
            drawn.pane.size.y,
            drawn.objects,
        );
        if self.reported == Some(next) {
            return;
        }
        self.reported = Some(next);
        outbox.push(SceneAction::Viewport {
            x: next.0,
            y: next.1,
            width: next.2,
            height: next.3,
            drawn: next.4,
        });
    }
}
