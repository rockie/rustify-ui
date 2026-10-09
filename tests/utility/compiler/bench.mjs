import assert from 'node:assert/strict'
import { createRequire } from 'node:module'
import { cpus, platform, release, arch, totalmem } from 'node:os'
import { performance } from 'node:perf_hooks'
import { writeFile } from 'node:fs/promises'
import { dirname, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'

const directory = dirname(fileURLToPath(import.meta.url))
const oracleRequire = createRequire(resolve(directory, 'oracle/package.json'))
const tailwind = oracleRequire('tailwindcss')
const iterations = 100
const candidates = ['w-[137px]', 'bg-[#123456]']
const css = '@tailwind utilities;'
const percentile = (values, fraction) => [...values].sort((a, b) => a - b)[Math.ceil(values.length * fraction) - 1]
const summarize = values => ({ samples: values.length, p50Ms: percentile(values, 0.5), p95Ms: percentile(values, 0.95), minMs: Math.min(...values), maxMs: Math.max(...values) })

// Warm the Node/JIT oracle; cold below means a new compiler session, not a fresh process.
for (let i = 0; i < 10; i++) (await tailwind.compile(css)).build(candidates)
const memoryBefore = process.memoryUsage()
const cold = []
for (let i = 0; i < iterations; i++) {
  const start = performance.now()
  const session = await tailwind.compile(css)
  session.build(candidates)
  cold.push(performance.now() - start)
}
const session = await tailwind.compile(css)
session.build(candidates)
const warm = []
for (let i = 0; i < iterations; i++) {
  const start = performance.now()
  session.build([`w-[${200 + i}px]`])
  warm.push(performance.now() - start)
}
const last = session.build([])
const noOp = []
for (let i = 0; i < iterations; i++) {
  const start = performance.now()
  const output = session.build(candidates)
  noOp.push(performance.now() - start)
  assert.equal(output, last)
}
const result = {
  schemaVersion: 1,
  measuredAt: new Date().toISOString(),
  environment: { node: process.version, platform: platform(), release: release(), arch: arch(), cpuModel: cpus()[0]?.model ?? 'unknown', cpuCount: cpus().length, totalMemoryBytes: totalmem() },
  oracle: { package: 'tailwindcss', version: oracleRequire('tailwindcss/package.json').version, commit: '1334c99db8fd26c8ea065375dd9259800863f072' },
  scope: 'Official Node compiler reference only. This does not measure Rust, WASM, crossing, DOM installation or GPU frames.',
  workload: { css, initialCandidates: candidates, warmAdditions: iterations, finalCandidateCount: candidates.length + iterations, timing: { cold: 'new compiler session parse plus initial build after Node JIT warmup', warm: 'one new width candidate per call; sorted full CSS output grows from 3 to 102 candidates', noOp: 'repeat initial candidates on the final warmed 102-candidate session' } },
  metrics: { cold: summarize(cold), warm: summarize(warm), noOp: summarize(noOp) },
  memory: { before: memoryBefore, after: process.memoryUsage(), meaning: 'Node process RSS/heap/external/arrayBuffers snapshots around timed workloads; includes reference compiler and GC behavior, not a Rust/WASM peak measurement' },
}
const outputIndex = process.argv.indexOf('--output')
if (outputIndex >= 0) await writeFile(resolve(process.argv[outputIndex + 1]), `${JSON.stringify(result, null, 2)}\n`)
console.log(JSON.stringify(result.metrics))
