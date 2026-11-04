use crate::{
    controller::StudioState,
    gallery_catalog::{GalleryEntry, Section as GallerySection, ENTRIES},
};
use leptos::prelude::*;
use rustify_components::*;
use rustify_ui::Locale;

fn tr(locale: Signal<Locale>, en: &'static str, zh: &'static str) -> &'static str {
    if locale.get() == Locale::Chinese {
        zh
    } else {
        en
    }
}
fn choice<T: Copy + PartialEq + Send + Sync + 'static>(
    label: &'static str,
    all: &'static [T],
    initial: T,
    text: impl Fn(T) -> String,
) -> (Signal<T>, AnyView) {
    let index = RwSignal::new(
        all.iter()
            .position(|value| *value == initial)
            .expect("gallery default"),
    );
    let options = all
        .iter()
        .copied()
        .enumerate()
        .map(|(index, value)| view! { <option value=index.to_string()>{text(value)}</option> })
        .collect_view();
    let control = view! { <label class="gallery-control"><span>{label}</span><select aria-label=label prop:value=move ||index.get().to_string() on:change=move |event| {
        if let Ok(next)=event_target_value(&event).parse::<usize>() { if next<all.len() { index.set(next); } }
    }>{options}</select></label> }.into_any();
    (Signal::derive(move || all[index.get()]), control)
}
macro_rules! visuals {
    ($size:ty, $variant:ty) => {{
        let (size, size_control) = choice("Size", <$size>::ALL, <$size>::default(), |value| {
            format!("{value:?}")
        });
        let (variant, variant_control) =
            choice("Variant", <$variant>::ALL, <$variant>::default(), |value| {
                value.as_str().into()
            });
        (
            Signal::derive(move || Responsive::from(size.get())),
            variant,
            view! { <div class="gallery-controls">{size_control}{variant_control}</div> },
        )
    }};
}
fn palette() -> (
    Signal<Option<rustify_ui::AccentColor>>,
    RwSignal<bool>,
    AnyView,
) {
    use rustify_ui::AccentColor;
    let (color, control) = choice(
        "Palette",
        &[None, Some(AccentColor::Blue), Some(AccentColor::Gray)],
        None,
        |value| value.map_or_else(|| "Inherit".into(), |value| format!("{value:?}")),
    );
    let high = RwSignal::new(false);
    (color,high,view! { <div class="gallery-controls">{control}<StateToggle label="High contrast" value=high /></div> }.into_any())
}
#[component]
fn StateToggle(label: &'static str, value: RwSignal<bool>) -> impl IntoView {
    view! { <label class="gallery-control"><input type="checkbox" prop:checked=move ||value.get() on:change=move |event|value.set(event_target_checked(&event)) />{label}</label> }
}
#[derive(Clone)]
struct GalleryLinks(String);
fn gallery_id(section: &str) -> String {
    format!(
        "{}-{section}",
        use_context::<GalleryLinks>().expect("gallery links").0
    )
}
fn gallery_href(section: &str) -> String {
    format!("#{}", gallery_id(section))
}

#[component]
pub fn Gallery(
    locale: Signal<Locale>,
    #[prop(optional_no_strip)] state: Option<StudioState>,
) -> impl IntoView {
    let owner = Owner::new();
    let view=owner.with(move || {
    provide_context(GalleryLinks(id::next("gallery")));
    let show_parts = RwSignal::new(false);
    view! { <section class="component-gallery" data-testid="component-gallery" data-show-parts=move ||show_parts.get().to_string()>
        <header class="gallery-header"><h2>{move ||tr(locale,"Components","组件陈列")}</h2>
            <p>{move ||tr(locale,"Working samples · explore treatments, states and parts.","真实组件样本 · 查看外观、状态与部件。")}</p>
            <label class="gallery-control"><input type="checkbox" prop:checked=move ||show_parts.get() on:change=move |event|show_parts.set(event_target_checked(&event)) />{move ||tr(locale,"Show parts","显示部件")}</label>
            <nav aria-label="Component sections">{GallerySection::ALL.into_iter().map(|section|{
                let label=section.label(); view! { <a href=gallery_href(label[0])>{move ||tr(locale,label[0],label[1])}</a> }
            }).collect_view()}</nav>
        </header>
        {GallerySection::ALL.into_iter().map(move |section| {
            let label=section.label();
            view! { <section class="gallery-section" id=gallery_id(label[0])>
                <h3>{move ||tr(locale,label[0],label[1])}</h3>
                <div class="gallery-grid">{ENTRIES.iter().filter(|entry|entry.section==section).map(move |entry|view! { <Sample entry=entry locale=locale /> }).collect_view()}</div>
            </section> }
        }).collect_view()}
        {state.map(move |state|view! { <LocalThemeSample state=state /> })}
    </section> }
    });
    leptos::tachys::reactive_graph::OwnedView::new_with_owner(view, owner)
}
#[component]
fn Sample(entry: &'static GalleryEntry, locale: Signal<Locale>) -> impl IntoView {
    view! { <article class="gallery-sample" data-gallery-entry=entry.references[0]>
        <h4>{entry.references.join(" · ")}</h4>
        <div class="gallery-demo">{sample(entry.references[0],locale)}</div>
        <details class="gallery-structure"><summary>{move ||tr(locale,"Structure & primitives","结构与原语")}</summary>
            <pre>{entry.parts.join("\n")}</pre>
            {entry.primitives_path.map(|path|view! { <code>{format!("rustify_components::{path}")}</code> })}
        </details>
    </article> }
}
fn sample(name: &str, locale: Signal<Locale>) -> AnyView {
    match name {
        "Button" => view! { <Buttons /> }.into_any(),
        "Checkbox" => view! { <CheckboxSample /> }.into_any(),
        "CheckboxGroup" => view! { <CheckboxGroupSample /> }.into_any(),
        "CheckboxCards" => view! { <CheckboxCardsSample /> }.into_any(),
        "Radio" => view! { <RadioSample /> }.into_any(),
        "RadioCards" => view! { <RadioCardsSample /> }.into_any(),
        "Switch" => view! { <SwitchSample /> }.into_any(),
        "Toggle" => view! { <ToggleSample /> }.into_any(),
        "ToggleGroup" => view! { <ToggleGroupSample /> }.into_any(),
        "SegmentedControl" => view! { <SegmentedSample /> }.into_any(),
        "Slider" => view! { <SliderSample /> }.into_any(),
        "Select" => view! { <SelectSample /> }.into_any(),
        "Form" => view! { <FieldSamples locale=locale /> }.into_any(),
        "Dialog" => view! { <DialogSample /> }.into_any(),
        "AlertDialog" => view! { <AlertSample /> }.into_any(),
        "Popover" => view! { <PopoverSample /> }.into_any(),
        "HoverCard" => view! { <HoverSample /> }.into_any(),
        "Tooltip" => view! { <TooltipSample /> }.into_any(),
        "DropdownMenu" => view! { <DropdownSample /> }.into_any(),
        "ContextMenu" => view! { <ContextSample /> }.into_any(),
        "Menubar" => view! { <MenubarSample /> }.into_any(),
        "NavigationMenu" => view! { <NavigationSample /> }.into_any(),
        "Toolbar" => view! { <ToolbarSample /> }.into_any(),
        "Tabs" => view! { <TabsSample /> }.into_any(),
        "TabNav" => view! { <TabNavSample /> }.into_any(),
        "Accordion" => view! { <AccordionSample /> }.into_any(),
        "Collapsible" => view! { <CollapsibleSample /> }.into_any(),
        "Avatar" => view! { <AvatarSample /> }.into_any(),
        "Badge" => view! { <BadgeSample /> }.into_any(),
        "Card" => view! { <CardSample /> }.into_any(),
        "Callout" => view! { <CalloutSample /> }.into_any(),
        "DataList" => view! { <DataListSample /> }.into_any(),
        "Table" => view! { <TableSample /> }.into_any(),
        "ScrollArea" => view! { <ScrollSample /> }.into_any(),
        "Progress" => view! { <ProgressSample /> }.into_any(),
        "Skeleton" => view! { <SkeletonSample /> }.into_any(),
        "Spinner" => view! { <SpinnerSample /> }.into_any(),
        "Toast" => view! { <ToastSample /> }.into_any(),
        "Box" => view! { <LayoutSample /> }.into_any(),
        "AspectRatio" => view! { <AspectRatio ratio=16.0/9.0><div class="gallery-ratio">"16 : 9"</div></AspectRatio> }.into_any(),
        "Separator" => view! { <SeparatorSample /> }.into_any(),
        "Text" => view! { <TypographySample /> }.into_any(),
        "AccessibleIcon" => view! { <AccessibilitySample /> }.into_any(),
        _ => unreachable!("gallery entry has a sample"),
    }
}
#[component]
fn Buttons() -> impl IntoView {
    let (color, high, appearance) = palette();
    let (size, variant, controls) = visuals!(ButtonSize, ButtonVariant);
    let disabled = RwSignal::new(false);
    let loading = RwSignal::new(false);
    let count = RwSignal::new(0);
    view! { {appearance} {controls}<div class="gallery-controls"><StateToggle label="Disabled" value=disabled /><StateToggle label="Loading" value=loading /></div>
        <div class="gallery-row"><Button color=color high_contrast=high size=size variant=variant disabled=disabled loading=loading on_click=move ||count.update(|n|*n+=1)>"Create"</Button>
        <IconButton color=color high_contrast=high size=size variant=variant disabled=disabled loading=loading aria_label="Add item" on_click=move ||count.update(|n|*n+=1)><Icon glyph=Glyph::Check /></IconButton></div>
        <output>{move ||format!("{} created",count.get())}</output>
    }
}
#[component]
fn CheckboxSample() -> impl IntoView {
    let (color, high, appearance) = palette();
    let (size, variant, controls) = visuals!(checkbox::CheckboxSize, checkbox::CheckboxVariant);
    let value = RwSignal::new(false);
    let disabled = RwSignal::new(false);
    let mixed = RwSignal::new(false);
    view! { {appearance} {controls}<StateToggle label="Disabled" value=disabled /><StateToggle label="Indeterminate" value=mixed />
        <label class="gallery-row"><Checkbox color=color high_contrast=high checked=value indeterminate=mixed on_change=move |next|{mixed.set(false);value.set(next);} size=size variant=variant disabled=disabled />"Receive updates"</label>
    }
}
#[component]
fn CheckboxGroupSample() -> impl IntoView {
    let (color, high, appearance) = palette();
    let (size, variant, controls) = visuals!(
        checkbox_group::CheckboxGroupSize,
        checkbox_group::CheckboxGroupVariant
    );
    let value = RwSignal::new(vec!["mail".into()]);
    let disabled = RwSignal::new(false);
    view! { {appearance} {controls}<StateToggle label="Disabled" value=disabled /><CheckboxGroup color=color high_contrast=high value=value on_change=move |next|value.set(next) options=Signal::stored(vec![CheckboxOption::new("mail","Email"),CheckboxOption::new("news","News")]) size=size variant=variant disabled=disabled aria_label="Channels" /> }
}
#[component]
fn CheckboxCardsSample() -> impl IntoView {
    let (color, high, appearance) = palette();
    let (size, variant, controls) = visuals!(
        checkbox_cards::CheckboxCardsSize,
        checkbox_cards::CheckboxCardsVariant
    );
    let value = RwSignal::new(vec!["mail".into()]);
    let disabled = RwSignal::new(false);
    view! { {appearance} {controls}<StateToggle label="Disabled" value=disabled /><checkbox_cards::Root color=color high_contrast=high value=value on_change=move |next|value.set(next) size=size variant=variant disabled=disabled aria_label="Channels">
        <checkbox_cards::Item value="mail">"Email updates"</checkbox_cards::Item><checkbox_cards::Item value="news">"Product news"</checkbox_cards::Item>
    </checkbox_cards::Root> }
}
#[component]
fn RadioSample() -> impl IntoView {
    let (color, high, appearance) = palette();
    let (size, variant, controls) =
        visuals!(radio_group::RadioGroupSize, radio_group::RadioGroupVariant);
    let value = RwSignal::new("personal".to_owned());
    let disabled = RwSignal::new(false);
    view! { {appearance} {controls}<StateToggle label="Disabled" value=disabled /><RadioGroup color=color high_contrast=high value=value on_change=move |next|value.set(next) options=Signal::stored(vec![RadioOption::new("personal","Personal"),RadioOption::new("team","Team")]) size=size variant=variant disabled=disabled aria_label="Plan" /> }
}
#[component]
fn RadioCardsSample() -> impl IntoView {
    let (color, high, appearance) = palette();
    let (size, variant, controls) =
        visuals!(radio_cards::RadioCardsSize, radio_cards::RadioCardsVariant);
    let value = RwSignal::new(Some("personal".to_owned()));
    let disabled = RwSignal::new(false);
    view! { {appearance} {controls}<StateToggle label="Disabled" value=disabled /><radio_cards::Root color=color high_contrast=high value=value on_change=move |next|value.set(next) size=size variant=variant disabled=disabled aria_label="Plan">
        <radio_cards::Item value="personal">"Personal"</radio_cards::Item><radio_cards::Item value="team">"Team"</radio_cards::Item>
    </radio_cards::Root> }
}
#[component]
fn SwitchSample() -> impl IntoView {
    let (color, high, appearance) = palette();
    let (size, variant, controls) = visuals!(switch::SwitchSize, switch::SwitchVariant);
    let value = RwSignal::new(false);
    let disabled = RwSignal::new(false);
    view! { {appearance} {controls}<StateToggle label="Disabled" value=disabled /><label class="gallery-row"><Switch color=color high_contrast=high checked=value on_change=move |next|value.set(next) size=size variant=variant disabled=disabled />"Notifications"</label> }
}
#[component]
fn ToggleSample() -> impl IntoView {
    let value = RwSignal::new(false);
    let disabled = RwSignal::new(false);
    view! { <StateToggle label="Disabled" value=disabled /><Toggle pressed=value on_change=move |next|value.set(next) disabled=disabled aria_label="Bold">"Bold"</Toggle> }
}
#[component]
fn ToggleGroupSample() -> impl IntoView {
    let value = RwSignal::new(vec!["bold".into()]);
    let disabled = RwSignal::new(false);
    view! { <StateToggle label="Disabled" value=disabled /><ToggleGroup value=value on_change=move |next|value.set(next) disabled=disabled multiple=true aria_label="Text formatting" items=Signal::stored(vec![ToggleItem::new("bold","Bold"),ToggleItem::new("italic","Italic")]) /> }
}
#[component]
fn SegmentedSample() -> impl IntoView {
    let (size, variant, controls) = visuals!(
        segmented_control::SegmentedControlSize,
        segmented_control::SegmentedControlVariant
    );
    let value = RwSignal::new(Some("week".to_owned()));
    let disabled = RwSignal::new(false);
    view! { {controls}<StateToggle label="Disabled" value=disabled /><segmented_control::Root value=value on_change=move |next|value.set(next) size=size variant=variant disabled=disabled aria_label="Period">
        <segmented_control::Item value="week">"Week"</segmented_control::Item><segmented_control::Item value="month">"Month"</segmented_control::Item>
    </segmented_control::Root> }
}
#[component]
fn SliderSample() -> impl IntoView {
    let (color, high, appearance) = palette();
    let (size, variant, controls) = visuals!(SliderSize, SliderVariant);
    let value = RwSignal::new(40.);
    let disabled = RwSignal::new(false);
    let range = RwSignal::new(vec![
        ThumbValue {
            id: "start".into(),
            value: 20.,
        },
        ThumbValue {
            id: "end".into(),
            value: 80.,
        },
    ]);
    view! { {appearance} {controls}<StateToggle label="Disabled" value=disabled />
        <Slider color=color high_contrast=high value=value on_change=move |next|value.set(next) size=size variant=variant disabled=disabled aria_label="Volume" />
        <RangeSlider color=color high_contrast=high values=range on_change=move |next|range.set(next) thumbs=vec![ThumbLabel { id:"start".into(),label:"Start".into() },ThumbLabel { id:"end".into(),label:"End".into() }] size=size variant=variant disabled=disabled aria_label="Range" />
        <output>{move ||format!("Volume {}",value.get())}</output>
    }
}
#[component]
fn SelectSample() -> impl IntoView {
    let (color, high, appearance) = palette();
    let (size, variant, controls) = visuals!(select::SelectSize, select::SelectTriggerVariant);
    let (content, content_control) = choice(
        "Content variant",
        select::SelectContentVariant::ALL,
        select::SelectContentVariant::default(),
        |v| v.as_str().into(),
    );
    let value = RwSignal::new("team".into());
    let open = RwSignal::new(false);
    let disabled = RwSignal::new(false);
    let empty = RwSignal::new(false);
    view! { {appearance} {controls}{content_control}<StateToggle label="Disabled" value=disabled /><StateToggle label="Empty" value=empty />
        <Select color=color high_contrast=high value=value on_change=move |next|value.set(next) open=open on_open_change=move |next|open.set(next) options=Signal::derive(move ||if empty.get(){vec![]}else{vec![SelectOption::new("personal","Personal"),SelectOption::new("team","Team")]}) size=size variant=variant content_variant=content disabled=disabled aria_label="Plan" />
        <Show when=move ||empty.get()><p>"No plans available."</p><button type="button" on:click=move |_|empty.set(false)>"Restore options"</button></Show>
    }
}
#[component]
fn FieldSamples(locale: Signal<Locale>) -> impl IntoView {
    use form::field as f;
    let (size, variant, controls) = visuals!(input::TextFieldSize, input::TextFieldVariant);
    let (area_size, area_variant, area_controls) =
        visuals!(textarea::TextAreaSize, textarea::TextAreaVariant);
    let disabled = RwSignal::new(false);
    let name = RwSignal::new(String::new());
    let notes = RwSignal::new(String::new());
    let amount = RwSignal::new(12.);
    let color = RwSignal::new("#336699".into());
    let submitted = RwSignal::new(false);
    let held = StoredValue::new(None::<Form>);
    let form = Form::new(&["name", "notes"], move || {
        submitted.set(true);
        held.get_value().expect("sample form").submitted(Ok(()));
    });
    held.set_value(Some(form));
    view! { <StateToggle label="Disabled" value=disabled />{controls}
        <FormRoot form=form rules=Vec::new>
            <f::Root form=form field="name"><f::Label>{move ||tr(locale,"Name","姓名")}</f::Label>
                <TextField value=name on_change=move |next|{name.set(next);form.changed("name");submitted.set(false);} name="name" size=size variant=variant disabled=disabled required=true />
                <f::Description>"Required for this local sample."</f::Description><f::Message />
            </f::Root>
            {area_controls}
            <f::Root form=form field="notes"><f::Label>{move ||tr(locale,"Notes","备注")}</f::Label>
                <TextArea value=notes on_change=move |next|{notes.set(next);form.changed("notes");} name="notes" rows=Some(3) size=area_size variant=area_variant disabled=disabled />
                <f::Message />
            </f::Root>
            <label class="gallery-control">"Quantity"<NumberField value=amount on_change=move |next|amount.set(next) disabled=disabled /></label>
            <label class="gallery-control">"Color"<ColorField value=color on_change=move |next|color.set(next) disabled=disabled /></label>
            <Button r#type=ButtonType::Submit disabled=disabled on_click=||{}>"Save locally"</Button>
            <p role="status">{move ||if submitted.get(){tr(locale,"Saved in this sample.","已保存在此样本。")}else{""}}</p>
        </FormRoot>
    }
}
macro_rules! sizing {
    ($size:ty) => {{
        let (size, control) = choice("Size", <$size>::ALL, <$size>::default(), |v| {
            format!("{v:?}")
        });
        (
            Signal::derive(move || Responsive::from(size.get())),
            control,
        )
    }};
}
#[component]
fn DialogSample() -> impl IntoView {
    let (size, control) = sizing!(dialog::DialogSize);
    let open = RwSignal::new(false);
    let disabled = RwSignal::new(false);
    view! { {control}<StateToggle label="Disabled" value=disabled /><dialog::Root open=open on_open_change=move |next|open.set(next)>
        <dialog::Trigger attr:disabled=move ||disabled.get()>"Edit profile"</dialog::Trigger><dialog::Portal><dialog::Overlay /><dialog::Content size=size>
            <dialog::Title>"Edit profile"</dialog::Title><dialog::Description>"Changes stay in this preview."</dialog::Description>
            <dialog::Close>"Done"</dialog::Close>
        </dialog::Content></dialog::Portal>
    </dialog::Root> }
}
#[component]
fn AlertSample() -> impl IntoView {
    let (size, control) = sizing!(dialog::DialogSize);
    let open = RwSignal::new(false);
    let count = RwSignal::new(0);
    view! { {control}<alert_dialog::Root open=open on_open_change=move |next|open.set(next)>
        <alert_dialog::Trigger>"Archive draft"</alert_dialog::Trigger><alert_dialog::Portal><alert_dialog::Overlay /><alert_dialog::Content size=size>
            <alert_dialog::Title>"Archive this draft?"</alert_dialog::Title><alert_dialog::Description>"This changes only the demonstration count."</alert_dialog::Description>
            <alert_dialog::Cancel>"Keep draft"</alert_dialog::Cancel><alert_dialog::Action on_action=move |_|count.update(|n|*n+=1)>"Archive"</alert_dialog::Action>
        </alert_dialog::Content></alert_dialog::Portal>
    </alert_dialog::Root><output>{move ||format!("{} archived",count.get())}</output> }
}
#[component]
fn PopoverSample() -> impl IntoView {
    let (size, control) = sizing!(popover::PopoverSize);
    let open = RwSignal::new(false);
    view! { {control}<popover::Root open=open on_open_change=move |next|open.set(next)>
        <popover::Trigger>"Quick settings"</popover::Trigger><popover::Portal><popover::Content size=size aria_label="Quick settings"><p>"Settings for this preview."</p><popover::Close>"Done"</popover::Close><popover::Arrow /></popover::Content></popover::Portal>
    </popover::Root> }
}
#[component]
fn HoverSample() -> impl IntoView {
    let (size, control) = sizing!(hover_card::HoverCardSize);
    let open = RwSignal::new(false);
    view! { {control}<hover_card::Root open=open on_open_change=move |next|open.set(next)><hover_card::Trigger href=gallery_href("Overlay")>"Rustify UI"</hover_card::Trigger>
        <hover_card::Portal><hover_card::Content size=size><strong>"Rustify UI"</strong><p>"Native Rust components for interactive documents."</p><hover_card::Arrow /></hover_card::Content></hover_card::Portal>
    </hover_card::Root> }
}
#[component]
fn TooltipSample() -> impl IntoView {
    let open = RwSignal::new(false);
    view! { <tooltip::Provider><tooltip::Root open=open on_open_change=move |next|open.set(next)>
        <tooltip::Trigger>"Save"</tooltip::Trigger><tooltip::Portal><tooltip::Content>"Save changes locally"<tooltip::Arrow /></tooltip::Content></tooltip::Portal>
    </tooltip::Root></tooltip::Provider> }
}
#[component]
fn DropdownSample() -> impl IntoView {
    let (color, high, appearance) = palette();
    use dropdown_menu as m;
    let (size, variant, controls) = visuals!(m::MenuSize, m::MenuVariant);
    let open = RwSignal::new(false);
    let checked = RwSignal::new(primitives::CheckedState::Unchecked);
    let sub = RwSignal::new(false);
    let action = RwSignal::new("No action");
    let disabled = RwSignal::new(false);
    view! { {appearance} {controls}<StateToggle label="Disabled" value=disabled />
        <m::Root open=open on_open_change=move |next|open.set(next)><m::Trigger disabled=disabled>"Document actions"</m::Trigger><m::Portal><m::Content color=color high_contrast=high size=size variant=variant>
            <m::Label>"Document"</m::Label><m::Item on_select=move |_|action.set("Duplicated")>"Duplicate"</m::Item>
            <m::CheckboxItem checked=checked on_checked_change=move |next|checked.set(if next {primitives::CheckedState::Checked}else{primitives::CheckedState::Unchecked})>"Show guides"</m::CheckboxItem>
            <m::Separator /><m::Sub open=sub on_open_change=move |next|sub.set(next)><m::SubTrigger>"Export"</m::SubTrigger><m::Portal><m::SubContent><m::Item on_select=move |_|action.set("Exported as PDF")>"PDF"</m::Item></m::SubContent></m::Portal></m::Sub>
        </m::Content></m::Portal></m::Root><output>{move ||action.get()}</output>
    }
}
#[component]
fn ContextSample() -> impl IntoView {
    let (color, high, appearance) = palette();
    use context_menu as m;
    let (size, variant, controls) = visuals!(m::MenuSize, m::MenuVariant);
    let open = RwSignal::new(false);
    let action = RwSignal::new("No action");
    view! { {appearance} {controls}<m::Root open=open on_open_change=move |next|open.set(next)><m::Trigger class="gallery-context">"Right-click, long-press or Shift+F10"</m::Trigger><m::Portal><m::Content color=color high_contrast=high size=size variant=variant>
        <m::Item on_select=move |_|action.set("Copied")>"Copy"</m::Item><m::Item on_select=move |_|action.set("Pasted")>"Paste"</m::Item>
    </m::Content></m::Portal></m::Root><output>{move ||action.get()}</output> }
}
#[component]
fn MenubarSample() -> impl IntoView {
    let (color, high, appearance) = palette();
    use menubar as m;
    let (size, variant, controls) = visuals!(m::MenuSize, m::MenuVariant);
    let value = RwSignal::new(None::<String>);
    let action = RwSignal::new("No action");
    view! { {appearance} {controls}<m::Root value=value on_value_change=move |next|value.set(next) aria_label="Document commands">
        <m::Menu value="file"><m::Trigger>"File"</m::Trigger><m::Portal><m::Content color=color high_contrast=high size=size variant=variant><m::Item on_select=move |_|action.set("New document")>"New"</m::Item></m::Content></m::Portal></m::Menu>
        <m::Menu value="edit"><m::Trigger>"Edit"</m::Trigger><m::Portal><m::Content color=color high_contrast=high size=size variant=variant><m::Item on_select=move |_|action.set("Undone")>"Undo"</m::Item></m::Content></m::Portal></m::Menu>
    </m::Root><output>{move ||action.get()}</output> }
}
#[component]
fn NavigationSample() -> impl IntoView {
    use navigation_menu as n;
    let value = RwSignal::new(None::<String>);
    let disabled = RwSignal::new(false);
    view! { <StateToggle label="Disabled" value=disabled /><n::Root value=value on_value_change=move |next|value.set(next) aria_label="Documentation">
        <n::List><n::Item value="learn"><n::Trigger disabled=disabled>"Learn"</n::Trigger><n::Content><n::Link href=gallery_href("Content")>"Content components"</n::Link><n::Link href=gallery_href("Typography")>"Typography"</n::Link></n::Content></n::Item>
        <n::Item value="reference"><n::Link href=gallery_href("Interaction") active=true>"Reference"</n::Link></n::Item></n::List><n::Indicator /><n::Viewport />
    </n::Root> }
}
#[component]
fn ToolbarSample() -> impl IntoView {
    use toolbar as t;
    let value = RwSignal::new(vec!["bold".into()]);
    let count = RwSignal::new(0);
    let disabled = RwSignal::new(false);
    let open = RwSignal::new(false);
    view! { <StateToggle label="Disabled" value=disabled /><t::Root aria_label="Editor tools">
        <t::Button disabled=disabled on_click=Some(Callback::new(move |_|count.update(|n|*n+=1)))>"Save"</t::Button><t::Separator />
        <t::ToggleGroup value=value on_change=move |next|value.set(next) mode=primitives::SelectionMode::Multiple><t::ToggleItem value="bold">"B"</t::ToggleItem><t::ToggleItem value="italic">"I"</t::ToggleItem></t::ToggleGroup>
        <dropdown_menu::Root open=open on_open_change=move |next|open.set(next)>
            {dropdown_menu::trigger_as(leptos::html::button().r#type("button").child("More").add_any_attr(toolbar::item(false.into())))}
            <dropdown_menu::Portal><dropdown_menu::Content><dropdown_menu::Item on_select=move |_|count.update(|n|*n+=1)>"Save copy"</dropdown_menu::Item></dropdown_menu::Content></dropdown_menu::Portal>
        </dropdown_menu::Root>
        <t::Link href=gallery_href("Menu")>"Menus"</t::Link>
    </t::Root><output>{move ||format!("{} saved",count.get())}</output> }
}
#[component]
fn TabsSample() -> impl IntoView {
    let (color, high, appearance) = palette();
    let (size, control) = sizing!(tabs::TabsSize);
    let value = RwSignal::new(Some("overview".into()));
    let disabled = RwSignal::new(false);
    view! { {appearance} {control}<StateToggle label="Disabled" value=disabled /><tabs::Root value=value on_change=move |next|value.set(next) entries=Signal::derive(move ||vec![tabs::PanelEntry::new("overview"),tabs::PanelEntry { value:"details".into(),disabled:disabled.get() }])>
        <tabs::List color=color high_contrast=high size=size aria_label="Project"><tabs::Trigger value="overview">"Overview"</tabs::Trigger><tabs::Trigger value="details">"Details"</tabs::Trigger></tabs::List>
        <tabs::Content value="overview"><p>"Project overview"</p></tabs::Content><tabs::Content value="details"><p>"Project details"</p></tabs::Content>
    </tabs::Root> }
}
#[component]
fn TabNavSample() -> impl IntoView {
    let (color, high, appearance) = palette();
    let (size, control) = sizing!(tab_nav::TabNavSize);
    view! { {appearance} {control}<tab_nav::Root color=color high_contrast=high size=size aria_label="Gallery sections"><tab_nav::Link href=gallery_href("Navigation") active=true>"Navigation"</tab_nav::Link><tab_nav::Link href=gallery_href("Content")>"Content"</tab_nav::Link></tab_nav::Root> }
}
#[component]
fn AccordionSample() -> impl IntoView {
    let value = RwSignal::new(accordion::AccordionValue::Single(None));
    let disabled = RwSignal::new(false);
    view! { <StateToggle label="Disabled" value=disabled /><accordion::Root value=value on_change=move |next|value.set(next) disabled=disabled collapsible=true>
        <accordion::Item value="first"><accordion::Header><accordion::Trigger>"What stays local?"</accordion::Trigger></accordion::Header><accordion::Content>"All sample values stay in this preview."</accordion::Content></accordion::Item>
        <accordion::Item value="second"><accordion::Header><accordion::Trigger>"Can I inspect this?"</accordion::Trigger></accordion::Header><accordion::Content>"Turn on picking to inspect part tokens."</accordion::Content></accordion::Item>
    </accordion::Root> }
}
#[component]
fn CollapsibleSample() -> impl IntoView {
    let open = RwSignal::new(false);
    let disabled = RwSignal::new(false);
    view! { <StateToggle label="Disabled" value=disabled /><collapsible::Root open=open on_open_change=move |next|open.set(next) disabled=disabled><collapsible::Trigger>"Show details"</collapsible::Trigger><collapsible::Content><p>"Details remain in the document flow."</p></collapsible::Content></collapsible::Root> }
}
#[component]
fn AvatarSample() -> impl IntoView {
    let (color, high, appearance) = palette();
    let (size, variant, controls) = visuals!(AvatarSize, AvatarVariant);
    view! { {appearance} {controls}<Avatar color=color high_contrast=high src="" alt="Alex Chen" fallback="AC" size=size variant=variant /> }
}
#[component]
fn BadgeSample() -> impl IntoView {
    let (color, high, appearance) = palette();
    let (size, variant, controls) = visuals!(BadgeSize, BadgeVariant);
    view! { {appearance} {controls}<Badge color=color high_contrast=high size=size variant=variant>"In progress"</Badge> }
}
#[component]
fn CardSample() -> impl IntoView {
    let (size, variant, controls) = visuals!(CardSize, CardVariant);
    view! { {controls}<Card size=size variant=variant><strong>"Project notes"</strong><p>"An authored surface for related content."</p></Card> }
}
#[component]
fn CalloutSample() -> impl IntoView {
    let (color, high, appearance) = palette();
    let (size, variant, controls) = visuals!(callout::CalloutSize, callout::CalloutVariant);
    view! { {appearance} {controls}<callout::Root color=color high_contrast=high size=size variant=variant><callout::Icon><Icon glyph=Glyph::Dot /></callout::Icon><callout::Text>"Your changes are saved in this preview."</callout::Text></callout::Root> }
}
#[component]
fn DataListSample() -> impl IntoView {
    let (color, high, appearance) = palette();
    let (size, control) = sizing!(data_list::DataListSize);
    view! { {appearance} {control}<data_list::Root size=size><data_list::Item><data_list::Label color=color high_contrast=high>"Status"</data_list::Label><data_list::Value>"Active"</data_list::Value></data_list::Item><data_list::Item><data_list::Label color=color high_contrast=high>"Owner"</data_list::Label><data_list::Value>"Alex Chen"</data_list::Value></data_list::Item></data_list::Root> }
}
#[component]
fn TableSample() -> impl IntoView {
    let (size, variant, controls) = visuals!(table::TableSize, table::TableVariant);
    let empty = RwSignal::new(false);
    view! { {controls}<StateToggle label="Empty" value=empty /><table::Root size=size variant=variant>
        <table::Caption>"Project members"</table::Caption><table::Header><table::Row><table::ColumnHeaderCell>"Name"</table::ColumnHeaderCell><table::ColumnHeaderCell>"Role"</table::ColumnHeaderCell></table::Row></table::Header>
        <table::Body>{move ||if empty.get(){view! { <table::Row><table::Cell colspan=2>"No members."<button type="button" on:click=move |_|empty.set(false)>"Restore members"</button></table::Cell></table::Row> }.into_any()}else{view! { <table::Row><table::RowHeaderCell>"Alex Chen"</table::RowHeaderCell><table::Cell>"Editor"</table::Cell></table::Row> }.into_any()}}</table::Body>
    </table::Root> }
}
#[component]
fn ScrollSample() -> impl IntoView {
    let (size, control) = sizing!(scroll_area::ScrollAreaSize);
    view! { {control}<scroll_area::Root size=size aria_label="Sample activity" class="gallery-scroll"><scroll_area::Viewport>{(1..=12).map(|n|view! { <p>{format!("Activity {n} · theme preview updated")}</p> }).collect_view()}</scroll_area::Viewport><scroll_area::Scrollbar><scroll_area::Thumb /></scroll_area::Scrollbar><scroll_area::Corner /></scroll_area::Root> }
}
#[component]
fn ProgressSample() -> impl IntoView {
    let (color, high, appearance) = palette();
    let (size, variant, controls) = visuals!(ProgressSize, ProgressVariant);
    let value = RwSignal::new(35.);
    let loading = RwSignal::new(false);
    view! { {appearance} {controls}<StateToggle label="Indeterminate" value=loading /><Progress class="w-full" color=color high_contrast=high value=value indeterminate=loading size=size variant=variant aria_label="Upload progress" /><button type="button" on:click=move |_|value.update(|n|*n=(*n+25.)%125.)>"Advance"</button><output>{move ||format!("{}%",value.get())}</output> }
}
#[component]
fn SkeletonSample() -> impl IntoView {
    let loading = RwSignal::new(true);
    view! { <StateToggle label="Loading" value=loading /><Skeleton loading=loading><p>"Content ready for review."</p></Skeleton> }
}
#[component]
fn SpinnerSample() -> impl IntoView {
    let (size, control) = sizing!(SpinnerSize);
    let loading = RwSignal::new(true);
    view! { {control}<StateToggle label="Loading" value=loading /><Spinner size=size loading=loading label="Loading preview"><span>"Preview ready"</span></Spinner> }
}
#[component]
fn ToastSample() -> impl IntoView {
    let open = RwSignal::new(false);
    let count = RwSignal::new(0);
    view! { <toast::Provider><Button on_click=move ||open.set(true)>"Show notification"</Button><toast::Viewport>
        <toast::Root open=open on_open_change=move |next|open.set(next) duration_ms=8000><toast::Title>"Draft saved"</toast::Title><toast::Description>"Your local preview is up to date."</toast::Description>
            <toast::Action alt_text="Undo saving the draft" on_action=move |_|{count.update(|n|*n+=1);open.set(false);}>"Undo"</toast::Action><toast::Close>"Dismiss"</toast::Close>
        </toast::Root>
    </toast::Viewport></toast::Provider><output>{move ||format!("{} undone",count.get())}</output> }
}
#[component]
fn LayoutSample() -> impl IntoView {
    let (container, container_control) = choice(
        "Container size",
        &[
            ContainerSize::One,
            ContainerSize::Two,
            ContainerSize::Three,
            ContainerSize::Four,
        ],
        ContainerSize::Four,
        |v| format!("{v:?}"),
    );
    let (section, section_control) = choice(
        "Section size",
        &[
            SectionSize::One,
            SectionSize::Two,
            SectionSize::Three,
            SectionSize::Four,
        ],
        SectionSize::Three,
        |v| format!("{v:?}"),
    );
    view! { {container_control}{section_control}<Section size=Signal::derive(move ||Responsive::from(section.get()))><Container size=Signal::derive(move ||Responsive::from(container.get()))><Flex class="gallery-row"><Box>"Box in Flex"</Box><Box>"Second box"</Box></Flex><Grid columns=Responsive::from(GridTracks::Count(2))><Box>"Grid one"</Box><Box>"Grid two"</Box></Grid><Inset><p>"Inset content"</p></Inset>{Reset(leptos::html::p().child("Reset native paragraph"))}</Container></Section> }
}

#[component]
fn SeparatorSample() -> impl IntoView {
    let (size, control) = sizing!(SeparatorSize);
    view! { {control}<p>"Before"</p><Separator size=size /><p>"After"</p> }
}
#[component]
fn TypographySample() -> impl IntoView {
    let (color, high, appearance) = palette();
    const SIZES: &[TextSize] = &[
        TextSize::One,
        TextSize::Two,
        TextSize::Three,
        TextSize::Four,
        TextSize::Five,
        TextSize::Six,
        TextSize::Seven,
        TextSize::Eight,
        TextSize::Nine,
    ];
    let (size, size_control) = choice("Text size", SIZES, TextSize::Three, |v| format!("{v:?}"));
    let (heading, heading_control) =
        choice("Heading size", SIZES, TextSize::Six, |v| format!("{v:?}"));
    let (code, code_control) = choice(
        "Code variant",
        &[
            CodeVariant::Solid,
            CodeVariant::Soft,
            CodeVariant::Outline,
            CodeVariant::Ghost,
        ],
        CodeVariant::default(),
        |v| format!("{v:?}"),
    );
    let (kbd, kbd_control) = choice(
        "Kbd variant",
        &[KbdVariant::Classic, KbdVariant::Soft],
        KbdVariant::default(),
        |v| format!("{v:?}"),
    );
    let size = Signal::derive(move || Responsive::from(size.get()));
    view! { {appearance} <div class="gallery-controls">{size_control}{heading_control}{code_control}{kbd_control}</div><Heading color=color high_contrast=high as_=HeadingLevel::H4 size=Signal::derive(move ||Responsive::from(heading.get()))>"A clear hierarchy"</Heading><Text color=color high_contrast=high size=size><Strong>"Strong"</Strong>" and "<Em>"emphasized"</Em>" text, "<Quote>"a quote"</Quote>" and "<Link color=color high_contrast=high href=gallery_href("Typography")>"a link"</Link>"."</Text><Blockquote color=color high_contrast=high size=size>"Good typography makes structure visible."</Blockquote><Code color=color high_contrast=high size=size variant=code>"let theme = resolve(document);"</Code><Kbd size=size variant=kbd>"⌘ K"</Kbd> }
}

#[component]
fn AccessibilitySample() -> impl IntoView {
    view! { <div class="gallery-row"><primitives::AccessibleIcon label="Information"><Icon glyph=Glyph::Dot /></primitives::AccessibleIcon><span>"Information icon with a name"</span><primitives::VisuallyHidden>"Additional screen-reader description"</primitives::VisuallyHidden></div> }
}
#[component]
fn LocalThemeSample(state: StudioState) -> impl IntoView {
    let low = RwSignal::new(false);
    let patch = Signal::derive(move || {
        rustify_ui::ThemeBoundaryPatch::from(rustify_ui::ThemeValuePatch(if low.get() {
            [
                ("card".into(), "#eeeeee".into()),
                ("card-foreground".into(), "#dddddd".into()),
            ]
            .into()
        } else {
            Default::default()
        }))
    });
    view! { <section class="gallery-local-theme"><h3>{move ||state.tr("Local theme sample","局部主题样本")}</h3><StateToggle label="Low contrast" value=low />
        <rustify_ui::ThemeBoundary patch=patch test_id="gallery-boundary">
            <crate::controller::RegisterPreviewSource state=state label=["Gallery local theme","陈列局部主题"] />
            <div class="gallery-local-card" data-theme-tokens="card,card-foreground" data-testid="gallery-local-card"><strong>{move ||state.tr("A local surface","局部表面")}</strong><p>{move ||state.tr("This sample controls its own boundary colors.","此样本控制自身边界的颜色。")}</p></div>
        </rustify_ui::ThemeBoundary>
    </section> }
}
