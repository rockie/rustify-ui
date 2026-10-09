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
const fixtures=[
  ['caller-source-array',() => {
    const body=[at('@apply','block')],b=rule('.b',body)
    const source={file:'source.css',code:'@apply flex;',body}
    const a=rule('.a',[{...at('@apply','flex'),src:[source,0.5,11.5]}])
    const input=[b,a,at('@tailwind','utilities')]
    return {input,roots:[input,b,a,body,source]}
  }],
  ['static-body-alias',() => {
    const child={...decl('color','red'),extra:'child'}
    const body=[child]
    const nested={...rule('&:hover',body),extra:{child,body}}
    const source={file:'source.css',code:'custom\ud800',child,body}
    const definition={...at('@utility','card',[nested]),extra:{nested},src:[source,0.5,20.5]}
    const input=[definition,at('@tailwind','utilities')]
    return {input,roots:[input,definition,definition.nodes,nested,body,child,source]}
  }],
  ['functional-body-alias',() => {
    const child=decl('color','--value(--color)')
    const body=[child]
    const source={file:'source.css',code:'functional\ud800',child,body}
    const tuple=[source,0.5,19.5]
    const nested={...rule('&:hover',body),extra:{child,body,tuple,source},src:tuple,dst:tuple}
    const definition={...at('@utility','tone-*',[nested]),extra:{child,body,nested}}
    const input=[at('@theme','',[decl('--color-card','red')]),definition,at('@tailwind','utilities')]
    return {input,roots:[input,definition,definition.nodes,nested,body,child,source]}
  }],
  ['context-at-root-fields',() => {
    const child=decl('color','red'),body=[child]
    const metadata={child,body}
    const context={kind:'context',context:{probe:metadata},nodes:body,extra:metadata}
    const root={kind:'at-root',nodes:[rule('.outside',[{kind:'comment',value:'kept',extra:metadata},decl('display','block')])],extra:metadata}
    const definition=at('@utility','card',[context,root])
    const input=[definition,at('@tailwind','utilities')]
    return {input,roots:[input,definition,definition.nodes,context,root,body,child,metadata]}
  }],
  ['aliased-functions',() => {
    const writes=[],child={...decl('color','--alpha(red / 50%)')}
    let value=child.value
    Object.defineProperty(child,'value',{enumerable:true,get:() => value,set:next => {writes.push(next);value=next}})
    const definition=at('@utility','card',[child,child])
    const input=[definition,at('@tailwind','utilities')]
    return {input,roots:[input,definition,definition.nodes,child,writes]}
  }],
]
let phases=0
for (const [name,fixture] of fixtures) {
  const actual=fixture(),expected=fixture()
  const reference=await tailwind.compileAst(expected.input)
  const compiler=new facade.AstCompiler(actual.input,JSON.stringify({limits}))
  try {
    assert.deepEqual(graph(actual.roots),graph(expected.roots),`${name}: constructor`);phases++
    for (const batch of [[],name==='functional-body-alias' ? ['tone-card'] : ['card'],['block'],[],['flex']]) {
      actual.roots.push(compiler.build(batch));expected.roots.push(reference.build(batch))
      assert.deepEqual(graph(actual.roots),graph(expected.roots),`${name}: build ${JSON.stringify(batch)}`);phases++
    }
  } finally {compiler.free()}
}
console.log(JSON.stringify({customOriginCases:fixtures.length,constructorAndBuildPhases:phases,actualWasm:true,wasmBytes:bytes.length}))
