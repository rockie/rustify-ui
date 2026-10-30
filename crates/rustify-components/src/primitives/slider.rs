//! Controlled sliders with stable thumb identities and accepted-value geometry.

#[cfg(target_arch = "wasm32")]
mod browser;
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
mod model;
use crate::{geometry::PropError, Orientation};
use leptos::prelude::*;
use model::{Config, Model, Session};
pub use model::{ThumbLabel, ThumbValue};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, PartialEq)]
struct Accepted {
    model: Option<Model>,
    issue: Option<PropError>,
}
#[derive(Clone, Copy, PartialEq)]
struct Entry {
    key: StoredValue<String>,
    dom_id: StoredValue<String>,
    disabled: Signal<bool>,
    named: bool,
}
#[derive(Clone, Default, PartialEq)]
struct Registry {
    entries: Vec<Entry>,
    by_dom: HashMap<String, usize>,
    enabled: HashSet<String>,
    invalid: bool,
}
#[derive(Clone, Copy)]
struct SliderContext {
    id: StoredValue<String>,
    form: StoredValue<Option<String>>,
    accepted: Memo<Accepted>,
    entries: RwSignal<Vec<Entry>>,
    registry: Memo<Registry>,
    disabled: Signal<bool>,
    read_only: Signal<bool>,
    invalid: Signal<bool>,
    described_by: Signal<String>,
    orientation: Orientation,
    inverted: bool,
    direction: Signal<rustify_ui::Direction>,
    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    change: Callback<Vec<ThumbValue>>,
    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    commit: Option<Callback<Vec<ThumbValue>>>,
    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    cancel: Option<Callback<()>>,
    session: StoredValue<Option<Session>>,
    session_revision: RwSignal<u64>,
    #[cfg(target_arch = "wasm32")]
    browser: StoredValue<browser::BrowserState, leptos::reactive::owner::LocalStorage>,
    #[cfg(not(target_arch = "wasm32"))]
    host_owner: StoredValue<leptos::reactive::owner::WeakOwner>,
    #[cfg(not(target_arch = "wasm32"))]
    host_children: StoredValue<Vec<Owner>>,
}
impl SliderContext {
    fn unavailable(self) -> bool {
        self.disabled.get()
            || self.accepted.with(|state| state.issue.is_some())
            || self.registry.with(|registry| registry.invalid)
    }
    fn reverse(self) -> bool {
        (self.orientation == Orientation::Horizontal
            && self.direction.get() == rustify_ui::Direction::RightToLeft)
            ^ self.inverted
    }
    fn register(self, entry: Entry) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let root = self.host_owner.with_value(|owner| owner.upgrade());
            let mut owner = Owner::current();
            self.host_children.update_value(|children| {
                while let Some(current) = owner.take() {
                    if root.as_ref() == Some(&current) {
                        break;
                    }
                    owner = current.parent();
                    children.push(current);
                }
            });
        }
        self.entries.update(|entries| entries.push(entry));
        #[cfg(target_arch = "wasm32")]
        on_cleanup(move || {
            if self.session.try_with_value(|session| {
                session
                    .as_ref()
                    .is_some_and(|session| session.active == entry.key.get_value())
            }) == Some(true)
            {
                self.end(false);
            }
            self.entries
                .try_update(|entries| entries.retain(|item| item.dom_id != entry.dom_id));
        });
    }
    fn bounds(self, key: &str) -> (f64, f64) {
        self.session_revision.track();
        self.accepted.with(|state| {
            state.model.as_ref().map_or((0.0, 100.0), |model| {
                self.session.with_value(|session| {
                    model.bounds(
                        key,
                        session
                            .as_ref()
                            .filter(|session| session.active == key)
                            .and_then(|session| session.neighbours.as_ref()),
                    )
                })
            })
        })
    }
}
pub(crate) fn declaration_order() -> Signal<Vec<String>> {
    let context = use_context::<SliderContext>();
    let order = Memo::new(move |_| {
        context.map_or_else(
            || std::sync::Arc::new(Vec::new()),
            |context| {
                context.accepted.with(|state| {
                    state.model.as_ref().map_or_else(
                        || std::sync::Arc::new(Vec::new()),
                        |model| model.order.clone(),
                    )
                })
            },
        )
    });
    Signal::derive(move || order.get().as_ref().clone())
}

fn optional(value: String) -> Option<String> {
    (!value.is_empty()).then_some(value)
}
fn part_state(context: Option<SliderContext>) -> impl leptos::attr::Attribute {
    use leptos::attr::custom::custom_attribute as data;
    (
        data("data-disabled", move || {
            context
                .is_some_and(|context| context.unavailable())
                .then_some("")
        }),
        data("data-readonly", move || {
            context
                .is_some_and(|context| context.read_only.get())
                .then_some("")
        }),
        data("data-invalid", move || {
            context
                .is_some_and(|context| context.invalid.get())
                .then_some("")
        }),
        data("dir", move || {
            context.map(|context| context.direction.get().attribute())
        }),
    )
}

fn classes(class: String) -> Vec<leptos::attr::any_attribute::AnyAttribute> {
    use leptos::attr::any_attribute::IntoAnyAttribute;
    (!class.is_empty())
        .then(|| {
            leptos::tachys::html::class::class(crate::macros::reactive_merge("", &class))
                .into_any_attr()
        })
        .into_iter()
        .collect()
}

/// Provides a controlled range and delegated pointer/keyboard default actions.
#[component]
pub fn Root(
    #[prop(into)] values: Signal<Vec<ThumbValue>>,
    #[prop(into)] on_change: Callback<Vec<ThumbValue>>,
    #[prop(optional_no_strip, into)] on_commit: Option<Callback<Vec<ThumbValue>>>,
    #[prop(optional_no_strip, into)] on_cancel: Option<Callback<()>>,
    #[prop(default = Signal::stored(0.0), into)] min: Signal<f64>,
    #[prop(default = Signal::stored(100.0), into)] max: Signal<f64>,
    #[prop(default = Signal::stored(1.0), into)] step: Signal<f64>,
    #[prop(optional, into)] min_steps_between: Signal<u32>,
    #[prop(optional, into)] preserve_order: Signal<bool>,
    #[prop(optional)] orientation: Orientation,
    #[prop(optional)] inverted: bool,
    #[prop(optional, into)] id: String,
    #[prop(optional, into)] name: String,
    #[prop(optional, into)] form: String,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] read_only: Signal<bool>,
    #[prop(optional, into)] invalid: Signal<bool>,
    #[prop(optional, into)] described_by: Signal<String>,
    #[prop(optional, into)] aria_label: String,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let owner = Owner::new();
    let view = owner.with(move || {
    let id = StoredValue::new(if id.is_empty() { crate::id::next("slider") } else { id });
    let form = StoredValue::new(optional(super::form::associated_form(form)));
    let (invalid, described_by) = crate::form::field::metadata(invalid, described_by);
    let node = NodeRef::<leptos::html::Div>::new();
    let entries = RwSignal::new(Vec::<Entry>::new());
    let accepted = Memo::new(move |previous: Option<&Accepted>| {
        let previous_model = previous.and_then(|state| state.model.as_ref());
        let config = Config { min: min.get(), max: max.get(), step: step.get(), min_steps_between: min_steps_between.get(), preserve_order: preserve_order.get() };
        match Model::new(&values.get(), config, previous_model) {
            Ok(model) => Accepted { model: Some(model), issue: None },
            Err(issue) => Accepted { model: previous_model.cloned(), issue: Some(issue) },
        }
    });
    let registry = Memo::new(move |_| {
        let entries = entries.get();
        let mut seen = HashSet::with_capacity(entries.len());
        let mut result = Registry { entries: entries.clone(), ..Registry::default() };
        for (at, entry) in entries.iter().enumerate() {
            let key = entry.key.get_value();
            if !seen.insert(key.clone()) || !entry.named { result.invalid = true; }
            result.by_dom.insert(entry.dom_id.get_value(), at);
            if !entry.disabled.get() { result.enabled.insert(key); }
        }
        result
    });
    let context = SliderContext {
        id, form, accepted, entries, registry, disabled, read_only, invalid, described_by, orientation, inverted,
        direction: rustify_ui::use_direction(), change: on_change, commit: on_commit, cancel: on_cancel,
        session: StoredValue::new(None), session_revision: RwSignal::new(0),
        #[cfg(target_arch = "wasm32")]
        browser: StoredValue::new_local(browser::BrowserState::default()),
        #[cfg(not(target_arch = "wasm32"))]
        host_owner: StoredValue::new(Owner::current().expect("slider rendering owner").downgrade()),
        #[cfg(not(target_arch = "wasm32"))]
        host_children: StoredValue::new(Vec::new()),
    };
    super::form::register(id, form.read_value().as_deref().unwrap_or_default(), super::form::ControlSpec {
        composing: Signal::stored(false), commit: None, flush: Callback::new(|_| {}),
        focus: Callback::new(move |_| {
            if let Some(entry) = context.registry.with_untracked(|registry| registry.entries.iter()
                .find(|entry| !entry.disabled.get_untracked()).copied()) { crate::dom::focus_id(&entry.dom_id.get_value()); }
        }),
        owns: Callback::new(move |_element: web_sys::Element| {
            #[cfg(target_arch = "wasm32")] { node.try_get_untracked().flatten().is_some_and(|root| root.contains(Some(&_element))) }
            #[cfg(not(target_arch = "wasm32"))] { false }
        }),
    });
    provide_context(context);
    #[cfg(target_arch = "wasm32")]
    {
        on_cleanup(move || context.end(false));
        Effect::new(move |_| {
            let unavailable = context.unavailable() || read_only.get();
            let enabled = registry.with(|registry| registry.enabled.clone());
            let active = context.session.with_value(|session| session.as_ref().map(|session| session.active.clone()));
            if let Some(active) = active {
                if unavailable || !enabled.contains(&active) || accepted.with(|state| state.model.as_ref().is_none_or(|model| model.get(&active).is_none())) { context.end(false); }
            }
        });
    }
    let children = children();
    let named = !name.is_empty();
    let name = StoredValue::new(name);
    let classes = crate::macros::reactive_merge_with(move || vec![class.get()]);
    view! {
        <div node_ref=node id=id.get_value() role="group" tabindex="-1" class=classes data-testid=test_id
            data-rustify-part="slider-root" data-orientation=orientation.attribute()
            data-disabled=move || context.unavailable().then_some("") data-readonly=move || read_only.get().then_some("")
            data-invalid=move || invalid.get().then_some("") dir=move || context.direction.get().attribute()
            data-rustify-issue=move || if accepted.with(|state| state.issue.is_some()) { Some("invalid-config") }
                else if registry.with(|registry| registry.invalid) { Some("invalid-thumb") } else { None }
            aria-label=optional(aria_label) aria-readonly=move || read_only.get().then_some("true")
            aria-describedby=move || optional(described_by.get())
            on:pointerdown=move |_event| { #[cfg(target_arch = "wasm32")] context.pointer_down(_event); }
            on:pointermove=move |_event| { #[cfg(target_arch = "wasm32")] context.pointer_move(_event); }
            on:pointerup=move |_event| { #[cfg(target_arch = "wasm32")] context.pointer_up(_event); }
            on:pointercancel=move |_event| { #[cfg(target_arch = "wasm32")] context.pointer_cancel(_event); }
            on:lostpointercapture=move |_event| { #[cfg(target_arch = "wasm32")] context.pointer_cancel(_event); }
            on:keydown=move |_event| { #[cfg(target_arch = "wasm32")] context.key_down(_event); }
            on:keyup=move |_event| { #[cfg(target_arch = "wasm32")] context.key_up(_event); }
            on:focusout=move |_event| { #[cfg(target_arch = "wasm32")] context.focus_out(_event); }>
            {children}
            <Show when=move || accepted.with(|state| state.model.is_none())>
                <span role="status">"Invalid slider configuration"</span>
            </Show>
            {move || {
                if !named || context.unavailable() { return Vec::new(); }
                accepted.with(|state| state.model.as_ref().map(|model| model.values.iter().map(|item| view! {
                    <input type="hidden" name=name.get_value() value=item.value.to_string() form=form.get_value() data-rustify-form-control=id.get_value() />
                }).collect::<Vec<_>>()).unwrap_or_default())
            }}
        </div>
    }
    });
    leptos::tachys::reactive_graph::OwnedView::new_with_owner(view, owner)
}

/// The measured track; caller content can contain Range and Thumb parts.
#[component]
pub fn Track(#[prop(optional, into)] class: String, children: Children) -> impl IntoView {
    track_as(leptos::html::span().child(children()), class)
}
pub fn track_as<E, At, Ch>(
    target: leptos::html::HtmlElement<E, At, Ch>,
    class: String,
) -> impl IntoView
where
    E: leptos::html::ElementType + Send,
    At: leptos::attr::Attribute + Send,
    Ch: RenderHtml + Send,
{
    let context = use_context::<SliderContext>();
    super::Slot(
        target,
        (
            classes(class),
            part_state(context),
            leptos::attr::custom::custom_attribute("data-rustify-part", "slider-track"),
            leptos::attr::custom::custom_attribute(
                "data-orientation",
                context.map(|context| context.orientation.attribute()),
            ),
            leptos::attr::custom::custom_attribute(
                "data-rustify-issue",
                context.is_none().then_some("missing-root"),
            ),
        ),
    )
}

/// The accepted interval; one thumb fills from the minimum to that value.
#[component]
pub fn Range(#[prop(optional, into)] class: String) -> impl IntoView {
    range_as(leptos::html::span(), class)
}
pub fn range_as<E, At, Ch>(
    target: leptos::html::HtmlElement<E, At, Ch>,
    class: String,
) -> impl IntoView
where
    E: leptos::html::ElementType + Send,
    At: leptos::attr::Attribute + Send,
    Ch: RenderHtml + Send,
{
    let context = use_context::<SliderContext>();
    let interval = Memo::new(move |_| {
        context.map_or((0.0, 0.0), |context| {
            context.accepted.with(|state| {
                state.model.as_ref().map_or((0.0, 0.0), |model| {
                    let fraction = |value| {
                        (value - model.config.min) / (model.config.max - model.config.min) * 100.0
                    };
                    let low = if model.values.len() <= 1 {
                        0.0
                    } else {
                        model
                            .values
                            .first()
                            .map_or(0.0, |item| fraction(item.value))
                    };
                    let high = model.values.last().map_or(0.0, |item| fraction(item.value));
                    if context.reverse() {
                        (100.0 - high, 100.0 - low)
                    } else {
                        (low, high)
                    }
                })
            })
        })
    });
    let geometry = crate::geometry::geometry_class(move || {
        let (low, high) = interval.get();
        if context.is_some_and(|context| context.orientation == Orientation::Vertical) {
            format!("bottom:{low}%;top:{}%;", 100.0 - high)
        } else {
            format!("left:{low}%;right:{}%;", 100.0 - high)
        }
    });
    super::Slot(
        target,
        (
            classes(class),
            part_state(context),
            super::slot::class_token(geometry),
            leptos::attr::custom::custom_attribute("data-rustify-part", "slider-range"),
            leptos::attr::custom::custom_attribute("aria-hidden", "true"),
            leptos::attr::custom::custom_attribute(
                "data-orientation",
                context.map(|context| context.orientation.attribute()),
            ),
            leptos::attr::custom::custom_attribute(
                "data-rustify-issue",
                context.is_none().then_some("missing-root"),
            ),
        ),
    )
}

/// Item-owned attributes for one custom native slider button.
#[derive(Clone, Debug)]
pub struct ThumbAttributes {
    pub id: String,
    pub control_id: String,
    pub aria_label: String,
    pub labelled_by: String,
    pub value_text: Signal<String>,
    pub disabled: Signal<bool>,
    pub test_id: String,
}
/// One focusable thumb. `id` is a stable application key, independent of its value.
#[component]
pub fn Thumb(
    #[prop(into)] id: String,
    #[prop(optional, into)] control_id: String,
    #[prop(optional, into)] aria_label: String,
    #[prop(optional, into)] labelled_by: String,
    #[prop(optional, into)] value_text: Signal<String>,
    #[prop(optional, into)] disabled: Signal<bool>,
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
) -> impl IntoView {
    thumb_as(
        leptos::html::button().class(crate::macros::reactive_merge("", &class)),
        ThumbAttributes {
            id,
            control_id,
            aria_label,
            labelled_by,
            value_text,
            disabled,
            test_id,
        },
    )
}
pub fn thumb_as<At, Ch>(
    target: leptos::html::HtmlElement<leptos::html::Button, At, Ch>,
    attributes: ThumbAttributes,
) -> impl IntoView
where
    At: leptos::attr::Attribute + Send,
    Ch: RenderHtml + Send,
{
    let context = use_context::<SliderContext>();
    let ThumbAttributes {
        id,
        control_id,
        aria_label,
        labelled_by,
        value_text,
        disabled,
        test_id,
    } = attributes;
    let key = StoredValue::new(id);
    let dom_id = StoredValue::new(crate::form::field::control_id(control_id, "slider-thumb"));
    let named = !aria_label.is_empty() || !labelled_by.is_empty();
    if let Some(context) = context {
        context.register(Entry {
            key,
            dom_id,
            disabled,
            named,
        });
    }
    let current = Memo::new(move |_| {
        context.and_then(|context| {
            context.accepted.with(|state| {
                state
                    .model
                    .as_ref()
                    .and_then(|model| model.get(&key.get_value()))
            })
        })
    });
    let bounds = Memo::new(move |_| {
        context.map_or((0.0, 100.0), |context| context.bounds(&key.get_value()))
    });
    let unavailable = move || {
        disabled.get()
            || current.get().is_none()
            || context.is_none_or(|context| context.unavailable())
    };
    let position = Memo::new(move |_| {
        context.map_or(0.0, |context| {
            context.accepted.with(|state| {
                state.model.as_ref().map_or(0.0, |model| {
                    let at = (current.get().unwrap_or(model.config.min) - model.config.min)
                        / (model.config.max - model.config.min)
                        * 100.0;
                    if context.reverse() {
                        100.0 - at
                    } else {
                        at
                    }
                })
            })
        })
    });
    let geometry = crate::geometry::geometry_class(move || {
        let at = position.get();
        if context.is_some_and(|context| context.orientation == Orientation::Vertical) {
            format!("bottom:{at}%;left:50%;")
        } else {
            format!("left:{at}%;top:50%;")
        }
    });
    use leptos::attr::custom::custom_attribute as data;
    super::Slot(
        target,
        (
            (
                leptos::attr::r#type("button"),
                leptos::attr::id(dom_id.get_value()),
                leptos::attr::form(context.and_then(|context| context.form.get_value())),
                data(
                    "data-rustify-form-control",
                    context.map(|context| context.id.get_value()),
                ),
                leptos::attr::role(move || current.get().map(|_| "slider")),
                leptos::attr::disabled(unavailable),
                super::slot::class_token(geometry),
                data("aria-label", optional(aria_label)),
                data("aria-labelledby", optional(labelled_by)),
                data("aria-valuenow", move || {
                    current.get().map(|value| value.to_string())
                }),
                data("aria-valuemin", move || bounds.get().0.to_string()),
                data("aria-valuemax", move || bounds.get().1.to_string()),
                data("aria-valuetext", move || optional(value_text.get())),
            ),
            (
                data("data-testid", optional(test_id)),
                data("data-rustify-part", "slider-thumb"),
                data(
                    "data-rustify-slider-id",
                    context.map(|context| context.id.get_value()),
                ),
                data(
                    "data-orientation",
                    context.map(|context| context.orientation.attribute()),
                ),
                data(
                    "aria-orientation",
                    context.map(|context| context.orientation.attribute()),
                ),
                data("aria-readonly", move || {
                    context
                        .is_some_and(|context| context.read_only.get())
                        .then_some("true")
                }),
                data("aria-invalid", move || {
                    context
                        .is_some_and(|context| context.invalid.get())
                        .then_some("true")
                }),
                data("aria-describedby", move || {
                    context.and_then(|context| optional(context.described_by.get()))
                }),
                data("data-disabled", move || unavailable().then_some("")),
                data("data-readonly", move || {
                    context
                        .is_some_and(|context| context.read_only.get())
                        .then_some("")
                }),
                data("data-invalid", move || {
                    context
                        .is_some_and(|context| context.invalid.get())
                        .then_some("")
                }),
                data("dir", move || {
                    context.map(|context| context.direction.get().attribute())
                }),
                data("data-rustify-issue", move || {
                    if context.is_none() {
                        Some("missing-root")
                    } else if !named {
                        Some("missing-name")
                    } else if current.get().is_none() {
                        Some("missing-value")
                    } else {
                        None
                    }
                }),
            ),
        ),
    )
}
