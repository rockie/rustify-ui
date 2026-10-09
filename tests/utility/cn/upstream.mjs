// Development-only oracle. Production never imports this module or upstream TS.
import { registerHooks } from "node:module"
import { existsSync, readFileSync, readdirSync } from "node:fs"
import { fileURLToPath, pathToFileURL } from "node:url"
import { resolve, relative } from "node:path"
import { createHash } from "node:crypto"
import { decode } from "./recipes.mjs"
export { decode }

export const fixtureRoot = fileURLToPath(new URL(".", import.meta.url))
export const upstreamRoot = resolve(fixtureRoot, "../../../ref/cn")
export const commit = "b7ec0fce43d824a60517fab508584eb860fdf717"
export const sha256 = (value) => createHash("sha256").update(value).digest("hex")
export const json = (value) => JSON.stringify(value, null, 2) + "\n"
export const read = (path) => readFileSync(resolve(upstreamRoot, path), "utf8")

registerHooks({
  resolve(specifier, context, next) {
    if (specifier.startsWith(".") && context.parentURL?.startsWith(pathToFileURL(upstreamRoot).href + "/")) {
      const url = new URL(specifier, context.parentURL)
      const path = fileURLToPath(url)
      const candidate = path.endsWith(".js") ? path.slice(0, -3) + ".ts" : path + ".ts"
      if (existsSync(candidate)) return next(pathToFileURL(candidate).href, context)
    }
    return next(specifier, context)
  },
})

export const load = (name) => import(pathToFileURL(resolve(upstreamRoot, `packages/cn/src/${name}.ts`)).href)

export function sourceFiles(dir = upstreamRoot) {
  return readdirSync(dir, { withFileTypes: true }).flatMap((entry) => {
    const path = resolve(dir, entry.name)
    if (entry.name === "node_modules" || entry.name === "dist" || entry.name === ".git") return []
    return entry.isDirectory() ? sourceFiles(path) : [relative(upstreamRoot, path).replaceAll("\\", "/")]
  }).sort()
}

export function sourceLocator(path, needle) {
  const lines = read(path).split("\n")
  const line = lines.findIndex((text) => text.includes(needle)) + 1
  if (!line) throw new Error(`missing source locator: ${path}: ${needle}`)
  return { path, line }
}

// Native values stay native until after execution. The tags are a fixture wire
// format, not an alternative cn input API or a production serialization scheme.
export function encode(value) {
  if (value === undefined) return { $js: "undefined" }
  if (typeof value === "bigint") return { $js: "bigint", value: String(value) }
  if (typeof value === "number" && (!Number.isFinite(value) || Object.is(value, -0))) {
    return { $js: "number", value: Object.is(value, -0) ? "-0" : String(value) }
  }
  if (typeof value === "function") return { $js: "function", fixture: "ignoredFunction" }
  if (Array.isArray(value)) return value.map(encode)
  if (value && typeof value === "object") {
    return { $js: "object", entries: Object.keys(value).map((key) => [key, encode(value[key])]) }
  }
  return value
}

export function tableJson(tables) {
  return Object.fromEntries(Object.entries(tables).map(([key, value]) => [key, ArrayBuffer.isView(value) ? Array.from(value) : value]))
}

export function assertLockedSource() {
  const lockPath = resolve(fixtureRoot, "source-lock.json")
  if (!existsSync(lockPath)) throw new Error("No source-lock.json; use --write to establish the reviewed baseline")
  const lock = JSON.parse(readFileSync(lockPath, "utf8"))
  if (lock.commit !== commit) throw new Error("cn reference commit mismatch")
  for (const file of lock.files) {
    if (sha256(read(file.path)) !== file.sha256) throw new Error(`cn source drift: ${file.path}`)
  }
  return lock
}
