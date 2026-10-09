// Actual Node primitives only. Discovery, fitting, compilation and lifecycle run in WASM.
import assert from 'node:assert/strict'
import * as fs from 'node:fs'
import { tmpdir } from 'node:os'
import { resolve, dirname, join, relative, sep, posix } from 'node:path'
import { pathToFileURL } from 'node:url'
import { spawnSync } from 'node:child_process'
import { gzipSync } from 'node:zlib'
import { load, assertLockedSource } from './upstream.mjs'
import {optionsCases,optionsReadEvidence} from './build-options-cases.mjs'
assertLockedSource()
const artifact=resolve(process.env.RUSTIFY_CN_BUNDLE??'target/utility/cn')
const {nodeHost,createCnBuild}=await import(pathToFileURL(join(artifact,'cn-node.mjs')))
const wasm=await import(pathToFileURL(join(artifact,'style.js')))
const bytes=fs.readFileSync(join(artifact,'style_bg.wasm'))
const initialized=await wasm.default({module_or_path:bytes})
assert.equal(initialized.memory.buffer instanceof SharedArrayBuffer,false)
assert.ok(WebAssembly.Module.imports(new WebAssembly.Module(bytes)).every(entry=>!/(?:fs|rayon|thread|wasi)/i.test(entry.name)))
const oracle=await load('build')
const compiler=await load('compiler')
const config=await load('config')
const referenceVite=await load('vite')
const referenceNext=await load('next')
const themeCss=await load('theme-css')
const limits={maxDepth:128,maxNodes:1000000,maxUtf16Units:10000000,maxTokens:1000000,cacheBytes:10000000}
const kind=st=>st.isSymbolicLink()?2:st.isDirectory()?1:st.isFile()?0:3
const host={cwd:()=>process.cwd(),resolve:(base,path)=>resolve(base,path),relative:(base,path)=>relative(base,path).split(sep).join("/"),dirname:path=>dirname(path),readText:p=>fs.readFileSync(p,'utf8'),writeText:(p,s)=>{fs.mkdirSync(dirname(p),{recursive:true});fs.writeFileSync(p,s)},realpath:p=>fs.realpathSync(p),kind:p=>kind(fs.statSync(p)),children:p=>fs.readdirSync(p,{withFileTypes:true}).map(e=>[e.name,kind(e)]),importModule:p=>import(pathToFileURL(p).href),gzipSize:source=>gzipSync(source,{level:9}).length,watch:(cwd,callback)=>fs.watch(cwd,{recursive:true,persistent:false},callback),reportError:e=>console.error(e)}
const temp=fs.mkdtempSync(join(tmpdir(),'rustify-cn-host-'))
let passed=0;const failures=[]
async function check(name,fn){try{await fn();passed++}catch(e){failures.push({name,error:e.stack??String(e)})}}
function project(name,files){const dir=join(temp,name);fs.mkdirSync(dir,{recursive:true});for(const [p,text]of Object.entries(files)){fs.mkdirSync(dirname(join(dir,p)),{recursive:true});fs.writeFileSync(join(dir,p),text)}return dir}
function comparable(r){return {source:r.source,changed:r.changed,tokens:[...r.tokens],scannedFiles:r.scannedFiles,cssFiles:r.cssFiles,usedGroups:r.usedGroups,totalGroups:r.totalGroups,warnings:r.warnings,groups:Object.keys(r.config.classGroups),fullGroups:Object.keys(r.fullConfig.classGroups)}}
try{
 await check('actual-WASM-build-export',()=>assert.equal(typeof wasm.CnBuildSession,'function'))
 if(typeof wasm.CnBuildSession==='function'){
  const session=new wasm.CnBuildSession(host,limits)
  await check('real-fs-default-subset-and-noop',async()=>{
   const cwd=project('default',{'index.tsx':'<div className="p-2 p-4 hover:bg-red-500"/>','node_modules/pkg/a.ts':'m-4','style.css':'@theme { --color-brand: red; }'})
   const expected=await oracle.build({cwd});fs.unlinkSync(join(cwd,'cn-tables.mjs'))
   const actual=await session.build({cwd});assert.deepEqual(comparable(actual),comparable(expected));assert.deepEqual(comparable(await session.build({cwd})),comparable(await oracle.build({cwd})))
  })

  async function compareBuild(name,files,options={}){
   const cwd=project(name,files);const opts={cwd,...options}
   const expected=await oracle.build(opts);fs.unlinkSync(resolve(cwd,options.out??'cn-tables.mjs'))
   const actual=await session.build(opts);assert.deepEqual(comparable(actual),comparable(expected));return {cwd,actual,expected}
  }
  await check('real-fs-safelist-and-tokens',()=>compareBuild('tokens',{'tokens.txt':'p-2 p-4 hover:m-1','safe.txt':'text-lg/7 text-xl/8','bad.ts':'missing'},{tokens:'tokens.txt',safelist:'safe.txt',css:false,out:'tables.ts'}))
  await check('real-fs-CSS-relative-import-and-reset',()=>compareBuild('css',{'app.ts':'font-semibold tracking-tight shadow-glow rounded-soft','theme.css':'@import "./more.css"; @theme { --font-weight-*: initial; --font-weight-semibold:600; --shadow-glow:1px; }','more.css':'@theme { --radius-soft: 1rem; --tracking-tight: .1rem; }'}))
  await check('real-fs-ignored-explicit-opt-in',()=>compareBuild('ignored',{'index.ts':'p-2','node_modules/pkg/a.ts':'m-4','.hidden/a.ts':'text-lg','.git/a.ts':'grid-cols-2','dist/a.ts':'flex'},{content:['**/*.ts','node_modules/pkg/*.ts','.hidden/*.ts','.git/*.ts','dist/*.ts']}))
  await check('real-fs-symbolic-link-cycle-output-alias',async()=>{
   const cwd=project('symlinks',{'src/a.ts':'p-2 p-4'});fs.symlinkSync(cwd,join(cwd,'src','loop'),'dir');fs.symlinkSync(join(cwd,'src'),join(cwd,'alias'),'dir')
   const first=await oracle.build({cwd,content:['**/*.ts'],out:'src/tables.ts'});fs.symlinkSync(join(cwd,'src','tables.ts'),join(cwd,'output-alias.ts'))
   const expected=await oracle.build({cwd,content:['**/*.ts'],out:'src/tables.ts'});const actual=await session.build({cwd,content:['**/*.ts'],out:'src/tables.ts'});assert.deepEqual(comparable(actual),comparable(expected));assert.ok(first.scannedFiles>=2)
  })
  await check('real-fs-long-token-and-invalid-UTF8',async()=>{
   const cwd=project('bytes',{'index.ts':'p-2 '+'x'.repeat(8193)});fs.writeFileSync(join(cwd,'bad.ts'),Buffer.from([0x70,0x2d,0x34,0x20,0xed,0xa0,0x80]));
   const expected=await oracle.build({cwd});fs.unlinkSync(join(cwd,'cn-tables.mjs'));const actual=await session.build({cwd});assert.deepEqual(comparable(actual),comparable(expected))
  })
  await check('real-module-default-extension-UTF16',()=>compareBuild('extension',{'index.ts':'p-2 p-4','config.mjs':'export default {prefix:"\\ud800",extend:{classGroups:{["\\udc00"]:["shape-\\ud800","shape-\\udc00"]},orderSensitiveModifiers:["\\ud800"]}}'},{config:'config.mjs',full:true,css:false}))
  await check('real-module-named-export-nullish-default',()=>compareBuild('named',{'index.ts':'shape-a shape-b','config.mjs':'export default null; export const config={extend:{classGroups:{shape:["shape-a","shape-b"]}}}'},{config:'config.mjs'}))
  await check('real-module-transform-sees-CSS-and-theme-getter',async()=>{
   globalThis.__rustifyThemeCalls=[]
   await compareBuild('transform',{'index.ts':'shape-a shape-b','style.css':'@theme { --radius-soft: 1rem; }','config.mjs':'export default base => { globalThis.__rustifyThemeCalls.push(base.theme.radius.some(v=>v==="soft")); const getter=theme=>["a","b"]; getter.isThemeGetter=true; base.classGroups.shape=[{shape:[getter]}]; return base }'},{config:'config.mjs'})
   assert.deepEqual(globalThis.__rustifyThemeCalls,[true,true]);delete globalThis.__rustifyThemeCalls
  })
  await check('real-module-transform-original-throw-identity',async()=>{
   const sentinel=globalThis.__rustifyThrow={kind:'transform'};const cwd=project('throw-transform',{'index.ts':'p-2','config.mjs':'export default ()=>{throw globalThis.__rustifyThrow}'})
   await assert.rejects(oracle.build({cwd,config:'config.mjs'}),e=>e===sentinel);await assert.rejects(session.build({cwd,config:'config.mjs'}),e=>e===sentinel);delete globalThis.__rustifyThrow
  })
  await check('real-module-import-cause-identity',async()=>{
   const sentinel=globalThis.__rustifyImportThrow={message:'import failed'};const cwd=project('throw-import',{'index.ts':'p-2','config.mjs':'throw globalThis.__rustifyImportThrow'})
   await assert.rejects(oracle.build({cwd,config:'config.mjs'}),e=>e.cause===sentinel);await assert.rejects(session.build({cwd,config:'config.mjs'}),e=>e.cause===sentinel);delete globalThis.__rustifyImportThrow
  })
  await check('real-module-missing-default',async()=>{
   const cwd=project('no-default',{'index.ts':'p-2','config.mjs':'export const other=1'});let expected;try{await oracle.build({cwd,config:'config.mjs'})}catch(e){expected=e.message};await assert.rejects(session.build({cwd,config:'config.mjs'}),e=>e.message===expected)
  })
  await check('real-module-custom-callback-refuses-emission',async()=>{
   const cwd=project('callback',{'index.ts':'probe-a','config.mjs':'export default {extend:{classGroups:{probe:[{probe:[v=>true]}]}}}'});let expected;try{await oracle.build({cwd,config:'config.mjs'})}catch(e){expected=e.message};await assert.rejects(session.build({cwd,config:'config.mjs'}),e=>e.message===expected)
  })
  await check('real-fs-write-error-cause',async()=>{
   const cwd=project('bad-out',{'index.ts':'p-2','blocked':'file'});let expected;try{await oracle.build({cwd,out:'blocked/tables.mjs'})}catch(e){expected=e};await assert.rejects(session.build({cwd,out:'blocked/tables.mjs'}),e=>e.message===expected.message&&e.cause.code===expected.cause.code)
  })
  await check('pure-UTF16-source-subset-stats-and-banner',()=>{
   const base=config.defaultConfig();base.prefix='\ud800';base.classGroups['\udc00']=['shape-\ud800','shape-\udc00'];base.orderSensitiveModifiers.push('\ud800');
   const tokens=new Set(['\ud800:shape-\ud800','\ud800:shape-\udc00','\ud800:p-2','\ud800:p-4']);
   const expected=compiler.subsetConfig(base,tokens);const actual=wasm.cn_subset_config(base,tokens,limits);assert.equal(actual.usedGroups,expected.usedGroups);assert.deepEqual(Object.keys(actual.config.classGroups),Object.keys(expected.config.classGroups));
   for(const lang of ['js','ts'])assert.equal(wasm.cn_compile_to_source(actual.config,{lang,banner:'// \ud800'},limits),compiler.compileToSource(expected.config,{lang,banner:'// \ud800'}));
   assert.deepEqual(wasm.cn_compile_stats(actual.config,limits),compiler.compileStats(expected.config));
  })
  await check('public-compileModel-complete-shape-callbacks-and-identity',()=>{
   assert.equal(typeof wasm.cn_compile_model,'function')
   const configs=[config.defaultConfig(),{theme:{},classGroups:{b:['b-a','b-b'],a:['a-a','a-b']},conflictingClassGroups:{},conflictingClassGroupModifiers:{},orderSensitiveModifiers:[]}]
   const callback=value=>value==='\ud800';const custom=config.defaultConfig();custom.classGroups['\ud800']=[{'probe':[callback]}];custom.prefix='\udc00';custom.orderSensitiveModifiers.push('\ud800');configs.push(custom)
   const shared={theme:{},classGroups:{a:[{a:['x','y']}],b:[{b:['x','y']}]},conflictingClassGroups:{},conflictingClassGroupModifiers:{},orderSensitiveModifiers:[]};configs.push(shared)
   for(const input of configs){
    const expected=compiler.compileModel(input),actual=wasm.cn_compile_model(input,limits)
    assert.deepEqual(actual,expected)
    assert.deepEqual(Object.keys(actual),Object.keys(expected))
    for(const name of expected.customNames)assert.equal(actual.impls[name],expected.impls[name])
    for(let i=0;i<expected.attachments.length;i++)for(let j=0;j<expected.sets.length;j++)assert.equal(actual.attachments[i].tails===actual.sets[j],expected.attachments[i].tails===expected.sets[j])
    for(const name of expected.customNames)assert.equal(actual.impls[name]('\ud800'),expected.impls[name]('\ud800'))
   }
   const sentinel={model:true};const throwing=config.defaultConfig();Object.defineProperty(throwing,'orderSensitiveModifiers',{get(){throw sentinel}});assert.throws(()=>wasm.cn_compile_model(throwing,limits),e=>e===sentinel)
   assert.throws(()=>wasm.cn_compile_model(config.defaultConfig(),{...limits,maxNodes:1}),e=>e.name==='InputLimit')
  })
  await check('pure-subset-callback-calls-and-throw-identity',()=>{
   const make=calls=>{const base=config.defaultConfig();base.classGroups.probe=[{probe:[v=>{calls.push(v);return true}]}];return base};const a=[],b=[];
   const tokens=new Set(['probe-\ud800','probe-\udc00','probe-a']);assert.equal(wasm.cn_subset_config(make(a),tokens,limits).usedGroups,compiler.subsetConfig(make(b),tokens).usedGroups);assert.deepEqual(a,b);
   const sentinel={callback:true};const base=make([]);base.classGroups.probe=[{probe:[()=>{throw sentinel}]}];assert.throws(()=>wasm.cn_subset_config(base,tokens,limits),e=>e===sentinel)
  })
  await check('actual-builder-run-Promise-coalescing',async()=>{
   const cwd=project('builder-flight',{'index.ts':'p-2'});const builder=session.create_builder({cwd});assert.equal(builder.last,null);const a=builder.run(),b=builder.run();assert.equal(a,b);const report=await a;assert.equal(report.changed,false);assert.equal(builder.last,report);builder.free()
  })
  await check('actual-builder-incremental-shared-selection',async()=>{
   const cwd=project('builder-watch',{'index.ts':'p-2 p-4','style.css':'@theme { --radius-soft: 1rem; }'});const builder=session.create_builder({cwd});const first=await builder.run();fs.writeFileSync(join(cwd,'index.ts'),'p-6');assert.equal(await builder.changed('index.ts'),first);fs.writeFileSync(join(cwd,'index.ts'),'p-6 m-4');const second=await builder.changed('index.ts');assert.notEqual(second,first);assert.ok('m' in second.config.classGroups);assert.equal(await builder.changed('cn-tables.mjs'),second);assert.equal(await builder.changed('node_modules/a.ts'),second);fs.unlinkSync(join(cwd,'index.ts'));assert.equal(await builder.changed('index.ts'),second);fs.writeFileSync(join(cwd,'index.ts'),'p-6 m-4');fs.writeFileSync(join(cwd,'style.css'),'@theme { --radius-wide: 2rem; }');assert.notEqual(await builder.changed('style.css'),second);builder.free()
  })
  await check('actual-builder-reject-queue-and-recover',async()=>{
   globalThis.__rustifyFlightThrow={flight:true};const cwd=project('builder-reject',{'index.ts':'p-2','config.mjs':'export default base=>{if(globalThis.__rustifyFlightThrow)throw globalThis.__rustifyFlightThrow;return base}'});const builder=session.create_builder({cwd,config:'config.mjs'});const a=builder.run(),b=builder.run();assert.equal(a,b);await assert.rejects(a,e=>e===globalThis.__rustifyFlightThrow);assert.equal(builder.last,null);delete globalThis.__rustifyFlightThrow;assert.ok(await builder.run());builder.free()
  })
  await check('actual-Vite-hooks-root-and-override',async()=>{
   const cwd=project('vite',{'index.ts':'p-2'}),other=project('vite-other',{'index.ts':'m-4'});const reference=referenceVite.cn({cwd:other});reference.configResolved({root:cwd});await reference.buildStart();const referenceSource=fs.readFileSync(join(other,'cn-tables.mjs'),'utf8');fs.unlinkSync(join(other,'cn-tables.mjs'));const plugin=session.vite({cwd:other});assert.equal(plugin.name,'cn:build');plugin.configResolved({root:cwd});assert.equal(await plugin.buildStart(),undefined);assert.ok(fs.existsSync(join(other,'cn-tables.mjs')));assert.ok(!fs.existsSync(join(cwd,'cn-tables.mjs')));assert.equal(fs.readFileSync(join(other,'cn-tables.mjs'),'utf8'),referenceSource);const before=fs.readFileSync(join(other,'cn-tables.mjs'),'utf8');fs.writeFileSync(join(other,'index.ts'),'p-2');await plugin.watchChange(join(other,'index.ts'),{event:'delete'});assert.equal(fs.readFileSync(join(other,'cn-tables.mjs'),'utf8'),before);assert.equal(await plugin.watchChange(join(other,'index.ts'),{event:'update'}),undefined);assert.notEqual(fs.readFileSync(join(other,'cn-tables.mjs'),'utf8'),before)
  })
  await check('actual-Next-factory-context-and-original-throw',async()=>{
   const cwd=project('next-factory',{'index.ts':'p-2'});const context={defaultConfig:{x:true}};const value={webpack:'same'};const reference=referenceNext.withCn(async(phase,ctx)=>{assert.equal(phase,'phase-production-build');assert.equal(ctx,context);return value},{cwd});assert.equal(await reference('phase-production-build',context),value);const referenceSource=fs.readFileSync(join(cwd,'cn-tables.mjs'),'utf8');fs.unlinkSync(join(cwd,'cn-tables.mjs'));const wrapped=session.with_cn(async(phase,ctx)=>{assert.equal(phase,'phase-production-build');assert.equal(ctx,context);assert.ok(fs.existsSync(join(cwd,'cn-tables.mjs')));return value},{cwd});assert.equal(await wrapped('phase-production-build',context),value);assert.equal(fs.readFileSync(join(cwd,'cn-tables.mjs'),'utf8'),referenceSource);const sentinel={next:true};const bad=session.with_cn(()=>{throw sentinel},{cwd});await assert.rejects(bad('phase-production-build',context),e=>e===sentinel)
  })
  await check('actual-Next-recursive-nonpersistent-watch-and-disposal',async()=>{
   const cwd=project('next-watch',{'src/index.ts':'p-2'});const real=nodeHost(),watchers=[];const observed={...real,watch:(dir,callback)=>{const watcher=real.watch(dir,callback);watchers.push(watcher);return watcher}};const bridge=await createCnBuild(limits,{module:pathToFileURL(join(artifact,'style.js')),binary:join(artifact,'style_bg.wasm'),host:observed});
   try{const wrapped=bridge.session.with_cn({ok:true},{cwd});assert.deepEqual(await wrapped('phase-development-server',{}),{ok:true});assert.equal(watchers.length,1);const file=join(cwd,'cn-tables.mjs'),before=fs.readFileSync(file,'utf8');fs.writeFileSync(join(cwd,'src/index.ts'),'p-2 m-4');for(let i=0;i<100&&fs.readFileSync(file,'utf8')===before;i++)await new Promise(r=>setTimeout(r,20));assert.notEqual(fs.readFileSync(file,'utf8'),before);const closed=new Promise(r=>watchers[0].once('close',r));bridge.close();await closed;const after=fs.readFileSync(file,'utf8');fs.writeFileSync(join(cwd,'src/index.ts'),'p-2 m-4 text-xl');await new Promise(r=>setTimeout(r,80));assert.equal(fs.readFileSync(file,'utf8'),after)}finally{real.close()}
  })
  await check('source-ports-expansion-and-matcher',()=>{
   const cwd=project('ports',{'src/a.ts':'p-2','src/b.tsx':'m-4','.hidden/c.ts':'text-xl'});const expected=oracle.expandGlobs(['src/**/*.{ts,tsx}'],cwd,{skippedUnreadable:0});assert.deepEqual(session.expand_globs(['src/**/*.{ts,tsx}'],cwd).files,expected);const matcher=session.create_content_matcher(['src/**/*.{ts,tsx}'],cwd);const ref=oracle.createContentMatcher(['src/**/*.{ts,tsx}'],cwd);for(const path of ['src/a.ts','src/b.tsx','.hidden/c.ts','src/a.css'])assert.equal(matcher.matches(resolve(cwd,path)),ref(resolve(cwd,path)));matcher.free()
  })
  await check('explicit-source-limit-rejects-instead-of-partial-build',async()=>{
   const cwd=project('limit',{'index.ts':'p-2'});const small=new wasm.CnBuildSession(host,{...limits,maxNodes:2});await assert.rejects(small.build({cwd}),e=>e.name==='RangeError');small.free()
  })

  await check('actual-Next-active-watch-exits-nonpersistent',()=>{
   const cwd=project('watch-exit',{'index.ts':'p-2'});
   const child=spawnSync(process.execPath,['--input-type=module','-e',`import {createCnBuild} from ${JSON.stringify(pathToFileURL(join(artifact,'cn-node.mjs')).href)};const bridge=await createCnBuild(${JSON.stringify(limits)},{module:new URL(${JSON.stringify(pathToFileURL(join(artifact,'style.js')).href)}),binary:${JSON.stringify(join(artifact,'style_bg.wasm'))}});globalThis.bridge=bridge;const factory=bridge.session.with_cn({}, {cwd:${JSON.stringify(cwd)}});await factory('phase-development-server',{});console.log('ready')`],{encoding:'utf8',timeout:10000});assert.equal(child.status,0,child.stderr);assert.equal(child.stdout,'ready\n')
  })
  await check('actual-CLI-all-flags-and-original-bin-output',async()=>{
   const cwd=project('cli',{'index.ts':'p-2 m-4','safe.txt':'text-lg','tokens.txt':'p-2 m-4','theme.css':'@theme { --radius-soft:1rem; }','config.mjs':'export default {extend:{classGroups:{shape:["shape-a"]}}}'})
   const bundle=artifact
   const argsCases=[[],['--help'],['--version'],['unknown'],['build','--out'],['build','--wat'],['build','--cwd',cwd,'--no-css'],['build','--cwd',cwd,'--content','**/*.{ts,tsx},safe.txt','--css','theme.css','--safelist','safe.txt','--config','config.mjs','-o','tables.ts'],['build','--cwd',cwd,'--tokens','tokens.txt','--safelist','safe.txt','--no-css','--quiet'],['build','--cwd',cwd,'--full','--no-css']]
   for(const args of argsCases){
    const original=spawnSync(process.execPath,['--import',resolve('tests/utility/cn/cli-oracle-hook.mjs'),resolve('ref/cn/packages/cn/bin/cn.mjs'),...args],{encoding:'utf8',timeout:10000});
    const actual=spawnSync(process.execPath,[join(bundle,'cn-cli.mjs'),...args],{encoding:'utf8',timeout:10000,env:{...process.env,RUSTIFY_CN_LIMITS:JSON.stringify(limits)}})
    assert.deepEqual({status:actual.status,stdout:actual.stdout,stderr:actual.stderr},{status:original.status,stdout:original.stdout,stderr:original.stderr},args.join(' '))
   }
  })

  await check('pure-source-default-options-and-UTF16-extraction',()=>{
   const base=config.defaultConfig();assert.equal(wasm.cn_compile_to_source(base,undefined,limits),compiler.compileToSource(base));
   const content='\ud800:shape-\udc00 p-2:: 😀p-4 \ud800 '+ 'x'.repeat(8193);const a=new Set(['existing']),b=new Set(['existing']);const expected=oracle.extractTokens(content,b);assert.equal(wasm.cn_extract_tokens(content,a,limits),expected);assert.deepEqual([...a],[...b]);
  })

  await check('CSS-helper-optional-extension-sections',()=>{
   const cwd=project('css-helper',{'empty.css':'body{}','extend.css':'@theme { --radius-soft:1rem; }','reset.css':'@theme { --radius-*:initial; }'});
   for(const file of ['empty.css','extend.css','reset.css'])assert.deepEqual(session.theme_from_css([join(cwd,file)]),themeCss.themeFromCss([join(cwd,file)]));
   for(const css of ['@theme {}','@themefoo {}','@import "tailwindcss";','@import url("tailwindcss/theme");','/* @theme */'])assert.equal(wasm.cn_is_tailwind_css(css),themeCss.isTailwindCss(css))
  })

  await check('real-fs-mini-glob-UTF16-question-and-final-newline',()=>{
   const cwd=project('glob-units',{'😀.ts':'p-2','a.ts\n':'m-4','b.ts':'text-xl'});
   for(const patterns of [['?.ts'],['??.ts'],['**/*.ts']]){
    assert.deepEqual(session.expand_globs(patterns,cwd).files,oracle.expandGlobs(patterns,cwd,{skippedUnreadable:0}));
    const matcher=session.create_content_matcher(patterns,cwd),ref=oracle.createContentMatcher(patterns,cwd);for(const path of ['😀.ts','a.ts\n','b.ts'])assert.equal(matcher.matches(join(cwd,path)),ref(join(cwd,path)));matcher.free()
   }
  })

  await check('production-CLI-default-policy-without-env',()=>{
   const env={...process.env};delete env.RUSTIFY_CN_LIMITS;
   const child=spawnSync(process.execPath,[join(artifact,'cn-cli.mjs'),'--version'],{encoding:'utf8',timeout:10000,env});assert.equal(child.status,0,child.stderr);assert.equal(child.stdout,'0.4.0\n')
  })
  await check('candidate-Set-add-duplicates-and-original-throw',()=>{
   const callsA=[],callsB=[];class Observed extends Set{constructor(calls){super();this.calls=calls}add(v){this.calls.push(v);return super.add(v)}}
   const a=new Observed(callsA),b=new Observed(callsB);assert.equal(wasm.cn_extract_tokens('p-2 p-2 p-4:',a,limits),oracle.extractTokens('p-2 p-2 p-4:',b));assert.deepEqual(callsA,callsB);const sentinel={add:true};assert.throws(()=>wasm.cn_extract_tokens('p-2',{add(){throw sentinel}},limits),e=>e===sentinel)
  })

  await check('CLI-summary-binary-half-rounding',()=>{
   assert.equal(session.cli_summary({source:'x'.repeat(1280)},{full:true,out:'tables.mjs'}),'cn build: all class groups kept (--full)\n  → tables.mjs (1.3 KB source, 0.0 KB gzip)\n')
  })

  await check('CLI-gzip-boundary-original-Node-zlib',async()=>{
   const fixture=JSON.parse(fs.readFileSync(new URL('./gzip-boundary.json',import.meta.url),'utf8'));const expected=compiler.compileToSource(fixture.config);const source=wasm.cn_compile_to_source(fixture.config,undefined,limits);assert.equal(source,expected);
   const gz=gzipSync(expected,{level:9}).length;assert.equal((gz/1024).toFixed(1),fixture.expectedKB);assert.equal(session.cli_summary({source},{full:true,out:'x.mjs'}),`cn build: all class groups kept (--full)\n  → x.mjs (${(source.length/1024).toFixed(1)} KB source, ${(gz/1024).toFixed(1)} KB gzip)\n`);
   const cwd=project('gzip-real-cli',{'config.mjs':`export default ()=>(${JSON.stringify(fixture.config)})`});const options={cwd,config:'config.mjs',full:true,css:false,out:'tables.mjs'};const report=await session.build(options);const summary=session.cli_summary(report,options);assert.ok(summary.includes(`${(gzipSync(report.source,{level:9}).length/1024).toFixed(1)} KB gzip`));
  })

  await check('Next-default-config-object-identity',async()=>{
   const cwd=project('next-default',{'index.ts':'p-2'});const opts={cwd,full:true,css:false};const original=referenceNext.withCn(undefined,opts);assert.equal(await original('phase-production-build',{}),await original('phase-production-build',{}));const factory=session.with_cn(undefined,opts);assert.equal(await factory('phase-production-build',{}),await factory('phase-production-build',{}))
  })

  await check('WASM-virtual-primitive-tree-shared-build-and-incremental',async()=>{
   const cwd=project('virtual-oracle',{'index.ts':'p-2 p-4','nested/a.html':'<div class="m-2">','.hidden/a.ts':'flex'});const expected=await oracle.build({cwd,css:false});
   const files=new Map([['/project/index.ts','p-2 p-4'],['/project/nested/a.html','<div class="m-2">'],['/project/.hidden/a.ts','flex']]);const dirs={'/project':[['.hidden',1],['index.ts',0],['nested',1]],'/project/.hidden':[['a.ts',0]],'/project/nested':[['a.html',0]]};
   const missing=p=>{throw new Error(`ENOENT: fixture '${p}'`)};
   const vhost={cwd:()=>'/project',resolve:(a,b)=>posix.resolve(a,b),relative:(a,b)=>posix.relative(a,b),dirname:p=>posix.dirname(p),readText:p=>files.has(p)?files.get(p):missing(p),writeText:(p,s)=>files.set(p,s),realpath:p=>files.has(p)||dirs[p]?p:missing(p),kind:p=>dirs[p]?1:files.has(p)?0:missing(p),children:p=>dirs[p]??missing(p),importModule:()=>Promise.reject(new Error('no virtual modules')),gzipSize:s=>gzipSync(s,{level:9}).length};
   const memory=new wasm.CnBuildSession(vhost,limits);try{const actual=await memory.build({cwd:'/project',css:false});assert.deepEqual(comparable(actual),comparable(expected));const builder=memory.create_builder({cwd:'/project',css:false});const first=await builder.run();files.set('/project/index.ts','p-6');assert.equal(await builder.changed('index.ts'),first);files.set('/project/index.ts','p-6 flex');assert.notEqual(await builder.changed('index.ts'),first);builder.free()}finally{memory.free()}
  })

  await check('config-import-primitive-throw-message-and-cause',async()=>{
   const cwd=project('import-primitive',{'index.ts':'p-2','config.mjs':'throw Symbol.for("rustify-import-primitive")'});let expected;try{await oracle.build({cwd,config:'config.mjs'})}catch(e){expected=e};await assert.rejects(session.build({cwd,config:'config.mjs'}),e=>e.message===expected.message&&e.cause===expected.cause)
  })
  await check('config-import-message-getter-original-throw',async()=>{
   const sentinel=globalThis.__rustifyMessageSentinel={messageGetter:true};globalThis.__rustifyMessageCause={get message(){throw sentinel}};const cwd=project('import-message-getter',{'index.ts':'p-2','config.mjs':'throw globalThis.__rustifyMessageCause'});await assert.rejects(oracle.build({cwd,config:'config.mjs'}),e=>e===sentinel);await assert.rejects(session.build({cwd,config:'config.mjs'}),e=>e===sentinel);delete globalThis.__rustifyMessageCause;delete globalThis.__rustifyMessageSentinel
  })

  await optionsCases({check,session,wasm,oracle,referenceVite,referenceNext,host,limits,project})
  await check('actual-CLI-transform-Symbol-throw-format',async()=>{
   const cwd=project('cli-symbol',{'index.ts':'p-2','config.mjs':'export default ()=>{throw Symbol.for("rustify-cli-symbol")}'});const args=['build','--cwd',cwd,'--config','config.mjs'];const original=spawnSync(process.execPath,['--import',resolve('tests/utility/cn/cli-oracle-hook.mjs'),resolve('ref/cn/packages/cn/bin/cn.mjs'),...args],{encoding:'utf8',timeout:10000});const actual=await session.run_cli(args);assert.deepEqual({stdout:actual.stdout,stderr:actual.stderr,status:actual.exitCode},{stdout:original.stdout,stderr:original.stderr,status:original.status})
  })
  session.free()
 }
}finally{fs.rmSync(temp,{recursive:true,force:true})}
if(process.env.RUSTIFY_CN_OPTIONS_EVIDENCE)fs.writeFileSync(process.env.RUSTIFY_CN_OPTIONS_EVIDENCE,JSON.stringify({schemaVersion:1,productionBundle:artifact,reads:optionsReadEvidence},null,2)+'\n')
console.log(JSON.stringify({passed,failures},null,2));if(failures.length)process.exitCode=1
