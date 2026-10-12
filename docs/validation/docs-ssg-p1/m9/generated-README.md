# site

A documentation site written in Markdown and built with
[Rustify Docs](https://github.com/rockie/rustify-ui/blob/main/docs/docs-ssg.md).

Generated with Rustify Docs 0.1.4. Install version 0.1.4
from source revision `1b81de25f23a25702f61dbfd3d0ecdfe323d4d44` on macOS with stable Rust:

```sh
cargo install --git https://github.com/rockie/rustify-ui --rev 1b81de25f23a25702f61dbfd3d0ecdfe323d4d44 --locked --profile docs-cli rustify-docs-cli
rustify-docs --version
rustify-docs --build-info
cargo install --list
```

Reinstall or upgrade to that revision:

```sh
cargo install --git https://github.com/rockie/rustify-ui --rev 1b81de25f23a25702f61dbfd3d0ecdfe323d4d44 --locked --profile docs-cli rustify-docs-cli --force
```

Roll back to the previous version, 0.1.3:

```sh
cargo install --git https://github.com/rockie/rustify-ui --rev 42757c627cb69456d241ba0de51b34f1ad85be3d --locked --profile docs-cli --force rustify-docs-cli
```

Check `rustify-docs --version`, the source revision in `cargo install --list`
and `manifest.cli_version` in a fresh build's `rustify-docs-manifest.json`.
Stop and restart `dev` after an upgrade or rollback to discard its in-memory
build cache. `build` starts from the complete inputs each time. Deploy the
whole rebuilt `dist/` with its matching manifest; keep a complete prior build
to restore it without mixing assets.

| Command | What it does |
| --- | --- |
| `rustify-docs dev` | Serves the site on your machine and updates it as you save |
| `rustify-docs check` | Reports every problem in the content, offline, changing nothing |
| `rustify-docs build` | Writes the site to `dist/` |
| `rustify-docs preview` | Serves `dist/` the way a static host would |

- Pages are the `.md` files in `content/`; files in `static/` are published
  at the site's root as they are.
- Set `url` under `[site]` in `rustify-docs.toml` to the address the site is
  published at; until then the build warns that pages have no canonical
  address and there is no sitemap. To publish under a path such as
  `/my-docs/`, set `base` to it as well (or build with `--base /my-docs/`).
- `dist/` and `.rustify-docs/` are ignored by `.gitignore`. A build writes a
  private work area `.rustify-docs/` next to its output; if you build into
  another directory with `--out`, ignore that directory and the
  `.rustify-docs/` beside it too.
- Upload the whole of `dist/` to any static host. It works under the
  configured base path without rewrite rules, and has a `404.html`. Each
  page carries its content security policy in a `<meta>` element; a host
  that can set response headers should also send it as the
  `Content-Security-Policy` header, which `rustify-docs preview` shows.
