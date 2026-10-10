---
version: 1
slug: "crates-rustify-docs-src-shell-rs"
primary_target: "crates/rustify-docs/src/shell.rs"
related_targets: ["crates/rustify-docs/assets/docs.css","crates/rustify-docs/assets/site.js","crates/rustify-docs/assets/mode.js","crates/rustify-docs/src/style.rs"]
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
