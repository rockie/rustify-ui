#[cfg(target_arch = "wasm32")]
mod catalog_region;
#[cfg(target_arch = "wasm32")]
mod sample_column;
#[cfg(target_arch = "wasm32")]
mod sample_region;
// Not gated: twenty fixed strings and their ids are checked on the host, where
// "there are twenty of them and no two share a name" needs no browser. Only
// the browser build draws them, which is what the allow is for.
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
mod samples;

#[cfg(target_arch = "wasm32")]
mod app {
    use super::catalog_region::{CatalogAction, CatalogProps, CatalogRegion};
    use leptos::prelude::*;
    use leptos::wasm_bindgen::prelude::*;
    use rustify_components::data_table::{Cell as GridCell, Column, DataTable};
    use rustify_components::{
        clx, provide_current_path, variants, Boundary, Button, ButtonVariant, Category, Checkbox,
        Dialog, Glyph, Icon, Label, Link, Menu, MenuItem, Progress, RadioGroup, RadioOption,
        ScrollArea, Select, SelectOption, Slider, Spinner, Support, Switch, Tab, TabPanel, Tabs,
        TextArea, TextField, Tooltip, Tree, TreeNode, CATALOG,
    };
    use rustify_ui::{
        mount, use_theme_values, Anchor, AppHandle, GpuRegion, LocalRect, Locale, MountConfig,
        RegionState, Selection, Theme, ThemedScope,
    };
    use std::cell::RefCell;
    use std::collections::{BTreeMap, BTreeSet};
    use std::marker::PhantomData;
    use std::rc::Rc;
    use std::sync::Arc;

    clx! {Panel, section, "rounded-md border border-border bg-card p-4"}
    clx! {Row, div, "flex items-center gap-2"}

    variants! {
        Chip {
            base: "inline-flex items-center rounded-md px-2 py-0.5 text-xs",
            variants: {
                variant: {
                    Yes: "bg-success text-primary-foreground",
                    Partial: "bg-warning text-primary-foreground",
                    No: "bg-muted text-muted-foreground",
                },
                size: {
                    Default: "px-2 py-0.5",
                    Lg: "px-3 py-1",
                }
            },
            component: { element: span }
        }
    }

    /// The catalogue's own words, in the two languages the SDK ships.
    ///
    /// The language itself is `rustify_ui::Locale` - one language for the page
    /// and for the SDK's own messages, because a page that said "close" in
    /// English inside a layer whose close button said 关闭 would be showing two
    /// answers to one question. What stays here is the *vocabulary*: these are
    /// the catalogue's words, and an SDK that shipped them would be
    /// translating a page it cannot see.
    ///
    /// The key is `&'static str` so a key with no translation can be its own
    /// answer: the page shows the key, which is how a missing entry gets
    /// noticed instead of leaking or panicking.
    fn t(locale: Locale, key: &'static str) -> &'static str {
        match (locale, key) {
            (Locale::English, "catalogue") => "catalogue",
            (Locale::English, "status") => "support at a glance",
            (Locale::English, "samples") => "text samples",
            (Locale::English, "samples-heading") => "twenty texts, drawn twice",
            (Locale::English, "samples-dom") => "browser",
            (Locale::English, "samples-gpu") => "region",
            (Locale::English, "samples-note") => {
                "The same string in both columns. A reviewer compares them with a \
                     reference rendering: order, direction, and whether anything is a box."
            }
            (Locale::English, "presentation") => "presentation",
            (Locale::English, "capabilities") => "capabilities",
            (Locale::English, "dom") => "DOM",
            (Locale::English, "gpu") => "GPU",
            (Locale::English, "across") => "across regions",
            (Locale::English, "theme") => "theme",
            (Locale::English, "language") => "language",
            (Locale::English, "motion") => "motion",
            (Locale::English, "less") => "less",
            (Locale::English, "full") => "full",
            (Locale::English, "example") => "example",
            (Locale::English, "states") => "states",
            (Locale::English, "default") => "default",
            (Locale::English, "disabled") => "disabled",
            (Locale::English, "read-only") => "read-only",
            (Locale::English, "invalid") => "invalid",
            (Locale::Chinese, "catalogue") => "组件目录",
            (Locale::Chinese, "status") => "能力总表",
            (Locale::Chinese, "samples") => "文本样本",
            (Locale::Chinese, "samples-heading") => "二十条文本，各画两遍",
            (Locale::Chinese, "samples-dom") => "浏览器",
            (Locale::Chinese, "samples-gpu") => "区域",
            (Locale::Chinese, "samples-note") => {
                "两列是同一个字符串。评审人对照参考渲染逐条判断：顺序、方向，以及有没有变成方框。"
            }
            (Locale::Chinese, "presentation") => "呈现",
            (Locale::Chinese, "capabilities") => "能力",
            (Locale::Chinese, "dom") => "DOM",
            (Locale::Chinese, "gpu") => "GPU",
            (Locale::Chinese, "across") => "跨区",
            (Locale::Chinese, "theme") => "主题",
            (Locale::Chinese, "language") => "语言",
            (Locale::Chinese, "motion") => "动效",
            (Locale::Chinese, "less") => "减少",
            (Locale::Chinese, "full") => "完整",
            (Locale::Chinese, "example") => "示例",
            (Locale::Chinese, "states") => "状态",
            (Locale::Chinese, "default") => "默认",
            (Locale::Chinese, "disabled") => "禁用",
            (Locale::Chinese, "read-only") => "只读",
            (Locale::Chinese, "invalid") => "错误",
            (_, other) => other,
        }
    }

    /// Which page the catalogue is showing. In-memory: the URL belongs to the
    /// router, which is P2 M4, and a nav that pretended to own it before then
    /// would have to be undone.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum Page {
        Category(usize),
        Status,
        /// B5: the twenty fixed texts, drawn by the browser and by the region
        /// side by side, for a reviewer to compare against a reference.
        Samples,
    }

    impl Page {
        fn path(self) -> String {
            match self {
                Self::Category(index) => {
                    format!("/{}", CATALOG[index].category.name().replace(' ', "-"))
                }
                Self::Status => "/status".to_string(),
                Self::Samples => "/samples".to_string(),
            }
        }
    }

    /// The three choices in the radio group, the three tabs and the three
    /// options in the chooser. Named once so both halves agree on the order.
    const CHOICES: [&str; 3] = ["first", "second", "third"];
    const SIZES: [&str; 3] = ["small", "medium", "large"];

    /// Everything the examples on a page are bound to.
    ///
    /// One value per control rather than one per example: the point of the
    /// page is that the DOM control, the region's control and the three
    /// examples in the state matrix are all showing the *same* value.
    #[derive(Clone, Copy)]
    struct Values {
        checked: RwSignal<bool>,
        chosen: RwSignal<usize>,
        on: RwSignal<bool>,
        size: RwSignal<f64>,
        fraction: RwSignal<f64>,
        spinning: RwSignal<bool>,
        tab: RwSignal<usize>,
        text: RwSignal<String>,
        notes: RwSignal<String>,
        chooser: RwSignal<usize>,
        tooltip: RwSignal<bool>,
        menu: RwSignal<bool>,
        dialog: RwSignal<bool>,
        select: RwSignal<bool>,
        /// How many changes the application has accepted. A disabled or
        /// read-only control must not move it, which is what V2 asks.
        actions: RwSignal<u32>,
    }

    impl Values {
        fn accept(&self) {
            self.actions.update(|count| *count += 1);
        }
    }

    /// Which of the four states an example is rendered in.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum ExampleState {
        Live,
        Disabled,
        ReadOnly,
        Invalid,
    }

    impl ExampleState {
        fn name(self) -> &'static str {
            match self {
                Self::Live => "default",
                Self::Disabled => "disabled",
                Self::ReadOnly => "read-only",
                Self::Invalid => "invalid",
            }
        }

        fn disabled(self) -> bool {
            self == Self::Disabled
        }

        fn read_only(self) -> bool {
            self == Self::ReadOnly
        }

        fn invalid(self) -> bool {
            self == Self::Invalid
        }
    }

    thread_local! {
        static HANDLES: RefCell<BTreeMap<u32, AppHandle>> = const { RefCell::new(BTreeMap::new()) };
        static NEXT_HANDLE: RefCell<u32> = const { RefCell::new(1) };
        static SNAPSHOT: RefCell<String> = const { RefCell::new(String::new()) };
        /// How to put each mounted scope back the way it loaded, by handle.
        static RESETS: RefCell<BTreeMap<u32, Rc<dyn Fn()>>> = const { RefCell::new(BTreeMap::new()) };
    }

    /// The application's own state, each piece with the value it starts with.
    ///
    /// A test resets the page between checks rather than loading it again:
    /// a load is dominated by starting the region, and so is a remount. So the
    /// scope and its region stay, and the application puts back what it owns.
    /// The first value is given once, where the signal is created, so a new
    /// piece of state cannot be added without saying what it resets to.
    #[derive(Default)]
    struct FirstLoad(Vec<Box<dyn Fn()>>);

    impl FirstLoad {
        /// A signal that a reset puts back to `initial`.
        fn signal<T: Clone + Send + Sync + 'static>(&mut self, initial: T) -> RwSignal<T> {
            let signal = RwSignal::new(initial.clone());
            self.0.push(Box::new(move || signal.set(initial.clone())));
            signal
        }

        /// State that is not a plain signal of this scope's.
        fn then(&mut self, reset: impl Fn() + 'static) {
            self.0.push(Box::new(reset));
        }

        /// Makes the resets reachable under the scope's handle, for as long
        /// as the scope lives.
        fn register(self, handle: u32) {
            let resets = self.0;
            let reset: Rc<dyn Fn()> = Rc::new(move || resets.iter().for_each(|reset| reset()));
            RESETS.with(|table| table.borrow_mut().insert(handle, reset));
            on_cleanup(move || {
                RESETS.with(|table| table.borrow_mut().remove(&handle));
            });
        }
    }

    fn chip(support: Support) -> ChipVariant {
        match support {
            Support::Yes => ChipVariant::Yes,
            Support::Partial => ChipVariant::Partial,
            Support::No => ChipVariant::No,
        }
    }

    /// A string as a JSON literal, by the browser's own encoder: whatever a
    /// reader typed into a field cannot break the snapshot it is reported in.
    fn json_string(text: &str) -> String {
        js_sys::JSON::stringify(&JsValue::from_str(text))
            .ok()
            .and_then(|json| json.as_string())
            .unwrap_or_else(|| "\"\"".to_string())
    }

    fn rect_json(rect: Option<LocalRect>) -> String {
        match rect {
            Some(rect) => format!(
                "{{\"x\":{:.1},\"y\":{:.1},\"width\":{:.1},\"height\":{:.1}}}",
                rect.x, rect.y, rect.width, rect.height
            ),
            None => "null".to_string(),
        }
    }
    /// The states this category actually has, beyond the ordinary one.
    ///
    /// Not every control has all three, and a matrix that renders a state a
    /// component does not have shows the same picture twice and calls one of
    /// them "invalid". The overlay categories have none of them here: four
    /// tooltips or four dialogs open at once are four layers over each other.
    fn states(category: Category) -> &'static [ExampleState] {
        use ExampleState::{Disabled, Invalid, ReadOnly};
        match category {
            Category::Button | Category::Link => &[Disabled],
            Category::TextField | Category::TextArea | Category::Checkbox => {
                &[Disabled, ReadOnly, Invalid]
            }
            Category::Radio | Category::Switch | Category::Slider => &[Disabled, ReadOnly],
            _ => &[],
        }
    }

    /// One category, drawn in one state, bound to the page's values.
    #[component]
    fn Example(category: Category, state: ExampleState, values: Values) -> impl IntoView {
        let disabled = Signal::derive(move || state.disabled());
        let read_only = Signal::derive(move || state.read_only());
        let invalid = Signal::derive(move || state.invalid());
        let prefix = state.name();
        let id = move |name: &str| format!("{prefix}-{name}");
        let trigger = NodeRef::<leptos::html::Span>::new();
        match category {
            Category::Button => view! {
                <Button
                    test_id=id("button")
                    disabled=disabled
                    on_click=move || values.accept()
                >
                    "press me"
                </Button>
            }
            .into_any(),
            Category::Label => view! {
                <Label control=id("named") test_id=id("label")>"a named field"</Label>
                <TextField
                    id=id("named")
                    test_id=id("label-field")
                    value=values.text
                    on_change=move |next| {
                        values.text.set(next);
                        values.accept();
                    }
                />
            }
            .into_any(),
            Category::Link => view! {
                <Link href="/link" test_id=id("link") disabled=disabled>
                    "a link to somewhere"
                </Link>
            }
            .into_any(),
            Category::Icon => view! {
                <Row>
                    <Icon glyph=Glyph::Check label="done" test_id=id("icon-check") />
                    <Icon glyph=Glyph::ChevronDown test_id=id("icon-chevron") />
                    <Icon glyph=Glyph::Close label="close" test_id=id("icon-close") />
                </Row>
            }
            .into_any(),
            Category::TextField => view! {
                <TextField
                    test_id=id("text-field")
                    aria_label="a text field"
                    value=values.text
                    disabled=disabled
                    read_only=read_only
                    invalid=invalid
                    on_change=move |next| {
                        values.text.set(next);
                        values.accept();
                    }
                />
            }
            .into_any(),
            Category::TextArea => view! {
                <TextArea
                    test_id=id("text-area")
                    aria_label="notes"
                    rows=3
                    value=values.notes
                    disabled=disabled
                    read_only=read_only
                    invalid=invalid
                    on_change=move |next| {
                        values.notes.set(next);
                        values.accept();
                    }
                />
            }
            .into_any(),
            Category::Checkbox => view! {
                <Checkbox
                    test_id=id("checkbox")
                    aria_label="a checkbox"
                    checked=values.checked
                    disabled=disabled
                    read_only=read_only
                    invalid=invalid
                    on_change=move |next| {
                        values.checked.set(next);
                        values.accept();
                    }
                />
            }
            .into_any(),
            Category::Radio => view! {
                <RadioGroup
                    test_id=id("radio")
                    aria_label="a choice of three"
                    value=Signal::derive(move || CHOICES[values.chosen.get()].to_string())
                    options=Signal::derive(|| {
                        CHOICES.iter().map(|name| RadioOption::new(*name, *name)).collect()
                    })
                    disabled=disabled
                    read_only=read_only
                    on_change=move |next: String| {
                        if let Some(index) = CHOICES.iter().position(|name| *name == next) {
                            values.chosen.set(index);
                            values.accept();
                        }
                    }
                />
            }
            .into_any(),
            Category::Switch => view! {
                <Switch
                    test_id=id("switch")
                    aria_label="a switch"
                    checked=values.on
                    disabled=disabled
                    read_only=read_only
                    on_change=move |next| {
                        values.on.set(next);
                        values.accept();
                    }
                />
            }
            .into_any(),
            Category::Select => view! {
                <Select
                    test_id=id("select")
                    aria_label="a size"
                    value=Signal::derive(move || SIZES[values.chooser.get()].to_string())
                    options=Signal::derive(|| {
                        SIZES.iter().map(|name| SelectOption::new(*name, *name)).collect()
                    })
                    open=values.select
                    on_open_change=move |open| values.select.set(open)
                    disabled=disabled
                    read_only=read_only
                    invalid=invalid
                    on_change=move |next: String| {
                        if let Some(index) = SIZES.iter().position(|name| *name == next) {
                            values.chooser.set(index);
                            values.accept();
                        }
                    }
                />
            }
            .into_any(),
            Category::Slider => view! {
                <Slider
                    test_id=id("slider")
                    aria_label="a size"
                    value=values.size
                    min=0.0
                    max=100.0
                    step=5.0
                    disabled=disabled
                    read_only=read_only
                    on_change=move |next| {
                        values.size.set(next);
                        values.accept();
                    }
                />
            }
            .into_any(),
            Category::Progress => view! {
                <Progress
                    test_id=id("progress")
                    aria_label="how far along"
                    value=Signal::derive(move || values.fraction.get() * 100.0)
                />
                <Progress test_id=id("progress-unknown") aria_label="working" indeterminate=true />
            }
            .into_any(),
            Category::Loading => view! {
                <Row>
                    <Spinner test_id=id("spinner") label="loading" />
                    <Button
                        test_id=id("spinner-toggle")
                        on_click=move || {
                            values.spinning.update(|spinning| *spinning = !*spinning);
                            values.accept();
                        }
                    >
                        "start or stop"
                    </Button>
                </Row>
            }
            .into_any(),
            Category::Tooltip => view! {
                <Tooltip
                    test_id=id("tooltip")
                    label="what this button does"
                    open=values.tooltip
                    on_open_change=move |open| values.tooltip.set(open)
                >
                    <Button test_id=id("tooltip-trigger") on_click=move || values.accept()>
                        "hover or focus me"
                    </Button>
                </Tooltip>
            }
            .into_any(),
            Category::Menu => view! {
                <span node_ref=trigger>
                    <Button
                        test_id=id("menu-trigger")
                        expanded=Signal::derive(move || Some(values.menu.get()))
                        on_click=move || values.menu.update(|open| *open = !*open)
                    >
                        "open the menu"
                    </Button>
                </span>
                <Menu
                    test_id=id("menu")
                    aria_label="what can be done"
                    open=values.menu
                    on_open_change=move |open| values.menu.set(open)
                    anchor=Signal::derive(move || match trigger.get() {
                        Some(element) => Anchor::element(&element.into()),
                        None => Anchor::Centred,
                    })
                    items=Signal::derive(|| {
                        vec![
                            MenuItem::new("first", "the first command"),
                            MenuItem::new("second", "the second command"),
                            MenuItem::new("third", "the third command")
                                .disabled("nothing is selected"),
                        ]
                    })
                    on_activate=move |_: String| values.accept()
                />
            }
            .into_any(),
            Category::Dialog => view! {
                <Button
                    test_id=id("dialog-trigger")
                    on_click=move || values.dialog.set(true)
                >
                    "open the dialog"
                </Button>
                <Show when=move || values.dialog.get() fallback=|| ()>
                    <Dialog
                        test_id="dialog"
                        title="a modal dialog"
                        description="the rest of the scope is inert while this is open"
                        open=values.dialog
                        on_open_change=move |open| values.dialog.set(open)
                    >
                        <Button test_id="dialog-accept" on_click=move || {
                            values.accept();
                            values.dialog.set(false);
                        }>
                            "accept"
                        </Button>
                    </Dialog>
                </Show>
            }
            .into_any(),
            Category::Tabs => view! {
                <Tabs
                    test_id=id("tabs")
                    aria_label="three tabs"
                    active=Signal::derive(move || CHOICES[values.tab.get()].to_string())
                    tabs=Signal::derive(|| {
                        vec![
                            Tab::new(CHOICES[0], CHOICES[0]),
                            Tab::new(CHOICES[1], CHOICES[1]),
                            Tab::new(CHOICES[2], CHOICES[2]),
                        ]
                    })
                    on_activate=move |next: String| {
                        if let Some(index) = CHOICES.iter().position(|name| *name == next) {
                            values.tab.set(index);
                            values.accept();
                        }
                    }
                >
                    <TabPanel value=CHOICES[0]>"what the first tab holds"</TabPanel>
                    <TabPanel value=CHOICES[1]>"what the second tab holds"</TabPanel>
                    <TabPanel value=CHOICES[2]>"what the third tab holds"</TabPanel>
                </Tabs>
            }
            .into_any(),
            // A table of a thousand rows, so that what the page shows is a
            // windowed table rather than a picture of one, and a tree over the
            // same thousand.
            Category::DataTable => {
                let rows = RwSignal::new(1_000usize);
                let columns = RwSignal::new(
                    (0..4)
                        .map(|index| {
                            Column::new(format!("c{index}"), format!("column {}", index + 1))
                        })
                        .collect::<Vec<_>>(),
                );
                let chosen = RwSignal::new(Selection::new());
                let focus = RwSignal::new(GridCell::default());
                let goto = RwSignal::new(None);
                view! {
                    <DataTable
                        test_id=id("data-table")
                        aria_label="a thousand rows"
                        class="h-48"
                        rows=rows
                        columns=columns
                        cell=Arc::new(|row: usize, column: usize| format!("r{row}c{column}"))
                        row_id=Arc::new(|row: usize| row as u32 + 1)
                        selected=chosen
                        goto=goto
                        focus=focus
                        on_select=move |row: usize| {
                            chosen.update(|selection| {
                                selection.toggle(row as u32 + 1);
                            });
                            values.accept();
                        }
                        on_activate=move |_row: usize| values.accept()
                    />
                }
                .into_any()
            }
            Category::Tree => {
                let nodes = RwSignal::new(
                    (0..3)
                        .map(|group| {
                            TreeNode::new(format!("g{group}"), format!("group {}", group + 1)).with(
                                (0..3)
                                    .map(|child| {
                                        TreeNode::new(
                                            format!("g{group}-{child}"),
                                            format!("group {}.{}", group + 1, child + 1),
                                        )
                                    })
                                    .collect(),
                            )
                        })
                        .collect::<Vec<_>>(),
                );
                let expanded = RwSignal::new(BTreeSet::from(["g0".to_string()]));
                let picked = RwSignal::new(Option::<String>::None);
                view! {
                    <Tree
                        test_id=id("tree")
                        aria_label="groups"
                        class="h-48"
                        nodes=nodes
                        expanded=expanded
                        selected=picked
                        on_select=move |key: String| {
                            picked.set(Some(key));
                            values.accept();
                        }
                    />
                }
                .into_any()
            }
            Category::ScrollArea => view! {
                <ScrollArea
                    test_id=id("scroll-area")
                    aria_label="a list that does not fit"
                    boundary=Boundary::Stop
                    class="h-24 border border-border p-2"
                >
                    {(1..=20)
                        .map(|line| view! { <p class="text-sm">{format!("line {line}")}</p> })
                        .collect_view()}
                </ScrollArea>
            }
            .into_any(),
        }
    }

    #[component]
    fn CategoryPage(index: usize, locale: Signal<Locale>, values: Values) -> impl IntoView {
        let entry = &CATALOG[index];
        let category = entry.category;
        let presentation = [
            ("dom", entry.presentation.dom),
            ("gpu", entry.presentation.gpu),
            ("across", entry.presentation.across_regions),
        ];
        view! {
            <Panel test_id="category-page">
                <h2 data-testid="category-name">{entry.category.name()}</h2>
                <h3>{move || t(locale.get(), "example")}</h3>
                <div data-testid="example" class="catalogue-example">
                    <Example category=category state=ExampleState::Live values=values />
                </div>
                <p data-testid="gpu-note" class="text-sm text-muted-foreground">
                    {entry.environment.note}
                </p>
                <Show when=move || !states(category).is_empty() fallback=|| ()>
                    <h3>{move || t(locale.get(), "states")}</h3>
                    <div data-testid="state-matrix" class="catalogue-states">
                        {states(category)
                            .iter()
                            .copied()
                            .map(|state| {
                                view! {
                                    <div data-testid=format!("state-{}", state.name())>
                                        <span class="text-xs text-muted-foreground">
                                            {move || t(locale.get(), state.name())}
                                        </span>
                                        <Example
                                            category=category
                                            state=state
                                            values=values
                                        />
                                    </div>
                                }
                            })
                            .collect_view()}
                    </div>
                </Show>
                <h3>{move || t(locale.get(), "presentation")}</h3>
                <Row>
                    {presentation
                        .into_iter()
                        .map(|(key, support)| {
                            view! {
                                <Row>
                                    <span>{move || t(locale.get(), key)}</span>
                                    <Chip
                                        variant=chip(support)
                                        test_id=format!("presentation-{key}")
                                    >
                                        {support.name()}
                                    </Chip>
                                </Row>
                            }
                        })
                        .collect_view()}
                </Row>
                <h3>{move || t(locale.get(), "capabilities")}</h3>
                <ul>
                    {entry
                        .capabilities()
                        .into_iter()
                        .map(|(name, capability)| {
                            view! {
                                <li>
                                    <Row>
                                        <strong>{name}</strong>
                                        <Chip
                                            variant=chip(capability.support)
                                            test_id=format!("capability-{name}")
                                        >
                                            {capability.support.name()}
                                        </Chip>
                                    </Row>
                                    <p>{capability.note}</p>
                                </li>
                            }
                        })
                        .collect_view()}
                </ul>
            </Panel>
        }
    }

    /// A number the way the reader's language writes it.
    ///
    /// The options are the application's, which is the whole contract: this
    /// page happens to want a currency with two decimals, and the SDK formats
    /// nothing of its own.
    fn formatted_number(locale: Locale) -> String {
        let options = js_sys::Object::new();
        let set = |key: &str, value: &str| {
            let _ =
                js_sys::Reflect::set(&options, &JsValue::from_str(key), &JsValue::from_str(value));
        };
        set("style", "currency");
        set("currency", "CNY");
        rustify_ui::format_number(locale, 1_234_567.891, &options)
    }

    /// A fixed instant, so the page reads the same on every run: the first of
    /// March 2026, noon UTC.
    fn formatted_date(locale: Locale) -> String {
        let options = js_sys::Object::new();
        let set = |key: &str, value: &str| {
            let _ =
                js_sys::Reflect::set(&options, &JsValue::from_str(key), &JsValue::from_str(value));
        };
        set("dateStyle", "long");
        set("timeZone", "UTC");
        rustify_ui::format_date(locale, 1_772_366_400_000.0, &options)
    }

    /// B5: the twenty texts, drawn by the browser and by a region beside it.
    ///
    /// A second region on the page rather than the header's: the header's
    /// region is showing whichever control the reader is looking at, and a
    /// page that borrowed it would have to put it back.
    #[component]
    fn SamplesPage(locale: Signal<Locale>) -> impl IntoView {
        let theme = use_theme_values();
        // Whether the wide font arrived. Read from the runtime's own record
        // rather than guessed: a font that failed is reported there as
        // `AssetLoadFailed`, and this page is the one that has to say so.
        let fonts = RwSignal::new(true);
        // Whether the page has been told to behave as though the font that
        // covers more than Latin never arrived. A control rather than a hidden
        // switch: this is the page where a reader would want to see what a
        // missing font looks like, and the real path - a resource that fails -
        // is exercised against a real broken file by the deployment build.
        let blocked = RwSignal::new(false);
        let lines = RwSignal::new(0usize);
        // How many samples the region replaced with the missing-glyph message
        // in its last draw. Reported by the region, because it is the only one
        // that knows what it drew.
        let marked = RwSignal::new(0usize);
        let region = RwSignal::new(RegionState::Starting);
        Effect::new(move || {
            // Tracked: the count rises when an asset does not arrive, whenever
            // that happens, so a font that fails after the region is already
            // drawing still reaches this page.
            rustify_ui::asset_failures();
            region.get();
            let failed = rustify_ui::with_log(|log| {
                log.entries().any(|entry| {
                    entry.kind == rustify_ui::ErrorKind::AssetLoadFailed
                        && entry
                            .asset
                            .as_deref()
                            .is_some_and(|asset| asset.ends_with(".ttf"))
                })
            });
            fonts.set(!failed);
        });
        let props = Signal::derive(move || crate::sample_region::SampleProps {
            theme: theme.get().unwrap_or_else(Theme::light),
            missing: (!fonts.get() || blocked.get()).then(|| {
                rustify_ui::Message::MissingGlyph
                    .text(locale.get())
                    .to_string()
            }),
        });
        let on_action = move |action| match action {
            crate::sample_region::SampleAction::Drew {
                lines: drawn,
                marked: count,
            } => {
                lines.set(drawn.len());
                marked.set(count);
            }
        };
        // Bound out here: `view!` cannot parse a turbofish in an attribute.
        let app = PhantomData::<crate::sample_region::SampleRegion>;
        view! {
            // A plain section rather than `Panel`: a component's children have
            // to be `Send`, and a GPU region's application marker is not. The
            // same finding as the splitter's in M5, and the same answer - the
            // page holds the region itself and borrows only the classes.
            <section
                class="rounded-md border border-border bg-card p-4"
                data-testid="samples-page"
            >
                <h2>{move || t(locale.get(), "samples-heading")}</h2>
                <p data-testid="samples-note">{move || t(locale.get(), "samples-note")}</p>
                <h3>{move || t(locale.get(), "messages-heading")}</h3>
                <ul class="messages" data-testid="sdk-messages">
                    {rustify_ui::Message::all()
                        .into_iter()
                        .map(|message| {
                            view! {
                                <li data-testid=format!("message-{}", message.key())>
                                    {move || message.text(locale.get())}
                                </li>
                            }
                        })
                        .collect_view()}
                </ul>
                <p data-testid="format-number">{move || formatted_number(locale.get())}</p>
                <p data-testid="format-date">{move || formatted_date(locale.get())}</p>
                // What the two halves say about each other, in order: what the
                // page asked for, what it projected, and what the region drew.
                // Three lines rather than one, because when this page went
                // wrong it was the *middle* one that had stopped - the switch
                // moved and the projection did not - and neither of the other
                // two could have said so.
                <dl class="samples-state">
                    <dt>"asked to block the wide font"</dt>
                    <dd data-testid="samples-blocked">{move || blocked.get().to_string()}</dd>
                    <dt>"projected into the region"</dt>
                    <dd data-testid="samples-asked">
                        {move || props.get().missing.is_some().to_string()}
                    </dd>
                    <dt>"lines drawn"</dt>
                    <dd data-testid="samples-drawn">
                        {move || format!("{} / {}", lines.get(), crate::samples::SAMPLES.len())}
                    </dd>
                    <dt>"drawn as the missing-glyph message"</dt>
                    <dd data-testid="samples-marked">{move || marked.get()}</dd>
                </dl>
                <Row>
                    <Checkbox
                        test_id="block-font"
                        aria_label="draw as though the wide font never arrived"
                        checked=Signal::derive(move || blocked.get())
                        on_change=move |value: bool| blocked.set(value)
                    />
                    <span class="text-sm">
                        "draw as though the wide font never arrived"
                    </span>
                </Row>
                <div class="samples">
                    <div class="samples-column">
                        <h3>{move || t(locale.get(), "samples-dom")}</h3>
                        {crate::samples::SAMPLES
                            .iter()
                            .map(|sample| {
                                view! {
                                    <p
                                        class="sample"
                                        lang=if sample.rtl { "ar" } else { "" }
                                        dir=if sample.rtl { "rtl" } else { "ltr" }
                                        data-testid=format!("sample-{}", sample.id)
                                        title=sample.label
                                    >
                                        {sample.text}
                                    </p>
                                }
                            })
                            .collect_view()}
                    </div>
                    <div class="samples-column">
                        <h3>{move || t(locale.get(), "samples-gpu")}</h3>
                        <GpuRegion
                            app=app
                            props=props
                            state=region
                            on_action=on_action
                            class="samples-canvas"
                            test_id="samples-region"
                        />
                    </div>
                </div>
            </section>
        }
    }

    #[component]
    fn StatusPage(locale: Signal<Locale>) -> impl IntoView {
        view! {
            <Panel test_id="status-page">
                <h2>{move || t(locale.get(), "status")}</h2>
                <table data-testid="status-table">
                    <thead>
                        <tr>
                            <th scope="col">{move || t(locale.get(), "catalogue")}</th>
                            <th scope="col">{move || t(locale.get(), "dom")}</th>
                            <th scope="col">{move || t(locale.get(), "gpu")}</th>
                            <th scope="col">{move || t(locale.get(), "across")}</th>
                        </tr>
                    </thead>
                    <tbody>
                        {CATALOG
                            .iter()
                            .map(|entry| {
                                view! {
                                    <tr data-testid=format!(
                                        "status-row-{}",
                                        entry.category.name().replace(' ', "-"),
                                    )>
                                        <th scope="row">{entry.category.name()}</th>
                                        <td>{entry.presentation.dom.name()}</td>
                                        <td>{entry.presentation.gpu.name()}</td>
                                        <td>{entry.presentation.across_regions.name()}</td>
                                    </tr>
                                }
                            })
                            .collect_view()}
                    </tbody>
                </table>
            </Panel>
        }
    }

    /// The catalogue, as one scope. `handle` is what JS knows the scope by,
    /// and so what its reset is registered under.
    #[component]
    fn Catalogue(handle: u32) -> impl IntoView {
        // What mirrors the region - its state and the rectangles it reports -
        // is created plainly: the region stays through a reset and goes on
        // reporting, so those are not the application's to put back. The
        // rest starts through `first`.
        let mut first = FirstLoad::default();
        let theme = first.signal(Theme::light());
        // The scope's language, in context: the SDK's own components read it
        // too, so a dialog's close button and this page's headings cannot end
        // up in two different languages.
        let first_locale = Locale::English;
        let locale = rustify_ui::provide_locale(first_locale);
        first.then(move || locale.set(first_locale));
        let locale_signal = Signal::derive(move || locale.get());
        // Every page's own state - a table's selection, a tree's expansion,
        // whether the samples page blocks its font - lives in that page and
        // is rebuilt when this is set, even to the page it already shows.
        let page = first.signal(Page::Category(0));
        let region = RwSignal::new(RegionState::Starting);
        // Where the region says it drew its own controls, so a test can put a
        // real pointer on one instead of guessing from the layout.
        let region_button = RwSignal::new(None::<LocalRect>);
        let region_control = RwSignal::new(None::<LocalRect>);
        // The canvas, so a layer can be anchored to a rectangle *inside* the
        // region rather than to the page.
        let canvas = NodeRef::<leptos::html::Canvas>::new();
        // Where the region drew its chooser, while its list is open. The list
        // is a DOM layer: a popup cannot leave the canvas.
        // Reported by the region, but whether the list is open is the page's
        // decision, and a reset closes it.
        let region_chooser = first.signal(None::<LocalRect>);
        let values = Values {
            checked: first.signal(false),
            chosen: first.signal(0),
            on: first.signal(false),
            size: first.signal(40.0),
            fraction: first.signal(0.35),
            spinning: first.signal(true),
            tab: first.signal(0),
            text: first.signal("hello".to_string()),
            notes: first.signal("two\nlines".to_string()),
            chooser: first.signal(1),
            tooltip: first.signal(false),
            menu: first.signal(false),
            dialog: first.signal(false),
            select: first.signal(false),
            actions: first.signal(0),
        };
        first.register(handle);
        provide_current_path(Signal::derive(move || page.get().path()));

        let switch_theme = move || {
            theme.update(|theme| {
                *theme = if theme.name == "light" {
                    Theme::dark()
                } else {
                    Theme::light()
                };
            });
        };

        // A memo rather than a derived signal: it fires only when the answer
        // changes, and what depends on it below is a *change* of category.
        let category = Memo::new(move |_| match page.get() {
            Page::Category(index) => CATALOG[index].category,
            // Neither the status page nor the samples page draws a control,
            // and a label saying so is a truer answer than the last page's
            // control left on screen.
            Page::Status | Page::Samples => Category::Label,
        });

        // The reported rectangle belongs to the control the region is drawing
        // now. Clearing it when the category changes means a caller waits for
        // the new one rather than aiming at the old one - and not clearing it
        // when the category has *not* changed means a caller is not left
        // waiting for a report the region has no reason to send.
        Effect::new(move || {
            category.get();
            region_control.set(None);
        });

        Effect::new(move || {
            let page = page.get();
            let snapshot = format!(
                "{{\"path\":\"{}\",\"theme\":\"{}\",\"locale\":\"{}\",\"categories\":{},\
                 \"button\":{},\"control\":{},\"region\":\"{}\",\"checked\":{},\"chosen\":{},\
                 \"on\":{},\"size\":{},\"fraction\":{:.2},\"spinning\":{},\"tab\":{},\
                 \"chooser\":\"{}\",\"text\":{},\"actions\":{},\"reduce_motion\":{},\
                 \"notes\":{},\"tooltip\":{},\"menu\":{},\"dialog\":{},\"listbox\":{},\
                 \"region_list\":{}}}",
                page.path(),
                theme.get().name,
                locale.get().tag(),
                CATALOG.len(),
                rect_json(region_button.get()),
                rect_json(region_control.get()),
                match region.get() {
                    RegionState::Starting => "starting",
                    RegionState::Ready => "ready",
                    RegionState::Suspended => "suspended",
                    RegionState::Lost => "lost",
                    RegionState::Failed(_) => "failed",
                    RegionState::Disposed => "disposed",
                },
                values.checked.get(),
                values.chosen.get(),
                values.on.get(),
                values.size.get(),
                values.fraction.get(),
                values.spinning.get(),
                values.tab.get(),
                SIZES[values.chooser.get()],
                json_string(&values.text.get()),
                values.actions.get(),
                theme.get().reduce_motion,
                json_string(&values.notes.get()),
                values.tooltip.get(),
                values.menu.get(),
                values.dialog.get(),
                values.select.get(),
                region_chooser.get().is_some(),
            );
            SNAPSHOT.with(|slot| *slot.borrow_mut() = snapshot);
        });

        let props = Signal::derive(move || CatalogProps {
            theme: theme.get(),
            category: category.get(),
            checked: values.checked.get(),
            chosen: values.chosen.get(),
            on: values.on.get(),
            size: values.size.get(),
            fraction: values.fraction.get(),
            spinning: values.spinning.get(),
            tab: values.tab.get(),
            chooser: SIZES[values.chooser.get()].to_string(),
            text: values.text.get(),
            note: CATALOG
                .iter()
                .find(|entry| entry.category == category.get())
                .map(|entry| entry.environment.note.to_string())
                .unwrap_or_default(),
            disabled: false,
            read_only: false,
        });
        // The view macro parses attribute values as expressions, and a
        // turbofish or a `match` arm inside one is not one of them.
        let region_app = PhantomData::<CatalogRegion>;
        let on_region_action = move |action| match action {
            CatalogAction::Toggle => switch_theme(),
            CatalogAction::Button(rect) => region_button.set(Some(rect)),
            CatalogAction::Control(rect) => region_control.set(Some(rect)),
            CatalogAction::SetChecked(next) => {
                values.checked.set(next);
                values.accept();
            }
            CatalogAction::Choose(index) => {
                values.chosen.set(index);
                values.accept();
            }
            CatalogAction::SetOn(next) => {
                values.on.set(next);
                values.accept();
            }
            CatalogAction::SetSize(next) => {
                values.size.set(next);
                values.accept();
            }
            CatalogAction::SetTab(index) => {
                values.tab.set(index);
                values.accept();
            }
            // The region drew the closed chooser and asked for the list. The
            // list is a DOM layer anchored to the rectangle the region
            // reported, which is how a popup that cannot leave the canvas is
            // still drawn against something inside it.
            CatalogAction::OpenChooser(rect) => region_chooser.set(Some(rect)),
        };

        view! {
            <ThemedScope theme=theme />
            <div class="catalogue" data-testid="catalogue">
                <header class="catalogue-header">
                    <h1>{move || t(locale.get(), "catalogue")}</h1>
                    <Row>
                        <Button
                            test_id="toggle-theme"
                            aria_label="switch theme"
                            on_click=switch_theme
                        >
                            {move || format!("{}: {}", t(locale.get(), "theme"), theme.get().name)}
                        </Button>
                        <Switch
                            test_id="toggle-motion"
                            aria_label="reduce motion"
                            checked=Signal::derive(move || theme.get().reduce_motion)
                            on_change=move |less| {
                                theme.update(|theme| theme.reduce_motion = less)
                            }
                        />
                        <span class="text-sm">
                            {move || {
                                let locale = locale.get();
                                format!(
                                    "{}: {}",
                                    t(locale, "motion"),
                                    if theme.get().reduce_motion {
                                        t(locale, "less")
                                    } else {
                                        t(locale, "full")
                                    },
                                )
                            }}
                        </span>
                        <Button
                            test_id="toggle-locale"
                            aria_label="switch language"
                            variant=ButtonVariant::Secondary
                            on_click=move || {
                                locale
                                    .set(match locale.get_untracked() {
                                        Locale::English => Locale::Chinese,
                                        Locale::Chinese => Locale::English,
                                    })
                            }
                        >
                            {move || {
                                format!("{}: {}", t(locale.get(), "language"), locale.get().tag())
                            }}
                        </Button>
                    </Row>
                    <div class="catalogue-region" data-testid="catalogue-region-host">
                        <GpuRegion
                            app=region_app
                            props=props
                            state=region
                            on_action=on_region_action
                            node_ref=canvas
                            class="catalogue-canvas"
                            test_id="catalogue-region"
                        />
                    </div>
                </header>
                <nav class="catalogue-nav" aria-label="catalogue">
                    <ul>
                        {CATALOG
                            .iter()
                            .enumerate()
                            .map(|(index, entry)| {
                                let name = entry.category.name();
                                view! {
                                    <li>
                                        <Button
                                            test_id=format!("nav-{}", name.replace(' ', "-"))
                                            variant=ButtonVariant::Ghost
                                            on_click=move || page.set(Page::Category(index))
                                        >
                                            {name}
                                        </Button>
                                    </li>
                                }
                            })
                            .collect_view()}
                        <li>
                            <Button
                                test_id="nav-status"
                                variant=ButtonVariant::Ghost
                                on_click=move || page.set(Page::Status)
                            >
                                {move || t(locale.get(), "status")}
                            </Button>
                        </li>
                        <li>
                            <Button
                                test_id="nav-samples"
                                variant=ButtonVariant::Ghost
                                on_click=move || page.set(Page::Samples)
                            >
                                {move || t(locale.get(), "samples")}
                            </Button>
                        </li>
                    </ul>
                </nav>
                <main class="catalogue-main">
                    {move || match page.get() {
                        Page::Category(index) => {
                            view! {
                                <CategoryPage
                                    index=index
                                    locale=locale_signal
                                    values=values
                                />
                            }
                                .into_any()
                        }
                        Page::Status => view! { <StatusPage locale=locale_signal /> }.into_any(),
                        Page::Samples => {
                            view! { <SamplesPage locale=locale_signal /> }.into_any()
                        }
                    }}
                </main>
                <Menu
                    test_id="region-chooser"
                    aria_label="a size, chosen from the region"
                    open=Signal::derive(move || region_chooser.get().is_some())
                    on_open_change=move |open| {
                        if !open {
                            region_chooser.set(None);
                        }
                    }
                    anchor=Signal::derive(move || {
                        match (canvas.get(), region_chooser.get()) {
                            (Some(canvas), Some(rect)) => Anchor::region(&canvas.into(), rect),
                            _ => Anchor::Centred,
                        }
                    })
                    items=Signal::derive(|| {
                        SIZES.iter().map(|name| MenuItem::new(*name, *name)).collect()
                    })
                    on_activate=move |asked: String| {
                        if let Some(index) = SIZES.iter().position(|name| *name == asked) {
                            values.chooser.set(index);
                            values.accept();
                        }
                    }
                />
            </div>
        }
    }

    #[wasm_bindgen]
    pub fn catalog_mount(container_id: &str) -> Result<u32, JsValue> {
        let document = leptos::prelude::document();
        let container = document
            .get_element_by_id(container_id)
            .ok_or_else(|| JsValue::from_str(&format!("no element with id {container_id}")))?
            .dyn_into::<leptos::web_sys::HtmlElement>()
            .map_err(|_| JsValue::from_str("the container is not an element"))?;
        // Chosen before the scope is built, so the scope can register its
        // reset under it. A mount that fails spends one, which costs nothing:
        // handles are never reused anyway.
        let id = NEXT_HANDLE.with(|next| {
            let id = *next.borrow();
            *next.borrow_mut() += 1;
            id
        });
        let handle = mount(
            container,
            MountConfig {
                scope: "catalog".to_string(),
                // The catalogue's paths are in memory: its nav is buttons, and
                // the address bar belongs to whatever page embeds it.
                url_owner: false,
                base: String::new(),
            },
            move || view! { <Catalogue handle=id /> },
        )
        .map_err(|error| JsValue::from_str(&error.to_string()))?;
        HANDLES.with(|handles| handles.borrow_mut().insert(id, handle));
        Ok(id)
    }

    #[wasm_bindgen]
    pub fn catalog_dispose(handle: u32) -> bool {
        HANDLES
            .with(|handles| handles.borrow_mut().remove(&handle))
            .is_some()
    }

    /// Puts a mounted scope's own state back to its first-load values, in
    /// place: the scope and its region stay. False for a handle that is not
    /// mounted.
    #[wasm_bindgen]
    pub fn catalog_reset(handle: u32) -> bool {
        // Taken out of the table before it runs, so nothing it sets off can
        // find the table borrowed.
        let reset = RESETS.with(|table| table.borrow().get(&handle).cloned());
        match reset {
            Some(reset) => {
                reset();
                true
            }
            None => false,
        }
    }

    #[wasm_bindgen]
    pub fn catalog_snapshot() -> String {
        SNAPSHOT.with(|slot| slot.borrow().clone())
    }

    #[wasm_bindgen]
    pub fn catalog_identify(runtime: u32, build: &str) {
        rustify_ui::identify_runtime(runtime, build);
    }

    #[wasm_bindgen]
    pub fn catalog_diagnostics() -> String {
        rustify_ui::report_json()
    }

    #[wasm_bindgen]
    pub fn catalog_live_regions() -> u32 {
        rustify_makepad::live_region_count() as u32
    }
}

fn main() {}
