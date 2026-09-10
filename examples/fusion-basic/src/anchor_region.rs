//! The region that owns the anchor grid: it draws nothing itself and only
//! relays what the grid reports.

use crate::anchor_grid::{Anchor, AnchorGrid, AnchorHit};
use rustify_ui::makepad_widgets::*;
use rustify_ui::{Pace, RegionApp};
use std::sync::Arc;

/// What the region draws: one colour per anchor and which one is selected.
/// The region holds neither; both come from the application.
#[derive(Clone, Debug, PartialEq)]
pub struct AnchorProps {
    pub colors: Arc<Vec<u32>>,
    pub selected: Option<usize>,
}

#[derive(Debug)]
pub enum AnchorAction {
    /// Where the region put its anchors. Only the latest one describes the
    /// geometry on screen, so it is admitted as a stream.
    Layout(Vec<Anchor>),
    Hit(AnchorHit),
}

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    startup() do #(AnchorRegion::script_component(vm)){
        ui: Root{
            main_window := Window{
                body +: {
                    grid := AnchorGrid{
                        width: Fill
                        height: Fill
                    }
                }
            }
        }
    }
}

#[derive(Script, ScriptHook)]
pub struct AnchorRegion {
    #[live]
    ui: WidgetRef,
}

impl RegionApp for AnchorRegion {
    type Props = AnchorProps;
    type Action = AnchorAction;

    fn pace(action: &AnchorAction) -> Pace {
        match action {
            AnchorAction::Layout(_) => Pace::Continuous("layout"),
            AnchorAction::Hit(_) => Pace::Discrete,
        }
    }

    fn script_mod(vm: &mut ScriptVm) -> ScriptValue {
        rustify_ui::makepad_widgets::script_mod(vm);
        crate::anchor_grid::script_mod(vm);
        self::script_mod(vm)
    }

    fn apply_props(&mut self, cx: &mut Cx, props: &AnchorProps) {
        if let Some(mut grid) = self.ui.widget(cx, ids!(grid)).borrow_mut::<AnchorGrid>() {
            grid.set_projection(cx, props.colors.as_ref(), props.selected);
        }
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, outbox: &mut Vec<AnchorAction>) {
        self.ui.handle_event(cx, event, &mut Scope::empty());
        let reported = self
            .ui
            .widget(cx, ids!(grid))
            .borrow_mut::<AnchorGrid>()
            .map(|mut grid| (grid.take_layout(), grid.take_hit()));
        if let Some((layout, hit)) = reported {
            if let Some(layout) = layout {
                outbox.push(AnchorAction::Layout(layout));
            }
            if let Some(hit) = hit {
                outbox.push(AnchorAction::Hit(hit));
            }
        }
    }
}
