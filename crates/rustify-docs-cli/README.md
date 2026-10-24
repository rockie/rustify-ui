# rustify-docs-cli

The rustify-docs static site command.

Commands: new, check, build, preview and dev. Content-only sites do not compile site Rust. Components sites build a host SSR helper and a browser hydration module. Use --help for command options and the author guide for configuration.

## Install and start

Use stable Rust (at least 1.99) on macOS:

```sh
cargo install rustify-docs-cli --version 0.2.1 --locked
rustify-docs --version
rustify-docs --build-info
cargo install --list
rustify-docs new my-site
cd my-site
rustify-docs check
rustify-docs build
rustify-docs preview
```

Expect version 0.2.1, `panic: unwind`, `distribution: registry`, and a
`framework` JSON record naming exact `rustify-docs`, `rustify-dom` and
`rustify-components` versions plus their library commit. Standard registry
installation uses the release profile. Content sites need no Node, Tailwind,
Makepad or site Rust build. `dev` serves in memory and updates pages on save.

For Rust components, add the wasm target, then create the complete project:

```sh
rustup target add wasm32-unknown-unknown
rustify-docs new my-components --template components
cd my-components
rustify-docs check
rustify-docs build
rustify-docs preview
```

Commit its manifest and checksum-bearing lock. `new` copies that lock offline
and changes only the root package name. The first check/build can download
locked dependencies; after caching them, both work offline. Read the generated
README before adding a component or changing its host/wasm features.

Upgrade by backing up the CLI installation command, manifest/lock, theme/fonts
and complete output, then reinstall the chosen version with `--force`. For
components, update all three dependencies to the new CLI's `--build-info`
versions together, explicitly regenerate the lock and run check/build. Stop
and restart dev after switching versions. A source/version mismatch is
`rust-framework-pair`; restore or upgrade the entire pair.

The historical 0.1.32 Git rollback is:

```sh
cargo install --git https://github.com/rockie/rustify-ui --rev 88566ba1ed08223a5834f794840fff5e48f4ba8a --locked --profile docs-cli --force rustify-docs-cli
```

Restore that version's original project manifest/lock, fonts/theme and whole
dist as well. A future registry rollback pins a verified older registry
version and restores its matching files. This release's scope is macOS;
Linux preflight and docs.rs compilation are separate evidence, and Windows
has not been verified. See the [author guide](https://github.com/rockie/rustify-ui/blob/main/docs/docs-ssg.md)
for the complete configuration and diagnostics. Licensed under MIT; see [LICENSE](LICENSE).

## Theme and local fonts

Both `new` templates include an inactive `theme.example.json`.

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
