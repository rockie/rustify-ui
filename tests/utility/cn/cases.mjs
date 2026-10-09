import { Script } from "node:vm"
import { read, encode, decode, sourceLocator } from "./upstream.mjs"
import { recipes, recipeInput } from "./recipes.mjs"
export { recipes, recipeInput }

// Read literal corpora from the reviewed upstream test, rather than copying a
// guessed compatibility list. Stop only once a complete expression parses.
function literal(path, name) {
  const text = read(path)
  const assignment = new RegExp(`const ${name} = `).exec(text)
  if (!assignment) throw new Error(`missing corpus ${path}:${name}`)
  const start = assignment.index + assignment[0].length
  for (let end = text.indexOf("\n", start); end >= 0; end = text.indexOf("\n", end + 1)) {
    try {
      const script = new Script("(" + text.slice(start, end).replace(/;\s*$/, "") + ")")
      return script.runInNewContext({}, { timeout: 1000 })
    } catch (error) {
      if (!(error instanceof SyntaxError) && error.name !== "SyntaxError") throw error
    }
  }
  throw new Error(`could not parse corpus ${path}:${name}`)
}

export function runtimeCases() {
  const out = []
  const add = (id, api, args, recipe = "default", source) => out.push({ id, api, recipe, args: encode(args), ...(source ? { source } : {}) })
  const path = "packages/conformance/tests/correctness.mjs"
  const batteries = literal(path, "batteries")
  for (const [family, inputs] of Object.entries(batteries)) {
    inputs.forEach((input, i) => add(`upstream-${family}-${i}`, "cn", [input], "default", sourceLocator(path, `  ${family}:`)))
  }
  const trios = literal(path, "trios")
  for (let i = 0; i < trios.length; i++) {
    const [a, b, c] = trios[i]
    const permutations = [[a, b, c], [a, c, b], [b, a, c], [b, c, a], [c, a, b], [c, b, a]]
    permutations.forEach((tokens, j) => add(`upstream-trio-${i}-${j}`, "twMerge", [tokens.join(" ")], "default", sourceLocator(path, "const trios")))
  }
  const pool = literal(path, "pool")
  let seed = 0x5eed1234
  const rnd = () => { seed ^= seed << 13; seed ^= seed >>> 17; seed ^= seed << 5; return (seed >>> 0) / 0x100000000 }
  for (let i = 0; i < 512; i++) {
    const tokens = Array.from({ length: 2 + Math.floor(rnd() * 8) }, () => pool[Math.floor(rnd() * pool.length)])
    add(`seeded-${i}`, "twMerge", [tokens.join(" ")], "default", sourceLocator(path, "const pool"))
  }
  const joinArgs = [
    [], [null, undefined, false, true, 0, -0, 0n, NaN, ""],
    ["p-2", ["px-4", false, ["py-1"]], null, "p-6"],
    [1, -1, 1.25, 1e21, 1e-7, Infinity, -Infinity, NaN, 3n],
    [{ "p-2": true, "p-4": false, flex: 1, block: "yes", empty: null }],
    [{ "2": true, "1": true, "x": true, "01": true, "": true }],
    [{ length: 2, 0: "p-4", 1: "m-2" }], [" a  b ", "\t p-2\n", "p-4"],
    [[[], [null, [undefined, "x", ["p-2", "p-4"]]]]], ["你好 p-2", "🚀 p-4", "custom custom"],
    [function ignoredFunction() {}, "p-2"], [{ "x y": true, " p-2 ": true, "p-4": [] }],
  ]
  for (const api of ["cn", "clsx", "twJoin", "twMerge", "lite.clsx"]) {
    // twJoin/twMerge accept strings and nested arrays by their public type.
    // Out-of-type JS shapes are explicitly identified instead of conflated
    // with typed Rust inputs.
    joinArgs.forEach((args, i) => add(`join-${api}-${i}`, api, args, "default", sourceLocator("packages/conformance/tests/join.mjs", "// ---------- 4. hand cases")))
  }
  const inputs = {
    extendLiteral: ["text-hero text-lg", "text-lg text-hero", "text-tiny text-hero", "text-hero text-red-500", "text-hero/7 text-lg"],
    extendTheme: ["text-huge text-lg", "text-lg text-huge", "md:text-huge md:text-sm"],
    directedConflict: ["bar-x foo-a", "foo-a bar-x", "bar-x bar-y foo-b", "hover:bar-x hover:foo-a"],
    override: ["shadow-custom shadow-lg", "shadow-lg shadow-custom", "shadow-custom shadow-custom", "shadow-sm shadow"],
    prefix: ["tw:p-2 tw:p-4", "p-2 tw:p-4", "tw:hover:md:p-2 tw:md:hover:p-4", "tw:[color:red] tw:[color:blue]", "tw:text-lg/7 tw:text-xl"],
    cacheDisabled: ["p-2 px-4 p-6", "text-lg/7 leading-6", "hover:md:p-2 md:hover:p-4"],
    identityTransform: ["p-2 px-4 p-6", "text-lg/7 leading-6"],
    fullConfig: ["p-2 px-4 p-6", "text-lg/7 leading-6"],
    predicate: ["text-12vw text-lg", "text-lg text-12vw", "text-12vw text-14vw", "text-12vw text-red-500"],
    themeGetter: ["customtext-sm customtext-lg", "customtext-lg customtext-sm"],
    throwsPredicate: ["probe-a probe-b"], throwsTransform: ["p-2 p-4"], unknownValidator: ["probe-a probe-b"],
    orderSensitive: ["custom:hover:p-2 hover:custom:p-4", "hover:focus:p-2 focus:hover:p-4"],
  }
  for (const [recipe, values] of Object.entries(inputs)) {
    for (const api of ["config.createCn", "config.createTwMerge", "config.extendTailwindMerge"]) {
      values.forEach((input, i) => add(`config-${recipe}-${api}-${i}`, api, [input], recipe))
    }
  }
  return out
}

export function executeCase(test, modules) {
  const args = decode(test.args)
  let fn
  if (test.api.startsWith("config.")) fn = modules.config[test.api.slice(7)](recipeInput(test.recipe, modules.config))
  else if (test.api === "lite.clsx") fn = modules.lite.clsx
  else fn = modules.index[test.api]
  try { return { ok: true, value: fn(...args) } }
  catch (error) { return { ok: false, error: { name: error.name, message: error.message } } }
}
