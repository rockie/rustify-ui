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
const directory=resolve(process.argv[2] && process.argv[2]!=='--removal-only'
  ? process.argv[2] : 'target/utility-conformance/compiler-api')
const removalOnly=process.argv.includes('--removal-only')
const facade=await import(pathToFileURL(resolve(directory,'style.js')))
const bytes=await readFile(resolve(directory,'style_bg.wasm'))
await facade.default({module_or_path:bytes})
const limits={input_bytes:262144,asset_bytes:262144,candidate_count:1024,ast_depth:32,
  css_bytes:262144,queued_bytes:262144,cache_bytes:16000000}
const decl=(property,value) => ({kind:'declaration',property,value,important:false})
const at=(name,params='',nodes=[]) => ({kind:'at-rule',name,params,nodes})
const rule=(selector,nodes) => ({kind:'rule',selector,nodes})
function snapshot(roots) {
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
function fixture(spec) {
  const source={file:'custom\ud800.css',code:'custom\udc00',extra:'kept'}
  const definitions=(spec.definitions ?? [['card',[['color','red']]]]).map(([name,entries],index) => ({
    ...at('@utility',name,entries.map(([property,value]) => ({
      ...(property==='@apply' ? at('@apply',value) : property==='&' ? rule('&',[decl('color',value)]) : decl(property,value)),
      extra:'old child',...(spec.source ? {src:[source,index+1.5,index+20]} : {}),
    }))),extra:'old definition',...(spec.source ? {src:[source,index+0.5,index+25]} : {}),
  }))
  const oldArrays=definitions.map(node => node.nodes),oldChildren=oldArrays.flat()
  const utilities=at('@tailwind','utilities')
  const theme=at('@theme','',[decl('--color-card','red'),decl('--spacing','0.25rem')])
  const secondTheme=at('@theme','default',[decl('--color-card','blue')])
  const applied=spec.apply ? rule('.applied',[at('@apply',spec.apply)]) : null
  const nodes=[...(spec.theme ? [theme] : []),...definitions,...(spec.theme ? [secondTheme] : []),...(applied ? [applied] : [])]
  const spliceCalls=[]
  if (spec.shared) Object.defineProperty(nodes,'splice',{value:function(...args) {
    spliceCalls.push(args)
    return Array.prototype.splice.apply(this,args)
  }})
  const input=spec.shared ? [{kind:'context',context:{probe:true},nodes},{kind:'context',context:{probe:false},nodes}]
    : spec.context ? [{kind:'context',context:{probe:true},nodes}] : nodes
  if (spec.utilities!==false) input.push(utilities)
  if (spec.frozen) { definitions.forEach(node => Object.freeze(node)); Object.freeze(source) }
  return {input,definitions,oldArrays,oldChildren,utilities,roots:[input,...definitions,...oldArrays,...oldChildren,source,utilities,theme,secondTheme,...(applied ? [applied] : []),spliceCalls]}
}
const cases=[
  {id:'static'},
  {id:'duplicate-definitions',definitions:[['card',[['color','red']]],['card',[['color','blue']]]]},
  {id:'apply-in-definition',source:true,definitions:[['card',[['@apply','flex block']]]],apply:'card underline'},
  {id:'dependency-order',source:true,definitions:[['card',[['@apply','base underline']]],['base',[['@apply','block']]]],apply:'card'},
  {id:'nested-apply',definitions:[['card',[['@apply','hover:block flex'],['color','red']]]],apply:'card'},
  {id:'raw-sort-and-duplicates',definitions:[['card',[['--tw-sort','display'],['display','block'],['display','block'],['&','red']]]],apply:'card'},
  {id:'context',context:true,definitions:[['card',[['@apply','block']]]],apply:'card'},
  {id:'theme-before-and-after',theme:true,definitions:[['card',[['@apply','bg-card m-2']]]],apply:'card'},
  {id:'frozen-old-definition',frozen:true,source:true,definitions:[['card',[['@apply','block']]]]},
  {id:'no-features',utilities:false},
  {id:'shared-array',shared:true,utilities:false},
  {id:'functional-normalization',theme:true,definitions:[['tone-*',[['color','--value(--color)']]]],batches:[[],['tone-card'],['block'],[],['flex']]},
  {id:'functional-arbitrary-normalization',definitions:[['tone-*',[['color','--value([ *])']]]],batches:[[],['tone-[red]'],['block'],[],['flex']]},
  {id:'function-in-definition',definitions:[['card',[['color','--alpha(red / 50%)'],['@apply','block']]]],apply:'card'},
]
let phases=0,noOps=0
const removalCases=new Set(['static','duplicate-definitions','apply-in-definition','context',
  'theme-before-and-after','frozen-old-definition','no-features','shared-array'])
const selected=removalOnly ? cases.filter(spec => removalCases.has(spec.id)) : cases
for (const spec of selected) {
  const actual=fixture(spec),expected=fixture(spec)
  const reference=await tailwind.compileAst(expected.input)
  const compiler=new facade.AstCompiler(actual.input,JSON.stringify({limits}))
  try {
    assert.deepEqual(snapshot(actual.roots),snapshot(expected.roots),`${spec.id}: constructor graph`);phases++
    let previous,wantedPrevious
    for (const batch of removalOnly ? [[],['block'],['flex'],[],['underline']] : spec.batches ?? [[],['card'],['block'],[],['flex']]) {
      const result=compiler.build(batch),wanted=reference.build(batch)
      actual.roots.push(result);expected.roots.push(wanted)
      assert.deepEqual(snapshot(actual.roots),snapshot(expected.roots),`${spec.id}: build ${JSON.stringify(batch)}`)
      if (wanted===wantedPrevious) { assert.equal(result,previous);noOps++ }
      previous=result;wantedPrevious=wanted;phases++
    }
  } finally { compiler.free() }
}
console.log(JSON.stringify({scope:removalOnly ? 'custom utility removal and old body apply' : 'full custom utility input graph',
  customInputCases:selected.length,constructorAndBuildPhases:phases,noOpIdentities:noOps,
  actualWasm:true,sharedMemory:false,wasmBytes:bytes.length,fullInputMutationConformance:'pending'}))
