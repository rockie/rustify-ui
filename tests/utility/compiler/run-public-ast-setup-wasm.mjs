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
for (const name of ['index.ts','ast.ts','theme.ts']) {
  const path = `packages/tailwindcss/src/${name}`
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
const decl = (property,value) => ({kind:'declaration',property,value,important:false})
const rule = (selector,nodes=[]) => ({kind:'rule',selector,nodes})
const at = (name,params='',nodes=[]) => ({kind:'at-rule',name,params,nodes})
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
  const source = {file:'theme\ud800.css',code:'theme\udc00',extra:'kept'}
  const themes = spec.themes.map(([params,entries],index) => ({...at('@theme',params,entries.map(([key,value]) =>
    ({...decl(key,value),...(spec.source ? {src:[source,index+0.5,index+8]} : {})}))),
    ...(spec.source ? {src:[source,index+0.5,index+9]} : {}),extra:'retained'}))
  const oldChildren = themes.map(theme => theme.nodes)
  if (spec.keyframes) themes[0].nodes.push(at('@keyframes','pulse',[rule('to',[decl('opacity','0')])]))
  const nested = spec.wrap ? [at('@media','screen',[...themes,rule('.sibling',[decl('color','red')])])] : themes
  const input = spec.nested ? [rule('.nested',[{kind:'context',context:{probe:true},nodes:nested}])] : spec.wrap ? nested : [...themes]
  if (spec.utilities !== false) input.push(at('@tailwind','utilities'))
  if (spec.frozen) { themes.forEach(theme => Object.freeze(theme)); oldChildren.forEach(nodes => Object.freeze(nodes)); Object.freeze(source) }
  return {input,roots:[input,...themes,...oldChildren,source]}
}
const cases = [
  {id:'plain',themes:[['',[['--color-card','red'],['--spacing','0.25rem']]]]},
  {id:'no-features',themes:[['',[['--color-card','red']]]],utilities:false},
  {id:'merge-default-reference',themes:[['',[['--color-card','red']]],['default',[['--color-card','blue'],['--spacing','0.25rem']]],['reference',[['--color-muted','gray']]]]},
  {id:'inline-static',themes:[['inline',[['--color-card','red']]],['static',[['--color-unused','blue']]]]},
  {id:'prefix-escaped',themes:[['prefix(tw)',[['--color-c\\61 rd','red'],['--spacing','0.25rem']]]],batches:[[],['tw:bg-card'],['tw:m-2'],[]]},
  {id:'clear-namespace',themes:[['',[['--color-card','red'],['--color-muted','gray'],['--spacing','0.25rem']]],['',[['--color-*','initial'],['--color-card','blue']]]]},
  {id:'nested-context',nested:true,themes:[['',[['--color-card','red']]],['',[['--spacing','0.25rem']]]]},
  {id:'theme-media-ancestry',wrap:true,themes:[['',[['--color-card','red']]]]},
  {id:'theme-style-media-ancestry',wrap:true,nested:true,themes:[['',[['--color-card','red']]]]},
  {id:'empty-theme',nested:true,themes:[['',[]]]},
  {id:'theme-sort-marker',nested:true,themes:[['',[['--tw-sort','display'],['--color-card','red']]]]},
  {id:'static-sort-marker',nested:true,themes:[['static',[['--tw-sort','display']]],['',[['--color-card','red']]]]},
  {id:'source',source:true,options:{from:'entry.css'},themes:[['',[['--color-card','red'],['--spacing','0.25rem']]],['default',[['--color-card','blue']]]]},
  {id:'source-empty-from',source:true,options:{from:''},themes:[['',[['--color-card','red']]]]},
  {id:'frozen-old-theme',source:true,frozen:true,themes:[['',[['--color-card','red'],['--spacing','0.25rem']]]]},
  {id:'function-value',themes:[['',[['--spacing','0.25rem'],['--color-card','--alpha(red / 50%)']]]]},
  {id:'non-scalar-name',themes:[['',[['--color-card\ud800','red']]]],batches:[[],['bg-card\ud800'],['block'],[]]},
  {id:'keyframes',keyframes:true,themes:[['',[['--animate-pulse','pulse 1s infinite']]]],batches:[[],['animate-pulse'],[],['block']]},
]
let phases=0, noOps=0
for (const spec of cases) {
  const actual=fixture(spec), expected=fixture(spec)
  const reference=await tailwind.compileAst(expected.input,spec.options)
  const compiler=new facade.AstCompiler(actual.input,JSON.stringify({limits,options:spec.options}))
  try {
    assert.deepEqual(snapshot(actual.roots),snapshot(expected.roots),`${spec.id}: constructor graph`); phases++
    let previous,wantedPrevious
    for (const batch of spec.batches ?? [[],['bg-card'],['m-2'],[],['block']]) {
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
  const marker=new Error(`${operation} setup`), calls=[]
  function make() {
    const value=fixture({themes:[['',[['--color-card','red']]],['',[['--spacing','0.25rem']]]]})
    if (operation==='index') {
      const original=value.input[0]
      Object.defineProperty(value.input,'0',{enumerable:true,configurable:true,get:() => original,
        set:next => { calls.push([next.kind,next.selector,next.nodes.length,Object.hasOwn(next,'src')]); throw marker }})
    } else {
      Object.defineProperty(value.input,'splice',{value:() => { throw marker },configurable:true})
    }
    return value
  }
  const expected=make(), actual=make()
  await assert.rejects(tailwind.compileAst(expected.input),error => error===marker)
  assert.throws(() => new facade.AstCompiler(actual.input,JSON.stringify({limits})),error => error===marker)
  assert.deepEqual(snapshot(actual.roots),snapshot(expected.roots),`${operation}: failed constructor prefix`)
  if (operation==='index') { assert.deepEqual(calls[0],['rule',':root, :host',0,true]); assert.deepEqual(calls[1],calls[0]) }
  exceptions++
}
const budgetSpec={source:true,themes:[['',[['--color-card','red'],['--spacing','0.25rem']]],['default',[['--color-card','blue']]]]}
function construct(cacheBytes) {
  const value=fixture(budgetSpec), before=snapshot(value.roots)
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
console.log(JSON.stringify({themeSetupCases:cases.length,constructorAndBuildPhases:phases,noOpIdentities:noOps,
  originalConstructorExceptions:exceptions,cacheBoundaries:3,constructorCacheThreshold:low,
  actualWasm:true,sharedMemory:false,wasmBytes:bytes.length,fullInputMutationConformance:'pending'}))
