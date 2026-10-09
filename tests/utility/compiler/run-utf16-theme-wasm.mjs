import assert from 'node:assert/strict'
import {readFile} from 'node:fs/promises'
import {resolve} from 'node:path'
import {createRequire} from 'node:module'
import {pathToFileURL} from 'node:url'

const directory = resolve('target/utility-conformance/compiler-api')
const facade = await import(pathToFileURL(resolve(directory, 'style.js')))
const bytes = await readFile(resolve(directory, 'style_bg.wasm'))
const exports = await facade.default({module_or_path:bytes})
assert.equal(exports.memory.buffer instanceof SharedArrayBuffer, false)
const limits = {input_bytes:262144,asset_bytes:65536,candidate_count:1024,ast_depth:32,
  css_bytes:262144,queued_bytes:65536,cache_bytes:8000000}
const config = overrides => JSON.stringify({limits:{...limits,...overrides}})
const units = text => Array.from({length:text.length}, (_, index) => text.charCodeAt(index))
function fromUnits(value) {
  let output = ''
  for (let index = 0; index < value.length; index += 8192) output += String.fromCharCode(...value.slice(index, index + 8192))
  return output
}
let boundaryCases = 0
// Theme entries and class completions retain original ECMAScript strings.
const guarded = new facade.DesignSystem('@theme { --color-\\d800 : red; } @tailwind utilities;', config())
try {
  const entries = guarded.themeEntries()
  assert.deepEqual(units(entries[0][0]), [45,45,99,111,108,111,114,45,0xd800]); boundaryCases++
  assert.equal(entries[0][1].value, 'red')
  assert.equal(Object.hasOwn(entries[0][1], 'src'), true)
  assert.equal(entries[0][1].src, undefined)
  assert.ok(guarded.getClassList().some(([name]) => name === 'bg-\ud800')); boundaryCases++
  assert.equal(guarded.parseCandidate('block')[0].root, 'block'); boundaryCases++
} finally { guarded.free() }
assert.throws(() => new facade.CssCompiler('@theme { --x: red; } @tailwind utilities;', config({cache_bytes:1})), error => {
  const result = error.cause.error
  return result.kind === 'limit' && result.resource === 'cache_bytes'
}); boundaryCases++

const fixture = JSON.parse(await readFile(new URL('./utf16-fixtures.json', import.meta.url), 'utf8'))
const observations = fixture.cases.filter(item => item.expectedThemeEntries)
assert.equal(observations.length, 111)
// Candidate ingress shares the original-unit source path.
const cases = observations
assert.equal(cases.length, 111)
let cssCases = 0, themeObservations = 0
for (const item of cases) {
  const input = fromUnits(item.inputUtf16Units)
  const api = new facade.CssCompiler(input, config())
  try {
    const css = api.build(item.candidatesUtf16Units.map(fromUnits)).replace(/^\/\*! tailwindcss[^\n]*\n/, '')
    assert.deepEqual(units(css), item.expectedCssUtf16Units, item.id)
    cssCases++
  } finally { api.free() }
}
for (const item of observations) {
  const design = new facade.DesignSystem(fromUnits(item.inputUtf16Units), config())
  try {
    const entries = design.themeEntries()
    const observed = entries.map(([key, token]) => ({keyUtf16Units:units(key),valueUtf16Units:units(token.value)}))
    assert.deepEqual(observed, item.expectedThemeEntries, `${item.id}: theme entries`)
    for (const [,token] of entries) {
      assert.equal(Object.hasOwn(token, 'src'), true, `${item.id}: src presence`)
      assert.equal(token.src, undefined, `${item.id}: src value`)
      assert.equal(Number.isInteger(token.options), true, `${item.id}: options`)
      assert.deepEqual(Object.keys(token), ['value','options','src'], `${item.id}: fields`)
    }
    themeObservations++
  } finally { design.free() }
}
const require = createRequire(resolve('tests/utility/compiler/oracle/package.json'))
assert.equal(require('tailwindcss/package.json').version, '4.1.13')
const {__unstable__loadDesignSystem:loadDesignSystem} = require('tailwindcss')
let trackingCases = 0
const tokenSnapshot = entries => Array.from(entries, ([key, token]) => ({key:units(key),value:units(token.value),options:token.options,src:token.src}))
for (const [options,prefix] of [['',''],['prefix(tw)','tw-']]) {
  const input = `@theme ${options} { --color-\\d800: red; --color-�: blue; --color-\\ud800: green; } @tailwind utilities;`
  const official = await loadDesignSystem(input)
  const design = new facade.DesignSystem(input, config())
  try {
    for (const raw of [
      `var(--${prefix}color-\\d800)`, `var(--${prefix}color-\ud800)`,
      `var(--${prefix}color-�)`, `var(--${prefix}color-\\ud800)`,
      `var(--${prefix}missing-\\110000)`, `var(--${prefix}color-\ud800)`,
      `var(--xx-color-\ud800)`, `--${prefix}color-\ud800`,
    ]) {
      let expectedError
      try { official.trackUsedVariables(raw) } catch (error) { expectedError = error }
      if (expectedError) {
        assert.throws(() => design.trackUsedVariables(raw), error => {
          assert(error instanceof RangeError)
          assert.deepEqual(units(error.name), units(expectedError.name))
          assert.deepEqual(units(error.message), units(expectedError.message))
          return true
        })
      } else design.trackUsedVariables(raw)
      assert.deepEqual(tokenSnapshot(design.themeEntries()), tokenSnapshot(official.theme.entries()), `${options}: tracking ${JSON.stringify(raw)}`)
      trackingCases++
    }
  } finally { design.free() }
}
for (const [options,prefix] of [['',''],['prefix(tw)','tw-']]) {
  for (const validFirst of [true,false]) {
    const input = `@theme ${options} { --z: 1; } @tailwind utilities;`
    const official = await loadDesignSystem(input)
    const design = new facade.DesignSystem(input, config())
    try {
      const valid = `var(--${prefix}z)`
      const invalid = `var(--${prefix}\\110000)`
      const raw = validFirst ? `${valid} ${invalid}` : `${invalid} ${valid}`
      assert.throws(() => official.trackUsedVariables(raw), RangeError)
      assert.throws(() => design.trackUsedVariables(raw), RangeError)
      assert.deepEqual(tokenSnapshot(design.themeEntries()), tokenSnapshot(official.theme.entries()), `failed tracking order: ${options}/${validFirst}`)
      official.trackUsedVariables(valid)
      design.trackUsedVariables(valid)
      assert.deepEqual(tokenSnapshot(design.themeEntries()), tokenSnapshot(official.theme.entries()), 'tracking resumes after RangeError')
      trackingCases++
    } finally { design.free() }
  }
}
assert.equal(trackingCases, 20)
const boundedTracking = new facade.DesignSystem('', config({input_bytes:4}))
try {
  assert.throws(() => boundedTracking.trackUsedVariables('\ud800xx'), error => error.cause.error.resource === 'input_bytes')
  assert.deepEqual(boundedTracking.themeEntries(), [])
  boundaryCases++
} finally { boundedTracking.free() }
// Find a tracking plan admitted without JS caches but rejected with retained
// candidate objects. This tests the combined budget without hardcoding Rust sizes.
let combinedTrackingLimit = false
for (let count = 1; count <= 256 && !combinedTrackingLimit; count++) {
  const input = '@theme { --z: red; } @tailwind utilities;'
  const baseline = new facade.DesignSystem(input, config({cache_bytes:32768}))
  const cached = new facade.DesignSystem(input, config({cache_bytes:32768}))
  try {
    const candidate = cached.parseCandidate('block')
    for (let index = 0; index < 16; index++) cached.parseCandidate(`unknown-${index}`)
    const before = tokenSnapshot(cached.themeEntries())
    const raw = `var(--z, ${'x'.repeat(count * 8)})`
    try { baseline.trackUsedVariables(raw) } catch (error) {
      assert.equal(error.cause.error.resource, 'cache_bytes')
      break
    }
    try { cached.trackUsedVariables(raw) } catch (error) {
      assert.equal(error.cause.error.kind, 'limit')
      assert.equal(error.cause.error.resource, 'cache_bytes')
      assert.deepEqual(tokenSnapshot(cached.themeEntries()), before, 'Limit leaves USED unchanged and entries readable')
      assert.equal(cached.parseCandidate('block'), candidate, 'Limit preserves cached JS identity')
      cached.trackUsedVariables('var(--z)')
      assert.equal(cached.themeEntries()[0][1].options, 16, 'short tracking recovers after Limit')
      combinedTrackingLimit = true
      boundaryCases++
    }
  } finally { baseline.free(); cached.free() }
}
assert.equal(combinedTrackingLimit, true, 'JS cache retention must enter tracking preflight')
assert.equal(themeObservations, 111)
console.log(JSON.stringify({cssCases,themeObservations,trackingCases,boundaryCases,
  candidateObservationCasesPending:0,actualWasm:true,sharedMemory:false,
  wasmBytes:bytes.length,fullUtf16Conformance:'pending',fullPublicValueConformance:'pending'}))
