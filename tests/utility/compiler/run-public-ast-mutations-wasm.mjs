import assert from 'node:assert/strict'
import {createHash} from 'node:crypto'
import {createRequire} from 'node:module'
import {readFile} from 'node:fs/promises'
import {resolve} from 'node:path'
import {pathToFileURL} from 'node:url'

const oracle = createRequire(resolve('tests/utility/compiler/oracle/package.json'))
const tailwind = oracle('tailwindcss')
assert.equal(oracle('tailwindcss/package.json').version, '4.1.13')
const inventory = JSON.parse(await readFile('tests/utility/compiler/inventory.json','utf8'))
assert.equal(inventory.baseline.commit,'1334c99db8fd26c8ea065375dd9259800863f072')
for (const file of ['index.ts','ast.ts','compile.ts']) {
  const path = `packages/tailwindcss/src/${file}`
  assert.equal(createHash('sha256').update(await readFile(resolve('ref/tailwindcss',path))).digest('hex'),
    inventory.sources.find(source => source.path === path)?.sha256,path)
}
const directory = resolve('target/utility-conformance/compiler-api')
const facade = await import(pathToFileURL(resolve(directory,'style.js')))
const bytes = await readFile(resolve(directory,'style_bg.wasm'))
const exports = await facade.default({module_or_path:bytes})
assert.equal(exports.memory.buffer instanceof SharedArrayBuffer,false)
const limits = {input_bytes:262144,asset_bytes:262144,candidate_count:1024,ast_depth:32,
  css_bytes:262144,queued_bytes:262144,cache_bytes:16000000}

function snapshot(roots) {
  const seen = new Map(), records = []
  function visit(value) {
    if (value === undefined) return {undefined:true}
    if (value === null || typeof value !== 'object') return value
    if (seen.has(value)) return {reference:seen.get(value)}
    const id = records.length
    seen.set(value,id); records.push(null)
    records[id] = {array:Array.isArray(value),fields:Object.keys(value).map(key => [key,visit(value[key])])}
    return {reference:id}
  }
  return {roots:roots.map(visit),records}
}
function fixture(from, frozen=false, nonConfigurable=false) {
  const source = {file:'entry\ud800.css',code:'source\udc00'}
  const utilities = {kind:'at-rule',name:'@tailwind',params:'utilities',nodes:[],
    ...(from ? {src:[source,1.5,9]} : {}),extra:'retained'}
  if (frozen) { Object.freeze(source); Object.freeze(utilities.src) }
  if (nonConfigurable) Object.defineProperty(utilities,'kind',{configurable:false})
  return {input:[utilities],utilities,children:utilities.nodes,source}
}
const sequences = [
  {id:'plain',batches:[[],['block'],['flex'],[],['block'],['unknown']]},
  {id:'source',from:'entry.css',batches:[['unknown'],['hover:block'],['border-2'],[],['flex']]},
  {id:'empty-from',from:'',batches:[['block'],['hover:flex'],[]]},
  {id:'raw-sort',batches:[['[--tw-sort:display]'],['block'],[]]},
  {id:'raw-resources',batches:[['before:block'],['border-2'],['hover:flex'],[]]},
  {id:'raw-empty-wrapper',batches:[['[&]:block'],['[&]:flex'],[]]},
  {id:'frozen-metadata',from:'entry.css',frozen:true,batches:[['border-2'],['hover:block'],[]]},
  {id:'non-configurable-writable-kind',nonConfigurable:true,batches:[['block'],['flex'],[]]},
]
let phases = 0, noOps = 0
for (const spec of sequences) {
  const expected = fixture(spec.from,spec.frozen,spec.nonConfigurable), actual = fixture(spec.from,spec.frozen,spec.nonConfigurable)
  const options = spec.from === undefined ? {} : {from:spec.from}
  const reference = await tailwind.compileAst(expected.input,options)
  const compiler = new facade.AstCompiler(actual.input,JSON.stringify({limits,options}))
  const a = [actual.input,actual.utilities,actual.children,actual.source]
  const b = [expected.input,expected.utilities,expected.children,expected.source]
  try {
    assert.deepEqual(snapshot(a),snapshot(b),`${spec.id}: constructor input graph`)
    phases++
    let previous, previousExpected
    for (const batch of spec.batches) {
      const result = compiler.build(batch), wanted = reference.build(batch)
      a.push(result); b.push(wanted)
      assert.deepEqual(snapshot(a),snapshot(b),`${spec.id}: build ${JSON.stringify(batch)} input/output graph`)
      if (wanted === previousExpected) { assert.equal(result,previous); noOps++ }
      previous=result; previousExpected=wanted; phases++
    }
    assert.equal(actual.children.length,0,'replaced utilities nodes leave the original array untouched')
    assert.equal(actual.utilities.extra,'retained')
  } finally { compiler.free() }
}
let exceptions = 0
for (const field of ['kind','context']) {
  const marker = new Error(`${field} setter`)
  const make = () => {
    const value = fixture()
    Object.defineProperty(value.utilities,field,{enumerable:true,configurable:true,
      get:() => field === 'kind' ? 'at-rule' : undefined,set:() => { throw marker }})
    return value
  }
  const expected = make(), actual = make()
  await assert.rejects(tailwind.compileAst(expected.input),error => error === marker)
  assert.throws(() => new facade.AstCompiler(actual.input,JSON.stringify({limits})),error => error === marker)
  assert.deepEqual(snapshot([actual.input]),snapshot([expected.input]))
  exceptions++
}
{
  const marker = new Error('nodes setter')
  function make() {
    const value = fixture(), calls = []
    let children = value.children, fail = false
    Object.defineProperty(value.utilities,'nodes',{enumerable:true,configurable:true,
      get:() => children,set:next => { calls.push(next.length); if (fail) throw marker; children=next }})
    return {...value,calls,fail:next => { fail=next }}
  }
  const expected = make(), actual = make()
  const reference = await tailwind.compileAst(expected.input)
  const compiler = new facade.AstCompiler(actual.input,JSON.stringify({limits}))
  try {
    const a = reference.build(['block']), b = compiler.build(['block'])
    expected.fail(true); actual.fail(true)
    assert.throws(() => reference.build(['flex']),error => error === marker)
    assert.throws(() => compiler.build(['flex']),error => error === marker)
    assert.equal(reference.build([]),a); assert.equal(compiler.build([]),b)
    assert.deepEqual(snapshot([actual.input,b]),snapshot([expected.input,a]))
    expected.fail(false); actual.fail(false)
    const c = reference.build(['grid']), d = compiler.build(['grid'])
    assert.deepEqual(snapshot([actual.input,b,d]),snapshot([expected.input,a,c]))
    assert.deepEqual(actual.calls,expected.calls)
    assert.deepEqual(actual.calls,[1,2,3])
    exceptions++; noOps++
  } finally { compiler.free() }
}
let boundaries = 0
function attempt(cacheBytes) {
  const value = fixture()
  let compiler
  try {
    compiler = new facade.AstCompiler(value.input,JSON.stringify({limits:{...limits,cache_bytes:cacheBytes}}))
    compiler.build(['block']); compiler.build(['flex'])
    return true
  } catch (error) {
    assert(error.cause?.error?.kind === 'limit' && error.cause.error.resource === 'cache_bytes')
    return false
  } finally { compiler?.free() }
}
let low=1, high=262144
assert(attempt(high))
while (low < high) {
  const mid=Math.floor((low+high)/2)
  if (attempt(mid)) high=mid
  else low=mid+1
}
assert(!attempt(low-1)); assert(attempt(low)); assert(attempt(low+1)); boundaries+=3
{
  const value = fixture(), compiler = new facade.AstCompiler(value.input,JSON.stringify({limits:{...limits,cache_bytes:low-1}}))
  try {
    const first=compiler.build(['block']), oldChildren=value.utilities.nodes
    const before=snapshot([value.input,first])
    assert.throws(() => compiler.build(['flex']),error => error.cause?.error?.kind === 'limit' && error.cause.error.resource === 'cache_bytes')
    assert.equal(value.utilities.nodes,oldChildren)
    assert.deepEqual(snapshot([value.input,first]),before)
    assert.equal(compiler.build([]),first)
    boundaries++; noOps++
  } finally { compiler.free() }
}
console.log(JSON.stringify({actualWasm:true,sharedMemory:false,wasmBytes:bytes.length,
  utilitiesMutationCases:sequences.length,constructorAndBuildPhases:phases,noOpIdentities:noOps,
  originalSetterExceptions:exceptions,cacheBoundaries:boundaries,twoBuildCacheThreshold:low,
  fullInputMutationConformance:'pending'}))
