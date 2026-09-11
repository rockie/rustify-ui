//! The selection strip as a region: a projection in, a jump out.
//!
//! It is the table view's GPU half. Clicking a bucket sends the table there,
//! which is the GPU-to-DOM path; a change of selection comes back the other
//! way as a new projection.

use crate::strip_view::SelectionStrip;
use rustify_ui::makepad_widgets::*;
use rustify_ui::{Pace, RegionApp};
use std::sync::Arc;

#[derive(Clone, Debug, PartialEq)]
pub struct StripProps {
    /// How many rows of each bucket are selected. Shared rather than copied:
    /// it changes when the selection does, not when the table scrolls.
    pub buckets: Arc<Vec<u16>>,
    /// The largest count in `buckets`, so the strip can scale itself without
    /// walking them again on every draw.
    pub peak: u16,
    /// The part of the table that is on screen, as bucket indices.
    pub viewport: (usize, usize),
}

#[derive(Debug)]
pub enum StripAction {
    /// A bucket was clicked: the table should go there.
    Jump(usize),
}

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    startup() do #(StripRegion::script_component(vm)){
        ui: Root{
            main_window := Window{
                body +: {
                    strip := SelectionStrip{
                        width: Fill
                        height: Fill
                    }
                }
            }
        }
    }
}

#[derive(Script, ScriptHook)]
pub struct StripRegion {
    #[live]
    ui: WidgetRef,
}

impl RegionApp for StripRegion {
    type Props = StripProps;
    type Action = StripAction;

    fn pace(action: &StripAction) -> Pace {
        match action {
            // A click, not a stream: every one of them means to go somewhere.
            StripAction::Jump(_) => Pace::Discrete,
        }
    }

    fn script_mod(vm: &mut ScriptVm) -> ScriptValue {
        rustify_ui::makepad_widgets::script_mod(vm);
        crate::strip_view::script_mod(vm);
        self::script_mod(vm)
    }

    fn apply_props(&mut self, cx: &mut Cx, props: &StripProps) {
        if let Some(mut strip) = self
            .ui
            .widget(cx, ids!(strip))
            .borrow_mut::<SelectionStrip>()
        {
            strip.show(cx, props.buckets.clone(), props.peak, props.viewport);
        }
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, outbox: &mut Vec<StripAction>) {
        self.ui.handle_event(cx, event, &mut Scope::empty());
        let picked = self
            .ui
            .widget(cx, ids!(strip))
            .borrow_mut::<SelectionStrip>()
            .and_then(|mut strip| strip.take_picked());
        if let Some(bucket) = picked {
            outbox.push(StripAction::Jump(bucket));
        }
    }
}
