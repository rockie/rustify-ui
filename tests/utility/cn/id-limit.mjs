// Development-only stress: original public threshold, no reduced test constant.
import assert from 'node:assert/strict'
import {readFileSync,writeFileSync} from 'node:fs'
import {resolve} from 'node:path'
import {pathToFileURL} from 'node:url'
import {load,assertLockedSource} from './upstream.mjs'
assertLockedSource()
const mode=process.argv[2];assert.ok(mode==='oracle'||mode==='wasm')
const tokens=2097152
const input=Array.from({length:tokens},(_,i)=>'[p'+i+':0]').join(' ')
let merge
if(mode==='oracle'){
  const {createEngine}=await load('engine')
  const {compileToTables}=await load('compiler')
  const compiled=compileToTables({theme:{},classGroups:{},conflictingClassGroups:{},conflictingClassGroupModifiers:{},orderSensitiveModifiers:[]})
  merge=createEngine(compiled.tables,compiled.validatorImpls,{cacheSize:0}).mergeUncached
}else{
  const module=await import(pathToFileURL(resolve('target/utility-conformance/cn/style.js')))
  await module.default({module_or_path:readFileSync('target/utility-conformance/cn/style_bg.wasm')})
  const session=new module.CnSession({theme:{},classGroups:{},conflictingClassGroups:{},conflictingClassGroupModifiers:{},orderSensitiveModifiers:[]},{maxDepth:128,maxNodes:tokens+1,maxUtf16Units:input.length+1,maxTokens:tokens,cacheBytes:20000000})
  merge=value=>session.merge_uncached(value)
}
let thrown
try{merge(input)}catch(error){thrown={name:error.name,message:error.message}}
const record={mode,tokens,utf16Units:input.length,expected:{name:'Error',message:'cn: too many distinct classes in one merge'},actual:thrown??null}
console.log(JSON.stringify(record,null,2))
writeFileSync(new URL('./evidence/id-limit-'+mode+'.json',import.meta.url),JSON.stringify(record,null,2)+'\n')
assert.deepEqual(record.actual,record.expected)
