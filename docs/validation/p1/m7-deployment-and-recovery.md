# M7 report: deployment, recovery and diagnostics

Date: 2026-09-09. Machine: macOS (Darwin 25.6.0), Apple Silicon, rustc 1.97.0-nightly (2026-05-19), Node 26.1.0, Playwright 1.63.0 (bundled Chromium, SwiftShader WebGL). Every result here is automated.

## Status

M7 is **not closed**. Most of its exit conditions have a passing automated result; the gaps are listed under "What is not done" and are specific, not general.

| Command | Result |
| --- | --- |
| `cargo xtask doctor` | 9/9 |
| `cargo xtask sources verify` | makepad drift attributable as before; nouislider 3 files, 0 modified |
| `cargo test --workspace --lib` | 51 (diagnostics 8 of them) |
| `cargo test -p xtask` | 13 (fault injection 3 of them) |
| `cd makepad && cargo test` | 13 |
| `cargo clippy --workspace --all-targets -- -D warnings` | no warnings |
| `cargo fmt --all -- --check` | passes |
| `npx playwright test --project=property-workbench` | **58/58** |
| `npx playwright test --project=fusion-basic` | **45/45** |
| `npx playwright test --project=deployment` | **6/6** |

One caveat recorded rather than smoothed over: in the batch run that produced the fusion and deployment numbers, the workbench project reported 57 passed and 1 failed. The next project's start cleared the artifacts before the failing test could be named, and a full re-run of the same build passed 58/58. It is recorded here as an unidentified flake - not as a pass of something that failed. The machine was running cargo work alongside the browser tests, which is the condition under which this suite's timings are known to inflate.

## What is delivered

### A lost context is a state to come back from, not a failure

A canvas that loses its WebGL context loses everything drawn with it and nothing the application holds. The region publishes `RegionState::Lost` - a state of its own, not `Failed` - and is torn down, because a `Cx` whose GL objects the browser has reclaimed can only draw wrong. The host prevents the loss from being permanent, and the platform says when the canvas can be used again: on `webglcontextrestored` the region is built again and the application's current projection is applied to it before anything is drawn.

| Rule | Evidence |
| --- | --- |
| A loss is reported as recoverable, and the region stops rather than draws | `comes back with the state the application still holds` - the region reads `lost`, no live region remains |
| The application's state is untouched by the loss | same test: the selection and the edited name are unchanged while the context is gone |
| A restored context brings the region back with that state, drawing the same picture | same test: `ready` again, and the region's pixels match what it drew before within the settle threshold |
| Twenty losses with an edit between each lose nothing | `twenty losses, with the state edited between them, lose nothing` - and the rebuilt region still answers a real pointer at the end |
| It is recorded as recoverable, with nothing for the application to do | same tests: one `GpuContextLost` entry per loss, whose suggestion is that the region is rebuilt with the current state |

The first attempt replaced the canvas element instead of waiting, on the theory that a lost context cannot be given back to the canvas that lost it. Leptos reused the element, the new region compiled its shaders against the dead context, and the region came back blank - worse than staying visibly lost, because it looks like it worked. Waiting for the platform's own restore is both simpler and what the platform actually offers.

### A bounded record that admits what it dropped

`crates/rustify-ui/src/diagnostics.rs`. The ten failure classes the plan registers, each with a next step, in a record bounded by a count (1,000) and a size (4 MiB). Reaching either drops the oldest entries and counts them, and every report leads with that count, so a tail is never mistaken for the whole story.

`detail` is a `&'static str` and everything that varies is a typed field. That is a structural answer to NFR-4's "do not log the user's text": there is no path for it to get in.

| Rule | Evidence |
| --- | --- |
| Ten classes, no duplicates, each with a next step | `the_ten_registered_kinds_are_all_there` |
| A lookup that found nothing is not a failure of the runtime | `an_answer_to_a_lookup_is_not_a_failure_of_the_runtime` |
| Either ceiling keeps the newest and says how many it dropped | `the_count_ceiling_keeps_the_newest_and_says_how_many_it_dropped`, `the_byte_ceiling_holds_even_when_the_count_would_not` |
| A report leads with what it dropped and names the runtime and build | `a_report_says_what_was_dropped_before_it_says_anything_else`, `the record names the runtime, the build, and what it dropped` |
| Entries carry a cause and a next step, and never the user's text | `every entry carries a cause and a next step, and no user content` |

Producers wired in this release: `InvalidContainer` and `OccupiedContainer` (mount), `GpuInitFailed` (a canvas with no WebGL2), `GpuContextLost`, `Backpressure` (a scope's queue full).

### Deployment

| Rule | Evidence |
| --- | --- |
| The same build runs under a sub-path and leaves the page it sits in alone | `boots, draws, and leaves the page it was embedded in alone` |
| It asks for nothing but its own build, under the strict policy | `asks for nothing but its own build` - every request same-origin and under the base; the served policy has no `unsafe-inline` and no `unsafe-eval` |
| Assets from two builds are refused at start, with what to do | `a bridge that claims another build stops the start and says so` |
| A missing, corrupt or truncated font costs glyphs, not the application | `a missing/corrupt/truncated font costs glyphs, not the application` - the runtime starts, both halves keep moving one value, nothing traps |
| Every byte of a build is reportable in the six categories | `cargo xtask report-size --example <name> --release` |

`cargo xtask serve --fault` injects the three ways a deployment is actually broken - a file that was not copied, one that arrived damaged, and assets from two builds - at the server, so what is under test is the running product rather than a doctored copy of it. A running server also takes `GET <base>__fault/<spec>`, which is how the browser tests move between them without a restart.

## What is not done

- **`AssetLoadFailed` has no producer.** The fork's own resource loader absorbs a font failure, so the SDK never learns of it. The observable behaviour is verified (the region keeps working); the reporting is not. Wiring it means a hook inside the fork's loader.
- **`UnsupportedCapability` has no producer either.** The refusals exist and warn once on the console (`makepad/platform/src/os/web/web.js`), but nothing routes them into the record, and no example asks a region for a capability, so there is nothing to test yet.
- **`RuntimeFatal` is not recorded in the ring.** The host notice exists and is verified (`a trapped runtime takes its controls off the page`), but by then the module cannot be called, so the entry would have to be written by the page rather than the SDK.
- **The two-second bound** on "a failed region says so within 2 s" (NFR-2) is not measured; the failure is visible but its latency is not asserted.
- **Truncation is proven on the host, not in a browser.** Filling the ring past 1,000 entries in a page would need a producer that can be driven that hard; the ceilings themselves are covered by host tests.
- **A shared-runtime trap's blast radius** is verified as "every mount in the runtime is dead and the page says so"; the plan also asks that the limit be written into the capability document, which is now done, but nothing automatic checks that claim.
