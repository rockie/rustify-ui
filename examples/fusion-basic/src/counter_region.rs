use rustify_ui::makepad_widgets::*;
use rustify_ui::RegionApp;

#[derive(Clone, Debug, PartialEq)]
pub struct CounterProps {
    pub count: i64,
}

#[derive(Debug)]
pub enum CounterAction {
    Increment,
}

script_mod! {
    use mod.prelude.widgets.*

    startup() do #(CounterRegion::script_component(vm)){
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

                        counter_label := Label{
                            text: "Count: 0"
                            draw_text.text_style.font_size: 24
                        }
                        increment_button := Button{
                            text: "GPU +1"
                        }
                    }
                }
            }
        }
    }
}

#[derive(Script, ScriptHook)]
pub struct CounterRegion {
    #[live]
    ui: WidgetRef,
}

impl RegionApp for CounterRegion {
    type Props = CounterProps;
    type Action = CounterAction;

    fn script_mod(vm: &mut ScriptVm) -> ScriptValue {
        rustify_ui::makepad_widgets::script_mod(vm);
        self::script_mod(vm)
    }

    fn apply_props(&mut self, cx: &mut Cx, props: &CounterProps) {
        self.ui
            .label(cx, ids!(counter_label))
            .set_text(cx, &format!("Count: {}", props.count));
        self.ui.redraw(cx);
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, outbox: &mut Vec<CounterAction>) {
        if let Event::Actions(actions) = event {
            if self.ui.button(cx, ids!(increment_button)).clicked(actions) {
                outbox.push(CounterAction::Increment);
            }
        }
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}
