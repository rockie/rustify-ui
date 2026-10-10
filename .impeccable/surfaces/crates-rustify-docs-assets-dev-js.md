---
version: 1
slug: "crates-rustify-docs-assets-dev-js"
primary_target: "crates/rustify-docs/assets/dev.js"
related_targets: ["crates/rustify-docs-cli/src/dev.rs"]
---

Mode: Operate. Target: the diagnostics layer `rustify-docs dev` adds on top of the default theme while an author writes: the error panel, its collapsed marker, the no-last-good diagnostic shell and the draft labels. Built in M6 (plan §5.5, §6 "dev 浮层"); never present in a production build.

Audience and job: an author with the page open beside the editor. They save, glance at the browser, and need to know within a second whether the page is current, and if not, exactly which file, line and column to fix and how. Success: fix and see the panel clear on the next save without restarting, losing scroll or losing their place in the text.

Confirmed with the user 2026-10-10: a floating panel at the bottom right that can be dismissed to a keyboard-reachable marker; not a top banner.

States and ranges: 0 to hundreds of diagnostics (errors and warnings), paths up to ~120 chars, messages and suggestions of one or two sentences; building (keep showing last good); failed with a last good generation (page readable underneath, panel lists problems, marker reads "N errors · showing the last good version"); failed with no last good (a diagnostic shell page in place of content); recovered (panel and marker clear by themselves, focus not stolen); the same failure saved again (revision updates, marker stays); connection lost and regained (state resynchronises). Drafts show "Draft / 草稿" in the page header and sidebar entry.

Constraints: same origin, CSP-safe external script, no wasm, keyboard and screen-reader operable (dialog semantics only while expanded; marker is a button; live region announces counts, never steals focus from reading), respects reduced motion, works at 320 px (panel becomes a bottom sheet capped in height and scrolls), interface in the page language.

## Direction contract

THESIS: The page stays the author's: problems surface as a compact, precise errata sheet in the corner, never a curtain over the content. Refuses full-screen error overlays and top banners that shove the page.

OWN-WORLD: The default theme's neutral popover surface with a hairline border and the destructive red used only for counts and the error rule; warnings in the warning hue; locations in the system mono as `path:line:col`; suggestions in muted text under each message. No shadows beyond the SDK popover elevation.

STORY: Save, see a red count and the first location, fix it, save, watch the panel empty and the marker disappear while the scroll position holds.

FIRST VIEWPORT: Bottom-right panel ~420 px wide, max 60 % viewport height: header row with "3 errors · 1 warning", revision and a close button; scrolling list grouped by file, each item location (mono), message, suggestion. Collapsed: a pill at the same corner, "3 errors · showing the last good version". At <640 px it spans the width as a bottom sheet.

FORM: Corner errata panel with dismiss-to-marker, chosen by the user over a top banner. Seed: user-pinned-docs-ssg-p1-dev.

FINISH: unreviewed and undocumented is unfinished; this build ends with the finish review, the verdict, DESIGN.md, and every shipping raster carrying its provenance
