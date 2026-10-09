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
for(const name of ['index.ts','ast.ts']) {
  const path=`packages/tailwindcss/src/${name}`
  assert.equal(createHash('sha256').update(await readFile(resolve('ref/tailwindcss',path))).digest('hex'),
    inventory.sources.find(source=>source.path===path)?.sha256,path)
}
const directory=resolve(process.argv[2] ?? 'target/utility-conformance/compiler-api')
const facade=await import(pathToFileURL(resolve(directory,'style.js')))
const bytes=await readFile(resolve(directory,'style_bg.wasm'))
await facade.default({module_or_path:bytes})
const limits={input_bytes:262144,asset_bytes:262144,candidate_count:1024,ast_depth:32,css_bytes:262144,queued_bytes:262144,cache_bytes:16000000}
function snapshot(roots) {
  const ids=new Map(),records=[]
  function visit(value) {
    if(value===undefined)return {undefined:true}
    if(value===null || typeof value!=='object')return value
    if(ids.has(value))return {reference:ids.get(value)}
    const id=records.length;ids.set(value,id);records.push(null)
    records[id]={array:Array.isArray(value),fields:Object.keys(value).map(key=>[key,visit(value[key])])}
    return {reference:id}
  }
  return {roots:roots.map(visit),records}
}
function fixture(condition) {
  const trace=[]
  function utilities(label) {
    const node={kind:'at-rule',name:'@tailwind',params:'utilities',nodes:[]}
    let params=node.params
    Object.defineProperty(node,'params',{enumerable:true,get:()=>params,set:value=>{
      trace.push([label,value]);params=value
    }})
    return node
  }
  const first=utilities('first'),second=utilities('second'),children=[first,second]
  const media={kind:'at-rule',name:'@media',params:condition,nodes:children}
  const invalid={kind:'at-rule',name:'@utility',params:'empty',nodes:[]},input=[media,invalid]
  return {input,trace,roots:[input,media,children,first,second,first.nodes,second.nodes,invalid,trace]}
}
async function observe(condition,wasm,mode) {
  const value=fixture(condition)
  let error
  try {
    const options={from:'entry.css',base:'/scope'},config=JSON.stringify({limits,options})
    if(wasm) {
      const compiler=mode==='sync'?new facade.AstCompiler(value.input,config):await facade.compileAst(value.input,config)
      compiler.free();assert.fail('expected semantic error')
    }else await tailwind.compileAst(value.input,options)
  }catch(failure){error=[failure.name,failure.message]}
  const prefix=snapshot(value.roots)
  for(let index=0;index<80;index++)await Promise.resolve()
  return {error,trace:value.trace,prefix,settled:snapshot(value.roots)}
}
const comparisons=[]
for(const condition of ['source("./src")','print source("./src")'])for(const mode of ['sync','async']) {
  const expected=await observe(condition,false,mode),actual=await observe(condition,true,mode)
  assert.equal(expected.trace.length,2,'replacement plus Stop continues to the second exact utilities sibling')
  assert.deepEqual(actual.error,expected.error,'semantic-error control')
  comparisons.push({condition,mode,matches:JSON.stringify(actual)===JSON.stringify(expected),expected,actual})
}
console.log(JSON.stringify({scope:'actual-WASM supplementary media source/Stop diagnostic',cases:2,entryComparisons:4,
  failures:comparisons.filter(value=>!value.matches).length,actualWasm:true,wasmBytes:bytes.length,comparisons,
  fullRegistrationConformance:'pending'}))
assert(comparisons.every(value=>value.matches),'media source replacement/Stop prefix')
