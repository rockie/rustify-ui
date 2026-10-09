// Fixture value shapes and configuration recipes. Pure ES module: the Node
// gates and the browser probe interpret runtime-fixtures.json identically.

export function decode(value) {
  if (Array.isArray(value)) return value.map(decode)
  if (value?.$js === "undefined") return undefined
  if (value?.$js === "bigint") return BigInt(value.value)
  if (value?.$js === "number") return value.value === "-0" ? -0 : Number(value.value)
  if (value?.$js === "function") return function ignoredFunction() {}
  if (value?.$js === "object") return Object.fromEntries(value.entries.map(([key, child]) => [key, decode(child)]))
  return value
}

export const recipes = {
  default: { kind: "default" },
  extendLiteral: { extend: { classGroups: { "font-size": [{ text: ["hero", "tiny"] }] } } },
  extendTheme: { extend: { theme: { text: ["huge"] } } },
  directedConflict: { extend: { classGroups: { foo: ["foo-a", "foo-b"], bar: ["bar-x", "bar-y"] }, conflictingClassGroups: { foo: ["bar"] } } },
  override: { override: { classGroups: { shadow: ["shadow-custom"] } } },
  prefix: { prefix: "tw" },
  cacheDisabled: { cacheSize: 0 },
  identityTransform: { kind: "config-callback", callback: "identity" },
  fullConfig: { kind: "full-config" },
  predicate: { kind: "validator-callback", callback: "endsWithVw" },
  themeGetter: { kind: "theme-getter-callback", callback: "textScale", isThemeGetter: true },
  throwsPredicate: { kind: "validator-callback", callback: "throwError" },
  throwsTransform: { kind: "config-callback", callback: "throwError" },
  unknownValidator: { extend: { classGroups: { probe: [{ probe: [{ $v: "notAValidator" }] }] } } },
  orderSensitive: { extend: { orderSensitiveModifiers: ["custom"] } },
}

export function recipeInput(name, config) {
  const value = recipes[name]
  if (!value) throw new Error(`unknown config recipe ${name}`)
  if (name === "default") return undefined
  if (name === "identityTransform") return (base) => base
  if (name === "fullConfig") return config.defaultConfig()
  if (name === "predicate") return { extend: { classGroups: { "font-size": [{ text: [(v) => v.endsWith("vw")] }] } } }
  if (name === "themeGetter") {
    const getter = (theme) => theme.text
    getter.isThemeGetter = true
    return { extend: { classGroups: { customText: [{ customtext: [getter] }] } } }
  }
  if (name === "throwsPredicate") return { extend: { classGroups: { probe: [{ probe: [() => { throw new Error("fixture validator failure") }] }] } } }
  if (name === "throwsTransform") return () => { throw new Error("fixture config failure") }
  return value
}
