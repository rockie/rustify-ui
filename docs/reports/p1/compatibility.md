# P1 compatibility report

Where this preview is claimed to work, where it is only observed, and what is
pinned so that a result can be reproduced. The living capability document is
[`docs/compatibility.md`](../../compatibility.md); this report is P1's
conclusion about the matrix.

## The matrix

| Environment | Status in P1 |
| --- | --- |
| macOS Chrome 152 (Darwin 25.6, Apple Silicon) | **Pass gate.** The build was opened in it; regions render and both halves move one value. The manual records that need a real Chrome session are listed as missing below |
| Playwright 1.63.0 bundled Chromium, headless, SwiftShader | Continuous regression. 114 browser tests across three projects. It is not the product Chrome and never stands in for it |
| macOS Safari | Observation only, recorded and never blocking. **Not run.** The record is one of the four missing manual files |
| Windows, Linux, mobile | Not tested. No support is claimed and none should be inferred |

`--use-angle=swiftshader-webgl` means every GPU figure in [the performance
report](performance.md) is a software figure. Context creation and shader
compilation there are far slower than in Chrome on the same machine, so the
headless numbers are a floor for correctness, not a performance statement.

## What is pinned

| Component | Version | Where |
| --- | --- | --- |
| Rust toolchain | `nightly-2026-05-20` (rustc 1.97.0-nightly), `rust-src`, `wasm32-unknown-unknown`, `-Z build-std` with a generated single-threaded target spec | `rust-toolchain.toml` |
| Leptos | `=0.8.20` from crates.io, feature `csr` only; no Leptos source is modified | `Cargo.toml`, `Cargo.lock` |
| wasm-bindgen | `0.2.128` crate and CLI support, run in-process; a mismatch fails the build | `Cargo.lock`, checked by `cargo xtask doctor` |
| Makepad | Hard fork in `makepad/`, never synced with upstream again; 1,455 files with a digest each | `sources.lock.json` |
| noUiSlider | `15.8.1`, MIT, vendored with its licence; the build copies it and nothing fetches it | `sources.lock.json`, `cargo xtask sources verify` |
| Playwright / Node | `@playwright/test 1.63.0`, Node 26.1.0 (tests only; the build needs no Node) | `package-lock.json` |

`cargo xtask sources verify` fails on a modified vendored file and lists every
file of the fork that has drifted since import, so a fork edit is attributable
to the milestone that made it.

## The deployment contract

Verified in M7: root, sub-path (`--base /tools/demo/`) and embedded in an
existing page, all from one build directory; every request same-origin and
under the base; no cross-origin isolation; the release policy needs no
`unsafe-inline` and no JavaScript `unsafe-eval`, because the message bridge is
generated at build time and the theme is written through the CSSOM. Assets from
two different builds are refused at start.

## Host-page coexistence

A scope mounts into an element the page owns. The page keeps its links,
scrolling and text selection; the scope writes no document title, no URL, no
hash and no `body` style; no hidden text box is left in the document; the theme
lands on the scope's own root rather than on the document. Two scopes on one
page keep separate state, separate themes and separate overrides, and closing
one leaves the other operable.

The exception is the one that cannot be papered over: **all scopes share one
wasm module**, so a trap ends all of them (ADR-1).

## Third-party components

Exactly one is verified — noUiSlider 15.8.1 — and nothing is claimed about any
other. What was checked: twenty rebuilds leave one instance and one
subscription, one external change produces exactly one callback, focus is not
taken across a rebuild, and the version cannot drift silently. What is not
supported: anything needing `eval`, an inline script, or a stylesheet from
another origin; a component that writes to `document.title`, the URL or history
reaches the host page, because the SDK does not sandbox it.

## Fonts

All fonts ship with the build (51,187,844 bytes across ten files) and are
fetched by a region when it needs them. A first load fetches exactly one of
them — `IBMPlexSans-Text.ttf`, 181,792 bytes — because that is all the Latin UI
needs; see [the performance report](performance.md) for the measured transfer.
Licences are in `makepad/widgets/resources/FONT-LICENSES.md`; every text font
is OFL 1.1.

Each region loads the fonts it needs independently, so two regions drawing the
same script hold two copies in linear memory. That is a recorded cost, not a
leak.
