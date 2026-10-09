# Rust compiler foundation

This module is a partial Rust port of Tailwind CSS 4.1.13 at commit
`1334c99db8fd26c8ea065375dd9259800863f072`. It compiles complete class strings,
without a scanner or a production JavaScript/TypeScript engine. The complete
compatibility inventory remains in `tests/utility/compiler/inventory.json`;
registry names are metadata, not proof that a utility or variant is implemented.

The CSS AST and serializer preserve declaration order, duplicate declarations,
and nested rules. Candidate/value parsing includes balanced segmentation,
arbitrary values/properties, important markers, and upstream math formatting.
CSS-native themes support default, inline, reference, static, namespace resets,
source order, and variable dependencies. Native source tracking is available
through the string API; complete parser and source-map conformance remain pending.

All 889 fixed built-in static utility roots have structural Rust AST emitters.
883 use generated templates; container and five transition handlers resolve
the current theme in Rust. Native tests compare every root in ordinary,
important, and hover forms, all roots together in sorted order, and six theme
configuration probes. This is static coverage under those inputs, not complete
compiler conformance. Nested static branches retain their CSS and explicitly
reject GPU projection until the GPU program can represent those branches.

The verified utility, variant, directive and error slices are recorded in
[coverage.json](../../../../tests/utility/compiler/coverage.json). CSS-native
custom utilities, custom variants, apply dependencies, theme functions,
virtual imports/references and inline source expansion have exact official
fixture comparisons. These finite matrices do not establish complete compiler
conformance. The frozen 486-case UTF-16 input matrix now passes through actual
WASM, including original non-scalar candidates and all 111 theme observations.
Complete source-map and upstream API/test coverage remain unfinished. Lossless
`Text` also preserves the four official legacy-theme boundary cases.
`@plugin` and `@config` execution return explicit unsupported errors.

Import/media options propagate `theme(...)`, `prefix(...)` and `source(...)`
to their declarations. `important` sets global utility importance; ordinary
media conditions retain their nesting. Registration bodies retain the fixed
upstream traversal boundaries. The fixed-version behavior of unused theme
pruning and unchanged-input builds is preserved, including declaration and
at-rule order.

The canonical native CSS entries are `public_api::compile_text` and
`CssCompiler::build_text(&[Text])`; their scalar convenience entries use the same
algorithms. `public_api::compile_ast` (also exported as `compile_ast_text`) accepts
`PublicAst = Arc<Vec<PublicValue>>`. `PublicValue` stores string values and object
keys as Text and uses direct typed JSON codecs, without a scalar Value intermediate.
`AstCompiler::build_text(&[Text])` reuses the core AST compilation path and
preserves input/previous result array reuse under the fixed fixture cases.

`CssCompiler::build_source_map` returns a decoded map of the initialized AST or
the last successful build. A nonempty `from` tracks the root source and enables
printed destinations; any supplied `from`, including an empty string, tracks
imported sources. Locations
use UTF-16 offsets, and mappings share their `Arc<DecodedSource>` table handles;
distinct authored/generated Source objects retain distinct identities even when
their text matches. Native tests compare 58 frozen compile groups, 313 builds and
6,970 initial/built mappings, including source reference indexes. Retained source
text, the mapped AST and map construction consume the logical cache budget.
Failed builds/map requests preserve the committed revision and previous map;
unchanged builds do not accumulate tracked storage. Builds without a `from`
option do not retain a mapped AST or allocate Source objects. DesignSystem
loading tracks imported sources when `from` is supplied, without retaining
an unused root mapped AST. The optional WASM public
facade now compares these frozen maps and AST builds, including shared decoded
Source table references and result array reuse. Full provenance and shared AST
node/Source identities remain unfinished.

`public_api::load_design_system` provides the twelve fixed design-system methods
using the Rust parser, emitters and registry. Frozen tests cover ordinary, inline
and prefixed themes, complete class-list digests, variant selector probes,
printing, property-sort metadata, class order and theme variable tracking.
Generated completion definitions retain namespace/literal metadata; Rust expands
them against the current theme and CSS-native utility registrations.

The `rustify-style-wasm` feature `compiler-api` exposes `CssCompiler`,
`AstCompiler` and `DesignSystem` as ordinary JavaScript classes. Build and compare
them with:

```sh
mbx xtask utility-build --group compiler-api
mbx xtask utility-conformance --check --group compiler-api
```

Constructors accept a CSS string (or AST array) and a JSON configuration with
`options`, a virtual `assets` bundle and all seven positive `limits`. Each asset
has `id`, optional `path`/`base`, and `content`. Limits are caller choices, with
no implicit defaults. The public facade is separately enabled from the runtime
Worker compiler and does not include scanning. `getClassOrder` returns JS BigInt,
`getVariantOrder` returns a Map keyed by cached `parseVariant` objects, and
`getVariants` supplies JS-owned selector callbacks. Repeated candidate/variant
parses retain their JS values within the logical cache quota. `themeEntries()`
returns typed Text keys/values and optional SourceLocation tuples. The WASM
adapter preserves shared Source objects across tokens and calls, including
original non-scalar source code. Candidate and variant objects, their caches,
printers, selector helpers and completion values use the same canonical Text
model. Scalar native Value convenience methods adapt only at the outer boundary.

`CssCompiler.substituteFunctionsInValue(value, atRule?)` exposes the existing
native function-value helper with original JS UTF-16 units. Its gate compares
75 frozen values and 18 error messages, plus every code unit and output chunk
boundaries. `CompileError::Syntax.message` is `Text`; direct typed JSON encoding
and decoding retain isolated units in the existing string field. Diagnostic
Display escapes isolated units for Rust logs and is separate from that wire
message. CompileError::RangeError retains the native range category and the same
Text codec/capacity accounting. Public WASM methods throw real Error or RangeError
objects with original message units and typed metadata in a non-enumerable cause;
protocol errors are TypeError. CompilerSession retains its JSON wire envelope.

CSS values now use one `ValueNode` tree with `Text` words, separators and
function names. Parsing, serialization, function substitution and scope variable
rewriting retain original UTF-16 units; direct typed node JSON decoding bypasses
serde's scalar tagged-enum buffer. `decode_arbitrary_text` also handles underscores
and math formatting by code unit, including ECMAScript whitespace rules.
`CssCompiler.decodeArbitraryValue(value)` exposes that helper through the existing
input/depth/output limits. Its actual WASM gate compares 306 inputs with the fixed
upstream helper and checks all code units plus quota/recovery boundaries. Native
tests add 324 explicit arbitrary-value expectations and lossless node JSON.

Declarations now use one `AstNode::Declaration` with Text property/value; the
separate TextDeclaration variant is removed. Function substitution, scope rewriting,
polyfills, deduplication and emission preserve those units. Direct typed AST JSON
decoding keeps the existing scalar wire schema and accepts isolated units.
Substitution retains the declaration identity and source/destination metadata;
fresh cloning retains sharing within the cloned graph. Native declaration tests
also check actual Text capacity and shared Source allocation charges.

Theme tokens now use one `BTreeMap<Text, ThemeToken>` and one Text insertion-order
vector. Keys, raw/prepared values, dependencies and used-variable sets preserve
original units through aliasing, default/reset/reference handling and emission.
Legacy key escape decoding and nested lookup use the same unit-based token map.
Token accounting charges map slots conservatively, collection capacities and
owned Text allocations; full temporary and provenance allocation budgets remain
unfinished. Seven native tests include all 111 frozen theme observations through
direct authored AST inputs. The earlier actual WASM gate checked 75 CSS cases with scalar candidates,
including eight raw non-scalar source cases. The current gate checks all 111 CSS
cases and all 111 public entry observations, including the remaining 36
candidate observations.
Twenty variable-tracking comparisons exercise raw/escaped keys, prefix slicing,
repeated calls and RangeError behavior against the fixed compiler. Eighteen import
cases additionally compare Source locations, thirty-six initial/built maps and
shared Source identity. Missing, empty and nonempty source paths are crossed with
missing, empty and nonempty `from`; an empty resolved path remains untracked. Entry snapshots and JS/source caches are
preflighted before cloning. Variable tracking preserves encounter order up to
a RangeError; cache limits are checked before committing the complete new USED
set. Parser temporaries, Text ingress and retained JS values enter the same
transaction budget. Limit failures leave USED and retained costs unchanged.
Candidate inputs and public string/key values now preserve Text throughout
this gate; complete public object graph conformance remains unfinished.

Utility families distinguish candidate diagnostics from compiler resource errors.
Opaque CSS values remain Text through classification, Theme resolution and emission. All AST text fields now use Text,
including rule selectors, at-rule names/params, comments and context paths. Direct
typed JSON decoding preserves isolated units and field-order independence.
Emission, at-rule function substitution, compound selector transforms and scope
keyframe/property rewriting retain units; printed source-map positions count
units directly, and AST caches charge owned Text capacity. Import layer/media/
supports wrappers also retain Text. Virtual loader paths remain scalar boundaries;
Candidate/Variant roots and public object string/key fields are Text. The production CSS
parser now scans original UTF-16 units, with Text buffers and error excerpts;
source spans are already unit offsets and require no later translation.
CompilerInput.css, SourceAsset.content and Stylesheet.content are Text.
compile_text and load_design_system_text are the canonical native entries;
scalar convenience entries only convert into the same parser. The WASM CSS and
DesignSystem constructors use the original units, and JSON assets and Worker
sessions use direct typed Text decoding. Plain CSS passthrough retains its Text
allocation and charges owned capacity. Candidate function results also retain
Text declarations; GPU projection explicitly rejects non-scalar values while
keeping the DOM AST. Utility families now resolve Theme payloads, concatenate
CSS fragments and collect declarations as Text. Candidate/Variant raw text,
values, modifiers, properties, selectors and dynamic names are Text. Compiler
cache keys, reports, candidate identities, @apply dependencies and inline source
expansion use that same model. Worker session JSON and public CSS/AST builds
retain original units; native scalar conveniences feed the canonical Text
algorithm. Complete temporary-memory budgets remain unfinished.

This is a bounded WASM adaptation with explicit virtual inputs. It does not yet
provide the upstream asynchronous loader callback API or mutable theme/utility/
variant registries. Complete nested object graphs, shared Source/AST node
identity remain pending. PublicValue objects use boxed insertion-ordered maps;
the WASM writer retains field insertion order and JavaScript enumerates integer
keys. A fixed-oracle gate verifies 121 object/array key observations, including
own `__proto__` and non-scalar keys. Complete mutable upstream API behavior, virtual path text boundaries,
DOM class ingress and Scanner String boundaries, and allocator/crossing/temporary
accounting also remain pending. M4–M7 are unfinished; the finite public API gate
reports `fullApiConformance: "pending"`. This Text migration has not been verified
in a browser.

`Compiler::parse_variant_object_text` returns the typed fixed design-system
variant shape or None, using the session's registered roots and theme.
`Compiler::parse_variant_json` remains its scalar JSON convenience entry. It preserves the
committed report, enforces the candidate byte/depth budgets and is exposed as
`CompilerSession.parse_variant` in the WASM facade. Its JSON observations are
checked separately from compilation, since parsing can accept a variant that
cannot apply to the selected utility.

`Compiler::new` requires seven positive caller limits. `prepare_add_text(&[Text])`
stages the full sorted union and its cache; `prepare_add(&[String])` is a scalar
convenience entry into that same algorithm; only `commit` publishes it. Failed preparations
and `discard` preserve the committed state. One private session token can be
pending at a time. Duplicate requests reuse parsed/generated candidates and
preserve the stylesheet revision when CSS is unchanged. Input, virtual assets,
candidate count, generated AST depth, output CSS, queue bytes, and logical cache
bytes are bounded. Logical cache bytes estimate retained candidate structures and
text; they are not an allocator or browser memory measurement. Virtual imports resolve only against the supplied asset bundle; the compiler
does not access a filesystem or network. Inline `@source` expansion is bounded
by the same candidate, queue, depth and cache limits.

`Compiler::source_metadata` exposes ordered path-based `@source` patterns and
exclusions, plus the selected root: automatic (`null`), disabled (`"none"`) or
an explicit `{base, pattern}`. Bases come from the entry's directory and virtual
import contexts; media `source(...)` retains the declaring file's base for its
selected utilities. Patterns are kept as authored, without filesystem resolution.
The WASM facade returns this information as JSON through `source_metadata`.
Candidate commits do not change it, and retained metadata is included in the
cache budget. With the optional `scanner` feature, `Compiler::scanner(base, tree)`
constructs the same root-plus-explicit-source selection used by the fixed CLI.
The caller supplies either a virtual tree or the existing native source backend.
Candidates and variables returned separately by Scanner must both enter
`prepare_add`; `--variable` inputs retain theme declarations without generating
utility selectors or GPU programs. These dependencies obey commit/discard and
candidate/cache/output quotas.

With both `compiler` and `scanner` facade features enabled,
`CompilerSession.prepare_content` accepts a JSON array of `{content, extension}`
and `prepare_sources` accepts a complete `{root, base, entries}` virtual snapshot.
Entries use `kind` (`directory`, `file`, `symlink`, `unavailable`) and `path`;
files also require `content` and integer `revision`, and symlinks require `target`.
Raw text bypasses CSS file discovery; snapshots obey CSS paths, ignore files and
exclusions. Missing supplied information is an explicit scan error. Snapshots
are rescanned and released after extraction; compiler candidates accumulate,
including after file removal. The facade stages ordinary preparation tokens,
so unsuccessful installation can discard and retry the same input. Source
bytes, candidate counts, path depth and virtual-tree storage are bounded before
publication. A scanner-disabled facade returns `UnsupportedFeature` for both
methods. Production example bundling builds compiler-only Workers by default;
`build-web --utility-scanner` explicitly includes extraction. The SDK exposes
`scan_content` and `scan_sources` through the same bounded installation queue,
and successful candidate/variable inputs survive Worker recovery without
retaining raw source. See the [runtime utility guide](../../../../docs/utility.md).

`Prepared::ast()` exposes the staged tree for scope transformation before an
installation is committed. GPU declarations and conditions are constructed by
the same emitters as CSS. Shared request/report and GPU types live in the
lightweight `protocol` and `program` modules; compiler reexports preserve their
original API paths. `upstream/tailwind/PROVENANCE.json` records source digests,
registry provenance, copied assets, and the MIT license.

The native regression suite and `compiler-slice` conformance gate compare the
frozen matrices in [the fixture guide](../../../../tests/utility/compiler/README.md).
The gate builds the real single-threaded WASM facade and runs the same CSS,
parser, error, incremental and quota checks in Node with scanner disabled.
Browser runtime, CSSOM installation and actual GPU drawing have separate
runners; neither host tests nor Node WASM tests replace those checks.

Arbitrary-value validity now uses one UTF-16 unit scanner. The Text entry borrows
its units and the scalar convenience entry feeds the same algorithm; lone units
remain opaque. It retains the fixed upstream Uint8Array(256) overflow behavior.
Five native tests cover all 65,536 units, scalar adapter equality, whitespace,
brackets, quotes, escapes and the 255/256/257/258 depth boundary. This foundation
is also used by the canonical Text candidate parser and public object helpers.

Animation usage tracking retains Text names and splits ECMAScript whitespace
and commas. It preserves non-scalar theme keyframes and distinguishes FEFF from
U+0085. Two native tests and the actual WASM animation runner compare direct and
theme-variable references, distinct lone/replacement names and scalar controls.

Setup and build now have explicit rendering phases. Setup decodes @theme keys
and substitutes functions/@apply; ordinary variable/keyframe usage analysis runs
at build. Features.None preserves the original input without that analysis.
The error gate compares all 61 frozen error phases, names, prototypes and message
units, plus failed-build/source-map recovery. Full upstream conformance remains
pending alongside the complete object graph and allocator audit.

The family payload gate adds 560 exact production-oracle CSS cases across 22
paths. Scalar class names exercise ordinary, inline and reference Theme values
with lone, paired, reversed, replacement and literal-escape units. Shadow and
drop-shadow cases also cover JavaScript replacement-string dollar markers.
The native test and `tests/utility/compiler/run-utf16-families-wasm.mjs` compare original output units;
`node tests/utility/compiler/utf16-families.mjs --check` verifies fixture contents and fixed source digests.
Container sorting and numeric/color classifiers use explicit JavaScript unit,
whitespace and case rules. This gate is separate from the 486-case candidate
input matrix and does not establish complete temporary allocation budgets.
Shadow replacement preflights the complete UTF-16 splice before allocating
its output. The explicit CSS quota includes joined shadow segments and fixed
wrappers; surrogate pairs formed between chunks retain their four-byte logical
cost. The actual WASM budget gate checks three expansion allocation witnesses,
108 exact public CSS boundaries and 39 recoveries. Full family scratch, cache and
crossing allocation conformance remains pending.

The current native compiler regression run passes 364 tests. Actual WASM runs
pass the original 486-case matrix (425 CSS results, 61 error phase/message
comparisons and 111 theme entries), all 111 theme CSS cases, and the additional
49 candidate/28 variant gate with 92 identity observations and seven AST builds.
Both [the full-input runner](../../../../tests/utility/compiler/run-utf16-input-wasm.mjs)
and [the candidate runner](../../../../tests/utility/compiler/run-utf16-candidates-wasm.mjs) are now included
in `compiler-api`. The existing public API gate again passes its unchanged
4 MiB cache limit, 21,771 class completions and 37 boundary observations.
These results cover finite inputs; they do not close the remaining M4–M7 work.
See the [M4 record](../../../../docs/validation/wasm-tailwind/M4.md) for artifacts
and the earlier RED observations.

Initial utilities, sources, media options, themes, custom variants and legacy
variant headers share one walk over the raw caller graph in `registration.rs`.
Theme replacement and directive removal happen in encounter order, including
the completed prefix before an error. Custom utility and variant bodies are
captured there. Functional utility normalization now updates the same raw
Current after the compatibility boundary and before deferred bodies are lowered.
JavaScript configuration is rejected on that raw root before normalization,
including directives inside still-attached utility bodies. Production setup no longer uses the
typed theme/custom variant collectors or retrospective initial theme mutations.
Final theme publication also runs on that raw Current before lowering, using the
original token values. The generated declarations retain their identities through
raw function substitution before typed lowering. The caller root and hoisted
keyframe tail are walked in sequence using the same Current and allowance; their
completed assignments survive later semantic failures. Utility completion metadata
is captured during normalization, before spacing functions are substituted. Rendering
tracks dependencies from selected theme tokens, while published children remain
available for setup substitutions.
The API gate compares 48 scenarios through both synchronous and asynchronous
entries (96 comparisons), plus four supplementary media/source comparisons,
including setter traces, retained aliases, rejection and settled graphs.
Import-time typed conversion, native adaptation, complete allocation admission
and host projection still need the remaining ownership and conformance work.
Authored variants now use captured receivers in initial walk order on the same
raw Current, before final lowering. The existing variant kernel wraps an opaque
caller body; generated wrappers reconnect its exact array. Custom slot substitution
keeps fresh template arrays and mutates caller resources at their actual positions.
Only the selected custom template is read from Current when it is cloned; there
is no global template refresh or Current clone per application. The old all-header
validation and child-first typed transform are removed. The default API gate adds
28 complete graph comparisons for ordered variants, nested/duplicate aliases,
live templates, extra fields, slot resources, shared declarations, raw theme values
and completed function effects. The later setup function Edits owner and its
separate failure journal are removed; theme preparation only derives token output
values and dependencies, without substituting keyframes again. Import-time typed
conversion, native adaptation, the remaining apply handoff and full input/host
allocation accounting still prevent a complete single-owner claim.

Public AST virtual imports now bind the actual parser nodes to native origins,
without a second parse or lexical Source sidecar. Public imports have fresh node
identities; when present, their Sources are fresh and src/dst are distinct tuples
sharing that Source.
Ordered context replacements, media options and moved apply parents use the live
caller arrays. Source-location pointer caches retain and charge their native owners.
The default compiler-api gate checks the complete 27-case input graph and the
21-case import matrix plus one array-order trace across 107 phases. Full loader,
accessor, failed-setup and allocation/performance conformance remains pending.

`compile_ast_setup` exports `AstSetupOutcome` and `AstSetupFailure` for hosts that
need the recorded caller effects on semantic errors. Ordinary/custom function
errors retain their completed assignments and preceding theme/registration
effects; the WASM constructor preflights the failure graph and crossing reserve
before replay. A host setter error wins over the later semantic error; quota
refusals leave the caller graph intact. The legacy `compile_ast` Result interface
is preserved. The finite function-phase matrix passes twelve semantic failures
and four original host exceptions. The import failure gate now passes fifteen
cases, five original host exceptions and four quota refusals without caller
writes. External loader callbacks and complete setup-phase coverage remain pending.
