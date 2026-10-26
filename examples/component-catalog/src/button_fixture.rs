use leptos::prelude::*;
use leptos::wasm_bindgen::{prelude::*, JsCast};
use rustify_components::{
    Button, ButtonColorRole, ButtonSize, ButtonType, ButtonVariant, Glyph, Icon, IconButton,
    Responsive, ThemeOverrideCounts, ThemePanel,
};
use rustify_ui::{
    mount, AccentColor, AppHandle, ConfigChange, Locale, MountConfig, ThemeConfigGroup,
    ThemeDocument, ThemeMode, ThemeRadius, ThemeScaling, ThemeScope,
};
use std::{cell::RefCell, sync::Arc};

thread_local! {
    static FIXTURE: RefCell<Option<AppHandle>> = const { RefCell::new(None) };
}

#[component]
fn ButtonFixture() -> impl IntoView {
    let source = RwSignal::new(ThemeDocument::standard("button-fixture", "Button fixture"));
    let mode = RwSignal::new(ThemeMode::Light);
    let resolved = Memo::new(move |_| {
        Arc::new(
            rustify_ui::resolve(
                &source.get(),
                mode.get(),
                &rustify_ui::theme::font_context(16., false),
            )
            .expect("fixed button fixture theme"),
        )
    });
    let loading = RwSignal::new(false);
    let expanded = RwSignal::new(false);
    let matrix_requests = RwSignal::new(0_u32);
    let submit_requests = RwSignal::new(0_u32);
    let submits = RwSignal::new(0_u32);
    let reset_requests = RwSignal::new(0_u32);
    let resets = RwSignal::new(0_u32);
    let default_requests = RwSignal::new(0_u32);
    let unavailable_requests = RwSignal::new(0_u32);
    let disabled_requests = RwSignal::new(0_u32);
    let icon_requests = RwSignal::new(0_u32);
    let panel_locale = RwSignal::new(Locale::English);
    let panel_accepting = RwSignal::new(true);
    let panel_requests = RwSignal::new(0_u32);
    let panel_resets = RwSignal::new(0_u32);
    let panel_patch_fields = RwSignal::new(0_usize);
    let panel_last_patch = RwSignal::new("");
    let panel_last_reset = RwSignal::new("");
    let panel_counts = Memo::new(move |_| ThemeOverrideCounts::from(&source.get().overrides));
    view! {
        <ThemeScope resolved=resolved />
        <div class="space-y-6 bg-background p-4 text-foreground" data-testid="button-fixture-body">
            <h2 class="text-xl font-bold">"Native button parameters"</h2>
            <div class="flex flex-wrap gap-3" data-testid="button-fixture-controls">
                <Button on_click=move || loading.update(|value| *value = !*value) test_id="button-toggle-loading">"Toggle loading"</Button>
                <Button on_click=move || mode.set(ThemeMode::Dark) test_id="button-mode-dark">"Dark"</Button>
                <Button on_click=move || mode.set(ThemeMode::Light) test_id="button-mode-light">"Light"</Button>
                <Button on_click=move || source.update(|doc| doc.config.scaling = ThemeScaling::P110) test_id="button-scale-110">"Scale 110%"</Button>
                <Button on_click=move || source.update(|doc| doc.config.scaling = ThemeScaling::P100) test_id="button-scale-100">"Scale 100%"</Button>
                <Button on_click=move || source.update(|doc| doc.config.radius = Some(ThemeRadius::Full)) test_id="button-theme-full">"Theme full radius"</Button>
                <Button on_click=move || source.update(|doc| {
                    for values in [&mut doc.overrides.light.0, &mut doc.overrides.dark.0] {
                        values.insert("primary".into(), "rgb(17 34 51)".into());
                        values.insert("primary-foreground".into(), "rgb(255 255 255)".into());
                    }
                }) test_id="button-override-primary">"Override primary"</Button>
            </div>
            <div class="space-y-2" data-testid="button-matrix">
                {ButtonVariant::ALL.iter().map(|&variant| view! {
                    <div class="flex flex-wrap items-center gap-2" data-testid=format!("button-row-{}", variant.as_str())>
                        <span class="w-28 text-sm">{variant.as_str()}</span>
                        {ButtonSize::ALL.iter().map(move |&size| view! {
                            <span class="inline-flex p-3">
                                <Button variant=variant size=size test_id=format!("button-{}-{}", variant.as_str(), size.number())
                                    on_click=move || matrix_requests.update(|count| *count += 1)>
                                    {format!("{} {}", variant.as_str(), size.number())}
                                </Button>
                            </span>
                        }).collect_view()}
                    </div>
                }).collect_view()}
            </div>
            <div class="flex flex-wrap items-center gap-4">
                <Button on_click=|| {} test_id="button-default">"Default action"</Button>
                <span class="bg-primary px-3 py-2 text-primary-foreground" data-testid="button-primary-token">"Primary pair"</span>
                <span class="bg-accent px-3 py-2 text-accent-foreground" data-testid="button-accent-token">"Accent pair"</span>
                <span class="bg-destructive px-3 py-2 text-destructive-foreground" data-testid="button-destructive-token">"Destructive pair"</span>
                <Button on_click=|| {} color_role=ButtonColorRole::Accent test_id="button-semantic-accent">"Semantic accent"</Button>
                <Button on_click=|| {} color_role=ButtonColorRole::Destructive test_id="button-semantic-destructive">"Semantic destructive"</Button>
            </div>
            <div class="flex flex-wrap items-center gap-3" data-testid="button-colors">
                {AccentColor::ALL.iter().map(|&color| view! {
                    <Button on_click=|| {} color=color test_id=format!("button-color-{}", color.as_str())>{color.as_str()}</Button>
                }).collect_view()}
                <Button on_click=|| {} color=AccentColor::Blue high_contrast=true test_id="button-high-contrast">"High contrast"</Button>
            </div>
            <div class="flex flex-wrap items-center gap-3" data-testid="button-radii">
                {ThemeRadius::ALL.iter().map(|&radius| view! {
                    <Button on_click=|| {} radius=radius test_id=format!("button-radius-{}", radius.as_str())>{radius.as_str()}</Button>
                }).collect_view()}
            </div>
            <div class="flex flex-wrap items-center gap-4">
                <Button on_click=|| {} size=Responsive {
                    initial: Some(ButtonSize::One), xs: Some(ButtonSize::Two),
                    sm: Some(ButtonSize::Three), md: Some(ButtonSize::Four),
                    lg: Some(ButtonSize::One), xl: Some(ButtonSize::Two),
                } test_id="button-responsive">"Responsive action"</Button>
                <Button on_click=|| {} variant=ButtonVariant::Outline size=ButtonSize::One color=AccentColor::Red radius=ThemeRadius::Full
                    class="h-[52px] rounded-none bg-destructive px-[22px]" test_id="button-class-override">"Caller utilities"</Button>
                <IconButton on_click=move || icon_requests.update(|count| *count += 1) aria_label="Close preview" test_id="button-icon">
                    <Icon glyph=Glyph::Close />
                </IconButton>
                <Button on_click=move || expanded.update(|value| *value = !*value) controls="button-details"
                    expanded=Signal::derive(move || Some(expanded.get())) test_id="button-disclosure">"Show details"</Button>
                <p id="button-details" hidden=move || !expanded.get()>"Controlled disclosure content"</p>
            </div>
            <form class="flex flex-wrap items-center gap-3" data-testid="button-native-form"
                on:submit=move |event| { event.prevent_default(); submits.update(|count| *count += 1); }
                on:reset=move |_| resets.update(|count| *count += 1)>
                <label>"Draft title" <input class="border border-border bg-input px-2 py-1" aria-label="Draft title" name="title" value="Initial" /></label>
                <Button on_click=move || default_requests.update(|count| *count += 1) test_id="button-native-default">"Ordinary action"</Button>
                <Button on_click=move || submit_requests.update(|count| *count += 1) r#type=ButtonType::Submit
                    loading=loading disabled=false attr:disabled=false attr:name="intent" attr:value="save" test_id="button-native-submit">"Save changes"</Button>
                <Button on_click=move || reset_requests.update(|count| *count += 1) r#type=ButtonType::Reset loading=loading test_id="button-native-reset">"Reset draft"</Button>
                <Button on_click=move || unavailable_requests.update(|count| *count += 1) r#type=ButtonType::Submit unavailable=true test_id="button-native-unavailable">"Unavailable request"</Button>
                <Button on_click=move || disabled_requests.update(|count| *count += 1) r#type=ButtonType::Submit disabled=true test_id="button-native-disabled">"Disabled request"</Button>
            </form>
            <div class="space-y-4" data-testid="button-panel-fixture">
                <div class="flex flex-wrap gap-3">
                    <Button on_click=move || panel_accepting.update(|value| *value = !*value) test_id="button-panel-toggle-accept">"Toggle accepting config requests"</Button>
                    <Button on_click=move || panel_locale.set(Locale::Chinese) test_id="button-panel-chinese">"中文"</Button>
                    <Button on_click=move || panel_locale.set(Locale::English) test_id="button-panel-english">"English"</Button>
                    <Button on_click=move || source.update(|doc| {
                        for (mode, values) in [
                            (ThemeMode::Light, vec![("primary", "rgb(17 34 51)"), ("ring", "rgb(68 85 102)"), ("card", "rgb(250 250 250)"), ("radius", "10px"), ("accent", "rgb(201 87 31)")]),
                            (ThemeMode::Dark, vec![("primary", "rgb(170 187 204)"), ("popover", "rgb(30 30 30)"), ("radius", "12px"), ("accent", "rgb(248 167 75)")]),
                        ] {
                            for (token, value) in values {
                                doc.overrides.values_mut(mode).0.insert(token.into(), value.into());
                            }
                        }
                    }) test_id="button-panel-seed">"Seed both-mode overrides"</Button>
                </div>
                <ThemePanel config=Signal::derive(move || resolved.get().config)
                    overrides=Signal::derive(move || panel_counts.get()) locale=panel_locale
                    on_change=move |patch| {
                        panel_requests.update(|count| *count += 1);
                        let fields = [
                            ("appearance", !matches!(patch.appearance, ConfigChange::Keep)),
                            ("accent_color", !matches!(patch.accent_color, ConfigChange::Keep)),
                            ("gray_color", !matches!(patch.gray_color, ConfigChange::Keep)),
                            ("panel_background", !matches!(patch.panel_background, ConfigChange::Keep)),
                            ("radius", !matches!(patch.radius, ConfigChange::Keep)),
                            ("scaling", !matches!(patch.scaling, ConfigChange::Keep)),
                        ];
                        panel_patch_fields.set(fields.iter().filter(|(_, changed)| *changed).count());
                        panel_last_patch.set(fields.iter().find(|(_, changed)| *changed).map_or("", |(field, _)| *field));
                        if panel_accepting.get_untracked() {
                            source.update(|doc| patch.apply(&mut doc.config));
                        }
                    }
                    on_reset=move |group: ThemeConfigGroup| {
                        panel_resets.update(|count| *count += 1);
                        panel_last_reset.set(group.field());
                        source.update(|doc| {
                            for values in [&mut doc.overrides.light.0, &mut doc.overrides.dark.0] {
                                for token in group.tokens() { values.remove(*token); }
                            }
                        });
                    } test_id="button-theme-panel" />
            </div>
            <output data-testid="button-counts" aria-live="polite"
                data-loading=move || loading.get().to_string()
                data-matrix-request=move || matrix_requests.get()
                data-submit-request=move || submit_requests.get()
                data-submit=move || submits.get()
                data-reset-request=move || reset_requests.get()
                data-reset=move || resets.get()
                data-default-request=move || default_requests.get()
                data-unavailable-request=move || unavailable_requests.get()
                data-disabled-request=move || disabled_requests.get()
                data-icon-request=move || icon_requests.get()
                data-panel-accepting=move || panel_accepting.get().to_string()
                data-panel-request=move || panel_requests.get()
                data-panel-reset=move || panel_resets.get()
                data-panel-patch-fields=move || panel_patch_fields.get()
                data-panel-last-patch=move || panel_last_patch.get()
                data-panel-last-reset=move || panel_last_reset.get()
                data-override-light=move || source.with(|doc| doc.overrides.light.0.keys().cloned().collect::<Vec<_>>().join(","))
                data-override-dark=move || source.with(|doc| doc.overrides.dark.0.keys().cloned().collect::<Vec<_>>().join(","))
                data-independent-light=move || source.with(|doc| doc.overrides.light.0.get("accent").cloned().unwrap_or_default())
                data-independent-dark=move || source.with(|doc| doc.overrides.dark.0.get("accent").cloned().unwrap_or_default())>
                "Native requests and events"
            </output>
        </div>
    }
}

#[wasm_bindgen]
pub fn catalog_button_fixtures(root: &str) -> Result<(), JsValue> {
    catalog_button_fixtures_dispose();
    let element = document()
        .get_element_by_id(root)
        .ok_or_else(|| JsValue::from_str("missing button fixture root"))?
        .dyn_into::<leptos::web_sys::HtmlElement>()?;
    let handle = mount(
        element,
        MountConfig {
            scope: "catalog-buttons".into(),
            ..Default::default()
        },
        || view! { <ButtonFixture /> },
    )
    .map_err(|error| JsValue::from_str(&error.to_string()))?;
    FIXTURE.with(|fixture| *fixture.borrow_mut() = Some(handle));
    Ok(())
}

#[wasm_bindgen]
pub fn catalog_button_fixtures_dispose() {
    FIXTURE.with(|fixture| fixture.borrow_mut().take());
}
