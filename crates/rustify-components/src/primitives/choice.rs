//! Native choices with application-owned values and delegated default actions.

use crate::{geometry::PropError, Orientation};
use leptos::prelude::*;
use std::collections::{HashMap, HashSet};

/// The accepted state of a checkbox. Mixed is not a successful checked value.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum CheckedState {
    #[default]
    Unchecked,
    Checked,
    Indeterminate,
}
impl CheckedState {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Unchecked => "unchecked",
            Self::Checked => "checked",
            Self::Indeterminate => "indeterminate",
        }
    }
    pub const fn aria(self) -> &'static str {
        match self {
            Self::Unchecked => "false",
            Self::Checked => "true",
            Self::Indeterminate => "mixed",
        }
    }
    fn activated(self) -> Self {
        if self == Self::Checked {
            Self::Unchecked
        } else {
            Self::Checked
        }
    }
}
impl From<bool> for CheckedState {
    fn from(value: bool) -> Self {
        if value {
            Self::Checked
        } else {
            Self::Unchecked
        }
    }
}

/// Whether a group accepts one or several values.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SelectionMode {
    #[default]
    Single,
    Multiple,
}

/// Item-owned attributes for a caller-supplied native group target.
///
/// A string value alone is shorthand for [`ItemAttributes::new`]. Ordinary
/// attributes and listeners remain on the target; state and ARIA use these props.
#[derive(Clone, Debug)]
pub struct ItemAttributes {
    pub value: String,
    pub id: String,
    pub aria_label: String,
    pub labelled_by: String,
    pub described_by: Signal<String>,
    pub disabled: Signal<bool>,
}
impl ItemAttributes {
    pub fn new(value: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            id: String::new(),
            aria_label: String::new(),
            labelled_by: String::new(),
            described_by: Signal::stored(String::new()),
            disabled: Signal::stored(false),
        }
    }
}
impl From<String> for ItemAttributes {
    fn from(value: String) -> Self {
        Self::new(value)
    }
}
impl From<&str> for ItemAttributes {
    fn from(value: &str) -> Self {
        Self::new(value)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Kind {
    Checkbox,
    Radio,
    Switch,
}
impl Kind {
    fn part(self) -> &'static str {
        match self {
            Self::Checkbox => "checkbox",
            Self::Radio => "radio",
            Self::Switch => "switch",
        }
    }
}

#[derive(Clone, Copy)]
struct Native {
    kind: Kind,
    id: StoredValue<String>,
    owner: StoredValue<String>,
    state: Signal<CheckedState>,
    change: Callback<CheckedState>,
    disabled: Signal<bool>,
    read_only: Signal<bool>,
    invalid: Signal<bool>,
    required: Signal<bool>,
    described_by: Signal<String>,
    name: StoredValue<Option<String>>,
    value: StoredValue<String>,
    form: StoredValue<Option<String>>,
    aria_label: StoredValue<Option<String>>,
    labelled_by: StoredValue<Option<String>>,
    node: NodeRef<leptos::html::Input>,
    group: Option<Group>,
    #[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
    peers: RadioPeers,
}

#[derive(Clone, Copy)]
struct RadioPeers(StoredValue<HashMap<String, Vec<Native>>>);
impl RadioPeers {
    fn current() -> Self {
        #[cfg(target_arch = "wasm32")]
        if let (Some(roots), Some(mut owner)) =
            (use_context::<rustify_ui::ScopeRoots>(), Owner::current())
        {
            let container = roots.container();
            while let Some(parent) = owner.parent() {
                let same_scope = parent
                    .with(|| use_context::<rustify_ui::ScopeRoots>())
                    .is_some_and(|roots| roots.container() == container);
                if !same_scope {
                    break;
                }
                owner = parent;
            }
            return owner.with(Self::in_current_owner);
        }
        Self::in_current_owner()
    }
    fn in_current_owner() -> Self {
        use_context::<Self>().unwrap_or_else(|| {
            let peers = Self(StoredValue::new(HashMap::new()));
            provide_context(peers);
            peers
        })
    }
    fn register(self, native: Native) {
        if native.kind != Kind::Radio {
            return;
        }
        if let Some(name) = native.name.get_value() {
            self.0
                .update_value(|groups| groups.entry(name.clone()).or_default().push(native));
            on_cleanup(move || {
                self.0.try_update_value(|groups| {
                    if let Some(group) = groups.get_mut(&name) {
                        group.retain(|item| item.id != native.id);
                        if group.is_empty() {
                            groups.remove(&name);
                        }
                    }
                });
            });
        }
    }
}

impl Native {
    fn settle(self) {
        #[cfg(target_arch = "wasm32")]
        if let (Some(node), Some(state)) = (
            self.node.try_get_untracked().flatten(),
            self.state.try_get_untracked(),
        ) {
            node.set_checked(state == CheckedState::Checked);
            node.set_indeterminate(state == CheckedState::Indeterminate);
        }
    }
    fn settle_peers(self) {
        self.settle();
        #[cfg(target_arch = "wasm32")]
        if self.kind == Kind::Radio {
            if let (Some(name), Some(node)) = (
                self.name.try_get_value().flatten(),
                self.node.try_get_untracked().flatten(),
            ) {
                let form = node.form();
                self.peers.0.try_with_value(|groups| {
                    if let Some(peers) = groups.get(&name) {
                        for peer in peers {
                            if peer
                                .node
                                .get_untracked()
                                .is_some_and(|node| node.form() == form)
                            {
                                peer.settle();
                            }
                        }
                    }
                });
            }
        }
    }
    fn request(self) {
        if !self.disabled.get_untracked() && !self.read_only.get_untracked() {
            self.change.run(if self.kind == Kind::Radio {
                CheckedState::Checked
            } else {
                self.state.get_untracked().activated()
            });
        }
        self.settle_peers();
    }
}

struct NativeOptions {
    kind: Kind,
    state: Signal<CheckedState>,
    change: Callback<CheckedState>,
    id: String,
    name: String,
    value: String,
    form: String,
    aria_label: String,
    labelled_by: String,
    described_by: Signal<String>,
    disabled: Signal<bool>,
    read_only: Signal<bool>,
    invalid: Signal<bool>,
    required: Signal<bool>,
    class: Signal<String>,
}
fn optional(value: String) -> Option<String> {
    (!value.is_empty()).then_some(value)
}
fn native_root(options: NativeOptions, children: Children) -> impl IntoView {
    let peers = RadioPeers::current();
    let direction = rustify_ui::use_direction();
    let owner = Owner::new();
    let view = owner.with(|| {
        let NativeOptions { kind, state, change, id, name, value, form, aria_label, labelled_by, described_by,
            disabled, read_only, invalid, required, class } = options;
        let id = StoredValue::new(crate::form::field::control_id(id, kind.part()));
        let form = super::form::associated_form(form);
        let (invalid, described_by) = crate::form::field::metadata(invalid, described_by);
        let native = Native { kind, id, owner: id, state, change, disabled, read_only, invalid, required,
            described_by, name: StoredValue::new(optional(name)), value: StoredValue::new(value),
            form: StoredValue::new(optional(form)), aria_label: StoredValue::new(optional(aria_label)),
            labelled_by: StoredValue::new(optional(labelled_by)), node: NodeRef::new(), group: None, peers };
        peers.register(native);
        super::form::register(native.owner, native.form.read_value().as_deref().unwrap_or_default(), super::form::ControlSpec {
            composing: Signal::stored(false), commit: None,
            flush: Callback::new(move |_| native.settle()),
            focus: Callback::new(move |_| crate::dom::focus_id(&native.id.get_value())),
            owns: Callback::new(move |_element: web_sys::Element| {
                #[cfg(target_arch = "wasm32")] { native.node.try_get_untracked().flatten().is_some_and(|node| AsRef::<web_sys::Element>::as_ref(&node) == &_element) }
                #[cfg(not(target_arch = "wasm32"))] { false }
            }),
        });
        provide_context(native);
        view! {
            <span class=move || class.get() data-rustify-part=format!("{}-root", kind.part())
                data-state=move || state.get().as_str()
                data-disabled=move || disabled.get().then_some("")
                data-readonly=move || read_only.get().then_some("") data-invalid=move || invalid.get().then_some("")
                dir=move || direction.get().attribute()
                on:click=move |event| {
                    if owns_input(&event, id) && (disabled.get_untracked() || read_only.get_untracked()) {
                        event.prevent_default();
                        native.settle_peers();
                    }
                }
                on:keydown=move |event| {
                    if kind == Kind::Radio && read_only.get_untracked()
                        && matches!(event.key().as_str(), "ArrowUp" | "ArrowDown" | "ArrowLeft" | "ArrowRight" | " ") {
                        event.prevent_default(); native.settle_peers();
                    }
                }
                on:change=move |event| {
                    if !owns_input(&event, id) { return; }
                    if !event.default_prevented() { native.request(); } else { native.settle_peers(); }
                }
            >{children()}</span>
        }
    });
    leptos::tachys::reactive_graph::OwnedView::new_with_owner(view, owner)
}

fn owns_input<E: AsRef<web_sys::Event>>(event: &E, owner: StoredValue<String>) -> bool {
    use leptos::wasm_bindgen::JsCast;
    event
        .as_ref()
        .target()
        .and_then(|target| target.dyn_into::<web_sys::Element>().ok())
        .and_then(|element| element.get_attribute("data-rustify-choice-owner"))
        .is_some_and(|id| id == owner.get_value())
}

/// Appends native control attributes without replacing caller listeners.
fn native_control_as<At, Ch>(
    target: leptos::html::HtmlElement<leptos::html::Input, At, Ch>,
    native: Option<Native>,
    class: String,
    test_id: String,
) -> impl IntoView
where
    At: leptos::attr::Attribute + Send,
    Ch: RenderHtml + Send,
{
    use leptos::attr::{self, custom::custom_attribute as data};
    use leptos::tachys::html::{class::class as css_class, node_ref::node_ref, property::prop};
    let Some(native) = native else {
        return super::Slot(
            target,
            (
                data("data-rustify-issue", "missing-root"),
                attr::disabled(true),
                css_class(class),
            ),
        )
        .into_any();
    };
    let direction = rustify_ui::use_direction();
    let Native {
        kind,
        id,
        owner,
        state,
        disabled,
        read_only,
        invalid,
        required,
        described_by,
        name,
        value,
        form,
        aria_label,
        labelled_by,
        node,
        group,
        ..
    } = native;
    super::Slot(
        target,
        (
            (
                node_ref(node),
                attr::r#type(if kind == Kind::Radio {
                    "radio"
                } else {
                    "checkbox"
                }),
                attr::id(id.get_value()),
                attr::name(name.get_value()),
                attr::value(value.get_value()),
                attr::form(form.get_value()),
                optional_attribute(
                    (!class.is_empty())
                        .then(|| css_class(crate::macros::reactive_merge("", &class))),
                ),
                optional_attribute((!test_id.is_empty()).then(|| data("data-testid", test_id))),
                data("data-rustify-choice-owner", owner.get_value()),
                data("data-rustify-form-control", owner.get_value()),
                data(
                    "data-rustify-group-id",
                    group.map(|group| group.id.get_value()),
                ),
                data("role", (kind == Kind::Switch).then_some("switch")),
            ),
            (
                data("aria-label", aria_label.get_value()),
                data("aria-labelledby", labelled_by.get_value()),
                data("aria-describedby", move || optional(described_by.get())),
                data("aria-checked", move || state.get().aria()),
                data("aria-invalid", move || invalid.get().then_some("true")),
                data("aria-readonly", move || read_only.get().then_some("true")),
                data("data-state", move || state.get().as_str()),
                data("data-disabled", move || disabled.get().then_some("")),
                data("data-readonly", move || read_only.get().then_some("")),
                data("data-invalid", move || invalid.get().then_some("")),
                data("dir", move || direction.get().attribute()),
                data(
                    "data-orientation",
                    group.map(|group| group.orientation.attribute()),
                ),
                data("tabindex", move || {
                    group.and_then(|group| group.tabindex(id))
                }),
            ),
            (
                attr::checked(move || state.get() == CheckedState::Checked),
                attr::required(move || required.get()),
                attr::disabled(move || disabled.get()),
                prop("checked", move || state.get() == CheckedState::Checked),
                prop("indeterminate", move || {
                    state.get() == CheckedState::Indeterminate
                }),
                prop("required", move || required.get()),
                prop("disabled", move || disabled.get()),
                data("data-rustify-part", format!("{}-control", kind.part())),
            ),
        ),
    )
    .into_any()
}

#[component]
pub fn Control(
    #[prop(optional, into)] class: String,
    #[prop(optional, into)] test_id: String,
) -> impl IntoView {
    native_control_as(
        leptos::html::input(),
        use_context::<Native>(),
        class,
        test_id,
    )
}

#[component]
pub fn Indicator(
    #[prop(optional, into)] class: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let native = use_context::<Native>();
    let direction = rustify_ui::use_direction();
    view! {
        <span class=crate::macros::reactive_merge("", &class) data-rustify-part=if native.is_some_and(|n| n.kind == Kind::Switch) { "switch-thumb" } else { "choice-indicator" }
            data-rustify-issue=native.is_none().then_some("missing-root") aria-hidden="true"
            data-orientation=native.and_then(|native| native.group.map(|group| group.orientation.attribute()))
            dir=move || direction.get().attribute()
            data-state=move || native.map_or("unchecked", |n| n.state.get().as_str())
            data-disabled=move || native.is_some_and(|n| n.disabled.get()).then_some("")
            data-readonly=move || native.is_some_and(|n| n.read_only.get()).then_some("")
            data-invalid=move || native.is_some_and(|n| n.invalid.get()).then_some("")>
            {children.map(|children| children())}
        </span>
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum GroupKind {
    Checkbox,
    Radio,
    Toggle,
}
#[derive(Clone, Copy)]
struct GroupItem {
    id: StoredValue<String>,
    value: StoredValue<String>,
    disabled: Signal<bool>,
    native: Option<Native>,
}
#[derive(Clone, Default)]
struct GroupIndex {
    items: Vec<GroupItem>,
    by_id: HashMap<String, usize>,
    open: Vec<usize>,
    positions: HashMap<String, usize>,
    invalid: bool,
    order: Vec<String>,
}
impl GroupIndex {
    fn new(items: Vec<GroupItem>) -> Self {
        let mut index = Self {
            items,
            ..Self::default()
        };
        index.order = index
            .items
            .iter()
            .map(|item| item.value.get_value())
            .collect();
        index.invalid = validate_keys(&index.order).is_err();
        for (position, item) in index.items.iter().enumerate() {
            index.by_id.insert(item.id.get_value(), position);
            if !item.disabled.get() {
                index
                    .positions
                    .insert(item.id.get_value(), index.open.len());
                index.open.push(position);
            }
        }
        index
    }
}

#[derive(Clone, Copy)]
struct Group {
    id: StoredValue<String>,
    kind: GroupKind,
    orientation: Orientation,
    mode: SelectionMode,
    allow_empty: bool,
    selected: Memo<HashSet<String>>,
    has_selection: Memo<bool>,
    change: Callback<Vec<String>>,
    items: RwSignal<Vec<GroupItem>>,
    index: Memo<GroupIndex>,
    stop: Memo<Option<String>>,
    last: RwSignal<Option<String>>,
    focus_owned: RwSignal<bool>,
    disabled: Signal<bool>,
    read_only: Signal<bool>,
    invalid: Signal<bool>,
    required: Signal<bool>,
    name: StoredValue<String>,
    form: StoredValue<String>,
    described_by: Signal<String>,
    node: NodeRef<leptos::html::Div>,
    content_revision: RwSignal<u64>,
    #[cfg(not(target_arch = "wasm32"))]
    host_owner: StoredValue<leptos::reactive::owner::WeakOwner>,
    #[cfg(not(target_arch = "wasm32"))]
    host_children: StoredValue<Vec<Owner>>,
}
impl Group {
    fn tabindex(self, id: StoredValue<String>) -> Option<&'static str> {
        (self.kind != GroupKind::Checkbox).then(|| {
            if self.stop.get().as_deref() == Some(id.read_value().as_str()) {
                "0"
            } else {
                "-1"
            }
        })
    }
    fn choose(self, value: &str) {
        if self.disabled.get_untracked() || self.read_only.get_untracked() {
            return;
        }
        let next = self.index.with_untracked(|index| {
            if index.invalid {
                return None;
            }
            selection_request(
                &self.selected.get_untracked(),
                &index.order,
                value,
                self.mode,
                self.allow_empty,
            )
        });
        if let Some(next) = next {
            self.change.run(next);
        }
    }

    fn settle(self) {
        self.index.try_with_untracked(|index| {
            for item in &index.items {
                if let Some(native) = item.native {
                    native.settle();
                }
            }
        });
    }
    fn event_item(self, event: &web_sys::Event) -> Option<GroupItem> {
        use leptos::wasm_bindgen::JsCast;
        let element = event.target()?.dyn_into::<web_sys::Element>().ok()?;
        if element.get_attribute("data-rustify-group-id")? != self.id.get_value() {
            return None;
        }
        let id = element.id();
        self.index
            .with_untracked(|index| index.by_id.get(&id).map(|&at| index.items[at]))
    }
    #[cfg(target_arch = "wasm32")]
    fn recover_focus(self, from: usize) {
        if !self
            .node
            .try_get_untracked()
            .flatten()
            .is_some_and(|root| root.is_connected())
        {
            return;
        }
        let next = self
            .index
            .try_with_untracked(|index| {
                index
                    .open
                    .iter()
                    .find(|&&at| at >= from)
                    .or_else(|| index.open.last())
                    .map(|&at| index.items[at].id.get_value())
            })
            .flatten();
        if let Some(next) = next {
            crate::dom::focus_id(&next);
        } else if let Some(root) = self.node.get_untracked() {
            let _ = root.focus();
        }
    }
    fn register(self, item: GroupItem) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            // OwnedView consumes each row during SSR. Keep its declaring
            // owners alive until the group finishes serializing all siblings.
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
        self.items.update(|items| items.push(item));
        #[cfg(target_arch = "wasm32")]
        on_cleanup(move || {
            #[cfg(target_arch = "wasm32")]
            let focused = self.focus_owned.try_get_untracked() == Some(true)
                && self.last.try_get_untracked().flatten().as_deref()
                    == Some(item.id.read_value().as_str());
            #[cfg(target_arch = "wasm32")]
            let position = self
                .index
                .try_with_untracked(|index| index.by_id.get(&item.id.get_value()).copied())
                .flatten()
                .unwrap_or_default();
            self.items
                .try_update(|items| items.retain(|entry| entry.id != item.id));
            #[cfg(target_arch = "wasm32")]
            if focused {
                self.recover_focus(position);
            }
        });
    }
    fn native(
        self,
        value: String,
        disabled: Signal<bool>,
        id: String,
        aria_label: String,
        described_by: Signal<String>,
    ) -> Native {
        let item_id = StoredValue::new(crate::form::field::control_id(id, "choice-item"));
        let value = StoredValue::new(value);
        let state = Signal::derive(move || {
            CheckedState::from(self.selected.with(|set| set.contains(&value.get_value())))
        });
        let unavailable = Signal::derive(move || {
            self.disabled.get() || disabled.get() || self.index.with(|index| index.invalid)
        });
        let required = Memo::new(move |_| {
            self.required.get()
                && !self.disabled.get()
                && self.index.with(|index| {
                    !self.has_selection.get()
                        && index
                            .open
                            .first()
                            .is_some_and(|&at| index.items[at].id == item_id)
                })
        });
        let peers = RadioPeers::current();
        let native = Native {
            kind: if self.kind == GroupKind::Radio {
                Kind::Radio
            } else {
                Kind::Checkbox
            },
            id: item_id,
            owner: self.id,
            state,
            change: Callback::new(move |_| self.choose(&value.get_value())),
            disabled: unavailable,
            read_only: self.read_only,
            invalid: self.invalid,
            required: required.into(),
            name: StoredValue::new(optional(self.name.get_value())),
            value,
            form: StoredValue::new(optional(self.form.get_value())),
            aria_label: StoredValue::new(optional(aria_label)),
            labelled_by: StoredValue::new(None),
            described_by: Signal::derive(move || {
                merge_ids(&[&self.described_by.get(), &described_by.get()])
            }),
            node: NodeRef::new(),
            group: Some(self),
            peers,
        };
        self.register(GroupItem {
            id: item_id,
            value,
            disabled,
            native: Some(native),
        });
        peers.register(native);
        native
    }
}

pub(crate) fn selection_request(
    selected: &HashSet<String>,
    order: &[String],
    value: &str,
    mode: SelectionMode,
    allow_empty: bool,
) -> Option<Vec<String>> {
    if !order.iter().any(|item| item == value) {
        return None;
    }
    let was = selected.contains(value);
    if mode == SelectionMode::Single {
        if was && !allow_empty {
            return None;
        }
        return Some(if was {
            Vec::new()
        } else {
            vec![value.to_string()]
        });
    }
    let mut emitted = HashSet::new();
    Some(
        order
            .iter()
            .filter(|item| {
                emitted.insert(item.as_str())
                    && if *item == value {
                        !was
                    } else {
                        selected.contains(*item)
                    }
            })
            .cloned()
            .collect(),
    )
}
pub(crate) fn merge_ids(ids: &[&str]) -> String {
    let mut seen = HashSet::new();
    ids.iter()
        .flat_map(|ids| ids.split_whitespace())
        .filter(|id| seen.insert(*id))
        .collect::<Vec<_>>()
        .join(" ")
}

struct GroupOptions {
    kind: GroupKind,
    mode: SelectionMode,
    allow_empty: bool,
    value: Signal<Vec<String>>,
    change: Callback<Vec<String>>,
    name: String,
    form: String,
    disabled: Signal<bool>,
    read_only: Signal<bool>,
    invalid: Signal<bool>,
    required: Signal<bool>,
    described_by: Signal<String>,
    aria_label: String,
    orientation: Orientation,
    looping: bool,
    toolbar: bool,
    class: Signal<String>,
    test_id: String,
}
fn group_root(options: GroupOptions, children: Children) -> impl IntoView {
    let owner = Owner::new();
    let view = owner.with(|| {
        let GroupOptions { kind, mode, allow_empty, value, change, name, form, disabled, read_only, invalid, required,
            described_by, aria_label, orientation, looping, toolbar, class, test_id } = options;
        let form = super::form::associated_form(form);
        let (invalid, described_by) = crate::form::field::metadata(invalid, described_by);
        let items = RwSignal::new(Vec::<GroupItem>::new());
        let invalid_selection = Memo::new(move |_| value.with(|values| validate_selection(values, mode).is_err()));
        let disabled = Signal::derive(move || disabled.get() || invalid_selection.get());
        let selected = Memo::new(move |_| value.with(|values| {
            values.iter().take(if mode == SelectionMode::Single { 1 } else { usize::MAX }).cloned().collect::<HashSet<_>>()
        }));
        let order = RwSignal::new(Vec::<String>::new());
        let index = Memo::new(move |_| {
            let mut entries = items.get();
            let positions = order.get().into_iter().enumerate().map(|(at, id)| (id, at)).collect::<HashMap<_,_>>();
            if !positions.is_empty() { entries.sort_by_key(|item| positions.get(&item.id.get_value()).copied().unwrap_or(usize::MAX)); }
            GroupIndex::new(entries)
        });
        let has_selection = Memo::new(move |_| index.with(|index| selected.with(|selected| {
            index.open.iter().any(|&at| selected.contains(&index.items[at].value.get_value()))
        })));
        let last = RwSignal::new(None::<String>);
        let stop = Memo::new(move |_| index.with(|index| {
            let selected = selected.get();
            last.get().filter(|id| index.positions.contains_key(id))
                .or_else(|| index.open.iter().find(|&&at| selected.contains(&index.items[at].value.get_value())).map(|&at| index.items[at].id.get_value()))
                .or_else(|| index.open.first().map(|&at| index.items[at].id.get_value()))
        }));
        let group = Group { id: StoredValue::new(crate::id::next("choice-group")), kind, orientation, mode, allow_empty, selected, has_selection,
            change, items, index, stop, last, focus_owned: RwSignal::new(false), disabled, read_only, invalid, required, described_by,
            name: StoredValue::new(name), form: StoredValue::new(form), node: NodeRef::new(), content_revision: RwSignal::new(0),
            #[cfg(not(target_arch = "wasm32"))]
            host_owner: StoredValue::new(owner.downgrade()),
            #[cfg(not(target_arch = "wasm32"))]
            host_children: StoredValue::new(Vec::new()),
        };
        super::form::register(group.id, &group.form.get_value(), super::form::ControlSpec {
            composing: Signal::stored(false), commit: None,
            flush: Callback::new(move |_| group.settle()),
            focus: Callback::new(move |_| { if let Some(id) = group.stop.get_untracked() { crate::dom::focus_id(&id); } }),
            owns: Callback::new(move |_element: web_sys::Element| {
                #[cfg(target_arch = "wasm32")] { group.node.try_get_untracked().flatten().is_some_and(|root| root.contains(Some(&_element))) }
                #[cfg(not(target_arch = "wasm32"))] { false }
            }),
        });
        provide_context(group);
        let direction = rustify_ui::use_direction();
        let content = children();
        #[cfg(target_arch = "wasm32")]
        observe_group_order(group, order);
        #[cfg(target_arch = "wasm32")]
        Effect::new(move |_| {
            let current = last.get();
            group.index.track();
            if let Some(current) = current {
                let focused = document().active_element().is_some_and(|active| active.id() == current);
                if focused && group.index.with(|index| !index.positions.contains_key(&current)) {
                    let next = group.index.with_untracked(|index| index.by_id.get(&current).copied().unwrap_or_default() + 1);
                    group.recover_focus(next);
                }
            }
        });
        view! {
            <div node_ref=group.node id=group.id.get_value() class=move || class.get() data-testid=test_id
                data-rustify-part=match kind { GroupKind::Checkbox => "checkbox-group", GroupKind::Radio => "radio-group", GroupKind::Toggle => "toggle-group" }
                data-rustify-group=group.id.get_value() tabindex="-1"
                aria-orientation=(kind != GroupKind::Checkbox).then_some(orientation.attribute())
                data-rustify-issue=move || if invalid_selection.get() { Some("invalid-selection") } else { index.with(|index| index.invalid.then_some("duplicate-value")) }
                data-disabled=move || disabled.get().then_some("") data-readonly=move || read_only.get().then_some("")
                data-invalid=move || invalid.get().then_some("") data-orientation=orientation.attribute()
                dir=move || direction.get().attribute()
                role=if toolbar { "toolbar" } else if kind == GroupKind::Radio { "radiogroup" } else { "group" }
                aria-label=optional(aria_label) aria-required=move || required.get().then_some("true")
                aria-readonly=move || read_only.get().then_some("true")
                on:focusin=move |event| { if let Some(item) = group.event_item(event.as_ref()) {
                    group.last.set(Some(item.id.get_value())); group.focus_owned.set(true);
                } }
                on:focusout=move |event| {
                    #[cfg(target_arch = "wasm32")]
                    {
                        use leptos::wasm_bindgen::JsCast;
                        let inside = event.related_target().and_then(|target| target.dyn_into::<web_sys::Node>().ok())
                            .is_some_and(|target| group.node.get_untracked().is_some_and(|root| root.contains(Some(&target))));
                        if inside { return; }
                        if event.related_target().is_none() {
                            if let Some(item) = group.event_item(event.as_ref()) {
                                let next = index.with_untracked(|index| if index.positions.contains_key(&item.id.get_value()) { None }
                                    else { index.by_id.get(&item.id.get_value()).map(|at| at + 1) });
                                if let Some(next) = next { group.recover_focus(next); return; }
                                // A removal's focusout arrives before the keyed
                                // row is dropped. Let its cleanup transfer focus
                                // before deciding that a null target left the group.
                                let previous = item.id.get_value();
                                rustify_ui::defer(move || {
                                    if group.last.try_get_untracked().flatten().as_deref() != Some(previous.as_str()) { return; }
                                    let inside = group.node.try_get_untracked().flatten().is_some_and(|root| {
                                        document().active_element().is_some_and(|active| root.contains(Some(&active)))
                                    });
                                    if !inside { let _ = group.focus_owned.try_set(false); }
                                });
                                return;
                            }
                        }
                        group.focus_owned.set(false);
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    let _ = event;
                }
                on:click=move |event| {
                    if group.event_item(event.as_ref()).is_some() && (disabled.get_untracked() || read_only.get_untracked()) {
                        event.prevent_default(); group.settle();
                    }
                }
                on:change=move |event| {
                    if let Some(item) = group.event_item(event.as_ref()) {
                        if !event.default_prevented() {
                            if let Some(native) = item.native { native.request(); }
                        }
                        group.settle();
                    }
                }
                on:keydown=move |event| {
                    if event.default_prevented() || event.is_composing() || disabled.get_untracked() || kind == GroupKind::Checkbox { return; }
                    let Some(item) = group.event_item(event.as_ref()) else { return; };
                    let key = event.key();
                    let step = match (orientation, key.as_str()) {
                        (Orientation::Horizontal, "ArrowLeft") => Some(if direction.get_untracked() == rustify_ui::Direction::RightToLeft { 1 } else { -1 }),
                        (Orientation::Horizontal, "ArrowRight") => Some(if direction.get_untracked() == rustify_ui::Direction::RightToLeft { -1 } else { 1 }),
                        (Orientation::Vertical, "ArrowUp") => Some(-1), (Orientation::Vertical, "ArrowDown") => Some(1), _ => None,
                    };
                    let next = index.with_untracked(|index| {
                        let here = index.positions.get(&item.id.get_value()).copied();
                        let next = if key == "Home" { Some(0) } else if key == "End" { index.open.len().checked_sub(1) }
                            else { step.and_then(|step| navigation(index.open.len(), here, step, looping)) }?;
                        index.open.get(next).map(|&at| index.items[at])
                    });
                    if let Some(next) = next {
                        event.prevent_default(); last.set(Some(next.id.get_value()));
                        crate::dom::focus_id(&next.id.get_value());
                        if kind == GroupKind::Radio { group.choose(&next.value.get_value()); group.settle(); }
                    }
                }
            >{content}
                <Show when=move || kind == GroupKind::Toggle && required.get()>
                    <input type="checkbox" data-rustify-part="toggle-constraint" tabindex="-1" aria-hidden="true"
                        data-rustify-form-control=group.id.get_value()
                        form=optional(group.form.get_value()) checked=move || has_selection.get() prop:checked=move || has_selection.get()
                        required=true disabled=move || disabled.get() || index.with(|index| index.open.is_empty())
                        on:invalid=move |event| { event.prevent_default(); if let Some(stop) = stop.get_untracked() { crate::dom::focus_id(&stop); } } />
                </Show>
            </div>
        }
    });
    leptos::tachys::reactive_graph::OwnedView::new_with_owner(view, owner)
}
fn navigation(count: usize, here: Option<usize>, step: i32, looping: bool) -> Option<usize> {
    if count == 0 {
        return None;
    }
    let Some(here) = here else {
        return Some(if step < 0 { count - 1 } else { 0 });
    };
    let next = here as i32 + step;
    if looping {
        Some(next.rem_euclid(count as i32) as usize)
    } else {
        Some(next.clamp(0, count as i32 - 1) as usize)
    }
}

struct ItemOptions {
    value: String,
    disabled: Signal<bool>,
    id: String,
    aria_label: String,
    described_by: Signal<String>,
    labelled_by: String,
    class: String,
    control_class: String,
    test_id: String,
}
fn group_item(options: ItemOptions, children: Option<Children>) -> impl IntoView {
    let ItemOptions {
        value,
        disabled,
        id,
        aria_label,
        described_by,
        labelled_by,
        class,
        control_class,
        test_id,
    } = options;
    let owner = Owner::new();
    let view = owner.with(|| {
        let Some(group) = use_context::<Group>() else {
            return view! { <span class=class data-rustify-issue="missing-root">{children.map(|children| children())}</span> }.into_any();
        };
        let mut native = group.native(value, disabled, id, aria_label, described_by);
        native.labelled_by = StoredValue::new(optional(labelled_by));
        provide_context(native);
        view! {
            <label class=crate::macros::reactive_merge("", &class) data-rustify-part="choice-item" data-state=move || native.state.get().as_str()
                data-disabled=move || native.disabled.get().then_some("")
                data-readonly=move || native.read_only.get().then_some("")
                data-invalid=move || native.invalid.get().then_some("")>
                <Control class=control_class test_id=test_id />
                {children.map(|children| children())}
            </label>
        }.into_any()
    });
    leptos::tachys::reactive_graph::OwnedView::new_with_owner(view, owner)
}

impl PartialEq for GroupIndex {
    fn eq(&self, other: &Self) -> bool {
        self.open == other.open
            && self.invalid == other.invalid
            && self.order == other.order
            && self
                .items
                .iter()
                .map(|item| item.id)
                .eq(other.items.iter().map(|item| item.id))
    }
}
fn validate_selection(values: &[String], mode: SelectionMode) -> Result<(), PropError> {
    if mode == SelectionMode::Single
        && values
            .first()
            .is_some_and(|first| values.iter().skip(1).any(|value| value != first))
    {
        Err(PropError {
            field: "value",
            message: "single choice expects at most one value".into(),
        })
    } else {
        Ok(())
    }
}
fn validate_keys(values: &[String]) -> Result<(), PropError> {
    let mut keys = HashSet::new();
    if values.iter().all(|value| keys.insert(value)) {
        Ok(())
    } else {
        Err(PropError {
            field: "value",
            message: "choice keys must be unique".into(),
        })
    }
}

/// Native checkbox parts. Values are always application-owned.
pub mod checkbox {
    use super::*;
    pub use super::{Control, Indicator};
    /// Establishes accepted state, form attributes and delegated input behavior.
    #[component]
    pub fn Root(
        #[prop(into)] checked: Signal<CheckedState>,
        #[prop(into)] on_change: Callback<CheckedState>,
        #[prop(optional, into)] id: String,
        #[prop(optional, into)] name: String,
        #[prop(default = "on".into(), into)] value: String,
        #[prop(optional, into)] form: String,
        #[prop(optional, into)] aria_label: String,
        #[prop(optional, into)] labelled_by: String,
        #[prop(optional, into)] described_by: Signal<String>,
        #[prop(optional, into)] disabled: Signal<bool>,
        #[prop(optional, into)] read_only: Signal<bool>,
        #[prop(optional, into)] invalid: Signal<bool>,
        #[prop(optional, into)] required: Signal<bool>,
        #[prop(optional, into)] class: Signal<String>,
        children: Children,
    ) -> impl IntoView {
        let state = checked;
        let change = on_change;
        native_root(
            NativeOptions {
                kind: Kind::Checkbox,
                state,
                change,
                id,
                name,
                value,
                form,
                aria_label,
                labelled_by,
                described_by,
                disabled,
                read_only,
                invalid,
                required,
                class,
            },
            children,
        )
    }
    /// Adds this Root's control attributes to one native input; caller listeners run first.
    pub fn control_as<At, Ch>(
        target: leptos::html::HtmlElement<leptos::html::Input, At, Ch>,
    ) -> impl IntoView
    where
        At: leptos::attr::Attribute + Send,
        Ch: RenderHtml + Send,
    {
        native_control_as(
            target,
            use_context::<Native>(),
            String::new(),
            String::new(),
        )
    }
}

/// Native radio parts. Values are always application-owned.
pub mod radio {
    use super::*;
    pub use super::{Control, Indicator};
    /// Establishes accepted state, form attributes and delegated input behavior.
    #[component]
    pub fn Root(
        #[prop(into)] checked: Signal<bool>,
        #[prop(into)] on_change: Callback<bool>,
        #[prop(optional, into)] id: String,
        #[prop(optional, into)] name: String,
        #[prop(default = "on".into(), into)] value: String,
        #[prop(optional, into)] form: String,
        #[prop(optional, into)] aria_label: String,
        #[prop(optional, into)] labelled_by: String,
        #[prop(optional, into)] described_by: Signal<String>,
        #[prop(optional, into)] disabled: Signal<bool>,
        #[prop(optional, into)] read_only: Signal<bool>,
        #[prop(optional, into)] invalid: Signal<bool>,
        #[prop(optional, into)] required: Signal<bool>,
        #[prop(optional, into)] class: Signal<String>,
        children: Children,
    ) -> impl IntoView {
        let state = Signal::derive(move || CheckedState::from(checked.get()));
        let change = Callback::new(move |state| on_change.run(state == CheckedState::Checked));
        native_root(
            NativeOptions {
                kind: Kind::Radio,
                state,
                change,
                id,
                name,
                value,
                form,
                aria_label,
                labelled_by,
                described_by,
                disabled,
                read_only,
                invalid,
                required,
                class,
            },
            children,
        )
    }
    /// Adds this Root's control attributes to one native input; caller listeners run first.
    pub fn control_as<At, Ch>(
        target: leptos::html::HtmlElement<leptos::html::Input, At, Ch>,
    ) -> impl IntoView
    where
        At: leptos::attr::Attribute + Send,
        Ch: RenderHtml + Send,
    {
        native_control_as(
            target,
            use_context::<Native>(),
            String::new(),
            String::new(),
        )
    }
}

/// Native switch parts. Values are always application-owned.
pub mod switch {
    use super::*;
    pub use super::{Control, Indicator as Thumb};
    /// Establishes accepted state, form attributes and delegated input behavior.
    #[component]
    pub fn Root(
        #[prop(into)] checked: Signal<bool>,
        #[prop(into)] on_change: Callback<bool>,
        #[prop(optional, into)] id: String,
        #[prop(optional, into)] name: String,
        #[prop(default = "on".into(), into)] value: String,
        #[prop(optional, into)] form: String,
        #[prop(optional, into)] aria_label: String,
        #[prop(optional, into)] labelled_by: String,
        #[prop(optional, into)] described_by: Signal<String>,
        #[prop(optional, into)] disabled: Signal<bool>,
        #[prop(optional, into)] read_only: Signal<bool>,
        #[prop(optional, into)] invalid: Signal<bool>,
        #[prop(optional, into)] required: Signal<bool>,
        #[prop(optional, into)] class: Signal<String>,
        children: Children,
    ) -> impl IntoView {
        let state = Signal::derive(move || CheckedState::from(checked.get()));
        let change = Callback::new(move |state| on_change.run(state == CheckedState::Checked));
        native_root(
            NativeOptions {
                kind: Kind::Switch,
                state,
                change,
                id,
                name,
                value,
                form,
                aria_label,
                labelled_by,
                described_by,
                disabled,
                read_only,
                invalid,
                required,
                class,
            },
            children,
        )
    }
    /// Adds this Root's control attributes to one native input; caller listeners run first.
    pub fn control_as<At, Ch>(
        target: leptos::html::HtmlElement<leptos::html::Input, At, Ch>,
    ) -> impl IntoView
    where
        At: leptos::attr::Attribute + Send,
        Ch: RenderHtml + Send,
    {
        native_control_as(
            target,
            use_context::<Native>(),
            String::new(),
            String::new(),
        )
    }
}

/// Composable native radio group parts.
pub mod radio_group {
    pub use super::Indicator;
    use super::*;
    /// Provides a private group identity independently of the optional native name.
    #[component]
    pub fn Root(
        #[prop(into)] value: Signal<Option<String>>,
        #[prop(into)] on_change: Callback<Option<String>>,
        #[prop(optional, into)] name: String,
        #[prop(optional, into)] form: String,
        #[prop(optional, into)] disabled: Signal<bool>,
        #[prop(optional, into)] read_only: Signal<bool>,
        #[prop(optional, into)] invalid: Signal<bool>,
        #[prop(optional, into)] required: Signal<bool>,
        #[prop(optional, into)] described_by: Signal<String>,
        #[prop(optional, into)] aria_label: String,
        #[prop(optional)] orientation: Orientation,
        #[prop(default = true)] r#loop: bool,
        #[prop(optional, into)] class: Signal<String>,
        #[prop(optional, into)] test_id: String,
        children: Children,
    ) -> impl IntoView {
        let values = Signal::derive(move || value.get().into_iter().collect());
        let change =
            Callback::new(move |values: Vec<String>| on_change.run(values.into_iter().next()));
        group_root(
            GroupOptions {
                kind: GroupKind::Radio,
                mode: SelectionMode::Single,
                allow_empty: false,
                value: values,
                change,
                name,
                form,
                disabled,
                read_only,
                invalid,
                required,
                described_by,
                aria_label,
                orientation,
                looping: r#loop,
                toolbar: false,
                class,
                test_id,
            },
            children,
        )
    }
    /// One label and its native input. Custom content may include an Indicator.
    #[component]
    pub fn Item(
        #[prop(into)] value: String,
        #[prop(optional, into)] disabled: Signal<bool>,
        #[prop(optional, into)] id: String,
        #[prop(optional, into)] aria_label: String,
        #[prop(optional, into)] labelled_by: String,
        #[prop(optional, into)] described_by: Signal<String>,
        #[prop(optional, into)] class: String,
        #[prop(optional, into)] control_class: String,
        #[prop(optional, into)] test_id: String,
        #[prop(optional)] children: Option<Children>,
    ) -> impl IntoView {
        group_item(
            ItemOptions {
                value,
                disabled,
                id,
                aria_label,
                described_by,
                labelled_by,
                class,
                control_class,
                test_id,
            },
            children,
        )
    }
    /// Uses a caller-supplied native input as a group item, preserving its listeners.
    pub fn item_as<At, Ch>(
        target: leptos::html::HtmlElement<leptos::html::Input, At, Ch>,
        attributes: impl Into<ItemAttributes>,
    ) -> impl IntoView
    where
        At: leptos::attr::Attribute + Send,
        Ch: RenderHtml + Send,
    {
        let attributes = attributes.into();
        let native = use_context::<Group>().map(|group| {
            let mut native = group.native(
                attributes.value,
                attributes.disabled,
                attributes.id,
                attributes.aria_label,
                attributes.described_by,
            );
            native.labelled_by = StoredValue::new(optional(attributes.labelled_by));
            native
        });
        native_control_as(target, native, String::new(), String::new())
    }
}

/// Composable native checkbox group parts.
pub mod checkbox_group {
    pub use super::Indicator;
    use super::*;
    /// Provides a private group identity independently of the optional native name.
    #[component]
    pub fn Root(
        #[prop(into)] value: Signal<Vec<String>>,
        #[prop(into)] on_change: Callback<Vec<String>>,
        #[prop(optional, into)] name: String,
        #[prop(optional, into)] form: String,
        #[prop(optional, into)] disabled: Signal<bool>,
        #[prop(optional, into)] read_only: Signal<bool>,
        #[prop(optional, into)] invalid: Signal<bool>,
        #[prop(optional, into)] required: Signal<bool>,
        #[prop(optional, into)] described_by: Signal<String>,
        #[prop(optional, into)] aria_label: String,
        #[prop(optional)] orientation: Orientation,
        #[prop(default = true)] r#loop: bool,
        #[prop(optional, into)] class: Signal<String>,
        #[prop(optional, into)] test_id: String,
        children: Children,
    ) -> impl IntoView {
        let values = value;
        let change = on_change;
        group_root(
            GroupOptions {
                kind: GroupKind::Checkbox,
                mode: SelectionMode::Multiple,
                allow_empty: false,
                value: values,
                change,
                name,
                form,
                disabled,
                read_only,
                invalid,
                required,
                described_by,
                aria_label,
                orientation,
                looping: r#loop,
                toolbar: false,
                class,
                test_id,
            },
            children,
        )
    }
    /// One label and its native input. Custom content may include an Indicator.
    #[component]
    pub fn Item(
        #[prop(into)] value: String,
        #[prop(optional, into)] disabled: Signal<bool>,
        #[prop(optional, into)] id: String,
        #[prop(optional, into)] aria_label: String,
        #[prop(optional, into)] labelled_by: String,
        #[prop(optional, into)] described_by: Signal<String>,
        #[prop(optional, into)] class: String,
        #[prop(optional, into)] control_class: String,
        #[prop(optional, into)] test_id: String,
        #[prop(optional)] children: Option<Children>,
    ) -> impl IntoView {
        group_item(
            ItemOptions {
                value,
                disabled,
                id,
                aria_label,
                described_by,
                labelled_by,
                class,
                control_class,
                test_id,
            },
            children,
        )
    }
    /// Uses a caller-supplied native input as a group item, preserving its listeners.
    pub fn item_as<At, Ch>(
        target: leptos::html::HtmlElement<leptos::html::Input, At, Ch>,
        attributes: impl Into<ItemAttributes>,
    ) -> impl IntoView
    where
        At: leptos::attr::Attribute + Send,
        Ch: RenderHtml + Send,
    {
        let attributes = attributes.into();
        let native = use_context::<Group>().map(|group| {
            let mut native = group.native(
                attributes.value,
                attributes.disabled,
                attributes.id,
                attributes.aria_label,
                attributes.described_by,
            );
            native.labelled_by = StoredValue::new(optional(attributes.labelled_by));
            native
        });
        native_control_as(target, native, String::new(), String::new())
    }
}

struct ToggleOptions {
    pressed: Signal<bool>,
    required: Signal<bool>,
    change: Callback<bool>,
    id: String,
    name: String,
    value: String,
    form: String,
    disabled: Signal<bool>,
    read_only: Signal<bool>,
    invalid: Signal<bool>,
    described_by: Signal<String>,
    aria_label: String,
    labelled_by: String,
    class: Option<Signal<String>>,
    test_id: String,
}
fn toggle_view<At, Ch>(
    target: leptos::html::HtmlElement<leptos::html::Button, At, Ch>,
    options: ToggleOptions,
) -> impl IntoView
where
    At: leptos::attr::Attribute + Send,
    Ch: RenderHtml + Send,
{
    let ToggleOptions {
        pressed,
        required,
        change,
        id,
        name,
        value,
        form,
        disabled,
        read_only,
        invalid,
        described_by,
        aria_label,
        labelled_by,
        class,
        test_id,
    } = options;
    let id = StoredValue::new(crate::form::field::control_id(id, "toggle"));
    let form = super::form::associated_form(form);
    let (invalid, described_by) = crate::form::field::metadata(invalid, described_by);
    let group = use_context::<Group>().filter(|group| group.kind == GroupKind::Toggle);
    let value = StoredValue::new(value);
    let actual_disabled = Signal::derive(move || {
        disabled.get()
            || group.is_some_and(|group| {
                group.disabled.get() || group.index.with(|index| index.invalid)
            })
    });
    let read_only =
        Signal::derive(move || read_only.get() || group.is_some_and(|group| group.read_only.get()));
    let invalid =
        Signal::derive(move || invalid.get() || group.is_some_and(|group| group.invalid.get()));
    let name = optional(if let Some(group) = group {
        group.name.get_value()
    } else {
        name
    });
    let form = optional(if let Some(group) = group {
        group.form.get_value()
    } else {
        form
    });
    if let Some(group) = group {
        group.register(GroupItem {
            id,
            value,
            disabled,
            native: None,
        });
    }
    let direction = rustify_ui::use_direction();
    let named = name.is_some();
    let constraint_form = form.clone();
    let boundary = NodeRef::<leptos::html::Span>::new();
    let control_key = group.map_or(id, |group| group.id);
    if group.is_none() {
        super::form::register(
            control_key,
            form.as_deref().unwrap_or_default(),
            super::form::ControlSpec {
                composing: Signal::stored(false),
                commit: None,
                flush: Callback::new(|_| {}),
                focus: Callback::new(move |_| crate::dom::focus_id(&id.get_value())),
                owns: Callback::new(move |_element: web_sys::Element| {
                    #[cfg(target_arch = "wasm32")]
                    {
                        boundary
                            .try_get_untracked()
                            .flatten()
                            .is_some_and(|root| root.contains(Some(&_element)))
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    {
                        false
                    }
                }),
            },
        );
    }
    view! {
        <span node_ref=boundary data-rustify-part="toggle-boundary" on:click=move |event| {
            use leptos::wasm_bindgen::JsCast;
            let owns = event.target().and_then(|target| target.dyn_into::<web_sys::Element>().ok())
                .and_then(|target| target.closest("[data-rustify-part=toggle-root]").ok().flatten())
                .is_some_and(|target| target.id() == id.get_value());
            if !owns || event.default_prevented() { return; }
            if actual_disabled.get_untracked() || read_only.get_untracked() { event.prevent_default(); return; }
            if let Some(group) = group { group.choose(&value.get_value()); }
            else { change.run(!pressed.get_untracked()); }
        }>
            {super::Slot(target, (
                (leptos::attr::r#type("button"), leptos::attr::id(id.get_value()), leptos::attr::form(form.clone()),
                leptos::attr::custom::custom_attribute("data-rustify-form-control", control_key.get_value()),
                optional_attribute(class.map(|class| leptos::tachys::html::class::class(move || class.get()))),
                optional_attribute((!test_id.is_empty()).then(|| leptos::attr::custom::custom_attribute("data-testid", test_id))),
                leptos::attr::custom::custom_attribute("aria-label", optional(aria_label)),
                leptos::attr::custom::custom_attribute("aria-labelledby", optional(labelled_by)),
                leptos::attr::custom::custom_attribute("aria-pressed", move || pressed.get().to_string()),
                leptos::attr::custom::custom_attribute("aria-readonly", move || read_only.get().then_some("true")),
                leptos::attr::custom::custom_attribute("aria-invalid", move || invalid.get().then_some("true")),
                leptos::attr::custom::custom_attribute("aria-describedby", move || optional(described_by.get()))),
                (leptos::attr::custom::custom_attribute("data-state", move || if pressed.get() { "on" } else { "off" }),
                leptos::attr::custom::custom_attribute("data-disabled", move || actual_disabled.get().then_some("")),
                leptos::attr::custom::custom_attribute("data-readonly", move || read_only.get().then_some("")),
                leptos::attr::custom::custom_attribute("data-invalid", move || invalid.get().then_some("")),
                leptos::attr::custom::custom_attribute("data-rustify-group-id", group.map(|group| group.id.get_value())),
                leptos::attr::custom::custom_attribute("tabindex", move || group.and_then(|group| group.tabindex(id))),
                leptos::attr::custom::custom_attribute("dir", move || direction.get().attribute()),
                leptos::attr::disabled(move || actual_disabled.get()),
                leptos::tachys::html::property::prop("disabled", move || actual_disabled.get()),
                leptos::attr::custom::custom_attribute("data-rustify-part", "toggle-root"))))}

            <Show when=move || required.get()>
                <input type="checkbox" data-rustify-part="toggle-constraint" tabindex="-1" aria-hidden="true"
                    data-rustify-form-control=control_key.get_value()
                    form=constraint_form.clone() checked=move || pressed.get() prop:checked=move || pressed.get()
                    required=true disabled=move || actual_disabled.get()
                    on:invalid=move |event| { event.prevent_default(); crate::dom::focus_id(&id.get_value()); } />
            </Show>
            <Show when=move || named && pressed.get() && !actual_disabled.get()>
                <input type="hidden" name=name.clone() value=value.get_value() form=form.clone() data-rustify-form-control=control_key.get_value() />
            </Show>
        </span>
    }
}

/// A controlled native toggle button and its optional accepted form value.
pub mod toggle {
    use super::*;
    #[component]
    pub fn Root(
        #[prop(into)] pressed: Signal<bool>,
        #[prop(optional, into)] required: Signal<bool>,
        #[prop(into)] on_change: Callback<bool>,
        #[prop(optional, into)] id: String,
        #[prop(optional, into)] name: String,
        #[prop(default = "on".into(), into)] value: String,
        #[prop(optional, into)] form: String,
        #[prop(optional, into)] disabled: Signal<bool>,
        #[prop(optional, into)] read_only: Signal<bool>,
        #[prop(optional, into)] invalid: Signal<bool>,
        #[prop(optional, into)] described_by: Signal<String>,
        #[prop(optional, into)] aria_label: String,
        #[prop(optional, into)] labelled_by: String,
        #[prop(optional, into)] class: Signal<String>,
        #[prop(optional, into)] test_id: String,
        children: Children,
    ) -> impl IntoView {
        toggle_view(
            leptos::html::button().child(children()),
            ToggleOptions {
                pressed,
                required,
                change: on_change,
                id,
                name,
                value,
                form,
                disabled,
                read_only,
                invalid,
                described_by,
                aria_label,
                labelled_by,
                class: Some(class),
                test_id,
            },
        )
    }
}

/// Composable toggle buttons; arrows move focus without changing selection.
pub mod toggle_group {
    use super::*;
    #[component]
    pub fn Root(
        #[prop(into)] value: Signal<Vec<String>>,
        #[prop(into)] on_change: Callback<Vec<String>>,
        #[prop(optional)] mode: SelectionMode,
        #[prop(default = true)] allow_empty: bool,
        #[prop(optional, into)] name: String,
        #[prop(optional, into)] form: String,
        #[prop(optional, into)] disabled: Signal<bool>,
        #[prop(optional, into)] read_only: Signal<bool>,
        #[prop(optional, into)] invalid: Signal<bool>,
        #[prop(optional, into)] required: Signal<bool>,
        #[prop(optional, into)] described_by: Signal<String>,
        #[prop(optional, into)] aria_label: String,
        #[prop(optional)] orientation: Orientation,
        #[prop(default = true)] r#loop: bool,
        #[prop(optional)] toolbar: bool,
        #[prop(optional, into)] class: Signal<String>,
        #[prop(optional, into)] test_id: String,
        children: Children,
    ) -> impl IntoView {
        group_root(
            GroupOptions {
                kind: GroupKind::Toggle,
                mode,
                allow_empty,
                value,
                change: on_change,
                name,
                form,
                disabled,
                read_only,
                invalid,
                required,
                described_by,
                aria_label,
                orientation,
                looping: r#loop,
                toolbar,
                class,
                test_id,
            },
            children,
        )
    }
    #[component]
    pub fn Item(
        #[prop(into)] value: String,
        #[prop(optional, into)] id: String,
        #[prop(optional, into)] aria_label: String,
        #[prop(optional, into)] labelled_by: String,
        #[prop(optional, into)] described_by: Signal<String>,
        #[prop(optional, into)] disabled: Signal<bool>,
        #[prop(optional, into)] class: Signal<String>,
        #[prop(optional, into)] test_id: String,
        children: Children,
    ) -> impl IntoView {
        item_as_inner(
            leptos::html::button().child(children()),
            ItemAttributes {
                value,
                disabled,
                id,
                aria_label,
                labelled_by,
                described_by,
            },
            Some(class),
            test_id,
        )
    }
    fn item_as_inner<At, Ch>(
        target: leptos::html::HtmlElement<leptos::html::Button, At, Ch>,
        attributes: ItemAttributes,
        class: Option<Signal<String>>,
        test_id: String,
    ) -> impl IntoView
    where
        At: leptos::attr::Attribute + Send,
        Ch: RenderHtml + Send,
    {
        let Some(group) = use_context::<Group>().filter(|group| group.kind == GroupKind::Toggle)
        else {
            return super::super::Slot(
                target,
                (
                    leptos::attr::custom::custom_attribute("data-rustify-issue", "missing-root"),
                    leptos::attr::disabled(true),
                ),
            )
            .into_any();
        };
        let ItemAttributes {
            value,
            disabled,
            id,
            aria_label,
            labelled_by,
            described_by,
        } = attributes;
        let held = StoredValue::new(value.clone());
        let pressed = Signal::derive(move || {
            group
                .selected
                .with(|selected| selected.contains(&held.get_value()))
        });
        toggle_view(
            target,
            ToggleOptions {
                pressed,
                required: Signal::stored(false),
                change: Callback::new(|_| {}),
                id,
                name: String::new(),
                value,
                form: String::new(),
                disabled,
                read_only: group.read_only,
                invalid: group.invalid,
                described_by: Signal::derive(move || {
                    merge_ids(&[&group.described_by.get(), &described_by.get()])
                }),
                aria_label,
                labelled_by,
                class,
                test_id,
            },
        )
        .into_any()
    }
    /// Preserves a caller's native button and target listeners.
    pub fn item_as<At, Ch>(
        target: leptos::html::HtmlElement<leptos::html::Button, At, Ch>,
        attributes: impl Into<ItemAttributes>,
    ) -> impl IntoView
    where
        At: leptos::attr::Attribute + Send,
        Ch: RenderHtml + Send,
    {
        item_as_inner(target, attributes.into(), None, String::new())
    }
}

pub(crate) fn use_checked_state() -> Signal<CheckedState> {
    use_context::<Native>().map_or_else(
        || Signal::stored(CheckedState::Unchecked),
        |native| native.state,
    )
}

pub(crate) fn group_content_revision() -> Signal<u64> {
    use_context::<Group>().map_or_else(|| Signal::stored(0), |group| group.content_revision.into())
}

#[cfg(target_arch = "wasm32")]
fn observe_group_order(group: Group, order: RwSignal<Vec<String>>) {
    use leptos::wasm_bindgen::{closure::Closure, JsCast};
    Effect::new(move |_| {
        let Some(root) = group.node.get() else { return };
        let root: web_sys::Element = root.into();
        let target = root.clone();
        let collect = move || {
            let mut ids = Vec::new();
            if let Ok(nodes) = root.query_selector_all("[data-rustify-group-id]") {
                for at in 0..nodes.length() {
                    if let Some(element) = nodes
                        .item(at)
                        .and_then(|node| node.dyn_into::<web_sys::Element>().ok())
                    {
                        if element.get_attribute("data-rustify-group-id").as_deref()
                            == Some(group.id.read_value().as_str())
                        {
                            ids.push(element.id());
                        }
                    }
                }
            }
            if order.get_untracked() != ids {
                order.set(ids);
            }
        };
        let callback =
            Closure::<dyn FnMut(js_sys::Array, web_sys::MutationObserver)>::new(move |_, _| {
                collect();
                group.content_revision.update(|revision| *revision += 1);
            });
        let Ok(observer) = web_sys::MutationObserver::new(callback.as_ref().unchecked_ref()) else {
            return;
        };
        let options = web_sys::MutationObserverInit::new();
        options.set_child_list(true);
        options.set_subtree(true);
        if observer.observe_with_options(&target, &options).is_err() {
            return;
        }
        // The browser disconnect method still works when the owning wasm instance has trapped.
        let abort = js_sys::Reflect::get(&observer, &"disconnect".into())
            .ok()
            .and_then(|method| method.dyn_into::<js_sys::Function>().ok())
            .map(|disconnect| rustify_ui::release_on_abort(disconnect.bind0(&observer)));
        let held = StoredValue::new_local(Some((observer, callback, abort)));
        on_cleanup(move || {
            held.update_value(|held| {
                if let Some((observer, _, _)) = held.take() {
                    observer.disconnect();
                }
            })
        });
        // Initial children are already mounted by the time effects run.
        if let Ok(nodes) = target.query_selector_all("[data-rustify-group-id]") {
            let ids = (0..nodes.length())
                .filter_map(|at| nodes.item(at))
                .filter_map(|node| node.dyn_into::<web_sys::Element>().ok())
                .filter(|node| {
                    node.get_attribute("data-rustify-group-id").as_deref()
                        == Some(group.id.read_value().as_str())
                })
                .map(|node| node.id())
                .collect::<Vec<_>>();
            if order.get_untracked() != ids {
                order.set(ids);
            }
        }
    });
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

#[cfg(test)]
mod tests {
    use super::*;
    fn values(items: &[&str]) -> Vec<String> {
        items.iter().map(|item| (*item).into()).collect()
    }
    #[test]
    fn mixed_activation_requests_checked_and_checked_requests_unchecked() {
        assert_eq!(
            CheckedState::Indeterminate.activated(),
            CheckedState::Checked
        );
        assert_eq!(CheckedState::Checked.activated(), CheckedState::Unchecked);
    }
    #[test]
    fn multiple_requests_deduplicate_and_keep_declaration_order() {
        let selected = values(&["c", "c"]).into_iter().collect();
        assert_eq!(
            selection_request(
                &selected,
                &values(&["a", "b", "c"]),
                "a",
                SelectionMode::Multiple,
                true
            ),
            Some(values(&["a", "c"]))
        );
        assert_eq!(
            selection_request(
                &selected,
                &values(&["a", "b", "c"]),
                "c",
                SelectionMode::Multiple,
                true
            ),
            Some(vec![])
        );
    }
    #[test]
    fn required_single_selection_cannot_be_implicitly_cleared() {
        let selected = values(&["a"]).into_iter().collect();
        assert_eq!(
            selection_request(
                &selected,
                &values(&["a", "b"]),
                "a",
                SelectionMode::Single,
                false
            ),
            None
        );
        assert_eq!(
            selection_request(
                &selected,
                &values(&["a", "b"]),
                "b",
                SelectionMode::Single,
                false
            ),
            Some(values(&["b"]))
        );
    }
    #[test]
    fn duplicate_keys_report_only_the_property_name() {
        assert_eq!(
            validate_keys(&values(&["a", "a"])).unwrap_err().field,
            "value"
        );
    }
    #[test]
    fn navigation_can_wrap_or_stop_at_an_edge() {
        assert_eq!(navigation(3, Some(2), 1, false), Some(2));
        assert_eq!(navigation(3, Some(2), 1, true), Some(0));
        assert_eq!(navigation(0, None, 1, true), None);
    }
}
