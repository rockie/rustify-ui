//! Layers this scope owns: menus, tooltips and dialogs, drawn as DOM above
//! everything else in the scope, its GPU regions included.
//!
//! A layer is a component, so the application decides when one exists. The
//! stack decides the rest: where it sits, what it covers, which layer answers
//! Escape, and where focus goes when it leaves. A modal layer makes the rest
//! of its own scope inert rather than merely painting over it, so a control
//! underneath cannot be clicked, focused or read out while it is open.

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

#[cfg(target_arch = "wasm32")]
pub(crate) use dom::EditSession;
#[cfg(target_arch = "wasm32")]
pub use dom::{use_overlay, Anchor, Layer, OverlayStack};

#[cfg(target_arch = "wasm32")]
mod dom {
    use super::{LayerId, LocalRect};
    use leptos::html::Div;
    use leptos::portal::Portal;
    use leptos::prelude::*;
    use leptos::wasm_bindgen::closure::Closure;
    use leptos::wasm_bindgen::JsCast;
    use leptos::web_sys::{AddEventListenerOptions, Element, Event, HtmlElement, KeyboardEvent};
    use send_wrapper::SendWrapper;
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
    }

    struct Roots {
        container: Element,
        content: Element,
        overlay: Element,
    }

    /// Listeners the stack keeps only while it has something to place.
    struct Watchers {
        escape: Closure<dyn FnMut(KeyboardEvent)>,
        moved: Closure<dyn FnMut(Event)>,
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
        watchers: Option<SendWrapper<Watchers>>,
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
                inner.layers.is_empty()
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
            let escape = Closure::<dyn FnMut(KeyboardEvent)>::new(move |event: KeyboardEvent| {
                let key = event.key();
                if key != "Escape" && key != "Enter" {
                    return;
                }
                match route(&weak, &key) {
                    // Nothing above the application wanted it.
                    Routed::Ignored => {}
                    Routed::Taken => {
                        event.prevent_default();
                        event.stop_propagation();
                    }
                }
            });
            let moved = self.moved;
            let on_moved =
                Closure::<dyn FnMut(Event)>::new(move |_: Event| moved.update(|n| *n += 1));

            // Capture, so the stack answers before anything inside the scope
            // does: an open layer outranks the application's own commands.
            let capture = AddEventListenerOptions::new();
            capture.set_capture(true);
            let _ = inner
                .roots
                .container
                .add_event_listener_with_callback_and_add_event_listener_options(
                    "keydown",
                    escape.as_ref().unchecked_ref(),
                    &capture,
                );
            // Capture, on the document: any container between a layer and its
            // anchor can scroll, and only the capture phase sees all of them.
            let options = AddEventListenerOptions::new();
            options.set_capture(true);
            options.set_passive(true);
            let _ = document().add_event_listener_with_callback_and_add_event_listener_options(
                "scroll",
                on_moved.as_ref().unchecked_ref(),
                &options,
            );
            let _ = window()
                .add_event_listener_with_callback("resize", on_moved.as_ref().unchecked_ref());
            inner.watchers = Some(SendWrapper::new(Watchers {
                escape,
                moved: on_moved,
            }));
        }

        fn disarm(&self) {
            let mut inner = self.lock();
            let Some(watchers) = inner.watchers.take() else {
                return;
            };
            let _ = inner
                .roots
                .container
                .remove_event_listener_with_callback_and_bool(
                    "keydown",
                    watchers.escape.as_ref().unchecked_ref(),
                    true,
                );
            let _ = document().remove_event_listener_with_callback_and_bool(
                "scroll",
                watchers.moved.as_ref().unchecked_ref(),
                true,
            );
            let _ = window().remove_event_listener_with_callback(
                "resize",
                watchers.moved.as_ref().unchecked_ref(),
            );
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
                inner.layers.last().map(|layer| layer.close.clone()),
            )
        };
        if let Some(composing) = composing {
            if composing() {
                // The key is part of what is being typed. It commits nothing
                // and closes nothing.
                return Routed::Taken;
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
            if let Some(close) = close_top {
                close();
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

    /// The first thing inside `root` that can take focus, or `root` itself.
    fn focus_first(root: &Element) {
        let candidates = "a[href], button:not([disabled]), input:not([disabled]), \
             select:not([disabled]), textarea:not([disabled]), [tabindex]:not([tabindex='-1'])";
        match root.query_selector(candidates) {
            Ok(Some(first)) => focus(&first),
            _ => focus(root),
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
        on_close: impl Fn() + Send + Sync + 'static,
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
        use_overlay().map(|stack| {
            let root = stack.overlay_root();
            let class = format!("rustify-layer {class}");
            let labelled_by = (!labelled_by.is_empty()).then(|| labelled_by.clone());
            let described_by = (!described_by.is_empty()).then(|| described_by.clone());
            let moved = stack.moved();
            let node = NodeRef::<Div>::new();
            let on_close = Arc::new(on_close);
            let left = RwSignal::new(0.0f64);
            let top = RwSignal::new(0.0f64);
            let registered = RwSignal::new(None::<LayerId>);
            let returns_to: StoredValue<Option<SendWrapper<Element>>> = StoredValue::new(None);

            Effect::new({
                let on_close = on_close.clone();
                move || {
                    moved.get();
                    let anchor = anchor.get();
                    let Some(element) = node.get() else {
                        return;
                    };
                    let Some(rect) = anchor.viewport_rect() else {
                        // Whatever it was anchored to is gone.
                        on_close();
                        return;
                    };
                    let (x, y) = match rect {
                        Some(rect) => rect.below(),
                        None => LocalRect::centred(
                            viewport(),
                            (
                                element.offset_width() as f64,
                                element.offset_height() as f64,
                            ),
                        ),
                    };
                    left.set(x);
                    top.set(y);
                }
            });

            Effect::new({
                let stack = stack.clone();
                let on_close = on_close.clone();
                move || {
                    let Some(element) = node.get() else {
                        return;
                    };
                    if registered.get_untracked().is_some() {
                        return;
                    }
                    let element: Element = element.into();
                    returns_to.set_value(
                        document()
                            .active_element()
                            .filter(|active| active.is_connected())
                            .map(SendWrapper::new),
                    );
                    let on_close = on_close.clone();
                    let close = Arc::new(move || on_close()) as Arc<dyn Fn() + Send + Sync>;
                    let id = stack.push(modal, element.clone(), close);
                    registered.set(Some(id));
                    if modal {
                        focus_first(&element);
                    }
                }
            });

            on_cleanup(move || {
                if let Some(id) = registered.get_untracked() {
                    stack.remove(id);
                }
                // Only a layer that still holds the keyboard hands it back: if
                // the user has already moved on, taking focus would be the
                // theft this exists to prevent.
                if !holds_focus(node.get_untracked().map(Into::into).as_ref()) {
                    return;
                }
                match returns_to.get_value() {
                    Some(element) if element.is_connected() => focus(&element),
                    _ => focus(&stack.fallback()),
                }
            });

            view! {
                <Portal mount=root>
                    <div
                        node_ref=node
                        class=class.clone()
                        data-testid=test_id.clone()
                        role=if modal { "dialog" } else { "presentation" }
                        aria-modal=modal.then_some("true")
                        aria-labelledby=labelled_by.clone()
                        aria-describedby=described_by.clone()
                        style:left=move || format!("{}px", left.get())
                        style:top=move || format!("{}px", top.get())
                    >
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
}
