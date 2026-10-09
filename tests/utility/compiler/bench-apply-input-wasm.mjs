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
const params=['block','flex underline','hover:block flex','size-[2px] block','border-2 block','[--unit:2px] block']
function fixture() {
  const source={file:'apply.css',code:params.map(value => `@apply ${value};`).join(' ')}
  return [{kind:'rule',selector:'.sheet',nodes:params.map((value,index) =>
    ({kind:'at-rule',name:'@apply',params:value,nodes:[],src:[source,index*20,index*20+19]}))},
    {kind:'at-rule',name:'@tailwind',params:'utilities',nodes:[]}]
}
function sample() {
  const constructors=[],builds=[],hash=createHash('sha256')
  const started=performance.now()
  for (let session=0;session<200;session++) {
    const input=fixture(), start=performance.now()
    const compiler=new facade.AstCompiler(input,JSON.stringify({limits,options:{from:'apply.css'}}))
    constructors.push(performance.now()-start)
    try {
      for (const batch of [[],['block'],['flex'],['size-[4px]']]) {
        const before=performance.now(), output=compiler.build(batch)
        builds.push(performance.now()-before)
        hash.update(JSON.stringify(output))
      }
    } finally { compiler.free() }
  }
  const elapsed=performance.now()-started
  const p95=values => values.sort((a,b) => a-b)[Math.ceil(values.length*0.95)-1]
  return {constructorP95Ms:p95(constructors),buildP95Ms:p95(builds),sessionsPerSecond:200000/elapsed,
    outputSha256:hash.digest('hex'),sessions:200,builds:800}
}
sample()
const samples=Array.from({length:5},sample)
assert(samples.every(value => value.outputSha256===samples[0].outputSha256))
const median=field => samples.map(value => value[field]).sort((a,b) => a-b)[2]
console.log(JSON.stringify({scope:'public AST constructor with six direct apply directives and four builds',
  samples,constructorP95Ms:median('constructorP95Ms'),buildP95Ms:median('buildP95Ms'),
  sessionsPerSecond:median('sessionsPerSecond'),wasmBytes:bytes.length,
  wasmSha256:createHash('sha256').update(bytes).digest('hex'),actualWasm:true,finalPerformanceAcceptance:'pending'}))
