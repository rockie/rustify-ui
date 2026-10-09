import assert from 'node:assert/strict'
import { readFile } from 'node:fs/promises'
import { resolve } from 'node:path'
import { pathToFileURL } from 'node:url'
const output = resolve('target/utility-conformance/scanner-pipeline')
const facade = await import(pathToFileURL(resolve(output,'style.js')))
const exports = await facade.default({module_or_path:await readFile(resolve(output,'style_bg.wasm'))})
assert.equal(typeof SharedArrayBuffer !== 'undefined' && exports.memory.buffer instanceof SharedArrayBuffer, false)
const limits = {input_bytes:262144,asset_bytes:262144,candidate_count:8192,ast_depth:64,css_bytes:2097152,queued_bytes:2097152,cache_bytes:8388608}
const external = JSON.parse(await readFile(new URL('./external-variables-fixtures.json',import.meta.url)))
let comparisons = 0
for (const item of external.cases) {
  const session = new facade.CompilerSession(JSON.stringify({input:{css:item.input,entry:'/app/input.css'},assets:[],limits,owner:null}))
  try {
    for (const [index,batch] of item.batches.entries()) {
      const prepared = JSON.parse(session.prepare(JSON.stringify(batch)))
      assert.equal(prepared.report.css,item.expectedBuilds[index],`${item.id} ${index}`)
      assert.ok(prepared.report.rejected.every(item=>!item.candidate.startsWith('--')))
      session.commit(prepared.token)
      comparisons++
    }
  } finally {session.free()}
}
const corpus = JSON.parse(await readFile(new URL('./source-pipeline-fixtures.json',import.meta.url)))
let scans = 0
for (const item of corpus.cases) {
  const session = new facade.CompilerSession(JSON.stringify({input:{css:item.input,entry:item.entry},assets:item.assets,limits,owner:null}))
  const files = new Map(item.files.map(file=>[file.path,{kind:'file',...file,revision:1}]))
  const snapshot = () => JSON.stringify({root:'/',base:item.base,entries:[...item.directories.map(path=>({kind:'directory',path})),...files.values()]})
  try {
    for (const [index,step] of item.steps.entries()) {
      if (step.operation === 'put') {files.set(step.path,{kind:'file',path:step.path,content:step.content,revision:index+2});continue}
      if (step.operation === 'remove') {files.delete(step.path);continue}
      const prepare = () => JSON.parse(step.operation === 'scan' ? session.prepare_sources(snapshot()) : session.prepare_content(JSON.stringify(step.changed)))
      const before = session.report()
      const prepared = prepare()
      assert.equal(prepared.report.css,step.expectedCss,`${item.id} step ${index}`)
      assert.equal(session.report(),before)
      if (index === 0) {
        session.discard(prepared.token)
        assert.equal(session.report(),before)
        const retry = prepare()
        assert.deepEqual(retry.report,prepared.report)
        session.commit(retry.token)
      } else { session.commit(prepared.token) }
      const repeated = prepare()
      assert.equal(repeated.report.changed,false)
      assert.equal(repeated.report.metrics.parsed_candidates,0)
      session.commit(repeated.token)
      scans++
    }
  } finally {session.free()}
}
const css = '@theme { --color-brand: #123456; } @tailwind utilities source(none); @source "./src/**";'
const configuration = (budget=limits) => JSON.stringify({input:{css,entry:'/app/input.css'},assets:[],limits:budget,owner:null})
const sources = JSON.stringify({root:'/',base:'/app',entries:[{kind:'file',path:'app/src/a.html',content:'w-[137px] var(--color-brand)',revision:1}]})
const session = new facade.CompilerSession(configuration())
try {
  const before = session.report()
  const prepared = JSON.parse(session.prepare_sources(sources))
  assert.throws(()=>session.prepare_content('[]'),error=>JSON.parse(error).error.kind==='pending_prepared')
  session.discard(prepared.token)
  assert.equal(session.report(),before)
  assert.throws(()=>session.prepare_sources(JSON.stringify({root:'/',base:'/app',entries:[{kind:'unavailable',path:'app/src'}]})),error=>JSON.parse(error).kind==='scan')
  assert.equal(session.report(),before)
  const recovered = JSON.parse(session.prepare_sources(sources))
  session.commit(recovered.token)
  assert.match(JSON.parse(session.report()).css,/--color-brand: #123456/)
} finally {session.free()}
for (const [resource,budget,input,operation] of [
  ['queued_bytes',{queued_bytes:2},sources,'prepare_sources'],
  ['input_bytes',{input_bytes:css.length+'/app/input.css'.length},JSON.stringify([{content:'w-[137px] '.repeat(100),extension:'html'}]),'prepare_content'],
  ['asset_bytes',{asset_bytes:1},sources,'prepare_sources'],
  ['candidate_count',{candidate_count:1},sources,'prepare_sources'],
  ['cache_bytes',{cache_bytes:200},sources,'prepare_sources'],
  ['source_depth',{ast_depth:3},JSON.stringify({root:'/',base:'/app',entries:[{kind:'file',path:'app/src/nested/a.html',content:'flex',revision:1}]}),'prepare_sources'],
]) {
  const limited = new facade.CompilerSession(configuration({...limits,...budget}))
  try {
    const before = limited.report()
    assert.throws(()=>limited[operation](input),error=> {const result=JSON.parse(error).error;return result.kind==='limit'&&result.resource===resource},resource)
    assert.equal(limited.report(),before,resource)
    const retry = JSON.parse(limited.prepare('[]'))
    limited.commit(retry.token)
  } finally {limited.free()}
}
console.log(JSON.stringify({externalCases:external.cases.length,externalComparisons:comparisons,sourceCases:corpus.cases.length,sourceScans:scans,discardRetryAndNoop:'passed',limitsAndMissingInformation:'passed',sharedMemory:false}))
