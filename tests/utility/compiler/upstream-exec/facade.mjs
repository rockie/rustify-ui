// One Rust/WASM facade instance shared by the package-entry and unit shims.
import { readFileSync } from 'node:fs'
import path from 'node:path'
import { fileURLToPath, pathToFileURL } from 'node:url'

const here = path.dirname(fileURLToPath(import.meta.url))
const output = path.resolve(here, '../../../..', process.env.RUSTIFY_UPSTREAM_WASM ?? 'target/upstream-exec/upstream-units')
export const facade = await import(pathToFileURL(path.join(output, 'style.js')).href)
facade.initSync({ module: readFileSync(path.join(output, 'style_bg.wasm')) })

// The setup file attributes these counts to the running test: a test is Rust
// evidence only when it crossed into the facade, and mixed when it also used
// an upstream TypeScript helper the shims re-export unchanged.
export function crossed() {
  globalThis.__rustifyFacadeCalls = (globalThis.__rustifyFacadeCalls ?? 0) + 1
}
export function foreign() {
  globalThis.__rustifyForeignCalls = (globalThis.__rustifyForeignCalls ?? 0) + 1
}
export function viaRust(call) {
  return (...args) => {
    crossed()
    return call(...args)
  }
}

// Upstream has no quotas; these only keep a runaway case from exhausting memory.
const limits = {
  input_bytes: 64 << 20, asset_bytes: 64 << 20, candidate_count: 1 << 20, ast_depth: 4096,
  css_bytes: 256 << 20, queued_bytes: 64 << 20, cache_bytes: 1 << 30,
}

export function configuration(opts = {}, assets = [], loader = false) {
  let options = { base: opts.base ?? '', polyfills: opts.polyfills ?? 3 }
  if (opts.from !== undefined) options.from = opts.from
  // Upstream's own build omits its license comment under NODE_ENV=test.
  let license_banner = process.env.NODE_ENV !== 'test'
  return JSON.stringify({ options, assets, loader, license_banner, limits })
}
