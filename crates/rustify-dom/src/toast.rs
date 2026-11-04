//! Controlled notification parts and the scope's single scheduling source.

#[cfg(target_arch = "wasm32")]
mod browser;
mod model;

use crate::overlay::{Cancelable, LayerDismiss};
use crate::shortcut::{Keystroke, Modifiers};
use leptos::prelude::*;
use model::{Events, Id, Pause, Scheduler};
use std::collections::BTreeMap;

const DEFAULT_DURATION_MS: u32 = 3200;
const MAX_DURATION_MS: u32 = i32::MAX as u32;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ToastTone {
    #[default]
    Neutral,
    Success,
    Warning,
    Error,
}
impl ToastTone {
    pub fn key(self) -> &'static str {
        match self {
            Self::Neutral => "neutral",
            Self::Success => "success",
            Self::Warning => "warning",
            Self::Error => "error",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ToastImportance {
    #[default]
    Background,
    Foreground,
}
impl ToastImportance {
    pub const ALL: &'static [Self] = &[Self::Background, Self::Foreground];
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Background => "background",
            Self::Foreground => "foreground",
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ToastQueuePolicy {
    #[default]
    Replace,
    Fifo,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SwipeDirection {
    Up,
    Down,
    Left,
    #[default]
    Right,
}
impl SwipeDirection {
    pub const ALL: &'static [Self] = &[Self::Up, Self::Down, Self::Left, Self::Right];
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Up => "up",
            Self::Down => "down",
            Self::Left => "left",
            Self::Right => "right",
        }
    }
    #[cfg(target_arch = "wasm32")]
    fn distance(self, x: f64, y: f64) -> f64 {
        match self {
            Self::Up => -y,
            Self::Down => y,
            Self::Left => -x,
            Self::Right => x,
        }
    }
    #[cfg(target_arch = "wasm32")]
    fn offset(self, x: f64, y: f64) -> (f64, f64) {
        match self {
            Self::Up => (0., y.min(0.)),
            Self::Down => (0., y.max(0.)),
            Self::Left => (x.min(0.), 0.),
            Self::Right => (x.max(0.), 0.),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ToastSwipe {
    pub direction: SwipeDirection,
    pub delta_x: f64,
    pub delta_y: f64,
}

#[derive(Clone)]
pub struct ToastAction {
    pub label: String,
    pub alt_text: String,
    pub on_action: Callback<Cancelable<()>>,
}

#[derive(Clone)]
pub struct ToastOptions {
    pub duration_ms: u32,
    pub tone: ToastTone,
    pub importance: ToastImportance,
    pub title: Option<String>,
    pub action: Option<ToastAction>,
}
impl Default for ToastOptions {
    fn default() -> Self {
        Self {
            duration_ms: DEFAULT_DURATION_MS,
            tone: ToastTone::Neutral,
            importance: ToastImportance::Background,
            title: None,
            action: None,
        }
    }
}

/// An accepted handle message. Its Root owns the same scheduling session as
/// a directly declared notification; callbacks remain in memory only.
#[derive(Clone)]
pub struct Toast {
    pub seq: u64,
    pub message: String,
    pub duration_ms: u32,
    pub tone: ToastTone,
    pub importance: ToastImportance,
    pub title: Option<String>,
    pub action: Option<ToastAction>,
}

#[derive(Clone, Debug)]
pub struct ToastConfig {
    pub policy: ToastQueuePolicy,
    pub visible_limit: usize,
    pub swipe_direction: SwipeDirection,
    pub swipe_threshold: f64,
    pub hotkey: Keystroke<'static>,
    pub viewport_label: String,
}
impl Default for ToastConfig {
    fn default() -> Self {
        Self {
            policy: ToastQueuePolicy::Replace,
            visible_limit: 1,
            swipe_direction: SwipeDirection::Right,
            swipe_threshold: 50.,
            hotkey: Keystroke {
                key: "F8",
                code: "F8",
                modifiers: Modifiers::default(),
            },
            viewport_label: String::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ToastIssue {
    pub field: &'static str,
    pub message: String,
}
impl ToastIssue {
    fn duration(value: u32) -> Self {
        Self {
            field: "duration_ms",
            message: format!("expected 0 through {MAX_DURATION_MS} milliseconds, received {value}"),
        }
    }
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
#[derive(Clone, Copy)]
struct Registration {
    id: Id,
    node: NodeRef<leptos::html::Div>,
    identity: StoredValue<String>,
    close: Callback<bool>,
    importance: Signal<ToastImportance>,
    escape: Option<Callback<Cancelable<LayerDismiss>>>,
    pause: Option<Callback<()>>,
    resume: Option<Callback<()>>,
    swipe_start: Option<Callback<Cancelable<ToastSwipe>>>,
    swipe_move: Option<Callback<Cancelable<ToastSwipe>>>,
    swipe_cancel: Option<Callback<Cancelable<ToastSwipe>>>,
    swipe_end: Option<Callback<Cancelable<ToastSwipe>>>,
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
#[derive(Clone, Copy)]
struct Runtime {
    model: RwSignal<Scheduler>,
    registrations: RwSignal<Vec<Registration>>,
    issues: RwSignal<BTreeMap<String, ToastIssue>>,
    pending_announcements: RwSignal<Vec<Id>>,
    polite: RwSignal<Vec<(u64, String)>>,
    assertive: RwSignal<Vec<(u64, String)>>,
    announcement_seq: RwSignal<u64>,
    #[cfg(target_arch = "wasm32")]
    client: StoredValue<browser::Client, leptos::reactive::owner::LocalStorage>,
    #[cfg(target_arch = "wasm32")]
    overlay: StoredValue<Option<crate::overlay::OverlayStack>>,
    #[cfg(target_arch = "wasm32")]
    viewport: RwSignal<Option<StoredValue<NodeRef<leptos::html::Div>>>>,
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
impl Runtime {
    fn issue(self, source: String, issue: Option<ToastIssue>) {
        self.issues.try_update(|issues| {
            if let Some(issue) = issue {
                issues.insert(source, issue);
            } else {
                issues.remove(&source);
            }
        });
    }
    fn registration(self, id: Id) -> Option<Registration> {
        self.registrations
            .try_with_untracked(|roots| roots.iter().find(|root| root.id == id).copied())
            .flatten()
    }
    fn dispatch(self, events: Events) {
        if !events.announce.is_empty() {
            self.pending_announcements
                .try_update(|pending| pending.extend(events.announce));
        }
        #[cfg(target_arch = "wasm32")]
        for id in events.close {
            if let Some(root) = self.registration(id) {
                root.close.run(false);
            }
        }
        #[cfg(target_arch = "wasm32")]
        self.flush_announcements();
    }
    fn sync(self, id: Id, open: bool, duration: u32) {
        #[cfg(target_arch = "wasm32")]
        let held_focus = self.holds_focus(id);
        if let Some((events, issue)) = self
            .model
            .try_update(|model| model.sync(id, open, duration, now()))
        {
            self.issue(format!("root-{id}-duration"), issue);
            self.dispatch(events);
        }
        #[cfg(target_arch = "wasm32")]
        if !open {
            self.cancel_swipe(id);
            if held_focus {
                self.focus_after(id);
            }
        }
    }
    fn request_close(self, id: Id) {
        self.model.try_update(|model| model.requested(id, now()));
        if let Some(root) = self.registration(id) {
            root.close.run(false);
        }
    }
    fn pause(self, id: Option<Id>, reason: Pause, active: bool) {
        let changed = self
            .model
            .try_with_untracked(|model| model.paused_by(id, reason) != active)
            .unwrap_or(false);
        if !changed {
            return;
        }
        let before = self
            .model
            .try_with_untracked(|model| {
                model
                    .visible_ids()
                    .into_iter()
                    .map(|id| (id, model.paused(id)))
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();
        if let Some(events) = self
            .model
            .try_update(|model| model.pause(id, reason, active, now()))
        {
            self.dispatch(events);
        }
        for (id, was_paused) in before {
            let paused = self
                .model
                .try_with_untracked(|model| model.paused(id))
                .unwrap_or(was_paused);
            if paused != was_paused {
                if let Some(root) = self.registration(id) {
                    if let Some(callback) = if paused { root.pause } else { root.resume } {
                        callback.run(());
                    }
                }
            }
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn now() -> f64 {
    0.
}
#[cfg(target_arch = "wasm32")]
fn now() -> f64 {
    window()
        .performance()
        .map(|clock| clock.now())
        .unwrap_or_default()
}

#[derive(Clone, Copy)]
pub struct ToastHandle {
    runtime: Runtime,
    messages: RwSignal<Vec<Toast>>,
    seq: RwSignal<u64>,
}

pub fn provide_toasts() -> ToastHandle {
    provide_toasts_with(ToastConfig::default())
}

pub fn provide_toasts_with(config: ToastConfig) -> ToastHandle {
    let (model, issues) = Scheduler::new(config);
    let runtime = Runtime {
        model: RwSignal::new(model),
        registrations: RwSignal::new(Vec::new()),
        issues: RwSignal::new(
            issues
                .into_iter()
                .map(|issue| (format!("config-{}", issue.field), issue))
                .collect(),
        ),
        pending_announcements: RwSignal::new(Vec::new()),
        polite: RwSignal::new(Vec::new()),
        assertive: RwSignal::new(Vec::new()),
        announcement_seq: RwSignal::new(0),
        #[cfg(target_arch = "wasm32")]
        client: StoredValue::new_local(browser::Client::default()),
        #[cfg(target_arch = "wasm32")]
        overlay: StoredValue::new(crate::overlay::use_overlay()),
        #[cfg(target_arch = "wasm32")]
        viewport: RwSignal::new(None),
    };
    let handle = ToastHandle {
        runtime,
        messages: RwSignal::new(Vec::new()),
        seq: RwSignal::new(0),
    };
    provide_context(handle);
    #[cfg(target_arch = "wasm32")]
    browser::install(runtime);
    handle
}

pub fn use_toasts() -> ToastHandle {
    use_context::<ToastHandle>().expect(
        "no toasts in scope; call provide_toasts in the scope root or render toast::Provider",
    )
}

impl ToastHandle {
    pub fn show(&self, message: impl Into<String>, mut options: ToastOptions) -> u64 {
        if options.duration_ms > MAX_DURATION_MS {
            self.runtime.issue(
                "show-duration".into(),
                Some(ToastIssue::duration(options.duration_ms)),
            );
            options.duration_ms = DEFAULT_DURATION_MS;
        } else {
            self.runtime.issue("show-duration".into(), None);
        }
        let Some(seq) = self.seq.try_update(|seq| {
            *seq += 1;
            *seq
        }) else {
            return 0;
        };
        let toast = Toast {
            seq,
            message: message.into(),
            duration_ms: options.duration_ms,
            tone: options.tone,
            importance: options.importance,
            title: options.title,
            action: options.action,
        };
        let policy = self
            .runtime
            .model
            .with_untracked(|model| model.config.policy);
        self.messages.try_update(|messages| {
            if policy == ToastQueuePolicy::Replace {
                messages.clear();
            }
            messages.push(toast);
        });
        seq
    }
    pub fn dismiss(&self, seq: u64) {
        self.messages
            .try_update(|messages| messages.retain(|toast| toast.seq != seq));
    }
    pub fn items(&self) -> Vec<Toast> {
        self.messages.get()
    }
    pub fn current(&self) -> Option<Toast> {
        self.messages.with(|messages| messages.first().cloned())
    }
    pub fn diagnostics(&self) -> Vec<ToastIssue> {
        self.runtime
            .issues
            .with(|issues| issues.values().cloned().collect())
    }
}

#[component]
pub fn Provider(#[prop(optional)] config: ToastConfig, children: Children) -> impl IntoView {
    let owner = Owner::new();
    let view = owner.with(move || {
        provide_toasts_with(config);
        children()
    });
    leptos::tachys::reactive_graph::OwnedView::new_with_owner(view, owner)
}

#[derive(Clone, Copy)]
struct RootContext {
    runtime: Runtime,
    id: Id,
    titles: RwSignal<Vec<String>>,
    descriptions: RwSignal<Vec<String>>,
}

#[component]
pub fn Root(
    #[prop(into)] open: Signal<bool>,
    #[prop(into)] on_open_change: Callback<bool>,
    #[prop(default = Signal::stored(DEFAULT_DURATION_MS), into)] duration_ms: Signal<u32>,
    #[prop(optional, into)] importance: Signal<ToastImportance>,
    #[prop(optional_no_strip, into)] on_escape_key_down: Option<Callback<Cancelable<LayerDismiss>>>,
    #[prop(optional_no_strip, into)] on_pause: Option<Callback<()>>,
    #[prop(optional_no_strip, into)] on_resume: Option<Callback<()>>,
    #[prop(optional_no_strip, into)] on_swipe_start: Option<Callback<Cancelable<ToastSwipe>>>,
    #[prop(optional_no_strip, into)] on_swipe_move: Option<Callback<Cancelable<ToastSwipe>>>,
    #[prop(optional_no_strip, into)] on_swipe_cancel: Option<Callback<Cancelable<ToastSwipe>>>,
    #[prop(optional_no_strip, into)] on_swipe_end: Option<Callback<Cancelable<ToastSwipe>>>,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] test_id: String,
    children: Children,
) -> impl IntoView {
    let owner = Owner::new();
    let view = owner.with(move || {
        let runtime = use_toasts().runtime;
        let id = runtime.model.write().register();
        let node = NodeRef::<leptos::html::Div>::new();
        #[cfg(target_arch = "wasm32")]
        let focused = StoredValue::new(false);
        let dom_id = crate::id::next("toast").unwrap_or_else(|| format!("rustify-toast-{id}"));
        let identity = StoredValue::new(dom_id.clone());
        let context = RootContext { runtime, id, titles: RwSignal::new(Vec::new()), descriptions: RwSignal::new(Vec::new()) };
        provide_context(context);
        runtime.registrations.update(|roots| roots.push(Registration { id, node, identity, close: on_open_change, importance, escape: on_escape_key_down, pause: on_pause, resume: on_resume, swipe_start: on_swipe_start, swipe_move: on_swipe_move, swipe_cancel: on_swipe_cancel, swipe_end: on_swipe_end }));
        runtime.sync(id, open.get_untracked(), duration_ms.get_untracked());
        #[cfg(target_arch = "wasm32")]
        Effect::new(move || {
            let open = open.get();
            let duration = duration_ms.get();
            untrack(move || runtime.sync(id, open, duration));
        });
        #[cfg(target_arch = "wasm32")]
        Effect::new(move || {
            let _ = (context.titles.get(), context.descriptions.get());
            let open = open.get();
            if node.get().is_some() {
                runtime.flush_announcements();
                if open { runtime.reconcile_presence(); }
            }
        });
        on_cleanup(move || {
            #[cfg(target_arch = "wasm32")]
            // Keyed views detach their DOM before this Owner is cleaned up.
            let held_focus = focused.get_value();
            #[cfg(target_arch = "wasm32")]
            {
                runtime.cancel_swipe(id);
                runtime.remove_geometry(id);
            }
            runtime.pending_announcements.try_update(|pending| pending.retain(|pending| *pending != id));
            runtime.issue(format!("root-{id}-duration"), None);
            if let Some(events) = runtime.model.try_update(|model| model.remove(id, now())) { runtime.dispatch(events); }
            #[cfg(target_arch = "wasm32")]
            if held_focus { runtime.focus_after(id); }
            runtime.registrations.try_update(|roots| roots.retain(|root| root.id != id));
        });
        let children = children();
        view! {
            <div node_ref=node id=dom_id class=move || format!("{} {}", identity.get_value(), class.get()) data-testid=test_id
                data-rustify-toast-root="" data-rustify-part="toast-root" role="group" tabindex="-1"
                data-state=move || if open.get() { "open" } else { "closed" }
                data-importance=move || importance.get().as_str()
                data-swipe-direction=move || runtime.model.with(|model| model.config.swipe_direction.as_str())
                aria-labelledby=move || { let ids=context.titles.get(); (!ids.is_empty()).then(|| ids.join(" ")) }
                aria-describedby=move || { let ids=context.descriptions.get(); (!ids.is_empty()).then(|| ids.join(" ")) }
                hidden=move || !open.get() || !runtime.model.with(|model| model.visible(id))
                inert=move || !open.get() || !runtime.model.with(|model| model.visible(id))
                on:focusin=move |event| {
                    let _ = &event;
                    #[cfg(target_arch = "wasm32")]
                    { focused.set_value(true); runtime.focus_enter(&event); }
                }
                on:focusout=move |event| {
                    let _ = &event;
                    #[cfg(target_arch = "wasm32")]
                    {
                        use leptos::wasm_bindgen::JsCast;
                        if let Some(next) = event.related_target().and_then(|target| target.dyn_into::<web_sys::Node>().ok()) {
                            focused.set_value(node.get_untracked().is_some_and(|node| node.contains(Some(&next))));
                        } else {
                            // Chromium blurs a node before removing it. Preserve focus until
                            // cleanup, then clear a surviving Root after an explicit blur.
                            crate::runtime::defer(move || {
                                let within = node.try_get_untracked().flatten().is_some_and(|node| document().active_element().is_some_and(|active| node.contains(Some(&active))));
                                focused.try_update_value(|focused| *focused = within);
                            });
                        }
                    }
                }
                on:pointerdown=move |event| { let _ = &event; #[cfg(target_arch = "wasm32")] runtime.swipe_down(id, &event); }
                on:pointermove=move |event| { let _ = &event; #[cfg(target_arch = "wasm32")] runtime.swipe_move(id, &event); }
                on:pointerup=move |event| { let _ = &event; #[cfg(target_arch = "wasm32")] runtime.swipe_up(id, &event, false); }
                on:pointercancel=move |event| { let _ = &event; #[cfg(target_arch = "wasm32")] runtime.swipe_up(id, &event, true); }>
                {children}
            </div>
        }
    });
    leptos::tachys::reactive_graph::OwnedView::new_with_owner(view, owner)
}

fn part_ids(title: bool) -> Option<(String, RootContext)> {
    let context = use_context::<RootContext>()?;
    let id = crate::id::next(if title {
        "toast-title"
    } else {
        "toast-description"
    })?;
    let ids = if title {
        context.titles
    } else {
        context.descriptions
    };
    ids.update(|ids| ids.push(id.clone()));
    let cleanup_id = id.clone();
    on_cleanup(move || {
        ids.try_update(|ids| ids.retain(|id| id != &cleanup_id));
    });
    Some((id, context))
}

#[component]
pub fn Title(#[prop(optional, into)] class: Signal<String>, children: Children) -> impl IntoView {
    let part = part_ids(true);
    view! { <div id=part.map(|(id, _)| id) class=move || class.get() data-rustify-toast-title="" data-rustify-part="toast-title">{children()}</div> }
}

#[component]
pub fn Description(
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let part = part_ids(false);
    view! { <div id=part.map(|(id, _)| id) class=move || class.get() data-rustify-toast-description="" data-rustify-part="toast-description">{children()}</div> }
}

#[component]
pub fn Action(
    #[prop(into)] alt_text: String,
    #[prop(into)] on_action: Callback<Cancelable<()>>,
    #[prop(optional, into)] class: Signal<String>,
    children: Children,
) -> impl IntoView {
    let context = use_context::<RootContext>();
    let invalid = alt_text.trim().is_empty();
    view! {
        <button type="button" class=move || class.get() data-rustify-part="toast-action" data-rustify-toast-alt=alt_text.clone()
            aria-description=alt_text data-rustify-issue=if context.is_none() { Some("missing-root") } else { invalid.then_some("missing-alt-text") }
            disabled=context.is_none()
            on:click=move |event| {
                if event.default_prevented() { return; }
                let request = Cancelable::new(());
                on_action.run(request.clone());
                if !request.default_prevented() { if let Some(context) = context { context.runtime.request_close(context.id); } }
            }>{children()}</button>
    }
}

#[component]
pub fn Close(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] test_id: String,
    #[prop(optional)] children: Option<Children>,
) -> impl IntoView {
    let context = use_context::<RootContext>();
    let locale = crate::i18n::use_locale();
    let children = children
        .map(|children| children())
        .unwrap_or_else(|| view! { {move || locale.text(crate::Message::Dismiss)} }.into_any());
    view! {
        <button type="button" class=move || class.get() data-testid=test_id data-rustify-part="toast-close"
            aria-label=move || locale.text(crate::Message::Dismiss) disabled=context.is_none() data-rustify-issue=context.is_none().then_some("missing-root")
            on:click=move |event| { if !event.default_prevented() { if let Some(context) = context { context.runtime.request_close(context.id); } } }>{children}</button>
    }
}

#[component]
pub fn Viewport(
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] test_id: String,
    children: ChildrenFn,
) -> impl IntoView {
    let runtime = use_toasts().runtime;
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = (runtime, class, test_id, children);
        ().into_any()
    }
    #[cfg(target_arch = "wasm32")]
    {
        let locale = crate::i18n::use_locale();
        let viewport = NodeRef::<leptos::html::Div>::new();
        let viewport_owner = StoredValue::new(viewport);
        runtime.viewport.set(Some(viewport_owner));
        Effect::new(move || {
            if viewport.get().is_some() {
                runtime.reconcile_presence();
            }
        });
        on_cleanup(move || {
            runtime.viewport.try_set(None);
            runtime.pause(None, Pause::Hover, false);
            runtime.pause(None, Pause::Focus, false);
        });
        let events = crate::overlay::NotificationEvents {
            viewport: viewport_owner,
            on_key_down: Callback::new(move |event| runtime.key(&event)),
            on_host_change: Callback::new(move |()| runtime.reconcile_presence()),
            on_focus_out: Callback::new(move |next| runtime.reconcile_focus(next)),
        };
        view! {
            <crate::overlay::NotificationPortal events=Some(events)>
                <div node_ref=viewport class=move || class.get() data-testid=test_id.clone() data-rustify-part="toast-viewport" role="region"
                    on:pointerenter=move |_| runtime.reconcile_presence()
                    on:pointermove=move |_| runtime.reconcile_presence()
                    on:pointerleave=move |_| runtime.reconcile_presence()
                    on:focusin=move |_| runtime.reconcile_presence()
                    on:focusout=move |event| { use leptos::wasm_bindgen::JsCast; runtime.reconcile_focus(event.related_target().and_then(|target|target.dyn_into::<web_sys::Element>().ok())); }
                    aria-label=move || { let label=runtime.model.with(|model| model.config.viewport_label.clone()); if label.is_empty() { locale.text(crate::Message::Notifications).to_string() } else { label } }>
                    <div data-rustify-toast-announcer="polite" class="rustify-toast-announcer" role="status" aria-live="polite" aria-atomic="false" aria-relevant="additions">
                        <For each=move || runtime.polite.get() key=|(seq,_)| *seq let:item><span>{item.1}</span></For>
                    </div>
                    <div data-rustify-toast-announcer="assertive" class="rustify-toast-announcer" role="status" aria-live="assertive" aria-atomic="false" aria-relevant="additions">
                        <For each=move || runtime.assertive.get() key=|(seq,_)| *seq let:item><span>{item.1}</span></For>
                    </div>
                    {children()}
                </div>
            </crate::overlay::NotificationPortal>
        }.into_any()
    }
}

/// Maps accepted handle messages to the same Root used by declarative parts.
/// `render` draws only the body and must not introduce another Root.
/// Use Title, Description and Action parts for associations and announcements.
#[component]
pub fn ToastRegion(
    #[prop(optional, into)] render: Option<Callback<Toast, AnyView>>,
    #[prop(optional, into)] class: Signal<String>,
    #[prop(optional, into)] test_id: String,
) -> impl IntoView {
    let handle = use_toasts();
    #[cfg(not(target_arch = "wasm32"))]
    {
        #[cfg(feature = "ssr")]
        if !handle.items().is_empty() {
            crate::StaticRenderIssues::record("toasts must start empty; use an inline callout or controlled toast::Root for static content");
        }
        let _ = (handle, render, class, test_id);
        ().into_any()
    }
    #[cfg(target_arch = "wasm32")]
    {
        let dismiss_id = StoredValue::new(format!("{test_id}-dismiss"));
        view! {
            <Viewport class=class test_id=test_id>
                <For each=move || handle.items() key=|toast| toast.seq let:toast>
                    {
                        let seq=toast.seq;
                        let duration_ms=toast.duration_ms;
                        let importance=toast.importance;
                        let tone=toast.tone.key();
                        let dismiss_id=dismiss_id.get_value();
                        view! { <Root open=true on_open_change=move |open: bool| {if !open {handle.dismiss(seq);}}
                            duration_ms=duration_ms importance=importance attr:data-tone=tone>
                            {match render {
                                Some(render)=>render.run(toast.clone()),
                                None=>{
                                    let action=toast.action.clone();
                                    view! {
                                        {toast.title.clone().map(|title| view! { <Title>{title}</Title> })}
                                        <Description>{toast.message.clone()}</Description>
                                        {action.map(|action|view! { <Action alt_text=action.alt_text on_action=action.on_action>{action.label}</Action> })}
                                        <Close test_id=dismiss_id/>
                                    }.into_any()
                                }
                            }}
                        </Root> }

                    }
                </For>
            </Viewport>
        }.into_any()
    }
}

#[cfg(all(test, feature = "ssr", not(target_arch = "wasm32")))]
mod ssg_tests;
