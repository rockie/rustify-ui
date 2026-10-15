---
title: Getting started
description: Run the site and change the first component's props.
sidebar:
  order: 1
---

# Getting started

From the project root, run:

```sh
rustify-docs dev
```

Open the address printed by the command. The first build may download
locked dependencies and compiles both the native renderer and browser
wasm. The page shows its loading state until the examples are ready.
Later builds reuse those compilation targets.

## Change an initial value

Open `content/index.md` and change the first counter's `initial` from `2`
to `10`. Save it with `dev` running. The page reloads with the new initial
value, using the already compiled Rust renderer and wasm.

```yaml
name: Counter
props:
  initial: 10
  label: Your first counter
```

Each block has its own state. Clicking one counter does not change another
block, even when both use the same registered component.

## Check and publish

```sh
rustify-docs check
rustify-docs build
rustify-docs preview
```

`check` validates content and both Rust targets without replacing your
published site. It may write private compilation caches. `build` validates
a complete candidate before publishing `dist/`; a failed candidate leaves
the previous successful output intact. `preview` serves that output as a
static host would.

Before publishing, set the public `url` and matching `base` in
`rustify-docs.toml`. Upload the complete `dist/` with its scripts, wasm,
fonts, manifest and `404.html`.

Next, [register your own function](writing-demos.md).
