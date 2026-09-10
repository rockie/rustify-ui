//! The region's half of the B5 sample page.
//!
//! Twenty lines of text and nothing else. The point is the comparison: the
//! browser draws the same twenty strings a few centimetres away, and a
//! reviewer decides whether the region got the order, the direction and the
//! glyphs right. Nothing here interprets the text; a region that "fixed" a
//! sample would be hiding the thing being looked at.

use crate::sample_column::SampleColumn;
use rustify_ui::makepad_widgets::widget::*;
use rustify_ui::makepad_widgets::*;
use rustify_ui::{LocalRect, Pace, RegionApp, Theme};

/// What the samples region draws.
#[derive(Clone, Debug, PartialEq)]
pub struct SampleProps {
    pub theme: Theme,
    /// What to draw in place of a sample whose glyphs this build cannot get.
    ///
    /// `None` while the fonts are there. When a font asset fails to arrive,
    /// the application puts the SDK's missing-glyph message here and the
    /// region draws it beside every sample that needs more than Latin - so
    /// the page says *why* it looks wrong, which a row of boxes does not.
    pub missing: Option<String>,
}

#[derive(Debug)]
pub enum SampleAction {
    /// What the region drew: where each sample's line ended up, in its own
    /// local pixels, and how many of them were replaced by the missing-glyph
    /// message. Reported when it changes, so a page can say what happened
    /// without anybody reading pixels off a screenshot.
    Drew {
        lines: Vec<LocalRect>,
        /// The samples that needed more than Latin and got the mark instead.
        /// Zero while the fonts are there.
        marked: usize,
    },
}

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    startup() do #(SampleRegion::script_component(vm)){
        ui: Root{
            main_window := Window{
                body +: {
                    column := SampleColumn{
                        width: Fill
                        height: Fill
                    }
                }
            }
        }
    }
}

#[derive(Script, ScriptHook)]
pub struct SampleRegion {
    #[live]
    ui: WidgetRef,
}

impl RegionApp for SampleRegion {
    type Props = SampleProps;
    type Action = SampleAction;

    fn pace(action: &SampleAction) -> Pace {
        match action {
            SampleAction::Drew { .. } => Pace::Continuous("drew"),
        }
    }

    fn script_mod(vm: &mut ScriptVm) -> ScriptValue {
        rustify_ui::makepad_widgets::script_mod(vm);
        rustify_ui::gpu::script_mod(vm);
        crate::sample_column::script_mod(vm);
        self::script_mod(vm)
    }

    fn apply_props(&mut self, cx: &mut Cx, props: &SampleProps) {
        if let Some(mut column) = self
            .ui
            .widget(cx, ids!(column))
            .borrow_mut::<SampleColumn>()
        {
            column.set_palette(cx, props.theme.background, props.theme.foreground);
            column.set_missing(cx, props.missing.clone());
        }
        self.ui.redraw(cx);
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, outbox: &mut Vec<SampleAction>) {
        self.ui.handle_event(cx, event, &mut Scope::empty());
        if let Some((lines, marked)) = self
            .ui
            .widget(cx, ids!(column))
            .borrow_mut::<SampleColumn>()
            .and_then(|mut column| column.take_drawn())
        {
            outbox.push(SampleAction::Drew { lines, marked });
        }
    }
}
