---
description: Migrate source consumers to the configured theme and native DOM component APIs.
---
# DOM component migration

This guide describes the current source checkout. Published SDK and Docs CLI
0.2.1 use the previous APIs and schema v1. Use one matching source checkout for
the SDK, styles and source CLI; these changes do not publish a release.

## Buttons and links

`Button` remains a native button. Replace the former visual choices as follows:

| Previous API | Current API |
| --- | --- |
| `ButtonVariant::Default` | `ButtonVariant::Solid` |
| `Secondary` | `Soft` with `color=Some(AccentColor::Gray)` |
| `Outline`, `Ghost` | Same names |
| `Accent` | `Solid` with `color_role=ButtonColorRole::Accent` |
| `Destructive` | `Solid` with `color_role=ButtonColorRole::Destructive` |
| `Link` | A native `Link` for navigation; `Ghost` and an explicit underline class for actions |
| `ButtonSize::Default`, `Sm`, `Lg` | `Two`, `One`, `Three` |
| `ButtonSize::Icon` | `IconButton`, with a required `aria_label`, default size `Two` |
| `button::ButtonClass` | Component props and caller `class` |

`Classic`, `Surface`, `GhostOffset` and size `Four` are also available. Constants
such as `size=ButtonSize::One` work directly. A changing size uses
`Signal<Responsive<ButtonSize>>`; change an old `Signal<ButtonSize>` to derive
`Responsive::from(size.get())`.

Unselected colors consume the effective semantic primary pair, including author
overrides. Explicit `color` affects that component's palette roles. The old
semantic accent and destructive pairs remain distinct from brand families.
Caller utility classes still override component defaults.

Use `r#type=ButtonType::Submit` or `Reset` for native form behavior; the default
is `Button`. `loading` preserves the label and width, sets busy/disabled state,
and refuses activation and native submission. `unavailable` retains focus and
delivers a request for the application to judge. An `IconButton` requires a
nonempty accessible name; its icon is decorative.

`Link` keeps native navigation, target/rel attribute passthrough, Within/Exact
matching and `aria-current`. Disabled links lose their href. Its size, weight,
trim and wrap are responsive; `LinkUnderline` selects Auto/Always/Hover/None.

## Layout and text

`Responsive<T>` has optional `initial`, `xs`, `sm`, `md`, `lg` and `xl` fields.
The minimum widths are 0/520/768/1024/1280/1640 CSS px. Missing values inherit
the closest smaller choice; the names differ from Tailwind's breakpoints.

```rust,ignore
use rustify_components::{Flex, FlexDirection, Gap, Responsive, Spacing};

view! {
    <Flex direction=Responsive {
        initial: Some(FlexDirection::Column),
        sm: Some(FlexDirection::Row),
        ..Default::default()
    } gap=Gap { gap: Responsive::from(Spacing::Scale(3)), ..Default::default() }>
        {children}
    </Flex>
}
```

Box/Flex/Grid/Container/Section accept applicable `Layout` and `Margin`
structures; padding lives in `Layout::padding`. Flex/Grid accept `Gap`.
Each structure exposes the fields in the [parameter matrix](validation/dom-components-p1/field-matrix.md).
`Spacing::Scale` uses the resolved theme scale. Explicit CSS sizes retain their
own units and are not multiplied by theme scaling. Inset consumes the nearest
container padding; Reset decorates exactly one typed native Leptos element.

Text selects span/div/label/p independently of visual size. Heading selects
H1–H6 independently of its default visual size Six. Blockquote, Code, Kbd,
Em, Quote and Strong retain their native semantics. TextSize, TextWeight,
TextAlign, TextTrim and TextWrap provide the applicable responsive choices.
Text and Link leave inherited size intact when size is omitted. Truncate is
a nonresponsive boolean.

## Styles and static rendering

Declare layer order before importing Tailwind's layers, then import the SDK
and the document runtime mappings into the application's one build:

```css
@layer theme, base, components, utilities;
@import "tailwindcss/theme.css" layer(theme);
@import "tailwindcss/utilities.css" layer(utilities) source(none);
@import "path/to/rustify-components/css/sdk.css";
@import "path/to/rustify-components/css/theme-v4.css";
@source "./src";
```

Regenerate the stylesheet after changing component classes. Applications
without their own Tailwind input can consume `rustify_components::CSS`.
Keep the SDK styles off the host's bare elements; no global reset is required.

For SSR, build with `ssr` separately from CSR/hydrate, seed `ssr_scope`, call
`provide_static_theme` with the complete snapshot and provide a `GeometryStyles`
collector. Write `styles.css()` into an external stylesheet. Use the same seed
and initial props when hydrating. Responsive rules share the existing scope
stylesheet lifecycle; invalid property values are available through
`GeometryStyles::diagnostics()` and do not terminate the application.

The browser `theme::Theme` entry reexports the SDK's ThemeScope, and
`theme::ThemeBoundary` uses the same effective source. Static rendering keeps
the existing `provide_static_theme` binding and external CSS contract.
`ThemePanel` is controlled: configuration and override counts are inputs,
changes are requests, and group reset requests explicitly cover both modes.

## Author documents

Use `base_values(mode)` for original author values and `effective_values(mode)`
for composed, unscaled values. Edit `overrides` to take precedence and remove
keys to reveal configuration/base. The former `.styles`, `.values()` and
`.values_mut()` accessors no longer describe the full author model.

`ThemeDocument::from_legacy` upgrades into Custom v2 without replacing author
colors. Configuration changes retain explicit overrides. `ThemeBoundaryPatch`
holds configuration and separate light/dark patches; `ThemeValuePatch::into()`
adapts a mode-independent sparse patch. See [Theme documents](themes.md) for
strict import rules, root preview mode and source-preserving local appearance.

## Existing exports that remain available

The table covers the pre-change root exports and supporting public modules
from `crates/rustify-components/src/lib.rs`. Entries without an API replacement
keep their current controlled behavior; an upstream component with a similar
name does not add new props or composition parts.

| Previous entry or group | Source checkout destination / boundary |
| --- | --- |
| `Button`, `ButtonSize`, `ButtonVariant` | Same root paths with the replacements above; no parallel LegacyButton implementation |
| `button::ButtonClass` | Component props plus `class`; the class-builder type is removed |
| `Link`, `LinkMatch`, `CurrentPath`, `provide_current_path`, `current_path`, `is_within` | Same paths and matching contract; visual options are additions |
| `Checkbox`, `RadioGroup`, `RadioOption`, `Switch` | Same controlled boolean/options APIs; no indeterminate state, standalone Radio or cards/groups added |
| `Select`, `SelectOption`, `Slider` | Same controlled value/open or single-value APIs; no public Select parts or multi-thumb Slider |
| `TextField`, `TextKind`, `TextArea`, `Label` | Same native field and naming APIs; no new text-field slot family |
| `Form`, `Field`, `FieldBinding`, `FormStatus`, `SubmitButton`, `provide_form`, `use_form` | Same validation/submission owner and binding APIs; not replaced by layout components or the reference Form parts |
| `Dialog`, `Tooltip`, `Menu`, `MenuItem`, `MenuItemKind`, `menu::anchor_at` / root `anchor_at` | Same scoped overlays and anchors; `anchor_at` remains browser-only. No AlertDialog/Popover/HoverCard/ContextMenu/DropdownMenu part families are implied |
| `Tabs`, `Tab`, `TabPanel`, `Orientation` | Same tabs data, panels and navigation semantics; no TabNav/SegmentedControl API |
| `ScrollArea`, `Boundary` | Same browser scroll and overscroll behavior |
| `Progress`, `Spinner`, `Toast`, `Toaster` | Same feedback APIs; no Skeleton or full Toast parts surface |
| `NumberField`, `ColorField` | Same focused Draft exception and preview/commit/cancel semantics |
| `DataTable`, `Column`, `Tree`, `TreeNode` | Same Rustify data controls; ordinary Table is a separate unimplemented reference capability |
| `ToggleGroup`, `ToggleItem` | Same controlled group/toolbar mode; no standalone Toggle or full Toolbar |
| `FilePicker`, `DropZone` | Same native file-selection/drop requests |
| `Splitter`, `PanelTabs`, `PanelTab`, `CommandPalette`, `Command` | Same workspace controls and data types |
| `Icon`, `Glyph` | Same SVG wrapper and six glyphs; IconButton reuses this icon path |
| `GeometryStyles`, `provide_geometry_styles`, `geometry` | Same external stylesheet collector, extended for media rules and field diagnostics |
| `clx!`, `variants!`, `macros`, `cn`, `paste` | Same class merging/generation helpers; caller utilities remain last in the merge and above component CSS layers |
| `CSS`, `CANDIDATES_JSON`, `styles` | Same package asset interfaces; regenerate the application's stylesheet when moving to changed source components |
| `id`, `roving` | Same deterministic identity and keyboard-navigation helpers |
| `CATALOG`, `Capability`, `Category`, `Entry`, `Presentation`, `Support` | Legacy 24-category navigation remains intact. Use `catalog::COMPONENTS`, `STABLE_REFERENCES`, `SUB_CAPABILITIES` and `catalogue_markdown()` for expanded source facts |

New layout, typography and theme types are listed in the generated
[component catalogue](components.md). Generated Leptos props remain in their
component modules; this migration does not promise React `Props` or ref
compatibility.

## Root Theme and the legacy Theme value

The existing `rustify_ui::Theme` (or `rustify_dom::Theme` for a DOM-only
consumer) remains a light/dark value palette. It has not been renamed into a
component. In a browser, give the new alias a local name when both are needed:

```rust,ignore
use leptos::prelude::*;
use rustify_components::theme::Theme as RootTheme;
use rustify_dom::{Theme as LegacyTheme, ThemeDocument};

let document = ThemeDocument::from_legacy(LegacyTheme::light(), LegacyTheme::dark());
// Resolve the document with the host's font/rem context, then mount:
view! { <RootTheme resolved=resolved /> }
```

`RootTheme` is only an import alias for the existing ThemeScope. It does not
wrap children or create a second mount. Supply it once inside an SDK mount;
use ThemeBoundary for local configuration and separate light/dark overrides.
Document appearance selects the root mode; an explicit `preview_mode` is a
preview choice, not a mutation of author appearance. Empty boundary patches
resume inheritance; invalid local candidates retain the parent snapshot and
report a diagnostic. See [the provider contract](themes.md#root-and-local-providers).

Legacy `ThemedScope`, `ThemeOverride`, `ThemePatch` and `Theme::light/dark`
consumption remain available. `ThemeDocument::from_legacy` and old v1 imports
select Custom configuration to preserve the old authored values. New
`ThemeDocument::standard` selects indigo/auto/translucent/medium/100%; changing
those choices does not remove explicit author overrides.

## JSON, CSS and spacing changes

| Previous access or format | Source checkout replacement |
| --- | --- |
| `ThemeDocument.styles`, `values(mode)`, `values_mut(mode)` | `base_values(mode)` / `base_values_mut(mode)` for original author tables; `effective_values(mode)` for unscaled composed values; edit sparse `overrides` for precedence |
| v1 `styles.light` / `styles.dark` JSON | Shared `import_json` accepts strict v1 and upgrades to Custom v2; use `export_json` for complete lossless v2 |
| Configuration inferred from final colors | Keep explicit `config` and `overrides`; author `accent` is a semantic role, while `accent_color` selects a named brand family |
| One current-mode patch reused as theme source | `ThemeBoundaryPatch` carries config and distinct `light`/`dark` maps; `ThemeValuePatch.into()` is the explicit same-patch-to-both adapter |
| Implicit v1 export | `export_json_v1_lossy` returns the JSON plus explicit losses; the old format cannot preserve configuration intent, override origins, full-radius roles or all derived geometry |
| Resolved CSS treated as an author document | `export_css` describes the resolved output; retain JSON to recover config, overrides, source strings and both author tables |

Source JSON input validates the whole candidate, both modes and the UTF-8
size limit. Unsupported versions, duplicate keys, mixed v1/v2 forms, malformed
values and unknown override keys are errors; unknown base string keys are
preserved and diagnosed. Display P3 Colors data does not expand the author
theme parser beyond its declared sRGB syntax.

Keep the utility unit, layout gap and component scale distinct. `spacing`
provides the utility unit (`--rustify-unit`, normally derived from .25rem);
`layout-gap` is an independent author value. Legacy `--spacing` remains its
legacy layout-gap projection. Theme step `Spacing::Scale(4)` reads
`--rustify-space-4` from the resolved snapshot; it does not apply scaling again.
`Spacing::Css("37px".into())` and explicit width/height strings stay 37px under
90/95/100/105/110 theme scaling. Breakpoints also stay fixed.

SSR renders with `ssr` separately from CSR/hydrate. Call `ssr_scope` with a
stable scope seed, bind the complete resolved snapshot with
`provide_static_theme`, and provide `GeometryStyles` before rendering children.
Publish the matching theme CSS and `GeometryStyles::css()` externally; the
static binding does not emit a browser provider or choose a new root mode.
Hydration uses the same seed and initial props. Scoped defaults and responsive
rules live in the components layer beneath utilities. CSSOM writes used by
provider/overlay projection may serialize a style attribute; they do not permit
arbitrary style strings or a relaxed Content Security Policy.

The source `build-web --release` pipeline removes the wasm `name` section to
avoid shipping expanded Rust generic names. Release wasm stack frames therefore
lose those function names; debug builds retain them. Export names and the
message-bridge fingerprint remain separate from this debugging metadata.

## Colors without a UI runtime

The source-only `rustify-colors` crate provides 31 named families with
light/dark, solid/alpha and sRGB/P3 strings, plus mode-independent black/white
alpha. Rust uses `ColorFamily::ALL`, `scale`, `css`, `black_alpha` and
`white_alpha`; steps are one-based 1–12. The optional Rust `p3` feature is
separate from static P3 CSS consumption.

Static consumers copy only the packaged assets they use, mark the receiving
element with `data-rustify-colors` and an explicit mode, and keep the asset
license. P3 styles import their relative sRGB fallback, so deploy both files.
No Colors selector changes unmarked host elements, `:root` or `.dark`.
See [the Colors package guide](../crates/rustify-colors/README.md) for exact
paths, markers and copying APIs. These assets require no React, Leptos,
Makepad, wasm or browser script.

## Published and source version pairing

The nine published Docs/DOM/style packages remain 0.2.1. Their frozen CLI
framework pair and registry source checks remain intact. The workspace's
unchanged version string does not make the new source APIs or v2 schema
available from crates.io; `rustify-colors` has not been published by this work.
An old registry CLI cannot consume complete v2 author documents.

Use one pinned checkout for the source SDK, stylesheet assets and source CLI.
Build the latter with the existing unwind-capable profile:

```sh
mbx build --locked -p rustify-docs-cli --profile docs-cli
target/docs-cli/rustify-docs --build-info
```

A source CLI author project must resolve its three direct framework packages
(`rustify-docs`, `rustify-dom`, `rustify-components`) to their canonical paths
in that checkout, following the existing development-source pairing check.
Do not mix registry and source framework dependencies, replace them with Git
dependencies to bypass the canonical-path check, or modify the frozen public
release record to claim a new registry pair.

The [independent consumer recipe](../tests/theme-export/README.md) describes
compiling the generated Studio Rust snippet and JSON outside the editor, with
root/child-path assets and strict CSP. The integration also supplies a DOM-only
consumer with default SDK features; running these entry points and recording
the source pair are separate acceptance steps, not implied by this guide.

The [source integration record](plan/DOM-COMPONENTS-P1.records/M6.md) records
the actual checkout, source CLI `--build-info` and independent author evidence.
Its validation baseline distinguishes working-tree evidence from a committed
source pair. A version string alone does not identify that pair.

## Storage upgrade, backup and rollback

Studio data belongs to the browser origin. Keep the previous deployment and its
matching manifest, lockfile and CLI available before changing versions. Close
older Studio tabs: they do not participate in the new storage lock protocol.

| Storage key | Purpose |
| --- | --- |
| `rustify-ui.theme-studio.v1` | The legacy library. The source upgrade reads it without rewriting it. |
| `rustify-ui.theme-studio.v2` | The new library: envelope version 2, revision, draft, named themes, preferences and optional migration metadata. |
| `rustify-ui.theme-studio.v1.backup.<sha256>` | Exact original v1 text, identified by its SHA-256 digest. A different original produces a separate backup. |

An existing v2 library is checked first. A bad v2 is protected rather than
replaced from v1. Only when v2 is absent does Studio convert the complete v1
library into memory. An unsupported version or one invalid saved theme rejects
the whole library; **Download original data** (下载原数据) keeps the rejected
text available before any explicit cache reset.

### Upgrade or continue in memory

1. With a valid v1 library, check the restored draft, named themes and
   preferences. Restoration starts with empty undo history and does not commit
   an upgrade. Editing and export remain available.
2. Choose **Review / recheck upgrade** (检查 / 重新检查升级). Read the complete
   candidate: it includes every legacy named theme and the current draft and
   preferences. **Download original before upgrade** (下载升级前原始数据) saves
   the original library text separately.
3. Choose **Upgrade and save current draft** (升级并保存当前草稿) to approve that
   candidate. A changed draft requires a refreshed review and another explicit
   confirmation. A changed v1 source also requires review again. Waiting for
   the lock leaves editing, undo/redo and export available; it is not a saved
   result.
4. Choose **Cancel upgrade** (取消升级) to keep working in memory. Cancellation
   preserves the open theme, history and preview mode, writes no cancellation
   marker, and keeps subsequent edits, preferences and ordinary library writes
   protected. Export work you need before closing: reopening restores the
   original v1 library and asks again. Use **Review / recheck upgrade** when
   ready to reconsider.

Before committing v2, migration writes the exact reviewed v1 text to its backup
key and reads it back. An existing backup must contain identical text; it is
never overwritten with different content. Only then can the v2 write be
confirmed. A failed backup does not permit a v2 commit; a failed v2 commit keeps
any backup already written. Normal saves, theme deletion and draft reset do
not remove these originals.

### Retrieve originals and recover a failed save

After a successful upgrade, **Download original before upgrade** reads the
backup named by the v2 migration metadata and checks its digest. It still works
after reopening or after an older window changes v1. Missing or damaged
backups report an error; Studio does not substitute the current v1 contents.
An older-window notice offers **Download the changed v1 data** (下载变化后的 v1
原文) separately when that data remains available.

Without secure Web Locks or storage access, use memory editing and downloads;
local write actions are rejected. Quota and other write failures pause saving.
Open **Pending / recent operations** (待处理 / 最近操作) to see retained named
actions and their results. After fixing the cause, **Retry retained saves**
(重试已保留的保存) approves those actions in their original order. For a failed
upgrade, retry returns to review; confirm the refreshed candidate explicitly.
**Cancel pending writes** (取消待写操作) discards pending writes while preserving
the open draft and history. A previous snapshot being saved does not mean that
newer edits have been saved; wait for **Saved on this device** (已保存到此设备).

### Recheck a conflicting version

The conflict view shows the external revision, draft name and named-theme
count. **View exact stored text** (查看存储原文) and **Download displayed
original** (下载当前展示原文) expose that version's text. Invalid text, including
an empty string, is distinct from a removed library. Invalid data remains
protected until an explicit reviewed reset.

| Choice | Effect |
| --- | --- |
| **Load other version** (载入另一版本) | Review the confirmation, then load the validated external version and cancel all pending writes. Export the current draft first if needed. |
| **Keep current draft** (保留当前草稿) | Save the current draft and preferences against the displayed external library. If the external library was removed, this creates a new one. |
| **Save current as a copy** (将当前草稿另存为副本) | Supply a new name to add the current draft to the displayed external library. |

Each choice compares the reviewed version again inside the lock. Another write
requires another decision. Keep and Save copy approve their displayed
candidate; retained pending named actions still need a separate retry or
cancellation. They are not silently replayed or discarded.

### Clear v2 or delete original backups

Open **Cache and original backups** (缓存与原始备份). Choose **Review clearing
v2 cache** (检查清除 v2 缓存), then **Clear reviewed v2 cache** (清除已检查的 v2
缓存). This cancels preceding pending writes, compares the reviewed v2 value
inside the lock, and removes only v2. It keeps the open draft, v1 and backups.
While removal is pending, edits stay in memory and library writes are rejected.
A failed removal does not report success. After success, use **Save current
draft again** (重新保存当前草稿) explicitly to resume persistence; old queued
writes cannot recreate the cache.

Backup deletion is separate. Choose **Review original backups** (检查原始备份)
and use each entry's **Download original** (下载原文) before choosing **Delete
only the reviewed backups** (只删除已检查的备份). Deletion compares the reviewed
key/value set under the same lock; a changed set requires another review.
Deleting backups is irreversible and leaves the current v1 and v2 keys alone.

### Roll back to the previous site

1. In **Export** (导出), choose **Theme JSON v2**, then **Download** (下载) to
   retain the open theme without losing configuration or override intent.
   Export other named themes you need individually, or separately archive the
   full v2 storage value: a theme export is not a whole-library/preferences
   backup.
2. If the old site must consume new work, choose **Compatible JSON v1 · lossy**
   (兼容 JSON v1 · 有损), read the displayed losses, then copy or download it
   for the old site's theme import. Keep the v2 file as well; v1 cannot restore
   the full configured component appearance or original override sources.
3. Close Studio tabs and restore the complete previous deployment, including
   its matching wasm, CSS, fonts and manifest. The old site reads v1, not v2;
   the new editor never automatically copies v2 edits back into that key.
4. If v1 was changed or removed and the pre-upgrade library is needed, archive
   any current v1 value first. Close Studio tabs and open a non-Studio page on
   the same origin, such as `build-manifest.json`. In the browser's storage
   inspector, restore the downloaded original library text verbatim to the v1
   key, then reopen the old site. This replaces that v1 library. The original
   download is a library envelope, not a single-theme
   file for the theme import dialog. Keep v2 and its exported files separate.

Returning to the source site selects a valid existing v2 first; old-site edits
in v1 remain separate. Verification results and their scope are kept in the
[integration record](plan/DOM-COMPONENTS-P1.records/M5.md).

## Capabilities that are still absent

The complete reference inventory includes Accordion, AlertDialog, content
components, selection cards, navigation families and other entries that have
no public Rust implementation yet. Existing Select/Dialog/Menu/Checkbox/
RadioGroup/Slider APIs expose their declared controlled subset; they do not
establish complete unstyled Primitives, three-state selection, multiple thumbs
or public part composition. Generic asChild, Slot and Slottable are absent.
Unstable reference exports are candidates only. Consult the generated
[reference and sub-capability tables](components.md#stable-reference-inventory)
before treating a same-name component or demo as complete support.
