//! The twenty samples, drawn as twenty lines.
//!
//! Its own widget rather than twenty labels in the script: the page compares
//! row against row, so the spacing has to be one number in one place, and the
//! column has to be able to say what it drew.

use crate::samples::SAMPLES;
use rustify_ui::makepad_widgets::widget::*;
use rustify_ui::makepad_widgets::*;
use rustify_ui::LocalRect;

script_mod! {
    use mod.prelude.widgets_internal.*

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
    marked: usize,
    #[rust]
    reported: bool,
}

impl SampleColumn {
    pub fn set_missing(&mut self, cx: &mut Cx, missing: Option<String>) {
        if self.missing != missing {
            self.missing = missing;
            self.redraw(cx);
        }
    }

    pub fn set_palette(&mut self, cx: &mut Cx, background: u32, foreground: u32) {
        let background = Vec4f::from_u32(background << 8 | 0xff);
        let foreground = Vec4f::from_u32(foreground << 8 | 0xff);
        if self.draw_bg.color != background || self.draw_text.color != foreground {
            self.draw_bg.color = background;
            self.draw_text.color = foreground;
            self.redraw(cx);
        }
    }

    /// What the last draw produced, once, so a caller hears about a draw that
    /// changed something rather than about every frame.
    pub fn take_drawn(&mut self) -> Option<(Vec<LocalRect>, usize)> {
        if self.reported {
            return None;
        }
        let lines = self.lines.clone()?;
        self.reported = true;
        Some((lines, self.marked))
    }
}

impl Widget for SampleColumn {
    fn handle_event(&mut self, _cx: &mut Cx, _event: &Event, _scope: &mut Scope) {}

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let pane = cx.walk_turtle(walk);
        self.draw_bg.draw_abs(cx, pane);
        let mut lines = Vec::with_capacity(SAMPLES.len());
        let mut marked = 0;
        for (index, sample) in SAMPLES.iter().enumerate() {
            let top = pane.pos.y + 2.0 + index as f64 * LINE;
            // A sample this build has no glyphs for is replaced by the reason,
            // not left as a row of boxes: the mark says whose fault it is.
            let text = match (&self.missing, sample.text.is_ascii()) {
                (Some(missing), false) => {
                    marked += 1;
                    missing.as_str()
                }
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
        if self.lines.as_deref() != Some(lines.as_slice()) || self.marked != marked {
            self.lines = Some(lines);
            self.marked = marked;
            self.reported = false;
        }
        DrawStep::done()
    }
}
