# Changelog

## 0.2.0 — candidate, not yet published

- Prepare nine crates.io packages: rustify-classification-macros,
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
see [font migration](docs/themes.md#020-candidate-docs-font-migration).
GPU applications use one Git source or unify rustify-dom with the documented
root patch. See [migration](docs/docs-ssg.md#020-registry-candidate-and-migration)
and [source pairing](docs/quickstart.md#020-dom-candidate-and-gpu-source-pairing).

Until publication and public checks complete, use the available 0.1.32
[Git installation](docs/docs-ssg.md#install-and-maintain-the-cli). Rollback
restores that CLI's original manifest/lock, theme/font files and complete dist.
Library/CLI tags, real archive sizes, supported-platform evidence and public
registry/docs.rs results are recorded only after the release gates pass.
