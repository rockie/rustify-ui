# M7 report: deployment, recovery and diagnostics

Date: 2026-09-09. Machine: macOS (Darwin 25.6.0), Apple Silicon, rustc 1.97.0-nightly (2026-05-19), Node 26.1.0, Playwright 1.63.0 (bundled Chromium, SwiftShader WebGL). Every result here is automated.

## Status

Every exit condition in M7's milestone row now has a passing automated result. It is **not recorded as closed**, for the same reason M6 is not: the plan runs its milestones in order, and M5 is still waiting on two checks only a person can do. Nothing in M7 waits on anything.

| M7 exit condition | Where it is met |
| --- | --- |
| V8: WebGL2 denied | `a region denied a GPU context says so and keeps its DOM half working` (M2), plus the two-second bound below |
| V8: three classes of asset failure | `a missing / corrupt / truncated font costs glyphs, not the application` |
| V8: twenty context losses with edits between | `twenty losses, with the state edited between them, lose nothing` |
| V8: shared runtime trap | `every mount in the runtime is dead, and the page says which` |
| V8: a cause and an alternative within a bound | `says so within two seconds and leaves the DOM half working` |
| V8: the rebuilt region has the current state | `comes back with the state the application still holds` |
| V9: root, sub-path, embedded in an existing page | `boots, draws, and leaves the page it was embedded in alone`, plus the root deployment every other project runs against |
| V9: no cross-origin isolation, no dynamic JS | `boots without cross-origin isolation and draws into the region`, `no dynamic JS execution is needed and the bridge hash matches` (M1) |
| V9: restricted CSP, nothing unknown outbound | `asks for nothing but its own build` |
| V9: assets from two builds | `a bridge that claims another build stops the start and says so` |
| V9: ordinary text that contains script | `is a value, not markup and not a script` |
| V9: the resource total is accurate | `cargo xtask report-size --example <name> --release` |
| V9: diagnostics bounded, truncation transparent | `filling it past its ceiling keeps the newest and says how many went` |
| Shared-trap limits written into the capability document | `docs/compatibility.md`, "Deployment" |

| Command | Result |
| --- | --- |
| `cargo xtask doctor` | 9/9 |
| `cargo xtask sources verify` | makepad drift now includes `platform/src/script/res.rs`, attributable to this milestone; nouislider 3 files, 0 modified |
| `cargo test --workspace --lib` | 52 (diagnostics 9 of them) |
| `cargo test -p xtask` | 13 (fault injection 3 of them) |
| `cd makepad && cargo test` | 13 |
| `cargo clippy --workspace --all-targets -- -D warnings` | no warnings |
| `cargo fmt --all -- --check` | passes |
| `npx playwright test --project=property-workbench` | **62/62** |
| `npx playwright test --project=fusion-basic` | **46/46** |
| `npx playwright test --project=deployment` | **6/6** |

One caveat recorded rather than smoothed over: in the batch run that produced the fusion and deployment numbers, the workbench project reported 57 passed and 1 failed. The next project's start cleared the artifacts before the failing test could be named, and a full re-run of the same build passed 58/58. It is recorded here as an unidentified flake - not as a pass of something that failed. The machine was running cargo work alongside the browser tests, which is the condition under which this suite's timings are known to inflate.

## What is delivered

### A lost context is a state to come back from, not a failure

A canvas that loses its WebGL context loses everything drawn with it and nothing the application holds. The region publishes `RegionState::Lost` - a state of its own, not `Failed` - and is torn down, because a `Cx` whose GL objects the browser has reclaimed can only draw wrong. The host prevents the loss from being permanent, and the platform says when the canvas can be used again: on `webglcontextrestored` the region is built again and the application's current projection is applied to it before anything is drawn.

| Rule | Evidence |
| --- | --- |
| A loss is reported as recoverable, and the region stops rather than draws | `comes back with the state the application still holds` - the region reads `lost`, no live region remains |
| A region that cannot start at all says so within two seconds, and the DOM half keeps working | `says so within two seconds and leaves the DOM half working` - measured inside the page, because a round trip through the harness costs more than the budget being measured |
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
| The ceiling holds in a browser, not only on the host | `filling it past its ceiling keeps the newest and says how many went` - 1,100 refused mounts, 1,000 kept, the rest counted |

Producers wired in this release: `InvalidContainer` and `OccupiedContainer` (mount), `GpuInitFailed` (a canvas with no WebGL2), `GpuContextLost`, `Backpressure` (a scope's queue full), `UnsupportedCapability`, and `AssetLoadFailed`.

The last one needed a change in the fork: `handle_http_error` in `makepad/platform/src/script/res.rs` knew a resource had failed and only logged it, so an embedder could not tell its host that a file it deployed had not arrived. It now calls an optional hook, installed by `identify_runtime`, which records the path and what to do. The drift is attributable to this commit, which is what `cargo xtask sources verify` asks of a fork edit.

### A capability a region does not get

The embedded contract refuses the five things that belong to the page rather than to a region - opening a URL, changing the URL, moving through history, fullscreen, and the document title. They were already refused and warned about on the console; now they reach the record, once per capability per region, with a next step.

Wiring it found something that had been happening silently on every single start: **Makepad names its window when it creates one, so every region asks to set the document title and is refused**. Nothing was wrong with the refusal - an embedded region has no document title to set - but nobody could have known it was happening. It is now the first entry in every runtime's record.

| Rule | Evidence |
| --- | --- |
| A refusal reaches the record with a cause, a next step and the region it came from | `is refused, recorded once, and does not move the page` |
| A region asking to open a URL does not move the page | same test: the page's URL is unchanged |
| A region in a loop cannot fill the record with one mistake | same test: ten more requests add no entries |

### What a trap in the shared module reaches

| Rule | Evidence |
| --- | --- |
| Every mount in the runtime is dead, not just the one that trapped | `every mount in the runtime is dead, and the page says which` - two scopes, four regions, all of their controls gone |
| The page says what was lost and what to do | same test: the notice names `RuntimeFatal`, says to reload, and says unsaved in-memory state is lost |
| Nothing the page still holds can call back into the trapped module | same test: the application's entry points are gone from `window` |

### Deployment

| Rule | Evidence |
| --- | --- |
| The same build runs under a sub-path and leaves the page it sits in alone | `boots, draws, and leaves the page it was embedded in alone` |
| It asks for nothing but its own build, under the strict policy | `asks for nothing but its own build` - every request same-origin and under the base; the served policy has no `unsafe-inline` and no `unsafe-eval` |
| Assets from two builds are refused at start, with what to do | `a bridge that claims another build stops the start and says so` |
| A missing, corrupt or truncated font costs glyphs, not the application | `a missing/corrupt/truncated font costs glyphs, not the application` - the runtime starts, both halves keep moving one value, nothing traps |
| A font that did not arrive is reported, by name, with what to do | same test, missing case: an `AssetLoadFailed` entry naming the file |
| Every byte of a build is reportable in the six categories | `cargo xtask report-size --example <name> --release` |

`cargo xtask serve --fault` injects the three ways a deployment is actually broken - a file that was not copied, one that arrived damaged, and assets from two builds - at the server, so what is under test is the running product rather than a doctored copy of it. A running server also takes `GET <base>__fault/<spec>`, which is how the browser tests move between them without a restart.

## What is not done

- **Two of the three asset failure classes are still silent.** A 404 fails the load and is now reported. Damaged bytes arrive with a 200: the load *succeeds*, and the failure is in decoding them, inside the fork's font code rather than its loader. Both are survivable and both are verified as survivable; only the missing case is visible, and the test asserts that difference rather than implying the class is covered.
- **`RuntimeFatal` is not recorded in the ring.** The host notice exists and is verified (`a trapped runtime takes its controls off the page`), but by then the module cannot be called, so the entry would have to be written by the page rather than the SDK.

