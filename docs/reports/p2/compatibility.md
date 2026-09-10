# P2 compatibility report

What this release was built against, what it was tested on, and what is only
observed. The capability detail lives in
[`docs/compatibility.md`](../../compatibility.md); this report says what was
*run* and where.

## The pass gate

| | |
| --- | --- |
| Gate | macOS Chrome at the Playwright-bundled Chromium version (1.63.0), SwiftShader WebGL |
| Observed, not gated | macOS Safari, recorded in the manual pass and not blocking |
| Not tested | Windows, Linux, Android, iOS, Firefox |

WebGL2 under SwiftShader is a software rasteriser. It exercises every code path
the region has, and it is not evidence about frame rates on real hardware -
which is one reason no performance budget is claimed.

## What is pinned, and by what

| Pinned | Version | Kept honest by |
| --- | --- | --- |
| Leptos | 0.8.20 (`csr`) | `Cargo.toml` workspace dependency, exact |
| wasm-bindgen / js-sys / web-sys | 0.2.128 / 0.3.105 / 0.3.105 | exact versions in the workspace |
| tw_merge | 0.1.21 | exact; its `prefix` option is deliberately unset (D13) |
| Tailwind | 4.1.13 | the committed stylesheet plus `cargo xtask css --check` |
| The Makepad fork | `sources.lock.json` | `cargo xtask sources verify` reports drift file by file |
| Rust/UI imports | `sources.lock.json.rust_ui` | each file marked verbatim or rewritten, and checked |
| Vendored browser files | `sources.lock.json.vendor` | a modified vendored file is an error, not drift |

The fork is first-class source and is *expected* to have drifted; the point of
the record is that every difference is attributable. The M6 change to
`platform/src/os/web/` - one new message and the wheel decision that reads it -
is the release's only fork change beyond M4's deployment-path fix.

## The build contract

The JS, the wasm and the generated message bridge have to be one build. The
bridge is extracted from the wasm at build time and its hash is checked at
start-up; a mismatch is `BuildContractMismatch` and a visible failure notice
rather than a page that half works. `cargo xtask serve --fault stale-bridge`
exercises it.

## Content security policy

The strict policy is what the examples are served under and what the tests run
against:

```
default-src 'none'; script-src 'self' 'wasm-unsafe-eval'; style-src 'self';
img-src 'self' data:; font-src 'self'; connect-src 'self'; base-uri 'none';
form-action 'none'
```

Nothing was added for P2. The export uses a `blob:` object URL and an anchor's
`download` attribute, which the policy does not govern - checked rather than
assumed, by comparing the bytes the browser saved against the bytes the
application meant to write.

## Deployment

A build is a plain static directory and carries the base it was made for.
Serving it from anywhere else is refused at start-up rather than producing a
page whose scripts 404. Deep links need a history fallback; `docs/navigation.md`
gives the nginx and local equivalents for both a root and a sub-path
deployment, and both are exercised by the same spec running under two servers.
