//! Native field editing shared by unstyled parts and themed wrappers.

#[cfg(any(target_arch = "wasm32", test))]
#[derive(Clone, Debug, Default)]
struct Completion {
    ended: Option<(String, String)>,
}
#[cfg(any(target_arch = "wasm32", test))]
impl Completion {
    fn start(&mut self) {
        self.ended = None;
    }
    fn end(&mut self, chosen: String) {
        self.ended = Some((chosen.clone(), chosen));
    }
    fn settled(&mut self, value: String) {
        if let Some((_, settled)) = &mut self.ended {
            *settled = value;
        }
    }
    fn input(&mut self, text: &str) -> bool {
        self.ended
            .take()
            .is_none_or(|(chosen, settled)| text != chosen && text != settled)
    }
}

use leptos::prelude::*;

/// Native input kinds; file and color picking use dedicated capabilities.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TextKind {
    #[default]
    Text,
    Email,
    Password,
    Number,
    Search,
    Tel,
    Url,
    Date,
    DateTimeLocal,
    Month,
    Time,
    Week,
    Hidden,
}
impl TextKind {
    pub(crate) const fn attribute(self) -> &'static str {
        match self {
            Self::Text => "text",
            Self::Email => "email",
            Self::Password => "password",
            Self::Number => "number",
            Self::Search => "search",
            Self::Tel => "tel",
            Self::Url => "url",
            Self::Date => "date",
            Self::DateTimeLocal => "datetime-local",
            Self::Month => "month",
            Self::Time => "time",
            Self::Week => "week",
            Self::Hidden => "hidden",
        }
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SlotSide {
    #[default]
    Start,
    End,
}
impl SlotSide {
    const fn attribute(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::End => "end",
        }
    }
}
#[derive(Clone)]
struct Options {
    id: String,
    name: String,
    form: String,
    aria_label: String,
    labelled_by: String,
    described_by: Signal<String>,
    placeholder: String,
    autocomplete: String,
    inputmode: String,
    pattern: Option<String>,
    minlength: Option<u32>,
    maxlength: Option<u32>,
    required: Signal<bool>,
    disabled: Signal<bool>,
    read_only: Signal<bool>,
    invalid: Signal<bool>,
    class: Signal<String>,
    input_class: String,
    test_id: String,
    kind: TextKind,
    rows: Option<u32>,
    min: String,
    max: String,
    step: String,
    spellcheck: Option<bool>,
}
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
#[derive(Clone, Copy)]
struct Behavior {
    part: &'static str,
    accepted: Signal<String>,
    shown: Signal<String>,
    composing: Signal<bool>,
    invalid: Signal<bool>,
    input: Callback<web_sys::Event>,
    focus: Option<Callback<web_sys::FocusEvent>>,
    blur: Option<Callback<()>>,
    key: Option<Callback<web_sys::KeyboardEvent>>,
    composition: Callback<bool>,
    commit: Option<Callback<()>>,
    reset: Option<Callback<()>>,
    role: Option<&'static str>,
    value_now: Option<Signal<String>>,
    value_min: Option<f64>,
    value_max: Option<f64>,
    mirror: bool,
    parse_message: &'static str,
}
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
#[derive(Clone, Copy)]
enum Editor {
    Input(NodeRef<leptos::html::Input>),
    Area(NodeRef<leptos::html::Textarea>),
}
#[cfg(target_arch = "wasm32")]
impl Editor {
    fn value(self) -> Option<String> {
        match self {
            Self::Input(node) => node.try_get_untracked().flatten().map(|node| node.value()),
            Self::Area(node) => node.try_get_untracked().flatten().map(|node| node.value()),
        }
    }
    fn set(self, value: &str) {
        match self {
            Self::Input(node) => {
                if let Some(node) = node.try_get_untracked().flatten() {
                    if node.value() != value {
                        node.set_value(value);
                    }
                }
            }
            Self::Area(node) => {
                if let Some(node) = node.try_get_untracked().flatten() {
                    if node.value() != value {
                        node.set_value(value);
                    }
                }
            }
        }
    }
    fn element(self) -> Option<web_sys::HtmlElement> {
        use leptos::wasm_bindgen::JsCast;
        match self {
            Self::Input(node) => node
                .try_get_untracked()
                .flatten()
                .map(|node| node.unchecked_into()),
            Self::Area(node) => node
                .try_get_untracked()
                .flatten()
                .map(|node| node.unchecked_into()),
        }
    }
    fn validity(self, message: &str) {
        match self {
            Self::Input(node) => {
                if let Some(node) = node.try_get_untracked().flatten() {
                    node.set_custom_validity(message);
                }
            }
            Self::Area(node) => {
                if let Some(node) = node.try_get_untracked().flatten() {
                    node.set_custom_validity(message);
                }
            }
        }
    }
    fn validation_message(self) -> String {
        match self {
            Self::Input(node) => node
                .try_get_untracked()
                .flatten()
                .and_then(|node| node.validation_message().ok()),
            Self::Area(node) => node
                .try_get_untracked()
                .flatten()
                .and_then(|node| node.validation_message().ok()),
        }
        .unwrap_or_default()
    }
}
#[derive(Clone, Copy)]
struct Context {
    key: StoredValue<String>,
    options: StoredValue<Options>,
    behavior: Behavior,
    area: bool,
    editors: RwSignal<Vec<(StoredValue<()>, Editor)>>,
    root: NodeRef<leptos::html::Div>,
    mirror: NodeRef<leptos::html::Input>,
    duplicate: Signal<bool>,
    #[cfg(target_arch = "wasm32")]
    completion: StoredValue<Completion>,
    #[cfg(target_arch = "wasm32")]
    composing_key: StoredValue<Option<rustify_ui::Listener>, leptos::reactive::owner::LocalStorage>,
    #[cfg(target_arch = "wasm32")]
    owned_message: StoredValue<Option<String>>,
}
impl Context {
    fn disabled(self) -> bool {
        self.duplicate.get() || self.options.with_value(|options| options.disabled.get())
    }
    fn readonly(self) -> bool {
        self.options.with_value(|options| options.read_only.get())
    }
    fn invalid(self) -> bool {
        self.behavior.invalid.get() || self.options.with_value(|options| options.invalid.get())
    }
    fn claim(self, editor: Editor) {
        let key = StoredValue::new(());
        self.editors.update(|editors| editors.push((key, editor)));
        on_cleanup(move || {
            self.editors
                .try_update(|editors| editors.retain(|(id, _)| *id != key));
        });
    }
}
#[cfg(target_arch = "wasm32")]
impl Context {
    fn editor(self) -> Option<Editor> {
        self.editors
            .with_untracked(|editors| editors.first().map(|(_, editor)| *editor))
    }
    fn owns(self, event: &web_sys::Event) -> bool {
        use leptos::wasm_bindgen::JsCast;
        let Some(key) = self.key.try_get_value() else {
            return false;
        };
        event
            .target()
            .and_then(|target| target.dyn_into::<web_sys::Element>().ok())
            .is_some_and(|target| {
                target.get_attribute("data-rustify-field-owner").as_deref() == Some(&key)
            })
    }
    fn settle(self) {
        let Some(editor) = self.editor() else {
            return;
        };
        editor.set(&self.behavior.shown.get_untracked());
        let message = self
            .behavior
            .invalid
            .get_untracked()
            .then_some(self.behavior.parse_message);
        if let Some(message) = message {
            editor.validity(message);
            self.owned_message.set_value(Some(message.into()));
        } else if let Some(previous) = self.owned_message.get_value() {
            if editor.validation_message() == previous {
                editor.validity("");
            }
            self.owned_message.set_value(None);
        }
    }
    fn flush(self) {
        self.settle();
        if let Some(mirror) = self.mirror.try_get_untracked().flatten() {
            let value = self.behavior.accepted.get_untracked();
            if mirror.value() != value {
                mirror.set_value(&value);
            }
            mirror.set_disabled(untrack(move || self.disabled()));
        }
    }
    fn input(self, event: web_sys::Event) {
        if !self.owns(&event) || self.behavior.composing.get_untracked() {
            return;
        }
        let text = self.editor().and_then(Editor::value).unwrap_or_default();
        let mut deliver = true;
        self.completion
            .update_value(|completion| deliver = completion.input(&text));
        // A different input after composition must remain a real request.
        if deliver
            && !event.default_prevented()
            && !untrack(move || self.disabled() || self.readonly())
        {
            self.behavior.input.run(event);
        }
        self.settle();
    }
    fn composition_start(self, event: web_sys::CompositionEvent) {
        if !self.owns(&event) || event.default_prevented() {
            return;
        }
        self.completion.update_value(Completion::start);
        self.behavior.composition.run(true);
        // Scope layer capture runs before the field. Reserve composing keys
        // at the document boundary, using the instance's abort-aware listener.
        self.composing_key.set_value(Some(rustify_ui::listen(
            document().as_ref(),
            "keydown",
            rustify_ui::ListenOptions {
                capture: true,
                passive: false,
            },
            move |event| {
                use leptos::wasm_bindgen::JsCast;
                if self.owns(&event) && self.behavior.composing.get_untracked() {
                    if let Some(event) = event.dyn_ref::<web_sys::KeyboardEvent>() {
                        if matches!(event.key().as_str(), "Enter" | "Escape") {
                            event.prevent_default();
                            event.stop_propagation();
                        }
                    }
                }
            },
        )));
    }
    fn composition_end(self, event: web_sys::CompositionEvent) {
        if !self.owns(&event) {
            return;
        }
        let chosen = self.editor().and_then(Editor::value).unwrap_or_default();
        self.behavior.composition.run(false);
        self.composing_key.update_value(|held| {
            held.take();
        });
        self.completion
            .update_value(|completion| completion.end(chosen));
        if !event.default_prevented() && !untrack(move || self.disabled() || self.readonly()) {
            self.behavior.input.run(event.into());
        }
        if self
            .editor()
            .and_then(Editor::element)
            .is_some_and(|element| document().active_element().as_ref() != Some(element.as_ref()))
        {
            if let Some(blur) = self.behavior.blur {
                blur.run(());
            }
        }
        self.settle();
        self.completion.update_value(|completion| {
            completion.settled(self.editor().and_then(Editor::value).unwrap_or_default())
        });
    }
    fn key(self, event: web_sys::KeyboardEvent) {
        if !self.owns(&event) || event.default_prevented() {
            return;
        }
        if self.behavior.composing.get_untracked() || event.is_composing() {
            if event.key() == "Enter" {
                event.prevent_default();
            }
            return;
        }
        if !untrack(move || self.disabled() || self.readonly()) {
            if let Some(key) = self.behavior.key {
                key.run(event);
                self.settle();
            }
        }
    }
    fn blur(self, event: web_sys::FocusEvent) {
        if !self.owns(&event) || self.behavior.composing.get_untracked() {
            return;
        }
        if let Some(blur) = self.behavior.blur {
            blur.run(());
        }
        self.settle();
    }
}
fn optional(text: String) -> Option<String> {
    (!text.is_empty()).then_some(text)
}
fn optional_attribute<A: leptos::attr::Attribute + 'static>(
    attribute: Option<A>,
) -> Vec<leptos::attr::any_attribute::AnyAttribute> {
    use leptos::attr::any_attribute::IntoAnyAttribute;
    attribute
        .into_iter()
        .map(IntoAnyAttribute::into_any_attr)
        .collect()
}
fn class_attribute(class: String) -> Vec<leptos::attr::any_attribute::AnyAttribute> {
    use leptos::attr::any_attribute::IntoAnyAttribute;
    (!class.is_empty())
        .then(|| {
            leptos::tachys::html::class::class(crate::macros::reactive_merge("", &class))
                .into_any_attr()
        })
        .into_iter()
        .collect()
}
fn boundary(
    mut options: Options,
    area: bool,
    make: impl FnOnce() -> Behavior,
    children: Children,
) -> impl IntoView {
    let owner = Owner::new();
    let view = owner.with(move || {
        options.id = crate::form::field::control_id(options.id, "field");
        options.form = super::form::associated_form(options.form);
        if let Some(binding) = use_context::<crate::form::FieldBinding>() {
            let described = options.described_by;
            options.described_by = Signal::derive(move || super::choice::merge_ids(&[&described.get(), &binding.described_by().get()]));
            let invalid = options.invalid;
            options.invalid = Signal::derive(move || invalid.get() || binding.invalid().get());
        }
        let behavior = make();
        let editors = RwSignal::new(Vec::new());
        let context = Context {
            key: StoredValue::new(crate::id::next("field-owner")), options: StoredValue::new(options.clone()), behavior,
            area, editors, root: NodeRef::new(), mirror: NodeRef::new(),
            duplicate: Signal::derive(move || editors.with(|editors| editors.len() > 1)),
            #[cfg(target_arch = "wasm32")]
            completion: StoredValue::new(Completion::default()),
            #[cfg(target_arch = "wasm32")]
            composing_key: StoredValue::new_local(None),
            #[cfg(target_arch = "wasm32")]
            owned_message: StoredValue::new(None),
        };
        provide_context(context);
        if let Some(form) = crate::form::use_form().or_else(|| use_context::<crate::form::field::FieldContext>().map(|field| field.form)) {
            form.on_reset(Callback::new(move |_| {
                if let Some(reset) = behavior.reset { reset.run(()); }
                #[cfg(target_arch = "wasm32")]
                {
                    context.completion.update_value(Completion::start);
                    context.composing_key.update_value(|held| { held.take(); });
                    context.flush();
                }
            }));
        }
        super::form::register(context.key, &options.form, super::form::ControlSpec {
            composing: behavior.composing, commit: behavior.commit,
            flush: Callback::new(move |_| { #[cfg(target_arch = "wasm32")] context.flush(); }),
            focus: Callback::new(move |_| { #[cfg(target_arch = "wasm32")] if let Some(element) = context.editor().and_then(Editor::element) { let _ = element.focus(); } }),
            owns: Callback::new(move |_element: web_sys::Element| {
                #[cfg(target_arch = "wasm32")]
                { context.root.try_get_untracked().flatten().is_some_and(|root| root.contains(Some(&_element))) }
                #[cfg(not(target_arch = "wasm32"))]
                { false }
            }),
        });
        #[cfg(target_arch = "wasm32")]
        {
            Effect::new(move |_| { context.editors.track(); behavior.shown.track(); behavior.invalid.track(); if !behavior.composing.get() { context.settle(); } });
            on_cleanup(move || { context.composing_key.try_update_value(|held| { held.take(); }); });
        }
        let classes = crate::macros::reactive_merge_with(move || vec![options.class.get()]);
        let hidden = options.kind == TextKind::Hidden;
        let part = format!("{}-root", behavior.part);
        let direction = rustify_ui::use_direction();
        view! {
            <div node_ref=context.root class=classes data-rustify-part=part data-kind=if area { "textarea" } else { options.kind.attribute() }
                data-disabled=move || context.disabled().then_some("") data-readonly=move || context.readonly().then_some("")
                data-invalid=move || context.invalid().then_some("") data-rustify-issue=move || context.duplicate.get().then_some("multiple-inputs")
                hidden=hidden dir=move || direction.get().attribute()
                on:input=move |_event| { #[cfg(target_arch = "wasm32")] context.input(_event); }
                on:compositionstart=move |_event| { #[cfg(target_arch = "wasm32")] context.composition_start(_event); }
                on:compositionend=move |_event| { #[cfg(target_arch = "wasm32")] context.composition_end(_event); }
                on:focusin=move |_event| { #[cfg(target_arch = "wasm32")] if context.owns(&_event) { if let Some(focus) = behavior.focus { focus.run(_event); } } }
                on:focusout=move |_event| { #[cfg(target_arch = "wasm32")] context.blur(_event); }
                on:keydown=move |_event| { #[cfg(target_arch = "wasm32")] context.key(_event); }>
                {children()}
                {if behavior.mirror && !options.name.is_empty() { Some(view! {
                    <input node_ref=context.mirror type="hidden" name=options.name.clone() form=optional(options.form.clone())
                        value=move || behavior.accepted.get() prop:value=move || behavior.accepted.get()
                        disabled=move || context.disabled() data-rustify-form-control=context.key.get_value() />
                }) } else { None }}
            </div>
        }
    });
    leptos::tachys::reactive_graph::OwnedView::new_with_owner(view, owner)
}
fn text_behavior(value: Signal<String>, change: Callback<String>, part: &'static str) -> Behavior {
    let composing = RwSignal::new(false);
    Behavior {
        part,
        accepted: value,
        shown: value,
        composing: composing.into(),
        invalid: Signal::stored(false),
        input: Callback::new(move |event| change.run(event_target_value(&event))),
        focus: None,
        blur: None,
        key: None,
        composition: Callback::new(move |active| composing.set(active)),
        commit: None,
        reset: None,
        role: None,
        value_now: None,
        value_min: None,
        value_max: None,
        mirror: false,
        parse_message: "",
    }
}
fn input_as<At, Ch>(
    target: leptos::html::HtmlElement<leptos::html::Input, At, Ch>,
    class: String,
    test_id: String,
) -> impl IntoView
where
    At: leptos::attr::Attribute + Send,
    Ch: RenderHtml + Send,
{
    use leptos::attr::custom::custom_attribute as data;
    let Some(context) = use_context::<Context>() else {
        return super::Slot(
            target,
            (
                leptos::attr::disabled(true),
                data("data-rustify-issue", "missing-root"),
            ),
        )
        .into_any();
    };
    if context.area {
        return super::Slot(
            target,
            (
                leptos::attr::disabled(true),
                data("data-rustify-issue", "wrong-editor"),
            ),
        )
        .into_any();
    }
    let node = NodeRef::new();
    context.claim(Editor::Input(node));
    let options = context.options.get_value();
    let behavior = context.behavior;
    let direction = rustify_ui::use_direction();
    let test_id = if test_id.is_empty() {
        options.test_id.clone()
    } else {
        test_id
    };
    let class = format!("{} {class}", options.input_class)
        .trim()
        .to_string();
    super::Slot(
        target,
        (
            (
                leptos::tachys::html::node_ref::node_ref(node),
                leptos::attr::id(options.id),
                leptos::attr::r#type(options.kind.attribute()),
                class_attribute(class),
                leptos::attr::value(behavior.shown.get_untracked()),
                leptos::attr::name(if behavior.mirror {
                    None
                } else {
                    optional(options.name)
                }),
                leptos::attr::form(optional(options.form)),
                leptos::attr::required(move || options.required.get()),
                leptos::attr::disabled(move || context.disabled()),
                leptos::attr::readonly(move || options.read_only.get()),
                optional_attribute(optional(options.placeholder).map(leptos::attr::placeholder)),
                optional_attribute(optional(options.autocomplete).map(leptos::attr::autocomplete)),
                optional_attribute(optional(options.inputmode).map(leptos::attr::inputmode)),
                optional_attribute(options.pattern.map(leptos::attr::pattern)),
                optional_attribute(options.minlength.map(leptos::attr::minlength)),
                optional_attribute(options.maxlength.map(leptos::attr::maxlength)),
            ),
            (
                optional_attribute(optional(options.min).map(leptos::attr::min)),
                optional_attribute(optional(options.max).map(leptos::attr::max)),
                optional_attribute(optional(options.step).map(leptos::attr::step)),
                optional_attribute(options.spellcheck.map(leptos::attr::spellcheck)),
                leptos::attr::role(behavior.role),
                optional_attribute(optional(test_id).map(|id| data("data-testid", id))),
                data("data-rustify-part", format!("{}-input", behavior.part)),
                data("data-rustify-field-owner", context.key.get_value()),
                data("data-rustify-form-control", context.key.get_value()),
                data("aria-label", optional(options.aria_label)),
                data("aria-labelledby", optional(options.labelled_by)),
                data("aria-describedby", move || {
                    optional(options.described_by.get())
                }),
                data("aria-invalid", move || context.invalid().then_some("true")),
                data("aria-readonly", move || {
                    context.readonly().then_some("true")
                }),
                data("aria-valuenow", move || {
                    behavior.value_now.map(|value| value.get())
                }),
                data(
                    "aria-valuemin",
                    behavior.value_min.map(crate::number_field::format_number),
                ),
                data(
                    "aria-valuemax",
                    behavior.value_max.map(crate::number_field::format_number),
                ),
                data("data-disabled", move || context.disabled().then_some("")),
                data("data-readonly", move || context.readonly().then_some("")),
                data("data-invalid", move || context.invalid().then_some("")),
                data("dir", move || direction.get().attribute()),
            ),
        ),
    )
    .into_any()
}
#[component]
pub fn Input(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
) -> impl IntoView {
    input_as(leptos::html::input(), class, test_id)
}
fn area_view() -> impl IntoView {
    let context = expect_context::<Context>();
    let node = NodeRef::new();
    context.claim(Editor::Area(node));
    let options = context.options.get_value();
    let initial = context.behavior.shown.get_untracked();
    let initial_html = initial
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('\r', "&#13;");
    #[cfg(feature = "ssr")]
    let initial_html = if initial.starts_with('\n') {
        format!("\n{initial_html}")
    } else {
        initial_html
    };
    view! { <textarea node_ref=node id=options.id class=crate::macros::reactive_merge("", &options.input_class)
    name=optional(options.name) form=optional(options.form) rows=options.rows required=move || options.required.get()
    disabled=move || context.disabled() readonly=move || context.readonly() placeholder=optional(options.placeholder)
    autocomplete=optional(options.autocomplete) inputmode=optional(options.inputmode) minlength=options.minlength maxlength=options.maxlength
    aria-label=optional(options.aria_label) aria-labelledby=optional(options.labelled_by)
    aria-describedby=move || optional(options.described_by.get()) aria-invalid=move || context.invalid().then_some("true")
    aria-readonly=move || context.readonly().then_some("true") data-testid=optional(options.test_id)
    data-rustify-part="text-area-input" data-rustify-field-owner=context.key.get_value() data-rustify-form-control=context.key.get_value()
    inner_html=initial_html /> }.add_any_attr((
        leptos::attr::custom::custom_attribute("data-disabled", move || context.disabled().then_some("")),
        leptos::attr::custom::custom_attribute("data-readonly", move || context.readonly().then_some("")),
        leptos::attr::custom::custom_attribute("data-invalid", move || context.invalid().then_some("")),
    ))
}

/// Single-line, controlled native fields and logical decoration slots.
pub mod text_field {
    use super::*;
    pub use super::{Input, InputProps, SlotSide};
    #[component]
    pub fn Root(
        #[prop(into)] value: Signal<String>,
        #[prop(into)] on_change: Callback<String>,
        #[prop(optional)] kind: TextKind,
        #[prop(optional, into)] id: String,
        #[prop(optional, into)] name: String,
        #[prop(optional, into)] form: String,
        #[prop(optional, into)] aria_label: String,
        #[prop(optional, into)] labelled_by: String,
        #[prop(optional, into)] described_by: Signal<String>,
        #[prop(optional, into)] placeholder: String,
        #[prop(optional, into)] autocomplete: String,
        #[prop(optional, into)] inputmode: String,
        #[prop(optional, into)] required: Signal<bool>,
        #[prop(optional, into)] disabled: Signal<bool>,
        #[prop(optional, into)] read_only: Signal<bool>,
        #[prop(optional, into)] invalid: Signal<bool>,
        #[prop(optional, into)] class: Signal<String>,
        #[prop(optional, into)] input_class: String,
        #[prop(optional, into)] test_id: String,
        #[prop(optional_no_strip, into)] pattern: Option<String>,
        #[prop(optional_no_strip, into)] minlength: Option<u32>,
        #[prop(optional_no_strip, into)] maxlength: Option<u32>,
        children: Children,
    ) -> impl IntoView {
        let rows = None;
        boundary(
            Options {
                id,
                name,
                form,
                aria_label,
                labelled_by,
                described_by,
                placeholder,
                autocomplete,
                inputmode,
                pattern,
                minlength,
                maxlength,
                required,
                disabled,
                read_only,
                invalid,
                class,
                input_class,
                test_id,
                kind,
                rows,
                min: String::new(),
                max: String::new(),
                step: String::new(),
                spellcheck: None,
            },
            false,
            move || text_behavior(value, on_change, "text-field"),
            children,
        )
    }
    /// Adds editor attributes to one native input; its listeners run before Root.
    pub fn control_as<At, Ch>(
        target: leptos::html::HtmlElement<leptos::html::Input, At, Ch>,
    ) -> impl IntoView
    where
        At: leptos::attr::Attribute + Send,
        Ch: RenderHtml + Send,
    {
        input_as(target, String::new(), String::new())
    }
    #[component]
    pub fn Slot(
        #[prop(optional)] side: SlotSide,
        #[prop(optional, into)] class: String,
        children: Children,
    ) -> impl IntoView {
        view! { <span class=crate::macros::reactive_merge("", &class) data-rustify-part="text-field-slot" data-side=side.attribute()>{children()}</span> }
    }
}
/// Multi-line controlled native text.
pub mod text_area {
    use super::*;
    #[component]
    pub fn Root(
        #[prop(into)] value: Signal<String>,
        #[prop(into)] on_change: Callback<String>,
        #[prop(optional_no_strip, into)] rows: Option<u32>,
        #[prop(optional, into)] id: String,
        #[prop(optional, into)] name: String,
        #[prop(optional, into)] form: String,
        #[prop(optional, into)] aria_label: String,
        #[prop(optional, into)] labelled_by: String,
        #[prop(optional, into)] described_by: Signal<String>,
        #[prop(optional, into)] placeholder: String,
        #[prop(optional, into)] autocomplete: String,
        #[prop(optional, into)] inputmode: String,
        #[prop(optional, into)] required: Signal<bool>,
        #[prop(optional, into)] disabled: Signal<bool>,
        #[prop(optional, into)] read_only: Signal<bool>,
        #[prop(optional, into)] invalid: Signal<bool>,
        #[prop(optional, into)] class: Signal<String>,
        #[prop(optional, into)] input_class: String,
        #[prop(optional, into)] test_id: String,
        #[prop(optional_no_strip, into)] pattern: Option<String>,
        #[prop(optional_no_strip, into)] minlength: Option<u32>,
        #[prop(optional_no_strip, into)] maxlength: Option<u32>,
    ) -> impl IntoView {
        let kind = TextKind::Text;
        boundary(
            Options {
                id,
                name,
                form,
                aria_label,
                labelled_by,
                described_by,
                placeholder,
                autocomplete,
                inputmode,
                pattern,
                minlength,
                maxlength,
                required,
                disabled,
                read_only,
                invalid,
                class,
                input_class,
                test_id,
                kind,
                rows,
                min: String::new(),
                max: String::new(),
                step: String::new(),
                spellcheck: None,
            },
            true,
            move || text_behavior(value, on_change, "text-area"),
            Box::new(|| area_view().into_any()),
        )
    }
}
/// Draft-based numeric text. Bounds are announced; the application owns clamping.
pub mod number_field {
    pub use super::text_field::control_as;
    use super::*;
    pub use super::{Input, InputProps};
    #[component]
    pub fn Root(
        #[prop(into)] value: Signal<f64>,
        #[prop(into)] on_change: Callback<f64>,
        #[prop(optional_no_strip, into)] on_preview: Option<Callback<f64>>,
        #[prop(optional_no_strip, into)] on_cancel: Option<Callback<()>>,
        #[prop(optional_no_strip, into)] min: Option<f64>,
        #[prop(optional_no_strip, into)] max: Option<f64>,
        #[prop(optional_no_strip, into)] step: Option<f64>,
        #[prop(optional, into)] id: String,
        #[prop(optional, into)] name: String,
        #[prop(optional, into)] form: String,
        #[prop(optional, into)] aria_label: String,
        #[prop(optional, into)] labelled_by: String,
        #[prop(optional, into)] described_by: Signal<String>,
        #[prop(optional, into)] placeholder: String,
        #[prop(optional, into)] autocomplete: String,
        #[prop(optional, into)] inputmode: String,
        #[prop(optional, into)] required: Signal<bool>,
        #[prop(optional, into)] disabled: Signal<bool>,
        #[prop(optional, into)] read_only: Signal<bool>,
        #[prop(optional, into)] invalid: Signal<bool>,
        #[prop(optional, into)] class: Signal<String>,
        #[prop(optional, into)] input_class: String,
        #[prop(optional, into)] test_id: String,
        #[prop(optional_no_strip, into)] pattern: Option<String>,
        #[prop(optional_no_strip, into)] minlength: Option<u32>,
        #[prop(optional_no_strip, into)] maxlength: Option<u32>,
        children: Children,
    ) -> impl IntoView {
        let kind = TextKind::Text;
        let rows = None;
        let inputmode = if inputmode.is_empty() {
            "decimal".into()
        } else {
            inputmode
        };
        let autocomplete = if autocomplete.is_empty() {
            "off".into()
        } else {
            autocomplete
        };
        boundary(
            Options {
                id,
                name,
                form,
                aria_label,
                labelled_by,
                described_by,
                placeholder,
                autocomplete,
                inputmode,
                pattern,
                minlength,
                maxlength,
                required,
                disabled,
                read_only,
                invalid,
                class,
                input_class,
                test_id,
                kind,
                rows,
                min: String::new(),
                max: String::new(),
                step: String::new(),
                spellcheck: None,
            },
            false,
            move || {
                use crate::number_field::{format_number, parse_number, stepped};
                let still = move || disabled.get_untracked() || read_only.get_untracked();
                let draft =
                    rustify_ui::Draft::new(value, |value| format_number(*value), parse_number)
                        .with_commit(move |value| {
                            if !still() {
                                on_change.run(value);
                            }
                        })
                        .with_preview(move |value| {
                            if !still() {
                                if let Some(preview) = on_preview {
                                    preview.run(value);
                                }
                            }
                        })
                        .with_cancel(move || {
                            if !still() {
                                if let Some(cancel) = on_cancel {
                                    cancel.run(());
                                }
                            }
                        });
                let key = draft.on_keydown();
                let step = step.unwrap_or(1.0);
                Behavior {
                    part: "number-field",
                    accepted: Signal::derive(move || format_number(value.get())),
                    shown: draft.text(),
                    composing: draft.composing(),
                    invalid: draft.invalid(),
                    input: Callback::new(draft.on_input()),
                    focus: Some(Callback::new(draft.on_focus())),
                    blur: Some(Callback::new(move |_| draft.blur())),
                    key: Some(Callback::new(move |event: web_sys::KeyboardEvent| {
                        let delta = match event.key().as_str() {
                            "ArrowUp" => step,
                            "ArrowDown" => -step,
                            "PageUp" => step * 10.0,
                            "PageDown" => -step * 10.0,
                            _ => return key(event),
                        };
                        event.prevent_default();
                        draft.commit();
                        on_change.run(stepped(value.get_untracked(), delta));
                    })),
                    composition: Callback::new(move |active| draft.set_composing(active)),
                    commit: Some(Callback::new(move |_| draft.commit())),
                    reset: Some(Callback::new(move |_| draft.reset())),
                    role: Some("spinbutton"),
                    value_now: Some(Signal::derive(move || format_number(value.get()))),
                    value_min: min,
                    value_max: max,
                    mirror: true,
                    parse_message: "Enter a finite number.",
                }
            },
            children,
        )
    }
}
/// Draft-based hexadecimal color text with an explicit, accepted-value swatch.
pub mod color_field {
    pub use super::text_field::control_as;
    use super::*;
    pub use super::{Input, InputProps};
    #[component]
    pub fn Root(
        #[prop(into)] value: Signal<String>,
        #[prop(into)] on_change: Callback<String>,
        #[prop(optional_no_strip, into)] on_preview: Option<Callback<String>>,
        #[prop(optional_no_strip, into)] on_cancel: Option<Callback<()>>,
        #[prop(optional)] alpha: bool,
        #[prop(optional, into)] id: String,
        #[prop(optional, into)] name: String,
        #[prop(optional, into)] form: String,
        #[prop(optional, into)] aria_label: String,
        #[prop(optional, into)] labelled_by: String,
        #[prop(optional, into)] described_by: Signal<String>,
        #[prop(optional, into)] placeholder: String,
        #[prop(optional, into)] autocomplete: String,
        #[prop(optional, into)] inputmode: String,
        #[prop(optional, into)] required: Signal<bool>,
        #[prop(optional, into)] disabled: Signal<bool>,
        #[prop(optional, into)] read_only: Signal<bool>,
        #[prop(optional, into)] invalid: Signal<bool>,
        #[prop(optional, into)] class: Signal<String>,
        #[prop(optional, into)] input_class: String,
        #[prop(optional, into)] test_id: String,
        #[prop(optional_no_strip, into)] pattern: Option<String>,
        #[prop(optional_no_strip, into)] minlength: Option<u32>,
        #[prop(optional_no_strip, into)] maxlength: Option<u32>,
        children: Children,
    ) -> impl IntoView {
        let kind = TextKind::Text;
        let rows = None;
        let autocomplete = if autocomplete.is_empty() {
            "off".into()
        } else {
            autocomplete
        };
        boundary(
            Options {
                id,
                name,
                form,
                aria_label,
                labelled_by,
                described_by,
                placeholder,
                autocomplete,
                inputmode,
                pattern,
                minlength,
                maxlength,
                required,
                disabled,
                read_only,
                invalid,
                class,
                input_class,
                test_id,
                kind,
                rows,
                min: String::new(),
                max: String::new(),
                step: String::new(),
                spellcheck: None,
            },
            false,
            move || {
                let still = move || disabled.get_untracked() || read_only.get_untracked();
                let draft = rustify_ui::Draft::new(value, String::clone, move |text| {
                    crate::color_field::parse_hex(text, alpha)
                })
                .with_commit(move |value| {
                    if !still() {
                        on_change.run(value);
                    }
                })
                .with_preview(move |value| {
                    if !still() {
                        if let Some(preview) = on_preview {
                            preview.run(value);
                        }
                    }
                })
                .with_cancel(move || {
                    if !still() {
                        if let Some(cancel) = on_cancel {
                            cancel.run(());
                        }
                    }
                });
                Behavior {
                    part: "color-field",
                    accepted: value,
                    shown: draft.text(),
                    composing: draft.composing(),
                    invalid: draft.invalid(),
                    input: Callback::new(draft.on_input()),
                    focus: Some(Callback::new(draft.on_focus())),
                    blur: Some(Callback::new(move |_| draft.blur())),
                    key: Some(Callback::new(draft.on_keydown())),
                    composition: Callback::new(move |active| draft.set_composing(active)),
                    commit: Some(Callback::new(move |_| draft.commit())),
                    reset: Some(Callback::new(move |_| draft.reset())),
                    role: None,
                    value_now: None,
                    value_min: None,
                    value_max: None,
                    mirror: true,
                    parse_message: "Enter a hexadecimal color.",
                }
            },
            children,
        )
    }
    #[component]
    pub fn Swatch(
        #[prop(optional, into)] class: String,
        #[prop(optional, into)] test_id: String,
    ) -> impl IntoView {
        let context = use_context::<Context>();
        let geometry = crate::geometry::geometry_class(move || {
            let color = context
                .and_then(|context| {
                    crate::color_field::parse_hex(&context.behavior.accepted.get(), true)
                })
                .unwrap_or_else(|| "transparent".into());
            format!("background-color:{color};")
        });
        view! { <span class=crate::macros::reactive_merge_with(move || vec![geometry.clone(), class.clone()]) data-rustify-part="color-field-swatch"
        data-testid=optional(test_id) aria-hidden="true" /> }
    }
}
/// A native label associated with a real editor, optionally inherited from Field.
#[component]
pub fn Label(
    #[prop(optional, into)] control: String,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let binding = use_context::<crate::form::FieldBinding>();
    let control = move || {
        if control.is_empty() {
            binding.map(|binding| binding.label_control())
        } else {
            Some(control.clone())
        }
    };
    view! { <label r#for=control class=crate::macros::reactive_merge("", &class) data-rustify-part="label" data-testid=optional(test_id)>{children()}</label> }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn composition_completion_delivers_the_end_only_once() {
        let mut state = Completion::default();
        state.end("previous".into());
        state.start();
        state.end("中文".into());
        assert!(!state.input("中文"));
        assert!(state.input("中文!"));
    }
    #[test]
    fn a_refused_composition_does_not_turn_the_settled_value_into_a_second_request() {
        let mut state = Completion::default();
        state.start();
        state.end("中文".into());
        state.settled("accepted".into());
        assert!(!state.input("accepted"));
        assert!(state.input("another"));
    }
    #[test]
    fn a_different_input_after_composition_is_not_suppressed() {
        let mut state = Completion::default();
        state.end("one".into());
        assert!(state.input("two"));
        assert!(state.input("one"));
    }
}
