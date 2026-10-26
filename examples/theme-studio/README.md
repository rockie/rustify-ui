# Theme Studio

Theme Studio is the [Rustify UI](../../README.md) local theme editor. It edits a
versioned document, previews one resolved theme through Leptos DOM and Makepad
GPU consumers, and exports JSON, Tailwind v4 CSS or a Rust integration snippet.
It has 44 built-in themes: two Rustify defaults and the 42 vendored tweakcn presets.
AI generation, accounts, cloud storage and community publishing are outside
this example.

## Run

Use the repository's [pinned toolchain](../../mise.toml) and run from the root:

```sh
mbx xtask build-web --example theme-studio --release
mbx xtask serve --example theme-studio --release
```

Open the URL printed by `serve`. The page's startup status becomes `ready` when
the editor mounts; GPU readiness is reported separately. `serve` serves an
existing build, so rebuild after source changes. The root product is
`target/makepad-wasm-app/release/theme-studio/`. Use HTTP static hosting and a
browser with WebAssembly; GPU preview uses WebGL2. It has no backend or online
font dependency.

For child-path hosting, use the same base on both commands:

```sh
mbx xtask build-web --example theme-studio --release --base /tools/demo/
mbx xtask serve --example theme-studio --release --base /tools/demo/
```

This writes a separate `theme-studio@tools-demo` directory beside the root
product. Publish the entire directory, including fonts, loader, bridge and build
manifest. The preview server uses the repository's strict CSP; mismatched build
and serve bases are rejected. Theme values belong to preview scope roots,
allowing independent light/dark previews on the same page.

## Edit and exchange

Choose or search a preset, change light/dark mode, and edit Colors, Typography
or Other. Base configuration offers appearance, brand and neutral families,
panel background, radius and scaling. Explicit overrides remain in place when
configuration changes. The source display can remove one token's override;
group reset removes only its responsibility keys in both modes. Preview mode
selects the displayed mode without changing the document's author appearance.
Common fields such as fonts, radius and shadow geometry edit both
modes; mode differences from imported or reference themes are preserved until
an explicit edit. DOM, GPU and compare views use the same resolved snapshot.
The scenes are Cards, Dashboard, Application, Marketing, Mail, Typography and
Color Palette. Unsupported GPU business controls remain available through DOM
preview. The English/Chinese shell preference does not translate token names,
author values or theme names.

Undo/redo and reset operate on committed theme edits. A continuous slider
gesture produces one history entry; unfinished invalid text keeps the last
valid preview. Text inputs and IME composition retain their native undo
behavior. The inspector connects marked DOM samples and GPU sample rectangles
to their editable tokens.

Import accepts pasted text or UTF-8 files with .json/.css extensions up to
256 KiB. Review the candidate's differences and diagnostics, then choose Apply
or Cancel. Changing the editor after review requires a new review before Apply.
A failed or canceled import leaves the existing theme intact. JSON requires both
complete modes and preserves author strings. CSS extracts supported theme
declarations, fills missing fields from the current document and reports
ignored rules; it is never executed or inserted into the page.

Export complete JSON v2 to preserve configuration and override intent. The
explicit **Compatible JSON v1 · lossy** choice lists the information that the
older format loses before copying or downloading. CSS offers standard
`:root`/`.dark` and Rustify scoped profiles, with HEX/RGB/HSL/OKLCH notation for
resolved sRGB colors. The Rust snippet loads the matching JSON file, theme.json,
resolves it with `font_context` and mounts ThemeScope. An ordinary provider
honors authored appearance; `mount_exported_preview` explicitly selects the
mode shown in Studio. Copy reports the actual clipboard
result; if access is refused, select the text or download it. A download reports
that it started, rather than claiming the file was saved by the user.

See the [SDK theme guide](../../docs/themes.md),
[source API and storage migration guide](../../docs/dom-components-migration.md),
[v2 document schema](../../docs/theme-document.schema.json) and
[independent export consumer](../../tests/theme-export/README.md). CSS exports
reference font files; copy those resources and their
[license notices](../../makepad/widgets/resources/FONT-LICENSES.md) with the
application. The shared font catalogue includes the repaired Rustify WenKai
family at the existing `LXGWWenKaiRegular.ttf` path. Unknown author stacks are
retained while preview diagnostics identify the bundled fallback.

## Local data and failures

New writes use `rustify-ui.theme-studio.v2`. Its envelope contains
`envelope_version: 2`, a monotonic `revision`, nullable `migrated_from_v1`,
the complete `draft`, `saved_themes` and `preferences`.
Preferences include `locale` (`en`/`zh`), favorites, scene, renderer
(`dom`/`gpu`/`compare`), preview width
(`responsive`/`desktop`/`tablet`/`mobile`) and mode (`light`/`dark`). The library
allows at most 100 named themes. Names contain 1–80 characters, are trimmed for
local actions and cannot duplicate another saved name. Overwrite and delete
require explicit DOM confirmation; rename preserves the id, and copy creates
a new id.

Saving requires a secure context and Web Locks. One ordered writer validates
each candidate, compares the last acknowledged storage bytes inside the lock,
then writes and checks the result. Only adjacent, unissued draft autosaves
merge; named actions keep their order. Waiting for a lock leaves editing,
history and export available. An old acknowledgement confirms its captured
snapshot and keeps newer edits, mode and preferences. Failure pauses the queue;
retained actions are visible and require an explicit retry or cancellation.
Without locks or storage access, editing and downloads remain available in
memory. Gesture updates do not enqueue autosaves of intermediate values;
commit or cancellation schedules the current committed draft.

When v2 is absent, a valid `rustify-ui.theme-studio.v1` library is restored in
memory. Review the complete candidate and explicitly confirm its upgrade.
Cancelling keeps later edits, mode changes and preferences write-protected;
reopening restores the original v1 data. Before committing v2, migration stores
the exact original bytes under `rustify-ui.theme-studio.v1.backup.<sha256>` and
reads them back. The v1 key remains unchanged. Original backups can be
downloaded after reopening; an older window changing v1 produces a separate
notice rather than replacing v2. Unknown versions or one invalid saved theme
reject the complete library, and a bad v2 never falls back to v1 automatically.

A malformed cache retains its original bytes for download and blocks automatic
replacement. Explicit Clear local cache compares the reviewed v2 bytes,
cancels pending writes and keeps the open draft, v1 source and original
backups. While removal waits, edits stay in memory and library actions are
explicitly rejected. After removal, saving requires an explicit resume.
Backup deletion is a separate confirmation that compares the reviewed key
set. A real change from another same-origin tab prompts Load, Keep or
Save copy; it does not replace the draft or silently merge changes. Load adopts
the validated external envelope. Keep explicitly writes the current draft;
Save copy preserves it under a new name in the external library. A failed write
keeps the conflict unresolved. Another write after a decision requires a new
decision. Keep and Save copy do not silently replay retained named actions;
retry or cancel those actions separately.

Runtime parsing errors include the field and message. Warnings explain font
fallback, gamut clipping or mode differences. A GPU failure leaves the DOM
preview available. If startup fails, inspect the visible diagnostic, console
and failed resource requests; verify the current build manifest and deployment
base before changing theme values.

## Checks and browser acceptance

```sh
node --experimental-vm-modules examples/theme-studio/vendor/tweakcn/normalize.mjs --check
mbx test -p rustify-ui --lib theme
mbx test -p theme-studio --bins
mbx xtask sources verify
mbx xtask css --check
mbx xtask build-web --example theme-studio --release
mbx xtask build-web --example theme-studio --release --base /tools/demo/
npx playwright test --project=theme-studio --workers=1 --output=test-results/theme-studio
npx playwright test --project=theme-studio-deep --workers=1 --output=test-results/theme-studio-deep
```

Run these projects in sequence. The root project starts its preview on port
4181; `theme-studio-deep` starts port 4182 beneath `/tools/demo/`. Stop manual
servers occupying those ports. The [Playwright configuration](../../playwright.config.ts)
starts only the servers required by the selected project and uses one worker
by default. Its startup and storage checks use fresh browser contexts, and
cross-tab checks create two pages in the same context so the browser delivers
real storage events. Rebuild both products before checking them.

The independent exported CSS/JSON/Rust consumers have their own root and
child-path projects. Build their static directories and server binary first, then run
each project separately:

```sh
sh tests/theme-export/build.sh
sh tests/theme-export/build.sh --base /tools/theme-export/ --out tests/theme-export/target/site-subpath
npx playwright test --project=theme-export --workers=1 --output=test-results/theme-export
npx playwright test --project=theme-export-deep --workers=1 --output=test-results/theme-export-deep
```

Those projects start strict CSP previews on ports 4183 (`/`) and 4184
(`/tools/theme-export/`); see the [fixture guide](../../tests/theme-export/README.md).
The fixture build also compiles their preview server.
Use one project and one browser run at a time. The commands describe checks to
run; this guide does not declare browser or milestone acceptance.

Theme update timing is a separate headed-browser measurement. After rebuilding
the root product, run it on the repository's real-GPU Chrome environment:

```sh
RUSTIFY_TIER=evidence npx playwright test --project=theme-studio theme-performance.spec.ts --headed --workers=1 --output=test-results/theme-studio-performance
```

The measurement fixture skips headless runs. A skipped CI run does not provide
DOM/GPU latency or idle measurements.

The vendored [provenance](vendor/tweakcn/provenance.json),
[normalized presets](vendor/tweakcn/presets.json),
[copied inputs](vendor/tweakcn/source/) and [Apache-2.0 license](vendor/tweakcn/LICENSE)
identify the reference snapshot. Normal builds read committed inputs and do not
read `ref/` or fetch remote presets. Source and font digests are registered in
[sources.lock.json](../../sources.lock.json).
