//! The region's half of the B5 sample page.
//!
//! Twenty lines of text and nothing else. The point is the comparison: the
//! browser draws the same twenty strings a few centimetres away, and a
//! reviewer decides whether the region got the order, the direction and the
//! glyphs right. Nothing here interprets the text; a region that "fixed" a
//! sample would be hiding the thing being looked at.

use crate::samples::SAMPLES;
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
    /// Where each sample's line ended up, in the region's own local pixels.
    /// Reported when it changes, so a test can check that twenty lines were
    /// drawn, in order, without reading pixels.
    Lines(Vec<LocalRect>),
}

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    mod.widgets.SampleColumnBase = #(SampleColumn::register_widget(vm))

    mod.widgets.SampleColumn = set_type_default() do mod.widgets.SampleColumnBase{
        width: Fill
        height: Fill
        draw_bg +: {
            color: #x0c111d
        }
        // The family that covers more than Latin. This page is the one place
        // in the catalogue that always needs it, so it is asked for up front
        // rather than when a value turns out not to be ASCII.
        draw_text +: {
            color: #xd0d5dd
            text_style: theme.font_regular_i18n{ font_size: 11.0 }
        }
    }

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

/// How much room one sample gets. Twenty of them have to fit the canvas, and a
/// line that scrolled out of it is a line nobody compares.
const LINE: f64 = 15.0;

#[derive(Script, ScriptHook, Widget)]
pub struct SampleColumn {
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
    draw_text: DrawText,
    #[rust]
    missing: Option<String>,
    #[rust]
    lines: Option<Vec<LocalRect>>,
    #[rust]
    reported: bool,
}

impl SampleColumn {
    fn set_missing(&mut self, cx: &mut Cx, missing: Option<String>) {
        if self.missing != missing {
            self.missing = missing;
            self.redraw(cx);
        }
    }

    fn set_palette(&mut self, cx: &mut Cx, background: u32, foreground: u32) {
        let background = Vec4f::from_u32(background << 8 | 0xff);
        let foreground = Vec4f::from_u32(foreground << 8 | 0xff);
        if self.draw_bg.color != background || self.draw_text.color != foreground {
            self.draw_bg.color = background;
            self.draw_text.color = foreground;
            self.redraw(cx);
        }
    }

    /// The lines of the last draw, once, so a caller hears about a layout
    /// rather than about every frame.
    fn take_lines(&mut self) -> Option<Vec<LocalRect>> {
        if self.reported {
            return None;
        }
        let lines = self.lines.clone()?;
        self.reported = true;
        Some(lines)
    }
}

impl Widget for SampleColumn {
    fn handle_event(&mut self, _cx: &mut Cx, _event: &Event, _scope: &mut Scope) {}

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let pane = cx.walk_turtle(walk);
        self.draw_bg.draw_abs(cx, pane);
        let mut lines = Vec::with_capacity(SAMPLES.len());
        for (index, sample) in SAMPLES.iter().enumerate() {
            let top = pane.pos.y + 2.0 + index as f64 * LINE;
            // A sample this build has no glyphs for is replaced by the reason,
            // not left as a row of boxes: the mark says whose fault it is.
            let text = match (&self.missing, sample.text.is_ascii()) {
                (Some(missing), false) => missing.as_str(),
                _ => sample.text,
            };
            self.draw_text
                .draw_abs(cx, dvec2(pane.pos.x + 6.0, top), text);
            lines.push(LocalRect::new(
                pane.pos.x + 6.0,
                top,
                (pane.size.x - 12.0).max(0.0),
                LINE,
            ));
        }
        let changed = self.lines.as_deref() != Some(lines.as_slice());
        if changed {
            self.lines = Some(lines);
            self.reported = false;
        }
        DrawStep::done()
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
            SampleAction::Lines(_) => Pace::Continuous("lines"),
        }
    }

    fn script_mod(vm: &mut ScriptVm) -> ScriptValue {
        rustify_ui::makepad_widgets::script_mod(vm);
        rustify_ui::gpu::script_mod(vm);
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
        if let Some(lines) = self
            .ui
            .widget(cx, ids!(column))
            .borrow_mut::<SampleColumn>()
            .and_then(|mut column| column.take_lines())
        {
            outbox.push(SampleAction::Lines(lines));
        }
    }
}
