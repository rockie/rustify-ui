# {{name}}

A documentation site written in Markdown and built with
[Rustify Docs](https://github.com/rockie/rustify-ui/blob/main/docs/docs-ssg.md).

Generated with Rustify Docs {{version}}. Install version {{install-version}}
from source revision `{{revision}}` on macOS with stable Rust:

```sh
{{install}}
rustify-docs --version
rustify-docs --build-info
cargo install --list
```

Reinstall or upgrade to that revision:

```sh
{{install}} --force
```

Roll back to the previous version, {{rollback-version}}:

```sh
{{rollback}}
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

To publish several languages, keep the default pages at their existing
paths and add translations below a language prefix, such as
`content/fr/guide/getting-started.md`. Add this to `rustify-docs.toml`:

```toml
[[i18n.locales]]
lang = "en"
label = "English"
prefix = ""

[[i18n.locales]]
lang = "fr"
label = "Français"
prefix = "fr"
messages = "locales/fr.toml"
```

The empty prefix belongs to `[site].lang`. Create `locales/fr.toml` with
project text, for example `copy = "Copier"`; omitted French UI keys use
English and are reported by `check`. Missing pages show the default
language body with a notice. The language menu links to the same logical
page, including its fallback. The [author reference](https://github.com/rockie/rustify-ui/blob/main/docs/docs-ssg.md#publish-several-languages)
also covers translated navigation, drafts and language-specific 404 pages.
