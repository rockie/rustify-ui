import assert from 'node:assert/strict'
import {createHash} from 'node:crypto'
import {readFile} from 'node:fs/promises'
import {createRequire} from 'node:module'
import {resolve} from 'node:path'
import {pathToFileURL} from 'node:url'

const oracle = createRequire(resolve('tests/utility/compiler/oracle/package.json'))
const tailwind = oracle('tailwindcss')
assert.equal(oracle('tailwindcss/package.json').version,'4.1.13')
const inventory = JSON.parse(await readFile('tests/utility/compiler/inventory.json','utf8'))
assert.equal(inventory.baseline.commit,'1334c99db8fd26c8ea065375dd9259800863f072')
for (const name of ['index.ts','ast.ts','apply.ts','compile.ts','utilities.ts']) {
  const path = `packages/tailwindcss/src/${name}`
  assert.equal(createHash('sha256').update(await readFile(resolve('ref/tailwindcss',path))).digest('hex'),
    inventory.sources.find(source => source.path === path)?.sha256,path)
}
const directory = resolve(process.argv[2] ?? 'target/utility-conformance/compiler-api')
const facade = await import(pathToFileURL(resolve(directory,'style.js')))
const bytes = await readFile(resolve(directory,'style_bg.wasm'))
const exports = await facade.default({module_or_path:bytes})
assert.equal(exports.memory.buffer instanceof SharedArrayBuffer,false)
const limits = {input_bytes:262144,asset_bytes:262144,candidate_count:1024,ast_depth:32,
  css_bytes:262144,queued_bytes:262144,cache_bytes:16000000}
const decl = (property,value) => ({kind:'declaration',property,value,important:false})
const at = (name,params='',nodes=[]) => ({kind:'at-rule',name,params,nodes})
const rule = (selector,nodes) => ({kind:'rule',selector,nodes})
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
function fixture(spec) {
  const source = {file:'apply\ud800.css',code:'apply\udc00',extra:'kept'}
  const shared = [source,3.5,40]
  const applies = (spec.params ?? ['block']).map((params,index) => ({...at('@apply',params),extra:'old directive',
    ...(spec.source ? {src:spec.sharedTuple ? shared : [source,index+3.5,index+40]} : {})}))
  const children = [decl('--authored','red'),...applies,decl('--after','blue')]
  if (spec.themes) children.unshift(at('@theme','',[decl('--color-card','red')]),at('@theme','default',[decl('--spacing','0.25rem')]))
  const parent = rule('.sheet',children)
  const input = spec.nested ? [rule('.outer',[{kind:'context',context:{probe:true},nodes:[at('@media','screen',[parent])]}])] : [parent]
  if (spec.utilities !== false) input.push(at('@tailwind','utilities'))
  if (spec.frozen) { applies.forEach(node => Object.freeze(node)); applies.forEach(node => Object.freeze(node.nodes)); Object.freeze(source); Object.freeze(shared) }
  return {input,children,applies,roots:[input,parent,children,...applies,...applies.map(node => node.nodes),source,shared]}
}
const cases = [
  {id:'single'},
  {id:'multiple',params:['flex block']},
  {id:'multiple-directives',params:['block','flex underline','grid']},
  {id:'duplicate-candidate-offset',source:true,params:['flex block flex']},
  {id:'whitespace-offsets',source:true,params:['flex\ufeff\tblock\nunderline']},
  {id:'variant',source:true,params:['hover:block flex']},
  {id:'sort-marker-and-shared-source',source:true,params:['size-[2px] block']},
  {id:'property-resource',source:true,params:['border-2 block']},
  {id:'non-scalar',source:true,params:['[--unit:\ud800] block']},
  {id:'nested-context',nested:true,params:['block','flex underline']},
  {id:'source-tuples-alias',source:true,sharedTuple:true,params:['block','flex underline']},
  {id:'frozen-old-directives',source:true,frozen:true,params:['flex block']},
  {id:'theme-splice-before-apply',source:true,themes:true,params:['block','flex underline']},
  {id:'no-utilities',utilities:false,params:['flex block']},
  {id:'empty-from',source:true,options:{from:''},params:['flex block']},
]
let phases=0,noOps=0
for (const spec of cases) {
  const actual=fixture(spec), expected=fixture(spec)
  const reference=await tailwind.compileAst(expected.input,spec.options)
  const compiler=new facade.AstCompiler(actual.input,JSON.stringify({limits,options:spec.options}))
  try {
    assert.deepEqual(snapshot(actual.roots),snapshot(expected.roots),`${spec.id}: constructor graph`); phases++
    let previous,wantedPrevious
    for (const batch of [[],['block'],['flex'],[],['size-[4px]']]) {
      const result=compiler.build(batch), wanted=reference.build(batch)
      actual.roots.push(result); expected.roots.push(wanted)
      assert.deepEqual(snapshot(actual.roots),snapshot(expected.roots),`${spec.id}: build ${JSON.stringify(batch)}`)
      if (wanted===wantedPrevious) { assert.equal(result,previous); noOps++ }
      previous=result; wantedPrevious=wanted; phases++
    }
  } finally { compiler.free() }
}
let exceptions=0
for (const operation of ['index','splice']) {
  const marker=new Error(`${operation} apply`), calls=[]
  function make() {
    const value=fixture({source:true,params:operation==='index' ? ['block'] : ['block','flex underline']})
    if (operation==='index') {
      const original=value.children[1]
      Object.defineProperty(value.children,'1',{enumerable:true,configurable:true,get:() => original,
        set:next => { calls.push([next.kind,next.property,next.value,Object.hasOwn(next,'src')]); throw marker }})
    } else Object.defineProperty(value.children,'splice',{value:() => { throw marker },configurable:true})
    return value
  }
  const expected=make(), actual=make()
  await assert.rejects(tailwind.compileAst(expected.input),error => error===marker)
  assert.throws(() => new facade.AstCompiler(actual.input,JSON.stringify({limits})),error => error===marker)
  assert.deepEqual(snapshot(actual.roots),snapshot(expected.roots),`${operation}: failed constructor prefix`)
  if (operation==='index') { assert.deepEqual(calls[0],['declaration','display','block',true]); assert.deepEqual(calls[1],calls[0]) }
  exceptions++
}
function construct(cacheBytes) {
  const value=fixture({source:true,params:['size-[2px] block','hover:flex border-2']}), before=snapshot(value.roots)
  try {
    const compiler=new facade.AstCompiler(value.input,JSON.stringify({limits:{...limits,cache_bytes:cacheBytes}}))
    compiler.free(); return true
  } catch (error) {
    assert.equal(error.cause.error.kind,'limit'); assert.equal(error.cause.error.resource,'cache_bytes')
    assert.deepEqual(snapshot(value.roots),before,'preflight rejects before caller mutations'); return false
  }
}
let low=1,high=1000000
assert.equal(construct(high),true)
while (low<high) { const middle=Math.floor((low+high)/2); if (construct(middle)) high=middle; else low=middle+1 }
assert.equal(construct(low-1),false); assert.equal(construct(low),true); assert.equal(construct(low+1),true)
console.log(JSON.stringify({applyInputCases:cases.length,constructorAndBuildPhases:phases,noOpIdentities:noOps,
  originalConstructorExceptions:exceptions,cacheBoundaries:3,constructorCacheThreshold:low,
  actualWasm:true,sharedMemory:false,wasmBytes:bytes.length,fullInputMutationConformance:'pending'}))
