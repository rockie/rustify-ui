import assert from 'node:assert/strict'
import {createHash} from 'node:crypto'
import {readFile} from 'node:fs/promises'
import {createRequire} from 'node:module'
import {resolve} from 'node:path'
import {pathToFileURL} from 'node:url'

const require = createRequire(resolve('tests/utility/compiler/oracle/package.json'))
assert.equal(require('tailwindcss/package.json').version, '4.1.13')
const {compile} = require('tailwindcss')
const inventory = JSON.parse(await readFile('tests/utility/compiler/inventory.json', 'utf8'))
assert.equal(inventory.baseline.commit, '1334c99db8fd26c8ea065375dd9259800863f072')
for (const path of ['packages/tailwindcss/src/index.ts', 'packages/tailwindcss/src/css-parser.ts',
  'packages/tailwindcss/src/css-functions.ts', 'packages/tailwindcss/src/compat/plugin-functions.ts']) {
  assert.equal(createHash('sha256').update(await readFile(resolve('ref/tailwindcss', path))).digest('hex'), inventory.sources.find(source => source.path === path)?.sha256, path)
}
const directory = resolve('target/utility-conformance/compiler-api')
const facade = await import(pathToFileURL(resolve(directory, 'style.js')))
const bytes = await readFile(resolve(directory, 'style_bg.wasm'))
const exports = await facade.default({module_or_path:bytes})
assert.equal(exports.memory.buffer instanceof SharedArrayBuffer, false)
const limits = {input_bytes:65536,asset_bytes:65536,candidate_count:1024,ast_depth:32,
  css_bytes:262144,queued_bytes:65536,cache_bytes:8000000}
const units = text => Array.from({length:text.length}, (_, index) => text.charCodeAt(index))
const snapshot = value => JSON.parse(JSON.stringify(value))
function sourceMapSnapshot(map) {
  const references = map.mappings.map(mapping => {
    if (!mapping.originalPosition) return null
    const index = map.sources.indexOf(mapping.originalPosition.source)
    assert.notEqual(index, -1, 'mapping retains the decoded Source table object')
    return index
  })
  return {decoded:snapshot(map),references}
}
let cssCases = 0, maps = 0, mappings = 0
// Scalar source contains an astral token; upstream legacy indexing produces a
// lone unit in an at-rule parameter. This exercises production AST substitution
// and map printing; the full source/candidate matrix remains pending.
for (const name of ['@media','@custom-media','@container','@supports']) for (const index of [0,1]) {
  const expression = `theme(padding.card.${index})`
  const rule = name === '@custom-media'
    ? `${name} --probe (width: ${expression}); .p { value:red; }`
    : `${name} (width: ${expression}) { .p { value:red; } }`
  const input = `@theme { --spacing-card: 😀; } ${rule} @tailwind utilities;`
  const options = {from:`/ast/${name.slice(1)}-${index}.css`}
  const official = await compile(input, options)
  const expected = official.build([])
  assert(units(expected).includes(index === 0 ? 0xd83d : 0xde00), 'oracle produces the original lone unit')
  const api = new facade.CssCompiler(input, JSON.stringify({limits,options}))
  try {
    assert.deepEqual(units(api.build([])), units(expected), `${name}:${index}: CSS`)
    cssCases++
    const actualMap = api.buildSourceMap()
    assert.deepEqual(sourceMapSnapshot(actualMap), sourceMapSnapshot(official.buildSourceMap()), `${name}:${index}: source map`)
    maps++; mappings += actualMap.mappings.length
    assert.deepEqual(units(api.build([])), units(expected), 'no-op retains prior text')
  } finally { api.free() }
}
assert.equal(cssCases, 8)
assert.equal(maps, 8)
console.log(JSON.stringify({cssCases,maps,mappings,actualWasm:true,sharedMemory:false,wasmBytes:bytes.length,
  fullSourceCandidateIngress:'pending',fullUtf16Conformance:'pending',fullPublicValueConformance:'pending'}))
