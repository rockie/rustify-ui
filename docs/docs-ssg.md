# Rustify Docs

Rustify Docs turns a directory of Markdown into a static documentation site.
Every page has its initial HTML before any script runs; code is highlighted
at build time into classes. A default content site has no Rust project and
downloads no wasm. A configured Rust project can add components and demos:
`check` and `build` compile their shared views for static generation and
browser attachment, and only pages with interactive islands load wasm. The
`rustify-docs` command (package `rustify-docs-cli`) checks and writes the site.

This page is the reference for what an author writes: the project layout,
`rustify-docs.toml`, frontmatter, the Markdown the site understands and the
problems `check` reports. The same Markdown file is meant to read well on
GitHub; where the two differ, the section says so.

Supported on macOS. Linux and Windows are not verified.

## Install and maintain the CLI

The verified CLI is **0.1.32**, from source C
`88566ba1ed08223a5834f794840fff5e48f4ba8a`, paired with library revision
`0fd34edaffda1926b38ea64c023b2fe1fc5115c5`. It supports content sites,
the components template and the multilingual examples below.
`rustify-docs --build-info` and `cargo install --list` identify the CLI
source C after installation.

The CLI is distributed from git, with its dependencies locked and the
workspace's `docs-cli` profile, which unwinds so that `dev` can recover from a
failed build. The pinned version below is **0.1.32**, verified from both local
git and GitHub on macOS arm64 with Rust 1.99.0. The source revision, profile,
generated template and production manifest were checked after installation.

```sh
cargo install --git https://github.com/rockie/rustify-ui --rev 88566ba1ed08223a5834f794840fff5e48f4ba8a --locked --profile docs-cli rustify-docs-cli
rustify-docs --version
rustify-docs --build-info
cargo install --list
```

Expect `rustify-docs 0.1.32`, `panic: unwind`, and source
`88566ba1ed08223a5834f794840fff5e48f4ba8a`. The install listing names that
revision too. To create and preview a content site:

```sh
rustify-docs new my-docs
cd my-docs
rustify-docs check
rustify-docs dev
```

Stop `dev`, then run `rustify-docs build` and `rustify-docs preview` to test
the production output. For content-only sites, no site compilation or
network request is needed by `new`, `build` or the default `check` after the
CLI is installed.

Reinstall or upgrade to this pinned version with `--force`:

```sh
cargo install --git https://github.com/rockie/rustify-ui --rev 88566ba1ed08223a5834f794840fff5e48f4ba8a --locked --profile docs-cli --force rustify-docs-cli
```

The rollback target for this release is **0.1.31**, the previous release,
from source C `1146b7d8febdf53ca1bc1a4c01946604e7625a95` with the same paired library
revision. Its public Git installation was verified when it was published.
Roll back with:

```sh
cargo install --git https://github.com/rockie/rustify-ui --rev 1146b7d8febdf53ca1bc1a4c01946604e7625a95 --locked --profile docs-cli --force rustify-docs-cli
```

After either command, check `--version`, the source in `cargo install --list`
and the `cli_version` in a fresh build's `rustify-docs-manifest.json`.
Restart `dev` after switching versions to discard its in-memory cache;
production `build` always reads the complete inputs. Restore a deployment by
replacing the entire output and its matching manifest with a complete prior
build. Keep each version's fingerprinted assets together.

`--help` and the generated README use the same source revision recorded at
CLI compilation. This reads Cargo's existing git metadata using Rust alone;
it requires no git command or network request. A source archive without
that metadata reports `source: unknown` and gives the pinned 0.1.3 recipe.
The bundled help also retains its older 0.1.3 rollback recipe; use the
0.1.31 target above for this release.

## Commands

| Command | What it does |
| --- | --- |
| `rustify-docs new <dir> [--template content\|components]` | Creates a content site (home page, two guides, `static/`, README and `.gitignore`) or a paired Rust component project in a new or empty directory. |
| `rustify-docs dev [--port <n>]` | Serves the site from memory while you write, drafts included, and updates open pages on save. Rust sites may write private compilation caches; source and published output stay unchanged. |
| `rustify-docs check [--format text\|json] [--deny-warnings] [--external-links]` | Reports problems without changing source, configuration or published output. Content-only checks compile no site Rust; configured Rust projects run both Cargo targets and may write private compilation caches. |
| `rustify-docs build [--base <path>] [--out <dir>] [--format text\|json] [--deny-warnings]` | Checks the site the same way, then writes it to `build.out` (default `dist/`). |
| `rustify-docs preview [--dir <dir>] [--base <path>] [--port <n>] [--host <address>]` | Serves a production build the way a static host would, without rebuilding it. |

`dev`, `check`, `build` and `preview` take `--config <path>` for a
`rustify-docs.toml` elsewhere
than the working directory, or a configuration file of another name: that
file is the one read (and, in `dev`, watched), its directory is the
project, and its problems are reported under its name. `--out` and `--dir` are relative to the working
directory, as paths on a command line are; paths inside `rustify-docs.toml`
are relative to the project. `--base` builds for another base path than
`site.base` (it must still match `site.url`'s path).

`new` and `dev` first check that `rustc --version` runs. A components
template or a configured Rust site also needs the active compiler's
`wasm32-unknown-unknown` standard library. Missing tools stop the command
before writing, watching or listening; the error gives the applicable
`rustup` command. `new` then refuses a directory that is not empty and writes
the complete project into a temporary sibling, renaming it into place only
after every file is written. Failure removes the temporary project and
preserves a pre-existing empty target. `new` and content-only builds never
run Cargo; the first Rust `check`, `build` or `dev` may fetch locked
dependencies. `check --external-links` is the explicit content-link network
check; without it, content-only checks make no network request.

`check --external-links` also requests every link to another site (in
pages, home page actions, `nav.top` and `nav.social`): each address once
however often it is linked, eight at a time, within 5 seconds to connect and
10 to answer, following up to five redirects, with GET where a server
refuses HEAD. An address that answers 400 or more is an `external-link`
error, and one that cannot be reached (no network, a timeout, a TLS failure)
an `external-link-unreachable` error, at every place it is written: a link
that could not be checked is never counted as working. TLS uses bundled
root certificates, not the system's. With `--format json` the summary adds
`external_addresses`, the number of addresses requested.

Exit status: 0 on success; 1 when the site has errors (with
`--deny-warnings`, warnings too) or the command fails; 2 for a usage error.
Text output goes to stderr and ends with the conclusion, such as
`check: 4 pages, 0 error(s), 1 warning(s)`; a successful build ends with its
page count, total size, file count, warnings and time, followed by its five
largest pages, and a failed one with its error count and the first error.
With `--format json`, stdout holds exactly one JSON object and nothing else,
also when a build is refused before it starts (an output it may not replace
or that another build or a preview holds: rules `output`, `output-busy`):

```json
{
  "schema_version": 1,
  "success": false,
  "diagnostics": [
    {
      "severity": "error",
      "rule": "broken-anchor",
      "file": "content/index.md",
      "line": 8,
      "column": 5,
      "message": "content/guide.md has no heading `#no-such-heading`",
      "suggestion": "link to an existing heading's id; ids are the heading text in lower case with spaces as `-`"
    }
  ],
  "summary": { "errors": 1, "warnings": 0, "pages": 2 }
}
```

A build's `summary` also has `files`, `bytes`, `largest` (the five largest
pages, `path` and `bytes`), `out` and `milliseconds`.

For content diagnostics, `file` is relative to the project root; `line` and
`column` count from 1
(columns in characters). A problem with no position in a file has `null`
there, and `suggestion` is `null` when there is nothing more specific to say
than the message. Diagnostics are sorted by file, line, column, severity and
rule, and each appears once.

Content errors such as an invalid `lang`, a second H1 or a skipped heading
level point to the YAML value or heading in the original file and include
a repair. `check`, `build` and `dev` report the same location and advice.
An I/O or output-lock failure has no invented source position.

Rust compiler diagnostics keep the original compiler message, primary
source span and help/notes. Files inside the project use relative paths;
a dependency outside it keeps the compiler's path. The CLI compiles the
original project rather than a relocated copy, so file, line and column
refer to the source being edited.

## Project layout

```text
rustify-docs.toml      the site's configuration
content/               Markdown pages, and the images and files they link
  index.md             →  /
  guide/index.md       →  /guide/
  guide/setup.md       →  /guide/setup/
static/                files published at the site root as they are
  logo.svg             →  /logo.svg
```

A page's address comes from its path: `index.md` is its directory's page and
any other `name.md` is `/name/`. Two files that would have the same address
are an error, and `404.md` is reserved for the generated not-found page.

Files are never published by accident. What a build publishes is every page,
every file under the asset directory at its own path (for addresses others
know, such as `/favicon.ico`), and every file a page or the theme links to or
shows, under `assets/files/` and named by its content, wherever it lives.
Files whose names mark them as private (`.env`, `.env.*`, `*.env`, `.git`,
`.DS_Store`, `.rustify-docs`), and everything under a directory so named,
are never read for publishing: not as a page (`content/.env.md` is none),
not as a file, not as an include, a logo or a preview image. A symbolic
link is followed only while it leads to a file inside the project that is
not private, whatever the link is called; one that leads outside
(`symlink-outside`) or to a private file (`private-file`) is an error, and
so is a content or asset directory that is itself such a link. A site with
no page to publish (an empty or missing content directory, or nothing but
drafts) is an error (`no-pages`), so a build never replaces a site with an
empty one.

## `rustify-docs.toml`

Every key is known: a misspelled one is an error at its position rather than
a setting that silently does nothing.

```toml
schema_version = 1                    # required; this CLI reads 1

[site]
name = "Rustify UI"                   # required
description = "…"
lang = "en"                           # default "en"; a BCP 47 tag
url = "https://example.com/docs/"     # optional; its path must equal base
base = "/docs/"                       # default "/"; written as "/" or "/a/b/"
image = "static/card.png"             # optional link-preview image (PNG, JPEG, GIF or WebP)

[content]
dir = "content"                       # default
# include = ["index.md", "guide/setup.md"]   # optional exact Markdown selection
[assets]
dir = "static"                        # default
[build]
out = "dist"                          # default; never the root or an input directory

[repo]                                # maps links to files outside the content
url = "https://github.com/owner/repo"
ref = "main"
path = ""                             # where the project sits in the repository
edit_url = "https://github.com/owner/repo/edit/main"   # optional: "Edit this page" links

[theme]
primary = { light = "#175cd3", dark = "#53b1fd" }   # optional; both #RRGGBB
logo = "static/logo.svg"              # or { light = "…", dark = "…" }

[[nav.groups]]                        # sidebar groups, in order
label = "Guide"
dir = "guide"                         # every page under content/guide/, by sidebar.order
scope = "/"                           # optional route prefix the group is shown under

[[nav.groups]]
label = "Reference"
items = ["reference/api.md"]          # or an explicit list of content paths

# With registered Rust pages, use a separate group:
# [[nav.groups]]
# label = "Examples"
# routes = ["/examples/"]             # registered routes, without site.base

[[nav.top]]                           # header links; also nav.social
label = "GitHub"
link = "https://github.com/owner/repo"

[dev]
port = 4188                           # default
```

Paths are relative to the project root and written with `/`. The logo files
and `site.image` must exist. `[landing.profiles.<name>]` tables declare what
landing pages may load; see [Landing pages](#landing-pages). A page's edit link is `edit_url`, then
`repo.path`, then the page's file (`…/edit/main/docs/content/guide/setup.md`). A brand colour that gives link text or button labels less than
4.5:1 contrast on the theme's background is a warning (`brand-contrast`).

`nav.top`, `nav.social` and a home page's `actions` are links like any other:
`guide/setup.md`, `/guide/setup/` and `https://…` resolve and are checked the
same way (a relative path in the configuration is relative to the content
directory), and a file only they name is published like one a page names.
`nav.top` links, then `nav.social` ones, show on every page: in the header
from 768 px, and in a row under it below that. The link to the page in view
is marked, and so is one whose page the page in view is under (a "Guide"
link to `guide/index.md` on every page under `/guide/`).

Without `[[nav.groups]]` the sidebar lists every page in one group named
after the site, plus one group per `sidebar.group` a page names. With groups,
a page none of them lists is a warning (`not-in-nav`); a group's `items`
naming a page that does not exist is an error.

A group chooses exactly one of `dir`, `items` or `routes`. `routes` lists
registered Rust pages in the declared order, using canonical addresses such
as `/examples/` without the deployment base, query or fragment. A duplicate
route, an unknown route or a Markdown page listed there is an error; use
`dir` or `items` for Markdown pages.

### Publish several languages

These multilingual examples use the paired CLI 0.1.32 and the framework
revision described above.

Declare every language, including the default `site.lang`. The default has
an empty prefix and keeps the existing addresses. Other languages each
have one unique URL segment:

```toml
[[i18n.locales]]
lang = "en"
label = "English"
prefix = ""

[[i18n.locales]]
lang = "zh"
label = "中文"
prefix = "zh"

[[i18n.locales]]
lang = "fr"
label = "Français"
prefix = "fr"
messages = "locales/fr.toml"

[i18n.locales.nav]
groups = [{ label = "Guides", items = ["guide/setup.md"] }]
top = [{ label = "Installation", link = "guide/setup.md" }]
```

`content/guide/setup.md`, `content/zh/guide/setup.md` and
`content/fr/guide/setup.md` share the logical route `/guide/setup/`.
With base `/docs/`, their addresses are `/docs/guide/setup/`,
`/docs/zh/guide/setup/` and `/docs/fr/guide/setup/`. Translate the content
without changing its logical path. A translation without a default-language
source is an `i18n-orphan` error. When `content.include` selects exact files,
add the translated paths to that list too; declaring a language does not
expand the selection.

The language menu links to the same logical page. A missing translation
uses the default source at the target address, with an untranslated notice,
target-language controls and the source language on its body and title.
A relative page link follows the current UI language; an explicitly
prefixed site link keeps the language it names. Images, includes and
repository files retain their real source paths. Anchors must exist in the
actual target version. The script preserves a hash on language switching
only when the target contains that ID.

The default-language pages define the page set. A default draft hides that
logical page in every language in production. A translated draft uses the
default body at its language address; `dev` can show drafts. Omitting i18n
keeps the previous independent `frontmatter.lang` behavior.

Each locale can replace the complete `nav` definition: `groups`, `top` and
`social`, with the existing field shapes. `items` and `dir` still select
default-language content paths, and `routes` still select logical registered
routes. The resulting navigation stays inside the current language, using
translated page titles where available. Do not put a language prefix in
these selectors. A supplied `frontmatter.lang` must match its directory's
declared language; tag comparisons ignore case and HTML retains the
configuration's spelling.

`messages` is a project-relative TOML file of plain strings:

```toml
menu = "Parcourir"
language = "Langue"
untranslated = "Cette page n’est pas encore traduite."

[demo]
loading = "Chargement de l’exemple…"

[dev]
revision = "révision {n}"
```

The fixed keys come from the [built-in label directory](../crates/rustify-docs/src/labels.rs).
English and Chinese can override individual keys. Other languages use the
provided strings and fall back to English one key at a time; each control
marks the language of its actual text. Missing project keys each produce
one `i18n-missing-label` warning. Unknown keys, non-string values, control
characters and changed format placeholders are errors; preserve `{n}` in
count and revision messages. Strings are escaped as text. Keep message
files outside the scanned content and assets directories: they are build
inputs and cannot be downloaded, included as source or published as assets,
including through a symbolic alias. Editing them triggers a dev rebuild.

Registered pages can declare `lang: "zh"` with the same logical `route` as
the default function. Names remain globally unique. Markdown and registered
pages can be translations of each other. For a missing registered version,
the default function is rendered again in the target environment;
`RenderContext.route` stays logical, `lang` is the target UI language and
`base` is the deployment base. Islands retain that same environment through
SSR and hydration. SDK controls support English and Chinese; another valid
language uses English SDK words without producing project-message warnings.

Real, indexable versions link to each other with `hreflang` and have their
own canonical address. A fallback has `noindex`, points its canonical to
the source version and is absent from the sitemap and alternate targets.
Without `site.url`, canonical, alternate and sitemap output are omitted.

Layouts remain left-to-right; RTL is unsupported. Removing a locale or
changing its prefix changes public addresses. No redirects or URL migration
are generated. Upgrade or roll back the CLI, framework, configuration and
content as one matched set.

### Select Markdown sources

Without `content.include`, the content directory keeps its recursive
discovery. Set an explicit list when only particular Markdown files belong
to the published site:

```toml
[content]
dir = "docs"
include = ["quickstart.md", "components.md"]
```

Each entry is relative to `content.dir`, so these name `docs/quickstart.md`
and `docs/components.md`. Entries must be literal `.md` files: glob patterns,
paths outside the content directory, private paths, duplicates and other
extensions are errors at the configuration entry. A missing or unsafe
selected file is also a located error. `include = []` selects zero Markdown
pages; it does not restore recursive discovery. A project with registered
pages can use that empty selection; without any page to publish, the normal
`no-pages` error still applies.

The selection is made before file bytes or frontmatter are read. Unselected
Markdown and unreferenced files under the content directory are not scanned
or published. Selected pages can still reference images and downloads:
those files are read through the normal resource graph, tracked as inputs
and published at their fingerprinted paths. The independent `assets.dir`
keeps its usual whole-directory publishing rules.

`nav` chooses wayfinding, while `content.include` chooses source pages. Keep
Markdown `items` within the selection and use `routes` for registered pages;
sidebar omission and `draft` do not substitute for the source selection.
The repository's [site configuration](../rustify-docs.toml) demonstrates
this with twelve selected guides and separate registered component routes.

### Import a theme document

To use an exported Theme Studio JSON document, replace `theme.primary` with:

```toml
[theme]
document = "theme.json"
```

`theme.document` and `theme.primary` cannot be set together. The document is
UTF-8 JSON up to 256 KiB, relative to the project root, and is a build input.
Keep it outside `assets.dir` and do not link it as a download. The CLI validates
both modes and applies the same resolved semantic colours, typography,
spacing and radius to the page and its island scopes. System mode follows
the reader's preference; islands do not store separate mode choices.

Referenced fonts and glyph fallbacks come from the CLI's bundled bytes and
licenses, with font files published as `assets/fonts/<digest>.ttf`. No font is
downloaded from another origin. Unknown theme tokens or font fallbacks keep
their diagnostics; missing packaged font bytes or licenses stop the build.
Without `theme.document`, the default reading theme uses system fonts and
adds no font assets. See [themes](themes.md) for the document format and
font selection.

## Frontmatter

YAML between `---` lines at the very start of a page. Every key is optional
and every key is known:

| Key | Meaning |
| --- | --- |
| `title` | The page's title. Without it, the first `# heading` is the title; a page with neither is an error. |
| `description` | A description for link previews and AI discovery. Without it the first paragraph is used (warning); an ordinary page with neither is an error. A `layout: home` or `layout: landing` page may omit both; a home page's AI entry uses the separate fallback below. |
| `lang` | Without i18n, the page's language tag, defaulting to `site.lang`. With i18n, it must match the language declared for its content directory. Interface strings use that language's catalog with per-key English fallback. |
| `sidebar.label`, `sidebar.order` | The sidebar's text for the page, and its position in its group (lower first; pages without an order follow, by path). |
| `sidebar.group` | The nav group the page is listed in, by label, instead of its directory's. It must name a configured group; without `[[nav.groups]]` it starts a group of that name. A group's explicit `items` list is never changed by it. |
| `draft` | `true` keeps the page out of a production build. |
| `index` | `false` asks search engines not to index the page (default `true`; `false` on a landing page). |
| `search` | `false` excludes this page from the current-language reader search (default `true`; `false` on a landing page), independently of `index` and `ai`. |
| `ai` | `false` excludes this page from `llms.txt` (default `true`; `false` on a landing page), independently of reader search and search-engine indexing. |
| `toc.levels` | The heading levels the page's table of contents lists, as `[2, 3]` (the default). |
| `layout`, `tagline`, `actions`, `features` | A home page: `layout: home`, a tagline, `actions` (`text`, `link`) and `features` (`title`, `details`). `layout: landing` is a [landing page](#landing-pages) with the same hero. |
| `landing` | The `[landing.profiles]` entry a `layout: landing` page loads under, by name. |

GitHub shows frontmatter as a small table at the top of the file.

## Markdown

CommonMark with GitHub's extensions: tables, task lists, footnotes,
strikethrough, alerts and extended autolinks (`www.example.com`,
`https://…` and e-mail addresses written bare).

A page has one top-level heading. When the frontmatter gives the title and
the page also starts with `# heading`, that heading is the page's title and
is not repeated; a second H1 is an error (`multiple-h1`). Markdown and
raw HTML headings share this rule, in their effective document order. Code,
comments and HTML template contents contribute no document heading. A heading
that skips a level (`##` straight to `####`) is a warning (`heading-skip`).

### Heading anchors

Every Markdown heading gets the id GitHub gives it, so `guide.md#some-heading` lands
on both:

- The text is NFKC-normalized and lower-cased. Letters and digits of every
  script stay, so Chinese headings keep their characters (`## 数据 API` →
  `#数据-api`); `-` and `_` stay; each whitespace character becomes `-`;
  every other punctuation mark or symbol is dropped. Nothing is collapsed or
  trimmed. A heading that leaves nothing gets `section`.
- Repeated headings get `-1`, `-2`, … in document order (`steps`, `steps-1`).
- `## Install the CLI {#install}` sets the id explicitly. Only `{#id}` is
  accepted; a class or another attribute is an error (`heading-attribute`),
  and two headings with one explicit id are an error
  (`duplicate-heading-id`). Automatic ids step around explicit ones.

Raw HTML headings keep the `id` their author writes. Without an id they
still participate in heading checks, but have no table-of-contents link.
When a home page merges its leading H1, only that heading's own id moves
to the hero. IDs on its discarded children are removed from the page's
anchor set; a link to one is a `broken-anchor`. Put that target in the
remaining content instead.

> [!WARNING]
> GitHub does not read `{#id}`: it shows the braces as part of the heading
> and derives its own anchor from the whole text. A link to the explicit id
> works on the site and not on GitHub. Prefer the automatic id when the file
> must read on both.

Footnotes link as `#fn-<name>` and back as `#fnref-<name>` (`#fnref-<name>-2`
for a second reference), and a link may target those ids like a heading's;
a footnote id the page does not have is a `broken-anchor`.

### Alerts

GitHub's five alerts render with their own styles and a heading in the
page's language:

```markdown
> [!NOTE]
> Useful to know even when skimming.
```

`[!NOTE]`, `[!TIP]`, `[!IMPORTANT]`, `[!WARNING]` and `[!CAUTION]` each
become `<div class="markdown-alert markdown-alert-<kind>" role="note">`, the
same class names GitHub uses. They render the same on GitHub.

### Links

| Written | Means |
| --- | --- |
| `setup.md`, `../reference/api.md#数据-api` | The page that file becomes, relative to the file the link is in, with the heading's anchor. The anchor must exist. |
| `guide/` | That directory's `index.md`. |
| `#install` | A heading on this page; it must exist. |
| `/guide/setup/` | A page's address, or with a file name (`/logo.svg`) a file in the asset directory. The site's base is added. |
| `diagram.svg`, `files/report.pdf` | A file in the content directory, published under `assets/files/` by its content. |
| `../../src/example.rs` | A file in the repository outside the content: the link becomes `<repo.url>/blob/<ref>/<path>` (`tree` for a directory). Without `[repo]` this is an error (`repo-link`). |
| `https://…`, `mailto:…` | Left as written. Links to other sites get `rel="noopener"`. |

Links to published pages and their anchors are checked, so a site has none that
leads nowhere. `javascript:`, `vbscript:` and `data:` links are errors.

When `content.include` explicitly excludes an existing non-private
Markdown file, a link to that file becomes a repository URL using `[repo]`.
This applies to Markdown links and raw HTML `<a>` links. Its fragment is
kept for the repository; the excluded body and its headings are not read or
published, and no local page is created for it. Without `[repo]`, the link
is a located `repo-link` error. A missing, private or escaping target remains
an error even when `[repo]` is configured.

Those repository links depend on the target's existence and original path
binding. Editing the excluded body does not invalidate the page's rendering
inputs; deleting the target or retargeting its symbolic link does. An
explicit source include or Markdown introduction is different: it reads
the requested source and therefore tracks its bytes.

Write links as relative `.md` paths when the file should also work on GitHub:
there, `setup.md#install` opens the file at that heading. A `/guide/setup/`
link is the site's address and leads nowhere on GitHub.

### Images

`![What it shows](diagram.svg)` is relative to the page; `![…](/logo.svg)`
is a file in the asset directory. The image must exist and is published
with the site. An image from another origin is an error
(`cross-origin-resource`): the site's content security policy would block
it, so download it into the project. An image with no alt text is a warning.

### Raw HTML

HTML in Markdown is allowed and is checked the way a browser reads it
(case, entities and malformed markup included). These are errors, because the
site's content security policy would block them in a reader's browser:

- `<script>` (`inline-script`), `<style>` and the `style` attribute
  (`inline-style`), `on…` event attributes (`event-handler`);
- `javascript:` or `vbscript:` URLs, however they are spelled (`script-url`);
- `<iframe>` (`embedded-frame`), except on a landing page whose profile
  lists its origin, `<frame>` (`embedded-frame`), `<object>`, `<embed>`
  (`embedded-plugin`), `<base>`, `<meta>` and `<link>`;
- anything the page would load from another origin: `img`, `srcset`,
  `source`, `video`, `audio`, `poster`, `track`, SVG `href` and the `url(…)`
  in a class such as `bg-[url(/hero.svg)]` must name a file in the project
  (an image may also use a `data:` URL).

`<a href>`, `<img src>` and the other URLs in raw HTML resolve exactly like
Markdown's, base and all, and keep their `?query` and `#fragment`
(`<use href="/icons.svg#mark">`). A class's `url(…)` does too: the class is
rewritten to the published file, so the stylesheet loads it under the base. An `<img>` without an `alt` attribute is a
warning. A link that opens another site in a new window (`target="_blank"`
or any named target) must say `rel="noopener"` (or `noreferrer`), or it is an
error (`new-window-opener`): the page it opens could otherwise script this
one. Links written in Markdown never open new windows, and those to other
sites always carry `rel="noopener"`.

Forms are allowed, but the strict policy blocks every submission: a form
on an ordinary page is a warning (`form-blocked`) and stays as written. A
[landing page](#landing-pages) may post a form to the origins its profile
lists, and embed frames from them.

### Code blocks

Fenced code is highlighted at build time in these languages; any other tag
(or none) shows the code as plain text:

| Language | Fence tags |
| --- | --- |
| Rust | `rust`, `rs` |
| TOML | `toml` |
| Shell | `sh`, `shell`, `bash`, `zsh` |
| JSON | `json` |
| JavaScript | `js`, `javascript`, `mjs`, `cjs` |
| TypeScript | `ts`, `typescript` |
| HTML | `html` |
| CSS | `css` |
| Markdown | `md`, `markdown` |
| YAML | `yaml`, `yml` |
| diff | `diff`, `patch` |

After the language the info string takes, in any order:

- `title="main.rs"`, shown in the block's header instead of the language;
- `linenos`, which numbers the lines;
- `highlight="1,3-5"`, which marks lines. A range outside the block is an
  error (`code-highlight-range`).

````markdown
```rust title="src/main.rs" linenos highlight="2"
fn main() {
    println!("hello");
}
```
````

Every block has a copy button, which copies the code exactly as written,
without line numbers. GitHub reads the first word as the language and
ignores the rest of the line.

### Code groups

Fenced blocks between two comments become one block with a tab per fence,
labelled by each fence's `title`:

````markdown
<!-- rustify:code-group name="install" -->

```sh title="Cargo"
cargo install --locked rustify-docs-cli
```

```toml title="Config"
schema_version = 1
```

<!-- rustify:end-code-group -->
````

Choosing a tab switches every group of that name, on this page and on the
next ones. Without script, every tab is shown one after another. Groups of
one name must have the same tabs across the site (`code-group-conflict`);
a group that is nested, unclosed, unnamed, holds something other than code
blocks or gives two blocks one title is an error (`code-group`).

GitHub hides the comments and shows the blocks one after another.

### Including code from the project

```markdown
<!-- rustify:include path="src/example.rs" region="main" lang="rust" title="Example" -->
```

The comment, on a line of its own, becomes a code block holding the lines
between `rustify-region:main` and `rustify-endregion:main` in that file. The
markers can sit in any comment syntax (`// rustify-region:main`,
`# rustify-region:main`); the marker lines are left out and the lines between
keep their bytes. `path` is relative to the project root and is required;
without `region` the whole file is included; `lang` defaults to the file's
extension and `title` to its path. The file is read as it is at build time
and never compiled.

These are errors at the comment: a file that does not exist, is outside the
project or is private (`include-missing`, `include-path`); a region that is
missing, repeated or not closed (`include-region`); a directive inside a
paragraph (`include-inline`).

GitHub shows nothing for the comment. To keep the file useful there, link the
source next to it, for example `The source is [on GitHub](../../src/example.rs).`,
which the site turns into the repository address.

### Drafts

A page with `draft: true` is shown, marked "Draft", while you write
(`dev`), and is not part of a production build. A published page that links
to a draft is an error in `check` and `build`, and a warning in `dev`.

## Rust components and demos

### Create a component project

The components template requires an installed CLI with a traceable source
revision, an explicit paired framework revision and its embedded dependency
lock. A development checkout without that frozen pair refuses generation;
it does not write an unlocked project or choose a framework revision for
the author. The published 0.1.32 CLI pins library revision
`0fd34edaffda1926b38ea64c023b2fe1fc5115c5` and embeds its generated dependency
lock. The installed CLI's `--build-info` identifies its C commit. Its public
Git installation has been verified on macOS arm64. A newly generated
project's check, build and browser interactions were verified with 0.1.31,
which pins the same library revision.

With a published paired CLI and stable Rust available, install the browser
target if needed, then create the project:

```sh
rustup target add wasm32-unknown-unknown
rustify-docs new my-docs --template components
cd my-docs
rustify-docs check
rustify-docs dev
```

The template supplies a props-driven counter, a registered source demo, a
home page and two guides, its README, `Cargo.toml`, `Cargo.lock` and the
native/browser entries. The site display name keeps the directory name.
The Cargo package name lowercases ASCII letters, joins runs of spaces,
non-ASCII characters and other separators with `-`, and removes leading and
trailing separators: `My Docs` becomes `my-docs`. An empty result becomes
`docs-site`; a name starting with a digit gets `docs-` prefixed. That name
also replaces the lockfile's root package name. The library stays
`site_docs`, so a directory name that is a Rust keyword does not change the
entry-point imports.

Commit the supplied manifest and lock. Creation itself compiles nothing;
the first `check`, `build` or `dev` compiles two targets and may download
dependencies. Later Rust saves reuse Cargo's compilation outputs, and
Markdown-only saves reuse the compiled renderer and wasm. The CLI includes
wasm-bindgen support; authors need no separate wasm-pack, wasm-bindgen CLI,
Node, Tailwind command or GPU toolchain.

### Register components and demos

A working manual project is the
[component fixture](../tests/docs-ssg/fixtures/components/). Its
[Cargo manifest](../tests/docs-ssg/fixtures/components/Cargo.toml) defines an
independent workspace, locked dependencies, a `docs-render` host binary and
a `cdylib`. Defaults are disabled: `ssr` forwards to
`rustify-docs/site-ssr`, and `hydrate` to `rustify-docs/hydrate`, with the
same features forwarded to directly used SDK, components and Leptos crates.
The wasm dependency is pinned to `wasm-bindgen = "=0.2.128"`.

The site's optional Rust configuration names files inside the project:

```toml
[rust]
manifest = "Cargo.toml"
registry = "src/lib.rs"
```

The registry file contains one `site!` declaration. This excerpt uses the
fixture's [counter view](../tests/docs-ssg/fixtures/components/src/demos.rs#L4)
and [demo function](../tests/docs-ssg/fixtures/components/src/demos.rs#L27):

```rust
pub mod demos;

pub fn site() -> rustify_docs::SiteRegistry {
    rustify_docs::site! {
        components {
            Counter { initial: i32 = 2, label: String = "Counter".to_string() } =>
                |props, children| demos::counter(props.initial, props.label, children),
            #[client_only(class = "min-h-48", message = "This counter starts in the browser.")]
            BrowserCounter { initial: i32 = 2, label: String = "Browser counter".to_string() } =>
                |props, children| demos::counter(props.initial, props.label, children),
        }
        demos { CounterDemo => crate::demos::counter_demo, }
    }
}
```

Names are unique Rust identifiers. Component fields generate typed serde
props with unknown fields rejected; `= expression` supplies a default in
trusted Rust. YAML supplies data, never Rust expressions. The adapter receives
the decoded props and checked child Markdown as a view. The
[host entry](../tests/docs-ssg/fixtures/components/src/bin/docs-render.rs)
calls `serve_site(site())`; the
[wasm entry](../tests/docs-ssg/fixtures/components/src/lib.rs) exports
`docs_hydrate` and calls `hydrate_site(site())`.

Embed a component or reference a registered demo in a complete fence:

````markdown
```rustify-component
name: Counter
props:
  initial: 2
  label: Shared initial props
children: |
  The same **safe child Markdown** appears in each instance.
```

```rustify-demo
name: CounterDemo
```
````

Props are optional when their defaults suffice; empty children are valid.
Missing required fields, wrong types, unknown fields and duplicate YAML keys
are errors. Children use the existing Markdown, link and HTML checks and
cannot contain another interactive island. Keep those blocks adjacent.

Each of these errors keeps the compiler's or parser's message and adds a
one-line repair built from the declarations themselves. An unknown name lists
every registered and builtin component, or every registered demo. An unknown
or refused prop lists the declared fields with their types and whether each
may be left out: its default expression, `optional` for a field declared as
`Option`, `required`, or, for an author type or alias whose omission rules
only the registration knows, the registration file to check. A field of a
plain scalar, `String`, `Option` or `Vec` form gets an example YAML value of
that form (its shape only; a value range is the message's to state), a
builtin's object list points to [Static builtins](#static-builtins), and a
nested serde path is repeated as `Reported props path`. A refused prop of a
registered component points to that prop; a builtin's error, a missing prop
and an unknown name point to the fence. Duplicate keys and malformed fences
say how to write the fence. `check`, `build` and `dev` (the terminal and the
page's problem panel) report the same location and repair.

Diagnostic messages and repairs use Rustify verbatim code fragments. This
notation preserves spaces, backslashes and LF/CRLF line endings; it is not
CommonMark. The JSON structure remains schema version 1 and can be read by
a standard JSON parser. A consumer formatting `message` or `suggestion`
must then read the fragment notation as follows:

- A nonempty code value without backticks has one backtick on each side,
  with no padding. An empty code value has two backticks, two ASCII spaces,
  then two backticks.
- A code value containing backticks has a fence one backtick longer than
  its longest backtick run, with one ASCII space between each fence and
  the value. Fixed prefixes belong inside the value: quote the complete
  heading ID or region marker, including `#` or `rustify-region:`.
- Ordinary author text, such as a file path outside a code fragment, stays
  unchanged unless it contains a backtick. In that case the whole value
  uses the same fragment notation and the panel's existing monospace style.

Scan maximal backtick runs from left to right. Pair an opening run with the
first later maximal run of exactly the same length; runs of other lengths
are content. An unmatched run stays ordinary text and scanning continues.
For a single-backtick pair, keep all enclosed characters. For a longer pair,
remove exactly one leading and one trailing ASCII space only when both are
present and the enclosed text has at least two characters, including when
it consists entirely of spaces. Preserve everything else, including newlines.
An empty fragment creates an empty code node. Keep non-backtick prose between
fragments so their fences do not merge; adjacent code values must be quoted
as one complete value.

These examples show the encoded fragment between the arrows; the quoted
values on either side use JSON string notation:

````text
""       -> ``  ``         -> ""
" a "    -> ` a `          -> " a "
"a`b"    -> `` a`b ``      -> "a`b"
"a``b"   -> ``` a``b ```   -> "a``b"
````

The terminal and JSON carry these delimiters; the panel removes them and
shows the original value. The verbatim guarantee covers Rustify's own
templates and the author values they insert. External messages from serde,
YAML parsers, rustc and I/O are passed through unchanged. The panel uses the
same reader for those messages: unmatched backticks remain visible, while
paired backticks are still formatted as code and can remain ambiguous.

A demo directly references a site-local function with a
`crate::module::function` path. Its Function and Full file tabs show and copy
the original source, including Unicode and line endings, from the same files
compiled for this build. Anonymous demo closures and generated or external
functions cannot provide that source identity.

To add a demo, write its function in a declared project module, register
that path in the existing `demos` block, then add a `rustify-demo` fence with
the registered name. With `dev` running, save the source and Markdown to
rebuild. Do not paste a second copy of the function into the page: the CLI
captures the function slice and containing file before compilation and
checks their generation before publishing. A `rustify:include` directive
instead displays a non-interactive source excerpt without registering a
demo.

Run the working fixture with the current checkout's CLI, from the repository
root; its lockfile and required wasm target must be available:

```sh
mbx build --locked -p rustify-docs-cli --profile docs-cli
RUSTIFY_DOCS_CARGO=mbx target/docs-cli/rustify-docs check --config tests/docs-ssg/fixtures/components/rustify-docs.toml
RUSTIFY_DOCS_CARGO=mbx target/docs-cli/rustify-docs build --config tests/docs-ssg/fixtures/components/rustify-docs.toml --out target/docs-ssg/components-manual
```

Rust checks build both targets with `--locked` and may populate Cargo and
`.rustify-docs` caches. Cargo may need to fetch dependencies on the first run.
They leave author source, configuration, the dependency lock and any
existing output unchanged. The CLI checks the
cold declaration against the compiled registry, batches static rendering and
keeps the matching wasm, generated glue and SDK runtime assets together.
Ordinary pages still contain no islands and request no wasm.

Initial rendering is synchronous and deterministic. Initial
`Resource`/`Suspense`/`Transition`/`ErrorBoundary` work that needs shared
resource data, errors or incomplete chunks is rejected. Start that work after
attachment, as the fixture's
[asynchronous action](../tests/docs-ssg/fixtures/components/src/demos.rs#L35)
does. Floating layers must start closed and toast regions empty. A
`client_only` declaration instead publishes its sized placeholder and message;
the browser creates its body after attachment. It supplies no SSR body.

Without JavaScript, the SSR view and source remain readable and the outer
fieldset disables input. Loading keeps the canvas locked. Successful
attachment enables it; a startup or later wasm failure restores every island
in that instance to its initial view and retains the source and shell.
Reload the page to retry.

When the reader's system asks for reduced motion
(`prefers-reduced-motion: reduce`), every transition inside an island's SDK
scope completes at once, the author component's own elements included, and
the SDK spinner stands still beside its status text. The site stylesheet
does this with important zero durations, so a utility class cannot undo it.
Keyframe animations an author component declares itself keep running; put
them behind `prefers-reduced-motion: no-preference`.

### Compilation profiles and version pairing

Keep the template's four profiles when adapting a Rust project:

| Command | Native `docs-render` | Browser library |
| --- | --- | --- |
| `check`, `dev` | `docs-host`: inherits dev, unwind, incremental, opt-level 1, debug 0, no LTO | `docs-wasm`: inherits dev, abort, incremental, opt-level 1, debug 0, no LTO |
| `build` | `docs-host-release`: inherits release, unwind, debug 0, thin LTO | `release`: abort, debug 0, thin LTO, one codegen unit |

The CLI compiles the native binary with `--no-default-features --features ssr`,
then the library for `wasm32-unknown-unknown` with
`--no-default-features --features hydrate`. Both use `--locked`. The CLI
itself uses the separate `docs-cli` unwind profile, which lets `dev` survive
a failed generation; that does not change the browser's abort requirement.

The generated project pins `rustify-docs`, `rustify-ui` and
`rustify-components` to one framework git revision. The CLI has its own
source revision and explicitly records the framework revision it accepts;
these revisions can differ. Its source identity, expected framework, helper
protocol, runtime assets and `wasm-bindgen = "=0.2.128"` must agree. A
mismatched or missing pair is rejected before rendering. Local framework
paths are accepted only for the development CLI's exact repository fixture,
not as a substitute for the published pair.

Before an upgrade, retain the CLI installation revision, `Cargo.toml`,
`Cargo.lock` and the complete successful output. Select a published pair,
install its CLI using that release's recorded recipe, update all three
framework dependencies together and explicitly regenerate the dependency
lock as part of the upgrade. Run `check` and `build`, then restart `dev` to
discard the previous compiled generation. Ordinary authoring commands do
not update the dependency lock for you.

To roll back, reinstall the retained CLI revision and restore its matching
manifest and lock. Restore the complete prior output with its manifest,
wasm, glue, scripts and fonts. Combining files from different builds can
pair new source with old executable code.

### Custom pages and layout slots

Custom pages register a canonical route, title, optional description and a
static function in the same `site!` declaration:

```rust
pages {
    ExamplesPage { route: "/examples/", title: "Examples", description: "Two counters with shared source." } => crate::demos::examples_page,
}
slots {
    footer => crate::demos::footer,
}
```

The template's counter demo can be reused from those functions:

```rust
pub fn examples_page(context: rustify_docs::site::RenderContext) -> impl IntoView {
    let first = context.demo("CounterDemo");
    let second = context.demo("CounterDemo");
    view! {
        <section>
            <h2 id="paired-counters">"Two independent counters"</h2>
            {first}
            {second}
        </section>
    }
}

pub fn footer(context: rustify_docs::site::RenderContext) -> impl IntoView {
    view! { <p><a href=context.base>"Home"</a></p> }
}
```

As in the template's
[src/demos.rs](../crates/rustify-docs-cli/templates/components/src/demos.rs), these functions need
`use leptos::prelude::*;`. `RenderContext` provides `route`, `lang` and
`base`; each `demo(name)` call uses the same island and source wrapper as a
fence, with independent IDs and state. `route` omits the deployment base
and language prefix; `lang` is the target UI language, including when the
function supplies a fallback version.
Use `base` when constructing links to other registered routes. The page is
written as `/examples/index.html` and uses the normal shell, navigation and
theme; it is not rendered by an application server for each request.

Routes and document titles are checked across Markdown and registered
pages. A page without an explicit description remains browsable. Custom
headings, links, images, classes and explicit demos enter the same final
document checks as Markdown content; a custom function does not bypass the
CSP or resource graph.

Registered page metadata also accepts literal optional `draft`, `index`,
`search` and `ai` booleans, with the same defaults as Markdown. A default
draft keeps every language version out of the production site and both
discovery indexes. A fallback inherits the source page's switches. A custom
page needs a nonblank explicit description to appear in `llms.txt`; text in
its HTML is not used as an inferred description.

The seven slots accept functions with the same `RenderContext`:

| Slot | Replaces or inserts |
| --- | --- |
| `header` | The brand area; framework navigation and mode controls remain |
| `footer` | The footer content, including on the 404 page |
| `sidebar_top`, `sidebar_bottom` | Content before or after sidebar navigation |
| `before_content`, `after_content` | Content before or after the article |
| `not_found` | The generated 404 body; the surrounding shell remains |

Unspecified slots keep their defaults. A slot can call `context.demo`, so a
demo in an unconditional shared slot can make every page an island page.
Keep shared reading content static or restrict the demo by `context.route`.
Add registered pages to a sidebar group with `routes`, for example:

```toml
[[nav.groups]]
label = "Examples"
routes = ["/examples/"]
```

### Reuse a Markdown introduction

A page or slot can request the source introduction through its
`RenderContext`:

```rust
let introduction = context.markdown_intro("README.md");
```

Place that returned view in the page, as the repository's
[home function](../site/src/pages.rs) does. The path is relative to the
project root, not `content.dir`. The CLI reads the UTF-8 Markdown after its
H1 and before its first H2, omitting both headings; without a later H2, it
uses the remainder after H1. A source with no Markdown H1, or an H2 before
that H1, is an error. The fragment uses the same Markdown writer,
highlighting, HTML policy and resource checks as ordinary content, with
diagnostics retaining their original source offsets.

Links keep the source file's meaning. In a root README, a link to a selected
guide becomes the guide's site route, while a link to another README section
or an unpublished file uses `[repo]`. A README's `#quick-start` therefore
still targets the repository README, rather than an unrelated heading on
the enclosing home page. Images resolve from the source file's directory;
code include directives keep their existing project-relative `path`
contract. These explicit reads enter the shared dependency/resource checks.

The Markdown file is a build input: changes rerender the introduction
without compiling its prose into browser wasm, and the `.md` file is not
published as a standalone asset. A registered root page supplies its own
complete home composition around the fragment. The repository's
[registration](../site/src/lib.rs) and home function provide the H1,
positioning sentence, Quick start / Components actions and feature list,
while keeping the normal shell and slot behavior. No copied README prose
is needed in Rust or in a second Markdown page.

### Static builtins

`rustify-component` also accepts three builtins without an author Rust project
or an interactive island. Their props are fixed:

| Name | Props |
| --- | --- |
| `Tabs` | `tabs`: a list of objects with string `label` and `content` fields. |
| `CardGrid` | `cards`: a list of objects with string `title`, `link` and `description`; `columns`: 1, 2 or 3, default 2. |
| `Steps` | `items`: a list of objects with string `title` and `description` fields. |

These text fields are plain text; rich Markdown belongs in `children`.
Card links use the normal page/link graph. Tabs show every panel without
JavaScript; the theme script adds independent tab controls. The
[builtin fixture page](../tests/docs-ssg/fixtures/components/content/builtins.md)
shows two tab groups, cards, steps and rich children. It loads no wasm even
inside the configured Rust project.

## Offline search and AI discovery

These features require the P4 CLI/framework candidate; the public P1 pin in
the installation section predates them. Content-only sites still compile no
Rust and need no Node, search service or search wasm.

Open **Search**, or press Cmd/Ctrl+K. The input receives focus immediately,
including while the same-origin module, Worker and current-language JSON
load. Arrow keys choose a result, Enter follows its ordinary page/section
link, Tab reaches the controls, and Escape closes to the element that opened
search. A failed load offers **Retry** and retains the input; an empty match
list instead suggests another word or prefix. Search runs independently of
interactive examples, so a failed island does not own the search session.

Chinese queries use whole words; English matches ignore ASCII case and
accept prefixes at word starts. All query words must occur within one
section's title, heading path or text. Title and heading matches rank ahead
of repeated body matches. Results are grouped by page, twenty at a time;
**Show more** keeps the actual total and current selection. Highlighting
uses the original text, including in titles.

Search stays within the page's UI language, including a legacy per-page
`lang` without translation pairing. With languages configured,
Missing translations appear with their source language and link to that
language's fallback reading address. Custom page bodies, home features,
ordinary code and initial component text are searchable. Navigation,
layout slots, example status, source paths, duplicate full-file panels and
copy controls are excluded. Only headings with an actual document ID get
their own section link. Drafts, `search: false` and 404 pages are excluded.

No search data or module is requested before opening search. Once an index
has loaded, queries and pagination use local Worker memory with no network
request. Query text is neither uploaded nor persisted. “Offline” describes
that local querying; a first visit without a connection cannot fetch an
uncached site. There is no Service Worker or persistent search cache.
Without scripts, the search control remains disabled with an explanation;
the document and ordinary navigation still work.

Each configured language also gets `llms.txt` at its root. Without i18n
declarations, the site keeps one root AI index for its unpaired pages.
It lists the site name and
description, groups entries by that language's navigation and appends
allowed unlisted pages under **Other pages**. Entries contain a title,
public HTML URL and nonempty description. Real translations use their
translated URL; missing translations identify the source language and link
to the actual source address, rather than the fallback UI address. With
`site.url`, URLs are absolute; without it they remain root-relative under
`site.base` and use the existing missing-URL warning.

AI descriptions try a trimmed explicit description, then the first body
paragraph. Empty or whitespace-only values do not mask the next candidate.
A Markdown home with neither uses its tagline, then the source-language
`ai.home_description` message (“Homepage of {site}.” in English, “{site}首页”
in Chinese). This fallback does not change the page's SEO description.
Other AI-enabled Markdown pages with neither candidate fail with
`missing-description`; add text or opt out with `ai: false`. Registered
pages require an explicit description. Drafts and 404 pages never appear.
The build emits no Markdown copies or `llms-full.txt`.

HTML, search JSON and AI indexes are rebuilt from the same candidate and
published together. A failed candidate leaves the previous complete output.
`llms.txt` is reserved at every language root, including its directory
form; authored resources and routes cannot take the framework's
`assets/search.` or `assets/search-worker.` names.

## Landing pages

A landing page is a Markdown page for paid traffic: `layout: landing`. It
shows the home page's hero without the site's navigation, and it is the one
kind of page that may load another origin's measurement script, send its
requests, embed a form service's frame and post a lead form. Landing pages
come with the first CLI release after 0.1.32: the pinned 0.1.32 release
above reports `[landing]` as an unknown key. Until that release, a CLI built
from the repository's current source has them, although it still reports
version 0.1.32; `rustify-docs --build-info` names the source it was built
from.

What a landing page may load is never written on the page. The
configuration declares named profiles, and a page selects one by name:

- Only a `layout: landing` page that selects a profile is served under a
  wider policy, and only by what that profile lists. Every other page, the
  home page and every `404.html` keep the strict policy byte for byte.
- A profile can add origins to five directives: `script-src`, `connect-src`,
  `img-src`, `frame-src` and `form-action`, and load scripts in `<head>` by
  address.
- Nothing ever allows inline script or style, a nonce or a hash, `eval`,
  plugins or `<base>`, and every page keeps `frame-ancestors 'none'`.

### Profiles

```toml
[landing.profiles.ads]                    # a name: a-z, 0-9 and -, up to 32
carry = ["gclid", "gbraid", "wbraid", "bd_vid", "utm_source", "utm_medium", "utm_campaign", "utm_term", "utm_content"]

[[landing.profiles.ads.scripts]]          # loaded in this order, after the theme's
src = "https://tag.example.com/tag.js?id=T-1"
load = "async"                            # async, or defer (the default)

[[landing.profiles.ads.scripts]]
src = "landing/init.js"                   # a .js file in the project, published by content

[landing.profiles.ads.sources]            # every list is optional
script = []                               # each external script's origin is added already
connect = ["https://*.tag.example.com"]
img = ["https://tag.example.com"]
frame = ["https://forms.example.com"]
form = ["https://leads.example.com"]

[[landing.profiles.ads.footer]]           # the page's footer links; like nav.top
label = "Privacy"
link = "privacy.md"
```

A source is an origin: `https://host`, `https://host:port` or
`https://*.suffix` (any host under `suffix`, not `suffix` itself; the suffix
has at least two labels). Hosts are ASCII; write an international domain in
its `xn--` form. `http://` is accepted only for `127.0.0.1` and
`localhost`, for local testing. A path, query, fragment, user name, IP
address (other than `127.0.0.1`), keyword such as `'unsafe-inline'`, bare
scheme, `data:` or `blob:` is an error (`landing-source`): CSP matches a
host-source by domain, and anything wider would widen every directive it is
in. Origins are compared in lower case without the scheme's default port,
and repeats are dropped.

A script's `src` is an `https://` address (with the same host rules, so
`http://` only on `127.0.0.1` and `localhost`; its origin joins
`script-src`) or a project-relative `.js` file, which must exist and is
published under `assets/files/` by its content (`landing-script`,
`landing-script-missing`). Keep such a file outside the asset directory, as
in `landing/init.js`: a file under `static/` is also published at its own
path. There is no inline script:
put a vendor's inline snippet in such a file (the examples below do).
A profile that no page selects is a warning (`landing-profile-unused`).

A page under a profile is served under this policy; the bracketed parts
appear only when the profile lists something for them:

```text
default-src 'none'; script-src 'self' 'wasm-unsafe-eval' [S…]; style-src 'self'; img-src 'self' data: [I…]; font-src 'self'; connect-src 'self' [C…]; media-src 'self'; worker-src 'self'; [frame-src F…;] object-src 'none'; base-uri 'none'; form-action <'none' | 'self' A…>; frame-ancestors 'none'
```

`form-action` is `'none'` unless the profile lists form targets; then it is
`'self'` and them, so a form may also post to the site and an endpoint may
send the reader back to a page on it. A profile that lists no source at
all (only `carry`, say) leaves its pages under the strict policy.

### The page

```markdown
---
title: Ship docs in an afternoon
description: A landing page for paid traffic.
layout: landing
landing: ads
tagline: Markdown in, a fast static site out.
actions:
  - text: Request a demo
    link: "#lead"
features:
  - title: Strict by default
    details: Every other page keeps the strict policy.
---

## Request a demo

<form id="lead" method="post" action="https://leads.example.com/lead">
<label for="lead-email">Work email</label>
<input id="lead-email" name="email" type="email" autocomplete="email" required>
<input type="hidden" name="gclid" data-rd-carry>
<button type="submit">Request a demo</button>
</form>
```

`landing:` names a declared profile (`landing-profile` lists the declared
ones) and is allowed only with `layout: landing`
(`landing-without-layout`). A landing page without `landing:` has the
layout under the strict policy.

The layout:

- The header holds only the brand, the logo and the site's name, and it is
  not a link, so the page has one way on. There are no header links,
  search, language or colour-mode switch; colours follow the reader's
  system setting, or a mode they chose elsewhere on the site. A `header`
  layout slot still replaces the brand.
- Then the hero as on a home page (`title`, `tagline`, `actions`,
  `features`; a leading `# Title` merges into it), then the page's
  Markdown. No sidebar, table of contents, previous/next links, date or
  edit link; the page is in no sidebar group (so it raises no
  `not-in-nav`), and it may omit `description`.
- The footer lists the profile's `footer` links, or a `footer` slot. With
  neither the page has no footer.
- `index`, `search` and `ai` default to `false` on a landing page: it is not
  indexed by search engines, in reader search or in `llms.txt` unless its
  frontmatter says `true`.

### Forms

A native form is the dependable way to collect a lead: it posts without
script, and a failed measurement script never stops it.

- On a landing page, a form's `action` (and a button's `formaction`) must be
  on an origin the profile lists in `sources.form` (`form-origin`), or be a
  page on the site, which resolves like a link and must exist
  (`broken-link`). A profile with no form targets admits no form.
- A form's target may carry a query string, which is kept: it is part of
  what the reader sends. `action="?campaign=paid"` submits to the page
  itself with that query, and `action="/thanks/?campaign=paid"` submits to
  the thanks page with it.
- The endpoint is yours or a form service's: it receives the
  `application/x-www-form-urlencoded` POST, stores the lead, and answers
  with a `303` redirect to a thanks page, on the site (which `'self'`
  admits) or on a listed origin. A thanks page is usually a landing page of
  its own, under the same profile, so it can report the conversion.
- On any other page the policy blocks every form, as it always did: a form
  there is a warning (`form-blocked`) and is published as written.
- Give a form an explicit `action`; the build checks forms through it.

### Embedded forms

An `<iframe>` is allowed only on a landing page, with a `src` on an origin
in `sources.frame` (`frame-origin`; elsewhere `embedded-frame`), no
`srcdoc`, and a `title` naming what it holds (`frame-title` warns without
one). The framed page is under its own service's policy, and the service
must allow being framed; if it refuses, the frame is blank and a native form
is the alternative. Services that embed by script and inline styles cannot
be used, because the page's `style-src` is never widened. Give a frame below
the fold `loading="lazy"`: carrying parameters into it (below) replaces it
with a copy at the new address, and a lazy frame that has not started
loading loads once, with them. A script that holds the frame element should
look it up again once the page has loaded.

### Carried parameters

`carry` lists the query parameters an advertising click adds (`gclid`,
`bd_vid`, `utm_*`, …). When a reader arrives with any of them, the theme
script copies each one the address has into the elements the page marks
with `data-rd-carry`:

- `<input type="hidden" name="…" data-rd-carry>`: the field named after the
  parameter takes its value, replacing what the field had;
- `<a href="…" data-rd-carry>` and `<iframe src="…" data-rd-carry>`: the
  parameters are added to the http(s) address, unless it has them already;
- the hero's `actions`, which are marked for you.

Parameters the profile does not list are never copied. Values are copied
as text, nothing is stored (no cookie, no `localStorage`) and nothing is
sent. Without script the page and its form still work; only the copied
values are missing.

### Hosting landing pages

Each landing page carries its policy in its `<meta>` element, and the
manifest records its header form under `page_policies`, by the page's path
in the output. `preview` and `dev` send each page's own header; every other
response keeps the strict one.

Browsers enforce the header and the `<meta>` together, so a host that sends
the strict `Content-Security-Policy` header on every path blocks a landing
page's scripts and forms: the page shows, its tag reports nothing and its
form does not submit. A host that sets headers must send each path in
`page_policies` its policy from the manifest (or no CSP header on those
paths); hosts that cannot set headers, such as GitHub Pages, need nothing.

### Privacy and consent

A script a profile loads runs with the page's full access: it can read the
page and what readers type into its forms. That is the cost of measurement,
and why it is confined to the landing pages that select the profile. Keep
the list short and review it like code. Consent banners, Consent Mode and
the privacy notice are the site's responsibility: put a consent tool's
setup in the profile's own script file, and link the privacy notice in the
profile's footer.

### Example: Google tag (GA4 and Google Ads)

The sources are those [Google's CSP guide](https://developers.google.com/tag-platform/security/guides/csp)
lists for GA4 with advertising features and for Google Ads conversions and
remarketing, plus two the tag was seen to use beyond it:
`https://googleads.g.doubleclick.net` as a script and
`https://ad.doubleclick.net` as an image. CSP has no wildcard for the end of
a host name, so each Google country domain a reader may be sent through,
such as `https://www.google.co.uk` or `https://www.google.com.hk`, must be
added to `img` and `connect` by itself.

```toml
[landing.profiles.google]
carry = ["gclid", "gbraid", "wbraid", "utm_source", "utm_medium", "utm_campaign", "utm_term", "utm_content"]

[[landing.profiles.google.scripts]]
src = "https://www.googletagmanager.com/gtag/js?id=G-XXXXXXXXXX"
load = "async"

[[landing.profiles.google.scripts]]
src = "landing/gtag-init.js"

[landing.profiles.google.sources]
script = ["https://www.googleadservices.com", "https://www.google.com", "https://googleads.g.doubleclick.net"]
img = ["https://www.googletagmanager.com", "https://*.google-analytics.com", "https://*.google.com", "https://*.g.doubleclick.net", "https://www.googleadservices.com", "https://pagead2.googlesyndication.com", "https://ad.doubleclick.net"]
connect = ["https://www.googletagmanager.com", "https://*.google-analytics.com", "https://*.google.com", "https://*.g.doubleclick.net", "https://www.googleadservices.com", "https://pagead2.googlesyndication.com", "https://ad.doubleclick.net"]
frame = ["https://www.googletagmanager.com"]
form = ["https://leads.example.com"]         # your lead endpoint, as in the page above
```

`landing/gtag-init.js` is Google's inline snippet as a file. It
reports a conversion on the page that marks one, here a thanks page with
`<p data-conversion>…</p>`:

```js
window.dataLayer = window.dataLayer || [];
function gtag() { dataLayer.push(arguments); }
gtag("js", new Date());
gtag("config", "G-XXXXXXXXXX");
gtag("config", "AW-XXXXXXXXX");
if (document.querySelector("main [data-conversion]")) {
  gtag("event", "conversion", { send_to: "AW-XXXXXXXXX/XXXXXXXXXXXXXXXXXXXX" });
}
```

### Example: Baidu Tongji

Baidu publishes no CSP list. Its `hm.js` sends each page view as an image
request to its own origin, and the time on the page as a request from
script to the same origin:

```toml
[landing.profiles.baidu]
carry = ["bd_vid", "utm_source", "utm_medium", "utm_campaign", "utm_term", "utm_content"]

[[landing.profiles.baidu.scripts]]
src = "https://hm.baidu.com/hm.js?0123456789abcdef0123456789abcdef"   # the site's 32-character id
load = "async"

[landing.profiles.baidu.sources]
img = ["https://hm.baidu.com"]
connect = ["https://hm.baidu.com"]
form = ["https://leads.example.com"]         # your lead endpoint, as in the page above
```

`hm.js` also reads some values with `new Function`, which a landing page's
policy never allows (no profile can add `'unsafe-eval'`): the console
reports it as a `script-src` violation for `eval`, and page views are still
recorded.

After deploying either profile, open a landing page with the browser's
console showing: a request the profile does not admit is reported there as a
Content Security Policy violation, naming its origin and directive. A host
that sends the page's policy as a header as well as the `<meta>` reports
each blocked request once for each.

## What a build writes

```text
dist/
  index.html, guide/index.html, …     one directory and index.html per page
  404.html                            the not-found page
  zh/404.html, fr/404.html, …          one error page per configured non-default language
  assets/site.<hash>.css              the stylesheet: the classes the pages use, minified
  assets/mode.<hash>.js, site.<hash>.js   the theme's two small scripts
  assets/search.<lang>.<hash>.json      each UI language's public search data
  assets/search.<hash>.js              the search module, loaded on demand
  assets/search-worker.<hash>.js       the same-origin search Worker
  llms.txt, zh/llms.txt, …             each language's AI discovery file
  assets/files/<name>.<hash>.<ext>    every file a page shows or links to
  assets/share.<hash>.png             the default link-preview image, without site.image
  logo.svg, favicon.ico, …            the asset directory, at its own paths
  sitemap.xml, robots.txt             the site's files for crawlers
  .nojekyll                           tells GitHub Pages to publish the files as they are
  rustify-docs-manifest.json          what this output is (below)
```

- Every page's addresses are absolute under the base (`/docs/guide/`), so the
  output works under that path on any host without rewrite rules, and the
  not-found page works at any depth.
- A file a page references is named by its content. Rebuilding unchanged
  content writes the same bytes; changing an image renames it and changes
  only the pages that show it; changing the logo changes every page (each
  shows it) and nothing else.
- Each page has a unique `<title>` within its configured language (`Page |
  Site`; two pages with one title are told apart by their parent page), its description, Open Graph and
  Twitter card tags, and when `site.url` is set, its canonical address and a
  preview image (`site.image`, or a built-in card). `index: false` adds
  `<meta name="robots" content="noindex">` and keeps the page out of the
  sitemap.
- In a git repository a page shows the date of the last commit that changed
  its file, in UTC. Outside one, or without git, pages have no date and the
  build warns once.
- Without `site.url` there is no canonical address, no preview image and no
  sitemap, and the build warns which setting to add; `robots.txt` is still
  written.
- Drafts, Markdown sources, included source files and private files are
  never in the output. The manifest lists exactly what is.

The manifest records `schema_version`, `mode` (always `production`), `base`,
`cli_version`, a `build_id` and every other file with its SHA-256 and size.
Multilingual builds also record `locale_not_found`, an explicit mapping
from each prefix (including the empty default) to its generated error file.
Sites with landing pages also record `page_policies`: each landing page's
path and the policy a host should send with it, for the pages whose profile
widens it; a site without any has no such field.
It holds no time, machine path or environment value. Pages with islands also
reference a fingerprinted `assets/islands/<hash>/` group containing their
wasm, glue, snippets and locked SDK runtime assets. Rust source is displayed
inside the demo's HTML; standalone Rust source files are not published.

### How a build replaces its output

A build writes the whole new site into a private work area beside the
output, `<output's parent>/.rustify-docs/<name>-<hash>/`, and only when all
of it is written and checked does it swap: the old output moves aside, the
new one moves into place, the old one is deleted. A build that fails, at any
point, leaves the previous output byte for byte (a full disk or a refused
permission included); one that is killed is finished or undone by the next
build: killed before the swap, the old output stays; between the two
renames, the next build puts it back; after them, the next build finishes
the clean-up. Until then `preview` refuses the output and says a build
stopped half-way. A first build that fails leaves no output at all.

One output has one writer: a build takes the work area's lock, a preview
shares it. A build while a preview of the same output runs (or another
build) stops at once and says so, rather than waiting; the lock goes with
the process holding it, so a killed build never leaves it taken. A build
replaces only a directory it can tell is a previous production build: one
that is not empty and has no manifest, or whose manifest is damaged or not
`production`, is never replaced (remove it yourself), and neither is a
symbolic link (give the directory it points to). The output cannot be the
project, a directory holding it, the content or asset directory, or inside
a `.git` or `.rustify-docs` directory. These refusals come before anything
is rendered.

Ignore `dist/` and `.rustify-docs/` in version control; `new` writes a
`.gitignore` that does. With `--out` elsewhere, ignore that directory and
the `.rustify-docs/` beside it.

## Writing with `dev`

`dev` serves both content and Rust sites from memory on the loopback
interface: at `dev.port` (default `4188`; the system picks another port when
that one is taken, and `dev` says which), or at `--port`, which must be free
(`--port 0` lets the system pick). It installs the watcher and prints the
address before the first build starts, then keeps running until stopped.
Open that address during compilation: before any successful generation,
the page says **Building your site** and updates when ready. A first Rust
build compiles the native renderer and browser module and can take longer
while dependencies are fetched and compiled.

- **Saves of selected inputs rebuild.** It watches `rustify-docs.toml`, the
  asset directory, theme documents and actual source/resource dependencies.
  Without `content.include`, content discovery remains recursive. With an
  explicit selection, it tracks the selected Markdown files and their
  actual byte dependencies, including introductions, images and downloads,
  without watching the whole content tree. Unselected Markdown and
  unreferenced files do not rebuild the site when their bodies change; for
  this repository's selection, that includes unreferenced validation files.
  Repository-only targets are tracked by existence and path binding:
  deletion, symbolic-link retargeting, replacement of an ancestor directory
  or restoration of a missing dependency triggers a new build. An excluded
  file explicitly used by an introduction or source include is a byte
  dependency and does rebuild. Rust sites also watch their registry,
  source, manifest, lock, local path dependencies and discovered build-script
  inputs. Output, `target/`,
  `.git/` and CLI private work areas are excluded. Saves that arrive
  together, or while a build runs, make one build afterwards; the last save
  always shows. Each build reads all of its inputs
  again, so nothing is missed when the system drops change events.
- **Compilation is reused when its inputs match.** A Markdown, ordinary
  configuration, theme or referenced-asset save reruns analysis and the
  precompiled renderer without invoking Cargo again when Rust source,
  manifests, lock, build inputs and compiler settings are unchanged.
  Rust changes rebuild the two targets serially with their development
  profiles, reusing Cargo's incremental outputs. The terminal reports
  native compilation, browser compilation and page rendering separately.
  The dev event stream records the generation, each target's result and
  completion time in epoch milliseconds; completion of compilation is
  distinct from a complete new page being served.
- **Building retains the last successful version.** Once one exists, the
  page remains visible with the **Building** state and current phase in the
  problem panel. Its HTML, displayed Rust source, wasm and glue stay
  together until a complete new generation succeeds.
- **Only the pages a build changed reload**, and they come back to where you
  were reading (and to the code tabs you chose). A paragraph reloads its own
  page; an included file or an image, the pages that use it; the
  configuration or the logo, every page.
- **A build that fails changes nothing you see.** The page stays the last
  version that built; a panel at the bottom right (a sheet along the bottom
  on a phone) lists every problem by file, with its line and column and what
  to do. Hide it and a marker keeps the count and opens it again. Saving the
  same mistake again updates it; the next build that succeeds clears both by
  itself, without taking focus. The terminal shows the same problems.
  Warnings alone wait behind the marker.
- **While nothing has built** (the first build failed), every address shows
  a page saying so, with the panel; it becomes the site at the first build
  that succeeds.
- **Saves during compilation cannot publish a mixed generation.** Rust
  source and demo slices are captured before compilation and checked again
  before rendering and committing. Content, includes and the imported
  theme are bound to the reads used for that render too. If those inputs
  change during a round, the candidate is discarded and the latest inputs
  are rebuilt. A failed or unreadable input leaves the last successful
  version in service and remains watched for recovery.
- **A failure inside `rustify-docs`** (a panic) is one more problem in the
  panel: the session goes on, and the next build starts afresh. This needs
  the command installed with `--profile docs-cli`; a build that aborts on a
  panic refuses to run `dev`.
- **Drafts are shown**, marked "Draft" in the page and in the sidebar, and a
  page linking to one gets a warning (in `check` and `build` it is an
  error).
- **Source and published output are unchanged.** Content sites compile no
  Rust; Rust sites may write Cargo and `.rustify-docs` private compilation
  caches. `dev` does not rewrite source, configuration, `Cargo.lock` or the
  production output, and takes no lock on that output, so
  `rustify-docs build` can run beside it. Pages served by `dev` load its
  panel script from
  `/__rustify-docs/`, which a production build never contains.
- Pages show the last-updated dates the repository had when `dev` started.

## Previewing and hosting

`preview` serves an existing build and nothing else. It refuses a directory
without a production manifest, one built for a different base than
`--base` asks for (and says which base it was built for), and an output
whose build stopped half-way. It listens on `127.0.0.1:4189` by default (the
system picks another port when that one is taken; `--port 0` always lets it
pick) and prints the address. `--host` takes an address to listen on, such
as the machine's LAN address for a phone; it serves the same read-only
files.

For multilingual builds, preview freezes the error-file mapping and bytes
from the output manifest while holding its shared lock. It can preview a
copied `dist` with `--dir` without the source project. Changing the project's
language configuration does not change an existing output's behavior.
A damaged mapping or a missing declared error file refuses startup and asks
for a rebuild. Outputs without the field retain the default-error behavior;
an ordinary directory containing `foo/404.html` does not declare a language.

Preview answers a missing address under a configured prefix with that
language's error page and HTTP 404, including without scripts. A host with
one fixed error file serves the root `404.html`; its external head script
selects the matching static language body before attaching any islands.
Without scripts, the default error and links to every language's home remain
readable. Both paths retain the real HTTP 404 status.

Any static host serves the output as it is, provided it:

- serves the output under the base it was built for, with directory
  addresses answered by their `index.html` (and a directory named without
  its slash redirected to it);
- answers an address with nothing behind it with status 404 and `404.html`;
- sends files with their types (`.css`, `.js`, `.svg`, `.json`), including
  `application/wasm` for `.wasm` on island pages.

The site's content security policy is in every page as a `<meta>` element.
A host that can set response headers should also send it as the
`Content-Security-Policy` header, as `preview` does: only the header can
carry `frame-ancestors 'none'`, which stops other sites from framing the
pages. GitHub Pages cannot set headers and uses the `<meta>` alone; it
serves `404.html` for missing addresses, and `.nojekyll` makes it publish
the files as they are. Landing pages under a profile carry a wider policy
of their own; a host that sends headers must send theirs from the
manifest (see [Hosting landing pages](#hosting-landing-pages)).

```text
default-src 'none'; script-src 'self' 'wasm-unsafe-eval'; style-src 'self'; img-src 'self' data:; font-src 'self'; connect-src 'self'; media-src 'self'; worker-src 'self'; object-src 'none'; base-uri 'none'; form-action 'none'; frame-ancestors 'none'
```

### Official site delivery

The repository's official sources configure English and Chinese, with real
Chinese home and quick-start pages. Other guides and component categories
keep explicit English-content fallback inside the Chinese reading shell.
The content allowlist excludes plans, reports and validation sources.

The existing `docs-pages.yml` workflow builds a complete fixed Git candidate
C with its paired library R. It changes only the official consumer's three
dependency lines and generated lock, checks the remaining frozen inputs,
then uploads and deploys the same-run artifact. Pages must already use the
Actions source and the existing `github-pages` environment; the workflow
does not enable Pages. This repository now has that configuration. The
archive includes `.nojekyll`, and its actual members are checked against
the complete manifest before upload.

The [public site](https://rockie.github.io/rustify-ui/) and public Git
installation of the pinned 0.1.32 CLI have been verified. The site was
published from C2 `88566ba1ed08223a5834f794840fff5e48f4ba8a` by workflow run
[39388531674](https://github.com/rockie/rustify-ui/actions/runs/39388531674); its
[release record](plan/DOCS-SSG-P4-gaps.records/M3.md) identifies the online
file and browser checks. The previous release, C1
`1146b7d8febdf53ca1bc1a4c01946604e7625a95` (0.1.31), is the last-good C. The
[P4 release record](plan/DOCS-SSG-P4.records/M7.md) identifies its workflow
runs, online checks and the rollback rehearsal.

After the required acceptance checks and publication authorization,
assign `DOCS_C` the approved complete CLI commit SHA and run:

```sh
gh workflow run docs-pages.yml --ref main -f candidate_sha="$DOCS_C"
```

Wait for that workflow's build and deployment to succeed before verification.
Set `DOCS_RUN_ID` to that run's numeric ID, then verify it explicitly:

```sh
python3 scripts/docs-pages-verify.py --url https://rockie.github.io/rustify-ui/ --candidate-sha "$DOCS_C" --run-id "$DOCS_RUN_ID"
```

The verifier checks C/R against the prepare receipt, compares the complete
Pages artifact and its manifest, then reads every public file and validates
HTML, search and AI destinations. It needs the run's unexpired artifacts.
Real online search, language switching, deep 404 and CSP checks still run
in a browser.

Keep the previous successful C and its artifact/manifest. Roll back by
dispatching the same workflow with that C, verify the restored build ID and
complete files, then restore the approved target C after the rehearsal.
The last-good C for this release is C1, the 0.1.31 revision in the CLI
rollback command above. The same workflow's complete rollback and restore
were rehearsed when C1 was published.

## The default theme

Every page's reading content is complete before any script runs: the
navigation, the table of contents and every code tab are there to read and
follow. The theme's two
small scripts (one in `<head>`, one deferred) add what needs a script, and
a failure in either hides nothing.

- **Colour mode.** Pages follow the reader's system light or dark setting.
  The System / Light / Dark switch in the header (shown only when script
  runs) overrides it, applies before the first paint on later pages, and is
  remembered per site base. Where storage is refused, the switch still works
  for the page in view.
- **Layout.** From 1280 px: navigation, article and table of contents side
  by side. From 768 px: the table of contents folds above the article. Below
  768 px: one column with the menu and the contents as panels that open and
  close without script; with script, following a link or pressing Escape
  closes them. Nothing scrolls sideways down to 320 px; wide code and tables
  scroll in their own box.
- **Code.** Each block's Copy button copies its exact text (never line
  numbers or highlighting) and says "Copied". Where the browser refuses (an
  `http://` address other than `localhost`, or a denied permission) the
  button says "Copy failed", the code is selected so it can be copied by
  hand, and a screen reader hears the same. Code groups show one tab at a
  time; choosing one switches every group of the same name on the page and
  is remembered on the next pages. Arrow keys, Home and End move between
  tabs.
- **Search.** A native dialog uses the reading width and theme tokens, with
  immediate input focus, text-node highlights, a live total, selected option
  semantics and explicit loading, empty and retry states. The result list
  scrolls independently; no additional H1 or search wasm is needed.
- **Table of contents.** The section being read is marked as the page
  scrolls; at the end of a page the last section is.
- **Images.** Raster images and SVGs with a fixed size get their width and
  height written into the page, so the text below does not move when they
  load. An SVG with only a `viewBox` scales to the column.
- **Home page.** `layout: home` shows the title, the tagline and the
  optional `actions` as a named reading navigation (the first button filled), then `features` as a list of
  titles and sentences, then any Markdown on the page. A leading `# Title`
  in that Markdown is merged into the hero and keeps its automatic or explicit
  id there. The existing merge rule also accepts a leading lowercase
  `<h1 …>…</h1>`. An H1 after introductory text or a comment, inside a
  container, or written as `<H1>` does not merge: it is a second H1 beside
  the hero, diagnosed at its source. Use H2 or start the body with `# Title`.
  Authored action text keeps the page's language even when interface labels
  fall back to English.
- **Landing page.** `layout: landing` shows the brand alone in the header,
  the home page's hero and the page's Markdown, and its profile's footer
  links; no navigation, search or switches (see
  [Landing pages](#landing-pages)).
- **Landmarks.** Home, content and 404 pages have a header, navigation,
  main and site footer. The footer sits outside main and has a named
  navigation with a return-home link under the site's base. It remains
  available without script, home actions, header links, git dates or an
  edit URL. Page-specific edit links and dates keep their own conditions.
- **Brand.** `theme.primary` colours links, the current location, focus
  rings and the primary action in each mode; button labels are black or
  white, whichever reads better. A single `theme.logo` shows in both modes;
  a light/dark pair switches with the mode, without script.

### Browser and measurement scope

P1 checks require the latest stable Google Chrome and real macOS Safari,
plus an iPhone/Safari and an Android/Chrome, with actual versions recorded.
Firefox's real-browser and Playwright-engine checks remain deferred through
P2. P4 verified the latest stable desktop Chrome, Firefox and Safari. By the
maintainer's decision P4 ran no physical-phone checks, so no phone browser is
claimed as verified; VoiceOver with Safari was accepted by the maintainer in
person.
Engine emulation and narrow screenshots supplement device checks.

Performance uses the frozen Playwright Chromium `153.0.8010.12`, separately
from stable-browser compatibility. At 375 px, CPU slowdown is 4×; CDP network
conditions are latency 150 ms, download 209,715 B/s and upload 96,000 B/s.
This is the approved P1 measurement contract, rather than the unmodified
DevTools Slow 4G preset. Each page type's largest page gets five cold-cache
samples, judged by its median. The budgets remain LCP ≤2.5 s, CLS ≤0.1 and
HTML + CSS + all basic scripts ≤100 KB as the sum of each file's gzip-9 size.
See the [frozen contract](plan/DOCS-SSG-P1.md#93-性能千次保存与真实环境合同)
and [current verification record](validation/docs-ssg-p1-gaps/M2.md) for
results and device evidence still to supply.

## Diagnostics

Errors stop a build; warnings do not, unless `--deny-warnings` is given.

| Rule | Severity | Raised for |
| --- | --- | --- |
| `config`, `config-*` | error | `rustify-docs.toml` that cannot be read, has an unknown key or an invalid value (`config-schema-version`, `config-path`, `config-out`, `config-base`, `config-url`, `config-lang`, `config-repo`, `config-color`, `config-logo`, `config-image`, `config-nav`) |
| `config-missing` | error | No `rustify-docs.toml` where `--config` points |
| `frontmatter`, `frontmatter-unclosed` | error | Frontmatter that is not valid, has an unknown key, or never closes |
| `lang` | error | A page `lang` that is not a language tag |
| `i18n-config`, `i18n-messages` | error | Invalid locale configuration or project messages; message inputs selected as public resources |
| `i18n-content-lang`, `i18n-orphan` | error | A page language disagreeing with its directory; a translation without a default-language source |
| `i18n-missing-page`, `i18n-missing-label` | warning | One missing production translation or one missing third-language project message key; `check --deny-warnings` fails for these too |
| `route-conflict`, `reserved-route` | error | Two pages with one address; a page at `/404/` |
| `missing-title` | error | A page with no title and no `#` heading |
| `missing-description` | error / warning | No description and no paragraph to take one from (error); no `description:` (warning) |
| `multiple-h1` | error | A second effective H1, including a home author's H1 that does not merge into the hero |
| `heading-skip` | warning | A heading more than one level below the one before |
| `heading-attribute`, `duplicate-heading-id` | error | A heading attribute other than `{#id}`; one explicit id twice |
| `broken-link`, `broken-anchor`, `empty-link` | error | A link to a page, file or heading that does not exist |
| `draft-link` | error (warning in `dev`) | A published page linking to a draft |
| `new-window-opener` | error | Author HTML opening another site in a new window (`target`) without `rel="noopener"` |
| `external-link` | error (`check --external-links` only) | A link to another site answered 400 or more |
| `external-link-unreachable` | error (`check --external-links` only) | A link to another site could not be requested at all |
| `internal` | error (`dev` only) | A build stopped inside rustify-docs; the last good version is still served |
| `repo-link` | error | A link to a repository file without `[repo]` |
| `script-url` | error | A `javascript:`, `vbscript:` or `data:` link |
| `missing-image`, `missing-media` | error | An image or media file that does not exist |
| `cross-origin-resource` | error | Something the page would load from another origin |
| `inline-script`, `inline-style`, `event-handler`, `embedded-frame`, `embedded-plugin`, `base-element`, `meta-element`, `link-element` | error | Raw HTML the content security policy forbids; `embedded-frame` also for an `<iframe>` with `srcdoc` or without `src`, or outside a landing page whose profile lists frame origins |
| `frame-origin` | error | A landing page's `<iframe>` whose `src` is not on an origin in its profile's `sources.frame` |
| `frame-title` | warning | An `<iframe>` without a `title` |
| `form-origin` | error | A landing page's form target not on an origin in its profile's `sources.form`, or a form under a profile with no form targets |
| `form-blocked` | warning | A form on a page whose policy blocks every submission |
| `landing-profile-name`, `landing-source`, `landing-script`, `landing-carry` | error | A `[landing.profiles]` name, source, script `src` or carried parameter that is not valid (or is repeated) |
| `landing-script-missing` | error | A profile's project script that does not exist |
| `landing-profile-unused` | warning | A profile no page selects |
| `landing-profile`, `landing-without-layout` | error | A page's `landing:` naming no declared profile, or written without `layout: landing` |
| `missing-alt` | warning | An image without alt text |
| `code-fence-info`, `code-highlight-range` | error | A fence info string that cannot be read; marked lines outside the block |
| `code-group`, `code-group-conflict` | error | A malformed code group; one group name with different tabs |
| `embed-payload`, `embed-nested` | error | Invalid or duplicate YAML fields; an interactive island inside component children |
| `unregistered-component`, `unregistered-demo`, `unknown-property`, `reserved-component` | error | A name or field absent from the registry; a registration using a builtin name |
| `invalid_props`, `embed-props` | error | Props that do not match the generated component or builtin types |
| `unsupported_initial_data`, `unsupported_initial_state` | error | An initial view needing shared asynchronous data/errors or an open floating layer/toast region |
| `rust-*` | error | Rust project, registry, source, compilation, framework pairing, helper or generation validation failures |
| `content-include`, `content-include-missing` | error | An invalid Markdown source selection or a selected file that cannot be read safely; the diagnostic points to its configuration entry |
| `markdown-intro`, `markdown-intro-marker` | error | An unsafe/unreadable introduction source, missing H1, or a missing/duplicated introduction placeholder in a registered view |
| `include`, `include-path`, `include-missing`, `include-region`, `include-inline` | error | An include directive that cannot be followed |
| `nav-missing-page`, `nav-duplicate-route`, `nav-route-origin` | error | A nav group listing an absent page; registered-route navigation repeating a route or naming a Markdown page |
| `nav-unknown-group` | error | A page's `sidebar.group` naming no configured group |
| `nav-empty` | warning | A `dir` group with no pages |
| `not-in-nav` | warning | A published page no nav group lists |
| `logo-missing` | error | `theme.logo` naming a file that does not exist |
| `brand-contrast` | warning | A primary colour below 4.5:1 for links or button labels |
| `theme-document`, `theme-font-resource`, `theme-document-resource` | error | An invalid imported theme, missing bundled font resources, or a theme document being published as an asset |
| `theme-token` | warning | An unknown imported token or resolved font fallback |
| `symlink-outside`, `unreadable`, `not-utf8` | error | An input that cannot be read safely |
| `private-file` | error | A link, image, logo or preview image that is, or leads to, a private file |
| `no-pages` | error | Nothing to publish: no content directory, no page, or only drafts |
| `image-missing` | error | `site.image` naming a file that does not exist |
| `missing-site-url` | warning | No `site.url`: no canonical addresses, preview image or sitemap |
| `no-git-history` | warning | No readable git history: pages show no last-updated date |
| `output-conflict` | error | A file in the asset directory at a path the build writes itself |
| `output-check`, `stylesheet` | error | A generated page that breaks the policy; a stylesheet that does not compile (a defect of the tool, reported rather than published) |
| `base` | error | `--base` that does not match `site.url` |
| `write` | error | The output could not be written; the previous output is kept |
| `output`, `output-busy` | error | An output the build may not replace, or one another build or a preview holds |

`check` never rewrites source or configuration to fix a problem and never
generates or replaces published output. Rust checks may write private
compilation caches; both targets use the existing dependency lock.
