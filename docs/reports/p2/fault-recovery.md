# P2 fault and recovery report

Every failure this release exercises against the running product, what the
product does about it, and the ones it cannot see. Nothing here is simulated by
doctoring a copy: the faults are injected into the deployment the tests are
running against.

## Deployment and build faults

| Fault | Injected by | What happens | Evidence |
| --- | --- | --- | --- |
| A file of the deployment is missing | `serve --fault missing:<path>` | The region reports `AssetLoadFailed` naming the asset; the application keeps running | M7 (P1), carried forward |
| A font arrives corrupt or truncated | `serve --fault corrupt:` / `truncated:` | Glyphs are lost, not the application | `m7-deployment.spec.ts` |
| The JS and the wasm are from different builds | `serve --fault stale-bridge` | `BuildContractMismatch` and a visible start-up notice; nothing half-works | `m7-deployment.spec.ts` |
| The build is served under a base it was not built for | `serve --base` mismatch | Refused at start-up with what to rebuild | `xtask` |

## Runtime faults

| Fault | What happens | Evidence |
| --- | --- | --- |
| WebGL2 unavailable at start | `GpuInitFailed`; the DOM half of the page is untouched | P1 M2 |
| A running region loses its GPU context | `GpuContextLost`; the region is rebuilt and the application's state projected into it again | `m7-recovery.spec.ts` |
| The shared wasm traps | `RuntimeFatal`; every scope on the page is dead and the loader says so rather than leaving a frozen page | P1 M7 |
| A scope's queue fills | `Backpressure`; the action did not run and is reported as not executed, never silently dropped | P1 M3 |
| Something arrives for a disposed scope, region or request | `Disposed`; no callback runs | P1 M2 |

## Refusals that are not faults

These are the product working. They are here because each one is a place where
a quieter implementation would have looked identical and been wrong.

| Refusal | What the product does |
| --- | --- |
| A second scope asks to own the page's URL | `UrlOwnerConflict`: the asking scope is not mounted, the owner is untouched |
| A guard refuses a navigation | Recorded as `NavigationBlocked` (informational), and the address bar and the view agree about where the user is |
| A guard refuses a move through history and cannot undo it | `NavigationRestoreFailed` - an error, because the user is now somewhere they did not choose |
| A form is told about a field it has no such thing | `UnknownField`: a misspelled name is a control that never validates, and that does not look like a typo from outside |
| The clipboard is refused | `ClipboardError::Denied`, the manual path is offered, and no success is reported |
| A file is too large or of a kind we do not read | Refused from the name and the size, before a byte is read |
| A file parses wrong | Nothing is applied - not even the lines before the bad one |
| A picker is dismissed | `Aborted`: not an error, and no import action |
| A drag is released over nothing, or over a target that refuses | No drop, and the count says so |
| A region's queue is asked for two continuous streams at once | Both are delivered; they supersede themselves and not each other (M6) |

## What the product cannot see

- A font that is present, valid, and simply has no glyph for the text. The
  region draws whatever that font produces for an absent glyph. The
  missing-glyph mark is for a font that *fails to arrive*, which is a different
  thing and is reported.
- A browser extension that mutates the page's DOM inside a scope.
- Whether a download the browser started actually reached the disk. The product
  hands the bytes over; what the browser does after that is outside it.
