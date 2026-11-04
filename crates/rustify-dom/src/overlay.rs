//! Layers this scope owns: menus, tooltips and dialogs, drawn as DOM above
//! everything else in the scope, its GPU regions included.
//!
//! A layer is a component, so the application decides when one exists. The
//! stack decides the rest: where it sits, what it covers, which layer answers
//! Escape, and where focus goes when it leaves. A modal layer makes the rest
//! of its own scope inert rather than merely painting over it, so a control
//! underneath cannot be clicked, focused or read out while it is open.

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
mod hierarchy;
mod placement;
pub use placement::{
    CollisionPadding, FloatingPosition, FloatingPositionError, PositionUpdate, Sticky,
    ViewportAlign,
};

/// A synchronous semantic event whose clones share one cancellation decision.
#[derive(Clone, Debug)]
pub struct Cancelable<T> {
    value: T,
    prevented: std::sync::Arc<std::sync::atomic::AtomicBool>,
}

impl<T> Cancelable<T> {
    /// Starts a fresh callback chain with its default action enabled.
    pub fn new(value: T) -> Self {
        Self {
            value,
            prevented: Default::default(),
        }
    }

    /// The semantic payload, independent of any browser event type.
    pub fn value(&self) -> &T {
        &self.value
    }

    /// Cancels the default action for every clone in this callback chain.
    pub fn prevent_default(&self) {
        self.prevented
            .store(true, std::sync::atomic::Ordering::Relaxed);
    }

    /// Whether a callback in this chain canceled the default action.
    pub fn default_prevented(&self) -> bool {
        self.prevented.load(std::sync::atomic::Ordering::Relaxed)
    }
}

/// Why the top layer was asked to dismiss.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LayerDismiss {
    Escape,
    PointerOutside,
    FocusOutside,
}
/// The resolved physical side of an anchored layer.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Side {
    Top,
    Right,
    #[default]
    Bottom,
    Left,
}
impl Side {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::Right => "right",
            Self::Bottom => "bottom",
            Self::Left => "left",
        }
    }
}
/// The resolved logical horizontal alignment of an anchored layer.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Align {
    #[default]
    Start,
    Center,
    End,
}
impl Align {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Start => "start",
            Self::Center => "center",
            Self::End => "end",
        }
    }
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LayerPlacement {
    pub side: Side,
    pub align: Align,
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
fn dismiss(
    hook: Option<leptos::prelude::Callback<Cancelable<LayerDismiss>>>,
    reason: LayerDismiss,
    close: impl FnOnce(),
) {
    use leptos::prelude::Callable;
    let event = Cancelable::new(reason);
    if let Some(hook) = hook {
        hook.try_run(event.clone());
    }
    if !event.default_prevented() {
        close();
    }
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
fn dismiss_outside(
    specific: Option<leptos::prelude::Callback<Cancelable<LayerDismiss>>>,
    interact: Option<leptos::prelude::Callback<Cancelable<LayerDismiss>>>,
    reason: LayerDismiss,
    close: impl FnOnce(),
) {
    use leptos::prelude::Callable;
    let event = Cancelable::new(reason);
    if let Some(hook) = specific {
        hook.try_run(event.clone());
    }
    if let Some(hook) = interact {
        hook.try_run(event.clone());
    }
    if !event.default_prevented() {
        close();
    }
}

#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
fn place(
    anchor: Option<LocalRect>,
    size: (f64, f64),
    viewport: (f64, f64),
    direction: crate::Direction,
    fit: bool,
) -> ((f64, f64), LayerPlacement) {
    let Some(anchor) = anchor else {
        return (
            LocalRect::centred(viewport, size),
            LayerPlacement {
                side: Side::Bottom,
                align: Align::Center,
            },
        );
    };
    let rtl = direction == crate::Direction::RightToLeft;
    let (start, end) = if rtl {
        (anchor.x + anchor.width - size.0, anchor.x)
    } else {
        (anchor.x, anchor.x + anchor.width - size.0)
    };
    let inside = |x: f64| x >= 0.0 && x + size.0 <= viewport.0;
    let (x, align) = if !fit || inside(start) {
        (start, Align::Start)
    } else if inside(end) {
        (end, Align::End)
    } else {
        (start.min(viewport.0 - size.0).max(0.0), Align::Start)
    };
    let under = anchor.y + anchor.height;
    let (y, side) = if !fit || under + size.1 <= viewport.1 {
        (under, Side::Bottom)
    } else if anchor.y - size.1 >= 0.0 {
        (anchor.y - size.1, Side::Top)
    } else {
        ((viewport.1 - size.1).max(0.0), Side::Bottom)
    };
    ((x, y), LayerPlacement { side, align })
}

/// A rectangle in whichever space it was measured in: a GPU region's local CSS
/// pixels for a region anchor, the viewport for a placed layer.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LocalRect {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
}

impl LocalRect {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// This rectangle seen from the viewport, given where the region it is
    /// measured in currently sits.
    pub fn in_viewport(self, region: LocalRect) -> LocalRect {
        LocalRect::new(
            region.x + self.x,
            region.y + self.y,
            self.width,
            self.height,
        )
    }

    /// Top-left of a layer placed directly under this rectangle, aligned to
    /// its left edge. P1 places and does not flip: a layer whose anchor is
    /// near the edge of the viewport stays with its anchor.
    pub fn below(self) -> (f64, f64) {
        (self.x, self.y + self.height)
    }

    /// Top-left of a layer of `size` placed against this rectangle and kept
    /// inside a viewport of `viewport`, moved as little as that takes.
    ///
    /// Across, it keeps the anchor's left edge until that would carry it past
    /// the right edge, and then ends at that edge instead. Down, it goes under
    /// the anchor if it fits there, over the anchor if it fits there, and
    /// otherwise as low as the viewport allows. The top-left corner never
    /// leaves the viewport: a layer larger than it is cut off at the far
    /// edges, so its first items stay in reach.
    pub fn below_within(self, size: (f64, f64), viewport: (f64, f64)) -> (f64, f64) {
        let (width, height) = size;
        let x = self.x.min(viewport.0 - width).max(0.0);
        let under = self.y + self.height;
        let y = if under + height <= viewport.1 {
            under
        } else if self.y - height >= 0.0 {
            self.y - height
        } else {
            (viewport.1 - height).max(0.0)
        };
        (x, y)
    }

    /// Top-left of a layer of `size` centred in a viewport of `viewport`.
    pub fn centred(viewport: (f64, f64), size: (f64, f64)) -> (f64, f64) {
        (
            ((viewport.0 - size.0) / 2.0).max(0.0),
            ((viewport.1 - size.1) / 2.0).max(0.0),
        )
    }
}

/// Identifies one open layer within its scope. Ids are never reused, so an id
/// kept after its layer closed keeps failing lookups.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct LayerId(u64);

/// Scope-owned portal content, retaining its creating theme and direction.
///
/// On the host, `client_only` emits a deterministic placeholder. Other host
/// uses record a static render issue: floating content must be attached in
/// the browser. A browser without a mount scope retains readable inline content.
#[leptos::component]
pub fn ScopePortal(
    #[prop(optional)] client_only: bool,
    children: leptos::children::ChildrenFn,
) -> impl leptos::IntoView {
    use leptos::prelude::*;
    #[cfg(not(target_arch = "wasm32"))]
    {
        #[cfg(feature = "ssr")]
        if !client_only {
            crate::StaticRenderIssues::record(
                "portal content requires client_only during static rendering",
            );
        }
        let _ = (client_only, children);
        view! { <span hidden data-rustify-part="portal-placeholder"></span> }
    }
    #[cfg(target_arch = "wasm32")]
    {
        use leptos::wasm_bindgen::JsCast;
        let _ = client_only;
        let stack = use_overlay();
        let theme = crate::theme::use_resolved_theme();
        let theme_source = crate::theme::use_theme_source();
        let locale = crate::use_locale();
        let direction = crate::use_direction();
        let node = NodeRef::<leptos::html::Div>::new();
        let ready = RwSignal::new(false);
        Effect::new(move |_| ready.set(true));
        let release = StoredValue::new(None::<crate::AbortRelease>);
        Effect::new(move |_| {
            let Some(node) = node.get() else { return };
            let element: leptos::web_sys::HtmlElement = node.into();
            if let Some(theme) = theme {
                crate::theme::provider::write_snapshot(
                    &element,
                    &theme.get(),
                    true,
                    theme_source.as_deref(),
                );
            }
            let _ = element.set_attribute("lang", locale.get().tag());
            let _ = element.set_attribute("dir", direction.get().attribute());
            if release.with_value(|held| held.is_none()) {
                if let Some(wrapper) = element.parent_element() {
                    // Bind the browser's own remove method. Abort must not
                    // invoke a closure in an instance which has trapped.
                    if let Ok(remove) = js_sys::Reflect::get(&wrapper, &"remove".into()) {
                        if let Some(remove) = remove.dyn_ref::<js_sys::Function>() {
                            release
                                .set_value(Some(crate::release_on_abort(remove.bind0(&wrapper))));
                        }
                    }
                }
            }
        });
        view! {
            <span hidden data-rustify-part="portal-placeholder"></span>
            {move || {
                if !ready.get() { return ().into_any(); }
                match stack.as_ref() {
                    Some(stack) => {
                        let children = children.clone();
                        view! {
                            <leptos::portal::Portal mount=stack.overlay_root()>
                                <div node_ref=node data-rustify-part="portal">{children()}</div>
                            </leptos::portal::Portal>
                        }.into_any()
                    }
                    None => view! { <div data-rustify-part="portal" data-rustify-issue="missing-scope">{children()}</div> }.into_any(),
                }
            }}
        }
    }
}

/// Where the keyboard is, among a modal layer's tab stops.
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TabFocus {
    /// On the stop at this index.
    On(usize),
    /// On something inside the layer that is not a stop, with this many
    /// stops before it in the document.
    After(usize),
}

/// The stop Tab (or Shift+Tab, `backwards`) has to be sent to so that it stays
/// inside a modal layer of `stops` tab stops, or `None` where the browser's own
/// move already does.
///
/// Only the two ends wrap. Everything between is the browser's ordinary tab
/// order, which is what a person expects inside a dialog as much as outside
/// one.
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
fn wrap_tab(stops: usize, focus: TabFocus, backwards: bool) -> Option<usize> {
    let last = stops.checked_sub(1)?;
    let at_end = match (focus, backwards) {
        (TabFocus::On(index), false) => index == last,
        (TabFocus::On(index), true) => index == 0,
        (TabFocus::After(before), false) => before == stops,
        (TabFocus::After(before), true) => before == 0,
    };
    at_end.then_some(if backwards { last } else { 0 })
}

#[cfg(target_arch = "wasm32")]
pub use dom::{focus_first, on_layer_content_cleanup, use_overlay, Anchor, Layer, OverlayStack};
#[cfg(target_arch = "wasm32")]
pub(crate) use dom::{
    reachable as notification_reachable, EditSession, NotificationEvents, NotificationPortal,
};

#[cfg(not(target_arch = "wasm32"))]
pub use host::{Anchor, Layer};

#[cfg(not(target_arch = "wasm32"))]
mod host {
    use leptos::prelude::*;

    /// A closed floating component's deterministic initial anchor.
    #[derive(Clone)]
    pub enum Anchor {
        Viewport { align: super::ViewportAlign },
        Centred,
    }

    /// Like Leptos' Portal, a layer has no host DOM. Floating components
    /// generate their triggers with the layer initially closed; opening and
    /// positioning the layer belongs to the browser runtime.
    #[component]
    pub fn Layer(
        #[prop(optional)] modal: bool,
        /// Whether this retained layer participates in the stack.
        #[prop(default = Signal::derive(|| true), into)]
        active: Signal<bool>,
        /// The measured interaction root; omission uses the outer wrapper.
        #[prop(optional_no_strip)]
        panel: Option<NodeRef<leptos::html::Div>>,
        /// Measured arrow geometry; used with FloatingPosition.
        #[prop(optional_no_strip)]
        arrow: Option<NodeRef<leptos::html::Div>>,
        /// Explicit scope-local trigger for restoring focus.
        #[prop(optional, into)]
        trigger_id: Signal<String>,
        #[prop(into)] anchor: Signal<Anchor>,
        #[prop(optional)] fit: bool,
        #[prop(optional_no_strip, into)] position: Option<Signal<super::FloatingPosition>>,
        on_close: impl Fn() + Send + Sync + 'static,
        #[prop(optional_no_strip, into)] on_escape_key_down: Option<
            Callback<super::Cancelable<super::LayerDismiss>>,
        >,
        /// Enables outside dismissal for this layer. Omission preserves legacy
        /// outside-pointer behavior; cancellation suppresses the close request.
        #[prop(optional_no_strip, into)]
        on_pointer_down_outside: Option<Callback<super::Cancelable<super::LayerDismiss>>>,
        #[prop(optional_no_strip, into)] on_focus_outside: Option<
            Callback<super::Cancelable<super::LayerDismiss>>,
        >,
        #[prop(optional_no_strip, into)] on_interact_outside: Option<
            Callback<super::Cancelable<super::LayerDismiss>>,
        >,
        #[prop(optional_no_strip, into)] on_open_auto_focus: Option<
            Callback<super::Cancelable<()>>,
        >,
        #[prop(optional_no_strip, into)] on_close_auto_focus: Option<
            Callback<super::Cancelable<()>>,
        >,
        #[prop(optional_no_strip, into)] direction: Option<Signal<crate::Direction>>,
        #[prop(optional_no_strip, into)] on_placement: Option<Callback<super::LayerPlacement>>,
        #[prop(optional, into)] labelled_by: String,
        #[prop(optional, into)] described_by: String,
        #[prop(optional, into)] class: String,
        #[prop(optional, into)] test_id: String,
        children: ChildrenFn,
    ) -> impl IntoView {
        #[cfg(feature = "ssr")]
        if active.get_untracked() {
            crate::StaticRenderIssues::record(
            "floating content must start closed; open it after attachment or declare a fixed client-only placeholder",
        );
        }
        let _ = (
            modal,
            active,
            panel,
            arrow,
            trigger_id,
            anchor,
            fit,
            position,
            on_close,
            on_escape_key_down,
            on_pointer_down_outside,
            on_focus_outside,
            on_interact_outside,
            on_open_auto_focus,
            on_close_auto_focus,
            direction,
            on_placement,
            labelled_by,
            described_by,
            class,
            test_id,
            children,
        );
    }
}

#[cfg(target_arch = "wasm32")]
mod dom {
    use super::{
        dismiss, dismiss_outside, place, wrap_tab, Cancelable, LayerDismiss, LayerId,
        LayerPlacement, LocalRect, TabFocus,
    };
    use crate::listeners::{listen, listen_reentrant, ListenOptions, Listener};
    use leptos::html::Div;
    use leptos::portal::Portal;
    use leptos::prelude::*;
    use leptos::wasm_bindgen::JsCast;
    use leptos::web_sys::{Element, HtmlElement, HtmlInputElement, KeyboardEvent, Node};
    use send_wrapper::SendWrapper;
    use std::collections::HashSet;
    use std::sync::{Arc, Mutex, Weak};

    #[derive(Clone, Copy)]
    pub(crate) struct NotificationEvents {
        pub viewport: StoredValue<NodeRef<Div>>,
        pub on_key_down: Callback<KeyboardEvent, bool>,
        pub on_host_change: Callback<()>,
        pub on_focus_out: Callback<Option<Element>>,
    }

    #[component]
    pub(crate) fn NotificationPortal(
        #[prop(optional_no_strip)] events: Option<NotificationEvents>,
        children: ChildrenFn,
    ) -> impl IntoView {
        let owner = Owner::new();
        let theme = crate::theme::use_resolved_theme();
        let theme_source = crate::theme::use_theme_source();
        let locale = crate::i18n::use_locale();
        let direction = crate::use_direction();
        let view = owner.with(move || use_overlay().map(|stack| {
            let root = stack.overlay_root();
            let node = NodeRef::<Div>::new();
            if let Some(theme) = theme { Effect::new(move || {
                let snapshot = theme.get();
                if let Some(node) = node.get() { let element: HtmlElement = node.into(); crate::theme::provider::write_snapshot(&element, &snapshot, true, theme_source.as_deref()); }
            }); }
            let parent = RwSignal::new(None);
            provide_context(ParentContext { id: parent, notification: true });
            if let Some(events) = events { provide_context(events); }
            let registered = StoredValue::new(false);
            Effect::new({ let stack = stack.clone(); move || {
                let Some(element) = node.get() else { return; };
                if registered.get_value() { return; }
                registered.set_value(true);
                stack.lock().notifications.push(NotificationHost { element: SendWrapper::new(element.into()), parent, events });
                stack.refresh_inert();
                stack.arm();
            }});
            on_cleanup(move || {
                stack.lock().notifications.retain(|host| host.parent != parent);
                stack.refresh_inert();
                let empty = {
                    let inner = stack.lock();
                    inner.layers.is_empty() && inner.edit.is_none() && inner.notifications.is_empty()
                };
                if empty { stack.disarm(); }
            });
            view! { <Portal mount=root><div node_ref=node data-rustify-notifications="" dir=move ||direction.get().attribute() lang=move ||locale.get().tag()>{children()}</div></Portal> }
        }));
        leptos::tachys::reactive_graph::OwnedView::new_with_owner(view, owner)
    }

    /// What a layer is placed against.
    #[derive(Clone)]
    pub enum Anchor {
        /// A rectangle inside a GPU region, in that region's local CSS pixels.
        /// The canvas identifies the region; once it has left the document the
        /// anchor is dead and the layer is asked to close.
        Region {
            canvas: SendWrapper<Element>,
            rect: LocalRect,
        },
        /// An element of this scope.
        Element(SendWrapper<Element>),
        /// Within the visible part of this mount scope.
        Viewport { align: super::ViewportAlign },
        /// Centred in the viewport for existing direct consumers.
        Centred,
    }

    impl Anchor {
        pub fn region(canvas: &Element, rect: LocalRect) -> Self {
            Self::Region {
                canvas: SendWrapper::new(canvas.clone()),
                rect,
            }
        }

        pub fn element(element: &Element) -> Self {
            Self::Element(SendWrapper::new(element.clone()))
        }

        /// The anchor rectangle in viewport coordinates. `None` means the
        /// thing it was anchored to has left the document; `Some(None)` means
        /// it is not anchored to anything.
        pub(crate) fn viewport_rect(&self) -> Option<Option<LocalRect>> {
            let element = match self {
                Self::Centred | Self::Viewport { .. } => return Some(None),
                Self::Region { canvas, .. } => canvas,
                Self::Element(element) => element,
            };
            if !element.is_connected() {
                return None;
            }
            let measured = (**element).get_bounding_client_rect();
            let measured = LocalRect::new(
                measured.left(),
                measured.top(),
                measured.width(),
                measured.height(),
            );
            Some(Some(match self {
                Self::Region { rect, .. } => rect.in_viewport(measured),
                _ => measured,
            }))
        }
    }

    #[derive(Clone)]
    struct LayerRelease(Arc<dyn Fn() + Send + Sync>);

    /// Releases the enclosing Layer while the Content's callback properties
    /// still belong to a live Owner. Composite Content calls this once at setup.
    pub fn on_layer_content_cleanup() {
        if let Some(release) = use_context::<LayerRelease>() {
            on_cleanup(move || (release.0)());
        }
    }

    #[derive(Clone, Copy)]
    struct ParentContext {
        id: RwSignal<Option<LayerId>>,
        notification: bool,
    }

    struct LayerRecord {
        id: LayerId,
        notification_owner: Option<StoredValue<NodeRef<Div>>>,
        element: SendWrapper<Element>,
        panel: SendWrapper<Element>,
        children_host: SendWrapper<Element>,
        notifications_host: SendWrapper<Element>,
        retire: Arc<dyn Fn() + Send + Sync>,
        close: Arc<dyn Fn() + Send + Sync>,
        dismiss: Arc<dyn Fn(LayerDismiss) + Send + Sync>,
        escape: Option<Callback<Cancelable<LayerDismiss>>>,
        outside: Option<Callback<Cancelable<LayerDismiss>>>,
        focus_outside: Option<Callback<Cancelable<LayerDismiss>>>,
        interact_outside: Option<Callback<Cancelable<LayerDismiss>>>,
        anchor: Signal<Anchor>,
        trigger_id: Signal<String>,
    }

    struct NotificationHost {
        element: SendWrapper<Element>,
        parent: RwSignal<Option<LayerId>>,
        events: Option<NotificationEvents>,
    }

    struct Roots {
        container: Element,
        content: Element,
        overlay: Element,
    }

    /// Listeners the stack keeps only while it has something to place. Held
    /// for their `Drop`, which is what takes them off the page.
    struct Watchers {
        _escape: Listener,
        _pointer: Listener,
        _pointer_up: Listener,
        _pointer_cancel: Listener,
        _focus: Listener,
    }
    struct LifecycleWatch {
        _tree: Option<SendWrapper<MutationWatch>>,
        _size: Option<SendWrapper<crate::ResizeObservation>>,
        _scroll: Listener,
        _resize: Listener,
    }
    struct MutationWatch {
        observer: web_sys::MutationObserver,
        _callback: leptos::wasm_bindgen::closure::Closure<
            dyn FnMut(js_sys::Array, web_sys::MutationObserver),
        >,
        _release: Option<crate::AbortRelease>,
    }
    impl Drop for MutationWatch {
        fn drop(&mut self) {
            self.observer.disconnect();
        }
    }
    fn observe_tree(element: &Element, moved: RwSignal<u32>) -> Option<MutationWatch> {
        use leptos::wasm_bindgen::closure::Closure;
        let callback =
            Closure::<dyn FnMut(js_sys::Array, web_sys::MutationObserver)>::new(move |_, _| {
                moved.try_update(|value| *value += 1);
            });
        let observer = web_sys::MutationObserver::new(callback.as_ref().unchecked_ref()).ok()?;
        let options = web_sys::MutationObserverInit::new();
        options.set_child_list(true);
        options.set_subtree(true);
        options.set_attributes(true);
        options.set_attribute_filter(&js_sys::Array::of4(
            &"disabled".into(),
            &"hidden".into(),
            &"id".into(),
            &"aria-disabled".into(),
        ));
        observer.observe_with_options(element, &options).ok()?;
        let release = js_sys::Reflect::get(&observer, &"disconnect".into())
            .ok()
            .and_then(|method| method.dyn_into::<js_sys::Function>().ok())
            .map(|disconnect| crate::release_on_abort(disconnect.bind0(&observer)));
        Some(MutationWatch {
            observer,
            _callback: callback,
            _release: release,
        })
    }

    /// A native editing session. It outranks the layer stack: a key pressed
    /// while text is being composed belongs to the composition and to nothing
    /// else.
    pub(crate) struct EditSession {
        pub(crate) composing: Arc<dyn Fn() -> bool + Send + Sync>,
        pub(crate) cancel: Arc<dyn Fn() + Send + Sync>,
    }

    struct Inner {
        roots: SendWrapper<Roots>,
        layers: Vec<LayerRecord>,
        notifications: Vec<NotificationHost>,
        hierarchy: super::hierarchy::Hierarchy,
        inert: Vec<SendWrapper<Element>>,
        pointer_target: Option<SendWrapper<Node>>,
        last_focus: Option<SendWrapper<Element>>,
        edit: Option<EditSession>,
        watchers: Option<Watchers>,
        lifecycle_users: usize,
        lifecycle: Option<LifecycleWatch>,
    }

    /// The scope's layer stack. Provided by `mount`, read with [`use_overlay`].
    #[derive(Clone)]
    pub struct OverlayStack {
        inner: Arc<Mutex<Inner>>,
        /// Raised whenever something moved a layer's anchor without changing
        /// the anchor itself.
        moved: RwSignal<u32>,
    }

    /// The stack of the scope the caller is rendered in.
    pub fn use_overlay() -> Option<OverlayStack> {
        use_context::<OverlayStack>()
    }

    impl OverlayStack {
        pub fn new(container: &Element, content: &Element, overlay: &Element) -> Self {
            Self {
                inner: Arc::new(Mutex::new(Inner {
                    roots: SendWrapper::new(Roots {
                        container: container.clone(),
                        content: content.clone(),
                        overlay: overlay.clone(),
                    }),
                    layers: Vec::new(),
                    notifications: Vec::new(),
                    hierarchy: super::hierarchy::Hierarchy::default(),
                    inert: Vec::new(),
                    pointer_target: None,
                    last_focus: None,
                    edit: None,
                    watchers: None,
                    lifecycle_users: 0,
                    lifecycle: None,
                })),
                moved: RwSignal::new(0),
            }
        }

        pub fn overlay_root(&self) -> Element {
            self.lock().roots.overlay.clone()
        }

        pub fn depth(&self) -> usize {
            self.lock().layers.len()
        }

        pub fn top(&self) -> Option<LayerId> {
            self.lock().hierarchy.top()
        }

        /// Asks the topmost layer to close. The layer belongs to the
        /// application, so the stack can only ask.
        pub fn close_top(&self) -> bool {
            let close = self.lock().layers.last().map(|layer| layer.close.clone());
            match close {
                Some(close) => {
                    close();
                    true
                }
                None => false,
            }
        }

        fn watch_lifecycle(&self, active: bool) {
            let released = {
                let mut inner = self.lock();
                if active {
                    if inner.lifecycle_users == 0 {
                        let moved = self.moved;
                        inner.lifecycle = Some(LifecycleWatch {
                            _tree: observe_tree(&inner.roots.container, moved)
                                .map(SendWrapper::new),
                            _size: crate::observe_resize(&inner.roots.container, move || {
                                moved.try_update(|n| *n += 1);
                            })
                            .map(SendWrapper::new),
                            _scroll: listen(
                                &document(),
                                "scroll",
                                ListenOptions {
                                    capture: true,
                                    passive: true,
                                },
                                move |_| {
                                    moved.try_update(|n| *n += 1);
                                },
                            ),
                            _resize: listen(
                                &window(),
                                "resize",
                                ListenOptions::default(),
                                move |_| {
                                    moved.try_update(|n| *n += 1);
                                },
                            ),
                        });
                    }
                    inner.lifecycle_users += 1;
                    None
                } else {
                    inner.lifecycle_users -= 1;
                    if inner.lifecycle_users == 0 {
                        inner.lifecycle.take()
                    } else {
                        None
                    }
                }
            };
            drop(released);
        }

        pub(crate) fn moved(&self) -> RwSignal<u32> {
            self.moved
        }

        /// Registers the scope's one native editing session. Ends any session
        /// still registered, so a scope never has two.
        pub(crate) fn begin_edit(&self, session: EditSession) {
            let fresh = self.lock().edit.replace(session).is_none();
            if fresh {
                self.watch_lifecycle(true);
            }
            self.arm();
        }

        pub(crate) fn end_edit(&self) {
            let (ended, empty) = {
                let mut inner = self.lock();
                (
                    inner.edit.take().is_some(),
                    inner.layers.is_empty() && inner.notifications.is_empty(),
                )
            };
            if ended {
                self.watch_lifecycle(false);
            }
            if empty {
                self.disarm();
            }
        }

        pub(crate) fn fallback_element(&self) -> Element {
            self.fallback()
        }

        pub(crate) fn notification_presence(
            &self,
            viewport: StoredValue<NodeRef<Div>>,
            active: Option<&Element>,
        ) -> (bool, bool) {
            let inner = self.lock();
            let hovered =
                notification_owner_at(&inner, |element| element.matches(":hover").unwrap_or(false));
            let focused = notification_owner_at(&inner, |element| {
                active.is_some_and(|active| element.contains(Some(active)))
            });
            (hovered == Some(viewport), focused == Some(viewport))
        }

        pub(crate) fn notification_fallback(&self) -> Element {
            let inner = self.lock();
            inner
                .hierarchy
                .top_modal()
                .and_then(|id| inner.layers.iter().find(|layer| layer.id == id))
                .map(|layer| (*layer.panel).clone())
                .unwrap_or_else(|| inner.roots.container.clone())
        }

        fn push(
            &self,
            parent: Option<LayerId>,
            notification: bool,
            notification_owner: Option<StoredValue<NodeRef<Div>>>,
            modal: bool,
            element: Element,
            panel: Element,
            children_host: Element,
            notifications_host: Element,
            retire: Arc<dyn Fn() + Send + Sync>,
            close: Arc<dyn Fn() + Send + Sync>,
            dismiss: Arc<dyn Fn(LayerDismiss) + Send + Sync>,
            escape: Option<Callback<Cancelable<LayerDismiss>>>,
            outside: Option<Callback<Cancelable<LayerDismiss>>>,
            focus_outside: Option<Callback<Cancelable<LayerDismiss>>>,
            interact_outside: Option<Callback<Cancelable<LayerDismiss>>>,
            anchor: Signal<Anchor>,
            trigger_id: Signal<String>,
        ) -> LayerId {
            let id = {
                let mut inner = self.lock();
                let id = inner.hierarchy.insert(parent, modal, notification);
                inner.layers.push(LayerRecord {
                    id,
                    notification_owner,
                    element: SendWrapper::new(element),
                    panel: SendWrapper::new(panel),
                    children_host: SendWrapper::new(children_host),
                    notifications_host: SendWrapper::new(notifications_host),
                    retire,
                    close,
                    dismiss,
                    escape,
                    outside,
                    focus_outside,
                    interact_outside,
                    anchor,
                    trigger_id,
                });
                id
            };
            self.arm();
            self.refresh_inert();
            id
        }

        fn remove(&self, id: LayerId) {
            let (retired, empty) = {
                let mut inner = self.lock();
                let ids = inner.hierarchy.remove(id);
                let retired: Vec<_> = ids
                    .iter()
                    .filter_map(|id| {
                        inner
                            .layers
                            .iter()
                            .find(|layer| layer.id == *id)
                            .map(|layer| layer.retire.clone())
                    })
                    .collect();
                inner.layers.retain(|layer| !ids.contains(&layer.id));
                (
                    retired,
                    inner.layers.is_empty()
                        && inner.edit.is_none()
                        && inner.notifications.is_empty(),
                )
            };
            // Move surviving notification branches before their former host's
            // resources and DOM can be released. Application callbacks run unlocked.
            self.refresh_inert();
            for retire in retired {
                retire();
            }
            if empty {
                self.disarm();
            }
        }

        fn refresh_inert(&self) {
            let mut inner = self.lock();
            let previous_inert = std::mem::take(&mut inner.inert);
            let order = inner.hierarchy.ordered().to_vec();
            inner
                .layers
                .sort_by_key(|layer| order.iter().position(|id| *id == layer.id));
            // A host can become hidden before migration runs; its surviving
            // notification control may still be active but temporarily unreachable.
            let actual = document().active_element();
            let focused = actual
                .clone()
                .filter(|element| element.dyn_ref::<HtmlElement>().is_some_and(reachable))
                .filter(|element| {
                    !document()
                        .body()
                        .is_some_and(|body| AsRef::<Element>::as_ref(&body) == element)
                })
                .or_else(|| {
                    let no_target = actual.as_ref().is_none_or(|element| {
                        document()
                            .body()
                            .is_some_and(|body| AsRef::<Element>::as_ref(&body) == element)
                            || !element.dyn_ref::<HtmlElement>().is_some_and(reachable)
                    });
                    no_target
                        .then(|| inner.last_focus.as_ref().map(|element| (**element).clone()))
                        .flatten()
                        .filter(|element| {
                            inner.layers.iter().any(|layer| {
                                inner.hierarchy.notification(layer.id)
                                    && layer.element.contains(Some(element))
                            }) || inner
                                .notifications
                                .iter()
                                .any(|host| host.element.contains(Some(element)))
                        })
                });
            let mut moves = Vec::new();
            let mut levels = Vec::new();
            for (index, layer) in inner.layers.iter().enumerate() {
                let host = inner
                    .hierarchy
                    .parent(layer.id)
                    .and_then(|parent| inner.layers.iter().find(|layer| layer.id == parent))
                    .map(|parent| &*parent.children_host)
                    .unwrap_or(&inner.roots.overlay);
                if let Some(wrapper) = layer.element.parent_element() {
                    if wrapper.parent_element().as_ref() != Some(host) {
                        moves.push((host.clone(), wrapper));
                    }
                }
                levels.push(((*layer.element).clone(), index + 1));
            }
            let modal = inner.hierarchy.top_modal();
            let notification_host = modal
                .and_then(|id| inner.layers.iter().find(|layer| layer.id == id))
                .map(|layer| &*layer.notifications_host)
                .unwrap_or(&inner.roots.overlay);
            let mut notifications_changed = Vec::new();
            let mut parents = Vec::new();
            for notification in &inner.notifications {
                if let Some(wrapper) = notification.element.parent_element() {
                    if wrapper.parent_element().as_ref() != Some(notification_host) {
                        moves.push((notification_host.clone(), wrapper));
                    }
                }
                parents.push(notification.parent);
                if let Some(events) = notification.events {
                    notifications_changed.push(events.on_host_change);
                }
            }
            let content = inner.roots.content.clone();
            let modal_element = modal
                .and_then(|id| inner.layers.iter().find(|layer| layer.id == id))
                .map(|layer| (*layer.element).clone());
            let layers: Vec<_> = inner
                .layers
                .iter()
                .map(|layer| {
                    (
                        (*layer.element).clone(),
                        inner.hierarchy.interactive(layer.id),
                        modal.is_some_and(|modal| inner.hierarchy.within(modal, layer.id)),
                    )
                })
                .collect();
            drop(inner);
            // Moving focused DOM synchronously emits focusout. Its notification
            // callbacks read the stack, so no DOM mutation may hold this lock.
            for element in previous_inert {
                let _ = element.remove_attribute("inert");
            }
            for (host, wrapper) in moves {
                let _ = host.append_child(&wrapper);
            }
            for (element, index) in levels {
                if let Some(element) = element.dyn_ref::<HtmlElement>() {
                    let _ = element.style().set_property("z-index", &index.to_string());
                }
            }
            for parent in parents {
                let _ = parent.try_set(modal);
            }
            let mut claims = Vec::new();
            if modal.is_some() {
                claim_inert(&content, &mut claims);
            }
            for (element, interactive, ancestor) in layers {
                if interactive {
                    let _ = element.remove_attribute("data-rustify-inactive-ancestor");
                } else if ancestor {
                    let _ = element.set_attribute("data-rustify-inactive-ancestor", "");
                    if let Some(modal) = &modal_element {
                        inert_outside_branch(&element, modal, &mut claims);
                    }
                } else {
                    claim_inert(&element, &mut claims);
                }
            }
            self.lock().inert = claims;
            if let Some(element) =
                focused.filter(|element| document().active_element().as_ref() != Some(element))
            {
                if element.dyn_ref::<HtmlElement>().is_some_and(reachable) {
                    focus(&element);
                }
            }
            for changed in notifications_changed {
                changed.run(());
            }
        }

        /// Installs the scope's listeners while layers, editing or notifications need them.
        fn arm(&self) {
            let mut inner = self.lock();
            if inner.watchers.is_some() {
                return;
            }
            // Weak: the listener lives inside the stack it would otherwise
            // keep alive, and a scope disposed with a layer open must still
            // drop everything.
            let weak = Arc::downgrade(&self.inner);
            // Capture, so the stack answers before anything inside the scope
            // does: an open layer outranks the application's own commands.
            let escape = listen(
                &inner.roots.container,
                "keydown",
                ListenOptions {
                    capture: true,
                    passive: false,
                },
                move |event| {
                    let Some(event) = event.dyn_ref::<KeyboardEvent>() else {
                        return;
                    };
                    let key = event.key();
                    if key == "Tab" {
                        let modal = weak.upgrade().and_then(|inner| {
                            let inner = inner
                                .lock()
                                .expect("overlay stack is never held across a panic");
                            inner
                                .hierarchy
                                .top_modal()
                                .and_then(|id| inner.layers.iter().find(|layer| layer.id == id))
                                .map(|layer| layer.panel.clone())
                        });
                        if let Some(modal) = modal {
                            keep_tab_inside(&modal, event);
                        }
                        return;
                    }
                    match if event.is_composing() {
                        if key == "Escape" || key == "Enter" {
                            Routed::Native
                        } else {
                            Routed::Ignored
                        }
                    } else {
                        route(&weak, event)
                    } {
                        // Nothing above the application wanted it.
                        Routed::Ignored => {}
                        // Reserve the key for the input method without
                        // cancelling the browser's composition action.
                        Routed::Native => event.stop_propagation(),
                        Routed::Taken => {
                            event.prevent_default();
                            event.stop_propagation();
                        }
                    }
                },
            );
            let weak = Arc::downgrade(&self.inner);
            let pointer = listen(
                &document(),
                "pointerdown",
                ListenOptions {
                    capture: true,
                    passive: false,
                },
                move |event| {
                    if event.default_prevented() {
                        return;
                    }
                    let Some(target) = event
                        .target()
                        .and_then(|target| target.dyn_into::<Node>().ok())
                    else {
                        return;
                    };
                    let Some(inner) = weak.upgrade() else {
                        return;
                    };
                    let top = {
                        let mut inner = inner
                            .lock()
                            .expect("overlay stack is never held across a panic");
                        inner.pointer_target = Some(SendWrapper::new(target.clone()));
                        inner.layers.last().map(|layer| {
                            (
                                layer.panel.clone(),
                                layer.anchor,
                                layer.trigger_id,
                                layer.outside,
                                layer.interact_outside,
                                layer.dismiss.clone(),
                            )
                        })
                    };
                    let Some((panel, anchor, trigger_id, specific, interact, close)) = top else {
                        return;
                    };
                    if (specific.is_none() && interact.is_none())
                        || inside(&panel, anchor, trigger_id, &target)
                    {
                        return;
                    }
                    dismiss_outside(specific, interact, LayerDismiss::PointerOutside, || {
                        close(LayerDismiss::PointerOutside)
                    });
                },
            );
            let weak = Arc::downgrade(&self.inner);
            let clear_pointer = move |_: web_sys::Event| {
                if let Some(inner) = weak.upgrade() {
                    inner
                        .lock()
                        .expect("overlay stack is never held across a panic")
                        .pointer_target = None;
                }
            };
            let pointer_up = listen(
                &document(),
                "pointerup",
                ListenOptions::default(),
                clear_pointer.clone(),
            );
            let pointer_cancel = listen(
                &document(),
                "pointercancel",
                ListenOptions::default(),
                clear_pointer,
            );
            let weak = Arc::downgrade(&self.inner);
            let focus_outside = listen_reentrant(
                &document(),
                "focusin",
                ListenOptions {
                    capture: true,
                    passive: false,
                },
                move |event| {
                    let Some(target) = event
                        .target()
                        .and_then(|target| target.dyn_into::<Node>().ok())
                    else {
                        return;
                    };
                    let Some(inner) = weak.upgrade() else {
                        return;
                    };
                    let (top, modal, duplicate) = {
                        let mut inner = inner
                            .lock()
                            .expect("overlay stack is never held across a panic");
                        if let Some(element) = target.dyn_ref::<Element>() {
                            if !document()
                                .body()
                                .is_some_and(|body| AsRef::<Element>::as_ref(&body) == element)
                            {
                                inner.last_focus = Some(SendWrapper::new(element.clone()));
                            }
                        }
                        let duplicate = inner.pointer_target.take().is_some_and(|pointer| {
                            pointer.contains(Some(&target)) || target.contains(Some(&pointer))
                        });
                        let top = inner.layers.last().map(|layer| {
                            (
                                layer.id,
                                layer.panel.clone(),
                                layer.anchor,
                                layer.trigger_id,
                                layer.focus_outside,
                                layer.interact_outside,
                                layer.dismiss.clone(),
                            )
                        });
                        let modal = inner
                            .hierarchy
                            .top_modal()
                            .and_then(|id| inner.layers.iter().find(|layer| layer.id == id))
                            .map(|layer| (layer.id, layer.panel.clone()));
                        (top, modal, duplicate)
                    };
                    if let Some((id, panel, anchor, trigger_id, specific, interact, close)) = top {
                        if !duplicate
                            && (specific.is_some() || interact.is_some())
                            && !inside(&panel, anchor, trigger_id, &target)
                        {
                            dismiss_outside(specific, interact, LayerDismiss::FocusOutside, || {
                                if !modal.as_ref().is_some_and(|(modal, _)| *modal == id) {
                                    close(LayerDismiss::FocusOutside);
                                }
                            });
                        }
                    }
                    if let Some((id, modal)) = modal {
                        let restore = {
                            let inner = inner
                                .lock()
                                .expect("overlay stack is never held across a panic");
                            inner.hierarchy.top_modal() == Some(id)
                                && document().active_element().is_some_and(|active| {
                                    inner.roots.container.contains(Some(&active))
                                        && !modal.contains(Some(&active))
                                })
                        };
                        if restore {
                            focus_first(&modal);
                        }
                    }
                },
            );
            inner.watchers = Some(Watchers {
                _escape: escape,
                _pointer: pointer,
                _pointer_up: pointer_up,
                _pointer_cancel: pointer_cancel,
                _focus: focus_outside,
            });
        }

        fn disarm(&self) {
            let watchers = {
                let mut inner = self.lock();
                inner.last_focus = None;
                inner.pointer_target = None;
                inner.watchers.take()
            };
            drop(watchers);
        }

        /// Where focus goes when a layer closes and its trigger is gone.
        fn fallback(&self) -> Element {
            self.lock().roots.container.clone()
        }

        fn lock(&self) -> std::sync::MutexGuard<'_, Inner> {
            self.inner
                .lock()
                .expect("overlay stack is never held across a panic")
        }
    }

    enum Routed {
        Taken,
        Native,
        Ignored,
    }

    // Ownership comes from the creating Owner. A migrated foreign viewport
    // takes precedence over the modal surface that physically contains it.
    fn notification_owner_at(
        inner: &Inner,
        contains: impl Fn(&Element) -> bool,
    ) -> Option<StoredValue<NodeRef<Div>>> {
        if let Some(host) = inner
            .notifications
            .iter()
            .find(|host| contains(&host.element))
        {
            return host.events.map(|events| events.viewport);
        }
        inner
            .layers
            .iter()
            .rev()
            .find(|layer| contains(&layer.element))
            .and_then(|layer| layer.notification_owner)
    }

    // Native editing wins. A notification-owned child layer outranks its
    // notification, which in turn outranks the modal hosting its viewport.
    fn route(weak: &Weak<Mutex<Inner>>, event: &KeyboardEvent) -> Routed {
        let Some(inner) = weak.upgrade() else {
            return Routed::Ignored;
        };
        let key = event.key();
        let (composing, cancel, close_top, notifications, notification_child) = {
            let inner = inner
                .lock()
                .expect("overlay stack is never held across a panic");
            let edit = inner.edit.as_ref();
            let active = document().active_element();
            let focused_owner = notification_owner_at(&inner, |element| {
                active
                    .as_ref()
                    .is_some_and(|active| element.contains(Some(active)))
            });
            let focused = inner.notifications.iter().find(|host| {
                host.events
                    .is_some_and(|events| Some(events.viewport) == focused_owner)
            });
            let notification_child = focused.and_then(|host| {
                let parent = host.parent.get_untracked();
                inner
                    .layers
                    .iter()
                    .rev()
                    .find(|layer| {
                        layer.notification_owner == focused_owner
                            && inner.hierarchy.interactive(layer.id)
                            && !parent
                                .is_some_and(|parent| inner.hierarchy.within(parent, layer.id))
                    })
                    .map(|layer| (layer.escape, layer.dismiss.clone()))
            });
            let notifications = inner
                .notifications
                .iter()
                .rev()
                .filter_map(|host| host.events)
                .filter(|events| key != "Escape" || Some(events.viewport) == focused_owner)
                .collect::<Vec<_>>();
            (
                edit.map(|edit| edit.composing.clone()),
                edit.map(|edit| edit.cancel.clone()),
                inner
                    .layers
                    .last()
                    .map(|layer| (layer.escape, layer.dismiss.clone())),
                notifications,
                notification_child,
            )
        };
        if let Some(composing) = composing {
            if composing() {
                return if key == "Escape" || key == "Enter" {
                    Routed::Native
                } else {
                    Routed::Ignored
                };
            }
            if key == "Escape" {
                if let Some(cancel) = cancel {
                    cancel();
                    return Routed::Taken;
                }
            }
            if key == "Enter" {
                return Routed::Ignored;
            }
        }
        if key == "Escape" {
            if let Some((hook, close)) = notification_child {
                dismiss(hook, LayerDismiss::Escape, || close(LayerDismiss::Escape));
                return Routed::Taken;
            }
        }
        for notification in notifications {
            if notification.on_key_down.run(event.clone()) {
                return Routed::Taken;
            }
        }
        if key == "Escape" {
            if let Some((hook, close)) = close_top {
                dismiss(hook, LayerDismiss::Escape, || close(LayerDismiss::Escape));
                return Routed::Taken;
            }
        }
        Routed::Ignored
    }

    fn inside(
        panel: &Element,
        anchor: Signal<Anchor>,
        trigger_id: Signal<String>,
        target: &Node,
    ) -> bool {
        if panel.contains(Some(target))
            || trigger_id
                .try_get_untracked()
                .filter(|id| !id.is_empty())
                .and_then(|id| document().get_element_by_id(&id))
                .is_some_and(|trigger| trigger.contains(Some(target)))
        {
            return true;
        }
        anchor
            .try_get_untracked()
            .and_then(|anchor| match anchor {
                Anchor::Element(element) => Some(element),
                Anchor::Region { canvas, .. } => Some(canvas),
                Anchor::Centred | Anchor::Viewport { .. } => None,
            })
            .is_some_and(|trigger| trigger.contains(Some(target)))
    }

    fn claim_inert(element: &Element, claims: &mut Vec<SendWrapper<Element>>) {
        if !element.has_attribute("inert") {
            let _ = element.set_attribute("inert", "");
            claims.push(SendWrapper::new(element.clone()));
        }
    }

    fn inert_outside_branch(
        root: &Element,
        modal: &Element,
        claims: &mut Vec<SendWrapper<Element>>,
    ) {
        let mut next = root.first_element_child();
        while let Some(child) = next {
            next = child.next_element_sibling();
            if child == *modal {
                continue;
            }
            if child.contains(Some(modal)) {
                inert_outside_branch(&child, modal, claims);
            } else {
                claim_inert(&child, claims);
            }
        }
    }

    fn focus(element: &Element) {
        if let Some(element) = element.dyn_ref::<HtmlElement>() {
            let options = leptos::web_sys::FocusOptions::new();
            options.set_prevent_scroll(true);
            let _ = element.focus_with_options(&options);
        }
    }

    /// Everything that can take focus, before the rules below narrow it.
    const FOCUSABLE: &str = "a[href], area[href], button, input:not([type='hidden']), select, \
         textarea, iframe, details > summary:first-of-type, \
         [contenteditable]:not([contenteditable='false']), [tabindex]";

    /// The elements inside `root` that Tab stops at, in the order it stops at
    /// them.
    ///
    /// The browser's rules, as far as a page can apply them: nothing disabled,
    /// inert, hidden, taken out with a negative `tabindex`, or not rendered;
    /// one radio button per group, the checked one if there is one; positive
    /// `tabindex` values first, in their own order.
    fn tab_stops(root: &Element) -> Vec<HtmlElement> {
        let Ok(found) = root.query_selector_all(FOCUSABLE) else {
            return Vec::new();
        };
        let mut stops: Vec<(i32, HtmlElement)> = (0..found.length())
            .filter_map(|index| found.item(index)?.dyn_into::<HtmlElement>().ok())
            .filter_map(|element| {
                let index = element
                    .get_attribute("tabindex")
                    .and_then(|value| value.trim().parse::<i32>().ok())
                    .unwrap_or(0);
                (index >= 0 && reachable(&element)).then_some((index, element))
            })
            .collect();

        let mut forms = Vec::new();
        let groups: Vec<_> = stops
            .iter()
            .map(|(_, element)| {
                let input = element
                    .dyn_ref::<HtmlInputElement>()
                    .filter(|input| input.type_() == "radio" && !input.name().is_empty())?;
                let form = input.form();
                let owner = forms
                    .iter()
                    .position(|candidate| candidate == &form)
                    .unwrap_or_else(|| {
                        forms.push(form);
                        forms.len() - 1
                    });
                Some(((input.name(), owner), input.checked()))
            })
            .collect();
        let checked: HashSet<_> = groups
            .iter()
            .flatten()
            .filter_map(|(group, checked)| checked.then_some(group.clone()))
            .collect();
        let mut first_of_group = HashSet::new();
        let mut index = 0;
        stops.retain(|_| {
            let keep = match &groups[index] {
                None => true,
                Some((group, is_checked)) if checked.contains(group) => *is_checked,
                Some((group, _)) => first_of_group.insert(group.clone()),
            };
            index += 1;
            keep
        });

        // Stable, so equal indices keep their document order.
        stops.sort_by_key(|(index, _)| if *index > 0 { *index } else { i32::MAX });
        stops.into_iter().map(|(_, element)| element).collect()
    }

    fn live_trigger(element: &Element) -> bool {
        if !element.is_connected()
            || element
                .matches(":disabled, [aria-disabled='true']")
                .unwrap_or(false)
            || element.closest("[hidden]").ok().flatten().is_some()
            || element.get_client_rects().length() == 0
        {
            return false;
        }
        !window()
            .get_computed_style(element)
            .ok()
            .flatten()
            .and_then(|style| style.get_property_value("visibility").ok())
            .is_some_and(|visibility| visibility == "hidden" || visibility == "collapse")
    }

    /// Whether `element` is on screen and neither disabled nor shut away.
    pub(crate) fn reachable(element: &HtmlElement) -> bool {
        if element.matches(":disabled").unwrap_or(false)
            || element
                .closest("[inert], [hidden]")
                .ok()
                .flatten()
                .is_some()
            || element.get_client_rects().length() == 0
        {
            return false;
        }
        !window()
            .get_computed_style(element)
            .ok()
            .flatten()
            .and_then(|style| style.get_property_value("visibility").ok())
            .is_some_and(|visibility| visibility == "hidden" || visibility == "collapse")
    }

    /// The first tab stop inside `root`, or `root` itself.
    pub fn focus_first(root: &Element) {
        match tab_stops(root).first() {
            Some(first) => {
                let _ = first.focus();
            }
            None => focus(root),
        }
    }

    /// Keeps Tab inside a modal layer: past the last stop it goes to the
    /// first, and Shift+Tab before the first goes to the last. Everything else
    /// is left to the browser.
    fn keep_tab_inside(root: &Element, event: &KeyboardEvent) {
        if event.key() != "Tab"
            || event.default_prevented()
            || event.is_composing()
            || event.ctrl_key()
            || event.alt_key()
            || event.meta_key()
        {
            return;
        }
        let Some(active) = document().active_element() else {
            return;
        };
        let stops = tab_stops(root);
        let focus = match stops
            .iter()
            .position(|stop| AsRef::<Element>::as_ref(stop) == &active)
        {
            Some(index) => TabFocus::On(index),
            None => TabFocus::After(
                stops
                    .iter()
                    .filter(|stop| {
                        stop.compare_document_position(&active) & Node::DOCUMENT_POSITION_FOLLOWING
                            != 0
                    })
                    .count(),
            ),
        };
        if let Some(target) = wrap_tab(stops.len(), focus, event.shift_key()) {
            event.prevent_default();
            let _ = stops[target].focus();
        }
    }

    /// Whether the keyboard is inside `layer`, or nowhere in particular.
    fn holds_focus(layer: Option<&Element>) -> bool {
        let Some(active) = document().active_element() else {
            return true;
        };
        if document()
            .body()
            .is_some_and(|body| active == *body.as_ref())
        {
            return true;
        }
        layer.is_some_and(|layer| layer.contains(Some(&active)))
    }

    fn viewport() -> (f64, f64) {
        let window = window();
        (
            window
                .inner_width()
                .ok()
                .and_then(|value| value.as_f64())
                .unwrap_or_default(),
            window
                .inner_height()
                .ok()
                .and_then(|value| value.as_f64())
                .unwrap_or_default(),
        )
    }

    /// The part of the viewport a fixed layer can be seen in: the window less
    /// its scroll bars, which would otherwise cover a layer placed flush with
    /// the edge.
    fn visible_area() -> (f64, f64) {
        match document().document_element() {
            Some(root) => (root.client_width() as f64, root.client_height() as f64),
            None => viewport(),
        }
    }

    #[derive(Clone, Copy)]
    struct LayerResources {
        registered: RwSignal<Option<LayerId>>,
        size: StoredValue<Option<SendWrapper<crate::ResizeObservation>>>,
        arrow_size: StoredValue<Option<SendWrapper<crate::ResizeObservation>>>,
        frame: StoredValue<Option<crate::FrameHandle>>,
        anchor: StoredValue<Option<SendWrapper<crate::ResizeObservation>>>,
        watching: StoredValue<bool>,
    }
    impl LayerResources {
        fn watch(self, stack: &OverlayStack, active: bool) {
            if self
                .watching
                .try_get_value()
                .is_some_and(|watching| watching != active)
            {
                let _ = self.watching.try_set_value(active);
                stack.watch_lifecycle(active);
            }
        }
        fn cancel_frame(self) {
            self.frame.try_update_value(|held| {
                if let Some(frame) = held.take() {
                    frame.cancel();
                }
            });
        }
        fn release(self, stack: &OverlayStack) {
            self.cancel_frame();
            self.size.try_update_value(|held| {
                held.take();
            });
            self.arrow_size.try_update_value(|held| {
                held.take();
            });
            self.anchor.try_update_value(|held| {
                held.take();
            });
            if let Some(id) = self.registered.try_get_untracked().flatten() {
                let _ = self.registered.try_set(None);
                stack.remove(id);
            }
        }
    }

    /// One layer of the scope's stack.
    ///
    /// The application owns whether the layer exists; `on_close` is how the
    /// stack asks it to stop existing, which happens on Escape at the top of
    /// the stack and when the anchor leaves the document.
    #[component]
    pub fn Layer(
        /// A modal layer makes the rest of its scope inert while it is open.
        #[prop(optional)]
        modal: bool,
        /// Whether this retained layer participates in the stack.
        #[prop(default = Signal::derive(|| true), into)]
        active: Signal<bool>,
        /// The measured interaction root; omission uses the outer wrapper.
        #[prop(optional_no_strip)]
        panel: Option<NodeRef<leptos::html::Div>>,
        /// Measured arrow geometry; used with FloatingPosition.
        #[prop(optional_no_strip)]
        arrow: Option<NodeRef<leptos::html::Div>>,
        /// Explicit scope-local trigger for restoring focus.
        #[prop(optional, into)]
        trigger_id: Signal<String>,
        #[prop(into)] anchor: Signal<Anchor>,
        /// Keeps an anchored layer inside the viewport, moving it off its
        /// anchor as little as that takes. Off, the layer stays exactly under
        /// its anchor wherever that is.
        #[prop(optional)]
        fit: bool,
        #[prop(optional_no_strip, into)] position: Option<Signal<super::FloatingPosition>>,
        on_close: impl Fn() + Send + Sync + 'static,
        #[prop(optional_no_strip, into)] on_escape_key_down: Option<
            Callback<super::Cancelable<super::LayerDismiss>>,
        >,
        /// Enables outside dismissal for this layer. Omission preserves legacy
        /// outside-pointer behavior; cancellation suppresses the close request.
        #[prop(optional_no_strip, into)]
        on_pointer_down_outside: Option<Callback<super::Cancelable<super::LayerDismiss>>>,
        #[prop(optional_no_strip, into)] on_focus_outside: Option<
            Callback<super::Cancelable<super::LayerDismiss>>,
        >,
        #[prop(optional_no_strip, into)] on_interact_outside: Option<
            Callback<super::Cancelable<super::LayerDismiss>>,
        >,
        #[prop(optional_no_strip, into)] on_open_auto_focus: Option<
            Callback<super::Cancelable<()>>,
        >,
        #[prop(optional_no_strip, into)] on_close_auto_focus: Option<
            Callback<super::Cancelable<()>>,
        >,
        #[prop(optional_no_strip, into)] direction: Option<Signal<crate::Direction>>,
        #[prop(optional_no_strip, into)] on_placement: Option<Callback<super::LayerPlacement>>,
        /// The id of the element that names this layer, for the layers that
        /// carry a role a reader announces. A modal dialog needs one; a
        /// presentation wrapper around a menu does not, because the menu
        /// inside it carries its own.
        #[prop(optional, into)]
        labelled_by: String,
        /// The id of the text that describes it, when there is one.
        #[prop(optional, into)]
        described_by: String,
        #[prop(optional, into)] class: String,
        #[prop(optional, into)] test_id: String,
        children: ChildrenFn,
    ) -> impl IntoView {
        let theme = crate::theme::use_resolved_theme();
        let theme_source = crate::theme::use_theme_source();
        let locale = crate::i18n::use_locale();
        let directional = direction.is_some() || on_placement.is_some();
        let direction = direction.unwrap_or_else(crate::use_direction);
        let parent = use_context::<ParentContext>();
        let notification_events = use_context::<NotificationEvents>();
        let owner = Owner::new();
        let view = owner.with(move || use_overlay().map(|stack| {
            let root = stack.overlay_root();
            let class = format!("rustify-layer {class}");
            let labelled_by = (!labelled_by.is_empty()).then(|| labelled_by.clone());
            let described_by = (!described_by.is_empty()).then(|| described_by.clone());
            let moved = stack.moved();
            let node = NodeRef::<Div>::new();
            let children_host = NodeRef::<Div>::new();
            let notifications_host = NodeRef::<Div>::new();
            if let Some(theme) = theme {
                Effect::new(move || {
                    let snapshot = theme.get();
                    if let Some(node) = node.get() {
                        let element: leptos::web_sys::HtmlElement = node.into();
                        crate::theme::provider::write_snapshot(&element, &snapshot, true, theme_source.as_deref());
                    }
                });
            }
            let on_close = Arc::new(on_close);
            let left = RwSignal::new(0.0f64);
            let top = RwSignal::new(0.0f64);
            let placement = RwSignal::new(LayerPlacement::default());
            let position_issue=RwSignal::new(None::<&'static str>);
            let last_position=StoredValue::new(super::FloatingPosition::default());
            let last_boundary=StoredValue::new(None::<LocalRect>);
            let placed = StoredValue::new(false);
            let alive = RwSignal::new(false);
            let detached = RwSignal::new(false);
            let seen_panel = StoredValue::new(false);
            let projected_scope = StoredValue::new(false);
            let reported_dead = StoredValue::new(false);
            let resources = LayerResources { registered: RwSignal::new(None), size: StoredValue::new(None), arrow_size: StoredValue::new(None), frame: StoredValue::new(None), anchor: StoredValue::new(None), watching: StoredValue::new(false) };
            provide_context(ParentContext { id: resources.registered, notification: false });
            let returns_to: StoredValue<Option<SendWrapper<Element>>> = StoredValue::new(None);

            Effect::new({ let stack = stack.clone(); move || resources.watch(&stack, active.get()) });

            let measure=Arc::new({
                let stack = stack.clone();
                let on_close = on_close.clone();
                move || untrack(|| {
                    if !active.get() { return; }
                    if parent.is_some_and(|parent|!parent.notification && parent.id.get().is_none()) {if alive.get_untracked(){alive.set(false);}return;}
                    let anchor = anchor.get();
                    let direction = direction.get();
                    let element = panel.unwrap_or(node).get().filter(|element|element.is_connected());
                    if element.is_some() { seen_panel.set_value(true); }
                    let explicit = trigger_id.get();
                    let valid_trigger = explicit.is_empty() || document().get_element_by_id(&explicit)
                        .filter(|element|stack.fallback().contains(Some(element))).is_some_and(|element|live_trigger(&element));
                    let scope = if matches!(anchor,Anchor::Viewport {..}) {
                        let bounds=stack.fallback().get_bounding_client_rect();
                        super::placement::visible_scope(LocalRect::new(bounds.left(),bounds.top(),bounds.width(),bounds.height()),visible_area())
                    } else {None};
                    let visible = !matches!(anchor,Anchor::Viewport {..}) || scope.is_some();
                    let valid = element.zip(anchor.viewport_rect()).filter(|_|valid_trigger && visible);
                    let Some((element,rect)) = valid else {

                        if alive.get_untracked() { alive.set(false); }
                        if seen_panel.get_value() && !reported_dead.get_value() {
                            reported_dead.set_value(true);
                            on_close();
                        }
                        return;
                    };
                    reported_dead.set_value(false);
                    let measured = element.get_bounding_client_rect();
                    let size = (measured.width(), measured.height());
                    if scope.is_none() && projected_scope.get_value() {
                        if let Some(wrapper)=node.get() {
                            let wrapper:HtmlElement=wrapper.into();
                            for name in ["--rustify-layer-scope-x","--rustify-layer-scope-y","--rustify-layer-scope-width","--rustify-layer-scope-height","--rustify-floating-available-width","--rustify-floating-available-height"] {let _=wrapper.style().remove_property(name);}
                        }
                        projected_scope.set_value(false);
                    }
                    let scoped = scope.map(|bounds| {
                        projected_scope.set_value(true);
                        let inset=document().document_element().and_then(|root|window().get_computed_style(&root).ok().flatten())
                            .and_then(|style|style.get_property_value("font-size").ok()).and_then(|size|size.trim_end_matches("px").parse::<f64>().ok()).unwrap_or(16.);
                        let align=match anchor {Anchor::Viewport {align}=>align,_=>super::ViewportAlign::Center};
                        let result=super::placement::place_viewport(bounds,size,align,inset);
                        if let Some(wrapper)=node.get() {
                            let wrapper:HtmlElement=wrapper.into();
                            for(name,value)in [("x",bounds.x),("y",bounds.y),("width",bounds.width),("height",bounds.height)] {
                                let _=wrapper.style().set_property(&format!("--rustify-layer-scope-{name}"),&format!("{value}px"));
                            }
                            let _=wrapper.style().set_property("--rustify-floating-available-width",&format!("{}px",result.available_width));
                            let _=wrapper.style().set_property("--rustify-floating-available-height",&format!("{}px",result.available_height));
                        }
                        result
                    });
                    let mut hidden=false;
                    let advanced=position.zip(rect).map(|(position,rect)| {
                        let requested=position.get();
                        let mut config=match requested.validate() {
                            Ok(())=>{position_issue.set(None);requested},
                            Err(error)=>{position_issue.set(Some(error.field));last_position.get_value()}
                        };
                        let area=visible_area();
                        let viewport=LocalRect::new(0.,0.,area.0,area.1);
                        let mut boundary=viewport;
                        let container=stack.lock().roots.container.clone();
                        let mut valid=true;
                        for id in &config.collision_boundary_ids {
                            let Some(bound)=document().get_element_by_id(id).filter(|element|container.contains(Some(element))) else {valid=false;break;};
                            let bounds=bound.get_bounding_client_rect();
                            let x=boundary.x.max(bounds.left());let y=boundary.y.max(bounds.top());
                            let right=(boundary.x+boundary.width).min(bounds.right());let bottom=(boundary.y+boundary.height).min(bounds.bottom());
                            boundary=LocalRect::new(x,y,(right-x).max(0.),(bottom-y).max(0.));
                        }
                        if valid {last_boundary.set_value(Some(boundary));last_position.set_value(config.clone());}
                        else {position_issue.set(Some("collision_boundary_ids"));boundary=last_boundary.get_value().unwrap_or(viewport);config=last_position.get_value();}
                        let tip=arrow.and_then(|arrow|arrow.get()).filter(|tip|tip.is_connected());
                        let arrow_size=tip.as_ref().map(|tip| {
                            let rect=tip.get_bounding_client_rect();
                            if matches!(tip.get_attribute("data-side").as_deref(),Some("left"|"right")) {(rect.height(),rect.width())}else{(rect.width(),rect.height())}
                        });
                        let result=super::placement::place_floating(rect,size,arrow_size,boundary,direction,&config);
                        hidden=config.hide_when_detached && result.detached;
                        if let Some(tip)=tip {
                            let tip:HtmlElement=tip.into();
                            let _=tip.set_attribute("data-side",result.placement.side.as_str());
                            let _=tip.set_attribute("data-align",result.placement.align.as_str());
                            let _=tip.style().set_property("visibility",if result.arrow_hidden || hidden {"hidden"}else{"inherit"});
                            if let Some((ax,ay))=result.arrow {
                                if let Some(wrapper)=node.get() {
                                    let wrapper=wrapper.get_bounding_client_rect();
                                    let _=tip.style().set_property("left",&format!("{}px",result.x+measured.left()-wrapper.left()+ax));
                                    let _=tip.style().set_property("top",&format!("{}px",result.y+measured.top()-wrapper.top()+ay));
                                }
                            }
                        }
                        let surface:Element=element.clone().into();
                        if let Some(surface)=surface.dyn_ref::<HtmlElement>() {
                            let _=surface.style().set_property("--rustify-floating-anchor-width",&format!("{}px",rect.width));
                            let _=surface.style().set_property("--rustify-floating-anchor-height",&format!("{}px",rect.height));
                            let _=surface.style().set_property("--rustify-floating-available-width",&format!("{}px",result.available_width));
                            let _=surface.style().set_property("--rustify-floating-available-height",&format!("{}px",result.available_height));
                        }
                        result
                    });
                    let ((x, y), resolved) = if let Some(result)=scoped {
                        ((result.x,result.y),LayerPlacement {side:super::Side::Bottom,align:super::Align::Center})
                    } else if let Some(result)=advanced {
                        ((result.x,result.y),result.placement)
                    } else if directional {
                        place(rect, size, if fit { visible_area() } else { viewport() }, direction, fit)
                    } else {
                        let coordinates = match rect {
                            Some(rect) if fit => rect.below_within(size, visible_area()),
                            Some(rect) => rect.below(),
                            None => LocalRect::centred(viewport(), size),
                        };
                        (coordinates, LayerPlacement::default())
                    };
                    let surface:Element=element.into();
                    let _=surface.set_attribute("data-side",resolved.side.as_str());
                    let _=surface.set_attribute("data-align",resolved.align.as_str());
                    left.set(x);
                    top.set(y);
                    if detached.get_untracked()!=hidden {detached.set(hidden);}
                    if !alive.get_untracked(){alive.set(true);}
                    let changed = placement.get_untracked() != resolved || !placed.get_value();
                    placement.set(resolved);
                    placed.set_value(true);
                    if changed { if let Some(callback) = on_placement { callback.try_run(resolved); } }
                })
            });

            let frame_epoch=RwSignal::new(0_u64);
            Effect::new(move || {
                moved.track();frame_epoch.track();anchor.track();direction.track();trigger_id.track();node.track();panel.unwrap_or(node).track();
                if let Some(position)=position {position.track();}
                if let Some(arrow)=arrow {arrow.track();}
                if let Some(parent)=parent {parent.id.track();}
                if !active.get() || parent.is_some_and(|parent|!parent.notification && parent.id.get().is_none()) {
                    resources.cancel_frame();

                    if alive.get_untracked(){alive.set(false);}
                    return;
                }
                if resources.frame.with_value(Option::is_none) {
                    let measure=measure.clone();
                    resources.frame.set_value(Some(crate::next_frame(move || {
                        resources.frame.try_update_value(|held| {held.take();});
                        measure();
                        if active.try_get_untracked()==Some(true) && alive.try_get_untracked()==Some(true)
                            && position.is_some() && last_position.try_get_value().is_some_and(|position|position.update==super::PositionUpdate::Always) {
                            frame_epoch.try_update(|epoch|*epoch=epoch.wrapping_add(1));
                        }
                    })));
                }
            });

            Effect::new(move || {
                let anchor = anchor.get();
                if !active.get() || !alive.get() || detached.get() { resources.anchor.set_value(None); return; }
                let observed = match anchor {
                    Anchor::Element(element) => Some((*element).clone()),
                    Anchor::Region { canvas, .. } => Some((*canvas).clone()),
                    Anchor::Centred | Anchor::Viewport {..} => None,
                };
                resources.anchor.set_value(observed.and_then(|element| {
                    crate::observe_resize(&element, move || { moved.try_update(|n| *n += 1); }).map(SendWrapper::new)
                }));
            });

            Effect::new(move || {
                let tip=arrow.and_then(|arrow|arrow.get()).filter(|tip|tip.is_connected());
                resources.arrow_size.set_value(if active.get() && alive.get() && !detached.get() {
                    tip.and_then(|tip|crate::observe_resize(&tip,move ||{moved.try_update(|n|*n+=1);})).map(SendWrapper::new)
                }else{None});
            });

            let deactivate: Arc<dyn Fn() + Send + Sync> = Arc::new({
                let weak = Arc::downgrade(&stack.inner);
                move || {
                    let Some(inner) = weak.upgrade() else { return; };
                    let stack = OverlayStack { inner, moved };
                    if resources.registered.try_get_untracked().flatten().is_none() { return; }
                    resources.release(&stack);
                    let event = Cancelable::new(());
                    if let Some(callback) = on_close_auto_focus { callback.try_run(event.clone()); }
                    if event.default_prevented() { return; }
                    let root: Option<Element> = node.try_get_untracked().flatten().map(Into::into);
                    if !holds_focus(root.as_ref()) { return; }
                    let explicit = trigger_id.try_get_untracked().and_then(|id|document().get_element_by_id(&id))
                        .filter(|element| stack.fallback().contains(Some(element)))
                        .or_else(||match anchor.try_get_untracked(){Some(Anchor::Element(element))=>Some((*element).clone()),_=>None});
                    let mut targets: Vec<_> = explicit.into_iter().chain(returns_to.try_get_value().flatten().map(|element| (*element).clone())).collect();
                    let parent_id = parent.and_then(|parent| parent.id.try_get_untracked().flatten());
                    let parent = {let inner=stack.lock();parent_id.and_then(|id|inner.layers.iter().find(|layer|layer.id==id)).map(|layer|(*layer.panel).clone())};
                    let fallback = stack.fallback();
                    // Native pointer focus follows pointerdown. Wait for that default
                    // action, then preserve any valid focus the interaction established.
                    let scope_alive=weak.clone();
                    crate::runtime::defer(move || {
                        if scope_alive.upgrade().is_none() || resources.registered.try_get_untracked().flatten().is_some() {return;}
                        let current=document().active_element();
                        let vacant=current.as_ref().is_none_or(|active|document().body().is_some_and(|body|AsRef::<Element>::as_ref(&body)==active)
                            || active==&fallback || root.as_ref().is_some_and(|root|root.contains(Some(active))));
                        if !vacant || !fallback.is_connected() {return;}
                        if let Some(target)=targets.drain(..).find(|element|element.is_connected() && element.matches(FOCUSABLE).unwrap_or(false) && element.dyn_ref::<HtmlElement>().is_some_and(reachable)) {focus(&target);}
                        else if let Some(parent)=parent.filter(|element|element.is_connected() && element.dyn_ref::<HtmlElement>().is_some_and(reachable)) {focus_first(&parent);}
                        else {focus(&fallback);}
                    });
                }
            });
            provide_context(LayerRelease(deactivate.clone()));
            Effect::new({
                let stack = stack.clone();
                let on_close = on_close.clone();
                let deactivate = deactivate.clone();
                move || {
                    let parent_id = parent.and_then(|parent| parent.id.get());
                    if !active.get() || !alive.get() || detached.get() || parent.is_some_and(|parent| !parent.notification && parent_id.is_none()) {
                        deactivate(); return;
                    }
                    let Some(element) = node.get() else { return; };
                    let Some(surface) = panel.unwrap_or(node).get() else { return; };
                    let Some(host) = children_host.get() else { return; };
                    let Some(notifications) = notifications_host.get() else { return; };
                    if resources.registered.get_untracked().is_some() { return; }
                    let surface: Element = surface.into();
                    let host: Element = host.into();
                    let notifications: Element = notifications.into();
                    if notifications.parent_element().as_ref() != Some(&surface) { let _ = surface.append_child(&notifications); }
                    if host.parent_element().as_ref() != Some(&surface) { let _ = surface.append_child(&host); }
                    resources.size.set_value(crate::observe_resize(&surface, move || { moved.try_update(|n| *n += 1); }).map(SendWrapper::new));
                    returns_to.set_value(document().active_element().filter(|element|element.is_connected()).map(SendWrapper::new));
                    let close = on_close.clone() as Arc<dyn Fn() + Send + Sync>;
                    let ask = on_close.clone();
                    let dismiss = Arc::new(move |_| ask());
                    let id = stack.push(parent_id, parent.is_some_and(|parent|parent.notification), notification_events.map(|events| events.viewport), modal, element.into(), surface.clone(), host, notifications, deactivate.clone(), close, dismiss, on_escape_key_down, on_pointer_down_outside, on_focus_outside, on_interact_outside, anchor, trigger_id);
                    resources.registered.set(Some(id));
                    crate::runtime::defer(move || {
                        if resources.registered.try_get_untracked().flatten()!=Some(id) {return;}
                        let event=Cancelable::new(());
                        if document().active_element().is_some_and(|active| {
                            surface.contains(Some(&active))
                                && active.dyn_ref::<HtmlElement>().is_some_and(reachable)
                        }) {event.prevent_default();}
                        if let Some(callback)=on_open_auto_focus {callback.try_run(event.clone());}
                        if resources.registered.try_get_untracked().flatten()!=Some(id) {return;}
                        if !event.default_prevented() && modal {focus_first(&surface);}
                    });
                }
            });
            on_cleanup(move || { resources.watch(&stack, false); deactivate(); resources.release(&stack); });

            view! {
                <Portal mount=root>
                    <div node_ref=node class=class.clone() data-testid=test_id.clone()
                        on:pointerenter=move |_| { if let Some(events) = notification_events { events.on_host_change.run(()); } }
                        on:pointermove=move |_| { if let Some(events) = notification_events { events.on_host_change.run(()); } }
                        on:pointerleave=move |_| { if let Some(events) = notification_events { events.on_host_change.run(()); } }
                        on:focusin=move |_| { if let Some(events) = notification_events { events.on_host_change.run(()); } }
                        on:focusout=move |event| { if let Some(events) = notification_events { events.on_focus_out.run(event.related_target().and_then(|target| target.dyn_into::<Element>().ok())); } }
                        hidden=move || !active.get() || parent.is_some_and(|parent| !parent.notification && parent.id.get().is_none())
                        style:visibility=move || (!alive.get() || detached.get()).then_some("hidden")
                        style:pointer-events=move || (!alive.get() || detached.get()).then_some("none")
                        inert=move || !alive.get() || detached.get()
                        data-rustify-issue=move || position_issue.get()
                        role=if modal && panel.is_none() { "dialog" } else { "presentation" } aria-labelledby=labelled_by.clone() aria-describedby=described_by.clone()
                        data-side=move || placement.get().side.as_str() data-align=move || placement.get().align.as_str()
                        dir=move || direction.get().attribute() lang=move || locale.get().tag()
                        style:left=move || format!("{}px",left.get()) style:top=move || format!("{}px",top.get())>
                        {children()}
                        <div node_ref=children_host data-rustify-layer-children="" />
                        <div node_ref=notifications_host data-rustify-layer-notifications="" />
                    </div>
                </Portal>
            }
        }));
        leptos::tachys::reactive_graph::OwnedView::new_with_owner(view, owner)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn outside_hooks_share_cancellation_and_only_uncanceled_interaction_closes() {
        use leptos::prelude::*;
        use std::sync::{Arc, Mutex};
        Owner::new().with(|| {
            let calls = Arc::new(Mutex::new(Vec::new()));
            let specific = Callback::new({
                let calls = calls.clone();
                move |event: Cancelable<LayerDismiss>| {
                    calls.lock().unwrap().push(*event.value());
                    event.prevent_default();
                }
            });
            let interact = Callback::new({
                let calls = calls.clone();
                move |event: Cancelable<LayerDismiss>| {
                    assert!(event.default_prevented());
                    calls.lock().unwrap().push(*event.value());
                }
            });
            dismiss_outside(
                Some(specific),
                Some(interact),
                LayerDismiss::FocusOutside,
                || panic!("canceled outside interaction closed"),
            );
            assert_eq!(
                *calls.lock().unwrap(),
                [LayerDismiss::FocusOutside, LayerDismiss::FocusOutside]
            );
            dismiss_outside(None, None, LayerDismiss::PointerOutside, || {
                calls.lock().unwrap().clear()
            });
            assert!(calls.lock().unwrap().is_empty());
        });
    }

    #[test]
    fn cancellation_is_shared_only_with_clones_of_the_same_event() {
        let event = Cancelable::new("choice");
        let callback_event = event.clone();
        callback_event.prevent_default();
        assert!(event.default_prevented());
        assert_eq!(event.value(), &"choice");
        assert!(!Cancelable::new("choice").default_prevented());
    }

    #[cfg(feature = "ssr")]
    #[test]
    fn portal_static_placeholder_requires_an_explicit_client_only_contract() {
        use leptos::prelude::*;
        Owner::new().with(|| {
            let issues = crate::StaticRenderIssues::default();
            crate::provide_static_render_issues(issues.clone());
            let html =
                view! { <ScopePortal client_only=true><p>"Browser content"</p></ScopePortal> }
                    .to_html();
            assert!(html.contains("portal-placeholder"));
            assert!(!html.contains("Browser content"));
            assert!(issues.messages().is_empty());
            let _ =
                view! { <ScopePortal><p>"Unsupported initial content"</p></ScopePortal> }.to_html();
            assert_eq!(issues.messages().len(), 1);
        });
    }

    #[cfg(feature = "ssr")]
    #[test]
    fn inactive_layer_is_a_valid_closed_static_initial_state() {
        use leptos::prelude::*;
        Owner::new().with(|| {
            let issues = crate::StaticRenderIssues::default();
            crate::provide_static_render_issues(issues.clone());
            let html = view! { <Layer active=false anchor=Anchor::Centred on_close=||{}><p>"Retained but closed"</p></Layer> }.to_html();
            assert!(!html.contains("Retained but closed"));
            assert!(issues.messages().is_empty());
        });
    }

    #[test]
    fn placement_reports_direction_and_the_edges_chosen_after_collision() {
        let anchor = LocalRect::new(100.0, 100.0, 80.0, 30.0);
        assert_eq!(
            place(
                Some(anchor),
                (160.0, 120.0),
                (1000.0, 800.0),
                crate::Direction::RightToLeft,
                true
            ),
            (
                (20.0, 130.0),
                LayerPlacement {
                    side: Side::Bottom,
                    align: Align::Start
                }
            )
        );
        let anchor = LocalRect::new(900.0, 700.0, 80.0, 30.0);
        assert_eq!(
            place(
                Some(anchor),
                (160.0, 120.0),
                (1000.0, 800.0),
                crate::Direction::LeftToRight,
                true
            ),
            (
                (820.0, 580.0),
                LayerPlacement {
                    side: Side::Top,
                    align: Align::End
                }
            )
        );
        let unfitted = place(
            Some(anchor),
            (160.0, 120.0),
            (1000.0, 800.0),
            crate::Direction::LeftToRight,
            false,
        );
        assert_eq!(unfitted.0, anchor.below());
        assert_eq!(
            unfitted.1,
            LayerPlacement {
                side: Side::Bottom,
                align: Align::Start
            }
        );
    }

    #[test]
    fn dismiss_hooks_cancel_only_the_default_close_of_that_semantic_event() {
        use leptos::prelude::*;
        use std::sync::{
            atomic::{AtomicUsize, Ordering},
            Arc,
        };
        Owner::new().with(|| {
            let closes = Arc::new(AtomicUsize::new(0));
            let canceled = Callback::new(|event: Cancelable<LayerDismiss>| {
                assert_eq!(*event.value(), LayerDismiss::Escape);
                event.prevent_default();
            });
            dismiss(Some(canceled), LayerDismiss::Escape, || {
                closes.fetch_add(1, Ordering::SeqCst);
            });
            assert_eq!(closes.load(Ordering::SeqCst), 0);
            dismiss(None, LayerDismiss::PointerOutside, || {
                closes.fetch_add(1, Ordering::SeqCst);
            });
            assert_eq!(closes.load(Ordering::SeqCst), 1);
        });
    }

    #[test]
    fn a_region_anchor_is_measured_from_the_region_it_belongs_to() {
        let region = LocalRect::new(100.0, 50.0, 400.0, 240.0);
        let anchor = LocalRect::new(80.0, 60.0, 72.0, 52.0);
        assert_eq!(
            anchor.in_viewport(region),
            LocalRect::new(180.0, 110.0, 72.0, 52.0)
        );
    }

    #[test]
    fn a_layer_sits_under_its_anchor_and_shares_its_left_edge() {
        assert_eq!(
            LocalRect::new(180.0, 110.0, 72.0, 52.0).below(),
            (180.0, 162.0)
        );
    }

    #[test]
    fn a_fitted_layer_stays_under_its_anchor_while_it_fits_there() {
        let anchor = LocalRect::new(100.0, 50.0, 80.0, 30.0);
        assert_eq!(
            anchor.below_within((160.0, 120.0), (1000.0, 800.0)),
            anchor.below()
        );
    }

    #[test]
    fn a_fitted_layer_moves_left_rather_than_past_the_right_edge() {
        // A point near the right edge, as a context menu opens at.
        let point = LocalRect::new(950.0, 100.0, 0.0, 0.0);
        assert_eq!(
            point.below_within((160.0, 120.0), (1000.0, 800.0)),
            (840.0, 100.0)
        );
    }

    #[test]
    fn a_fitted_layer_goes_over_its_anchor_when_it_does_not_fit_under() {
        let anchor = LocalRect::new(100.0, 700.0, 80.0, 30.0);
        assert_eq!(
            anchor.below_within((160.0, 120.0), (1000.0, 800.0)),
            (100.0, 580.0)
        );
    }

    #[test]
    fn a_fitted_layer_that_fits_neither_side_is_as_low_as_the_viewport_allows() {
        let anchor = LocalRect::new(100.0, 60.0, 80.0, 30.0);
        assert_eq!(
            anchor.below_within((160.0, 120.0), (1000.0, 160.0)),
            (100.0, 40.0)
        );
        // Larger than the viewport: the top-left corner stays reachable.
        assert_eq!(
            anchor.below_within((1200.0, 900.0), (1000.0, 800.0)),
            (0.0, 0.0)
        );
    }

    #[test]
    fn a_centred_layer_is_centred_and_never_placed_off_the_top_or_left() {
        assert_eq!(
            LocalRect::centred((1000.0, 800.0), (400.0, 200.0)),
            (300.0, 300.0)
        );
        // Larger than the viewport: the top-left corner stays reachable.
        assert_eq!(
            LocalRect::centred((300.0, 200.0), (400.0, 400.0)),
            (0.0, 0.0)
        );
    }

    #[test]
    fn tab_wraps_only_at_the_two_ends_of_a_modal_layer() {
        use TabFocus::{After, On};
        // (stops, focus, backwards, where Tab is sent; None = the browser's move)
        let table = [
            // Forward: only past the last stop.
            (3, On(0), false, None),
            (3, On(1), false, None),
            (3, On(2), false, Some(0)),
            // Backward: only before the first.
            (3, On(0), true, Some(2)),
            (3, On(1), true, None),
            (3, On(2), true, None),
            // One stop keeps the keyboard in both directions.
            (1, On(0), false, Some(0)),
            (1, On(0), true, Some(0)),
            // Focus on something that is not a stop - a heading focused by
            // script - wraps only if every stop is behind it going forward, or
            // ahead of it going back.
            (3, After(3), false, Some(0)),
            (3, After(1), false, None),
            (3, After(0), true, Some(2)),
            (3, After(2), true, None),
            // A layer with nothing to stop at leaves Tab to the browser.
            (0, After(0), false, None),
            (0, After(0), true, None),
        ];
        for (stops, focus, backwards, sent) in table {
            assert_eq!(
                wrap_tab(stops, focus, backwards),
                sent,
                "{stops} stops, {focus:?}, backwards: {backwards}"
            );
        }
    }
}
