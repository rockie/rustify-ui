//! Local form values belong to this preview, independently of the theme editor.
use leptos::prelude::*;
use rustify_components::{
    checkbox_cards as cards, checkbox_group as checks, form::field as f, radio_cards as plans,
    radio_group as radios, select, slider, Button, ButtonVariant, Checkbox, ColorField, Container,
    Form, FormRoot, Gap, Grid, GridTracks, Heading, HeadingLevel, NumberField, Responsive, Slider,
    Spacing, SubmitButton, TextArea, TextField, TextSize,
};
use rustify_ui::{FieldErrorSource, Locale};
use wasm_bindgen::{JsCast, JsValue};

pub(super) fn tr(locale: Locale, en: &'static str, zh: &'static str) -> &'static str {
    if locale == Locale::Chinese {
        zh
    } else {
        en
    }
}
fn window_values() -> Vec<slider::ThumbValue> {
    vec![
        slider::ThumbValue {
            id: "start".into(),
            value: 20.,
        },
        slider::ThumbValue {
            id: "end".into(),
            value: 80.,
        },
    ]
}
fn native_data(id: &str) -> Result<String, JsValue> {
    let form = document()
        .get_element_by_id(id)
        .ok_or_else(|| JsValue::from_str("local form is not attached"))?
        .dyn_into::<web_sys::HtmlFormElement>()?;
    let data = web_sys::FormData::new_with_form(&form)?;
    let pairs = js_sys::try_iter(&data)?
        .ok_or_else(|| JsValue::from_str("FormData is not iterable"))?
        .map(|pair| {
            let pair = js_sys::Array::from(&pair?);
            Ok((
                pair.get(0).as_string().unwrap_or_default(),
                pair.get(1).as_string().unwrap_or_default(),
            ))
        })
        .collect::<Result<Vec<(String, String)>, JsValue>>()?;
    serde_json::to_string_pretty(&pairs).map_err(|error| JsValue::from_str(&error.to_string()))
}

fn error_source_description(form: Form, field: &'static str) -> Signal<String> {
    Signal::derive(move || {
        if form.error_source(field).is_some() {
            format!("{}-source", form.error_id(field))
        } else {
            String::new()
        }
    })
}

#[component]
fn SampleMessage(
    form: Form,
    field: &'static str,
    locale: Signal<Locale>,
    #[prop(optional)] test_id: &'static str,
) -> impl IntoView {
    view! {
        <Show when=move || form.error_source(field).is_some()>
            <p id=format!("{}-source", form.error_id(field)) class="text-sm font-medium" data-testid=format!("{test_id}-source")>
                {move || match form.error_source(field) {
                    Some(FieldErrorSource::Constraint) => tr(locale.get(), "Input requirement", "输入要求"),
                    Some(FieldErrorSource::Rule) => tr(locale.get(), "Form rule", "表单规则"),
                    Some(FieldErrorSource::Check) => tr(locale.get(), "Validation check", "校验检查"),
                    Some(FieldErrorSource::Application) => tr(locale.get(), "Local save", "本地保存"),
                    None => "",
                }}
            </p>
        </Show>
        <f::Message test_id=test_id />
    }
}

#[component]
pub fn FormSamples(locale: Signal<Locale>) -> impl IntoView {
    let name = RwSignal::new(String::new());
    let description = RwSignal::new(String::new());
    let consent = RwSignal::new(rustify_components::primitives::CheckedState::Indeterminate);
    let topics = RwSignal::new(vec!["updates".to_string()]);
    let plan = RwSignal::new(Some("personal".to_string()));
    let channels = RwSignal::new(vec!["mail".to_string()]);
    let cadence = RwSignal::new(Some("weekly".to_string()));
    let strength = RwSignal::new(50.0);
    let window = RwSignal::new(window_values());
    let amount = RwSignal::new(12.0);
    let color = RwSignal::new("#336699".to_string());
    let region = RwSignal::new(None::<String>);
    let region_open = RwSignal::new(false);
    let data = RwSignal::new(String::new());
    let saved = RwSignal::new(false);
    let save_count = RwSignal::new(0u32);
    let fail = RwSignal::new(false);
    let held = StoredValue::new(None::<Form>);
    let form = Form::new(
        &[
            "name",
            "description",
            "topics",
            "cadence",
            "plan",
            "channels",
            "strength",
            "window",
            "amount",
            "color",
            "region",
            "consent",
        ],
        move || {
            let Some(form) = held.try_get_value().flatten() else {
                return;
            };
            let Ok(values) = native_data(&form.control_id("native")) else {
                form.submitted(Err(tr(
                    locale.get_untracked(),
                    "The local form could not be read.",
                    "无法读取本地表单。",
                )
                .into()));
                return;
            };
            data.set(values);
            saved.set(false);
            save_count.update(|count| *count += 1);
            rustify_ui::defer(move || {
                let Some(form) = held.try_get_value().flatten() else {
                    return;
                };
                let failed = fail.get_untracked();
                fail.set(false);
                if failed {
                    form.set_application_error(
                        "name",
                        Some(
                            tr(
                                locale.get_untracked(),
                                "This demo name is unavailable. Edit it and try again.",
                                "此演示名称暂不可用。修改后重试。",
                            )
                            .into(),
                        ),
                    );
                    form.submitted(Err(tr(
                        locale.get_untracked(),
                        "Local save did not complete.",
                        "本地保存未完成。",
                    )
                    .into()));
                    if let Some(input) = document()
                        .get_element_by_id(&form.control_id("name"))
                        .and_then(|node| node.dyn_into::<web_sys::HtmlElement>().ok())
                    {
                        let _ = input.focus();
                    }
                } else {
                    saved.set(true);
                    form.submitted(Ok(()));
                }
            });
        },
    );
    held.set_value(Some(form));
    let reset = move || {
        name.set(String::new());
        description.set(String::new());
        consent.set(rustify_components::primitives::CheckedState::Indeterminate);
        topics.set(vec!["updates".into()]);
        plan.set(Some("personal".into()));
        channels.set(vec!["mail".into()]);
        cadence.set(Some("weekly".into()));
        strength.set(50.);
        window.set(window_values());
        amount.set(12.);
        color.set("#336699".into());
        region.set(None);
        region_open.set(false);
        data.set(String::new());
        saved.set(false);
        fail.set(false);
        form.reset();
    };
    let columns = Responsive {
        initial: Some(GridTracks::Count(1)),
        md: Some(GridTracks::Count(2)),
        ..Default::default()
    };
    let gap = Gap {
        gap: Responsive::from(Spacing::Scale(5)),
        ..Default::default()
    };
    view! {
        <Container class="md:col-span-2 space-y-6 border-t border-border pt-6" test_id="form-samples">
            <header class="space-y-2">
                <Heading as_=HeadingLevel::H2 size=Responsive::from(TextSize::Five)>{move || tr(locale.get(), "Form components", "表单组件")}</Heading>
                <p class="text-sm text-muted-foreground">{move || tr(locale.get(), "A complete form. Values and saves stay in this preview; new form controls use DOM rendering.", "一个完整表单。数据与保存仅保留在此预览中；新增表单控件仅由 DOM 呈现。")}</p>
            </header>
            <FormRoot form=form id=form.control_id("native") test_id="sample-form" on_reset=Callback::new(move |_| reset())
                rules=move || if name.get_untracked().trim().chars().count() < 2 { vec![("name", tr(locale.get_untracked(), "Use at least two characters.", "请至少输入两个字符。").into())] } else { Vec::new() }>
                <Grid columns=columns gap=gap>
                    <f::Root form=form field="name" class="space-y-2">
                        <f::Label>{move || tr(locale.get(), "Display name", "显示名称")}</f::Label>
                        <f::Description class="text-sm text-muted-foreground">{move || tr(locale.get(), "Required. Your input is kept when the language changes.", "必填。切换语言时保留你的输入。")}</f::Description>
                        <TextField value=name on_change=move |next| { name.set(next); form.changed("name"); saved.set(false); } name="name" described_by=error_source_description(form,"name") required=true test_id="sample-name" />
                        <SampleMessage form=form field="name" locale=locale test_id="sample-name-error" />
                    </f::Root>
                    <f::Root form=form field="description" class="space-y-2">
                        <f::Label>{move || tr(locale.get(), "Notes", "说明")}</f::Label>
                        <TextArea value=description on_change=move |next| { description.set(next); form.changed("description"); } name="description" described_by=error_source_description(form,"description") test_id="sample-description" />
                        <SampleMessage form=form field="description" locale=locale />
                    </f::Root>
                    <f::Root form=form field="topics" class="space-y-2">
                        <f::Label attr:id=form.control_id("topics-label")>{move || tr(locale.get(), "Topics", "订阅内容")}</f::Label>
                        <f::Description class="text-sm text-muted-foreground">{move || tr(locale.get(), "Required. Choose at least one topic.", "必填。请至少选择一项订阅内容。")}</f::Description>
                        <checks::Root value=topics attr:aria-labelledby=form.control_id("topics-label") on_change=move |next| { topics.set(next); form.changed("topics"); } name="topics" described_by=error_source_description(form,"topics") required=true class="flex flex-wrap gap-4">
                            <checks::Item value="updates" test_id="sample-topic-updates">{move || tr(locale.get(), "Updates", "更新")}</checks::Item>
                            <checks::Item value="guides" test_id="sample-topic-guides">{move || tr(locale.get(), "Guides", "指南")}</checks::Item>
                            <checks::Item value="archived" disabled=true>{move || tr(locale.get(), "Archived", "已归档")}</checks::Item>
                        </checks::Root><SampleMessage form=form field="topics" locale=locale />
                    </f::Root>
                    <f::Root form=form field="cadence" class="space-y-2">
                        <f::Label attr:id=form.control_id("cadence-label")>{move || tr(locale.get(), "Frequency", "频率")}</f::Label>
                        <radios::Root value=cadence attr:aria-labelledby=form.control_id("cadence-label") on_change=move |next| { cadence.set(next); form.changed("cadence"); } name="cadence" described_by=error_source_description(form,"cadence") class="flex flex-wrap gap-4">
                            <radios::Item value="daily" test_id="sample-daily">{move || tr(locale.get(), "Daily", "每天")}</radios::Item>
                            <radios::Item value="weekly" test_id="sample-weekly">{move || tr(locale.get(), "Weekly", "每周")}</radios::Item>
                        </radios::Root><SampleMessage form=form field="cadence" locale=locale />
                    </f::Root>
                    <f::Root form=form field="plan" class="space-y-2">
                        <f::Label attr:id=form.control_id("plan-label")>{move || tr(locale.get(), "Profile type", "资料类型")}</f::Label>
                        <plans::Root value=plan attr:aria-labelledby=form.control_id("plan-label") on_change=move |next| { plan.set(next); form.changed("plan"); } name="plan" described_by=error_source_description(form,"plan") required=true>
                            <plans::Item value="personal" test_id="sample-plan-personal"><strong>{move || tr(locale.get(), "Personal", "个人")}</strong><span class="block text-sm text-muted-foreground">{move || tr(locale.get(), "For your own workspace", "用于个人工作区")}</span></plans::Item>
                            <plans::Item value="team" test_id="sample-plan-team"><strong>{move || tr(locale.get(), "Team", "团队")}</strong><span class="block text-sm text-muted-foreground">{move || tr(locale.get(), "Share with collaborators", "与协作者共享")}</span></plans::Item>
                        </plans::Root><SampleMessage form=form field="plan" locale=locale />
                    </f::Root>
                    <f::Root form=form field="channels" class="space-y-2">
                        <f::Label attr:id=form.control_id("channels-label")>{move || tr(locale.get(), "Channels", "通知渠道")}</f::Label>
                        <cards::Root value=channels attr:aria-labelledby=form.control_id("channels-label") on_change=move |next| { channels.set(next); form.changed("channels"); } name="channels" described_by=error_source_description(form,"channels")>
                            <cards::Item value="mail" test_id="sample-channel-mail">{move || tr(locale.get(), "Email summary", "邮件摘要")}</cards::Item>
                            <cards::Item value="feed" test_id="sample-channel-feed">{move || tr(locale.get(), "Activity feed", "动态列表")}</cards::Item>
                        </cards::Root><SampleMessage form=form field="channels" locale=locale />
                    </f::Root>
                    <f::Root form=form field="strength" class="space-y-2">
                        <f::Label attr:id=form.control_id("strength-label")>{move || tr(locale.get(), "Intensity", "强度")}</f::Label>
                        <Slider value=strength labelled_by=form.control_id("strength-label") on_change=move |next| { strength.set(next); form.changed("strength"); } name="strength" described_by=error_source_description(form,"strength") test_id="sample-strength" />
                        <output class="text-sm tabular-nums">{move || strength.get()}</output><SampleMessage form=form field="strength" locale=locale />
                    </f::Root>
                    <f::Root form=form field="window" class="space-y-2">
                        <f::Label attr:id=form.control_id("window-label")>{move || tr(locale.get(), "Active range", "有效范围")}</f::Label>
                        <slider::Root values=window attr:aria-labelledby=form.control_id("window-label") on_change=move |next| { window.set(next); form.changed("window"); } name="window" described_by=error_source_description(form,"window") min_steps_between=5>
                            <slider::Track><slider::Range /></slider::Track>
                            <slider::Thumb id="start" aria_label="Range start" test_id="sample-range-start" attr:aria-label=move || tr(locale.get(), "Range start", "范围起点") />
                            <slider::Thumb id="end" aria_label="Range end" test_id="sample-range-end" attr:aria-label=move || tr(locale.get(), "Range end", "范围终点") />
                        </slider::Root>
                        <output class="text-sm tabular-nums">{move || window.with(|values| values.iter().map(|value| value.value.to_string()).collect::<Vec<_>>().join(" – "))}</output><SampleMessage form=form field="window" locale=locale />
                    </f::Root>
                    <f::Root form=form field="amount" class="sample-amount-field space-y-2">
                        <f::Label>{move || tr(locale.get(), "Quantity", "数量")}</f::Label>
                        <f::Description class="text-sm text-muted-foreground">
                            {move || tr(locale.get(), "A focused draft commits on Enter, blur or submit.", "聚焦时保留草稿，按 Enter、失焦或提交时确认。")}
                            <span class="sample-number-draft-help" data-testid="sample-amount-draft-help">{move || tr(locale.get(), "Number incomplete. Continue typing or press Escape to keep the accepted value.", "数字尚未完成。请继续输入，或按 Escape 保留接受值。")}</span>
                        </f::Description>
                        <NumberField value=amount on_change=move |next| { amount.set(next); form.changed("amount"); } name="amount" described_by=error_source_description(form,"amount") min=0.0 max=100.0 required=true test_id="sample-amount" />
                        <SampleMessage form=form field="amount" locale=locale />
                    </f::Root>
                    <f::Root form=form field="color" class="space-y-2">
                        <f::Label>{move || tr(locale.get(), "Colour", "颜色")}</f::Label>
                        <ColorField value=color on_change=move |next| { color.set(next); form.changed("color"); } name="color" described_by=error_source_description(form,"color") required=true test_id="sample-color" /><SampleMessage form=form field="color" locale=locale />
                    </f::Root>
                    <f::Root form=form field="region" class="space-y-2">
                        <f::Label>{move || tr(locale.get(), "Region", "地区")}</f::Label>
                        <select::Root value=region on_change={move |next: Option<String>| { region.set(next); form.changed("region"); }} open=region_open on_open_change=move |next| region_open.set(next)
                            entries=Signal::derive(move || vec![select::SelectEntry::new("east", tr(locale.get(), "East Asia", "东亚")), select::SelectEntry::new("west", tr(locale.get(), "Western Europe", "西欧"))]) name="region" described_by=error_source_description(form,"region") required=true>
                            <select::Trigger test_id="sample-region" class="w-full">{move || view! { <select::Value placeholder=tr(locale.get(), "Choose a region", "请选择地区") /> }}<select::Icon /></select::Trigger>
                            <select::Content test_id="sample-region-list"><select::Viewport>
                                <select::Group><select::Label>{move || tr(locale.get(), "Available regions", "可用地区")}</select::Label><select::Item value="east" /><select::Item value="west" /></select::Group>
                            </select::Viewport></select::Content>
                        </select::Root><SampleMessage form=form field="region" locale=locale test_id="sample-region-error" />
                    </f::Root>
                    <f::Root form=form field="consent" class="space-y-2">
                        <f::Label>{move || tr(locale.get(), "Local demonstration notice", "本地演示提示")}</f::Label>
                        <label class="flex items-center gap-2"><Checkbox
                            checked=Signal::derive(move || consent.get()==rustify_components::primitives::CheckedState::Checked)
                            indeterminate=Signal::derive(move || consent.get()==rustify_components::primitives::CheckedState::Indeterminate)
                            on_change=move |next| { consent.set(if next { rustify_components::primitives::CheckedState::Checked } else { rustify_components::primitives::CheckedState::Unchecked }); form.changed("consent"); }
                            name="consent" described_by=error_source_description(form,"consent") required=true test_id="sample-consent" />
                            <span>{move || tr(locale.get(), "I understand that this saves only in memory.", "我知道数据仅保存到内存中。")}</span></label>
                        <SampleMessage form=form field="consent" locale=locale test_id="sample-consent-error" />
                    </f::Root>
                </Grid>
                <div class="flex flex-wrap items-center gap-3">
                    <SubmitButton form=form rules=Vec::new test_id="sample-save">{move || tr(locale.get(), "Save locally", "本地保存")}</SubmitButton>
                    <button type="reset" class="text-sm underline underline-offset-4" data-testid="sample-reset">{move || tr(locale.get(), "Reset form", "重置表单")}</button>
                    <Button variant=ButtonVariant::Outline high_contrast=true on_click=move || fail.update(|flag| *flag=!*flag) attr:aria-pressed=move || fail.get().to_string() test_id="sample-fail">{move || tr(locale.get(), "Fail next local save", "让下次本地保存失败")}</Button>
                </div>
                <p role="status" class="text-sm" data-testid="sample-save-status">{move || if form.submitting() { tr(locale.get(), "Saving locally…", "正在本地保存…").to_string() } else if saved.get() { format!("{} · {}", tr(locale.get(), "Saved in this preview only", "仅已保存到此预览"), save_count.get()) } else { form.failure().unwrap_or_default() }}</p>
                <details class="text-sm"><summary>{move || tr(locale.get(), "Submitted form data", "已提交的表单数据")}</summary><pre class="mt-3 max-w-full overflow-auto whitespace-pre-wrap break-words" data-testid="sample-form-data">{move || data.with(|values| if values.is_empty() { tr(locale.get(), "Submit the form to inspect its native values.", "提交表单后在此查看原生表单值。").into() } else { values.clone() })}</pre></details>
            </FormRoot>
            <details class="border-t border-border pt-4" data-testid="form-states-details">
                <summary class="cursor-pointer font-medium">{move || tr(locale.get(), "States and parts", "状态与部件")}</summary>
                <crate::form_states::FormStates locale=locale />
            </details>
        </Container>
    }
}
