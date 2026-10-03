# Theme documents

The versioned theme model preserves author values and resolves one snapshot for DOM and GPU consumers. The legacy `Theme`, `ThemePatch`, `ThemedScope` and `ThemeOverride` APIs keep their existing contracts. Conversion is one way: a complete document cannot be reduced to the legacy opaque palette without losing information.

Use [Theme Studio](../examples/theme-studio/README.md) to edit and export a document, or start with the Rust API below. For the application's build setup, see the [quick start](quickstart.md#using-tailwind-v4-in-your-app).

## Create and resolve a document

```rust
use rustify_ui::{resolve, ResolveContext, Theme, ThemeDocument, ThemeMode};

fn main() -> Result<(), rustify_ui::ThemeError> {
    let mut document = ThemeDocument::from_legacy(Theme::light(), Theme::dark());
    document.name = "My theme".into();
    document.styles.light.0.insert("primary".into(), "oklch(0.6 0.2 260 / 80%)".into());
    let context = ResolveContext { rem_px: 20.0, ..Default::default() };
    let snapshot = resolve(&document, ThemeMode::Light, &context)?;
    assert_eq!(snapshot.spacing_px, 5.0);
    Ok(())
}
```

In a browser, supply the document root's actual computed font size as `rem_px`. Resolve again when that size, the mode, the document or the available fonts change. `ResolveContext::default()` uses 16px and an empty font catalogue; its generic font fallback is suitable for parsing. Consumers must supply their packaged font faces and three fallback families to select the same font resources for DOM and GPU. This does not guarantee identical rasterization. `ResolvedFont` reports the requested families, selected family, resource path and fallback status.

For the bundled catalogue, use `rustify_ui::theme::font_context(rem_px, reduce_motion)`. The second argument supplies the host's reduced-motion preference; the resolved snapshot reduces motion when either the host or the document requests it. Theme Studio reads the root's computed size and watches its style/class, window resize and the media preference. A custom host must refresh its own context when that environment changes.

The public types and validation live in [document.rs](../crates/rustify-ui/src/theme/document.rs), [metrics.rs](../crates/rustify-ui/src/theme/metrics.rs) and [resolve.rs](../crates/rustify-ui/src/theme/resolve.rs). `ThemeValuePatch` contains sparse author values; `ThemeValues::patched` rejects unknown keys, and the resulting values must be resolved before use.

## JSON structure

[theme-document.schema.json](theme-document.schema.json) describes the exchange structure. A document has exactly `schema_version`, `id`, `name` and `styles`; both `styles.light` and `styles.dark` are required. Version 1 uses string values for every token, including `reduce-motion` (`"true"` or `"false"`). IDs and names contain 1–80 characters, include a non-whitespace character and exclude control characters.

Each mode requires 49 tokens. Unknown string-valued tokens remain in the document and generate a preview diagnostic. Non-string values fail deserialization. The schema checks the structure; `resolve` checks author syntax and numeric limits using the provided context. Validate both modes before accepting an imported document.

| Group | Tokens |
| --- | --- |
| Surfaces | `background`, `foreground`, `card`, `card-foreground`, `popover`, `popover-foreground` |
| Semantic colors | `primary`, `secondary`, `muted`, `accent`, `destructive`, each with its `-foreground`; `border`, `input`, `ring`, `success`, `warning` |
| Data and sidebar | `chart-1` through `chart-5`; `sidebar`, `sidebar-foreground`, `sidebar-primary`, `sidebar-primary-foreground`, `sidebar-accent`, `sidebar-accent-foreground`, `sidebar-border`, `sidebar-ring` |
| Typography | `font-sans`, `font-serif`, `font-mono`, `font-size`, `letter-spacing` |
| Geometry | `radius`, `spacing`, `layout-gap` |
| Shadows | `shadow-color`, `shadow-opacity`, `shadow-blur`, `shadow-spread`, `shadow-offset-x`, `shadow-offset-y` |
| Motion | `reduce-motion` |

The document retains differences between modes, including differences in fields normally edited together. `ResolvedTheme::diagnostics` identifies those common-field differences. Resolving does not change the document.

## Color and size rules

Colors accept short or long HEX with optional alpha, `rgb`/`rgba`, `hsl`/`hsla`, `oklch` and `transparent`. RGB/HSL accept the supported comma and space forms, percentage channels and alpha. Hue accepts degrees, radians, turns and grads. Named colors, `var`, `calc`, relative colors, `color-mix` and Display P3 are rejected. See [color.rs](../crates/rustify-ui/src/theme/color.rs) for executable syntax and conversion tests.

Preview colors are unpremultiplied, gamma encoded sRGB RGBA. OKLCH conversion clips out-of-gamut sRGB channels and adds a diagnostic; the original author value stays in JSON. Alpha must be within 0–1. RGB/HSL channels and OKLCH lightness follow CSS range clamping. `format_color` formats a resolved color without changing the author input. Contrast calculations composite alpha and return `None` when the background is transparent and no opaque canvas color is known.

Lengths accept `px`, `rem` and unitless zero. Every resolved author length is limited to ±4096 CSS pixels; radius and blur cannot be negative, while spacing and font size must be positive. `layout-gap` is independent of the Tailwind utility unit `spacing`. Letter spacing accepts `em`, plus reference preset values in `px`, `rem` or `normal`; the resolved value is in −0.5…0.5em. NaN and infinity are rejected. Font stacks are parsed as family lists; declaration delimiters, functions, URLs and backslash escapes are rejected.

For radius `r`, the resolved sm/md/lg/xl corners are `max(0, r−4)`, `max(0, r−2)`, `r` and `r+4`. At `r=6px`, these are 2/4/6/10px. This does not change the legacy 4/6/8px sm/md/lg scale. Shadow sizes resolve to one or two structured layers; color alpha is multiplied by shadow opacity. A legacy conversion starts with zero shadow opacity.

## Mount a browser scope

Compile one Tailwind stylesheet for the application. Import the opt-in mapping after `sdk.css`; this is the order used by [Theme Studio's Tailwind input](../examples/theme-studio/tailwind.css):

```css
@import "tailwindcss/theme.css" layer(theme);
@import "tailwindcss/utilities.css" layer(utilities) source(none);
@import "../../crates/rustify-components/css/sdk.css";
@import "../../crates/rustify-components/css/theme-v4.css";
@source "./src";
```

These paths are relative to a repository example's input. In another application, point them at its SDK source files and its own static class sources. Preflight remains an application choice. The mapping reads `--rustify-unit`, derived radius, font and shadow variables; it supplies no new theme values to the host page. Keep static complete class names such as `bg-card text-card-foreground p-4 rounded-lg shadow-lg` in the scanned sources.

The following is the Rust snippet exported by the studio. Place the downloaded JSON file, theme.json, beside it. This exact snippet is compiled by the [independent consumer fixture](../tests/theme-export/README.md):

```rust
use leptos::prelude::*;
use rustify_ui::{mount, AppHandle, MountConfig, ResolvedTheme, ThemeScope};
use rustify_ui::theme::{font_context, import_json, resolve, ThemeError, ThemeMode};
use std::sync::Arc;

// Put the downloaded theme.json beside this source file.
// Include sdk.css and then theme-v4.css in one Tailwind v4 build.
// Copy the listed same-origin font files; CSS does not embed font bytes.
pub fn load_exported_theme(mode: ThemeMode) -> Result<Arc<ResolvedTheme>, ThemeError> {
    let context = font_context(16., false);
    let document = import_json(include_str!("theme.json"), &context)?.document;
    Ok(Arc::new(resolve(&document, mode, &context)?))
}

// A GPU region receives this same Arc<ResolvedTheme> through its typed props.
// Retain the returned AppHandle for as long as the preview is mounted.
pub fn mount_exported_theme(
    container: leptos::web_sys::HtmlElement,
) -> Result<AppHandle, String> {
    let initial = load_exported_theme(ThemeMode::Light).map_err(|error| error.to_string())?;
    mount(container, MountConfig {
        scope: "theme-preview".into(), url_owner: false, base: "/".into(),
    }, move || {
        let theme = RwSignal::new(initial);
        view! {
            <ThemeScope resolved=theme />
            <main class="bg-background text-foreground font-sans p-4 rounded-lg">
                "Your themed application"
            </main>
        }
    }).map_err(|error| error.to_string())
}
```

The fixture deliberately uses a 16px root and no host motion preference. Replace those arguments with the current host environment in a dynamic application. Keep the returned `AppHandle` alive. Publish newly resolved `Arc<ResolvedTheme>` values through the signal when the mode or document changes; pass that same snapshot through typed GPU props.

[ThemeScope](../crates/rustify-ui/src/theme/provider.rs#L77) claims the mounted scope root, writes its complete CSSOM projection and `data-theme`, and restores previous values on disposal. Use it inside an SDK `mount`; each independently mounted container can have its own mode. It does not write theme values to `documentElement` or `body`. A root may have one theme provider, including the legacy provider.

[ThemeBoundary](../crates/rustify-ui/src/theme/provider.rs#L114) wraps children with a `Signal<ThemeValuePatch>`. It keeps the enclosing mode and re-resolves radius, typography and shadows from the outer snapshot plus the patch. An empty patch removes local values and restores inheritance. Validate a candidate with `ResolvedTheme::patched` before publishing it; an invalid boundary patch uses the enclosing theme and records a diagnostic. SDK overlay layers consume the effective theme where they were created, including boundary values; they do not need a separate theme provider.

## Fonts and GPU consumers

[fonts.rs](../crates/rustify-ui/src/theme/fonts.rs#L34) defines six selectable faces: IBM Plex Sans, Noto Sans, Rustify WenKai, Noto Serif, Liberation Mono and JetBrains Mono. Slot fallbacks are IBM Plex Sans, Noto Serif and Liberation Mono. `dom_font_stack` and `gpu_font_family` append the bundled CJK and emoji glyph fallbacks in the same order. An unknown author stack stays in JSON while `ResolvedFont::fallback` and diagnostics explain the packaged family used for preview. Language preferences do not rewrite author font stacks or theme names.

Rustify WenKai is the repaired, renamed regular WenKai face. Its resource path remains `makepad_widgets/resources/LXGWWenKaiRegular.ttf`. The repair preserves glyph outlines and metrics while replacing the invalid cmap arrangement; see the [font notices](../makepad/widgets/resources/FONT-LICENSES.md), [repair script](../scripts/repair-theme-font.py) and [source records](../sources.lock.json). Copy resources and notices with the static build. CSS exports list their same-origin paths and emit `@font-face`; downloading a stylesheet does not download or embed the font bytes.

DOM consumers use `ResolvedTheme::properties` through ThemeScope. GPU regions receive the same `Arc<ResolvedTheme>` in their application props; the [studio preview props](../examples/theme-studio/src/preview_region.rs) show this arrangement. Call the GPU controls' `apply_theme(cx, &snapshot)` rather than converting RGBA into the legacy opaque palette. For custom text, [apply_text_theme](../crates/rustify-ui/src/gpu/theme.rs#L28) selects the font slot, converts CSS pixels to Makepad points with `css_px_to_points` and applies `letter_spacing_em`. `rustify_ui::theme::gpu_font_family(cx, &snapshot.fonts[index])` is the public lower-level font resolver. These browser/GPU APIs are available on `wasm32`.

DOM and Makepad share color channels, metrics, fonts and structured shadow layers. Their font rasterization and blur kernels can differ; matching inputs do not promise identical pixels. Check the rendered result as well as the props.

## Import and export

The [codec](../crates/rustify-ui/src/theme/codec.rs) exports `import_json`, `import_css`, `export_json`, `export_css`, `ThemeImport`, `CssProfile` and `THEME_INPUT_LIMIT` through `rustify_ui::theme`. Import and export are limited to 256 KiB of UTF-8 text. Import returns a candidate document and diagnostics; it does not mutate application state or apply CSS.

JSON import validates the version, identity and both complete modes, rejecting duplicate token keys. It uses the document's own id/name. Unknown string tokens remain in JSON and are excluded from CSS/GPU projection. CSS import takes a base document: it keeps its id/name, overlays imported light values on the base light mode, and overlays imported light then dark values on the base dark mode. Missing values and ignored rules are diagnosed.

The CSS parser recognizes `:root`, `.dark` and exported Rustify scope selectors, including comments, multiple blocks and `@layer`. The last repeated declaration wins with a warning. Unsupported values of known tokens reject the whole import, including a bad declaration later overwritten by another value. Other selectors, unknown variables, Tailwind mappings and import statements are ignored with diagnostics. It never inserts the source CSS, loads a URL or import, or executes HTML or scripts. `shadow-x`/`shadow-y` alias the editable offsets; an arbitrary `box-shadow` cannot reconstruct the editable shadow parameters.

| Export | Consumption |
| --- | --- |
| `export_json(document, context)` | Complete author strings and both modes; use with ThemeScope for dynamic Rustify applications |
| `export_css(document, CssProfile::TailwindV4, context, format)` | Tailwind v4 input with `:root`/`.dark`; compile with the application's class sources |
| `export_css(document, CssProfile::RustifyScoped, context, format)` | Import after `sdk.css` in one Tailwind input; mark each static root with the document id and mode |

`ColorFormat::{Hex, Rgb, Hsl, Oklch}` changes notation for the same resolved sRGB colors. JSON preserves the original notation and out-of-gamut author input. CSS includes derived geometry, typography and shadow mappings; its colors and selected fallback fonts describe the resolved preview.

For example, the scoped fixture's JSON id is `fixture-a`:

```html
<section data-rustify-scope="preview-a"
         data-rustify-theme-css="fixture-a" data-theme="dark">
  <button class="bg-primary text-primary-foreground p-4 rounded-lg shadow-lg">
    Preview
  </button>
</section>
```

Static exported CSS and a running ThemeScope have different owners. A ThemeScope writes inline CSSOM values, which take precedence over pasted export rules. Import the JSON and publish a resolved snapshot to update such a scope. Keep exported CSS for static consumers. The [independent export fixture](../tests/theme-export/README.md) compiles both profiles in all four formats and provides root/child-path consumers and SDK-resolved expectations.

## Failure diagnostics and legacy compatibility

`ThemeError` reports a field and message; `ThemeImport::diagnostics` and `ResolvedTheme::diagnostics` report ignored inputs, gamut clipping, font fallback and common-field mode differences. Show a candidate's diagnostics before applying it. Keep the current valid snapshot if parsing or resolution fails. Contrast returns an unknown result when a translucent background has no known opaque canvas, rather than inventing a passing ratio.

| Symptom | Check |
| --- | --- |
| Colors change but new spacing/radius/fonts do not | Compile [theme-v4.css](../crates/rustify-components/css/theme-v4.css) after `sdk.css`, scan the static utility classes, and inspect the scope's `--rustify-*` properties |
| Pasted CSS does not replace a live theme | Import its JSON and update ThemeScope's resolved signal |
| Requested font differs from preview | Inspect `ResolvedFont`, its fallback diagnostic, the exported `@font-face` and actual font response |
| Child-path scripts or fonts fail | Build and serve with the same `--base`; publish the complete resource directory |
| GPU cannot start | Read the region state and runtime diagnostics; keep DOM preview available |

Legacy `Theme`/`ThemePatch`, `ThemedScope`/`ThemeOverride` and opaque GPU palette methods remain available. New documents and GPU `apply_theme` preserve RGBA and the extended token set. The [theme-v4.css mapping](../crates/rustify-components/css/theme-v4.css) opts new scopes into their independent runtime variables while preserving legacy fallbacks, including default 4/6/8px sm/md/lg radii, fixed Tailwind font/shadow definitions and legacy shadow-color utility composition. Converting from legacy fills a complete document; converting an extended document back would lose information.

## Built-in preset source

The studio contains two Rustify presets and all 42 presets in the local tweakcn snapshot. [provenance.json](../examples/theme-studio/vendor/tweakcn/provenance.json) records the source hashes, complete key list and observed author-value differences. The upstream commit was not available; content hashes identify this snapshot. The copied input files and Apache-2.0 license accompany the normalized data.

Normalization uses the reference merge order: default light then preset light; default dark then preset light then preset dark. The Rust loader adds the five Rustify-only tokens without overwriting reference author values. Normal builds read the committed JSON, never `ref/` or a remote service.

Check the vendored normalization and Rust contracts:

```sh
node --experimental-vm-modules examples/theme-studio/vendor/tweakcn/normalize.mjs --check
mbx test -p rustify-ui --lib theme
mbx test -p theme-studio --bins
mbx xtask sources verify
```
