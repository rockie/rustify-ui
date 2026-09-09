#[cfg(target_arch = "wasm32")]
mod theme_region;

#[cfg(target_arch = "wasm32")]
mod app {
    use super::theme_region::{ThemeAction, ThemeProps, ThemeRegion};
    use leptos::prelude::*;
    use leptos::wasm_bindgen::prelude::*;
    use rustify_components::{clx, provide_current_path, variants};
    use rustify_ui::{
        mount, AppHandle, Button, GpuRegion, MountConfig, RegionState, Theme, ThemedScope, CATALOG,
    };
    use std::cell::RefCell;
    use std::collections::BTreeMap;
    use std::marker::PhantomData;

    pub use rustify_ui::makepad_widgets;

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

        fn t(self, key: &str) -> &'static str {
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
                (Self::En, "shipped") => "in this release",
                (Self::En, "absent") => "not in this release",
                (Self::ZhCn, "catalogue") => "组件目录",
                (Self::ZhCn, "status") => "能力总表",
                (Self::ZhCn, "presentation") => "呈现",
                (Self::ZhCn, "capabilities") => "能力",
                (Self::ZhCn, "dom") => "DOM",
                (Self::ZhCn, "gpu") => "GPU",
                (Self::ZhCn, "across") => "跨区",
                (Self::ZhCn, "theme") => "主题",
                (Self::ZhCn, "language") => "语言",
                (Self::ZhCn, "shipped") => "本期交付",
                (Self::ZhCn, "absent") => "本期不含",
                // A key with no translation is a bug in this table, and saying
                // so on the page is how it gets found.
                (_, other) => Box::leak(format!("[{other}]").into_boxed_str()),
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

    thread_local! {
        static HANDLES: RefCell<BTreeMap<u32, AppHandle>> = const { RefCell::new(BTreeMap::new()) };
        static NEXT_HANDLE: RefCell<u32> = const { RefCell::new(1) };
        static SNAPSHOT: RefCell<String> = const { RefCell::new(String::new()) };
    }

    fn chip(support: rustify_ui::Support) -> ChipVariant {
        match support {
            rustify_ui::Support::Yes => ChipVariant::Yes,
            rustify_ui::Support::Partial => ChipVariant::Partial,
            rustify_ui::Support::No => ChipVariant::No,
        }
    }

    #[component]
    fn CategoryPage(index: usize, locale: Signal<Locale>) -> impl IntoView {
        let entry = &CATALOG[index];
        let presentation = [
            ("dom", entry.presentation.dom),
            ("gpu", entry.presentation.gpu),
            ("across", entry.presentation.across_regions),
        ];
        view! {
            <Panel test_id="category-page">
                <h2 data-testid="category-name">{entry.category.name()}</h2>
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
                                        "status-{}",
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

        Effect::new(move || {
            let page = page.get();
            let snapshot = format!(
                "{{\"path\":\"{}\",\"theme\":\"{}\",\"locale\":\"{}\",\"categories\":{},\"region\":\"{}\"}}",
                page.path(),
                theme.get().name,
                locale.get().tag(),
                CATALOG.len(),
                match region.get() {
                    RegionState::Starting => "starting",
                    RegionState::Ready => "ready",
                    RegionState::Suspended => "suspended",
                    RegionState::Lost => "lost",
                    RegionState::Failed(_) => "failed",
                    RegionState::Disposed => "disposed",
                },
            );
            SNAPSHOT.with(|slot| *slot.borrow_mut() = snapshot);
        });

        let props = Signal::derive(move || ThemeProps { theme: theme.get() });
        // The view macro parses attribute values as expressions, and a
        // turbofish or a `match` arm inside one is not one of them.
        let region_app = PhantomData::<ThemeRegion>;
        let on_region_action = move |action| match action {
            ThemeAction::Toggle => switch_theme(),
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
                        <Button
                            test_id="toggle-locale"
                            aria_label="switch language"
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
                                            test_id=format!(
                                                "nav-{}",
                                                name.replace(' ', "-"),
                                            )
                                            on_click=move || page.set(Page::Category(index))
                                        >
                                            {name}
                                        </Button>
                                    </li>
                                }
                            })
                            .collect_view()}
                        <li>
                            <Button test_id="nav-status" on_click=move || page.set(Page::Status)>
                                {move || locale.get().t("status")}
                            </Button>
                        </li>
                    </ul>
                </nav>
                <main class="catalogue-main">
                    {move || match page.get() {
                        Page::Category(index) => {
                            view! { <CategoryPage index=index locale=locale.into() /> }.into_any()
                        }
                        Page::Status => view! { <StatusPage locale=locale.into() /> }.into_any(),
                    }}
                </main>
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
