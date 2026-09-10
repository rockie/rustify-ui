# Forms

A form here is a state machine, not a store. It holds what is wrong with each
field, whether a validation is running, whether a save is in flight and whether
anything has changed - and it deliberately does **not** hold the values.

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
