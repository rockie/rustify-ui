# Navigation

One page has one address bar. A scope says whether it owns that one or has a
location of its own, and everything else follows from the answer.

```rust
mount(container, MountConfig {
    scope: "workbench".to_string(),
    url_owner: true,
    base: "/tools/demo/".to_string(),
}, App)
```

At most one scope on a page may say `url_owner: true`. A second one is refused
with `UiError::UrlOwnerConflict` and is not mounted; the scope that has it is
untouched by the attempt. The sensible answer to that error is to mount again
without it - the application then runs with a location of its own, which is
what an instance embedded in somebody else's page wants anyway.

A scope that does not own the URL still routes. It has `Routes`, `Link`,
`use_location`, `use_params` and `navigate`; what it does not have is any
listener on `window`. Its links are intercepted on its own container, because
a link inside it that fell through to the browser would navigate the page away
from underneath whoever embedded it.

## What is in the address, and what is not

`Routes` matches static segments and `:param`, in the order declared, and a
path that matches nothing is an answer - the application's not-found view -
rather than a failure. There is no nested routing, no route-level
authorisation, no server redirect and no form action. A component that expects
`leptos_router`'s context will not work here; see `compatibility.md`.

Query strings are handed over as they arrived. What a query means is the
application's business.

## Links

`Link` renders a real `<a>` with a real `href`, so it can be opened in a new
tab, copied, and read by everything that reads links. The scope's click handler
takes only the ordinary case, using the browser's own conditions for "this link
opens here": the primary button, no modifier key, same origin, under the
scope's base, no `target`, no `download`, no `rel="external"`, and nothing has
already called `preventDefault`. A link that fails any of them is left to the
browser.

The handler is on the scope's container, not on `window`. A host page's own
links are not inside it and never reach it.

## Guards, and the number in `history.state`

`NavigationGuard::register(has_unsaved)` refuses to leave while the signal is
true, and registers the browser's own "are you sure" only while it is - a page
that always asks is a page whose users learn to dismiss the question.

Refusing a link is easy: the click is the router's, and it does nothing.
Refusing a *back* is not, because by the time the router hears about it the
browser has already moved. So every entry this router pushes carries a number
in `history.state.rustify.index`, and putting the user back is one
`history.go` of the difference between where they are and where they were.

The number is a **depth in the history that exists**, not a running total. A
push truncates whatever was ahead, so after twenty moves, twenty backs and one
new move, the new entry is at depth one. A counter that kept climbing would
ask the browser to travel twenty-one entries through a history that has one,
and the user would be left where the guard promised they would not be.

`tests/browser/p2-navigation.spec.ts` holds the measurements this is built on:

- One `go()` crosses several entries at once, and the event that follows
  carries the state the entry it landed on was pushed with.
- Stepping forward one at a time recovers one step. From `/d`, `go(-3)` lands
  on `/a` and `go(+1)` returns to `/b`.
- The same URL twice is two entries. Only the number tells them apart.
- A back pressed during a recovery is its own event, so the router judges by
  the number it reads and never by counting the moves it asked for. Three
  re-aims that do not converge stop rather than fighting the user, and the
  attempt is recorded as `NavigationRestoreFailed`.

### Known limitations

- **An entry the router did not push has no number.** A host script's entry, an
  older build's, a session the browser restored: those are accepted rather than
  undone, whatever a guard would have said. They are not the router's to move.
- **A guard cannot stop a reload or a close.** `beforeunload` asks; the browser
  decides, and shows its own words.
- **Three attempts, then it stops.** A user holding the back button can outrun
  a recovery. The router accepts where they ended up and says so, rather than
  moving them again.

## Deploying under a path

A page's own references are rewritten to absolute paths at build time, for
every build and not only for one under a sub-path: `./app.js` at `/objects/42`
resolves to `/objects/app.js`, and an application with routes has deep links
wherever it is served. The policy forbids `<base>` (`base-uri 'none'`), so
rewriting is what is left - and it is the honest place for it, because the path
a build is deployed under is a property of the build.

```
mbx xtask build-web --example property-workbench --release --base /tools/demo/
mbx xtask serve     --example property-workbench --release --base /tools/demo/ --spa
```

A build for a sub-path is a second product and gets its own directory
(`…@tools-demo`); one build produces both. `serve` refuses to start when the
build it is pointed at was made for a different base, because the page would
load and its scripts would not.

`--spa` answers a path that names no file with `index.html`, so a deep link
opened cold reaches the application. Only for paths that look like a route: a
request for `app.js` that is missing stays a 404, because a missing script
answered with HTML is a deployment fault that hides itself somewhere much
later.

## A GPU region inside an application that routes

A region resolves its own resources - fonts, images, data - against the path
the application is **deployed** at, which the host reads from the build's
manifest and hands over at startup. It does not use the address bar. At
`/objects/1` that would send every font request to
`/objects/1/makepad_widgets/...`, which is a 404 and a region with no text.

A region never touches history itself (P1 C-5). A navigation that starts in a
region is an action the application receives and answers with `navigate()`,
like any other.
