//! A native form boundary over the application's value-free Form handle.
use crate::form::{field::FieldContext, Form};
use leptos::prelude::*;
use rustify_ui::Submit;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

type Rules = Arc<dyn Fn() -> Vec<(&'static str, String)> + Send + Sync>;

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
#[derive(Clone, Copy)]
pub(crate) struct ControlSpec {
    pub composing: Signal<bool>,
    pub commit: Option<Callback<()>>,
    pub flush: Callback<()>,
    pub focus: Callback<()>,
    pub owns: Callback<web_sys::Element, bool>,
}
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
#[derive(Clone, Copy)]
struct Control {
    key: StoredValue<String>,
    field: Option<&'static str>,
    spec: ControlSpec,
}
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
#[derive(Clone, Copy)]
struct Context {
    form: Form,
    id: StoredValue<String>,
    node: NodeRef<leptos::html::Form>,
    controls: RwSignal<Vec<Control>>,
    index: Memo<HashMap<String, usize>>,
    fields: StoredValue<HashSet<&'static str>>,
    rules: StoredValue<Rules>,
    asked: Option<Callback<Submit>>,
    reset: Option<Callback<()>>,
}
/// The native form id inherited by a logical child, including scope portals.
pub(crate) fn associated_form(explicit: String) -> String {
    if explicit.is_empty() {
        use_context::<Context>().map_or(explicit, |context| context.id.get_value())
    } else {
        explicit
    }
}
pub(crate) fn inside(form: Form) -> bool {
    use_context::<Context>().is_some_and(|context| context.form.same(form))
}
/// Registers one logical control; all of its native editors/mirrors share the key.
pub(crate) fn register(key: StoredValue<String>, native_form: &str, spec: ControlSpec) {
    let Some(context) =
        use_context::<Context>().filter(|context| context.id.read_value().as_str() == native_form)
    else {
        return;
    };
    let field = use_context::<FieldContext>()
        .filter(|field| field.form.same(context.form))
        .map(|field| field.field)
        .filter(|field| context.fields.with_value(|fields| fields.contains(field)));
    context
        .controls
        .update(|controls| controls.push(Control { key, field, spec }));
    #[cfg(target_arch = "wasm32")]
    on_cleanup(move || {
        context
            .controls
            .try_update(|controls| controls.retain(|control| control.key != key));
    });
}

/// Runs native submit/reset through one Form handle. Values remain application-owned.
#[component]
pub fn Root(
    form: Form,
    rules: impl Fn() -> Vec<(&'static str, String)> + Send + Sync + 'static,
    #[prop(optional_no_strip, into)] on_reset: Option<Callback<()>>,
    #[prop(optional_no_strip, into)] on_asked: Option<Callback<Submit>>,
    #[prop(optional, into)] id: String,
    #[prop(optional, into)] aria_label: String,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let owner = Owner::new();
    let view = owner.with(move || {
        if use_context::<Context>().is_some() {
            return view! { <div data-rustify-part="form-root" data-rustify-issue="nested-form">{children()}</div> }.into_any();
        }
        let id = StoredValue::new(if id.is_empty() { crate::id::next("form-root") } else { id });
        let node = NodeRef::new();
        let controls = RwSignal::new(Vec::<Control>::new());
        let index = Memo::new(move |_| controls.with(|controls| controls.iter().enumerate().map(|(at, control)| (control.key.get_value(), at)).collect()));
        let context = Context { form, id, node, controls, index, fields: StoredValue::new(form.fields().into_iter().collect()),
            rules: StoredValue::new(Arc::new(rules)), asked: on_asked, reset: on_reset };
        provide_context(context);
        crate::form::provide_form(form);
        #[cfg(target_arch = "wasm32")]
        context.connect();
        let classes = crate::macros::reactive_merge_with(move || vec![class.get()]);
        view! { <form node_ref=node id=id.get_value() novalidate=true class=classes
            aria-label=(!aria_label.is_empty()).then_some(aria_label) data-testid=test_id data-rustify-part="form-root">
            {children()}
        </form> }.into_any()
    });
    leptos::tachys::reactive_graph::OwnedView::new_with_owner(view, owner)
}

#[cfg(target_arch = "wasm32")]
impl Context {
    fn native(self) -> Option<web_sys::HtmlFormElement> {
        self.node.try_get_untracked().flatten()
    }
    fn notify(self, asked: Submit) {
        if let Some(callback) = self.asked {
            callback.run(asked);
        }
    }
    fn ordered(self, form: &web_sys::HtmlFormElement) -> Vec<(web_sys::Element, Option<Control>)> {
        let controls = self.controls.get_untracked();
        self.index.with_untracked(|index| {
            let elements = form.elements();
            (0..elements.length())
                .filter_map(|at| elements.item(at))
                .map(|element| {
                    let control = element
                        .get_attribute("data-rustify-form-control")
                        .and_then(|key| index.get(&key).map(|&at| controls[at]))
                        .filter(|control| control.spec.owns.run(element.clone()));
                    (element, control)
                })
                .collect()
        })
    }
    fn focus(self, field: &'static str) {
        if let Some(form) = self.native() {
            if let Some(control) = self
                .ordered(&form)
                .into_iter()
                .find_map(|(_, control)| control.filter(|control| control.field == Some(field)))
            {
                control.spec.focus.run(());
                return;
            }
        }
        crate::dom::focus_id(&self.form.control_id(field));
    }
    fn submit(self, event: web_sys::Event) {
        use leptos::wasm_bindgen::JsCast;
        let Some(form) = self.native() else {
            return;
        };
        if event.default_prevented() || event.target().as_ref() != Some(form.as_ref()) {
            return;
        }
        event.prevent_default();
        if self.form.submitting() {
            self.notify(Submit::Busy);
            return;
        }
        if form
            .query_selector("[data-rustify-issue=nested-form]")
            .ok()
            .flatten()
            .is_some()
        {
            return;
        }
        let ordered = self.ordered(&form);
        let mut seen = HashSet::new();
        let controls: Vec<_> = ordered
            .iter()
            .filter_map(|(_, control)| *control)
            .filter(|control| seen.insert(control.key))
            .collect();
        if controls
            .iter()
            .any(|control| control.spec.composing.get_untracked())
        {
            return;
        }
        for control in controls {
            if let Some(commit) = control.spec.commit {
                commit.run(());
            }
            control.spec.flush.run(());
        }
        let mut errors = HashMap::new();
        let mut unbound = None;
        // Re-read after commits: an application may synchronously replace controls.
        for (element, control) in self.ordered(&form) {
            if let Some(message) = constraint(&element) {
                if let Some(field) = control.and_then(|control| control.field) {
                    errors.entry(field).or_insert(message);
                } else if unbound.is_none() {
                    unbound = Some(element);
                }
            }
        }
        if let Some(element) = unbound {
            if let Some(element) = element.dyn_ref::<web_sys::HtmlElement>() {
                let _ = element.focus();
            }
            return;
        }
        self.form.replace_constraints(errors);
        let asked = self.rules.with_value(|rules| self.form.submit(|| rules()));
        self.notify(asked);
    }
    fn reset(self, event: web_sys::Event) {
        let Some(form) = self.native() else {
            return;
        };
        if event.default_prevented() || event.target().as_ref() != Some(form.as_ref()) {
            return;
        }
        event.prevent_default();
        if self.form.submitting() {
            self.notify(Submit::Busy);
            return;
        }
        if let Some(reset) = self.reset {
            reset.run(());
        }
    }
    fn connect(self) {
        use leptos::wasm_bindgen::JsCast;
        let held = StoredValue::new_local(None::<Vec<rustify_ui::Listener>>);
        let focus_owner = self.id.get_value();
        self.form.install_focus(
            focus_owner.clone(),
            Callback::new(move |field| self.focus(field)),
        );
        on_cleanup(move || {
            self.form.remove_focus(&focus_owner);
            held.try_update_value(|held| {
                held.take();
            });
        });
        Effect::new(move |_| {
            let Some(form) = self.node.get() else {
                return;
            };
            let Some(parent) = form.parent_node() else {
                return;
            };
            // The caller's handlers run on the form first. Parent bubbling is
            // synchronous and can still prevent the native default action.
            held.set_value(Some(vec![
                rustify_ui::listen(
                    parent.unchecked_ref(),
                    "submit",
                    rustify_ui::ListenOptions::default(),
                    move |event| self.submit(event),
                ),
                rustify_ui::listen(
                    parent.unchecked_ref(),
                    "reset",
                    rustify_ui::ListenOptions::default(),
                    move |event| self.reset(event),
                ),
            ]));
        });
    }
}
#[cfg(target_arch = "wasm32")]
fn constraint(element: &web_sys::Element) -> Option<String> {
    use leptos::wasm_bindgen::JsCast;
    let check = js_sys::Reflect::get(element, &"checkValidity".into())
        .ok()?
        .dyn_into::<js_sys::Function>()
        .ok()?;
    if check
        .call0(element)
        .ok()
        .and_then(|value| value.as_bool())
        .unwrap_or(false)
    {
        return None;
    }
    Some(
        js_sys::Reflect::get(element, &"validationMessage".into())
            .ok()
            .and_then(|value| value.as_string())
            .unwrap_or_default(),
    )
}

// Field composition shares metadata and the Form handle, without a skin.
pub use crate::form::field::{
    Description, DescriptionProps, Label, LabelProps, Message, MessageProps, Root as Field,
    RootProps as FieldProps,
};
