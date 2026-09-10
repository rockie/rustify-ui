use crate::name_field::NameField;
use crate::object_grid::{GridCell, ObjectGrid};
use rustify_ui::makepad_widgets::makepad_platform::{CxOsApi, OpenUrlInPlace};
use rustify_ui::makepad_widgets::*;
use rustify_ui::{LocalRect, Pace, RegionApp, RustifyCheckBox, RustifySlider, Theme};
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
    pub notes: String,
    pub selected: Option<u32>,
    pub hovered: Option<u32>,
    /// While the name is being edited elsewhere, the region draws nothing in
    /// its place: the native control has that rectangle.
    pub editing_name: bool,
    pub editing_notes: bool,
    /// A locked object refuses a new name. The region draws the same value the
    /// panel's checkbox shows, and its own checkbox asks for the same change.
    pub locked: bool,
    /// 0..=100 in steps of 5, the same range the panel's slider offers.
    pub size: f64,
    /// How many times the application has asked the region to open a link.
    /// A counter rather than a flag: the region acts on the change, and an
    /// application that asks twice means it twice.
    pub open_link_requests: u32,
    /// The scope's theme. One table drives both halves, so a colour cannot
    /// mean one thing in the panel and another in the region.
    pub theme: Theme,
}

/// Which piece of text the region is handing over.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EditField {
    /// One line.
    Name,
    /// Several.
    Notes,
}

#[derive(Debug)]
pub enum SelectionAction {
    SelectPrevious,
    SelectNext,
    Pick(u32),
    /// The object the pointer is over, or `None` once it left the grid. One
    /// per pointer move: a state, not a history.
    Hover(Option<u32>),
    /// The user asked to edit text the region draws, and the region hands over
    /// the rectangle it drew it into, in its own local CSS pixels.
    Edit {
        field: EditField,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
    },
    /// Where the region drew the controls of its second row, in its own local
    /// CSS pixels. Reported when it changes, so a caller can put a pointer on
    /// one of them without a second copy of the layout.
    Controls {
        locked: LocalRect,
        size: LocalRect,
        /// Where the two pieces of text it draws ended up. Reported for the
        /// same reason the controls are: what a pointer has to be aimed at is
        /// the region's to say, and a caller that guesses at a pixel offset is
        /// a caller that breaks the first time a panel changes width.
        name: LocalRect,
        notes: LocalRect,
    },
    /// The user asked to lock or unlock the object. A request, not a value:
    /// the region draws whatever comes back.
    SetLocked(bool),
    /// The user dragged the size along its track. One per move, so only the
    /// latest matters.
    SetSize(f64),
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

                            // First in the row on purpose: a control whose
                            // position depends on the width of a value moves
                            // out from under the pointer when the value does.
                            previous_button := Button{ text: "previous" }
                            next_button := Button{ text: "next" }
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
                                // The family that covers more than Latin costs
                                // 30 MB to fetch, so it is drawn - and thereby
                                // fetched - only once a value needs it.
                                wide_name := View{
                                    width: Fill
                                    height: Fill
                                    align: Center
                                    visible: false

                                    wide_name_label := Label{
                                        width: Fill
                                        draw_text +: {
                                            text_style: theme.font_regular_i18n{
                                                font_size: 14
                                            }
                                        }
                                    }
                                }
                            }
                            View{
                                width: 160
                                height: 26
                                flow: Overlay
                                align: Center

                                notes_field := NameField{
                                    width: Fill
                                    height: Fill
                                }
                                notes_label := Label{
                                    width: Fill
                                    text: "no notes"
                                    draw_text.text_style.font_size: 14
                                }
                                wide_notes := View{
                                    width: Fill
                                    height: Fill
                                    align: Center
                                    visible: false

                                    wide_notes_label := Label{
                                        width: Fill
                                        draw_text +: {
                                            text_style: theme.font_regular_i18n{
                                                font_size: 14
                                            }
                                        }
                                    }
                                }
                            }
                            position_label := Label{
                                width: 90
                                text: "0 / 0"
                            }
                        }

                        // A second row, so the first one keeps the places the
                        // pointer tests measure from its left edge.
                        View{
                            width: Fill
                            height: Fit
                            flow: Right
                            spacing: 8
                            // Left, not centred: a row that centres its
                            // children moves them whenever one of them
                            // changes width.
                            align: Align{x: 0., y: 0.5}

                            locked_box := RustifyCheckBox{}
                            locked_label := Label{
                                width: 70
                                text: "locked"
                                draw_text.text_style.font_size: 12
                            }
                            size_slider := RustifySlider{}
                            size_label := Label{
                                width: 70
                                text: "size 0"
                                draw_text.text_style.font_size: 12
                            }
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

/// A Makepad rectangle as the SDK's own local rectangle: the region's local
/// CSS pixels, which is what an application anchors and aims at.
fn local(rect: Rect) -> LocalRect {
    LocalRect::new(rect.pos.x, rect.pos.y, rect.size.x, rect.size.y)
}

/// What a multi-line value looks like in a single line of the region.
fn first_line(notes: &str) -> &str {
    notes.lines().next().unwrap_or("")
}

#[derive(Script, ScriptHook)]
pub struct ObjectRegion {
    #[live]
    ui: WidgetRef,
    /// Set while applying props, reported from the next event so the
    /// application hears about it on its own callback path.
    #[rust]
    rejected: Option<u32>,
    /// The last link request the region acted on, so one request opens one
    /// link and a redraw opens none.
    #[rust]
    opened_links: u32,
    /// The last geometry reported, so only a change is sent.
    #[rust]
    controls: Option<(LocalRect, LocalRect, LocalRect, LocalRect)>,
    /// Which values have needed more than Latin. Once one has, that label keeps
    /// drawing with the wider family: the file is already here, and moving
    /// back would only make the same value change shape.
    #[rust]
    wide_name: bool,
    #[rust]
    wide_notes: bool,
}

/// Whether a value needs glyphs the default family does not carry.
///
/// On the web the theme draws with a Latin-only family on purpose: the faces
/// that cover CJK and emoji are 30 MB together, and most applications never
/// draw one of their glyphs. A value that does need them says so by containing
/// one, and only then is the label moved to the family that has them - which
/// is when those files are fetched.
fn needs_wide_coverage(text: &str) -> bool {
    !text.is_ascii()
}

impl RegionApp for ObjectRegion {
    type Props = SelectionProps;
    type Action = SelectionAction;

    fn pace(action: &SelectionAction) -> Pace {
        match action {
            // Both are pointer streams: what matters is where they ended up.
            SelectionAction::Hover(_)
            | SelectionAction::SetSize(_)
            | SelectionAction::Controls { .. } => Pace::Continuous,
            _ => Pace::Discrete,
        }
    }

    fn script_mod(vm: &mut ScriptVm) -> ScriptValue {
        rustify_ui::makepad_widgets::script_mod(vm);
        rustify_ui::gpu::script_mod(vm);
        crate::name_field::script_mod(vm);
        crate::object_grid::script_mod(vm);
        self::script_mod(vm)
    }

    fn apply_props(&mut self, cx: &mut Cx, props: &SelectionProps) {
        if props.open_link_requests > self.opened_links {
            self.opened_links = props.open_link_requests;
            // A region is not the page. Asking anyway is how the refusal, and
            // the record of it, are exercised for real rather than assumed.
            cx.open_url("https://example.invalid/", OpenUrlInPlace::No);
        }
        // The native control shows the text while it is being edited, so the
        // region would otherwise draw a second copy of it underneath.
        let name = if props.editing_name {
            ""
        } else {
            props.name.as_str()
        };
        if !self.wide_name && needs_wide_coverage(name) {
            self.wide_name = true;
            self.ui.widget(cx, ids!(wide_name)).set_visible(cx, true);
        }
        let (plain_name, wide_name) = if self.wide_name {
            ("", name)
        } else {
            (name, "")
        };
        self.ui.label(cx, ids!(name_label)).set_text(cx, plain_name);
        self.ui
            .label(cx, ids!(wide_name_label))
            .set_text(cx, wide_name);
        // Several lines do not fit one line of a header, so the region shows
        // the first of them; the native control gets all of them when the
        // session starts, and nothing while it is running.
        let notes = if props.editing_notes {
            ""
        } else {
            first_line(&props.notes)
        };
        if !self.wide_notes && needs_wide_coverage(notes) {
            self.wide_notes = true;
            self.ui.widget(cx, ids!(wide_notes)).set_visible(cx, true);
        }
        let (plain_notes, wide_notes) = if self.wide_notes {
            ("", notes)
        } else {
            (notes, "")
        };
        self.ui
            .label(cx, ids!(notes_label))
            .set_text(cx, plain_notes);
        self.ui
            .label(cx, ids!(wide_notes_label))
            .set_text(cx, wide_notes);
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
        self.ui
            .label(cx, ids!(size_label))
            .set_text(cx, &format!("size {}", props.size as i64));
        if let Some(mut locked) = self
            .ui
            .widget(cx, ids!(locked_box))
            .borrow_mut::<RustifyCheckBox>()
        {
            // Nothing to lock while nothing is selected, so the control says
            // so rather than offering a change that would be refused.
            let disabled = props.selected.is_none();
            locked.set_state(cx, props.locked, disabled, false);
            locked.set_palette(
                cx,
                props.theme.input,
                props.theme.primary,
                props.theme.border,
            );
        }
        if let Some(mut size) = self
            .ui
            .widget(cx, ids!(size_slider))
            .borrow_mut::<RustifySlider>()
        {
            size.set_range(cx, 0.0, 100.0, 5.0);
            size.set_state(cx, props.size, props.selected.is_none(), false);
            size.set_palette(
                cx,
                props.theme.input,
                props.theme.primary,
                props.theme.foreground,
            );
        }
        if let Some(mut grid) = self.ui.widget(cx, ids!(grid)).borrow_mut::<ObjectGrid>() {
            grid.set_palette(
                cx,
                props.theme.background,
                props.theme.foreground,
                // The hover ring is drawn in the quiet *text* colour, not on
                // the quiet surface: those were one token until the component
                // classes needed to tell them apart.
                props.theme.muted_foreground,
            );
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
        // The text the region draws is a display of the value; clicking it asks
        // the application for a real text control over that rectangle.
        for (field, id) in [
            (EditField::Name, ids!(name_field)),
            (EditField::Notes, ids!(notes_field)),
        ] {
            let requested = self
                .ui
                .widget(cx, id)
                .borrow_mut::<NameField>()
                .and_then(|mut box_| box_.take_edit_request());
            if let Some(rect) = requested {
                outbox.push(SelectionAction::Edit {
                    field,
                    x: rect.pos.x,
                    y: rect.pos.y,
                    width: rect.size.x,
                    height: rect.size.y,
                });
            }
        }
        let locked_box = self
            .ui
            .widget(cx, ids!(locked_box))
            .borrow_mut::<RustifyCheckBox>()
            .map(|mut locked| (locked.take_change(), locked.drawn(cx)));
        if let Some((asked, drawn_locked)) = locked_box {
            if let Some(locked) = asked {
                outbox.push(SelectionAction::SetLocked(locked));
            }
            let size_slider = self
                .ui
                .widget(cx, ids!(size_slider))
                .borrow_mut::<RustifySlider>()
                .map(|mut size| (size.take_change(), size.drawn(cx)));
            if let Some((asked, drawn_size)) = size_slider {
                if let Some(size) = asked {
                    outbox.push(SelectionAction::SetSize(size));
                }
                let name = self.ui.widget(cx, ids!(name_label)).area().rect(cx);
                let notes = self.ui.widget(cx, ids!(notes_label)).area().rect(cx);
                if let (Some(locked), Some(size)) = (drawn_locked, drawn_size) {
                    let reported = (local(locked), local(size), local(name), local(notes));
                    if self.controls != Some(reported) {
                        self.controls = Some(reported);
                        outbox.push(SelectionAction::Controls {
                            locked: reported.0,
                            size: reported.1,
                            name: reported.2,
                            notes: reported.3,
                        });
                    }
                }
            }
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
