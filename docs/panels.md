---
description: Build controlled DOM panels, safe confirmations and notifications with shared raw and themed parts.
---
# Panels and feedback

Use these source APIs for a DOM application. They require no GPU feature and do
not add Makepad components. The panel and feedback additions described here are
not in the published 0.2.1 packages. Use the SDK, components, CSS and examples
from the same source checkout; see the [migration guide](dom-components-migration.md)
for changed calls and the [catalogue](components.md) for individual fields.

## Prepare the DOM runtime and styles

Start from the [DOM quick start](quickstart.md#a-dom-application-without-gpu-regions)
and retain the current [consumer path dependencies](../tests/dom-only/consumer/Cargo.toml)
for these source APIs. The quick start's registry substitutions describe the
released API. The default `rustify-ui` path dependency enables no GPU feature.
Boot the DOM loader before mounting a scope, and construct these components under that
scope. A Toast provider also belongs inside the mounted scope. Its resources
end with its Owner or the instance abort signal.

Choose one styling path:

| Entry | External stylesheets |
| --- | --- |
| `rustify_components::primitives::<family>` | `rustify_ui::runtime::DOM_RUNTIME_CSS` plus `rustify_components::PRIMITIVES_CSS`, then your own skin |
| `rustify_components::<family>` and convenience exports | DOM runtime CSS plus `rustify_components::CSS`, or compile the application's Tailwind input importing [sdk.css](../crates/rustify-components/css/sdk.css) once |

The raw and themed parts share state, keyboard behavior, focus and scheduling.
Raw parts supply structure without a palette, font or elevation. Themed sources
are available as `styles::PANELS`, `styles::CONTENT` and `styles::FEEDBACK`; these
skin fragments alone do not replace structural rules. Follow the
[CSS setup](dom-components-migration.md#styles-and-static-rendering) when compiling
utilities. Component rules use the components layer so caller utilities can
change their presentation without a global reset.

## Choose a panel

| Need | Public parts | State and semantics |
| --- | --- | --- |
| Switch between local sections | `tabs::{Root, List, Trigger, Content}` | `value: Signal<Option<String>>`, explicit `entries: Signal<Vec<PanelEntry>>`, `on_change`; automatic activation by default, or `ActivationMode::Manual` |
| Expand sections | `accordion::{Root, Item, Header, Trigger, Content}` | `AccordionValue::Single` or `Multiple`; `collapsible=false` prevents closing the single active item |
| Expand one region | `collapsible::{Root, Trigger, Content}` | `open`, `on_open_change`, optional disabled state |
| Select one setting | `segmented_control::{Root, Item}` | Controlled optional value and native radio-group form participation; not a tab panel |
| Navigate to a page | `tab_nav::{Root, Link}` | Real hrefs and `active`; not roving local tab selection |
| Edit a scoped task | `dialog::{Root, Trigger, Portal, Overlay, Content, Title, Description, Close}` | `open`, `on_open_change`; `modal=true` by default |
| Confirm a consequential action | `alert_dialog::{Root, Trigger, Portal, Overlay, Content, Title, Description, Cancel, Action}` | Safe initial Cancel focus; only Action invokes `on_action` |
| Show anchored controls | `popover::{Root, Trigger, Anchor, Portal, Content, Close, Arrow}` | Nonmodal by default; optional `modal=true` |
| Read supplemental link content | `hover_card::{Root, Trigger, Portal, Content, Arrow}` | A native anchor trigger, delayed hover and focus, selectable supplemental content |
| Describe one control | `tooltip::{Provider, Root, trigger_as, Portal, Content, Arrow}` | Noninteractive description; shared hover delay is optional |

Tabs, Accordion, Collapsible and the floating families have matching
`primitives` namespaces. SegmentedControl reuses `primitives::radio_group`;
TabNav has no separate raw family. Convenience `Tabs`, `TabPanel`, `Dialog` and `Tooltip` assemble these
same controlled parts.

The themed visual defaults are deliberately family-specific:

| Part | Defaults and visual fields |
| --- | --- |
| Tabs List / TabNav Root | Size 2 from 1/2; responsive wrap/justify, color and high_contrast |
| SegmentedControl Root | Size 2 from 1/2/3; Surface or Classic, default Surface; radius, without color/high_contrast |
| Dialog / AlertDialog Content | Size 3 from 1–4; Center alignment; Layout width/height/min/max with default max_width 600px |
| Popover Content | Size 2 from 1–4; Layout with default max_width 480px |
| HoverCard Content | Size 2 from 1–3; Layout with default max_width 480px |
| Tooltip Content | Responsive width/min_width/max_width; default max_width 360px, without a size or variant |
| Progress | Size 2 from 1/2/3; Surface from Classic/Surface/Soft; color/high_contrast/radius and animation duration_ms=5000 |
| Spinner / Skeleton | Spinner size 2 from 1/2/3; Skeleton accepts Layout dimensions; loading defaults true |

Use each family's named enums and `ALL`, existing Responsive/Layout types and
`margin: Signal<Margin>` where exposed. Accordion, Collapsible and Toast use a
Rustify appearance without invented reference size/variant scales. See the
[catalogue](components.md) for the content families and the full exposed subset.

Requests do not change the displayed value themselves. Set the supplied signal
to accept a request, keep it to refuse, or update it later to delay acceptance.
Use stable, unique item values. Read `data-rustify-issue` when a required part,
name or association is missing; invalid operations are disabled while readable
content remains available.

Tabs arrow keys move focus; automatic activation also requests selection, while
manual activation waits for Enter/Space. Horizontal movement follows direction.
Disabled items are skipped. Accordion exposes native headings and expansion
relationships. Closed retained content is hidden and inert, including when
`force_mount=true`; it does not become another keyboard destination.

## Compose a safe confirmation, choice and result

This component clears a local note only through the confirmation Action, keeps
the accepted delivery choice in the panel and adds an optional transient notice.
Mount it using the DOM quick start. The note and choice remain application state.

```rust
use leptos::prelude::*;
use rustify_components::{
    alert_dialog as confirm, callout, popover, Select, SelectOption, Toaster,
};
use rustify_ui::toast::{provide_toasts, ToastOptions, ToastTone};

#[component]
fn NotePanel() -> impl IntoView {
    let confirm_open = RwSignal::new(false);
    let settings_open = RwSignal::new(false);
    let select_open = RwSignal::new(false);
    let note = RwSignal::new("Review the delivery address".to_string());
    let delivery = RwSignal::new("standard".to_string());
    let result = RwSignal::new("The note has not been cleared.".to_string());
    let choices = Signal::stored(vec![
        SelectOption::new("standard", "Standard"),
        SelectOption::new("express", "Express"),
    ]);
    let notices = provide_toasts();

    view! {
        <section aria-labelledby="note-panel-heading">
            <h2 id="note-panel-heading">"Delivery note"</h2>
            <label>"Note"
                <input prop:value=move || note.get()
                    on:input=move |event| note.set(event_target_value(&event)) />
            </label>
            <confirm::Root open=confirm_open
                on_open_change=move |next| confirm_open.set(next)>
                <confirm::Trigger>"Clear note"</confirm::Trigger>
                <confirm::Portal>
                    <confirm::Overlay />
                    <confirm::Content>
                        <confirm::Title>"Clear this note?"</confirm::Title>
                        <confirm::Description>
                            "The local note will be emptied. Delivery stays unchanged."
                        </confirm::Description>
                        <confirm::Cancel>"Keep note"</confirm::Cancel>
                        <confirm::Action on_action=move |_| {
                            note.set(String::new());
                            result.set("The local note was cleared.".to_string());
                            notices.show("The local note was cleared.", ToastOptions {
                                tone: ToastTone::Success,
                                ..Default::default()
                            });
                        }>"Clear note"</confirm::Action>
                    </confirm::Content>
                </confirm::Portal>
            </confirm::Root>
            <popover::Root open=settings_open
                on_open_change=move |next| settings_open.set(next)>
                <popover::Trigger>"Delivery options"</popover::Trigger>
                <popover::Portal>
                    <popover::Content aria_label="Delivery options">
                        <Select value=delivery options=choices
                            on_change=move |next| delivery.set(next)
                            open=select_open on_open_change=move |next| select_open.set(next)
                            aria_label="Delivery speed" />
                        <popover::Close>"Done"</popover::Close>
                    </popover::Content>
                </popover::Portal>
            </popover::Root>
            <callout::Root>
                <callout::Text>{move || result.get()}</callout::Text>
            </callout::Root>
            <p>"Accepted delivery: "{move || delivery.get()}</p>
            <Toaster />
        </section>
    }
}
```

Callout is static explanation by default. Set its optional `role` to
`Some(callout::CalloutRole::Status)` or `Alert` only when an announcement is
needed. Keep consequential results and important actions available in the panel;
a disappearing notice must not be their only entry point.

## Dismissal, focus and native targets

A floating composition has one Root and one dedicated Portal:
`Root(Trigger, Portal(Overlay?, Content(..., Arrow?)))`. Content does not create
another Portal. Overlay is a sibling of Content, outside its dialog role.
`force_mount` belongs to Portal; Accordion/Collapsible use their Content prop.

Name Dialog with Title; AlertDialog also requires Description and Cancel.
AlertDialog initially focuses Cancel, ignores outside dismissal and invokes Action once per
activation. Its `on_action` receives `Cancelable<()>`; call `prevent_default()`
to prevent its default close request. Cancel never invokes the action. For an
asynchronous operation, keep application state authoritative: accept a close
only when appropriate, and display any failure in persistent content.

Content exposes cancellable `on_escape_key_down`, `on_pointer_down_outside`,
`on_focus_outside`, `on_interact_outside`, `on_open_auto_focus` and
`on_close_auto_focus` callbacks. The outside callbacks share cancellation for
one interaction. A cancelled Escape is consumed by that layer and does not close
its parent. Accepted close restores a usable trigger, then a usable parent/scope
fallback when the trigger is gone. Nested Select, Popover and Dialog use the same
scope stack; native text editing and composition receive Escape first.

Modal means **this scope**, not the whole page. Dialog, AlertDialog and modal
Popover omit `aria-modal`; the host and other scopes remain usable. The active
modal constrains focus and interaction only in its own scope. Do not add
`aria-modal="true"` to describe that local behavior as a page-wide modal.

Use `trigger_as`/`close_as`/`cancel_as`/`action_as` to decorate the family's
supported native target. Dialog/Popover triggers are native buttons; HoverCard
uses a native anchor. Existing listeners and attributes remain on that element;
caller cancellation precedes the component default. Generic React `asChild`, ref
composition, erased fragments and multiple-child Slot composition are absent.

For dynamic caller classes on a typed target, use additive directives such as
`.class(("p-8", move || expanded.get()))`. A whole-string
`.class(move || classes.get())` replaces the native class attribute when it
updates, removing the part and geometry tokens attached by Slot. This applies to
Badge/Card decoration and other typed native targets. Badge/Card convenience `class`
props accept a static String; put changing utility tokens on a typed target.

The Tooltip convenience entry now takes `trigger`, without a children wrapper:

```rust
<Tooltip open=help_open on_open_change=move |next| help_open.set(next)
    label="Dates use your local time zone"
    trigger=leptos::html::button().r#type("button").child("About dates") />
```

Tooltip is a description, not an interactive panel. Use HoverCard for selectable
supplemental text and links, or Popover for controls. Tooltip Provider defaults
to a 700ms delay and 300ms skip delay; HoverCard defaults to 700ms open and 300ms
close delay. `FloatingPosition` controls side, logical alignment, offsets,
collision boundaries and arrows. Its default is Bottom/Center; Select explicitly
uses Bottom/Start. Actual placement is exposed through `data-side/data-align`.

## Notifications use one scheduler

Choose one assembly under the mounted scope:

- Call `rustify_ui::toast::provide_toasts()` or `provide_toasts_with(config)` and
  render one themed `Toaster`. Use the returned handle's `show`, `dismiss`,
  `current` and `items`. The Toaster supplies Viewport and one Root per message.
- Render `toast::Provider` with `toast::Viewport` and controlled
  `toast::{Root, Title, Description, Action, Close}` parts. Supply Root's `open`
  and `on_open_change` just as for other controlled components.

Create a `ToastAction` callback under an Owner that survives the message, such as
the mounted application. A callback created inside transient dialog content ends
with that content even if the message remains visible.

`primitives::toast` reexports the core implementation; themed parts add a skin.
`Toast(snapshot)` remains available when mapping real handle items by `seq`; it
already creates a Root. Do not wrap it in another Root, or use it as Toaster's
body renderer. Fixed previews use Callout or an explicit controlled Root with
its provider context. A detached snapshot with an out-of-context Close is not a
working notification.

Defaults are Replace, one visible slot, 3200ms, rightward swipe with a 50px
threshold, and F8. Empty `viewport_label` uses the locale's notification name.
Set `duration_ms=0` for a persistent item. Configure FIFO explicitly:

```rust
use rustify_ui::toast::{provide_toasts_with, ToastConfig, ToastQueuePolicy};

let notices = provide_toasts_with(ToastConfig {
    policy: ToastQueuePolicy::Fifo,
    visible_limit: 2,
    ..Default::default()
});
```

Queued Roots retain their full reading time until visible. Handle Replace
atomically replaces its accepted list. Declarative Replace requests that the old
Root close once; if the application refuses, the new Root waits. Expiry is also
a request: refusal leaves the Root open without repeatedly requesting closure.
A new open session or duration change rearms it.

Viewport hover/focus pauses all visible sessions in **that Provider**, including
retained notification Portal descendants. Window blur and document hidden add
pause reasons; a swipe pauses its own Root. Only clearing every reason resumes
the remaining time. One cancellable earliest-deadline timer serves the Provider;
do not add a component-local timeout around these parts.

`ToastOptions` adds `importance`, optional `title` and optional `action` to
`duration_ms` and `tone`. Use `..Default::default()` in literals. Snapshot/options
are Clone, not Copy/Eq, because `ToastAction` carries a cancellable callback.
Action requires `alt_text` describing an equivalent persistent operation.
`ToastTone` changes appearance; Background/Foreground importance selects the
separate polite/assertive announcement channel. The interactive Viewport is not
a whole-region live announcement. Close has a localized accessible name.

F8 enters the first usable notification in the current scope; Tab exits
naturally. Escape requests closure of the focused notification after native
editing and its actual descendant layers, before its parent modal. Closing
returns to another usable notice, the valid previous focus or the scope fallback.
Swipe hooks can cancel start/move/end; a cancelled, reverse or short swipe does
not dismiss. Invalid configuration and duration inputs are reported by the
handle's `diagnostics()`.

### Notifications inside modal workflows

The same Viewport DOM moves into the current top modal's notification host, or
back to the scope overlay when no modal remains. Already-open notification
Popover/Select subtrees keep their Root/Content nodes, Owner, business open,
registration order, session and remaining time. Their parent/Portal host is
rebound without reopening, repeating autofocus or announcing again. A modal
opened from a notification does not become its own ancestor. Each Provider's
notification ownership remains distinct even within the same scope.

Migration retains a valid focused node in that subtree; it does not pull focus
from a usable outside node. A genuinely closed or invalid descendant still
cleans up normally. Scope disposal and instance abort release resources rather
than transferring notifications to the host page. Creation-time theme, locale
and direction continue to follow the creating context after DOM movement.

## Loading, progress and static HTML

`Progress` retains `value/max/indeterminate`. Parts expose
`progress::Root(value: Signal<Option<f64>>, max)` and Indicator. `None` is
indeterminate; `Some(value)` uses original units in `aria-valuenow` and
`aria-valuemax`, while only the visual width is a percentage. Use a finite
positive max and finite value in `0..=max`. Invalid values report diagnostics
and render indeterminate; invalid max falls back to 100. Give it `aria_label` or
`labelled_by`, with optional `value_label` for readable units.
Its width follows the parent layout; add `class="w-full"` in a grid that uses
`justify-items: start`, or supply the application's intended width.

`Spinner` defaults to `loading=true`, size 2 and a localized loading label.
Without children it is a named status while loading; with children it retains
them while hiding their reading and interaction. `Skeleton` retains children
without adding a status announcement. Both restore the same nodes and drafts
when loading ends, without asynchronous refocus. Keep an important loading
explanation outside Skeleton's hidden content.

`spinner_as(target, loading, label, size)` and
`skeleton_as(target, loading, layout)` preserve a native target and its listeners.
Spinner appends a status sibling, so its parent must accept flow content: a whole
table is a valid target, but a row/cell needs Skeleton or a legal outer container.
Convenience children use a div wrapper; an empty Skeleton is a span. An img/video
Skeleton cannot paint a normal element background while the replaced element is
hidden; wrap it in a legal container when that placeholder is needed. Loading
children are inert/hidden to assistive technology, so do not put the only cancel
or recovery control inside them. Reduced motion keeps the loading state while
removing the decorative animation.

Use the `ssr` feature only for **SSG build-time generation**, separately from
CSR/hydrate. Seed `ssr_scope`, provide the static theme and GeometryStyles, and
write collected geometry to an external stylesheet. Hydration needs the same
seed, accepted values and declared entry order. Accepted inline panels, content,
image fallback and progress values can be static HTML. Floating Dialog/Popover/
HoverCard/Tooltip must start closed; Portal emits a deterministic placeholder.
An initially open floating panel records StaticRenderIssues, and force_mount
does not bypass that boundary. Runtime ToastRegion/Viewport output is empty in
SSG; use Callout for a fixed result. No request-time or streaming SSR is supplied.

## Public examples and checks

The [themed DOM consumer](../tests/dom-only/consumer/src/panels.rs) and
[raw consumer](../tests/dom-only/primitives/src/panels.rs) demonstrate controlled
panels. Their [themed feedback](../tests/dom-only/consumer/src/feedback.rs) and
[raw feedback](../tests/dom-only/primitives/src/feedback.rs) entries exercise
loading and notification parts. [Panel browser scenarios](../tests/dom-only/panels.spec.ts),
[feedback scenarios](../tests/dom-only/feedback.spec.ts),
[static panel checks](../crates/rustify-components/tests/panels_ssg.rs) and
[static feedback checks](../crates/rustify-components/tests/feedback_ssg.rs) are
the corresponding verification entry points. These links identify source checks;
DOM assertions do not certify screen-reader announcements, OS IME or physical
touch-device behavior.
