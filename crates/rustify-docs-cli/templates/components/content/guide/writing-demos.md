---
title: Writing demos
description: Write a Rust function, register it and place its live view in Markdown.
sidebar:
  order: 2
---

# Writing demos

A demo is a Rust function that returns a Leptos view. Its registered name
connects the live view to the exact function and full-file source.

## Write the function

Add this function to `src/demos.rs`, which already imports the Leptos
prelude and defines `counter()`:

```rust
pub fn double_counter_demo() -> impl IntoView {
    counter(14, "Double counter".to_string(), ().into_any())
}
```

## Register it once

Extend the existing `demos` block inside `site()` in `src/lib.rs`:

```rust
demos {
    CounterDemo => crate::demos::counter_demo,
    DoubleCounterDemo => crate::demos::double_counter_demo,
}
```

The function path gives the build a source identity. Refer directly to the
function instead of wrapping it in an anonymous closure.

## Put it on a page

Add this block to `content/index.md`:

````markdown
```rustify-demo
name: DoubleCounterDemo
```
````

Save with `rustify-docs dev` running. After the rebuild, the new counter
starts at 14 and Increment changes it to 15. The source controls show the
function and full file from the same build. A Rust error points to your
original file and leaves the last successful page available until you fix
it and save again.

## Keep the first view deterministic

For a reusable component, declare its typed props in the `components`
block and pass the supplied child view into your component. Markdown
props set the initial state; later input belongs to that instance.

The host and browser must produce the same first tree and IDs. Start
browser measurements or async work after attachment. A component requiring
browser-only initialization can opt into a `client_only` placeholder in
its registration, with an explicit message and reserved size.

The project-root `README.md` has complete props, child content,
`client_only`, custom-page, layout-slot, theme and deployment examples.
