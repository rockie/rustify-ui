import assert from 'node:assert/strict'
import {createHash} from 'node:crypto'
import {readFile} from 'node:fs/promises'
import {resolve} from 'node:path'
import {pathToFileURL} from 'node:url'

const directory=resolve(process.argv[2] ?? 'target/utility-conformance/compiler-api')
const facade=await import(pathToFileURL(resolve(directory,'style.js')))
const bytes=await readFile(resolve(directory,'style_bg.wasm'))
const exports=await facade.default({module_or_path:bytes})
assert.equal(exports.memory.buffer instanceof SharedArrayBuffer,false)
const config=JSON.stringify({limits:{input_bytes:262144,asset_bytes:262144,candidate_count:1024,
  ast_depth:32,css_bytes:262144,queued_bytes:262144,cache_bytes:16000000}})
const input=`@theme { --color-card:red; --spacing:0.25rem; ${Array.from({length:400},(_,index) => `--color-unused-${index}:blue;`).join(' ')} } @tailwind utilities;`
const batches=[['bg-card'],...Array.from({length:16},(_,index) => [`p-${index+1}`])]
const quantile=(values,percentile) => [...values].sort((a,b) => a-b)[Math.ceil(values.length*percentile)-1]
const setup=[],build=[],runs=[]
let checksum,cssBytes=0
function run(measure) {
  const digest=createHash('sha256'),start=performance.now()
  for (let index=0;index<20;index++) {
    const before=performance.now(),compiler=new facade.CssCompiler(input,config)
    if (measure) setup.push(performance.now()-before)
    try {
      for (const batch of batches) {
        const before=performance.now(),css=compiler.build(batch)
        if (measure) { build.push(performance.now()-before); cssBytes+=Buffer.byteLength(css) }
        digest.update(css)
      }
    } finally { compiler.free() }
  }
  const sum=digest.digest('hex')
  if (checksum===undefined) checksum=sum
  else assert.equal(sum,checksum,'each run emits identical CSS')
  if (measure) runs.push(performance.now()-start)
}
run(false)
for (let sample=0;sample<5;sample++) run(true)
console.log(JSON.stringify({actualWasm:true,sharedMemory:false,wasmBytes:bytes.length,
  wasmSha256:createHash('sha256').update(bytes).digest('hex'),themeTokens:402,measuredSessions:setup.length,
  changedBuilds:build.length,runMedianMs:quantile(runs,0.5),setupP95Ms:quantile(setup,0.95),
  buildP95Ms:quantile(build,0.95),changedBuildsPerSecond:build.length/(runs.reduce((a,b) => a+b,0)/1000),
  cssBytes,cssDigest:checksum,fullPerformanceConformance:'pending'}))
