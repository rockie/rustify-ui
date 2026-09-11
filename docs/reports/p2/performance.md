# P2 performance report

**The PRD's B1 figures are a gate, not a comparison.** A-3 was closed on
2026-09-11: the user approved R29 and R30 (B1 column) as pass/fail. The numbers
live in `tests/browser/budgets.ts`, `tests/browser/p2-budget.spec.ts` asserts
them, and `cargo xtask verify --suite p2` runs that project along with the rest.
A build that misses one fails the suite.

Every figure below is measured **inside the page**. A start-up timed from the
Playwright process carries the harness's own cost, and on this machine that cost
is not small - see "What the harness costs" below, where it is measured rather
than waved at.

## The gate

Machine: macOS (Darwin 25.6.0), Apple Silicon. Chromium 1.63.0 bundled with
Playwright, WebGL2 on SwiftShader. Release builds, 2026-09-11.

| Requirement | Budget | Measured | |
| --- | --- | --- | --- |
| R29 AC1 · cold start, 30 loads | p95 ≤ 5,000 ms | **p95 171 ms**, p50 159 ms, worst 171 ms, 0 failed starts | pass |
| R29 AC3 · hot start, 30 loads | p95 ≤ 2,000 ms | **p95 159 ms**, p50 152 ms, worst 161 ms, 0 failed starts | pass |
| R29 AC2 · first load, compressed | ≤ 8 MiB (8,388,608 B) | **2,772,456 B** gzipped over 17 files, 0 uncounted | pass |
| R30 AC1 · 1,000 cross-region actions | p95 ≤ 50 ms, p99 ≤ 100 ms, 0 errored | **p95 42.2 ms, p99 43.8 ms**, worst 53.2 ms, 0 refused, 0 errors | pass |

**What the gate does not cover**, stated here because a gate read as covering
more than it measures is worse than none: one machine and one browser, under a
software rasteriser; B0, and R30's AC2/AC3, whose B2 and B3 loads this phase
does not build; and start-up over a throttled link, which is observed in
`m8-network.spec.ts` and is not a reading of AC2 because that server sends no
`Content-Encoding`. Those are recorded as not measured, never as passed.

## How each one is measured, and why that way

| Measurement | How | Why this way |
| --- | --- | --- |
| Cold start | A fresh browser context per load; an init script marks, in the page, when the application mounted and when the region reported ready | A context is the boundary the browser actually resets. Disabling the HTTP cache is not enough: the compiled module is kept in a code cache that setting does not touch, and thirty loads in one context measured that cache from the second load on (p50 105 ms, p95 10 s - a bimodal curve that was an artefact, not a build) |
| Hot start | One context, so the cache carries; a **page** of its own per load, closed before the next opens | A reload in place puts the previous document's death in the new document's timeline (below). What R29 asks about is how long the user waits for a start-up, not how long the browser takes to throw the last page away |
| Transfer | Every resource the first load fetched, re-read in the page and gzipped with `CompressionStream` | The budget counts *compressed* bytes, and the preview server sends none. Reading the built directory instead would count files a first load never asks for - the CJK and emoji faces alone are 29 MB of them |
| Interaction latency | 1,000 actions alternating region→DOM and DOM→region, each timed from the input to the frame after the value landed | Both directions, because either alone is half of what R30 asks. Stopping when the value lands would leave out the frame the user waits for; waiting two frames would measure the display's clock |

## Transfer, by category

What a first load actually fetches, gzipped in the browser from the bytes it
fetched, 2026-09-11:

| Category | Gzipped | Uncompressed | Files |
| --- | --- | --- | --- |
| wasm | 2,625,807 | 11,555,017 | 1 |
| fonts | 83,779 | 182,092 | 1 |
| js | 53,611 | 235,265 | 9 |
| css | 7,844 | 37,752 | 4 |
| data | 882 | 3,119 | 1 |
| document | 533 | 1,431 | 1 |
| **total** | **2,772,456** | **12,014,676** | **17** |

The module is 95% of it, and it compresses to 23% of its size - which is why the
budget is a compressed one and why serving this build uncompressed would be a
deployment mistake rather than a detail.

**One Latin font on a first load, out of 51 MB in the directory.** The families
that cover CJK and emoji are fetched when a value needs them, and the first
Chinese glyph is what pays for them: 2 files, 29,718,416 bytes
(`LXGWWenKaiRegular.ttf` 19,074,264 and `NotoColorEmoji.ttf` 10,644,152). That is
a deliberate deferral, not an optimisation: a page that never draws a non-Latin
glyph never fetches them, and a page that draws one waits for 29 MB. It is
outside AC2's first-load budget by construction, and it is stated here so that
nobody discovers it from a user.

## What the harness costs, and where it landed

Worth recording because it corrupted an earlier figure in this very report: the
first version said a cold load took **9,916 ms**, timed from the Playwright
process. Measured in the page, the same start-up is about 160 ms.

A navigation in this harness costs about **10.9 seconds of wall clock** whatever
the page does - measured directly, by timing the load from the harness while the
page timed itself. The application's own teardown is not it: `dispose()` was
measured at **7 to 17 ms** inside the page. When the new document's clock starts
before the browser has finished discarding the old one, those seconds land in the
new page's timeline: reloading in place, three to six loads in thirty came back
at 10.5 s, and their own resource timings showed the document received in 1 ms
and nothing parsed until 10,460 ms.

So: a page per load, closed before the next one opens, and every number taken in
the page. Nothing here is a claim about how long a *reload* feels on this
machine; that is a property of this harness, and it is the reason the gate is
built the way it is.

## What the preview server had to learn

`cargo xtask serve` sent `Cache-Control: no-cache` and no validator, so nothing
it served could ever be revalidated. It now sends an `ETag` (size and
modification time - hashing eleven megabytes per request would trade the
download for a read) and answers `If-None-Match` with a 304; `cargo test -p
xtask` covers the matching, and `curl` confirms the 304 end to end.

The module is still fetched in full on every hot load, and that is the browser's
doing rather than the server's: a Playwright context will not hold an entry of
11.5 MB. So the hot-start figure above is a load with the *small* files cached
and the module re-fetched - which on a loopback costs about 26 ms, and on a real
link would not. The gate records the figure; it does not pretend the module came
from a cache.

## Start-up, interaction and memory, measured in the page

Thirty mounts, a thousand inputs, and a hundred mount rounds:

| Measurement | Result | Beside |
| --- | --- | --- |
| Mount to ready, 30 rounds | median **64.0 ms**, worst 73.3 ms | Not R29's cold start: a mount into a page whose module is already compiled |
| A thousand inputs the application accepted | median **0.000 ms**, p95 **0.100 ms**, worst 0.400 ms | The controlled round trip alone, without the frame. R30's gate is the figure in the table above, which includes it |
| 80 measured mount rounds (after 20 to warm up) | memory 39,649,280 → **39,649,280 bytes**, 1 region, 0 timers, 0 tasks left | R32. No growth at all across the measured rounds |
| 100 hide-and-restore rounds | 39,649,280 → **39,649,280 bytes**, and the region still working | R31 |

## What the diagnostics record costs

| | Time for 1,000 entries | Held |
| --- | --- | --- |
| Record on | 2.5 ms (2.5 µs each) | 113,000 bytes |
| Record off | 1.5 ms (1.5 µs each) | 0 |

With nothing going wrong the record costs nothing measurable: input latency is
median 0.000 ms and p95 0.100 ms with it on *and* with it off.

## What a long run does to memory

`m8-endurance` used to fail its memory-tail assertion, and it was reported as a
defect older than P2. It was neither older nor a defect: the assertion was
measuring a **cache filling**.

Two things grow when this fixture is driven hard, and both are bounded:

1. **The text caches.** The shaper and the layouter hold 4,096 entries each.
   Every object in B1 has a name, a position and a size the region draws, so
   walking a thousand objects is three thousand strings drawn for the first
   time - and a two-minute run at ten actions a second is *shorter than one lap
   of the fixture*. Its tail was a cache that had not finished filling.
2. **Script-evaluation garbage.** A props application that sets a shader value
   evaluates a small script and leaves four blocks behind; the runtime collects
   at a slack of 20,000 objects (`SCRIPT_HEAP_SLACK`), so a busy region
   sawtooths within that and an idle one never collects at all.

Proved by walking the lap and then walking it again, with a counting allocator
in the build:

| Pass | Live bytes retained | Per action |
| --- | --- | --- |
| First lap, 1,050 objects | 3,172,712 | ~3,021 B |
| Second pass, 300 of the same objects | 91,203 | 304 B |
| Third pass, 300 of the same objects | 93,200 | 304 B |

The second and third passes retain exactly four blocks per action - one script
evaluation - and linear memory does not move at all. The test now walks the lap
first and then measures; the details are in
[M10's report](../../validation/p2/m10.md).

## What these numbers are not

- Not a frame-rate claim. The gate runs WebGL2 under SwiftShader, a software
  rasteriser.
- Not a claim about a cold cache over a real network. The tests serve from
  localhost; the throttled observations are in `m8-network.spec.ts` and are
  uncompressed.
- Not a claim about any machine but the one named at the top.
- Not stable to the last millisecond. One run of the latency gate on a busy
  machine came back at p95 108 ms and p99 426 ms where the quiet runs give 42 ms
  and 44 ms. The budget has room for the quiet case and not for a machine doing
  something else at the same time, which is worth knowing before reading a
  single red run as a regression.
