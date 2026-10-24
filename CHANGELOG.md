# Changelog

## 0.2.0

- Publish nine crates.io packages: rustify-classification-macros,
  rustify-style, rustify-docs-syntax, rustify-docs-macros, rustify-dom,
  rustify-components, rustify-docs, rustify-docs-cli and rustify-style-wasm.
  The Makepad fork, GPU SDK facade and bridge remain Git-only.
- Move shared DOM/theme/action/lifecycle types into rustify-dom; the GPU
  facade reexports the same types. New Docs component projects use
  rustify_dom. Existing imports can use a rustify-ui Cargo alias for the core.
  Remove rustify-components/gpu; applications enable GPU on their SDK.
- Pair the CLI with exact registry versions and real template-lock checksums.
  Keep strict canonical paths for source development and historical Git
  pairing in the consumer helpers. Default registry installation uses standard
  release with unwind; source installation retains docs-cli.
- Default Docs to system fonts. Each project can declare local woff2/woff/ttf/otf
  faces; generated SHA-256 resources use font-display swap. No production font
  bytes ship in the CLI. New projects include a complete inactive theme example.
  Undeclared named families warn; CJK/emoji and wrapping can differ.
- Package CSS/JS/templates/licenses locally, preserve optional repository
  corpus gates and configure single-mode docs.rs builds. Add immutable R/C
  archive receipts, drift checks and resumable publish/index/rate-limit state.

For an old 0.1.32 component project, back up manifest/lock, theme/fonts and
whole output; generate a new components project, keep its new manifest/lock
and port author inputs. Manual migration changes the three dependencies,
imports and feature forwards together, then explicitly creates a real lock.
An imported old theme can use generic stacks or declare licensed local fonts;
see [font migration](docs/themes.md#020-docs-font-migration).
GPU applications use one Git source or unify rustify-dom with the documented
root patch. See [migration](docs/docs-ssg.md#020-registry-pairing-and-migration)
and [source pairing](docs/quickstart.md#020-dom-core-and-gpu-source-pairing).

Install with `cargo install rustify-docs-cli --version 0.2.0 --locked`; no
custom profile is needed. For rollback, use the fixed 0.1.32
[Git installation](docs/docs-ssg.md#install-and-maintain-the-cli) and restore
its original manifest/lock, theme/font files and complete dist.

Library R is `4deb101dff9227ac4f0f313c10dc4c7b5815640c`; CLI C is
`ba9260ee08855a7469e4728403c087f35c8406d0`. The nine archives range from
4,479 to 477,548 compressed bytes. macOS and Ubuntu archive installation,
package check/doc and author smoke passed with Rust 1.99.0; runtime/browser
validation remains macOS Chrome. Full Linux runtime and Windows are not
verified. Actual upload, public installation, docs.rs and tag results belong
to the [release record](docs/plan/CRATES-IO-P1.records/M6.md). This release
does not deploy the official site.

The CLI is binary-only: docs.rs's `rustdoc --lib` build has no library target.
Its package README, `--help` and author guide document installation and use;
library API documentation is hosted separately on docs.rs.
