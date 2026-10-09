# Tailwind compiler reference

Development fixtures for the [Rustify UI](../../../README.md) utility compiler. The oracle is Tailwind CSS 4.1.13 at commit `1334c99db8fd26c8ea065375dd9259800863f072`; its npm dependencies have a separate lockfile and are excluded from production builds.

Run from the repository root with Node 26:

```sh
npm ci --ignore-scripts --prefix tests/utility/compiler/oracle
node tests/utility/compiler/reference.mjs --check
node tests/utility/compiler/inventory.mjs --check-registry
```

`reference.mjs` checks two frozen official compiler cases, invalid candidate rejection, repeated candidate calls and candidate order. It removes only Tailwind's generated license banner. Declaration and rule order, duplicates, custom properties and formatting remain exact. It does not run a Rust compiler or claim Rust conformance.

`compiler-smoke.css` compiles `w-[137px]` and `bg-[#123456]` without a scanner or theme input. `css-scope-property-nesting.css` additionally uses `ring-[3px]` and `before:content-["probe"]` inside `@scope ([data-utility-probe])`. The official output includes global `@property` registrations and dependency fallbacks. [fixtures.json](fixtures.json) records inputs, CSS digests and expected browser values. Both exact CSS outputs have native Rust regressions and real WASM assertions in [run-wasm.mjs](run-wasm.mjs). Browser evidence belongs in milestone validation records.

[inventory.json](inventory.json) enumerates 109 upstream source/test/snapshot files, 889 static utility roots, 282 functional utility roots, 88 variant roots and 1,125 syntactic test calls. Every test or parameterization has a unique source ID, location and digest. Runtime registry enumeration includes computed registrations and theme-derived breakpoints; registration sites identify the corresponding source expressions. Execution and fixture conversion remain pending; the implemented slice below is verified separately.

[upstream-test-ledger.mjs](upstream-test-ledger.mjs) checks all 109 source hashes
and reconstructs [upstream-test-ledger.json](upstream-test-ledger.json). It retains
1,215 original calls, including 1,125 tests, and statically expands 1,398 test
registrations. It records 1,851 assertion templates, 2,140 registration-to-template
edges, 45 controlled assertion templates, one upstream skip and 13 external
snapshot exports. Templates and edges do not count runtime assertion executions.
All validation, fixture mappings, scope decisions and skip disposition remain
pending. Three parser classification suggestions preserve the original inventory.
The `compiler-api` gate checks deterministic reconstruction and ten ledger tests;
it does not execute these upstream tests.

```sh
node tests/utility/compiler/upstream-test-ledger.mjs --check
node tests/utility/compiler/upstream-test-ledger.test.mjs
```

[upstream-exec](upstream-exec/) executes those upstream test files with vitest
2.0.5, the version upstream locks, from the fixed `ref/tailwindcss` tree. With the
package entry (`compile`, `compileAst`, `__unstable__loadDesignSystem`) and the
internal modules a test imports (CSS and value parsers, segment, arbitrary value
decoding, brace expansion, compare, escape, shadow colors, line tables, AST
printing and optimization, translation maps, key paths, candidate and variant
parsing against registries a test builds) resolved to the Rust/WASM facade, each
test runs its original assertions. The official TypeScript run is the control.
A test counts as Rust evidence only when it reached the facade and called no
other upstream module; harness code (`test-utils`, `DefaultMap`) is exempt.
[upstream-test-execution.json](upstream-test-execution.json) records every
expanded test's disposition: 1,183 pass through Rust, none fail; 173 need a
JavaScript `@plugin`/`@config` module and fail only with the required
`UnsupportedFeature`; 33 test helpers that only serve JavaScript plugins and
configs; 7 test upstream's JavaScript `walk` traversal API; one builds a design
system without compatibility hooks and is checked exactly by
`internal-entries.mjs`; one is skipped upstream.

```sh
npm ci --prefix tests/utility/compiler/upstream-exec
mbx xtask utility-conformance --check --group compiler-upstream
```

`--group compiler-full` runs `compiler-api`, `compiler-slice` and
`compiler-upstream` in turn.

[run-public-ast-registration-wasm.mjs](run-public-ast-registration-wasm.mjs) runs
in `compiler-api` to check initial directive order and completed mutation prefixes.
It compares 48 scenarios through the synchronous constructor and actual Promise
entry against the fixed oracle, retaining caller objects, arrays, Source metadata,
setter traces and rejected/settled graphs. All 96 comparisons currently pass.
[run-public-ast-media-source-wasm.mjs](run-public-ast-media-source-wasm.mjs) runs
in the same gate and passes four additional comparisons for media source traversal.
These finite checks do not establish complete registration conformance.
The optional first argument is a
compiler-api module directory; without it the existing conformance target is used.
[Design and isolated cursor evidence](../../../docs/validation/wasm-tailwind/registration-design/README.md)
preserve the earlier investigation; production registration now uses the shared
raw graph walk. Import-time lowering, ownership admission and host projection
still require further checks.

[run-public-ast-ownership-wasm.mjs](run-public-ast-ownership-wasm.mjs) runs in
`compiler-api` and passes fourteen scenarios through both entries (28 comparisons).
It checks successful variant assignments before later errors, nested applications,
repeated receiver aliases, live custom templates, setter order and retained Source
and extra fields. Slot/resource checks preserve original arrays, caller property
and keyframe hoisting, and fresh template metadata. Shared declarations, raw theme
publication and completed theme/keyframe function effects remain covered. All
checks compare complete retained graphs and build outputs, including settled state.
Authored function substitution now uses that same raw Current before lowering,
including published theme declarations, attached utility bodies and the keyframe
tail. Functional utility completion metadata is captured before substitution.
The late setup function Edits owner and its separate failure journal are removed.
[run-public-ast-function-order-wasm.mjs](run-public-ast-function-order-wasm.mjs)
also runs in the default gate: nineteen scenarios through both entries compare
complete prefix, build and settled graphs, including shared values and original
setter exceptions. These finite witnesses do not establish complete ownership
conformance; import-time lowering, native adaptation, the remaining apply handoff,
host projection and unified peaks remain pending.

The ledger includes package exports, compiler options, CSS directives, functions, source maps, ordering and theme/dependency behavior. Colors, default-theme data/resolvers and color-palette flattening are in scope as Rust equivalents. User JavaScript plugin/config execution is explicitly excluded and requires an `UnsupportedFeature` diagnostic. Mixed tests retain their CSS obligations; the exclusion does not remove built-in legacy utilities or selectors.

[upstream-provenance.json](upstream-provenance.json) records comparison of every inventoried source and CSS asset against the fixed commit archive. [LICENSE.tailwindcss](LICENSE.tailwindcss) preserves the upstream MIT notice. Local `ref/` is needed only to regenerate or compare the source inventory:

```sh
node tests/utility/compiler/inventory.mjs --source ref/tailwindcss --check
node tests/utility/compiler/reference.mjs --write
```

Running `inventory.mjs` without `--check` regenerates the pending inventory. Review the changes when intentionally updating compatibility or fixture status; regeneration is not a conformance gate.

M4 expansion fixtures are separate from the complete pending ledger. `static.mjs`
checks 889 official static roots in base, important, and hover forms, six theme
configuration probes, their full sorted union, and the generated structural AST
registry: 2,674 comparisons. `variants.mjs` checks all 66 nonresponsive static
variant roots with width and important content candidates, six nested variants,
and a sorted union: 139 comparisons. `effects.mjs` checks 72
ring/shadow/content/pseudo/radius cases, including 15 rounded roots with
arbitrary, default-theme, and named-theme values, and a union. The native Rust
tests compare these exact outputs. These fixture scripts execute only the
official development oracle.

`functional-spacing.mjs` checks 83 positive/negative spacing, sizing, inset,
margin, padding, scroll and gap registrations against 5,999 official parameter
cases. `dynamic-variants.mjs` checks 517 compound, attribute, positional,
supports, responsive and container cases, including all 66 static variants as
compound children, arbitrary modifiers, variable shorthand, and sorted unions.
`colors.mjs` checks 19 color branches with 1,808 positive CSS cases and 424
invalid cases. All 97 other interpretations now pass exact CSS comparisons through typography,
border-width and background handlers.
`typography.mjs` checks 1,452 CSS cases, including the text-shadow namespace collision.
`borders.mjs` checks 5,889 width/style/decoration/opacity cases over 19 roots.
`backgrounds.mjs` checks 1,370 image/position/size cases, including 54 prefix inputs.
`gradients.mjs` checks 3,594 linear/conic/radial and color/position-stop cases
over eight roots, including 96 prefix inputs.
`transforms.mjs` checks 13,334 cases over 34 functional and 50 static regression
roots, including 408 prefix inputs. `theme.mjs` checks 43 variable
dependency, keyframe, cycle, reset and full default-theme input cases.
`filters.mjs` checks 11,884 cases over 22 functional and three static roots,
including 264 prefix inputs, including nested relative-color fallbacks.
`shadows.mjs` checks 15,991 text/box/inset-shadow and ring cases over six
functional and four static roots, including 72 prefix inputs.
`prefix.mjs` checks 38 CSS cases and four exact invalid-prefix error messages,
including last-option precedence, variable/keyframe dependencies and escaping.
Frozen fixtures retain their historical pending labels; native and WASM runners
now compare those prefix cases against their original CSS.
`motion.mjs` checks 5,003 animation and transition/delay/duration/ease cases
over five functional and twelve static roots, including theme collisions,
prefix and live keyframes. Resources retain official output order.
`grid.mjs` checks 16,772 placement/span/tracks/flow cases over eighteen functional
and twenty-five static regression roots, including prefix inputs.
`imports.mjs` checks 129 virtual import/reference CSS outputs and fourteen
constructor errors with relative origins, layers, supports, media, theme,
prefix, important and source options, registry traversal and reference output
suppression.
`layout.mjs` checks 8,911 flex/order/columns/object-position cases over nine
functional and thirty-four static regression roots, plus existing basis behavior.
`effects-complete.mjs` checks 17,039 rounded/content cases over sixteen functional
and thirty-one static regression roots, including empty/reference theme values.
`arbitrary-properties.mjs` checks 21,112 arbitrary-property cases, including opacity
modifiers for color and other properties, malformed value functions and sorted unions.
`masks.mjs` checks 23,350 CSS cases over 27 functional and 56 static regression roots.
`miscellaneous.mjs` checks 10,835 cases over 14 functional and 69 static regression roots.
`structural-spacing.mjs` checks 6,102 cases over seven functional and nine static roots,
including nested sibling spacing, reverse declarations and virtual sort metadata.
`functions.mjs` checks 266 CSS outputs and eleven exact error messages for
`--alpha`, `--spacing`, `--theme` and `theme` across authored values, candidates,
theme options, at-rules, keyframes, nested selectors and property fallbacks,
including unchanged-input behavior when no compiler feature is used.
`legacy-theme.mjs` checks 785 CSS outputs and twenty-four exact error messages
across 119 built-in namespaces and fifty dynamic namespace dependencies.
`custom-utilities.mjs` checks 19,524 CSS outputs and forty-one configuration
errors for CSS-native static and functional utilities, value and modifier
substitution, registry collisions, resources, nesting and candidate boundaries.
Its 1,304 warning cases retain official metadata; runtime warning API parity
remains pending. The WASM runner also checks template cache limits and failed
expansion recovery without publishing a partial stylesheet.
`custom-variants.mjs` checks 5,849 CSS cases and 933 exact constructor errors;
12 incremental cases execute 72 additive builds. It covers registration,
compound variants, authored applications, slot object identities and resource
ordering, including used and unused custom utilities. The WASM runner checks
cache, CSS and depth failures and subsequent commit recovery.
`parser-errors.mjs` checks 2,011 CSS outputs and 183 exact constructor errors
for EOF, comments, strings, custom properties, whitespace and arbitrary candidate
syntax; fifteen existing cases are referenced by fixture digest.
`variant-parameters.mjs` checks 6,673 official parser JSON shapes (2,963 null
results), 6,719 CSS outputs and twenty-four constructor errors across twelve
theme registries. Eighty-four existing cases are referenced by input, case and
official-result digests. The WASM runner executes these referenced cases and
compares parser shapes through each session's production `parse_variant` API,
including static, functional, compound and arbitrary variants, modifiers,
theme collisions, prefix inputs, nesting and ordering unions.
The parser's byte and depth budgets are checked in all twelve modes; failures
and parsing leave the committed report unchanged.
`source-directives.mjs` checks 1,607 CSS cases and 303 exact syntax errors,
including the fixed upstream brace-expansion test inputs. Fifteen incremental
cases run ninety additive builds, yielding 1,682 CSS comparisons. Source
exclusions apply to both inline and explicitly submitted candidates.
All 1,607 successful inputs also compare the official `sources` and `root`
metadata through `CompilerSession.source_metadata`, before and after candidate
commits. Coverage includes path exclusions, quoted root options, nested virtual
imports, reference files and media sourceBase propagation. The six formerly
pending metadata cases now use their unchanged official CSS and metadata.
These fixtures use 8,192 candidate slots, 2 MiB CSS and queue budgets, and an
8 MiB cache budget. Metadata storage is charged to the cache budget. Connecting
this discovery information to the SDK source panel remains separate work.
`source-pipeline.mjs` combines the fixed native Oxide Scanner with the official
compiler across eight source configurations and 56 operations: automatic,
disabled and explicit roots, import/reference bases, path/inline exclusions,
ignore files, CSS variable extraction, raw changes and deleted/modified files.
The native Rust test compares candidates plus variables and the resulting CSS;
the scanner-enabled real WASM runner compares 40 scan/build results and checks
repeat/discard/retry behavior, missing information and quotas.
`external-variables.mjs` checks 213 cases and 1,272 incremental official builds,
including prefixes, escaped names, unknown variables, theme modes, keyframes,
inline/excluded sources and configurations without utilities. The compiler-only
WASM slice also executes these incremental comparisons.

```sh
mbx xtask utility-conformance --check --group scanner-pipeline
```

This separate gate builds the facade with `compiler,scanner`; it does not enable
source scanning in production example Workers or claim SDK/browser integration.
The resolved legacy tree is charged to each session's explicit cache budget;
only the 88 full-default-theme oracle cases declare a 4 MiB cache budget.
Earlier native 64-bit conservative cache charges were 462,323 bytes for an empty CSS
theme and 1,868,507 bytes for the full default CSS theme; these are owned-data
accounting bounds, not allocator measurements or fixed production defaults.
Four official cases whose output contains isolated UTF-16 units are frozen in
[legacy-theme-boundaries.json](legacy-theme-boundaries.json). Native and WASM
runners now compare their lossless output by code unit. The original 486-case
compiler input/output UTF-16 matrix now passes through actual WASM; full UTF-16
conformance beyond that frozen matrix remains pending.
These use Rust algorithms and declarations, rather than a dynamic CSS lookup.
The larger upstream inventory remains pending; other functional branches,
directives and full variant/parser/API parameterizations still need work.

```sh
node tests/utility/compiler/static.mjs --check
node tests/utility/compiler/effects.mjs --check
node tests/utility/compiler/effects-complete.mjs --check
node tests/utility/compiler/arbitrary-properties.mjs --check
node tests/utility/compiler/custom-utilities.mjs --check
node tests/utility/compiler/custom-variants.mjs --check
node tests/utility/compiler/parser-errors.mjs --check
node tests/utility/compiler/variant-parameters.mjs --check
node tests/utility/compiler/source-directives.mjs --check
node tests/utility/compiler/variants.mjs --check
node tests/utility/compiler/functional-spacing.mjs --check
node tests/utility/compiler/dynamic-variants.mjs --check
node tests/utility/compiler/colors.mjs --check
node tests/utility/compiler/typography.mjs --check
node tests/utility/compiler/borders.mjs --check
node tests/utility/compiler/backgrounds.mjs --check
node tests/utility/compiler/gradients.mjs --check
node tests/utility/compiler/transforms.mjs --check
node tests/utility/compiler/theme.mjs --check
node tests/utility/compiler/filters.mjs --check
node tests/utility/compiler/shadows.mjs --check
node tests/utility/compiler/prefix.mjs --check
node tests/utility/compiler/motion.mjs --check
node tests/utility/compiler/grid.mjs --check
node tests/utility/compiler/layout.mjs --check
node tests/utility/compiler/masks.mjs --check
node tests/utility/compiler/miscellaneous.mjs --check
node tests/utility/compiler/structural-spacing.mjs --check
node tests/utility/compiler/functions.mjs --check
node tests/utility/compiler/legacy-theme.mjs --check
node tests/utility/compiler/imports.mjs --check
mbx xtask utility-conformance --check --group compiler-slice
```

After building the compiler WASM facade through the conformance command, run
`node tests/utility/compiler/run-wasm.mjs --variant-parameters` to repeat just
the variant parameter matrix and the shared session checks. `--slice` includes
this matrix in the complete implemented slice.

The conformance gate checks the official oracle, native Rust tests, and the real
WASM facade. [run-wasm.mjs](run-wasm.mjs) compares 214,909 CSS cases (216,133
exact comparisons including incremental builds), 3,002 exact error messages
and 6,673 variant parser JSON results plus the two fixed compiler fixtures.
It checks scanner exclusion and private token behavior, and reports
`fullConformance: "pending"`.
It also compares 1,607 source metadata results and separately checks the four
frozen UTF-16 theme outputs by code unit. The separate original 486-case UTF-16
input/output runner now passes and is included in the compiler-api gate.
New fixture expansions must pass this gate before claiming WASM coverage. Browser installation and
GPU acceptance use separate runtime runners.

The reference benchmark is replayable with:

```sh
node tests/utility/compiler/bench.mjs --output tests/utility/compiler/benchmark-baseline.json
```

Run it without concurrent Cargo or browser measurements. [benchmark-baseline.json](benchmark-baseline.json) includes 100 cold session, warm incremental and no-op samples, Node memory snapshots and the machine description. Cold means a new compiler session after Node JIT warmup. Warm calls add one width candidate to a stylesheet that grows from 3 to 102 candidates. These figures measure the official Node reference; Rust/WASM, crossing, CSSOM and GPU measurements are separate requirements.


`public-api.mjs --check` verifies the frozen public API oracle against Tailwind
4.1.13. It covers compile/compileAst build reuse, base/from/import contexts,
Features/Polyfills, tracked source mappings, design-system methods and warnings.
`public-api-fixtures.json` still declares implementation and full API conformance
pending; generating this oracle does not prove Rust production equivalence.

The optional public facade has a separate implemented slice:

```sh
mbx xtask utility-conformance --check --group compiler-api
```

This checks the fixed oracle and generated completion definitions, native public
API tests, then actual WASM objects in [run-public-api-wasm.mjs](run-public-api-wasm.mjs).
It compares 313 CSS builds, 371 decoded maps with 6,970 mappings, and 61 AST
builds. Three design-system modes exercise twelve methods, 21,771 total class
completions and 756 selector probes. It also checks decoded Source identity,
result array reuse, undefined field presence, actual BigInt and Map values,
parse caches, selector callback lifetime and failure recovery. The current
actual WASM run passes all of these observations, including 37 boundaries and
the original 4 MiB cache limit for class-list completions; the earlier cache
regression is repaired.

The same gate also runs [run-utf16-functions-wasm.mjs](run-utf16-functions-wasm.mjs)
against `CssCompiler.substituteFunctionsInValue`. It compares 75 frozen function
values, 18 exact error message unit sequences, and eight crossing/quota/recovery
checks, including all 65,536 units. The typed native error JSON roundtrip covers
isolated units without a scalar `serde_json::Value` intermediate. These are
expression helper tests; the separate full-input runner now also passes all
486 cases, including 61 compile/build errors and 111 raw theme observations.

[utf16-value-native.rs](utf16-value-native.rs) checks the single lossless
`ValueNode` representation: 27 frozen authored groups, 75 frozen function helpers,
raw function names, parser/segmentation edges and typed JSON for every code unit.
It also compares 324 explicit underscore, URL, var/theme and math expectations.
The same gate runs these native tests and
[run-utf16-values-wasm.mjs](run-utf16-values-wasm.mjs) against
`CssCompiler.decodeArbitraryValue`. That runner imports the fixed upstream helper
with its recorded source digests and compares 306 inputs plus nine boundary
checks, including all 65,536 code units. The complete frozen stylesheet/candidate
matrix now passes separately; unified temporary-memory accounting remains pending.

[utf16-declaration-native.rs](utf16-declaration-native.rs) adds seven tests to
the same gate for the canonical Text declaration fields: scalar JSON schema,
all code units and reordered fields, 18 frozen property/value emitter groups,
important output, deduplication, ordinary/fresh clone sharing, Source allocation
charges, 75 frozen function outputs and nine scope paths. Function substitution
and scope rewriting retain source/destination and declaration identity. These
internal AST tests do not cover full public AST object identity or raw stylesheet
input crossing.

[utf16-theme-native.rs](utf16-theme-native.rs) adds seven tests for one Text theme
map and insertion order, raw/prepared values, default/overwrite/reset behavior,
namespace exceptions, dependency cycles, selected Source identity and allocation
capacity. Its fixture adapter builds flat authored AST blocks directly and checks
all 111 frozen theme observations; it does not test the production CSS parser.
[run-utf16-theme-wasm.mjs](run-utf16-theme-wasm.mjs) runs in the same gate and
previously compared 75 CSS cases with scalar candidates, including eight raw
non-scalar source cases. The current gate compares all 111 CSS cases and all
111 exact public theme observations. Twenty tracking
comparisons cover raw/escaped keys, prefix behavior, repeated calls and errors,
including USED changes before a RangeError in encounter order;
six quota/recovery boundaries include a non-scalar tracking input. The remaining
36 candidate CSS observations now pass, leaving zero pending cases in this gate.

[run-utf16-theme-sources-wasm.mjs](run-utf16-theme-sources-wasm.mjs) adds eighteen
import cases, thirty-six source maps, twenty-four shared Source identity checks and two
shared-cache preflight rejection/recovery checks. Undefined, empty and nonempty
from values preserve the distinct upstream import/root/printing behavior; Source
code retains original units. Native public API tests compare all 111 entries
through typed JSON, output limits, tracking and imported Source spans. Tracking
tests also reject over-budget parser/unknown-variable allocations and verify
that Limit leaves USED and native retained costs unchanged; WASM tests include
retained JS cache costs and recovery.

[utf16-ast-fields-native.rs](utf16-ast-fields-native.rs) adds twelve tests for Text selectors,
at-rule names/params, comments and context paths. It covers all code units in
nested, reordered typed JSON, 27 frozen direct-AST CSS observations, context
pruning, clone/source identity and actual field capacity. It also tests at-rule
function substitution, scope keyframe/property bindings and source coordinates.
The source-map module directly tests printed positions and retained AST capacity.
These tests cover internal AST contracts; the production parser tests below
cover raw CSS. The public object bridge now stores strings and keys as Text
through one PublicValue tree and direct typed JSON; complete public object graph
conformance remains unfinished. Existing public API and runtime WASM gates
continue to verify their recorded fixtures.

[run-utf16-ast-fields-wasm.mjs](run-utf16-ast-fields-wasm.mjs) also runs in the API
gate. It compares eight CSS builds and their source maps with the installed fixed
4.1.13 compiler, checking the locked source digests. Scalar input containing an
astral theme value produces lone units through legacy string indexing inside four
at-rule kinds; this exercises Text parameter substitution, emitted map coordinates
and Source table identity in actual WASM. The frozen source/candidate matrix
now passes; the separate candidate gate below exercises the canonical public
AST object bridge. Complete object graph identity remains pending.

[utf16-parser-native.rs](utf16-parser-native.rs) adds thirteen production-parser
and compiler tests. They compare 156 frozen authored/map/error cases (102 CSS
and 54 compile errors), all 92 authored parser inputs, 18 parser error messages,
all ten raw map/theme observations and nine primitive-index candidate outputs.
They also cover all code units in a license comment, BOM/CRLF/source spans,
Text virtual assets, plain CSS preservation, scalar convenience entries and
input/depth limits. CompilerInput and SourceAsset have an additional direct
JSON regression in protocol.rs. Input text is never escaped or replaced to
make it acceptable to the parser.

[run-utf16-parser-wasm.mjs](run-utf16-parser-wasm.mjs) runs in the API gate. It
compares the same 156 cases through original JS strings, including 102 decoded
maps and their Source-table references against the fixed compiler. Eleven
boundary observations include exact logical input limits, a non-scalar JSON
asset and source map, and two Worker-session prepare/commit/report sequences.

[run-utf16-input-wasm.mjs](run-utf16-input-wasm.mjs) now passes the original
486-case input matrix through actual WASM: 425 exact CSS results, 61 error
phase/message matches and all 111 theme observations, with zero failed cases
or observations. It tests original strings for every fixture, including
non-scalar candidates, and is now included in `compiler-api`. The facade throws
real Error/RangeError objects with original message units and typed cause
metadata; session wire errors retain their JSON envelope. Earlier RED logs
remain historical evidence in the [M4 record](../../../docs/validation/wasm-tailwind/M4.md).

[run-utf16-candidates-wasm.mjs](run-utf16-candidates-wasm.mjs) is also included
in `compiler-api`. Its current actual WASM run passes 49 candidate comparisons,
28 variant comparisons, 92 identity observations and seven public AST builds.
Candidate/Variant values, modifiers, roots, selectors and dynamic names, compiler
cache/report/identity fields, @apply and inline source expansion, Worker sessions
and public build inputs use canonical Text. Native CSS/AST `build(&[String])`
conveniences feed `build_text(&[Text])`. PublicAst is `Arc<Vec<PublicValue>>`;
PublicValue strings and object keys are Text, decoded directly without scalar
Value staging. The current native compiler suite passes 364 tests.

[utf16-error-native.rs](utf16-error-native.rs) adds seven tests for all 61 error
phases/categories/messages, ordinary/keyframe/function-output usage, Features.None,
failed-build recovery, dependency handling and typed RangeError JSON/capacity.
[run-utf16-errors-wasm.mjs](run-utf16-errors-wasm.mjs) is part of the API gate:
59 setup and two build errors include six real RangeError prototypes, three
recovery/passthrough checks and one protocol TypeError. This frozen error matrix
now passes; complete public-object contracts remain unfinished. See the
[M4 record](../../../docs/validation/wasm-tailwind/M4.md) for the current artifact
and historical full-matrix failures.

The WASM classes take an explicit JSON configuration with seven positive limits
and a virtual asset bundle; they adapt the upstream API rather than implementing
its asynchronous filesystem callbacks or mutable registries. Complete public
object graphs and Source/AST contracts remain unfinished. PublicValue arrays use
Arc storage; objects use Arc storage with a stable identity and insertion-ordered
Text fields. [run-public-object-order-wasm.mjs](run-public-object-order-wasm.mjs),
included in `compiler-api`, checks 121 object/array key observations against the
fixed production oracle, including candidate/value fields, variants, generated
AST and authored non-scalar/numeric/own `__proto__` keys. Native tests retain
decode order across duplicate-key overwrite, mutation and JSON roundtrip and
charge spare map capacity. Complete mutable
API behavior, virtual path boundaries, DOM class ingress and Scanner String
boundaries, and allocator/crossing/temporary accounting also remain pending.
M4–M7 and full API conformance remain unfinished. This Text migration has not
been verified in a browser; Node WASM gates do not replace runtime browser or
GPU acceptance.

[run-public-ast-graph-wasm.mjs](run-public-ast-graph-wasm.mjs), included in
`compiler-api` with `--input-mutations --apply-locations --apply-multi-locations`, compares 27
declaration/Source/output graphs with the fixed compiler: 1,013 objects, ten direct identity checks, four
boundaries and two recoveries. Coverage includes equal-content distinct Sources, shared tuples,
frozen metadata, original units, transformed declarations, imported src/dst,
stable banner and Theme declarations, and additive builds. A throwing output
getter retains the last published AST on the next no-op call, while admitted
candidates remain available for the next changed build, matching the oracle.
Public @apply provenance travels through core substitution instead of inheriting
one parent span. Sixteen added cases cover multiple candidates/directives, last
occurrence offsets, ECMAScript whitespace, variants, shared tuples, properties,
original UTF-16 Source/candidate units, fractional/negative offsets, missing src,
from omitted/empty, frozen metadata, nested custom utilities and no utilities
anchor. Original Source identity and native integer JSON representation are
preserved. Fresh clones preserve alias relationships while creating fresh
Source/tuple identities; source metadata and growth after substitution are
charged. The `--input-mutations` probe now passes and is part of the default gate.
The complete custom-input and metadata-origin runners below also pass. Complete mutable provenance,
location/helper and public-object conformance remain pending.

[run-public-ast-mutations-wasm.mjs](run-public-ast-mutations-wasm.mjs), included
in `compiler-api`, compares eight utilities cases across 38 constructor/build
phases, twelve no-op identities, three original setter exceptions and four
cache boundaries. It observes the original utilities object, old children array,
Source metadata, raw candidate trees and every returned output together. The
single native raw candidate cache retains internal nodes; final pruning happens
in render. Raw/output declarations share JS identity, each changed build assigns
a fresh raw children array, and quota preflight covers both graphs before
publishing. Rejection leaves caller input and prior output intact. Setter failure
retains admitted candidates and the prior published output for no-op recovery.
The UTF-16 candidate runner gives each compiler its own equivalent input because
construction now mutates it. Import/source/variant/function/apply setup,
all-family raw structures, further array operations and complete accessor/proxy/partial
failure contracts remain pending. See the
[design record](../../../docs/validation/wasm-tailwind/ast-input-mutation-design.md).

[run-public-ast-setup-wasm.mjs](run-public-ast-setup-wasm.mjs), included in
`compiler-api`, checks eighteen direct theme setup cases, 105 constructor/build
phases, 29 no-op identities, two original constructor exceptions and three cache
boundaries. First theme replacement is an index assignment; later themes are
removed through the original array's splice method. The new rule starts empty
and receives final ordered declarations afterwards. It shares declaration
identity with optimized output and preserves old theme objects and arrays.
Source, prefix/escaped names, defaults/reference/inline/static, namespace clearing,
nested style/media, empty blocks, internal sort markers and keyframes are covered.
Constructor quota rejection precedes writes; setter/splice errors preserve their
original exceptions and completed prefixes. Broader setup/error timing and theme
optimization/build mutations remain pending.

Theme pruning now follows rule optimization. One real unused declaration preserves
the nonempty-rule decision when needed, then is deleted by identity, avoiding
rebuilding every unused token. The optional
[bench-theme-pruning-wasm.mjs](bench-theme-pruning-wasm.mjs) accepts a compiler-api
module directory and measures a 402-token stylesheet across 100 sessions and
1,700 changed builds. The controlled before/after CSS digests match; timings and
limits are recorded in [M4](../../../docs/validation/wasm-tailwind/M4.md).
This synthetic benchmark does not certify M7 performance or allocation budgets.

[run-public-ast-apply-input-wasm.mjs](run-public-ast-apply-input-wasm.mjs), included
in `compiler-api`, compares fifteen direct apply cases across 90 constructor/build
phases, eighteen no-op identities, two original constructor exceptions and three
cache boundaries. Original input arrays, old directives/children, Source tuples
and every build share one observation graph. Native replacement events retain raw
sort declarations and at-root Source metadata; optimized output shares the same
declarations. Single replacements assign the original index; multi-node replacements
call the original splice method. Earlier theme deletion and apply expansion adjust
indices using prefix lengths. Utilities setup follows apply, preserving throwing
setter/splice prefixes. Constructor preflight rejects before caller writes.
The custom runners below now cover body fields/normalization/functions. Empty
expansion, broader shared AST arrays and failure/accessor timing remain pending; the complete input probe passes its finite matrix.

[bench-apply-input-wasm.mjs](bench-apply-input-wasm.mjs) accepts an optional
compiler-api module directory. It measures six direct apply directives over 200
sessions and 800 builds per sample, five measured samples plus warmup. The previous
preserved module and current module emit the same serialized AST digest; constructor
and build p95 values are recorded in [M4](../../../docs/validation/wasm-tailwind/M4.md).
This no-theme fixture does not certify M7 performance or allocation budgets.

[run-public-ast-custom-input-wasm.mjs](run-public-ast-custom-input-wasm.mjs) is
included in `compiler-api` in its complete default mode: fourteen cases and 84
constructor/build phases, with twenty no-op identities. It compares definitions,
old bodies/children, arbitrary child fields, Source metadata, splice traces, functional
normalization/functions and output graphs. The removal-only eight-case option remains
available for historical comparisons; shared arrays invoke splice once.
An optional first positional argument selects a preserved compiler-api module.

[run-public-ast-custom-origin-wasm.mjs](run-public-ast-custom-origin-wasm.mjs) adds
five cases and thirty phases for a live caller array referenced by Source metadata,
static/functional child/body/src/dst/extra tuple aliases, Context/AtRoot/comment fields,
and aliased function setter traces. Native tests also check origin-independent Eq and
serialization, ordinary versus fresh clone, current COW/source/body aliases and quota
rejection. The public AstNode literal/pattern migration affects Rust source consumers;
the JSON schema remains unchanged. Complete imports, failure prefixes, all accessor
and Proxy timing, and session-wide allocation accounting remain pending.

[bench-custom-removal-wasm.mjs](bench-custom-removal-wasm.mjs) measures sixteen
static definitions over 200 sessions and 800 builds per sample, five samples plus
warmup. The origin comparison uses the preserved pre-origin module and current module
serially; output JSON digests match. Timings and the observed added cost are recorded
in [M4](../../../docs/validation/wasm-tailwind/M4.md). The earlier deletion-only
comparison remains historical. The [origin design](../../../docs/validation/wasm-tailwind/custom-origin-design/synthesis.md)
and isolated PublicValue prototype do not certify unified production peak accounting.

[run-public-ast-budget-wasm.mjs](run-public-ast-budget-wasm.mjs), also in
`compiler-api`, checks 19 input quota boundaries and twelve successful calls
after rejection. An 18-level shared DAG is rejected before JSON expansion:
fresh-module linear-memory growth fell from 8,650,752 to 65,536 bytes.
The input budget measures expanded JSON bytes with memoized storage sizes,
including original UTF-16 keys and values, before traversing AST depth.
Native tests compare these lengths for all 65,536 code units. Memory growth is
an allocation witness; complete temporary/crossing accounting remains pending.

[run-utf16-animations-wasm.mjs](run-utf16-animations-wasm.mjs) compares 23
production-compiler inputs across initial, added-candidate and repeated builds:
69 exact CSS results and 69 source maps. It covers lone-surrogate and scalar
keyframe names, theme animation variables, lone/replacement identity, and
ECMAScript whitespace including FEFF and U+0085. Two native parser/compiler tests
cover the same pruning boundary.

The value module adds five arbitrary-validity tests covering all UTF-16 units
and the fixed upstream 256-byte bracket stack, alongside opaque quote/escape and
whitespace behavior. Its scalar and Text entries share one units algorithm,
now consumed by the canonical Text candidate parser and public object helpers.
Complete public object graph and allocator conformance remain pending.

[utf16-families.mjs](utf16-families.mjs) generates 560 production-oracle CSS
cases in [utf16-family-fixtures.json](utf16-family-fixtures.json), with input and
expected output encoded as UTF-16 units. Twenty ordinary family paths each
cover eight unit shapes in ordinary, inline and reference Theme modes; another
80 shadow and drop-shadow cases cover replacement-string dollar markers.
Every selected candidate independently produces CSS; the first 480 cases also
verify that the production output contains the original Theme payload.
[utf16-family-native.rs](utf16-family-native.rs) and
[run-utf16-families-wasm.mjs](run-utf16-families-wasm.mjs) compare exact units
through native and actual WASM compilation. This gate is included in
`compiler-api`; it keeps candidate inputs scalar and does not replace the
separate 486-case original candidate input matrix or complete allocation audit.

[run-shadow-budget-wasm.mjs](run-shadow-budget-wasm.mjs), also included in
`compiler-api`, observes fresh WASM linear-memory growth for three repeated
prefix/suffix replacement expansions. It checks 108 below/equal/above public
CSS quotas across four shadow roots, three Theme modes and three unit/marker
shapes, plus 39 failed-build recoveries that retain CSS/maps and retry block.
Successful CSS is compared with the pinned production compiler, including the
banner. The growth check is an allocation witness rather than a complete
allocator peak or temporary-memory audit.

```sh
node tests/utility/compiler/utf16-families.mjs --check
mbx test --locked -p rustify-style --no-default-features --features compiler --lib compiler::utf16_family_tests::
node tests/utility/compiler/run-utf16-families-wasm.mjs
node tests/utility/compiler/run-shadow-budget-wasm.mjs
node tests/utility/compiler/run-utf16-input-wasm.mjs
node tests/utility/compiler/run-utf16-candidates-wasm.mjs
```

[run-public-ast-import-input-wasm.mjs](run-public-ast-import-input-wasm.mjs),
included in compiler-api, checks 21 import graphs plus one original array-order
trace across 107 constructor/build phases. Coverage includes omitted/empty/tracked
from, repeated/nested imports, layer/media/supports and caller src, reference/theme,
imported utilities/custom/functions/apply, moved media children, source/prefix/theme/
important options, BOM and isolated units, and an undefined preceding declaration.
Native tests check fresh imported Sources and shared output declarations, and retain
the native owner of every address-keyed source-location cache entry. Broader loader,
failed-setup, accessor and shared-array behavior remains pending.

[bench-import-input-wasm.mjs](bench-import-input-wasm.mjs) accepts a module directory
and measures sixteen repeated tracked imports over 100 sessions and 400 builds
per sample, five samples plus warmup. It hashes only CSS-bearing AST fields because
this change intentionally repairs source and input identity. Complete graph gates
check those separately. The observed increased constructor cost and artifact sizes
are recorded in [M4](../../../docs/validation/wasm-tailwind/M4.md); M7 acceptance
and unified temporary accounting remain pending.

[run-public-ast-failed-setup-wasm.mjs](run-public-ast-failed-setup-wasm.mjs) is
included in compiler-api. It compares twelve function-phase semantic failure
graphs, four original setter exceptions, and three admission boundaries. Coverage
includes aliases, preceding theme/registration effects, at-rule params, custom
bodies, Source metadata, isolated units and imported declarations. The native
carrier exposes the error, recorded prefix and conservative reserve; the host
checks graph/journal/handle costs before writes, then preserves the original host
exception or returns the semantic error. Complete failed setup timing and peak
allocation accounting remain pending.

[run-public-ast-import-failure-wasm.mjs](run-public-ast-import-failure-wasm.mjs)
is included in compiler-api. It compares fifteen semantic failures at the
constructor's modeled rejection boundary and after retained tasks settle, including
two to eight nested imports, sibling order, parser errors and synchronous parameter
errors. Five setter exceptions preserve the original error object; one case checks
first settlement against a later host fault. Four quotas reject before any caller
write, with an untracked failed-parser Source as an admission control. Captured
contexts remain observable even when the publishing setter throws. The Rust-owned
session posts each retained job separately and releases its owners after quiescence.
The finite-bundle Promise publication and orphan reporting are covered below;
external loader callbacks, arbitrary successful accessor/Proxy mutations and
unified allocator peaks remain pending.
The passing scope, performance cost and historical RED evidence are recorded in
[M4](../../../docs/validation/wasm-tailwind/M4.md).

[run-public-ast-async-wasm.mjs](run-public-ast-async-wasm.mjs), included in
compiler-api, compares twenty actual Promise cases with the pinned oracle:
external microtask markers, observation epochs, full prefix and settled graphs,
five builds and no-op identities. Six orphan cases check native rejection events,
including a joined-task control and the original completion exception after an
original begin exception. Four admission quotas reject without caller writes.
The initial theme/media phase precedes the compatibility await; normalization,
final theme and function operations follow it. Calls without imports retain both
await boundaries. JavaScript only posts callbacks and publishes native Promise
settlement; the importer and reaction transitions remain Rust-owned.

```js
const compiler = await facade.compileAst(input, JSON.stringify({limits, options, assets}))
try {
  const output = compiler.build(['block'])
} finally {
  compiler.free()
}
```

The Promise resolves the exported AstCompiler class. The existing constructor
continues to model a synchronous observation. Both consume finite virtual assets;
arbitrary externally delayed loaders, successful Proxy changes to subsequent
walks, full helper/API conformance and unified peak accounting remain pending.
The current local benchmark measures the constructor, not the new Promise entry.

`run-public-ast-apply-failure-wasm.mjs` checks sixteen apply failures through both
the synchronous constructor and `compileAst` Promise: unknown candidates,
earlier completed directives and parents, nested and multi-node replacements,
custom definitions/dependencies/cycles, theme and keyframes, Source identity,
media flattening, tracked imports, and non-scalar class names. All 32 comparisons
check exact errors and complete caller graphs; Promise cases also compare
rejection epochs and settled graphs. Eight host checks preserve original setter
exceptions and their completed prefixes. The runner checks immediately below,
at, and above the synchronous semantic admission threshold: admitted failures
retain earlier effects, while quota refusals leave the complete graph unchanged.
Two asynchronous quota refusals also require no synchronous or detached writes.
The runner is included in the default compiler-api gate. Earlier setup failures,
wider custom/plain ordering, dynamic shared-array/Proxy behavior, and unified
allocator peaks remain pending.

`run-public-ast-function-order-wasm.mjs` compares nineteen ordinary/custom
function scenarios through the synchronous constructor and Promise entry. Its
38 comparisons include eight original host exceptions and cover both orders,
interleaved definitions, first-error choice, completed failure prefixes, shared
declarations, transparent contexts, theme, Source metadata, tracked imports,
and caller at-root comments that resemble private markers. It compares complete
caller and build graphs, setter order, Promise epochs, settled state, and five
builds including no-op identity. The native compiler now substitutes functions
at each definition's original position using one shared Edits journal; the
apply arena uses the same provenance-aware private-slot lookup. The runner is
in the default compiler-api gate. Earlier phases, wider normalization/variant/
theme interactions, dynamic Proxy/shared arrays and unified peaks remain pending.

`bench-function-order-wasm.mjs` accepts an optional module directory and measures
sixteen interleaved ordinary/custom function pairs, 200 sessions and 800 builds
per sample, with five samples after warmup. Complete output JSON digests must
match across samples; final performance acceptance remains pending.
