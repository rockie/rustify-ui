import assert from 'node:assert/strict'
import {createHash} from 'node:crypto'
import {readFile} from 'node:fs/promises'
import {createRequire} from 'node:module'
import {resolve} from 'node:path'
import {pathToFileURL} from 'node:url'

const oracle = createRequire(resolve('tests/utility/compiler/oracle/package.json'))
assert(process.argv.slice(2).every(argument => ['--input-mutations','--apply-locations','--apply-multi-locations'].includes(argument)))
const inputMutations = process.argv.includes('--input-mutations')
const applyLocations = process.argv.includes('--apply-locations')
const applyMultiLocations = process.argv.includes('--apply-multi-locations')
const tailwind = oracle('tailwindcss')
assert.equal(oracle('tailwindcss/package.json').version, '4.1.13')
const inventory = JSON.parse(await readFile('tests/utility/compiler/inventory.json','utf8'))
assert.equal(inventory.baseline.commit, '1334c99db8fd26c8ea065375dd9259800863f072')
for (const path of ['index.ts','ast.ts','compile.ts','css-functions.ts','at-import.ts']) {
  const source = `packages/tailwindcss/src/${path}`
  assert.equal(createHash('sha256').update(await readFile(resolve('ref/tailwindcss',source))).digest('hex'),
    inventory.sources.find(entry => entry.path === source)?.sha256,source)
}
const directory = resolve('target/utility-conformance/compiler-api')
const facade = await import(pathToFileURL(resolve(directory,'style.js')))
const bytes = await readFile(resolve(directory,'style_bg.wasm'))
const exports = await facade.default({module_or_path:bytes})
assert.equal(exports.memory.buffer instanceof SharedArrayBuffer,false)
const limits = {input_bytes:262144,asset_bytes:262144,candidate_count:1024,ast_depth:32,
  css_bytes:262144,queued_bytes:262144,cache_bytes:16000000}
const atRule = (name,params,nodes=[]) => ({kind:'at-rule',name,params,nodes})
const declaration = (property,value) => ({kind:'declaration',property,value,important:false})
function fixture({frozen=false,lone=false,transform=false,apply=false,applyStart,applyParams='block',applyMore=[],applySource=true,custom=false,utilities=true}={}) {
  const prefix = custom ? '@utility pair { @apply block flex; } ' : ''
  const source = {file:lone?'entry\ud800.css':'entry.css',code:apply
    ? `${prefix}.authored { --x: red; --y: blue; ${[applyParams,...applyMore].map(params => `@apply ${params};`).join(' ')} } @tailwind utilities;`
    : lone?'source\udc00':'source'}
  const distinct = {...source}
  const first = {...declaration('--x',transform?'--spacing(2)':'red'),src:[source,1,2]}
  const second = {...declaration('--y','blue'),src:[source,3,4],dst:[distinct,1,2]}
  const rule = {kind:'rule',selector:'.authored',nodes:[first,second],src:[source,0,5]}
  if (apply) {
    let offset = prefix.length
    for (const params of [applyParams,...applyMore]) {
      const start = source.code.indexOf('@apply',offset), end = source.code.indexOf(';',start)
      rule.nodes.push({...atRule('@apply',params),...(applySource ? {src:[source,applyStart ?? start,end]} : {})})
      offset = end + 1
    }
  }
  const ast = [rule]
  if (custom) {
    const start = prefix.indexOf('@apply')
    ast.unshift(atRule('@utility','pair',[
      {...atRule('@apply','block flex'),src:[source,start,prefix.indexOf(';',start)]},
    ]))
  }
  if (transform) ast.unshift(atRule('@theme','',[declaration('--spacing','0.25rem')]))
  if (utilities) ast.push({...atRule('@tailwind','utilities'),src:[source,6,7]})
  if (frozen) {
    Object.freeze(source); Object.freeze(distinct)
    Object.freeze(first.src); Object.freeze(first)
    Object.freeze(second.src); Object.freeze(second.dst); Object.freeze(second)
  }
  return {ast,source,distinct,first,second,rule}
}
// Traversal IDs describe the full alias graph. Equal contents in two distinct
// objects remain distinct; original inputs and both outputs share one table.
function graphSnapshot(roots) {
  const seen = new Map(), records = []
  function visit(value) {
    if (value === undefined) return {undefined:true}
    if (value === null || typeof value !== 'object') return value
    if (seen.has(value)) return {reference:seen.get(value)}
    const id = records.length
    seen.set(value,id); records.push(null)
    records[id] = {array:Array.isArray(value),fields:Object.keys(value).map(key => [key,visit(value[key])])}
    return {reference:id}
  }
  return {roots:roots.map(visit),records}
}
function authored(ast) { return ast.find(node => node.selector === '.authored') }
function declarations(ast) {
  return ast.flatMap(node => node.kind === 'declaration' ? [node] : node.nodes ? declarations(node.nodes) : [])
}
let graphCases = 0, identityChecks = 0, graphObjects = 0, boundaries = 0, recoveries = 0
const multiApplyCases = [
  {id:'apply-multi-location',applyParams:'flex block'},
  {id:'apply-duplicate-offset',applyParams:'flex block flex'},
  {id:'apply-whitespace-offset',applyParams:'flex\ufeff\tblock\nunderline'},
  {id:'apply-variant-location',applyParams:'hover:block flex'},
  {id:'apply-shared-tuple',applyParams:'size-[2px] block'},
  {id:'apply-properties-location',applyParams:'border-2 block'},
  {id:'apply-lone-source-and-candidate',applyParams:'[--unit:\ud800] block',lone:true},
  {id:'apply-multiple-directives',applyParams:'flex block',applyMore:['underline size-[4px]']},
  {id:'apply-fractional-multiple',applyParams:'flex block',applyStart:33.5},
  {id:'apply-negative-multiple',applyParams:'flex block',applyStart:-1},
  {id:'apply-undefined-source',applyParams:'flex block',applySource:false},
  {id:'apply-without-from',applyParams:'flex block',options:{}},
  {id:'apply-empty-from',applyParams:'flex block',options:{from:''}},
  {id:'apply-frozen-metadata',applyParams:'flex block',frozen:true},
  {id:'apply-nested-custom-utility',applyParams:'pair underline',custom:true},
  {id:'apply-no-utilities',applyParams:'flex block',utilities:false},
].map(spec => ({options:{from:'entry.css'},apply:true,...spec}))
for (const spec of [
  {id:'authored',options:{from:'entry.css'}},
  {id:'without-from',options:{}},
  {id:'empty-from',options:{from:''}},
  {id:'frozen-shared-source',options:{from:'entry.css'},frozen:true},
  {id:'original-source-units',options:{from:'entry.css'},lone:true},
  {id:'transformed-declaration',options:{from:'entry.css'},transform:true},
  ...(applyLocations ? [
    {id:'apply-location',options:{from:'entry.css'},apply:true},
    {id:'apply-fractional-location',options:{from:'entry.css'},apply:true,applyStart:33.5},
    {id:'apply-negative-location',options:{from:'entry.css'},apply:true,applyStart:-1},
  ] : []),
  ...(applyMultiLocations ? multiApplyCases : []),
  {id:'no-features',options:{from:'entry.css'},utilities:false},
]) {
  const expectedInput = fixture(spec), actualInput = fixture(spec)
  const expected = await tailwind.compileAst(expectedInput.ast,spec.options)
  const actual = new facade.AstCompiler(actualInput.ast,JSON.stringify({limits,options:spec.options}))
  try {
    const firstExpected = expected.build(['block']), firstActual = actual.build(['block'])
    assert.deepEqual(firstActual,firstExpected,`${spec.id}: first AST`)
    const secondExpected = expected.build(['flex']), secondActual = actual.build(['flex'])
    const noOpExpected = expected.build([]), noOpActual = actual.build([])
    const roots = (input,first,second,noOp) => [
      ...(inputMutations ? [input.ast] : []),input.source,input.distinct,input.first,input.second,first,second,noOp]
    const reference = graphSnapshot(roots(expectedInput,firstExpected,secondExpected,noOpExpected))
    const result = graphSnapshot(roots(actualInput,firstActual,secondActual,noOpActual))
    assert.deepEqual(result,reference,`${spec.id}: original/input/output object graph`)
    graphObjects += result.records.length; graphCases++
    if (spec.id === 'authored') {
      const a = authored(firstActual), b = authored(secondActual)
      const checks = [a !== actualInput.rule,a.nodes[0] === actualInput.first,
        a.src[0] === actualInput.source,a.src[0] === a.nodes[1].src[0],
        a.nodes[1].dst[0] === actualInput.distinct,a.nodes[1].dst[0] !== a.src[0],
        a.nodes[0] === b.nodes[0],a.src[0] === b.src[0],a.nodes[0].src === actualInput.first.src,
        declarations(firstActual).find(node => node.property === 'display' && node.value === 'block') ===
        declarations(secondActual).find(node => node.property === 'display' && node.value === 'block')]
      assert(checks.every(Boolean)); identityChecks += checks.length
    }
  } finally { actual.free() }
}

const imported = '.imported { --a:red; --b:blue; }'
const assets = [{id:'pkg.css',path:'/mapped/pkg.css',base:'/discover',content:imported}]
const options = {from:'entry.css',base:'/discover'}
const input = () => [atRule('@import','"pkg.css"'),atRule('@tailwind','utilities')]
const expectedInput = input(), actualInput = input()
const expected = await tailwind.compileAst(expectedInput,{...options,loadStylesheet:async () => ({path:assets[0].path,base:assets[0].base,content:imported})})
const actual = new facade.AstCompiler(actualInput,JSON.stringify({limits,options,assets}))
try {
  const a = expected.build(['block']), b = actual.build(['block'])
  const c = expected.build(['flex']), d = actual.build(['flex'])
  const result = graphSnapshot([...(inputMutations ? [actualInput] : []),b,d])
  assert.deepEqual(result,graphSnapshot([...(inputMutations ? [expectedInput] : []),a,c]),'imported Source and declarations')
  graphCases++; graphObjects += result.records.length
} finally { actual.free() }

const marker = new Error('original getter exception')
const getter = declaration('--x','red')
Object.defineProperty(getter,'src',{enumerable:true,get(){throw marker}})
assert.throws(() => new facade.AstCompiler([getter],JSON.stringify({limits})),error => error === marker)
boundaries++
const cyclic = declaration('--x','red'); cyclic.src = cyclic
assert.throws(() => new facade.AstCompiler([cyclic],JSON.stringify({limits:{...limits,ast_depth:2}})),
  error => error.cause?.error?.kind === 'limit' && error.cause.error.resource === 'ast_depth')
boundaries++
const tight = new facade.AstCompiler(input().slice(1),JSON.stringify({limits:{...limits,cache_bytes:32768}}))
try {
  const previous = tight.build(['block'])
  const saved = graphSnapshot([previous])
  assert.throws(() => tight.build(Array.from({length:128},(_,index) => `[--graph-${index}:${index}]`)),
    error => error.cause?.error?.kind === 'limit' && error.cause.error.resource === 'cache_bytes')
  assert.equal(tight.build([]),previous)
  assert.deepEqual(graphSnapshot([previous]),saved)
  assert(tight.build(['flex']).some(node => node.selector === '.flex'))
  boundaries++; recoveries++
} finally { tight.free() }
const crossingInput = () => [atRule('@tailwind','utilities')]
const crossingOracle = await tailwind.compileAst(crossingInput())
const crossingActual = new facade.AstCompiler(crossingInput(),JSON.stringify({limits}))
try {
  function recover(api) {
    const previous = api.build(['block'])
    const node = previous.find(node => node.selector === '.block').nodes[0]
    const descriptor = Object.getOwnPropertyDescriptor(node,'value')
    const marker = new Error('original output getter exception')
    Object.defineProperty(node,'value',{enumerable:true,configurable:true,get(){throw marker}})
    assert.throws(() => api.build(['flex']),error => error === marker)
    Object.defineProperty(node,'value',descriptor)
    const noOp = api.build([]), next = api.build(['grid'])
    return {noOpSame:noOp===previous,noOpFlex:noOp.some(node => node.selector === '.flex'),
      nextGridHasFlex:next.some(node => node.selector === '.flex')}
  }
  const expected = recover(crossingOracle), actual = recover(crossingActual)
  assert.deepEqual(expected,{noOpSame:true,noOpFlex:false,nextGridHasFlex:true})
  assert.deepEqual(actual,expected,'output getter failure retains prior AST and admitted candidates')
  boundaries++; recoveries++
} finally { crossingActual.free() }
assert.equal(graphCases,8 + (applyLocations ? 3 : 0) + (applyMultiLocations ? multiApplyCases.length : 0)); assert.equal(identityChecks,10)
assert.equal(boundaries,4); assert.equal(recoveries,2)
console.log(JSON.stringify({graphCases,identityChecks,graphObjects,boundaries,recoveries,
  actualWasm:true,sharedMemory:false,wasmBytes:bytes.length,inputMutationConformance:'pending',
  singleCandidateApplyLocations:applyLocations ? 'checked' : 'not-run',applyLocationConformance:'pending',
  multiCandidateApplyLocations:applyMultiLocations ? multiApplyCases.length : 0,
  fullPublicObjectGraphConformance:'pending'}))
