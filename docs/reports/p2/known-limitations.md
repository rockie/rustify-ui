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
- **`math_aot::batch_edges` overflows its stack**, and predates P2. It is a test
  of the fork's script VM in `makepad/platform/script`; P2 changed three files
  under `makepad/`, all in `os/web/`. Checked rather than argued: the pre-P2
  commit built in a `git worktree` fails the same way from a freshly compiled
  binary. `cargo xtask verify --suite p2` reports it as a failed step, which is
  correct - it is a failure, just not a new one.
- **`m8-endurance`'s memory tail assertion fails, and predates P2.** Proved in
  M2 by building the pre-P2 commit in a worktree and re-running the same
  assertion: it failed there too, with a larger margin. The behavioural half of
  that test passes on both.

## Not verified here

- The four manual records (`cargo xtask verify --suite p2` lists them):
  VoiceOver over the catalogue and B1, real pinyin input, the twenty samples
  against a reference rendering, and the 200%/400% reflow walkthrough. The
  contrast *ratios* are a host test; what needs a person is looking at the pages
  at those zoom levels.
- Safari is observed, not gated. macOS Chrome at a fixed version is the gate.
- No performance budget is claimed. A-3 stands: the numbers are baselines beside
  R29/R30, not a pass or a fail.
