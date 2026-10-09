# Initial directive registration: native grounding

Scope: architect Phase A/how code exploration only. References below are current working-tree anchors, with one explicitly identified RED artifact produced by the parent task and read here. This explorer ran no tests/builds/browser checks/services and edited no production files. The plan still records 3/7 completed milestones (`docs/plan/WASM-TAILWIND.md:37`); this document does not establish additional coverage. The local reference package declares Tailwind `4.1.13` (`ref/tailwindcss/packages/tailwindcss/package.json:3`), also pinned in `mise.toml:4`.

## Components Found

| Component | Exact anchor | Responsibility |
| --- | --- | --- |
| `Compiler::new_with_ast` | `crates/rustify-style/src/compiler/mod.rs:208` | Owns the parsed/import-expanded native AST and sequences registration, preparation, variant application, functions, apply, and initial rendering. |
| `imports::expand_with_base`, `transform_media`, `media` | `crates/rustify-style/src/compiler/imports.rs:348`, `:247`, `:123` | Expands imports, then independently rewrites every eligible media wrapper and propagates its directive options. |
| `sources::Selection` and `extract` | `crates/rustify-style/src/compiler/sources.rs:8`, `:198` | Collect inline candidates, exclusions, source metadata, and the first eligible utilities source root; remove source directives and duplicate/reference utilities. |
| `sources::select_utilities` | `crates/rustify-style/src/compiler/sources.rs:398` | Separate recursive filtering of tailwind nodes and consumption of import-importance markers. |
| `CustomVariants::extract` / private `definition` | `crates/rustify-style/src/compiler/custom_variants.rs:43`, `:234` | Validate/remove custom variant definitions, convert root legacy variants, preserve registry order across overrides, and build templates. |
| `CustomUtilities::extract_marked` / `extract_inner` | `crates/rustify-style/src/compiler/custom_utilities.rs:172`, `:179` | Validate/remove utility definitions, eagerly normalize functional bodies, record completion data, and retain private position markers. |
| `Theme::extract` | `crates/rustify-style/src/compiler/theme.rs:45` | Extract tokens/keyframes and prefix, validate theme bodies, and retain the first theme position as a private anchor. |
| `variants::authored_headers`, `validate_authored`, `apply_authored` | `crates/rustify-style/src/compiler/variants.rs:507`, `:568`, `:579` | Collect remaining application headers, validate against the completed theme/variant registry, then transform applications. |
| `origin::Edits` / `Snapshot` | `crates/rustify-style/src/compiler/origin.rs:115`, `:304` | Preserve caller identity and current public views while recording field mutations; currently used for utility normalization and function substitution. |
| `AstNode` children helpers | `crates/rustify-style/src/compiler/ast.rs:327`, `:339` | Uniform children access for Rule, AtRule-with-body, Context, and AtRoot. Context transparency remains a visitor responsibility. |
| Upstream `parseCss` / `walk` | `ref/tailwindcss/packages/tailwindcss/src/index.ts:137`, `src/ast.ts:136` | After import substitution, one mutation-aware, preorder walk dispatches all initial directives; registration closures execute later. |

## Flow

### Native sequence and stage boundaries

1. `new_with_options` parses input with optional source identity and calls `new_with_ast` (`mod.rs:194–205`). The latter checks depth/assets and calls `imports::expand_with_base` (`:216–229`). Import expansion remains an earlier stage in both native and reference: upstream substitutes imports before its initial directive walk (`index.ts:147–149`).
2. `expand_with_base` invokes the optional public `media()` hook and then native `transform_media` (`imports.rs:376–379`). Native media processing therefore finishes across the whole tree before any source, utility, custom variant, or theme validation.
3. `sources::extract` runs over the expanded tree (`mod.rs:231`). Its borrowed context carries `base`, `source_base`, `nested`, and `reference` (`sources.rs:203–209`). It removes `@source`, removes duplicate/reference utilities, parses the selected utilities source root, and resets its params to `utilities` (`:243–275`, `:294–360`). Source-owned storage is charged immediately afterward (`mod.rs:232–249`).
4. `select_utilities`, custom variant extraction, authored variant header collection, custom utility extraction, and theme extraction run in that order (`mod.rs:250–256`). These are separate traversals, not one source-order dispatcher.
5. The optional `compatibility()` hook runs only after all those extractions succeed (`mod.rs:257–258`). Feature probes inspect the residual tree and extracted bodies (`:260–280`). Theme budgets subtract source/custom storage; the custom variants and source exclusions are attached to Theme (`:291–298`).
6. `theme.prepare_functions` executes before authored variant validation/application (`mod.rs:299–302`). That method processes theme token values, then keyframes (`theme.rs:273–302`). This is outside the initial registration walk and remains a separate ordering issue from this exploration's central question.
7. Authored variants are applied to the residual AST, then separately to custom bodies (`mod.rs:301–302`; `custom_utilities.rs:403–410`). Directive validation and `custom.prepare_functions` follow (`mod.rs:303–304`). The function visitor follows private definition markers to substitute custom bodies at their authored positions (`custom_utilities.rs:427–446`).
8. On a function-stage failure, the optional hook receives theme, normalization mutations, and completed function mutations (`mod.rs:305–314`). Normalization mutations are only retrieved here or on later success (`:315–316`); initial source/custom/theme extraction uses `?` without an equivalent early-failure callback (`:231`, `:251`, `:255–256`). Apply has its own failure hook (`:318–333`).

### Fixed upstream initial walk

The walk is preorder, visits siblings in array order, merges Context values without changing `parent`, and does not call the visitor for Context itself (`ast.ts:150–162`). `replaceWith` modifies the current array immediately; Continue revisits replacements, Skip advances past replacements and suppresses child traversal (`:172–204`, `:210–220`). These semantics are part of observable ordering, not merely iteration mechanics.

At each at-rule, `parseCss` tests: utilities (`index.ts:169`), utility (`:219`), source (`:253`), variant/legacy conversion (`:306`), custom variant (`:347`), media (`:442`), theme (`:529`). Most nodes match one branch; converting legacy `@variant` changes the name and deliberately falls through to custom variant handling in the same callback (`:309–347`).

- Utilities: ignore later selected nodes before examining their options, ignore reference nodes without selecting them, then parse source options and remember the node (`index.ts:173–215`).
- Utility: validate nesting, body nonemptiness, then name via `createCssUtility`; queue a closure and continue the initial walk into its body (`:219–250`). There is no Skip or early return at the end of this branch.
- Source: reject nonempty body before nesting, parse optional `not`/`inline`, validate quotes, expand/register, then remove the node (`:253–302`).
- Custom variant: reject nesting first; remove its AST position **before** validating its name/body/selector; queue the registration closure and return, so the removed body is not visited by the initial walk (`:347–439`).
- Media: handle option tokens in media-param order; perform targeted child walks for source/theme/prefix propagation; set global importance, wrap reference context, preserve unknown tokens or flatten the wrapper (`:442–525`). Returning Continue causes flattened replacements to be revisited through the shared walk.
- Theme: parse all options, select the last prefix, apply reference context, validate nonempty prefix, collect tokens/keyframes, replace first position with the theme rule or remove later positions, then Skip (`:529–580`). Theme-body collection skips keyframes children (`:550–552`); context wrappers inside a theme body are transparent through `walk`.

After that entire walk succeeds, upstream builds its design system, runs compatibility hooks, executes custom variant closures, then executes custom utility closures (`index.ts:584–614`). It emits theme output, applies collected variant nodes in collection order, substitutes functions, substitutes apply, and finally removes utilities (`:616–685`).

### Functional normalization happens after initial validation upstream

`createCssUtility` recognizes a valid functional name and returns a closure (`utilities.ts:5744–5748`, `:5775`). The closure's declaration walk parses/normalizes `--value` and `--modifier` and assigns `child.value` (`:5796–5801`, `:5853–5874`, `:5925`). This runs at `index.ts:612–613`, after every initial theme/source/custom-definition error has been ruled out and after custom variant registration.

Native functional extraction instead runs `normalize` immediately (`custom_utilities.rs:256–257`), before completing even later utilities in that extraction and before `Theme::extract` (`mod.rs:255–256`). Completion extraction also runs eagerly immediately after normalization (`custom_utilities.rs:259–260`), parsing declaration values again (`:88–104`). Native normalization uses `origin::Edits` and produces its mutation snapshot only if collection completes successfully (`:307–318`). Calling this eager normalizer from a consolidated initial visitor would not reproduce the upstream stage.

## Boundaries

### Existing handler seams, without proposing new interfaces

| Existing code | Reusable work | Coupled traversal/stage work that cannot be reused unchanged |
| --- | --- | --- |
| `sources.rs:70` `expand`; `:249–273` root parsing; `:294–359` source handling | Quote validation, inline expansion, metadata population, and existing limits | Private read loop owns removal, utilities selection, context propagation, and definition-body skips; source errors currently outrank earlier utility/theme errors. |
| `imports.rs:132` `append_theme`; `:160` `append_source`; `:185–210` media option dispatch | Existing propagation rules and token processing | Both helpers are nested inside `media`; `media` recursively calls `transform_media` before returning (`:223`). It also encodes importance as a later marker (`:211–221`), rather than updating registration state at the media visit. |
| `custom_variants.rs:234` `definition` | Name/selector/body validation and template construction | Collector handles nesting, removal, legacy classification, override indexing, and final storage check (`:63–103`). Definitions own the detached body. |
| `custom_utilities.rs:202–255` name/body validation and `:270–276` definition storage | Validated ASCII names, root index storage, and body ownership | Functional normalization/completion (`:256–263`) are mixed into collection. Position markers (`:277–287`) and provenance snapshot finalization (`:307–318`) serve later function passes. |
| `theme.rs:119–136` prefix; `:143–235` token/keyframe handling | Last-prefix selection, existing message construction, token reset/default/order semantics | Theme's reader owns recursive visitation and first-anchor replacement (`:237–256`); direct child loop differs from upstream transparent Context walk. |
| `variants.rs:507` `authored_headers` | Header collection intent | It is another whole traversal. Applications must remain deferred until all custom variants are known; header collection and definition removal need the same legacy classification. |

The registration outputs already consumed by `new_with_ast` are source Selection, importance, CustomVariants, CustomUtilities, Theme, and authored headers (`mod.rs:231–256`). All are session configuration, independent of later candidate revisions. Imports/public setup, origin views, feature probes, function/apply mutations, and resource limits are adjacent boundaries that a shared initial traversal must still respect.

## Non-Obvious Things (Gotchas)

### Source order and failure prefixes

- Native's type-based phase order can report a later source error before an earlier empty utility error. Code-inferred example: `@utility foo {} @source unquoted;`. Native runs `sources::extract` before utility validation (`mod.rs:231`, `:255`); upstream visits and rejects the empty utility first (`index.ts:224–227`). This is an inferred counterexample, not a test result.
- Similarly, an earlier invalid theme prefix can lose to a later invalid custom variant name in native because custom variants are extracted before themes (`mod.rs:251`, `:256`), whereas upstream validates the first theme when encountered (`index.ts:536–543`).
- Native media rewriting is globally earlier than these errors (`imports.rs:379`). Upstream only propagates a media wrapper when the shared walk reaches it (`index.ts:442`); an earlier error prevents later wrapper effects.
- The parent's actual-WASM RED confirms these phase mechanisms. `docs/validation/wasm-tailwind/utf16-families-registration-detailed-before-red.log:1` contains 36 cases/72 comparisons and 68 failure records; counting those records yields 8 wrong-error comparisons, 64 prefix mismatches, and 0 epoch mismatches. `utility-empty-before-source-bad` expects the empty utility error and gets the source quote error; `theme-prefix-before-variant-name` expects prefix rejection and gets variant-name rejection; `source-bad-before-media-reference` expects the source quote error and gets the media theme(reference) error. The same summary also shows later-media params setters firing after an earlier source failure where the expected trace is empty. The harness constructs these orders at `tests/utility/compiler/run-public-ast-registration-wasm.mjs:38–50` and concrete nodes at `:77–92`; it checks pinned source hashes at `:13–16`. The log explicitly retains `fullRegistrationConformance: "pending"`. This confirms the listed mechanisms within that bounded RED corpus, not complete conformance.
- Source/custom/theme collectors use `mem::take` and only reinstall their output vector on success (`sources.rs:217–218`, `:378`; `custom_variants.rs:50–51`, `:94`; `custom_utilities.rs:192–193`, `:304`; `theme.rs:79–80`, `:256`). On `?` failure, that local native subtree is not restored. It is unsuitable as the mechanism for a caller-visible partial mutation prefix; public caller effects require their own correctly ordered records/views.
- Upstream custom variant removal precedes its validation (`index.ts:353–376`), while theme replacement happens only after its body validation (`:546–580`). A uniform "validate then remove" policy would change failed AST prefixes.

### Context, reference, and definition bodies

- Native sources treats Context as transparent for nesting, overrides nonempty base, inherits source_base, and ORs reference (`sources.rs:219–239`). Custom variant/utility collectors likewise recurse through Context without setting nested (`custom_variants.rs:85`, `custom_utilities.rs:293–294`). Other children are nested. Upstream Context uses object spread override rather than reference OR (`ast.ts:158`): explicit nested `reference: false` can clear an inherited true in a caller-created AST; native collectors shown here do not model that override.
- Reference suppresses utilities and theme-token emission eligibility, but does not suppress custom registration or source collection: native utility selection checks reference (`sources.rs:245`), Theme marks tokens reference (`theme.rs:226–227`), while custom collectors only inspect nesting. Upstream has the corresponding reference check for utilities/themes (`index.ts:182`, `:532`) and none in utility/source/custom-variant registration branches.
- Root legacy `@variant` means a definition only if no body/empty body or any descendant `@slot`; a nested `@variant` remains an application even when slot-bearing (`custom_variants.rs:63–67`, `:207–210`; upstream `index.ts:309–343`). Existing media/source passes duplicate this legacy classification to skip definition bodies (`imports.rs:255–260`; `sources.rs:284–292`, `:407–410`).
- `@theme` and `@custom-variant` bodies, plus root legacy definition bodies, are opaque to the native media/source passes at the above skip points. Upstream removes custom definitions and skips theme bodies after dedicated theme collection, so generic initial descent also avoids those bodies.
- Utilities are different: upstream continues its initial walk through their bodies (`index.ts:249–250`; default Continue in `ast.ts:192`, descent `:213–220`). Native detaches the entire utility body before theme extraction (`custom_utilities.rs:271–289`; `mod.rs:255–256`). Thus themes inside a utility body can be encountered upstream but cannot be seen by the later native theme reader. This is a concrete code-path difference that a shared initial traversal must account for; no executable fixture was run.
- Theme body collection accepts Context-wrapped declarations upstream because `walk` is transparent (`ast.ts:157–161`, `index.ts:547`). Native `theme.rs:143–162` iterates immediate children and rejects a Context child as invalid. Keyframe bodies are opaque to initial theme validation in both (`theme.rs:144–147`, `index.ts:550–552`).
- Native private utility markers require `AtRoot { origin: None }`, an exact single comment, and a valid index (`custom_utilities.rs:334–357`). The caller-origin guard at `:348` matters: public AtRoot input is not a trusted registration slot.

### Prefix and utility selection

- Upstream parses all theme options and chooses the last recognized `prefix(...)` before validation (`index.ts:84–102`, `:530–543`). Native explicitly does the same last-prefix selection (`theme.rs:119–136`). Empty last prefixes do not reset an existing prefix in either implementation (upstream truthiness at `:536`; native filter at `:128`). A shared walk must retain this within-node rule, while processing distinct theme blocks in encounter order.
- Native theme option splitting uses all Rust `char::is_whitespace` (`theme.rs:111–118`); upstream calls `segment(params, ' ')` (`index.ts:88`). These are visibly different algorithms; parity for non-space separators was not established here.
- Upstream media prefix/theme propagation appends options to each encountered theme and skips its body (`index.ts:471–502`); native `append_theme` uses `continue` after append (`imports.rs:142–146`). These child subwalks are still needed even in a single *initial* outer traversal; "one traversal" cannot mean deleting the specialized subtree work required by fixed behavior.
- Native `sources::extract` selects only `@tailwind` params starting with `utilities` (`sources.rs:244`), but its later `select_utilities` filters **any** `@tailwind` (`:419–423`). Upstream only selects/removes utility-looking tailwind rules in the initial walk (`index.ts:169–176`). Arbitrary non-utilities directives therefore need separate classification rather than merging those two existing native conditions blindly.

### Allocation and limits

- Every consuming native pass above constructs a new `Vec` while consuming the old vector allocation. A shared outer traversal can reduce repeated container rebuilding, but this is not evidence of measured peak reduction. Moves do not clone all AST payloads.
- Specific actual copies remain: `append_source` clones its selected tailwind node (`imports.rs:166`); media params are cloned (`:273`); theme keyframes and token values are cloned (`theme.rs:146`, `:220–223`); headers clone params (`variants.rs:514`). `Text` is an owned `Vec<u16>` with derived Clone (`crates/rustify-style/src/text.rs:7–9`), so these Text clones duplicate buffers.
- Normalization reparses values, serializes argument ASTs, and builds warning copies (`custom_utilities.rs:741–749`, `:765–785`, `:797–801`); completion then reparses each declaration (`:95`). Those costs occur per functional definition and are distinct from outer traversal count.
- `Edits::from_nodes` recursively visits origins and imports existing view books (`origin.rs:121–141`); finishing creates an Arc-backed snapshot and binding walks nodes/definition bodies (`:295–326`; `custom_utilities.rs:310–317`). Repeating full snapshot rebuilding for each directive would add traversal/allocation work rather than exploit the current pass-wide seam.
- Custom variants/utilities only check their measured owned storage after collection (`custom_variants.rs:101–104`, `custom_utilities.rs:319–324`). Combined setup budget is calculated later (`mod.rs:291–295`, `:354–367`). These final storage totals do not prove a bound on simultaneous old/new vectors, normalization temporaries, or snapshots. Native Theme's `owned_bytes` shown here accounts for token/order storage (`theme.rs:327–355`); it does not include its `keyframes` Vec in that method. Peak-budget correctness is an open validation item, not established by those checks.

## Files Read

- Context/configuration: `README.md` (first 180 lines), `PRODUCT.md`, `DESIGN.md` (first 120 lines), `package.json`, `mise.toml`, `Cargo.toml`, `docs/plan/WASM-TAILWIND.md` (first 56 lines and targeted searches).
- Explorer contract: `.agents/skills/how/references/explorer-prompt.md`.
- Native code: `crates/rustify-style/src/compiler/mod.rs` (entry/setup and directive validation); `imports.rs` (media/expansion); `imports/bundle.rs` (first 92 lines only); `sources.rs` (Selection and registration); `custom_variants.rs` (registration/definition/storage); `custom_utilities.rs` (definitions/completion/extraction/normalization/function preparation/storage); `theme.rs` (extraction/functions/storage); `variants.rs` (selected root/application sections); `ast.rs` (children/depth/printing); `origin.rs` (Edits construction/snapshot); `crates/rustify-style/src/text.rs` (first 117 lines).
- Reference code: `ref/tailwindcss/packages/tailwindcss/package.json` (version); `src/index.ts` (initial parse and post-walk setup); `src/ast.ts` (walk); `src/utilities.ts` (functional create/registration/normalization, 5720–5934).
- Parent-produced evidence: `tests/utility/compiler/run-public-ast-registration-wasm.mjs` (first 95 lines); `docs/validation/wasm-tailwind/utf16-families-registration-detailed-before-red.log` (first JSON line, parsed locally to extract bounded records/counts; no test rerun).

## Open Questions

1. This exploration did not trace public setup/replay implementation. Whether early source/custom/theme failures already have a public-side prefix mechanism elsewhere cannot be claimed from `PublicSetup` and the native call sites alone; native extraction `?` exits do not pass such a prefix through that trait here.
2. Identity aliasing, shared arrays, getters/Proxy traps, and source/destination fields require the other public-AST slice. Owned native `Vec<AstNode>` traversal cannot itself establish parity for those observations.
3. The parent's RED artifact confirms the named source/utility/theme/media order failures and bounded prefix graph differences. Other code-inferred examples, Context override behavior, utility-nested theme handling, whitespace segmentation, and normalization warnings were not executed or verified here.
4. Registration consolidation must not accidentally claim downstream parity: theme function preparation precedes variant validation natively (`mod.rs:299–300`), and native application is recursive postorder (`variants.rs:585–609`) versus upstream application over preorder-collected node references (`index.ts:333–342`, `:640–659`). These are adjacent order boundaries, not resolved by the proposed exploration.
5. No timing, allocation count, or peak-memory experiment was performed. Existing storage measurements are concrete implementation evidence, not performance/peak proofs.
