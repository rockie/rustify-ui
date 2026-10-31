//! A label, a control and the error that belongs to it, wired together.
//!
//! The wiring is the point. A label that names its control, an
//! `aria-invalid` that matches the error on screen, and an
//! `aria-describedby` that points at text which is actually there - three
//! things every form needs and every form gets slightly wrong. The control is
//! the caller's; what it is handed is the three answers.

use leptos::prelude::*;

use super::provider::Form;

#[derive(Clone, Copy)]
pub(crate) struct FieldContext {
    pub form: Form,
    pub field: &'static str,
    controls: StoredValue<Vec<(StoredValue<()>, String)>>,
}

pub(crate) fn control_id(explicit: String, part: &str) -> String {
    if let Some(context) = use_context::<FieldContext>() {
        let first = context.controls.with_value(Vec::is_empty);
        let id = if explicit.is_empty() {
            if first {
                context.form.default_control_id(context.field)
            } else {
                crate::id::next(part)
            }
        } else {
            explicit
        };
        let key = StoredValue::new(());
        context
            .controls
            .update_value(|controls| controls.push((key, id.clone())));
        if let Some(binding) = use_context::<FieldBinding>() {
            if first {
                binding.id.set(id.clone());
            }
            // Server serialization drops each control view before the enclosing
            // field finishes. Its accepted ID must survive that drop for labels.
            #[cfg(target_arch = "wasm32")]
            on_cleanup(move || {
                if let Some(next) = context.controls.try_update_value(|controls| {
                    controls.retain(|(control, _)| *control != key);
                    controls.first().map(|(_, id)| id.clone())
                }) {
                    binding.id.try_set(
                        next.unwrap_or_else(|| context.form.default_control_id(context.field)),
                    );
                }
            });
        }
        return id;
    }
    if explicit.is_empty() {
        crate::id::next(part)
    } else {
        explicit
    }
}

/// Projects the nearest field's real description/error references onto a control.
pub(crate) fn metadata(
    invalid: Signal<bool>,
    described_by: Signal<String>,
) -> (Signal<bool>, Signal<String>) {
    let Some(binding) = use_context::<FieldBinding>() else {
        return (invalid, described_by);
    };
    (
        Signal::derive(move || invalid.get() || binding.invalid().get()),
        Signal::derive(move || {
            crate::primitives::choice::merge_ids(&[
                &described_by.get(),
                &binding.described_by().get(),
            ])
        }),
    )
}

/// Native association metadata shared by all field parts.
#[derive(Clone, Copy)]
pub struct FieldBinding {
    id: RwSignal<String>,
    #[cfg(all(feature = "ssr", not(target_arch = "wasm32")))]
    label_marker: StoredValue<String>,
    description_id: StoredValue<String>,
    error_id: StoredValue<String>,
    error: Memo<Option<(String, rustify_ui::FieldErrorSource)>>,
    descriptions: RwSignal<u32>,
    messages: RwSignal<u32>,
    described_by: Signal<String>,
    invalid: Signal<bool>,
}
impl FieldBinding {
    pub fn id(&self) -> String {
        self.id.get()
    }
    pub(crate) fn label_control(&self) -> String {
        #[cfg(all(feature = "ssr", not(target_arch = "wasm32")))]
        {
            self.label_marker.get_value()
        }
        #[cfg(not(all(feature = "ssr", not(target_arch = "wasm32"))))]
        {
            self.id()
        }
    }
    /// IDs of mounted hints and the currently visible error, without duplicates.
    pub fn described_by(&self) -> Signal<String> {
        self.described_by
    }
    pub fn description_id(&self) -> String {
        self.description_id.get_value()
    }
    pub fn error_id(&self) -> String {
        self.error_id.get_value()
    }
    pub fn error_source(&self) -> Signal<Option<rustify_ui::FieldErrorSource>> {
        let error = self.error;
        Signal::derive(move || error.get().map(|(_, source)| source))
    }
    pub fn invalid(&self) -> Signal<bool> {
        self.invalid
    }
}
fn boundary(
    form: Form,
    field: &'static str,
    content: impl FnOnce(FieldBinding) -> AnyView,
) -> impl IntoView {
    let owner = Owner::new();
    let view = owner.with(move || {
        let id = RwSignal::new(form.default_control_id(field));
        form.bind_control_id(field, id);
        let description_id = StoredValue::new(form.description_id(field));
        let error_id = StoredValue::new(form.error_id(field));
        let error = Memo::new(move |_| form.error(field).zip(form.error_source(field)));
        let descriptions = RwSignal::new(0);
        let messages = RwSignal::new(0);
        let binding = FieldBinding {
            id,
            #[cfg(all(feature = "ssr", not(target_arch = "wasm32")))]
            label_marker: StoredValue::new(format!("\0{}-label\0", form.default_control_id(field))),
            description_id,
            error_id,
            error,
            descriptions,
            messages,
            described_by: Signal::derive(move || {
                let hint = if descriptions.get() > 0 {
                    description_id.get_value()
                } else {
                    String::new()
                };
                let message = if messages.get() > 0 && error.get().is_some() {
                    error_id.get_value()
                } else {
                    String::new()
                };
                crate::primitives::choice::merge_ids(&[&hint, &message])
            }),
            invalid: Signal::derive(move || error.get().is_some()),
        };
        provide_context(binding);
        provide_context(FieldContext {
            form,
            field,
            controls: StoredValue::new(Vec::new()),
        });
        let view = content(binding);
        #[cfg(all(feature = "ssr", not(target_arch = "wasm32")))]
        let view = super::ssr_field::FieldHtml { view, binding }.into_any();
        view
    });
    leptos::tachys::reactive_graph::OwnedView::new_with_owner(view, owner)
}
/// A metadata boundary with caller-composed Label, Description, Message and controls.
#[component]
pub fn Root(
    form: Form,
    field: &'static str,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView {
    boundary(form, field, move |binding| {
        view! {
            <div class=crate::macros::reactive_merge("", &class) data-rustify-part="field-root"
                data-invalid=move || binding.invalid.get().then_some("")>{children()}</div>
        }
        .into_any()
    })
}
#[component]
pub fn Label(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    view! { <crate::primitives::field::Label class=class test_id=test_id>{children()}</crate::primitives::field::Label> }
}
#[component]
pub fn Description(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    let Some(binding) = use_context::<FieldBinding>() else {
        return view! { <p class=crate::macros::reactive_merge("", &class) data-rustify-part="field-description" data-rustify-issue="missing-root">{children()}</p> }.into_any();
    };
    binding.descriptions.update(|count| *count += 1);
    #[cfg(target_arch = "wasm32")]
    on_cleanup(move || {
        binding.descriptions.try_update(|count| *count -= 1);
    });
    view! { <p id=binding.description_id() class=crate::macros::reactive_merge("", &class) data-rustify-part="field-description">{children()}</p> }.into_any()
}
#[component]
pub fn Message(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
) -> impl IntoView {
    let Some(binding) = use_context::<FieldBinding>() else {
        return view! { <p class=crate::macros::reactive_merge("", &class) data-rustify-part="field-message" data-rustify-issue="missing-root" data-testid=test_id /> }.into_any();
    };
    binding.messages.update(|count| *count += 1);
    #[cfg(target_arch = "wasm32")]
    on_cleanup(move || {
        binding.messages.try_update(|count| *count -= 1);
    });
    view! { <Show when=move || binding.error.get().is_some()>
        <p id=binding.error_id() class=crate::macros::reactive_merge("", &class) data-name="FieldError"
            data-rustify-part="field-message" data-testid=test_id.clone()
            data-source=move || binding.error.get().map(|(_, source)| source.as_str())>
            {move || binding.error.get().map(|(message, _)| message).unwrap_or_default()}
        </p>
    </Show> }.into_any()
}
/// A themed label, description, control and error using the same field boundary.
#[component]
pub fn Field(
    form: Form,
    field: &'static str,
    #[prop(into)] label: String,
    #[prop(optional, into)] description: String,
    control: impl Fn(FieldBinding) -> AnyView + Send + Sync + 'static,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    boundary(form, field, move |binding| {
        let has_description = !description.is_empty();
        view! {
            <div class=crate::macros::reactive_merge("flex flex-col gap-1.5", &class)
                data-name="Field" data-rustify-part="field-root" data-testid=format!("field-{field}")
                data-invalid=move || binding.invalid.get().then_some("")>
                <crate::label::Label test_id=format!("label-{field}")>{label}</crate::label::Label>
                {has_description.then(|| view! { <Description class="text-sm text-muted-foreground">{description}</Description> })}
                {control(binding)}
                <Message class="text-sm text-destructive" test_id=format!("error-{field}") />
            </div>
        }.into_any()
    })
}
