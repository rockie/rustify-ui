# Rustify Colors

The exact strings from the local Radix Colors 3.0.0 snapshot: 31 families,
light/dark, solid/alpha, sRGB/Display P3, plus black and white alpha scales.
There are no Rust dependencies, UI runtime, build script, or bundled fonts.
The original MIT notice is in [vendor/colors/LICENSE](vendor/colors/LICENSE).

Use the Rust table with sRGB enabled by default:

```rust
use rustify_colors::{ColorFamily, ColorGamut, ColorMode};
let primary = ColorFamily::Indigo.scale(ColorMode::Light, ColorGamut::Srgb, false);
assert_eq!(primary.step(9), Some("#3e63dd"));
```

`ColorFamily::ALL` lists every family. Steps are one-based, from 1 through 12;
out-of-range queries return `None`. `black_alpha` and `white_alpha` do not have
an appearance. Enable the non-default `p3` Cargo feature for `ColorGamut::P3`.
Strings are preserved verbatim; this package does not normalize color spaces
or promise that every foreground/background combination is accessible.

For a static page, copy only the files you use from the packaged `css/` directory:

```html
<link rel="stylesheet" href="/colors/indigo-light.css">
<link rel="stylesheet" href="/colors/indigo-dark.css">
<link rel="stylesheet" href="/colors/indigo-light-alpha.css">
<section data-rustify-colors="indigo" data-rustify-colors-mode="light">
  <!-- Your external stylesheet can use var(--indigo-9) or var(--indigo-a9). -->
</section>
<section data-rustify-colors="indigo" data-rustify-colors-mode="dark"></section>
```

Each file sets twelve custom properties on an explicitly marked element. Space
separate family names to load several on the same element, for example
`data-rustify-colors="indigo mint black"`. No selector changes `html`, `body`,
`:root`, `.dark` or unmarked elements. Load both mode files when both modes are
needed; the host's color-scheme preference does not change your explicit mode.
Black/white assets are named `black-alpha.css` / `white-alpha.css`, set
`--black-a1..a12` / `--white-a1..a12`, and require only the family marker.

For P3, link `indigo-light-p3.css` instead. It imports its sibling
`indigo-light.css` before applying P3 values inside both `@supports` and
`@media (color-gamut: p3)`. Deploy both files at the same relative paths.
Alpha follows the same naming rule: `indigo-light-alpha-p3.css`. sRGB files
never import P3 assets. Static P3 files do not require enabling a Rust feature.
When redistributing copied assets, retain and distribute `vendor/colors/LICENSE`
with them, including its full copyright and MIT permission notice.

Rust build tools can copy assets without knowing a checkout path:
`ColorFamily::Indigo.css(ColorMode::Light, ColorGamut::Srgb, false)` returns
a `CssAsset` with the packaged relative `path` and `content`.

The repository regenerates these assets with `mbx xtask colors` and verifies
them with `mbx xtask colors --check`. Generation reads the vendored constant
objects and never executes TypeScript or reads the ignored `ref/` directory.
Provenance and both independently named fingerprint algorithms are recorded
in the repository's `sources.lock.json`. Publication is a separate step;
this new package is currently consumed from this source checkout.
