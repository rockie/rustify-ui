---
description: Forms as a state machine for field errors, validation, saving and unsaved changes, while the application keeps the values.
---
# Forms

A form here is a state machine, not a store. It holds what is wrong with each
field, whether a validation is running, whether a save is in flight and whether
anything has changed - and it deliberately does **not** hold the values.

## Choose the public entry point

The current source checkout provides both unstyled parts in
`rustify_components::primitives` and themed parts in the matching top-level
modules. They use the same controlled behavior. These additions are not part
of the published 0.2.1 API; use matching source dependencies and styles as
explained in the [migration guide](dom-components-migration.md).

| Parts namespace | Composition |
| --- | --- |
| `checkbox`, `switch`, `radio` | Root, Control, Indicator or Thumb; Checkbox uses `CheckedState::Unchecked`, `Checked`, `Indeterminate` |
| `checkbox_group`, `radio_group`, `toggle_group` | Root and Item; selection order and keyboard movement follow declared items |
| `toggle` | Root is the native button |
| `slider` | Root, Track, Range and stable, named Thumb parts |
| `text_field`, `text_area` | TextField Root/Input/Slot; TextArea Root is the native textarea |
| `number_field`, `color_field` | Root/Input and optional colour Swatch; focused input uses Draft |
| `select` | Root/Trigger/Value/Icon, Content/Viewport, Group/Label, Item/ItemText/ItemIndicator, Separator and scroll buttons |
| `form` | Root, Field, Label, Description and Message |

Themed `checkbox_cards` and `radio_cards` use the same group behavior with
label/description card content. The existing `Checkbox`, `Switch`,
`RadioGroup`, `Select`, `Slider`, `TextField` and other convenience exports
assemble the corresponding parts.

For a page with unstyled parts, write `rustify_components::PRIMITIVES_CSS` and
`rustify_ui::runtime::DOM_RUNTIME_CSS` to separate external stylesheets. Do not
also load `rustify_components::CSS` on that page. Give custom sliders a visible
track and thumb size in your own CSS; the structural stylesheet deliberately
supplies no visual dimensions, brand palette, fonts or elevation. Public
`data-rustify-part` and state attributes can be styled without depending on
private wrapper depth.

Themed consumers load `rustify_components::CSS`, or import `sdk.css` into the
application's one Tailwind build. The themed stylesheet already includes the
structural rules. [Quick start](quickstart.md#using-tailwind-v4-in-your-app)
contains the build and asset recipe. ThemeScope supplies configured theme
geometry; component classes do not load fonts or styles automatically.

## A native form made from parts

This example saves only to local component state. The application owns `title`
and explicitly completes each save. Run it inside an SDK mount as described in
[the DOM-only recipe](quickstart.md#a-dom-application-without-gpu-regions).

```rust,ignore
use leptos::prelude::*;
use rustify_components::{primitives as p, Form};

#[component]
fn LocalForm() -> impl IntoView {
    let title = RwSignal::new(String::new());
    let saved = RwSignal::new(false);
    let completion = StoredValue::new(None::<Form>);
    let form = Form::new(&["title"], move || {
        saved.set(true);
        if let Some(form) = completion.get_value() {
            form.submitted(Ok(()));
        }
    });
    completion.set_value(Some(form));
    view! {
        <p::form::Root form=form rules=Vec::new
            on_reset=Callback::new(move |_| {
                title.set(String::new());
                saved.set(false);
                form.reset();
            })>
            <p::form::Field form=form field="title">
                <p::form::Label>"Title"</p::form::Label>
                <p::form::Description>"Required"</p::form::Description>
                <p::text_field::Root value=title name="title" required=true
                    on_change=move |next| {
                        title.set(next);
                        form.changed("title");
                    }>
                    <p::text_field::Input />
                </p::text_field::Root>
                <p::form::Message />
            </p::form::Field>
            <button type="submit">"Save locally"</button>
            <button type="reset">"Reset"</button>
            <output>{move || if saved.get() { "Saved locally" } else { "" }}</output>
        </p::form::Root>
    }
}
```

For a themed version use `FormRoot`, `form::field::{Root, Label, Description,
Message}` and `text_field::{Root, Input, Slot}`. The themed input convenience
`TextField` is also valid inside the same Field. Keep the application's
signals, save callback and `form.changed` calls unchanged.

Root renders one native form and handles real submit/reset events. It flushes
registered Draft/composition participants, checks the form's actual native
controls and then calls the existing form owner. Native required controls
outside a Field, including controls linked with `form=`, still block before
rules or save callbacks. Nested FormRoot is diagnosed and does not create a
second native form. The `on_reset` callback changes application values;
FormRoot prevents the browser from silently restoring a second set of defaults.
Reset is refused while saving.

Inside FormRoot, SubmitButton is a native submit button and Root owns rules.
Outside it, the existing direct SubmitButton path still calls the owner. Use
native `name`, `form`, `required`, `disabled` and the controls' accepted values
for FormData. No name means no submitted key. Anonymous radio groups retain
independent identity without adding internal names to FormData; explicitly
named groups in different forms may reuse a name.

## Select metadata and independent content

The entries exist while Content is closed, so SSR, Value and native form
submission do not depend on mounting the popup. `text_value` is also the search
text; rich Item children and group labels do not replace it.

```rust,ignore
use leptos::prelude::*;
use rustify_components::primitives::select as s;

#[component]
fn Fruit() -> impl IntoView {
    let value = RwSignal::new(None::<String>);
    let open = RwSignal::new(false);
    view! {
        <s::Root value=value on_change={move |next: Option<String>| value.set(next)}
            open=open on_open_change=move |next| open.set(next)
            entries=vec![s::SelectEntry::new("a", "Apricot"),
                         s::SelectEntry::new("b", "Banana")]
            name="fruit" aria_label="Fruit">
            <s::Trigger><s::Value placeholder="Choose fruit" /><s::Icon>"▾"</s::Icon></s::Trigger>
            <s::Content><s::Viewport>
                <s::Group><s::Label>"Fruit"</s::Label>
                    <s::Item value="a"><s::ItemText />" · Orchard"<s::ItemIndicator>"✓"</s::ItemIndicator></s::Item>
                    <s::Item value="b" />
                </s::Group>
            </s::Viewport></s::Content>
        </s::Root>
    }
}
```

Put the Root inside your native form or FormRoot to submit `fruit`. The themed
`select` module exposes the same parts; its default Item assembles text and a
check indicator. The existing `Select`/`SelectOption` convenience view remains
available, with empty string mapped to no selection. New entries must have
unique, nonempty values. Unknown or removed accepted values display the
placeholder and submit an empty mirror without changing the business signal.
Illegal or missing Item/entry pairs expose `data-rustify-issue` and are not
usable targets.

Trigger retains focus. Arrows and Home/End navigate, Enter/Space request a
selection, Tab requests closure and continues normal focus movement. Printable
characters locate entries with a 1000ms prefix buffer; repeated characters
cycle. Disabled entries are skipped. Read-only Select can inspect its list and
continues submitting the accepted value, while refusing new value requests.
Only mounted listboxes/options appear in `aria-controls`/`aria-activedescendant`.

`on_select: Callback<Cancelable<String>>` runs synchronously before the value
and close requests. `prevent_default()` cancels both. Optional Escape, outside
pointer and opening/closing autofocus hooks also use Cancelable. Rejected or
delayed application requests leave the accepted label/native mirror unchanged.
Content lives in the creating scope's overlay and retains its direction/theme;
its arbitrary attributes and native listeners reach the actual listbox.

## Named sliders and input completion

For multiple thumbs use `slider::Root` with `Vec<ThumbValue { id, value }>` and
explicit Thumb parts, or `RangeSlider` with ThumbLabel declarations. Keep each
id stable when values cross. `min_steps_between` and `preserve_order` describe
normalization; displayed, ARIA and submitted values use that same normalized
projection. A named Root emits repeated native names, readable with
`FormData.getAll(name)`; retain business thumb identities in application state.

Slider `on_change` requests previews. `on_commit` reports the last requested
candidate once when a pointer/keyboard gesture completes, even if the
application refused it; it does not report an invented accepted value. Escape
or a lost capture cancels without a commit. Other thumbs continue updating by
stable identity. Single-value Slider uses this same behavior.

Text inputs and TextArea display accepted values and preserve selection when
no native value write is necessary. IME composition does not publish partial
input. Number/Color fields keep a focused Draft, with preview requests for
parseable input and one completion on Enter, blur or FormRoot submit. Their
named mirrors still hold accepted values. Read-only and disabled controls do
not publish value requests.

Source entry points: `crates/rustify-components/src/primitives/form.rs:74`,
`crates/rustify-components/src/primitives/select.rs:227`, and
`crates/rustify-components/src/primitives/slider.rs:177`. The
[generated catalogue](components.md) lists the declared props and visual
subsets; typed `control_as`/`trigger_as`/`item_as` and `primitives::Slot` accept
known native Leptos targets, rather than arbitrary rendered component trees.

## Why the values are not here

A controlled component's displayed value comes only from its `value` prop. If
the form kept a second copy, "what is in this field" would have two answers,
and the two would differ the moment one path updated without the other. That is
the exact bug P1 took out of the controls themselves; putting it back one layer
up would have been no better.

So the values are the application's, the cross-field rules are the
application's functions, and `FormState` is what decides *when* those functions
run and what may happen next.

```rust
let mut form = FormState::new(&["name", "email", "team"]);
form.changed("name");           // the user typed
form.set_error("name", Some("required".into()));
form.can_submit();              // false
```

## Generations, and the late answer

Asynchronous validation is the part that goes wrong quietly. A check starts for
what the field held a moment ago, the user types again, and the old answer
arrives last and wins.

`validating(field)` hands out a `Generation` and `validated(field, generation,
result)` refuses one that is not the current generation. A late answer about a
value the user has already replaced changes nothing - it is not an error, it is
simply about a field that has moved on.

## Submitting

`submit(rules)` is the one place a save can begin, and it answers with what
happened rather than with a boolean:

| Answer | Meaning |
| --- | --- |
| `Busy` | A save is already in flight. Nothing started - this is what makes twenty clicks one save. |
| `Blocked { first_error }` | Something is wrong now. The named field is the first in field order, which is where to put the keyboard. |
| `Waiting` | A validation has to finish first. The answer arrives from `validated` when the last one does. |
| `Save` | Save, exactly once per request. The application calls `submitted` with the outcome. |

Two rules inside `submit` are worth knowing about because they were both found
by tests rather than by reading:

- **Rules withdraw only their own errors.** Each error records where it came
  from - a cross-field rule, or an asynchronous check - and re-running the
  rules clears only the rule's own. Without that, pressing save after a failed
  asynchronous check wiped that check's error and saved anyway. The test is
  called `asking_twenty_times_after_an_async_failure_never_saves`.
- **A waiting request is bound to the form as it was.** `Waiting` records every
  field's generation at that moment. If any field moves on before the checks
  finish, the request is void: the user changed something after asking to save,
  and saving the thing they asked about is no longer what they asked for.

## Error sources and reset

A field retains separate Constraint, Rule, Check and Application errors and
shows the first active one in that order. Re-running rules only replaces Rule;
it cannot erase a failed asynchronous Check or an Application error.
`changed(field)` clears that field's previous errors/pending validation and
advances its generation. `set_error` retains its existing Rule meaning;
`set_application_error` records an application result. Message renders a public
`data-source` marker, and FieldBinding exposes `error_source()` for custom UI.

`reset()` returns false during a save. Otherwise it clears errors/dirty state
and advances generations, so old validation results and waiting submissions
cannot revive the reset form. It does not reset your value signals.

Source: `crates/rustify-dom/src/form.rs:35` and the public Form handle in
`crates/rustify-components/src/form/provider.rs`.

## Fields the form does not have

`changed`, `set_error` and the rest take `&'static str`, and a name the form was
not built with is recorded as `ErrorKind::UnknownField` rather than ignored. A
misspelled field name is a control that never validates and a form that never
becomes dirty; neither of those looks like a typo from the outside, which is why
it is written down.

## The components

`rustify_components::Form`, `Field`, `SubmitButton` and `FormStatus` put this on
screen. Three details of theirs are contractual:

- The submit button that cannot submit is `aria-disabled`, not `disabled`. A
  natively disabled button swallows the click, so a person pressing it learns
  nothing; `aria-disabled` keeps it focusable and lets the press produce the
  explanation.
- A field with an error carries `aria-invalid` and an `aria-describedby`
  pointing at the message, so the reason reaches a screen reader at the moment
  the field does.
- `FormStatus` is a live region: the outcome of a save is announced without
  moving the keyboard away from where the user is.

## Leaving with unsaved work

`dirty()` is what a navigation guard asks. The workbench registers
`beforeunload` only while some guard's `has_unsaved` is true, so a page with
nothing to lose closes without a dialogue. See `docs/navigation.md` for how a
refused navigation is undone.

## Input methods

A field reports nothing while an input method is composing in it, and reports
once when the composition ends. The keys an input method puts in the field are
how a character is being *looked up*, not the value: a form that took them would
record "gongzuo" as somebody's name and keep it if they abandoned the lookup.

The same rule keeps the control from writing its held value back into the
element mid-composition, which would take the input method's own text away from
the person using it.
