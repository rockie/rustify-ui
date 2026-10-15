---
name: "Rustify Docs reading theme"
description: "The default theme every `rustify-docs build` page wears: one article column, quiet wayfinding, one blue."
colors:
  primary: "#175cd3"
  primary-foreground: "#ffffff"
  ring: "#175cd3"
  background: "#f9fafb"
  foreground: "#1d2939"
  muted-foreground: "#667085"
  secondary: "#eaecf0"
  border: "#858f9e"
  warning: "#f79009"
  code-bg: "#f2f4f7"
  line-hl: "#e3ebf8"
  note: "#0969da"
  note-text: "#0550ae"
  tip: "#1a7f37"
  tip-text: "#116329"
  important: "#8250df"
  important-text: "#6639ba"
  alert-warning: "#9a6700"
  alert-warning-text: "#7d4e00"
  caution: "#cf222e"
  caution-text: "#a40e26"
  popover: "#ffffff"
  dark-primary: "#53b1fd"
  dark-primary-foreground: "#000000"
  dark-ring: "#53b1fd"
  dark-background: "#0c111d"
  dark-foreground: "#f9fafb"
  dark-muted-foreground: "#98a2b3"
  dark-secondary: "#1d2939"
  dark-border: "#717c8e"
  dark-warning: "#fdb022"
  dark-code-bg: "#161b26"
  dark-line-hl: "#1f2a3d"
  dark-note: "#4493f8"
  dark-note-text: "#79c0ff"
  dark-tip: "#3fb950"
  dark-tip-text: "#7ee787"
  dark-important: "#ab7df8"
  dark-important-text: "#d2a8ff"
  dark-alert-warning: "#d29922"
  dark-alert-warning-text: "#e3b341"
  dark-caution: "#f85149"
  dark-caution-text: "#ffa198"
  dark-popover: "#1d2939"
typography:
  home-title:
    fontFamily: "ui-sans-serif, system-ui, sans-serif"
    fontSize: "clamp(2.25rem, 1.6rem + 2.6vw, 3.25rem)"
    fontWeight: 700
    lineHeight: 1.08
    letterSpacing: "-0.03em"
  tagline:
    fontFamily: "ui-sans-serif, system-ui, sans-serif"
    fontSize: "1.1875rem"
    lineHeight: 1.55
  headline:
    fontFamily: "ui-sans-serif, system-ui, sans-serif"
    fontSize: "2rem"
    fontWeight: 700
    lineHeight: 1.2
    letterSpacing: "-0.02em"
  title-h2:
    fontFamily: "ui-sans-serif, system-ui, sans-serif"
    fontSize: "1.375rem"
    fontWeight: 650
    lineHeight: 1.3
    letterSpacing: "-0.01em"
  title-h3:
    fontFamily: "ui-sans-serif, system-ui, sans-serif"
    fontSize: "1.125rem"
    fontWeight: 600
    lineHeight: 1.3
  body:
    fontFamily: "ui-sans-serif, system-ui, sans-serif"
    fontSize: "1rem"
    lineHeight: 1.75
  nav:
    fontFamily: "ui-sans-serif, system-ui, sans-serif"
    fontSize: "0.875rem"
    lineHeight: 1.4
  label:
    fontFamily: "ui-sans-serif, system-ui, sans-serif"
    fontSize: "0.8125rem"
    fontWeight: 600
  table:
    fontFamily: "ui-sans-serif, system-ui, sans-serif"
    fontSize: "0.9375rem"
    lineHeight: 1.5
    fontFeature: "tnum"
  dev:
    fontFamily: "ui-sans-serif, system-ui, sans-serif"
    fontSize: "0.875rem"
    lineHeight: 1.45
  dev-mono:
    fontFamily: "ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, 'Liberation Mono', 'Courier New', monospace"
    fontSize: "0.75rem"
  code:
    fontFamily: "ui-monospace, SFMono-Regular, Menlo, Monaco, Consolas, 'Liberation Mono', 'Courier New', monospace"
    fontSize: "0.875rem"
    lineHeight: 1.65
rounded:
  panel: "0.375rem"
  control: "0.5rem"
  mode-option: "0.3125rem"
  chip: "0.25rem"
  pill: "999px"
  focus: "2px"
spacing:
  measure: "36rem"
  header: "3.5rem"
  frame: "76rem"
  sidebar: "15rem"
  toc: "14rem"
  column-gap: "2.5rem"
  gutter: "1rem"
  gutter-md: "1.5rem"
  flow: "1.125rem"
  section: "3rem"
  dev-panel: "26.25rem"
components:
  action-primary:
    backgroundColor: "{colors.primary}"
    textColor: "{colors.primary-foreground}"
    rounded: "{rounded.control}"
    padding: "0 1.25rem"
    height: "2.75rem"
  action-secondary:
    textColor: "{colors.foreground}"
    rounded: "{rounded.control}"
    padding: "0 1.25rem"
    height: "2.75rem"
  action-secondary-hover:
    backgroundColor: "{colors.secondary}"
  site-footer:
    textColor: "{colors.primary}"
  mode-option:
    textColor: "{colors.muted-foreground}"
    rounded: "{rounded.mode-option}"
    padding: "0.1875rem 0.5rem"
  mode-option-pressed:
    backgroundColor: "{colors.secondary}"
    textColor: "{colors.foreground}"
  nav-link:
    textColor: "{colors.muted-foreground}"
    typography: "{typography.nav}"
    padding: "0.3125rem 0 0.3125rem 0.875rem"
  nav-link-current:
    textColor: "{colors.primary}"
  code-panel:
    backgroundColor: "{colors.code-bg}"
    rounded: "{rounded.panel}"
    typography: "{typography.code}"
  copy-button:
    textColor: "{colors.muted-foreground}"
    rounded: "{rounded.panel}"
    padding: "0.25rem 0.5rem"
  inline-code:
    backgroundColor: "{colors.code-bg}"
    rounded: "{rounded.chip}"
    padding: "0.125rem 0.375rem"
  table-header:
    backgroundColor: "{colors.code-bg}"
    padding: "0.5rem 0.875rem"
  pager-link:
    textColor: "{colors.primary}"
    rounded: "{rounded.panel}"
    padding: "0.75rem 1rem"
  draft-label:
    textColor: "{colors.foreground}"
    rounded: "{rounded.panel}"
    padding: "0.0625rem 0.5rem"
  dev-panel:
    backgroundColor: "{colors.popover}"
    textColor: "{colors.foreground}"
    rounded: "{rounded.panel}"
    typography: "{typography.dev}"
    width: "{spacing.dev-panel}"
  dev-panel-head:
    padding: "0.5rem 0.5rem 0.5rem 0.875rem"
  dev-item:
    padding: "0.5rem 0.875rem"
  dev-location:
    textColor: "{colors.muted-foreground}"
    typography: "{typography.dev-mono}"
  dev-rule-error:
    textColor: "{colors.caution-text}"
    typography: "{typography.dev-mono}"
  dev-rule-warning:
    textColor: "{colors.alert-warning-text}"
    typography: "{typography.dev-mono}"
  dev-hide:
    textColor: "{colors.muted-foreground}"
    rounded: "{rounded.panel}"
    size: "2rem"
  dev-hide-hover:
    backgroundColor: "{colors.secondary}"
    textColor: "{colors.foreground}"
  dev-marker:
    backgroundColor: "{colors.popover}"
    textColor: "{colors.foreground}"
    rounded: "{rounded.pill}"
    padding: "0.375rem 0.75rem"
  dev-marker-hover:
    backgroundColor: "{colors.secondary}"
---

# Design System: Rustify Docs reading theme

This is the reading theme's own design document, as required by [docs/plan/DOCS-SSG-P1.md §6](docs/plan/DOCS-SSG-P1.md). It extends the Rustify world recorded in [DESIGN.md](DESIGN.md) to documentation pages and does not replace it: DESIGN.md still governs the SDK defaults and Theme Studio. Everything below is taken from the shipped theme: [docs.css](crates/rustify-docs/assets/docs.css), [shell.rs](crates/rustify-docs/src/shell.rs), [style.rs](crates/rustify-docs/src/style.rs), [highlight.rs](crates/rustify-docs/src/highlight.rs), [icons.rs](crates/rustify-docs/src/icons.rs), [site.js](crates/rustify-docs/assets/site.js), [mode.js](crates/rustify-docs/assets/mode.js) and [labels.rs](crates/rustify-docs/src/labels.rs), checked against the review renders in `.impeccable/review/rustify-docs/`. The surface brief is [crates-rustify-docs-src-shell-rs.md](.impeccable/surfaces/crates-rustify-docs-src-shell-rs.md). The dev-only problem layer (see "Dev problem panel" under Components) is taken from [dev.css](crates/rustify-docs/assets/dev.css), [dev.js](crates/rustify-docs/assets/dev.js), `DevLabels` in labels.rs and `render_dev_shell` in shell.rs, checked against `.impeccable/review/rustify-docs-dev/`. Its brief is [crates-rustify-docs-assets-dev-js.md](.impeccable/surfaces/crates-rustify-docs-assets-dev-js.md).

## Overview

**Creative North Star: "A page you read, not a portal you browse"**

The article column owns the screen. The header, sidebar, table of contents and pager are quiet wayfinding in small muted type that steps back from the text. The page uses Rustify's cool neutral surfaces and slate text with one blue. That blue is the configurable primary pair, and it marks links, the reader's current location, focus and search selection or matches. Depth comes from hairlines and a slightly deeper neutral panel, never from shadow. Every page has complete reading content in static HTML before any script runs. Script adds search, the mode switch, copy, code-group tabs and the TOC tracker; without it, reading content remains available.

Where the theme differs from DESIGN.md:

- **Typeface:** system sans and system mono, not IBM Plex Sans. The no-webfont constraint, the strict CSP and the 100 KB page budget require this.
- **Type ramp:** a reading ramp on a 16px body with 1.75 leading, not the shell's compact 14px.
- **Measure:** a 36rem reading measure.
- **Frame:** a 76rem page frame of its own.

**Key Characteristics:**

- A single article column at a 36rem measure, flanked by a 15rem sidebar and a 14rem TOC that fold into native `<details>` panels on narrower screens.
- One blue (`primary`/`ring`, brand-overridable per mode) for links, the current page or section, and focus.
- Flat surfaces with 1px hairline dividers. Code, tables and the table header sit on the deeper `code-bg` neutral.
- One 6px radius for every panel.
- Code panels follow the mode: a light panel in light mode, a dark panel in dark mode.

## Colors

The palette is cool neutral with a single blue. Alert hues come from GitHub's five-alert vocabulary, and the syntax palette is GitHub-derived. Keys without a prefix are the light mode. `dark-*` keys apply when `:root[data-theme="dark"]` is set, and when `data-theme="system"` is set and the reader's system prefers dark. All other semantic tokens are the SDK's `Theme::light()`/`Theme::dark()` values, passed through unchanged by `style.rs`.

### Primary

- **Reading Blue** (`primary`, `ring`; light default #175cd3, dark default #53b1fd): link text, the current sidebar entry and TOC section, the selected code tab underline, the checked task box, pager titles, the primary home action and every focus outline. It is one step deeper than the SDK default `#1570ef` because body links sit on #f9fafb (5.73:1). Sites override the pair through `Brand`. `ring` always equals `primary`.
- **Primary Label** (`primary-foreground`): this is never authored. `foreground_on()` sets it per mode to whichever of black or white has more contrast on that mode's primary. With the defaults it is #ffffff in light and #000000 in dark (unlike the SDK's dark `#0c111d`). The task-box check mark (`--rd-check`) uses the same colour.

### Semantic (alerts)

Each alert has an edge hue (`--rd-<kind>`) and a title hue (`--rd-<kind>-text`). The alert surface is a 7% mix of the edge hue into `background`, and its border is a 38% mix. A test holds every title hue at 4.5:1 or better on the mode's muted surface. Copy feedback borrows two of these title hues: tip-text marks "copied" and caution-text marks "failed".

### Neutral

- **Cool Paper** (`background`, #f9fafb / #0c111d): the page, the header and every un-tinted surface.
- **Slate Ink** (`foreground`): headings, strong text, brand name, selected tab, pressed mode option, hover state of muted links.
- **Wayfinding Grey** (`muted-foreground`): sidebar and TOC entries, code-bar text, list markers, blockquotes, footnotes, page meta, pager labels, tagline and feature text. It measures 4.76:1 on the page and 4.51:1 on the code panel in light.
- **Hairline** (`secondary`): every divider. This covers the header rule, the sidebar rail, H2 underline, panel borders, table rows, footnote/meta rules, feature rules and the mode-switch frame. It is also the pressed mode option and secondary-action hover fill.
- **Control Edge** (`border`): the edges that must stay visible: the task box, the secondary home action, blockquote rule and the sidebar entry's hover rail.
- **Code Panel** (`code-bg`, the SDK `muted` values #f2f4f7 / #161b26): code blocks, code groups, inline code and table header cells.
- **Marked Line** (`line-hl`, #e3ebf8 / #1f2a3d): highlighted code lines.
- `warning` draws only the draft label's border.
- **Popover** (`popover`, the SDK's #ffffff / #1d2939): the search dialog, language menu, dev problem panel and its marker. In dark it equals `secondary`, which is why the dev panel's inner rules use their own mix (below).

Tints are mixed in `oklab` from these tokens:

- Selection: primary 24%.
- Scrollbar: muted-foreground 45%, or 55% in code.
- Copy hover: foreground 7%.
- Overflow cue: foreground 14%.
- Primary hover: primary 86% into foreground.
- Search result selection: primary 8% into popover, with a primary edge; matched terms use primary 20% into popover and normal popover text.
- Search backdrop: foreground 30% into transparent.
- Dev panel rules (`--rd-dev-divider`): border 35% into popover, mixed in `srgb`. It stands off the popover in both modes where `secondary` disappears in dark.

`docs.css` picks no hue of its own.

### Syntax palette

Highlighting is emitted as `hl-` classes, and each colour is held at 4.5:1 or better on its mode's `code-bg`.

| Scopes | Light | Dark |
| --- | --- | --- |
| comment (italic) | #57606a | #9198a1 |
| keyword, storage | #b42318 | #ff7b72 |
| string | #0a3069 | #a5d6ff |
| constant, markup heading | #0550ae | #79c0ff |
| function | #6639ba | #d2a8ff |
| type, class, tag | #116329 | #7ee787 |
| attribute, parameter, property key | #953800 | #ffa657 |
| inserted | #116329 | #7ee787 |
| deleted, invalid | #a40e26 | #ffa198 |
| diff range/header | #6639ba | #d2a8ff |

### Named Rules

**The One Blue Rule.** Primary means "link, you are here, focus, or a search selection or match". Never use it for decoration or headings. Full primary fills belong to the primary home action and checked task box; search selection and matched terms use restrained tints.

**The Generated Pair Rule.** Authors choose the primary for each mode, and the theme derives everything else: `ring` copies it, `primary-foreground` and the check mark are computed black or white. Never hand-pick a label colour for a brand primary.

## Typography

**Body Font:** the system sans stack (`ui-sans-serif, system-ui, sans-serif` and emoji fallbacks, Tailwind's `--font-sans`).
**Mono Font:** the system mono stack (`ui-monospace, SFMono-Regular, Menlo, …, monospace`).

**Character:** the reader's own interface face at a comfortable reading size. It is familiar, costs nothing to load and renders CJK and Latin equally. The hierarchy comes from size, weight and tight negative tracking on the large headings. No second face is used.

### Hierarchy

- **Home title** (700, `clamp(2.25rem, 1.6rem + 2.6vw, 3.25rem)`, 1.08, −0.03em, balanced): the site name on the home page only.
- **Tagline** (400, 1.1875rem, 1.55, muted, max 36rem, `pretty` wrap): one line of positioning under the home title.
- **Page title / H1** (700, 2rem, 1.2, −0.02em, balanced): exactly one per page. The title becomes the H1 unless the article has its own. Markdown and effective raw HTML headings share the single-H1 and no-skipped-level checks; code, comments and HTML template contents are excluded. The home hero merges only the existing leading lowercase H1 form and retains its id. An author H1 that does not merge is a located error.
- **H2** (650, 1.375rem, 1.3, −0.01em): 3rem above, with a hairline underline and 0.5rem padding.
- **H3** (600, 1.125rem): 2.25rem above. **H4–H6** (600, 1rem): 1.75rem above.
- **Body** (400, 1rem, 1.75): the article text at a 36rem measure, about 75 Latin or 36 CJK characters per line. Blocks are separated by 1.125rem.
- **Navigation** (0.875rem, 1.4): sidebar entries, TOC entries, narrow-panel summaries (500), footnotes and page meta.
- **Label** (0.8125rem): sidebar group headings and the "On this page" heading (600), mode options, code bar, copy button, code tabs, pager labels and draft label.
- **Table** (0.9375rem, 1.5, tabular figures).
- **Code** (mono, 0.875rem, 1.65, tab size 4). Inline code is 0.875em on the code panel.

### Named Rules

**The Measure Rule.** Prose, code and tables share the 36rem column. Width overflow scrolls inside its own panel, never the page.

**The Quiet Chrome Rule.** Wayfinding never exceeds 0.875rem or weight 600. Size and weight belong to the article's headings.

## Layout

- **Frame:** a centred frame of at most 76rem. Gutters are 1rem below 48rem and 1.5rem from 48rem, with a 2.5rem column gap.
- **Header:** sticky, at least 3.5rem tall (`--rd-header`), with a bottom hairline. The brand sits left; Search, the language menu when configured, and the mode switch sit right. Let the bar grow when its controls wrap at narrow widths.
- **Under 48rem (768px):** one column, ordered sidebar → TOC → article. The sidebar and TOC are native `<details>` panels separated by hairlines, with a drawn chevron that rotates when open.
- **48rem–79.99rem:** a 15rem sticky sidebar column plus the article. The TOC stays a `<details>` panel above the article, constrained to the measure.
- **From 80rem (1280px):** three columns: 15rem sidebar, flexible article, 14rem TOC. The sidebar and TOC columns are sticky under the header, scroll on their own and have 3rem block padding.
- **Narrow vs wide navigation:** both are always rendered, and exactly one is `display: none` at any width. Assistive technology therefore meets one navigation, and a closed `<details>` never hides the wide column from WebKit.
- **Article padding:** 2rem top and bottom, or 3rem from 48rem.
- **Below the article:** footnotes, page meta and pager each start 3rem down (2rem for the pager) behind a hairline.
- **Pager:** stacks below 40rem, two columns from 40rem with Next always in column 2.
- **Home page:** no sidebar.
  - Block padding is 4rem top and 5rem bottom. The hero is at most 44rem wide, the tagline at most 36rem.
  - Actions sit 2rem below with 0.75rem gaps.
  - Nonempty actions form the named Start reading navigation. Actions remain optional.
  - Features begin 4rem below. They are one column, two from 40rem, and three from 64rem only when the count is 3, 5 or more (2 and 4 stay in pairs).
  - The page's own Markdown follows at the measure, 3rem down.
- **Anchors:** headings and any `[id]` scroll to `header + 1.5rem` below the top.
- **Site footer:** every home, content and 404 page ends with a body-level footer outside main, behind a 1px `secondary` hairline. It has 1.5rem block padding and one underlined primary link at 0.875rem in the named Footer links navigation, returning to the current language's home (the site's base without configured languages). It does not depend on home actions, header links, dates or an edit URL. Local page metadata stays inside the article column.

The surface's incremental component sidecar is [.impeccable/docs-ssg.design.json](.impeccable/docs-ssg.design.json). Its source is this document; the root `.impeccable/design.json` continues to describe the SDK and Theme Studio.

## Elevation & Depth

Flat. The reading theme has no `box-shadow` anywhere; the only shadow a docs page can carry is the dev problem layer's, which no production build contains. Layers are separated by 1px `secondary` hairlines and by the `code-bg` panel tone against `background`. The sticky header is opaque `background` with a hairline, not a blur or a shadow.

Depth cues come from state, not elevation:

- On a horizontally scrollable code line, a soft 0.875rem foreground-14% fade marks the side with more to read. It is covered by panel-coloured `local` gradients at the scroll ends.
- Highlighted lines carry the `line-hl` band across the full scroll width.

**Motion.** Smooth scrolling is the only motion on a built page, and it applies only under `prefers-reduced-motion: no-preference`. Under `dev`, the floating problem panel also enters over 120ms (`ease-out`, opacity 0 and 0.375rem lower to rest), under the same media query. Under `reduce`, every transition inside an island's SDK scope, author components' elements included, is instant: zero durations and delays marked important, because the docs layer cannot otherwise outrank the utilities or the SDK's unlayered rules; the SDK spinner is still beside its status text. Mode changes, panel toggles, tab switches and copy feedback are instant.

**The Hairline Rule.** Separate with a 1px `secondary` rule or a tonal panel, never with a shadow, a heavy top bar or a thick coloured side stripe.

### Shadow Vocabulary (dev layer only)

- **Dev float** (`box-shadow: 0 4px 6px -1px rgb(0 0 0 / 0.1), 0 2px 4px -2px rgb(0 0 0 / 0.1)`): the floating problem panel and its marker, which sit over the article and must read as above it. Dropped (`none`) when the panel is inline on the nothing-built page.

## Shapes

- **Panels: one 6px radius (`--rd-radius`, 0.375rem).** This covers code panels, code groups, tables, alerts, images, pager links and the search dialog and result options.
- **Controls: slightly softer (8px).** This covers the home actions and the mode-switch frame. Mode options nest inside at 5px.
- **Chips: 4px.** This covers inline code and task boxes.
- **Pill: fully round (999px).** Only the dev marker on screens 640px and wider. On phones it takes the 6px panel radius.
- **Focus outlines: 2px corner.**
- **Icons:** drawn on a 24-unit grid with a 2-unit round stroke in `currentColor`, at 1em (1.125rem in alert titles). Every icon is `aria-hidden` and sits beside words that say the same thing.
- **Narrow-panel chevron:** a CSS-drawn bordered square rotated 45°, turning to 225° when open.

## Components

### Header, brand and mode switch

- **Brand:** an optional logo (24px tall) followed by the site name, in foreground at weight 600, truncating with an ellipsis. A light/dark logo pair switches through CSS without script. The logo has empty `alt` because the name carries the meaning. On the home page the brand is `aria-current="page"`.
- **Mode switch:** a group of three buttons (System, Light, Dark) with a 1px hairline frame and 3px padding.
  - Options are muted, 0.8125rem, each an icon plus a word.
  - The pressed option gets a `secondary` fill, foreground text and weight 500. Hovering an option turns it foreground.
  - Below 30rem the words become visually hidden and the icons carry the switch.
  - The switch ships `hidden` and appears only once `site.js` runs.
  - `mode.js` applies a stored choice synchronously in `<head>`. The storage key is per base path. When storage is refused, the page stays on system mode.
- **Header links (`nav.top`, then `nav.social`):** muted 0.875rem links at weight 500, 1.25rem apart, the social group after a 1px `secondary` hairline. Hover turns a link foreground. The link to the page in view (`aria-current="page"`) or to the section it is under (`aria-current="true"`) is primary. From 48rem they sit in the bar before the mode switch and scroll sideways inside it if they must. Below that they are a row of their own under the header with a bottom hairline, scrolling sideways by themselves, so the page never does. The row is not sticky. One of the two navigations is always `display: none`.

### Local search (P4)

Search is an on-demand DOM reading aid in the existing world, independent of demo wasm. It adds no palette, font, shadow or decorative motion.

- **Entry:** a named button beside language and mode, with a 1px `border` edge, panel radius, 0.8125rem type and 0.375rem 0.5rem padding. The desktop Ctrl / Cmd K hint is 0.75rem and hides below 48rem. The button ships disabled; the base script enables it when native modal dialogs are available. Without script it retains its name and disabled state, with a visible explanation and ordinary navigation.
- **Dialog:** a native `<dialog>` on `popover` with `popover-foreground` text, a 1px `border` edge and the panel radius. Its width is the smaller of the reading measure and the viewport minus two 1rem gutters; its maximum height is the dynamic viewport minus 2rem. Desktop padding is 1.25rem; below 48rem, padding and top margin are 1rem. The backdrop is a foreground-30% tint. There is no second H1.
- **Reading order:** a visible input label and explicit Close share the first row, followed by the input, status, polite live total, result list and Retry or Show more when relevant. The input uses `background` and `foreground`, the primary caret and an opaque 3px `ring` outline at a 2px offset. Status, paths and excerpts use 0.875rem type with 1.5 leading; result titles use weight 600.
- **Contained scrolling:** only the result list flexes and scrolls, capped at 55dvh with contained overscroll. Other dialog rows do not shrink. Long results wrap within their option; input, Close and the recovery/pagination controls remain outside the result scroll region.
- **Focus and selection:** opening from Search or Ctrl/Cmd+K synchronously focuses the input before loading the module, Worker and current-language index. Native modal behavior contains focus. Up/Down select a result through `aria-activedescendant` and `aria-selected`; Enter follows it. The selected or hovered option has a primary edge and primary-8% fill. Matched text keeps normal popover text over a primary-20% tint. IME composition defers query/selection handling until composition ends.
- **States and recovery:** distinguish closed, idle, loading, results, empty and error. Loading says that typing may continue; empty suggests another word or shorter prefix. Failure is an alert with Retry and preserves the input. Retry returns focus to the input. Results are grouped by page; the live total counts every matching page, while Show more appends the next group without replacing the current selection.
- **Close and reopen:** Escape or Close returns focus to the actual connected trigger. An unmodified result activation, including a same-page section, closes the dialog. Closing discards that session's pending work; a rapid reopen starts a fresh session, so a late result or queued close event cannot replace or dispose it. Retain the reader's input across retry and reopen.
- **Language:** titles, paths and excerpts carry their source language. A fallback result displays the target-language code badge and an explicit source-language explanation; catalog labels and totals keep each resolved message's actual language. Search stays within the selected language and deployment base, including on a deep 404.

Sources: [shell.rs](crates/rustify-docs/src/shell.rs), [docs.css](crates/rustify-docs/assets/docs.css), [site.js](crates/rustify-docs/assets/site.js), [search.js](crates/rustify-docs/assets/search.js) and [search-worker.js](crates/rustify-docs/assets/search-worker.js). The [local artifact identity](docs/validation/docs-ssg-p4/m6/artifacts/identity.json) records CLI 0.1.24 / C `18b229d9` / R `a2ad3940`, build `65ded86c0e6910b0`. Its generated search scripts match these sources.

The [finish review](docs/validation/docs-ssg-p4/m6/design/finish-review.md) records `ship` for the local M6 DOM visual scope of fifteen full-page captures: search in both themes at 1280/375/320px, text/layout emulation, loading, failure, 404 and no-script, plus Chinese official home and quickstart at 1280/375px. Search captures are C2 0.1.23/R2 evidence for the unchanged C3 static markup/CSS; the four official captures are C3 0.1.24/R3. The C3 [root](docs/validation/docs-ssg-p4/m6/browser/search-root/results.json) and [meta-CSP](docs/validation/docs-ssg-p4/m6/browser/search-meta/results.json) interaction reports pass 21/21 each; the [affected subpath checks](docs/validation/docs-ssg-p4/m6/browser/search-subpath-c3/results.json) pass 15/15. The earlier [C2 subpath report](docs/validation/docs-ssg-p4/m6/browser/search-subpath/results.json) passes 26/26 for the broader unchanged search styles and behavior. These are local evidence, not full M6 or P4 acceptance. Native stable browsers, physical phones, spoken VoiceOver, native 200% zoom and publication remain M7 gates; `search-text-200.png` is text/layout emulation.

### Languages and untranslated pages (P3)

Language selection extends the static reading shell; it adds no palette, type scale, loading state or new presentation path. It appears only when more than one language is configured.

- **Native menu:** a `<details>` beside the mode switch, with the current language's configured autonym as its summary. Each option is an ordinary `<a hreflang>` to the same logical page, labelled with that language's autonym and its own `lang`. The current option has `aria-current="page"`, primary text and weight 600. A fallback option appends the target catalog's untranslated message at normal weight, marked with that message's actual language.
- **Material and size:** 0.8125rem type with 1.5 leading; the summary has 0.375rem 0.5rem padding. The menu uses the existing `popover` surface, 1px `border` edge and panel radius, with 0.375rem inset and 0.5rem option padding. Hover uses `secondary`; focus uses the existing ring. Its width fits its content, capped at 24rem or the viewport minus two 1rem gutters; its height is capped at 60vh and scrolls internally.
- **Narrow layout:** below 48rem the menu is positioned from the header bar, 1rem from the viewport's right edge. Long option text wraps inside the gutters. At 320px the summary may wrap and the brand may truncate; language and mode controls remain reachable.
- **Navigation and keyboard:** opening, Tab and Enter use native details and link behavior. With script, Escape closes the menu and returns focus to its summary. A language link retains the current hash only when that exact decoded ID exists in the destination; otherwise it goes to the destination's top. Do not guess a translated heading ID. Without script the same-page language links remain usable.
- **Source and interface:** the document's `lang` is the selected interface language. The article, source title and source-derived navigation text retain their actual source language. Header, sidebar, pager and home/footer destinations stay within the selected language; a fallback title can remain in its source language. Built-in interface words remain English/Chinese; an optional configured catalog supplies other languages, with missing keys falling back individually to English and marked `lang="en"`.
- **Copy states:** the action, copied/failed state and polite status each carry that message's actual language. For example, French “Copier” can become English “Copied” when only the latter key is missing; neither changes the document or article language.
- **Untranslated notice:** a quiet `role="note"` paragraph follows H1 and precedes the body, including when the source supplies the leading H1. It uses `code-bg`, a `secondary` hairline, the panel radius, 0.75rem 1rem padding, 0.875rem type and 1.6 leading. Its words carry the resolved message language. This is a stable reading state, with no loading indicator or error treatment.

### Localized not-found pages (P3)

Each configured language has a physical `404.html` using its own navigation, error catalog and home destination, plus ordinary autonym links to every language home. Keep the same article frame, H1, footer and quiet recovery links. The error page is `noindex` and has no canonical or alternate-language metadata.

For a static host with one error document, the root error shell contains the default language as its active body and other language bodies in inert templates. The existing external `mode.js` chooses by the complete first path segment after the deployment base, updates document language/title and activates one body before resolving the existing islands `parsed` boundary. Shell enhancements then bind to that selected tree; shared slots and existing DOM islands occur once in it. An unknown or partial prefix uses the default language. Without script, that host retains the default error body and all language-home links; a server selecting a physical language error file can show that language without script. Existing no-script island canvases keep their native controls disabled.

These rules were checked against source R `820cea0f` and installed CLI C `bae5b882` (0.1.21), identified in the [installation evidence](docs/validation/docs-ssg-p3/m5/installed-identity-final.json), and all twelve [final captures](docs/validation/docs-ssg-p3/m5/browser/final/). The [root](docs/validation/docs-ssg-p3/m5/browser/docs-i18n-root.json), [subpath](docs/validation/docs-ssg-p3/m5/browser/docs-i18n-subpath.json) and [meta-CSP host](docs/validation/docs-ssg-p3/m5/browser/docs-i18n-meta.json) content checks each passed 8/8: native keyboard navigation and Escape focus return, actual-language attributes and copy states, destination anchors, true 404/CSP, no-script reading and light/dark 1280/375/320px reflow with reduced motion. This evidence does not establish spoken screen-reader output, native zoom, physical devices, full RTL support, dev behavior or full P3 acceptance.

### Sidebar rail

- Group headings are 0.8125rem, weight 600, foreground. Groups are 1.75rem apart.
- Each group's list hangs on a 1px `secondary` rail. Entries are muted 0.875rem links with a 0.875rem inset.
- Hovering an entry draws the rail segment in `border` and turns the text foreground.
- The current page draws the segment in `primary` and turns the text primary at weight 500.
- Draft entries append " · Draft" in the interface language.
- On load, the wide sidebar scrolls so that the current entry is visible.

### Table of contents

- Wide, it has a foreground heading "On this page" (0.8125rem, weight 600) and muted 0.875rem entries. H3 entries are inset 0.75rem.
- The section the reader is in has `aria-current="location"` and is drawn in primary at weight 500. Both TOC copies follow it.
- `site.js` tracks the last heading above a 96px line. Above the first heading it marks the first, and at the page bottom it marks the last.
- A page with no H2/H3 renders no TOC.

### Narrow panels

- The sidebar and TOC become `<details>` panels: a 0.875rem, weight-500 summary with the drawn chevron, and a hairline below.
- With script, following a link closes the panel, and Escape closes it and returns focus to the summary.

### Alerts

Note, tip, important, warning and caution:

- A 7% tint of the alert's hue, a 38% hairline border and the panel radius. Padding is 0.875rem 1rem.
- The title row pairs the drawn icon with the localized kind name, in the alert's text hue at weight 600.
- No thick side rule. No glyph icons.

### Code panel

- **Panel:** a `<figure>` on `code-bg` with a hairline border, the panel radius and 1.5rem block margin.
- **Bar:** at least 2.5rem tall, carrying the title or language label (muted, 0.8125rem, ellipsis) and the copy button, with a hairline below.
- **Pre block:** focusable (`tabindex="0"`) with an inset focus outline (−3px offset), and 0.875rem block padding.
- **Lines:** 1rem inline padding. Line numbers (opt-in) are 3ch, muted, tabular and unselectable.
- **Highlighted lines:** fill the full scroll width with `line-hl`.
- **Overflow cue:** described under Elevation & Depth.
- **Copy button:**
  - Ships hidden, so it never appears as a dead affordance without script. It holds a drawn copy icon and the label.
  - Hover gives a foreground-7% fill and foreground text.
  - Copy uses the raw source from a `<template>`, never the rendered lines.
  - **Copied:** the label reads "Copied" in tip-text for 2s, announced through the polite live region.
  - **Failed** (insecure origin, denied permission): the label reads "Copy failed" in caution-text for 4s. The full message announces that the code is selected, and the code is selected for copying by hand.

### Code group tabs

- **Without script:** consecutive titled code panels.
- **With script:** one panel under a tab row with a hairline below.
  - Tabs are muted 0.8125rem labels with a 2px transparent underline.
  - The selected tab gets a primary underline, foreground text and weight 500.
  - The group's single copy button sits at the right end of the row and copies the visible panel.
  - Arrow keys, Home and End move between tabs.
  - A choice switches every group of the same name on the page and is remembered per base path.

### DOM demos and static blocks (P2 M2)

An executable example remains part of the article, using the existing panel radius, hairlines, semantic colours and code treatment. The page shell remains static HTML; this extension adds no visual world or token scale.

The DOM canvas comes first, padded by 1rem, followed by an external status row and optional source. Without script, its outer fieldset disables native controls and a noscript explanation remains readable. Loading locks only the canvas; ready releases it without moving the status or source. Failure restores the original view and exposes an alert and Reload button in the external row. Ordinary navigation, source tabs and copying stay outside the island's input lock and failed instance. Client-only examples retain the author's fixed-size placeholder and browser explanation before attachment.

**The Stable Recovery Footprint Rule.** The status grid reserves the intrinsic size of the complete recovery message and button in every state, including ready, using invisible, aria-hidden recovery items and a compact minimum (4.5rem). Allow wrapping and larger text to increase that footprint; 72px in the reviewed default-size captures is an observation, not a fixed height. Do not restore the removed 8rem mobile floor.

Source uses the existing Function / Full file code-group controls and raw-text templates, with a quiet project-relative path. Copy preserves the original text and line endings. Both panels remain readable without script; with script, the visible source scrolls internally up to 24rem and remains keyboard-focusable.

- **Static Tabs:** all labelled panels ship visible, with the control row hidden. The base script adds unique tab/panel ARIA pairings, a primary underline and local Left/Right/Home/End selection. Peer groups, nested code groups and appended child content retain their own state and content.
- **CardGrid:** an ordinary list of native links and descriptions, separated by hairlines rather than decorative cards. It reflows to one column on narrow screens; the validated column setting permits one to three columns.
- **Steps:** an ordered list with paragraph titles and descriptions. Primary-coloured native markers convey a real sequence. Cards, steps and their checked child content remain available without JavaScript.

These static blocks create no island or wasm requirement. Sources: [demo.rs](crates/rustify-docs/src/demo.rs), [builtins.rs](crates/rustify-docs/src/builtins.rs), [docs.css](crates/rustify-docs/assets/docs.css), [mode.js](crates/rustify-docs/assets/mode.js) and [site.js](crates/rustify-docs/assets/site.js). The [M2 design synchronization record](docs/validation/docs-ssg-p2/m2/design-documentation.md) records the generated-artifact comparison, current captures, scoped finish verdict and verification limits; it does not establish full P2 or publication acceptance.

### Registered pages, layout slots and imported themes (P2 M3)

Registered pages share the reading shell, ordinary navigation, headings, source panels and resource rules. Optional descriptions remain optional. The seven slots provide author content within the existing layout: header replaces the brand area while framework navigation and mode remain; footer replaces its content; sidebar top/bottom appear once outside the responsive navigation pair; before/after content surround the article; not-found replaces the generated 404 body while its heading and shared slots remain.

Nested `RenderContext::demo` views use the same canvas, status, source and recovery treatment as fenced demos. A page or slot containing ordinary content creates no interactive island. Give illustrative resource samples visible captions and a graphic that can be distinguished from an empty form control.

An explicit `theme.document` supplies validated light/dark values and packaged font choices; it does not change the default reading theme. Keep the 36rem measure and inherited layout. Project the imported palette, derived utility corners, type size and letter spacing onto the document and scopes; reading panels use its radius. The page owns system preference and first-frame CSS, and attached scopes follow its mode through their shared context, including floating controls. Package the selected same-version font bytes and glyph fallbacks under neutral same-origin digest paths, with their notices.

Keep complete Rust class selectors for initially hidden states. Resolve their resource URL values through the same published resource graph; changing a selector would prevent a later browser branch from finding its style.

The [M3 evidence](docs/validation/docs-ssg-p2/M3.md) and [bounded finish review](docs/validation/docs-ssg-p2/m3/finish-review.md) record the actual browser and image boundaries. The 640 CSS-pixel/DPR-2 no-script capture represents 200% layout zoom emulation across all CSS pixels, including explicit-size author controls. It is not a measurement on a physical device or the user's native browser zoom setting.

### English SDK site (P2 M5)

The official English site is a Read surface in the P1 world: cool neutral surfaces, system sans and mono, a 36rem article measure, hairlines and the existing responsive shell. The configured navigation contains twelve Guides and twenty-four Components destinations in catalogue order. The documentation site uses static DOM content and explicit DOM demo islands; GPU capabilities are described through links to repository examples.

**Home.** The static page has one "Rustify UI" H1, a positioning sentence, and ordinary "Quick start" / "Components" links in the existing primary / secondary action treatment. Both actions remain visible in the 1280px first viewport. Three features cover DOM interfaces, optional GPU regions and static deployment using the existing hairline list. The introduction comes from the repository README through `RenderContext::markdown_intro("README.md")`, then follows at the reading measure.

**Categories and source.** Each of the twenty-four category pages uses CATALOG's description and capabilities with a registered real DOM demo. Retain the existing canvas, stable external status row, project-relative source path, Function / Full file tabs and Copy control. Demo registrations name their source modules directly. Capabilities use a named, keyboard-focusable `rd-table` region: `overflow-x: auto` keeps a wide table inside its own panel, with the existing table type, hairlines and code-background header cells.

**Document links.** The official Link demo explicitly sets `attr:target="_self"` for its fragment destination. This lets the document anchor use native navigation past the island's local router; it is an author choice in that demo. The shell's ordinary navigation and narrow `<details>` continue to use the same ordered destinations under the configured base.

**Default theme scopes.** Within a Docs root, `:root[data-rustify-scope] [data-rustify-scope]` inherits the SDK theme properties from the document. A nested island's light fallback must not replace the active dark or system-dark palette. Imported theme projection is emitted after the default token rules and can supply its explicit values. This is the documentation stylesheet's scope rule, not a change to the SDK's defaults in other applications.

Sources: [site registry](site/src/lib.rs), [official pages](site/src/pages.rs), [Link demo](site/src/demos/basics.rs), [site configuration](rustify-docs.toml), [docs.css](crates/rustify-docs/assets/docs.css) and `tokens_css` / `site_css_with_theme` in [style.rs](crates/rustify-docs/src/style.rs).

The [M5 design synchronization record](docs/validation/docs-ssg-p2/m5/design-documentation.md) separates these source-backed rules from acceptance evidence. The [first finish review](docs/validation/docs-ssg-p2/m5/finish-review.md) recorded `fix` for unreadable dark Data table rows; the [single confirmation](docs/validation/docs-ssg-p2/m5/finish-review-r2.md) recorded `ship` for that resolved fix and visible regressions only. Its nine local Chromium captures cover home, Data table and Quick start first viewports at 1280×900 light/dark and 375×812 light. They do not establish visual acceptance for every page or section, physical phones, Safari, performance or publication; the separate local browser results remain in [M5.md](docs/validation/docs-ssg-p2/M5.md).

### Chinese official reading pages (P4)

The Chinese home uses the existing hero, primary/secondary actions and three hairline features with real Chinese copy; its navigation stays under the chosen language. The translated quickstart keeps the source section IDs so incoming anchors survive. Other guides and component categories retain the P3 source-language and untranslated-note treatment, with their existing DOM demos and source controls. This adds no visual system or GPU presentation.

Sources: [official pages](site/src/pages.rs) and [Chinese quickstart](docs/zh/quickstart.md). The [C3 official subpath report](docs/validation/docs-ssg-p4/m6/browser/official-subpath/results.json) passes 55/55, covering the translated home/quickstart, all twenty-four Chinese fallback categories and English demos, twelve guides, no-script, theme and local automated accessibility/reflow checks. The four Chinese page captures are included in the bounded P4 finish review above; native-device, spoken-assistive and publication acceptance remain separate.

### Tables

- A table scrolls inside its own panel: a hairline border with the panel radius.
- Cells have 0.5rem 0.875rem padding, are top-aligned and separated by hairline row rules.
- Header cells sit on `code-bg` at weight 600, start-aligned. Figures are tabular.

### Task boxes

- A task box is a record, not a control. It is 1rem square, with a 4px radius, a `border`-coloured edge and a `background` fill, a default cursor and full opacity, never the browser's grey disabled look.
- Checked, it fills with primary and shows the generated check mark. The list item drops its bullet.

### Inline content

- **Links:** primary with a 1px underline at 0.2em offset. On hover the underline thickens to 2px.
- **Inline code:** a `code-bg` chip.
- **Blockquote:** a 1px `border` rule with muted text.
- **Horizontal rule:** a hairline with 2.5rem margins.
- **Images:** the panel radius.
- **List markers:** muted.
- **Selection:** a primary-24% tint.

### Footnotes

- References are 0.75em tabular superscripts without underline.
- The list sits behind a hairline, 3rem down, in muted 0.875rem. Each note ends with a drawn back-arrow icon, labelled for assistive technology.

### Page meta and pager

- **Page meta:** an optional "Edit this page" link (primary, no underline, underline on hover) and "Last updated" with a tabular `<time>`. It sits in a muted 0.875rem row behind a hairline.
- **Pager:** bordered link panels with the panel radius. Each holds a muted 0.8125rem "Previous" or "Next" label above a primary title at weight 500, and Next aligns to the end. Hovering turns the border primary.

### Home hero, actions and features

- **Hero:** left-aligned, in the frame, not centred: the site name as H1, the tagline, and up to two actions.
- **Primary action:** filled primary with the generated label colour. Hover mixes it 86% toward foreground, darker on light and lighter on dark.
- **Secondary action:** a `border` outline with foreground text and a `secondary` fill on hover.
- **Action sizing:** both are 2.75rem tall, 0.9375rem, weight 600, with 8px corners.
- **Features:** a list of a title (1rem, weight 600) and one muted sentence (1.6 leading), each item standing under a hairline. No cards, icons or illustration.

### 404

- The site shell with the H1 "Page not found", one sentence and a "Back to the home page" link. It has no TOC or pager and is marked `noindex`.
- Links are root-relative under the base, so it works at any depth.

### Dev problem panel (dev only)

An errata sheet in the corner of the page the author is reading: compact, precise, and never a curtain over the content. It exists only on pages `rustify-docs dev` serves. `dev.css` and `dev.js` are linked only when `site.assets.dev` is set, are served from `/__rustify-docs/`, and no `build` output contains either.

**Placement.**

- **Floating (a last good version exists):** fixed 1rem from the bottom-right corner, z-index 60, `min(26.25rem, 100vw − 2rem)` wide and at most 60vh tall. The list scrolls inside it (`overscroll-behavior: contain`); the head stays.
- **Bottom sheet (under 640px):** spans the width along the bottom edge, at most 50vh, with only a top border and top-only 6px corners.
- **Inline (nothing has built yet):** `render_dev_shell_with_status` uses the same static site shell, `noindex` and no TOC. During the first build its H1 is "Building your site", followed by "The first dependency build takes longer. Keep this page open: it updates when the site is ready." The failure shell retains "The site does not build yet" and its repair sentence. The panel sits 1.5rem below the sentence in the article column, full column width, open, unbounded in height, with no hide button, no marker, no shadow and no entrance.

**Surface.** `popover` with a 1px edge and the 6px panel radius. The edge is `border` when floating (it must stay visible over any article content) and `secondary` when inline (it is page content, so a hairline like every other panel). Text is `foreground` at 0.875rem/1.45.

**Head.** A row with a `--rd-dev-divider` rule below, padded 0.5rem 0.5rem 0.5rem 0.875rem:

1. **Counts** (weight 600): "3 errors · 1 warning". The error count is `caution-text`, the warning count `alert-warning-text`; the " · " between them is `foreground`. A kind with no problems is omitted.
2. **State line** under the counts, muted 0.8125rem/400: "showing the last good version" or "nothing has built yet". A visually hidden " · " keeps the sentence whole for assistive technology. When the build succeeded with warnings only, there is no state line.
3. **Revision** on the first line, muted 0.75rem, tabular: "revision 3".
4. **Hide** at the end of the first line (floating only): a 2rem square (2.5rem on phones) holding the drawn X icon at 1rem, muted, 6px radius, `secondary` fill and `foreground` on hover. Its accessible name and tooltip are "Hide problems".

**List.** One `<section>` per file, files with any error first, then the rest in the order the build reported them, so the first problem under a red count is an error.

- **File heading:** an `h2` in mono 0.75rem/600, `foreground`, padded 0.5rem 0.875rem 0.125rem, breaking only after `/`.
- **Between files:** one `--rd-dev-divider` rule with 0.375rem above and 0.25rem below. The problems of one file are separated by space only (0.5rem 0.875rem item padding), never by rules.
- **Where row:** mono 0.75rem: the location `line:col` in muted tabular figures, then the rule id in the severity's text hue (`caution-text` for errors, `alert-warning-text` for warnings), prefixed by a visually hidden "Error: " or "Warning: ". The row wraps with 0.25rem/0.625rem gaps.
- **Message:** `foreground`, 0.125rem below the row. Preserve compiler line breaks with `white-space: pre-wrap`; long text wraps inside the list.
- **Suggestion** (when present): muted, 0.125rem below the message, with the same preserved line breaks and wrapping.
- **Quoted text:** what a message or suggestion quotes in backticks is set as `<code>` in mono 0.8125rem with no chip fill, and breaks only after each `/` (a `<wbr>` per slash). Long paths therefore wrap at their segments, never mid-name.

**Marker.** What the panel leaves when hidden: a button at the same corner, `popover` with a 1px `border` edge, the dev float shadow, a 999px pill padded 0.375rem 0.75rem, `secondary` fill on hover. It holds the drawn circled-exclamation icon (1rem) and the one-line summary: "3 errors · 1 warning · showing the last good version", counts coloured as in the head. Under 640px it becomes a full-width box (0.75rem from the sides and bottom) with the 6px panel radius, and its text wraps.

**M4 building feedback.** The same marker can show "Building", the current native-renderer/browser-module/page-rendering phase, retained diagnostic counts and the last-good state. Building and phase text inherit `foreground`; only actual error and warning counts take their semantic hues. The underlying document and controls remain available. An earlier error remains readable while the next save builds; it clears only when the new result clears it. First-build phase and revision use the existing inline panel, not an island, spinner or estimated percentage.

**The Last-Good Feedback Rule.** Change build feedback within the existing dev layer while the last successful document remains available. A completed result clears building; its actual diagnostics determine whether the panel or marker remains.

**States.**

- **First build running:** the static building shell and inline region show the current phase and revision. Heading and explanation exist before the dev script; stage feedback is added by that external script.
- **Building over last good:** retain the complete page and previous diagnostics, with neutral building/phase text in the existing panel or marker. Building alone does not force a floating panel open or steal focus.
- **Failed, last good shown:** the page stays the last good version; the panel opens by itself unless the author hid it earlier in this tab (remembered in `sessionStorage` until the problems clear).
- **Hidden:** only the marker shows.
- **Warnings only:** the panel does not open by itself; the marker shows the warning count.
- **Nothing built:** the inline region on the dev shell page.
- **Same failure saved again:** the revision number updates and the open or hidden choice stays.
- **Recovered:** once building ends, a result with no problems hides panel and marker and forgets the hidden choice. Warnings retain their marker; the building label clears. A build that changes the page reloads it and restores the scroll position.
- **Reconnected:** the event stream resynchronizes using the page path, last good revision and dev session. A dropped stream retries; a closed stream is opened again after 1s. A changed page or session reloads through the existing scroll-restoration path. No separate connection banner or recovery action is added.
- **Language:** English/Chinese `DevLabels` are the built-ins; configured catalogs resolve the layer's `dev.*` messages individually, retaining each message's actual language.

**Motion.** The floating panel enters over 120ms `ease-out` from opacity 0 and 0.375rem lower. Under `prefers-reduced-motion: reduce` there is none, and the inline panel never animates.

**Keyboard and assistive technology.**

- The floating panel is `role="dialog"` with `aria-modal="false"`, labelled by the counts line. It is in the tree only while expanded. Inline, the panel is `role="region"` with the same label.
- The list is a focusable (`tabindex="0"`) region named "Problems", so a keyboard can scroll it.
- The marker is a button with `aria-controls` on the panel and `aria-expanded`.
- Focus is never moved by a build, with one exception: when the problems clear while focus is on Hide or the marker, focus continues from the page's `main` (`#content`, without scrolling) instead of dropping to the top of the document. Otherwise only the author's own action moves it: Hide (or Escape inside the floating panel) hides to the marker and focuses it; the marker opens the panel and focuses Hide. The inline list on the nothing-built page has no Hide and ignores Escape.
- Hide and the marker take the theme's 3px `ring` outline at a 2px offset.
- One visually hidden `role="status"` paragraph exists from the start. It announces the summary when building starts or its phase changes, the summary and revision when a failure with errors arrives or its revision changes, and "Problems fixed; the page is up to date" when errors clear. Warning-only results are not announced.

The [M4 design synchronization record](docs/validation/docs-ssg-p2/m4/design-documentation.md) links the completed source and nine supplied captures. Its [finish review](docs/validation/docs-ssg-p2/m4/finish-review.md) records `ship` for that visual packet, with no material fix; static captures do not establish focus, speech, reconnect, performance, physical-phone or native-zoom acceptance.

### States

- **Focus:** every `a`, `button`, `summary` and `[tabindex]` gets an opaque 3px `ring` outline at a 2px offset, inset (−3px) inside code panels and tab rows. A skip link ("Skip to content") appears at the top left on focus.
- **Problems (dev only):** see "Dev problem panel".
- **Draft (dev only):** a "Draft" label above the title, foreground 0.8125rem/500 with a 1px `warning` border. The sidebar entry carries " · Draft".
- **No script:** every page is complete. The mode follows the system through CSS, code is readable and selectable, panels open natively, and code groups read as consecutive blocks. Script-only controls (mode switch, copy) ship `hidden`.
- **Blocked storage:** choices hold for the current page only.
- **Interface language:** `labels.rs` supplies English/Chinese built-ins. Without configured overrides, other page languages use English UI marked `lang="en"`; optional configured catalogs resolve each key separately. The document's selected UI language and the body's source language remain distinct, including on untranslated projections.

## Do's and Don'ts

### Do:

- **Do** keep prose, code and tables in the 36rem column, and let wide content scroll inside its own panel.
- **Do** use primary only for links, the current page or section, the selected tab, checked task boxes, the primary action, focus and restrained search selection/match tints. Let `ring` and `primary-foreground` follow from the brand pair.
- **Do** separate with 1px `secondary` hairlines and the `code-bg` tone. Use the single 6px panel radius for any new panel.
- **Do** draw new icons on the 24-unit, 2-stroke `currentColor` grid, hidden from assistive technology, next to a word.
- **Do** ship script-dependent controls `hidden` and reveal them from `site.js`. Search is the explicit exception: it ships disabled with its name and a no-script explanation, while ordinary reading navigation remains available.
- **Do** confirm actions in place and through the `#rd-status` polite live region, in the page's interface language.
- **Do** gate any new motion behind `prefers-reduced-motion: no-preference`.
- **Do** keep dev-only feedback in the corner panel and marker: `caution-text` and `alert-warning-text` for counts and rule ids only, locations as mono `line:col`, and focus moved only by the author's own action.

### Don't:

- **Don't** load web fonts or IBM Plex Sans on documentation pages. The theme is system sans and system mono, under a strict CSP and a per-page weight budget.
- **Don't** add shadows, blurred or heavy top bars, coloured sidebars, or cards on the home page or in content. The dev float shadow belongs to the dev layer alone and never ships.
- **Don't** report build problems as a full-screen overlay or a top banner that pushes the page, and don't let any dev-only asset reach a `build` output.
- **Don't** give alerts a thick side stripe, or use Unicode glyphs or emoji as icons.
- **Don't** use primary or the alert hues as decoration or as a heading colour.
- **Don't** style task boxes as interactive controls, or let them fall back to the browser's disabled grey.
- **Don't** apply this ramp, measure or frame to Theme Studio or SDK components. DESIGN.md governs those.
