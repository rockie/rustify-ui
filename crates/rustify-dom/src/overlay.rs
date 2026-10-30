//! Layers this scope owns: menus, tooltips and dialogs, drawn as DOM above
//! everything else in the scope, its GPU regions included.
//!
//! A layer is a component, so the application decides when one exists. The
//! stack decides the rest: where it sits, what it covers, which layer answers
//! Escape, and where focus goes when it leaves. A modal layer makes the rest
//! of its own scope inert rather than merely painting over it, so a control
//! underneath cannot be clicked, focused or read out while it is open.

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
}
/// The resolved vertical side of an anchored layer.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Side {
    Top,
    #[default]
    Bottom,
}
impl Side {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Top => "top",
            Self::Bottom => "bottom",
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
                crate::theme::provider::write_snapshot(&element, &theme.get(), true);
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
pub(crate) use dom::EditSession;
#[cfg(target_arch = "wasm32")]
pub use dom::{use_overlay, Anchor, Layer, OverlayStack};

#[cfg(not(target_arch = "wasm32"))]
pub use host::{Anchor, Layer};

#[cfg(not(target_arch = "wasm32"))]
mod host {
    use leptos::prelude::*;

    /// A closed floating component's deterministic initial anchor.
    #[derive(Clone)]
    pub enum Anchor {
        Centred,
    }

    /// Like Leptos' Portal, a layer has no host DOM. Floating components
    /// generate their triggers with the layer initially closed; opening and
    /// positioning the layer belongs to the browser runtime.
    #[component]
    pub fn Layer(
        #[prop(optional)] modal: bool,
        #[prop(into)] anchor: Signal<Anchor>,
        #[prop(optional)] fit: bool,
        on_close: impl Fn() + Send + Sync + 'static,
        #[prop(optional_no_strip, into)] on_escape_key_down: Option<
            Callback<super::Cancelable<super::LayerDismiss>>,
        >,
        /// Enables outside dismissal for this layer. Omission preserves legacy
        /// outside-pointer behavior; cancellation suppresses the close request.
        #[prop(optional_no_strip, into)]
        on_pointer_down_outside: Option<Callback<super::Cancelable<super::LayerDismiss>>>,
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
        crate::StaticRenderIssues::record(
            "floating content must start closed; open it after attachment or declare a fixed client-only placeholder",
        );
        let _ = (
            modal,
            anchor,
            fit,
            on_close,
            on_escape_key_down,
            on_pointer_down_outside,
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
        dismiss, place, wrap_tab, Cancelable, LayerDismiss, LayerId, LayerPlacement, LocalRect,
        TabFocus,
    };
    use crate::listeners::{listen, ListenOptions, Listener};
    use leptos::html::Div;
    use leptos::portal::Portal;
    use leptos::prelude::*;
    use leptos::wasm_bindgen::JsCast;
    use leptos::web_sys::{Element, HtmlElement, HtmlInputElement, KeyboardEvent, Node};
    use send_wrapper::SendWrapper;
    use std::collections::HashSet;
    use std::sync::{Arc, Mutex, Weak};

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
        /// Centred in the viewport: what a dialog uses.
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
                Self::Centred => return Some(None),
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

    struct LayerRecord {
        id: LayerId,
        modal: bool,
        element: SendWrapper<Element>,
        close: Arc<dyn Fn() + Send + Sync>,
        dismiss: Arc<dyn Fn(LayerDismiss) + Send + Sync>,
        escape: Option<Callback<Cancelable<LayerDismiss>>>,
        outside: Option<Callback<Cancelable<LayerDismiss>>>,
        anchor: Signal<Anchor>,
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
        _scroll: Listener,
        _resize: Listener,
        _pointer: Listener,
        _mutation: Option<SendWrapper<MutationWatch>>,
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
        next: u64,
        edit: Option<EditSession>,
        watchers: Option<Watchers>,
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
                    next: 1,
                    edit: None,
                    watchers: None,
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
            self.lock().layers.last().map(|layer| layer.id)
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

        pub(crate) fn moved(&self) -> RwSignal<u32> {
            self.moved
        }

        /// Registers the scope's one native editing session. Ends any session
        /// still registered, so a scope never has two.
        pub(crate) fn begin_edit(&self, session: EditSession) {
            self.lock().edit = Some(session);
            self.arm();
        }

        pub(crate) fn end_edit(&self) {
            let empty = {
                let mut inner = self.lock();
                inner.edit = None;
                inner.layers.is_empty()
            };
            if empty {
                self.disarm();
            }
        }

        pub(crate) fn fallback_element(&self) -> Element {
            self.fallback()
        }

        fn push(
            &self,
            modal: bool,
            element: Element,
            close: Arc<dyn Fn() + Send + Sync>,
            dismiss: Arc<dyn Fn(LayerDismiss) + Send + Sync>,
            escape: Option<Callback<Cancelable<LayerDismiss>>>,
            outside: Option<Callback<Cancelable<LayerDismiss>>>,
            anchor: Signal<Anchor>,
        ) -> LayerId {
            let id = {
                let mut inner = self.lock();
                let id = LayerId(inner.next);
                inner.next += 1;
                inner.layers.push(LayerRecord {
                    id,
                    modal,
                    element: SendWrapper::new(element),
                    close,
                    dismiss,
                    escape,
                    outside,
                    anchor,
                });
                id
            };
            self.arm();
            self.refresh_inert();
            id
        }

        fn remove(&self, id: LayerId) {
            let empty = {
                let mut inner = self.lock();
                inner.layers.retain(|layer| layer.id != id);
                inner.layers.is_empty() && inner.edit.is_none()
            };
            self.refresh_inert();
            if empty {
                self.disarm();
            }
        }

        /// Everything below the topmost modal layer is inert: the scope's own
        /// content, and any layer opened before that one.
        fn refresh_inert(&self) {
            let inner = self.lock();
            let modal_at = inner.layers.iter().rposition(|layer| layer.modal);
            set_inert(&inner.roots.content, modal_at.is_some());
            for (index, layer) in inner.layers.iter().enumerate() {
                set_inert(&layer.element, modal_at.is_some_and(|top| index < top));
            }
        }

        /// Installs the scope's listeners on the way from no layers to one.
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
                    if key != "Escape" && key != "Enter" {
                        return;
                    }
                    match if event.is_composing() {
                        Routed::Native
                    } else {
                        route(&weak, &key)
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
            let moved = self.moved;
            // Capture, on the document: any container between a layer and its
            // anchor can scroll, and only the capture phase sees all of them.
            let scroll = listen(
                &document(),
                "scroll",
                ListenOptions {
                    capture: true,
                    passive: true,
                },
                move |_| moved.update(|n| *n += 1),
            );
            let resize = listen(&window(), "resize", ListenOptions::default(), move |_| {
                moved.update(|n| *n += 1)
            });
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
                    let Some(inner) = weak.upgrade() else {
                        return;
                    };
                    let top = {
                        let inner = inner
                            .lock()
                            .expect("overlay stack is never held across a panic");
                        inner.layers.last().map(|layer| {
                            (
                                layer.element.clone(),
                                layer.anchor,
                                layer.outside,
                                layer.dismiss.clone(),
                            )
                        })
                    };
                    let Some((element, anchor, Some(hook), close)) = top else {
                        return;
                    };
                    let Some(target) = event
                        .target()
                        .and_then(|target| target.dyn_into::<Node>().ok())
                    else {
                        return;
                    };
                    if element.contains(Some(&target)) {
                        return;
                    }
                    let trigger = anchor.try_get_untracked().and_then(|anchor| match anchor {
                        Anchor::Element(element) => Some((*element).clone()),
                        Anchor::Region { canvas, .. } => Some((*canvas).clone()),
                        Anchor::Centred => None,
                    });
                    if trigger.is_some_and(|trigger| trigger.contains(Some(&target))) {
                        return;
                    }
                    dismiss(Some(hook), LayerDismiss::PointerOutside, || {
                        close(LayerDismiss::PointerOutside)
                    });
                },
            );
            let mutation = observe_tree(&inner.roots.container, moved).map(SendWrapper::new);
            inner.watchers = Some(Watchers {
                _escape: escape,
                _scroll: scroll,
                _resize: resize,
                _pointer: pointer,
                _mutation: mutation,
            });
        }

        fn disarm(&self) {
            let watchers = self.lock().watchers.take();
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

    /// The command order of §6.2, in the one place that can enforce it:
    /// composition and native editing first, then the top layer, then whatever
    /// the application does with the key.
    fn route(weak: &Weak<Mutex<Inner>>, key: &str) -> Routed {
        let Some(inner) = weak.upgrade() else {
            return Routed::Ignored;
        };
        let (composing, cancel, close_top) = {
            let inner = inner
                .lock()
                .expect("overlay stack is never held across a panic");
            let edit = inner.edit.as_ref();
            (
                edit.map(|edit| edit.composing.clone()),
                edit.map(|edit| edit.cancel.clone()),
                inner
                    .layers
                    .last()
                    .map(|layer| (layer.escape, layer.dismiss.clone())),
            )
        };
        if let Some(composing) = composing {
            if composing() {
                // The key is part of what is being typed. It commits nothing
                // and closes nothing.
                return Routed::Native;
            }
            if key == "Escape" {
                if let Some(cancel) = cancel {
                    cancel();
                    return Routed::Taken;
                }
            }
            // Enter outside a composition belongs to the field itself, which
            // sees it on the way down.
            return Routed::Ignored;
        }
        if key == "Escape" {
            if let Some((hook, close)) = close_top {
                dismiss(hook, LayerDismiss::Escape, || close(LayerDismiss::Escape));
                return Routed::Taken;
            }
        }
        Routed::Ignored
    }

    fn set_inert(element: &Element, inert: bool) {
        if inert {
            let _ = element.set_attribute("inert", "");
        } else {
            let _ = element.remove_attribute("inert");
        }
    }

    fn focus(element: &Element) {
        if let Some(element) = element.dyn_ref::<HtmlElement>() {
            let _ = element.focus();
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

        let group = |element: &HtmlElement| {
            element
                .dyn_ref::<HtmlInputElement>()
                .filter(|input| input.type_() == "radio" && !input.name().is_empty())
                .map(|input| (input.name(), input.checked()))
        };
        let checked: HashSet<String> = stops
            .iter()
            .filter_map(|(_, element)| group(element))
            .filter_map(|(name, checked)| checked.then_some(name))
            .collect();
        let mut first_of_group = HashSet::new();
        stops.retain(|(_, element)| match group(element) {
            None => true,
            Some((name, is_checked)) if checked.contains(&name) => is_checked,
            Some((name, _)) => first_of_group.insert(name),
        });

        // Stable, so equal indices keep their document order.
        stops.sort_by_key(|(index, _)| if *index > 0 { *index } else { i32::MAX });
        stops.into_iter().map(|(_, element)| element).collect()
    }

    /// Whether `element` is on screen and neither disabled nor shut away.
    fn reachable(element: &HtmlElement) -> bool {
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
    fn focus_first(root: &Element) {
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
        anchor: StoredValue<Option<SendWrapper<crate::ResizeObservation>>>,
        tab: StoredValue<Option<Listener>>,
    }
    impl LayerResources {
        fn release(self, stack: &OverlayStack) {
            self.size.try_update_value(|held| {
                held.take();
            });
            self.anchor.try_update_value(|held| {
                held.take();
            });
            self.tab.try_update_value(|held| {
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
        #[prop(into)] anchor: Signal<Anchor>,
        /// Keeps an anchored layer inside the viewport, moving it off its
        /// anchor as little as that takes. Off, the layer stays exactly under
        /// its anchor wherever that is.
        #[prop(optional)]
        fit: bool,
        on_close: impl Fn() + Send + Sync + 'static,
        #[prop(optional_no_strip, into)] on_escape_key_down: Option<
            Callback<super::Cancelable<super::LayerDismiss>>,
        >,
        /// Enables outside dismissal for this layer. Omission preserves legacy
        /// outside-pointer behavior; cancellation suppresses the close request.
        #[prop(optional_no_strip, into)]
        on_pointer_down_outside: Option<Callback<super::Cancelable<super::LayerDismiss>>>,
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
        let directional = direction.is_some() || on_placement.is_some();
        let direction = direction.unwrap_or_else(crate::use_direction);
        use_overlay().map(|stack| {
            let root = stack.overlay_root();
            let class = format!("rustify-layer {class}");
            let labelled_by = (!labelled_by.is_empty()).then(|| labelled_by.clone());
            let described_by = (!described_by.is_empty()).then(|| described_by.clone());
            let moved = stack.moved();
            let node = NodeRef::<Div>::new();
            if let Some(theme) = theme {
                Effect::new(move || {
                    let snapshot = theme.get();
                    if let Some(node) = node.get() {
                        let element: leptos::web_sys::HtmlElement = node.into();
                        crate::theme::provider::write_snapshot(&element, &snapshot, true);
                    }
                });
            }
            let on_close = Arc::new(on_close);
            let left = RwSignal::new(0.0f64);
            let top = RwSignal::new(0.0f64);
            let placement = RwSignal::new(LayerPlacement::default());
            let placed = StoredValue::new(false);
            let alive = RwSignal::new(true);
            let reported_dead = StoredValue::new(false);
            let resources = LayerResources { registered: RwSignal::new(None), size: StoredValue::new(None), anchor: StoredValue::new(None), tab: StoredValue::new(None) };
            let returns_to: StoredValue<Option<SendWrapper<Element>>> = StoredValue::new(None);
            let restore_focus = RwSignal::new(true);

            Effect::new({
                let stack = stack.clone();
                let on_close = on_close.clone();
                move || {
                    moved.track();
                    let anchor = anchor.get();
                    let direction = direction.get();
                    let Some(element) = node.get() else { return; };
                    let Some(rect) = anchor.viewport_rect() else {
                        if alive.get_untracked() { alive.set(false); }
                        resources.release(&stack);
                        if !reported_dead.get_value() {
                            reported_dead.set_value(true);
                            on_close();
                        }
                        return;
                    };
                    reported_dead.set_value(false);
                    if !alive.get_untracked() { alive.set(true); }
                    let measured = element.get_bounding_client_rect();
                    let size = (measured.width(), measured.height());
                    let ((x, y), resolved) = if directional {
                        place(rect, size, if fit { visible_area() } else { viewport() }, direction, fit)
                    } else {
                        let coordinates = match rect {
                            Some(rect) if fit => rect.below_within(size, visible_area()),
                            Some(rect) => rect.below(),
                            None => LocalRect::centred(viewport(), size),
                        };
                        (coordinates, LayerPlacement::default())
                    };
                    left.set(x); top.set(y);
                    let changed = placement.get_untracked() != resolved || !placed.get_value();
                    placement.set(resolved);
                    placed.set_value(true);
                    if changed { if let Some(callback) = on_placement { callback.try_run(resolved); } }
                }
            });

            Effect::new(move || {
                let anchor = anchor.get();
                if !alive.get() { resources.anchor.set_value(None); return; }
                let observed = match anchor {
                    Anchor::Element(element) => Some((*element).clone()),
                    Anchor::Region { canvas, .. } => Some((*canvas).clone()),
                    Anchor::Centred => None,
                };
                resources.anchor.set_value(observed.and_then(|element| {
                    crate::observe_resize(&element, move || { moved.try_update(|n| *n += 1); }).map(SendWrapper::new)
                }));
            });

            Effect::new({
                let stack = stack.clone();
                let on_close = on_close.clone();
                move || {
                    if !alive.get() { return; }
                    let Some(element) = node.get() else { return; };
                    if resources.registered.get_untracked().is_some() { return; }
                    let element: Element = element.into();
                    resources.size.set_value(crate::observe_resize(&element, move || { moved.try_update(|n| *n += 1); }).map(SendWrapper::new));
                    let trigger = match anchor.get_untracked() { Anchor::Element(element) => Some((*element).clone()), _ => None };
                    returns_to.set_value(trigger.filter(|element| element.matches(FOCUSABLE).unwrap_or(false)).or_else(|| document().active_element()).filter(|element| element.is_connected()).map(SendWrapper::new));
                    let close = on_close.clone() as Arc<dyn Fn() + Send + Sync>;
                    let ask = on_close.clone();
                    let dismiss = Arc::new(move |reason| { restore_focus.set(reason != LayerDismiss::PointerOutside); ask(); });
                    let id = stack.push(modal, element.clone(), close, dismiss, on_escape_key_down, on_pointer_down_outside, anchor);
                    resources.registered.set(Some(id));
                    let event = Cancelable::new(());
                    if let Some(callback) = on_open_auto_focus { callback.try_run(event.clone()); }
                    if resources.registered.try_get_untracked().flatten() != Some(id) { return; }
                    if !event.default_prevented() && modal { focus_first(&element); }
                    if modal {
                        let root = element.clone();
                        resources.tab.set_value(Some(listen(&element, "keydown", ListenOptions::default(), move |event| {
                            if let Some(event) = event.dyn_ref::<KeyboardEvent>() { keep_tab_inside(&root, event); }
                        })));
                    }
                }
            });

            on_cleanup(move || {
                resources.release(&stack);
                let event = Cancelable::new(());
                if let Some(callback) = on_close_auto_focus { callback.try_run(event.clone()); }
                if event.default_prevented() || !restore_focus.try_get_untracked().unwrap_or(false) { return; }
                let trigger = match anchor.try_get_untracked() { Some(Anchor::Element(element)) => Some((*element).clone()), _ => None };
                let owned = holds_focus(node.try_get_untracked().flatten().map(Into::into).as_ref())
                    || holds_focus(trigger.as_ref());
                if !owned { return; }
                match returns_to.try_get_value().flatten() {
                    Some(element) if element.is_connected() => focus(&element),
                    _ => focus(&stack.fallback()),
                }
            });

            view! {
                <Portal mount=root>
                    <div node_ref=node class=class.clone() data-testid=test_id.clone()
                        hidden=move || !alive.get()
                        role=if modal { "dialog" } else { "presentation" }
                        aria-modal=modal.then_some("true") aria-labelledby=labelled_by.clone() aria-describedby=described_by.clone()
                        data-side=move || placement.get().side.as_str() data-align=move || placement.get().align.as_str()
                        dir=move || direction.get().attribute()
                        style:left=move || format!("{}px", left.get()) style:top=move || format!("{}px", top.get())>
                        {children()}
                    </div>
                </Portal>
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
