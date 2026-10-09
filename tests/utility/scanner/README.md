# Fixed scanner contract and compatibility ledger

`rustify_style::scanner::scan(content, extension)` returns unique, sorted
`ScanResult { candidates, variables }`. The hint is an upstream extension without
a leading dot. Every raw-memory input, including CSS text, uses preprocessing
and full line-by-line extraction. Source programs are not evaluated. CSS files
discovered by `Scanner::scan` use only variables referenced through `var(...)`.

The stateful API is `Scanner::new(public_sources, SourceTree)` followed by
`scan`, `scan_content`, `get_files`, `get_globs`, `get_normalized_sources` and
`get_candidates_with_positions`. `scan_sources(tree, public_sources)` is the
one-shot entry. `SourceScanResult` also carries files, globs and normalized
sources. The upstream combined candidate list is the sorted union of this API's
separate candidate and variable fields. `scan_content` returns only newly seen
names; full `scan` returns cumulative names, including after a file is deleted.

`SourceTree` has concrete Virtual and Native variants, with shared selection
and preprocessing. `NativeSourceTree` requires `scanner-fs` on non-WASM targets.
Even enabling that feature on WASM does not compile its filesystem adapter.
Virtual trees use POSIX paths, explicit file revisions, directory entries and
symlinks. Supply a complete snapshot including relevant ancestor ignore files,
repository markers and external sources; omitted paths in a complete snapshot
mean absent paths. Use `mark_unavailable` to represent missing source information
and receive `MissingSourceInformation`, rather than silently omitting it.
Mutate virtual inputs via `scanner.tree_mut().virtual_mut()` and submit changes.
Invalid UTF-8 file bytes return an explicit diagnostic, including on repeated
scans, rather than using upstream's unchecked conversion or Vue panic.

## Fixed baseline

Tailwind CSS v4.1.13, commit `1334c99db8fd26c8ea065375dd9259800863f072`, was
fetched using the immutable codeload archive URL in `inventory.json`. There is
no independent Git checkout inside the reference tree. The ledger records
original file hashes, preserved extractor files, adapted source APIs and the
ignore fork source/license. The independent native oracle has its own Cargo
workspace and lockfile; production has no dependency on `ref/`.

`fixtures.json` has 24 actual upstream extractor/preprocessor cases, including
all ten named preprocessors plus default dispatch. `source-fixtures.json` has
12 actual upstream Scanner scenarios and 51 operations covering source rules,
ignore precedence/repository boundaries, CSS files versus raw CSS, explicit
memory changes, deletion/no-op, symlinks, watch globs and byte positions.
`reference.py` executes the fixed upstream for both corpora and checks their
expected outputs and immutable reference/vendored hashes.

All 42 upstream native Scanner tests retain their original expected outputs in
`upstream_native_tests.rs`. The API shim runs each operation against both a real
temporary directory and its virtual snapshot, asserting equal results. Temporary
fixtures retain their `TempDir` guards so they are removed when each test ends.
The pure closure also retains the upstream extractor/preprocessor tests.

| Contract | Status | Evidence |
| --- | --- | --- |
| Complete extractor and extension/preprocessor dispatch | Verified | Original tests; 24-case fixed memory corpus |
| Public source fields and all Auto/Pattern/Ignored/External modes | Verified | 42 upstream source tests and native/virtual parity |
| Brace expansion, static hoisting, canonicalization and absolute paths | Verified | Fixed upstream source expectations |
| Default ignored directories/extensions/files/binary files | Verified | Original lists and upstream tests; file-only binary rules |
| Explicit/negated source precedence and gitignore overrides | Verified | Shared selection; upstream source tests |
| Parent/nested `.ignore`, `.gitignore`, repo boundaries and git excludes | Verified | Source corpus and upstream repo-boundary scenarios |
| External/node_modules sources and symlinks/cycles | Verified | Upstream native tests with virtual snapshots |
| Files, optimized/cached globs and normalized sources | Verified | Original watch-glob expectations and JSON operations |
| Incremental/repeated/changed/new/deleted content and files | Verified | Cumulative names, new-name deltas, revision/membership rules |
| Candidate byte positions and legacy `-[]` migration | Verified | Fixed upstream position output, including Unicode |
| Missing virtual source information and invalid UTF-8 | Verified | Explicit diagnostic unit tests |
| Actual shared source corpus in single-threaded WASM | Verified | Plain WASM execution, zero imports, no shared memory |
| JS facade / Worker / source panel integration | Main-task integration | Source core ready; not claimed by this standalone probe |

`inventory.json` records intentional API adaptations: explicit tree and Result
boundaries replace implicit OS access; the unused upstream ScanOptions carrier
and paths::Path wrapper are represented by the stable source API rather than
being copied as additional public wrappers. The documented native verification
runs on macOS; other native operating systems remain unverified.

## Reproduction

```sh
python3 tests/utility/scanner/reference.py
mbx test --locked -p rustify-style --no-default-features --features scanner-fs --lib
mbx clippy --locked -p rustify-style --no-default-features --features scanner-fs --all-targets -- -D warnings
mbx check --locked -p rustify-style --no-default-features --features scanner-fs --target wasm32-unknown-unknown
mbx build --locked -p rustify-style --no-default-features --features scanner --example scanner-probe --release --target wasm32-unknown-unknown
node tests/utility/scanner/run-wasm.mjs
mbx build --locked -p rustify-style --no-default-features --features scanner --example scanner-source-probe --release --target wasm32-unknown-unknown
node tests/utility/scanner/run-source-wasm.mjs
```

`verification.json` records the earlier memory risk gate; `source-verification.json`
records the completed source-core checks. Probe byte sizes include test-only
serde/JSON corpora and are not production WASM package measurements.
