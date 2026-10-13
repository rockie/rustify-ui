# M3 design synchronization

The extension brief in `.impeccable/surfaces/crates-rustify-docs-src-shell-rs.md` was prepared before layout changes. Existing Read direction, system-font default, 36rem measure and stable demo recovery treatment remain the default contract. Explicit theme imports provide the author's palette, typography and radius.

- `DESIGN.docs-ssg.md` now documents registered pages, the seven slots, static versus interactive content, shared mode context, neutral font resources and preserving hidden class selectors while rewriting their resource values.
- `docs/themes.md` explains the JSON configuration, validation, font and licensing contract. `sources.lock.json` identifies the unchanged font bytes and the older Liberation source/notice.
- The [finish review](finish-review.md) opened the initial six captures and identified the two unlabelled outline graphics. One material fix changed these to filled bars and added visible image/background captions. The final fixed batch contains seven screenshots, including deep 404 and layout zoom emulation; recheck disposition is ship, scoped to the fix and supplied evidence.
- [Subpath results](custom-subpath-r3/results.json) are 12/12; [meta/static-host results](custom-islands-meta-r1/results.json) are 36/36. Actual font response bytes and zoom metrics are in [font-and-zoom-summary.json](custom-subpath-r3/font-and-zoom-summary.json).

The original root-font-size doubling did not scale explicit-pixel author controls, so the final no-script check uses 640 CSS pixels at DPR 2 for a 1280 physical-pixel window. Its input remains 14 CSS px and is 28 physical px. This is declared emulation, with native browser zoom, physical phones, first users and full P2 gates retained as separate boundaries. No new context/detector round or full visual redesign was run.
