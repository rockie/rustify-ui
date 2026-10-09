# Native GPU declaration resolver

`resolve_gpu(programs, resolved_theme, context)` consumes compiler IR and returns a
complete `GpuComputedStyle`, visible diagnostics, and `Applied`, `Partial`, or
`Rejected`. It does not read class prefixes, draw a widget, or publish a revision.
The widget supplies its current `apply_theme` baseline and previous successful
snapshot. The caller owns program/context invalidation and applies successful
snapshots before drawing. `changed` compares rendering values, excluding font
request/fallback metadata.

The context separates browser viewport, parent dimensions, control dimensions,
layout rem, media rem, and parent font size. It includes logical direction,
explicit control states/input modality, actual font face/weight resources, and
per-control drawing capabilities. `focus-visible` requires both an explicit
visible-focus state and keyboard modality. Default face entries describe the
packaged catalogue at weight 400; additional weights require actual host resource
entries. A missing requested weight produces a diagnostic instead of synthetic
bold. Opacity capability is valid only for a leaf requiring no group composition.

`variables` supplies raw compiler/host bindings; context bindings override SDK
tokens, then active custom-property declarations cascade over them. SDK layout
`spacing` is not implicitly Tailwind `--spacing`. Compiler theme values and
`@property` initial values must be supplied by the host when IR references them;
the integration test obtains raw bindings from `CompileReport` field `gpu_variables` rather
than guessing defaults from class names. The caller must refresh the widget
baseline when its theme changes.

| Matrix area | Pure projection verified | Limits and remaining integration |
| --- | --- | --- |
| Color/alpha | HEX, RGB, HSL, OKLCH, black/white, transparent, final currentColor; variable fallback/cycles; compiler oklab-to-transparent alpha mix | Other named colors and arbitrary color functions return diagnostics; pixel/alpha checks are recorded separately in M5 |
| Padding/gap/dimensions | Physical/logical shorthands, px/rem/em, bounded scalar arithmetic in calc, explicit percentages, auto/min/max | Unbound proportion references reject; host maps dimensions/padding/gap into layout/Walk |
| Flex | Non-reversed row/column, finite align/justify values; display does not override direction | Grid/table, shrink/basis/wrap/reverse and other DOM layout models reject; row gap and column alignment geometry match the DOM in the browser |
| Radius/border/opacity | Uniform corners/borders, CSS corner scaling, solid/absent borders, leaf alpha | Elliptical or nonuniform shapes reject when capability absent; group compositing unsupported |
| Text | Packaged family selection, explicit resource weights, font-size points helper, line height/tracking | Host connects chosen resource/measurements to text widget; variable font weight axes are not inferred |
| Shadow/ring | Ordered outer ShadowLayer list, custom-variable ring width/color/offset expressions | Inset and other unsupported effects reject; host preserves CSS layer drawing order |
| States | Dark/hover/active/focus/focus-visible/disabled; recognized self pseudo selectors and specificity | Button pointer/keyboard and ThemeScope dark integration have browser evidence |
| Responsive | Compiler-provided viewport media expressions, ranges, orientation, hover capability; separate media rem | Container queries/other unsupported media features reject; browser viewport/resize invalidation belongs to host |
| Arbitrary/custom utility | Actual declarations and values pass the same capability checks regardless of candidate name | Arbitrary relationships, unsupported selectors/properties/functions reject; no arbitrary CSS execution |
| Unsupported/policy | Candidate/property/reason diagnostics; Strict preserves previous; Partial applies supported values; empty IR rejects | Host must display diagnostic/status and must not label Partial as DOM/GPU parity |

Cascade expands supported shorthands into longhands and compares importance,
provable self-selector specificity, and compiler order. The entire supplied
program set is evaluated from the baseline, so class deletion and state exit
restore defaults. Unsupported declarations/conditions remain visible even when
their condition is inactive. Malformed/unsupported values in active declarations
remain visible even when another declaration wins. Nonfinite context/baseline
measurements reject. Variable expansion is bounded at depth 64, 4096
substitutions, and 1 MiB output; numeric expression nesting is bounded at 32.
CSS-wide keyword inheritance/revert and generic registered-property semantics
are not implemented by this finite evaluator; destinations using those values
must be resolved by the host or receive an unsupported-value diagnostic.

Run the pure native matrix with the repository cache:

```sh
mbx test -p rustify-ui --features utility --lib utility::gpu::tests::
mbx clippy -p rustify-ui --features utility --all-targets -- -D warnings
```

Run actual compiler-to-resolver tests without adding compiler to the UI's
production dependency closure:

```sh
python3 crates/rustify-ui/src/utility/gpu/compiler_test_runner.py
```

The script creates/removes a temporary Cargo workspace, copies the root lock,
reuses the workspace's Makepad smallvec/bitflags patches and target cache, and
runs offline. Its six tests cover each supported projection area using real
compiler programs, including static flex, named spacing/colors/ring/shadow,
arbitrary properties, typed states/media, and visible unsupported targets.
The input supplies the relevant explicit theme tokens. It does not substitute
for the full compiler compatibility gate, which `mbx xtask utility-conformance
--check` runs and M4 records.

The utility widgets pass their actual parent content box to the resolver.
Container padding and borders are excluded before resolving child percentages;
unknown axes remain unknown and produce diagnostics. Resize invalidates the
values without reparsing candidates. Control dimensions are provided after
measurement for percentage corners; elliptical corners retain the explicit
capability rejection. Browser percentages, min/max, padding and resize evidence
are recorded in [M5](../../../../../docs/validation/wasm-tailwind/M5.md).

No native Makepad window, browser/WebGL2 drawing, geometry/pixel sampling,
DOM/GPU revision parity, context restoration, or performance result is claimed
by these tests. The browser evidence for each matrix row, including text line
boxes and shadow/ring layer order against the DOM, is tabulated in M5; runtime
integration and the final full-range verification belong to M6 and M7.
