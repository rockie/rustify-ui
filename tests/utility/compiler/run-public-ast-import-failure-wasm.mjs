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
for (const name of ['index.ts','ast.ts','apply.ts','utilities.ts','at-import.ts']) {
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
// Finite virtual-bundle constructor boundary and detached task graphs.
// Actual async API/loader interleaving and unhandled rejection remain separate.
// Compare both rejection and quiescent snapshots, including captured contexts.
const assets=[
  {id:'good.css',path:'/mapped/good.css',base:'',content:'.good { color: red; }'},
  {id:'outer.css',path:'/mapped/outer.css',base:'',content:'@import "good.css";'},
  {id:'parser.css',path:'/mapped/parser.css',base:'',content:'.broken { color: red;'},
  {id:'nested-parser.css',path:'/mapped/nested-parser.css',base:'',content:'@import "parser.css";'},
  {id:'nested-missing.css',path:'/mapped/nested-missing.css',base:'',content:'@import "missing.css";'},
]
for (let depth=2;depth<=8;depth++) assets.push({id:`chain${depth}.css`,path:`/mapped/chain${depth}.css`,base:'',
  content:`@import "${depth===2?'outer.css':`chain${depth-1}.css`}";`})
const cases=[
  ['single-missing',['missing.css']],
  ['first-missing',['missing.css','good.css']],
  ['last-missing',['good.css','missing.css']],
  ['nested-first-missing',['outer.css','missing.css']],
  ...Array.from({length:7},(_,i)=>[`chain${i+2}-then-missing`,[`chain${i+2}.css`,'missing.css']]),
  ['leaf-parser-error',['good.css','parser.css']],
  ['leaf-nested-parser-error',['good.css','nested-parser.css']],
  ['nested-missing-leaf',['nested-missing.css','good.css']],
  ['leaf-parameter-error',['good.css','parameter-error']],
]
function fixture(ids,{throwIndex,throwNodes}={}) {
  const trace=[],captured=[]
  const sentinel=new Error("original host exception")
  const input=new Proxy(ids.map(id=>at('@import',id==='parameter-error'?'"good.css" supports(display: grid) layer(a)':JSON.stringify(id))),{
    set(target,key,value) {
      if (/^\d+$/.test(key)) {
        trace.push(['index',key])
        captured.push(value)
        let nodes=value.nodes
        Object.defineProperty(value,'nodes',{enumerable:true,configurable:true,get:()=>nodes,set:next=>{trace.push(['nodes',key,next.length]);if(key===throwNodes)throw sentinel;nodes=next}})
      }
      if(key===throwIndex)throw sentinel
      return Reflect.set(target,key,value)
    },
  })
  return {input,trace,sentinel,roots:[input,...input,trace,captured]}
}
const failures=[]
const hostCases=[
  ['index-first',['good.css','chain3.css'],{throwIndex:'0'}],
  ['index-second',['good.css','chain3.css'],{throwIndex:'1'}],
  ['nodes-first',['good.css','chain3.css'],{throwNodes:'0'}],
  ['nodes-deep',['chain3.css','good.css'],{throwNodes:'0'}],
  ['nodes-second',['good.css','chain3.css'],{throwNodes:'1'}],
  ['earlier-missing-wins',['good.css','missing.css','chain3.css'],{throwNodes:'0'},false],
]
for (const [name,ids,host={},hostWins=true] of [...cases,...hostCases]) {
  const expected=fixture(ids,host),actual=fixture(ids,host)
  let wanted,received
  try {await tailwind.compileAst(expected.input,{from:'entry.css',loadStylesheet:async(id)=>{
    const asset=assets.find(asset=>asset.id===id)
    if (!asset) throw new Error('missing')
    return asset
  }})} catch(error) {wanted=error}
  const wantedPrefix=graph(expected.roots)
  const wantedPrefixTrace=structuredClone(expected.trace)
  for (let i=0;i<80;i++) await Promise.resolve()
  const wantedSettled=graph(expected.roots)
  try {new facade.AstCompiler(actual.input,JSON.stringify({limits,options:{from:'entry.css'},assets}))} catch(error) {received=error}
  assert(wanted && received,`${name}: constructors reject`)
  if (host.throwIndex!==undefined || host.throwNodes!==undefined) {
    if (hostWins) {assert.equal(wanted,expected.sentinel,name);assert.equal(received,actual.sentinel,name)}
    else {assert.notEqual(wanted,expected.sentinel,name);assert.notEqual(received,actual.sentinel,name);assert.equal(received.cause.error.kind,'missing_asset')}
  }
  // Custom async loader Error messages are not part of this graph-only probe.
  try {assert.deepEqual(graph(actual.roots),wantedPrefix,`${name}: import failure prefix`)} catch(error) {
    failures.push(name)
    console.log(JSON.stringify({name,expectedTrace:wantedPrefixTrace,actualTrace:actual.trace,comparison:error.message.split('\n')[0]}))
  }
  for (let i=0;i<80;i++) await Promise.resolve()
  try {assert.deepEqual(graph(actual.roots),wantedSettled,`${name}: retained tasks after rejection`)} catch(error) {
    failures.push(`${name}:after-rejection`)
    console.log(JSON.stringify({name,phase:'after rejection',comparison:error.message.split('\n')[0]}))
  }
}
const largeParser={id:'large-parser.css',path:'/mapped/large-parser.css',base:'',content:'.broken {'+' '.repeat(65536)}
const largeLeaf={id:'large-leaf.css',path:'/mapped/large-leaf.css',base:'',content:'.leaf { --payload: '+'x'.repeat(65536)+'; }'}
const tailAssets=[largeLeaf,...Array.from({length:8},(_,i)=>({id:`large-chain${i}.css`,base:'',content:`@import "${i===0?'large-leaf.css':`large-chain${i-1}.css`}";`}))]
for (const [name,ids,bundle,cache,extra={}] of [
  ['input-no-writes',['missing.css'],assets,64],
  ['failed-parser-source-owner',['large-parser.css'],[largeParser],32768],
  ['deferred-large-fragment',['missing.css','large-chain7.css'],tailAssets,32768],
  ['deferred-depth',['missing.css','chain3.css'],assets,16000000,{ast_depth:4}],
]) {
  const actual=fixture(ids),before=graph(actual.roots)
  assert.throws(()=>new facade.AstCompiler(actual.input,JSON.stringify({limits:{...limits,...extra,cache_bytes:cache},options:{from:'entry.css'},assets:bundle})),
    error=>error.cause?.error?.kind==='limit',name)
  assert.deepEqual(graph(actual.roots),before,`${name}: admission writes nothing`)
  for (let i=0;i<80;i++) await Promise.resolve()
  assert.deepEqual(graph(actual.roots),before,`${name}: no detached jobs admitted`)
}
// Same parser failure without a tracked Source fits the small cache, demonstrating
// that the failed-parser reserve is ownership-sensitive rather than asset length.
const untracked=fixture(['large-parser.css'])
assert.throws(()=>new facade.AstCompiler(untracked.input,JSON.stringify({limits:{...limits,cache_bytes:32768},assets:[largeParser]})),
  error=>error.cause?.error?.kind==='syntax')
assert.deepEqual(untracked.trace,[['index','0']])
console.log(JSON.stringify({importFailureCases:cases.length,originalHostExceptionCases:hostCases.length-1,settlementPriorityCases:1,
  noWriteQuotaCases:4,sourceOwnerQuotaControls:1,failedCases:failures,phase:'compileAst rejection and retained task quiescence',
  deeperAsyncHorizon:'prefix and retained tasks compared through eight nested imports',actualWasm:true,wasmBytes:bytes.length,fullImportFailureConformance:'pending'}))
assert.deepEqual(failures,[],'import loader failure prefixes')
