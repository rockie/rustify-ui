import { performance } from "node:perf_hooks"
import { cpus, platform, arch, release, totalmem, loadavg } from "node:os"
import { readFileSync, writeFileSync } from "node:fs"
import { resolve } from "node:path"
import { fixtureRoot, commit, load, assertLockedSource, sha256, json, decode } from "./upstream.mjs"

assertLockedSource()
const args = process.argv.slice(2)
const option = (name, fallback) => {
  const at = args.indexOf(name)
  if (at < 0) return fallback
  if (at + 1 >= args.length) throw new Error(`missing value for ${name}`)
  return args[at + 1]
}
const samples = Number(option("--samples", "7"))
const iterations = Number(option("--iterations", "100000"))
if (!Number.isSafeInteger(samples) || samples < 3 || !Number.isSafeInteger(iterations) || iterations < 1000) throw new Error("samples >= 3 and iterations >= 1000 must be integers")
const index = await load("index")
const config = await load("config")
const engineModule = await load("engine")
const tables = (await load("tables.generated")).default
const fixtureText = readFileSync(resolve(fixtureRoot, "runtime-fixtures.json"), "utf8")
const fixtures = JSON.parse(fixtureText).cases.filter((test) => test.id.startsWith("upstream-") && test.api === "cn")
const corpus = fixtures.map((test) => decode(test.args)[0])
const variants = corpus.map((input) => ["inline-flex items-center px-2 py-1", input, "px-4"])
const uncached = engineModule.createEngine(tables, undefined, { cacheSize: 0 })
const cached = engineModule.createEngine(tables)
let sink = 0
const measure = (run, count = iterations) => {
  for (let i = 0; i < Math.min(count, 20000); i++) sink ^= run(i).length
  const times = []
  for (let sample = 0; sample < samples; sample++) {
    const start = performance.now()
    for (let i = 0; i < count; i++) sink ^= run(i).length
    times.push((performance.now() - start) * 1e6 / count)
  }
  times.sort((a, b) => a - b)
  return { unit: "ns/call", callsPerSample: count, samples, p50: times[Math.floor(times.length / 2)], p95: times[Math.ceil(times.length * .95) - 1], sortedSamples: times }
}
const before = process.memoryUsage()
const measurements = {
  cnVariadicWarm: measure((i) => index.cn(...variants[i % variants.length])),
  cnSingleWarm: measure((i) => index.cn(corpus[i % corpus.length])),
  mergeStringWarm: measure((i) => cached.mergeString(corpus[i % corpus.length])),
  mergeUncached: measure((i) => uncached.mergeUncached(corpus[i % corpus.length])),
  clsxJoinOnly: measure((i) => index.clsx(...variants[i % variants.length])),
  createConfigAndFirstMerge: measure((i) => config.createCn()(corpus[i % corpus.length]), 40),
}
const report = {
  schemaVersion: 1, measuredAt: new Date().toISOString(), upstreamCommit: commit,
  environment: { node: process.version, v8: process.versions.v8, platform: platform(), arch: arch(), osRelease: release(), cpuModel: cpus()[0]?.model, logicalCpus: cpus().length, totalMemoryBytes: totalmem(), loadAverage: loadavg() },
  corpus: { fixtureSha256: sha256(fixtureText), inputs: corpus.length, chars: corpus.reduce((sum, input) => sum + input.length, 0), variantShape: "base + upstream battery input + caller override" },
  protocol: { warmupCalls: "min(callsPerSample,20000)", samples, iterations, build: "fixed upstream source executed by Node 26 native TypeScript stripping", coldDefinition: "new config merger plus lazy compilation and first merge; module imports already warm", performanceGate: "measurement only; no invented SLO" },
  measurements, memory: { before, after: process.memoryUsage(), interpretation: "process-level snapshots, not retained engine memory or a leak bound" }, sink,
  pending: ["Rust internal native/release", "Rust internal WASM/release", "JS/WASM crossing", "headed browser comparison"],
}
const out = resolve(fixtureRoot, option("--out", "benchmark.local.json"))
writeFileSync(out, json(report))
console.log(`cn JS reference benchmark: ${out}`)
for (const [name, result] of Object.entries(measurements)) console.log(`${name}: p50 ${result.p50.toFixed(1)} ns/call; p95 ${result.p95.toFixed(1)} ns/call`)
console.log("This is a Node JS reference measurement; it makes no Rust or JS/WASM performance claim.")
