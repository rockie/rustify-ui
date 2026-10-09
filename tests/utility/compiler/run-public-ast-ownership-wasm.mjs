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
for(const name of ['index.ts','ast.ts','theme.ts','utilities.ts','variants.ts','compile.ts']) {
  const path=`packages/tailwindcss/src/${name}`
  assert.equal(createHash('sha256').update(await readFile(resolve('ref/tailwindcss',path))).digest('hex'),
    inventory.sources.find(source=>source.path===path)?.sha256,path)
}
const directory=resolve(process.argv[2] ?? 'target/utility-conformance/compiler-api')
const facade=await import(pathToFileURL(resolve(directory,'style.js')))
const bytes=await readFile(resolve(directory,'style_bg.wasm'))
await facade.default({module_or_path:bytes})
const limits={input_bytes:262144,asset_bytes:262144,candidate_count:1024,ast_depth:32,
  css_bytes:262144,queued_bytes:262144,cache_bytes:16000000}
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
function fixture(name) {
  const trace=[],retained=[]
  let input,candidates=[]
  if(name==='authored-success-before-failure') {
    const first=at('@variant','hover',[decl('color','red')])
    const second=at('@variant','missing',[decl('color','blue')])
    input=[rule('.one',[first]),rule('.two',[second])]
    retained.push(first,second,first.nodes,second.nodes)
  } else if(name==='authored-outer-first-error' || name==='authored-nested-success') {
    const inner=at('@variant',name==='authored-outer-first-error'?'missing-inner':'focus',[decl('color','red')])
    const outer=at('@variant',name==='authored-outer-first-error'?'missing-outer':'hover',[inner])
    input=[rule('.nested',[outer])]
    retained.push(outer,inner,outer.nodes,inner.nodes)
  } else if(name==='authored-duplicate-alias') {
    const shared=at('@variant','hover',[decl('color','red')])
    input=[rule('.one',[shared]),rule('.two',[shared])]
    retained.push(shared,shared.nodes)
  } else if(name==='authored-live-template') {
    const shared=at('@variant','hover',[at('@slot')])
    const variant=at('@custom-variant','live',[shared])
    input=[variant,rule('.first',[shared]),rule('.second',[at('@variant','live',[decl('color','red')])])]
    retained.push(variant,variant.nodes,shared,shared.nodes)
    candidates=['live:block']
  } else if(name==='authored-slot-resources' || name==='authored-template-resources') {
    const property=at('@property','--sample',[decl('syntax','"*"'),decl('inherits','false'),decl('initial-value','initial')])
    const keyframes=at('@keyframes','sample',[rule('to',[decl('opacity','1')])])
    const src=[{file:'entry.css',code:'resources'},0,9]
    property.src=src;property.dst=src;keyframes.src=src
    property.extra={retained:true,sharedSource:src[0]}
    const resourceNodes=[property,keyframes]
    const variant=at('@custom-variant','resources',[rule('&:hover',[
      ...(name==='authored-template-resources'?resourceNodes:[]),at('@slot')])])
    const authored=at('@variant','resources',[
      ...(name==='authored-slot-resources'?resourceNodes:[]),decl('color','red')])
    input=[variant,rule('.resources',[authored])]
    retained.push(variant,variant.nodes,authored,authored.nodes,property,keyframes,
      property.nodes,keyframes.nodes,property.extra,src)
  } else if(name==='authored-assign-fields') {
    const src=[{file:'entry.css',code:'red'},0,3]
    const first=at('@variant','hover',[decl('color','red')])
    first.src=src;first.dst=src;first.extra={retained:true}
    for(const key of ['kind','selector','nodes']) {
      let value=first[key]
      Object.defineProperty(first,key,{enumerable:true,get:()=>value,set:next=>{
        trace.push([key,graph([next])]);value=next
      }})
    }
    input=[rule('.one',[first])]
    retained.push(first,first.nodes,src,first.extra)
  } else if(name==='shared-variant-utility-declaration' || name==='shared-variant-two-utilities') {
    const shared=decl('color','--value(--color)')
    if(name==='shared-variant-two-utilities') {
      let value=shared.value
      Object.defineProperty(shared,'value',{enumerable:true,get:()=>value,set:next=>{
        trace.push(['shared.value',next]);value=next
      }})
    }
    const variant=at('@custom-variant','aliased',[rule('&:hover',[shared,at('@slot')])])
    const utility=at('@utility','color-*',[shared])
    input=[variant,utility]
    retained.push(shared,variant,utility,variant.nodes,utility.nodes)
    if(name==='shared-variant-two-utilities') {
      const second=at('@utility','other-*',[shared])
      input.push(second);retained.push(second,second.nodes)
    }
    candidates=['aliased:block']
  } else if(name==='theme-function-first-error') {
    const authored=decl('color','theme(--missing-authored)')
    const themed=decl('--x','theme(--missing-theme)')
    input=[rule('.first',[authored]),at('@theme','',[themed])]
    retained.push(authored,themed)
  } else if(name==='theme-keyframes-function-prefix') {
    const first=decl('width','--spacing(2)'),second=decl('color','theme(--missing)')
    const keyframes=at('@keyframes','spin',[rule('to',[first,second])])
    const theme=at('@theme','',[decl('--spacing','.25rem'),keyframes])
    input=[theme]
    retained.push(theme,theme.nodes,keyframes,keyframes.nodes,first,second)
  } else {
    const nodes=[decl('--spacing','.25rem'),decl('--x','--spacing(2)')]
    if(name==='theme-function-failure')nodes.push(decl('--bad','theme(--missing)'))
    const theme=at('@theme','',nodes)
    input=[theme]
    retained.push(theme,theme.nodes)
    let current=input[0]
    Object.defineProperty(input,'0',{enumerable:true,get:()=>current,set:next=>{
      current=next
      if(next.kind==='rule') {
        let nodes=next.nodes
        Object.defineProperty(next,'nodes',{enumerable:true,get:()=>nodes,set:value=>{
          trace.push(['theme.nodes',graph([value])]);nodes=value
        }})
      }
    }})
  }
  input.push(at('@tailwind','utilities'))
  return {input,candidates,roots:[input,trace,...retained]}
}
async function observe(name,wasm,mode) {
  const value=fixture(name)
  let compiler,error,output
  try {
    const config=JSON.stringify({limits,options:{from:'entry.css'}})
    compiler=wasm?(mode==='async'?await facade.compileAst(value.input,config):new facade.AstCompiler(value.input,config))
      :await tailwind.compileAst(value.input,{from:'entry.css'})
    output=compiler.build(value.candidates)
  } catch(failure) {error=[failure.name,failure.message]}
  const prefix=graph([...value.roots,output])
  compiler?.free?.()
  for(let index=0;index<80;index++)await Promise.resolve()
  return {error,prefix,settled:graph([...value.roots,output])}
}
const failures=[]
const cases=['authored-success-before-failure','authored-outer-first-error','authored-nested-success',
  'authored-duplicate-alias','authored-live-template','authored-assign-fields',
  'authored-slot-resources','authored-template-resources',
  'shared-variant-utility-declaration','shared-variant-two-utilities',
  'theme-publication-before-functions','theme-function-failure','theme-function-first-error','theme-keyframes-function-prefix']
for(const name of cases) {
  for(const mode of ['sync','async']) {
    const expected=await observe(name,false,mode),actual=await observe(name,true,mode)
    if(name==='authored-success-before-failure' || name==='authored-outer-first-error' || name==='theme-function-failure'
      || name==='theme-function-first-error' || name==='theme-keyframes-function-prefix')assert(expected.error)
    else assert.equal(expected.error,undefined)
    try {assert.deepEqual(actual,expected,name)}
    catch {failures.push({name,mode,expected,actual})}
  }
}
console.log(JSON.stringify({ownershipCases:cases.length,entryComparisons:cases.length*2,completePrefixAndSettledGraphs:true,
  failures,actualWasm:true,wasmBytes:bytes.length,fullOwnershipConformance:'pending'}))
assert.deepEqual(failures,[],'postcompat shared ownership and completed mutation prefix')
