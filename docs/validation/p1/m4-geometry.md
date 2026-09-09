# M4 report: geometry, layers and focus

Date: 2026-09-09. Machine: macOS (Darwin 25.6.0), Apple Silicon, rustc 1.97.0-nightly (2026-05-19, toolchain nightly-2026-05-20), Node 26.1.0, Playwright 1.63.0 (bundled Chromium, SwiftShader WebGL). Builds: `cargo xtask build-web --example {fusion-basic,property-workbench} --release`. Tests: `npx playwright test --project=fusion-basic` (`m1-probes`, `m2-runtime`, `m3-state`, `m4-geometry`, `m4-overlay`) and `--project=property-workbench` as regression.

## Status

M4 is **closed**. Every exit condition on the M4 row of the plan holds.

## The fixture

`fusion-basic` gained a geometry fixture, mounted on demand by the page so the M1-M3 probes see the scopes they were written for. It is a region of twenty anchors inside two nested scrollers, the inner one narrower and shorter than the region, plus twenty focus items, a menu opened from a GPU anchor and a modal dialog.

The region reports where it drew each anchor and which one a click landed on (`examples/fusion-basic/src/anchor_grid.rs`). Display and hit are then both checked against the same claim rather than against a second copy of the layout kept in the test.

## Exit conditions

| Condition | Evidence |
| --- | --- |
| V4 / R09 AC1: three viewports x three zooms, twenty anchors, display and hit within 1 CSS px | `twenty anchors agree with the picture and with the pointer`, run at 1024x768, 1440x900 and 1920x1080, each at 100%, 125% and 200%. Per anchor: the marker's centroid in a screenshot is within 1 CSS px of where the region says the anchor is, and a real click at that point is reported back as that anchor with local coordinates within 1 CSS px |
| V4 / R09 AC2: a hundred rounds of resizing and scrolling two containers, nothing outside the clip | `a hundred rounds of resizing and scrolling keep the region inside its clip` - the region is resized and both scrollers moved each round; a strip just past the clip edge holds no region pixels, and every click on the region past that edge (more than twenty of them) delivers no action |
| V4 / R09 AC3: back from 0x0 within 500 ms, at most 2 misaligned frames | `suspends, and comes back at the size it returns to` - hidden, resized while hidden, then shown; measured inside the page with `requestAnimationFrame`, the first frame drawn is already at the new size |
| DPR follows a zoom | `the backing store matches the device pixel ratio after zooming` at 125%, 200%, 100% and 300% |
| V4 / R10 AC1: a DOM menu opened from a GPU anchor, within 1 CSS px, not occluded by the region | `checkAnchoredMenu` inside each of the nine matrix runs, plus `a menu opened from a GPU anchor sits on it, and stays on it while the page scrolls`, which also scrolls both containers under it and checks `elementFromPoint` lands on the menu |
| V4 / R10 AC2: a hundred clicks on a modal over a GPU control, zero actions underneath | `a modal over the region takes a hundred clicks and passes none of them down` |
| V4 / R10 AC3: two layers, Escape closes the top only, focus returns | `escape closes one layer at a time and hands focus back to what opened it`; `a layer whose trigger is gone hands focus to the scope instead` |
| V4: twenty Tab / Shift+Tab items | `tab and shift-tab walk them in order, skipping the disabled one` - nineteen stops forward and backward, the disabled item in neither, the read-only item reachable and unchanged by typing |
| No host focus stolen; no hidden Makepad text box | `mounting a scope and starting its region take no focus from the host page` - the host link keeps focus while a scope is disposed and mounted again, and the document holds no `textarea` |
| Command ownership | `an open layer owns escape, and the application gets it back afterwards` - the application's own Escape command runs when the stack is empty and neither time a layer is closed |
| Focus is not taken from where the user put it | `a layer that no longer holds focus does not take it back` |

## What M4 added

- **A resolution watch.** A change of device pixel ratio leaves the canvas box the same size, so the size observer never fires and the backing store keeps the old resolution. Embedded regions watch a media query on the current resolution and re-arm it at the new ratio (`makepad/platform/src/os/web/web.js`).
- **`RegionState::Suspended`.** A canvas laid out to nothing, or hidden, is not a failure and not a disposal. The host reports the change (`crates/rustify-makepad/web/embedded.js`), the region stops pumping until it has an area again, and whatever was queued for it runs on the pump that follows.
- **A scope's layer stack** (`crates/rustify-ui/src/overlay.rs`). `mount` now gives a scope two roots: its content, and a layer plane above it. A `Layer` component portals into that plane, is placed against a rectangle inside a GPU region or against an element, and follows that anchor while anything between them scrolls. A modal layer makes the scope's content `inert`, so a control underneath cannot be clicked, focused or read out. One keydown listener per scope, in the capture phase, closes exactly one layer per Escape and stops the key before the application's own commands see it.
- **Focus that goes back where it came from.** A layer records what had focus when it opened and returns it on close, or focuses the scope itself when that element has gone.

## Counts

`npx playwright test --project=fusion-basic` 42 (m1-probes 8, m2-runtime 11, m3-state 2, m4-geometry 12, m4-overlay 9); `--project=property-workbench` 19 as regression; `cargo test --workspace --lib` 17; `cargo test -p xtask` 10; `cd makepad && cargo test` 13; clippy clean; `cargo xtask doctor` 8/8.

## Known limits recorded rather than fixed

- A layer is placed under its anchor and is not flipped or clamped: an anchor near the bottom of the viewport puts its layer there too. Flipping is not in R09/R10 and is not implemented.
- A modal makes its own scope inert; it does not trap focus against the browser's own UI or against a host page outside the scope. That is what "disables this scope's underlying events" means here.
- The zoom matrix is exercised as a device scale factor with a CSS viewport scaled to match, which is what browser zoom does to a page. Chrome's emulation does not re-evaluate a resolution media query when only the scale factor changes, so the live-zoom test changes both, as a zoom does.
