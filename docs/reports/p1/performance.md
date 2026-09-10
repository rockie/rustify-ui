# P1 performance report

Baselines. **No budget is claimed.** The PRD's R29/R30 numbers were never
approved (A-6), and the fixture here is P1's own — one scope, one region, a
thousand objects — not the PRD's B0 or B1 load. A small load is not allowed to
stand in for a large one, so nothing below is compared to a target.

Machine: macOS (Darwin 25.6.0), Apple Silicon; rustc 1.97.0-nightly
(2026-05-19); Node 26.1.0; Playwright 1.63.0 with its bundled Chromium on
**SwiftShader**, a software rasteriser. Every GPU figure is therefore a
software figure. Builds: `cargo xtask build-web --release`.

Every timing is taken **inside the page**. A round trip through the test
harness costs ten to thirty times what is being measured, so a figure taken
from outside would describe the harness.

## Start-up

| Measurement | Result |
| --- | --- |
| Mount to first frame, 30 rounds | median **55 ms**, worst 57 ms, first sample 38 ms, no upward drift |
| Navigation to `ready`, loopback, empty cache | **6.7 s** |
| Navigation to `ready`, 12 Mbit/s, 40 ms latency | **6.6 s** |
| Navigation to `ready`, 3 Mbit/s, 40 ms latency | **24.3 s** |

Nine megabytes at 12 Mbit/s is about six seconds of transfer, and the browser
compiles the wasm while it streams: the total matches the loopback's, so at
that speed nothing is gained by making the link faster. At a quarter of it the
link is the whole story — 24 seconds, and the extra eighteen are the download.

## What a deployment sends

| Measurement | Result |
| --- | --- |
| Built directory, six categories | wasm 8,596,632 · js 217,713 · css 8,937 · fonts 51,187,844 · images 20,701 · data 12,037 · **total 60,043,864 bytes** |
| The same, gzip -9 (`report-size --compressed`) | wasm 2,246,721 · js 50,586 · css 2,982 · fonts 34,990,754 · images 10,110 · data 5,106 · **total 37,306,259 bytes** |
| **A first load with an empty cache** | **9,009,274 bytes** over 14 requests: wasm 8,596,932 · js 220,413 · css 9,837 · fonts 182,092 (one file) |

The gap between the directory and the load is the point: a deployment stores
60 MB and a first load fetches 9 MB. Ten font files ship; a first load fetches
one of them, because the Latin UI is all the region draws until a value needs
more. The measured server sends no `Content-Encoding`, so the wire bytes are
the file bytes plus headers; the gzip row is what a compressing host would
send instead.

The first value that needs more than Latin costs **29,718,416 bytes** — the
Chinese face and the emoji face together, three times everything else a first
load fetches. That is a deployment fact, not a defect, and it is why a region
draws with a Latin-only family until a value says otherwise.

## Interaction

| Measurement | Result |
| --- | --- |
| A controlled input round trip, 1,000 samples | median **< 0.1 ms**, p95 0.1 ms, worst 0.4 ms; all thousand accepted, none refused |

What is timed is the whole controlled round trip — a keystroke reaching the
application and the control coming back in step with it — not a frame being
presented.

## What the record costs

The diagnostic record can be switched off since M8, which is what makes its own
cost measurable rather than assumed.

| Measurement | Result |
| --- | --- |
| 1,000 refused mounts with the record on | see `diagnostics on:` in the M8 run log — one entry written per refusal, the ring holding its 1,000-entry ceiling |
| The same 1,000 with the record off | nothing kept, 1,000 counted as suppressed, and never slower than with it on |
| 1,000 controlled inputs, record on and off | the happy path writes no entry either way, so having a record costs nothing until something has something to say |

## Memory and a run that lasts

| Measurement | Result |
| --- | --- |
| 100 mount rounds (20 warm-up, 80 measured) | linear memory **unchanged** across the eighty; regions back to 1, timers 0, tasks 0 |
| 100 hide-and-restore rounds | memory unchanged; the region still answers a real pointer afterwards |
| 250 mount/dispose rounds (M2) | one allocator step to a high-water mark, then identical at every sample; browser resources back to baseline |
| Four regions after start-up (M1) | 123,994,112 bytes of linear memory; wasm memory never shrinks |
| Endurance, 10 discrete actions a second | see the endurance section below |

Linear memory never shrinks, so every memory result here is a trend, not a
level: what is asserted is that the tail of a run is flat.

### Endurance

Two minutes at ten actions a second: **1,200 sent, 1,200 accepted, 0 refused**
at 9.92/s — nothing lost, duplicated or late. The memory curve was
`36896768 … 41680896`: flat, a climb of 4.8 MB, then flat again for the last
third. The test asserts that the last quarter is flat and prints the whole
curve, because a two-minute run cannot tell a plateau from a slow climb.

**The two-hour run found a leak**, which is what a two-hour run is for. Memory
grew 37 MB → 189 MB in doublings; the cause was the script VM's heap, which an
embedded region never collected. After the fix, fifteen minutes at the same
rate ends at 43.5 MB and flat. The whole account - curve, cause, host test and
measurement after - is in [the M8
report](../../validation/p1/m8-baselines.md).

## What is still not measured

- **A device GPU.** SwiftShader is a software rasteriser.
- **Chrome rather than the bundled Chromium.** The pass-gate browser was used to open the build, not to take these numbers.
- **A real network.** The throttled figures are Chrome's own emulation over a loopback server, not a link.
- **The PRD's B0/B1 loads.** P1's fixture is its own.
