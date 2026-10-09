import assert from 'node:assert/strict'
import {createRequire} from 'node:module'
import {readFile} from 'node:fs/promises'
import {resolve} from 'node:path'
import {pathToFileURL} from 'node:url'

const require = createRequire(resolve('tests/utility/compiler/oracle/package.json'))
assert.equal(require('tailwindcss/package.json').version, '4.1.13')
const {compile, __unstable__loadDesignSystem:loadDesignSystem} = require('tailwindcss')
const directory = resolve('target/utility-conformance/compiler-api')
const facade = await import(pathToFileURL(resolve(directory, 'style.js')))
const bytes = await readFile(resolve(directory, 'style_bg.wasm'))
const exports = await facade.default({module_or_path:bytes})
assert.equal(exports.memory.buffer instanceof SharedArrayBuffer, false)
const limits = {input_bytes:262144,asset_bytes:262144,candidate_count:1024,ast_depth:32,
  css_bytes:262144,queued_bytes:262144,cache_bytes:16000000}
const snapshot = value => JSON.parse(JSON.stringify(value))
const input = '@theme static { --root: root; } @import "a.css"; @tailwind utilities;'
let sourceCases = 0, maps = 0, sourceIdentityChecks = 0
for (const value of ['�','\ud800']) {
  const content = `@theme static { --first: red; --second: blue; --lone: ${value}; }`
  for (const path of ['/virtual/a.css','',undefined]) {
  const assets = [{id:'a.css',...(path === undefined ? {} : {path}),base:'/virtual',content}]
  for (const from of [undefined,'','/virtual/entry.css']) {
    const options = {base:'/virtual',...(from === undefined ? {} : {from})}
    const config = JSON.stringify({options,assets,limits})
    const officialOptions = {...options, loadStylesheet:async () => ({...assets[0],path:assets[0].path ?? assets[0].id})}
    const official = await loadDesignSystem(input, officialOptions)
    const design = new facade.DesignSystem(input, config)
    try {
      const entries = design.themeEntries()
      assert.deepEqual(snapshot(entries), snapshot(Array.from(official.theme.entries())))
      assert.equal(entries[0][1].src, undefined, 'DesignSystem root CSS is untracked')
      if (from !== undefined && path !== '') {
        const first = entries[1][1].src[0]
        assert.equal(first.code, content)
        assert.equal(first.file, path ?? 'a.css')
        assert.equal(entries[2][1].src[0], first)
        assert.equal(entries[3][1].src[0], first)
        assert.equal(design.themeEntries()[1][1].src[0], first)
        sourceIdentityChecks += 3
      } else {
        for (const [,token] of entries) assert.equal(token.src, undefined)
      }
      sourceCases++
    } finally { design.free() }
    const reference = await compile(input, officialOptions)
    const compiler = new facade.CssCompiler(input, config)
    try {
      assert.deepEqual(snapshot(compiler.buildSourceMap()), snapshot(reference.buildSourceMap()), 'initial map')
      assert.equal(compiler.build([]), reference.build([]))
      const map = compiler.buildSourceMap()
      assert.deepEqual(snapshot(map), snapshot(reference.buildSourceMap()), 'built map')
      for (const mapping of map.mappings) {
        if (mapping.originalPosition) assert(map.sources.includes(mapping.originalPosition.source))
      }
      maps += 2
    } finally { compiler.free() }
  }
}
}
let budgetCases = 0
const bounded = new facade.DesignSystem('@theme { --x: red; --y: blue; } @tailwind utilities;', JSON.stringify({limits:{...limits,cache_bytes:16384}}))
try {
  assert.equal(bounded.themeEntries().length, 2)
  const cached = bounded.parseCandidate('block')
  let rejected = false
  for (let index = 0; index < 128; index++) {
    bounded.parseCandidate(`unknown-${index}`)
    try { bounded.themeEntries() } catch (error) {
      assert(error instanceof Error)
      assert.equal(error.cause.error.kind, 'limit')
      assert.equal(error.cause.error.resource, 'cache_bytes')
      rejected = true
      break
    }
  }
  assert.equal(rejected, true, 'retained JS caches must enter entry preflight')
  assert.equal(bounded.parseCandidate('block'), cached, 'failed snapshot preserves cached JS identity')
  budgetCases++
} finally { bounded.free() }
// Reject the first source snapshot at a bounded limit; cached parser identity
// and a different small call remain usable without publishing Source storage.
let initialSourceLimit = false
// Keep the Source crossing larger than a new typed candidate's parse/crossing peak.
const sourceContent = `@theme static { --x: red; } /*! ${'x'.repeat(4096)} */`
for (const cache_bytes of [2048,4096,8192,16384,32768,65536]) {
  let design
  try {
    design = new facade.DesignSystem('@import "a.css"; @tailwind utilities;', JSON.stringify({
      options:{from:'entry.css'}, assets:[{id:'a.css',path:'a.css',content:sourceContent}],
      limits:{...limits,cache_bytes},
    }))
    const cached = design.parseCandidate('block')
    try { design.themeEntries() } catch (error) {
      assert.equal(error.cause.error.resource, 'cache_bytes')
      assert.equal(design.parseCandidate('block'), cached)
      assert(Array.isArray(design.parseCandidate('flex')), 'new small calls recover after failed first Source snapshot')
      initialSourceLimit = true
      budgetCases++
      break
    }
  } catch (error) {
    assert.equal(error.cause.error.resource, 'cache_bytes')
  } finally { design?.free() }
}
assert.equal(initialSourceLimit, true)
assert.equal(sourceCases, 18)
assert.equal(maps, 36)
assert.equal(sourceIdentityChecks, 24)
console.log(JSON.stringify({sourceCases,maps,sourceIdentityChecks,budgetCases,actualWasm:true,
  sharedMemory:false,wasmBytes:bytes.length,fullPublicValueConformance:'pending'}))
