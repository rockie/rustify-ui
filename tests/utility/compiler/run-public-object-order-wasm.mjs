import assert from 'node:assert/strict'
import {createHash} from 'node:crypto'
import {readFile} from 'node:fs/promises'
import {createRequire} from 'node:module'
import {resolve} from 'node:path'
import {pathToFileURL} from 'node:url'

const oracle = createRequire(resolve('tests/utility/compiler/oracle/package.json'))
const tailwind = oracle('tailwindcss')
assert.equal(oracle('tailwindcss/package.json').version, '4.1.13')
const provenance = JSON.parse(await readFile('crates/rustify-style/upstream/tailwind/PROVENANCE.json','utf8'))
assert.equal(provenance.commit, '1334c99db8fd26c8ea065375dd9259800863f072')
for (const path of ['candidate.ts','ast.ts','compile.ts','design-system.ts']) {
  const source = `packages/tailwindcss/src/${path}`
  assert.equal(createHash('sha256').update(await readFile(resolve('ref/tailwindcss',source))).digest('hex'),
    provenance.sourceDigests[source],source)
}
const directory = resolve('target/utility-conformance/compiler-api')
const facade = await import(pathToFileURL(resolve(directory,'style.js')))
const bytes = await readFile(resolve(directory,'style_bg.wasm'))
const exports = await facade.default({module_or_path:bytes})
assert.equal(exports.memory.buffer instanceof SharedArrayBuffer,false)
const limits = {input_bytes:262144,asset_bytes:262144,candidate_count:1024,ast_depth:32,
  css_bytes:262144,queued_bytes:262144,cache_bytes:16000000}
const config = JSON.stringify({limits})
let objects = 0
function compareKeys(actual, expected, path) {
  if (expected === null || typeof expected !== 'object') return
  assert.deepEqual(Object.keys(actual),Object.keys(expected),path)
  objects++
  for (const key of Object.keys(expected)) compareKeys(actual[key],expected[key],`${path}.${key}`)
}
const input = '@theme { --color-card: red; --spacing: 0.25rem; } @tailwind utilities;'
const expected = await tailwind.__unstable__loadDesignSystem(input)
const actual = new facade.DesignSystem(input,config)
try {
  for (const raw of ['block','hover:block','bg-card/50','content-[\'\ud800\']','[--probe:\udc00]',
    'group-hover/name:block','data-[probe]:block','w-[length:2px]']) {
    const parsed = actual.parseCandidate(raw), reference = expected.parseCandidate(raw)
    compareKeys(parsed,reference,raw)
    for (let index=0;index<reference.length;index++) {
      compareKeys(actual.compileAstNodes(parsed[index],0),expected.compileAstNodes(reference[index],0),`${raw}.ast`)
    }
  }
  for (const raw of ['hover','[&.\ud800]','group-hover/name','data-[probe]','min-[20px]']) {
    compareKeys(actual.parseVariant(raw),expected.parseVariant(raw),`variant:${raw}`)
  }
} finally { actual.free() }
const source = [{kind:'rule',selector:'.probe',nodes:[{kind:'declaration',property:'--x',value:'\ud800',important:false}],
  z:1,'\ud800':2,a:3,'10':4,'2':5,'01':6,'4294967295':7,'__proto__':8}]
// Define an own __proto__ key without object-literal prototype semantics.
Object.defineProperty(source[0],'__proto__',{value:8,enumerable:true,configurable:true,writable:true})
const ast = new facade.AstCompiler(source,config)
const expectedAst = await tailwind.compileAst(source)
try { compareKeys(ast.build([]),expectedAst.build([]),'authoredAST') } finally { ast.free() }
console.log(JSON.stringify({objects,actualWasm:true,sharedMemory:false,wasmBytes:bytes.length,
  fullPublicValueConformance:'pending'}))
