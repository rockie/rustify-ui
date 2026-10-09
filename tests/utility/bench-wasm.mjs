// The Rust cn and compiler behind their JS facades: every call crosses JS/WASM,
// on the same workloads, inputs and sample protocol as the JS references and
// the native example. Run through `mbx xtask utility-bench`.
// Usage: node tests/utility/bench-wasm.mjs <cn-facade-dir> <compiler-facade-dir> <out.json>
import assert from 'node:assert/strict'
import { readFile, writeFile } from 'node:fs/promises'
import { performance } from 'node:perf_hooks'
import { resolve } from 'node:path'
import { pathToFileURL } from 'node:url'

const [cnDir, compilerDir, out] = process.argv.slice(2)
const json = async path => JSON.parse(await readFile(new URL(path, import.meta.url), 'utf8'))
const percentile = (values, fraction) => [...values].sort((a, b) => a - b)[Math.ceil(values.length * fraction) - 1]
const glue = async dir => {
  const module = await import(pathToFileURL(resolve(dir, 'style.js')).href)
  const exports = module.initSync({ module: await readFile(resolve(dir, 'style_bg.wasm')) })
  // Linear memory only grows, so its size after a workload is that
  // instance's high-water mark: the Worker's WASM memory for the same input.
  return { ...module, memoryBytes: () => exports.memory.buffer.byteLength }
}

// cn: per-call mean ns per sample, exactly as tests/utility/cn/bench.mjs.
const cn = await glue(cnDir)
const corpus = (await json('./cn/runtime-fixtures.json')).cases
  .filter(test => test.id.startsWith('upstream-') && test.api === 'cn').map(test => test.args[0])
const variants = corpus.map(input => ['inline-flex items-center px-2 py-1', input, 'px-4'])
const limits = { maxDepth: 128, maxNodes: 100000, maxUtf16Units: 1000000, maxTokens: 100000, cacheBytes: 10000000 }
const samples = 7, iterations = 100000
let sink = 0
const perCall = (run, count = iterations) => {
  for (let i = 0; i < Math.min(count, 20000); i++) sink ^= run(i).length
  const times = []
  for (let sample = 0; sample < samples; sample++) {
    const start = performance.now()
    for (let i = 0; i < count; i++) sink ^= run(i).length
    times.push((performance.now() - start) * 1e6 / count)
  }
  times.sort((a, b) => a - b)
  return { unit: 'ns/call', callsPerSample: count, samples, p50: times[Math.floor(times.length / 2)], p95: times[Math.ceil(times.length * .95) - 1], sortedSamples: times }
}
const cached = new cn.CnSession(undefined, limits)
const uncached = new cn.CnSession({ cacheSize: 0 }, limits)
const cnMeasurements = {
  cnVariadicWarm: perCall(i => cached.merge(variants[i % variants.length])),
  cnSingleWarm: perCall(i => cached.merge([corpus[i % corpus.length]])),
  mergeStringWarm: perCall(i => cached.tw_merge([corpus[i % corpus.length]])),
  mergeUncached: perCall(i => uncached.merge([corpus[i % corpus.length]])),
  clsxJoinOnly: perCall(i => cached.join(variants[i % variants.length])),
  // Compiles the default configuration object, as createCn() and Cn::new(Config::default()) do.
  createConfigAndFirstMerge: perCall(i => { const session = new cn.CnSession(cn.cn_default_config(), limits); const value = session.merge([corpus[i % corpus.length]]); session.free(); return value }, 40),
}

// Compiler: the Worker session protocol (prepare then commit), on the
// reference's cold/warm/no-op workload and on the Worker batch.
const compiler = await glue(compilerDir)
const compilerLimits = { input_bytes: 1 << 20, asset_bytes: 1 << 20, candidate_count: 1 << 16, ast_depth: 64, css_bytes: 64 << 20, queued_bytes: 1 << 22, cache_bytes: 1 << 30 }
const config = JSON.stringify({ input: { css: '@tailwind utilities;', entry: 'memory:input.css' }, assets: [], limits: compilerLimits, owner: null })
const add = (session, candidates) => {
  const { token, report } = JSON.parse(session.prepare(JSON.stringify(candidates)))
  session.commit(token)
  return report
}
const summarize = values => ({ samples: values.length, p50Ms: percentile(values, 0.5), p95Ms: percentile(values, 0.95), minMs: Math.min(...values), maxMs: Math.max(...values) })
const initial = ['w-[137px]', 'bg-[#123456]']
for (let i = 0; i < 10; i++) { const session = new compiler.CompilerSession(config); assert.ok(add(session, initial).css.includes('137px')); session.free() }
const cold = [], warm = [], noOp = []
for (let i = 0; i < 100; i++) {
  const start = performance.now()
  const session = new compiler.CompilerSession(config)
  add(session, initial)
  cold.push(performance.now() - start)
  session.free()
}
const session = new compiler.CompilerSession(config)
add(session, initial)
for (let i = 0; i < 100; i++) { const start = performance.now(); add(session, [`w-[${200 + i}px]`]); warm.push(performance.now() - start) }
const last = add(session, [])
for (let i = 0; i < 100; i++) {
  const start = performance.now()
  const report = add(session, initial)
  noOp.push(performance.now() - start)
  assert.equal(report.revision, last.revision)
}
session.free()
const statics = (await json('./compiler/static-fixtures.json')).union.candidates
const dynamic = (await json('./compiler/dynamic-variant-fixtures.json')).cases.flatMap(item => item.candidates)
const batch = [...new Set([...statics, ...dynamic])]
const prepareBatch = () => { const s = new compiler.CompilerSession(config); const { report } = JSON.parse(s.prepare(JSON.stringify(batch))); s.free(); return report }
const memoryBeforeBatch = compiler.memoryBytes()
const shape = prepareBatch()
for (let i = 0; i < 5; i++) prepareBatch()
const batchSamples = []
for (let sample = 0; sample < 5; sample++) {
  const times = []
  for (let i = 0; i < 20; i++) { const start = performance.now(); prepareBatch(); times.push(performance.now() - start) }
  batchSamples.push({ p50: percentile(times, 0.5), p95: percentile(times, 0.95) })
}
const report = {
  schemaVersion: 1, build: 'wasm32 release facade in Node ' + process.version, sink,
  cn: { inputs: corpus.length, measurements: cnMeasurements, linearMemoryBytes: cn.memoryBytes() },
  compiler: { workload: 'Worker session prepare+commit on the compiler reference workload', metrics: { cold: summarize(cold), warm: summarize(warm), noOp: summarize(noOp) } },
  batch: { candidates: batch.length, admitted: shape.admitted.length, cssBytes: shape.css.length,
    p50Ms: percentile(batchSamples.map(item => item.p50), 0.5), p95Ms: percentile(batchSamples.map(item => item.p95), 0.5), samples: batchSamples,
    linearMemoryBytes: { beforeBatch: memoryBeforeBatch, afterBatches: compiler.memoryBytes() } },
}
await writeFile(out, JSON.stringify(report, null, 2) + '\n')
console.log(JSON.stringify({ compiler: report.compiler.metrics, batch: [report.batch.p50Ms, report.batch.p95Ms] }))
