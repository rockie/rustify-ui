---
description: Create, resolve, import and export versioned theme documents and apply one resolved snapshot to DOM scopes and GPU regions.
---
# Theme documents

The versioned theme model preserves author values and resolves one snapshot for DOM and GPU consumers. The legacy `Theme`, `ThemePatch`, `ThemedScope` and `ThemeOverride` APIs keep their existing contracts. Conversion is one way: a complete document cannot be reduced to the legacy opaque palette without losing information.

The current source implements schema v2. Published SDK/CLI **0.2.1** supports
schema v1 only; it cannot consume a full v2 document. Source consumers must use
the matching SDK and source CLI checkout; this change does not publish a release.

Use [Theme Studio](../examples/theme-studio/README.md) to edit and export a document, or start with the Rust API below. For the application's build setup, see the [quick start](quickstart.md#using-tailwind-v4-in-your-app).

## Create and resolve a document

```rust
use rustify_ui::{resolve, ResolveContext, Theme, ThemeDocument, ThemeMode};

fn main() -> Result<(), rustify_ui::ThemeError> {
    let mut document = ThemeDocument::from_legacy(Theme::light(), Theme::dark());
    document.name = "My theme".into();
    document.overrides.light.0.insert("primary".into(), "oklch(0.6 0.2 260 / 80%)".into());
    let context = ResolveContext { rem_px: 20.0, ..Default::default() };
    let snapshot = resolve(&document, ThemeMode::Light, &context)?;
    assert_eq!(snapshot.spacing_px, 5.0);
    Ok(())
}
```

In a browser, supply the document root's actual computed font size as `rem_px`. Resolve again when that size, the mode, the document or the available fonts change. `ResolveContext::default()` uses 16px and an empty font catalogue; its generic font fallback is suitable for parsing. Consumers must supply their packaged font faces and three fallback families to select the same font resources for DOM and GPU. This does not guarantee identical rasterization. `ResolvedFont` reports the requested families, selected family, resource path and fallback status.

For the bundled catalogue, use `rustify_ui::theme::font_context(rem_px, reduce_motion)`. The second argument supplies the host's reduced-motion preference; the resolved snapshot reduces motion when either the host or the document requests it. Theme Studio reads the root's computed size and watches its style/class, window resize and the media preference. A custom host must refresh its own context when that environment changes.

The public types and validation live in [document.rs](../crates/rustify-dom/src/theme/document.rs), [metrics.rs](../crates/rustify-dom/src/theme/metrics.rs) and [resolve.rs](../crates/rustify-dom/src/theme/resolve.rs). `ThemeValuePatch` contains sparse author values; `ThemeValues::patched` rejects unknown keys, and the resulting values must be resolved before use.

## Root and local providers

`ThemeScope` claims one mount root. Its `resolved` signal retains the complete
two-mode `ThemeSource` and resolution context. Document `appearance` selects
the root mode; an optional `preview_mode` signal explicitly overrides it.
Studio uses this preview input without changing the author document. A nested
boundary's explicit appearance still wins within that boundary.

```rust,ignore
use rustify_ui::{Appearance, ConfigChange, ThemeBoundary, ThemeBoundaryPatch,
    ThemeConfigPatch, ThemeScope};

view! {
    <ThemeScope resolved=resolved />
    <ThemeBoundary patch=Signal::derive(|| ThemeBoundaryPatch {
        config: ThemeConfigPatch {
            appearance: ConfigChange::Set(Appearance::Dark),
            ..Default::default()
        },
        ..Default::default()
    })>
        <ApplicationArea />
    </ThemeBoundary>
}
```

Boundary configuration distinguishes `Keep` (inherit) from `Set(None)`
(Custom). Its `light` and `dark` maps are independent author overrides. A
mode-independent `ThemeValuePatch` converts with `.into()`; this conversion
copies the patch to both modes, while retaining the original two base tables.
An empty boundary reuses the parent's snapshot and removes its CSS projection;
an invalid patch falls back to the parent and records a diagnostic. Overlays
subscribe to the effective snapshot at their creation point, including local
mode, fonts and geometry. Scope disposal restores the properties and attributes
it replaced; the JS abort release also performs this cleanup after a fatal.

`provide_static_theme` binds a snapshot whose matching external CSS was already
generated. It retains the same source for opposite-mode local resolution;
it does not rewrite the static page or select a different root mode.

## JSON structure

[theme-document.schema.json](theme-document.schema.json) describes canonical v2.
Its full form contains `schema_version: 2`, `id`, `name`, two complete `base`
tables, `config`, and two sparse `overrides` maps. Configuration selects
appearance, brand family, neutral family/auto, panel background, radius and
90/95/100/105/110 scaling. `null` selects Custom for the four optional shortcuts;
it does not mean boundary inheritance. The brand family is separate from the
existing semantic `accent` token.

`ThemeDocument::standard(id, name)` creates indigo/auto/translucent/medium/100
with no overrides. `from_legacy` and `custom(id, name, base)` preserve the two
authored tables, choose Custom, and retain the 24 shortcut-generated semantic
keys as explicit overrides. Configuration changes never erase these overrides.
Read the base with `base_values(mode)` and composed, unscaled author strings
with `effective_values(mode)` or `ResolvedTheme::values`. Edit `overrides` to
take priority; remove an override key to reveal configuration/base again.

The equivalent compact Custom wire form contains `schema_version: 2`, `id`,
`name` and `styles.light/dark`. It cannot be mixed with full-form fields. The
serializer uses it only when all configuration and overrides have that exact
meaning. Strict v1 input has the same `styles` shape and upgrades without
growing its canonical byte length. Full intent and unknown author strings
remain in v2 JSON; JSON is serialized as compact UTF-8. IDs and names contain
1–80 characters, include a non-whitespace character and exclude control characters.

Each base mode requires 49 string tokens, including `reduce-motion` (`"true"`
or `"false"`). Unknown string keys are preserved in base and diagnosed, but
are not allowed in overrides. Duplicate token keys, unknown structural fields,
mixed forms, unsupported versions and non-string token values are rejected.
`resolve` checks author syntax and numeric limits, including masked base values.
Validate both modes and `check_document_size` before accepting an edited candidate.

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

Colors accept short or long HEX with optional alpha, `rgb`/`rgba`, `hsl`/`hsla`, `oklch` and `transparent`. RGB/HSL accept the supported comma and space forms, percentage channels and alpha. Hue accepts degrees, radians, turns and grads. Named colors, `var`, `calc`, relative colors, `color-mix` and Display P3 are rejected. See [color.rs](../crates/rustify-dom/src/theme/color.rs) for executable syntax and conversion tests.

Preview colors are unpremultiplied, gamma encoded sRGB RGBA. OKLCH conversion clips out-of-gamut sRGB channels and adds a diagnostic; the original author value stays in JSON. Alpha must be within 0–1. RGB/HSL channels and OKLCH lightness follow CSS range clamping. `format_color` formats a resolved color without changing the author input. Contrast calculations composite alpha and return `None` when the background is transparent and no opaque canvas color is known.

Lengths accept `px`, `rem` and unitless zero. Every resolved author length is limited to ±4096 CSS pixels; radius and blur cannot be negative, while spacing and font size must be positive. `layout-gap` is independent of the Tailwind utility unit `spacing`. Letter spacing accepts `em`, plus reference preset values in `px`, `rem` or `normal`; the resolved value is in −0.5…0.5em. NaN and infinity are rejected. The default Packaged resolver parses family lists and rejects declaration delimiters, functions, URLs and backslash escapes. The Css policy preserves named families and quoted generic names, supports escaped quotes/backslashes inside quoted family names, and adds an unquoted generic fallback for each slot.

The Custom path keeps sm/md/lg/xl corners `max(0, r−4)`, `max(0, r−2)`, `r`
and `r+4`; at `r=6px`, these are 2/4/6/10px. Configured themes derive six finite
radius levels from 3/4/6/8/12/16px and the selected factor. Full/thumb geometry
is separate, so necessary checkbox/radio/switch shapes remain independent.
Scaling applies once after geometry and every shadow layer are derived; it
does not change stored author strings, colors, em letter spacing or 9999px
full sentinels. Shadow color alpha is multiplied by shadow opacity. A legacy
conversion starts with zero shadow opacity.

## Static documentation themes

A Rustify Docs site can import the complete JSON exported by Theme Studio:

```toml
[theme]
document = "theme.json"
```

The path is relative to the site project. Keep the document inside the project and outside private files and generated output. It is a build input and is watched during development; it is never published as a download. Do not combine `theme.document` with `theme.primary`: the imported document supplies both primary colors.

The historical Git CLI **0.1.32** validates both modes against its
bundled catalogue and publishes selected faces, existing CJK/emoji fallbacks
and their notices. Its unchanged recipe and bytes are retained at the
[fixed 0.1.32 source](https://github.com/rockie/rustify-ui/tree/88566ba1ed08223a5834f794840fff5e48f4ba8a).

### 0.2.0 Docs font migration

The **0.2.0 CLI** keeps the 256 KiB complete JSON import and projects
colors, metrics, typography and mode onto the page and its island scopes.
It resolves font stacks as CSS choices and embeds no font bytes. The default
reading appearance is unchanged when no document is imported. For local
font setup, use the inactive complete example and these steps:

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

When upgrading an old imported theme, either change both modes' three font
slots to the generic families or explicitly copy and declare licensed local
files. Exact glyphs and metrics can change with system fallback. The old
catalogue includes IBM Plex Sans, Noto Sans, Rustify WenKai, Noto Serif,
Liberation Mono, JetBrains Mono and Noto Color Emoji. Rustify WenKai is the
repository's repaired and renamed face. Liberation Mono 1.04 uses GPL v2
with its original font exceptions and source obligations; the other six
faces use OFL 1.1. Upgrading the CLI does not remove obligations on font files
you already distribute. Consult the [original notices](../makepad/widgets/resources/FONT-LICENSES.md)
and the old release's [font provenance](https://github.com/rockie/rustify-ui/blob/88566ba1ed08223a5834f794840fff5e48f4ba8a/sources.lock.json).
To roll back, restore the backed-up project configuration, manifest/lock,
theme and font files together, then reinstall the fixed Git CLI 0.1.32 using
the [author guide](docs-ssg.md#install-and-maintain-the-cli).

Complete class names in registered Rust sources, including initially hidden branches, contribute to the site's stylesheet. A dynamically assembled class still needs an explicit complete candidate in the source; importing a theme does not enable arbitrary runtime CSS.

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

The following is a simplified ordinary-provider example. Place the downloaded
`theme.json` beside it. For the complete snippet, including
`load_exported_theme_with_context` and `mount_exported_preview`, select **Rust
integration** in Studio. The [independent consumer fixture](../tests/theme-export/README.md)
compiles that complete exported snippet, rather than this simplified example.
Download **setup notes** from any export format to retain the actual source
revision, selected preview mode, host context, resolved font paths and licenses.
JSON v2 requires the matching source SDK and CSS; only Docs consumers also need
the source CLI from the same checkout. Published 0.2.1 cannot read complete v2.
Static CSS consumption needs no wasm or CLI; compile Tailwind profiles through
their Tailwind v4 build.

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

This simplified example uses a 16px root and no host motion preference. In a
dynamic application, use the complete export's context-aware loader with the
host environment. To match Studio's explicit preview, call
`mount_exported_preview(container, config, ThemeMode::Light, &font_context(16., false))`,
replacing Light, 16 and false with the mode and context in your setup notes.
`container` is your application's `HtmlElement`; `config` is a `MountConfig`
whose `base` is the actual deployment path (including any subpath). The ordinary
`mount_exported_theme` honors authored appearance; explicit preview does not
rewrite the document. Keep the returned `AppHandle` alive. Publish newly resolved `Arc<ResolvedTheme>` values through the signal when the mode or document changes; pass that same snapshot through typed GPU props.

[ThemeScope](../crates/rustify-dom/src/theme/provider.rs#L89) claims the mounted scope root, writes its complete CSSOM projection and `data-theme`, and restores previous values on disposal. Use it inside an SDK `mount`; each independently mounted container can have its own mode. It does not write theme values to `documentElement` or `body`. A root may have one theme provider, including the legacy provider.

[ThemeBoundary](../crates/rustify-dom/src/theme/provider.rs) wraps children with a
`Signal<ThemeBoundaryPatch>`. It supports independent light/dark value patches
and local appearance, and re-resolves radius, typography and shadows from the
outer theme sources plus the patch. An empty patch restores inheritance.
Validate a candidate with `ResolvedTheme::with_boundary` before publishing it;
an invalid patch uses the enclosing theme and records a diagnostic. For a
mode-independent legacy `ThemeValuePatch`, explicitly adapt it with `.into()`. SDK overlay layers consume the effective theme where they were created, including boundary values; they do not need a separate theme provider.

## Fonts and GPU consumers

[fonts.rs](../crates/rustify-dom/src/theme/fonts.rs#L34) defines six selectable faces: IBM Plex Sans, Noto Sans, Rustify WenKai, Noto Serif, Liberation Mono and JetBrains Mono. Slot fallbacks are IBM Plex Sans, Noto Serif and Liberation Mono. `dom_font_stack` and `gpu_font_family` append the bundled CJK and emoji glyph fallbacks in the same order. An unknown author stack stays in JSON while `ResolvedFont::fallback` and diagnostics explain the packaged family used for preview. Language preferences do not rewrite author font stacks or theme names.

Rustify WenKai is the repaired, renamed regular WenKai face. Its resource path remains `makepad_widgets/resources/LXGWWenKaiRegular.ttf`. The repair preserves glyph outlines and metrics while replacing the invalid cmap arrangement; see the [font notices](../makepad/widgets/resources/FONT-LICENSES.md), [repair script](../scripts/repair-theme-font.py) and [source records](../sources.lock.json). Copy resources and notices with the static build. CSS exports list their same-origin paths and emit `@font-face`; downloading a stylesheet does not download or embed the font bytes.

DOM consumers use `ResolvedTheme::properties` through ThemeScope. GPU regions receive the same `Arc<ResolvedTheme>` in their application props; the [studio preview props](../examples/theme-studio/src/preview_region.rs) show this arrangement. Call the GPU controls' `apply_theme(cx, &snapshot)` rather than converting RGBA into the legacy opaque palette. For custom text, [apply_text_theme](../crates/rustify-ui/src/gpu/theme.rs#L28) selects the font slot, converts CSS pixels to Makepad points with `css_px_to_points` and applies `letter_spacing_em`. `rustify_ui::theme::gpu_font_family(cx, &snapshot.fonts[index])` is the public lower-level font resolver. These browser/GPU APIs are available on `wasm32`.

DOM and Makepad share color channels, metrics, fonts and structured shadow layers. Their font rasterization and blur kernels can differ; matching inputs do not promise identical pixels. Check the rendered result as well as the props.

## Import and export

The [codec](../crates/rustify-dom/src/theme/codec.rs) exports `import_json`, `import_css`, `export_json`, `export_css`, `ThemeImport`, `CssProfile` and `THEME_INPUT_LIMIT` through `rustify_ui::theme`. Import and export are limited to 256 KiB of UTF-8 text. Import returns a candidate document and diagnostics; it does not mutate application state or apply CSS.

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
