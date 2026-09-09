# M8 report: baselines and the acceptance suite (in progress)

Date: 2026-09-09. Machine: macOS (Darwin 25.6.0), Apple Silicon, rustc 1.97.0-nightly (2026-05-19), Node 26.1.0, Playwright 1.63.0 (bundled Chromium, **SwiftShader** WebGL - a software rasteriser, so every GPU-side figure here is a software figure and not a device one). Fixture: property-workbench, one scope, one region, a thousand objects.

## Status

M8 is **in progress**. Its automated half has results; its manual half has none, and `cargo xtask verify --suite p1` prints that as a missing record rather than passing over it.

**None of these numbers are the PRD's B0 or B1 loads.** They are P1's own fixture, and the plan forbids letting a small load stand in for a large one. Nothing here is claimed against R29's budgets.

## What is measured

### V10: two clean builds of each example

`cargo xtask verify --suite p1` builds each example twice from scratch and compares the manifests. Both examples produce **identical** manifests - same build id, same bridge hash, same bytes per file. The wasm is reproducible from this source and toolchain, which means a build id identifies a deployment rather than merely labelling one.

### V11: what a start and an interaction cost

| Measurement | Result |
| --- | --- |
| Mount to first frame, 30 rounds | median **55 ms**, worst 57 ms, first sample 38 ms; no upward drift across the thirty (`thirty mounts, measured in the page`) |
| A controlled input round trip, 1,000 samples | median **< 0.1 ms**, p95 0.1 ms, worst 0.4 ms; all thousand taken, none refused (`a thousand inputs reach the application, and their cost is a figure`) |
| Build size, six categories | `cargo xtask report-size`; fusion-basic total 59.5 MB, property-workbench 60.0 MB, of which 51.2 MB is fonts |

Every figure is taken inside the page. A round trip through the test harness costs ten to thirty times what is being measured, so a number taken from outside would describe the harness.

### V12: what a long run leaves behind

| Measurement | Result |
| --- | --- |
| 100 mount rounds, 20 to warm up and 80 measured | linear memory **unchanged** across the eighty; regions back to 1, timers 0, tasks 0 (`a hundred mount rounds, twenty to warm up and eighty measured`) |
| 100 hide and restore rounds | memory unchanged; the region still answers a real pointer afterwards |
| Endurance, 10 discrete actions a second | **1,200 sent, 1,200 accepted, 0 refused** at 9.92/s over two minutes: nothing lost, nothing duplicated, nothing late |

The endurance memory curve is worth printing rather than summarising: `36896768, 36896768, 36896768, 36896768, 37552128, 38207488, 38862848, 39518208, 41680896, 41680896, 41680896, 41680896` - flat, a climb of 4.8 MB, then flat again for the last third. The test asserts that the last quarter is flat and prints the whole curve, because a two-minute run cannot tell a plateau from a slow climb. That is what the two-hour setting is for.

## What is not measured

- **The two-hour run has not been done.** The fixture takes it (`RUSTIFY_ENDURANCE_MINUTES=120`), and until it is run the memory curve above is a two-minute shape, not a two-hour one.
- **The diagnostics on/off comparison** has no switch to compare: the record is always on and its cost has not been isolated.
- **Network conditions and transfer size.** The figures are from a loopback server with no throttling; nothing here says what this costs over a real connection.
- **A device GPU.** SwiftShader is a software rasteriser. Every GPU figure is a software figure.
- **The manual matrix**: pinyin, VoiceOver, contrast and reflow, and the Safari observation. `verify --suite p1` lists the four files it looks for and reports them as not handed in.

## The acceptance suite

`cargo xtask verify --suite p1` runs the sources record, the doctor, fmt, clippy, the three test suites, two clean builds of each example and the three browser projects, then lists the manual records and says which are missing. It exits non-zero only when an automated step fails; a missing manual record is printed and the run says plainly that P1 cannot be called complete without it.
