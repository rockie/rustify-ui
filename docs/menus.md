---
description: Compose controlled command menus, native site navigation and toolbars.
---
# Menus, navigation and toolbars

Use these parts inside a mounted DOM scope, with the loader and stylesheet setup
from the [DOM quick start](quickstart.md#a-dom-application-without-gpu-regions).
Use matching source SDK and CSS; these APIs are not in registry 0.2.1.
The default SDK needs no GPU feature. Themed consumers load DOM runtime CSS plus
`rustify_components::CSS`, or compile an application input importing `sdk.css`.
Raw consumers use DOM runtime CSS plus `rustify_components::PRIMITIVES_CSS` and
their own skin. `styles::MENUS` exposes only the themed menu skin.

| Need | Themed module | Application state |
| --- | --- | --- |
| Commands behind a button | `dropdown_menu` | `open: Signal<bool>`, `on_open_change` |
| Commands for a focused or pointed-at object | `context_menu` | `open`, `on_open_change` |
| A top-level application command bar | `menubar` | `value: Signal<Option<String>>`, `on_value_change` |
| Real links with optional disclosure content | `navigation_menu` | `value`, `on_value_change` |
| One keyboard sequence of commands and settings | `toolbar` | Each command or toggle group owns its state |

Each module also exists under `rustify_components::primitives`. Both expressions
share behavior. A callback requests a change; only updating the supplied signal
accepts it. Keep item values unique and stable. These components do not persist
application state or bind displayed shortcuts to global keyboard commands.

## A command menu

```rust
use leptos::prelude::*;
use rustify_components::dropdown_menu as menu;

#[component]
fn DocumentMenu() -> impl IntoView {
    let open = RwSignal::new(false);
    let export = RwSignal::new(false);
    let result = RwSignal::new("No command selected");
    view! {
        <menu::Root open=open on_open_change=move |next| open.set(next)>
            <menu::Trigger>"Document"</menu::Trigger>
            <menu::Portal>
                <menu::Content aria_label="Document commands">
                    <menu::Group><menu::Label>"File"</menu::Label>
                        <menu::Item on_select=move |_| result.set("Saved")>"Save"</menu::Item>
                    </menu::Group>
                    <menu::Separator />
                    <menu::Sub open=export on_open_change=move |next| export.set(next)>
                        <menu::SubTrigger>"Export"</menu::SubTrigger>
                        <menu::Portal><menu::SubContent>
                            <menu::Item on_select=move |_| result.set("PDF exported")>"PDF"</menu::Item>
                        </menu::SubContent></menu::Portal>
                    </menu::Sub>
                </menu::Content>
            </menu::Portal>
        </menu::Root>
        <output>{move || result.get()}</output>
    }
}
```

Use exactly one Trigger and Portal per Root/Sub. Put Content under Portal, and
SubContent under the Sub's Portal. `on_select` receives `Cancelable<()>`; calling
`prevent_default()` keeps the menu open. Checkbox/radio change requests still
run; accept or refuse those through their value callbacks. `disabled` items
remain readable but are skipped by navigation.
Use `text_value` when visible children are not a suitable typeahead label, and
`described_by` to associate a visible explanation with a disabled command.

`CheckboxItem` takes `Signal<primitives::CheckedState>` and
`on_checked_change: Callback<bool>`; map the request to Checked or Unchecked.
Indeterminate is an application-supplied display state. `RadioGroup` takes an
optional string value and `on_value_change: Callback<String>`; its `RadioItem`
children have string values. Selecting the accepted radio value emits no change.
Themed checkbox/radio items include indicators; raw items compose ItemIndicator.

For a context menu, use `context_menu::Root`, Trigger, Portal and Content with
the same children. Trigger is a focusable div by default; `trigger_as(native)`
decorates one native element. Right-click, Shift+F10, the ContextMenu key and
touch long-press open at the target. A custom native target must be focusable
for keyboard access. `dropdown_menu::trigger_as` accepts a native button.

## Compose an application shell

The following patterns belong inside the same mounted component. Every open
value is independent application state.

```rust
use leptos::prelude::*;
use rustify_components::{dropdown_menu as menu, menubar as bar,
    navigation_menu as nav, toolbar};

#[component]
fn ApplicationShell() -> impl IntoView {
    let command = RwSignal::new(None::<String>);
    let section = RwSignal::new(None::<String>);
    let more = RwSignal::new(false);
    let status = RwSignal::new("Ready");
    view! {
        <bar::Root value=command on_value_change=move |next| command.set(next)
            aria_label="Application commands">
            <bar::Menu value="file"><bar::Trigger>"File"</bar::Trigger>
                <bar::Portal><bar::Content>
                    <bar::Item on_select=move |_| status.set("Saved")>"Save"</bar::Item>
                </bar::Content></bar::Portal>
            </bar::Menu>
            <bar::Menu value="edit"><bar::Trigger>"Edit"</bar::Trigger>
                <bar::Portal><bar::Content>
                    <bar::Item on_select=move |_| status.set("Undone")>"Undo"</bar::Item>
                </bar::Content></bar::Portal>
            </bar::Menu>
        </bar::Root>
        <nav::Root value=section on_value_change=move |next| section.set(next)
            aria_label="Documentation">
            <nav::List>
                <nav::Item value="learn"><nav::Trigger>"Learn"</nav::Trigger>
                    <nav::Content><nav::Link href="/guide/">"Guide"</nav::Link></nav::Content>
                </nav::Item>
                <nav::Item value="home"><nav::Link href="/" active=true>"Home"</nav::Link></nav::Item>
                <nav::Indicator />
            </nav::List>
            <nav::Viewport />
        </nav::Root>
        <toolbar::Root aria_label="Document tools">
            <toolbar::Button on_click=Some(Callback::new(move |_| status.set("Saved")))>"Save"</toolbar::Button>
            <toolbar::Separator />
            <menu::Root open=more on_open_change=move |next| more.set(next)>
                {menu::trigger_as(leptos::html::button().r#type("button")
                    .child("More").add_any_attr(toolbar::item(false.into())))}
                <menu::Portal><menu::Content>
                    <menu::Item on_select=move |_| status.set("Copied")>"Save copy"</menu::Item>
                </menu::Content></menu::Portal>
            </menu::Root>
            <input aria-label="Search documents" />
        </toolbar::Root>
        <p role="status">{move || status.get()}</p>
    }
}
```

Menubar Menu owns one Trigger/Portal/Content composition. Its Root uses one
optional value to identify the open top-level menu. Submenus use the same Sub
parts as DropdownMenu. Left/Right and hover switch enabled top-level entries.

NavigationMenu is a named `nav`, not a command menu. Content remains inside its
Item so Tab follows native document order; Viewport measures the active content
and supplies an outer surface. Indicator tracks the active trigger. `Sub` adds
an independent controlled nested list. Root defaults to a 200ms hover delay and
a 300ms skip-delay window. Link keeps href, target, rel and download; active
adds `aria-current="page"`. Modified clicks and new-tab navigation remain native.
Use `on_select` cancellation only when the application intentionally takes over
navigation. Keep loading links present and place progress or errors beside them.

Toolbar Button, Link and ToggleItem share one roving sequence. ToggleGroup uses
`Signal<Vec<String>>`, `on_change`, `SelectionMode` and `allow_empty`; inside a
toolbar it has group semantics. `toolbar::item(disabled_signal)` registers a
composed native focusable target, such as the menu trigger above. Inputs remain
ordinary tab stops; their arrows/Home/End retain editing behavior. The standalone
ToggleGroup `toolbar` option remains useful for a single group.

## Focus, closing and appearance

Menus support arrows, Home/End, typeahead and Enter/Space. Tab is consumed inside
menu content; use Escape, selection or an outside interaction to leave. Escape
or the backward horizontal arrow closes only the current submenu and returns
to SubTrigger. A selection or outside interaction closes the tree. This
submenu Escape behavior differs from Radix's whole-tree Escape closing.

DropdownMenu and ContextMenu default to scope-local modal behavior. They omit
page-wide `aria-modal`; other scopes remain usable. Content exposes cancellable
Escape, outside interaction and open/close autofocus hooks, following the
[panel lifecycle](panels.md). Closing restores focus when appropriate; removing
a trigger or disposing the scope releases the affected floating branch.

Menu Content accepts responsive MenuSize One/Two (default Two), MenuVariant
Solid/Soft (default Solid), optional `color` and `high_contrast`. SubContent
inherits those choices. Item color can emphasize a specific command. Menubar's
bar, NavigationMenu and Toolbar use Rustify semantic styles and caller `class`;
they do not declare size/variant or local palette props. Caller utilities remain
above component rules. On narrow screens constrain the shell and allow local
horizontal scrolling instead of changing navigation orientation implicitly.

Floating menus render closed during SSG; initially open requests are reported
through StaticRenderIssues. Inline NavigationMenu content can render its accepted
state. Use the same `ssr_scope` seed and initial props for hydration, with the
existing GeometryStyles collector and external stylesheet contract. There is
no request-time server rendering. See [migration](dom-components-migration.md)
for the retained data-driven Menu entry and its changed closing behavior.

Public API sources: [DropdownMenu](../crates/rustify-components/src/dropdown_menu.rs),
[ContextMenu](../crates/rustify-components/src/context_menu.rs),
[Menubar](../crates/rustify-components/src/menubar.rs),
[NavigationMenu](../crates/rustify-components/src/navigation_menu.rs),
[Toolbar](../crates/rustify-components/src/toolbar.rs).
