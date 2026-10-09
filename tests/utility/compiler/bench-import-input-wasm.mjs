import assert from 'node:assert/strict'
import {createHash} from 'node:crypto'
import {readFile} from 'node:fs/promises'
import {resolve} from 'node:path'
import {performance} from 'node:perf_hooks'
import {pathToFileURL} from 'node:url'

const directory=resolve(process.argv[2] ?? 'target/utility-conformance/compiler-api')
const facade=await import(pathToFileURL(resolve(directory,'style.js')))
const bytes=await readFile(resolve(directory,'style_bg.wasm'))
await facade.default({module_or_path:bytes})
const limits={input_bytes:262144,asset_bytes:262144,candidate_count:1024,ast_depth:32,
  css_bytes:262144,queued_bytes:262144,cache_bytes:16000000}
const assets=[{id:'one.css',path:'/mapped/one.css',base:'/discover',content:'.imported { color: red; --x: blue; }'}]
const config=JSON.stringify({limits,options:{from:'entry.css',base:'/discover'},assets})
function fixture() {
  return [...Array.from({length:16},() => ({kind:'at-rule',name:'@import',params:'"one.css"',nodes:[]})),
    {kind:'at-rule',name:'@tailwind',params:'utilities',nodes:[]}]
}
// Compare CSS-bearing fields only: this change intentionally repairs src/dst
// and input identities. Their complete graphs are checked by separate gates.
function shape(nodes) {
  return nodes.map(node => ({kind:node.kind,...Object.fromEntries(['name','params','selector','property','value','important'].filter(key => key in node).map(key => [key,node[key]])),
    ...('nodes' in node ? {nodes:shape(node.nodes)} : {})}))
}
function sample() {
  const constructors=[],builds=[],hash=createHash('sha256')
  const started=performance.now()
  for (let session=0;session<100;session++) {
    const input=fixture(), start=performance.now()
    const compiler=new facade.AstCompiler(input,config)
    constructors.push(performance.now()-start)
    try {
      for (const batch of [[],['block'],['flex'],['grid']]) {
        const before=performance.now(), output=compiler.build(batch)
        builds.push(performance.now()-before)
        hash.update(JSON.stringify(shape(output)))
      }
    } finally { compiler.free() }
  }
  const elapsed=performance.now()-started
  const p95=values => values.sort((a,b) => a-b)[Math.ceil(values.length*0.95)-1]
  return {constructorP95Ms:p95(constructors),buildP95Ms:p95(builds),sessionsPerSecond:100000/elapsed,
    outputSha256:hash.digest('hex'),sessions:100,builds:400}
}
sample()
const samples=Array.from({length:5},sample)
assert(samples.every(value => value.outputSha256===samples[0].outputSha256))
const median=field => samples.map(value => value[field]).sort((a,b) => a-b)[2]
console.log(JSON.stringify({scope:'public AST constructor with sixteen repeated tracked imports and four builds',
  samples,constructorP95Ms:median('constructorP95Ms'),buildP95Ms:median('buildP95Ms'),
  sessionsPerSecond:median('sessionsPerSecond'),wasmBytes:bytes.length,
  wasmSha256:createHash('sha256').update(bytes).digest('hex'),actualWasm:true,finalPerformanceAcceptance:'pending'}))
