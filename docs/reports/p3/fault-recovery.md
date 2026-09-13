# P3 fault and recovery

Every failure this release was made to survive, what the product does about it,
and the ones it cannot see. P1's and P2's rows still hold and are not repeated;
what is here is what P3 added — an instance boundary, a bounded retry, four
fault classes run twenty times each, and two hours of a load that must not lose
an action.

## The instance boundary

P1 shipped one wasm instance with several mounts inside it, and said plainly
that a trap took all of them (P1 D9). R26 AC3 asks for the opposite: one
application instance fails, the others on the page carry on. P3 makes the
instance a wasm instance.

A trap has **four doors**, and all four were made to arrive at the same place:

| Door | How it is entered | Who notices |
| --- | --- | --- |
| An exported call | the page calls into the application and the call traps | the loader's call boundary: it reports the failure, then rethrows |
| A host task | the runtime's own `defer`ed work traps | the runtime's `try` around it |
| A DOM event handler | a click handler traps, with no Rust frame below it | the page-level `error` listener, attributed by the glue URL in `error.stack` |
| The page, simulating one | `enter_fatal` | itself |

After any of them the failed instance shows a notice that says unsaved
in-memory state is gone and offers to restart; the other instance on the page
accepted **100 of 100** actions, kept its regions, and kept its own diagnostics
ring. Health is judged by what still works, not by an absence of console noise:
a dead instance's closures can be finalised later and complain, which says
nothing about its neighbour, so `pageerror = 0` is not the criterion.

**`Drop` does not run after a trap.** `panic = "abort"` means the destructors
that would remove the router's listeners, the overlay's listeners and the URL
ownership never run, and those are registered on `window` and `document` —
objects that outlive the instance. So their lifetime is held on the JavaScript
side: one `AbortController` per instance, every page-level listener registered
with its signal, and `enter_fatal` aborts it before it destroys anything else.
The page-level URL owner moved from a `thread_local!` slot — one per wasm
instance, so two instances could each have believed they owned the address bar
— to an attribute on the document element carrying the instance number.

**Restarts are bounded at three.** Each restart evaluates a fresh copy of the
glue at a new URL, and a module record lives as long as the document: the dead
instance's linear memory is not released until the page is. Three is therefore
a bound on what a page can be made to hold, and the fourth attempt offers only
"reload the page".

## The four fault classes, twenty occurrences each

Each was run against an expectation written **before** the run. "Confirmed"
means what the application accepted and reported, read from the page's own
snapshot rather than from a control: a fault may cost a frame, a context or an
answer without costing one of those.

| Fault | Injected by | What it costs | What was true afterwards |
| --- | --- | --- | --- |
| A font that does not arrive | `serve --fault missing:` on the Latin face, one document per occurrence | the glyphs a region would have drawn | the runtime started, both halves moved one value together, and the file was named in the record with what to do about it |
| An answer that arrives after a newer one | two loads started 380 ms apart, answered in reverse | nothing | the newer answer stood, and the confirmed state was the one from before either was asked |
| A scope closed from inside a region action | the application closes its own scope in the callback a click delivered, while the pump is still on the stack | the scope, deliberately | the runtime was alive and unfailed, no region was left, and a new scope took an action and reported it |
| A GPU context lost and given back | `lose_context` / `restore_context` on the region's canvas | the pixels, until the rebuild | every confirmed value was still the application's, the region went `lost` then `ready`, and each loss was recorded once as something to do nothing about |

Evidence: [`p3-faults.spec.ts`](../../../tests/browser/p3-faults.spec.ts),
[M7](../../validation/p3/m7.md).

## Two hours of not failing

R33 asks for two hours at ten actions a second with nothing crashed, nothing
unhandled, and no confirmed state lost or duplicated. B4 — two instances, two
regions each — was driven for 120 minutes by a timer **inside the page**,
because a round trip through the test harness per action would set the rate
rather than measure it.

| Figure | Required | Measured |
| --- | --- | --- |
| Actions sent / accepted | equal | **72,000 / 72,000** (36,000 in each instance) |
| Actions lost, duplicated, refused, errored | 0 | **0** |
| Instances failed | 0 | **0**; four regions alive at the end |
| Latency, head p95 against tail p95 | no worse at the end | **1.5 ms → 1.3 ms**, worst 19.4 ms over 7,200 timed actions |
| Linear memory | flat at the end | +5.96 MB and +5.70 MB, **all before minute 44**; flat for the last 77 minutes |

A lost action and a duplicated one are the same arithmetic from opposite sides,
which is why both are one assertion: the count the application reports must
equal the count the driver sent.

**The warm-up is 5,000 rounds and that is a finding, not a constant.** Every
label these regions draw carries a count, so no string is ever drawn twice, and
the shaper and layouter hold four thousand entries each. A run that starts on
empty caches measures them filling: a ten-minute run rose for six minutes and
was flat for four. Past that point a four-minute run moves **zero bytes**. A
growth assertion made before the working set is walked measures the working
set and calls it a leak.

## Bounded retry, and a GPU that never arrives

A region that cannot get a WebGL2 context asks again at 0, 250 and 750 ms and
then fails for good — under two seconds from first failure to a notice the
person can read, and never a loop. The first two attempts are recorded as
`GpuInitRetry` (informational), the last as `GpuInitFailed`. The DOM half of
the application keeps working throughout.

This doubled the diagnostics a starting region writes, which broke an existing
check that counted entries. The fix was to count by `kind` rather than by
total — a check that asserts a number of entries is asserting how many things
happened, which is rarely what it meant.

## What this release still cannot see

- **A fault in the page's own script before the loader runs.** The notice is
  drawn by the loader; there is nothing to draw it before then.
- **Whether a restarted instance's predecessor released anything.** It did not,
  and that is by design (see above) — but the page cannot observe the dead
  instance to confirm it, because the call boundary answers `InstanceDead`.
  Asking a dead instance a question after a trap is asking one that must fail;
  the instance's diagnostics have to be read **before** the trap, and a check
  written the other way round passes for the wrong reason.
- **A trap inside the browser's own compositor or driver.** Context loss is
  observable and handled; a driver that hangs is not.
- **`document.hidden`.** Two ways of making it true were tried in the headless
  browser — another tab brought to the front, and CDP's
  `Page.setWebLifecycleState` — and neither worked. The element-level hidden
  path is measured; this one is recorded as untested rather than as a pass.
