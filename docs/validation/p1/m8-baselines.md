# M8 report: baselines and the acceptance suite

Date: 2026-09-09 and 2026-09-10 (the two-hour runs cross midnight). Machine: macOS (Darwin 25.6.0), Apple Silicon, rustc 1.97.0-nightly (2026-05-19), Node 26.1.0, Playwright 1.63.0 (bundled Chromium, **SwiftShader** WebGL - a software rasteriser, so every GPU-side figure here is a software figure and not a device one). Fixture: property-workbench, one scope, one region, a thousand objects.

## Status

M8's automated half is complete. The two-hour endurance run found a real leak - 145 MB in two hours - which is fixed; the section below has the curve, the cause and the measurement after. Its manual half has none of its four records, and `cargo xtask verify --suite p1` prints that as a missing record rather than passing over it. M8 is therefore **not closed**, and neither are M5, M6 and M7, which have waited on the same two people-only checks since 2026-09-09.

**None of these numbers are the PRD's B0 or B1 loads.** They are P1's own fixture, and the plan forbids letting a small load stand in for a large one. Nothing here is claimed against R29's budgets (A-6).

## What is measured

### V10: two clean builds of each example

`cargo xtask verify --suite p1` builds each example twice from scratch and compares the manifests. Both examples produced **identical** manifests - same build id, same bridge hash, same bytes per file - when the suite was last run in full, before this milestone's own changes; the closing run repeats it on the build these figures come from. The wasm is reproducible from this source and toolchain, which means a build id identifies a deployment rather than merely labelling one.

### V11: what a start and an interaction cost

| Measurement | Result |
| --- | --- |
| Mount to first frame, 30 rounds | median **55 ms**, worst 57 ms, first sample 38 ms; no upward drift across the thirty (`thirty mounts, measured in the page`) |
| A controlled input round trip, 1,000 samples | median **< 0.1 ms**, p95 0.1 ms, worst 0.4 ms; all thousand taken, none refused (`a thousand inputs reach the application, and their cost is a figure`) |
| Build size, six categories | `cargo xtask report-size`: fusion-basic 59.5 MB, property-workbench **60,043,864 B** (wasm 8,596,632 · js 217,713 · css 8,937 · fonts 51,187,844 · images 20,701 · data 12,037) |
| The same, gzip -9 (`--compressed`) | **37,306,259 B** (wasm 2,246,721 · fonts 34,990,754 · the rest under 70 KB) |

Every figure is taken inside the page. A round trip through the test harness costs ten to thirty times what is being measured, so a number taken from outside would describe the harness.

### V11: what a deployment actually sends, and over what link

New in this milestone (`tests/browser/m8-network.spec.ts`), because "network conditions and transfer size" was the largest hole left in V9/V11.

| Measurement | Result |
| --- | --- |
| A first load with an empty cache | **9,009,274 bytes** over 14 requests: wasm 8,596,932 · js 220,413 · css 9,837 · fonts 182,092 (**one** file) |
| Ready, loopback, no throttling | 6.7 s |
| Ready, 12 Mbit/s, 40 ms latency | 6.6 s |
| Ready, 3 Mbit/s, 40 ms latency | 24.3 s |
| The first value that needs more than Latin | **29,718,416 bytes**: `LXGWWenKaiRegular.ttf` 19,074,264 and `NotoColorEmoji.ttf` 10,644,152 |
| Whole-suite result | the three network tests pass on this build; the whole `--project=property-workbench` suite is re-run by `verify --suite p1` after the two-hour endurance |

Two facts worth separating. The build **stores** 60 MB and a first load **fetches** 9 MB: ten font files ship and one is fetched, because until a value needs more, the Latin face is all a region draws with. And nine megabytes at 12 Mbit/s is about six seconds of transfer that the browser compiles through, so the total matches the loopback's; at a quarter of that speed the link is the whole story.

Getting those two figures right needed one fix in the harness: network emulation belongs to the DevTools session that set it, and a second session attached to the same page left it ambiguous which one the browser honoured - a throttled load came back at loopback speed once because of it. One session per test, reused across profiles.

The served policy sends no `Content-Encoding`, so the wire bytes are the file bytes plus headers; the gzip row above is what a compressing host would send instead.

### V12: the diagnostics switch

Also new. V12 asks for a comparison with the record on and off, and until this milestone there was no switch to compare. `Diagnostics::set_recording` turns writing off; what it turns away is counted as `suppressed` and printed in the report beside `dropped`, so a record taken while it was off says so rather than looking empty.

| Measurement | Result |
| --- | --- |
| 1,000 refused mounts, record on | one entry each, the ring holding at its 1,000 ceiling; per-entry cost printed by `the same refusals with the record on and with it off` |
| The same 1,000, record off | nothing kept, `suppressed` exactly 1,000, `count`/`bytes`/`dropped` unchanged, and never slower than with it on |
| 1,000 controlled inputs, on and off | no entry written either way: a record costs nothing on a path that has nothing to say |

### V12: what a long run leaves behind

| Measurement | Result |
| --- | --- |
| 100 mount rounds, 20 to warm up and 80 measured | linear memory **unchanged** across the eighty; regions back to 1, timers 0, tasks 0 |
| 100 hide and restore rounds | memory unchanged; the region still answers a real pointer afterwards |
| Endurance, 10 discrete actions a second, two minutes | **1,200 sent, 1,200 accepted, 0 refused** at 9.92/s: nothing lost, nothing duplicated, nothing late |

The two-minute memory curve is worth printing rather than summarising: `36896768, 36896768, 36896768, 36896768, 37552128, 38207488, 38862848, 39518208, 41680896, 41680896, 41680896, 41680896` - flat, a climb of 4.8 MB, then flat again for the last third. The test asserts that the last quarter is flat and prints the whole curve, because a two-minute run cannot tell a plateau from a slow climb.

### The two-hour run, and the leak it found

The first two-hour run **failed**, and the failure was the point of running it.
Linear memory grew from 36,896,768 to 189,136,896 bytes - 145 MB - and not
smoothly: in doublings.

| At | Jump | To |
| --- | --- | --- |
| 1.3 min | +2.4 MB | 40 MB |
| 3.5 min | +4.1 MB | 45 MB |
| 7.7 min | +8.1 MB | 53 MB |
| 16.0 min | +16.1 MB | 71 MB |
| 32.8 min | +32.1 MB | 105 MB |
| 66.5 min | +64.1 MB | 175 MB |

Each jump twice the last, at intervals that also doubled (13, 25, 50, 101
samples): one container growing by a fixed amount per action and reallocating
at double capacity. Nothing else about the run was wrong - 72,000 sent, 72,000
accepted, 0 refused, at 10.00/s, no errors, one live listener at the end.

**The container was the script VM's heap.** Every props application that sets a
value on a shader evaluates a small script through `script_apply_eval!`, and
each evaluation leaves three objects behind. Nothing ever collected them: a
desktop application runs its script at startup, so the collector was only
reachable from script itself, and an embedded region that applies props on
every action makes garbage for as long as it runs.

A ten-millisecond host test says the same thing as the two-hour browser run:
five hundred evaluations of one callsite add fifteen hundred live heap entries,
and one sweep takes all of them back
(`what_a_reused_eval_callsite_leaves_behind_is_collectable`, in the fork's
script VM). That the sweep reclaims them is what makes it garbage rather than
state, and it is what the fix depends on.

The fix is in the region's pump (`crates/rustify-makepad/src/wasm/host.rs`):
between pumps, when the VM is not held and nothing the pump produced is still
on its stack, a region collects if it has made more than 20,000 entries of
garbage since its last sweep. An amount rather than a period - an idle region
never collects, and a busy one pays for the sweep with what it reclaims.

Measured after the fix, fifteen minutes at ten actions a second: 9,000 sent,
9,000 accepted, 0 refused, memory **36,896,768 -> 43,515,904** and flat for the
last twenty-eight samples. The old build was at 71 MB and still doubling by
that point in its run.

<!-- endurance-two-hours-after-fix -->

## A defect this milestone found

Measuring the transfer answered a question nobody had asked directly: **what does a Chinese value cost?** The answer was that it cost nothing, because it was never drawn. A region drew CJK as `.notdef` boxes and no CJK font was ever fetched, which contradicted both the M5 report and `docs/compatibility.md`.

Cause: on the web the theme's default font families name one Latin face, because the Chinese and emoji faces are 30 MB together; the families that name all three are published separately as `font_regular_i18n` and its variants. Fix and evidence are in [the M5 report](m5-text-and-semantics.md); the cost of the first such value is the 29,718,416-byte row above.

This is the kind of defect the acceptance milestone exists to find, and it would have been the first thing the pinyin session hit.

## What is still not measured

- **A device GPU.** SwiftShader is a software rasteriser. Every GPU figure is a software figure.
- **Chrome rather than the bundled Chromium** for the numbers. The pass-gate browser was used to open the build, not to take these figures.
- **A real network.** The throttled runs are Chrome's own emulation in front of a loopback server.
- **The manual matrix**: pinyin, VoiceOver, contrast and reflow, and the Safari observation. `verify --suite p1` lists the four files it looks for and reports them as not handed in.

## The acceptance suite

`cargo xtask verify --suite p1` runs the sources record, the doctor, fmt, clippy, the three test suites, the presence of the six reports and the requirement matrix, two clean builds of each example and the three browser projects, then lists the manual records and says which are missing. It exits non-zero only when an automated step fails; a missing manual record is printed and the run says plainly that P1 cannot be called complete without it.

## The six reports

Delivered under [`docs/reports/p1/`](../../reports/p1/): functional, performance, compatibility, accessibility, fault and recovery, known limitations, plus the R01-R40 requirement matrix. `verify --suite p1` fails if one of them is missing, because unlike the manual records they are the milestone's own output.
