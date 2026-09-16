# Vellum

A Rustify UI port of the local Vellum design editor. Leptos owns the document and editor state; one `GpuRegion` renders the scene through Makepad WebGL2.

```sh
cargo xtask build-web --example vellum --release
cargo xtask serve --example vellum --release --port 4179
```

Open `http://127.0.0.1:4179/`. Add `?nogpu` to exercise the GPU unavailable state. The DOM shell, document model, page switching and automation remain available.

The implementation is tracked in [VELLUM.md](../../docs/plan/VELLUM.md); [framework integration and validation](../../docs/vellum.md) explains what the SDK provides and what the application owns. The shell retains Vellum's markup and scoped stylesheet, with controlled inspector fields, page state, a searchable layer tree, menus, dialogs and a command palette. Pointer editing, native text sessions, component edits, snapping, pen editing, rulers and shortcuts are implemented. Local persistence, portable files, image/font import, PNG/SVG export and frame presentation are connected through Rust and SDK browser capabilities. `window.vellum` exposes the editor automation interface and `window.__vellum` exposes framework lifecycle diagnostics.

The upstream copyright and license are preserved in `LICENSE-VELLUM`.

## Rendering checks

The float texture clip table passes a browser pixel test with four nested rounded frame clips. The shader traverses the complete ancestor chain; no two-clip fallback is used.

The renderer uses premultiplied output and `ONE, ONE_MINUS_SRC_ALPHA` blending. Canvas pixels are premultiplied before upload, and the region canvas opts into a premultiplied browser context before initialization. Two half-transparent red/blue layers over `[26,26,29]` produce `[70,6,135]`, within one channel unit of the source-over result.

`tests/vellum/m2-scene.spec.ts` also checks one GPU region, 5,000 editable shapes in one draw call, bounded texture retention across twenty zoom levels and the no-GPU state. Use only the independent configuration:

```sh
npx playwright test -c tests/vellum/playwright.config.ts m2-scene.spec.ts
```

Text, paths and images use browser Canvas 2D rasterization. Cold zoom changes refine existing rasters in 8 ms batches after each GPU presentation, retaining the prior resolution while work remains. Local fonts are loaded from decoded bytes so the existing strict CSP remains sufficient. The raster LRU has a 64 MiB budget; undo and asset/font changes invalidate both CPU pixels and matching GPU contents.

```sh
VELLUM_ARTIFACT_SCOPE=m3-raster npx playwright test -c tests/vellum/playwright.config.ts m3-raster.spec.ts
VELLUM_ARTIFACT_SCOPE=visual npx playwright test -c tests/vellum/playwright.config.ts visual.spec.ts
```

The six starter-page comparisons enforce the user-approved maximum of 2% differing pixels per image (1600 × 1000; RGB channel difference sum greater than 24).

The optional original-renderer comparison needs local `ref/Vellum-main`, installed Google Chrome and a display. It verifies that the original actually selected WebGPU and records six screenshots against its Canvas fallback:

```sh
VELLUM_WEBGPU_PROBE=1 VELLUM_ARTIFACT_SCOPE=m3-webgpu npx playwright test -c tests/vellum/playwright.config.ts m3-webgpu.spec.ts
```

Pointer and overlay parity is checked against the original using real keyboard/mouse input and CDP touch events:

```sh
VELLUM_ARTIFACT_SCOPE=m4-pointer npx playwright test -c tests/vellum/playwright.config.ts m4-pointer.spec.ts
```

The thirteen checks include six matching geometry scripts, pixel-identical interaction overlays at RGB threshold 24, cancellation/blur recovery and a page-measured pan presentation p95 of 44.5 ms.

The shell uses SDK `Layer` for Escape ordering, initial focus, focus return and modal background inertness. Application code supplies menu navigation and the modal Tab loop; fields retain focused drafts and group live edits into one undo step.

```sh
VELLUM_ARTIFACT_SCOPE=m5-shell npx playwright test -c tests/vellum/playwright.config.ts m5-shell.spec.ts
```

With the complete shell, starter-page differences are 0.0335%–1.651%. Open main-menu and help-dialog comparisons are 1.661625% and 0.6770625%, respectively, under the same 2% gate.

Native text uses SDK `TextEdit`, including rotation and camera transforms. Browser text undo and composition stay in the control; document edits commit the session before starting another transaction. The SDK clipboard reports permission failures and the editor retains an internal layer copy. Token JSON downloads use SDK file export.

```sh
VELLUM_ARTIFACT_SCOPE=m6-edit npx playwright test -c tests/vellum/playwright.config.ts m6-edit.spec.ts
```

Thirteen checks cover editing, components, layouts, native text, clipboard refusal and token downloads. Text and pen/anchor edits match the original geometry; full-page screenshot differences are 0.0486875% and 0.075125%. CDP composition checks do not replace a manual operating-system IME check.

Documents save to IndexedDB 500 ms after the latest committed change, with localStorage fallback when IndexedDB cannot open or read. An unfinished drag or text transaction is not a recovery snapshot. Storage errors retain the in-memory document and offer portable export. Restarting a failed runtime reloads the last saved copy; the loader allows three restarts.

File entry points use SDK `files`, including cancellation and rejected inputs. Document imports are limited to 80 MiB, images to 25 MiB and 64 million pixels, and fonts to 15 MiB. PNG exports share the scene painter and are bounded by 64 million pixels and 16,384 pixels per side. Frame presentation uses SDK `Layer`, arrow-key navigation and prototype links.

```sh
VELLUM_ARTIFACT_SCOPE=m7-files npx playwright test -c tests/vellum/playwright.config.ts m7-files.spec.ts
```

## Final verification

The original 36 checks run in order in one editor session, including light/dark captures:

```sh
VELLUM_ARTIFACT_SCOPE=smoke npx playwright test -c tests/vellum/playwright.config.ts smoke.spec.ts
```

Run other specs individually with the same independent configuration. CI builds the example and runs the suite; comparisons requiring `ref/Vellum-main` skip when that optional source is absent. `visual.spec.ts` always checks all six starter views and two shell views at the fixed 2% gate.

The optional installed-Chrome walkthrough covers native UI actions, context-loss recovery and six matched views against the original WebGPU renderer:

```sh
VELLUM_HEADED_PROBE=1 VELLUM_ARTIFACT_SCOPE=headed npx playwright test -c tests/vellum/playwright.config.ts m8-ui.spec.ts
```

See the [validation report](../../docs/validation/vellum/m8.md) for measured results and remaining manual checks. OS pinyin and physical trackpad input are explicitly untested; headed automation and CDP events do not establish those results.
