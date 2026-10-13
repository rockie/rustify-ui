---
title: Static built-in components
description: Tabs, cards and steps without an interactive island.
---

# Static built-in components

These components render as ordinary HTML. This page has no island and needs no wasm,
even though other pages in the project use the Rust registry.

## Two independent tab groups

````rustify-component
name: Tabs
props:
  tabs:
    - label: First group overview
      content: |
        An overview of the first independent tab group.
    - label: First group code
      content: |
        A literal <tag> & raw value stays text.
children: |
  Additional **checked Markdown** includes a [registered counter](counter.md).

  ```rust
  let text = "<tag> & raw";
  ```
````

```rustify-component
name: Tabs
props:
  tabs:
    - label: Second group overview
      content: Independent group content.
    - label: Second group details
      content: A different panel with independent tab state.
```

## Cards with ordinary links

```rustify-component
name: CardGrid
props:
  columns: 2
  cards:
    - title: Registered counters
      link: counter.md
      description: Three independent counters with matching initial props.
    - title: All categories
      link: proxy.md
      description: The real 24-category release proxy.
children: |
  Follow the cards using ordinary links.
```

## Sequential instructions

```rustify-component
name: Steps
props:
  items:
    - title: Read the static example
      description: Inspect the initial DOM view before attachment.
    - title: Open a counter
      description: Open the counter page and increment one instance.
children: |
  These instructions remain available without JavaScript.
```
