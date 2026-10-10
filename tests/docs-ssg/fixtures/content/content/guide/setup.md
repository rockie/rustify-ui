---
title: Setup
description: Installing the CLI and writing a first page.
sidebar:
  order: 2
toc:
  levels: [2, 3]
---

# Setup

> [!NOTE]
> Every command here runs offline.

## Install the CLI {#install-the-cli}

<!-- rustify:code-group name="install" -->

```sh title="Cargo"
cargo install --locked rustify-docs-cli
```

```toml title="Config"
schema_version = 1
```

<!-- rustify:end-code-group -->

### Check the version

Run `rustify-docs --version`; see https://example.com/releases for others.

| Command | Writes | Network |
|:--------|:------:|--------:|
| `check` | nothing | no |
| `build` | `dist/` | no |

- [x] Install
- [ ] Write a page[^first]

![The pipeline](pipeline.svg)

<!-- rustify:include path="src/example.rs" region="main" -->

The source is [on GitHub](../../src/example.rs).

> [!WARNING]
> Drafts are left out of a production build.

[^first]: A page is a Markdown file under `content/`.
