//! An application-owned state inspector for the public form parts.
use crate::form_samples::tr;
use leptos::prelude::*;
use rustify_components::{self as c, primitives as p};
use rustify_ui::{AccentColor, Locale, ThemeRadius};

#[derive(Clone, Debug)]
enum Request {
    Checked(p::CheckedState),
    Flag(bool),
    Choice(Option<String>),
    Choices(Vec<String>),
    Text(String),
    Color(String),
    Number(f64),
    Scalar(f64),
    Range(Vec<c::slider::ThumbValue>),
}
impl Request {
    fn describe(&self) -> String {
        match self {
            Self::Checked(value) => format!("{value:?}"),
            Self::Flag(value) => value.to_string(),
            Self::Choice(value) => serde_json::to_string(value).unwrap(),
            Self::Choices(value) => serde_json::to_string(value).unwrap(),
            Self::Text(value) | Self::Color(value) => serde_json::to_string(value).unwrap(),
            Self::Number(value) | Self::Scalar(value) => value.to_string(),
            Self::Range(values) => values
                .iter()
                .map(|value| format!("{}={}", value.id, value.value))
                .collect::<Vec<_>>()
                .join(", "),
        }
    }
}
const PARTS: [&str; 15] = [
    "Checkbox",
    "Switch",
    "Toggle",
    "RadioGroup",
    "CheckboxGroup",
    "ToggleGroup",
    "RadioCards",
    "CheckboxCards",
    "Slider",
    "RangeSlider",
    "TextField",
    "TextArea",
    "NumberField",
    "ColorField",
    "Select",
];
macro_rules! size {
    ($ty:ty, $index:expr) => {
        <$ty>::ALL[$index]
    };
}
macro_rules! variant {
    ($ty:ty, $name:expr) => {
        match $name {
            "classic" => <$ty>::Classic,
            "soft" => <$ty>::Soft,
            _ => <$ty>::Surface,
        }
    };
}

#[component]
pub fn FormStates(locale: Signal<Locale>) -> impl IntoView {
    let part = RwSignal::new("Checkbox".to_string());
    let themed = RwSignal::new(true);
    let policy = RwSignal::new("accept".to_string());
    let size = RwSignal::new(1usize);
    let variant = RwSignal::new("surface".to_string());
    let content_variant = RwSignal::new(c::select::SelectContentVariant::Solid);
    let color = RwSignal::new(None::<AccentColor>);
    let radius = RwSignal::new(None::<ThemeRadius>);
    let contrast = RwSignal::new(false);
    let state = RwSignal::new("normal".to_string());
    let direction = RwSignal::new(p::Direction::LeftToRight);
    let cancel_select = RwSignal::new(false);
    let checked = RwSignal::new(p::CheckedState::Indeterminate);
    let flag = RwSignal::new(false);
    let choice = RwSignal::new(Some("a".to_string()));
    let choices = RwSignal::new(vec!["a".to_string()]);
    let text = RwSignal::new("Example".to_string());
    let color_value = RwSignal::new("#336699".to_string());
    let number = RwSignal::new(12.5);
    let scalar = RwSignal::new(50.0);
    let range = RwSignal::new(vec![
        c::slider::ThumbValue {
            id: "start".into(),
            value: 20.,
        },
        c::slider::ThumbValue {
            id: "end".into(),
            value: 80.,
        },
    ]);
    let open = RwSignal::new(false);
    let count = RwSignal::new(0u32);
    let last = RwSignal::new(String::new());
    let events = RwSignal::new(String::new());
    let pending = RwSignal::new(None::<Request>);
    let apply = Callback::new(move |request: Request| match request {
        Request::Checked(value) => checked.set(value),
        Request::Flag(value) => flag.set(value),
        Request::Choice(value) => choice.set(value),
        Request::Choices(value) => choices.set(value),
        Request::Text(value) => text.set(value),
        Request::Color(value) => color_value.set(value),
        Request::Number(value) => number.set(value),
        Request::Scalar(value) => scalar.set(value),
        Request::Range(value) => range.set(value),
    });
    let request = Callback::new(move |value: Request| {
        count.update(|count| *count += 1);
        last.set(value.describe());
        match policy.get_untracked().as_str() {
            "delay" => pending.set(Some(value)),
            "refuse" => pending.set(None),
            _ => {
                pending.set(None);
                apply.run(value);
            }
        }
    });
    let accepted = Signal::derive(move || match part.get().as_str() {
        "Checkbox" => Request::Checked(checked.get()),
        "Switch" | "Toggle" => Request::Flag(flag.get()),
        "RadioGroup" | "RadioCards" | "Select" => Request::Choice(choice.get()),
        "CheckboxGroup" | "CheckboxCards" | "ToggleGroup" => Request::Choices(choices.get()),
        "NumberField" => Request::Number(number.get()),
        "ColorField" => Request::Color(color_value.get()),
        "Slider" => Request::Scalar(scalar.get()),
        "RangeSlider" => Request::Range(range.get()),
        _ => Request::Text(text.get()),
    });
    let show_variants =
        Signal::derive(move || !matches!(part.get().as_str(), "Toggle" | "ToggleGroup"));
    let show_contrast = Signal::derive(move || {
        !matches!(
            part.get().as_str(),
            "Toggle" | "ToggleGroup" | "TextField" | "TextArea" | "NumberField" | "ColorField"
        )
    });
    let show_radius = Signal::derive(move || {
        matches!(
            part.get().as_str(),
            "Switch"
                | "Slider"
                | "RangeSlider"
                | "TextField"
                | "TextArea"
                | "NumberField"
                | "ColorField"
                | "Select"
        )
    });
    view! {
        <div class="mt-5 space-y-5" data-testid="form-states">
            <div class="grid gap-4 sm:grid-cols-2 xl:grid-cols-3">
                <label class="space-y-2"><span>{move || tr(locale.get(), "Component", "组件")}</span>
                    <select class="block w-full rounded-sm border border-border bg-input p-2 text-sm" data-testid="parts-component" on:change=move |event| { part.set(event_target_value(&event)); open.set(false); variant.set("surface".into()); pending.set(None); count.set(0); last.set(String::new()); events.set(String::new()); }>
                        {PARTS.into_iter().map(|name| view! { <option value=name>{name}</option> }).collect_view()}
                    </select>
                </label>
                <label class="space-y-2"><span>{move || tr(locale.get(), "Presentation", "呈现方式")}</span>
                    <select class="block w-full rounded-sm border border-border bg-input p-2 text-sm" data-testid="parts-presentation" on:change=move |event| themed.set(event_target_value(&event)=="themes")>
                        <option value="themes">"Themes"</option><option value="primitives">"Primitives"</option>
                    </select>
                </label>
                <label class="space-y-2"><span>{move || tr(locale.get(), "Value requests", "值请求策略")}</span>
                    <select class="block w-full rounded-sm border border-border bg-input p-2 text-sm" data-testid="parts-policy" on:change=move |event| policy.set(event_target_value(&event))>
                        <option value="accept">{move || tr(locale.get(), "Accept", "接受")}</option><option value="refuse">{move || tr(locale.get(), "Refuse", "拒绝")}</option><option value="delay">{move || tr(locale.get(), "Accept later", "延迟接受")}</option>
                    </select>
                </label>
                <Show when=move || themed.get() && show_variants.get()>
                    <label class="space-y-2"><span>{move || tr(locale.get(), "Size", "尺寸")}</span><select class="block w-full rounded-sm border border-border bg-input p-2 text-sm" data-testid="parts-size" prop:value=move || (size.get()+1).to_string() on:change=move |event| size.set(event_target_value(&event).parse::<usize>().unwrap_or(2).clamp(1,3)-1)>
                        <option value="1">"1"</option><option value="2">"2"</option><option value="3">"3"</option>
                    </select></label>
                    <label class="space-y-2"><span>{move || tr(locale.get(), "Variant", "外观")}</span><select class="block w-full rounded-sm border border-border bg-input p-2 text-sm" data-testid="parts-variant" prop:value=move || variant.get() on:change=move |event| variant.set(event_target_value(&event))>
                        <option value="surface">"surface"</option><option value="classic">"classic"</option>
                        <Show when=move || !matches!(part.get().as_str(), "RadioCards" | "CheckboxCards")><option value="soft">"soft"</option></Show>
                        <Show when=move || part.get()=="Select"><option value="ghost-offset">"ghost-offset"</option><option value="ghost">"ghost"</option></Show>
                    </select></label>
                    <label class="space-y-2"><span>{move || tr(locale.get(), "Local colour", "局部颜色")}</span><select class="block w-full rounded-sm border border-border bg-input p-2 text-sm" data-testid="parts-color" prop:value=move || match color.get() { Some(AccentColor::Blue) => "blue", Some(AccentColor::Red) => "red", Some(AccentColor::Gray) => "gray", _ => "inherit" } on:change=move |event| color.set(match event_target_value(&event).as_str() { "blue" => Some(AccentColor::Blue), "red" => Some(AccentColor::Red), "gray" => Some(AccentColor::Gray), _ => None })>
                        <option value="inherit">{move || tr(locale.get(), "Inherit", "继承")}</option><option value="blue">{move || tr(locale.get(), "Blue", "蓝色")}</option><option value="red">{move || tr(locale.get(), "Red", "红色")}</option><option value="gray">{move || tr(locale.get(), "Gray", "中性灰")}</option>
                    </select></label>
                </Show>
                <label class="space-y-2"><span>{move || tr(locale.get(), "State", "状态")}</span><select class="block w-full rounded-sm border border-border bg-input p-2 text-sm" data-testid="parts-state" on:change=move |event| state.set(event_target_value(&event))>
                    <option value="normal">{move || tr(locale.get(), "Normal", "正常")}</option><option value="disabled">{move || tr(locale.get(), "Disabled", "禁用")}</option><option value="readonly">{move || tr(locale.get(), "Read only", "只读")}</option><option value="invalid">{move || tr(locale.get(), "Invalid", "无效")}</option>
                </select></label>
                <Show when=move || themed.get() && show_radius.get()>
                    <label class="space-y-2"><span>{move || tr(locale.get(), "Radius", "圆角")}</span><select class="block w-full rounded-sm border border-border bg-input p-2 text-sm" data-testid="parts-radius" prop:value=move || match radius.get() { Some(ThemeRadius::None) => "none", Some(ThemeRadius::Full) => "full", _ => "inherit" } on:change=move |event| radius.set(match event_target_value(&event).as_str() { "none" => Some(ThemeRadius::None), "full" => Some(ThemeRadius::Full), _ => None })>
                        <option value="inherit">{move || tr(locale.get(), "Inherit", "继承")}</option><option value="none">"0"</option><option value="full">{move || tr(locale.get(), "Full", "全圆角")}</option>
                    </select></label>
                </Show>
            </div>
            <Show when=move || themed.get() && part.get()=="Select"><label class="block space-y-2 text-sm"><span>{move || tr(locale.get(), "List variant", "列表外观")}</span><select class="rounded-sm border border-border bg-input p-2" data-testid="parts-content-variant" prop:value=move || content_variant.get().as_str() on:change=move |event| content_variant.set(if event_target_value(&event)=="soft" { c::select::SelectContentVariant::Soft } else { c::select::SelectContentVariant::Solid })><option value="solid">"solid"</option><option value="soft">"soft"</option></select></label></Show>
            <div class="flex flex-wrap items-center gap-4 text-sm">
                <label class="flex items-center gap-2"><input type="checkbox" data-testid="parts-rtl" on:change=move |event| direction.set(if event_target_checked(&event) { p::Direction::RightToLeft } else { p::Direction::LeftToRight }) />"RTL"</label>
                <Show when=move || themed.get() && show_contrast.get()><label class="flex items-center gap-2"><input type="checkbox" data-testid="parts-contrast" prop:checked=move || contrast.get() on:change=move |event| contrast.set(event_target_checked(&event)) />{move || tr(locale.get(), "High contrast", "高对比")}</label></Show>
                <Show when=move || part.get()=="Select"><label class="flex items-center gap-2"><input type="checkbox" data-testid="parts-cancel" prop:checked=move || cancel_select.get() on:change=move |event| cancel_select.set(event_target_checked(&event)) />{move || tr(locale.get(), "Cancel selection", "取消选择")}</label></Show>
            </div>
            <Show when=move || part.get()=="Checkbox"><c::Button variant=c::ButtonVariant::Outline on_click=move || checked.set(p::CheckedState::Indeterminate) test_id="parts-mixed">{move || tr(locale.get(), "Set indeterminate", "设为不确定状态")}</c::Button></Show>
            <p class="text-sm text-muted-foreground">{move || tr(locale.get(), "Interact with the example. Requests are separate from accepted values. Number and colour fields keep a draft until input completes.", "操作下方样本，请求与接受值分别显示。数字与颜色字段在输入完成前保留草稿。")}</p>
            <p::DirectionProvider direction=direction>
                <div class="form-parts-example min-w-0 rounded-md border border-border p-4" data-testid="parts-example" data-presentation=move || if themed.get() { "themes" } else { "primitives" }>
                    {move || {
                        let index = size.get(); let treatment = variant.get(); let local = color.get(); let radius = radius.get(); let hc = contrast.get();
                        let disabled = state.get()=="disabled"; let readonly = state.get()=="readonly"; let invalid = state.get()=="invalid";
                        let locale = locale.get(); let email = tr(locale,"Email","邮件"); let feed = tr(locale,"Activity feed","动态列表"); let label = tr(locale,"Interactive example","交互样本"); let start_label = tr(locale,"Range start","范围起点"); let end_label = tr(locale,"Range end","范围终点");
                        let entries = vec![p::select::SelectEntry::new("a", email), p::select::SelectEntry::new("b", feed), p::select::SelectEntry::new("off",tr(locale,"Unavailable","暂不可用")).disabled()];
                        if themed.get() {
                            match part.get().as_str() {
                                "Checkbox" => view! { <c::checkbox::Root checked=checked on_change=move |value| request.run(Request::Checked(value)) disabled=disabled read_only=readonly invalid=invalid aria_label=label size=size!(c::checkbox::CheckboxSize,index) variant=variant!(c::checkbox::CheckboxVariant,treatment.as_str()) color=local high_contrast=hc><c::checkbox::Control test_id="parts-control" /><c::checkbox::Indicator /></c::checkbox::Root> }.into_any(),
                                "Switch" => view! { <c::switch::Root checked=flag on_change=move |value| request.run(Request::Flag(value)) disabled=disabled read_only=readonly invalid=invalid aria_label=label size=size!(c::switch::SwitchSize,index) variant=variant!(c::switch::SwitchVariant,treatment.as_str()) color=local high_contrast=hc radius=radius><c::switch::Control test_id="parts-control" /><c::switch::Thumb /></c::switch::Root> }.into_any(),
                                "Toggle" => view! { <c::toggle::Root pressed=flag on_change=move |value| request.run(Request::Flag(value)) disabled=disabled read_only=readonly invalid=invalid test_id="parts-control">{email}</c::toggle::Root> }.into_any(),
                                "RadioGroup" => view! { <c::radio_group::Root value=choice on_change=move |value| request.run(Request::Choice(value)) disabled=disabled read_only=readonly invalid=invalid aria_label=label size=size!(c::radio_group::RadioGroupSize,index) variant=variant!(c::radio_group::RadioGroupVariant,treatment.as_str()) color=local high_contrast=hc class="flex flex-wrap gap-4" test_id="parts-control"><c::radio_group::Item value="a" test_id="parts-a">{email}</c::radio_group::Item><c::radio_group::Item value="b" test_id="parts-b">{feed}</c::radio_group::Item></c::radio_group::Root> }.into_any(),
                                "CheckboxGroup" => view! { <c::checkbox_group::Root value=choices on_change=move |value| request.run(Request::Choices(value)) disabled=disabled read_only=readonly invalid=invalid aria_label=label size=size!(c::checkbox_group::CheckboxGroupSize,index) variant=variant!(c::checkbox_group::CheckboxGroupVariant,treatment.as_str()) color=local high_contrast=hc class="flex flex-wrap gap-4" test_id="parts-control"><c::checkbox_group::Item value="a" test_id="parts-a">{email}</c::checkbox_group::Item><c::checkbox_group::Item value="b" test_id="parts-b">{feed}</c::checkbox_group::Item></c::checkbox_group::Root> }.into_any(),
                                "ToggleGroup" => view! { <c::toggle_group::Root value=choices on_change=move |value| request.run(Request::Choices(value)) mode=p::SelectionMode::Multiple disabled=disabled read_only=readonly invalid=invalid aria_label=label test_id="parts-control"><c::toggle_group::Item value="a" test_id="parts-a">{email}</c::toggle_group::Item><c::toggle_group::Item value="b" test_id="parts-b">{feed}</c::toggle_group::Item></c::toggle_group::Root> }.into_any(),
                                "RadioCards" => view! { <c::radio_cards::Root value=choice on_change=move |value| request.run(Request::Choice(value)) disabled=disabled read_only=readonly invalid=invalid aria_label=label size=size!(c::radio_cards::RadioCardsSize,index) variant=if treatment=="classic" {c::radio_cards::RadioCardsVariant::Classic} else {c::radio_cards::RadioCardsVariant::Surface} color=local high_contrast=hc test_id="parts-control"><c::radio_cards::Item value="a" test_id="parts-a">{email}</c::radio_cards::Item><c::radio_cards::Item value="b" test_id="parts-b">{feed}</c::radio_cards::Item></c::radio_cards::Root> }.into_any(),
                                "CheckboxCards" => view! { <c::checkbox_cards::Root value=choices on_change=move |value| request.run(Request::Choices(value)) disabled=disabled read_only=readonly invalid=invalid aria_label=label size=size!(c::checkbox_cards::CheckboxCardsSize,index) variant=if treatment=="classic" {c::checkbox_cards::CheckboxCardsVariant::Classic} else {c::checkbox_cards::CheckboxCardsVariant::Surface} color=local high_contrast=hc test_id="parts-control"><c::checkbox_cards::Item value="a" test_id="parts-a">{email}</c::checkbox_cards::Item><c::checkbox_cards::Item value="b" test_id="parts-b">{feed}</c::checkbox_cards::Item></c::checkbox_cards::Root> }.into_any(),
                                "Slider" => view! { <c::Slider value=scalar on_change=move |value| request.run(Request::Scalar(value)) disabled=disabled read_only=readonly invalid=invalid aria_label=label size=size!(c::slider::SliderSize,index) variant=variant!(c::slider::SliderVariant,treatment.as_str()) color=local high_contrast=hc radius=radius test_id="parts-control" /> }.into_any(),
                                "RangeSlider" => view! { <c::slider::Root values=range on_change=move |value| request.run(Request::Range(value)) on_commit=Callback::new(move |_| events.set("commit".into())) on_cancel=Callback::new(move |_| events.set("cancel".into())) disabled=disabled read_only=readonly invalid=invalid aria_label=label size=size!(c::slider::SliderSize,index) variant=variant!(c::slider::SliderVariant,treatment.as_str()) color=local high_contrast=hc radius=radius test_id="parts-control"><c::slider::Track><c::slider::Range /></c::slider::Track><c::slider::Thumb id="start" aria_label=start_label test_id="parts-start" /><c::slider::Thumb id="end" aria_label=end_label test_id="parts-end" /></c::slider::Root> }.into_any(),
                                "TextArea" => view! { <c::TextArea value=text on_change=move |value| request.run(Request::Text(value)) disabled=disabled read_only=readonly invalid=invalid aria_label=label size=size!(c::textarea::TextAreaSize,index) variant=variant!(c::textarea::TextAreaVariant,treatment.as_str()) color=local radius=radius test_id="parts-control" /> }.into_any(),
                                "NumberField" => view! { <c::NumberField value=number on_change=move |value| request.run(Request::Number(value)) disabled=disabled read_only=readonly invalid=invalid aria_label=label size=size!(c::number_field::NumberFieldSize,index) variant=variant!(c::number_field::NumberFieldVariant,treatment.as_str()) color=local radius=radius test_id="parts-control" /> }.into_any(),
                                "ColorField" => view! { <c::ColorField value=color_value on_change=move |value| request.run(Request::Color(value)) disabled=disabled read_only=readonly invalid=invalid aria_label=label size=size!(c::color_field::ColorFieldSize,index) variant=variant!(c::color_field::ColorFieldVariant,treatment.as_str()) color=local radius=radius test_id="parts-control" /> }.into_any(),
                                "Select" => {
                                    let variant = match treatment.as_str() {"classic"=>c::select::SelectTriggerVariant::Classic,"soft"=>c::select::SelectTriggerVariant::Soft,"ghost-offset"=>c::select::SelectTriggerVariant::GhostOffset,"ghost"=>c::select::SelectTriggerVariant::Ghost,_=>c::select::SelectTriggerVariant::Surface};
                                    view! { <c::select::Root value=choice on_change=move |value| request.run(Request::Choice(value)) open=open on_open_change=move |value| open.set(value) entries=entries disabled=disabled read_only=readonly invalid=invalid aria_label=label size=size!(c::select::SelectSize,index) on_select=Callback::new(move |event: rustify_ui::Cancelable<String>| { events.set(format!("select:{}",event.value())); if cancel_select.get_untracked() { event.prevent_default(); } })><c::select::Trigger variant=variant color=local radius=radius test_id="parts-control" /><c::select::Content variant=content_variant color=local high_contrast=hc test_id="parts-select-list"><c::select::Viewport><c::select::Item value="a" /><c::select::Item value="b" /><c::select::Item value="off" /></c::select::Viewport></c::select::Content></c::select::Root> }.into_any()
                                },
                                _ => view! { <c::TextField value=text on_change=move |value| request.run(Request::Text(value)) disabled=disabled read_only=readonly invalid=invalid aria_label=label size=size!(c::text_field::TextFieldSize,index) variant=variant!(c::text_field::TextFieldVariant,treatment.as_str()) color=local radius=radius test_id="parts-control" /> }.into_any(),
                            }
                        } else {
                            match part.get().as_str() {
                                "Checkbox" => view! { <p::checkbox::Root checked=checked on_change=move |value| request.run(Request::Checked(value)) disabled=disabled read_only=readonly invalid=invalid aria_label=label><p::checkbox::Control test_id="parts-control" /><p::checkbox::Indicator>{move || if checked.get()==p::CheckedState::Indeterminate { "−" } else { "✓" }}</p::checkbox::Indicator></p::checkbox::Root> }.into_any(),
                                "Switch" => view! { <p::switch::Root checked=flag on_change=move |value| request.run(Request::Flag(value)) disabled=disabled read_only=readonly invalid=invalid aria_label=label><p::switch::Control test_id="parts-control" /><p::switch::Thumb /></p::switch::Root> }.into_any(),
                                "Toggle" => view! { <p::toggle::Root pressed=flag on_change=move |value| request.run(Request::Flag(value)) disabled=disabled read_only=readonly invalid=invalid test_id="parts-control">{email}</p::toggle::Root> }.into_any(),
                                "RadioGroup" | "RadioCards" => view! { <p::radio_group::Root value=choice on_change=move |value| request.run(Request::Choice(value)) disabled=disabled read_only=readonly invalid=invalid aria_label=label test_id="parts-control"><p::radio_group::Item value="a" test_id="parts-a">{email}</p::radio_group::Item><p::radio_group::Item value="b" test_id="parts-b">{feed}</p::radio_group::Item></p::radio_group::Root> }.into_any(),
                                "CheckboxGroup" | "CheckboxCards" => view! { <p::checkbox_group::Root value=choices on_change=move |value| request.run(Request::Choices(value)) disabled=disabled read_only=readonly invalid=invalid aria_label=label test_id="parts-control"><p::checkbox_group::Item value="a" test_id="parts-a">{email}</p::checkbox_group::Item><p::checkbox_group::Item value="b" test_id="parts-b">{feed}</p::checkbox_group::Item></p::checkbox_group::Root> }.into_any(),
                                "ToggleGroup" => view! { <p::toggle_group::Root value=choices on_change=move |value| request.run(Request::Choices(value)) mode=p::SelectionMode::Multiple disabled=disabled read_only=readonly invalid=invalid aria_label=label test_id="parts-control"><p::toggle_group::Item value="a" test_id="parts-a">{email}</p::toggle_group::Item><p::toggle_group::Item value="b" test_id="parts-b">{feed}</p::toggle_group::Item></p::toggle_group::Root> }.into_any(),
                                "Slider" | "RangeSlider" => {
                                    let single = part.get()=="Slider";
                                    let values = if single { Signal::derive(move || vec![c::slider::ThumbValue {id:"start".into(),value:scalar.get()}]) } else { range.into() };
                                    view! { <p::slider::Root values=values on_change={move |value: Vec<c::slider::ThumbValue>| { if single { if let Some(value)=value.first() { request.run(Request::Scalar(value.value)); } } else { request.run(Request::Range(value)); } }} on_commit=Callback::new(move |_| events.set("commit".into())) on_cancel=Callback::new(move |_| events.set("cancel".into())) disabled=disabled read_only=readonly invalid=invalid aria_label=label test_id="parts-control"><p::slider::Track><p::slider::Range /></p::slider::Track><p::slider::Thumb id="start" aria_label=start_label test_id="parts-start" /><Show when=move || !single><p::slider::Thumb id="end" aria_label=end_label test_id="parts-end" /></Show></p::slider::Root> }.into_any()
                                },
                                "TextArea" => view! { <p::text_area::Root value=text on_change=move |value| request.run(Request::Text(value)) disabled=disabled read_only=readonly invalid=invalid aria_label=label test_id="parts-control" /> }.into_any(),
                                "NumberField" => view! { <p::number_field::Root value=number on_change=move |value| request.run(Request::Number(value)) disabled=disabled read_only=readonly invalid=invalid aria_label=label><p::number_field::Input test_id="parts-control" /></p::number_field::Root> }.into_any(),
                                "ColorField" => view! { <p::color_field::Root value=color_value on_change=move |value| request.run(Request::Color(value)) disabled=disabled read_only=readonly invalid=invalid aria_label=label><p::color_field::Input test_id="parts-control" /><p::color_field::Swatch /></p::color_field::Root> }.into_any(),
                                "Select" => view! { <p::select::Root value=choice on_change=move |value| request.run(Request::Choice(value)) open=open on_open_change=move |value| open.set(value) entries=entries disabled=disabled read_only=readonly invalid=invalid aria_label=label on_select=Callback::new(move |event: rustify_ui::Cancelable<String>| { events.set(format!("select:{}",event.value())); if cancel_select.get_untracked() { event.prevent_default(); } })><p::select::Trigger test_id="parts-control"><p::select::Value /><p::select::Icon>"▾"</p::select::Icon></p::select::Trigger><p::select::Content class="form-parts-popup" test_id="parts-select-list"><p::select::Viewport><p::select::Item value="a" /><p::select::Item value="b" /><p::select::Item value="off" /></p::select::Viewport></p::select::Content></p::select::Root> }.into_any(),
                                _ => view! { <p::text_field::Root value=text on_change=move |value| request.run(Request::Text(value)) disabled=disabled read_only=readonly invalid=invalid aria_label=label><p::text_field::Input test_id="parts-control" /></p::text_field::Root> }.into_any(),
                            }
                        }
                    }}
                </div>
            </p::DirectionProvider>
            <dl class="grid gap-2 text-sm sm:grid-cols-2">
                <div><dt class="text-muted-foreground">{move || tr(locale.get(), "Request count", "请求次数")}</dt><dd data-testid="parts-count">{move || count.get()}</dd></div>
                <div><dt class="text-muted-foreground">{move || tr(locale.get(), "Last request", "最后请求")}</dt><dd class="break-words" data-testid="parts-request">{move || last.with(|value| if value.is_empty() { "—".into() } else { value.clone() })}</dd></div>
                <div><dt class="text-muted-foreground">{move || tr(locale.get(), "Accepted value", "接受值")}</dt><dd class="break-words" data-testid="parts-accepted">{move || accepted.get().describe()}</dd></div>
                <div><dt class="text-muted-foreground">{move || tr(locale.get(), "Commit / cancel event", "确认／取消事件")}</dt><dd class="break-words" data-testid="parts-events">{move || events.with(|value| if value.is_empty() { "—".into() } else { value.clone() })}</dd></div>
            </dl>
            <c::Button disabled=Signal::derive(move || pending.get().is_none()) on_click=move || { if let Some(value) = pending.get_untracked() { apply.run(value); pending.set(None); } } test_id="parts-apply">{move || tr(locale.get(), "Accept pending value", "接受待定值")}</c::Button>
        </div>
    }
}
