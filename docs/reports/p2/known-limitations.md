# P2 known limitations

Everything not built, not verified, or built in a way that should not be read as
general. A limitation is here whether it was a decision, a discovery or a
shortage of time, and which of those it was is said plainly.

## Decided against, and not built

| Not provided | Why |
| --- | --- |
| Nested routing, route-level authorisation, server redirects, form actions | The SDK owns one URL per page and matches static segments and `:param`. Anything more is a router, and a router was not what this release needed (ADR-6). |
| `leptos_router` compatibility | The SDK provides none of its contexts, so a third-party component calling `leptos_router::hooks::*` panics for want of one. |
| An arbitrary dock tree, layout in the URL, layout persistence | Three panels with declared minimums is what B1 asks for. A dock tree is a different product. |
| Dragging out to the operating system, pen pressure, touch gestures | Excluded by R-5. The SDK's drag is a pointer session; HTML5 drag and drop is used only for files coming *in*. |
| Plural rules, message interpolation, locale-aware collation | Two languages of the SDK's own words, and `Intl` for numbers and dates. An application needing more should use a message library. |
| A right-to-left interface language | `Direction` exists and the components use logical properties, so adding one is a catalogue rather than a rewrite. None ships, and nothing here claims one was tested. |

## Built, with a boundary worth knowing

- **The clipboard is a property of the build.** `navigator.clipboard` is behind
  `--cfg=web_sys_unstable_apis`; `cargo xtask build-web` sets it, and a build
  made another way answers `Unavailable` rather than failing. See
  `docs/compatibility.md`.
- **A refusal is never dressed up.** Every clipboard rejection arrives as
  `Denied` and the component offers the manual path. Nothing reports a success
  it did not have.
- **The shipped fonts have no Arabic or Hebrew.** The theme's wide family covers
  Latin, Chinese and emoji. The B5 sample page draws right-to-left text so that
  the gap is visible rather than hidden.
- **A GPU region cannot be a component's child.** Its application marker is not
  `Send`, and anything passed through a Leptos component's children must be. The
  splitter draws its dividers over a row the application lays out; the samples
  page holds its own region and borrows only the classes. Found twice, written
  down in `docs/workspace.md`.
- **One region owns a full Makepad `Cx`.** Carried over from P1: script VM,
  theme and font atlas per region, and wasm memory never shrinks.
- **The script VM's interpreter has to be built optimised.** `makepad-stitch`
  is threaded code: every instruction is a sibling call to the next, and only an
  optimising build turns those into jumps. `makepad/platform/script/Cargo.toml`
  asks for `opt-level = 1` on that package alone; without it any wasm loop of a
  few thousand trips overflows the thread stack, and the abort names whichever
  test reached it first. (This is what `math_aot::batch_edges` was.)
- **A region's text caches fill before they settle.** The shaper and the
  layouter hold 4,096 entries each, so a workload that keeps drawing strings it
  has not drawn before keeps allocating - about 3 KB per new string in B1 - until
  it has seen its working set. It is bounded and it is not a leak: walking the
  same thousand objects a second time costs 304 bytes an action, and a third
  time the same. Any endurance measurement has to walk the set once before it
  measures anything.
- **Script-evaluation garbage is collected by the amount, not the clock.** A
  props application that sets a shader value leaves four blocks behind, and the
  runtime collects at a slack of 20,000 objects. A busy region's memory
  sawtooths within that slack; an idle one never collects at all.

## Not verified here

- VoiceOver: the user reported the dialog walkthrough passed on 2026-09-11
  and waived the remaining catalogue and B1 checks. Those remain untested;
  this does not establish full screen-reader support.
- Pinyin, text samples, and contrast and zoom have overall user-reported
  passes dated 2026-09-11. Individual input strings, reference images, per-sample
  DOM/GPU comparisons and per-zoom/theme observations were not supplied. These
  records close the P2 manual acceptance disposition; they are not independent
  evidence that the Arabic/Hebrew font gap is fixed or the full matrix passed.
- Safari is observed, not gated. macOS Chrome at a fixed version is the gate.
- The performance gate is one machine and one browser. R29 and R30's B1 figures
  became a pass/fail on 2026-09-11 (A-3 closed) and this build passes all four -
  on macOS Chrome under SwiftShader, which is not a performance machine. Nothing
  is claimed for any other combination, for B0, or for R30's AC2/AC3, whose B2
  and B3 loads P2 does not build.
- A reload of the workbench in place can cost about ten seconds in this harness
  before the new document is parsed. Measured, and not the application's: its own
  teardown is 7-17 ms. See the performance report.
