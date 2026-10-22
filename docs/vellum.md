---
description: How the Vellum editor example maps onto Rustify UI's DOM and GPU layers, with reproducible figures and twin comparison results.
---
# Vellum example and framework capabilities

This example ports Vellum's editing rules to Rust and uses Rustify UI to manage projection, lifecycle and input across DOM and GPU regions. See the [VELLUM plan](plan/VELLUM.md) for implementation status.

Final validation passed 142 Rust tests and 80 tests across 11 independent browser scripts, including 36 original smoke checks run consecutively in one session. The six startup images differed by 0.0335%–1.651%, each within the user-approved ≤2% limit; menu, modal and post-edit images also passed. CI includes example builds and independent browser tests. The [final acceptance report](validation/vellum/m8.md) contains the evidence and the two device-input checks still awaiting manual validation.

## Five-layer mapping

| Vellum layer | Rustify UI counterpart | Implementation status |
| --- | --- | --- |
| [Scene canvas](../examples/vellum/src/region.rs) | One [GpuRegion](../crates/rustify-ui/src/region.rs) and application shaders | Scene, clipping, blending, batching and memory probes passed |
| [Overlay canvas](../examples/vellum/src/overlay.rs) | DOM Canvas 2D and the [Rust pointer state machine](../examples/vellum/src/pointer.rs) | Pointer state machine, all overlay graphics and twin comparisons passed |
| [Text editing layer](../examples/vellum/src/shell/text_session.rs) | SDK [TextEdit](../crates/rustify-dom/src/text.rs) | Native sessions, transforms, composition input and transactions passed |
| [DOM shell](../examples/vellum/src/shell/mod.rs) | Leptos, [ThemedScope](../crates/rustify-dom/src/theme.rs) and [Layer](../crates/rustify-dom/src/overlay.rs) | Controlled inspector, layer tree, menus and modals passed |
| [Persistence](../examples/vellum/src/storage.rs) | Application IndexedDB/localStorage and [mount cleanup](../crates/rustify-dom/src/mount.rs) | Debounced saves, fallback, failures and recovery after instance restart passed |

## Responsibilities by capability

| Capability | Framework provides | Application implements | Additional work with bare wasm and handwritten HTML/JS |
| --- | --- | --- | --- |
| Document model, geometry and history | Editing rules belong to the application | `.vellum` validation, scene transforms, editing commands and transaction history | The same business rules and validation |
| GPU regions and projection | `GpuRegion` creation, retries, context recovery, props/actions scheduling and unmount; runtime frame and GPU-byte accounting | Scene packing, Vellum shaders, raster texture cache, visible-object and draw-count statistics | GPU runtime integration, DOM size synchronisation, rebuilding and reprojecting after recovery, resource cleanup, scheduling and diagnostics |
| Page state and themes | Leptos signals and controlled views, `ThemedScope`, scope cleanup | Document, camera, page selection and Vellum markup/styles | State subscriptions, DOM updates, theme propagation and subscription cleanup |
| Text/path/image rasters | Makepad `ImageBuffer` upload, region texture reclamation, mount-scope cleanup | Canvas 2D painters, grapheme wrapping, image cover, raster LRU, font loading and invalidation | The same browser measurement and painters, plus texture upload and lifecycle integration |
| Workspace shell and overlays | Leptos controlled views, SDK `Layer` stack with modal Tab cycling, overlay root, `ThemedScope`, `Draft`, toast queue (`provide_toasts`), `listen`/`is_text_entry` | Original markup/icons, draft validation and commit rules, menu navigation, command semantics and toast markup | State subscriptions and updates, overlay ordering, Escape dispatch, focus restoration, background inert state, theme synchronisation and cleanup |
| Native text sessions | SDK `TextEdit`, composition priority, deduplicated session endings, invalidation on external values, optional CSS transforms | World matrix, font styles, adaptive height, text transactions and exclusion of the edited layer from region drawing | Control placement, native draft/external-value boundaries, IME/Escape priority and resource cleanup |
| System clipboard and token files | SDK `clipboard` result callbacks and `files::export` downloads | Vellum payload validation, internal-copy fallback, layer-copy semantics and token JSON | Browser permission rejection, system API integration and download-resource lifecycle |
| File import/export | SDK `files::pick/import_files/export` for selection, read results and downloads | Size/format limits, document replacement transactions, image decoding/placement, embedded fonts, filenames and export content | File entry points, cancellation/rejection callbacks, Blob and download-URL lifecycle |
| Persistence and instance recovery | Mount cleanup; loader fatal notification, restart entry and three-restart limit | IndexedDB/localStorage, post-commit debounce, option saves, document recovery and unsaved state | Instance identity, unmounting old scopes, restart UI and resource cleanup |
| Frame presentation | SDK `Layer` modal stack and focus management, resize observation | Top-level frame filtering, Canvas previews, previous/next navigation, prototype hit testing and navigation | Modal hierarchy, background inert state, Escape dispatch, focus restoration and observer cleanup |

### Document and rules layer

The [document model](../examples/vellum/src/document.rs) handles format validation, IDs unique across pages, same-page parent/child relationships, cycle/depth limits and round trips for unknown fields. `serde_json` enables `float_roundtrip`, preserving fractional rotation, path/image coordinates and unknown values across repeated saves and restores. Three values with 1 ULP drift found by consecutive smoke checks have separate model regressions. The [starter document](../examples/vellum/src/starter.rs) reads the Forma golden fixture generated by the original implementation; the runtime does not load its JavaScript. [History](../examples/vellum/src/history.rs) retains up to 80 complete document snapshots. Image and font strings share `Rc<str>`, so undo after document replacement can restore assets without copying binary content at every step.

[Scene composition](../examples/vellum/src/scene.rs) computes world/inverse matrices, bounding boxes, inherited opacity and clipping chains. [Editing commands](../examples/vellum/src/commands.rs), hit testing and GPU projection share these results. [Text layout](../examples/vellum/src/text_layout.rs) accepts measurement through `Measure`: host tests use fixed character widths and browsers use Canvas 2D. [SVG export](../examples/vellum/src/svg_export.rs) reuses the same wrapping results.

These modules describe file and editing rules independently of windows, DOM and GPU contexts. The framework supports projection, lifecycle, controlled input and overlays while the application owns the business model.

Validation entry points:

```sh
mbx test -p vellum
mbx run -q -p vellum -- --starter > /tmp/forma-rust.vellum
node tests/vellum/interop.mjs --check-starter /tmp/forma-rust.vellum
```

The interoperability tool uses local reference source, checks each existing field of the original golden fixture and invokes the original parser. Pure Rust tests remain available without `ref/Vellum-main`.

### Region lifecycle and scene projection

The [application](../examples/vellum/src/app.rs) stores the document, camera and selection in Leptos signals. A projection effect composes the scene and prepares browser rasters, then passes `SceneProps` to the single [GPU region](../examples/vellum/src/region.rs). The region returns draw statistics and owns no editing state. `Pace::Continuous("stats")` lets the framework coalesce statistics notifications of the same kind.

The [SDK GpuRegion](../crates/rustify-ui/src/region.rs) handles initialisation retries, rebuilding after `Lost`, reapplying current props, restoring resize observation and unmount cleanup. The application retains its document through GPU initialisation failure or WebGL context loss. A Wasm trap still terminates the whole instance and loses unsaved edits. SDK projection scheduling presents in the same frame after processing props; the application needs no additional JavaScript loop for region lifecycle. `?nogpu` provides a reproducible GPU-unavailable state while the page and document commands continue to work.

The application owns its [shaders](../examples/vellum/src/shaders.rs) and texture-retention policy: shapes use analytic distance fields, while text, paths and images use browser rasters. Camera translation is cancelled in CPU `f64` calculations before uploading `f32`. Textures retain only the keys needed by the current projection. Application statistics count actual draw items; framework accounting tracks runtime frames and GPU bytes. These separate measures support batching assertions and resource checks after zooming.

### Browser rasters and fonts

The [painter](../examples/vellum/src/painter.rs) measures and draws text through Canvas 2D, using browser `Intl.Segmenter` and locale-aware case conversion to preserve the original graphemes, wrapping and font fallback. Paths and lines use `Path2D`; the browser decodes images and draws them with cover scaling. Screen and export painters share these rules. SDK Makepad text shaping does not replace this measurement, because a different measurer would change wrapping and glyphs.

The [raster cache](../examples/vellum/src/raster.rs) caches RGBA by node version and quantised resolution within 64 MiB. CPU cache eviction and GPU retention for the current frame have separate resource boundaries. The region uploads premultiplied pixels through `ImageBuffer`; replacing the raster content object updates the existing texture even when its key stays the same. Undo, document restoration and font changes clear old rasters so that reused version numbers on history branches cannot display stale text.

The [font cache](../examples/vellum/src/fonts.rs) belongs to the mount scope. It decodes local-font data URLs into bytes before passing them to `FontFace`, avoiding data/blob exceptions to the strict `font-src 'self'` CSP. The document retains the original data URL so `.vellum` files can carry fonts. Unmount removes fonts registered by that scope. Changing font selection updates text height through the original property command; restoring a saved font does not rewrite the document's height.

Cold zoom initially rasterised 93–102 text items at once, with measured CPU raster time of 31.7–36.6 ms. The planned staged approach now stops adding deferrable nodes after 8 ms in each batch, retaining their previous resolution until the next batch scheduled after actual GPU presentation. The 8 ms budget decides whether to begin another item; it cannot interrupt a single Canvas operation. Measured batches took 2.8–9.5 ms and the first cold-zoom frame took 15–21.6 ms. Refinement completed after 2, 4 and 6 presentations respectively. New nodes without an older raster still render synchronously. Page `performance.now()` and SDK presentation-frame counts record all timings.

The seven checks in `m3-raster.spec.ts` cover original Unicode wrapping in the same browser, direction/letter-spacing/underline pixels, undo branches, local fonts without CSP violations, image cover and replacement under the same key, LRU eviction and resolution boundaries. Text regions had zero channel differences at threshold 24. Six large rasters caused 104 cumulative evictions; the cache retained 58,867,200 bytes, below 64 MiB.

A later regression found 4 differing pixels at decoration endpoints. Browser diagnosis showed that the reused Canvas changed rendering backend on its 3rd readback, moving endpoint alpha from 127 to 98/105. GPU and software antialiasing cover edges differently. Forcing GPU removed these 4 pixels but increased cold-zoom refinement from about 0.1 seconds to about 3 seconds, so that option was rejected. Explicit `willReadFrequently=true` selects the software path suited to RGBA readback. Text layout remains exactly equal; text crops use the user-approved ≤2% visual gate and report actual differences. Here 4/120,000 is only 0.003333%. Evidence is in `test-results/vellum/m7-raster-probe.log` and `m7-raster-fixed.log`; the latter records the rejected GPU approach. The final seven checks in `m7-canvas-raster.log` passed. Refinement at cold zoom 0.49 returned to 101.1 ms/4 frames, with the text crop still differing by 4 pixels.

### Pointers, overlay canvas and sizing

The [pointer state machine](../examples/vellum/src/pointer.rs) updates Leptos signals from DOM events and reuses pure Rust hit-testing, snapping and geometry rules. The [overlay canvas](../examples/vellum/src/overlay.rs) uses the same scene matrix and camera to draw selection outlines, nine handles, dimensions, marquee selection, guides, pen graphics and rulers. The application owns editing state; the GPU region receives projections. Canvas 2D preserves the original screen-pixel line widths, browser text metrics and pointer-hit coordinates.

SDK `observe_resize` synchronises canvas dimensions. DOM overlays and GPU projection share the viewport and DPR. The framework handles resize observation and region lifecycle; the application handles gestures, transactions and overlay graphics. Pointer-down starts a history transaction and pointer-up commits it. Escape, pointer cancellation and focus loss during an active gesture restore the original document. [Keyboard bindings](../examples/vellum/src/keys.rs) preserve native input behavior and handle canvas tools, undo/redo, arrow-key nudging, zoom and spacebar panning.

The application explicitly registers 9 pointer/wheel/focus-loss listeners and 2 keyboard listeners. `Bindings` removes them and releases closures during Leptos mount-scope cleanup. Bare wasm needs the same editing handlers plus state subscriptions, view refresh, size observation and unmount ordering. The 13 checks in `m4-pointer.spec.ts` passed: original rectangle creation/dragging/undo/redo/zoom/rotation; equal geometry and selection for six gesture groups; zero differences in the independent overlay canvas at an RGB channel-sum threshold >24; Escape/cancellation/focus-loss restoration, pen-anchor editing and actual CDP two-pointer input. Hover, live marquee selection, snapping guides, pen preview and rulers have additional pixel assertions.

Across 30 consecutive pans of the starter page, input-to-SDK-presentation latency had p95 44.5 ms and maximum 55.6 ms. It did not trigger ADR-2's p95>50 ms reassessment, so the camera remains in application signals. Timing runs in the page, from pointer-event dispatch until `requestAnimationFrame` observes an increased SDK frame count; it does not measure physical display latency. Data is in `test-results/vellum/m4-pan.json`, with interactions and overlay images in `test-results/vellum/artifacts/m4-pointer/`. This budget now lives in `m4-pan.spec.ts`, which no longer starts SwiftShader and skips automatically in headless mode. Run it manually with `--headed` on a GPU-equipped machine: both builds before and after the change took 130–140 ms under SwiftShader.

### Workspace shell and overlays

The [shell](../examples/vellum/src/shell/mod.rs) stores menu, dialog, panel and search state in Leptos signals. The top bar, pages, inspector, assets and toolbar use `view!`. Switching pages preserves each page's camera and selection. Only the [layer tree](../examples/vellum/src/shell/layer_tree.rs) retains escaped strings and delegated events to limit listener counts in large documents. HTML characters in names cannot become markup. Indentation uses CSSOM without relaxing strict CSP.

A Memo deduplicates tree markup. Selection updates only existing row classes and ARIA, preserving native double-click and drag behavior. The 10 checks in `m5-shell.spec.ts` passed, covering search, visibility/locking, safe renaming, native drag reparenting and undo, page camera/selection memory, all menu types, field transactions and modal operations. In the tested macOS Chromium, holding Shift before dragging suppresses native drag startup, as in the original. The test first drags 8 pixels to trigger `dragstart`, then presses Shift, verifies the modifier at `drop`, and checks that all six world-matrix components remain unchanged.

[Fields](../examples/vellum/src/shell/fields.rs) maintain local drafts while focused. Valid numbers update the canvas immediately and blur commits one history step. Intermediate minus signs, exponents and empty values survive reactive refreshes. Hex colors support three and six digits; invalid values restore the document color and show a message. The inspector derives from selected nodes. Normal property changes preserve input DOM and focus without suspending the whole panel.

[Menus](../examples/vellum/src/shell/menus.rs), [dialogs](../examples/vellum/src/shell/dialogs.rs) and the [command palette](../examples/vellum/src/shell/palette.rs) use SDK `Layer`. The framework handles Escape ordering, initial focus, focus restoration and modal background inert state; modal layers also cycle Tab/Shift+Tab between their first and last controls. The application handles menu arrow keys, Home/End and filtering/executing 33 original commands. SDK `provide_toasts` returns a `ToastHandle` that owns queueing, duration and replacement. Vellum retains its own markup in `#toast`, which stays mounted but hides between messages. A Portal in the overlay root keeps it outside the inert background without entering the Escape stack. SDK `Draft` stores inspector number/color text during editing; input previews values while the application decides commit or Escape behavior.

`ThemedScope` propagates light/dark themes throughout the scope; the shell retains the original styling. Modals in the SDK overlay root still inherit the theme. `m5-shell.spec.ts` checks accessible names for inputs, theme and menus, with assertions for focus restoration, Tab/Shift+Tab, background inert state and layer cleanup after closing. All browser cases check uncaught errors and CSP violations.

With the complete shell, pointer regressions passed 13/13 and scene regressions passed 6/6. This run measured panning presentation p95 at 34.8 ms and CPU property-edit p95 at 6.2 ms for 5,000 shapes; all 20 edits produced a presentation and the draw count remained 1. Data comes from `test-results/vellum/m5-pointer-regression.log` and `m5-scene-regression.log`, without a performance promise across machines.

### Native text sessions, history and clipboard

[Text sessions](../examples/vellum/src/shell/text_session.rs) use the native multiline textarea from [SDK TextEdit](../crates/rustify-dom/src/text.rs). The browser owns the caret, selection, composition and native undo inside text. The application retains the document, measurement and one history entry per session. Editing makes the GPU skip that text layer; ending the session restores canvas text. Escape commits under the original semantics, as do Ctrl/⌘Enter and blur. Reopening from the canvas with Enter first cancels that key's default action, preventing it from reaching the newly focused input and replacing selected text. Before another canvas gesture, editing command or page switch, the current session ends synchronously so two operations cannot merge into one undo entry.

The SDK added an optional `transform` property set through CSSOM; existing callers without it remain unchanged. The anchor supplies only the canvas viewport origin and untransformed text width/height. The application supplies the full world matrix multiplied by camera zoom and sets transform origin `0 0`, avoiding a second AABB offset during rotation or zoom. The application still sets font, direction, letter spacing, decoration, opacity and height after input.

When external text changes, the SDK reports session invalidation. The application discards its draft and pending history rather than writing an old snapshot over the external value. `History::discard_pending` has a regression that failed before the fix and passed afterward. The diagnostic entry `__vellum.replaceTextExternally` injects this change outside normal editing transactions. Each session adds only one input listener for native-control height, removed with the session.

[Clipboard integration](../examples/vellum/src/shell/clipboard.rs) calls SDK system read/write APIs. A copied message appears only after successful system writing; rejection explains that internal Vellum paste remains available. Paste first parses a system Vellum payload and supports plain text when no internal copy exists. Layer data passes document-parser validation before entering a transaction. The payload retains original node/root-ID/asset formats and adds an optional font field, leaving old payloads usable. Token JSON uses SDK file export and CSS copy uses SDK clipboard entry points.

The permitted `cargo test -p rustify-ui --lib text::` command succeeded, but this DOM module had no host unit tests (0 matched, 133 filtered); this is not behavior coverage. `m6-edit.spec.ts` verifies native controls in the browser. `cargo check -p property-workbench --target wasm32-unknown-unknown` passed, checking existing callers' source compatibility.

The 13 checks in `m6-edit.spec.ts` passed: Unicode sessions, rotation/camera positioning, system undo, synchronous transaction boundaries, text-tool/double-click/blur behavior, external-value invalidation, clipboard rejection and invalid payloads, cross-page component propagation/overrides, layout constraints, alignment/distribution, token download and twin edits. CDP composition confirmed that Enter/Escape do not end a session during composition. These browser events do not replace a manual system Pinyin IME check. The text-tool branch cancels the compatibility mousedown's default focus move so a newly mounted native control does not immediately blur.

After text and pen/anchor edits, both documents had identical geometry. Full-page differences were 779/1,600,000 (0.0486875%) and 1,202/1,600,000 (0.075125%), both below 2%. Evidence is `test-results/vellum/m6-edit-fixed.log`, `m6-edit-visual.json` and `artifacts/m6-edit-fixed/`. The 136 model tests, 5 shader tests, Clippy and formatting passed.

### Files, persistence and presentation

[File integration](../examples/vellum/src/fileio.rs) opens selectors with SDK `files::pick`, reads files with `files::import_files` and downloads bytes with `files::export`. The application enforces format validation and limits of 80 MiB for documents, 25 MiB for images and 15 MiB for fonts. Rejection and cancellation do not commit edits. The browser decodes images; images exceeding 64 million pixels are rejected. The longest edge on placement is at most 800 logical pixels, while assets retain their original resolution. Document import creates one undoable whole-document replacement; history snapshots restore its assets.

[Persistence](../examples/vellum/src/storage.rs) uses the original `vellum-editor/documents/current` key, falling back to `vellum-document` local storage if IndexedDB opening or reading fails. Document changes debounce for 500 ms. Pending drag, inspector or text transactions are not written as recovery copies. Writes are serial and track save generations; saved status appears only after that copy was successfully written. Failures show red status and an export hint. `vellum-options` and `vellum-welcomed` retain user options separately. Normal unmount cleans listeners, database connections and pending writes; generation counters invalidate old tasks. Loader instance restart runs recovery again, and unsaved edits do not survive between instances.

Fault injection found that bare browser timers could keep saving after instance fatal failure. Save and toast delays now use SDK `rustify_ui::defer_after`. Page input and database events use `rustify_ui::listen` with the instance AbortSignal; animation frames use `rustify_ui::next_frame`, preserving rAF timing while browser cancellation follows signal termination. `rustify_ui::release_on_abort` registers bound native JS methods for transactions, database connections, open requests and local fonts. On termination the browser aborts transactions, closes connections (including those opened afterward) and removes local fonts without calling dead Wasm. `app.js` no longer has resource-close glue. The example does not cancel every already queued internal framework Promise microtask; application async completion checks `rustify_ui::instance_failed` so it cannot keep changing the document or storage after failure.

This work also fixed the existing closure-release gap in the [SDK file picker](../crates/rustify-dom/src/files.rs): selection or cancellation immediately removes listeners, releases closures and calls back once. Listeners bind to the runtime termination signal. Browser regressions first observed 2 listeners retained after cancellation; repeated cancellation and successful selection returned to 0 after the fix. Fonts follow document replacement and Undo/Redo, removing deleted families and invalidating older loads. The same-name font-switch width regression also failed before the fix and passed afterward. Native JS termination callbacks remove fonts on fatal failure; restart then reloads them from the saved document.

PNG and [presentation previews](../examples/vellum/src/shell/presentation.rs) share the Canvas painter and wait for image decoding. Export uses its own image cache so async rendering of an old document snapshot cannot change the current screen cache. Output is limited to 64 million pixels and 16,384 pixels per edge. Pure Rust still generates SVG with browser text measurement. The presentation layer uses SDK `Layer` for modal ordering and focus; the application filters top-level visible frames, cycles navigation and resolves prototype links. Async previews use generations and scope-liveness checks to prevent rapid page changes or closing from updating an older view.

All 22 checks in `tests/vellum/m7-files.spec.ts` passed, including actual file selection/cancellation, drag-and-drop, rejection paths, aborted storage transactions, three fatal restarts, failed-instance font cleanup, 550×250 PNG pixels, SVG structure and prototype navigation. The log is `test-results/vellum/m7-complete.log`. Bidirectional opening retained three pages and 202 layers; `interop.mjs` also parsed browser-exported files successfully (`m7-complete-interop.log`). The permitted SDK command `cargo test -p rustify-ui --lib files::` passed 7 tests with 126 filtered. Browser checks cover the picker lifecycle.

## Reproducible figures

The final release has the following six size categories from `size_report` in `test-results/vellum/m8-build-manifest.json`. Values are raw bytes, not compressed transfer sizes.

| Category | Bytes |
| --- | ---: |
| Wasm | 20,221,645 |
| JavaScript, including framework and generated glue | 244,922 |
| CSS | 32,652 |
| Fonts | 51,187,844 |
| Images | 20,701 |
| Data and licenses | 8,050 |
| Total | 71,715,814 |

The build pipeline copies fonts with Makepad resources. Total output size is not the actual first-viewport download. Two consecutive `cargo xtask build-web --example vellum --release` builds produced identical SHA-256 hashes for all 43 files, including the manifest. Evidence is `test-results/vellum/m8-build-determinism.json` and `m8-build-compare.log`.

The application entry `app.js` contains 145 lines and 7,115 bytes, from `wc -l examples/vellum/app.js` and the manifest's `files["app.js"]`. It connects the loader and a small automation facade; native resource closing on fatal failure has moved into the SDK as described above. Editing rules, file format, persistence policy and Canvas painters are Rust. Other JavaScript comes from the framework or generated glue.

Application Rust explicitly registers 16 persistent DOM listeners through `rustify_ui::listen`: 9 for pointer/wheel/focus loss, 2 for keyboard, 2 for drag-and-drop, 1 for outside-menu clicks and 2 for persistence page lifecycle. Native text sessions temporarily add 1 input listener. IndexedDB opening/reading/writing add 3/2/3 temporary listeners respectively. `release_on_abort` and `next_frame` also register native JS releases on AbortSignal: 1 for a database connection, 1 per local font, 1 per pending rAF, 1 per open request (plus 1 native success listener on that request for late connections), and 1 per active transaction. Thus 16 is not the whole instance total and excludes Leptos declarative events, internal SDK listeners and ResizeObserver. The SDK file picker adds 2 temporary listeners, released on selection or cancellation.

Check registration sites with these commands, also expanding the two event loops in `pointer.rs`:

```sh
rg -n 'for name in|bindings\.listen' examples/vellum/src/pointer.rs
rg -n '\blisten\(' examples/vellum/src/{keys,fileio,storage}.rs examples/vellum/src/shell/{menus,text_session}.rs
rg -n '\brelease\(|close_when_opened\(|remove_on_failure\(|next_frame\(' examples/vellum/src
```

`m8-metrics.spec.ts` disables HTTP caching in three independent browser contexts and records ready, first SDK presentation and completion of raster refinement using page `performance.now()`. Data is in `test-results/vellum/m8-startup-1.json` through `m8-startup-3.json`. This does not establish that OS caches were cleared or measure physical display latency.

The three measured startups below used Chromium 153.0.8010.12 and SwiftShader. The first sample's longer ready time is retained; three samples do not support a performance guarantee across machines.

| Sample | ready (ms) | First presentation (ms) | First raster refinement complete (ms) |
| --- | ---: | ---: | ---: |
| 1 | 1913.2 | 337.6 | 337.7 |
| 2 | 314.0 | 256.2 | 256.4 |
| 3 | 313.3 | 256.8 | 257.0 |

The Forma golden fixture has three pages and 202 layers: 171, 31 and 0 respectively. `node tests/vellum/interop.mjs --generate` regenerates and validates it from local original source.

The browser rendering probe used Chromium, SwiftShader, 1600 × 1000 and DPR 1. For 5,000 shapes the actual draw count was 1, and all 20 consecutive property edits produced presentation frames. Repeating 20 zoom levels over three rounds produced identical GPU-byte sequences, peaking at 12,846,744 bytes. Current raster-texture retention changed with viewport and resolution without growing each round. Figures come from `test-results/vellum/m2-scene.log` and are not performance limits.

The final build remeasured CPU edit p95 at 6.3 ms for 5,000 shapes, with all 20 edits presented and draw count still 1. Across 30 pans, input-to-SDK-presentation p95 was 34.8 ms, below the 50 ms reassessment condition. Cross-resolution cold zoom first presented after 14.7–19.6 ms and finished refinement in 67.5–146.3 ms over 2–5 frames. The startup page had 171 layers, 173 instances, 144 draws and 1 region, with SDK GPU accounting at 1,450,576 bytes. These are local samples, not guarantees across machines. `test-results/vellum/m8-performance.json` summarises the final `m2-scene.spec.ts`, `m3-raster.spec.ts` and `m4-pointer.spec.ts` logs.

Four nested ancestor corner clips passed pixel assertions, supporting the floating-point texture clipping chain. Measured premultiplied blending was `[70,6,135]` versus theoretical `[70.25,6.5,134.75]`. The browser context's `premultipliedAlpha` was `true`. Neither probe needed the fallback implementation in the plan.

## Additional capabilities and application-owned work

The pure Rust document model supports host validation of file interoperability and editing semantics. The application owns browser rasters, overlay canvas, input semantics and persistence policy. The framework owns mount scopes, signal projection, region recovery, overlay ordering, native editing and browser-capability entry points. SDK changes in this phase added optional `TextEdit` transforms and `files::pick` cleanup without modifying the Makepad fork.

## Twin comparison results

The first light fit-all startup comparison differed at 30,109 / 1,600,000 pixels: **1.8818125%**, using an absolute RGB channel-difference sum >24. Performance/engine labels were aligned, the welcome message was hidden, and the original ran with `?canvas` in the same Chromium. This early difference included shell colors, layer-tree indentation and icons still missing at that point, plus shape/shadow raster differences. It is not the final acceptance result. After M3 the user confirmed a per-image ≤2% gate.

An additional headed Google Chrome 152.0.7977.84 run confirmed that the original used `WebGPU` with the `apple metal-3` adapter, and compared its Canvas output in the same browser. Light/dark differences across the three pages were 2.4268% / 1.0125%, 0.3712% / 0.7527%, and 0% / 0%. Data is in `test-results/vellum/m3-webgpu.json`. Document hashes, cameras, viewports and DPR were fixed. Visual inspection of the startup page found matching main content and placement, with GPU shadows tighter than blurred Canvas shadows. The original Canvas and GPU backends themselves differ, so all pixel differences cannot be attributed to the port.

The final six Rust/original-Canvas comparisons are below. Images use 1600×1000, DPR 1, and mark pixels whose RGB difference sum exceeds 24. Every image has a ≤2% assertion. All six passed in `test-results/vellum/m8-final-visual.log`.

| Page | Light difference | Dark difference |
| --- | ---: | ---: |
| Design exploration | 1.651% | 0.742375% |
| Design system | 0.0768125% | 0.311375% |
| Playground | 0.03875% | 0.0335% |

Complete data and image locations are in `test-results/vellum/visual.json` and `test-results/vellum/artifacts/m8-final-visual/`.

Final main-menu difference was 1.661625%, and the help modal differed by 0.6770625%. After text and pen edits, differences were 0.0335% and 0.075125%, also within the per-image ≤2% gate. Data is in `test-results/vellum/shell-visual.json` and `test-results/vellum/m6-edit-visual.json`, with logs `m8-final-visual.log` and `m8-final-m6-edit.log`. Earlier M3–M7 phase figures remain in the plan's completion records; the table above is the baseline for later reductions.

The final headed Chrome 152/Apple Metal run also compared six images with original WebGPU under identical documents and cameras. Full-page differences were 0.019125%–0.2719375%, and canvas-region differences were 0.0015922%–0.0073641%. The agent inspected image groups and confirmed content, position and font layout, with remaining shell differences in icons, text truncation and small spacing. WebGL context loss and recovery preserved the document and camera exactly: region count went 0→1, center color and drawing recovered. See the [final validation report](validation/vellum/m8.md). Formal 2% acceptance still uses the same-browser Canvas reference.
