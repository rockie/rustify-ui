use crate::object_grid::{GridCell, ObjectGrid};
use rustify_ui::makepad_widgets::*;
use rustify_ui::RegionApp;
use std::sync::Arc;

/// What the region shows: the current selection, projected out of the
/// application's objects. The region never holds the object list itself.
#[derive(Clone, Debug, PartialEq)]
pub struct SelectionProps {
    pub name: String,
    pub position: String,
    /// `0xRRGGBB`; the region turns it into a colour when it draws.
    pub color: u32,
    /// Every object, in the application's order. Shared rather than copied:
    /// it only changes when the objects do, not when the selection moves.
    pub cells: Arc<Vec<GridCell>>,
    pub selected: Option<u32>,
}

#[derive(Debug)]
pub enum SelectionAction {
    SelectPrevious,
    SelectNext,
    Pick(u32),
    /// The projection named the same object twice and was refused; the region
    /// still shows the last unambiguous one.
    RejectedDuplicate(u32),
}

script_mod! {
    use mod.prelude.widgets.*
    use mod.widgets.*

    startup() do #(ObjectRegion::script_component(vm)){
        ui: Root{
            main_window := Window{
                body +: {
                    View{
                        width: Fill
                        height: Fill
                        flow: Down
                        spacing: 8
                        padding: 8

                        View{
                            width: Fill
                            height: Fit
                            flow: Right
                            spacing: 8
                            align: Center

                            swatch := RoundedView{
                                width: 32
                                height: 32
                            }
                            name_label := Label{
                                text: "no selection"
                                draw_text.text_style.font_size: 16
                            }
                            position_label := Label{
                                text: "0 / 0"
                            }
                            previous_button := Button{ text: "previous" }
                            next_button := Button{ text: "next" }
                        }
                        grid := ObjectGrid{
                            width: Fill
                            height: Fill
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
    /// Set while applying props, reported from the next event so the
    /// application hears about it on its own callback path.
    #[rust]
    rejected: Option<u32>,
}

impl RegionApp for ObjectRegion {
    type Props = SelectionProps;
    type Action = SelectionAction;

    fn script_mod(vm: &mut ScriptVm) -> ScriptValue {
        rustify_ui::makepad_widgets::script_mod(vm);
        crate::object_grid::script_mod(vm);
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
        if let Some(mut grid) = self.ui.widget(cx, ids!(grid)).borrow_mut::<ObjectGrid>() {
            self.rejected = grid
                .set_cells(cx, props.cells.as_ref().clone(), props.selected)
                .err();
        }
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
        // Polled after the tree has seen the event, so a click is reported in
        // the same pump that handled it.
        let picked = self
            .ui
            .widget(cx, ids!(grid))
            .borrow_mut::<ObjectGrid>()
            .and_then(|mut grid| grid.take_picked());
        if let Some(id) = picked {
            outbox.push(SelectionAction::Pick(id));
        }
        if let Some(id) = self.rejected.take() {
            outbox.push(SelectionAction::RejectedDuplicate(id));
        }
    }
}
