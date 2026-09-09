use crate::name_field::NameField;
use crate::object_grid::{GridCell, ObjectGrid};
use rustify_ui::makepad_widgets::*;
use rustify_ui::{Pace, RegionApp};
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
    pub hovered: Option<u32>,
    /// While the name is being edited elsewhere, the region draws nothing in
    /// its place: the native control has that rectangle.
    pub editing_name: bool,
}

#[derive(Debug)]
pub enum SelectionAction {
    SelectPrevious,
    SelectNext,
    Pick(u32),
    /// The object the pointer is over, or `None` once it left the grid. One
    /// per pointer move: a state, not a history.
    Hover(Option<u32>),
    /// The user asked to edit the name the region draws, and hands over the
    /// rectangle it drew it into, in the region's local CSS pixels.
    EditName {
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    },
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
                            // Fixed widths: the buttons beside them must not
                            // move while the user is typing a name.
                            // The box and the glyphs are two widgets in one
                            // place: the field owns the rectangle and the
                            // click, the label owns the text.
                            View{
                                width: 220
                                height: 26
                                flow: Overlay
                                align: Center

                                name_field := NameField{
                                    width: Fill
                                    height: Fill
                                }
                                name_label := Label{
                                    width: Fill
                                    text: "no selection"
                                    draw_text.text_style.font_size: 14
                                }
                            }
                            position_label := Label{
                                width: 90
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

    fn pace(action: &SelectionAction) -> Pace {
        match action {
            SelectionAction::Hover(_) => Pace::Continuous,
            _ => Pace::Discrete,
        }
    }

    fn script_mod(vm: &mut ScriptVm) -> ScriptValue {
        rustify_ui::makepad_widgets::script_mod(vm);
        crate::name_field::script_mod(vm);
        crate::object_grid::script_mod(vm);
        self::script_mod(vm)
    }

    fn apply_props(&mut self, cx: &mut Cx, props: &SelectionProps) {
        // The native control shows the text while it is being edited, so the
        // region would otherwise draw a second copy of it underneath.
        let name = if props.editing_name {
            ""
        } else {
            props.name.as_str()
        };
        self.ui.label(cx, ids!(name_label)).set_text(cx, name);
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
                .set_cells(
                    cx,
                    props.cells.as_ref().clone(),
                    props.selected,
                    props.hovered,
                )
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
        // The name the region draws is a display of the value; clicking it
        // asks the application for a real text control over that rectangle.
        let requested = self
            .ui
            .widget(cx, ids!(name_field))
            .borrow_mut::<NameField>()
            .and_then(|mut field| field.take_edit_request());
        if let Some(rect) = requested {
            outbox.push(SelectionAction::EditName {
                x: rect.pos.x,
                y: rect.pos.y,
                width: rect.size.x,
                height: rect.size.y,
            });
        }
        // Polled after the tree has seen the event, so a click is reported in
        // the same pump that handled it.
        let reported = self
            .ui
            .widget(cx, ids!(grid))
            .borrow_mut::<ObjectGrid>()
            .map(|mut grid| (grid.take_picked(), grid.take_hover_report()));
        if let Some((picked, hovered)) = reported {
            if let Some(id) = picked {
                outbox.push(SelectionAction::Pick(id));
            }
            if let Some(hovered) = hovered {
                outbox.push(SelectionAction::Hover(hovered));
            }
        }
        if let Some(id) = self.rejected.take() {
            outbox.push(SelectionAction::RejectedDuplicate(id));
        }
    }
}
