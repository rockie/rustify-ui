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
const cases=[
  {name:'plain-first',order:['plain','custom']},
  {name:'custom-first',order:['custom','plain']},
  {name:'alternating',order:['plain','custom','other','second-custom']},
  {name:'plain-before-custom-error',order:['plain','custom-error']},
  {name:'plain-error-before-custom',order:['plain-error','custom']},
  {name:'custom-before-plain-error',order:['custom','plain-error']},
  {name:'two-errors-plain-first',order:['alpha-error','custom-error']},
  {name:'shared-plain-first',order:['plain','custom','other'],shared:true},
  {name:'shared-custom-first',order:['custom','plain','other'],shared:true},
  {name:'context-first',order:['plain','custom','other'],context:true},
  {name:'context-error',order:['plain','custom-error'],context:true},
  {name:'theme-error',order:['plain','custom-error'],theme:true},
  {name:'import-first',order:['import','custom','plain-error']},
  {name:'source-first',order:['plain','custom','other'],source:true},
  {name:'author-at-root',order:['fake-marker','custom','plain']},
  {name:'host-plain-first',order:['plain','custom'],throwLabel:'plain'},
  {name:'host-custom-second',order:['plain','custom','other'],throwLabel:'custom'},
  {name:'host-plain-second',order:['custom','plain'],throwLabel:'plain'},
  {name:'host-before-semantic',order:['plain','custom-error'],throwLabel:'plain'},
]
function fixture(spec,tick) {
  const marker=new Error('original function setter'),trace=[],source={file:'entry\ud800.css',code:'functions\udc00',extra:'retained'}
  function declaration(label,value) {
    const node=decl('color',value)
    let current=value
    Object.defineProperty(node,'value',{enumerable:true,get:()=>current,set:next=>{trace.push([label,next,tick()]);if(label===spec.throwLabel)throw marker;current=next}})
    if(spec.source)node.src=[source,1,22]
    return node
  }
  const first=declaration('plain','--alpha(red / 50%)')
  const custom=spec.shared?first:declaration('custom','--alpha(blue / 25%)')
  const other=declaration('other','--alpha(green / 75%)')
  const last=declaration('second-custom','--alpha(black / 25%)')
  const nodes={plain:first,custom:at('@utility','card',[custom]),other,
    'second-custom':at('@utility','card-two',[last]),
    'plain-error':decl('padding','--spacing(2)'),
    'alpha-error':decl('color','--alpha(red)'),
    'custom-error':at('@utility','card',[decl('padding','--spacing(2)')]),
    'fake-marker':{kind:'at-root',nodes:[{kind:'comment',value:'utility definition 0'}]},
    import:at('@import','"leaf.css"')}
  let input=spec.order.map(key=>nodes[key])
  if(spec.context)input=[{kind:'context',context:{base:'/nested'},nodes:input}]
  if(spec.theme)input.unshift(at('@theme','',[decl('--color-test','red')]))
  input.push(at('@tailwind','utilities'))
  const assets=[{id:'leaf.css',path:'/mapped/leaf.css',base:'',content:'.leaf { color: --alpha(black / 50%); }'}]
  return {input,assets,marker,roots:[input,first,custom,other,last,trace,...Object.values(nodes),...Object.values(nodes).filter(node=>node.nodes).map(node=>node.nodes)]}
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
  if(spec.throwLabel)assert.equal(error,value.marker,'original function setter identity')
  const epoch=tick,prefix=graph(value.roots),builds=[],identity=[]
  let previous
  if(compiler)for(const batch of [[],['card','card-two'],[],['block'],[]]) {
    const output=compiler.build(batch)
    identity.push(output===previous);previous=output
    builds.push(graph([...value.roots,output]))
  }
  compiler?.free?.()
  active=false
  for(let index=0;index<80;index++)await Promise.resolve()
  return {error:error?[error.name,error.message]:null,...(mode==='async'?{epoch}:{}),prefix,builds,identity,settled:graph(value.roots)}
}
function trace(prefix) {
  function decode(value) {
    return value && typeof value==='object' && value.reference!==undefined
      ?prefix.records[value.reference].fields.map(([,child])=>decode(child)):value
  }
  return decode(prefix.roots[5])
}
const failures=[]
for(const spec of cases)for(const mode of ['sync','async']) {
  const expected=await observe(spec,false,mode),actual=await observe(spec,true,mode)
  try{assert.deepEqual(actual,expected,spec.name)}catch(error){failures.push({name:spec.name,mode,message:error.message.split('\n')[0],expectedError:expected.error,actualError:actual.error,expectedTrace:trace(expected.prefix),actualTrace:trace(actual.prefix)})}
}
console.log(JSON.stringify({functionOrderCases:cases.length,entryComparisons:cases.length*2,
  completePrefixBuildAndSettledGraphs:true,originalHostExceptionChecks:cases.filter(spec=>spec.throwLabel).length*2,failures,actualWasm:true,wasmBytes:bytes.length,fullFailedSetupConformance:'pending'}))
assert.deepEqual(failures,[],'function traversal order and shared views')
