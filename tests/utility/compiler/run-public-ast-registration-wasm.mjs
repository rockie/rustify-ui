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
for(const name of ['index.ts','ast.ts','theme.ts','utilities.ts','at-import.ts']) {
  const path=`packages/tailwindcss/src/${name}`
  assert.equal(createHash('sha256').update(await readFile(resolve('ref/tailwindcss',path))).digest('hex'),
    inventory.sources.find(source=>source.path===path)?.sha256,path)
}
const directory=resolve(process.argv[2] ?? 'target/utility-conformance/compiler-api')
const facade=await import(pathToFileURL(resolve(directory,'style.js')))
const bytes=await readFile(resolve(directory,'style_bg.wasm'))
await facade.default({module_or_path:bytes})
const limits={input_bytes:262144,asset_bytes:262144,candidate_count:1024,ast_depth:32,css_bytes:262144,queued_bytes:262144,cache_bytes:16000000}
const decl=(property,value)=>({kind:'declaration',property,value,important:false})
const at=(name,params='',nodes=[])=>({kind:'at-rule',name,params,nodes})
const rule=(selector,nodes)=>({kind:'rule',selector,nodes})
function graph(roots) {
  const seen=new Map(),records=[]
  function visit(value) {
    if(value===undefined)return {undefined:true}
    if(value===null || typeof value!=='object')return value
    if(seen.has(value))return {reference:seen.get(value)}
    const id=records.length;seen.set(value,id);records.push(null)
    records[id]={array:Array.isArray(value),fields:Object.keys(value).map(key=>[key,visit(value[key])])}
    return {reference:id}
  }
  return {roots:roots.map(visit),records}
}
const cases=[
  ...['source-bad','source-body','source-nested','inline-zero','utility-empty','utility-name',
    'utility-nested','variant-name','variant-empty','variant-both','variant-nested',
    'theme-prefix','theme-child','theme-child-undefined','theme-namespace','media-reference'].map(bad=>({name:bad,order:['theme',bad,'later-media']})),
  ...['source','inline','variant','legacy','legacy-slot','themes','media'].map(prefix=>({name:`${prefix}-prefix`,order:[prefix,'utility-empty','later-media']})),
  ...[['utility-empty','source-bad'],['theme-prefix','variant-name'],['variant-empty','inline-zero'],
    ['source-bad','media-reference']].flatMap(pair=>[pair,[...pair].reverse()].map(order=>({name:order.join('-before-'),order}))),
  {name:'context-prefix',order:['theme','source','variant','utility-name'],context:true},
  {name:'source-metadata',order:['themes','variant','source','theme-child'],source:true},
  {name:'import-prefix',order:['import','source','utility-name']},
  {name:'functional-before-initial-error',order:['functional','theme','utility-name']},
  {name:'theme-before-late-media-error',order:['theme','source','media-reference']},
  {name:'variant-invalid-is-removed',order:['source','variant-name','theme']},
  {name:'authored-unknown-after-theme',order:['theme','authored-unknown']},
  {name:'utility-authored-unknown-after-theme',order:['theme','utility-authored-unknown']},
  ...['utility-body-theme','utility-body-source','utility-body-variant','theme-context-body',
    'context-overrides','shared-source-body','shared-invalid-variant-body',
    'shared-utility-theme-body','shared-theme-body'].map(kind=>({name:kind,order:['theme',kind,'utility-empty']})),
]
function fixture(spec,tick) {
  const trace=[],all=[],arrays=[],source={file:'registration\ud800.css',code:'setup\udc00',extra:'retained'}
  function track(node,label) {
    all.push(node)
    if(spec.source)node.src=[source,1.5,27.5]
    for(const field of ['kind','name','params','nodes'])if(Object.hasOwn(node,field)) {
      let current=node[field]
      if(Array.isArray(current))arrays.push(current)
      Object.defineProperty(node,field,{enumerable:true,get:()=>current,
        set:next=>{trace.push([label,field,Array.isArray(next)?{array:true}:next,tick()]);current=next}})
    }
    return node
  }
  const theme=()=>at('@theme','',[decl('--color-test','red')])
  function make(kind) {
    let nodes
    switch(kind) {
      case 'theme':nodes=[theme()];break
      case 'themes':nodes=[theme(),at('@theme','default',[decl('--color-other','blue'),at('@keyframes','spin',[rule('to',[decl('opacity','0')])])])];break
      case 'source':nodes=[at('@source','"./*.html"')];break
      case 'inline':nodes=[at('@source','inline("block hidden")')];break
      case 'variant':nodes=[at('@custom-variant','tone (&:hover)')];break
      case 'legacy':nodes=[at('@variant','tone (&:hover)')];break
      case 'legacy-slot':nodes=[at('@variant','tone',[rule('&:hover',[at('@slot')])])];break
      case 'media':nodes=[at('@media','theme(inline)',[theme()])];break
      case 'later-media':nodes=[at('@media','prefix(tw) theme(static)',[theme()])];break
      case 'source-bad':nodes=[at('@source','*.html')];break
      case 'source-body':nodes=[at('@source','"./*.html"',[decl('color','red')])];break
      case 'source-nested':nodes=[rule('.nested',[at('@source','"./*.html"')])];break
      case 'inline-zero':nodes=[at('@source','inline("block{1..3..0}")')];break
      case 'utility-empty':nodes=[at('@utility','card')];break
      case 'utility-name':nodes=[at('@utility','Bad',[decl('color','red')])];break
      case 'utility-nested':nodes=[rule('.nested',[at('@utility','card',[decl('color','red')])])];break
      case 'variant-name':nodes=[at('@custom-variant','Bad (&:hover)')];break
      case 'variant-empty':nodes=[at('@custom-variant','tone')];break
      case 'variant-both':nodes=[at('@custom-variant','tone (&:hover)',[at('@slot')])];break
      case 'variant-nested':nodes=[rule('.nested',[at('@custom-variant','tone (&:hover)')])];break
      case 'authored-unknown':nodes=[at('@variant','missing',[decl('color','red')])];break
      case 'utility-authored-unknown':nodes=[at('@utility','card',[at('@variant','missing',[decl('color','red')])])];break
      case 'theme-prefix':nodes=[at('@theme','prefix(Bad)',[decl('--color-test','red')])];break
      case 'theme-child':nodes=[at('@theme','',[decl('color','red')])];break
      case 'theme-child-undefined':nodes=[at('@theme','',[decl('color',undefined)])];break
      case 'theme-namespace':nodes=[at('@theme','',[decl('--color-*','red')])];break
      case 'media-reference':nodes=[at('@media','theme(reference)',[decl('color','red')])];break
      case 'functional':nodes=[at('@utility','tone-*',[decl('color','--value([ *])')])];break
      case 'utility-body-theme':nodes=[at('@utility','card',[theme()])];break
      case 'utility-body-source':nodes=[at('@utility','card',[at('@source','"./*.html"')])];break
      case 'utility-body-variant':nodes=[at('@utility','card',[at('@custom-variant','tone (&:hover)')])];break
      case 'theme-context-body':nodes=[at('@theme','',[{kind:'context',context:{reference:false},nodes:[decl('--color-context','blue')]}])];break
      case 'context-overrides':nodes=[{kind:'context',context:{reference:true,base:'/outer'},nodes:[
        {kind:'context',context:{reference:false,base:''},nodes:[at('@tailwind','utilities source("./inner")'),theme()]},
      ]}];break
      case 'shared-source-body':
      case 'shared-invalid-variant-body':
      case 'shared-theme-body': {
        const shared=kind==='shared-source-body'?[at('@source','"./*.html"')]
          :kind==='shared-invalid-variant-body'?[at('@custom-variant','Bad (&:hover)')]:[theme()]
        nodes=[{kind:'context',context:{base:'/one'},nodes:shared},{kind:'context',context:{base:'/two'},nodes:shared}]
        break
      }
      case 'shared-utility-theme-body': {
        const shared=[theme(),decl('color','--value([ *])')]
        nodes=[at('@utility','tone-*',shared),{kind:'context',context:{base:'/later'},nodes:shared}]
        break
      }
      case 'import':nodes=[at('@import','"leaf.css"')];break
      default:throw new Error(kind)
    }
    function visit(node,label) {
      if(node.nodes)node.nodes.forEach((child,index)=>visit(child,`${label}.${index}`))
      track(node,label)
    }
    nodes.forEach((node,index)=>visit(node,`${kind}.${index}`))
    return nodes
  }
  let input=spec.order.flatMap(make)
  if(spec.context)input=[track({kind:'context',context:{base:'/nested'},nodes:input},'context')]
  input.push(track(at('@tailwind','utilities'),'utilities'))
  const assets=[{id:'leaf.css',path:'/mapped/leaf.css',base:'',content:'@theme { --color-leaf:red; } .leaf { color:red; }'}]
  return {input,assets,roots:[input,trace,...all,...arrays,source]}
}
async function observe(spec,wasm,mode) {
  let tick=0,active=true
  function marker(){if(active)queueMicrotask(()=>{tick++;marker()})}if(mode==='async')marker()
  const value=fixture(spec,()=>tick)
  let compiler,error
  try {
    const config=JSON.stringify({limits,assets:value.assets,options:{from:'entry.css'}})
    compiler=wasm?(mode==='async'?await facade.compileAst(value.input,config):new facade.AstCompiler(value.input,config))
      :await tailwind.compileAst(value.input,{from:'entry.css',loadStylesheet:async()=>value.assets[0]})
  }catch(failure){error=failure}
  assert(error,`${spec.name}: registration must fail`)
  const epoch=tick,prefix=graph(value.roots)
  compiler?.free?.();active=false
  for(let index=0;index<80;index++)await Promise.resolve()
  return {error:[error.name,error.message],...(mode==='async'?{epoch}:{}),prefix,settled:graph(value.roots)}
}
const failures=[]
function trace(prefix) {
  function decode(value) {
    if(!value || typeof value!=='object' || value.reference===undefined)return value
    const record=prefix.records[value.reference]
    return record.array?record.fields.map(([,child])=>decode(child))
      :Object.fromEntries(record.fields.map(([key,child])=>[key,decode(child)]))
  }
  return decode(prefix.roots[1])
}
for(const spec of cases)for(const mode of ['sync','async']) {
  const expected=await observe(spec,false,mode),actual=await observe(spec,true,mode)
  try{assert.deepEqual(actual,expected,spec.name)}catch(error){failures.push({name:spec.name,mode,
    expectedError:expected.error,actualError:actual.error,errorMatches:JSON.stringify(actual.error)===JSON.stringify(expected.error),
    epochMatches:actual.epoch===expected.epoch,prefixMatches:JSON.stringify(actual.prefix)===JSON.stringify(expected.prefix),
    settledMatches:JSON.stringify(actual.settled)===JSON.stringify(expected.settled),
    expectedTrace:trace(expected.prefix),actualTrace:trace(actual.prefix)})}
}
console.log(JSON.stringify({registrationCases:cases.length,entryComparisons:cases.length*2,
  completePrefixAndSettledGraphs:true,failures,actualWasm:true,wasmBytes:bytes.length,fullRegistrationConformance:'pending'}))
assert.deepEqual(failures,[],'initial registration order and completed mutation prefix')
