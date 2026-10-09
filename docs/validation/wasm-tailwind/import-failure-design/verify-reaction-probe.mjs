// Architecture experiment only: compares Rust reaction scheduling with the
// pinned JS oracle. Dependency fixtures do not validate production AST parsing.
import assert from 'node:assert/strict'
import {createHash} from 'node:crypto'
import {readFileSync} from 'node:fs'
import {spawnSync} from 'node:child_process'
import {resolve} from 'node:path'
const inventory=JSON.parse(readFileSync('tests/utility/compiler/inventory.json','utf8'))
assert.equal(inventory.baseline.commit,'1334c99db8fd26c8ea065375dd9259800863f072')
for (const name of ['index.ts','at-import.ts','ast.ts','css-parser.ts']) {
  const path=`packages/tailwindcss/src/${name}`
  assert.equal(createHash('sha256').update(readFileSync(resolve('ref/tailwindcss',path))).digest('hex'),
    inventory.sources.find(source=>source.path===path)?.sha256,path)
}
const directory=resolve(import.meta.dirname)
function run(command,args) {
  const result=spawnSync(command,args,{encoding:'utf8'})
  assert.equal(result.status,0,result.stderr || result.error?.message)
  return result.stdout.trim().split('\n').map(line=>JSON.parse(line))
}
const native=run(process.argv[2] ?? '/private/tmp/rustify-import-reaction-probe',[])
const oracle=run(process.execPath,[resolve(directory,'horizon-oracle.mjs')])
const expected=oracle.filter(row=>row.name)
const unhandled=oracle.find(row=>row.unhandledRejections)
const error=value=>value?.startsWith('Missing closing }')?'parser':value?.startsWith('`layer(')?'parameters':value ?? null
function compare(actual) {
  assert.equal(actual.length,expected.length)
  const byName=new Map(actual.map(row=>[row.name,row]))
  for (const wanted of expected) {
    const got=byName.get(wanted.name)
    assert(got,wanted.name)
    for (const field of ['atCatch','trace','afterCatch']) assert.deepEqual(got[field],wanted[field],`${wanted.name}: ${field}`)
    assert.equal(got.error,error(wanted.error),`${wanted.name}: selected error`)
  }
}
compare(native)
// The instrument must reject exactly the all-success-before-error workaround.
const drained=structuredClone(native)
const deep=drained.find(row=>row.name==='chain2-then-missing')
deep.trace.push(...deep.afterCatch);deep.afterCatch=[]
assert.throws(()=>compare(drained),/chain2-then-missing/)
const cancelled=structuredClone(native)
cancelled.find(row=>row.name==='host-nodes-first').afterCatch=[]
assert.throws(()=>compare(cancelled),/host-nodes-first/)
assert.deepEqual(unhandled.unhandledRejections,['missing missing.css'])
console.log(JSON.stringify({schedulingCases:native.length,calibrationRejected:2,
  traces:'load, original index writes, nodes writes before/after observation; all marker epochs',
  unhandledRejection:'observed in official oracle; NOT implemented by prototype',
  productionImportFailureGate:'pending',productionAstConformance:false,prototypeOnly:true}))
