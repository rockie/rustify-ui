# cn reference baseline

This development oracle supports [Rustify UI](../../../README.md)'s
[WASM Tailwind plan](../../../docs/plan/WASM-TAILWIND.md), M1 and R-1. It runs
the supplied cn source at commit `b7ec0fce43d824a60517fab508584eb860fdf717`
(package version `0.4.0`) and stores fixed outputs for the Rust/WASM port.
Passing this oracle does **not** establish Rust or WASM compatibility.
The separate [actual-JS runtime facade evidence](WASM-NATIVE.md) records
947 fixed WASM cases, 56 added scenarios, resource accounting and explicit
remaining gaps. The [production build/tools bridge](BUILD-HOST.md) separately
records 56 actual Node/WASM host scenarios and repeats that runtime gate against
the packaged cn artifact. These records do not change this frozen reference
inventory.

## Replay

Use Node 26; the recorded run used `v26.1.0`. No npm dependencies are needed.
From the repository root:

```sh
node tests/utility/cn/reference.mjs --check
```

The command verifies upstream file SHA-256 hashes, executes the fixed inputs,
checks emitted JS tables against in-memory tables, and compares every generated
artifact byte for byte. It reports 947 runtime cases, 5 compiler cases, 25
validator families, 3 cache sequences and 12 build cases. Temporary build
projects are confined to `.work/` and removed after execution.

`upstream.mjs` uses Node's native TypeScript stripping and a resolution hook to
load the original upstream TypeScript files. It does not install or replace a package,
and none of this code belongs in a production build. `package.json` and the
independent empty dependency lock provide equivalent `npm run check`,
`npm run generate` and `npm run bench` commands from this directory.

The upstream tree is investigation material under the repository's gitignored
`ref/cn/`. A fresh checkout therefore needs that fixed source before replay.
If `ref/cn/` is absent, run the following from the repository root:

```sh
(
  set -eu
  test ! -e ref/cn
  mkdir -p ref tests/utility/cn/.work
  cn_stage=$(mktemp -d tests/utility/cn/.work/download-XXXXXX)
  trap 'rm -rf "$cn_stage"' EXIT
  curl --fail --location \
    https://codeload.github.com/shadcn-ui/cn/tar.gz/b7ec0fce43d824a60517fab508584eb860fdf717 \
    --output "$cn_stage/source.tar.gz"
  mkdir "$cn_stage/tree"
  tar -xzf "$cn_stage/source.tar.gz" --strip-components=1 -C "$cn_stage/tree"
  mv "$cn_stage/tree" ref/cn
  node tests/utility/cn/reference.mjs --check
)
```

This retrieval recipe has not been executed by this fixture package; baseline
generation used the supplied downloaded tree. The subsequent check is the
authority for its file contents. The supplied tree has no Git object database;
`source-lock.json` records the commit as supplied provenance and locks the
actual inspected bytes. It does not claim independent Git commit verification.

## Artifacts and coverage

| File | Purpose |
| --- | --- |
| `source-lock.json` | Fixed upstream source, license and test-file hashes |
| `inventory.json` | All 9 package entries, runtime exports, public type declarations/reexports, config fields, 384 default groups, 19 theme scales, validators, table fields, build options, test files and behavior contracts |
| `default-config.json` | Complete upstream declarative configuration in definition order |
| `default-tables.json` | Actual shipped default table object, with typed arrays serialized as numeric arrays |
| `runtime-fixtures.json` | Upstream hand batteries and trio permutations, 512 deterministic pool samples, JS join shapes and custom config/callback cases |
| `compiler-fixtures.json` | Default/empty/literal/subset table outputs, source hashes, callback emission error, validator probes, cached/uncached sequences and mutable argument behavior |
| `build-fixtures.json` | Content/token/full builds, CSS theme/import/safelist/config inputs, TS output hash, unchanged rebuild, errors, builder edits, matcher and token cap |
| `benchmark-baseline.json` | Measured Node JS reference and replay protocol |

All inventory rows retain `in-scope-pending-rust-verification`. The 13 locked
upstream test files include generated cases, fuzz, timeout children, CLI,
plugins, logical utilities and theme discovery. The inventory records their
complete source hashes; section names and literal assertion labels are
navigation aids, not an assertion that those suites were executed here.
Fixtures are an initial deterministic corpus. The full upstream suite and
Rust/WASM differential checks remain required for V-2.

Runtime fixture arguments use a tagged wire format: `{ "$js": "undefined" }`,
tagged nonfinite numbers/negative zero, tagged BigInt, ignored functions, and
objects represented by ordered key/value entries. `encode`/`decode` preserve
these distinctions until the actual JS call; ordinary JSON serialization would
lose them. `args` is always a tagged representation of the argument array.
Configuration callback recipes live in `cases.mjs`; they define an identity
transform, theme getter, suffix predicate and throwing callbacks explicitly.
Out-of-type JavaScript join shapes are recorded alongside typed inputs to
support the later facade checks.

Build paths are normalized to the temporary fixture root. Token collections
are compared as sorted sets. Compiled table fields, configuration definition
order, merge output strings and generated source hashes remain exact. Build
configuration hashes refer to the `json()` serialization from `upstream.mjs`;
the retained group names and resolved theme scales also remain visible.

Regenerate only after reviewing changed fixture inputs or inventory code:

```sh
node tests/utility/cn/reference.mjs --write
node tests/utility/cn/reference.mjs --check
```

`--write` also verifies an existing source lock. It cannot silently accept
changed upstream source. Upstream upgrades require an explicit baseline and
lock review before generating new expected outputs.

## Performance replay

Run while compilation and other benchmark work are idle:

```sh
node tests/utility/cn/bench.mjs --samples 7 --iterations 100000
```

The default report is the ignored benchmark.local.json. Use `--out` to choose
another path. The checked baseline used those default sample/iteration values
and records CPU, OS, Node/V8, input hash, sample timings, warmup and process
memory snapshots. Each sample records nanoseconds per call; p50/p95 summarize
those sample averages. They are not single-call latency quantiles.

The measured families separate variadic cn, single-input cn, cached merge,
uncached merge, clsx joining and config creation plus first merge. The latter
includes lazy compilation with module imports already warm. Node execution is
the JS reference; Rust native/internal, Rust WASM/internal, JS/WASM crossing
and headed-browser comparisons remain pending. There is no numeric SLO.
Process memory snapshots do not establish a cache bound or leak result.

## Attribution

The upstream cn source is MIT licensed; its notice is preserved in
[LICENSE-CN](LICENSE-CN). Its README credits Dany Castillo's MIT-licensed
tailwind-merge default configuration as the source of its generated defaults,
and credits Luke Edwards's clsx semantics. The copied configuration/table data
retain those upstream source and credit references in the source lock and this
README. The benchmark numbers here are measured locally and do not reuse
upstream performance claims.
