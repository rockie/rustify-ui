import assert from 'node:assert/strict'
import {createHash} from 'node:crypto'
import {readFile} from 'node:fs/promises'
import {resolve} from 'node:path'
import {pathToFileURL} from 'node:url'

const fixture = JSON.parse(await readFile('tests/utility/compiler/utf16-family-fixtures.json','utf8'))
assert.equal(fixture.sourceCommit,'1334c99db8fd26c8ea065375dd9259800863f072')
assert.equal(fixture.oracleVersion,'4.1.13')
assert.equal(fixture.cases.length,560)
for (const source of fixture.sources) assert.equal(createHash('sha256')
  .update(await readFile(resolve('ref/tailwindcss',source.path))).digest('hex'),source.sha256,source.path)
const directory = resolve('target/utility-conformance/compiler-api')
const facade = await import(pathToFileURL(resolve(directory,'style.js')))
const bytes = await readFile(resolve(directory,'style_bg.wasm'))
const exports = await facade.default({module_or_path:bytes})
assert.equal(exports.memory.buffer instanceof SharedArrayBuffer,false)
const limits = {input_bytes:262144,asset_bytes:262144,candidate_count:1024,ast_depth:32,
  css_bytes:262144,queued_bytes:262144,cache_bytes:16000000}
function fromUnits(units) {
  let output = ''
  for (let offset=0;offset<units.length;offset+=8192) output += String.fromCharCode(...units.slice(offset,offset+8192))
  return output
}
const units = text => Array.from({length:text.length},(_,index)=>text.charCodeAt(index))
let matches=0
const failures=[]
for (const item of fixture.cases) {
  let compiler
  try {
    compiler = new facade.CssCompiler(fromUnits(item.inputUtf16Units),JSON.stringify({limits}))
    const actual = units(compiler.build(item.candidates).replace(/^\/\*! tailwindcss[^\n]*\n/,''))
    if (JSON.stringify(actual) === JSON.stringify(item.expectedCssUtf16Units)) matches++
    else failures.push({id:item.id,kind:'css_difference',actualUnits:actual})
  } catch(error) {
    failures.push({id:item.id,kind:error.cause?.error?.kind??error.name,message:error.message})
  } finally {compiler?.free()}
}
for (const failure of failures) console.log(JSON.stringify(failure))
console.log(JSON.stringify({families:fixture.families.length,cases:fixture.cases.length,cssMatches:matches,
  failedCases:failures.length,actualWasm:true,sharedMemory:false,wasmBytes:bytes.length,fullUtf16Conformance:'pending'}))
if (failures.length) process.exitCode=1
