import assert from 'node:assert/strict'
import {createHash} from 'node:crypto'
import {createRequire} from 'node:module'
import {readFile} from 'node:fs/promises'
import {resolve} from 'node:path'
import {pathToFileURL} from 'node:url'

const fixture = JSON.parse(await readFile('tests/utility/compiler/utf16-family-fixtures.json','utf8'))
assert.equal(fixture.sourceCommit,'1334c99db8fd26c8ea065375dd9259800863f072')
assert.equal(fixture.oracleVersion,'4.1.13')
for (const source of fixture.sources) assert.equal(createHash('sha256')
  .update(await readFile(resolve('ref/tailwindcss',source.path))).digest('hex'),source.sha256,source.path)
const require = createRequire(resolve('tests/utility/compiler/oracle/package.json'))
assert.equal(require('tailwindcss/package.json').version,'4.1.13')
const {compile} = require('tailwindcss')
const directory = resolve('target/utility-conformance/compiler-api')
const bytes = await readFile(resolve(directory,'style_bg.wasm'))
const limits = {input_bytes:262144,asset_bytes:262144,candidate_count:1024,ast_depth:32,
  css_bytes:1024,queued_bytes:262144,cache_bytes:2000000}
const failures=[]
let cases=0,recoveries=0
for (const [index,[root,namespace,marker]] of [
  ['shadow','shadow','$`'],['drop-shadow','drop-shadow','$`'],['shadow','shadow',"$'"],
].entries()) {
  // A separate module gives every observation fresh allocator/linear-memory state.
  const url = pathToFileURL(resolve(directory,'style.js'))
  url.searchParams.set('shadow-budget',index)
  const facade = await import(url.href)
  const exports = await facade.default({module_or_path:bytes})
  assert.equal(exports.memory.buffer instanceof SharedArrayBuffer,false)
  const offset = `1${'x'.repeat(4096)}`
  const value = marker === '$`' ? `${offset} 0 ${marker.repeat(2048)}`
    : `0 0 ${marker.repeat(2048)} ${offset}`
  const input = `@theme inline { --${namespace}-brand: ${value}; } @tailwind utilities;`
  const compiler = new facade.CssCompiler(input,JSON.stringify({limits}))
  try {
    const baseline = compiler.build([])
    const map = JSON.parse(JSON.stringify(compiler.buildSourceMap()))
    const before = exports.memory.buffer.byteLength
    assert.throws(()=>compiler.build([`${root}-brand`]),error=>{
      assert(error instanceof Error)
      assert.equal(error.cause.kind,'compile')
      assert.equal(error.cause.error.kind,'limit')
      assert(['css_bytes','cache_bytes'].includes(error.cause.error.resource))
      return true
    })
    const growth = exports.memory.buffer.byteLength-before
    // Page rounding is allowed; these small inputs must not allocate expanded megabytes.
    const maximumGrowth = limits.cache_bytes*2
    if (growth>maximumGrowth) failures.push({root,marker,growth,maximumGrowth})
    assert.equal(compiler.build([]),baseline)
    assert.deepEqual(JSON.parse(JSON.stringify(compiler.buildSourceMap())),map)
    const reference = await compile(input)
    assert.equal(compiler.build(['block']),reference.build(['block']))
    cases++;recoveries++
    console.log(JSON.stringify({root,marker,growth,maximumGrowth}))
  } finally {compiler.free()}
}
// Successful outputs must still fit at the exact logical CSS boundary, including
// non-scalar Theme colors, all shadow wrappers and the public API banner.
const facade = await import(pathToFileURL(resolve(directory,'style.js')).href)
await facade.default({module_or_path:bytes})
let boundaryCases=0,boundaryRecoveries=0
for (const root of ['shadow','inset-shadow','text-shadow','drop-shadow']) {
  for (const mode of ['', 'inline', 'reference']) {
    for (const color of ['\ud800$&', '\udc00$`', '😀$\'']) {
      const input = `@theme ${mode} { --${root}-brand: 0 0 ${color}; } @tailwind utilities;`
      const candidate = `${root}-brand`
      const reference = await compile(input)
      const expected = reference.build([candidate])
      const cssBytes = Buffer.byteLength(expected)
      assert(cssBytes>0)
      for (const cap of [cssBytes-1,cssBytes,cssBytes+1]) {
        const compiler = new facade.CssCompiler(input,JSON.stringify({limits:{...limits,css_bytes:cap}}))
        try {
          const baseline = compiler.build([])
          if (cap<cssBytes) {
            const map = JSON.parse(JSON.stringify(compiler.buildSourceMap()))
            assert.throws(()=>compiler.build([candidate]),error=>{
              assert.equal(error.cause.error.kind,'limit')
              assert.equal(error.cause.error.resource,'css_bytes')
              assert.equal(error.cause.error.limit,cap)
              return true
            })
            assert.equal(compiler.build([]),baseline)
            assert.deepEqual(JSON.parse(JSON.stringify(compiler.buildSourceMap())),map)
            const fresh = await compile(input)
            assert.equal(compiler.build(['block']),fresh.build(['block']))
            boundaryRecoveries++
          } else assert.equal(compiler.build([candidate]),expected)
          boundaryCases++
        } finally {compiler.free()}
      }
    }
  }
}
console.log(JSON.stringify({expansionCases:cases,recoveries,boundaryCases,boundaryRecoveries,failedCases:failures.length,
  actualWasm:true,sharedMemory:false,wasmBytes:bytes.length,fullTemporaryBudgetConformance:'pending'}))
if (failures.length) process.exitCode=1
