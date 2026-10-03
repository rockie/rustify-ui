# Independent theme export consumers

This acceptance fixture consumes exports outside Theme Studio. Run its commands
from the repository root with the existing `mbx`, Rust/WASM and Tailwind 4.1.13
toolchain. It uses its own Cargo workspace, lockfile and target directory. Fetch
the locked dependencies once before the offline build; a cold cache also needs
the repository's Makepad packaging tool dependencies:

```sh
mbx fetch --locked --manifest-path tests/theme-export/Cargo.toml
mbx fetch --locked --manifest-path makepad/tools/cargo_makepad/Cargo.toml
sh tests/theme-export/build.sh
sh tests/theme-export/build.sh --base /tools/theme-export/ --out tests/theme-export/target/site-subpath
```

The root build writes `tests/theme-export/target/site-root`, deployed at `/`.
The child-path build writes `tests/theme-export/target/site-subpath`, deployed at
`/tools/theme-export/`. `--css-only` generates the JSON, Rust snippet and compiled
CSS fixtures without building or packaging the WASM consumer. It does not make
`rust.html` runnable in a fresh output directory. All build commands inside the
script are offline and locked.

The generator calls the actual SDK `export_json`, `export_css`, `import_json`,
`import_css` and `resolve`. It includes the current Studio [data_tools.rs](../../examples/theme-studio/src/data_tools.rs) source
to obtain its `rust_snippet()` string, then writes that exact string and the JSON
to `generated/theme.rs` and `generated/theme.json`. The minimal WASM binary
compiles those files, calls the snippet's `mount_exported_theme` for its light
region, and calls its `load_exported_theme` with a second `ThemeScope` for dark.
Both application handles stay alive until `theme_export_dispose()` is called.

The consumer uses the repository's actual `cargo_makepad` packaging, loader,
embedded Makepad bridge and bridge extractor. It has no alternate provider or
mock SDK. Tailwind compiles both CSS profiles and all four color notations using
the fixed 4.1.13 CLI. Compiler inputs add explicit utility sources; the
`*.export.css` files preserve the SDK's raw output.

The generator reads `THEME_FONTS`, `FONT_FACES` and `FONT_GLYPH_FALLBACKS` for
selected families, `@font-face` declarations and bundled font paths. It copies
the actual resources and their existing license notices. The generated files do
not embed font bytes or invent a separate face catalogue. Inputs deliberately
vary font size, tracking, unit spacing, radius, translucent primary and shadow
parameters between modes and documents.

## Strict CSP preview

The preview binary reuses `xtask/src/serve.rs` with `CspMode::Strict`, without SPA
fallback. Choose a free port using the test runner, set `THEME_EXPORT_PORT`, then
run one fixture at a time:

```sh
mbx run --locked --offline --manifest-path tests/theme-export/Cargo.toml \
  -p theme-export-generator --bin theme-export-serve -- \
  --root tests/theme-export/target/site-root --base / --port "$THEME_EXPORT_PORT"
```

For the child deployment, replace the last line with:

```sh
  --root tests/theme-export/target/site-subpath --base /tools/theme-export/ --port "$THEME_EXPORT_PORT"
```

The server prints its actual URL. All fixture assets use relative paths; the
generated build manifest selects the deployment base for the WASM loader and
Makepad resources. No fixture script uses inline JavaScript. The build script
does not start a server or browser.

The repository's [Playwright configuration](../../playwright.config.ts) runs
the generated sites with their own strict CSP servers. The build script also
compiles the server. Build both products, then run one project at a time from
the repository root:

```sh
sh tests/theme-export/build.sh
sh tests/theme-export/build.sh --base /tools/theme-export/ --out tests/theme-export/target/site-subpath
npx playwright test --project=theme-export --workers=1 --output=test-results/theme-export
npx playwright test --project=theme-export-deep --workers=1 --output=test-results/theme-export-deep
```

`theme-export` serves `target/site-root` on port 4183 at `/`;
`theme-export-deep` serves `target/site-subpath` on port 4184 at
`/tools/theme-export/`. Stop manual servers on those ports before testing.
Both projects run [theme-export-runtime.spec.ts](../browser/theme-export-runtime.spec.ts) with the configured single
worker. `RUSTIFY_THEME_EXPORT_SERVER` can point at an already built server
binary; its default is `tests/theme-export/target/debug/theme-export-serve`.
These commands are acceptance entry points, not a statement that the browser
checks passed.

## Browser targets

Open `index.html` for links, or use these paths beneath the chosen base:

| Page | Content |
| --- | --- |
| `standard-{hex,rgb,hsl,oklch}.html` | Tailwind v4 `:root` plus `.dark` consumers |
| `scoped-{hex,rgb,hsl,oklch}.html` | Two distinct documents, each with light and dark scope roots |
| `rust.html` | Compiled Studio Rust snippet plus exported JSON, mounted with ThemeScope |
| `expected.json` | Actual SDK-resolved properties, metrics, color channels, shadow layers and font resources |

CSS fixture roots have `data-testid="fixture-a-light"` and
`"fixture-a-dark"`; scoped pages also have `"fixture-b-light"` and
`"fixture-b-dark"`. Each prefix has the following targets:

| Suffix | Consumption to check |
| --- | --- |
| `-sans`, `-serif`, `-mono` | Resolved family, glyph samples and loaded same-origin fonts |
| `-radius-sm`, `-radius-md`, `-radius-xl` | Radius utilities and `p-4` unit spacing |
| `-shadow` | Large radius and authored `shadow-lg` layers |
| `-ring` | `ring-2 ring-ring shadow-none` composition |
| `-shadow-color` | `shadow-red-500` overriding authored shadow colors |

`host-sentinel` is outside all scoped roots. Check it while comparing the two
document scopes. `expected.json` contains four entries keyed by `document_id`
and `mode`; `properties` is an array of name/value pairs suitable for
`Object.fromEntries`. Compare resolved numeric values with an appropriate
color-format rounding tolerance, rather than authored color strings.

On `rust.html`, wait for `runtime-status[data-status="ready"]`. Its failure
state is `data-status="failed"` and reports the loader error. `runtime-light`
contains the original snippet's `main`. The dark targets are
`runtime-dark-main`, `runtime-sans`, `runtime-serif`, `runtime-mono` and
`runtime-ring`. Both scope containers should have
`data-rustify-theme-runtime="v4"` and their respective `data-theme` values.

## Local checks

```sh
mbx test --locked --offline --manifest-path tests/theme-export/Cargo.toml -p theme-export-generator --bins
mbx clippy --locked --offline --manifest-path tests/theme-export/Cargo.toml -p theme-export-generator --all-targets -- -D warnings
```

These execute the actual included import/export, persistence, bridge and server
unit tests. Build and unit checks establish compilation and source use; they do
not establish browser font loading, computed styles, CSP execution or mount
behavior. Those checks belong to the repository's serial browser acceptance
run against the generated root and child deployments.
