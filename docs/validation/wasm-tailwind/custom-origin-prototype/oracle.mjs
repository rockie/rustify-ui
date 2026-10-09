import assert from 'node:assert/strict'
import {createRequire} from 'node:module'
import {writeFile} from 'node:fs/promises'
import {resolve} from 'node:path'
const require=createRequire(resolve('tests/utility/compiler/oracle/package.json'))
const tailwind=require('tailwindcss')
assert.equal(require('tailwindcss/package.json').version,'4.1.13')
const decl=(property,value) => ({kind:'declaration',property,value,important:false})
const source=() => ({file:'s\ud800',code:'c\udc00',extra:'source'})
const first=source(),tuple=[first,1.5,9]
const child={...decl('color','--alpha(--value(--color) / 50%)'),extra:{first,equalDistinct:source()},src:tuple,dst:tuple}
const second={...decl('background','red'),extra:{child}}
const body=[child,second]
const rule={kind:'rule',selector:'& .nested',nodes:body,extra:{body,child}}
const supports={kind:'at-rule',name:'@supports',params:'(display: grid)',nodes:[rule],extra:first}
const comment={kind:'comment',value:'comment',extra:'comment metadata'}
const context={kind:'context',context:{probe:true},nodes:[comment],extra:'context metadata'}
const root={kind:'at-root',nodes:[decl('--probe','1')],extra:'root metadata'}
const nodes=[supports,context,root]
const definition={kind:'at-rule',name:'@utility',params:'tone-*',nodes,extra:{wrapperOnly:'wrapper',body:nodes}}
await tailwind.compileAst([definition,{kind:'at-rule',name:'@tailwind',params:'utilities',nodes:[]}])
function snapshot(roots) {
  const seen=new Map(),records=[]
  function visit(value) {
    if (typeof value==='string') return {units:Array.from({length:value.length},(_,i) => value.charCodeAt(i))}
    if (value===null || typeof value!=='object') return value
    if (seen.has(value)) return {reference:seen.get(value)}
    const id=records.length;seen.set(value,id);records.push(null)
    records[id]={array:Array.isArray(value),fields:Object.keys(value).map(k => [k,visit(value[k])])}
    return {reference:id}
  }
  return {roots:roots.map(visit),records}
}
assert.equal(child.value,'color-mix(in oklab, --value(--color-*) 50%, transparent)')
await writeFile(process.argv[2] ?? new URL('./oracle.json',import.meta.url),JSON.stringify({
  value:child.value,functional:snapshot([structuredClone(definition)]),static:snapshot([structuredClone(definition.nodes)])})+'\n')
console.log(JSON.stringify({tailwind:'4.1.13',rootScopes:2,value:child.value,fullNativeIntegration:'pending'}))
