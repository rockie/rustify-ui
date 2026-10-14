---
title: "author-docs"
description: Try a Rust component and read the exact function that builds it.
---

# author-docs

The examples on this page use real Rust views. Their first render is
available before the browser attaches the controls.

## Set props from Markdown

```rustify-component
name: Counter
props:
  initial: 2
  label: Your first counter
children: |
  Change the value with **Increment** or type a number in the field.
```

## Read a registered function

```rustify-demo
name: CounterDemo
```

The function and its full file stay next to the example. You can copy
either source view without leaving the page.

Continue with [Getting started](guide/getting-started.md) or
[Writing demos](guide/writing-demos.md).
