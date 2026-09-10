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

## The figures

Filled in by M8 from the run recorded in `docs/validation/p2/m8.md`. Each row
carries the R29/R30 figure it sits beside and says plainly whether the release
claims anything about it, which it does not.

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
