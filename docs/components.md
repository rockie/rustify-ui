# Component catalogue (P1)

R18 asks for eighteen categories with no blank support cell. P1 ships nine of
them and lists the other nine here as well, saying plainly that they are not in
this release. The table is generated from `crates/rustify-ui/src/catalog.rs`; a
host test fails if this file and that table disagree.

Every row answers the same six questions:

- **properties** — the values the component takes, and what it does with a
  value the application refuses.
- **actions** — what the user can make it do and what the application hears.
- **theme** — which of the scope's tokens it follows.
- **input** — pointer, keyboard and text input.
- **accessibility** — name, role, value, state, and the keyboard path to the
  same result.
- **environment** — where it has actually been run.

`across regions` says the component keeps one value and one action path across
the DOM/GPU boundary: either both halves draw it, or it can be anchored to a
rectangle inside a GPU region.

`yes` means implemented and covered by a test in this release, `partial` means
implemented with the stated limit, `no` means not in this release. There is no
fourth value: a question nobody answered would be a gap in the catalogue, not a
state a component can be in.

<!-- catalogue -->
| category | DOM | GPU | across regions | properties | actions | theme | input | accessibility | environment |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| button | yes | yes | yes | yes — label, disabled; a disabled button emits nothing | yes — click; the DOM and the GPU button call the same application action | yes — scope tokens: primary, foreground, radius | yes — pointer and Enter/Space, from either half | yes — native button in the DOM; the GPU button has a DOM control with the same name and action | yes — macOS Chrome, strict CSP, WebGL2 region; no inline script or style |
| label | yes | yes | yes | yes — text; optional association with a control | no — a label has none; clicking one focuses its control | yes — scope tokens: foreground, muted | yes — selectable text in the DOM | yes — names the control it wraps; GPU text is decoration and stays out of the tree | yes — macOS Chrome, strict CSP; GPU text needs the region's font |
| link | no | no | no | no — not in P1; the eighteen-category catalogue is P2 M2 | no — not in P1; the eighteen-category catalogue is P2 M2 | no — not in P1; the eighteen-category catalogue is P2 M2 | no — not in P1; the eighteen-category catalogue is P2 M2 | no — not in P1; the eighteen-category catalogue is P2 M2 | no — not in P1; the eighteen-category catalogue is P2 M2 |
| icon | no | no | no | no — not in P1; the eighteen-category catalogue is P2 M2 | no — not in P1; the eighteen-category catalogue is P2 M2 | no — not in P1; the eighteen-category catalogue is P2 M2 | no — not in P1; the eighteen-category catalogue is P2 M2 | no — not in P1; the eighteen-category catalogue is P2 M2 | no — not in P1; the eighteen-category catalogue is P2 M2 |
| text field | yes | partial | yes | yes — value, disabled, read-only; a refused value is shown as the application's, not the keystrokes' | yes — input and commit; the application owns the value | yes — scope tokens: input, border, ring, radius | yes — the browser's own control, so IME composition is the platform's | yes — native input with a label; five keyboard journeys | partial — the GPU half displays the value and hands editing to a native control over its rectangle; it is not a GPU text editor |
| text area | yes | partial | yes | yes — value, rows, disabled, read-only | yes — input and commit on leaving; Enter is a newline, not a commit | yes — scope tokens: input, border, ring, radius | yes — the browser's own control; several lines | yes — native textarea with a label | partial — the GPU half shows the first line only; editing is the native control's |
| checkbox | yes | yes | yes | yes — checked, disabled, read-only; read-only reverts the browser's own toggle | yes — change; both halves call one application action | yes — scope tokens: primary, border, ring | yes — pointer and Space, from either half | yes — native checkbox in the DOM; the GPU one has a DOM control with the same name, state and action | yes — macOS Chrome, strict CSP, WebGL2 region |
| radio | no | no | no | no — not in P1; the eighteen-category catalogue is P2 M2 | no — not in P1; the eighteen-category catalogue is P2 M2 | no — not in P1; the eighteen-category catalogue is P2 M2 | no — not in P1; the eighteen-category catalogue is P2 M2 | no — not in P1; the eighteen-category catalogue is P2 M2 | no — not in P1; the eighteen-category catalogue is P2 M2 |
| switch | no | no | no | no — not in P1; the eighteen-category catalogue is P2 M2 | no — not in P1; the eighteen-category catalogue is P2 M2 | no — not in P1; the eighteen-category catalogue is P2 M2 | no — not in P1; the eighteen-category catalogue is P2 M2 | no — not in P1; the eighteen-category catalogue is P2 M2 | no — not in P1; the eighteen-category catalogue is P2 M2 |
| select | no | no | no | no — not in P1; the eighteen-category catalogue is P2 M2 | no — not in P1; the eighteen-category catalogue is P2 M2 | no — not in P1; the eighteen-category catalogue is P2 M2 | no — not in P1; the eighteen-category catalogue is P2 M2 | no — not in P1; the eighteen-category catalogue is P2 M2 | no — not in P1; the eighteen-category catalogue is P2 M2 |
| slider | yes | yes | yes | yes — value, min, max, step, disabled, read-only; the value is clamped and snapped before the application sees it | yes — change; both halves call one application action | yes — scope tokens: primary, border, ring | yes — pointer drag and arrow keys, from either half | yes — native range in the DOM; the GPU one has a DOM control with the same name and value | yes — macOS Chrome, strict CSP, WebGL2 region |
| progress | no | no | no | no — not in P1; the eighteen-category catalogue is P2 M2 | no — not in P1; the eighteen-category catalogue is P2 M2 | no — not in P1; the eighteen-category catalogue is P2 M2 | no — not in P1; the eighteen-category catalogue is P2 M2 | no — not in P1; the eighteen-category catalogue is P2 M2 | no — not in P1; the eighteen-category catalogue is P2 M2 |
| loading | yes | no | yes | yes — the four states of `Load`: loading, empty, ready, error; empty is an answer, not a missing one | yes — one application-defined retry on the error state | yes — scope tokens: muted, destructive | yes — the retry control is an ordinary button | yes — the state is a live status; the error is an alert with the retry beside it | yes — a region can be covered by one, but the view itself is DOM |
| tooltip | no | no | no | no — not in P1; the eighteen-category catalogue is P2 M2 | no — not in P1; the eighteen-category catalogue is P2 M2 | no — not in P1; the eighteen-category catalogue is P2 M2 | no — not in P1; the eighteen-category catalogue is P2 M2 | no — not in P1; the eighteen-category catalogue is P2 M2 | no — not in P1; the eighteen-category catalogue is P2 M2 |
| menu | yes | no | yes | yes — anchor, items, open state owned by the application | yes — item activation and close; Escape closes the top layer only | yes — scope tokens: background, border, radius | yes — pointer and keyboard; the layer owns Escape while it is open | yes — menu/menuitem roles; focus returns to the control it was opened from | partial — anchors to a rectangle inside a GPU region within one CSS pixel, but does not flip or clamp to the viewport |
| dialog | yes | no | yes | yes — modal, anchor, open state owned by the application | yes — close; Escape closes the top layer only | yes — scope tokens: background, border, radius | yes — clicks do not reach what a modal covers, region included | yes — the scope's own content is inert while a modal is open; focus returns on close | partial — only this scope is made inert; a host page outside it is not |
| tabs | no | no | no | no — not in P1; the eighteen-category catalogue is P2 M2 | no — not in P1; the eighteen-category catalogue is P2 M2 | no — not in P1; the eighteen-category catalogue is P2 M2 | no — not in P1; the eighteen-category catalogue is P2 M2 | no — not in P1; the eighteen-category catalogue is P2 M2 | no — not in P1; the eighteen-category catalogue is P2 M2 |
| scroll area | no | no | no | no — not in P1; the eighteen-category catalogue is P2 M2 | no — not in P1; the eighteen-category catalogue is P2 M2 | no — not in P1; the eighteen-category catalogue is P2 M2 | no — not in P1; the eighteen-category catalogue is P2 M2 | no — not in P1; the eighteen-category catalogue is P2 M2 | no — not in P1; the eighteen-category catalogue is P2 M2 |
<!-- catalogue -->

## What "not in P1" means

The nine absent categories — link, icon, radio, switch, select, progress,
tooltip, tabs, scroll area — have no implementation, no example and no test
here. They are the subject of P2 M2, which is where the eighteen-category
catalogue with a runnable example per category is delivered. Nothing in P1
should be read as a claim about them.

## Where the nine live

- `Button`, `Label`, `TextField`, `TextArea`, `Checkbox`, `Slider`, `LoadView` —
  `crates/rustify-ui/src/components.rs`.
- `Menu` and `Dialog` — `Layer` in `crates/rustify-ui/src/overlay.rs`; a menu is
  a layer anchored to a rectangle, a dialog is a modal layer.
- The GPU halves of button, label, checkbox and slider are Makepad widgets
  driven by the same application value; see
  `examples/property-workbench/src/object_region.rs`.

## The one third-party component

P1 also verifies one component the SDK did not write: noUiSlider 15.8.1,
vendored into the checkout. It is not in the table above, because the table is
about what this SDK offers. What it is for, and what is and is not claimed about
third-party components in general, is in `docs/compatibility.md`.
