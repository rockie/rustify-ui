import assert from 'node:assert/strict'
import { readFile } from 'node:fs/promises'
import { resolve } from 'node:path'
import { pathToFileURL } from 'node:url'
import { createHash } from 'node:crypto'

const index = process.argv.indexOf('--output')
const output = resolve(index < 0 ? 'target/utility-conformance/compiler-api' : process.argv[index + 1])
const facade = await import(pathToFileURL(resolve(output, 'style.js')))
const bytes = await readFile(resolve(output, 'style_bg.wasm'))
const exports = await facade.default({ module_or_path: bytes })
assert.equal(typeof facade.CssCompiler, 'function', 'public CSS compiler must cross actual WASM')
assert.equal(typeof facade.AstCompiler, 'function', 'public AST compiler must cross actual WASM')
assert.equal(typeof facade.DesignSystem, 'function', 'public design system must cross actual WASM')
assert.equal(typeof facade.scan_content, 'undefined')
assert.equal(typeof SharedArrayBuffer !== 'undefined' && exports.memory.buffer instanceof SharedArrayBuffer, false)
const fixture = JSON.parse(await readFile(new URL('./public-api-fixtures.json', import.meta.url), 'utf8'))
const limits = { input_bytes: 65536, asset_bytes: 65536, candidate_count: 1024,
  ast_depth: 32, css_bytes: 262144, queued_bytes: 65536, cache_bytes: 4194304 }
const configuration = item => JSON.stringify({ options: item.options, assets: item.assets ?? [], limits })
const snapshot = value => JSON.parse(JSON.stringify(value, (_, item) =>
  typeof item === 'bigint' ? { bigint: item.toString() } : item === undefined ? { undefined: true } : item))
function hydrate(value) {
  if (value && typeof value === 'object' && !Array.isArray(value) && value.undefined === true && Object.keys(value).length === 1) return undefined
  if (Array.isArray(value)) return value.map(hydrate)
  if (value && typeof value === 'object') return Object.fromEntries(Object.entries(value).map(([key, child]) => [key, hydrate(child)]))
  return value
}
function sourceMapSnapshot(map) {
  const sourceReferences = map.mappings.map(mapping => {
    if (!mapping.originalPosition) return null
    const index = map.sources.indexOf(mapping.originalPosition.source)
    assert.notEqual(index, -1, 'WASM mapping must share the decoded Source table object')
    return index
  })
  return { decoded: snapshot(map), sourceReferences }
}
let cssBuilds = 0, maps = 0, mappings = 0, astBuilds = 0, designCases = 0, classCompletions = 0, selectorProbes = 0
for (const item of fixture.compileCases) {
  if (item.expectedError) {
    assert.throws(() => new facade.CssCompiler(item.input, configuration(item)), error =>
      error.cause.error.message === item.expectedError)
    continue
  }
  const api = new facade.CssCompiler(item.input, configuration(item))
  try {
    assert.equal(api.features, item.expectedFeatures, item.id)
    assert.deepEqual(api.sources, item.expectedSources, item.id)
    assert.deepEqual(api.root, item.expectedRoot, item.id)
    assert.deepEqual(api.warnings, item.expectedWarnings, item.id)
    assert.deepEqual(api.stylesheetLoads, item.expectedLoadCalls, item.id)
    const initial = sourceMapSnapshot(api.buildSourceMap())
    assert.deepEqual(initial, item.expectedInitialSourceMap, `${item.id}: initial map`)
    maps++; mappings += initial.decoded.mappings.length
    for (const [index, batch] of item.batches.entries()) {
      assert.equal(api.build(batch), item.expectedBuilds[index].css, `${item.id}: build ${index}`)
      const map = sourceMapSnapshot(api.buildSourceMap())
      assert.deepEqual(map, item.expectedBuilds[index].sourceMap, `${item.id}: map ${index}`)
      cssBuilds++; maps++; mappings += map.decoded.mappings.length
    }
  } finally { api.free() }
}
for (const item of fixture.compileAstCases) {
  const input = hydrate(item.inputAst)
  const api = new facade.AstCompiler(input, configuration(item))
  try {
    assert.equal(api.features, item.expectedFeatures, item.id)
    assert.deepEqual(api.sources, item.expectedSources, item.id)
    assert.deepEqual(api.root, item.expectedRoot, item.id)
    assert.deepEqual(api.stylesheetLoads, item.expectedLoadCalls, item.id)
    let previous
    for (const [index, batch] of item.batches.entries()) {
      const ast = api.build(batch), expected = item.expectedBuilds[index]
      assert.deepEqual(snapshot(ast), expected.ast, `${item.id}: AST ${index}`)
      assert.equal(ast === input, expected.sameAsInput, `${item.id}: input array reuse ${index}`)
      assert.equal(previous !== undefined && ast === previous, expected.sameAsPrevious, `${item.id}: previous array reuse ${index}`)
      previous = ast; astBuilds++
    }
  } finally { api.free() }
}
assert.equal(cssBuilds, 313); assert.equal(maps, 371); assert.equal(mappings, 6970); assert.equal(astBuilds, 61)

for (const item of fixture.designSystemCases) {
  const api = new facade.DesignSystem(item.input, configuration(item))
  try {
    assert.equal(api.important, item.important)
    for (const result of item.candidateResults) {
      const parsed = api.parseCandidate(result.raw)
      assert.deepEqual(snapshot(parsed), result.parsed)
      assert.equal(api.parseCandidate(result.raw), parsed, 'candidate cache returns the same JS array')
      for (const [index, candidate] of parsed.entries()) {
        assert.equal(api.printCandidate(candidate), result.printed[index])
        assert.deepEqual(snapshot(api.compileAstNodes(candidate, 0)), result.compiled[index].flagsNone)
        assert.deepEqual(snapshot(api.compileAstNodes(candidate, 1)), result.compiled[index].flagsRespectImportant)
      }
    }
    const parsedVariants = new Map()
    for (const result of item.variantResults) {
      const parsed = api.parseVariant(result.raw)
      assert.deepEqual(snapshot(parsed), result.parsed)
      assert.equal(api.parseVariant(result.raw), parsed, 'variant cache returns the same JS object')
      if (parsed !== null) {
        assert.equal(api.printVariant(parsed), result.printed)
        parsedVariants.set(result.raw, parsed)
      }
    }
    const classes = api.getClassList()
    assert.equal(classes.length, item.expectedClassList.count)
    assert.equal(createHash('sha256').update(JSON.stringify(classes)).digest('hex'), item.expectedClassList.sha256)
    assert.deepEqual(classes.filter(([name]) => /^(?:-?m-|w-|bg-brand|flex$|probe$)/.test(name)), item.expectedClassList.representatives)
    classCompletions += classes.length
    const variants = api.getVariants().map(({selectors, ...metadata}) => ({...metadata,
      selectorProbes: [{}, {value: metadata.values[0] ?? 'card'}, {value:'[state=open]', modifier:'named'}].map(options => {
        selectorProbes++
        return {options, selectors: selectors(options)}
      }),
    }))
    assert.deepEqual(variants, item.expectedVariants)
    const order = api.getClassOrder(item.classOrderInput)
    assert(order.some(([, value]) => typeof value === 'bigint'), 'class order carries actual JS BigInt')
    assert.deepEqual(snapshot(order), item.expectedClassOrder)
    assert.deepEqual(api.candidatesToCss(item.candidateResults.map(({raw}) => raw)), item.expectedCandidatesToCss)
    const variantOrder = api.getVariantOrder()
    assert(variantOrder instanceof Map)
    assert.deepEqual([...variantOrder].map(([variant, order]) => ({variant: snapshot(variant), order})), item.expectedVariantOrder)
    for (const variant of parsedVariants.values()) assert(variantOrder.has(variant), 'Map key is the cached parseVariant object')
    assert(api.invalidCandidates instanceof Set)
    assert.deepEqual([...api.invalidCandidates], item.expectedInvalidCandidates)
    for (const result of item.resolveResults) assert.deepEqual(snapshot(api.resolveThemeValue(result.path, result.forceInline)), result.result)
  } finally { api.free() }
  const tracker = new facade.DesignSystem(item.input, configuration(item))
  try {
    assert.deepEqual(snapshot(tracker.themeEntries()), item.trackUsedVariables.beforeTracking)
    tracker.trackUsedVariables(item.trackUsedVariables.input)
    assert.deepEqual(snapshot(tracker.themeEntries()), item.trackUsedVariables.afterTracking)
    tracker.trackUsedVariables(item.trackUsedVariables.input)
    assert.deepEqual(snapshot(tracker.themeEntries()), item.trackUsedVariables.repeatedTracking)
  } finally { tracker.free() }
  designCases++
}
assert.equal(designCases, 3); assert.equal(classCompletions, 21771); assert.equal(selectorProbes, 756)

let boundaryCases = 0
const config = overrides => JSON.stringify({ options: { from: 'input.css' }, assets: [], limits: { ...limits, ...overrides } })
const failed = (operation, kind, resource) => {
  assert.throws(operation, thrown => {
    const result = thrown.cause.error
    return result.kind === kind && (resource === undefined || result.resource === resource)
  })
  boundaryCases++
}
for (const key of Object.keys(limits)) {
  failed(() => new facade.CssCompiler('@tailwind utilities;', config({ [key]: 0 })), 'invalid_limit', key)
  failed(() => new facade.AstCompiler([], config({ [key]: 0 })), 'invalid_limit', key)
  failed(() => new facade.DesignSystem('', config({ [key]: 0 })), 'invalid_limit', key)
}
failed(() => new facade.CssCompiler('@tailwind utilities;', config({ input_bytes: 1 })), 'limit', 'input_bytes')
failed(() => new facade.AstCompiler([{kind: 'comment', value: 'a'.repeat(400)}], config({input_bytes: 80})), 'limit', 'input_bytes')
failed(() => new facade.AstCompiler([{kind: 'comment', value: 'a'.repeat(400)}], config({cache_bytes: 80})), 'limit', 'cache_bytes')
let nested = {kind:'declaration', property:'color', value:'red', important:false}
for (let index = 0; index < 20; index++) nested = {kind:'rule', selector:'&', nodes:[nested]}
failed(() => new facade.AstCompiler([nested], config({ast_depth: 2})), 'limit', 'ast_depth')
const cycle = []; cycle.push(cycle)
failed(() => new facade.AstCompiler(cycle, config({ast_depth: 2})), 'limit', 'ast_depth')
const api = new facade.CssCompiler('@tailwind utilities;', config({css_bytes: 110}))
try {
  const before = api.build(['flex']), map = sourceMapSnapshot(api.buildSourceMap())
  failed(() => api.build(['w-[137px]']), 'limit', 'css_bytes')
  assert.equal(api.build(['\ud800']), before, 'unknown non-scalar candidate is admitted as invalid without changing CSS'); boundaryCases++
  assert.equal(api.build([]), before)
  assert.deepEqual(sourceMapSnapshot(api.buildSourceMap()), map)
  const thrown = new Error('caller array accessor failed')
  const values = []; Object.defineProperty(values, '0', {get() {throw thrown}})
  assert.throws(() => api.build(values), error => error === thrown); boundaryCases++
  assert.equal(api.build([]), before, 'throwing getter must release the WASM mutable borrow')
  const revoked = Proxy.revocable([], {}); revoked.revoke()
  assert.throws(() => api.build(revoked.proxy), error => error instanceof TypeError); boundaryCases++
  assert.equal(api.build([]), before, 'revoked Proxy must release the WASM mutable borrow')
} finally { api.free() }
const astApi = new facade.AstCompiler([{kind:'at-rule',name:'@tailwind',params:'utilities',nodes:[]}], config({css_bytes:110}))
try {
  const previous = astApi.build(['flex'])
  failed(() => astApi.build(['w-[137px]']), 'limit', 'css_bytes')
  assert.equal(astApi.build([]), previous, 'failed AST build must retain its last JS array')
} finally { astApi.free() }
const design = new facade.DesignSystem('@theme {--color-brand:red}', config({}))
const first = design.parseVariant('data-[a_b]'), second = design.parseVariant('data-[a b]')
assert.deepEqual(first, second); assert.notEqual(first, second)
const aliasOrder = design.getVariantOrder()
assert.equal(aliasOrder.get(first), 0); assert.equal(aliasOrder.get(second), 1); boundaryCases++
const callback = design.getVariants().find(variant => variant.name === 'hover').selectors
const hoverSelectors = fixture.designSystemCases[0].expectedVariants.find(variant => variant.name === 'hover').selectorProbes[0].selectors
const callbackError = new Error('selector options failed')
assert.throws(() => callback({get value() {throw callbackError}}), error => error === callbackError)
assert.deepEqual(callback({}), hoverSelectors); boundaryCases++
design.free()
assert.deepEqual(callback({}), hoverSelectors, 'JS callback owns its native state after wrapper free'); boundaryCases++
const boundedDesign = new facade.DesignSystem('', config({candidate_count:1, cache_bytes:32768}))
try {
  const flex = boundedDesign.parseCandidate('flex')
  failed(() => boundedDesign.parseCandidate('block'), 'limit', 'candidate_count')
  failed(() => boundedDesign.getClassList(), 'limit', 'cache_bytes')
  assert.equal(boundedDesign.parseCandidate('flex'), flex, 'quota failure preserves the cached JS array')
  assert.deepEqual(boundedDesign.candidatesToCss(['flex']), ['.flex {\n  display: flex;\n}\n'])
  boundedDesign.important = true
  assert.equal(boundedDesign.compileAstNodes(flex[0], 0)[0].node.nodes[0].important, false)
  assert.equal(boundedDesign.compileAstNodes(flex[0], 1)[0].node.nodes[0].important, true)
  boundaryCases++
} finally { boundedDesign.free() }
console.log(JSON.stringify({ cssBuilds, maps, mappings, astBuilds, designCases, classCompletions, selectorProbes, boundaryCases, actualWasm: true, sharedMemory: false, wasmBytes: bytes.length, fullApiConformance: 'pending' }))
