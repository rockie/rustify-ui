//! The twenty visible properties of an object, and the rules over them.
//!
//! Four of them - name, notes, locked and size - are applied as they are
//! typed, because the region draws them and a region showing a stale name
//! while the user types is the fusion bug this project exists to avoid. The
//! rest are edited as a draft and applied by a save, which is where the rules,
//! the checks and the single-flight live. The form knows about all twenty: the
//! live four tell it when they change, so a rule comparing a live property
//! with a drafted one still holds.

use leptos::prelude::*;
use rustify_components::{Field, Form, SelectOption, Switch, TextArea, TextField, TextKind};

/// In the order a person meets them. "The first error" means the first of
/// these, and that is where the keyboard goes.
pub const FIELDS: [&str; 20] = [
    "name",
    "notes",
    "locked",
    "size",
    "owner",
    "email",
    "reviewer",
    "category",
    "priority",
    "tags",
    "reference",
    "url",
    "start",
    "end",
    "weight",
    "location",
    "summary",
    "visible",
    "archived",
    "id",
];

/// The fifteen an object carries that a save applies.
#[derive(Clone, Debug, Default, PartialEq, Hash)]
pub struct Details {
    pub owner: String,
    pub email: String,
    pub reviewer: String,
    pub category: String,
    pub priority: String,
    pub tags: String,
    pub reference: String,
    pub url: String,
    pub start: String,
    pub end: String,
    pub weight: String,
    pub location: String,
    pub summary: String,
    pub visible: bool,
    pub archived: bool,
}

impl Details {
    pub fn seeded(n: u32) -> Self {
        Self {
            owner: format!("owner-{n:04}"),
            email: format!("owner-{n:04}@example.com"),
            reviewer: String::new(),
            category: "shape".to_string(),
            priority: "normal".to_string(),
            tags: "one,two".to_string(),
            reference: format!("REF-{n:04}"),
            url: "https://example.com".to_string(),
            start: "2026-01-01".to_string(),
            end: "2026-12-31".to_string(),
            weight: "100".to_string(),
            location: "shelf 1".to_string(),
            summary: format!("object {n}"),
            visible: true,
            archived: false,
        }
    }
}

pub const CATEGORIES: [&str; 3] = ["shape", "text", "group"];
pub const PRIORITIES: [&str; 3] = ["low", "normal", "high"];

/// Every synchronous rule, including the ones no single field can run.
///
/// Returned rather than applied so the caller decides when they run: the
/// moment of asking to submit, which is the only moment at which a rule
/// comparing two fields has both of them in a state worth judging.
pub fn rules(
    name: &str,
    locked: bool,
    saved_owner: &str,
    draft: &Details,
) -> Vec<(&'static str, String)> {
    let mut errors = Vec::new();
    if name.trim().is_empty() {
        errors.push(("name", "a name is required".to_string()));
    }
    if draft.owner.trim().is_empty() {
        errors.push(("owner", "an owner is required".to_string()));
    }
    if !draft.email.contains('@') || draft.email.starts_with('@') || draft.email.ends_with('@') {
        errors.push(("email", "an address needs a name and a host".to_string()));
    }
    if draft
        .tags
        .split(',')
        .filter(|tag| !tag.trim().is_empty())
        .count()
        > 5
    {
        errors.push(("tags", "at most five tags".to_string()));
    }
    if !draft.reference.starts_with("REF-") {
        errors.push(("reference", "a reference starts with REF-".to_string()));
    }
    if !draft.url.starts_with("https://") && !draft.url.starts_with("http://") {
        errors.push(("url", "a link starts with http:// or https://".to_string()));
    }
    for (field, value) in [("start", &draft.start), ("end", &draft.end)] {
        if !is_date(value) {
            errors.push((field, "a date is YYYY-MM-DD".to_string()));
        }
    }
    // Cross-field: a range that runs backwards is neither field's fault, and
    // the message says which pair it is about.
    if is_date(&draft.start) && is_date(&draft.end) && draft.end < draft.start {
        errors.push(("end", "the end is before the start".to_string()));
    }
    match draft.weight.trim().parse::<f64>() {
        Ok(weight) if (0.0..=1000.0).contains(&weight) => {}
        Ok(_) => errors.push(("weight", "a weight is between 0 and 1000".to_string())),
        Err(_) => errors.push(("weight", "a weight is a number".to_string())),
    }
    // Cross-field: what one field requires depends on another's value.
    if draft.priority == "high" && draft.reviewer.trim().is_empty() {
        errors.push(("reviewer", "a high priority needs a reviewer".to_string()));
    }
    // Cross-field across the live/drafted line: `locked` is applied as it is
    // typed, `owner` is drafted, and the rule spans both.
    if locked && draft.owner != saved_owner {
        errors.push(("owner", "a locked object keeps its owner".to_string()));
    }
    errors
}

fn is_date(value: &str) -> bool {
    let parts: Vec<&str> = value.split('-').collect();
    parts.len() == 3
        && parts[0].len() == 4
        && parts[1].len() == 2
        && parts[2].len() == 2
        && parts
            .iter()
            .all(|part| part.chars().all(|c| c.is_ascii_digit()))
}

/// The fifteen drafted properties.
#[component]
pub fn DetailsFields(
    form: Form,
    draft: RwSignal<Details>,
    disabled: Signal<bool>,
    /// Called after the draft has taken a new value, with the field's name.
    on_changed: Callback<&'static str>,
) -> impl IntoView {
    // One shape for the fourteen that are a line or a block of text.
    let text = move |field: &'static str,
                     label: &'static str,
                     kind: TextKind,
                     read: fn(&Details) -> String,
                     write: fn(&mut Details, String)| {
        view! {
            <Field
                form=form
                field=field
                label=label
                control=move |binding| {
                    view! {
                        <TextField
                            id=binding.id()
                            test_id=field
                            kind=kind
                            value=Signal::derive(move || draft.with(read))
                            disabled=disabled
                            invalid=binding.invalid()
                            described_by=binding.described_by()
                            on_change=move |value| {
                                draft.update(|draft| write(draft, value));
                                on_changed.run(field);
                            }
                        />
                    }
                        .into_any()
                }
            />
        }
    };

    view! {
        {text("owner", "owner", TextKind::Text, |d| d.owner.clone(), |d, v| d.owner = v)}
        {text("email", "email", TextKind::Email, |d| d.email.clone(), |d, v| d.email = v)}
        {text("reviewer", "reviewer", TextKind::Text, |d| d.reviewer.clone(), |d, v| d.reviewer = v)}
        <Field
            form=form
            field="category"
            label="category"
            control=move |binding| {
                view! {
                    <SelectField
                        binding_id=binding.id()
                        test_id="category"
                        options=CATEGORIES
                        value=Signal::derive(move || draft.with(|d| d.category.clone()))
                        disabled=disabled
                        invalid=binding.invalid()
                        described_by=binding.described_by()
                        on_change=Callback::new(move |value: String| {
                            draft.update(|d| d.category = value);
                            on_changed.run("category");
                        })
                    />
                }
                    .into_any()
            }
        />
        <Field
            form=form
            field="priority"
            label="priority"
            control=move |binding| {
                view! {
                    <SelectField
                        binding_id=binding.id()
                        test_id="priority"
                        options=PRIORITIES
                        value=Signal::derive(move || draft.with(|d| d.priority.clone()))
                        disabled=disabled
                        invalid=binding.invalid()
                        described_by=binding.described_by()
                        on_change=Callback::new(move |value: String| {
                            draft.update(|d| d.priority = value);
                            on_changed.run("priority");
                        })
                    />
                }
                    .into_any()
            }
        />
        {text("tags", "tags", TextKind::Text, |d| d.tags.clone(), |d, v| d.tags = v)}
        {text("reference", "reference", TextKind::Text, |d| d.reference.clone(), |d, v| d.reference = v)}
        {text("url", "link", TextKind::Url, |d| d.url.clone(), |d, v| d.url = v)}
        {text("start", "start", TextKind::Text, |d| d.start.clone(), |d, v| d.start = v)}
        {text("end", "end", TextKind::Text, |d| d.end.clone(), |d, v| d.end = v)}
        {text("weight", "weight", TextKind::Text, |d| d.weight.clone(), |d, v| d.weight = v)}
        {text("location", "location", TextKind::Text, |d| d.location.clone(), |d, v| d.location = v)}
        <Field
            form=form
            field="summary"
            label="summary"
            control=move |binding| {
                view! {
                    <TextArea
                        id=binding.id()
                        test_id="summary"
                        rows=2
                        value=Signal::derive(move || draft.with(|d| d.summary.clone()))
                        disabled=disabled
                        invalid=binding.invalid()
                        described_by=binding.described_by()
                        on_change=move |value| {
                            draft.update(|d| d.summary = value);
                            on_changed.run("summary");
                        }
                    />
                }
                    .into_any()
            }
        />
        <Field
            form=form
            field="visible"
            label="visible"
            control=move |binding| {
                view! {
                    <Switch
                        id=binding.id()
                        test_id="visible"
                        checked=Signal::derive(move || draft.with(|d| d.visible))
                        disabled=disabled
                        described_by=binding.described_by()
                        on_change=move |on| {
                            draft.update(|d| d.visible = on);
                            on_changed.run("visible");
                        }
                    />
                }
                    .into_any()
            }
        />
        <Field
            form=form
            field="archived"
            label="archived"
            control=move |binding| {
                view! {
                    <Switch
                        id=binding.id()
                        test_id="archived"
                        checked=Signal::derive(move || draft.with(|d| d.archived))
                        disabled=disabled
                        described_by=binding.described_by()
                        on_change=move |on| {
                            draft.update(|d| d.archived = on);
                            on_changed.run("archived");
                        }
                    />
                }
                    .into_any()
            }
        />
    }
}

/// A chooser over a fixed list of words, kept open by the application.
#[component]
fn SelectField(
    #[prop(into)] binding_id: String,
    #[prop(into)] test_id: String,
    options: [&'static str; 3],
    #[prop(into)] value: Signal<String>,
    #[prop(into)] disabled: Signal<bool>,
    #[prop(into)] invalid: Signal<bool>,
    #[prop(into)] described_by: Signal<String>,
    on_change: Callback<String>,
) -> impl IntoView {
    let open = RwSignal::new(false);
    // A list a check left open is state like any other, and a reset closes it.
    if let Some(resets) = crate::reset::Resets::used() {
        resets.on_reset(move || open.set(false));
    }
    view! {
        <rustify_components::Select
            id=binding_id
            test_id=test_id
            value=value
            options=Signal::derive(move || {
                options.iter().map(|name| SelectOption::new(*name, *name)).collect()
            })
            open=open
            on_open_change=move |next| open.set(next)
            disabled=disabled
            invalid=invalid
            described_by=described_by
            on_change=move |next: String| on_change.run(next)
        />
    }
}
