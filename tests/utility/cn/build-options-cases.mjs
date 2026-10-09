// Differential options read order against the original locked source.
import assert from 'node:assert/strict'
import * as fs from 'node:fs'
import {join} from 'node:path'
export const optionsReadEvidence=[]
function sameReads(stage,actual,expected){optionsReadEvidence.push({stage,actual:[...actual],expected:[...expected]});assert.deepEqual(actual,expected)}
export async function optionsCases({check,session,wasm,oracle,referenceVite,referenceNext,host,limits,project}) {
 const sentinel={optionThrow:true};
 function traced(values,script={},enumerate=false){
  const log=[],counts={};const raw=new Proxy(values,{get(target,key,receiver){log.push(`get:${String(key)}`);const n=counts[key]=(counts[key]??0)+1;return key in script?script[key](n,target):Reflect.get(target,key,receiver)},ownKeys(target){if(enumerate)log.push('ownKeys');return Reflect.ownKeys(target)},getOwnPropertyDescriptor(target,key){if(enumerate)log.push(`descriptor:${String(key)}`);return Reflect.getOwnPropertyDescriptor(target,key)}})
  return {raw,log,values}
 }
 const shape=r=>({source:r.source,tokens:[...r.tokens],scannedFiles:r.scannedFiles,usedGroups:r.usedGroups,totalGroups:r.totalGroups,warnings:r.warnings});
 async function compare(name,values,script={}){
  const cwd=project('option-'+name,{'index.ts':'p-2 p-4','tokens.txt':'m-2 m-4','safe.txt':'text-lg','theme.css':'@theme { --color-probe: blue; }'});
  const a=traced({cwd,...values},script),b=traced({cwd,...values},script)
  const expected=await oracle.build(a.raw);fs.rmSync(expected.outPath,{force:true});const actual=await session.build(b.raw)
  sameReads(name,b.log,a.log);assert.deepEqual(shape(actual),shape(expected))
 }
 await check('options-build-content-read-order',()=>compare('content',{css:false,content:['*.ts']}))
 await check('options-build-default-CSS-and-empty-content-read-order',()=>compare('default',{}))
 await check('options-build-CSS-token-safelist-repeated-reads',()=>compare('tokens',{css:'theme.css',tokens:'tokens.txt',safelist:'safe.txt'}))
 await check('options-build-full-short-circuit-unread-throwing-getters',()=>compare('full',{css:false,full:true},{tokens(){throw sentinel},content(){throw sentinel},safelist(){throw sentinel}}))
 await check('options-build-full-reads-may-change-between-stages',()=>compare('dynamic-full',{css:false,content:['*.ts']},{full:n=>n===2}))
 await check('options-build-getter-throw-original-identity',async()=>{
  const cwd=project('option-throw',{'index.ts':'p-2'});const a=traced({cwd},{css(){throw sentinel}}),b=traced({cwd},{css(){throw sentinel}})
  await assert.rejects(oracle.build(a.raw),e=>e===sentinel);await assert.rejects(session.build(b.raw),e=>e===sentinel);sameReads(`options-${optionsReadEvidence.length+1}`,b.log,a.log)
 })
 await check('options-build-await-transform-sees-late-full',async()=>{
  const cwd=project('option-await',{'index.ts':'p-2','config.mjs':'export default c=>{globalThis.__rustifyOptionValues.full=true;return c}'});
  const a=traced({cwd,css:false,config:'config.mjs'}),b=traced({cwd,css:false,config:'config.mjs'});
  try{globalThis.__rustifyOptionValues=a.values;const expected=await oracle.build(a.raw);fs.rmSync(expected.outPath,{force:true});globalThis.__rustifyOptionValues=b.values;const actual=await session.build(b.raw);sameReads(`options-${optionsReadEvidence.length+1}`,b.log,a.log);assert.deepEqual(shape(actual),shape(expected))}finally{delete globalThis.__rustifyOptionValues}
 })
 await check('options-builder-create-run-changed-lazy-reads',async()=>{
  const cwd=project('option-builder',{'index.ts':'p-2','theme.css':'@theme { --color-probe: blue; }'});const a=traced({cwd,css:false,content:['*.ts']}),b=traced({cwd,css:false,content:['*.ts']});const ref=oracle.createBuilder(a.raw),actual=session.create_builder(b.raw);sameReads(`options-${optionsReadEvidence.length+1}`,b.log,a.log)
  a.log.length=b.log.length=0;await ref.run();await actual.run();sameReads(`options-${optionsReadEvidence.length+1}`,b.log,a.log)
  a.log.length=b.log.length=0;await ref.changed('cn-tables.mjs');await actual.changed('cn-tables.mjs');sameReads(`options-${optionsReadEvidence.length+1}`,b.log,a.log)
  a.values.full=b.values.full=true;a.log.length=b.log.length=0;await ref.changed('index.ts');await actual.changed('index.ts');sameReads(`options-${optionsReadEvidence.length+1}`,b.log,a.log)
  actual.free()
 })
 await check('options-builder-known-CSS-short-circuits-option-reads',async()=>{
  const cwd=project('option-builder-css',{'index.ts':'p-2','theme.css':'@theme { --color-probe: blue; }'});const a=traced({cwd,content:['*.ts']}),b=traced({cwd,content:['*.ts']});const ref=oracle.createBuilder(a.raw),actual=session.create_builder(b.raw);await ref.run();await actual.run();a.log.length=b.log.length=0;await ref.changed('theme.css');await actual.changed('theme.css');sameReads(`options-${optionsReadEvidence.length+1}`,b.log,a.log);actual.free()
 })
 await check('options-Vite-spread-and-deferred-builder-getters',async()=>{
  const cwd=project('option-vite',{'index.ts':'p-2'});const a=traced({cwd,css:false,content:['*.ts'],extra:'retained'}, {},true),b=traced({cwd,css:false,content:['*.ts'],extra:'retained'},{},true);const ref=referenceVite.cn(a.raw),actual=session.vite(b.raw);sameReads(`options-${optionsReadEvidence.length+1}`,b.log,a.log);ref.configResolved({root:cwd});actual.configResolved({root:cwd});sameReads(`options-${optionsReadEvidence.length+1}`,b.log,a.log);a.log.length=b.log.length=0;await ref.buildStart();await actual.buildStart();sameReads(`options-${optionsReadEvidence.length+1}`,b.log,a.log)
  const c=traced({cwd,css:false}),d=traced({cwd,css:false});const refLazy=referenceVite.cn(c.raw),actualLazy=session.vite(d.raw);await refLazy.buildStart();await actualLazy.buildStart();sameReads('Vite-lazy-buildStart',d.log,c.log)
 })
 await check('options-Next-factory-builder-and-development-cwd-read-order',async()=>{
  const cwd=project('option-next',{'index.ts':'p-2'});const a=traced({cwd,css:false}),b=traced({cwd,css:false});const ref=referenceNext.withCn({},a.raw),actual=session.with_cn({},b.raw);sameReads(`options-${optionsReadEvidence.length+1}`,b.log,a.log);const expected=ref('phase-production-build',{}),got=actual('phase-production-build',{});expected.catch(()=>{});got.catch(()=>{});sameReads('Next-call-before-await',b.log,a.log);await expected;await got;sameReads(`options-${optionsReadEvidence.length+1}`,b.log,a.log);a.log.length=b.log.length=0;await ref('phase-development-server',{});await actual('phase-development-server',{});sameReads(`options-${optionsReadEvidence.length+1}`,b.log,a.log)
 })
 await check('options-build-repeated-path-and-error-label-getters',async()=>{
  const cwd=project('option-labels',{'index.ts':'p-2','tokens.txt':'m-2','safe.txt':'text-lg'});
  for(const key of ['tokens','safelist']){
   const values={cwd,css:false,tokens:'tokens.txt',safelist:'safe.txt'};const script={[key]:n=>n===1?'condition.txt':n===2?'missing.txt':'display-label.txt'};const a=traced({...values},script),b=traced({...values},script);
   let expected;try{await oracle.build(a.raw)}catch(e){expected=e}
   await assert.rejects(session.build(b.raw),e=>e.message===expected.message&&e.cause.code===expected.cause.code);sameReads(`options-${optionsReadEvidence.length+1}`,b.log,a.log)
  }
 })
 await check('options-Vite-async-hooks-reject-original-getter-throw',async()=>{
  const cwd=project('option-vite-throws',{'index.ts':'p-2'});const a=traced({cwd},{content(){throw sentinel}}),b=traced({cwd},{content(){throw sentinel}});const ref=referenceVite.cn(a.raw),actual=session.vite(b.raw);
  const expected=ref.buildStart();expected.catch(()=>{});let result;assert.doesNotThrow(()=>{result=actual.buildStart()});assert.equal(typeof result.then,'function');await assert.rejects(expected,e=>e===sentinel);await assert.rejects(result,e=>e===sentinel);sameReads(`options-${optionsReadEvidence.length+1}`,b.log,a.log)
  const refChange=referenceVite.cn({cwd}),actualChange=session.vite({cwd});const change={get event(){throw sentinel}};const er=refChange.watchChange('index.ts',change);er.catch(()=>{});let ar;assert.doesNotThrow(()=>{ar=actualChange.watchChange('index.ts',change)});await assert.rejects(er,e=>e===sentinel);await assert.rejects(ar,e=>e===sentinel)
 })
 await check('options-CLI-summary-original-print-stage-getters',()=>{
  const original=fs.readFileSync(new URL('../../../ref/cn/packages/cn/bin/cn.mjs',import.meta.url),'utf8');const body=original.slice(original.indexOf('  const gz ='),original.lastIndexOf('\n}'));
  // Executes the original fixed print body, with injected primitive gzip/console.
  const print=new Function('result','opts','gzipSync','console',body);
  const report={source:'x',scannedFiles:1,tokens:new Set(['p-2']),usedGroups:1,totalGroups:2};const a=traced({out:'tables.mjs',full:false}),b=traced({out:'tables.mjs',full:false});const lines=[];print(report,a.raw,()=>({length:21}),{log:v=>lines.push(v)});const customHost={...host,gzipSize:()=>21};const actual=new wasm.CnBuildSession(customHost,limits);assert.equal(actual.cli_summary(report,b.raw),lines.join('\n')+'\n');sameReads(`options-${optionsReadEvidence.length+1}`,b.log,a.log);actual.free()
 })

 await check('options-Next-getter-bounded-factory-reentry',async()=>{
  const cwd=project('option-next-reentry',{'index.ts':'p-2'});
  const run=async wrap=>{let factory,entered=false,pending;const trace=traced({css:false},{cwd(){if(!entered){entered=true;pending=factory('phase-production-build',{})}return cwd}});factory=wrap({},trace.raw);const outer=factory('phase-production-build',{});await Promise.all([outer,pending]);return trace.log};
  const expected=await run(referenceNext.withCn),actual=await run((config,opts)=>session.with_cn(config,opts));sameReads('Next-bounded-getter-reentry',actual,expected)
 })

 await check('options-Vite-getter-bounded-hook-reentry',async()=>{
  const cwd=project('option-vite-reentry',{'index.ts':'p-2'});
  const run=async wrap=>{let plugin,entered=false,pending;const trace=traced({css:false},{cwd(){if(!entered){entered=true;pending=plugin.buildStart()}return cwd}});plugin=wrap(trace.raw);const outer=plugin.buildStart();await Promise.all([outer,pending]);return trace.log};
  const expected=await run(referenceVite.cn),actual=await run(opts=>session.vite(opts));sameReads('Vite-bounded-getter-reentry',actual,expected)
 })

}
