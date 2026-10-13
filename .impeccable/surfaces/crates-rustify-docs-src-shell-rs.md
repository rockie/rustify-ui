---
version: 1
slug: "crates-rustify-docs-src-shell-rs"
primary_target: "crates/rustify-docs/src/shell.rs"
related_targets: ["crates/rustify-docs/assets/docs.css","crates/rustify-docs/assets/site.js","crates/rustify-docs/assets/mode.js","crates/rustify-docs/assets/islands.js","crates/rustify-docs/src/style.rs"]
---

Mode: Read. Target: the Rustify Docs default theme — every page a `rustify-docs build` writes (home, content, 404) and its two small scripts. Scope is docs/plan/DOCS-SSG-P1.md §6 for P1; components, demos, search and language switching are later phases and must not appear as dead affordances.

Audience and job: developers evaluating or using a library, arriving from search, a README link or a shared anchor, often mid-task. Their question is concrete ("how do I install / what does this option do"); success is finding the section, reading it at a comfortable measure, copying code exactly, and moving to the next page. Authors are the second audience: their Markdown must look deliberate with no styling work.

Confirmed with the user 2026-10-10: home is a quiet docs entry (identity, one-line positioning, two plain-link primary actions, feature grid; no illustration or decorative cards); code panels follow the mode (light panel in light mode); the dev diagnostics overlay has its own brief.

States and ranges: titles 2–80 chars, CJK and Latin; sidebars from 3 to 300 entries in several groups; pages from one paragraph to 40 sections; code from one line to 200 lines and long unbreakable lines; tables wider than the column; zero to five alerts; no TOC when a page has no H2/H3; drafts (dev only) marked in header and sidebar; logo absent, single or light/dark pair; brand primary any pair passing D11 contrast. No script, blocked storage and refused clipboard are real states, not errors to hide.

Constraints: static HTML before any script, no wasm, no inline style or script (strict CSP), system sans and mono only, one stylesheet compiled from used classes, `data-theme` system/light/dark on the root, ≤100 KB gzip per page with its CSS/JS, LCP ≤2.5 s and CLS ≤0.1 on Slow 4G, AA contrast in both modes, 320 px and 200 % zoom without page-level horizontal scroll, reduced motion respected. Breakpoints: ≥1280 px three columns; 768–1279 px two columns with the TOC folded into a native details; <768 px one column with sidebar in a native details. Interface strings follow the page language (en/zh, English fallback marked lang=en).

## Direction contract

THESIS: A page you read, not a portal you browse: the article column owns the screen and everything else is quiet wayfinding that steps back. Refuses the default docs chrome of heavy top bars, card-tiled homes and colored sidebars.

OWN-WORLD: Rustify's cool neutral surfaces (#f9fafb / #0c111d), slate text, one blue (configurable primary pair) reserved for links, the current location and focus. Hairline dividers, no shadows, 6 px radii on panels, system sans at ~16 px body with generous line height, system mono for code on a slightly deeper neutral panel. Alerts are tinted notes with a hairline border and a drawn icon in their semantic hue; no thick side rules, no Unicode glyphs as icons.

STORY: Readers land on the heading they were sent to, see where they are (sidebar current item, TOC location), read without distraction, copy a block with one click and a spoken confirmation, and continue with previous/next. On the home page they learn what the project is in one sentence and take one of two paths.

FIRST VIEWPORT: Content page at 1280 px: 56 px header (logo + site name left, mode switch right), 240 px sidebar left, article column ~720 px with H1 at ~32 px, TOC right in muted small type with the current section marked in primary. Home at 1280 px: left-aligned identity block in the article column — site name as H1, tagline, two buttons (primary filled, secondary outline) above the fold, then a two- or three-column feature list of title + one sentence separated by hairlines. Signature interaction: the TOC tracks the section under the reader, and copy confirms in place and to screen readers.

FORM: Approved three-tier reading layout from plan §6 (sidebar / article / TOC, collapsing to native details). Seed: user-pinned-docs-ssg-p1; no concept tournament because structure, density and mode were fixed by the plan and the user's answers.

FINISH: unreviewed and undocumented is unfinished; this build ends with the finish review, the verdict, DESIGN.md, and every shipping raster carrying its provenance

## P2 extension: a demo inside the article

The approved P1 reading world stays in place. Under DOCS-SSG-P2 §6, a reader now compares a working DOM component with its exact Rust implementation. The demo remains a paragraph-sized piece of evidence within the article; the page, navigation and reading controls remain static HTML.

Composition: one hairline-bounded region, a padded canvas first, a persistent status row directly beneath it, then the existing code-panel treatment. Keep the status row's minimum height through loading, ready and failure so attaching the component does not move the source. The canvas contains only the registered view; no decorative toolbar or duplicate product identity. Source controls use native buttons with visible focus and the existing understated active underline: “Function” and “Full file”, followed by “Copy code”. The file's project-relative path is quiet context, never a machine path. Source panels scroll internally, with a bounded height of 24rem and no page-level horizontal overflow; keyboard focus can reach the scroll area. Copy uses the existing raw-text template, exact line endings, an in-place result and a spoken confirmation.

State contract: without scripts, native controls are disabled by their outer fieldset and “Enable JavaScript to interact” appears in noscript; the view and source remain in the accessible tree. With scripts, “Loading interactive example…” occupies the external status row and only the canvas is inert. Ready releases input without changing the canvas or status dimensions. Failure restores the initial view, states “This example could not start. Reload the page to try again.” and puts “Reload page” in that same row, outside the locked canvas. Notify failure once with role=alert; retain source tabs, copying, ordinary links and the shell. Client-only components have the author's fixed-size placeholder and explanation; no empty canvas is published. Empty source is an authoring error, while empty child content is valid.

Ranges: three identical demos on one page, full files with Unicode, CRLF and long lines, closed floating controls that open later, 375px and 200% zoom, light/dark/system and reduced motion. Reuse semantic tokens for status and failure in both themes, with AA text and focus contrast. Loading feedback is text first; any spinner respects reduced motion. A failed wasm instance makes all its demo blocks visibly terminal and cannot become ready through a late callback.

Acceptance: browser checks cover input, source switching/copying, no-script/loading/ready/failure/client-only, focus return from floating content, console/CSP and failed requests. Capture desktop and mobile together after implementation, perform a bounded finish review, and record the verdict at its tested scope. This extension specifies the new states; it does not claim they have been built or accepted.

## P2 extension: registered pages, layout slots and imported themes

Registered pages keep the same reading column, heading hierarchy, ordinary navigation and source controls. Their HTML and explicit demos enter the same final document and resource checks as Markdown. A missing description does not produce an empty diagnostic panel for the reader. The page title and a usable way onward remain present.

Slot content appears once per document. Header overrides occupy the identity area while framework navigation and theme controls stay reachable. Sidebar top/bottom content surrounds the existing wide/narrow navigation pair in its shared container; never clone an island between responsive copies. Before/after-content additions follow the article's spacing. The footer and other shared additions also appear on the not-found page, whose override changes only the error body. Keep hydration markers and the external demo state/source region intact through every insertion.

Without an imported theme, preserve the P1 system fonts and cool neutral reading world. An explicitly imported Theme Studio document supplies both color modes, radius and typography to the page and its islands from one validated projection. System mode selects the first frame through the existing external head script and CSS; islands share that preference. Use packaged fonts and native fallbacks with no remote requests. Preserve legibility, visible focus, bounded source scrolling and the quiet relative-path treatment in both modes. Theme or slot errors stop a candidate build and retain the previous output rather than publishing an empty page.

Acceptance extends the existing browser scope to registered navigation order, cross-page anchors/resources, all seven slots and deep 404, repeated context demos, light/dark/system first frame, packaged font bytes and fallback, no-script, 375px and 200% reflow. Capture the affected desktop/mobile states together for one bounded finish review and synchronize the observed rules into DESIGN.docs-ssg.md; this brief does not certify the implementation.

## P2 extension: the English SDK site

The official site remains a Read surface in the approved P1 world. Its static home uses “Rustify UI” as H1, one sentence of positioning, and ordinary “Quick start” and “Components” links in the existing primary/secondary action treatment. Keep both actions visible in the 1280px first viewport. Follow with three concise features grounded in the repository: DOM components, optional GPU regions, and static deployment. Use the existing hairline feature list and system typography. Render the introduction directly from the repository README through the Markdown writer; its quick-start link must still make sense outside the README. No copied introduction, illustration, new palette, or additional home controls are needed.

Guides contains exactly the twelve selected source files in the plan. Components contains the twenty-four CATALOG categories in catalogue order. Category pages keep the existing article heading, a short description and at least one real DOM demo with its exact function/full-file source controls. Use the existing narrow-screen navigation details for the same ordered thirty-six destinations; current-location state and ordinary links must work at the root and under /rustify-ui/. GPU capabilities are described with links to the relevant repository examples. The documentation site itself uses no GPU presentation.

Reuse the demo state and accessibility contracts above for every category. Check the longest navigation labels, wide tables and source lines at 320/375px and 200% zoom, together with light/dark/system mode, keyboard focus, no-script content and deep not-found navigation. A missing source or invalid Markdown link is a build diagnostic; retain the previous successful preview rather than publishing an empty article. After implementation, batch home/category/guide desktop and narrow captures for one bounded finish review and synchronize the observed rules into DESIGN.docs-ssg.md. This extension is a design specification, not evidence that the official site has passed acceptance.
