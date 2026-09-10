# Workspace

Three pieces: panels side by side, tabs over them, and every command in one
list. Each has a part that is easy to get wrong silently, and each keeps that
part as a function with its own tests.

## Panels and dividers

```rust
<div class="workspace" style:grid-template-columns=move || columns(&sizes.get())>
    {list} {region} {form}
    <Splitter sizes=sizes mins=vec![220.0, 320.0, 360.0] on_resize=… />
</div>
```

**The splitter does not contain the panels.** A panel may hold a GPU region,
whose application marker is not `Send`, and anything passed through a Leptos
component's children has to be. So the application keeps its own markup and its
own container - `position: relative`, `display: grid`, columns from
`workspace::columns` - and the splitter draws its dividers over the boundaries.
What it owns is the arithmetic, the dividers and the keyboard.

A drag takes room from one panel and gives it to its neighbour, so the row's
total never changes and nothing further along it moves. Neither of the pair
goes below the minimum the application declared, because only the application
knows what its own content needs: a divider stops rather than pushing through.

Each divider is a `separator` with a name, takes a tab stop, and moves on the
arrow keys by a step the caller declares. It captures the pointer on the way
down, so a drag that outruns it keeps going and every move afterwards arrives
at the divider rather than at whatever is under the cursor.

A region in a panel is resized by the same path any other resize takes (P1's
`ResizeObserver`), and the property that matters afterwards is P1's V4: the
geometry a region reports and the geometry a pointer hits are the same thing.
`p2-workspace.spec.ts` re-checks it across a hundred adjustments.

## Tabs over panels

`PanelTabs` is controlled: it shows what the application says is active and
asks for the rest. It is not the catalogue's `Tabs` - the difference is the
close control, and therefore the question of what happens to the panel showing
when it is the one closed.

`after_close` answers it: the right-hand neighbour, or the left when there is
nothing to the right, or nothing at all when it was the last. Closing one that
is **not** showing changes nothing - the case that is easy to get wrong by
recomputing anyway, and silent when you do, because the user simply ends up
somewhere they did not ask to be.

The strip asks; the application decides. It is the application that knows
whether closing a view means discarding work, and what the next view should be.
A tab can be marked `permanent()`, so there is always somewhere to be.

Closing is not choosing: the close control stops the click from reaching the
tab it is about to remove.

## Commands

`CommandPalette` is a modal layer on the SDK's own overlay stack, so Escape,
the focus trap and the return of focus are the stack's.

Every word of the query has to appear somewhere in a command's label or
keywords, so "close tab" finds one command rather than everything that mentions
closing.

**A command that cannot run stays in the list.** It is reachable by the arrows,
it says why beside its label, it is announced with that reason, and activating
it does nothing. Hiding it tells a person the application has lost it; showing
it with a reason tells them something about their own state.

Availability is a signal on the command, not a value captured when the list was
built, so what a person sees is the answer at the moment they look.

Enter during an IME composition belongs to the composition. Running a command
on it would be the wrong command, chosen by somebody who was typing a word.

## What is not here

- **No arbitrary docking tree.** Panels are a row; tabs are a strip over one of
  them. A dock that can be rearranged into any shape is not in this release
  (D7), and neither is a native multi-window layout.
- **No layout in the URL.** The address carries the object being looked at
  (`docs/navigation.md`); which panel is how wide and which view is showing are
  in memory. A workspace that put its layout in the address bar would fill a
  person's history with resizes.
- **No layout persistence.** Sizes and open views start where the application
  puts them on every load.
