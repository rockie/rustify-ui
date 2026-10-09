import assert from 'node:assert/strict'
import {createHash} from 'node:crypto'
import {createRequire} from 'node:module'
import {readFile} from 'node:fs/promises'
import {resolve} from 'node:path'
import {pathToFileURL} from 'node:url'

const fixture = JSON.parse(await readFile('tests/utility/compiler/utf16-fixtures.json', 'utf8'))
for (const source of fixture.sources) {
  assert.equal(createHash('sha256').update(await readFile(resolve('ref/tailwindcss', source.path))).digest('hex'), source.sha256, source.path)
}
const require = createRequire(resolve('tests/utility/compiler/oracle/package.json'))
assert.equal(require('tailwindcss/package.json').version, '4.1.13')
const {compile} = require('tailwindcss')
const directory = resolve('target/utility-conformance/compiler-api')
const facade = await import(pathToFileURL(resolve(directory, 'style.js')))
const bytes = await readFile(resolve(directory, 'style_bg.wasm'))
const exports = await facade.default({module_or_path:bytes})
assert.equal(exports.memory.buffer instanceof SharedArrayBuffer, false)
const limits = {input_bytes:262144,asset_bytes:262144,candidate_count:1024,ast_depth:32,
  css_bytes:262144,queued_bytes:262144,cache_bytes:16000000}
const inputs = []
for (const name of ['\ud800','\udc00','\ud800\ud800','\udc00\udc00','\udc00\ud800','😀','�','card']) {
  for (const themed of [false, true]) {
    inputs.push(`@theme { ${themed ? `--animate-custom: ${name} 1s; ` : ''}@keyframes ${name} { to { opacity: 0; } } } .p { animation: ${themed ? 'var(--animate-custom)' : name+' 1s'}; } @tailwind utilities;`)
  }
}
for (const whitespace of ['\ufeff','\u0085','\t',' ','\u2028','\u2007']) {
  inputs.push(`@theme { @keyframes card { to { opacity: 0; } } } .p { animation: card${whitespace}1s; } @tailwind utilities;`)
}
inputs.push('@theme { @keyframes \ud800 { to { opacity: 0; } } @keyframes � { to { opacity: 1; } } } .p { animation: \ud800 1s, � 2s; } @tailwind utilities;')
const snapshot = value => JSON.parse(JSON.stringify(value))
let cssComparisons = 0, maps = 0
for (const [index, input] of inputs.entries()) {
  const options = {from:'/virtual/animations.css'}
  const reference = await compile(input, options)
  const compiler = new facade.CssCompiler(input, JSON.stringify({limits,options}))
  try {
    for (const candidates of [[], ['block'], []]) {
      assert.equal(compiler.build(candidates), reference.build(candidates), `animation ${index}: ${JSON.stringify(candidates)}`)
      assert.deepEqual(snapshot(compiler.buildSourceMap()), snapshot(reference.buildSourceMap()), `animation ${index}: maps`)
      cssComparisons++; maps++
    }
  } finally { compiler.free() }
}
assert.equal(inputs.length, 23)
assert.equal(cssComparisons, 69)
assert.equal(maps, 69)
console.log(JSON.stringify({animationCases:inputs.length,cssComparisons,maps,actualWasm:true,
  sharedMemory:false,wasmBytes:bytes.length,fullUtf16Conformance:'pending'}))
