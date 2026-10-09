import { commit, read, sha256, sourceFiles, sourceLocator } from "./upstream.mjs"

const pending = "in-scope-pending-rust-verification"
const configPath = "packages/cn/src/compiler.ts"
const typePath = "packages/cn/src/types.ts"
const fields = (path, name) => {
  const text = read(path)
  const match = new RegExp(`(?:interface|type) ${name}[^\\n]*\\{([\\s\\S]*?)\\n\\}`).exec(text)
  if (!match) throw new Error(`missing type ${name}`)
  const bodyStart = match.index + match[0].indexOf("{") + 1
  return [...match[1].matchAll(/^  ([\w]+)\??:/gm)].map((fieldMatch) => ({
    name: fieldMatch[1], source: { path, line: text.slice(0, bodyStart + fieldMatch.index).split("\n").length }, status: pending,
  }))
}

export function inventory(modules, config) {
  const pkg = JSON.parse(read("packages/cn/package.json"))
  const entryNames = { ".": "index", "./tables": "tables.generated" }
  const entrypoints = Object.keys(pkg.exports).map((entry) => {
    const name = entryNames[entry] ?? entry.slice(2)
    const path = `packages/cn/src/${name}.ts`
    const kind = ["vite", "next"].includes(name) ? "tool-integration-adapter" : "core-semantics"
    return {
      packageEntry: entry === "." ? "cn" : "cn/" + entry.slice(2),
      source: { path, line: 1 }, kind,
      status: pending,
      runtimeExports: Object.keys(modules[name]).sort().map((exportName) => ({ name: exportName, status: pending })),
      typeExports: [...read(path).matchAll(/export (?:interface|type) ([\w]+)/g)].map((match) => ({
        name: match[1], source: sourceLocator(path, match[0]), status: pending,
      })),
      // Keep the exact declaration for re-exported type lists; no type gets
      // silently removed because it has no JavaScript runtime representation.
      typeReexports: [...read(path).matchAll(/export type \{[\s\S]*?\}(?: from ["'][^"']+["'])?/g)].map((match) => ({
        declaration: match[0], source: { path, line: read(path).slice(0, match.index).split("\n").length }, status: pending,
      })),
    }
  })
  const testFiles = sourceFiles().filter((path) => /^packages\/conformance\/tests\/.*\.mjs$/.test(path))
  return {
    schemaVersion: 1, upstream: { repository: "https://github.com/shadcn-ui/cn", commit, packageVersion: pkg.version },
    statusMeaning: "All rows track Rust/WASM compatibility. Running the JS oracle does not change these statuses.",
    entrypoints,
    config: {
      full: fields(configPath, "CnConfig"), extension: fields(configPath, "ConfigExtension"),
      extensionGroups: fields(configPath, "ConfigExtensionGroups"),
      classGroupDefinition: { source: sourceLocator(configPath, "export type ClassGroupDef"), status: pending,
        forms: ["literal", "nested prefix object", "$v marker", "$t theme marker", "validator function", "function with isThemeGetter=true"] },
      inputForms: ["undefined default", "extension", "full config", "default-config transform callback"],
      validators: Object.keys(modules.config.validators).map((name) => ({
        name, marker: modules.config.validators[name], source: sourceLocator("packages/cn/src/config.ts", `  ${name}:`), status: pending,
      })),
    },
    tables: fields(typePath, "Tables"), engineOptions: fields(typePath, "EngineOptions"), engineMethods: fields(typePath, "Engine"),
    defaultConfig: {
      classGroups: Object.entries(config.classGroups).map(([name, definitions]) => ({ name, definitions, status: pending })),
      theme: Object.entries(config.theme).map(([name, definitions]) => ({ name, definitions, status: pending })),
      conflictingClassGroups: Object.entries(config.conflictingClassGroups).map(([name, targets]) => ({ name, targets, status: pending })),
      conflictingClassGroupModifiers: Object.entries(config.conflictingClassGroupModifiers).map(([name, targets]) => ({ name, targets, status: pending })),
      orderSensitiveModifiers: config.orderSensitiveModifiers.map((name) => ({ name, status: pending })),
      postfixLookupClassGroups: (config.postfixLookupClassGroups ?? []).map((name) => ({ name, status: pending })),
      source: { path: "packages/cn/src/default-config.generated.ts", line: 1 },
    },
    build: {
      options: ["cwd", "content", "out", "safelist", "config", "tokens", "full", "css"].map((name) => ({
        name, source: sourceLocator("packages/cn/src/build.ts", `    ${name}?:`), status: pending,
      })),
      resultFields: ["outPath", "source", "changed", "tokens", "config", "fullConfig", "scannedFiles", "cssFiles", "usedGroups", "totalGroups", "warnings"],
      builder: ["run", "changed", "last"],
      cli: {
        source: { path: "packages/cn/bin/cn.mjs", line: 1 }, status: pending,
        flags: ["--content", "-o", "--out", "--safelist", "--config", "--tokens", "--full", "--css", "--no-css", "--cwd", "-q", "--quiet", "-h", "--help", "-v", "--version"],
        classification: "argument parsing and output are tool integration; build/subset/compile semantics remain core requirements",
      },
      subsetContract: {
        source: sourceLocator("packages/cn/src/build.ts", "// Subsetting contract"), status: pending,
        semantics: "Scanned classes merge identically to full tables; unscanned classes may pass through unmerged. Dynamic expansion is a separate added API.",
      },
    },
    upstreamTests: testFiles.map((path) => ({
      path, sha256: sha256(read(path)), status: pending,
      sections: read(path).split("\n").flatMap((text, index) =>
        /^\s*\/\/ (?:[-]+ |\d+\.|[A-C]\. |Without|Only|Logical|Legacy|The audited|A side|`start`)/.test(text)
          ? [{ label: text.trim().slice(3), source: { path, line: index + 1 }, status: pending }] : []),
      literalAssertionLabels: [...read(path).matchAll(/\b(?:expect|hand|compare)\(\s*["']([^"']+)["']/g)].map(([_, label]) => label),
      coverageNote: "The complete locked file is the authority, including generated loops, fuzz, timeout children and dynamic assertion labels; listed labels are navigation aids, not coverage claims.",
    })),
    behaviorContracts: [
      ["join-shapes", "packages/cn/src/engine.ts", "const resolveValue", "cn/clsx add number and dictionary handling; twJoin/twMerge accept strings and nested arrays. Runtime JS array-like values follow the original engine, independent of public TypeScript narrowing."],
      ["mutable-arguments", "packages/cn/src/engine.ts", "// arg-identity cache", "Mutable arrays/dictionaries cannot be cached by object identity; replay changed values."],
      ["lazy-config", "packages/cn/src/config.ts", "const resolveConfig", "Undefined, transform function, complete config, and extension forms retain their resolution behavior and exception boundaries."],
      ["config-order", "packages/cn/src/compiler.ts", "export const mergeConfigs", "Clone the base, apply prefix, then override fields, then append extensions in definition order."],
      ["callback-registry", "packages/cn/src/compiler.ts", "const resolveValidator", "Function validators use stable registry identities; unknown markers throw. Theme-getter functions remain distinct from predicates."],
      ["subset-parse", "packages/cn/src/compiler.ts", "export const subsetConfig", "Classify full candidates with prefix, modifier/bracket/parenthesis, important and slash handling; keep reached groups and directed conflict metadata."],
      ["table-encoding", "packages/conformance/tests/table-encoding.mjs", "const configs", "Default, subset, empty, empty-tail, duplicate, escaped and Unicode literal tables must match in-memory compilation for JS and TS emissions."],
      ["callback-emission-error", "packages/cn/src/compiler.ts", "if (m.customNames.length > 0)", "Custom validator functions run in-memory; serialized module emission explicitly throws."],
      ["build-filesystem", "packages/cn/src/build.ts", "const walk", "Ignored directories can be explicitly opted into except .git; follow symlinks, avoid ancestor cycles, count unreadable paths."],
      ["build-matcher", "packages/cn/src/build.ts", "export const createContentMatcher", "Pure changed-path matching follows literal/glob/ignored-directory rules and rejects paths outside cwd."],
      ["build-token-cap", "packages/cn/src/build.ts", "export const extractTokens", "Candidate extraction is over-approximation; count tokens skipped above the fixed 8192-character cap."],
      ["build-options", "packages/cn/src/build.ts", "export const build", "Defaults, tokens/full scan suppression, CSS theme registration before custom config, safelist, source/self-symlink exclusion and output extension affect build semantics."],
      ["build-noop", "packages/cn/src/build.ts", "// Leave an identical module untouched", "Identical source leaves the output file untouched and reports changed=false; warnings and errors remain observable."],
      ["builder-coalescing", "packages/cn/src/build.ts", "export const createBuilder", "Coalesce concurrent runs; class edits reaching dropped groups rebuild, existing groups/deleted sources do not; CSS theme dependencies trigger rebuild."],
      ["css-theme-discovery", "packages/cn/src/theme-css.ts", "export const themeFromCss", "Read @theme namespaces/reset/removal/subproperties, relative CSS imports and cycles, and report every file; package imports are not followed."],
      ["vite-hooks", "packages/cn/src/vite.ts", "export const cn", "Build tables before resolution, update from changed source files, and ignore delete events."],
      ["next-wrapper", "packages/cn/src/next.ts", "export const withCn", "Build before returning object/function config; development watches recursively with nonpersistent watcher and reports changed-file errors."],
      ["cli-errors", "packages/cn/bin/cn.mjs", "const fail", "CLI argument parsing and display adapt the core build; errors print cn: and exit nonzero; help/version remain tool concerns."],
      ["engine-bounds", "packages/conformance/tests/bounds.mjs", "// Three hazards", "Wide conflict fan-out, arbitrary group IDs above 8191 and raw-prefix reset must retain correctness and termination."],
      ["engine-epoch", "packages/conformance/tests/hardening.mjs", "// Regression guard", "Epoch int32 wrap and epoch zero require explicit seeded-state tests under a timeout."],
    ].map(([id, path, needle, semantics]) => ({ id, source: sourceLocator(path, needle), semantics, status: pending })),
    remainingVerification: [
      "Run every locked upstream conformance case through Rust and WASM; current fixtures are a deterministic initial corpus, not full V-2.",
      "Callback realm ownership/exceptions and general JS input crossing require the actual WASM facade.",
      "Native compiler table/source, build warnings/errors and plugin/CLI adapter mapping need per-entry equivalence evidence.",
      "Bounds/hardening/cache/property fuzz remain pending Rust verification even if the reference execution succeeds.",
    ],
  }
}
