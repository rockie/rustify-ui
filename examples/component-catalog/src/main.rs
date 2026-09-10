#[cfg(target_arch = "wasm32")]
mod catalog_region;

#[cfg(target_arch = "wasm32")]
mod app {
    use super::catalog_region::{CatalogAction, CatalogProps, CatalogRegion};
    use leptos::prelude::*;
    use leptos::wasm_bindgen::prelude::*;
    use rustify_components::{
        clx, provide_current_path, variants, Boundary, Button, ButtonVariant, Category, Checkbox,
        Dialog, Glyph, Icon, Label, Link, Menu, MenuItem, Progress, RadioGroup, RadioOption,
        ScrollArea, Select, SelectOption, Slider, Spinner, Support, Switch, Tab, TabPanel, Tabs,
        TextArea, TextField, Tooltip, CATALOG,
    };
    use rustify_ui::{
        mount, Anchor, AppHandle, GpuRegion, LocalRect, MountConfig, RegionState, Theme,
        ThemedScope,
    };
    use std::cell::RefCell;
    use std::collections::BTreeMap;
    use std::marker::PhantomData;

    clx! {Panel, section, "rui:rounded-md rui:border rui:border-border rui:bg-card rui:p-4"}
    clx! {Row, div, "rui:flex rui:items-center rui:gap-2"}

    variants! {
        Chip {
            base: "rui:inline-flex rui:items-center rui:rounded-md rui:px-2 rui:py-0.5 rui:text-xs",
            variants: {
                variant: {
                    Yes: "rui:bg-success rui:text-primary-foreground",
                    Partial: "rui:bg-warning rui:text-primary-foreground",
                    No: "rui:bg-muted rui:text-muted-foreground",
                },
                size: {
                    Default: "rui:px-2 rui:py-0.5",
                    Lg: "rui:px-3 rui:py-1",
                }
            },
            component: { element: span }
        }
    }

    /// The catalogue's own words, in the two languages the plan names.
    ///
    /// This is not the SDK's message catalogue - that is M7's, and it will
    /// replace this. What it is here for is the switch: a page that cannot
    /// change language cannot show that changing it changes nothing else.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum Locale {
        En,
        ZhCn,
    }

    impl Locale {
        fn tag(self) -> &'static str {
            match self {
                Self::En => "en",
                Self::ZhCn => "zh-CN",
            }
        }

        /// The key is `&'static str` so a key with no translation can be its
        /// own answer: the page shows the key, which is how a missing entry
        /// gets noticed instead of leaking or panicking.
        fn t(self, key: &'static str) -> &'static str {
            match (self, key) {
                (Self::En, "catalogue") => "catalogue",
                (Self::En, "status") => "support at a glance",
                (Self::En, "presentation") => "presentation",
                (Self::En, "capabilities") => "capabilities",
                (Self::En, "dom") => "DOM",
                (Self::En, "gpu") => "GPU",
                (Self::En, "across") => "across regions",
                (Self::En, "theme") => "theme",
                (Self::En, "language") => "language",
                (Self::En, "motion") => "motion",
                (Self::En, "less") => "less",
                (Self::En, "full") => "full",
                (Self::En, "example") => "example",
                (Self::En, "states") => "states",
                (Self::En, "default") => "default",
                (Self::En, "disabled") => "disabled",
                (Self::En, "read-only") => "read-only",
                (Self::En, "invalid") => "invalid",
                (Self::ZhCn, "catalogue") => "组件目录",
                (Self::ZhCn, "status") => "能力总表",
                (Self::ZhCn, "presentation") => "呈现",
                (Self::ZhCn, "capabilities") => "能力",
                (Self::ZhCn, "dom") => "DOM",
                (Self::ZhCn, "gpu") => "GPU",
                (Self::ZhCn, "across") => "跨区",
                (Self::ZhCn, "theme") => "主题",
                (Self::ZhCn, "language") => "语言",
                (Self::ZhCn, "motion") => "动效",
                (Self::ZhCn, "less") => "减少",
                (Self::ZhCn, "full") => "完整",
                (Self::ZhCn, "example") => "示例",
                (Self::ZhCn, "states") => "状态",
                (Self::ZhCn, "default") => "默认",
                (Self::ZhCn, "disabled") => "禁用",
                (Self::ZhCn, "read-only") => "只读",
                (Self::ZhCn, "invalid") => "错误",
                (_, other) => other,
            }
        }
    }

    /// Which page the catalogue is showing. In-memory: the URL belongs to the
    /// router, which is P2 M4, and a nav that pretended to own it before then
    /// would have to be undone.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    enum Page {
        Category(usize),
        Status,
    }

    impl Page {
        fn path(self) -> String {
            match self {
                Self::Category(index) => {
                    format!("/{}", CATALOG[index].category.name().replace(' ', "-"))
                }
                Self::Status => "/status".to_string(),
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
    }

    fn chip(support: Support) -> ChipVariant {
        match support {
            Support::Yes => ChipVariant::Yes,
            Support::Partial => ChipVariant::Partial,
            Support::No => ChipVariant::No,
        }
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
    /// Whether a state matrix says anything for this category.
    ///
    /// A label, an icon and a progress bar have one state; a tooltip, a menu
    /// and a dialog have one that is a layer, and four of those open at once
    /// would be four layers over each other rather than a matrix.
    fn has_states(category: Category) -> bool {
        matches!(
            category,
            Category::Button
                | Category::Link
                | Category::TextField
                | Category::TextArea
                | Category::Checkbox
                | Category::Radio
                | Category::Switch
                | Category::Slider
                | Category::Tabs
        )
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
            Category::ScrollArea => view! {
                <ScrollArea
                    test_id=id("scroll-area")
                    aria_label="a list that does not fit"
                    boundary=Boundary::Stop
                    class="rui:h-24 rui:border rui:border-border rui:p-2"
                >
                    {(1..=20)
                        .map(|line| view! { <p class="rui:text-sm">{format!("line {line}")}</p> })
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
                <h3>{move || locale.get().t("example")}</h3>
                <div data-testid="example" class="catalogue-example">
                    <Example category=category state=ExampleState::Live values=values />
                </div>
                <p data-testid="gpu-note" class="rui:text-sm rui:text-muted-foreground">
                    {entry.environment.note}
                </p>
                <Show when=move || has_states(category) fallback=|| ()>
                    <h3>{move || locale.get().t("states")}</h3>
                    <div data-testid="state-matrix" class="catalogue-states">
                        {[ExampleState::Disabled, ExampleState::ReadOnly, ExampleState::Invalid]
                            .into_iter()
                            .map(|state| {
                                view! {
                                    <div data-testid=format!("state-{}", state.name())>
                                        <span class="rui:text-xs rui:text-muted-foreground">
                                            {move || locale.get().t(state.name())}
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
                <h3>{move || locale.get().t("presentation")}</h3>
                <Row>
                    {presentation
                        .into_iter()
                        .map(|(key, support)| {
                            view! {
                                <Row>
                                    <span>{move || locale.get().t(key)}</span>
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
                <h3>{move || locale.get().t("capabilities")}</h3>
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

    #[component]
    fn StatusPage(locale: Signal<Locale>) -> impl IntoView {
        view! {
            <Panel test_id="status-page">
                <h2>{move || locale.get().t("status")}</h2>
                <table data-testid="status-table">
                    <thead>
                        <tr>
                            <th scope="col">{move || locale.get().t("catalogue")}</th>
                            <th scope="col">{move || locale.get().t("dom")}</th>
                            <th scope="col">{move || locale.get().t("gpu")}</th>
                            <th scope="col">{move || locale.get().t("across")}</th>
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

    #[component]
    fn Catalogue() -> impl IntoView {
        let theme = RwSignal::new(Theme::light());
        let locale = RwSignal::new(Locale::En);
        let page = RwSignal::new(Page::Category(0));
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
        let region_chooser = RwSignal::new(None::<LocalRect>);
        let values = Values {
            checked: RwSignal::new(false),
            chosen: RwSignal::new(0),
            on: RwSignal::new(false),
            size: RwSignal::new(40.0),
            fraction: RwSignal::new(0.35),
            spinning: RwSignal::new(true),
            tab: RwSignal::new(0),
            text: RwSignal::new("hello".to_string()),
            notes: RwSignal::new("two\nlines".to_string()),
            chooser: RwSignal::new(1),
            tooltip: RwSignal::new(false),
            menu: RwSignal::new(false),
            dialog: RwSignal::new(false),
            select: RwSignal::new(false),
            actions: RwSignal::new(0),
        };
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
            // The status page draws no control, and a label saying so is a
            // truer answer than the last page's control left on screen.
            Page::Status => Category::Label,
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
                 \"chooser\":\"{}\",\"text\":\"{}\",\"actions\":{},\"reduce_motion\":{}}}",
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
                values.text.get().replace('"', "'"),
                values.actions.get(),
                theme.get().reduce_motion,
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
                    <h1>{move || locale.get().t("catalogue")}</h1>
                    <Row>
                        <Button
                            test_id="toggle-theme"
                            aria_label="switch theme"
                            on_click=switch_theme
                        >
                            {move || format!("{}: {}", locale.get().t("theme"), theme.get().name)}
                        </Button>
                        <Switch
                            test_id="toggle-motion"
                            aria_label="reduce motion"
                            checked=Signal::derive(move || theme.get().reduce_motion)
                            on_change=move |less| {
                                theme.update(|theme| theme.reduce_motion = less)
                            }
                        />
                        <span class="rui:text-sm">
                            {move || {
                                let locale = locale.get();
                                format!(
                                    "{}: {}",
                                    locale.t("motion"),
                                    if theme.get().reduce_motion {
                                        locale.t("less")
                                    } else {
                                        locale.t("full")
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
                                    .update(|locale| {
                                        *locale = match locale {
                                            Locale::En => Locale::ZhCn,
                                            Locale::ZhCn => Locale::En,
                                        };
                                    })
                            }
                        >
                            {move || {
                                format!("{}: {}", locale.get().t("language"), locale.get().tag())
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
                                {move || locale.get().t("status")}
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
                                    locale=locale.into()
                                    values=values
                                />
                            }
                                .into_any()
                        }
                        Page::Status => view! { <StatusPage locale=locale.into() /> }.into_any(),
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
        let handle = mount(
            container,
            MountConfig {
                scope: "catalog".to_string(),
            },
            Catalogue,
        )
        .map_err(|error| JsValue::from_str(&error.to_string()))?;
        let id = NEXT_HANDLE.with(|next| {
            let id = *next.borrow();
            *next.borrow_mut() += 1;
            id
        });
        HANDLES.with(|handles| handles.borrow_mut().insert(id, handle));
        Ok(id)
    }

    #[wasm_bindgen]
    pub fn catalog_dispose(handle: u32) -> bool {
        HANDLES
            .with(|handles| handles.borrow_mut().remove(&handle))
            .is_some()
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
