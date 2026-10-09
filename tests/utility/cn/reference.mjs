import assert from "node:assert/strict"
import { existsSync, mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs"
import { resolve, relative } from "node:path"
import { pathToFileURL } from "node:url"
import { fixtureRoot, commit, load, read, json, sha256, sourceFiles, tableJson, assertLockedSource } from "./upstream.mjs"
import { inventory } from "./inventory.mjs"
import { runtimeCases, executeCase, recipeInput } from "./cases.mjs"

const write = process.argv.includes("--write")
if (!write && !process.argv.includes("--check")) throw new Error("Use --check or --write")
if (!write || existsSync(resolve(fixtureRoot, "source-lock.json"))) assertLockedSource()
const names = ["index", "engine", "tables.generated", "config", "compiler", "build", "vite", "next", "lite", "validators"]
const modules = Object.fromEntries(await Promise.all(names.map(async (name) => [name, await load(name)])))
const defaultConfig = modules.config.defaultConfig()
const checkedFiles = sourceFiles().filter((path) => path === "LICENSE" || path === "README.md" || path === "pnpm-lock.yaml" || path === "package.json" || path.startsWith("packages/cn/") || path.startsWith("packages/conformance/tests/"))
const lock = {
  schemaVersion: 1, repository: "https://github.com/shadcn-ui/cn", commit,
  packageVersion: JSON.parse(read("packages/cn/package.json")).version,
  provenance: "Commit specified by WASM-TAILWIND.md and the supplied downloaded tree. SHA-256 locks every inspected source and upstream test; this archive has no Git object database, so the commit identity is a supplied provenance claim, not independently proven here.",
  files: checkedFiles.map((path) => ({ path, sha256: sha256(read(path)) })),
}

const runtime = runtimeCases().map((test) => ({ ...test, expected: executeCase(test, modules) }))

const compilerCases = []
const empty = { theme: {}, classGroups: {}, conflictingClassGroups: {}, conflictingClassGroupModifiers: {}, orderSensitiveModifiers: [] }
const subsetTokens = ["p-2", "p-4", "px-3", "text-lg/7", "leading-6", "hover:p-2", "hover:p-4", "[color:red]", "[color:blue]", "custom"]
const configs = {
  default: defaultConfig,
  empty,
  literals: { ...empty, classGroups: { unicode: ["shape-😀", "shape-😁", "shape-é"], escaped: ['quote-"a', 'quote-"b', "slash-\\a", "slash-\\b"], emptySuffix: ["prefix", "prefix-more"], duplicate: ["same", "same"] } },
  subset: modules.compiler.subsetConfig(defaultConfig, subsetTokens).config,
}
for (const [id, config] of Object.entries(configs)) {
  const compiled = modules.compiler.compileToTables(config)
  const source = modules.compiler.compileToSource(config)
  const emitted = (await import("data:text/javascript;base64," + Buffer.from(source).toString("base64"))).default
  assert.deepEqual(emitted, compiled.tables, `${id}: emitted tables equal in-memory tables`)
  const emittedCn = modules.engine.createCn(emitted)
  const dynamicCn = modules.config.createCn(config)
  const mergeInputs = [...subsetTokens, "p-2 p-4", "px-3 p-4", "text-lg/7 leading-6", "shape-😀 shape-😁", "custom custom", "[color:red] [color:blue]"]
  const merges = mergeInputs.map((input) => {
    const expected = dynamicCn(input)
    assert.equal(emittedCn(input), expected)
    return { input, expected }
  })
  compilerCases.push({ id, input: config, expected: {
    stats: modules.compiler.compileStats(config), tables: tableJson(compiled.tables),
    sourceSha256: sha256(source), sourceBytes: Buffer.byteLength(source),
    tsSourceSha256: sha256(modules.compiler.compileToSource(config, { lang: "ts", banner: "// fixture banner" })),
    customValidatorNames: Object.keys(compiled.validatorImpls), merges,
  } })
}
const custom = recipeInput("predicate", modules.config)
const customFull = modules.config.mergeConfigs(defaultConfig, custom)
let emissionError
try { modules.compiler.compileToSource(customFull) }
catch (error) { emissionError = { name: error.name, message: error.message } }
assert.ok(emissionError, "custom callbacks must not be emitted as source")
compilerCases.push({ id: "custom-callback-not-serializable", recipe: "predicate", expected: { emissionError } })

const validatorInputs = ["", "0", "1", "-1", "1.5", "1/2", "50%", "2xl", "sm", "hero", "[13px]", "[length:var(--x)]", "[color:red]", "[1.5]", "[url(x)]", "[0_0_2px_black]", "(--x)", "(length:--x)", "(family-name:--x)", "[var(--x)]", "1e3", "Infinity", "NaN", "  "]
const validators = Object.keys(modules.config.validators).map((name) => {
  const cn = modules.config.createCn({ ...empty, classGroups: { probe: [{ probe: [modules.config.validators[name]] }] } })
  return { name, cases: validatorInputs.map((input) => ({ input, predicate: modules.validators[name](input), mergeInput: `probe-${input} probe-${input}`, expected: cn(`probe-${input} probe-${input}`) })) }
})
const compiledDefault = modules.compiler.compileToTables(defaultConfig)
assert.deepEqual(compiledDefault.tables, modules["tables.generated"].default, "shipped default tables equal current compiler output")
const engineCases = []
for (const cacheSize of [0, 1, 500]) {
  const engine = modules.engine.createEngine(compiledDefault.tables, undefined, { cacheSize })
  const sequence = ["p-2 p-4", "p-2 p-4", "px-2 p-3", "p-2 p-4", "[--x:1] [--x:2]", "hover:md:p-2 md:hover:p-4", " custom custom "]
  engineCases.push({ cacheSize, sequence: sequence.map((input) => ({ input, seenBefore: engine.seenBefore(input), mergeString: engine.mergeString(input), mergeUncached: engine.mergeUncached(input) })) })
}
const mutableArray = ["p-2", "p-4"]
const mutableObject = { "p-2": true, "p-4": false }
const mutations = [modules.index.cn(mutableArray), modules.index.cn(mutableObject)]
mutableArray[1] = "p-6"
mutableObject["p-4"] = true
mutations.push(modules.index.cn(mutableArray), modules.index.cn(mutableObject))

mkdirSync(resolve(fixtureRoot, ".work"), { recursive: true })
const temp = mkdtempSync(resolve(fixtureRoot, ".work/project-"))
const buildCases = []
const file = (path, text) => {
  const target = resolve(temp, path)
  mkdirSync(resolve(target, ".."), { recursive: true })
  writeFileSync(target, text)
}
const normalized = (result) => ({
  changed: result.changed, tokens: [...result.tokens].sort(),
  configSha256: sha256(json(result.config)), fullConfigSha256: sha256(json(result.fullConfig)),
  keptClassGroups: Object.keys(result.config.classGroups), fullClassGroupCount: Object.keys(result.fullConfig.classGroups).length,
  theme: result.fullConfig.theme, scannedFiles: result.scannedFiles,
  cssFiles: result.cssFiles.map((path) => relative(temp, path)),
  usedGroups: result.usedGroups, totalGroups: result.totalGroups, warnings: result.warnings,
  sourceSha256: sha256(result.source), sourceBytes: Buffer.byteLength(result.source),
})
try {
  file("src/view.html", '<div class="p-2 p-4 text-lg/7 leading-6 custom"></div>')
  file("tokens.txt", "p-2 p-4 hover:p-2 hover:p-4")
  file("safelist.txt", "shadow-card shadow-lg")
  file("theme.css", '@import "./nested.css"; @theme { --radius-*: initial; --radius-card: 3px; --text-hero: 24px; }')
  file("nested.css", "@theme { --shadow-card: 0 1px black; }")
  file("config.mjs", 'export default {extend:{classGroups:{display:["custom-display"]}}}')
  const variants = {
    content: { content: ["src/**/*.html"], css: false },
    tokens: { tokens: "tokens.txt", css: false },
    full: { full: true, css: false },
    css: { tokens: "tokens.txt", css: "theme.css", safelist: "safelist.txt" },
    config: { content: ["src/**/*.html"], config: "config.mjs", css: false },
    tsOutput: { tokens: "tokens.txt", css: false, out: "tables-ts.ts" },
  }
  for (const [id, options] of Object.entries(variants)) {
    const opts = { cwd: temp, out: `tables-${id}.mjs`, ...options }
    const first = await modules.build.build(opts)
    const second = await modules.build.build(opts)
    assert.equal(first.changed, true)
    assert.equal(second.changed, false)
    assert.equal(first.source, second.source)
    const expected = normalized(first)
    if (!first.outPath.endsWith(".ts")) {
      const tables = (await import(pathToFileURL(first.outPath).href)).default
      const cn = modules.engine.createCn(tables)
      expected.merges = ["p-2 p-4", "m-2 m-4", "shadow-card shadow-lg", "text-hero text-lg", "custom-display block"].map((input) => ({ input, expected: cn(input) }))
    }
    buildCases.push({ id, options, expected, unchangedSecondBuild: !second.changed })
  }
  for (const [id, options] of Object.entries({ noFiles: { content: ["missing/**/*.html"] }, missingTokens: { tokens: "missing.txt" }, unknownValidatorConfig: { config: "bad-config.mjs", full: true } })) {
    file("bad-config.mjs", 'export default {extend:{classGroups:{probe:[{probe:[{$v:"unknown"}]}]}}}')
    let expected
    try { await modules.build.build({ cwd: temp, out: `error-${id}.mjs`, css: false, ...options }) }
    catch (error) { expected = { name: error.name, message: error.message.replaceAll(temp, "<fixture>") } }
    assert.ok(expected, `${id} should fail`)
    buildCases.push({ id, options, expected })
  }
  const builder = modules.build.createBuilder({ cwd: temp, content: ["src/**/*.html"], out: "builder.mjs", css: false })
  assert.equal(builder.last, null)
  const first = await builder.run()
  file("src/view.html", '<div class="p-6"></div>')
  const known = await builder.changed("src/view.html")
  file("src/view.html", '<div class="m-4"></div>')
  const fresh = await builder.changed("src/view.html")
  buildCases.push({ id: "builder-dropped-group-edit", expected: { firstGroups: Object.keys(first.config.classGroups), reusedKnownResult: known === first, addedGroupChanged: fresh.changed, lastIsNewResult: builder.last === fresh, freshGroups: Object.keys(fresh.config.classGroups) } })
  const matcher = modules.build.createContentMatcher(["**/*.html", "node_modules/allowed/*.html"], temp)
  buildCases.push({ id: "content-matcher", expected: ["src/view.html", "node_modules/hidden/view.html", "node_modules/allowed/view.html", ".git/view.html", "../outside.html", "src/view.ts"].map((path) => ({ path, matched: matcher(resolve(temp, path)) })) })
  const tokens = new Set()
  const skippedLong = modules.build.extractTokens('class="p-2 hover:p-4" <x> ' + "x".repeat(8193), tokens)
  buildCases.push({ id: "token-length-cap", expected: { skippedLong, tokens: [...tokens] } })
} finally {
  rmSync(temp, { recursive: true, force: true })
}

const artifacts = {
  "source-lock.json": lock,
  "inventory.json": inventory(modules, defaultConfig),
  "default-config.json": defaultConfig,
  "default-tables.json": tableJson(modules["tables.generated"].default),
  "runtime-fixtures.json": { schemaVersion: 1, commit, seed: "0x5eed1234", cases: runtime },
  "compiler-fixtures.json": { schemaVersion: 1, commit, cases: compilerCases, validators, engineCases, mutableInputs: { expectedSequence: mutations } },
  "build-fixtures.json": { schemaVersion: 1, commit, cases: buildCases },
}
for (const [name, artifact] of Object.entries(artifacts)) {
  const path = resolve(fixtureRoot, name)
  if (write) writeFileSync(path, json(artifact))
  else assert.equal(readFileSync(path, "utf8"), json(artifact), `${name}: fixed reference differs; investigate source/input changes, do not silently rewrite expected`)
}
console.log(`cn reference ${write ? "generated" : "verified"}: ${runtime.length} runtime cases, ${compilerCases.length} compiler cases, ${validators.length} validator families, ${engineCases.length} cache sequences, ${buildCases.length} build cases`)
console.log("Rust/WASM compatibility remains pending; this command validates the locked JS oracle only.")
