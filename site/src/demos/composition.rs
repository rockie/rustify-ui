use leptos::prelude::*;
use rustify_components::form::field as f;
use rustify_components::form::{FieldBinding, Form, FormRoot, FormStatus, SubmitButton};
use rustify_components::{id, Button, ButtonVariant, Label, Select, SelectOption, TextField};

/// A real form: application-owned values, changed-field tracking, submit rules
/// and a reset that restores the last accepted state.
pub fn form_demo() -> impl IntoView {
    let name = RwSignal::new("Ada Lovelace".to_string());
    let email = RwSignal::new("ada@example.com".to_string());
    let submitted = RwSignal::new(None::<(String, String)>);
    let form = Form::new(&["name", "email"], move || {
        submitted.set(Some((name.get_untracked(), email.get_untracked())));
    });
    let rules = move || {
        let mut failures: Vec<(&'static str, String)> = Vec::new();
        if name.get_untracked().trim().is_empty() {
            failures.push(("name", "A display name is required.".to_string()));
        }
        if !email.get_untracked().contains('@') {
            failures.push(("email", "Enter an address with an @ sign.".to_string()));
        }
        failures
    };
    let name_for_change = name;
    let email_for_change = email;
    let form_for_reset = form;
    view! {
        <div class="space-y-4">
            <FormRoot form=form rules=rules aria_label="Contact details">
                <f::Root form=form field="name">
                    <f::Label>"Display name"</f::Label>
                    <TextField value=name on_change=move |value| { name_for_change.set(value.clone()); form.changed("name"); } />
                    <f::Description>"Shown on your profile page."</f::Description>
                    <f::Message />
                </f::Root>
                <f::Root form=form field="email">
                    <f::Label>"Email"</f::Label>
                    <TextField value=email on_change=move |value| { email_for_change.set(value.clone()); form.changed("email"); } />
                    <f::Description>"Only used for this example."</f::Description>
                    <f::Message />
                </f::Root>
                <div class="flex flex-wrap items-center gap-3">
                    <SubmitButton form=form rules=move || {
                        let mut rules: Vec<(&'static str, String)> = Vec::new();
                        if name.get_untracked().trim().is_empty() {
                            rules.push(("name", "A display name is required.".to_string()));
                        }
                        if !email.get_untracked().contains('@') {
                            rules.push(("email", "Enter an address with an @ sign.".to_string()));
                        }
                        rules
                    }>"Save contact"</SubmitButton>
                    <Button variant=ButtonVariant::Outline on_click=move || {
                        let _ = form_for_reset.reset();
                        name.set("Ada Lovelace".to_string());
                        email.set("ada@example.com".to_string());
                        submitted.set(None);
                    }>"Reset"</Button>
                </div>
                <FormStatus form=form saving="Saving…" />
            </FormRoot>
            <p class="text-sm break-words">"Submitted: "
                <output data-demo-state="">{move || submitted.get().map_or_else(|| "Not submitted".to_string(), |(name, email)| format!("{name} <{email}>"))}</output>
            </p>
        </div>
    }
}

/// Field parts carry label, description and message wiring for one control.
pub fn field_parts_demo() -> impl IntoView {
    let project = RwSignal::new("rustify-ui".to_string());
    let form = Form::new(&["project"], || ());
    let control = move |binding: FieldBinding| {
        view! {
            <TextField id=binding.id() value=project
                invalid=binding.invalid() described_by=binding.described_by()
                on_change=move |value| project.set(value) />
        }
        .into_any()
    };
    view! {
        <div class="space-y-4">
            <f::Field form=form field="project" label="Project name" description="Two or more characters." control=control />
            <p class="text-sm">"Accepted project: "
                <output data-demo-state="">{move || project.get()}</output>
            </p>
        </div>
    }
}

/// One select's state: a long list, a disabled entry, and the open state the
/// application controls.
pub fn select_parts_demo() -> impl IntoView {
    let field_id = id::next("select-parts-field");
    let release = RwSignal::new("0.3.0".to_string());
    let open = RwSignal::new(false);
    let options = (1..=12)
        .map(|minor| {
            let value = format!("0.{minor}.0");
            let option = SelectOption::new(value.clone(), format!("Release {value}"));
            if minor == 11 {
                option.disabled()
            } else {
                option
            }
        })
        .collect::<Vec<_>>();
    view! {
        <div class="space-y-4">
            <div class="max-w-sm space-y-2">
                <Label control=field_id.clone()>"Release"</Label>
                <Select id=field_id value=release on_change=move |value| release.set(value)
                    open=open on_open_change=move |value| open.set(value) aria_label="Release"
                    options=options />
            </div>
            <p class="text-sm">"Selected: "<output data-demo-state="">{move || release.get()}</output>
                " · List: 12 entries · Disabled: 0.11.0 · Open: "{move || if open.get() { "yes" } else { "no" }}</p>
            <p class="text-sm text-muted-foreground">"Closing the list returns focus to the trigger; the disabled entry cannot be chosen by pointer or keyboard."</p>
        </div>
    }
}

/// Two related selects where the second follows the first: grouped choices as
/// one coherent decision.
pub fn select_grouped_demo() -> impl IntoView {
    let region = RwSignal::new("eu".to_string());
    let city = RwSignal::new("berlin".to_string());
    let region_open = RwSignal::new(false);
    let city_open = RwSignal::new(false);
    let cities = move || match region.get().as_str() {
        "eu" => vec![
            SelectOption::new("berlin", "Berlin"),
            SelectOption::new("lisbon", "Lisbon"),
        ],
        "apac" => vec![
            SelectOption::new("osaka", "Osaka"),
            SelectOption::new("taipei", "Taipei"),
        ],
        _ => vec![
            SelectOption::new("sf", "San Francisco"),
            SelectOption::new("nyc", "New York"),
        ],
    };
    let region_for_change = region;
    view! {
        <div class="space-y-4">
            <div class="max-w-sm grid grid-cols-2 gap-3">
                <div class="space-y-2">
                    <Label control="select-grouped-region">"Region"</Label>
                    <Select id="select-grouped-region" value=region on_change=move |value| {
                        region_for_change.set(value.clone());
                        city.set(match value.as_str() {
                            "eu" => "berlin".to_string(),
                            "apac" => "osaka".to_string(),
                            _ => "sf".to_string(),
                        });
                    } open=region_open on_open_change=move |value| region_open.set(value)
                        aria_label="Region"
                        options=vec![
                            SelectOption::new("eu", "Europe"),
                            SelectOption::new("apac", "Asia-Pacific"),
                            SelectOption::new("amer", "Americas"),
                        ] />
                </div>
                <div class="space-y-2">
                    <Label control="select-grouped-city">"City"</Label>
                    <Select id="select-grouped-city" value=city on_change=move |value| city.set(value)
                        open=city_open on_open_change=move |value| city_open.set(value)
                        aria_label="City" options=cities() />
                </div>
            </div>
            <p class="text-sm">"Location: "
                <output data-demo-state="">{move || format!("{} · {}", region.get(), city.get())}</output>
            </p>
        </div>
    }
}
