import assert from 'node:assert/strict'
import {createHash} from 'node:crypto'
import {readFile} from 'node:fs/promises'
import {createRequire} from 'node:module'
import {resolve} from 'node:path'
import {pathToFileURL} from 'node:url'
const oracle=createRequire(resolve('tests/utility/compiler/oracle/package.json'))
const tailwind=oracle('tailwindcss')
assert.equal(oracle('tailwindcss/package.json').version,'4.1.13')
const inventory=JSON.parse(await readFile('tests/utility/compiler/inventory.json','utf8'))
assert.equal(inventory.baseline.commit,'1334c99db8fd26c8ea065375dd9259800863f072')
for (const name of ['index.ts','ast.ts','apply.ts','utilities.ts','css-functions.ts']) {
  const path=`packages/tailwindcss/src/${name}`
  assert.equal(createHash('sha256').update(await readFile(resolve('ref/tailwindcss',path))).digest('hex'),
    inventory.sources.find(source => source.path===path)?.sha256,path)
}
const directory=resolve(process.argv[2] ?? 'target/utility-conformance/compiler-api')
const facade=await import(pathToFileURL(resolve(directory,'style.js')))
const bytes=await readFile(resolve(directory,'style_bg.wasm'))
await facade.default({module_or_path:bytes})
const limits={input_bytes:262144,asset_bytes:262144,candidate_count:1024,ast_depth:32,css_bytes:262144,queued_bytes:262144,cache_bytes:16000000}
const decl=(property,value) => ({kind:'declaration',property,value,important:false})
const at=(name,params='',nodes=[]) => ({kind:'at-rule',name,params,nodes})
const rule=(selector,nodes) => ({kind:'rule',selector,nodes})
function graph(roots) {
  const seen=new Map(),records=[]
  function visit(value) {
    if (value===undefined) return {undefined:true}
    if (value===null || typeof value!=='object') return value
    if (seen.has(value)) return {reference:seen.get(value)}
    const id=records.length
    seen.set(value,id);records.push(null)
    records[id]={array:Array.isArray(value),fields:Object.keys(value).map(key => [key,visit(value[key])])}
    return {reference:id}
  }
  return {roots:roots.map(visit),records}
}
function fixture(kind) {
  const writes=[]
  const first=decl('color',kind==='non-scalar' ? '--alpha(\ud800 / 50%)' : '--alpha(red / 50%)')
  let value=first.value
  Object.defineProperty(first,'value',{enumerable:true,get:()=>value,set:next=>{writes.push(next);value=next}})
  const bad=decl('padding','--spacing(2)')
  const body=kind==='alias' ? [first,first,bad] : [first,bad]
  let input=body,assets=[],options={}
  switch (kind) {
    case 'nested': input=[rule('.one',body)]; break
    case 'theme': input=[at('@theme','',[decl('--color-test','red')]),...body]; break
    case 'multiple-themes': input=[at('@theme','',[decl('--color-test','red')]),at('@theme','default',[decl('--color-other','blue')]),...body]; break
    case 'custom': input=[at('@utility','tone-*',[decl('color','--value([ *])')]),at('@utility','card',[decl('color','--alpha(blue / 25%)')]),...body]; break
    case 'custom-failure': input=[at('@utility','card',body)]; break
    case 'params': input=[at('@supports','(color: --alpha(blue / 25%))',body)]; break
    case 'import':
    case 'import-failure': {
      input=[at('@import','"one.css"'),...body]
      options={base:'/discover',from:'entry.css'}
      assets=[{id:'one.css',path:'/mapped/one.css',base:'/discover',content:kind==='import' ? '.loaded { color: red; }' : '.loaded { color: --alpha(blue / 25%); padding: --spacing(2); }'}]
      break
    }
    case 'source': {
      const source={file:'entry\ud800.css',code:'theme\udc00',extra:'retained'}
      const theme=at('@theme','',[{...decl('--color-test','red'),src:[source,2.5,9]}])
      theme.src=[source,1,10]
      input=[theme,...body]
      break
    }
  }
  return {input,writes,first,bad,assets,options,roots:[input,body,first,bad,writes,...input,...input.filter(node=>Array.isArray(node.nodes)).map(node=>node.nodes)]}
}
const cases=['plain','nested','alias','theme','multiple-themes','custom','params','custom-failure','non-scalar','source','import','import-failure']
for (const kind of cases) {
  const expected=fixture(kind),actual=fixture(kind)
  let wanted,received
  try {await tailwind.compileAst(expected.input,{...expected.options,...(expected.assets.length ? {loadStylesheet:async() => expected.assets[0]} : {})})} catch(error) {wanted=error}
  try {new facade.AstCompiler(actual.input,JSON.stringify({limits,assets:actual.assets,options:actual.options}))} catch(error) {received=error}
  assert(wanted && received,`${kind}: semantic failure`)
  assert.equal(received.name,wanted.name);assert.equal(received.message,wanted.message)
  assert.deepEqual(graph(actual.roots),graph(expected.roots),`${kind}: failed function prefix`)
}
let hostFailures=0
for (const kind of ['plain','theme','custom-failure','second-write']) {
  const marker=new Error(`original ${kind} setter`)
  function throwing() {
    const value=fixture(kind)
    let target=value.first
    if (kind==='second-write') {
      target=decl('background','--alpha(blue / 25%)')
      value.input.splice(1,0,target)
      value.roots.push(target)
    }
    const original=target.value
    Object.defineProperty(target,'value',{enumerable:true,get:()=>original,set:()=>{throw marker}})
    return value
  }
  const expected=throwing(),actual=throwing()
  await assert.rejects(tailwind.compileAst(expected.input),error=>error===marker)
  assert.throws(()=>new facade.AstCompiler(actual.input,JSON.stringify({limits})),error=>error===marker)
  assert.deepEqual(graph(actual.roots),graph(expected.roots),`${kind}: original host exception and completed prefix`)
  hostFailures++
}
// Locate the admitted semantic-error boundary; every quota refusal must leave
// the entire caller graph intact, including earlier registration/function writes.
function construct(cacheBytes) {
  const value=fixture('custom'),before=graph(value.roots)
  let received
  try {new facade.AstCompiler(value.input,JSON.stringify({limits:{...limits,cache_bytes:cacheBytes}}))} catch(error) {received=error}
  assert(received,'constructor must fail')
  if (received.cause?.error?.kind==='limit') {
    assert.equal(received.cause.error.resource,'cache_bytes')
    assert.deepEqual(graph(value.roots),before,'quota refusal has no caller writes')
    return false
  }
  assert.match(received.message,/--spacing/)
  assert.equal(value.first.value,'color-mix(in oklab, red 50%, transparent)')
  assert.equal(value.writes.length,1)
  return true
}
let low=1,high=1000000
assert.equal(construct(high),true)
while (low<high) {const middle=Math.floor((low+high)/2);if (construct(middle)) high=middle;else low=middle+1}
assert.equal(construct(low-1),false);assert.equal(construct(low),true);assert.equal(construct(low+1),true)
const small=fixture('plain')
assert.throws(()=>new facade.AstCompiler(small.input,JSON.stringify({limits:{...limits,cache_bytes:64}})),error=>error.cause?.error?.kind==='limit')
assert.deepEqual(small.writes,[]);assert.equal(small.first.value,'--alpha(red / 50%)')
console.log(JSON.stringify({functionFailureCases:cases.length,originalHostExceptionCases:hostFailures,
  cacheBoundaries:3,semanticFailureCacheThreshold:low,quotaNoWriteCases:1,
  actualWasm:true,sharedMemory:false,wasmBytes:bytes.length,fullFailedSetupConformance:'pending'}))
