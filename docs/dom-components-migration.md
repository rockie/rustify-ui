---
description: Migrate source consumers to the configured theme and native DOM component APIs.
---
# DOM component migration

This guide describes the current source checkout. Published SDK and Docs CLI
0.2.1 use the previous APIs and schema v1. Use one matching source checkout for
the SDK, styles and source CLI; these changes do not publish a release.

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

For SSR, build with `ssr` separately from CSR/hydrate, seed `ssr_scope`, call
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
