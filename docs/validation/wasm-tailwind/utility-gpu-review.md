disposition: fix

## persistence

Pass for this local Utility Lab UI extension. PRODUCT.md and incumbent DESIGN.md exist; the surface contract and quality bar are recorded in `utility-gpu-design.md`. Its FORM explicitly identifies a local extension with no concept-seed, approved comp, new identity or shipping raster, so no comp/seed gate applies. The contract honestly records that it was written after implementation. Existing design files were reviewed as authority, not rewritten.

Both required captures were opened: `utility-gpu/matrix-desktop.png` is 1280 × 1522 and `utility-gpu/matrix-mobile.png` is 390 × 1994. They show the document top, full content, readable inputs and actual GPU samples without blank or black regions. `matrix-main-manifest.json` identifies build `a6d42f3c4669da61`. The supplied browser reports record 6 GPU, 12 runtime and 7 deep-path checks passed, with no skipped, unexpected or flaky results. This is a file-and-capture review; I ran no browser, server, detector or product tests. Scope is the captured UtilityLeaf preview/control UI, not RustifyButton integration, compiler completeness, M5 acceptance or the whole WASM-TAILWIND goal.

## fidelity

| Element / promise | Assessment | Evidence |
| --- | --- | --- |
| TYPE | match | Screenshots retain the compact IBM Plex Sans tool shell. `app.css` explicitly uses Liberation Mono for textarea/pre; DESIGN.md typography declares Liberation Mono in `color-code` and describes its mono adapter. Built `theme-fonts.css` declares the bundled LiberationMono-Regular.ttf. The detector's `mono` warning is not evidence of a new font. |
| MATERIAL | match | Real DOM and GPU samples, neutral white drawing surfaces and thin boundaries serve the tool task. No imitation physical material or substituted illustration. The GPU report records actual color/font/geometry and lifecycle checks. |
| GROUND | match | The captures retain the cool neutral incumbent field; `app.css` declares #f9fafb with #1d2939 foreground, as the inherited OWN-WORLD requires. |
| THESIS / class entry and Apply | match | Complete class input, clear Apply action, compiler restart and CSS copy are visible in both captures. Form submit and compilation/loading/error handlers are present in `app.js`. |
| STORY / four-node inputs and output | match | Button input remains primary; named container/text/color-block inputs and the generated CSS/GPU reports are discoverable disclosure controls. Independent scope and unscoped fixture remain visible. |
| FIRST VIEWPORT / desktop comparison alignment | contradicted | Desktop DOM panel begins at about y=549 and GPU panel at y=573: the GPU explanatory paragraph wraps to two lines while DOM uses one. Equal 264px sample frames and their contents are vertically offset by 24px, weakening the side-by-side geometry comparison (`app.css:21–27`, screenshot). |
| FIRST VIEWPORT / mobile composition | adaptation | The 390px capture stacks DOM, GPU and independent scope, wraps actions and long input text, and shows no visible horizontal overflow. This is explicitly required by FIRST VIEWPORT; `app.css:41` supplies the single-column layout. |
| Keyboard / disabled sample | match within supplied evidence | `app.js:29–36` forwards focus, Space/Enter and click from the DOM role=button mirror; `main.rs` preserves disabled state and suppresses disabled activation. The GPU browser report records pointer, keyboard focus-visible and disabled checks passed. No screen-reader certification is inferred. |
| Support-boundary diagnostics | contradicted | `main.rs:170–174` summarizes only the button, while the pretty-printed report contains four node statuses. `app.js:61–62` reports “Applied” from compiler support alone. A rejected/partial container, text or swatch can therefore leave the prominent display reading “Applied”, “GPU ready”, and “Button: applied”; its actual outcome is only inside collapsed JSON. This does not fulfill the four-node support-boundary promise clearly. |
| GPU failure and recovery | contradicted | `main.rs:167` discards the `RegionState::Failed` payload and supplies only “GPU unavailable. DOM preview remains available.” There is no GPU failure reason or recovery instruction in this branch; Restart compiler calls only `utility_lab_retry` (`app.js:66–68`). The context-loss test proves successful automatic restoration, not actionable terminal failure UI. |
| FORM / inherited world | match | Local extension keeps the incumbent palette, fonts, control corners and thin separators. No approved-comp obligations apply. |

## ceiling

The Operate quality bar is limited by comparison alignment and failure/support feedback, rather than missing ornament or motion. Current typography, restrained boundaries, disclosures and mobile stacking use the incumbent world's native devices. No decorative imagery or motion is owed. Detector exit 2 leaves linked-CSS coverage incomplete in source; the built font declaration resolves the one supplied font warning, without asserting a complete detector pass.

## material_fixes

1. **GPU failure / craft-floor States and Copy:** retain a readable failure reason from `RegionState::Failed` beside the GPU status and provide a valid recovery instruction or action; distinguish GPU recovery from compiler restart, and keep DOM editing available. Verify the terminal failure branch, not only automatic context restoration.
2. **THESIS / four-node support boundary:** show a concise visible rejected/partial summary for every affected node, with its candidate/property/reason and a useful recovery such as removing the class or choosing supported-only application; prevent generic “Applied” and a button-only success summary from implying that all four GPU nodes applied. Keep full values in the existing disclosure.
3. **FIRST VIEWPORT / comparison alignment:** align the top edges of the DOM and GPU preview frames at desktop widths despite different explanatory text lengths; keep the existing mobile stack and frame sizes. Recapture the same desktop/mobile files to verify alignment and wrapping.

## keep

Keep the inherited neutral shell, single Apply emphasis, real GPU pixels and truthful applied/drawn revisions, native editable class fields, independent scope, compact disclosures, keyboard mirror and mobile stacking; do not substitute a fake successful preview or extend this verdict to RustifyButton or the full plan.

## verdict

Verdict pass over the original three fixes only, using build `9ae7212d2dc204b7`. The same required captures were reopened and are valid: desktop is actually 1280 × 1621, mobile is 390 × 2029. The additional terminal-failure and support-boundary captures were also opened. The supplied GPU report records all 8 checks passed with no skipped, unexpected or flaky results. No browser/server/detector was run by this reviewer. The original build's evidence remains historical in `utility-gpu/previous-a6/`.

| Original material fix | Score | Evidence |
| --- | --- | --- |
| 1. Terminal GPU failure reason and recovery | resolved | `gpu-terminal-failure.png` visibly identifies the missing WebGL2 context, states that DOM editing remains available, and instructs the user to copy class inputs and reload to retry GPU. The DOM preview shows the new input applied; `main.rs:167` now retains the Failed reason. |
| 2. Visible outcomes for all four nodes | resolved | `gpu-boundaries.png` shows Container/Text/Color block rejected beside the existing Button applied summary; each rejected node has visible candidate, property, reason, and remove-class/supported-only recovery. Full JSON remains folded below. `main.rs:174–185` emits these summaries for all nodes, and `app.js:62` now distinguishes DOM installation from GPU results. The supplied boundary check includes visible rejected/partial outcomes. |
| 3. Desktop frame alignment and mobile preservation | resolved | `matrix-desktop.png` shows both frame tops at approximately y=637 and both sample tops at approximately y=662; the supplied alignment check passed. `matrix-mobile.png` retains the stacked layout and readable wrapped controls. |

## remaining

One regression introduced by fix 3 remains: `.previews` retains `gap: 32px` while the new child subgrids inherit that value between heading, description and frame (`app.css:21–22`). The desktop recapture visibly adds 32px twice inside each comparison group; the GPU frame starts around y=637 instead of the original y=573, and its ready status moves from about y=853 to y=917. This dilutes the incumbent compact hierarchy and makes status less immediate. Separate the desktop column gap from the internal row gap, retaining aligned frames and the existing mobile spacing, then recapture the same files. No additional issue hunt was performed; the RustifyButton integration and full M5 remain outside this scoring pass.

disposition: fix

## verdict

Second verdict pass, restricted to the remaining spacing regression and preservation of the original three fixes. All four same-named captures were reopened and are valid: desktop 1280 × 1557, mobile 390 × 2029, terminal failure 1280 × 1570, and boundaries 1280 × 1729. `spacing-root-browser.json` records 15 expected checks with no skipped, unexpected or flaky results. Manifest build remains `9ae7212d2dc204b7`; the CSS-only correction is visible in `app.css:21` and the new captures. No browser/server/detector was run by this reviewer.

| Scored item | Score | Evidence |
| --- | --- | --- |
| Remaining desktop spacing regression | resolved | `app.css:21` separates `column-gap: 32px` from `row-gap: 0`. Desktop frame tops return to approximately y=573 together, and GPU ready returns to approximately y=853: both move up 64px while comparison alignment remains intact. The mobile capture retains the previous stacked layout and height. |
| Original fix 1: terminal GPU failure reason and recovery | resolved | The refreshed failure capture still names the missing WebGL2 context, retains usable DOM editing, and gives copy-inputs/reload recovery instructions. |
| Original fix 2: visible outcomes for all nodes | resolved | The refreshed boundary capture retains visible Container/Text/Color block rejected summaries and candidate/property/reason/recovery text, alongside the separate button result. |
| Original fix 3: aligned desktop frames and preserved mobile layout | resolved | The refreshed desktop frames remain aligned after removing the internal row gaps; the refreshed mobile capture remains stacked and readable. |

## remaining

Clear for the scored fixes. No regression from the spacing correction was observed in these recaptures. This ship disposition covers only the original three fixes and their spacing regression; it is not whole-surface, RustifyButton integration, M5 or WASM-TAILWIND goal approval. No new issue hunt was performed.

disposition: ship
