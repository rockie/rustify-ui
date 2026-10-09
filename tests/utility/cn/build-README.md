# cn build core differential checks

The implementation lives in `crates/rustify-style/src/cn/build.rs` and its
`build/` modules. It implements the fixed cn `build`, `expandGlobs`,
`createContentMatcher`, `extractTokens`, `themeFromCss`, `isTailwindCss`, and
`createBuilder` behavior in Rust. `tools.rs` provides CLI parsing/execution,
Vite lifecycle hooks, and Next config preparation/watch-event routing.

`BuildSourceTree::{Native, Virtual}` supplies concrete project sources. Native
IO and virtual byte files use lossy UTF-8 decoding, like Node's filesystem text
reads. Both support symlink traversal with branch-local cycle detection. The
virtual tree also represents unreadable paths, enabling deterministic warning
and error tests without depending on host permissions.

`ProjectOptions` retains optional `cwd`: omission uses the host current
directory (virtual root `/`); an explicit directory overrides a Vite root.
`ProjectReport` carries the emitted `BuildReport` plus absolute output path,
scan count, CSS dependencies, and warnings. Builds leave identical outputs
untouched. Builders return shared reports, preserve the identity for edits that
reach no dropped groups, and provide synchronous and awaitable entry points.
Concurrent/reentrant async run requests share a flight and at most one
follow-up; a failed flight clears its queue and can restart.

Config module loading remains an explicit host boundary, not a removed
requirement. `ConfigLoader` receives the source tree, absolute module path,
and base config. The JS host must actually load the module, select its
`default ?? config` export, and apply an extension or transform in its own
callback realm. Rust-native callers can use the marker-form JSON adapter.
The fixture loader is a typed Rust equivalent of the frozen module exports;
it does not execute JavaScript. The actual WASM module-loader connection is
pending integration.

Likewise Vite/Next hosts attach these Rust lifecycle cores to their real
framework hooks. Next preparation exposes a development watch directory; the
host installs its recursive, nonpersistent watcher and forwards filenames to
`watch_event` or `watch_event_async`. A Rust core callback is not evidence that
an actual Vite or Next application has been run against the Rust implementation.
The CLI core returns streams and exit status; an executable host decides where
to write them and how to exit. No process-global state or watcher is created by
the core. Gzip sizes are measured with flate2's pure Rust level-9 backend, and
the frozen CLI summaries match the upstream text. Compressed bytes are not
claimed identical to Node zlib for arbitrary inputs.

## Run

```sh
node tests/utility/cn/reference.mjs --check
node tests/utility/cn/build-contract.mjs --check
mbx test -p rustify-style --lib cn::
mbx clippy -p rustify-style --lib -- -D warnings
mbx check -p rustify-style --no-default-features --features cn --target wasm32-unknown-unknown
rustfmt --edition 2024 --check crates/rustify-style/src/cn/build.rs
```

The oracle requires the fixed upstream tree described in [README.md](README.md).
It validates every locked source before execution. To reproduce the additional
fixed expected data, use `node tests/utility/cn/build-contract.mjs --write`, then
review the fixture diff. This is development-only Node/TypeScript execution;
production runs Rust. The CLI oracle uses the locked CLI source, redirecting
only its built-module import to the locked source module and registering the
same development TS resolver used by the reference harness.

## Coverage and boundaries

The Rust runner in `src/cn/build/tests.rs` checks all 12 original
`build-fixtures.json` cases: complete config hashes and emitted source hashes,
subset/full/tokens/content/CSS/config/safelist/TS output/no-op/error contracts,
builder identity, matcher behavior, and the 8192 UTF-16 candidate cap.

`build-contract-fixtures.json` adds actual fixed-upstream results for 15 glob
selectors, 7 matcher selectors, 7 CSS import/reset/malformed-block cases,
10 additional build cases, 10 discovery predicates, 9 CLI invocations,
concurrent builder behavior, and Vite/Next lifecycle preparation. Additional
Rust tests check reentrant coalescing, failed-flight recovery, native permission
warnings and non-file entries, native/virtual equivalence, dangling output
symlinks, JSON configuration, async Next factories, and gzip decompression.

Coverage maps to the complete inspected API rather than treating the initial
12 cases as the full upstream contract. Further randomized differential tests,
platform-specific IO errors, Windows filesystem validation, live Vite/Next
host wiring, JS config module/callback realms, and actual WASM build execution
remain to be verified. No build performance or browser validation is recorded
by this runner.
