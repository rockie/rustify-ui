# P1 known limitations

Everything this preview does not do, could not verify, or did on purpose in a
way a reader might otherwise mistake for a general capability. Nothing here is
a promise about a later release.

## Not built

- **Nine of the eighteen component categories** are absent: link, icon, radio, switch, select, progress, tooltip, tabs, scroll area. `rustify_ui::CATALOG` and [`docs/components.md`](../../components.md) say `no` in every column for each, with a reason.
- **No router, deep links or history.** A region is already refused any URL, history or title change; the application side of navigation is P2.
- **No workspace**: no resizable panels, tabs or command palette.
- **No large-data list, tree or table.** The fixture's thousand objects are P1's own load and are not the PRD's B1.
- **No general cross-region drag, clipboard or file import/export.**
- **No GPU text editor.** A region displays text and hands editing to a native control placed over the rectangle it drew.
- **No trap isolation between scopes.** One wasm module serves the page; a trap ends every scope on it, and the page says so and asks for a reload (ADR-1, D9).

## Built, with a boundary worth naming

- **Overlays are placed under their anchor** and are never flipped or clamped to the viewport.
- **A modal makes its own scope `inert`.** It does not trap focus against the browser's UI or against the host page outside the scope.
- **A local theme override around a GPU region is written but unexercised.** Both examples put regions outside overrides, so "a region inside an override draws with the patched values" is code that compiles and is not verified.
- **The reduced-motion token is published and read**, but neither example has an animation long enough to see the difference.
- **The zoom matrix is a device pixel ratio plus a scaled CSS viewport**, which is what browser zoom does to a page — not a person operating the zoom control.
- **One third-party DOM component is verified** (noUiSlider 15.8.1) and nothing is claimed about any other library or version.
- **Fonts are per region.** Two regions drawing the same script hold two copies in linear memory; four regions occupied ~124 MB of linear memory at start-up. Recorded as a cost to audit, not a leak.
- **A region's default family covers Latin only.** The CJK and emoji faces are 30 MB together, so a label is moved to the wider family when the value it draws needs it — and that is when those files are fetched. A value that mixes scripts in a label that has not yet moved draws the missing glyphs as `.notdef` until the next projection.

## Measured, but only here

- **Every GPU figure is a software figure.** The headless browser runs on SwiftShader.
- **Memory does not flatten over two hours.** The two-minute run's tail is flat; the two-hour run's is not, and the test fails on it. It is an open defect against V12, not a measurement - see [the performance report](performance.md).
- **Transfer figures are from a loopback server with no compression.** The gzip figure that a compressing host would send is reported separately by `cargo xtask report-size --compressed`.
- **No PRD budget is claimed.** A-6 stands: P1 delivers baselines, and the PRD's R29/R30 numbers were never approved.

## Verified as survivable, not as visible

- **Damaged and truncated assets are silent.** They arrive with a 200, so the load succeeds and the failure happens in decoding, deeper than the loader's hook. All three classes are verified as survivable; only the missing one is reported.
- **`RuntimeFatal` never reaches the record.** After a trap the module cannot be called; only the page can say anything, and its static notice is what is verified.

## Not handed in

The four manual records — pinyin, VoiceOver, contrast and reflow, Safari — are
missing. `cargo xtask verify --suite p1` lists them and says plainly that P1
cannot be called complete without them. See [the accessibility
report](accessibility.md) for what each has to contain.
