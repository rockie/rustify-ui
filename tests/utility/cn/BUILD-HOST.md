# cn production build/tools bridge

The packaged cn WASM executes build, subset, emission, discovery and incremental
selection in Rust. The Node host supplies filesystem/path primitives, real module
import, native watchers and gzip size. The production bundle passes 56 host
scenario groups against the supplied cn `0.4.0` source at commit
`b7ec0fce43d824a60517fab508584eb860fdf717`, followed by 947 fixed and 56 added
runtime cases. The public API ledger includes the complete original raw `compileModel`
JS shape; remaining environment checks are explicit below.

## Replay

From the repository root with the configured mbx toolchain and Node 26:

```sh
mbx xtask utility-build --group cn
node tests/utility/cn/run-build-host.mjs
mbx test --locked -p rustify-style --no-default-features --features cn cn::
mbx clippy --locked -p rustify-style --no-default-features --features cn --all-targets -- -D warnings
mbx clippy --locked -p rustify-style-wasm --no-default-features --features cn --target wasm32-unknown-unknown -- -D warnings
```

The runner defaults to `target/utility/cn`; `RUSTIFY_CN_BUNDLE` selects a copied
bundle directory. It loads the packaged host and WASM, reads real temporary
projects and evaluates actual config modules. Its CLI comparisons execute both
the packaged executable and the original `ref/cn/packages/cn/bin/cn.mjs`; a test-only resolution
hook directs the original executable's missing distribution import to its
unchanged TypeScript source. No JS compiler or discovery algorithm replaces Rust.

The final cn-only release bundle contains `target/utility/cn/style.js`,
`target/utility/cn/style_bg.wasm`, `target/utility/cn/cn-node.mjs`,
`target/utility/cn/cn-cli.mjs`, the original MIT license and
`target/utility/cn/build-manifest.json`.
Its WASM is 2,137,348 bytes, with SHA-256
`cd39e9bf16760ee625e589c79aa6bd85966b035bbb21b1ecc550710a9359df0b`.
The glue is 43,725 bytes, with SHA-256
`23224e7550a864ec4a005ece0cd850122120e923e6a7686a5082ed662e1e9fb5`.
Memory is not shared, and the inspected module imports contain no filesystem,
WASI, Rayon or thread entry. See [machine evidence](build-host-verification.json),
[host result](build-host-green.log), [runtime regression](build-host-runtime-regression.log)
and the per-API [coverage ledger](build-coverage.json).

## Calling the bundle

```js
import { createCnBuild } from './target/utility/cn/cn-node.mjs'

const limits = {
  maxDepth: 128,
  maxNodes: 1_000_000,
  maxUtf16Units: 100_000_000,
  maxTokens: 1_000_000,
  cacheBytes: 16_000_000,
}
const bridge = await createCnBuild(limits)
try {
  const report = await bridge.session.build({ cwd: process.cwd() })
  console.log(report.outPath, report.changed)
} finally {
  bridge.close()
}
```

The caller supplies positive integer limits. The executable uses the same
values shown above as its caller policy; they are SDK resource bounds, not
upstream business thresholds. Set `RUSTIFY_CN_LIMITS` to a JSON limits object
to override that executable policy. JSON is used only for this numeric policy,
not for configurations, callbacks or emitted source.

```sh
node target/utility/cn/cn-cli.mjs --content 'src/**/*.tsx' --out cn-tables.mjs
```

`createCnBuild` reads WASM bytes explicitly from the sibling URL, then imports
the sibling glue. Its `close()` stops owned watchers and releases the session.
Keep the bridge alive while its builders, matchers or plugin functions are in use.
Vite's adapter is `bridge.session.vite(options)`; Next's config wrapper is
`bridge.session.with_cn(nextConfig, options)`.

## API mapping

The glue exposes explicit snake-case exports and methods; it does not reproduce
the original package's import paths. Helper functions are exported by `target/utility/cn/style.js`.

| Original API | Production WASM entry | Verified behavior |
| --- | --- | --- |
| `build` | `CnBuildSession.build(options)` | Real FS, config import, CSS/theme, emitted source, unchanged output |
| `createBuilder` | `session.create_builder(options)` | `run()`, `changed(file)`, `last`; same active Promise, queued follow-up and report identity |
| `expandGlobs` | `session.expand_globs(patterns, cwd)` | Files, unreadable count, ignored paths, explicit opt-in and symlinks |
| `createContentMatcher` | `session.create_content_matcher(patterns, cwd)` | Returned `CnContentMatcher.matches(file)`; UTF-16 glob `?` semantics |
| `extractTokens` | `cn_extract_tokens(content, set, limits)` | Original Set callback order, duplicates and thrown-value identity |
| `themeFromCss` | `session.theme_from_css(entries)` | Actual reads/imports, resets and extension sections |
| `isTailwindCss` | `cn_is_tailwind_css(content)` | Original detection |
| `subsetConfig` | `cn_subset_config(config, tokens, limits)` | Lossless configuration and retained callback handles |
| `compileToSource` | `cn_compile_to_source(config, options, limits)` | Exact JS/TS source, UTF-16 literals/banner and original callback rejection |
| `compileStats` | `cn_compile_stats(config, limits)` | Original six model counters |
| `compileToTables` | `cn_compile_to_tables(config, limits)` | Original typed arrays and callable validator implementations |
| `vite.cn` | `session.vite(options)` | Actual hooks, cwd overrides, delete/update behavior and source differential |
| `next.withCn` | `session.with_cn(nextConfig, options)` | Actual factory/Promise/context, shared default object and recursive nonpersistent watch |
| CLI | `session.parse_cli/run_cli/cli_summary` | Original arguments, streams, exit code and summary; executable Node entry |
| `compileModel` | `cn_compile_model(config, limits)` | Complete raw model fields/types, pre-renumber literal IDs, callback and shared array identity |

The report preserves `outPath`, `source`, `changed`, `tokens` as a JS Set,
`config`, `fullConfig`, `scannedFiles`, `cssFiles`, `usedGroups`, `totalGroups`
and `warnings`. Config functions and callback handles remain in their original
realm. A config module uses the original `default ?? config` selection;
transforms receive the actual base config, and original thrown values or causes
retain identity. Native Rust's existing UTF-8 configuration API is preserved.

Runtime helpers (`cn_default_config`, `cn_from_theme`, validators,
`cn_merge_configs`, table compilation and `CnSession.fromTables`) retain their
[separate runtime contract](WASM-NATIVE.md), replayed on this production bundle.
The replay used a temporary working directory whose
`target/utility-conformance/cn` symlink targeted the packaged cn directory;
the frozen runtime runner and fixtures were unchanged.

## Boundaries and remaining verification

Node's actual UTF-8 file reads and path encoding define the physical source
boundary. JS config keys, prefixes, literals, callbacks, tables and source remain
UTF-16, including lone surrogates. They do not pass through native `Config` or
JSON. Rust performs discovery and incremental selection for Native, Virtual and
concrete Host source trees. A separate primitive virtual-tree fixture compares
actual WASM build and incremental reports with the original native oracle.

Each build flight limits admitted IO operations, input nodes, code units,
candidates and recursive depth. Compiler and retained cache limits follow the
[runtime accounting](WASM-NATIVE.md). Temporary work may occupy products of
these bounds. Arbitrary allocations inside module evaluation, JS callbacks,
filesystem primitives or native gzip remain external host allocations.

The native Rust gzip helper uses miniz; its DEFLATE output is not claimed equal
to Node zlib. A genuine differential produced 5.9 KB versus the original 5.8 KB
gzip summary for the 10,968-unit source in [the boundary fixture](gzip-boundary.json).
Production now asks Node `gzipSync(source, { level: 9 })` for compressed size;
Rust still owns all summary calculations/formatting and CLI business rules.
The rounding, gzip, default-option, glob, theme, Next identity and thrown-value
RED logs are enumerated in the machine record.

Full Vite HMR and Next framework projects were not launched; actual returned
hooks/factories and native watcher lifecycle were exercised. Windows host paths
and platform-specific IO errors, randomized build corpus, browser and performance
gates were not verified in this phase. Named option getters now retain the
original repeated reads, branch short circuits and IO/await stages. The
[options contract and actual traces](BUILD-OPTIONS.md) record this closure.
One initial native watcher timeout was not reproduced by an isolated event probe
or two later full runs on the same bundle; its cause remains unproven and is
recorded in the machine evidence.

The original source paths and individual hashes, changed production source
hashes, commands and logs are recorded in [machine evidence](build-host-verification.json).
The supplied archive has no Git object database; the commit is supplied
provenance and inspected bytes are locked by SHA-256. The packaged MIT notice
retains copyright 2026 shadcn. Historical runtime-only 714,887-byte evidence
remains in [WASM-NATIVE.md](WASM-NATIVE.md).
