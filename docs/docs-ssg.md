# Rustify Docs

Rustify Docs turns a directory of Markdown into a static documentation site.
Every page is complete HTML before any script runs; code is highlighted at
build time into classes; the reader's browser downloads no wasm. The
`rustify-docs` command (package `rustify-docs-cli`) reads the content, checks
it and writes the site. It does not compile a site's Rust, because a content
site has none.

This page is the reference for what an author writes: the project layout,
`rustify-docs.toml`, frontmatter, the Markdown the site understands and the
problems `check` reports. The same Markdown file is meant to read well on
GitHub; where the two differ, the section says so.

Supported on macOS. Linux and Windows are not verified.

## Commands

| Command | What it does |
| --- | --- |
| `rustify-docs new <dir> [--template content]` | Creates a site (a home page, two guide pages, `static/`, a README and a `.gitignore`) in a new or empty directory. |
| `rustify-docs check [--format text\|json] [--deny-warnings]` | Reads the whole site and reports every problem at once. It writes no file and makes no network request. |
| `rustify-docs build [--base <path>] [--out <dir>] [--format text\|json] [--deny-warnings]` | Checks the site the same way, then writes it to `build.out` (default `dist/`). |
| `rustify-docs preview [--dir <dir>] [--base <path>] [--port <n>] [--host <address>]` | Serves a production build the way a static host would, without rebuilding it. |

Every command takes `--config <path>` for a `rustify-docs.toml` elsewhere
than the working directory. `--out` and `--dir` are relative to the working
directory, as paths on a command line are; paths inside `rustify-docs.toml`
are relative to the project. `--base` builds for another base path than
`site.base` (it must still match `site.url`'s path). `dev` is not in this
build yet; it says so and exits with status 2.

`new` first checks that `rustc --version` runs (a content site compiles
nothing, but sites with components of their own will), then refuses a
directory that is not empty, and writes the site into a temporary sibling
that it renames into place only when every file is written: a failure
leaves nothing behind. `new` and `build` never run Cargo or a compiler.

Exit status: 0 on success; 1 when the site has errors (with
`--deny-warnings`, warnings too) or the command fails; 2 for a usage error.
Text output goes to stderr and ends with the conclusion, such as
`check: 4 pages, 0 error(s), 1 warning(s)`; a successful build ends with its
page count, total size, file count, warnings and time, followed by its five
largest pages, and a failed one with its error count and the first error.
With `--format json`, stdout holds exactly one JSON object and nothing else:

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

`file` is relative to the project root; `line` and `column` count from 1
(columns in characters). A problem with no position in a file has `null`
there, and `suggestion` is `null` when there is nothing more specific to say
than the message. Diagnostics are sorted by file, line, column, severity and
rule, and each appears once.

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
`.DS_Store`, `.rustify-docs`) are never read for publishing, and a symbolic
link that points outside the project is not followed.

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

[[nav.top]]                           # header links; also nav.social
label = "GitHub"
link = "https://github.com/owner/repo"

[dev]
port = 4188                           # default
```

Paths are relative to the project root and written with `/`. The logo files
and `site.image` must exist. A page's edit link is `edit_url`, then
`repo.path`, then the page's file (`…/edit/main/docs/content/guide/setup.md`). A brand colour that gives link text or button labels less than
4.5:1 contrast on the theme's background is a warning (`brand-contrast`).

`nav.top`, `nav.social` and a home page's `actions` are links like any other:
`guide/setup.md`, `/guide/setup/` and `https://…` resolve and are checked the
same way (a relative path in the configuration is relative to the content
directory).

Without `[[nav.groups]]` the sidebar lists every page in one group named
after the site, plus one group per `sidebar.group` a page names. With groups,
a page none of them lists is a warning (`not-in-nav`); a group's `items`
naming a page that does not exist is an error.

## Frontmatter

YAML between `---` lines at the very start of a page. Every key is optional
and every key is known:

| Key | Meaning |
| --- | --- |
| `title` | The page's title. Without it, the first `# heading` is the title; a page with neither is an error. |
| `description` | One sentence for search results and link previews. Without it the first paragraph is used (warning); a page with neither is an error. |
| `lang` | The page's language tag; defaults to `site.lang`. The interface is in that language when the theme has its words (English and Chinese). |
| `sidebar.label`, `sidebar.order` | The sidebar's text for the page, and its position in its group (lower first; pages without an order follow, by path). |
| `sidebar.group` | The nav group the page is listed in, by label, instead of its directory's. It must name a configured group; without `[[nav.groups]]` it starts a group of that name. A group's explicit `items` list is never changed by it. |
| `draft` | `true` keeps the page out of a production build. |
| `index` | `false` asks search engines not to index the page (default `true`). |
| `search`, `ai` | Recorded for later releases; nothing reads them yet. |
| `toc.levels` | The heading levels the page's table of contents lists, as `[2, 3]` (the default). |
| `layout`, `tagline`, `actions`, `features` | A home page: `layout: home`, a tagline, `actions` (`text`, `link`) and `features` (`title`, `details`). |

GitHub shows frontmatter as a small table at the top of the file.

## Markdown

CommonMark with GitHub's extensions: tables, task lists, footnotes,
strikethrough, alerts and extended autolinks (`www.example.com`,
`https://…` and e-mail addresses written bare).

A page has one top-level heading. When the frontmatter gives the title and
the page also starts with `# heading`, that heading is the page's title and
is not repeated; a second `#` heading is an error (`multiple-h1`). A heading
that skips a level (`##` straight to `####`) is a warning (`heading-skip`).

### Heading anchors

Every heading gets the id GitHub gives it, so `guide.md#some-heading` lands
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

> [!WARNING]
> GitHub does not read `{#id}`: it shows the braces as part of the heading
> and derives its own anchor from the whole text. A link to the explicit id
> works on the site and not on GitHub. Prefer the automatic id when the file
> must read on both.

Footnotes link as `#fn-<name>` and back as `#fnref-<name>`.

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

Every page link and anchor is checked, so a published site has none that
leads nowhere. `javascript:`, `vbscript:` and `data:` links are errors.

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
- `<iframe>`, `<frame>` (`embedded-frame`), `<object>`, `<embed>`
  (`embedded-plugin`), `<base>`, `<meta>` and `<link>`;
- anything the page would load from another origin: `img`, `srcset`,
  `source`, `video`, `audio`, `poster`, `track` and SVG `href` must name a
  file in the project (an `img` may also use a `data:` URL).

`<a href>`, `<img src>` and the other URLs in raw HTML resolve exactly like
Markdown's, base and all. An `<img>` without an `alt` attribute is a
warning.

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

## What a build writes

```text
dist/
  index.html, guide/index.html, …     one directory and index.html per page
  404.html                            the not-found page
  assets/site.<hash>.css              the stylesheet: the classes the pages use, minified
  assets/mode.<hash>.js, site.<hash>.js   the theme's two small scripts
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
- Each page has a unique `<title>` (`Page | Site`; two pages with one title
  are told apart by their parent page), its description, Open Graph and
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
It holds no time, machine path or environment value.

### How a build replaces its output

A build writes the whole new site into a private work area beside the
output, `<output's parent>/.rustify-docs/<name>-<hash>/`, and only when all
of it is written and checked does it swap: the old output moves aside, the
new one moves into place, the old one is deleted. A build that fails, at any
point, leaves the previous output byte for byte; one that is killed is
finished or undone by the next build. A first build that fails leaves no
output at all.

One output has one writer: a build takes the work area's lock, a preview
shares it. A build while a preview of the same output runs (or another
build) stops at once and says so, rather than waiting. A directory that is
not empty and has no manifest is never replaced, nor can the output be the
project, a directory holding it, or the content or asset directory.

Ignore `dist/` and `.rustify-docs/` in version control; `new` writes a
`.gitignore` that does. With `--out` elsewhere, ignore that directory and
the `.rustify-docs/` beside it.

## Previewing and hosting

`preview` serves an existing build and nothing else. It refuses a directory
without a production manifest, one built for a different base than
`--base` asks for (and says which base it was built for), and an output
whose build stopped half-way. It listens on `127.0.0.1:4189` by default (the
system picks another port when that one is taken; `--port 0` always lets it
pick) and prints the address. `--host` takes an address to listen on, such
as the machine's LAN address for a phone; it serves the same read-only
files.

Any static host serves the output as it is, provided it:

- serves the output under the base it was built for, with directory
  addresses answered by their `index.html` (and a directory named without
  its slash redirected to it);
- answers an address with nothing behind it with status 404 and `404.html`;
- sends files with their types (`.css`, `.js`, `.svg`, `.json`).

The site's content security policy is in every page as a `<meta>` element.
A host that can set response headers should also send it as the
`Content-Security-Policy` header, as `preview` does: only the header can
carry `frame-ancestors 'none'`, which stops other sites from framing the
pages. GitHub Pages cannot set headers and uses the `<meta>` alone; it
serves `404.html` for missing addresses, and `.nojekyll` makes it publish
the files as they are.

```text
default-src 'none'; script-src 'self' 'wasm-unsafe-eval'; style-src 'self'; img-src 'self' data:; font-src 'self'; connect-src 'self'; media-src 'self'; worker-src 'self'; object-src 'none'; base-uri 'none'; form-action 'none'; frame-ancestors 'none'
```

## Diagnostics

Errors stop a build; warnings do not, unless `--deny-warnings` is given.

| Rule | Severity | Raised for |
| --- | --- | --- |
| `config`, `config-*` | error | `rustify-docs.toml` that cannot be read, has an unknown key or an invalid value (`config-schema-version`, `config-path`, `config-out`, `config-base`, `config-url`, `config-lang`, `config-repo`, `config-color`, `config-logo`, `config-image`, `config-nav`) |
| `config-missing` | error | No `rustify-docs.toml` where `--config` points |
| `frontmatter`, `frontmatter-unclosed` | error | Frontmatter that is not valid, has an unknown key, or never closes |
| `lang` | error | A page `lang` that is not a language tag |
| `route-conflict`, `reserved-route` | error | Two pages with one address; a page at `/404/` |
| `missing-title` | error | A page with no title and no `#` heading |
| `missing-description` | error / warning | No description and no paragraph to take one from (error); no `description:` (warning) |
| `multiple-h1` | error | A second top-level heading |
| `heading-skip` | warning | A heading more than one level below the one before |
| `heading-attribute`, `duplicate-heading-id` | error | A heading attribute other than `{#id}`; one explicit id twice |
| `broken-link`, `broken-anchor`, `empty-link` | error | A link to a page, file or heading that does not exist |
| `draft-link` | error (warning in `dev`) | A published page linking to a draft |
| `repo-link` | error | A link to a repository file without `[repo]` |
| `script-url` | error | A `javascript:`, `vbscript:` or `data:` link |
| `missing-image`, `missing-media` | error | An image or media file that does not exist |
| `cross-origin-resource` | error | Something the page would load from another origin |
| `inline-script`, `inline-style`, `event-handler`, `embedded-frame`, `embedded-plugin`, `base-element`, `meta-element`, `link-element` | error | Raw HTML the content security policy forbids |
| `missing-alt` | warning | An image without alt text |
| `code-fence-info`, `code-highlight-range` | error | A fence info string that cannot be read; marked lines outside the block |
| `code-group`, `code-group-conflict` | error | A malformed code group; one group name with different tabs |
| `include`, `include-path`, `include-missing`, `include-region`, `include-inline` | error | An include directive that cannot be followed |
| `nav-missing-page` | error | A nav group listing a page that does not exist |
| `nav-unknown-group` | error | A page's `sidebar.group` naming no configured group |
| `nav-empty` | warning | A `dir` group with no pages |
| `not-in-nav` | warning | A published page no nav group lists |
| `logo-missing` | error | `theme.logo` naming a file that does not exist |
| `brand-contrast` | warning | A primary colour below 4.5:1 for links or button labels |
| `symlink-outside`, `unreadable`, `not-utf8` | error | An input that cannot be read safely |
| `image-missing` | error | `site.image` naming a file that does not exist |
| `missing-site-url` | warning | No `site.url`: no canonical addresses, preview image or sitemap |
| `no-git-history` | warning | No readable git history: pages show no last-updated date |
| `output-conflict` | error | A file in the asset directory at a path the build writes itself |
| `output-check`, `stylesheet` | error | A generated page that breaks the policy; a stylesheet that does not compile (a defect of the tool, reported rather than published) |
| `base` | error | `--base` that does not match `site.url` |
| `write` | error | The output could not be written; the previous output is kept |

`check` never changes a file to fix a problem.
