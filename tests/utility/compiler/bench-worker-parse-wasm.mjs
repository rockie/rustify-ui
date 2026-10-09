// Serial before/after timing of the product Worker compiler on a large
// candidate batch: session construction plus one prepare of every candidate.
// Usage: node bench-worker-parse-wasm.mjs <before-compiler-dir> <after-compiler-dir> [out.json]
import assert from 'node:assert/strict'
import { readFile, writeFile } from 'node:fs/promises'
import { cpus, platform, release } from 'node:os'
import { resolve } from 'node:path'
import { performance } from 'node:perf_hooks'
import { pathToFileURL } from 'node:url'

const [beforeDir, afterDir, out] = process.argv.slice(2)
const load = async name => JSON.parse(await readFile(new URL(`./${name}`, import.meta.url), 'utf8'))
const statics = (await load('static-fixtures.json')).union.candidates
const variants = (await load('dynamic-variant-fixtures.json')).cases.flatMap(item => item.candidates)
const candidates = [...new Set([...statics, ...variants])]
const limits = { input_bytes: 1 << 20, asset_bytes: 1 << 20, candidate_count: 1 << 16, ast_depth: 64, css_bytes: 64 << 20, queued_bytes: 1 << 22, cache_bytes: 1 << 30 }
const config = JSON.stringify({ input: { css: '@tailwind utilities;', entry: 'memory:input.css' }, assets: [], limits, owner: null })
const payload = JSON.stringify(candidates)
const percentile = (values, fraction) => [...values].sort((a, b) => a - b)[Math.ceil(values.length * fraction) - 1]

async function measure(dir) {
  const glue = await import(pathToFileURL(resolve(dir, 'style.js')).href + `?${dir}`)
  glue.initSync({ module: await readFile(resolve(dir, 'style_bg.wasm')) })
  const run = () => {
    const session = new glue.CompilerSession(config)
    const { report } = JSON.parse(session.prepare(payload))
    session.free()
    return report
  }
  const shape = run()
  for (let index = 0; index < 5; index++) run()
  const samples = []
  for (let sample = 0; sample < 5; sample++) {
    const times = []
    for (let index = 0; index < 20; index++) {
      const start = performance.now()
      run()
      times.push(performance.now() - start)
    }
    samples.push({ p50: percentile(times, 0.5), p95: percentile(times, 0.95) })
  }
  const median = key => percentile(samples.map(item => item[key]), 0.5)
  return { cssBytes: shape.css.length, admitted: shape.admitted.length, p50Ms: median('p50'), p95Ms: median('p95'), samples }
}

const before = await measure(beforeDir)
const after = await measure(afterDir)
assert.equal(after.cssBytes, before.cssBytes, 'same output size')
assert.equal(after.admitted, before.admitted, 'same admitted count')
const result = { candidates: candidates.length, environment: { node: process.version, platform: platform(), release: release(), cpu: cpus()[0]?.model }, before, after,
  p50ChangePercent: (after.p50Ms / before.p50Ms - 1) * 100, p95ChangePercent: (after.p95Ms / before.p95Ms - 1) * 100 }
if (out) await writeFile(out, JSON.stringify(result, null, 1) + '\n')
console.log(JSON.stringify({ candidates: result.candidates, before: [before.p50Ms, before.p95Ms], after: [after.p50Ms, after.p95Ms], p50ChangePercent: result.p50ChangePercent, p95ChangePercent: result.p95ChangePercent }))
