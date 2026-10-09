import assert from 'node:assert/strict'
import {createHash} from 'node:crypto'
import {readFile} from 'node:fs/promises'
import {resolve} from 'node:path'
import {pathToFileURL} from 'node:url'

const fixture = JSON.parse(await readFile('tests/utility/compiler/utf16-fixtures.json', 'utf8'))
assert.equal(fixture.sourceCommit, '1334c99db8fd26c8ea065375dd9259800863f072')
assert.equal(fixture.cases.length, 486)
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
const units = text => Array.from({length:text.length}, (_, index) => text.charCodeAt(index))
function fromUnits(value) {
  let output = ''
  for (let index = 0; index < value.length; index += 8192) output += String.fromCharCode(...value.slice(index, index + 8192))
  return output
}
const same = (left,right) => JSON.stringify(left) === JSON.stringify(right)
function failure(error) {
  return error instanceof Error && error.cause?.kind === 'compile' ? error.cause.error : {kind:'unexpected_js_error',message:String(error)}
}
let cssMatches = 0, errorPhaseMessageMatches = 0, themeMatches = 0
const failures = []
for (const item of fixture.cases) {
  let api, actual, phase = 'compile'
  try {
    api = new facade.CssCompiler(fromUnits(item.inputUtf16Units), config)
    phase = 'build'
    actual = units(api.build(item.candidatesUtf16Units.map(fromUnits)).replace(/^\/\*! tailwindcss[^\n]*\n/, ''))
  } catch (error) {
    const observed = failure(error)
    if (item.expectedError && error instanceof Error && same(units(error.name), item.expectedError.nameUtf16Units)
      && ['syntax','range_error'].includes(observed.kind) && phase === item.expectedError.phase
      && same(units(observed.message), item.expectedError.messageUtf16Units)) errorPhaseMessageMatches++
    else failures.push({id:item.id,stage:phase,kind:observed.kind,message:observed.message,feature:observed.feature,
      expected:item.expectedError ? 'error phase/message' : 'CSS'})
  } finally { api?.free() }
  if (actual) {
    if (item.expectedCssUtf16Units && same(actual,item.expectedCssUtf16Units)) cssMatches++
    else failures.push({id:item.id,stage:'build',kind:'css_difference',actualUnits:actual,expected:item.expectedError ? 'error' : 'CSS'})
  }
  if (item.expectedThemeEntries) {
    let design
    try {
      design = new facade.DesignSystem(fromUnits(item.inputUtf16Units), config)
      const actualEntries = design.themeEntries().map(([key, token]) => ({keyUtf16Units:units(key),valueUtf16Units:units(token.value)}))
      if (same(actualEntries,item.expectedThemeEntries)) themeMatches++
      else failures.push({id:item.id,stage:'themeEntries',kind:'entry_difference',actualEntries})
    } catch (error) {
      const observed = failure(error)
      failures.push({id:item.id,stage:'themeEntries',kind:observed.kind,message:observed.message,feature:observed.feature})
    } finally { design?.free() }
  }
}
for (const item of failures) console.log(JSON.stringify(item))
console.log(JSON.stringify({cases:486,cssMatches,errorPhaseMessageMatches,themeObservations:111,themeMatches,
  failedObservations:failures.length,failedCases:new Set(failures.map(item=>item.id)).size,
  actualWasm:true,sharedMemory:false,wasmBytes:bytes.length,
  errorNameConformance:'checked with real Error/RangeError prototypes and original name/message units',
  fullUtf16Conformance:'pending',fullPublicValueConformance:'pending'}))
if (failures.length) process.exitCode = 1
