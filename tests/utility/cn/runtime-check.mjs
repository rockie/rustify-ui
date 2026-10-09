// Runs the fixed runtime corpus against an initialized cn WASM facade. Pure ES
// module, shared by the Node gate and the strict-CSP browser probe, so both
// cross the same JS/WASM boundary with the same arguments and comparisons.
import { decode, recipeInput } from "./recipes.mjs"

const method = (api) => api === "clsx" ? "join" : api === "lite.clsx" ? "lite" : api === "twJoin" ? "tw_join"
  : api === "twMerge" || api.endsWith("createTwMerge") || api.endsWith("extendTailwindMerge") ? "tw_merge" : "merge"

function same(left, right) {
  if (left === right) return true
  if (typeof left !== "object" || typeof right !== "object" || left === null || right === null) return false
  const keys = Object.keys(left)
  return keys.length === Object.keys(right).length && keys.every((key) => same(left[key], right[key]))
}

export function runFixedCases(wasm, fixtures, limits) {
  const configHelpers = { defaultConfig: wasm.cn_default_config }
  let passed = 0
  const failures = []
  for (const test of fixtures) {
    const config = test.api.startsWith("config.") ? recipeInput(test.recipe, configHelpers) : undefined
    const session = new wasm.CnSession(config, limits)
    let actual
    try {
      actual = { ok: true, value: session[method(test.api)](decode(test.args)) }
    } catch (error) {
      actual = { ok: false, error: { name: error.name, message: error.message } }
    }
    if (same(actual, test.expected)) passed++
    else failures.push({ id: test.id, actual, expected: test.expected })
    session.free()
  }
  return { fixedCases: fixtures.length, passed, failures }
}
