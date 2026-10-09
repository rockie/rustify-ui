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
for (const name of ['index.ts','ast.ts','at-import.ts','compat/apply-compat-hooks.ts']) {
  const path=`packages/tailwindcss/src/${name}`
  assert.equal(createHash('sha256').update(await readFile(resolve('ref/tailwindcss',path))).digest('hex'),
    inventory.sources.find(source=>source.path===path)?.sha256,path)
}
const directory=resolve(process.argv[2] ?? 'target/utility-conformance/compiler-api')
const facade=await import(pathToFileURL(resolve(directory,'style.js')))
const bytes=await readFile(resolve(directory,'style_bg.wasm'))
await facade.default({module_or_path:bytes})
const limits={input_bytes:262144,asset_bytes:262144,candidate_count:1024,ast_depth:32,css_bytes:262144,queued_bytes:262144,cache_bytes:16000000}
const at=(name,params='',nodes=[])=>({kind:'at-rule',name,params,nodes})
const decl=(property,value)=>({kind:'declaration',property,value,important:false})
const assets=[{id:'leaf.css',path:'/mapped/leaf.css',base:'',content:'.leaf { color: red; }'},
  {id:'parser.css',path:'/mapped/parser.css',base:'',content:'.broken { color: red;'},
  {id:'nested-missing.css',path:'/mapped/nested-missing.css',base:'',content:'@import "missing.css";'}]
for(let depth=1;depth<=8;depth++)assets.push({id:`chain${depth}.css`,path:`/mapped/chain${depth}.css`,base:'',
  content:`@import "${depth===1?'leaf.css':`chain${depth-1}.css`}";`})
function graph(roots) {
  const seen=new Map(),records=[]
  function visit(value) {
    if(value===undefined)return{undefined:true}
    if(value===null || typeof value!=='object')return value
    if(seen.has(value))return{reference:seen.get(value)}
    const id=records.length;seen.set(value,id);records.push(null)
    records[id]={array:Array.isArray(value),fields:Object.keys(value).map(key=>[key,visit(value[key])])}
    return{reference:id}
  }
  return{roots:roots.map(visit),records}
}
const cases=[
  {name:'no-import-success',ids:[]},
  {name:'leaf-success',ids:['leaf.css']},
  {name:'siblings-success',ids:['leaf.css','chain3.css']},
  ...Array.from({length:8},(_,i)=>({name:`chain${i+1}-missing`,ids:[`chain${i+1}.css`,'missing.css']})),
  {name:'missing-first',ids:['missing.css','leaf.css']},
  {name:'parser-error',ids:['leaf.css','parser.css']},
  {name:'parameter-error',ids:['leaf.css','parameter-error']},
  {name:'host-index',ids:['leaf.css','chain3.css'],throwIndex:'0'},
  {name:'host-nodes',ids:['leaf.css','chain3.css'],throwNodes:'0'},
  {name:'theme-and-functions',ids:[],theme:true},
  {name:'theme-import-and-functions',ids:['leaf.css'],theme:true},
  {name:'theme-and-function-error',ids:[],theme:true,bad:true},
  {name:'custom-function-error',ids:[],custom:true,bad:true},
]
async function observe(spec,construct,wasm=false) {
  let tick=0,active=true
  function marker(){if(active)queueMicrotask(()=>{tick++;marker()})}marker()
  const trace=[],captured=[],sentinel=new Error('original async host exception')
  const first=decl('color','--alpha(red / 50%)')
  let value=first.value
  Object.defineProperty(first,'value',{enumerable:true,get:()=>value,set:next=>{trace.push(['value',tick]);queueMicrotask(()=>trace.push(['value-marker',tick]));value=next}})
  const nodes=spec.ids.map(id=>at('@import',id==='parameter-error'?'"leaf.css" supports(display: grid) layer(a)':JSON.stringify(id)))
  if(spec.theme)nodes.push(at('@theme','',[decl('--color-test','red')]))
  if(spec.custom)nodes.push(at('@utility','card',[first,decl('padding','--spacing(2)')]))
  else nodes.push(first,...(spec.bad?[decl('padding','--spacing(2)')]:[]))
  nodes.push(at('@tailwind','utilities'))
  const input=new Proxy(nodes,{set(target,key,next){
    if(/^\d+$/.test(key)) {
      trace.push(['index',key,tick]);captured.push(next)
      let children=next.nodes
      Object.defineProperty(next,'nodes',{enumerable:true,configurable:true,get:()=>children,set:body=>{
        trace.push(['nodes',key,tick]);queueMicrotask(()=>trace.push(['nodes-marker',key,tick]))
        if(key===spec.throwNodes)throw sentinel
        children=body
      }})
      queueMicrotask(()=>trace.push(['index-marker',key,tick]))
    }
    if(key===spec.throwIndex)throw sentinel
    return Reflect.set(target,key,next)
  }})
  const roots=[input,...input,first,trace,captured]
  let compiler,error
  try {
    const pending=construct(input)
    assert(pending instanceof Promise,'compileAst returns a native Promise')
    compiler=await pending
    if(wasm && compiler)assert(compiler instanceof facade.AstCompiler,'Promise resolves the exported compiler class')
  } catch(failure){error=failure}
  const atObservation=tick,prefix=graph(roots),prefixTrace=structuredClone(trace)
  if(spec.throwIndex!==undefined || spec.throwNodes!==undefined)assert.equal(error,sentinel,spec.name)
  let previous
  const identity=[]
  const builds=compiler?[[],['block'],[],['flex'],[]].map(batch=>{
    const output=compiler.build(batch)
    identity.push(output===previous);previous=output
    return graph([...roots,output])
  }):[]
  compiler?.free?.()
  for(let i=0;i<80;i++)await Promise.resolve()
  active=false
  return{atObservation,prefix,prefixTrace,settled:graph(roots),builds,identity,error:error?.name,semanticError:spec.bad?error?.message:undefined}
}
const failures=[]
if(typeof facade.compileAst!=='function')failures.push('missing-async-entry')
for(const spec of cases) {
  const expected=await observe(spec,input=>tailwind.compileAst(input,{from:'entry.css',loadStylesheet:async id=>{
    const asset=assets.find(asset=>asset.id===id)
    if(!asset)throw new Error('missing')
    return asset
  }}))
  const actual=await observe(spec,input=>typeof facade.compileAst==='function'
    ?facade.compileAst(input,JSON.stringify({limits,options:{from:'entry.css'},assets}))
    :(async()=>new facade.AstCompiler(input,JSON.stringify({limits,options:{from:'entry.css'},assets})))(),true)
  try{assert.deepEqual(actual,expected,spec.name)}catch(error){
    failures.push(spec.name)
    console.log(JSON.stringify({name:spec.name,expectedEpoch:expected.atObservation,actualEpoch:actual.atObservation,
      expectedTrace:expected.prefixTrace,actualTrace:actual.prefixTrace,
      comparison:error.message.split('\n')[0]}))
  }
}
const orphanFailures=[]
for(const spec of [
  {name:'orphan-missing-before-params',ids:['missing.css','parameter-error']},
  {name:'orphan-missing-before-host',ids:['missing.css'],throwIndex:'0'},
  {name:'orphan-parser-before-host',ids:['parser.css'],throwIndex:'0'},
  {name:'orphan-nested-missing-before-host',ids:['nested-missing.css'],throwIndex:'0'},
  {name:'joined-missing-control',ids:['missing.css','leaf.css']},
]) {
  async function capture(construct,wasm=false) {
    const events=[]
    const listener=(error,promise)=>{
      assert(promise instanceof Promise)
      assert(error instanceof Error)
      events.push(error.message==='missing' || error.cause?.error?.kind==='missing_asset' ? 'missing_asset' : [error.name,error.message])
    }
    process.on('unhandledRejection',listener)
    try {
      await observe(spec,construct,wasm)
      await new Promise(setImmediate)
      return events
    } finally {process.off('unhandledRejection',listener)}
  }
  const wanted=await capture(input=>tailwind.compileAst(input,{from:'entry.css',loadStylesheet:async id=>{
    const asset=assets.find(asset=>asset.id===id)
    if(!asset)throw new Error('missing')
    return asset
  }}))
  const actual=await capture(input=>facade.compileAst(input,JSON.stringify({limits,options:{from:'entry.css'},assets})),true)
  try{assert.deepEqual(actual,wanted,spec.name)}catch(error){
    orphanFailures.push(spec.name);console.log(JSON.stringify({name:spec.name,expectedUnhandled:wanted,actualUnhandled:actual}))
  }
}
// A second original host fault belongs to the orphan task, while the first
// fault remains the public compileAst rejection. Both error identities survive.
for(const wasm of [false,true]) {
  const first=new Error('original begin'),second=new Error('original completion'),events=[]
  const input=new Proxy([at('@import','"leaf.css"')],{set(target,key,value){
    if(key==='0') {
      Object.defineProperty(value,'nodes',{enumerable:true,configurable:true,get:()=>[],set:()=>{throw second}})
      throw first
    }
    return Reflect.set(target,key,value)
  }})
  const listener=(error,promise)=>{assert(promise instanceof Promise);events.push(error)}
  process.on('unhandledRejection',listener)
  try {
    const pending=wasm?facade.compileAst(input,JSON.stringify({limits,options:{from:'entry.css'},assets}))
      :tailwind.compileAst(input,{from:'entry.css',loadStylesheet:async()=>assets[0]})
    await assert.rejects(pending,error=>error===first)
    await new Promise(setImmediate)
    assert.equal(events.length,1)
    assert.equal(events[0],second,'orphan event keeps the original completion exception')
  } finally {process.off('unhandledRejection',listener)}
}
const largeParser={id:'large-parser.css',path:'/mapped/large-parser.css',content:'.broken {'+' '.repeat(65536)}
for(const [name,bundle,options,quota] of [
  ['input-cache',assets,{from:'entry.css'},{cache_bytes:64}],
  ['retained-parser-source',[largeParser],{from:'entry.css'},{cache_bytes:32768}],
  ['no-import-cache',[],{}, {cache_bytes:64}],
  ['queued-reactions',assets.slice(0,1),{from:'entry.css'},{queued_bytes:600}],
]) {
  const input=name==='no-import-cache'?[decl('color','--alpha(red / 50%)')]:Array.from({length:name==='queued-reactions'?16:1},()=>at('@import',JSON.stringify(name==='retained-parser-source'?'large-parser.css':'leaf.css')))
  const roots=[input,...input],before=graph(roots)
  const config=JSON.stringify({limits:{...limits,...quota},options,assets:bundle})
  if(name==='queued-reactions')assert(config.length<=quota.queued_bytes,'queue case is admitted at JSON ingress')
  const pending=facade.compileAst(input,config)
  assert(pending instanceof Promise,name)
  assert.deepEqual(graph(roots),before,`${name}: synchronous admission writes nothing`)
  await assert.rejects(pending,error=>error.cause?.error?.kind==='limit',name)
  for(let i=0;i<80;i++)await Promise.resolve()
  assert.deepEqual(graph(roots),before,`${name}: no detached writes`)
}
console.log(JSON.stringify({asyncCases:cases.length,externalMicrotaskMarkers:true,completePrefixAndSettledGraphs:true,
  failures,orphanCases:6,orphanFailures,originalOrphanHostExceptionCases:1,quotaNoWriteCases:4,actualWasm:true,wasmBytes:bytes.length,
  externalLoaderCallbacks:'pending',fullAsyncApiConformance:'pending'}))
assert.deepEqual(failures,[],'async compileAst publication and external microtask interleaving')
assert.deepEqual(orphanFailures,[],'orphan import Promise rejection events')
