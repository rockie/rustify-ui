# Public panel consumer

A DOM-only source consumer created from the public onboarding documentation.
The application owns note, accepted delivery choice and feedback state. It uses
the matching source DOM core, components and exported CSS, with no GPU feature.

Public sources used for the original panel sample:

- Root README: source-version boundary and panel guide link.
- `docs/quickstart.md`, “A DOM application without GPU regions” and “Composable
  form components from source”: dependencies, loader, asset export and build-dom.
- `docs/panels.md`, “Prepare the DOM runtime and styles”, “Compose a safe
  confirmation, choice and result”, and “Dismissal, focus and native targets”.
- `docs/dom-components-migration.md`: source pairing and Callout status role.
- The quick start's linked `tests/dom-only/consumer` manifest, `src/lib.rs`,
  `src/bin/export-dom-assets.rs` and public page: exact dependency versions,
  mount/handle lifetime and asset writer shape. No private component implementation
  or development-plan content supplied application steps.

From the repository root:

```sh
mbx generate-lockfile --offline --manifest-path tests/dom-only/public-panel-consumer/Cargo.toml
mbx xtask build-dom --manifest-path tests/dom-only/public-panel-consumer/Cargo.toml
mbx xtask serve --dir target/dom/public-panel-consumer --port 4195
```

The static product is `target/dom/public-panel-consumer/`. The `serve` command
does not build. Do not open `index.html` directly as a file.
The committed consumer lock file makes subsequent `build-dom` calls reproducible;
that command requires it through `--locked`. Offline lock generation above uses
dependencies already cached in this development checkout.

Manual workflow:

1. Load the page and wait for the note input; `html[data-ready="true"]` marks
   successful mounting.
2. Change Note, open Clear note, confirm that Keep note receives initial focus,
   and cancel. The note and result must remain unchanged.
3. Open Delivery options, open Delivery speed, choose Express and choose Done.
   Accepted delivery must read `express`.
4. Open Clear note and activate its Clear note action. The input becomes empty,
   the persistent result reports that the note was cleared, a success Toast
   appears, and the accepted delivery remains `express`.
5. Repeat the confirmation using keyboard navigation; inspect console and failed
   requests. Check the same content at a narrow viewport.

Validation on 2026-11-01:

- `cargo fmt --manifest-path tests/dom-only/public-panel-consumer/Cargo.toml -- --check`
  passed.
- Offline lock generation passed; `mbx xtask build-dom --manifest-path
  tests/dom-only/public-panel-consumer/Cargo.toml` passed and produced
  `target/dom/public-panel-consumer/`.
- The build reported unused `fallback_element` and `guest_click` warnings in the
  source DOM core. No consumer compile errors occurred.
- No public onboarding step or API gap remained after following the quick start's
  explicit lock-file prerequisite.
- `npx --no-install playwright test -c tests/dom-only/public-panel-consumer/playwright.config.ts --project=public-panel`
  passed (1 browser scenario): safe cancellation, keyboard Express selection,
  keyboard confirmation, 390px layout and persistent feedback after Toast expiry.
  Console, page errors, failed requests and CSP checks were empty. Screenshot:
  `docs/validation/dom-components-p3/m7-public-consumer-390.png`.
- This is an agent role simulation from public guides, not a human usability or
  assistive-technology result.

## Menu application shell

The DOM-only `ApplicationShell` in `src/lib.rs` surrounds the original
`NotePanel` with application commands, native site navigation, a toolbar with
a composed dropdown trigger, and a context menu on the saved note. The
application owns all open values, the current note and its session-only saved
copy. Save updates that copy; Restore updates the same input used by the
original confirmation workflow. Reloading starts a new session.

The source SDK dependencies and asset exporter use the same checkout;
`rustify_components::CSS` supplies the matching themed CSS. See
[the menu guide](../../../docs/menus.md) for the public composition API.

The existing quiet, single-column layout, system typography and spacing remain
the visual basis. `public/app.css` adds the header, locally scrollable toolbar,
saved-note target and help section, with a 390px layout. No GPU is involved.

Run from the repository root:

```sh
cargo fmt --manifest-path tests/dom-only/public-panel-consumer/Cargo.toml -- --check
mbx xtask build-dom --manifest-path tests/dom-only/public-panel-consumer/Cargo.toml
npx --no-install playwright test -c tests/dom-only/public-panel-consumer/playwright.config.ts --project=public-panel
```

The `public-panel` project serves this consumer at `http://127.0.0.1:4206` and
runs the original panel workflow plus the application-shell workflow in
`public-panel.spec.ts`. Screenshots go to the configured Playwright output
directory, using `testInfo.outputPath`, so running the original scenario no
longer overwrites the archived P3 screenshot.

Application-shell workflow:

1. Edit Note, focus File and press ArrowDown/Enter to save; verify the visible
   saved copy and session feedback.
2. Edit again, open File and press ArrowRight to switch to Edit; select Restore
   saved note and verify the input returns to the saved copy.
3. Open Help and follow the native Delivery guide link; follow Current note
   back. Verify its href and `aria-current` semantics.
4. Save from the toolbar, edit again, and use ArrowRight to reach More. Open it,
   dismiss with Escape, reopen and restore; verify focus and the resulting note.
5. Right-click Saved note and replace the saved copy. Edit again, focus Saved
   note and press Shift+F10/Enter to restore; verify focus returns to the target.
6. Inspect the complete desktop shell and its 390px layout with the More menu
   open; check horizontal overflow, browser console, page errors, failed
   requests and CSP violations.
