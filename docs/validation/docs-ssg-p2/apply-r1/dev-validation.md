# Dev dependency browser regression

DOM-only dev validation used the installed CLI in [dev-pair.json](dev-pair.json): CLI 0.1.15, source C `45878ce31763fe21b9276e563e2920d180787dee`, paired framework R `7930fd9a978563e76468fef69d14f444159c47b5`. The CLI build-info, binary SHA-256 and R read from C's framework-revision asset are recorded there; each dependency attachment confirms the actual three framework package sources in both isolated feature graphs.

## Results

| Run | Result | Evidence |
| --- | --- | --- |
| Hydrate-only manifest regression | 1 passed, 57.3 seconds | [log](dev-hydrate-only-green.log), [JSON](dev-hydrate-only-green-results.json), [attachments](dev-hydrate-only-green/test-1/) |
| Renamed same-name build-input regression | 1 passed, 2.6 minutes | [log](dev-renamed-inputs-green.log), [JSON](dev-renamed-inputs-green-results.json), [attachments](dev-renamed-inputs-green/test-1/) |
| Complete dev-components file | 4 passed, 5.7 minutes | [log](dev-components-green.log), [JSON](dev-components-green-results.json), [attachments](dev-components-green/) |

[dev-green-summary.json](dev-green-summary.json) preserves individual test durations/statuses. JSON reporter attachment bodies were decoded into the corresponding `test-*/` directories and replaced with file paths; test results were not changed.

The hydrate-only test changes only its optional local dependency's default feature after initialization. SSR and hydration initial values remain 7; post-click value/background change from 17 / `rgb(18, 58, 188)` to 27 / `rgb(254, 220, 186)`. Wasm SHA-256 and compile generation change. A subsequent Markdown-only save keeps the compiled pair and wasm SHA and does not add any compiler-wrapper command.

The build-input test uses two independent-workspace packages named `payload`, versions 1.0.0 and 2.0.0, with renamed path dependencies and separate `rerun-if-changed=payload.txt` files. Each package's save and restore changes its corresponding SSR initial value, wasm interaction, wasm SHA and generation without editing site source. Deleting either file reports a build failure while retaining the last-good SSR, live DOM and wasm; the retained controls remain interactive. Restores clear errors and hide the problem panel; the fixture's two warning markers remain.

Across all six executed test instances, browser audits record zero CSP violations, console errors, page errors, failed requests and HTTP responses >=400. Before/after dev lock bytes match for every dependency regression. Metadata attachments contain only the author, added local packages, three framework packages and their relevant resolve nodes/features, not third-party package metadata.

## Commands

All three commands used:

```sh
export PATH="/opt/homebrew/opt/python@3.12/libexec/bin:$PATH"
export RUSTIFY_DOCS_BIN="/tmp/rustify-apply-r1-installed.SgjSel/bin/rustify-docs"
export CARGO_NET_OFFLINE=true
```

For each run, `PLAYWRIGHT_JSON_OUTPUT_NAME` was set to the corresponding `dev-*-green-results.json`; stdout/stderr were redirected to `dev-*-green.log`:

```sh
npx playwright test --config=playwright.docs.config.ts --project=docs-dev \
  tests/docs-ssg/dev-components.spec.ts --grep 'hydrate-only dependency manifests' \
  --workers=1 --reporter=line,json --output=docs/validation/docs-ssg-p2/apply-r1/dev-hydrate-only-green

npx playwright test --config=playwright.docs.config.ts --project=docs-dev \
  tests/docs-ssg/dev-components.spec.ts --grep 'renamed same-name build inputs' \
  --workers=1 --reporter=line,json --output=docs/validation/docs-ssg-p2/apply-r1/dev-renamed-inputs-green

npx playwright test --config=playwright.docs.config.ts --project=docs-dev \
  tests/docs-ssg/dev-components.spec.ts --workers=1 --reporter=line,json \
  --output=docs/validation/docs-ssg-p2/apply-r1/dev-components-green
```

Runs were sequential, using background runners and finite sleep/status checks. All exited 0. No self-owned dev, compiler or browser process remained when the runtime slot was returned.

## Scope

This validates the real ephemeral dev server in Playwright Chromium, not stable Chrome/Safari, Portal hosting or GPU presentation. Other pure-content dev suites and native/performance checks were not run in this slot. The old CLI was not rerun successfully; [dev-setup-attempts.md](dev-setup-attempts.md) distinguishes its invalid setup attempts from product red evidence. Existing review evidence was not modified.
