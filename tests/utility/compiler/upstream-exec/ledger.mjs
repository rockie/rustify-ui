// Maps executed upstream vitest results onto the static upstream test ledger.
// Usage: node ledger.mjs [--write|--check] [--rust summary.json] [--official summary.json]
//
// Dispositions:
//   rust_passed            reached the Rust facade and passed the upstream assertions
//   rust_failed            reached the Rust facade and failed for another reason
//   excluded_js_module     failed only because it needs a JavaScript @plugin/@config module
//   excluded_js_helper     unit suite for a helper that only serves JavaScript plugins/configs
//   excluded_js_api        upstream's JavaScript AST traversal API, which has no Rust surface
//   checked_internal_entry internal construction checked separately by internal-entries.mjs
//   mixed_upstream_helper  reached Rust but also ran an upstream TypeScript helper
//   not_exercised          passed or failed without reaching Rust (internal unit tests)
//   upstream_skipped       skipped upstream as well
import assert from 'node:assert/strict'
import { readFileSync, writeFileSync } from 'node:fs'
import path from 'node:path'
import { fileURLToPath } from 'node:url'

const here = path.dirname(fileURLToPath(import.meta.url))
const repo = path.resolve(here, '../../../..')
const arg = (name, fallback) => {
  let index = process.argv.indexOf(name)
  return index < 0 ? fallback : process.argv[index + 1]
}
const ledgerPath = path.join(here, '../upstream-test-ledger.json')
const outputPath = path.join(here, '../upstream-test-execution.json')
const ledger = JSON.parse(readFileSync(ledgerPath, 'utf8'))
const load = file => JSON.parse(readFileSync(path.resolve(file), 'utf8'))
const rust = load(arg('--rust', path.join(repo, 'target/upstream-exec/rust.summary.json')))
const official = load(arg('--official', path.join(repo, 'target/upstream-exec/official.summary.json')))
assert.equal(rust.summary.impl, 'rust')
assert.equal(official.summary.impl, 'official')

const prefix = 'packages/tailwindcss/'
const group = tests => {
  let groups = new Map()
  for (let test of tests) {
    let key = `${prefix}${test.file}:${test.line}`
    groups.set(key, [...(groups.get(key) ?? []), test])
  }
  return groups
}

// Vitest locates `each` registrations at the call that receives the name,
// which can follow the parameter table; pair those groups by file and size.
function pair(results) {
  let byLocation = group(results.tests)
  let ledgerGroups = new Map()
  for (let test of ledger.expandedTests) {
    let [file, line] = test.sourceTestId.split(':')
    let key = `${file}:${line}`
    ledgerGroups.set(key, [...(ledgerGroups.get(key) ?? []), test])
  }
  let unclaimed = new Set([...byLocation.keys()].filter(key => !ledgerGroups.has(key)))
  let pairs = new Map()
  for (let [key, tests] of ledgerGroups) {
    let found = byLocation.get(key)
    if (!found) {
      let [file, line] = key.split(':')
      let candidates = [...unclaimed]
        .filter(other => other.startsWith(`${file}:`) && byLocation.get(other).length === tests.length)
        .map(other => [other, Number(other.split(':')[1])])
        .filter(([, otherLine]) => otherLine > Number(line))
        .sort((a, b) => a[1] - b[1])
      assert.ok(candidates.length > 0, `no executed group for ${key}`)
      unclaimed.delete(candidates[0][0])
      found = byLocation.get(candidates[0][0])
    }
    assert.equal(found.length, tests.length, `test count differs at ${key}`)
    tests.forEach((test, index) => pairs.set(test.id, found[index]))
  }
  assert.equal(unclaimed.size, 0, `executed tests without ledger entries: ${[...unclaimed].join(', ')}`)
  assert.equal(pairs.size, ledger.expandedTests.length)
  return pairs
}

// Upstream unit suites whose subject exists only to run JavaScript plugins or
// configs; the plan defers that execution. Their tests are adjudicated, not run.
const javascriptOnly = {
  'packages/tailwindcss/src/compat/selector-parser.test.ts': 'selector parser used only by the JavaScript plugin API',
  'packages/tailwindcss/src/compat/apply-config-to-theme.test.ts': 'converts a JavaScript config theme into CSS variables',
  'packages/tailwindcss/src/compat/apply-keyframes-to-theme.test.ts': 'converts JavaScript config keyframes',
  'packages/tailwindcss/src/compat/config/resolve-config.test.ts': 'resolves JavaScript config objects and presets',
  'packages/tailwindcss/src/compat/flatten-color-palette.test.ts': 'plugin-author helper over JavaScript color objects',
}

// Tests of upstream's JavaScript AST traversal (`walk` with `replaceWith` and
// `WalkAction`). The compiler traverses its own trees internally and exposes
// no JavaScript traversal callbacks; printing and optimization are Rust.
const javascriptApi = {
  'packages/tailwindcss/src/ast.test.ts': [
    'allows the placement of context nodes',
    'should stop walking when returning `WalkAction.Stop`',
    'should only visit children once when calling `replaceWith` with single element array',
    'should only visit children once when calling `replaceWith` with multi-element array',
    'should never visit children when calling `replaceWith` with `WalkAction.Skip`',
    'should skip the correct number of children based on the replaced children nodes',
  ],
  'packages/tailwindcss/src/value-parser.test.ts': ['can be used to replace a function call'],
}

// Tests that build a design system through the internal `buildDesignSystem`,
// which skips the compatibility hooks every public entry runs. Their exact
// Rust result is checked by `internal-entries.mjs`.
const internalEntry = {
  'packages/tailwindcss/src/intellisense.test.ts': {
    getClassList: 'upstream snapshot plus the 20 legacy utilities every public entry registers',
  },
}

const rustPairs = pair(rust)
const officialPairs = pair(official)
const javascriptModule = /unsupported feature: @(plugin|config)\b/

function disposition(test) {
  let executed = rustPairs.get(test.id)
  let control = officialPairs.get(test.id)
  assert.equal(executed.name, control.name, `${test.id}: names differ between runs`)
  if (control.status === 'skipped' || control.status === 'pending') {
    assert.equal(test.upstreamMode, 'skip', `${test.id}: skipped although the ledger runs it`)
    return 'upstream_skipped'
  }
  assert.equal(control.status, 'passed', `${test.id}: official control did not pass`)
  if (javascriptOnly[test.id.split(':')[0]] && !(executed.rust && !executed.mixed && executed.status === 'passed')) {
    return 'excluded_js_helper'
  }
  let file = test.id.split(':')[0]
  if (javascriptApi[file]?.includes(executed.name)) return 'excluded_js_api'
  if (internalEntry[file]?.[executed.name] && executed.rust && executed.status === 'failed') return 'checked_internal_entry'
  if (!executed.rust) return 'not_exercised'
  if (executed.mixed) return executed.status === 'passed' ? 'mixed_upstream_helper' : 'rust_failed'
  if (executed.status === 'passed') return 'rust_passed'
  let errors = executed.errors ?? []
  if (errors.length > 0 && errors.every(error => javascriptModule.test(`${error.message}\n${error.actual}`))) {
    return 'excluded_js_module'
  }
  return 'rust_failed'
}

const tests = ledger.expandedTests.map(test => {
  let executed = rustPairs.get(test.id)
  let entry = { id: test.id, name: executed.name, suites: executed.suites, disposition: disposition(test) }
  if (entry.disposition === 'excluded_js_helper') entry.evidence = [javascriptOnly[test.id.split(':')[0]]]
  if (entry.disposition === 'excluded_js_api') entry.evidence = ['JavaScript AST traversal API']
  if (entry.disposition === 'checked_internal_entry') entry.evidence = [internalEntry[test.id.split(':')[0]][executed.name]]
  if (entry.disposition === 'rust_failed' || entry.disposition === 'excluded_js_module') {
    entry.evidence = (executed.errors ?? []).map(error => error.actual && error.actual !== 'undefined' ? `${error.message} | actual: ${error.actual}` : error.message)
  }
  return entry
})
const count = name => tests.filter(test => test.disposition === name).length
const summary = {
  expandedTests: tests.length,
  rustPassed: count('rust_passed'),
  rustFailed: count('rust_failed'),
  excludedJavaScriptModule: count('excluded_js_module'),
  excludedJavaScriptHelper: count('excluded_js_helper'),
  excludedJavaScriptApi: count('excluded_js_api'),
  checkedInternalEntry: count('checked_internal_entry'),
  mixedUpstreamHelper: count('mixed_upstream_helper'),
  notExercised: count('not_exercised'),
  upstreamSkipped: count('upstream_skipped'),
}
const byFile = {}
for (let test of tests) {
  let file = test.id.split(':')[0].slice(prefix.length)
  byFile[file] ??= { rust_passed: 0, rust_failed: 0, excluded_js_module: 0, excluded_js_helper: 0, excluded_js_api: 0, checked_internal_entry: 0, mixed_upstream_helper: 0, not_exercised: 0, upstream_skipped: 0 }
  byFile[file][test.disposition]++
}
const record = {
  schemaVersion: 1,
  baseline: { version: ledger.baseline.version, commit: ledger.baseline.commit, ledger: 'upstream-test-ledger.json' },
  meaning: 'Executed upstream vitest files with the package entry (compile, compileAst, __unstable__loadDesignSystem) resolved to the Rust/WASM facade. A test is Rust evidence only when it reached the facade. The official TypeScript run is the harness control.',
  summary,
  byFile,
  tests,
}
const text = JSON.stringify(record, null, 1) + '\n'
if (process.argv.includes('--write')) writeFileSync(outputPath, text)
else if (process.argv.includes('--check')) assert.equal(readFileSync(outputPath, 'utf8'), text, 'upstream-test-execution.json is stale')
console.log(JSON.stringify(summary))
for (let [file, row] of Object.entries(byFile).sort()) {
  if (row.rust_failed) console.log(`rust_failed ${row.rust_failed} ${file}`)
}
