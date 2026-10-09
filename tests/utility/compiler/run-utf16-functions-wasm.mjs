import assert from 'node:assert/strict'
import {readFile} from 'node:fs/promises'
import {resolve} from 'node:path'
import {pathToFileURL} from 'node:url'

const directory = resolve('target/utility-conformance/compiler-api')
const facade = await import(pathToFileURL(resolve(directory, 'style.js')))
const wasm = await readFile(resolve(directory, 'style_bg.wasm'))
const exports = await facade.default({module_or_path: wasm})
assert.equal(typeof facade.CssCompiler.prototype.substituteFunctionsInValue, 'function')
assert.equal(exports.memory.buffer instanceof SharedArrayBuffer, false)
const limits = {input_bytes:65536, asset_bytes:65536, candidate_count:1024,
  ast_depth:32, css_bytes:262144, queued_bytes:65536, cache_bytes:8000000}
const config = overrides => JSON.stringify({limits: {...limits, ...overrides}})
const units = value => Array.from({length:value.length}, (_, index) => value.charCodeAt(index))
function fromUnits(value) {
  let result = ''
  for (let start = 0; start < value.length; start += 8192) result += String.fromCharCode(...value.slice(start, start + 8192))
  return result
}
const values = JSON.parse(await readFile(new URL('./utf16-function-fixtures.json', import.meta.url), 'utf8'))
let valueCases = 0, errorCases = 0, boundaryCases = 0
for (const item of values.cases) {
  const api = new facade.CssCompiler(item.themeCss, config())
  try {
    assert.deepEqual(units(api.substituteFunctionsInValue(fromUnits(item.valueUtf16Units))), item.expectedValueUtf16Units, item.id)
    valueCases++
  } finally { api.free() }
}
const fixture = JSON.parse(await readFile(new URL('./utf16-fixtures.json', import.meta.url), 'utf8'))
const api = new facade.CssCompiler('', config())
try {
  for (const item of fixture.cases.filter(item => /^utf16:error:(missing-theme|missing-css-theme|alpha-message):/.test(item.id))) {
    const input = fromUnits(item.inputUtf16Units)
    const prefix = '.p { value: '
    assert(input.startsWith(prefix))
    const expression = input.slice(prefix.length, input.indexOf(';', prefix.length))
    assert.throws(() => api.substituteFunctionsInValue(expression), thrown => {
      const error = thrown.cause.error
      assert.equal(error.kind, 'syntax', item.id)
      assert.deepEqual(units(error.message), item.expectedError.messageUtf16Units, item.id)
      return true
    })
    assert.equal(api.substituteFunctionsInValue('plain'), 'plain', 'error releases the WASM borrow')
    errorCases++
  }
} finally { api.free() }
const limited = new facade.CssCompiler('', config({input_bytes:4, css_bytes:4}))
try {
  assert.equal(limited.substituteFunctionsInValue('\ud800x'), '\ud800x'); boundaryCases++
  assert.equal(limited.substituteFunctionsInValue('😀'), '😀'); boundaryCases++
  assert.throws(() => limited.substituteFunctionsInValue('\ud800xx'), thrown => thrown.cause.error.resource === 'input_bytes'); boundaryCases++
  assert.equal(limited.substituteFunctionsInValue('\udc00'), '\udc00'); boundaryCases++
} finally { limited.free() }
const completeUnits = Array.from({length:65536}, (_, index) => index)
const completeText = fromUnits(completeUnits)
const complete = new facade.CssCompiler('', config({input_bytes:262144, css_bytes:262144}))
try {
  assert.deepEqual(units(complete.substituteFunctionsInValue(completeText)), completeUnits, 'every UTF-16 unit survives the actual crossing'); boundaryCases++
  const splitPair = 'a'.repeat(8191) + '😀' + '\ud800\udc01\udc00\ufffd\ue000\\ud800'
  assert.equal(complete.substituteFunctionsInValue(splitPair), splitPair, 'output chunks preserve pairs and isolated units'); boundaryCases++
  assert.throws(() => complete.substituteFunctionsInValue({toString() {throw new Error('must not coerce')}}), thrown => thrown.cause.kind === 'protocol'); boundaryCases++
  assert.equal(complete.substituteFunctionsInValue('\ud800'), '\ud800'); boundaryCases++
} finally { complete.free() }
assert.equal(valueCases, 75); assert.equal(errorCases, 18)
console.log(JSON.stringify({valueCases,errorCases,boundaryCases,actualWasm:true,sharedMemory:false,fullUtf16Conformance:'pending'}))
