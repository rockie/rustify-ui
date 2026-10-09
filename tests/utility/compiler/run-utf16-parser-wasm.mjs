import assert from 'node:assert/strict'
import {createHash} from 'node:crypto'
import {readFile} from 'node:fs/promises'
import {createRequire} from 'node:module'
import {resolve} from 'node:path'
import {pathToFileURL} from 'node:url'

const require = createRequire(resolve('tests/utility/compiler/oracle/package.json'))
assert.equal(require('tailwindcss/package.json').version, '4.1.13')
const {compile} = require('tailwindcss')
const fixture = JSON.parse(await readFile('tests/utility/compiler/utf16-fixtures.json', 'utf8'))
assert.equal(fixture.sourceCommit, '1334c99db8fd26c8ea065375dd9259800863f072')
for (const source of fixture.sources) {
  assert.equal(createHash('sha256').update(await readFile(resolve('ref/tailwindcss', source.path))).digest('hex'), source.sha256, source.path)
}
const directory = resolve('target/utility-conformance/compiler-api')
const facade = await import(pathToFileURL(resolve(directory, 'style.js')))
const bytes = await readFile(resolve(directory, 'style_bg.wasm'))
const exports = await facade.default({module_or_path:bytes})
assert.equal(exports.memory.buffer instanceof SharedArrayBuffer, false)
const limits = {input_bytes:262144,asset_bytes:262144,candidate_count:1024,ast_depth:32,
  css_bytes:262144,queued_bytes:262144,cache_bytes:8000000}
const units = text => Array.from({length:text.length}, (_, index) => text.charCodeAt(index))
function fromUnits(value) {
  let output = ''
  for (let index = 0; index < value.length; index += 8192) output += String.fromCharCode(...value.slice(index, index + 8192))
  return output
}
function scalar(value) {
  for (let index = 0; index < value.length; index++) {
    const unit = value[index]
    if (unit >= 0xd800 && unit <= 0xdbff) {
      const low = value[++index]
      if (!(low >= 0xdc00 && low <= 0xdfff)) return false
    } else if (unit >= 0xdc00 && unit <= 0xdfff) return false
  }
  return true
}
function mapSnapshot(map) {
  const references = map.mappings.map(mapping => {
    if (!mapping.originalPosition) return null
    const index = map.sources.indexOf(mapping.originalPosition.source)
    assert.notEqual(index, -1, 'mapping shares the decoded Source table object')
    return index
  })
  return {decoded:JSON.parse(JSON.stringify(map)),references}
}
const selected = fixture.cases.filter(item => ['authored','map','error'].includes(item.id.split(':')[1])
  && item.candidatesUtf16Units.every(scalar))
assert.equal(selected.length, 156)
let cssCases = 0, compileErrors = 0, maps = 0, mappings = 0, boundaryCases = 0
// This is the authored-source/parser gate. The remaining compiler matrix still
// needs Text candidates, family intermediates and the lossless public value tree.
for (const item of selected) {
  const input = fromUnits(item.inputUtf16Units)
  const options = {from:`/utf16/${item.id}.css`}
  const config = JSON.stringify({limits,options})
  if (item.expectedError) {
    assert.equal(item.expectedError.phase, 'compile')
    assert.throws(() => new facade.CssCompiler(input, config), error => {
      const result = error.cause.error
      assert(error instanceof Error, item.id)
      assert.deepEqual(units(error.name), item.expectedError.nameUtf16Units, item.id)
      assert.equal(result.kind, error.name === 'RangeError' ? 'range_error' : 'syntax', item.id)
      assert.deepEqual(units(result.message), item.expectedError.messageUtf16Units, item.id)
      return true
    }); compileErrors++
    continue
  }
  const candidates = item.candidatesUtf16Units.map(fromUnits)
  const official = await compile(input, options)
  const api = new facade.CssCompiler(input, config)
  try {
    const expected = official.build(candidates)
    assert.deepEqual(units(api.build(candidates)), units(expected), `${item.id}: official CSS`)
    assert.deepEqual(units(expected.replace(/^\/\*! tailwindcss[^\n]*\n/, '')), item.expectedCssUtf16Units, `${item.id}: frozen CSS`)
    cssCases++
    const actualMap = api.buildSourceMap()
    assert.deepEqual(mapSnapshot(actualMap), mapSnapshot(official.buildSourceMap()), `${item.id}: source map`)
    maps++; mappings += actualMap.mappings.length
    assert.deepEqual(units(api.build([])), units(expected), `${item.id}: no-op`)
  } finally { api.free() }
}
const config = overrides => JSON.stringify({limits:{...limits,...overrides}})
assert.throws(() => new facade.CssCompiler({}, config()), error => error.cause.kind === 'protocol'); boundaryCases++
// A lone code unit costs three logical bytes, while its retained Text storage is
// charged separately. Quota checks must neither escape nor replace the input.
const input = '@unknown \ud800;'
const logicalBytes = input.length + 2
assert.throws(() => new facade.CssCompiler(input, config({input_bytes:logicalBytes-1})), error => {
  const result = error.cause.error
  return result.kind === 'limit' && result.resource === 'input_bytes'
}); boundaryCases++
const exact = new facade.CssCompiler(input, config({input_bytes:logicalBytes}))
try { assert.deepEqual(units(exact.build([])), units(input)); boundaryCases++ } finally { exact.free() }
// JSON asset and Worker-session inputs must reach the same Text parser.
const asset = {id:'opaque.css',path:'/virtual/opaque.css',base:'.',content:'.\ud800 { value: \udc00; }'}
const importInput = '@import "opaque.css";'
const options = {from:'/virtual/entry.css',base:''}
const officialImport = await compile(importInput, {...options,loadStylesheet:async () => ({path:asset.path,base:asset.base,content:asset.content})})
const imported = new facade.CssCompiler(importInput, JSON.stringify({limits,options,assets:[asset]}))
try {
  assert.deepEqual(units(imported.build([])), units(officialImport.build([]))); boundaryCases++
  const actualMap = imported.buildSourceMap()
  assert.deepEqual(mapSnapshot(actualMap), mapSnapshot(officialImport.buildSourceMap())); boundaryCases++
} finally { imported.free() }
for (const [css,assets] of [[input,[]],[importInput,[asset]]]) {
  const session = new facade.CompilerSession(JSON.stringify({input:{css,entry:'entry.css'},assets,limits,owner:null}))
  try {
    const prepared = JSON.parse(session.prepare('[]'))
    const expected = assets.length ? units(officialImport.build([]).replace(/^\/\*! tailwindcss[^\n]*\n/, '')) : units(input)
    assert.deepEqual(units(prepared.report.css), expected); boundaryCases++
    assert.deepEqual(units(JSON.parse(session.commit(prepared.token)).css), expected); boundaryCases++
    assert.deepEqual(units(JSON.parse(session.report()).css), expected); boundaryCases++
  } finally { session.free() }
}
console.log(JSON.stringify({selectedCases:selected.length,cssCases,compileErrors,maps,mappings,boundaryCases,
  actualWasm:true,sharedMemory:false,wasmBytes:bytes.length,remainingCompilerMatrixCases:fixture.cases.length-selected.length,
  fullUtf16Conformance:'pending',fullPublicValueConformance:'pending'}))
