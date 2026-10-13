use leptos::prelude::*;
use rustify_components::data_table::Cell;
use rustify_components::*;
use rustify_ui::{provide_toasts, Selection, ToastHandle, ToastOptions, ToastTone};
use std::collections::BTreeSet;
use std::sync::Arc;

#[derive(Clone, Copy)]
pub(crate) struct State {
    pub count: RwSignal<u32>,
    pub text: RwSignal<String>,
    pub area: RwSignal<String>,
    pub checked: RwSignal<bool>,
    pub radio: RwSignal<String>,
    pub switched: RwSignal<bool>,
    pub select: RwSignal<String>,
    pub select_open: RwSignal<bool>,
    pub slider: RwSignal<f64>,
    pub tooltip: RwSignal<bool>,
    pub menu: RwSignal<bool>,
    pub menu_action: RwSignal<String>,
    pub dialog: RwSignal<bool>,
    pub tab: RwSignal<String>,
    pub table_selection: RwSignal<Selection>,
    pub table_focus: RwSignal<Cell>,
    pub table_activated: RwSignal<Option<usize>>,
    pub table_sorted: RwSignal<Option<(usize, bool)>>,
    pub expanded: RwSignal<BTreeSet<String>>,
    pub tree: RwSignal<Option<String>>,
    pub number: RwSignal<f64>,
    pub color: RwSignal<String>,
    pub toggles: RwSignal<Vec<String>>,
    pub sizes: RwSignal<Vec<f64>>,
    pub toasts: ToastHandle,
    pub asynchronous: RwSignal<String>,
    #[cfg(target_arch = "wasm32")]
    pub keys: RwSignal<u32>,
    #[cfg(target_arch = "wasm32")]
    pub resizes: RwSignal<u32>,
}

impl State {
    fn new() -> Self {
        let mut selection = Selection::new();
        selection.set([101]);
        Self {
            count: RwSignal::new(7),
            text: RwSignal::new("Résumé <&>".to_string()),
            area: RwSignal::new("\nRésumé </textarea> & <line>\r\nCRLF\rCR\nlast".to_string()),
            checked: RwSignal::new(true),
            radio: RwSignal::new("north".to_string()),
            switched: RwSignal::new(false),
            select: RwSignal::new("red".to_string()),
            select_open: RwSignal::new(false),
            slider: RwSignal::new(42.5),
            tooltip: RwSignal::new(false),
            menu: RwSignal::new(false),
            menu_action: RwSignal::new(String::new()),
            dialog: RwSignal::new(false),
            tab: RwSignal::new("detail".to_string()),
            table_selection: RwSignal::new(selection),
            table_focus: RwSignal::new(Cell { row: 1, column: 1 }),
            table_activated: RwSignal::new(None),
            table_sorted: RwSignal::new(Some((1, true))),
            expanded: RwSignal::new(BTreeSet::from(["root".to_string()])),
            tree: RwSignal::new(Some("alpha".to_string())),
            number: RwSignal::new(-3.25),
            color: RwSignal::new("#1a2b3c80".to_string()),
            toggles: RwSignal::new(vec!["italic".to_string()]),
            sizes: RwSignal::new(vec![143.5, 211.25]),
            toasts: provide_toasts(),
            asynchronous: RwSignal::new("idle".to_string()),
            #[cfg(target_arch = "wasm32")]
            keys: RwSignal::new(0),
            #[cfg(target_arch = "wasm32")]
            resizes: RwSignal::new(0),
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub fn snapshot(self) -> String {
        serde_json::json!({
            "count": self.count.get_untracked(),
            "text": self.text.get_untracked(),
            "area": self.area.get_untracked(),
            "checked": self.checked.get_untracked(),
            "radio": self.radio.get_untracked(),
            "switched": self.switched.get_untracked(),
            "select": self.select.get_untracked(),
            "select_open": self.select_open.get_untracked(),
            "slider": self.slider.get_untracked(),
            "tooltip": self.tooltip.get_untracked(),
            "menu": self.menu.get_untracked(),
            "menu_action": self.menu_action.get_untracked(),
            "dialog": self.dialog.get_untracked(),
            "tab": self.tab.get_untracked(),
            "table_selection": self.table_selection.get_untracked().ids().collect::<Vec<_>>(),
            "table_focus": [self.table_focus.get_untracked().row, self.table_focus.get_untracked().column],
            "table_activated": self.table_activated.get_untracked(),
            "table_sorted": self.table_sorted.get_untracked(),
            "expanded": self.expanded.get_untracked(),
            "tree": self.tree.get_untracked(),
            "number": self.number.get_untracked(),
            "color": self.color.get_untracked(),
            "toggles": self.toggles.get_untracked(),
            "sizes": self.sizes.get_untracked(),
            "toast": self.toasts.current().map(|toast| toast.message),
            "asynchronous": self.asynchronous.get_untracked(),
            "keys": self.keys.get_untracked(),
            "resizes": self.resizes.get_untracked(),
        })
        .to_string()
    }
}

#[component]
pub(crate) fn CatalogView(scope: String) -> impl IntoView {
    let state = State::new();
    let held_scope = StoredValue::new(scope.clone());
    let id = move |part: &str| format!("{}-{part}", held_scope.get_value());
    let label_control = id("label-control");
    let description = id("description");
    let link_destination = id("link-destination");
    let menu_anchor = NodeRef::<leptos::html::Button>::new();
    let observed = NodeRef::<leptos::html::Div>::new();
    #[cfg(target_arch = "wasm32")]
    let anchor = Signal::derive(move || {
        menu_anchor
            .get()
            .map(|node| rustify_ui::Anchor::element(node.as_ref()))
            .unwrap_or(rustify_ui::Anchor::Centred)
    });
    #[cfg(not(target_arch = "wasm32"))]
    let anchor = Signal::derive(|| rustify_ui::Anchor::Centred);

    #[cfg(target_arch = "wasm32")]
    {
        crate::browser::register(scope.clone(), state);
        let listener = rustify_ui::listen(
            window().as_ref(),
            "harness-key",
            rustify_ui::ListenOptions::default(),
            move |_| state.keys.update(|count| *count += 1),
        );
        on_cleanup(move || drop(listener));
        let observation = StoredValue::new_local(None::<rustify_ui::ResizeObservation>);
        Effect::new(move |_| {
            if let Some(node) = observed.get() {
                observation.set_value(rustify_ui::observe_resize(node.as_ref(), move || {
                    state.resizes.update(|count| *count += 1);
                }));
            }
        });
    }

    view! {
        <div data-harness-catalog=scope.clone() class="harness-catalog" node_ref=observed>
            <p id=description.clone() class="harness-description">"Deterministic values, generated IDs and external geometry."</p>
            <section data-category=Category::Button.name() class="harness-card">
                <h3>"Button"</h3>
                <Button test_id=id("button") variant=ButtonVariant::Outline on_click=move || state.count.update(|n| *n += 1)>"Count"</Button>
                <Button test_id=id("button-disabled") disabled=true on_click=move || state.count.update(|n| *n += 100)>"Disabled"</Button>
                <output data-testid=id("count")>{move || state.count.get()}</output>
            </section>
            <section data-category=Category::Label.name() class="harness-card">
                <h3>"Label"</h3>
                <Label test_id=id("label") control=label_control.clone()>"Named field"</Label>
                <input id=label_control data-testid=id("label-control") value="Label target" />
            </section>
            <section data-category=Category::Link.name() class="harness-card">
                <h3>"Link"</h3>
                <Link test_id=id("link") href=format!("#{link_destination}")>"Local destination"</Link>
                <span id=link_destination>"Destination"</span>
            </section>
            <section data-category=Category::Icon.name() class="harness-card">
                <h3>"Icon"</h3>
                <Icon test_id=id("icon") glyph=Glyph::Close label="Close drawing" />
            </section>
            <section data-category=Category::TextField.name() class="harness-card">
                <h3>"Text field"</h3>
                <TextField test_id=id("text") value=state.text on_change=move |value| state.text.set(value) aria_label="Editable text" described_by=description.clone() />
                <TextField test_id=id("text-readonly") value="Read only" on_change=|_| {} read_only=true disabled=true aria_label="Read only text" />
            </section>
            <section data-category=Category::TextArea.name() class="harness-card">
                <h3>"Text area"</h3>
                <TextArea test_id=id("area") value=state.area on_change=move |value| state.area.set(value) rows=3 aria_label="Multiline text" />
            </section>
            <section data-category=Category::Checkbox.name() class="harness-card">
                <h3>"Checkbox"</h3>
                <Checkbox test_id=id("checkbox") checked=state.checked on_change=move |checked| state.checked.set(checked) aria_label="Included" described_by=description.clone() />
            </section>
            <section data-category=Category::Radio.name() class="harness-card">
                <h3>"Radio"</h3>
                <RadioGroup test_id=id("radio") value=state.radio options=vec![RadioOption::new("north", "North"), RadioOption::new("blocked", "Blocked").disabled(), RadioOption::new("south", "South")] on_change=move |value| state.radio.set(value) aria_label="Direction" />
            </section>
            <section data-category=Category::Switch.name() class="harness-card">
                <h3>"Switch"</h3>
                <Switch test_id=id("switch") checked=state.switched on_change=move |checked| state.switched.set(checked) aria_label="Enabled" />
            </section>
            <section data-category=Category::Select.name() class="harness-card">
                <h3>"Select"</h3>
                <Select test_id=id("select") value=state.select options=vec![SelectOption::new("red", "Red"), SelectOption::new("blocked", "Blocked").disabled(), SelectOption::new("blue", "Blue")] on_change=move |value| state.select.set(value) open=state.select_open on_open_change=move |open| state.select_open.set(open) aria_label="Colour choice" />
            </section>
            <section data-category=Category::Slider.name() class="harness-card">
                <h3>"Slider"</h3>
                <Slider test_id=id("slider") value=state.slider on_change=move |value| state.slider.set(value) min=10.0 max=90.0 step=2.5 aria_label="Amount" />
            </section>
            <section data-category=Category::Progress.name() class="harness-card">
                <h3>"Progress"</h3>
                <Progress test_id=id("progress") value=state.slider max=125.0 aria_label="Completion" />
            </section>
            <section data-category=Category::Loading.name() class="harness-card">
                <h3>"Loading"</h3>
                <Spinner test_id=id("loading") label="Loading fixture" />
            </section>
            <section data-category=Category::Tooltip.name() class="harness-card">
                <h3>"Tooltip"</h3>
                <Tooltip test_id=id("tooltip") open=state.tooltip on_open_change=move |open| state.tooltip.set(open) label="Help for this control">
                    <button type="button" data-testid=id("tooltip-trigger")>"Hover for help"</button>
                </Tooltip>
            </section>
            <section data-category=Category::Menu.name() class="harness-card">
                <h3>"Menu"</h3>
                <button type="button" node_ref=menu_anchor data-testid=id("menu-trigger") on:click=move |_| state.menu.set(true)>"Open menu"</button>
                <Menu test_id=id("menu") open=state.menu on_open_change=move |open| state.menu.set(open) anchor=anchor items=vec![MenuItem::heading("Actions"), MenuItem::new("copy", "Copy"), MenuItem::new("blocked", "Unavailable").disabled("Fixture unavailable"), MenuItem::separator(), MenuItem::new("delete", "Delete")] on_activate=move |action| state.menu_action.set(action) aria_label="Fixture actions" />
            </section>
            <section data-category=Category::Dialog.name() class="harness-card">
                <h3>"Dialog"</h3>
                <Button test_id=id("dialog-trigger") on_click=move || state.dialog.set(true)>"Open dialog"</Button>
                <Dialog test_id=id("dialog") open=state.dialog on_open_change=move |open| state.dialog.set(open) title="Fixture dialog" description="This scope is inert while open.">
                    <button type="button">"Dialog action"</button>
                </Dialog>
            </section>
            <section data-category=Category::Tabs.name() class="harness-card">
                <h3>"Tabs"</h3>
                <Tabs test_id=id("tabs") active=state.tab tabs=vec![Tab::new("summary", "Summary"), Tab::new("blocked", "Blocked").disabled(), Tab::new("detail", "Detail")] on_activate=move |active| state.tab.set(active) aria_label="Fixture panels">
                    <TabPanel value="summary"><p>"Summary panel"</p></TabPanel>
                    <TabPanel value="blocked"><p>"Unavailable panel"</p></TabPanel>
                    <TabPanel value="detail"><p>"Detail panel"</p></TabPanel>
                </Tabs>
            </section>
            <section data-category=Category::ScrollArea.name() class="harness-card">
                <h3>"Scroll area"</h3>
                <ScrollArea test_id=id("scroll") aria_label="Scrollable content" class="harness-scroll">
                    <p>"First item"</p><p>"Second item"</p><p>"Third item"</p><p>"Fourth item"</p><p>"Fifth item"</p><p>"Last item"</p>
                </ScrollArea>
            </section>
            <section data-category=Category::DataTable.name() class="harness-card">
                <h3>"Data table"</h3>
                <DataTable test_id=id("table") rows=4usize columns=vec![Column::new("name", "Name"), Column::new("value", "Value"), Column::new("kind", "Kind")] cell=Arc::new(|row, column| format!("R{} C{}", row + 1, column + 1)) row_id=Arc::new(|row| row as u32 + 100) selected=state.table_selection goto=RwSignal::new(None) focus=state.table_focus on_select=move |row| {state.table_selection.update(|selected| { selected.toggle(row as u32 + 100); });} on_activate=move |row| state.table_activated.set(Some(row)) sorted=state.table_sorted on_sort=Arc::new(move |column| state.table_sorted.update(|sort| *sort=Some((column, !sort.is_some_and(|(was, ascending)| was==column && ascending))))) row_height=31.5 column_width=137.25 initial_viewport=(480.0, 160.0) aria_label="Four row fixture" class="harness-table" />
            </section>
            <section data-category=Category::Tree.name() class="harness-card">
                <h3>"Tree"</h3>
                <Tree test_id=id("tree") nodes=vec![TreeNode::new("root", "Root").with(vec![TreeNode::new("alpha", "Alpha"), TreeNode::new("beta", "Beta")]), TreeNode::new("other", "Other")] expanded=state.expanded selected=state.tree on_select=move |key| state.tree.set(Some(key)) aria_label="Fixture groups" />
            </section>
            <section data-category=Category::Toast.name() class="harness-card">
                <h3>"Toast"</h3>
                <Toast test_id=id("static-toast") toast=rustify_ui::Toast {seq: 0, message: "Deterministic status".to_string(), tone: ToastTone::Success} />
                <Button test_id=id("toast-trigger") on_click=move || {state.toasts.show("Runtime status", ToastOptions {duration_ms: 800, tone: ToastTone::Success});}>"Show timed toast"</Button>
                <Toaster test_id=id("toaster") />
            </section>
            <section data-category=Category::NumberField.name() class="harness-card">
                <h3>"Number field"</h3>
                <NumberField test_id=id("number") value=state.number on_change=move |value| state.number.set(value) min=-10.0 max=10.0 step=0.5 aria_label="Signed decimal" />
            </section>
            <section data-category=Category::ColorField.name() class="harness-card">
                <h3>"Colour field"</h3>
                <ColorField test_id=id("color") value=state.color on_change=move |value| state.color.set(value) alpha=true aria_label="RGBA colour" />
            </section>
            <section data-category=Category::ToggleGroup.name() class="harness-card">
                <h3>"Toggle group"</h3>
                <ToggleGroup test_id=id("toggle") value=state.toggles on_change=move |value| state.toggles.set(value) items=vec![ToggleItem::new("bold", "Bold"), ToggleItem::new("italic", "Italic"), ToggleItem::new("underline", "Underline").disabled()] multiple=true toolbar=true aria_label="Text style" />
            </section>
            <section data-extra="splitter" class="harness-card">
                <h3>"Splitter geometry"</h3>
                <div class="harness-splitter">
                    <Splitter test_id=id("splitter") sizes=state.sizes mins=vec![80.0, 80.0] on_resize=move |sizes| state.sizes.set(sizes) step=7.5 divider_label="Resize fixture panel" />
                </div>
            </section>
            <section data-extra="asynchronous" class="harness-card">
                <h3>"Browser work after attachment"</h3>
                <output data-testid=id("asynchronous")>{move || state.asynchronous.get()}</output>
            </section>
        </div>
    }
}
