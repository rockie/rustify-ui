# Runtime utilities

`UtilityScope` generates CSS for complete class names that were absent from the
static build. Enable the SDK's `utility` feature and create one scope from its
mounted `ScopeRoots`. Parsing, scanning and compilation run in a separate Rust
WASM Worker. The existing offline Tailwind CLI and static stylesheet remain in
place. Components merge classes with the Rust `cn` (`rustify_style::cn`), with or
without a scope. The design and its acceptance record are in
[WASM-TAILWIND](plan/WASM-TAILWIND.md).

## Build and try the source panel

The default build contains a compiler-only Worker. Complete class names work
without the scanner:

```sh
mbx xtask build-web --example utility-lab --release
mbx xtask serve --example utility-lab --release --port 4186
```

To include source extraction in the Worker and enable the HTML source panel:

```sh
mbx xtask build-web --example utility-lab --release --utility-scanner
mbx xtask serve --example utility-lab --release --port 4186
```

Open `http://127.0.0.1:4186/`. Enter complete classes and apply them to the DOM
and GPU samples. The optional source panel installs rules extracted from HTML;
apply the intended preview classes in the main input afterward. A build with
`--base /tools/utility-lab/` also produces the corresponding subpath deployment.
Serve that product with the same `--base`. `serve` does not rebuild source.

`--utility-scanner` changes the independent Worker features, not the application's
WASM dependencies. The Worker build manifest records `scanner_enabled` and asset
digests. The flag requires an application built with the SDK's `utility` feature.
See [xtask/src/utility.rs](../xtask/src/utility.rs).

Dynamic builds also produce `utility/static-candidates.json`. The Worker manifest
names this seed and includes its digest in the build identity. The build extracts
class selectors from the generated static utilities layer (or unlayered utilities)
without scanning application source. Initialization verifies the seed and merges
it with the scope's classes before compilation. Replacement and recovery reuse
the same seed, so an absent DOM class can still participate in the original utility
order. Seed transfer, candidate count and retained storage consume the explicit
queue/candidate/cache budgets; a digest mismatch rejects initialization.

## SDK calls

The browser-only API is exported from `rustify_ui::utility`. Its signatures are
defined in [browser.rs](../crates/rustify-ui/src/utility/browser.rs).

| Call | Input and completion |
| --- | --- |
| `UtilityScope::new(&roots, config)` | Owns the root, Worker and stylesheet; validates positive limits and the input/asset byte budgets. |
| `UtilityScope::new_with_cn(&roots, config, cn)` | Keeps component callbacks in the main realm; provide `cn_context()` before constructing component children. |
| `ensure_classes(&[String])` | Complete class names and explicit `--variable` dependencies. Does not run the extractor. Already installed inputs return the current snapshot. |
| `scan_content(&[SourceContent])` | Raw text with an extension hint, such as `html`, `rs` or `json`. Ignores CSS path discovery and uses the fixed preprocessor/extractor. |
| `scan_sources(&SourceSnapshot)` | Complete virtual directory snapshot selected by CSS source metadata, ignore files and exclusions. |
| `flush()` | FIFO barrier after previously queued operations. Returns the installed snapshot without recompiling it. |
| `replace_config(config, cn)` | Rebuilds from the static seed, retained candidates, current DOM classes and premerged component raw fragments; publishes CSS, GPU programs and cn together after installation. Supersedes prior queued/in-flight work. |
| `rebuild(&[String])` | Recompiles the installed configuration from the static seed, current DOM classes, premerged component raw fragments and the given classes only, releasing every other retained candidate. Pass what the DOM does not show: classes used by GPU regions and scanned `--variable` dependencies. Supersedes prior queued/in-flight work; a failure keeps the installed sheet, GPU programs, cn and candidates. |
| `configuration()` | Last installed compiler configuration; retained even when its acknowledgement is lost. |
| `cn_context()` | Main-realm component engine and committed configuration epoch. |
| `snapshot()` | Last successfully installed `Arc<CommittedSnapshot>`, or `None` before installation. |
| `last_error()` | Last observed compilation/runtime error. |
| `retry()` | Starts a fresh Worker epoch and replays the installed candidate and variable set. Retains the current sheet during recovery. Rejects an in-flight preparation. |
| `dispose()` | Releases the Worker, observer, owner attribute and stylesheet, and cancels queued completions. |

`UtilityConfig` contains `input`, `assets`, `limits` and `request_timeout_ms`.
`CompilerInput` currently contains CSS text and its `entry` path. Supply all seven
positive `Limits`: `input_bytes`, `asset_bytes`, `candidate_count`, `ast_depth`,
`css_bytes`, `queued_bytes` and `cache_bytes`. JSON encoding also counts toward
the queue limit. The SDK estimates owned report/candidate storage and includes
queued and in-flight inputs in its cache budget. These estimates are not browser
allocator measurements. See [protocol.rs](../crates/rustify-style/src/protocol.rs#L112)
and [state.rs](../crates/rustify-ui/src/utility/state.rs).

Async compilation methods return `Outcome`, a
`Result<Arc<CommittedSnapshot>, UtilityError>`. A completion succeeds after sheet
installation, snapshot publication and Worker acknowledgement. If acknowledgement
is lost, the call fails while the installed snapshot remains the recovery source.
Prepare/install failures retain the previous snapshot. Adjacent class requests
can coalesce; source requests retain FIFO order.

Components compiled with `rustify-components/utility` retain their base, variant,
size and caller fragments. Configuration replacement premerges these fragments
before Worker compilation, then publishes the prepared results with the installed
sheet. Callback failures, invalid CSS and installation failures keep the previous
component classes and compiler configuration. If registered fragments change
while replacement is pending, it fails with `Superseded`; retry the replacement
using the current props. Same-epoch class additions retain their own completion.
After ack loss, explicit `retry()` uses the installed configuration and candidates.

## Source inputs

`SourceContent` contains UTF-8 `content` and an `extension` without a leading dot.
Scanning does not evaluate Rust or JavaScript string construction. Pass the
resolved complete class name to `ensure_classes` when its value becomes known.

`SourceSnapshot` contains `root`, `base` and `entries`. Each entry has a `kind` and
`path`: `directory`, `file`, `symlink` or `unavailable`. Files also contain
`content` and integer `revision`; symlinks contain `target`. Supply relevant
ignore files and repository markers. Missing information marked `unavailable`
returns a scan error. Paths are virtual; the Worker does not access the filesystem
or fetch source files. The types are exported by
[utility/mod.rs](../crates/rustify-ui/src/utility/mod.rs).

Each snapshot is rescanned and released after extraction. Compiled candidates
accumulate, so deleting a source file does not remove its installed CSS until
`rebuild` is called with the classes still in use. When `candidate_count` or
`cache_bytes` rejects new classes, `rebuild` with the active set frees the quota
without raising the limits. Successful
snapshots retain extracted candidates and variable dependencies for recovery;
the SDK does not keep the original source text. A compiler-only Worker returns
`CompileError::UnsupportedFeature` for both scanning calls and remains usable
for direct class requests.

## DOM and GPU results

The installed report contains CSS, admitted/rejected/unsupported candidates,
metrics, renderer programs and variable bindings. CSS and bindings use the
lossless `rustify_style::text::Text` type. `to_utf8()` is fallible; use `units()`
when inspecting ECMAScript code units. The Worker/SDK JSON path preserves lone
surrogates through stylesheet installation. Complete non-scalar compiler input
compatibility remains pending.

GPU programs are an explicit supported projection. Pass relevant programs and
the current theme/control context to `resolve_gpu`; use strict or explicit partial
application and wait for the widget's drawn revision when validating the output.
Refer to the [resolver contract](../crates/rustify-ui/src/utility/gpu/README.md)
and [widget integration](../crates/rustify-ui/src/gpu/utility/README.md).

## Compatibility, limits and performance

| Area | Contract | Check |
| --- | --- | --- |
| cn | Fixed shadcn-ui/cn `b7ec0fc`: runtime, configuration, validators and callbacks, table compilation and build. The JS facade keeps configuration callbacks in the caller's realm. | `node tests/utility/reference.mjs --check cn` (947 fixed cases plus 56 shape/callback cases on real WASM); `mbx xtask utility-baseline --group cn` runs the 947 cases in a strict-CSP page |
| Scanner | Fixed Oxide extractor, preprocessors and source selection on real or virtual directories | `node tests/utility/reference.mjs --check scanner` |
| Compiler | Tailwind CSS 4.1.13: the upstream test files run against the Rust compiler, each test with a recorded disposition | `mbx xtask utility-conformance --check` (needs the local `ref/tailwindcss` tree and `npm ci --prefix tests/utility/compiler/upstream-exec`) |
| GPU | The finite projection in the [resolver contract](../crates/rustify-ui/src/utility/gpu/README.md); everything else is a visible diagnostic | `utility-gpu.spec.ts` |

Keyframes are global to the document, so each scope renames its keyframes to
`rustify-<owner>-<name>` and rewrites every reference that resolves to a
keyframes name, including the value of a theme token such as `--animate-card:
card 1s linear infinite`. An animation that reads the token through `var()` in
the scope, inline styles included, therefore finds the scope's keyframes, and
setting the token on an element (for example to `none`) still takes effect. Reading
that token as a plain value returns the renamed name. A public custom property
whose uses need different names, one use as a keyframes name and another as a
timing function or plain value, fails with `UnsupportedFeature` and keeps the
installed revision. Tailwind's internal `--tw-*` variables get owner-private
copies instead.

Not supported: JavaScript `@plugin` and `@config` fail with `UnsupportedFeature`
after upstream's own syntax checks; remote `@import` fails with
`UnsupportedFeature` and is never fetched; arbitrary host objects such as Proxies
as `compileAst` input are not claimed equivalent. GPU text wraps at the content
box as CSS does, but a single word wider than the box breaks between graphemes
instead of overflowing. Pixel comparisons between the DOM and GPU previews assume
the canvas sits on whole CSS pixels.

`mbx xtask utility-bench` measures the Rust cn and compiler natively and through
their JS facades beside the fixed JS references, on identical workloads;
`tests/utility/bench-utility-lab.mjs` and `tests/utility/bench-browser.mjs` time
the runtime in headed Chrome. No threshold is applied. On the reference machine,
a new class reaches an installed stylesheet in about 2.5 ms and a repeated class
returns in about 0.2 ms; a cold compiler session is far faster than the JS
reference, while cached cn merges are slower than upstream's string cache. The
recorded numbers and their interpretation are in
[the M7 record](validation/wasm-tailwind/M7.md).
