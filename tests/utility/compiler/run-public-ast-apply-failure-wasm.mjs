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
for(const name of ['index.ts','ast.ts','apply.ts','css-functions.ts']) {
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
const cases=['unknown','second-apply','second-parent','nested','multi-replacement','content-replacement',
  'theme','custom','dependency','cycle','keyframes','theme-keyframes','source','media','import','non-scalar']
function fixture(kind,tick=()=>0,host={}) {
  const trace=[],captured=[]
  const first=decl('color','--alpha(red / 50%)')
  let value=first.value
  Object.defineProperty(first,'value',{enumerable:true,get:()=>value,set:next=>{
    trace.push(['value',tick()]);if(host.key==='value')throw host.error;value=next
  }})
  const good=at('@apply',kind==='multi-replacement'?'block underline':kind==='content-replacement'?'content-none':'block')
  const bad=at('@apply',kind==='non-scalar'?'missing-\ud800':'missing-apply-class')
  const body=new Proxy([first,...(kind==='unknown'?[]:[good]),bad],{set(target,key,next){
    trace.push(['body',key,tick()]);if(/^\d+$/.test(key))captured.push(next)
    if(host.key===key)throw host.error
    return Reflect.set(target,key,next)
  }})
  let input=[rule('.one',body)],assets=[],options={}
  switch(kind) {
    case 'second-parent':input=[rule('.one',[first,good]),rule('.two',[bad])];break
    case 'nested':input=[rule('.one',[first,rule('&:hover',body)])];break
    case 'theme':input.unshift(at('@theme','',[decl('--color-test','red')]));break
    case 'custom':input=[at('@utility','card',[first,good]),...input];break
    case 'dependency':input=[at('@utility','card',[first,good]),rule('.one',[at('@apply','card'),bad])];break
    case 'cycle':input=[at('@utility','one',[first,at('@apply','two')]),at('@utility','two',[at('@apply','one')])];break
    case 'keyframes':input=[first,at('@keyframes','spin',[rule('to',[bad])])];break
    case 'theme-keyframes':input=[at('@theme','',[decl('--color-test','red'),at('@keyframes','spin',[rule('to',[bad])])]),first];break
    case 'source':{
      const source={file:'entry\ud800.css',code:'apply\udc00',extra:'retained'}
      good.src=[source,10,30];good.dst=good.src;bad.src=[source,31,55]
      input.unshift(at('@theme','',[{...decl('--color-test','red'),src:[source,1,9]}]));break
    }
    case 'media':input=[at('@media','important',input)];break
    case 'import':assets=[{id:'leaf.css',path:'/mapped/leaf.css',base:'',content:'.leaf { @apply block; @apply missing-apply-class; }'}];input=[at('@import','"leaf.css"'),first];options={from:'entry.css'};break
  }
  input.push(at('@tailwind','utilities'))
  const roots=[input,body,first,good,bad,trace,captured,...input]
  for(const node of input)if(node.nodes)roots.push(node.nodes)
  return {input,roots,assets,options}
}
function upstream(value) {
  return tailwind.compileAst(value.input,{...value.options,...(value.assets.length?{loadStylesheet:async()=>value.assets[0]}:{})})
}
async function observe(kind,construct,host={}) {
  let tick=0,active=true
  function marker(){if(active)queueMicrotask(()=>{tick++;marker()})}marker()
  const value=fixture(kind,()=>tick,host)
  let error
  try {await construct(value)}catch(failure){error=failure}
  const epoch=tick,prefix=graph(value.roots)
  active=false
  for(let index=0;index<80;index++)await Promise.resolve()
  assert(error,`${kind}: expected semantic failure`)
  if(host.error)assert.equal(error,host.error,'original host exception identity')
  return {name:error.name,message:error.message,epoch,prefix,settled:graph(value.roots)}
}
const failures=[]
for(const kind of cases) {
  for(const mode of ['sync','async']) {
    try {
      if(mode==='async') {
        const expected=await observe(kind,upstream)
        const actual=await observe(kind,value=>facade.compileAst(value.input,JSON.stringify({limits,assets:value.assets,options:value.options})))
        assert.deepEqual(actual,expected,`${kind}: Promise rejection graph and epoch`)
      } else {
        const expected=fixture(kind),actual=fixture(kind)
        let wanted,received
        try{await upstream(expected)}catch(error){wanted=error}
        try{new facade.AstCompiler(actual.input,JSON.stringify({limits,assets:actual.assets,options:actual.options}))}catch(error){received=error}
        assert(wanted && received,`${kind}: semantic failure`)
        assert.equal(received.name,wanted.name);assert.equal(received.message,wanted.message)
        assert.deepEqual(graph(actual.roots),graph(expected.roots),`${kind}: completed setup and apply prefix`)
      }
    }catch(error){failures.push({kind,mode,message:error.message})}
  }
}
let hostExceptionChecks=0
for(const [kind,key] of [['second-apply','value'],['second-apply','1'],['multi-replacement','2'],['multi-replacement','length']]) {
  const host={key,error:new Error(`original apply ${key}`)}
  for(const mode of ['sync','async']) {
    if(mode==='async') {
      const expected=await observe(kind,upstream,host)
      const actual=await observe(kind,value=>facade.compileAst(value.input,JSON.stringify({limits,assets:value.assets,options:value.options})),host)
      assert.deepEqual(actual,expected,`${kind}/${key}: host rejection prefix and epoch`)
    } else {
      const expected=fixture(kind,()=>0,host),actual=fixture(kind,()=>0,host)
      await assert.rejects(upstream(expected),error=>error===host.error)
      assert.throws(()=>new facade.AstCompiler(actual.input,JSON.stringify({limits})),error=>error===host.error)
      assert.deepEqual(graph(actual.roots),graph(expected.roots),`${kind}/${key}: host rejection prefix`)
    }
    hostExceptionChecks++
  }
}
function admission(cacheBytes) {
  const value=fixture('source'),before=graph(value.roots)
  let error
  try{new facade.AstCompiler(value.input,JSON.stringify({limits:{...limits,cache_bytes:cacheBytes}}))}catch(failure){error=failure}
  assert(error,'expected rejected setup')
  if(error.cause?.error?.kind==='limit') {
    assert.equal(error.cause.error.resource,'cache_bytes')
    assert.deepEqual(graph(value.roots),before,'cache refusal leaves the complete graph unchanged')
    return false
  }
  assert.match(error.message,/missing-apply-class/)
  assert.equal(value.input[1].nodes[1].property,'display','admitted failure retains the earlier apply')
  return true
}
let low=1,high=1000000
assert.equal(admission(high),true)
while(low<high){const middle=Math.floor((low+high)/2);if(admission(middle))high=middle;else low=middle+1}
assert.equal(admission(low-1),false);assert.equal(admission(low),true);assert.equal(admission(low+1),true)
for(const cacheBytes of [64,low-1]) {
  const value=fixture('source'),before=graph(value.roots)
  const pending=facade.compileAst(value.input,JSON.stringify({limits:{...limits,cache_bytes:cacheBytes}}))
  assert(pending instanceof Promise)
  assert.deepEqual(graph(value.roots),before,'async refusal has no synchronous writes')
  await assert.rejects(pending,error=>error.cause?.error?.kind==='limit')
  for(let index=0;index<80;index++)await Promise.resolve()
  assert.deepEqual(graph(value.roots),before,'async refusal has no detached writes')
}
console.log(JSON.stringify({applyFailureCases:cases.length,entries:2,checks:cases.length*2,failures,
  hostExceptionChecks,cacheBoundaries:3,semanticFailureCacheThreshold:low,asyncQuotaNoWriteCases:2,
  actualWasm:true,wasmBytes:bytes.length,fullFailedSetupConformance:'pending'}))
assert.deepEqual(failures,[],'all apply failure prefixes')
