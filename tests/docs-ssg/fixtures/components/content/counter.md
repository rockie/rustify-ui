---
title: Registered counters
description: Three instances share declarations while keeping independent state.
---

# Registered counters

## Three independent instances

Each counter starts with the same props and child content. Incrementing one leaves the others at two.

```rustify-component
name: Counter
props:
  initial: 2
  label: Shared initial props
children: |
  The same **safe child Markdown** appears in each instance.
```

```rustify-component
name: Counter
props:
  initial: 2
  label: Shared initial props
children: |
  The same **safe child Markdown** appears in each instance.
```

```rustify-component
name: Counter
props:
  initial: 2
  label: Shared initial props
children: |
  The same **safe child Markdown** appears in each instance.
```

## Function and full file

The registered function starts at seven. Its original function and containing file appear below the view.

```rustify-demo
name: CounterDemo
```
