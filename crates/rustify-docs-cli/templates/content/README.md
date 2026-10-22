# {{name}}

A documentation site written in Markdown and built with
[Rustify Docs](https://github.com/rockie/rustify-ui/blob/main/docs/docs-ssg.md).

Generated with Rustify Docs {{version}}. Install this exact registry version
on macOS with stable Rust:

```sh
{{install}}
rustify-docs --version
rustify-docs --build-info
cargo install --list
```

Reinstall or upgrade to that version:

```sh
{{install}} --force
```

Historical Git rollback to {{rollback-version}}:

```sh
{{rollback}}
```

Check `rustify-docs --version`, `--build-info`, the version in `cargo install --list`
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

## Theme and local fonts

The default reading theme uses system fonts and makes no font requests. To
customize a complete theme without building Theme Studio:

1. Copy `theme.example.json` to `theme.json`. The example is inactive until you
   set `document` below.
2. In both `styles.light` and `styles.dark`, set the needed font token, for
   example `"font-sans": "Brand Sans, sans-serif"`. Keep all other tokens.
3. Put a font you may redistribute at `fonts/brand-sans.woff2`, then add:

```toml
[theme]
document = "theme.json"

[[theme.fonts]]
family = "Brand Sans"
src = "fonts/brand-sans.woff2"
weight = "400" # or an ascending range such as "100 900"
style = "normal" # or "italic"
```

Registration alone does not select a font; the document's font stack selects
it. Paths are project-local, preferably outside `static/`; woff2, woff, ttf
and otf are supported. URLs, escaping symlinks, private files, output files,
empty files and duplicate family/weight/style faces fail the build. Keep
font notices and any redistribution/source obligations with your project.
The build publishes declared bytes at same-origin `assets/fonts/<sha256>.<ext>`
URLs with `font-display: swap`. It performs no font download.

Run `rustify-docs check --deny-warnings` and `rustify-docs build`, then preview
the complete output. Both modes and islands use the same CSS font policy.
The CLI embeds no font files. An imported but undeclared named family gives
`theme-font-system`: use a local declaration or change the relevant tokens in
both modes to `sans-serif`, `serif` or `monospace`. Remaining warnings still
fail `--deny-warnings`. CJK and emoji now follow the author stack and system
fallback rather than the old bundled glyph fallbacks.
