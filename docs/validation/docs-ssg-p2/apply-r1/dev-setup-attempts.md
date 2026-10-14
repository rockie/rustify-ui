# Dev regression setup attempts

These attempts are test setup diagnostics, not successful product-defect reproductions. Original review evidence under `../code-review-r1/` was not modified.

- An old CLI attempt placed a package with its own `[workspace]` below `site.root`; Cargo metadata rejected multiple workspace roots. The regression now places `browser-palette`, `payload-v1` and `payload-v2` beside `site.root` under `site.dir`, using `../…` path dependencies and preserving independent workspaces.
- A subsequent old CLI attempt reached actual compilation but the foreground bash tool was cancelled at its 300-second limit, before Playwright's teardown saved an audit. Its detached dev process was terminated and its temporary project removed. No product red result is claimed.
- The cancelled attempt navigated to the initial building shell. The dedicated dependency regressions now release the compiler, wait for successful initialization, then navigate to the counter page. They use a 30-second hydration readiness gate with failure screenshot/audit; the existing first-building-shell regression is unchanged.
- Invalid artifacts formerly at `dev-hydrate-only-red/`, `dev-hydrate-only-red-results.json` and `dev-hydrate-only-red.log` were removed. Valid paired-CLI runs use `dev-hydrate-only-green*`, `dev-renamed-inputs-green*` and `dev-components-green*` instead.

Long runtime commands now execute in the background with finite sleep/status checks, so the bash tool's per-call limit does not cancel the browser runner.
