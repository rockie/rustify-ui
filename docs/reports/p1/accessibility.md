# P1 accessibility report

What a person using the keyboard, a screen reader or a magnified view gets from
this preview, and what has not been checked. Evidence is in
[M4](../../validation/p1/m4-geometry.md),
[M5](../../validation/p1/m5-text-and-semantics.md) and
[M6](../../validation/p1/m6-components-async-theme.md).

**This is not a WCAG 2.2 AA conformance statement.** The plan's §9.4 limits P1
to a walkthrough of the two examples against the numbers R35 names. Nothing
here is an audit, and the preview's own component subset is nine of eighteen
categories.

## What is verified without a person

| Rule | Result | Evidence |
| --- | --- | --- |
| Every meaningful control has one entry in the accessibility tree | Twenty controls found by role and name, none of them twice | M5 |
| A region's pixels are decoration, not a mute second copy | The canvas carries `aria-hidden`; every value it draws is reachable in the panel | M5 |
| The pointer path and the DOM path reach the same object | A real click on a grid cell and a "go to object" by name select the same object | M5 |
| A lookup answers within five seconds | `found` and `disposed` are immediate and final; a wait for an absent id ends at its deadline as `timeout` | M5 |
| Five journeys with the keyboard alone | Select, edit, run a command, read the result, undo the selection | M5 |
| Tab order | Nineteen stops forward and backward over twenty items; the disabled item is in neither direction; the read-only item is reachable and unchanged by typing | M4 |
| Focus is not taken | Mounting a scope and starting its region take no focus from the host page; a layer that no longer holds focus does not take it back | M4 |
| Focus returns | A layer records what had focus when it opened and gives it back on close, or focuses the scope when that element has gone | M4 |
| Escape belongs to the top layer | Two layers: Escape closes the top one only, and the application's own Escape command runs only when the stack is empty | M4 |
| A modal does not leak clicks | A hundred clicks on a modal over a GPU control produce zero actions underneath | M4 |
| Editing is the browser's own control | The caret, selection, system undo and the input method belong to a real `input`/`textarea` over the rectangle the region drew, not to a redrawing of them | M5 |
| Composition is not interrupted | Enter and Escape during composition belong to the composition; one composition produces one value | M5 |
| A status is a status, not a button | `LoadView` puts the application's retry beside the state rather than inside it, so a reader hears the state and not a button's label | M6 |
| A theme is values, not different controls | Thirty theme switches, ten controls found by role and name each time; the pointer path and the DOM path still agree afterwards | M6 |
| Reduced motion | The token is published as a transition duration of zero: the result still arrives, it just does not travel | M6 |
| A third-party component keeps its name | The wrapper carries the label, so the vendored slider is not a nameless second control in the tree | M6 |

## What needs a person, and has not been done

These four are the manual matrix. `cargo xtask verify --suite p1` looks for
each file and reports it as not handed in; P1 cannot be called complete while
any is missing.

| Record | What it must contain | File |
| --- | --- | --- |
| Real macOS pinyin | Twenty Chinese phrases through a real input method in Chrome: no lost or duplicated characters, no command conflict, and the committed value drawn correctly in both halves | `docs/validation/p1/manual/pinyin.md` |
| VoiceOver + Chrome | The five keyboard journeys: announced name, role, value and state; no duplicate announcements; no focus trap | `docs/validation/p1/manual/voiceover.md` |
| Contrast and reflow | Ordinary text 4.5:1, large text 3:1, necessary non-text information 3:1; 200% text zoom and 400% reflow of what applies, over both examples | `docs/validation/p1/manual/contrast-and-zoom.md` |
| Safari observation | The same build's main path in macOS Safari — recorded, never blocking | `docs/validation/p1/manual/safari.md` |

A record needs the device, OS, browser and assistive-technology versions, the
build id, the steps, the expectation, the result, and where the evidence is.
Synthetic events are not a substitute for any of them and are not recorded as a
pass.

## Known limits

- A layer is placed under its anchor and is never flipped or clamped to the viewport.
- A modal makes its own scope `inert`; it does not trap focus against the browser's own UI or against the host page outside the scope.
- The zoom matrix is exercised as a device pixel ratio with a correspondingly scaled CSS viewport, which is what browser zoom does to a page — not as a person operating the zoom control.
- Nine of the eighteen component categories are absent, so nothing here says anything about their semantics.
- A GPU region draws text; it is not a text editor. Editing is always a native control over the region's rectangle.
