// Executes the fixed upstream vitest files and summarizes per-file results.
// A test is Rust evidence only when it crossed into the facade at least once.
// Usage: node run.mjs [--impl rust|official] [--files a.test.ts,b.test.ts] [--json out.json]
import { spawnSync } from 'node:child_process'
import { readFileSync, mkdirSync, rmSync, existsSync, writeFileSync } from 'node:fs'
import path from 'node:path'
import { fileURLToPath } from 'node:url'

const here = path.dirname(fileURLToPath(import.meta.url))
const pkg = path.join(here, '../../../../ref/tailwindcss/packages/tailwindcss')
const arg = (name, fallback) => {
  let index = process.argv.indexOf(name)
  return index < 0 ? fallback : process.argv[index + 1]
}
const impl = arg('--impl', 'rust')
const files = arg('--files', 'src/**/*.test.ts')
const out = path.resolve(arg('--json', path.join(here, '../../../../target/upstream-exec', `${impl}.json`)))
const crossings = out.replace(/\.json$/, '.crossings.jsonl')
mkdirSync(path.dirname(out), { recursive: true })
rmSync(crossings, { force: true })
const vitest = path.join(here, 'node_modules/vitest/vitest.mjs')
const result = spawnSync(process.execPath, [vitest, 'run', '--config', path.join(here, 'vitest.config.mjs'), '--reporter=json', `--outputFile=${out}`], {
  cwd: here,
  env: { ...process.env, CI: '1', RUSTIFY_UPSTREAM_IMPL: impl, RUSTIFY_UPSTREAM_FILES: files, RUSTIFY_UPSTREAM_CROSSINGS: crossings },
  stdio: ['ignore', 'ignore', 'inherit'],
})
const report = JSON.parse(readFileSync(out, 'utf8'))
const key = (file, suites, name, line) => `${file}\0${suites.join('\0')}\0${name}\0${line}`
const crossed = new Map()
if (existsSync(crossings)) {
  for (const line of readFileSync(crossings, 'utf8').split('\n').filter(Boolean)) {
    let entry = JSON.parse(line)
    let id = key(entry.file, entry.suites, entry.name, entry.line)
    let seen = crossed.get(id) ?? { calls: 0, foreign: 0, errors: [] }
    crossed.set(id, { calls: seen.calls + entry.calls, foreign: seen.foreign + (entry.foreign ?? 0), errors: [...seen.errors, ...entry.errors] })
  }
}
const tests = []
for (const file of report.testResults) {
  for (const test of file.assertionResults) {
    let record = crossed.get(key(file.name, test.ancestorTitles, test.title, test.location?.line ?? null))
    let calls = record?.calls ?? 0
    tests.push({
      file: path.relative(pkg, file.name), suites: test.ancestorTitles, name: test.title,
      line: test.location?.line ?? null, column: test.location?.column ?? null,
      status: test.status, rust: calls > 0, mixed: (record?.foreign ?? 0) > 0,
      failure: test.status === 'failed' ? (test.failureMessages[0] ?? '').split('\n')[0].slice(0, 400) : null,
      errors: test.status === 'failed' ? record?.errors ?? [] : undefined,
    })
  }
}
const summary = { impl, passed: report.numPassedTests, failed: report.numFailedTests, skipped: report.numPendingTests + report.numTodoTests, total: report.numTotalTests }
if (impl === 'rust') {
  summary.rustPassed = tests.filter(test => test.rust && !test.mixed && test.status === 'passed').length
  summary.mixed = tests.filter(test => test.mixed).length
  summary.rustFailed = tests.filter(test => test.rust && test.status === 'failed').length
  summary.notExercised = tests.filter(test => !test.rust && test.status !== 'skipped' && test.status !== 'pending').length
}
writeFileSync(out.replace(/\.json$/, '.summary.json'), JSON.stringify({ summary, tests }, null, 1) + '\n')
const byFile = new Map()
for (const test of tests) {
  let row = byFile.get(test.file) ?? { file: test.file, pass: 0, fail: 0, skip: 0, rustPass: 0, rustFail: 0 }
  if (test.status === 'passed') row.pass++, test.rust && row.rustPass++
  else if (test.status === 'failed') row.fail++, test.rust && row.rustFail++
  else row.skip++
  byFile.set(test.file, row)
}
for (const row of [...byFile.values()].sort((a, b) => b.fail - a.fail || a.file.localeCompare(b.file))) {
  console.log(`${String(row.pass).padStart(4)} pass ${String(row.fail).padStart(4)} fail ${String(row.skip).padStart(2)} skip | rust ${String(row.rustPass).padStart(4)} pass ${String(row.rustFail).padStart(4)} fail  ${row.file}`)
}
console.log(JSON.stringify(summary), `exit ${result.status}`)
// Vitest exits 1 for failed tests; anything else means the run itself broke.
// Failures under Rust are adjudicated by ledger.mjs; the control must pass.
if (![0, 1].includes(result.status) || (impl === 'official' && result.status !== 0)) process.exit(1)
