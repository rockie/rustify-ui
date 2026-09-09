# M6 report: the component subset, asynchronous state and the theme

Date: 2026-09-09. Machine: macOS (Darwin 25.6.0), Apple Silicon, rustc 1.97.0-nightly (2026-05-19), Node 26.1.0, Playwright 1.63.0 (bundled Chromium, SwiftShader WebGL). Every result below is automated; M6 has no exit condition that needs a person, and no manual Chrome session was run for it. Commands: `cargo xtask doctor`, `cargo xtask sources verify`, `cargo test --workspace --lib`, `cargo test -p xtask`, `cargo clippy --workspace --all-targets -- -D warnings`, `cargo fmt --all -- --check`, `cargo xtask build-web --release` for both examples, `npx playwright test --project=property-workbench`, `npx playwright test --project=fusion-basic`.

## Status

M6's own work is **delivered and passing**: every exit condition in the plan's milestone row has an automated result below, and none of them needs a person.

It is **not recorded as closed**, because M5 is not. M5's two remaining items are a real macOS pinyin session and a VoiceOver + Chrome pass, and §9.4 forbids substituting synthetic input for either. The plan runs its milestones in order, so M6 waits on that rather than on anything of its own. P1 stays at 4/8.

Results on this machine, on the build these numbers were taken from:

| Command | Result |
| --- | --- |
| `cargo xtask doctor` | 9/9 (the ninth is the new `vendored` check) |
| `cargo xtask sources verify` | makepad drift attributable as before; nouislider 3 files, 0 modified |
| `cargo test --workspace --lib` | 43 (registry 3, binding 9, catalog 7, components 4, overlay 3, scheduler 5, task 5, theme 7) |
| `cargo test -p xtask` | 10 |
| `cd makepad && cargo test` | 13 |
| `cargo clippy --workspace --all-targets -- -D warnings` | no warnings |
| `cargo fmt --all -- --check` | passes |
| `cargo xtask build-web --release`, both examples | passes |
| `npx playwright test --project=property-workbench` | **54/54** (m3-workbench 19, m5-text 8, m5-semantics 5, m6-theme 3, m6-async 4, m6-components 15) |
| `npx playwright test --project=fusion-basic` | **45/45** (m6-mainpath 3 new) |

## What is delivered

### A catalogue that answers for all eighteen categories

`crates/rustify-ui/src/catalog.rs` holds one table: the eighteen categories R18 names, each with three presentation columns (DOM, GPU, across regions) and the six capability classes §6.3 asks for - properties, actions, theme, input, accessibility, environment. Every cell carries a level and a reason; the type has no `Unknown`, because a question nobody answered is a gap in the catalogue rather than a state a component can be in.

Nine categories are shipped in P1: button, label, text field, text area, checkbox, slider, loading, menu, dialog. The other nine - link, icon, radio, switch, select, progress, tooltip, tabs, scroll area - are in the table saying `no` in every column, with the same one-line reason: they are P2 M2's work. `docs/components.md` prints the same table and a host test fails if the two disagree.

| Rule | Evidence |
| --- | --- |
| Eighteen categories, each exactly once | `every_one_of_the_eighteen_categories_appears_exactly_once` |
| No blank cell; every answer carries its reason | `no_cell_is_blank_and_every_answer_carries_its_reason`, `every_row_is_printed_with_ten_filled_cells` |
| The nine that are not here say so in every column | `the_nine_this_release_does_not_ship_say_so_in_every_column` |
| The document and the code hold the same table | `the_capability_document_holds_the_table_the_code_holds` |
| The catalogue is on the page, not only in the source | `the catalogue answers for every category R18 names` |

### The component subset

`crates/rustify-ui/src/components.rs` (DOM) and `crates/rustify-ui/src/gpu.rs` (the GPU halves).

Every control is controlled in the strict sense: what the user does is a request, and what the control shows afterwards is the application's answer. A DOM control puts itself back in step with the application after every keystroke, so a refused value is not left on screen as if it had been taken. A GPU control never flips or moves its own value either; it reports the request and draws what the projection brings back. Disabled and read-only controls make no request at all - there is nothing to refuse later because nothing was asked.

`snap` is one rule, used by the DOM slider and the GPU slider, so a drag on one cannot produce a value the other could not.

| Rule | Evidence |
| --- | --- |
| A refused value is replaced by the value in force, with a reason | `the field goes back to the value in force and says why` |
| A failure offers the application's own retry, and it asks again | `the retry the application defined asks the same question again` |
| A read-only control is reachable, focusable and unchanged by typing | `a read-only field is reachable and unchanged by typing` |
| A business rule (locked) makes a field read-only in both halves | `locking an object makes its name read-only in both halves` |
| Disabled controls change nothing and take no keyboard | `a disabled control neither changes a value nor takes the keyboard` |
| One value behind the DOM slider and the region | `the DOM slider and the region show the object's own value` |
| The GPU slider snaps to the same steps and stays inside the range | `the region's own slider asks for a value on the same steps` |
| The GPU checkbox asks and the application answers; the DOM checkbox follows | `the region's own checkbox asks, and the application answers` |
| The value rules hold without a browser | `a_value_outside_the_range_comes_back_inside_it`, `a_value_between_two_steps_lands_on_one_of_them`, `a_range_with_no_step_only_clamps`, `a_range_given_backwards_is_still_a_range` |

The GPU controls report where they ended up (`RustifyCheckBox::drawn`, `RustifySlider::drawn`) from the drawn area rather than the walked rectangle: a parent that aligns its children moves them after the walk, and what a pointer has to be aimed at is where they ended up. The workbench projects that geometry back out, so a test drives the control where the region says it is instead of keeping a second copy of the layout.

### Four states and a retry

`crates/rustify-ui/src/task.rs` and the `LoadView` component. `Empty` is an answer rather than `Ready(None)`, the retry is the application's (only it knows what was being asked), and it sits beside the status rather than inside it so a reader of the status hears the state and not a button's label.

Evidence: `each of the four states says what it is`, `an answer that arrives after a newer one does not undo it`, `twenty pairs answered backwards all end on the newer answer`, `an answer for a scope that has gone reaches nothing`, `the retry the application defined asks the same question again`, plus the host tests `a_hundred_pairs_answered_backwards_all_end_on_the_newer_answer` and `nothing_delivers_into_a_view_that_has_gone` (a hundred tickets after the close, none delivered).

### One theme, and a local override

`crates/rustify-ui/src/theme.rs`. The token table now carries the metrics §6.3 asks for as well as the colours: text size, spacing, corner radius, and the reduced-motion preference as a transition duration of zero - the result still arrives, it just does not travel.

`ThemeOverride` changes part of a theme for one area. It writes only the properties the patch names, onto its own element, and removes the ones it stops naming: an override that cannot be turned off is not local, it is permanent. Everything unnamed inherits, because custom properties inherit.

Children of an override also get the patched table through `use_theme_values`, so a region rendered inside one would draw with the numbers its stylesheet uses. Neither example renders a region inside an override, so that half of the mechanism is written and compiled but not exercised; it is listed under the known limits rather than claimed.

| Rule | Evidence |
| --- | --- |
| A switch moves the panel and the region together | `switching the theme moves the DOM and the GPU together` |
| Twenty switches return to the start; the host page never carries a theme | `the host page keeps its own appearance across twenty switches` |
| Twenty override rounds change one area and nothing around it | `twenty rounds change one area and nothing around it` |
| A second scope on the page keeps its own theme and its own override | `another scope on the page keeps its own theme` |
| A theme is values, not a different set of controls: thirty switches, ten controls found by role and name each time, then the pointer path and the DOM path still agree | `thirty switches leave every control where a name can find it` |
| The patch rules hold without a browser | `an_override_writes_only_what_it_names`, `a_patched_theme_keeps_everything_the_patch_did_not_name`, `every_property_a_patch_can_write_is_one_it_can_also_take_back`, `less_movement_is_no_movement_rather_than_no_result` |

### One fixed-version third-party DOM component

noUiSlider 15.8.1, vendored into `examples/property-workbench/vendor/nouislider/` with its licence, recorded in `sources.lock.json` with a digest per file, checked by `cargo xtask sources verify` and listed by `cargo xtask doctor`. The build copies it; nothing fetches it. It is bound to the same application value as the SDK's own slider.

| Rule | Evidence |
| --- | --- |
| Twenty rebuilds leave one component in the document and one in the shim, created once more than destroyed each round | `each rebuild leaves one component and one subscription` |
| One live component means one callback for one change - a leaked subscription would report two | same test: a nudge from outside the application raises the callback count by exactly one |
| Focus is not taken across a rebuild | same test: the focused control is unchanged after each of the twenty rounds |
| The value it reports becomes the application's, and the SDK's slider shows it | same test |
| The version cannot drift silently | `cargo xtask sources verify` fails on a modified vendored file |

The page deliberately shows two sliders for one value - the SDK's and this one - because what is being checked is that a component the SDK did not write can be rebuilt beside a region without leaving anything behind. A real application would show one. The wrapper carries the name, so the component's own handle is not a nameless second slider in the accessibility tree.

### Select, edit, confirm, in both examples

| Step | property-workbench | fusion-basic |
| --- | --- | --- |
| Select | a real click on a grid cell in the region picks an object | a real click on an anchor the region reported selects it, and the region rings it |
| Edit | the region hands its name rectangle to a real text control | the menu anchored to that anchor opens a modal over the colour |
| Confirm | Enter commits the value and ends the session | "apply" commits the colour and closes the modal |
| Abandon | Escape leaves the value untouched | "close" leaves the colour untouched |
| Refused | the commit ends the session, the old name stays in both halves, the panel says why | the modal stays open, the old colour stays, the modal says why |

Evidence: `the main path from a pointer on the region to a committed value` and `a confirmed value the application refuses leaves the old one in both halves` (workbench); `the main path from a pointer on the region to a committed value`, `an abandoned edit changes nothing` and `a value the application refuses leaves the old one and says why` (fusion-basic).

## Three defects found and fixed on the way

- The workbench region drew a notes value it never updated: `apply_props` set the name and the position but not the notes label, and the helper that would have done it was dead code. The region showed "no notes" whatever the object held.
- The page-level seams were single slots. A second scope on the page took each slot, and its teardown left the page with nothing even though the first scope was still mounted. They are now one entry per mounted scope, withdrawn by the scope that made it - the same lesson as M5's, applied to every seam rather than one of them.
- A widget reported the rectangle the layout walked rather than the one it ended up in. A parent with `align: Center` moves its children after the walk, so a pointer aimed at the reported position missed by the width of the centring - a report that is wrong in exactly the way that makes it useless. The row is now left-aligned as well, so a value that grows no longer moves the controls beside it.

## Known limits

- The eighteen-category catalogue lists nine categories it does not implement. Nothing here should be read as a claim about them.
- The GPU text field displays a value and hands editing to a native control over its rectangle. It is not a GPU text editor, and the catalogue says so.
- The GPU half of a menu, a dialog and a loading view is `no`: they are DOM layers, which is what lets them cross a region.
- The reduced-motion token is published and read, but no animation in either example is long enough for a person to see the difference.
- No GPU region is rendered inside a `ThemeOverride` in either example, so "a region inside an override draws with the patched values" is unverified. What is verified is that an override changes its own area and nothing else - not the panel, not the scope root, not the region beside it, not the host page, and not a second scope.
