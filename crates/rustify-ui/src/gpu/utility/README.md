# SDK utility preview widgets

`RustifyUtilityPreview` owns a finite container with the existing `RustifyButton`
and Text/ColorBlock leaves. `RustifyUtilityLeaf` can also be used independently. These widgets consume
precompiled `GpuProgram` arrays and complete resolver snapshots; their drawing
path never parses class strings. They retain the existing neutral preview shell
and use packaged theme font resources.

Register `gpu::utility::script_mod(vm)` after `gpu::script_mod(vm)`, then instantiate
`mod.widgets.RustifyUtilityPreview` in the host script. The script exports
`RustifyUtilityLeaf` as well. Both widgets require the UI `utility` feature and
the existing Makepad GPU/WASM host.

The host passes `UtilityPreviewProps` to `apply_props(cx, &props)` when programs,
theme, context, or labels change. Each slot has a `UtilityNodeProps` containing
revision, programs, resolved theme, explicit resolver context, label, and kind.
The composite enforces its slots' kinds. Initialize each context's `base_style`
from the intended widget theme baseline and refresh it on theme changes. Populate
`context.variables` from the compiler report's `gpu_variables`, then add fresh
host overrides. Pass actual viewport, parent/control dimensions, and font
resources; none are inferred from candidate names. Context changes needing more
than viewport/state use `apply_props` with a fresh context.

`update_viewport(cx, [width, height])` reevaluates the committed programs.
`set_node_state(cx, kind, state)` supplies explicit host state for any slot;
`set_button_state` is a convenience. Button input also handles pointer hover,
primary press/release, keyboard activation, Tab focus, focus loss, and keyboard
versus pointer focus-visible modality. Disabled state cancels activation/focus.
`take_click()` consumes the Button action. The host owns wider focus traversal,
accessibility semantics, resize/theme invalidation, and event routing.

`report()` returns each slot's complete computed style, state, diagnostics,
status, actual border-box geometry, and three separate revisions:

| Field | Meaning |
| --- | --- |
| `attempted_revision` | Latest requested props, including a rejected request |
| `applied_revision` | Latest successful complete or Partial snapshot |
| `drawn_revision` | Applied revision observed after an actual completed paint with font resources ready |

`text_rows` lists the lines of the last painted label in region coordinates:
each line's text without the whitespace collapsed at a wrap, the inked advance
from its first to its last visible glyph, and its line box (the explicit line
height, or the font's ascender to descender).

Strict rejection retains the previous snapshot and committed programs; those
programs keep responding to state/viewport changes while the rejected attempt
remains visible. An equal rendering snapshot does not request a redundant widget
redraw. Its new revision becomes drawn only on a real draw; a host requiring a
physical frame for every revision must schedule that frame. Font loading retries
request redraw and keep the previous drawn revision until resources are ready.

The drawing path applies background/border/radius, ordered outer shadows/rings,
padding, width/height constraints, opacity for eligible leaves, actual text
shaping and resource binding, font size, explicit line height, and tracking.
The composite supports three-child row/column flow, gap, finite align/justify,
and auto cross-axis stretch with child constraints. It is a bounded SDK
container, not a general DOM layout engine. A label stays on one line while it
fits the content box; otherwise it wraps between words at that width, and the
text item then spans the whole width, as a shrunk CSS flex item does. An auto
width measures against the parent content box (or its own max width) less
padding and borders. A single word wider than the box breaks between graphemes,
where CSS `overflow-wrap: normal` lets it overflow. Explicit newlines remain
supported. Font bindings are cached until the selected family/resource changes.

Container and Button decline group opacity. A solid ColorBlock uses one combined
background/border shader and supports opacity; its shadows require group
composition and therefore decline nonunit opacity. Text supports opacity only
without background/border/shadows. The same guard applies during props and state
updates; Partial restores baseline opacity and reports the skipped effect.
Nonuniform borders/corners remain unsupported. Nondefault font weights require
real distinct weighted resources; the existing public font adapter cannot select
a variable-font weight axis. These boundaries remain visible diagnostics.

Native model tests run as part of the resolver suite:

```sh
mbx test -p rustify-ui --features utility --lib utility::gpu::tests::
mbx clippy -p rustify-ui --features utility --all-targets -- -D warnings
mbx check -p rustify-ui --features utility --target wasm32-unknown-unknown
```

Native tests prove snapshot/revision/state/layout contracts. WASM type checking
does not execute the script shader or verify pixels, actual pointer/keyboard
events, font loading, WebGL2/context restoration, or DOM/GPU comparison. Those
require the real host's browser acceptance run.

`examples/utility-lab/src/preview_region.rs` supplies the application-owned
`UtilityLabRegion` adapter. Its props are `UtilityLabProps { preview,
button_state }`: the optional state is an explicit command from the host's DOM
accessibility mirror; absent it, Makepad retains independent input state. Actions are
`UtilityLabAction::Report(serde_json::Value)` and `ButtonClick(serde_json::Value)`.
The report has four named slots with snake_case fields and lowercase
`applied`/`partial`/`rejected` status. Lengths and line height use tagged
`kind`/`value` objects. The adapter forces one real draw for a changed revision
tuple, updates media viewport from browser `innerWidth`/`innerHeight` before drawing, and
reads/deduplicates complete snapshots on the NextFrame after Draw. It does not
assign drawn revisions itself. State-only changes also queue a deduplicated
NextFrame report with the truthful previous drawn revision, without demanding a
redundant draw. The DOM application owns mounting the region and
consuming actions.

The existing `RustifyButton` also accepts feature-gated `ButtonUtilityProps`
through `apply_utility_programs(cx, &props)`. Props contain revision, programs,
resolved theme, and explicit context. The button replaces the caller's baseline
and previous-style fields with its actual theme/palette/text defaults and last
successful snapshot. Existing `set_state` owns label and disabled;
`set_utility_state` supplies an explicit host state command. Theme/palette/state
and `update_utility_viewport` refresh the committed programs. `utility_report`
returns the same truthful node report, and `clear_utility` restores saved drawing
defaults and ordinary ownership. Applying an empty program set restores the
current complete numerical baseline. Without utility props, the original event
and drawing path remains active. This adapter uses the Button's own drawing
fields, font resources, geometry, and click API; it does not substitute a Leaf.
The composite converts only the Button slot into these props. Its default width
comes from the actual Button Walk (the SDK script uses 170px); `apply_theme`
sets height to `spacing_px * 9`, horizontal padding to `spacing_px * 4`, primary
and primary-foreground colors, md radius, and the theme's Sans font. The demo's
legacy document supplies a 0.25rem spacing unit separately from layout-gap;
with rem 16 this yields 36px height and 16px horizontal padding. The complete
Button report carries those defaults rather than the former Leaf baseline.
