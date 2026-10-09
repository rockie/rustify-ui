# Actual JS runtime facade evidence

The actual-JS WASM facade passes 947 fixed runtime cases and 56 additional
scenarios. This is a focused M2 runtime/config/compiler-table gate; the gaps
below remain open. The historical reference inventory is unchanged.

Replay from the repository root:

```sh
mbx xtask utility-conformance --check --group cn-native
mbx test --locked -p rustify-style --no-default-features --features cn cn::
mbx clippy --locked -p rustify-style --no-default-features --features cn --all-targets -- -D warnings
mbx clippy --locked -p rustify-style-wasm --no-default-features --features cn --target wasm32-unknown-unknown -- -D warnings
```

The runtime-only phase passed 947/947 fixed cases, 56/56 added scenarios, 31 Rust
tests and both strict Clippy commands. Its historical 714,887-byte release facade
remains recorded in the linked module evidence. The subsequent
[production build/tools phase](BUILD-HOST.md) repeats all 947 + 56 runtime cases
on the larger cn bundle; its current size and SHA-256 are recorded separately.
Its memory was not shared and its imports contained no filesystem, WASI,
Rayon or thread entry. See [machine evidence](wasm-native-verification.json),
[WASM module evidence](evidence/wasm-native.json) and the linked command logs.

`CnSession` takes the original JS configuration and an explicit positive
integer limits object (`maxDepth`, `maxNodes`, `maxUtf16Units`, `maxTokens`,
`cacheBytes`). Its join/merge methods take an actual JS argument array.
No JSON serialization transports those arguments or callback-bearing config.
Strings, keys, literals, prefixes, modifiers, callback arguments and outputs
use UTF-16 code units. Native Rust APIs retain their existing UTF-8 signatures;
new UTF-16 types and entry points carry values that Rust `String` cannot.
Default typed tables are converted once per instance, without JSON fallback.

The extra tests cover prototype enumeration, getter/proxy throw identity,
mutable/sparse/array-like inputs, BigInt and nonfinite values, lone surrogates,
configuration extension and transformation, callable theme getters and
validators, lazy initialization, function identity, token memo collisions,
callback reentry, bounded recursive/cyclic inputs, compiled path/trie limits,
cumulative theme-getter output limits and the source's 4096 context/property
reset behavior. Original thrown JS values cross the boundary unchanged.
Reentry deliberately follows the supplied version's observable shared scratch
behavior, using short Rust critical sections that end before invoking JS.
The red logs under `evidence/` retain the failures preceding the fixes.

## Resource accounting

`cacheBytes` bounds retained Rust string content, not total process memory.
Token memo keys, canonical modifier contexts and arbitrary property keys,
including their duplicate retained copies, share half of that byte budget.
Whole-result cache entries share one quarter and argument tuple/bucket/churn
caches share one quarter. Accounting includes both generations' map keys and
values, argument tuple strings and results, bucket keys in each generation,
retained prediction entries and churn keys/base strings/results. Code-unit
lengths are multiplied by two and accumulated; counting only fixed slots
would not establish this bound. `cache_usage()` exposes output, memo and
argument text bytes and their sum. Explicit budget or node pressure may retire
entries earlier than upstream.

Token memo slots and doorkeeper metadata have fixed sizes. Whole-result generations are bounded by their admitted cache size; argument
prediction entries are capped by caller `maxNodes`; context/property tracking stops at the supplied
source thresholds. Input/config parsing accounts nodes and UTF-16 units,
including all theme getter results against shared counters. Compiler checks
cover definition depth/nodes, hyphen-path depth, radix recursion and actual
character trie node allocation. Working strings, sorting, model copies and
scratch allocations are bounded by products of these explicit limits; they
are not part of the retained cache byte total. The fixed shipped default
tables do not consume a caller's custom input node/depth budget. Foreign JS
object graphs, function captures and allocations inside user callbacks are
opaque to the Rust allocator and are not claimed to obey this byte budget.

## Compatibility closures and remaining work

The previous FIFO cache, eager configuration-read ordering and missing
numeric ID guard have been replaced. The result cache uses the source's two
generations and promotion/admission rules. The argument wrapper uses tuple
buckets, prediction links, generation rotation and the two-way churn table.
Objects and arrays resolve anew before a tuple can hit; their object identity
never substitutes for current values. Callback-state, callback-count,
mutable-getter and churn scenarios compare against the locked source.

JS definitions retain deferred property reads and actual iterators until the
compiler reaches them. Theme marker/getter lookup happens at expansion time.
Class-group entries snapshot at the initial trie stage; group arrays and
returned iterables advance as definitions are processed. Conflict fields and
methods are read after trie construction, both conflict maps execute before
their result iteration, and order modifiers/prefix are read at the final
metadata stage. These tested side effects, marker Proxy reads, thrown values
and UTF-16 diagnostic text agree with the reference. This is evidence for the
listed contracts and scenarios, not a formal equivalence claim for every
arbitrary JavaScript program. Explicit limits and validation still reject
cycles, reentrant configuration, unbounded inputs and malformed table data;
metadata map results must be arrays within the declared limits.

The original public ID threshold was exercised with 2,097,152 distinct
arbitrary properties, totaling 26,151,865 UTF-16 input units. Both implementations
throw `Error: cn: too many distinct classes in one merge`. The pre-fix WASM
returned normally; its failure log is retained. Replay this stress separately
from the ordinary fixture gate:

```sh
node tests/utility/cn/id-limit.mjs oracle
node tests/utility/cn/id-limit.mjs wasm
```

See [oracle evidence](evidence/id-limit-oracle.json) and
[WASM evidence](evidence/id-limit-wasm.json). No reduced threshold stands in
for this public test. The error is checked after tokenization before claiming,
matching the source's zero/single-token fast paths. Native `Cn` APIs return
`CnError`; the existing convenience function returning `String` preserves its
signature and panics with that message at the threshold.

Source emission/stats/subsetting and the build/tools/plugin bridge now have
[actual production Node/WASM evidence](BUILD-HOST.md), including the complete
public raw `compileModel` JS shape. Full framework projects and Windows host
behavior remain unverified. Neither this runtime phase nor the
build/tools phase reran browser or performance gates; those belong to the
root integration.

The adapted originals and final production hashes are listed in the machine
record. The supplied source archive has no Git object database; the commit
identity is supplied provenance while inspected bytes are locked by SHA-256.
The retained production MIT notice is
`crates/rustify-style/upstream/cn/LICENSE` (copyright 2026 shadcn).
