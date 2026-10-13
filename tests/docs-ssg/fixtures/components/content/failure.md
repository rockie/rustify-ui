---
title: Runtime failure recovery
description: A later callback fails while source and navigation remain readable.
---

# Runtime failure recovery

Change the first counter, then choose “Crash this example.” Every island in this page belongs to one
wasm instance, so the failure restores their initial views and leaves the source and navigation available.

## A peer island

```rustify-component
name: Counter
props:
  initial: 2
```

## Failing callback

```rustify-demo
name: FailureDemo
```
