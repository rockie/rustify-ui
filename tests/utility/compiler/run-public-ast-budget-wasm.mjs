import assert from 'node:assert/strict'
import {createHash} from 'node:crypto'
import {readFile} from 'node:fs/promises'
import {resolve} from 'node:path'
import {pathToFileURL} from 'node:url'

const inventory = JSON.parse(await readFile('tests/utility/compiler/inventory.json','utf8'))
assert.equal(inventory.baseline.commit,'1334c99db8fd26c8ea065375dd9259800863f072')
const source = 'packages/tailwindcss/src/ast.ts'
assert.equal(createHash('sha256').update(await readFile(resolve('ref/tailwindcss',source))).digest('hex'),
  inventory.sources.find(entry => entry.path === source)?.sha256)
const directory = resolve('target/utility-conformance/compiler-api')
const facade = await import(pathToFileURL(resolve(directory,'style.js')))
const bytes = await readFile(resolve(directory,'style_bg.wasm'))
const exports = await facade.default({module_or_path:bytes})
assert.equal(exports.memory.buffer instanceof SharedArrayBuffer,false)
const limits = {input_bytes:262144,asset_bytes:262144,candidate_count:1024,ast_depth:32,
  css_bytes:262144,queued_bytes:262144,cache_bytes:16000000}
const isInputLimit = error => error.cause?.error?.kind === 'limit' && error.cause.error.resource === 'input_bytes'
let boundaries = 0, recoveries = 0
let shared = 'leaf'
for (let level=0;level<18;level++) shared = [shared,shared]
const input = [{kind:'declaration',property:'--x',value:'red',important:false,
  src:[{file:'dag.css',code:'source',metadata:shared},0,1]}]
const before = exports.memory.buffer.byteLength
assert.throws(() => new facade.AstCompiler(input,JSON.stringify({limits:{...limits,input_bytes:1024,cache_bytes:65536}})),isInputLimit)
const growth = exports.memory.buffer.byteLength-before
console.log(JSON.stringify({sharedDagLevels:18,growth,maximumGrowth:131072}))
// This is a fresh-module allocation witness, not a complete allocator audit.
assert(growth<=131072,'input quota must reject the shared DAG before expanding its JSON')
boundaries++
for (const text of ['ordinary','"\\\b\t\n\f\r\u0000','😀','\ud800','\udc00','\ud800\udc00\ud800']) {
  const ast = [{kind:'comment',value:text}]
  const expected = Buffer.byteLength(JSON.stringify(ast))
  assert.throws(() => new facade.AstCompiler(ast,JSON.stringify({limits:{...limits,input_bytes:expected-1}})),isInputLimit)
  boundaries++
  for (const size of [expected,expected+1]) {
    const api = new facade.AstCompiler(ast,JSON.stringify({limits:{...limits,input_bytes:size}}))
    try { assert.equal(api.build([]),ast); boundaries++; recoveries++ } finally { api.free() }
  }
}
assert.equal(boundaries,19); assert.equal(recoveries,12)
console.log(JSON.stringify({boundaries,recoveries,sharedDagLevels:18,growth,maximumGrowth:131072,
  actualWasm:true,sharedMemory:false,wasmBytes:bytes.length,fullTemporaryBudgetConformance:'pending'}))
