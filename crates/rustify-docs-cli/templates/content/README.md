# {{name}}

A documentation site written in Markdown and built with
[Rustify Docs](https://github.com/rockie/rustify-ui/blob/main/docs/docs-ssg.md).

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
