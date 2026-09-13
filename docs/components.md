# Component catalogue

R18 asks for eighteen categories with no blank support cell. There are twenty:
the table and the tree that large data needed are categories of their own
rather than something R18 named. All twenty have a DOM component; what differs
between them is what a GPU region can draw of one, and every row says which and
why. The table is generated from
`crates/rustify-components/src/catalog.rs` by `cargo xtask catalog --write
docs/components.md`, and `--check` fails when the two disagree.

Every row answers the same six questions:

- **properties** — the values the component takes, and what it does with a
  value the application refuses.
- **actions** — what the user can make it do and what the application hears.
- **theme** — which of the scope's tokens it follows.
- **input** — pointer, keyboard and text input.
- **accessibility** — name, role, value, state, and the keyboard path to the
  same result.
- **environment** — where it has actually been run, and what it needs.

`across regions` says the component keeps one value and one action path across
the DOM/GPU boundary: either both halves draw it, or it can be anchored to a
rectangle inside a GPU region.

`yes` means implemented and covered by a test in this release, `partial` means
implemented with the stated limit, `no` means not in this release. There is no
fourth value: a question nobody answered would be a gap in the catalogue, not a
state a component can be in.

Six categories have no GPU half, and for three different reasons. A tooltip, a
menu and a dialog are DOM layers *anchored to* a region rather than drawn
inside one: a popup that has to escape the canvas cannot be drawn in it. A link
is the odd one out - a region reports a click and the application navigates,
because a region never touches the page's history (C-5). A data table and a
tree are DOM because what they are for is semantics: a row number a screen
reader can say, a cell a keyboard can reach, text an input method can edit
(`docs/data.md`). A region draws the band beside the table, not the table.

<!-- catalogue -->
| category | DOM | GPU | across regions | properties | actions | theme | input | accessibility | environment |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| button | yes | yes | yes | yes — label as children, variant, size, disabled; a disabled button leaves the tab order and calls nothing | yes — click; the DOM and the region button call the same application action | yes — seven variants and four sizes, all drawn from the scope's tokens | yes — pointer and Enter/Space, from either half | yes — a native button; the region's has a DOM control with the same name and action | yes — macOS Chrome, strict CSP, WebGL2 region; no inline script or style |
| label | yes | yes | yes | yes — children as the text, and the id of the control it names | no — a label has none; clicking one moves focus to the control it names | yes — foreground, at the size the scope's text token sets | yes — selectable text in the DOM | yes — names the control it points at; a region's label is decoration and stays out of the tree | yes — macOS Chrome, strict CSP; a region's text needs that region's font |
| link | yes | no | no | yes — href, exact or within matching, disabled; a disabled link keeps its place and loses its href | yes — follows the link, and marks itself aria-current=page while the path is at or below its own | yes — primary and the focus ring | yes — pointer and Enter, plus everything the browser gives a link | yes — a native anchor; the current one says so rather than only looking different | partial — the current path comes from the application until the SDK router arrives in M4; a region reports a click and the application navigates, because a region never touches history (C-5) |
| icon | yes | yes | yes | yes — one of six glyphs, or any SVG the application passes as children; a label makes it an image and no label makes it decoration | no — none; it is a drawing | yes — it takes the text colour around it, in both halves | no — none; an icon-only control is the button around it | yes — aria-hidden without a label and role=img with one - a second name after the control's own is noise | yes — the same six shapes on the same twenty-four unit grid in both halves; no icon font and no fetched asset |
| text field | yes | partial | yes | yes — value, kind, placeholder, disabled, read-only, invalid, described-by; a refused value is replaced by the application's rather than left on screen | yes — input; the application owns the value | yes — the input surface, its border, the focus ring, and the destructive border when invalid | yes — the browser's own control, so IME composition is the platform's | yes — aria-invalid and aria-describedby tie it to its error; a Label names it | partial — the region half displays the value and hands editing to a native control over its rectangle; it is not a GPU text editor |
| text area | yes | partial | yes | yes — value, rows, placeholder, disabled, read-only, invalid, described-by | yes — input; Enter is a newline and commits nothing | yes — the same tokens as the text field | yes — the browser's own control; several lines | yes — the same error wiring as the text field | partial — the region half shows the first line only; editing is the native control's |
| checkbox | yes | yes | yes | yes — checked, disabled, read-only, invalid; read-only takes the click and puts the box back | yes — change; both halves call one application action | yes — the box, its border and the mark, with the focus ring on the box | yes — pointer and Space, from either half | yes — the browser's own checkbox under a drawn box, so the state and the keyboard stay the platform's | yes — macOS Chrome, strict CSP, WebGL2 region |
| radio | yes | yes | yes | yes — the options, which one is chosen, disabled, read-only; an option can be disabled on its own | yes — change carrying the chosen value; a region's radio asks for itself and the application decides what that does to the rest | yes — the disc, its ring and the dot | yes — one tab stop for the group and the arrows within it, which is the browser's own behaviour | yes — role=radiogroup over native radios, each option named by its own label | yes — macOS Chrome, strict CSP, WebGL2 region |
| switch | yes | yes | yes | yes — checked, disabled, read-only; it never flips itself | yes — change; both halves call one application action | yes — the track in two colours and the knob | yes — pointer and Enter/Space | yes — role=switch with aria-checked; the region's has a DOM control with the same name and state | yes — macOS Chrome, strict CSP, WebGL2 region |
| select | yes | partial | yes | yes — value, the options, open, placeholder, disabled, read-only, invalid; an option can be disabled on its own | yes — change and open-change, both the application's to grant | yes — the closed control, the list surface and the chosen row | yes — Enter/Space and the arrows open it; the arrows and Home/End move within it, Enter chooses, Escape closes the top layer only | yes — a combobox with aria-expanded over a listbox of options, with aria-activedescendant saying which is current | partial — experimental in a region: it draws the closed control and reports its rectangle, and the list is a DOM layer anchored to that, because a popup cannot leave the canvas |
| slider | yes | yes | yes | yes — value, min, max, step, disabled, read-only; the value is clamped and snapped before the application sees it | yes — change; both halves call one application action | yes — the track, the filled part and the knob | yes — pointer drag and arrow keys, from either half | yes — a native range control; the region's has a DOM control with the same name and value | yes — macOS Chrome, strict CSP, WebGL2 region |
| progress | yes | yes | yes | yes — value, max, and whether the quantity is known at all | no — none; it is a reading | yes — the track and the filled part | no — none | yes — role=progressbar; with nothing known it carries no aria-valuenow, which says 'in progress' rather than 'nothing done' | partial — the region draws a known fraction; an unknown one is the DOM's, because animating it in a region would keep the page's GPU awake |
| loading | yes | yes | yes | yes — whether it is spinning, and the four states of Load in the SDK's own LoadView: loading, empty, ready, error | yes — one application-defined retry on the error state | yes — the arc takes the text colour around it | yes — the retry control is an ordinary button | yes — role=status with a name, so the state is announced rather than only drawn | yes — a region's arc turns on the pass clock and asks for the next frame only while it is spinning; a scope that asks for less motion gets the same arc held still |
| tooltip | yes | no | yes | yes — open and the text; the application owns whether it is showing | yes — open-change on pointer and on focus, which is what the imported one lacked and what made it invisible to a keyboard | yes — the layer surface and its text | yes — hover and focus both open it; Escape closes the top layer only | yes — role=tooltip, attached to the control it describes rather than to a wrapper no reader announces | partial — a region's tooltip is a DOM layer anchored to a rectangle inside it; a popup drawn in the canvas cannot leave it (F16) |
| menu | yes | no | yes | yes — open, the items, and a reason on any item that cannot run; a command that cannot run stays in the list | yes — activation and open-change; activating an item closes the menu once | yes — the panel surface, its border and the hovered row | yes — the arrows and Home/End move within it, Enter and Space activate, Escape closes the top layer only | yes — role=menu over menuitems, the keyboard lands on the first reachable one, and focus returns to what opened it | partial — anchors to a rectangle inside a region within one CSS pixel, but does not flip or clamp to the viewport |
| dialog | yes | no | yes | yes — open, title, description, modal, and whether the backdrop dismisses it | yes — open-change; Escape closes the top layer only | yes — the panel surface, its border and the backdrop | yes — the keyboard cannot leave a modal one, and clicks do not reach what it covers, region included | yes — role=dialog with aria-modal, labelled by its own title and described by its own description; focus returns on close | partial — a modal covers the scope's regions as well as its DOM and clicks reach neither; only this scope is made inert, and a host page outside it is not |
| tabs | yes | partial | yes | yes — the tabs, which is active, and whether one is disabled; the panels stay in the document so what is in them survives a look elsewhere | yes — activate; the application owns which tab is showing | yes — the strip, the active tab's face and the focus ring | yes — one tab stop for the strip, the arrows and Home/End within it, wrapping and stepping over the unreachable | yes — a tablist over tabs and tabpanels, each panel labelled by its own tab | partial — experimental in a region: the strip is drawn and reports which tab was asked for, and the panel below it is the application's to draw |
| scroll area | yes | yes | yes | yes — what it contains, and whether a wheel arriving at the edge scrolls the page behind it | no — none of its own; the scrolling is the browser's | yes — the scrollbar takes the scope's border colour rather than the browser's | yes — wheel, drag and the keyboard - it takes a tab stop, because a box only a mouse can scroll is unreachable without one | yes — role=group with a name, so a reader knows which box the keyboard landed in | partial — a region's is makepad's own ScrollBars on a View; the boundary a wheel is handed back at is P2 M6 |
| data table | yes | no | partial | yes — how many rows, which columns, a callback per cell and a callback per row identity; it holds no data, so a sorted, filtered or edited view is the same table asked different questions | yes — select a row, open a row, and ask to be shown a row; scrolling is the browser's | yes — border, muted header and the accent a selected row is drawn in | yes — pointer, the arrows, PageUp/PageDown, Home/End and Ctrl+Home/End, Space to select and Enter or F2 to open | yes — role=grid with aria-rowcount over every row, not only the drawn ones; each row carries its real aria-rowindex and its business id, and one cell at a time takes the tab stop | partial — macOS Chrome, strict CSP; no region draws a table - the GPU half of this load is the selection strip beside it - and the row elements are a pool, so a reader meets the rows on screen plus the overscan and reaches the rest by scrolling |
| tree | yes | no | partial | partial — two fixed levels of groups, which are open, and which one is chosen; no arbitrary depth and no virtualisation | yes — open or close a group, and choose one | yes — accent for the chosen group and the scope's focus ring | yes — pointer; the arrows walk what is showing, right opens a group and steps into it, left closes one and steps out of a child | yes — role=tree and role=treeitem with aria-expanded, aria-level and aria-selected; one tab stop for the whole tree | partial — macOS Chrome, strict CSP, no inline script or style; no region draws a tree - a region takes the group a person chose as a projection, like any other value |
<!-- catalogue -->

## Where they live

- The twenty DOM components — `crates/rustify-components/src/`, one file per
  category. The class strings came from Rust/UI; `sources.lock.json` records
  what was imported and what the rewrite changed.
- The GPU halves — `crates/rustify-ui/src/gpu/`. `RustifyButton`,
  `RustifyCheckBox`, `RustifyRadio`, `RustifyToggle`, `RustifySlider`,
  `RustifyProgress`, `RustifySpinner`, `RustifyIcon`, and the two experimental
  ones, `RustifyDropDown` and `RustifyTabBar`. A scrolling region is makepad's
  own `ScrollBars` on a `View`.
- The overlay four stand on the SDK's layer stack —
  `crates/rustify-ui/src/overlay.rs` — which owns anchoring, the Escape order,
  the focus trap and the return of focus.
- P1's own component subset is still where it was
  (`crates/rustify-ui/src/components.rs`) and is unchanged: the two examples
  built on it keep working, and the new crate is where `class` and the
  catalogue contract live.

A runnable example of every category, in each state it has, and with the GPU
half beside it, is `examples/component-catalog` — one page per row of this
table.
