# P1 fault and recovery report

Every failure P1 exercises, what the product does about it, and which ones it
cannot see. Evidence is in [M2](../../validation/p1/m2-runtime.md),
[M7](../../validation/p1/m7-deployment-and-recovery.md) and
[M8](../../validation/p1/m8-baselines.md).

Faults are injected against the running product: a served deployment broken by
`cargo xtask serve --fault`, a real `WEBGL_lose_context`, a real trap. Nothing
here is a doctored copy of the build under test.

## What happens to a running scope

| Fault | What the product does | Bound | Evidence |
| --- | --- | --- | --- |
| A canvas cannot get a WebGL2 context | The region publishes `Failed(GpuUnavailable)` and the DOM half of the same scope keeps working; one `GpuInitFailed` entry | The failure is on screen within 2 s, measured inside the page | M2, M7 |
| A GPU context is lost | The region publishes `Lost` — a state to come back from, not a failure — and is torn down, because a `Cx` whose GL objects the browser reclaimed can only draw wrong | The browser's own `webglcontextrestored`; twenty rounds with an edit between each lose nothing | M7 |
| A context is restored | The region is built again and the application's current projection is applied before the first draw | The rebuilt region answers a real pointer | M7 |
| A scope's action queue fills | The action is refused with `Backpressure` and the application is told to run it again; it did not happen | The queue holds 1,024 | M3, M7 |
| A region asks for a capability it does not have (open a URL, change the URL, move through history, fullscreen, the document title) | Refused, and recorded once per capability per region with a next step. Every region asks to set the document title as it starts, because Makepad names its window; that refusal is the first entry in every runtime's record | Once per capability per region: a region in a loop cannot fill the record | M7 |
| The wasm module traps | Every mount scope in that runtime is dead. The page removes their controls, says `RuntimeFatal`, says unsaved in-memory state is lost, and asks for a reload; nothing the page still holds can call back into the module | Whole page; this is the ADR-1 boundary and is not per-scope isolation | M3, M7 |

## What happens to a deployment

| Fault | What the product does | Evidence |
| --- | --- | --- |
| A file was not copied (404) | The runtime starts, both halves keep working, and the glyphs that font would have drawn are lost. An `AssetLoadFailed` entry names the file and says to check it is deployed beside the build | M7 |
| A file arrived damaged | Survivable in the same way — and **not reported**. It arrives with a 200, so the load succeeds and the failure is in decoding, deeper than the loader's hook | M7 |
| A file arrived truncated | The same: survivable, not reported | M7 |
| Assets from two different builds | The start is stopped with `BuildContractMismatch` and a message saying to redeploy matching assets. The bridge module carries the schema hash the wasm exports | M7 |
| The policy forbids wasm (`--csp no-wasm`) | The start fails visibly rather than silently | M1 |

The three asset classes are all verified as survivable; only the first is
visible in the record. The tests assert which class is which rather than
implying the whole class is covered.

## The record itself

One bounded record per runtime: ten registered failure classes, each with a
next step, bounded by 1,000 entries and 4 MiB. Reaching either ceiling drops
the oldest and counts them, and every report leads with `dropped`, so a tail is
never mistaken for the whole story. Proven in the browser as well as on the
host: 1,100 refused mounts leave 1,000 entries and count the rest (M7).

`detail` is a `&'static str` and everything that varies is a typed field. That
is a structural answer to "do not log the user's text": there is no path for it
to get in (M7).

Since M8 the record has a switch. Off, it keeps nothing and counts what it
turned away, so a report taken while it was off says so — and the cost of
keeping it can be measured against the same path without it. See [the
performance report](performance.md).

## Recovery that P1 does not offer

- **No per-scope trap isolation.** One wasm module serves every scope on the page (ADR-1). A trap ends all of them.
- **`RuntimeFatal` is not in the record.** After a trap the module cannot be called, so only the page can say anything, and the page's static notice is what is verified.
- **Damaged and truncated assets have no producer.** See above.
- **No retry of a failed asset.** A font that did not arrive is reported once; nothing fetches it again.
