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
for (const name of ['index.ts','ast.ts','at-import.ts','css-parser.ts','apply.ts']) {
  const path=`packages/tailwindcss/src/${name}`
  assert.equal(createHash('sha256').update(await readFile(resolve('ref/tailwindcss',path))).digest('hex'),
    inventory.sources.find(source => source.path===path)?.sha256,path)
}
assert(process.argv.length<=3)
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
const sheet=(id,content,extra={}) => ({id,path:`/mapped/${id}`,base:'/discover',content,...extra})
const fixtures=[
  ['untracked',[at('@import','"one.css"'),at('@tailwind','utilities')],[sheet('one.css','.one { color: red; }')],{}],
  ['empty-from',[at('@import','"one.css"'),at('@tailwind','utilities')],[sheet('one.css','.one { color: red; }')],{from:''}],
  ['tracked',[at('@import','"one.css"'),at('@tailwind','utilities')],[sheet('one.css','.one { color: red; }')],{from:'entry.css'}],
  ['nested',[at('@import','"one.css"'),at('@tailwind','utilities')],[sheet('one.css','@import "two.css"; .one { color: red; }'),sheet('two.css','.two { display: flex; }')],{from:'entry.css'}],
  ['repeated',[at('@import','"one.css"'),at('@import','"one.css"'),at('@tailwind','utilities')],[sheet('one.css','.one { color: red; }')],{from:'entry.css'}],
  ['wrappers',[at('@import','"one.css" layer(a) supports(display: grid) screen'),at('@tailwind','utilities')],[sheet('one.css','.one { color: red; }')],{from:'entry.css'}],
  ['wrapper-source',[{...at('@import','"one.css" layer(a)'),src:[{file:'entry.css',code:'source'},0.5,4.5]},at('@tailwind','utilities')],[sheet('one.css','.one { color: red; }')],{from:'entry.css'}],
  ['reference',[at('@reference','"one.css"'),at('@tailwind','utilities')],[sheet('one.css','@theme { --color-card: red; }')],{from:'entry.css'}],
  ['theme',[at('@import','"one.css"'),at('@theme','',[decl('--color-other','blue')]),at('@tailwind','utilities')],[sheet('one.css','@theme { --color-card: red; }')],{from:'entry.css'}],
  ['imported-utilities',[at('@import','"one.css"')],[sheet('one.css','@tailwind utilities;')],{from:'entry.css'}],
  ['custom',[at('@import','"one.css"'),at('@tailwind','utilities')],[sheet('one.css','@utility card { color: red; }')],{from:'entry.css'}],
  ['flattened-media-apply',[at('@import','"one.css"'),at('@tailwind','utilities')],[sheet('one.css','@media important { @apply flex block; } .one { color: red; }')],{from:'entry.css'}],
  ['apply',[at('@import','"one.css"'),at('@tailwind','utilities')],[sheet('one.css','.one { @apply flex block; }')],{from:'entry.css'}],
  ['functions',[at('@import','"one.css"'),at('@tailwind','utilities')],[sheet('one.css','.one { color: --alpha(red / 50%); }')],{from:'entry.css'}],
  ['parser',[at('@import','"one.css"'),at('@tailwind','utilities')],[sheet('one.css','\ufeff/*! license */ .\ud800 { --x: { a: \";\" }; color: \"\udc00\"; }')],{from:'entry.css'}],
  ['undefined-before',[{kind:'declaration',property:'color',value:undefined,important:false},at('@import','"one.css"'),at('@tailwind','utilities')],[sheet('one.css','.one { color: red; }')],{from:'entry.css'}],
]
fixtures.push(
  ['important',[at('@import','"one.css" important'),at('@tailwind','utilities')],[sheet('one.css','.one { color: red; }')],{from:'entry.css'}],
  ['theme-options',[at('@import','"one.css" theme(inline)'),at('@tailwind','utilities')],[sheet('one.css','@theme { --color-card: red; }')],{from:'entry.css'}],
  ['prefix-options',[at('@import','"one.css" prefix(tw)'),at('@tailwind','utilities')],[sheet('one.css','@theme { --color-card: red; }')],{from:'entry.css'}],
  ['theme-units',[at('@import','"one.css" theme(inline --\ud800)'),at('@tailwind','utilities')],[sheet('one.css','@theme { --color-card: red; }')],{from:'entry.css'}],
  ['source-options',[at('@import','"one.css" source(none)')],[sheet('one.css','@tailwind utilities;')],{from:'entry.css'}],
)
let phases=0,objects=0
for (const [name,input,assets,extra] of fixtures) {
  const actual=structuredClone(input),expected=structuredClone(input)
  const actualRoots=[actual,...actual],expectedRoots=[expected,...expected]
  const options={base:'/discover',...extra}
  const reference=await tailwind.compileAst(expected,{...options,loadStylesheet:async(id,base) => {
    const asset=assets.find(asset => asset.id===id || asset.id===`${base}/${id}`)
    assert(asset,`${name}: ${id}`)
    return {path:asset.path,base:asset.base,content:asset.content}
  }})
  const compiler=new facade.AstCompiler(actual,JSON.stringify({limits,assets,options}))
  try {
    assert.deepEqual(graph(actualRoots),graph(expectedRoots),`${name}: constructor`);phases++
    for (const batch of [[],['card','bg-card','block','tw:bg-card','tw:block'],['flex'],[]]) {
      actualRoots.push(compiler.build(batch));expectedRoots.push(reference.build(batch))
      const snapshot=graph(actualRoots)
      assert.deepEqual(snapshot,graph(expectedRoots),`${name}: build ${JSON.stringify(batch)}`)
      objects+=snapshot.records.length;phases++
    }
  } finally {compiler.free()}
}
function observedInput() {
  const observations=[]
  const input=new Proxy([at('@import','"one.css"'),at('@import','"one.css"'),at('@tailwind','utilities')],{
    set(target,key,value) {
      if (/^\d+$/.test(key)) observations.push({index:Number(key),lengths:target.map(node => node.nodes.length)})
      return Reflect.set(target,key,value)
    },
  })
  return {input,observations}
}
const actual=observedInput(),expected=observedInput()
const assets=[sheet('one.css','.one { color: red; }')]
const reference=await tailwind.compileAst(expected.input,{from:'entry.css',loadStylesheet:async() => ({path:assets[0].path,base:assets[0].base,content:assets[0].content})})
const compiler=new facade.AstCompiler(actual.input,JSON.stringify({limits,options:{from:'entry.css'},assets}))
try {
  assert.deepEqual(actual.observations,expected.observations,'original array replacement order');phases++
  assert.deepEqual(graph([actual.input,compiler.build(['block'])]),graph([expected.input,reference.build(['block'])]),'observed import graph');phases++
} finally {compiler.free()}
console.log(JSON.stringify({importGraphCases:fixtures.length,originalArrayOrderCases:1,mediaOptionsCases:5,phases,objects,actualWasm:true,wasmBytes:bytes.length,fullImportConformance:'pending',failedSetupPrefix:'pending'}))
