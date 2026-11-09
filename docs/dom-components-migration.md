---
description: Migrate source consumers to the configured theme and native DOM component APIs.
---
# DOM component migration

This guide describes the current source checkout. Published SDK and Docs CLI
0.2.1 use the previous APIs and schema v1. Use one matching source checkout for
the SDK, styles and source CLI; these changes do not publish a release.

## How to migrate

1. **Version pairing.** Published SDK/Docs 0.2.1 ship schema v1 and the
   previous component API. Use one matching source checkout for the SDK,
   styles and source CLI; the first registry release containing schema v2 has
   not been decided yet, and until it is, older published consumers are not
   compatible with it.
2. **Style entry.** Import `rustify_components::CSS` (or the structured
   `PRIMITIVES_CSS` sources) once per page; see
   [Styles and static rendering](#styles-and-static-rendering).
3. **Theme documents.** Author, resolve and scope themes through the v2
   document schema described in [the theme guide](themes.md) and
   [Author documents](#author-documents).
4. **Component replacement.** Walk the sections below for the families you
   use; for an exhaustive key-by-key view use
   [Legacy API destinations](components.md#legacy-api-destinations).
5. **Verify.** Rerun your own checks against the source checkout, plus the
   verification paths each catalogue row names.

## Buttons and links

`Button` remains a native button. Replace the former visual choices as follows:

| Previous API | Current API |
| --- | --- |
| `ButtonVariant::Default` | `ButtonVariant::Solid` |
| `Secondary` | `Soft` with `color=Some(AccentColor::Gray)` |
| `Outline`, `Ghost` | Same names |
| `Accent` | `Solid` with `color_role=ButtonColorRole::Accent` |
| `Destructive` | `Solid` with `color_role=ButtonColorRole::Destructive` |
| `Link` | A native `Link` for navigation; `Ghost` and an explicit underline class for actions |
| `ButtonSize::Default`, `Sm`, `Lg` | `Two`, `One`, `Three` |
| `ButtonSize::Icon` | `IconButton`, with a required `aria_label`, default size `Two` |
| `button::ButtonClass` | Component props and caller `class` |

`Classic`, `Surface`, `GhostOffset` and size `Four` are also available. Constants
such as `size=ButtonSize::One` work directly. A changing size uses
`Signal<Responsive<ButtonSize>>`; change an old `Signal<ButtonSize>` to derive
`Responsive::from(size.get())`.

Unselected colors consume the effective semantic primary pair, including author
overrides. Explicit `color` affects that component's palette roles. The old
semantic accent and destructive pairs remain distinct from brand families.
Caller utility classes still override component defaults.

Use `r#type=ButtonType::Submit` or `Reset` for native form behavior; the default
is `Button`. `loading` preserves the label and width, sets busy/disabled state,
and refuses activation and native submission. `unavailable` retains focus and
delivers a request for the application to judge. An `IconButton` requires a
nonempty accessible name; its icon is decorative.

`Link` keeps native navigation, target/rel attribute passthrough, Within/Exact
matching and `aria-current`. Disabled links lose their href. Its size, weight,
trim and wrap are responsive; `LinkUnderline` selects Auto/Always/Hover/None.

## Layout and text

`Responsive<T>` has optional `initial`, `xs`, `sm`, `md`, `lg` and `xl` fields.
The minimum widths are 0/520/768/1024/1280/1640 CSS px. Missing values inherit
the closest smaller choice; the names differ from Tailwind's breakpoints.

```rust,ignore
use rustify_components::{Flex, FlexDirection, Gap, Responsive, Spacing};

view! {
    <Flex direction=Responsive {
        initial: Some(FlexDirection::Column),
        sm: Some(FlexDirection::Row),
        ..Default::default()
    } gap=Gap { gap: Responsive::from(Spacing::Scale(3)), ..Default::default() }>
        {children}
    </Flex>
}
```

Box/Flex/Grid/Container/Section accept applicable `Layout` and `Margin`
structures; padding lives in `Layout::padding`. Flex/Grid accept `Gap`.
Each structure exposes the fields in the [parameter matrix](validation/dom-components-p1/field-matrix.md).
`Spacing::Scale` uses the resolved theme scale. Explicit CSS sizes retain their
own units and are not multiplied by theme scaling. Inset consumes the nearest
container padding; Reset decorates exactly one typed native Leptos element.

Text selects span/div/label/p independently of visual size. Heading selects
H1–H6 independently of its default visual size Six. Blockquote, Code, Kbd,
Em, Quote and Strong retain their native semantics. TextSize, TextWeight,
TextAlign, TextTrim and TextWrap provide the applicable responsive choices.
Text and Link leave inherited size intact when size is omitted. Truncate is
a nonresponsive boolean.

## Forms and selection

Use `rustify_components::primitives` for unstyled parts and the matching
`checkbox`, `radio_group`, `slider`, `text_field`, `select` and other modules
for themed parts. Convenience components use that same behavior; they do not
keep a second accepted value. [Forms](forms.md) includes complete composition,
submit/reset and Select examples.

| Previous use | Current source behavior / migration |
| --- | --- |
| A boolean Checkbox | The convenience API remains boolean plus `indeterminate`; parts use `CheckedState::{Unchecked, Checked, Indeterminate}`. Mixed is not a submitted checked value |
| A fixed radio/options view | Keep RadioGroup/RadioOption or compose radio_group Root/Item; checkbox_group is multi-value and selection cards reuse those groups |
| A scalar Slider | Keep Slider; add on_commit/on_cancel if completion matters. Multi-thumb Root uses `Vec<ThumbValue { id, value }>` and stable named thumbs, never array-position identity |
| A Select options array | Keep Select/SelectOption or provide Root entries independently of Item content. New Root uses Option<String>; the convenience API maps empty string to None |
| Keyboard focus in a Select listbox | Trigger now retains focus as the combobox. Read aria-activedescendant there; the referenced option id is opaque and exists only while mounted |
| Selected rows inferred from old data-active | Item data-state is checked/unchecked for accepted selection; data-highlighted is temporary navigation. A refused request never marks the candidate selected |
| Reading a number/colour editor's text for submission | Give the field a name and read native FormData; the named mirror represents accepted values. Focused Draft is completed by Enter, blur or FormRoot submit |
| Calling a save again after submit returns Save | Form owns the save callback and runs it once. Call submitted(result) when it finishes; SubmitButton inside FormRoot uses native submission and Root rules |
| Resetting only browser defaults | Supply on_reset to reset application values and call Form::reset. Root prevents native defaults from diverging from controlled signals; Busy refuses reset |
| Replacing all errors with one message | Constraint, Rule, Check and Application coexist. Field Message displays the first active source and exposes data-source; changed(field) withdraws old field results |

Inside a Field, metadata connects the first actual control with the label,
description and visible error. Give composite groups their own accessible
group name; name each slider thumb. Keep `name` for native submission, separate
from the field's validation key and generated DOM ids. Anonymous radio groups
remain independent and do not add private keys to FormData.

`NumberField` and `ColorField` keep `on_change` as the completion callback.
Their optional `on_preview` and `on_cancel` props take an explicit optional
callback: use `on_preview=Some(Callback::new(move |next| { /* preview */ }))`
and `on_cancel=Some(Callback::new(move |_| { /* cancel */ }))`, or omit them.
The callback type comes from `leptos::prelude::*`. Existing bare closures for
these two optional props need this wrapper.

The form visual sizes are 1/2/3 and support Responsive; use each component's
own size/variant enum and ALL constants. They are not Button's four-size type.
Checkbox, Radio, Switch, Slider and ordinary fields use classic/surface/soft;
cards use surface/classic. Select Trigger adds ghost-offset/ghost and Content
uses solid/soft. Colour, high contrast and radius apply only where declared in
the [catalogue](components.md). TextField Slot has its own colour and responsive
gap/px/pl/pr; Start/End follows direction while pl/pr retain physical meaning.

Pure parts consumers load DOM_RUNTIME_CSS plus PRIMITIVES_CSS. Themed consumers
load CSS or compile sdk.css once; sdk.css already imports the structure. Give
unstyled Slider a visible track/thumb size in application CSS. Typed Slot,
control_as, trigger_as and item_as decorate supported native Leptos elements
and preserve caller classes/listeners. They do not take arbitrary rendered
component trees. Cancelable Select/Layer hooks must cancel synchronously.

SSR Select starts closed. Its explicit entries already provide selected text
and native form options, while Content is absent. Initially open floating
content is rejected through StaticRenderIssues; open it after attachment.
Use identical scope seed, values and declared item sequence during hydration.

## Structured content and native scroll areas

The source checkout adds these DOM families. Where size or variant is exposed,
use that family's named enum and `ALL`; a shared theme does not add matching GPU
components.
The [public exports](../crates/rustify-components/src/lib.rs) and generated
[component catalogue](components.md) identify the supported fields.

| Family | Public entry and migration boundary |
| --- | --- |
| Avatar | `Avatar` requires src, alt and fallback and composes `avatar::{Root, Image, Fallback}`; `primitives::avatar` provides the same loading behavior without a skin. Size 1–9 defaults to 3; solid/soft defaults to soft. Give nondecorative images readable alt/fallback text; image attributes pass through to Image |
| Badge | `Badge` is a span; `badge_as(target, BadgeOptions)` decorates one native element. Size 1/2/3 defaults to 1; solid/soft/surface/outline defaults to soft. Color, high contrast, radius and margin are explicit fields |
| Card | `Card` is a div; `card_as(target, CardOptions)` keeps a native link or other target and its listeners. Size 1–5 defaults to 1; surface/classic/ghost defaults to surface. Layout and margin apply to the target; Card adds no click handler or tab stop |
| Callout | `callout::{Root, Icon, Text}` retains static text by default. Set `role=Some(callout::CalloutRole::Status)` or `Some(callout::CalloutRole::Alert)` only when an announcement is intended. Size 1/2/3 defaults to 2; soft/surface/outline defaults to soft. Icon is decorative by default; Root has color/high_contrast, without a radius prop |
| DataList | `data_list::{Root, Item, Label, Value}` renders dl/div/dt/dd. Root size 1/2/3 defaults to 2; responsive Horizontal/Vertical defaults to Horizontal and trim uses TextTrim. Item accepts responsive Align; Label has responsive width/min_width/max_width and local color/high_contrast. Keep multiple Value/dd nodes in their Item and retain long text |
| Table | `table::{Root, Header, Body, Row, ColumnHeaderCell, RowHeaderCell, Cell, Caption}` renders native table sections and cells. Root size 1/2/3 defaults to 2, surface/ghost defaults to ghost, and responsive TableLayout selects Auto/Fixed. Row align and Cell justify/dimensions/Padding are separate props; header scope, colspan/rowspan/headers and Caption preserve native relationships. Place Root inside an outer local scrollport for horizontal overflow |
| AspectRatio | `AspectRatio` and `aspect_ratio::Root` reexport `primitives::aspect_ratio::Root`. A finite positive ratio defaults to 1; invalid input reports ratio and retains the last usable external geometry. Children remain ordinary DOM content |
| Separator | `Separator`/`separator::Root` adds responsive size 1–4 (default 1), responsive orientation, Gray color by default and margin. `primitives::separator::Root` takes a single orientation signal and decorative=true by default. Nondecorative separators expose role/orientation; neither adds a keyboard stop |

Badge, Card, Callout, DataList and Table have no separate `primitives` namespace.
Their native tags, attribute passthrough and typed Badge/Card targets are the
composition surface. Badge/Card convenience `class` is a static String. For
dynamic utility tokens on typed targets, use an additive directive such as
`.class(("p-8", move || expanded.get()))`; whole-string reactive
`.class(move || classes.get())` replaces the native attribute and removes Slot
part/geometry tokens when it updates. Table is independent of DataTable's interactive grid,
sorting and virtualization. Images remain URL assets outside wasm.

Avatar static HTML keeps the fallback; hydration takes over that content before
handling current image results. Source generations prevent an old URL's load or
error result from replacing the current request. AspectRatio and explicit content
dimensions use the existing GeometryStyles collector during static rendering.

### ScrollArea parts and the convenience entry

Both `scroll_area` and `primitives::scroll_area` expose Root, Viewport, Scrollbar,
Thumb and Corner. The [shared native parts](../crates/rustify-components/src/primitives/scroll_area.rs)
keep Viewport as the only overflow element and keyboard entry. Root supplies its
default aria_label/labelled_by; explicit Viewport naming overrides it. Root itself
adds no tab stop, and custom bars are decorative to assistive technology.

```rust,ignore
use rustify_components::{scroll_area as scroll, Orientation};

view! {
    <scroll::Root class="h-48" aria_label="Entry history"
        boundary=scroll::Boundary::Stop>
        <scroll::Viewport test_id="history">
            <p>"Local entry history"</p>
        </scroll::Viewport>
        <scroll::Scrollbar orientation=Orientation::Horizontal><scroll::Thumb /></scroll::Scrollbar>
        <scroll::Scrollbar orientation=Orientation::Vertical><scroll::Thumb /></scroll::Scrollbar>
        <scroll::Corner />
    </scroll::Root>
}
```

Root accepts Scrollbars Vertical/Horizontal/Both (default Both), Visibility
Auto/Always/Hover/Scroll (default Hover), hide_delay_ms=600, Boundary, naming and
optional direction. Direction otherwise comes from the nearest provider. Themed
Root also accepts responsive ScrollAreaSize 1/2/3 (default 1), optional radius and
margin. Give Root a height or other layout constraint when its content must scroll.

The convenience `ScrollArea` composes these same parts. Its `class` now styles
Root; its existing `test_id` still identifies the real Viewport, and Root receives
`<test_id>-root` when a test id is supplied. Existing `scrollTop`/`scrollLeft`
inspection by that test id continues to address the native scrollport. Use an
explicit Viewport for native scroll-element attributes and listeners.

Native scrollLeft/scrollTop remain the source of truth. Custom thumb size/offset
come from actual viewport, content and track dimensions; the thumb is at least
18px and capped by track length. Track clicks and pointer dragging change the
native scroll position, using the same logical RTL mapping for reading and
writing. Content is not translated to simulate scrolling. Native wheel, touch,
inertia and viewport keyboard scrolling remain available; Boundary selects only
overscroll propagation or containment.

Auto displays overflowing axes; Always keeps tracks present and disables them
without overflow. Hover displays while hovered/focused and delays hiding; Scroll
starts with actual scrolling, with hover/focus retaining the visible session.
Active dragging keeps its target visible. Pointerup/cancel/lost capture and
unmount release capture and short listeners. Static HTML retains native content
and its name; measured thumb geometry starts empty until browser attachment.

## Controlled panels and feedback

Use the [panel guide](panels.md) for a complete DOM confirmation → auxiliary
choice → persistent result example and the raw/themed runtime prerequisites.
These APIs belong to the source checkout, not registry 0.2.1. Keep SDK, styles and
callers on the same source revision; rollback those pieces together. There is no
business-data or theme-document schema migration for these components.

| Previous call or expectation | Source API and behavior |
| --- | --- |
| `Tabs(active, tabs, on_activate)` / `TabPanel` | Retained convenience over `tabs::{Root, List, Trigger, Content}`. Parts use optional value, explicit PanelEntry declarations and on_change. New activation_mode defaults Automatic; Manual moves focus without selecting until Enter/Space. direction is explicit or inherited |
| A custom disclosure or linked tab strip | Accordion supports Single/Multiple; Collapsible controls one open region. SegmentedControl is a native radio-group setting, while TabNav preserves href navigation. They are separate semantics, not Tabs variants |
| `Dialog(open, title, description, ...)` | Same purpose over Root/Trigger/Portal/Overlay/Content/Title/Description/Close. AlertDialog adds safe Cancel and cancellable Action. Content has dismissal/autofocus hooks; one Portal owns the layer. Retained closed content is hidden/inert |
| Page-wide modal ARIA assumed from modal=true | Modal is scoped. Dialog, AlertDialog, modal Popover and direct modal Layer omit aria-modal; other scopes and the host remain usable |
| Tooltip children wrapper | Pass one native `trigger` or use `tooltip::trigger_as`. Keep existing attributes/listeners on it. HoverCard uses a native anchor and allows supplemental reading; Popover holds controls |
| `Progress(value=20, max=40)` asserted as aria-valuenow=50 | Now aria-valuenow=20 and aria-valuemax=40; only width is 50%. Parts use Option value; None/indeterminate omits aria-valuenow. Name with aria_label/labelled_by and optionally value_label |
| A permanently active Spinner | `loading` defaults true; false removes its status and restores retained children. Label defaults to localized Loading. `spinner_as` adds a status sibling and needs a flow-content parent |
| Replacing children with a placeholder | Skeleton/loading wrappers retain nodes and input drafts, hide/inert them while loading, then restore keyboard access without refocus. Use skeleton_as for native table rows/cells and a legal container for replaced images |
| `ToastOptions { duration_ms, tone }` | Add `..Default::default()` or supply importance/title/action. Zero duration is persistent; default duration remains 3200ms |
| Copy/Eq notification snapshots | Toast/ToastOptions are Clone because actions carry callbacks. Clone snapshots or map handle.items by seq |
| Fixed literal Toast previews | Use Callout or an explicit controlled toast::Root under its Provider. Real `Toast(snapshot)` owns one Root; never nest it inside another Root |
| A separate queue/timer around Toaster | Remove it. Core Provider/Root is the single scheduler used by primitives, themed parts, ToastHandle, ToastRegion and Toaster |

Toast defaults to Replace with visible_limit=1. Explicit ToastConfig can select
Fifo and a positive visible_limit. Queued entries do not spend reading time;
declarative replacement waits for the old Root's accepted close. A rejected
expiry close is not retried until a new session or duration change. Viewport
hover/focus pauses that Provider's visible Roots; blur/hidden and a Root's swipe
compose with those reasons. Clearing them resumes remaining time. duration_ms=0
stays until application dismissal.

Action needs alt_text and a cancellable on_action; Close has a localized name.
Tone is visual, importance chooses polite/assertive announcement. The whole
interactive Viewport is not a live region. Keep essential recovery actions in
persistent content. F8 enters the current scope's notifications; Tab leaves
naturally and Escape respects native editing and open notification descendants.

The current modal's notification host receives the same Viewport DOM. Existing
notification child layers rebind their parent and host while keeping Owner,
Root/Content identity, open state, registration order and remaining duration.
Migration does not reopen, autofocus or reannounce. A modal opened from a toast
cannot become its own ancestor. Closing the host migrates surviving independent
notification branches before normal descendant cleanup; scope disposal/instance
abort still releases everything. The creating theme/direction/locale remains in
force, and other Providers do not supply pause or Escape ownership.

Floating Portal starts closed in SSG and reports an initially open request via
StaticRenderIssues; force_mount cannot override that rule. ToastRegion/Viewport
are empty during SSG. Static accepted inline content and Callout remain available;
no request-time SSR is added.

## Menus, navigation and toolbars

The source SDK supplies matching raw/themed `dropdown_menu`, `context_menu`,
`menubar`, `navigation_menu` and `toolbar` modules. Use the
[menu guide](menus.md) for controlled application-shell examples, typed native
targets and keyboard behavior. Menubar/NavigationMenu Root use an optional
string value; DropdownMenu/ContextMenu Root use open. Toolbar groups retain
application-owned selection. The standalone ToggleGroup `toolbar` option is a
single-group convenience; Toolbar's own ToggleGroup participates in its parent
focus sequence with group semantics.

`Menu`, `MenuItem`, `MenuItemKind` and `anchor_at` retain their signatures for
data-driven or GPU/point-anchored commands. Menu is nonmodal, opens on the first
enabled item, and now requests closing for pointer/focus interaction outside
the anchor element or canvas. Interactions inside an anchor canvas remain the
application's responsibility through GPU actions. Its default skin is menu
size 2/solid. The DOM now uses div elements with menuitem roles instead of the
old ul/li/button shape; use roles or retained `menu-item-{id}` test ids instead
of tag selectors. Disabled reasons remain associated descriptions and shortcuts
remain display/ARIA metadata.

Use parts for a DOM trigger, check/radio items or submenus. Menu content consumes
Tab. Escape closes one submenu and returns to its SubTrigger; selection or an
outside interaction closes the tree. NavigationMenu links keep native navigation
and modifier/new-tab behavior; they are not command menu items. Roll source,
compiled CSS and examples back together when reverting this API change. No
theme or business-data schema changes are required.

## Styles and static rendering

Declare layer order before importing Tailwind's layers, then import the SDK
and the document runtime mappings into the application's one build:

```css
@layer theme, base, components, utilities;
@import "tailwindcss/theme.css" layer(theme);
@import "tailwindcss/utilities.css" layer(utilities) source(none);
@import "path/to/rustify-components/css/sdk.css";
@import "path/to/rustify-components/css/theme-v4.css";
@source "./src";
```

Regenerate the stylesheet after changing component classes. Applications
without their own Tailwind input can consume `rustify_components::CSS`.
Keep the SDK styles off the host's bare elements; no global reset is required.

[sdk.css](../crates/rustify-components/css/sdk.css) imports
[panels.css](../crates/rustify-components/css/panels.css),
[content.css](../crates/rustify-components/css/content.css) and
[feedback.css](../crates/rustify-components/css/feedback.css) and
[menus.css](../crates/rustify-components/css/menus.css) for themed skins, alongside [primitives.css](../crates/rustify-components/css/primitives.css)
for structural rules. `styles::PANELS`, `styles::CONTENT` and `styles::FEEDBACK`
and `styles::MENUS` expose those skin sources; `PRIMITIVES_CSS`/`styles::PRIMITIVES` expose the structural source.
Raw form, panel, media, scroll and feedback consumers load DOM_RUNTIME_CSS
plus PRIMITIVES_CSS and supply their own visual treatment. Themed consumers load
the compiled CSS bundle or compile sdk.css once. A skin fragment alone does not
supply the raw structure. Visual defaults remain beneath caller utilities.

For SSG build-time HTML, build with `ssr` separately from CSR/hydrate, seed `ssr_scope`, call
`provide_static_theme` with the complete snapshot and provide a `GeometryStyles`
collector. Write `styles.css()` into an external stylesheet. Use the same seed
and initial props when hydrating. Responsive rules share the existing scope
stylesheet lifecycle; invalid property values are available through
`GeometryStyles::diagnostics()` and do not terminate the application.

The browser `theme::Theme` entry reexports the SDK's ThemeScope, and
`theme::ThemeBoundary` uses the same effective source. Static rendering keeps
the existing `provide_static_theme` binding and external CSS contract.
`ThemePanel` is controlled: configuration and override counts are inputs,
changes are requests, and group reset requests explicitly cover both modes.

## Author documents

Use `base_values(mode)` for original author values and `effective_values(mode)`
for composed, unscaled values. Edit `overrides` to take precedence and remove
keys to reveal configuration/base. The former `.styles`, `.values()` and
`.values_mut()` accessors no longer describe the full author model.

`ThemeDocument::from_legacy` upgrades into Custom v2 without replacing author
colors. Configuration changes retain explicit overrides. `ThemeBoundaryPatch`
holds configuration and separate light/dark patches; `ThemeValuePatch::into()`
adapts a mode-independent sparse patch. See [Theme documents](themes.md) for
strict import rules, root preview mode and source-preserving local appearance.

## Existing exports and the legacy fact source

The complete per-key destination table is generated: see
[Legacy API destinations](components.md#legacy-api-destinations). It resolves
every baseline public key exactly once (exact keys win over wildcards) with a
fate, destination and reason. The baseline was extracted mechanically from
commit 11b69129, then completed against the old signatures: theme-component
props, impl methods keyed `file::Type::method` so receivers stay distinct
(`Theme::properties` and `ThemePatch::properties` are separate keys;
`ThemeFont::face` and `slot` are kept const methods), and the chart colour
tokens. Exceptions worth prose:

- `CSS`, `CANDIDATES_JSON` and `styles` package assets remain; structural rules
  come from `PRIMITIVES_CSS`/`styles::PRIMITIVES`, with skin sources in
  `styles::PANELS`, `styles::CONTENT` and `styles::FEEDBACK`. Pick one themed
  or unstyled CSS path.
- `Icon` and `Glyph` keep the same SVG wrapper; the glyph set is now seven
  (`DividerHorizontal` added) and `IconButton` reuses the icon path.
- `GeometryStyles`, `provide_geometry_styles` and `geometry` are the same
  stylesheet collector, extended for media rules and field diagnostics.
- `id` and `roving` keep their deterministic identity and keyboard-navigation
  helpers.
## Root Theme and the legacy Theme value

The existing `rustify_ui::Theme` (or `rustify_dom::Theme` for a DOM-only
consumer) remains a light/dark value palette. It has not been renamed into a
component. In a browser, give the new alias a local name when both are needed:

```rust,ignore
use leptos::prelude::*;
use rustify_components::theme::Theme as RootTheme;
use rustify_dom::{Theme as LegacyTheme, ThemeDocument};

let document = ThemeDocument::from_legacy(LegacyTheme::light(), LegacyTheme::dark());
// Resolve the document with the host's font/rem context, then mount:
view! { <RootTheme resolved=resolved /> }
```

`RootTheme` is only an import alias for the existing ThemeScope. It does not
wrap children or create a second mount. Supply it once inside an SDK mount;
use ThemeBoundary for local configuration and separate light/dark overrides.
Document appearance selects the root mode; an explicit `preview_mode` is a
preview choice, not a mutation of author appearance. Empty boundary patches
resume inheritance; invalid local candidates retain the parent snapshot and
report a diagnostic. See [the provider contract](themes.md#root-and-local-providers).

Legacy `ThemedScope`, `ThemeOverride`, `ThemePatch` and `Theme::light/dark`
consumption remain available. `ThemeDocument::from_legacy` and old v1 imports
select Custom configuration to preserve the old authored values. New
`ThemeDocument::standard` selects indigo/auto/translucent/medium/100%; changing
those choices does not remove explicit author overrides.

## JSON, CSS and spacing changes

| Previous access or format | Source checkout replacement |
| --- | --- |
| `ThemeDocument.styles`, `values(mode)`, `values_mut(mode)` | `base_values(mode)` / `base_values_mut(mode)` for original author tables; `effective_values(mode)` for unscaled composed values; edit sparse `overrides` for precedence |
| v1 `styles.light` / `styles.dark` JSON | Shared `import_json` accepts strict v1 and upgrades to Custom v2; use `export_json` for complete lossless v2 |
| Configuration inferred from final colors | Keep explicit `config` and `overrides`; author `accent` is a semantic role, while `accent_color` selects a named brand family |
| One current-mode patch reused as theme source | `ThemeBoundaryPatch` carries config and distinct `light`/`dark` maps; `ThemeValuePatch.into()` is the explicit same-patch-to-both adapter |
| Implicit v1 export | `export_json_v1_lossy` returns the JSON plus explicit losses; the old format cannot preserve configuration intent, override origins, full-radius roles or all derived geometry |
| Resolved CSS treated as an author document | `export_css` describes the resolved output; retain JSON to recover config, overrides, source strings and both author tables |

Source JSON input validates the whole candidate, both modes and the UTF-8
size limit. Unsupported versions, duplicate keys, mixed v1/v2 forms, malformed
values and unknown override keys are errors; unknown base string keys are
preserved and diagnosed. Display P3 Colors data does not expand the author
theme parser beyond its declared sRGB syntax.

Keep the utility unit, layout gap and component scale distinct. `spacing`
provides the utility unit (`--rustify-unit`, normally derived from .25rem);
`layout-gap` is an independent author value. Legacy `--spacing` remains its
legacy layout-gap projection. Theme step `Spacing::Scale(4)` reads
`--rustify-space-4` from the resolved snapshot; it does not apply scaling again.
`Spacing::Css("37px".into())` and explicit width/height strings stay 37px under
90/95/100/105/110 theme scaling. Breakpoints also stay fixed.

SSR renders with `ssr` separately from CSR/hydrate. Call `ssr_scope` with a
stable scope seed, bind the complete resolved snapshot with
`provide_static_theme`, and provide `GeometryStyles` before rendering children.
Publish the matching theme CSS and `GeometryStyles::css()` externally; the
static binding does not emit a browser provider or choose a new root mode.
Hydration uses the same seed and initial props. Scoped defaults and responsive
rules live in the components layer beneath utilities. CSSOM writes used by
provider/overlay projection may serialize a style attribute; they do not permit
arbitrary style strings or a relaxed Content Security Policy.

The source `build-web --release` pipeline removes the wasm `name` section to
avoid shipping expanded Rust generic names. Release wasm stack frames therefore
lose those function names; debug builds retain them. Export names and the
message-bridge fingerprint remain separate from this debugging metadata.

## Colors without a UI runtime

The source-only `rustify-colors` crate provides 31 named families with
light/dark, solid/alpha and sRGB/P3 strings, plus mode-independent black/white
alpha. Rust uses `ColorFamily::ALL`, `scale`, `css`, `black_alpha` and
`white_alpha`; steps are one-based 1–12. The optional Rust `p3` feature is
separate from static P3 CSS consumption.

Static consumers copy only the packaged assets they use, mark the receiving
element with `data-rustify-colors` and an explicit mode, and keep the asset
license. P3 styles import their relative sRGB fallback, so deploy both files.
No Colors selector changes unmarked host elements, `:root` or `.dark`.
See [the Colors package guide](../crates/rustify-colors/README.md) for exact
paths, markers and copying APIs. These assets require no React, Leptos,
Makepad, wasm or browser script.

## Rustify extensions

The extension families (DataTable, Tree, FilePicker/DropZone, Splitter,
PanelTabs, CommandPalette, Icon) gained new optional parameters: `empty_label`
on DataTable and Tree (empty-state words default to the SDK's locale-aware
message), and `invalid` on FilePicker and DropZone (refusal visuals plus
`aria-invalid`/described-by wiring). `Message` gained `Commands`,
`TypeCommand`, `NoMatches` and `ResizePanel` — exhaustive `Match` arms must be
added, and `Message::all()` now returns 23 entries instead of 19. `Glyph`
gained `DividerHorizontal`. PanelTabs tabs render behind a presentation frame
that carries `data-state`; the close control is a mouse-only target outside
the accessibility tree whose hover hint is `close_label` (no longer an
accessible name), and the Delete key closes the active closable tab.
Selected rows and tree items use paired accent colors, and extension parts
carry `data-rustify-part` markers for tooling.

## Default palette and color semantics

The default `--primary` and `--accent-foreground` values changed with the v2
palette; `--ring` is unchanged. Applications that pinned the old defaults
should re-check contrast after upgrading. New applications scope tokens with
`ThemeScope`/`ThemeBoundary`; the legacy `ThemedScope` paths remain available
by user decision with no removal planned.

Chart colours `chart-1`..`chart-5` remain theme tokens. Each falls back to a
named semantic token when a theme does not override it: `chart-1`→`--primary`,
`chart-2`→`--success`, `chart-3`→`--warning`, `chart-4`→`--destructive`,
`chart-5`→`--muted-foreground`. Applications keep the same token names; only
the fallback source is now explicit.

## Published and source version pairing

The nine published Docs/DOM/style packages remain 0.2.1. Their frozen CLI
framework pair and registry source checks remain intact. The workspace's
unchanged version string does not make the new source APIs or v2 schema
available from crates.io; `rustify-colors` has not been published by this work.
An old registry CLI cannot consume complete v2 author documents.

In Studio, **Export** defaults to complete JSON v2. **Download setup notes**
(下载接入说明) saves `theme-setup.txt` alongside your theme, without adding
metadata to JSON. It records the build's source revision, current explicit
preview mode and context, and the actual light/dark requested and resolved fonts.
Copy the listed external font files and distribute their original licenses from
`makepad/widgets/resources/FONT-LICENSES.md` in that checkout. Font URLs are
relative to deployed CSS; retain their paths or adjust them for your base/CDN.

Use one pinned checkout for the source SDK and stylesheet assets. Only Docs
consumers also need its source CLI. Static CSS consumers need no wasm or CLI;
compile exported Tailwind profiles through the existing Tailwind v4 build.
Build the Docs CLI with the existing unwind-capable profile:

```sh
mbx build --locked -p rustify-docs-cli --profile docs-cli
target/docs-cli/rustify-docs --build-info
```

A source CLI author project must resolve its three direct framework packages
(`rustify-docs`, `rustify-dom`, `rustify-components`) to their canonical paths
in that checkout, following the existing development-source pairing check.
Do not mix registry and source framework dependencies, replace them with Git
dependencies to bypass the canonical-path check, or modify the frozen public
release record to claim a new registry pair.

The complete Rust export includes
`mount_exported_preview(container, config, ThemeMode::Light, &font_context(16., false))`.
Use the setup notes' actual mode/rem/motion values; `container` is the application's
mount node and `config: MountConfig` supplies its unique scope and deployment
`base`. Keep the returned `AppHandle` alive. Ordinary `mount_exported_theme`
honors authored appearance; explicit preview leaves the author document intact.
For local providers use `Signal<ThemeBoundaryPatch>`, independent light/dark
patches and optional local appearance; validate with `ResolvedTheme::with_boundary`.
Adapt a legacy mode-independent `ThemeValuePatch` explicitly with `.into()`.

The [independent consumer recipe](../tests/theme-export/README.md) describes
compiling the generated Studio Rust snippet and JSON outside the editor, with
root/child-path assets and strict CSP. The integration also supplies a DOM-only
consumer with default SDK features; running these entry points and recording
the source pair are separate acceptance steps, not implied by this guide.

The [final source integration record](plan/DOM-COMPONENTS-P1.records/M7.md#最终源码配对)
pins SDK, CSS and source CLI to
`e745a4cfbb51a54e1d7838beadf6b1cd3881aec4`, with actual `--build-info`,
canonical author dependencies and browser evidence. Its SDK/runtime/CLI trees
match the UI fix `d712990c`. A version string alone does not identify that pair.

The first registry release that includes schema v2 has not been decided yet; until it is, older published consumers are not compatible with it.
## Storage upgrade, backup and rollback

Studio data belongs to the browser origin. Keep the previous deployment and its
matching manifest, lockfile and CLI available before changing versions. Close
older Studio tabs: they do not participate in the new storage lock protocol.

| Storage key | Purpose |
| --- | --- |
| `rustify-ui.theme-studio.v1` | The legacy library. The source upgrade reads it without rewriting it. |
| `rustify-ui.theme-studio.v2` | The new library: envelope version 2, revision, draft, named themes, preferences and optional migration metadata. |
| `rustify-ui.theme-studio.v1.backup.<sha256>` | Exact original v1 text, identified by its SHA-256 digest. A different original produces a separate backup. |

An existing v2 library is checked first. A bad v2 is protected rather than
replaced from v1. Only when v2 is absent does Studio convert the complete v1
library into memory. An unsupported version or one invalid saved theme rejects
the whole library; **Download original data** (下载原数据) keeps the rejected
text available before any explicit cache reset.

### Upgrade or continue in memory

1. With a valid v1 library, check the restored draft, named themes and
   preferences. Restoration starts with empty undo history and does not commit
   an upgrade. Editing and export remain available.
2. Choose **Review / recheck upgrade** (检查 / 重新检查升级). Read the complete
   candidate: it includes every legacy named theme and the current draft and
   preferences. **Download original before upgrade** (下载升级前原始数据) saves
   the original library text separately.
3. Choose **Upgrade and save current draft** (升级并保存当前草稿) to approve that
   candidate. A changed draft requires a refreshed review and another explicit
   confirmation. A changed v1 source also requires review again. Waiting for
   the lock leaves editing, undo/redo and export available; it is not a saved
   result.
4. Choose **Cancel upgrade** (取消升级) to keep working in memory. Cancellation
   preserves the open theme, history and preview mode, writes no cancellation
   marker, and keeps subsequent edits, preferences and ordinary library writes
   protected. Export work you need before closing: reopening restores the
   original v1 library and asks again. Use **Review / recheck upgrade** when
   ready to reconsider.

Before committing v2, migration writes the exact reviewed v1 text to its backup
key and reads it back. An existing backup must contain identical text; it is
never overwritten with different content. Only then can the v2 write be
confirmed. A failed backup does not permit a v2 commit; a failed v2 commit keeps
any backup already written. Normal saves, theme deletion and draft reset do
not remove these originals.

### Retrieve originals and recover a failed save

After a successful upgrade, **Download original before upgrade** reads the
backup named by the v2 migration metadata and checks its digest. It still works
after reopening or after an older window changes v1. Missing or damaged
backups report an error; Studio does not substitute the current v1 contents.
An older-window notice offers **Download the changed v1 data** (下载变化后的 v1
原文) separately when that data remains available.

Without secure Web Locks or storage access, use memory editing and downloads;
local write actions are rejected. Quota and other write failures pause saving.
Open **Pending / recent operations** (待处理 / 最近操作) to see retained named
actions and their results. After fixing the cause, **Retry retained saves**
(重试已保留的保存) approves those actions in their original order. For a failed
upgrade, retry returns to review; confirm the refreshed candidate explicitly.
**Cancel pending writes** (取消待写操作) discards pending writes while preserving
the open draft and history. A previous snapshot being saved does not mean that
newer edits have been saved; wait for **Saved on this device** (已保存到此设备).

### Recheck a conflicting version

The conflict view shows the external revision, draft name and named-theme
count. **View exact stored text** (查看存储原文) and **Download displayed
original** (下载当前展示原文) expose that version's text. Invalid text, including
an empty string, is distinct from a removed library. Invalid data remains
protected until an explicit reviewed reset.

| Choice | Effect |
| --- | --- |
| **Load other version** (载入另一版本) | Review the confirmation, then load the validated external version and cancel all pending writes. Export the current draft first if needed. |
| **Keep current draft** (保留当前草稿) | Save the current draft and preferences against the displayed external library. If the external library was removed, this creates a new one. |
| **Save current as a copy** (将当前草稿另存为副本) | Supply a new name to add the current draft to the displayed external library. |

Each choice compares the reviewed version again inside the lock. Another write
requires another decision. Keep and Save copy approve their displayed
candidate; retained pending named actions still need a separate retry or
cancellation. They are not silently replayed or discarded.

### Clear v2 or delete original backups

Open **Cache and original backups** (缓存与原始备份). Choose **Review clearing
v2 cache** (检查清除 v2 缓存), then **Clear reviewed v2 cache** (清除已检查的 v2
缓存). This cancels preceding pending writes, compares the reviewed v2 value
inside the lock, and removes only v2. It keeps the open draft, v1 and backups.
While removal is pending, edits stay in memory and library writes are rejected.
A failed removal does not report success. After success, use **Save current
draft again** (重新保存当前草稿) explicitly to resume persistence; old queued
writes cannot recreate the cache.

Backup deletion is separate. Choose **Review original backups** (检查原始备份)
and use each entry's **Download original** (下载原文) before choosing **Delete
only the reviewed backups** (只删除已检查的备份). Deletion compares the reviewed
key/value set under the same lock; a changed set requires another review.
Deleting backups is irreversible and leaves the current v1 and v2 keys alone.

### Roll back to the previous site

1. In **Export** (导出), choose **Theme JSON v2**, then **Download** (下载) to
   retain the open theme without losing configuration or override intent.
   Export other named themes you need individually, or separately archive the
   full v2 storage value: a theme export is not a whole-library/preferences
   backup.
2. If the old site must consume new work, choose **Compatible JSON v1 · lossy**
   (兼容 JSON v1 · 有损), read the displayed losses, then copy or download it
   for the old site's theme import. Keep the v2 file as well; v1 cannot restore
   the full configured component appearance or original override sources.
3. Close Studio tabs and restore the complete previous deployment, including
   its matching wasm, CSS, fonts and manifest. The old site reads v1, not v2;
   the new editor never automatically copies v2 edits back into that key.
4. If v1 was changed or removed and the pre-upgrade library is needed, archive
   any current v1 value first. Close Studio tabs and open a non-Studio page on
   the same origin, such as `build-manifest.json`. In the browser's storage
   inspector, restore the downloaded original library text verbatim to the v1
   key, then reopen the old site. This replaces that v1 library. The original
   download is a library envelope, not a single-theme
   file for the theme import dialog. Keep v2 and its exported files separate.

Returning to the source site selects a valid existing v2 first; old-site edits
in v1 remain separate. Verification results and their scope are kept in the
[integration record](plan/DOM-COMPONENTS-P1.records/M5.md).

## Capabilities that are still absent

The raw form/selection, menu/navigation, panel, media and scroll families
provide their declared parts; the native content families above do not imply
other raw namespaces or complete reference coverage. Generic React asChild/ref
composition and Slottable remain unsupported; Rust typed Slot supports one
known native target. Unstable reference exports are candidates only. Consult the
generated [reference and sub-capability tables](components.md#stable-reference-inventory)
for each mapping rather than treating a matching name as complete support.

### Native Tooltip targets and anchored parts

The source Tooltip convenience entry accepts one native target. Existing listeners,
attributes and keyboard entry remain on that element:

```rust
<Tooltip open=help_open on_open_change=move |next|help_open.set(next)
    label="Dates use your local time zone"
    trigger=leptos::html::button().r#type("button").child("About dates") />
```

Use `tooltip::{Provider, Root, trigger_as, Portal, Content, Arrow}` for shared hover
intent and custom descriptions. Content stays noninteractive; put selectable
supplemental text and links in HoverCard, or editing controls in Popover.
`hover_card::trigger_as` accepts a native anchor; Popover and Dialog use native
buttons. Application `open` remains the displayed state, including when an open
or close request is refused. `force_mount` belongs to Portal and does not emit
browser floating content during static HTML generation.
