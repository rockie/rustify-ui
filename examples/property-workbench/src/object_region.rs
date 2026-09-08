use rustify_ui::makepad_widgets::*;
use rustify_ui::RegionApp;

/// What the region shows: the current selection, projected out of the
/// application's objects. The region never holds the object list itself.
#[derive(Clone, Debug, PartialEq)]
pub struct SelectionProps {
    pub name: String,
    pub position: String,
    /// `0xRRGGBB`; the region turns it into a colour when it draws.
    pub color: u32,
}

#[derive(Debug)]
pub enum SelectionAction {
    SelectPrevious,
    SelectNext,
}

script_mod! {
    use mod.prelude.widgets.*

    startup() do #(ObjectRegion::script_component(vm)){
        ui: Root{
            main_window := Window{
                body +: {
                    View{
                        width: Fill
                        height: Fill
                        flow: Down
                        spacing: 12
                        padding: 16
                        align: Center

                        swatch := RoundedView{
                            width: 140
                            height: 140
                        }
                        name_label := Label{
                            text: "no selection"
                            draw_text.text_style.font_size: 20
                        }
                        position_label := Label{
                            text: "0 / 0"
                        }
                        View{
                            width: Fit
                            height: Fit
                            flow: Right
                            spacing: 8

                            previous_button := Button{ text: "previous" }
                            next_button := Button{ text: "next" }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Script, ScriptHook)]
pub struct ObjectRegion {
    #[live]
    ui: WidgetRef,
}

impl RegionApp for ObjectRegion {
    type Props = SelectionProps;
    type Action = SelectionAction;

    fn script_mod(vm: &mut ScriptVm) -> ScriptValue {
        rustify_ui::makepad_widgets::script_mod(vm);
        self::script_mod(vm)
    }

    fn apply_props(&mut self, cx: &mut Cx, props: &SelectionProps) {
        self.ui
            .label(cx, ids!(name_label))
            .set_text(cx, &props.name);
        self.ui
            .label(cx, ids!(position_label))
            .set_text(cx, &props.position);
        let mut swatch = self.ui.widget(cx, ids!(swatch));
        // `+:` merges into the existing DrawQuad; `:` would replace it with a
        // plain object and the apply would be refused.
        let color = Vec4f::from_u32(props.color << 8 | 0xff);
        script_apply_eval!(cx, swatch, {
            draw_bg +: {
                color: #(color)
            }
        });
        self.ui.redraw(cx);
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, outbox: &mut Vec<SelectionAction>) {
        if let Event::Actions(actions) = event {
            if self.ui.button(cx, ids!(previous_button)).clicked(actions) {
                outbox.push(SelectionAction::SelectPrevious);
            }
            if self.ui.button(cx, ids!(next_button)).clicked(actions) {
                outbox.push(SelectionAction::SelectNext);
            }
        }
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
