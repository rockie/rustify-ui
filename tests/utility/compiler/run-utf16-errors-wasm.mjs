import assert from 'node:assert/strict'
import {createHash} from 'node:crypto'
import {readFile} from 'node:fs/promises'
import {resolve} from 'node:path'
import {pathToFileURL} from 'node:url'

const fixture = JSON.parse(await readFile('tests/utility/compiler/utf16-fixtures.json', 'utf8'))
for (const source of fixture.sources) {
  assert.equal(createHash('sha256').update(await readFile(resolve('ref/tailwindcss', source.path))).digest('hex'), source.sha256, source.path)
}
const directory = resolve('target/utility-conformance/compiler-api')
const facade = await import(pathToFileURL(resolve(directory, 'style.js')))
const bytes = await readFile(resolve(directory, 'style_bg.wasm'))
const exports = await facade.default({module_or_path:bytes})
assert.equal(exports.memory.buffer instanceof SharedArrayBuffer, false)
const limits = {input_bytes:262144,asset_bytes:262144,candidate_count:1024,ast_depth:32,
  css_bytes:262144,queued_bytes:262144,cache_bytes:16000000}
const config = JSON.stringify({limits})
const units = value => Array.from({length:value.length}, (_, index) => value.charCodeAt(index))
const fromUnits = value => String.fromCharCode(...value)
const snapshot = value => JSON.parse(JSON.stringify(value))
const cases = fixture.cases.filter(item => item.expectedError)
assert.equal(cases.length, 61)
let compileErrors = 0, buildErrors = 0, rangeErrors = 0, recoveryCases = 0
for (const item of cases) {
  let api, phase = 'compile', observed
  try {
    api = new facade.CssCompiler(fromUnits(item.inputUtf16Units), config)
    phase = 'build'
    api.build(item.candidatesUtf16Units.map(fromUnits))
  } catch (error) { observed = error }
  try {
    assert(observed instanceof Error, `${item.id}: real Error object`)
    assert.equal(phase, item.expectedError.phase, `${item.id}: phase`)
    assert.deepEqual(units(observed.name), item.expectedError.nameUtf16Units, `${item.id}: name`)
    assert.deepEqual(units(observed.message), item.expectedError.messageUtf16Units, `${item.id}: message`)
    assert.equal(observed instanceof RangeError, observed.name === 'RangeError', `${item.id}: prototype`)
    assert.equal(observed.cause.kind, 'compile', `${item.id}: typed cause`)
    assert.equal(observed.cause.error.kind, observed.name === 'RangeError' ? 'range_error' : 'syntax')
    assert.deepEqual(units(observed.cause.error.message), item.expectedError.messageUtf16Units)
    if (phase === 'compile') compileErrors++; else {
      buildErrors++
      const map = snapshot(api.buildSourceMap())
      assert.throws(() => api.build(item.candidatesUtf16Units.map(fromUnits)), error => {
        assert(error instanceof RangeError)
        assert.deepEqual(units(error.message), item.expectedError.messageUtf16Units)
        assert.notEqual(error, observed)
        return true
      })
      assert.deepEqual(snapshot(api.buildSourceMap()), map, `${item.id}: failed build retains map`)
      recoveryCases++
    }
    if (observed instanceof RangeError) rangeErrors++
  } finally { api?.free() }
}
assert.equal(compileErrors, 59)
assert.equal(buildErrors, 2)
assert.equal(rangeErrors, 6)
const passthrough = '.p { value: var(--custom-\\110000); }'
const api = new facade.CssCompiler(passthrough, config)
try {
  assert.equal(api.features, 0)
  assert.equal(api.build([]), passthrough)
  assert.equal(api.build(['block']), passthrough)
  assert.equal(api.build([]), passthrough)
  recoveryCases++
} finally { api.free() }
assert.throws(() => new facade.CssCompiler({}, config), error => {
  assert(error instanceof TypeError)
  assert.equal(error.message, 'expected a string')
  assert.equal(error.cause.kind, 'protocol')
  return true
})
console.log(JSON.stringify({errorCases:cases.length,compileErrors,buildErrors,rangeErrors,recoveryCases,
  protocolErrors:1,actualWasm:true,sharedMemory:false,wasmBytes:bytes.length,fullUtf16Conformance:'pending'}))
