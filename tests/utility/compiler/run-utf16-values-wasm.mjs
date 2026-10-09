import assert from 'node:assert/strict'
import {createHash} from 'node:crypto'
import {readFile} from 'node:fs/promises'
import {createRequire} from 'node:module'
import {resolve} from 'node:path'
import {pathToFileURL} from 'node:url'

const root = resolve('.')
const require = createRequire(resolve('tests/utility/compiler/oracle/package.json'))
assert.equal(require('tailwindcss/package.json').version, '4.1.13')
const {createJiti} = require('jiti')
const provenance = JSON.parse(await readFile(resolve('crates/rustify-style/upstream/tailwind/PROVENANCE.json'), 'utf8'))
assert.equal(provenance.commit, '1334c99db8fd26c8ea065375dd9259800863f072')
for (const source of ['packages/tailwindcss/src/value-parser.ts', 'packages/tailwindcss/src/utils/decode-arbitrary-value.ts', 'packages/tailwindcss/src/utils/math-operators.ts']) {
  const digest = createHash('sha256').update(await readFile(resolve('ref/tailwindcss', source))).digest('hex')
  assert.equal(digest, provenance.sourceDigests[source], source)
}
const jiti = createJiti(import.meta.url, {moduleCache:false, fsCache:false})
const {decodeArbitraryValue} = await jiti.import(resolve(root, 'ref/tailwindcss/packages/tailwindcss/src/utils/decode-arbitrary-value.ts'))
const directory = resolve('target/utility-conformance/compiler-api')
const facade = await import(pathToFileURL(resolve(directory, 'style.js')))
const exports = await facade.default({module_or_path: await readFile(resolve(directory, 'style_bg.wasm'))})
assert.equal(typeof facade.CssCompiler.prototype.decodeArbitraryValue, 'function')
assert.equal(exports.memory.buffer instanceof SharedArrayBuffer, false)
const limits = {input_bytes:262144,asset_bytes:65536,candidate_count:1024,ast_depth:32,
  css_bytes:262144,queued_bytes:65536,cache_bytes:8000000}
const config = overrides => JSON.stringify({limits:{...limits,...overrides}})
const units = text => Array.from({length:text.length}, (_, index) => text.charCodeAt(index))
function fromUnits(value) {
  let output = ''
  for (let index = 0; index < value.length; index += 8192) output += String.fromCharCode(...value.slice(index, index + 8192))
  return output
}
const fixture = JSON.parse(await readFile(new URL('./utf16-fixtures.json', import.meta.url), 'utf8'))
const samples = fixture.cases.filter(item => item.id.startsWith('utf16:authored:value:')).map(item => {
  const css = fromUnits(item.inputUtf16Units)
  const prefix = '.p { value: '
  assert(css.startsWith(prefix), item.id)
  return [item.id, css.slice(prefix.length, css.indexOf(';', prefix.length))]
})
assert.equal(samples.length, 9)
const patterns = [
  '{u}_x\\_y', 'url({u}_x\\_y)', '_{u}_url({u}_x\\_y)',
  'var(--{u}_x,_{u})', 'theme(padding.{u}_x,_{u})',
  'var(--{u}\\_1_5,_var(--x_2_5,_{u}))',
  'theme(padding.{u}\\_1_5,_theme(padding.x_2_5,_{u}))',
  '_{u}_fn(_{u}_,_{u})', '_{u}_var(--x_1_5,_{u})',
  '_{u}_theme(padding.x_1_5,_{u})', 'var(_{u},_{u})',
  'outer("{u}_x",inner({u}\\_y))',
  'calc({u}+2)', 'calc(2+{u})', 'calc(var(--{u}_x)*100)',
  'min({u},calc(100%-20px))', 'max({u},1+2)',
  'clamp(-3px+4px,var(--{u}_x),-3px+4px)',
  'calc(1px*(7--12/24)+var(--{u}_x))',
  'min(-3.4e-2-var(--{u}_x),calc-size(auto))',
  'clamp(-10e3-var(--{u}_x),calc-size(max-content),var(--{u}_x)+-10e3)',
  'calc(var(--{u}_x)*calc(100%+50%))',
  'calc(env(safe-area-{u},calc(10px+20px))+5px)',
  'min(theme(spacing.{u}_bar),fit-content,calc(20*calc(40-30)))',
  'minmax({u}-content,25%)', 'round(to-zero,{u},1+3)',
  'calc({u}x+{u}y)', 'calc({u}x + {u}y)', 'atan(1_+_-infinity)_{u}',
  'fit-content(min(max-content,max(min-content,calc(20px+1em))),{u})',
  'var(--{u}_x,_calc(100%+1rem))', 'calc(({u})+(-2*-3))',
  'calc({u}1\ufeff+2)', 'calc({u}1\u0085+{u})',
]
let decodeCases = 0, boundaryCases = 0
const api = new facade.CssCompiler('', config())
try {
  for (const [id, sample] of samples) for (const pattern of patterns) {
    const input = pattern.replaceAll('{u}', sample)
    assert.deepEqual(units(api.decodeArbitraryValue(input)), units(decodeArbitraryValue(input)), `${id}:${pattern}`)
    decodeCases++
  }
  const allUnits = fromUnits(Array.from({length:65536}, (_, index) => index))
  assert.deepEqual(units(api.decodeArbitraryValue(allUnits)), units(decodeArbitraryValue(allUnits)), 'all units follow fixed decoder grammar'); boundaryCases++
  assert.throws(() => api.decodeArbitraryValue({toString(){throw Error('must not coerce')}}), thrown => thrown.cause.kind === 'protocol'); boundaryCases++
  assert.equal(api.decodeArbitraryValue('\ud800'), '\ud800'); boundaryCases++
} finally { api.free() }
const inputLimited = new facade.CssCompiler('', config({input_bytes:4, css_bytes:4}))
try {
  assert.equal(inputLimited.decodeArbitraryValue('\ud800_'), '\ud800 '); boundaryCases++
  assert.throws(() => inputLimited.decodeArbitraryValue('😀_'), thrown => thrown.cause.error.resource === 'input_bytes'); boundaryCases++
  assert.equal(inputLimited.decodeArbitraryValue('\\_'), '_'); boundaryCases++
} finally { inputLimited.free() }
const outputLimited = new facade.CssCompiler('', config({css_bytes:9,ast_depth:1}))
try {
  assert.throws(() => outputLimited.decodeArbitraryValue('calc(1+2)'), thrown => thrown.cause.error.resource === 'css_bytes'); boundaryCases++
  assert.throws(() => outputLimited.decodeArbitraryValue('outer(inner(x))'), thrown => thrown.cause.error.resource === 'ast_depth'); boundaryCases++
  assert.equal(outputLimited.decodeArbitraryValue('\udc00'), '\udc00'); boundaryCases++
} finally { outputLimited.free() }
assert.equal(decodeCases, 306)
assert.equal(boundaryCases, 9)
console.log(JSON.stringify({decodeCases,boundaryCases,allCodeUnits:65536,actualWasm:true,sharedMemory:false,fullUtf16Conformance:'pending'}))
