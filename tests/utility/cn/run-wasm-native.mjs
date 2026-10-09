// JavaScript only reconstructs fixtures, creates callbacks, and initializes WASM.
// Every join, enumeration, classification, conflict, and configuration compile
// under test executes in the Rust facade/core.
import assert from 'node:assert/strict'
import { readFileSync, writeFileSync } from 'node:fs'
import { resolve } from 'node:path'
import { pathToFileURL } from 'node:url'
import { decode, load, assertLockedSource } from './upstream.mjs'
import { runFixedCases } from './runtime-check.mjs'
assertLockedSource()
const root=resolve('target/utility-conformance/cn')
const wasm=await import(pathToFileURL(resolve(root,'style.js')))
const bytes=readFileSync(resolve(root,'style_bg.wasm'))
const exports=await wasm.default({module_or_path:bytes})
assert.equal(exports.memory.buffer instanceof SharedArrayBuffer,false)
const module=new WebAssembly.Module(bytes)
assert.ok(WebAssembly.Module.imports(module).every(entry=>!/(?:fs|rayon|thread|wasi)/i.test(entry.name)),'no filesystem/thread imports')
const limits={maxDepth:128,maxNodes:100000,maxUtf16Units:1000000,maxTokens:100000,cacheBytes:10000000}
const fixtures=JSON.parse(readFileSync(new URL('./runtime-fixtures.json',import.meta.url),'utf8')).cases
const {passed,failures}=runFixedCases(wasm,fixtures,limits)
console.log(JSON.stringify({fixedCases:fixtures.length,passed,failures},null,2))
if(failures.length)process.exitCode=1
else{
  const upstream={config:await load('config'),engine:await load('engine'),index:await load('index'),compiler:await load('compiler'),validators:await load('validators')}
  let extra=0
  const check=(name,fn)=>{try{fn();extra++}catch(error){failures.push({id:name,error:String(error)})}}
  const fresh=(config=undefined,l=limits)=>new wasm.CnSession(config,l)
  check('native-shapes/prototype-enumeration',()=>{
    const prototype={inherited:1,'2':true,shadowed:true};const value=Object.create(prototype)
    value['3']=1;value['1']=1;value.z=1;Object.defineProperty(value,'shadowed',{value:true,enumerable:false});value[Symbol('hidden')]=true
    const s=fresh();assert.equal(s.join([value]),upstream.index.clsx(value));s.free()
  })
  check('native-shapes/getter-exception-identity',()=>{
    const sentinel={kind:'getter'};const value={get length(){throw sentinel}};const s=fresh()
    assert.throws(()=>s.join([value]),e=>e===sentinel);assert.throws(()=>s.tw_join([value]),e=>e===sentinel);s.free()
  })
  check('native-shapes/proxy-exception-identity',()=>{
    const sentinel=Symbol('proxy');const value=new Proxy({},{ownKeys(){throw sentinel}});const s=fresh();assert.throws(()=>s.join([value]),e=>e===sentinel);s.free()
  })
  check('native-shapes/sparse-mutated-array',()=>{
    const a=['p-2',,'p-4'];const s=fresh();assert.equal(s.merge([a]),upstream.index.cn(a));a[2]='p-6';assert.equal(s.merge([a]),upstream.index.cn(a));s.free()
  })
  check('native-shapes/array-like-fractional-length',()=>{const value={length:1.5,0:'p-2',1:'p-4'};const s=fresh();assert.equal(s.tw_join([value]),upstream.index.twJoin(value));s.free()})
  check('native-shapes/bigint-nonfinite-symbol-function',()=>{const values=[0n,12n,-12n,NaN,Infinity,-Infinity,-0,Symbol('ignored'),()=>{},new Number(5)];const s=fresh();assert.equal(s.join(values),upstream.index.clsx(...values));assert.equal(s.tw_join(values),upstream.index.twJoin(...values));s.free()})
  check('utf16/lone-surrogates-and-sort',()=>{
    const values=['\ud800 p-2','\udc00 p-4','\ud800 custom','\udc00 custom','😀 p-2','\ud800:hover:p-2 hover:\ud800:p-4','[\ud800:1] [\udc00:2] [\ud800:3]']
    const s=fresh();for(const value of values)assert.equal(s.merge([value]),upstream.index.cn(value));assert.equal(s.join(['\ud800','\udc00']),'\ud800 \udc00');s.free()
  })
  check('utf16/literals-prefix-groups-order-sensitive',()=>{
    const config={prefix:'\ud800',extend:{classGroups:{['\udc00']:['shape-\ud800','shape-\udc00'],other:['o-a','o-b']},conflictingClassGroups:{other:['\udc00']},orderSensitiveModifiers:['\ud800']}}
    const reference=upstream.config.createCn(config);const s=fresh(config)
    for(const input of ['\ud800:shape-\ud800 \ud800:shape-\udc00','\ud800:shape-\ud800 \ud800:o-a','\ud800:\ud800:hover:p-2 \ud800:hover:\ud800:p-4'])assert.equal(s.merge([input]),reference(input));s.free()
  })
  check('callbacks/memo-calls-and-lone-surrogates',()=>{
    const callsA=[],callsB=[];const config=callback=>({cacheSize:0,extend:{classGroups:{probe:[{probe:[callback]}]}}})
    const s=fresh(config(v=>{callsA.push(v);return true}));const ref=upstream.config.createCn(config(v=>{callsB.push(v);return true}))
    for(const input of ['probe-a probe-a','probe-a probe-b','probe-\ud800 probe-\ud800','probe-\udc00 probe-a'])assert.equal(s.merge([input]),ref(input))
    assert.deepEqual(callsA,callsB);assert.ok(callsA.includes('\ud800'));s.free()
  })
  check('callbacks/exceptions-original-and-retry',()=>{
    const sentinel={kind:'validator'};let count=0;const s=fresh({extend:{classGroups:{probe:[{probe:[()=>{count++;throw sentinel}]}]}}})
    assert.throws(()=>s.merge(['probe-a']),e=>e===sentinel);assert.throws(()=>s.merge(['probe-a']),e=>e===sentinel);assert.equal(count,2);s.free()
  })
  check('callbacks/validator-reentry',()=>{
    let s;let ref;let a=0,b=0;const conf=f=>({extend:{classGroups:{probe:[{probe:[f]}]}}})
    s=fresh(conf(()=>{a++;assert.equal(s.merge(['p-2 p-4']),'p-4');return true}))
    ref=upstream.config.createCn(conf(()=>{b++;assert.equal(ref('p-2 p-4'),'p-4');return true}))
    assert.equal(s.merge(['probe-a probe-b']),ref('probe-a probe-b'));assert.equal(a,b);s.free()
  })
  check('callbacks/function-identity-dedupe',()=>{
    let a=0,b=0;const fnA=()=>{a++;return false},fnB=()=>{b++;return false};const conf=f=>({extend:{classGroups:{one:[{probe:[f]}],two:[{probe:[f]}]}}})
    const s=fresh(conf(fnA));const ref=upstream.config.createCn(conf(fnB));assert.equal(s.merge(['probe-a probe-a']),ref('probe-a probe-a'));assert.equal(a,b);s.free()
  })
  check('callbacks/lazy-transform-empty',()=>{let called=0;const s=fresh(config=>{called++;return config});assert.equal(called,0);assert.equal(s.merge([]),'');assert.equal(called,0);assert.equal(s.merge(['p-2 p-4']),'p-4');assert.equal(called,1);s.free()})
  check('callbacks/lazy-transform-error-identity',()=>{const sentinel=Symbol('config');const s=fresh(()=>{throw sentinel});assert.throws(()=>s.merge(['x']),e=>e===sentinel);assert.throws(()=>s.merge(['x']),e=>e===sentinel);s.free()})
  check('callbacks/theme-getter-error-identity',()=>{const sentinel=12n;const getter=()=>{throw sentinel};getter.isThemeGetter=true;const s=fresh({extend:{classGroups:{probe:[{probe:[getter]}]}}});assert.throws(()=>s.merge(['probe-a']),e=>e===sentinel);s.free()})
  check('limits/cycle-and-repeated-reference',()=>{const s=fresh();const cycle=[];cycle.push(cycle);assert.throws(()=>s.merge([cycle]),e=>e.name==='CyclicInput');const repeated=['x'];assert.equal(s.join([repeated,repeated]),'x x');s.free()})
  check('limits/depth-nodes-units-tokens-arraylike',()=>{
    const s=fresh(undefined,{maxDepth:2,maxNodes:8,maxUtf16Units:16,maxTokens:2,cacheBytes:16});
    for(const value of [[[[['x']]]],['a b c'],['x'.repeat(17)],[{length:Infinity}]])assert.throws(()=>s.merge(value),e=>e.name==='InputLimit')
    assert.throws(()=>s.tw_join([{length:Infinity}]),e=>e.name==='InputLimit');s.free()
  })
  check('limits/missing-and-zero',()=>{assert.throws(()=>fresh(undefined,{}),e=>e.name==='InvalidLimits');assert.throws(()=>fresh(undefined,{...limits,maxDepth:0}),e=>e.name==='InvalidLimits')})
  check('helpers/markers-and-predicates',()=>{assert.deepEqual(wasm.cn_from_theme('\ud800'),{$t:'\ud800'});for(const name of ['isAny','isNumber','isArbitraryValue']){assert.deepEqual(wasm.cn_validator_marker(name),upstream.config.validators[name]);for(const input of ['1','NaN','[\ud800]'])assert.equal(wasm.cn_validator(name,input),upstream.validators[name](input))}})
  check('enumeration/getter-mutation-and-prototype',()=>{
    function values(){const proto={inherited:1};const value=Object.create(proto);Object.defineProperty(value,'a',{enumerable:true,get(){delete value.b;proto.added=1;return true}});value.b=1;return value}
    const s=fresh();assert.equal(s.join([values()]),upstream.index.clsx(values()));s.free()
  })
  check('enumeration/array-like-dynamic-length',()=>{
    const value=()=>{let n=0;return {get length(){return ++n<3?2:1},0:'a',1:'b'}}
    const s=fresh();assert.equal(s.tw_join([value()]),upstream.index.twJoin(value()));s.free()
  })
  check('config/inherited-extension-and-undefined',()=>{
    const groups=Object.create({probe:[{probe:['a','b']}]});groups.display=undefined
    const config={override:{classGroups:groups},extend:{classGroups:Object.create({other:['o-a','o-b']})}}
    const s=fresh(config),ref=upstream.config.createCn(config);for(const input of ['probe-a probe-b','o-a o-b','block flex'])assert.equal(s.merge([input]),ref(input));s.free()
  })
  check('config/theme-getter-original-theme-identity',()=>{
    const config=wasm.cn_default_config();const theme=config.theme;let calls=0
    const getter=t=>{assert.equal(t,theme);calls++;return t.text};getter.isThemeGetter=true;config.classGroups.probe=[{probe:[getter]}]
    const s=fresh(config);assert.equal(s.merge(['probe-sm probe-lg']),'probe-lg');assert.equal(calls,1);s.free()
  })
  check('config/theme-getter-strict-boolean-marker',()=>{
    let calls=0;const callback=value=>{assert.equal(typeof value,'string');calls++;return true};callback.isThemeGetter=1
    const config={extend:{classGroups:{probe:[{probe:[callback]}]}}};const s=fresh(config);assert.equal(s.merge(['probe-a probe-b']),'probe-b');assert.equal(calls,2);s.free()
  })
  check('config/transform-in-place-and-callback-handle',()=>{
    let count=0;const sentinel=[];const callback=value=>{count++;sentinel.push(value);return true};const s=fresh(base=>{base.classGroups.probe=[{probe:[callback]}];return base})
    assert.equal(s.merge(['probe-\ud800 probe-\udc00']),'probe-\udc00');assert.deepEqual(sentinel,['\ud800','\udc00']);assert.equal(count,2);s.free()
  })
  check('limits/config-cycle-and-reentrant-configuration',()=>{
    const def={probe:[]};def.probe.push(def);const cyclic=fresh({extend:{classGroups:{probe:[def]}}});assert.throws(()=>cyclic.merge(['probe-a']),e=>e.name==='CyclicInput');cyclic.free()
    let s;s=fresh(()=>{s.merge(['x']);return wasm.cn_default_config()});assert.throws(()=>s.merge(['x']),e=>e.name==='ReentrantConfiguration');s.free()
  })
  check('limits/recursive-validator-is-bounded',()=>{
    let s;s=fresh({extend:{classGroups:{probe:[{probe:[()=>s.merge(['probe-a'])]}]}}},{...limits,maxDepth:8});assert.throws(()=>s.merge(['probe-a']),e=>e.name==='InputLimit');s.free()
  })
  check('limits/default-tables-do-not-consume-input-budget',()=>{const s=fresh(undefined,{maxDepth:4,maxNodes:8,maxUtf16Units:16,maxTokens:2,cacheBytes:16});assert.equal(s.merge(['p-2 p-4']),'p-4');s.free()})
  check('helpers/default-config-and-merge-configs',()=>{
    const native=upstream.config.defaultConfig();const actual=wasm.cn_default_config();assert.deepEqual(actual,native)
    const callback=()=>true;const extension={extend:{classGroups:{probe:[{probe:[callback]}]}}}
    const merged=wasm.cn_merge_configs(actual,extension,limits);assert.equal(merged.classGroups.probe[0].probe[0],callback);assert.deepEqual(merged,upstream.config.mergeConfigs(native,extension));
  })
  check('tables/compile-and-native-function-handles',()=>{
    const compiler=upstream.compiler;const config=wasm.cn_default_config();const callback=()=>true;config.classGroups.probe=[{probe:[callback]}]
    const actual=wasm.cn_compile_to_tables(config,limits),expected=compiler.compileToTables(config)
    assert.deepEqual(actual,expected);assert.equal(Object.values(actual.validatorImpls)[0],callback)
    const s=wasm.CnSession.fromTables(actual.tables,actual.validatorImpls,{cacheSize:0},limits);assert.equal(s.merge(['probe-a probe-b']),'probe-b');s.free()
  })
  check('tables/lossless-custom-literals-and-prefix',()=>{
    const config=wasm.cn_default_config();config.classGroups['\ud800']=['shape-\ud800','shape-\udc00'];config.prefix='\udc00'
    const actual=wasm.cn_compile_to_tables(config,limits),expected=upstream.compiler.compileToTables(config);assert.deepEqual(actual,expected)
    const s=wasm.CnSession.fromTables(actual.tables,actual.validatorImpls,{prefix:actual.prefix},limits);const ref=upstream.config.createCn(config)
    assert.equal(s.merge(['\udc00:shape-\ud800 \udc00:shape-\udc00']),ref('\udc00:shape-\ud800 \udc00:shape-\udc00'));s.free()
  })
  check('tables/invalid-input-is-diagnostic',()=>{
    const actual=wasm.cn_compile_to_tables(wasm.cn_default_config(),limits);const bad={...actual.tables,edgeTarget:new Int32Array([2147483647])};assert.throws(()=>wasm.CnSession.fromTables(bad,{},undefined,limits),e=>e instanceof Error)
    const config=wasm.cn_default_config();config.classGroups.probe=[{probe:[()=>true]}];const custom=wasm.cn_compile_to_tables(config,limits);assert.throws(()=>wasm.CnSession.fromTables(custom.tables,{},undefined,limits),e=>e.message.startsWith('cn: missing validator'))
  })
  check('callbacks/token-memo-collision-and-admission',()=>{
    let a=0,b=0;const conf=f=>({cacheSize:0,extend:{classGroups:{probe:[{probe:[f]}]}}})
    const s=fresh(conf(()=>{a++;return true})),ref=upstream.config.createCn(conf(()=>{b++;return true}))
    const tokens=Array.from({length:12000},(_,i)=>'probe-'+i)
    for(const input of [tokens.join(' '),tokens.slice(0,4000).join(' '),tokens.slice(8000).join(' ')])assert.equal(s.merge([input]),ref(input))
    assert.equal(a,b);s.free()
  })
  check('callbacks/reentry-varying-inner-shapes',()=>{
    for(const inner of ['p-2','p-2 p-4',' p-2  p-4 ','m-2 p-4 text-lg','custom custom','hover:p-2 p-4']){
      let s,ref;const config=callback=>({cacheSize:0,extend:{classGroups:{probe:[{probe:[callback]}]}}})
      s=fresh(config(()=>{s.merge([inner]);return true}));ref=upstream.config.createCn(config(()=>{ref(inner);return true}))
      for(const input of ['probe-a probe-b',' probe-c probe-d ','hover:probe-e hover:probe-f','probe-a'])assert.equal(s.merge([input]),ref(input),inner+': '+input)
      s.free()
    }
  })
  check('engine/seen-before-and-byte-budget',()=>{
    const compiler=upstream.compiler,engine=upstream.engine.createEngine(compiler.compileToTables(upstream.config.defaultConfig()).tables)
    const s=fresh();for(const input of ['p-2 p-4','p-2 p-4','m-2 m-4','\ud800 p-2','\ud800 p-2']){assert.equal(s.seen_before(input),engine.seenBefore(input));assert.equal(s.merge_string(input),engine.mergeString(input))}s.free()
    const small=fresh(undefined,{...limits,cacheBytes:16});for(let i=0;i<20;i++){small.merge(['p-'+i]);small.merge(['p-'+i])}assert.ok(small.cache_usage().stringBytes<=8);assert.ok(small.cache_usage().memoStringBytes<=8);assert.ok(small.cache_usage().totalStringBytes<=16);small.free()
  })
  check('limits/compiled-path-and-theme-expansion-depth',()=>{
    const config={extend:{classGroups:{probe:['x-'.repeat(64)+'a']}}};const s=fresh(config,{...limits,maxDepth:8});assert.throws(()=>s.merge(['p-2 p-4']),e=>e.name==='InputLimit');s.free()
    const full={theme:{},classGroups:{probe:[{$t:'0'}]},conflictingClassGroups:{},conflictingClassGroupModifiers:{},orderSensitiveModifiers:[]}
    for(let i=0;i<64;i++)full.theme[String(i)]=i===63?['probe-a']:[{$t:String(i+1)}]
    const chain=fresh(full,{...limits,maxDepth:8});assert.throws(()=>chain.merge(['probe-a']),e=>e.name==='InputLimit');chain.free()
  })
  check('limits/cyclic-theme-getter-is-diagnostic',()=>{
    const getter=()=>[getter];getter.isThemeGetter=true;const s=fresh({extend:{classGroups:{probe:[{probe:[getter]}]}}});assert.throws(()=>s.merge(['probe-a']),e=>e.name==='CyclicInput');s.free()
  })
  check('config/non-string-marker-keys-are-literal-objects',()=>{
    const config={extend:{classGroups:{probe:[{$v:['a','b']},{$t:['a','b']}]}}};const s=fresh(config),ref=upstream.config.createCn(config);for(const input of ['$v-a $v-b','$t-a $t-b'])assert.equal(s.merge([input]),ref(input));s.free()
  })
  check('config/marker-getter-reads-preserve-native-values',()=>{
    function config(){let reads=0;const marker={get $v(){return ++reads===1?'isNumber':'isAny'}};return {read:()=>reads,input:{extend:{classGroups:{probe:[{probe:[marker]}]}}}}}
    const a=config(),b=config();const s=fresh(a.input),ref=upstream.config.createCn(b.input);assert.equal(s.merge(['probe-a probe-b']),ref('probe-a probe-b'));assert.equal(a.read(),b.read());s.free()
  })
  check('limits/theme-getter-results-share-total-input-budget',()=>{
    const getter=()=>['x'.repeat(12000)];getter.isThemeGetter=true;const config={extend:{classGroups:{probe:[getter,getter,getter]}}};const s=fresh(config,{...limits,maxUtf16Units:50000});assert.throws(()=>s.merge(['probe-a']),e=>e.name==='InputLimit');s.free()
  })
  check('callbacks/context-reset-clears-token-memo',()=>{
    let a=0,b=0;const conf=f=>({cacheSize:0,extend:{classGroups:{probe:[{probe:[f]}]}}});const s=fresh(conf(()=>{a++;return true})),ref=upstream.config.createCn(conf(()=>{b++;return true}))
    for(const input of ['probe-a',Array.from({length:4100},(_,i)=>'v'+i+':p-2').join(' '),'probe-a'])assert.equal(s.merge([input]),ref(input))
    assert.equal(a,b);s.free()
  })
  check('callbacks/property-reset-clears-token-memo',()=>{
    let a=0,b=0;const conf=f=>({cacheSize:0,extend:{classGroups:{probe:[{probe:[f]}]}}});const s=fresh(conf(()=>{a++;return true})),ref=upstream.config.createCn(conf(()=>{b++;return true}))
    for(const input of ['probe-a',Array.from({length:4100},(_,i)=>'[--v'+i+':1]').join(' '),'probe-a'])assert.equal(s.merge([input]),ref(input))
    assert.equal(a,b);s.free()
  })
  check('cache/two-generation-callback-state-after-reset',()=>{
    let a=true,b=true;const conf=f=>({cacheSize:1,extend:{classGroups:{probe:[{probe:[f]}]}}});const s=fresh(conf(()=>a)),ref=upstream.config.createTwMerge(conf(()=>b))
    const churn=Array.from({length:4100},(_,i)=>'v'+i+':p-2').join(' ')
    for(const input of ['probe-a probe-b','probe-a probe-b','p-2 p-4','p-2 p-4',churn])assert.equal(s.tw_merge([input]),ref(input))
    a=false;b=false;assert.equal(s.tw_merge(['probe-a probe-b']),ref('probe-a probe-b'));s.free()
  })
  check('cache/argument-tuple-survives-result-generations',()=>{
    let a=true,b=true;const conf=f=>({cacheSize:1,extend:{classGroups:{probe:[{probe:[f]}]}}});const s=fresh(conf(()=>a)),ref=upstream.config.createCn(conf(()=>b))
    for(let i=0;i<2;i++)assert.equal(s.merge(['probe-a','probe-b']),ref('probe-a','probe-b'))
    const churn=Array.from({length:4100},(_,i)=>'v'+i+':p-2').join(' ')
    for(const input of ['p-2 p-4','p-2 p-4','m-2 m-4','m-2 m-4','gap-2 gap-4','gap-2 gap-4',churn])assert.equal(s.merge([input]),ref(input))
    a=false;b=false;assert.equal(s.merge(['probe-a','probe-b']),ref('probe-a','probe-b'));s.free()
  })
  check('config/theme-getter-mutates-later-array-and-fields',()=>{
    function input(){const c=upstream.config.defaultConfig();const later=['probe-a'];const getter=t=>{later.push('probe-b');c.prefix='x';c.orderSensitiveModifiers.push('hold');c.conflictingClassGroups.late=['display'];return []};getter.isThemeGetter=true;c.classGroups.early=[getter];c.classGroups.late=[{probe:later}];return c}
    const s=fresh(input()),ref=upstream.config.createCn(input());for(const input of ['x:probe-a x:probe-b','x:block x:probe-a','x:hold:hover:p-2 x:hover:hold:p-4'])assert.equal(s.merge([input]),ref(input));s.free()
  })
  check('config/theme-getter-mutates-theme-marker-later',()=>{
    function input(){const c=upstream.config.defaultConfig();const getter=t=>{t.custom=['a','b'];return []};getter.isThemeGetter=true;c.classGroups.early=[getter];c.classGroups.late=[{probe:[{$t:'custom'}]}];return c}
    const s=fresh(input()),ref=upstream.config.createCn(input());assert.equal(s.merge(['probe-a probe-b']),ref('probe-a probe-b'));s.free()
  })
  check('config/property-read-order-is-observable',()=>{
    function input(log){const c=upstream.config.defaultConfig();const out={};for(const key of Object.keys(c))Object.defineProperty(out,key,{enumerable:true,get(){log.push(key);return c[key]}});return out}
    const a=[],b=[];const s=fresh(input(a)),ref=upstream.config.createCn(input(b));assert.equal(s.merge(['p-2 p-4']),ref('p-2 p-4'));assert.deepEqual(a,b);s.free()
  })
  check('cache/mutable-object-getters-stay-live',()=>{
    function value(log){let active='probe-a';const v={};for(const k of ['probe-a','probe-b'])Object.defineProperty(v,k,{enumerable:true,get(){log.push(k);return active===k}});return {v,change:()=>active='probe-b'}}
    const a=[],b=[],va=value(a),vb=value(b);const s=fresh(),ref=upstream.index.cn;
    for(let i=0;i<5;i++)assert.equal(s.merge([va.v,'p-2']),ref(vb.v,'p-2'));va.change();vb.change();assert.equal(s.merge([va.v,'p-2']),ref(vb.v,'p-2'));assert.deepEqual(a,b);s.free()
  })
  check('cache/tuple-bucket-churn-and-generations',()=>{
    function execute(merge){const out=[];for(let pass=0;pass<3;pass++)for(let i=0;i<400;i++)out.push(merge('probe-base','probe-'+i));for(let i=0;i<1100;i++){out.push(merge('base'+i,'p-2 p-4'));out.push(merge('base'+i,'p-2 p-4'))}for(const i of [0,40,399,0])out.push(merge('probe-base','probe-'+i));return out}
    let a=0,b=0;const config=f=>({cacheSize:2,extend:{classGroups:{probe:[{probe:[f]}]}}});const s=fresh(config(()=>{a++;return true})),ref=upstream.config.createCn(config(()=>{b++;return true}));assert.deepEqual(execute((...args)=>s.merge(args)),execute(ref));assert.equal(a,b);s.free()
  })
  check('config/custom-iterator-mutation-and-throw-identity',()=>{
    function input(log){const c=upstream.config.defaultConfig();let i=0;const iterable={ [Symbol.iterator](){log.push('iterator');return {next(){log.push('next'+i);return i++<2?{value:'probe-'+i,done:false}:{done:true}}}}};c.classGroups.custom=iterable;return c}
    const a=[],b=[];const s=fresh(input(a)),ref=upstream.config.createCn(input(b));assert.equal(s.merge(['probe-1 probe-2']),ref('probe-1 probe-2'));assert.deepEqual(a,b);s.free()
    const sentinel=Symbol('iteration');const c=wasm.cn_default_config();c.classGroups.probe={ [Symbol.iterator](){throw sentinel}};const fail=fresh(c);assert.throws(()=>fail.merge(['x']),e=>e===sentinel);fail.free()
  })
  check('config/marker-proxy-read-timing',()=>{
    function input(log){const marker=new Proxy({$v:'isAny'},{ownKeys(t){log.push('keys');return Reflect.ownKeys(t)},get(t,k,r){if(k==='$v')log.push('v');return Reflect.get(t,k,r)}});const c=upstream.config.defaultConfig();c.classGroups.custom=[{probe:[marker]}];return c}
    const a=[],b=[];const s=fresh(input(a)),ref=upstream.config.createCn(input(b));assert.equal(s.merge(['probe-a probe-b']),ref('probe-a probe-b'));assert.deepEqual(a,b);s.free()
  })
  check('config/single-empty-string-triggers-lazy-config',()=>{const sentinel=Symbol('empty');const s=fresh(()=>{throw sentinel});assert.throws(()=>s.merge(['']),e=>e===sentinel);s.free()})
  check('config/metadata-array-methods-and-throws',()=>{
    function input(log){const c=upstream.config.defaultConfig();c.conflictingClassGroups.display=['p'];const original=c.conflictingClassGroups.display.map;c.conflictingClassGroups.display.map=function(fn){log.push('map');return original.call(this,fn)};c.orderSensitiveModifiers.join=function(sep){log.push('join:'+sep);return 'hold'};return c}
    const a=[],b=[];const s=fresh(input(a)),ref=upstream.config.createCn(input(b));for(const input of ['p-2 flex','hold:hover:p-2 hover:hold:p-4'])assert.equal(s.merge([input]),ref(input));assert.deepEqual(a,b);s.free()
    const sentinel={kind:'map'};const c=wasm.cn_default_config();c.conflictingClassGroups.display=[];c.conflictingClassGroups.display.map=()=>{throw sentinel};const fail=fresh(c);assert.throws(()=>fail.merge(['x']),e=>e===sentinel);fail.free()
  })
  check('cache/lone-array-bucket-churn-callback-counts',()=>{
    let a=0,b=0;const config=f=>({cacheSize:2,extend:{classGroups:{probe:[{probe:[f]}]}}});const s=fresh(config(()=>{a++;return true})),ref=upstream.config.createCn(config(()=>{b++;return true}));for(let pass=0;pass<5;pass++)for(let i=0;i<300;i++)assert.equal(s.merge([['probe-base','probe-'+i]]),ref(['probe-base','probe-'+i]));assert.equal(a,b);s.free()
  })
  check('utf16/unknown-marker-error-message-stays-lossless',()=>{
    const config={extend:{classGroups:{probe:[{probe:[{$v:'\ud800'}]}]}}};let expected;try{upstream.config.createCn(config)('x')}catch(error){expected={name:error.name,message:error.message}}const s=fresh(config);assert.throws(()=>s.merge(['x']),error=>error.name===expected.name && error.message===expected.message);s.free()
  })
  check('config/conflict-map-before-iteration-order',()=>{
    function input(log){const c=upstream.config.defaultConfig();for(const [field,label] of [['conflictingClassGroups','base'],['conflictingClassGroupModifiers','mod']]){const value=['p'];value.map=function(fn){log.push(label+' map');const result=Array.prototype.map.call(this,fn);result[Symbol.iterator]=function*(){log.push(label+' iter');yield* Array.prototype[Symbol.iterator].call(this)};return result};c[field].display=value}return c}
    const a=[],b=[];const s=fresh(input(a)),ref=upstream.config.createCn(input(b));assert.equal(s.merge(['p-2 flex']),ref('p-2 flex'));assert.deepEqual(a,b);s.free()
  })
  console.log(JSON.stringify({extraPassed:extra,failures},null,2))
  if(failures.length)process.exitCode=1
  else writeFileSync(new URL('./evidence/wasm-native.json',import.meta.url),JSON.stringify({fixedCases:fixtures.length,fixedPassed:passed,extraPassed:extra,failures:[],wasmBytes:bytes.length,sharedMemory:false,imports:WebAssembly.Module.imports(module),actualNativeJsValues:true,inputJsonSerialization:false},null,2)+'\n')
}
