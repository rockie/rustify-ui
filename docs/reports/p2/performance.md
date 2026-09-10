# P2 performance report

Baselines for B1, beside the PRD's R29 and R30 figures. **No budget is claimed.**
A-3 stands: these are measurements of one build on one machine under a software
rasteriser, and the reason they are recorded is so that the next change can be
compared with them - not so that a number can be called a pass.

Machine and build are stated in each milestone report. Every figure below is
measured *inside the page*: a browser timing taken from the Playwright process
is ten to thirty times the in-page cost, so a harness-side number would be a
measurement of the harness.

## What is measured, and how

| Measurement | How | Why this way |
| --- | --- | --- |
| Cold start | `performance` marks inside the page, from navigation to the region reporting ready | The harness's own wait includes process and protocol time that the user never pays |
| Interaction latency | Counted in one `page.evaluate`: actions the application *accepted*, not events dispatched | An action the scope refused is not a slow action, it is a refused one, and counting dispatches hides that |
| Transfer | `cargo xtask report-size`, six categories, read off the built directory, and again gzipped | The directory is the deployment; nothing is estimated |
| Memory | `hooks.runtime.stats()` inside the page, sampled over the run | wasm linear memory never shrinks, so a tail is the only honest shape to report |

## Transfer, by category

Read off the built directories with `cargo xtask report-size`, release builds,
2026-09-10. These are what the deployment *contains*; what a first load actually
fetches is smaller, and is measured separately in the browser (below).

| Example | wasm | js | css | fonts | images | data | total |
| --- | --- | --- | --- | --- | --- | --- | --- |
| fusion-basic | 8,279,424 | 189,641 | 3,208 | 51,187,844 | 20,701 | 9,044 | 59,689,862 |
| component-catalog | 11,307,927 | 189,157 | 29,308 | 51,187,844 | 20,701 | 8,943 | 62,743,880 |
| property-workbench | 11,477,091 | 232,565 | 36,552 | 51,187,844 | 20,701 | 9,456 | 62,964,209 |
| property-workbench at `/tools/demo/` | 11,477,091 | 232,565 | 36,552 | 51,187,844 | 20,701 | 12,352 | 62,967,105 |

The font total dominates every row and is almost entirely the two families that
cover more than Latin. They are **not** fetched unless a value needs them: a
label whose text is not ASCII moves to the wider family, and that move is what
fetches the file. What a first load actually costs, and what the first Chinese
glyph adds to it, are measured in the browser rather than inferred from this
table.

The same wasm serves the root and the sub-path deployment; only the manifest
and the page differ, which is the 2,896-byte gap in the `data` column.

## What a first load actually costs

Counted by the browser, empty cache, localhost, release build of B1, 2026-09-11.

| | Ready in | Over the wire |
| --- | --- | --- |
| Loopback | 9,916 ms | 11,935,619 bytes |
| 12 Mbit/s, 40 ms latency | 18,687 ms | 11,935,619 bytes |
| 3 Mbit/s, 40 ms latency | 32,179 ms | 11,753,527 bytes |

By category on the first load: wasm 11,477,391 (1 file), js 235,265 (9), css
37,752 (4), fonts 182,092 (1), data 3,119 (1).

**182 KB of fonts, out of 51 MB in the directory.** The families that cover
CJK and emoji are fetched when a value needs them, and the first Chinese glyph
is what pays for them: 2 files, 29,718,416 bytes
(`LXGWWenKaiRegular.ttf` 19,074,264 and `NotoColorEmoji.ttf` 10,644,152). That
is the single largest number in this report and it is a *deliberate* deferral,
not an optimisation - a page that never draws a non-Latin glyph never fetches
them at all, and a page that draws one waits for 29 MB.

R29's start-up figure is 5 s. This build is 9.9 s on a loopback. **Nothing is
claimed either way**: A-3 says these are baselines, the gate runs a software
rasteriser, and the wasm is a debug-symbol-free release build with no size work
done on it at all.

## Start-up, interaction and memory, measured in the page

Thirty mounts, a thousand inputs, and a hundred mount rounds, all counted
inside the page for the reason at the top of this report.

| Measurement | Result | Beside |
| --- | --- | --- |
| Mount to ready, 30 rounds | median **64.0 ms**, worst 73.3 ms | Not R29's cold start: this is a mount into a page whose module is already compiled |
| A thousand inputs the application accepted | median **0.000 ms**, p95 **0.100 ms**, worst 0.400 ms | R30's latency targets. Comfortably inside them, and measured as accepted actions rather than dispatched events |
| 80 measured mount rounds (after 20 to warm up) | memory 39,649,280 → **39,649,280 bytes**, 1 region, 0 timers, 0 tasks left | R32. No growth at all across the measured rounds |
| 100 hide-and-restore rounds | 39,649,280 → **39,649,280 bytes**, and the region still working | R31 |

## What the diagnostics record costs

Measured both ways, because a record whose cost cannot be subtracted is a
record nobody can size.

| | Time for 1,000 entries | Held |
| --- | --- | --- |
| Record on | 2.5 ms (2.5 µs each) | 113,000 bytes |
| Record off | 1.5 ms (1.5 µs each) | 0 |

With nothing going wrong, the record costs nothing measurable: input latency is
median 0.000 ms and p95 0.100 ms with it on *and* with it off.

## The one long-running failure

`m8-endurance`'s memory tail assertion fails, and it predates P2. This was
established in M2 rather than asserted: the pre-P2 commit was built in a
`git worktree`, the build directory was swapped, and the same assertion failed
there with a *larger* margin (851,968 bytes of tail growth before P2 against
786,432 after, on a threshold of about 420 KB). The behavioural half of the same
test - 1,200 actions sent, 1,200 received, none refused, no errors - passes on
both.

It is reported as a known limitation rather than as a P2 regression, and the
measurement that established that is in [M2's report](../../validation/p2/m2.md).

## What these numbers are not

- Not a frame-rate claim. The gate runs WebGL2 under SwiftShader, a software
  rasteriser.
- Not a claim about a cold cache over a real network. The tests serve from
  localhost.
- Not a claim about any machine but the one in the milestone report.
