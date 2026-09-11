# Data larger than the screen

A hundred thousand rows do not fit in a document, and they do not need to. What
a person is looking at is sixty of them; what the application holds is all of
them; what the table holds is neither. This page is about where each of those
three lives and what happens when they disagree.

## The table holds no data

`DataTable` is told how many rows there are and given a way to read a cell. It
never holds a value.

```rust
<DataTable
    rows=rows                     // Signal<usize>
    columns=columns               // Signal<Vec<Column>>
    cell=cell                     // Arc<dyn Fn(row, column) -> String>
    row_id=row_id                 // Arc<dyn Fn(row) -> u32>
    selected=selection            // Signal<Selection>
    goto=goto                     // RwSignal<Option<usize>>
    focus=focus                   // RwSignal<Cell>
    on_select=select
    on_activate=open
/>
```

That is the whole contract, and it is what makes a sorted view, a filtered one
and an edited row the same table asked different questions: the application
changes what `cell` answers and tells the table how many rows there are now.
Nothing is copied into the component and nothing has to be invalidated.

`row` is a position in the view - first row, second row - not an identity.
`row_id` is the identity, and the two must not be confused: a sort changes
every row's position and no row's identity.

## What is actually in the document

Row elements are a pool. There are as many as fit the viewport plus ten of
overscan either side, and the pool is the same size wherever the table is
scrolled to - at the top, in the middle, and against the last row. Scrolling
moves them and rewrites their text; it does not make new ones. Columns work
the same way across, with two of overscan.

So the `<For>` is keyed by the slot, not by the row. Which row a slot is
showing is written on it, in the two attributes a person's screen reader and a
test both read:

- `aria-rowindex` - the row number, from 1.
- `data-row-id` - the identity `row_id` gave for it.

`aria-rowcount` on the grid is the whole table, not the pool, so a screen
reader says "row 50,000 of 100,000" rather than "row 12 of 81".

Two consequences are worth knowing before they surprise you:

**The scroller needs a height of its own.** The window is worked out from the
viewport it is given. Put the table in a container that grows to fit its
contents and the arithmetic inverts: the scroller becomes as tall as the
spacer, the viewport becomes as tall as the whole table, and the window asks
for every row there is. At a hundred thousand rows that is not a slow page but
a dead one. The window therefore has a ceiling of two hundred rows, so a
viewport with no height of its own is visibly wrong rather than fatal - but the
fix is to give the container a height, usually `min-height: 0` on the flex
parent it is in.

**Which row a slot shows is a reactive attribute.** It is written when its own
effect runs, which can be after the table has already moved the keyboard. So
anything inside the table that has to find a cell finds it by its slot, and the
focus event that follows is not read back as a person having chosen a cell.
Outside the table, read `aria-rowindex`: by the time an event handler or a test
sees it, it is settled.

## A selection is of things, not of positions

`Selection` (in `rustify-ui`) is a set of identities, and it has three rules,
which live in one place because three different views read one selection.

- **Sorting and filtering keep it.** Moving a thing does not deselect it, and
  neither does hiding it.
- **Deleting clears it.** `remove_deleted(&gone)` after every deletion. A
  selected identity that no longer exists is a leak, and it would come back the
  moment an identity was reused - which is why identities never are.
- **How many are hidden is a question about the view.** The selection does not
  know what is on screen, so it is asked:

```rust
let counts = selection.counts(|id| view.contains(id));
// counts.visible, counts.hidden, counts.total()
```

The hidden count is what a person needs to see before they act on a selection
they cannot fully see. The table's status line says
`selected 12 (of which 5 not in view)`, in an `aria-live="polite"` region, so
it is announced rather than only displayed.

## Getting to a row you cannot see

A hundred thousand rows need a way to say "that one" that is not scrolling.
There are three, and all three are ordinary DOM controls with labels:

- **Go to row** - a number field. Sets `goto`, which the table clears once it
  has shown the row, so asking for the same row twice works.
- **The groups** - a two-level tree beside the table, `role="tree"`, one tab
  stop, arrows within it. Right opens a closed group and steps into an open
  one; left closes an open one and steps out of a child.
- **The strip** - the whole table as one band under it, showing where the
  selection is. Clicking a bucket sends the table there. It is drawn by a GPU
  region and it is decoration: everything it can do, the number field can do
  too.

From the keyboard the grid is a grid: arrows, PageUp/PageDown by a screenful,
Home and End for the ends of the row, Ctrl+Home and Ctrl+End for the ends of
the table, Space to select, Enter or F2 to open the row. Walking off the window
scrolls the table - on both axes - and the keyboard lands on the cell once the
window has been worked out, so a walk to the last row does not leave the focus
behind.

One tab stop for the whole grid: the focused cell has `tabindex="0"` and every
other cell `-1`.

## Work that does not fit in a frame

Sorting a hundred thousand rows takes about fifty milliseconds. That is three
frames, and a frame you cannot draw is a page that has stopped. There is no
thread to move it to either: an ordinary deployment is not cross-origin
isolated, so there is no shared memory and no worker that could see these rows
without being sent thirty-two megabytes of them.

So the work is sliced. `rustify_ui::Job` takes a closure that does as much as
it can within a `Budget` and says `More` or `Done`; between two slices the
browser gets its turn back, through the host's own task rather than a timer.

```rust
let job = Job::new(requests.issue(), version, job::now, total, move |budget| {
    let done = sort.step(rows, &mut || budget.exhausted());
    budget.did(sort.processed() - charged);
    if done { Step::Done(sort.into_order()) } else { Step::More }
});
job::run(job, move |ended| { /* Done, Stale or Cancelled */ });
```

Three rules hold it together, and each of them is a thing that goes wrong
otherwise.

**A slice is a length of time, not a number of rows.** Eight milliseconds, with
a row count as an upper bound so that a step which turns out to be cheap cannot
run away with the turn. Ask `budget.exhausted()` every few hundred rows: a
clock read per row costs more than the row.

**A job that ran while the data changed has nothing to say.** The version is
read before a slice touches anything. If it has moved, the slice does not run,
nothing is delivered, and the application is told `Stale`. This is not
politeness: a job holds row *positions*, a deletion moves every position after
it, and a slice that read one anyway would read the wrong row - or, at the end
of the sample, no row at all.

The application decides what to do about it. This one asks again, once, with
the version it now has. Once, because a second write during the re-run is
somebody typing, and a job restarted on every keystroke never finishes one.

**Only the newest job may deliver.** One `Requests` handle: starting a job ends
the one before it, closing the scope ends them all, and `requests.cancel()` is
what a cancel button does. A cancelled job is not interrupted - it stops before
its next slice - so there is never half a result anywhere. What the view shows
is what it showed before, because nothing is applied until a job is finished.

## Sorting, filtering and finding

All three are jobs, and they differ only in what they are asked and what they
produce.

- **A sort** orders the view it is given, so sorting a filtered view sorts what
  is left. It is a merge sort over row positions, stable, so sorting by a second
  column refines the first rather than reshuffling it.
- **A filter** is asked of the whole sample rather than of the view - a row the
  last filter hid is a row this one may want - and the view it produces is the
  rows that matched, in the sample's order. Text is matched within a cell: cells
  are fixed width and butt up against each other, so a search over a whole row
  would find things that are not in it.
- **A group** in the tree is the same job with a different question: not what is
  in a row but where it is.
- **A find** is asked of the view, because where it lands is a place in what the
  person is looking at. It stops at the first match and scrolls there.

While one runs, the status line says how far it has got and the cancel button
is live. Cancelling is instant to look at - the status changes in the same
frame as the click, ahead of the slice that is already running noticing.

## Editing a row

The row that is open is shown as a form of twenty fields beside the table, and
that is where a value is changed - not in the cell. A cell is text; an editor
is a control with a label, a description and an error that can be read. Saving
writes the fields that changed and moves the sample's version on.

The version is what a job in flight watches, and it moves for *any* write:
sorting, filtering and finding all read cell values, so an edit makes a running
job's answer stale even though no row was added or removed.

A write does not wait for a job either. Rows that went are taken out of the
view and the ones after them move up; rows that arrived go where they were put
if the view has no order of its own, and at the end if it has - a sorted view
has no correct place for a row nothing has compared yet. The view then says it
was built for an older version, which is what "stale" on the screen means:
still usable, no longer the answer.

## What is not here

No server paging, no data source protocol, no formula engine, no column
resizing or reordering, no grouping by value, and no virtualised tree. No
worker and no second thread: a job is sliced on the one thread there is. The
table draws what it is told and holds nothing.
