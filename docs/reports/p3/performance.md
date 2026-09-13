# P3 performance report

**These are gates, not comparisons.** A-3 closed on 2026-09-11 when the user
approved R29 and R30 as pass/fail. The numbers live in
`tests/browser/budgets.ts`; three projects assert them — `budget` for B1,
`budget-minimal` for B0, `budget-data` for B2 and the jobs — and
`cargo xtask verify --suite p3` runs all of them. A build that misses one fails
the suite. R30 AC2 over B3 is asserted by `budget-scene`, which runs on headed
Chrome by hand and is explained below.

Every figure is measured **inside the page**. A navigation in this harness
costs about 10.9 seconds of wall clock whatever the page does, against
start-ups of 140–190 ms; timed from the test process, this application's cold
start was once reported in an earlier report as 9,916 ms. That is a property of
the harness, and it is why the clocks are where they are.

Machine: macOS 26.6.2 (Darwin 25.6.0), Apple Silicon. Playwright 1.63.0 with
its bundled Chromium, WebGL2 on SwiftShader, except where the table says headed
Chrome. Release builds, 2026-09-13. Loads: baseline `p3-loads-1`
(`tests/browser/loads.ts`).

## The gates

### R29 · B0, the smallest complete application

Ten DOM controls, twenty GPU controls in one region, one shared state, Latin
text only.

| Requirement | Budget | Measured | |
| --- | --- | --- | --- |
| AC1 · cold start, 30 loads | p95 ≤ 2,500 ms | **p95 149 ms**, p50 138 ms, worst 213 ms, 0 failed starts | pass |
| AC3 · hot start, 30 loads | p95 ≤ 1,000 ms | **p95 140 ms**, p50 135 ms, worst 142 ms, 0 failed starts | pass |
| AC2 · first load, compressed | ≤ 3 MiB (3,145,728 B) | **2,436,583 B** gzipped over 14 files, 0 uncounted | pass |

Where the start-up goes: the module's bytes arrive by p95 26 ms cold and 25 ms
hot; everything after them — compiling, booting the runtime, building the view
and the region — is p95 124 ms and 116 ms. The work is in the second half, and
the first is a loopback.

**B0's own budget is tighter than B1's and that is the point.** R29 gives B0
3 MiB where B1 gets 8; B1 spends 2.78 MB on its module alone, so a build that
passed B1's budget could miss B0's by a factor of three and nobody would know
from the B1 gate. Headroom here is 709,145 bytes.

### R29 / R30 · B1, the workbench

Re-run unchanged from P2, on this tree:

| Requirement | Budget | Measured | |
| --- | --- | --- | --- |
| AC1 · cold start, 30 loads | p95 ≤ 5,000 ms | **p95 188 ms**, p50 178 ms | pass |
| AC3 · hot start, 30 loads | p95 ≤ 2,000 ms | **p95 159 ms** | pass |
| AC2 · first load, compressed | ≤ 8 MiB | **2,780,592 B** over 17 files | pass |
| R30 AC1 · 1,000 cross-region actions | p95 ≤ 50 ms, p99 ≤ 100 ms | **p95 45.5 ms**, p99 46.8 ms, worst 81.4 ms, 0 refused, 0 errors | pass |

**B1's latency has 4.5 ms of margin and a history.** It rose across four
milestones on a build whose action path did not change — 41.7, 44.9, 45.3,
46.3 ms — and then fell to 45.5 ms on a tree that changed nothing at all. The
likeliest reading is the machine rather than the build; the fall supports it,
but it is a reading and not a finding, and it is recorded in
[known limitations](known-limitations.md) as unresolved.

### R30 AC2 / AC3 · B2, a hundred thousand rows

A hundred thousand rows by twenty columns, scrolled three rows a frame from
inside the page, for five independent runs of 60 seconds each after 10 seconds
that are not measured. Each run has to hold on its own; the budget is not the
best of them.

| Requirement | Budget | Measured | |
| --- | --- | --- | --- |
| AC2 · effective frame interval, p95 | ≤ 20 ms, each of 5 runs | **16.70, 16.80, 16.70, 16.70, 16.70 ms** | pass |
| AC2 · share over 50 ms | ≤ 1%, each of 5 runs | **0.00%** in all five, of 3,601 samples each | pass |
| AC2 · every drive accepted | recomputes = drives | **3,601 = 3,601** in all five | pass |
| AC2 · effective frames | ≥ 99% of drives | **3,601 of 3,601** — 100% | pass |
| AC3 · single-column sort, 20 rounds | p95 ≤ 500 ms | **p95 116 ms**, p50 98 ms, worst 132 ms | pass |
| AC3 · text filter, 20 rounds | p95 ≤ 500 ms | **p95 32 ms**, p50 31 ms, worst 32 ms | pass |
| AC3 · cancel to shown, 20 rounds | p95 ≤ 100 ms | **p95 14.3 ms**, p50 13.8 ms, worst 14.4 ms | pass |
| AC3 · results | equal to the independent list | equal, at three columns × 200 rows | pass |

The negative probe did what it is there to do: with the drive continuing and
the content frozen, **0 presentations for 301 drives over 302 callbacks**, no
samples at all, verdict "does not hold".

Two things in that table are worth reading together. The interval is 16.70 ms
— one display frame — and 4,201 callbacks produced 3,601 presentations, which
is the 10-second warm-up (600 callbacks) plus one presentation for every driven
frame after it. The table is redrawing every frame it is asked to and never
missing one.

**A sort takes 116 ms and holds no frame while it does.** The job is sliced to
an 8 ms budget with a `MessageChannel` yield between slices, so the 116 ms is
spread across frames rather than spent inside one. The filter is cheaper
because it is one pass over the sample where a sort is a sort.

### R30 AC2 · B3, ten thousand objects — headed Chrome only

Ten thousand rectangles with eight-character labels, at most two layers deep,
the camera panned `[7, 3]` scene pixels per driven frame, five independent
60-second runs after 10 seconds that are not measured. Headed Chrome, run by
hand.

| Requirement | Budget | Measured | |
| --- | --- | --- | --- |
| AC2 · effective frame interval, p95 | ≤ 20 ms, each of 5 runs | **9.30 ms** in all five | pass |
| AC2 · share over 50 ms | ≤ 1%, each of 5 runs | **0.00%** in all five, of ~7,201 samples each | pass |
| AC2 · effective frames | ≥ 99% of drives | **7,201 of 7,201** — 100%, in all five | pass |
| AC2 · the scene accepted the camera | more than zero | yes | pass |
| Culling matches the independent list | equal at three cameras | equal | pass |

The negative probe: **0 presentations for 601 drives over 602 callbacks**, with
**601 pumps**. That last number is what makes this probe worth having — the
region was pumping and receiving props the whole time, and declined only to
apply the camera. The picture genuinely stopped while everything around it kept
running, and the gate produced no samples at all.

**This display runs at 120 Hz, and the figure is against its ceiling, not the
budget's.** Sixty seconds produced 7,201 effective presentations, which is
120 per second; the physical floor for an interval here is 8.33 ms and the
measurement is 9.30. The scene is presenting on essentially every frame the
display offers.

**9.30 ms is also the number that fell when a defect was fixed.** M1's probe
measured 17.60 ms on this machine, and the cause was found in M4: the props
pump asked for the next frame from a microtask, and a repeat request made
before the previous callback had run was dropped — so two asks landed on one
frame and the third went by empty, presenting on every second frame. Fulfilling
that frame in place halved the interval. The budget was met before and after;
what changed is that the scene now draws every frame it is asked to.

## What an "effective presentation" is, and why the gate is built on it

An animation-frame callback arrives about every 16.7 ms whether or not
anything changed. A frame budget measured on callbacks therefore **passes a
frozen screen**, which is the one thing it must never do.

So each gate reads a content version instead:

| Load | Version | Moves when |
| --- | --- | --- |
| B2 | the table's `window_version()` | the table works out its visible range again |
| B3 | the runtime's `stats().frames` | a region begins a canvas render |

A callback in which the version did not move produces **no sample**; the
interval it represents is carried into the next effective frame. What is
reported is the interval between two frames whose content actually differs.

**And each gate run begins with a negative probe that has to fail.** For five
seconds the drive continues and the content is frozen — B3 through the
application's `freeze_scene(true)`, which keeps the region pumping while it
declines to apply the camera; B2 by re-driving the same scroll position, which
needs no switch because the table's content version is a function of where it
is scrolled to. If the frozen run *meets* the budget, the gate is measuring
callbacks and the whole project fails before a real figure is taken.

Two further assertions guard the same thing from the other side: every drive
must be accepted (B2's recomputes equal its drives; B3's accepted count is the
region's own), and effective frames must be at least 99% of drives. A region
keeping up with two thirds of its input is not keeping up.

## Why B3 does not run where everything else does

M1's second probe drove the same scene the same way under both environments:

| Environment | p95 effective frame interval | Share over 50 ms |
| --- | --- | --- |
| Headless Chromium, SwiftShader | 50.10 ms | 13.9% |
| Headed Chrome 152.0.7977.84 | 17.60 ms | 0% |

SwiftShader is a software rasteriser and R30's figures are about a machine with
a GPU. Under it, a failure would say nothing about the budget and a pass would
say nothing about the machine. So B3's gate is its own project, it runs on
headed Chrome, CI does not run it, and the spec skips itself with that
explanation rather than reporting a rasteriser as a red build.

B2's gate runs headless because B2 is DOM work — the rows are elements and the
cells are text nodes — and the rasteriser is not what is under test. M1's first
probe established the environment is capable of it: a bare windowed grid of 768
text nodes, scrolled three rows a frame, measured p95 16.80 ms with 0% over
50 ms.

## What these gates do not cover

Stated here because a gate read as covering more than it measures is worse than
no gate at all.

- **One machine, one browser.** R29 asks for each performance machine and fully
  supported browser to pass on its own. One has.
- **B2 and B3 are measured separately.** Nothing here claims they hold at the
  same time on one page.
- **The GPU figure in R32 is a ledger, not a driver reading.** See
  [compatibility](compatibility.md).
- **Cold start is on the loopback.** It measures what the machine costs, not
  what a link costs. The throttled figures stay observations in
  `m8-network.spec.ts`, where the server sends no `Content-Encoding` and they
  are therefore not a reading of AC2.
- **The first non-Latin glyph is outside AC2 by construction.** The CJK and
  emoji faces are 29,718,416 bytes and are fetched when a value needs them. A
  page that never draws one never pays; a page that draws one waits. Deliberate,
  and stated so that nobody discovers it from a user.
