import assert from 'node:assert/strict'
import {createRequire} from 'node:module'
import {resolve} from 'node:path'
const oracle=createRequire(resolve('tests/utility/compiler/oracle/package.json'))
const tailwind=oracle('tailwindcss')
const unhandled=[]
process.on('unhandledRejection',error=>unhandled.push(error.message))
assert.equal(oracle('tailwindcss/package.json').version,'4.1.13')
const at=id=>({kind:'at-rule',name:'@import',params:JSON.stringify(id),nodes:[]})
const assets=new Map([['leaf.css','.leaf { color: red; }']])
for(let depth=1;depth<=10;depth++)assets.set(`chain${depth}.css`,`@import "${depth===1?'leaf.css':`chain${depth-1}.css`}";`)
for(let depth=0;depth<=10;depth++)assets.set(`bad${depth}.css`,depth===0?'@import "missing.css";':`@import "bad${depth-1}.css";`)
for (const [name,ids] of [
 ...Array.from({length:11},(_,d)=>[`chain${d}-then-missing`,[d===0?'leaf.css':`chain${d}.css`,'missing.css']]),
 ...Array.from({length:4},(_,d)=>[`nestedbad${d}-leaf`,[`bad${d}.css`,'leaf.css']]),
 ['nestedgood-first-missing',['chain3.css','missing.css']],
 ['leaf-chain3',['leaf.css','chain3.css']],
 ['chain3-leaf',['chain3.css','leaf.css']],
 ['badparams-leaf',['leaf.css','parameter-error']],
 ['leaf-parsererror',['leaf.css','parser.css']],
 ['leaf-nestedparsererror',['leaf.css','nestedparser.css']],
 ['nested-error-versus-direct',['bad0.css','missing.css']],
 ['missing-versus-sync-params',['missing.css','parameter-error']],
 ['host-index-first',['leaf.css','chain3.css']],
 ['host-nodes-first',['leaf.css','chain3.css']],
]) {
 assets.set('parser.css','.broken { color: red;')
 assets.set('nestedparser.css','@import "parser.css";')
 let tick=0,pump=true
 function mark(){if(pump)queueMicrotask(()=>{tick++;mark()})}mark()
 const trace=[]
 const input=new Proxy(ids.map(id=>id==='parameter-error'?{...at('leaf.css'),params:'"leaf.css" supports(display: grid) layer(a)'}:at(id)),{set(target,key,value){
   if(/^\d+$/.test(key)){trace.push(['index',key,tick]);let nodes=value.nodes;Object.defineProperty(value,'nodes',{enumerable:true,configurable:true,get:()=>nodes,set:next=>{trace.push(['nodes',key,tick]);if(name==='host-nodes-first' && key==='0')throw new Error('host nodes');nodes=next}})}
   if(name==='host-index-first' && key==='0')throw new Error('host index')
   return Reflect.set(target,key,value)
 }})
 let error
 try{await tailwind.compileAst(input,{loadStylesheet:async(id)=>{trace.push(['load',id,tick]);if(!assets.has(id))throw new Error('missing '+id);return{path:id,base:'',content:assets.get(id)}}})}catch(e){error=e.message}
 const atCatch=tick,seen=trace.length
 for(let i=0;i<60;i++)await Promise.resolve()
 pump=false
 console.log(JSON.stringify({name,error,atCatch,trace:trace.slice(0,seen),afterCatch:trace.slice(seen)}))
}

await new Promise(setImmediate)
console.log(JSON.stringify({unhandledRejections:unhandled}))
