//! The rectangle the region keeps for the name, and the click that asks to
//! edit it.
//!
//! The field owns the box and reports it; the label drawn over it owns the
//! glyphs. Asked to edit, the region hands this rectangle over and stops
//! drawing the text into it: from then on the real text control has it, and
//! with it the caret, the selection and the input method.

use rustify_ui::makepad_widgets::widget::*;
use rustify_ui::makepad_widgets::*;

script_mod! {
    use mod.prelude.widgets_internal.*
    use mod.widgets.*

    mod.widgets.NameFieldBase = #(NameField::register_widget(vm))

    mod.widgets.NameField = set_type_default() do mod.widgets.NameFieldBase{
        width: 220
        height: 26
        draw_bg +: {
            color: #x1d2939
        }
    }
}

#[derive(Script, ScriptHook, Widget)]
pub struct NameField {
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
    #[rust]
    editing: bool,
    /// The box of the last draw, so what is handed over is what was seen.
    #[rust]
    drawn: Option<Rect>,
    #[rust]
    edit_request: Option<Rect>,
}

impl NameField {
    pub fn set_editing(&mut self, cx: &mut Cx, editing: bool) {
        if self.editing != editing {
            self.editing = editing;
            self.redraw(cx);
        }
    }

    pub fn take_edit_request(&mut self) -> Option<Rect> {
        self.edit_request.take()
    }
}

impl Widget for NameField {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event, _scope: &mut Scope) {
        let Some(drawn) = self.drawn else {
            return;
        };
        if let Hit::FingerUp(fe) = event.hits(cx, self.draw_bg.area()) {
            if fe.is_over && fe.is_primary_hit() && !self.editing {
                self.edit_request = Some(drawn);
            }
        }
    }

    fn draw_walk(&mut self, cx: &mut Cx2d, _scope: &mut Scope, walk: Walk) -> DrawStep {
        let pane = cx.walk_turtle(walk);
        self.draw_bg.draw_abs(cx, pane);
        self.drawn = Some(pane);
        DrawStep::done()
    }
}
