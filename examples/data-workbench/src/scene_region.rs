//! The scene as a region: one camera, one view, and a report of what it drew.

use crate::scene_view::SceneView;
use rustify_ui::makepad_widgets::*;
use rustify_ui::{Pace, RegionApp};

/// What the region is looking at.
///
/// The camera is an absolute position rather than a movement: a continuous
/// stream keeps only its newest value, so a stream of movements would lose
/// whatever was overwritten, while the newest position is always the right
/// answer.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SceneProps {
    pub camera: (f64, f64),
    pub highlight: Option<u32>,
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
}

impl RegionApp for SceneRegion {
    type Props = SceneProps;
    type Action = SceneAction;

    fn pace(action: &SceneAction) -> Pace {
        match action {
            // A stream about where the region is looking: only the newest can
            // still be true.
            SceneAction::Viewport { .. } => Pace::Continuous("viewport"),
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
        if let Some(mut scene) = self.ui.widget(cx, ids!(scene)).borrow_mut::<SceneView>() {
            scene.look_at(cx, props.camera, props.highlight);
        }
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, outbox: &mut Vec<SceneAction>) {
        self.ui.handle_event(cx, event, &mut Scope::empty());
        let Some(drawn) = self
            .ui
            .widget(cx, ids!(scene))
            .borrow_mut::<SceneView>()
            .and_then(|scene| scene.drawn())
        else {
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
