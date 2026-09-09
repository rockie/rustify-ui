# M3 report: one authoritative state behind a DOM panel and a GPU view

Date: 2026-09-09. Machine: macOS (Darwin 25.6.0), Apple Silicon, rustc 1.97.0-nightly (2026-05-19, toolchain nightly-2026-05-20), Node 26.1.0, Playwright 1.63.0 (bundled Chromium, SwiftShader WebGL), Chrome 152.0.7977.83 installed. Builds: `cargo xtask build-web --example fusion-basic --release`, `cargo xtask build-web --example property-workbench --release`. Tests: `npx playwright test --project=property-workbench` (`tests/browser/m3-workbench.spec.ts`), `npx playwright test --project=fusion-basic` (`m1-probes`, `m2-runtime`, `m3-state`) as regression.

## Status

M3 is **closed**. Every exit condition on the M3 row of the plan holds.

## Exit conditions

| Condition | Evidence |
| --- | --- |
| V2: 1,000 objects with stable ids | `a thousand objects with stable ids, the first one selected` - 1,000 fixed ids, the first selected, and the grid on screen with no interaction at all (the first projection has to survive the pump that creates the region) |
| V2: 10,000 sequenced actions | `ten thousand GPU actions arrive once each, in order` - 10,000 pointer round trips dispatched inside the page; the application counted exactly 10,000 accepted, 0 refused, and the selection ring of 1,000 objects landed back where it started |
| V2: 100 objects updated in one batch | `a hundred objects change in one update and the GPU shows the result` - one controlled model update, one new projection, the first four colours each one step along the palette, and the region redrawn |
| V2: 120 Hz pointer stream with 20 clicks and saves | `a pointer stream loses none of the clicks and saves interleaved with it` - 240 pointer moves paced at 120 Hz by a real timer inside the page, 20 GPU button clicks and 20 DOM renames interleaved with them; exactly 20 discrete actions accepted, 0 refused, the selection at object 21 under the twentieth name, and object 20 still carrying the nineteenth |
| V2: reverse 100, add and remove 10, duplicate id | `reordering keeps the selection on the object, not on the position`; `adding and removing objects keeps every other id where it was`; `a list that names one object twice is refused, not guessed at` |
| Round trips in both directions | DOM to GPU: `renaming and recolouring in the DOM changes what the GPU draws`. GPU to DOM: `the GPU view moves the selection the application owns`, `clicking a cell in the GPU grid selects that object in the DOM` |
| One authoritative business state | Every path - DOM buttons, GPU buttons, grid picks, pointer moves - goes through the application's own `step`/`selected`/`hovered` signals; the region holds no object list and draws only what is projected into it. `the object under the pointer is the object a click there selects` checks the two derived values against each other |
| Re-entrancy and batch error regressions | `closing from inside a region action leaves the runtime alive`; `a scope disposed while its region is still starting leaves nothing behind`; `the batch a closing region produced is drawn before its Cx goes`; `a fatal drops the work the runtime had already scheduled`; `a trapped runtime takes its controls off the page` |

## What closed M3

The scheduler had `Pace::Continuous` and unit tests for it since the first M3 commit, but nothing produced one: every action a region emitted was admitted as discrete. This step gave it a producer and let the pace travel with the action.

- **A region declares the pace of its own actions.** `RegionApp::pace` (`crates/rustify-makepad/src/wasm/app.rs`) defaults to `Pace::Discrete`; `ObjectRegion` returns `Pace::Continuous` for `SelectionAction::Hover`. `Pace` moved to `crates/rustify-makepad/src/pace.rs` so it is part of the region-to-scope contract rather than a detail of the queue, and `submit_all` now asks for it per action instead of assuming.
- **A pointer stream that is business state.** `ObjectGrid` reports the cell under the pointer on every move and `None` when it leaves; the application stores it, projects it back, and the grid draws a ring around it. The region does not draw its own pointer state - it draws what it was handed, like every other value.
- **A header that does not move under the user.** The region's name and position labels are fixed width. They were sized to their text, so renaming an object shifted the two buttons beside them - a control moving out from under a pointer that is about to press it. The browser tests address those buttons through one constant now (`NEXT_BUTTON` in `tests/browser/m3-workbench.spec.ts`).

## The host test for the stream

`a_stream_of_moves_neither_delays_nor_crowds_out_the_clicks_between_them` (`crates/rustify-ui/src/binding.rs`) submits 4,000 moves and 20 clicks against a queue that holds 1,024, draining between bursts the way the browser gets its turn between them. Not one click is refused, each turn delivers the pointer state it ended on followed by the click that came after it, and the collapsed move keeps its place in the order.

That collapse is not observable end to end today: each browser event pumps its region synchronously and produces at most one action, which the scope then delivers immediately, so the queue never has two moves in it at once. The end-to-end test therefore checks what V2 asks for - that a stream of moves loses, delays or reorders none of the discrete actions inside it - and the collapse itself is checked where it can be: in the queue.

## Measurements (baselines, no budget claimed)

| Measure | Value |
| --- | --- |
| property-workbench release build | wasm 7,875,119 B, JS 169,376 B, CSS 2,003 B, fonts 51,187,844 B, images 20,701 B, data 7,969 B |
| fusion-basic release build | wasm 7,732,615 B, JS 166,962 B, CSS 885 B, fonts 51,187,844 B, images 20,701 B, data 8,219 B |
| 10,000 GPU actions | 10,000 accepted, 0 refused, whole run 18.4 s under Playwright |
| 240-move stream at 120 Hz with 20 clicks and 20 saves | 20 accepted, 0 refused, whole test 15.0 s |

## Known limits recorded rather than fixed

- The pointer stream collapses in the queue but never has to in the browser, for the reason above. A region that emits several actions from one event - the drag session in M4 - is the first case where it will.
- Font memory is still per region (`IBMPlexSans-Text.ttf` loaded once per region); carried forward from M2 as a to-do, not a leak.
- The browser tests address the region's own buttons by a fraction of the canvas, which depends on the header layout. Fixed-width labels make that stable against the values, not against a redesign of the header.
