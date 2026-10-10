---
title: Getting started
description: Preview the site while you write, then build it for publishing.
sidebar:
  order: 1
---

# Getting started

## Preview while you write

```sh
rustify-docs dev
```

Open the address it prints. Saving a file updates the page you are reading.

## Check and build

```sh
rustify-docs check
rustify-docs build
rustify-docs preview
```

`check` reports every broken link, missing image or unsafe HTML with its
file, line and column. `build` writes the site to `dist/`, and `preview`
serves that output the way a static host would.

Next: [write a page](writing-pages.md).
