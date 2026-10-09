# Tailwind Oxide extraction closure

Fixed source: Tailwind CSS v4.1.13, commit
`1334c99db8fd26c8ea065375dd9259800863f072`.
The reference archive was downloaded from
https://codeload.github.com/tailwindlabs/tailwindcss/tar.gz/1334c99db8fd26c8ea065375dd9259800863f072 .

`oxide/` copies `crates/oxide/src/{extractor,cursor.rs,fast_skip.rs,fixtures}`
without source changes. `throughput.rs` is compiled only for upstream native
tests. `classification-macros/` copies the private host proc-macro crate, with its
manifest explicitly enabling `syn/full` (normally supplied transitively); it
builds byte classification tables and introduces no WASM runtime dependency.
`LICENSE-TAILWIND` preserves the upstream MIT notice.

The scanner facade supplies the upstream extension dispatch and line splitting
without the filesystem scanner. `bstr` and `regex` are the only external runtime
dependencies of the full preprocessing closure. Filesystem discovery, ignore,
globs, Rayon, tracing, environment variables and watches are excluded from this
closure. Their required compatibility work remains tracked as pending in
`tests/utility/scanner/inventory.json` and `README.md`.

The inventory records hashes of every vendored Oxide file, its public symbols,
and its preserved tests, plus the complete upstream scanner source/test files.
