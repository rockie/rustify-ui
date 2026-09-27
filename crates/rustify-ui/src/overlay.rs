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

#[cfg(target_arch = "wasm32")]
mod dom {
    use super::{wrap_tab, LayerId, LocalRect, TabFocus};
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
                    match route(&weak, &key) {
                        // Nothing above the application wanted it.
                        Routed::Ignored => {}
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
            inner.watchers = Some(Watchers {
                _escape: escape,
                _scroll: scroll,
                _resize: resize,
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
            // Dropped with the layer, which takes the listener with it.
            let tab_loop = StoredValue::new(None::<Listener>);

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
                        // Inert keeps the keyboard off the scope behind the
                        // layer, but Tab past the last stop would still leave
                        // for whatever follows the layer on the page. Bubbling,
                        // so a control that uses Tab itself goes first.
                        let root = element.clone();
                        tab_loop.set_value(Some(listen(
                            &element,
                            "keydown",
                            ListenOptions::default(),
                            move |event| {
                                if let Some(event) = event.dyn_ref::<KeyboardEvent>() {
                                    keep_tab_inside(&root, event);
                                }
                            },
                        )));
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
