---
name: "Rustify UI"
description: "Incumbent scoped components and the neutral Theme Studio workbench."
colors:
  background: "#f9fafb"
  foreground: "#1d2939"
  primary: "#1570ef"
  primary-foreground: "#ffffff"
  secondary: "#eaecf0"
  secondary-foreground: "#1d2939"
  muted: "#f2f4f7"
  muted-foreground: "#667085"
  accent: "#12b76a"
  accent-foreground: "#ffffff"
  destructive: "#d92d20"
  destructive-foreground: "#ffffff"
  card: "#ffffff"
  popover: "#ffffff"
  success: "#12b76a"
  warning: "#f79009"
  border: "#858f9e"
  input: "#ffffff"
  ring: "#1570ef"
  dark-background: "#0c111d"
  dark-foreground: "#f9fafb"
  dark-primary: "#53b1fd"
  dark-primary-foreground: "#0c111d"
  dark-secondary: "#1d2939"
  dark-secondary-foreground: "#f9fafb"
  dark-muted: "#161b26"
  dark-muted-foreground: "#98a2b3"
  dark-accent: "#32d583"
  dark-accent-foreground: "#0c111d"
  dark-destructive: "#f97066"
  dark-destructive-foreground: "#0c111d"
  dark-card: "#101828"
  dark-popover: "#1d2939"
  dark-success: "#32d583"
  dark-warning: "#fdb022"
  dark-border: "#717c8e"
  dark-input: "#101828"
  dark-ring: "#53b1fd"
  shell-divider: "#d0d5dd"
  shell-subtle-divider: "#eaecf0"
  shell-secondary-text: "#475467"
  shell-search-border: "#98a2b3"
  shell-selected: "#eff8ff"
  shell-selected-border: "#84caff"
  shell-selected-text: "#175cd3"
  shell-error: "#b42318"
  picker-primary: "#2563eb"
  picker-selected: "#eff6ff"
  picker-selected-text: "#1d4ed8"
typography:
  shell-body:
    fontFamily: "IBM Plex Sans, sans-serif"
    fontSize: "14px"
    lineHeight: 1.5
  v4-adapter-body:
    fontFamily: "IBM Plex Sans, sans-serif"
    fontSize: "15px"
  title:
    fontFamily: "IBM Plex Sans, sans-serif"
    fontSize: "18px"
    lineHeight: 1.3
    letterSpacing: "-0.02em"
  preview-title:
    fontFamily: "IBM Plex Sans, sans-serif"
    fontSize: "16px"
    fontWeight: 600
  label:
    fontFamily: "IBM Plex Sans, sans-serif"
    fontSize: "13px"
    fontWeight: 600
  caption:
    fontFamily: "IBM Plex Sans, sans-serif"
    fontSize: "12px"
  color-code:
    fontFamily: "Liberation Mono, monospace"
    fontSize: "12px"
  export-code:
    fontFamily: "var(--font-mono, ui-monospace, monospace)"
    fontSize: ".8125rem"
    lineHeight: 1.6
rounded:
  legacy-sm: "4px"
  legacy-md: "6px"
  legacy-lg: "8px"
  v4-default-sm: "2px"
  v4-default-md: "4px"
  v4-default-lg: "6px"
  v4-default-xl: "10px"
  shell-field: "6px"
  shell-token: "5px"
  shell-panel: "8px"
spacing:
  utility-unit: "0.25rem"
  legacy-layout-gap: "8px"
  shell-xs: "4px"
  shell-sm: "8px"
  shell-md: "12px"
  shell-lg: "16px"
  shell-xl: "20px"
  shell-2xl: "24px"
  shell-bottom: "48px"
components:
  button-primary:
    textColor: "{colors.primary-foreground}"
    rounded: "{rounded.legacy-md}"
    padding: "0.5rem 1rem"
    height: "2.25rem"
    backgroundColor: "{colors.primary}"
  button-secondary:
    textColor: "{colors.secondary-foreground}"
    rounded: "{rounded.legacy-md}"
    padding: "0.5rem 1rem"
    height: "2.25rem"
    backgroundColor: "{colors.secondary}"
  button-outline:
    textColor: "{colors.foreground}"
    rounded: "{rounded.legacy-md}"
    padding: "0.5rem 1rem"
    height: "2.25rem"
    backgroundColor: "{colors.background}"
  button-ghost:
    textColor: "{colors.foreground}"
    rounded: "{rounded.legacy-md}"
    padding: "0.5rem 1rem"
    height: "2.25rem"
  button-accent:
    textColor: "{colors.accent-foreground}"
    rounded: "{rounded.legacy-md}"
    padding: "0.5rem 1rem"
    height: "2.25rem"
    backgroundColor: "{colors.accent}"
  button-destructive:
    textColor: "{colors.destructive-foreground}"
    rounded: "{rounded.legacy-md}"
    padding: "0.5rem 1rem"
    height: "2.25rem"
    backgroundColor: "{colors.destructive}"
  button-link:
    textColor: "{colors.primary}"
    rounded: "{rounded.legacy-md}"
    padding: "0.5rem 1rem"
    height: "2.25rem"
  text-field:
    backgroundColor: "{colors.input}"
    textColor: "{colors.foreground}"
    rounded: "{rounded.legacy-md}"
    padding: "0.25rem 0.75rem"
    height: "2.25rem"
  studio-tabs:
    textColor: "{colors.shell-selected-text}"
    padding: "10px 0"
  preview-card:
    backgroundColor: "{colors.card}"
    textColor: "{colors.foreground}"
    rounded: "{rounded.v4-default-lg}"
    padding: "1.5rem"
  preview-tag:
    backgroundColor: "{colors.accent}"
    textColor: "{colors.accent-foreground}"
    rounded: "{rounded.v4-default-md}"
    padding: "0.25rem 0.5rem"
---

# Design System: Rustify UI

## Overview

**Creative North Star: "One theme, two renderers"**

Rustify UI uses scoped semantic colors, compact controls and visible keyboard focus. Theme Studio applies that incumbent language to a neutral workbench, keeping the editing tools readable while authored themes change the DOM and GPU previews.

This document records the current SDK defaults and the implemented Theme Studio shell. It does not impose a palette, typeface, density or shadow on user-authored previews, other applications built with the SDK, or every example in this repository. The approved surface composition remains in the Theme Studio surface brief; the frontmatter names default tokens, not immutable theme values.

**Key Characteristics:**

- Scoped semantic tokens shared by DOM and GPU.
- Stable neutral editing tools with blue selection and focus.
- Compact controls, independent scrolling and explicit state feedback.

Evidence: [PRODUCT.md](PRODUCT.md), [approved surface brief](.impeccable/surfaces/examples-theme-studio-src-app-rs.md), [plan §6](docs/plan/THEME-STUDIO.md#6-前端与交互设计). This is a code extraction following the finish verdict, not a replacement visual direction.

## Colors

The default system pairs cool neutral surfaces with a clear blue action color and semantic green, amber and red feedback. Frontmatter entries without a prefix describe `Theme::light()`; `dark-*` entries describe `Theme::dark()`. `shell-*` and `picker-*` are literal Theme Studio styles and are not SDK-wide aliases.

### Primary

`primary` and `ring` carry action and focus blue. Foreground pairs belong to their backgrounds: use `primary-foreground` on primary controls. The dark defaults supply their own pair. Shell selected controls use the pale blue `shell-selected`, its border and darker text, while the color picker retains its separate `picker-*` blue family.

### Secondary

`secondary` supplies quiet filled actions. Green `accent` and `success`, amber `warning`, and red `destructive` are semantic roles; they are not a decorative multi-accent requirement. Shell field failures use `shell-error`.

### Neutral

`background`, `card`, `popover`, `input`, `foreground`, `muted` and `muted-foreground` are scoped component roles. SDK `border` is the control boundary; Theme Studio's lighter `shell-divider` and `shell-subtle-divider` separate regions. Do not silently substitute the shell dividers for SDK control borders. Dark defaults distinguish card and popover surfaces.

**The Scope Rule.** Keep editing chrome neutral; apply authored colors only inside the intended preview scope and its owned overlays.

Configured source themes use the separate `ThemeDocument::standard` defaults:
indigo brand, auto neutral pairing, translucent panel background, medium radius
and 100% scaling. Legacy `Theme::light/dark` values and the frontmatter remain
the compatibility palette; converting them with `from_legacy` selects Custom
configuration. Brand families do not rename the existing semantic green
`accent`. Explicit author overrides remain above generated configuration.
Component-local color choices affect that component's roles and stop at its
boundary. An omitted component color consumes the effective semantic pair.

The root `ThemeScope` and nested `ThemeBoundary` retain a complete two-mode
author source. Explicit local appearance can differ from the root; owned
floating layers use the creating boundary's effective snapshot. Keep theme
projection on the scope/boundary node and restore it on cleanup.

Evidence: [configuration defaults and responsibility](crates/rustify-dom/src/theme/config.rs),
[resolved geometry and roles](crates/rustify-dom/src/theme/resolve.rs),
[provider ownership](crates/rustify-dom/src/theme/provider.rs),
[local palette boundary](crates/rustify-components/css/button.css).

Sources: [legacy defaults](crates/rustify-dom/src/theme.rs), [SDK fallback CSS](crates/rustify-components/css/sdk.css), [shell CSS](examples/theme-studio/app.css), [picker CSS](examples/theme-studio/color-editor.css). The sidecar's synthetic tonal ramps are panel visualization aids, not additional shipped color tokens.

## Typography

The shell uses IBM Plex Sans with a sans-serif fallback. Its hierarchy is deliberately compact: the frontmatter records body, title, preview title, label and caption roles. At the shell's 800px breakpoint, the main title becomes 16px. Small technical labels also exist at 10–11px in token chips, channel rows and renderer identifiers; these are current specialist labels, not a recommended general body size.

The legacy SDK's body size is separate from the shell body. The v4 legacy adapter supplies IBM Plex Sans for sans, `ui-serif, serif` for serif, and Liberation Mono for mono. Color strings use the monospace color-code role; import/export areas use the export-code role. User-authored font stacks and letter spacing govern the preview.

[Bundled font declarations](web/theme-fonts.css) provide IBM Plex Sans, Noto Sans, Rustify WenKai, Noto Serif, Liberation Mono and Noto Color Emoji at normal 400, plus JetBrains Mono at 100–800; all use `font-display: swap`. Do not infer separately bundled semibold or italic files from CSS weights. [Theme document defaults](crates/rustify-dom/src/theme/document.rs) and resolved font diagnostics define the preview contract.

The source DOM typography API keeps semantic elements separate from visual
size: Text selects span/div/label/p; Heading selects h1–h6 with visual size 6
by default. Text without size inherits. Blockquote, Code, Kbd, Em, Quote and
Strong retain their native roles; use responsive size/weight/alignment/trim/wrap
only on components that expose those fields. Truncation preserves the full
text node and is an explicit, nonresponsive choice. Caller utilities can
override the component layer without changing the underlying semantic element.

Evidence: [typography props and native tags](crates/rustify-components/src/typography.rs),
[typography rules](crates/rustify-components/css/typography.css).

## Layout

The following composition belongs to Theme Studio, not every Rustify screen. [app.css](examples/theme-studio/app.css) and [app.rs](examples/theme-studio/src/app.rs) implement a full-height grid (`100dvh`) with a 320px editor and a flexible `minmax(0,1fr)` preview. Both regions scroll independently. The separator is an 8px pointer target, adjusts the editor within 260–520px, and supports 10px arrow-key steps plus Home/End.

Editor padding is 24px 20px 48px with a 16px stack gap; preview padding is 24px 24px 48px. The shell uses fine dividers and grouped rows rather than one card per field. Spacing frontmatter records observed reusable steps; the legacy 8px layout gap is distinct from the 0.25rem Tailwind utility unit. Opt-in v4 uses `--rustify-unit` for authored utility spacing.

| Maximum viewport width | Implemented behavior |
| --- | --- |
| 1100px | Side-by-side DOM/GPU comparison stacks vertically. |
| 800px | Editor horizontal padding becomes 16px; preview padding becomes 20px 16px 48px; metrics use two columns instead of four. |
| 600px | Edit/Preview tabs replace the two-column shell; separator hides; preview has a return-to-edit action. Editor content height is `calc(100dvh - 49px)` and remains scrollable. |
| 560px | Import/export form rows become one column and local library rows stack. |

The preview width selector deliberately sets renderer widths to 1024px, 768px or 390px; these are simulated content widths, not responsive breakpoints. Automatic width follows the available space. Export/import dialogs use viewport-minus-2rem width and height limits, a 64rem maximum width and internal vertical scrolling ([data-tools.css](examples/theme-studio/data-tools.css)).

### Reusable DOM layout

Box/Flex/Grid retain div/span semantics independently of display. Container has
an outer alignment layer and an inner content layer; size and width/height
constraints belong to the content. Section remains a section with theme spacing.
Inset consumes only the nearest container's declared padding. Reset decorates
one typed native element and does not add a wrapper or reset the host page.

`Responsive<T>` uses minimum widths 0/520/768/1024/1280/1640 CSS px. Sparse values
inherit the nearest smaller choice; a missing initial value retains the
component default. These are component breakpoints, independent of the Studio
shell thresholds and Tailwind's same-named breakpoints.

Theme space steps consume the already resolved `--rustify-space-N` snapshot;
font sizes, line heights, finite radii and control metrics likewise scale once.
Explicit CSS sizes and breakpoints retain their declared units and are not
multiplied by theme scaling. Keep utilities above component defaults by declaring
`@layer theme, base, components, utilities` before the Tailwind imports.
Responsive geometry uses external SSR rules or the existing constructed
stylesheet's CSSOM, with low-weight scoped selectors. Invalid values expose
a prop diagnostic and leave component defaults usable.

Evidence: [layout props](crates/rustify-components/src/layout.rs),
[layout defaults](crates/rustify-components/css/layout.css),
[responsive model](crates/rustify-components/src/responsive.rs),
[style collector](crates/rustify-components/src/geometry.rs).

## Elevation & Depth

The shell is flat, with white editing surfaces, a quiet workspace and dividers. It does not use decorative shadows to establish its columns. The inspector outlines content with a blue border and a white 1px outer stroke. SDK dialogs and Select popovers use the legacy `shadow-lg` geometry; the exact inherited geometry is recorded in the sidecar from [theme-v4.css](crates/rustify-components/css/theme-v4.css).

Authored preview shadow parameters replace the opt-in shadow layers. The legacy-to-document adapter starts with zero shadow opacity. Consequently a preview's absent or strong shadow is theme data, not a failure to follow shell styling.

Scope transitions use 150ms by default and 0ms when reduced motion is requested ([theme.rs](crates/rustify-dom/src/theme.rs), [controls.css](crates/rustify-components/css/controls.css)). Color editing updates immediately; the picker additionally resets scroll behavior under `prefers-reduced-motion: reduce`. No decorative shell animation is required.

## Shapes

The frontmatter distinguishes legacy radii from the Custom document scale.
Legacy small/medium/large use `r−2`, `r`, `r+2`; Custom documents retain
`max(0,r−4)`, `max(0,r−2)`, `r`, `r+4`. Configured source themes derive finite
levels from 3/4/6/8/12/16px, the chosen radius factor and scaling. Full/thumb
geometry remains separate; necessary radio, checkbox and switch shapes do not
become ordinary rounded rectangles. Reusing a Tailwind radius class across
these runtimes can produce a different radius intentionally.

Shell search and color fields use gently curved corners; inspector panels are slightly softer. Color token/reset pairs join into one split control with rounded outer edges. Sliders and swatches retain their circular or compact geometry. Do not turn switch capsules or radio circles into ordinary rounded rectangles when changing theme radius.

Sources: [SDK radius utilities](crates/rustify-components/css/sdk.css), [v4 projection](crates/rustify-components/css/theme-v4.css), [resolved radii](crates/rustify-dom/src/theme/resolve.rs), [picker CSS](examples/theme-studio/color-editor.css).

## Components

### Buttons

Compact and explicit. Source [Button](crates/rustify-components/src/button.rs)
provides Classic/Solid/Soft/Surface/Outline/GhostOffset/Ghost, with Solid and
size 2 as defaults. Its four reference control heights are 24/32/40/48px before
theme scaling; Ghost keeps its own offset geometry. Frontmatter samples retain
the previous legacy controls and do not define the new visual size scale.
`IconButton` reuses the native button behavior and requires an accessible name.

Variants select local palette or effective semantic role pairs; hover/active
surfaces are defined in [button.css](crates/rustify-components/css/button.css).
Keyboard focus follows Radix Themes 3.3.0: a solid 2px step-8 outline, offset
2px for Classic/Solid and −1px for Surface/Outline/Ghost/GhostOffset. Soft uses
the component's step 8 with −1px offset; other variants use the scope ring,
or the selected component color's step 8 except Gray. Link uses the same 2px
outline with 2px offset, a `0.07em × radius-factor` radius and no focused
underline. Native disabled or loading controls refuse activation and native
form defaults. Loading preserves the original label/width and adds a decorative
spinner. `unavailable` keeps its explanatory, focusable request contract rather
than becoming native disabled. Navigation remains a native Link, with matching,
underline and typography choices rather than a button variant.

### Inputs / Fields

[TextField](crates/rustify-components/src/input.rs) and [Select](crates/rustify-components/src/select.rs) share a compact outlined field, input surface and visible focus ring. TextField read-only state uses muted fill; invalid state changes border and ring to destructive; disabled state halves opacity. Select retains the existing combobox/listbox interaction and owned overlay rather than introducing a native arrow beside a second chevron.

Shell search uses its own stronger search border and a 3px translucent blue outline with 2px offset. Picker fields use the picker blue at 40% opacity, the same outline thickness and offset, and the shell error border for invalid input. Keep error text visible next to the relevant field; long token names and errors wrap.

### Navigation

[SDK Tabs](crates/rustify-components/src/tabs.rs) use a muted rounded strip with selected background/text and a 3px focus ring. Theme Studio's field tabs instead use a flat underline: a 2px selected blue border, 13px semibold labels and a separate visible focus outline. Selected mode/renderer actions use pressed state. Mobile Edit/Preview tabs keep the active workspace reachable. Preserve tab roles, arrow-key navigation and focus return.

### Cards / Containers / Tags

Preview cards and accent tags are actual themed examples in [scenes.rs](examples/theme-studio/src/scenes.rs), not shell scaffolding or standalone SDK Card/Badge primitives. The documented card sample uses v4 large radius, card colors, a semantic border and 1.5rem padding. The tag uses v4 medium radius, accent colors and compact padding. Their colors, size and density must follow the current preview theme.

### Dialogs, diagnostics and inspector

[Dialog](crates/rustify-components/src/dialog.rs) uses popover colors, a bordered rounded panel, 1.5rem padding, an optional 30%-foreground backdrop and the existing overlay focus trap/Escape/return behavior. Theme Studio extends its size for readable import/export content. Failed import retains the prior theme; a storage failure leaves export available. Empty searches and empty local libraries have explicit recovery actions.

The inspector's selected rectangle uses a fixed outline above the preview and an informational pale-blue panel with token buttons. It is an inspection aid, not a permanent decoration. GPU status and recovery text follow the editor locale; failures remain visible while DOM editing remains available. The host startup status hides when ready and remains actionable when fatal or failed.

## Do's and Don'ts

### Do:

- Do keep the Theme Studio editing tools on their neutral light scope while previewing light, dark or low-contrast themes.
- Do preserve authored preview colors and show contrast or fallback diagnostics honestly.
- Do use semantic scope tokens for components and the existing Select, Dialog and keyboard navigation behavior.
- Do keep utility spacing separate from the legacy layout gap and use the radius scale for the active runtime.
- Do show real empty, error, saved and GPU recovery states in the selected editor language.

### Don't:

- Don't apply the shell palette or IBM Plex Sans to every authored preview.
- Don't turn Theme Studio's two-column layout into a requirement for all SDK applications.
- Don't remove focus indicators, disable access to export after storage failure, or replace failed GPU content with a fake successful preview.
- Don't add decorative imagery, large display typography or wrapper cards to the Theme Studio editing controls.
