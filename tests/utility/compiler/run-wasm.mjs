import assert from 'node:assert/strict'
import { createHash } from 'node:crypto'
import { readFile } from 'node:fs/promises'
import { resolve } from 'node:path'
import { pathToFileURL } from 'node:url'

const output = resolve('target/utility-conformance/compiler')
const facade = await import(pathToFileURL(resolve(output, 'style.js')))
const bytes = await readFile(resolve(output, 'style_bg.wasm'))
const module = await WebAssembly.compile(bytes)
assert.equal(WebAssembly.Module.exports(module).some(item => item.name.includes('scan_content')), false)
const exports = await facade.default({ module_or_path: module })
const sharedMemory = typeof SharedArrayBuffer !== 'undefined' && exports.memory.buffer instanceof SharedArrayBuffer
assert.equal(sharedMemory, false)
const corpus = JSON.parse(await readFile(new URL('./fixtures.json', import.meta.url), 'utf8'))
const smoke = corpus.cases.find(item => item.name === 'compiler-smoke')
const limits = { input_bytes: 65536, asset_bytes: 65536, candidate_count: 1024,
  ast_depth: 32, css_bytes: 262144, queued_bytes: 65536, cache_bytes: 1048576 }
const config = owner => JSON.stringify({ input: { css: smoke.css, entry: 'memory:input.css' }, assets: [], limits, owner })
const session = new facade.CompilerSession(config(null))
try {
  const prepared = JSON.parse(session.prepare(JSON.stringify(smoke.candidates)))
  assert.equal(prepared.report.css, smoke.expectedCss)
  assert.equal(JSON.parse(session.report()).revision, 0)
  const wrong = prepared.token + 1
  assert.throws(() => session.commit(wrong), error => JSON.parse(error).error.kind === 'stale_prepared')
  session.discard(prepared.token)
  assert.equal(JSON.parse(session.report()).revision, 0)
  const retry = JSON.parse(session.prepare(JSON.stringify(smoke.candidates)))
  assert.equal(JSON.parse(session.commit(retry.token)).css, smoke.expectedCss)
  const repeated = JSON.parse(session.prepare(JSON.stringify(smoke.candidates)))
  assert.equal(repeated.report.changed, false)
  assert.equal(repeated.report.metrics.parsed_candidates, 0)
  assert.equal(repeated.report.metrics.generated_candidates, 0)
  session.commit(repeated.token)
  const rejected = JSON.parse(session.prepare(JSON.stringify(smoke.invalidCandidates)))
  assert.equal(rejected.report.rejected.length, smoke.invalidCandidates.length)
  session.commit(rejected.token)
} finally { session.free() }
const scoped = new facade.CompilerSession(config('wasm-smoke'))
try {
  const prepared = JSON.parse(scoped.prepare(JSON.stringify(smoke.candidates)))
  assert.match(prepared.report.css, /@scope \(\[data-rustify-style-owner="wasm-smoke"\]\)/)
  assert.match(prepared.report.css, /width: 137px/)
} finally { scoped.free() }
const nesting = corpus.cases.find(item => item.name === 'css-scope-property-nesting')
assert.ok(nesting, 'official nesting fixture must exist')
const nestingSession = new facade.CompilerSession(JSON.stringify({
  input: { css: nesting.css, entry: 'memory:input.css' }, assets: [], limits, owner: null,
}))
try {
  const prepared = JSON.parse(nestingSession.prepare(JSON.stringify(nesting.candidates)))
  assert.equal(prepared.report.css, nesting.expectedCss)
  assert.equal(prepared.report.unsupported.length, 0)
  assert.equal(prepared.report.gpu_variables['--tw-ring-offset-width'], '0px')
  assert.equal(prepared.report.gpu_variables['--tw-ring-offset-color'], '#fff')
  nestingSession.commit(prepared.token)
} finally { nestingSession.free() }
let sliceCases = 0
let incrementalComparisons = 0
let incrementalCases = 0
const functionSession = new facade.CompilerSession(JSON.stringify({
  input: {css: '@theme { --color-brand: #123456; --color-alias: --theme(--color-brand); --spacing: 4px; } @tailwind utilities;', entry: 'input.css'},
  assets: [], limits, owner: null,
}))
try {
  const candidates = ['bg-alias', 'w-[--spacing(3)]']
  const first = JSON.parse(functionSession.prepare(JSON.stringify(candidates)))
  assert.equal(first.report.gpu_variables['--color-alias'], 'var(--color-brand)')
  const width = first.report.gpu_programs.find(program => program.candidate === candidates[1])
  assert.deepEqual(width.declarations[0].value, {kind: 'css_expression', value: 'calc(var(--spacing) * 3)'})
  functionSession.commit(first.token)
  const repeat = JSON.parse(functionSession.prepare(JSON.stringify(candidates)))
  assert.equal(repeat.report.metrics.parsed_candidates, 0)
  assert.equal(repeat.report.metrics.generated_candidates, 0)
  functionSession.commit(repeat.token)
} finally { functionSession.free() }
const boundedFunctions = new facade.CompilerSession(JSON.stringify({
  input: {css: '@tailwind utilities;', entry: 'input.css'}, assets: [], limits: {...limits, css_bytes: 80}, owner: null,
}))
try {
  const first = JSON.parse(boundedFunctions.prepare(JSON.stringify(['block'])))
  boundedFunctions.commit(first.token)
  const before = boundedFunctions.report()
  assert.throws(() => boundedFunctions.prepare(JSON.stringify(['hidden', '[value:theme(fontFamily.sans)_theme(fontFamily.sans)]'])), error => {
    const result = JSON.parse(error).error
    return result.kind === 'limit' && result.resource === 'css_bytes' && result.limit === 80
  })
  assert.equal(boundedFunctions.report(), before)
} finally { boundedFunctions.free() }
assert.throws(() => new facade.CompilerSession(JSON.stringify({
  input: {css: '@theme inline { --font-value: abcdefghijklmnopqrstuvwxyz; --font-a: --theme(--font-value); --font-b: --theme(--font-value); } @tailwind utilities;', entry: 'input.css'},
  assets: [], limits: {...limits, css_bytes: 1000, cache_bytes: 40}, owner: null,
})), error => { const result = JSON.parse(error).error
  return result.kind === 'limit' && result.resource === 'cache_bytes' && result.limit === 40
})
assert.throws(() => new facade.CompilerSession(JSON.stringify({
  input: {css: '@utility probe { color:red; } @tailwind utilities;', entry: 'input.css'},
  assets: [], limits: {...limits, cache_bytes: 1}, owner: null,
})), error => { const result = JSON.parse(error).error
  return result.kind === 'limit' && result.resource === 'cache_bytes' && result.limit === 1
})
const boundedCustom = new facade.CompilerSession(JSON.stringify({
  input: {css: '@utility probe-* { --result:--value([*]) --value([*]) --value([*]) --value([*]); } @tailwind utilities;', entry: 'input.css'},
  assets: [], limits: {...limits, css_bytes: 80}, owner: null,
}))
try {
  const first = JSON.parse(boundedCustom.prepare(JSON.stringify(['block'])))
  boundedCustom.commit(first.token)
  const before = boundedCustom.report()
  assert.throws(() => boundedCustom.prepare(JSON.stringify(['probe-[a-long-repeated-arbitrary-value]'])), error => {
    const result = JSON.parse(error).error
    return result.kind === 'limit' && result.resource === 'css_bytes' && result.limit === 80
  })
  assert.equal(boundedCustom.report(), before)
  const retry = JSON.parse(boundedCustom.prepare(JSON.stringify(['hidden'])))
  assert.equal(retry.report.unsupported.length, 0)
  boundedCustom.commit(retry.token)
} finally { boundedCustom.free() }
assert.throws(() => new facade.CompilerSession(JSON.stringify({
  input: {css: '@custom-variant sample (&:hover); @tailwind utilities;', entry: 'input.css'},
  assets: [], limits: {...limits, cache_bytes: 1}, owner: null,
})), error => { const result = JSON.parse(error).error
  return result.kind === 'limit' && result.resource === 'cache_bytes' && result.limit === 1
})
for (const [caseLimits, candidates, resource] of [
  [{...limits, css_bytes: 80}, ['sample:w-[13px]'], 'css_bytes'],
  [{...limits, ast_depth: 3}, ['sample:sample:sample:sample:w-[13px]'], 'ast_depth'],
]) {
  const boundedVariant = new facade.CompilerSession(JSON.stringify({
    input: {css: '@custom-variant sample (&:hover, &:focus); @tailwind utilities;', entry: 'input.css'},
    assets: [], limits: caseLimits, owner: null,
  }))
  try {
    const first = JSON.parse(boundedVariant.prepare(JSON.stringify(['block'])))
    boundedVariant.commit(first.token)
    const before = boundedVariant.report()
    assert.throws(() => boundedVariant.prepare(JSON.stringify(candidates)), error => {
      const result = JSON.parse(error).error
      return result.kind === 'limit' && result.resource === resource
    })
    assert.equal(boundedVariant.report(), before)
    const retry = JSON.parse(boundedVariant.prepare(JSON.stringify(['hidden'])))
    assert.equal(retry.report.unsupported.length, 0)
    boundedVariant.commit(retry.token)
  } finally { boundedVariant.free() }
}
const boundedLegacy = new facade.CompilerSession(JSON.stringify({
  input: {css: '@tailwind utilities;', entry: 'input.css'}, assets: [], limits: {...limits, cache_bytes: 100000}, owner: null,
}))
try {
  const first = JSON.parse(boundedLegacy.prepare(JSON.stringify(['block'])))
  boundedLegacy.commit(first.token)
  const before = boundedLegacy.report()
  assert.throws(() => boundedLegacy.prepare(JSON.stringify(['hidden', 'h-[theme(spacing.12)]'])), error => {
    const result = JSON.parse(error).error
    return result.kind === 'limit' && result.resource === 'cache_bytes'
  })
  assert.equal(boundedLegacy.report(), before)
  const retry = JSON.parse(boundedLegacy.prepare(JSON.stringify(['hidden'])))
  assert.equal(retry.report.unsupported.length, 0)
  boundedLegacy.commit(retry.token)
} finally { boundedLegacy.free() }
assert.throws(() => new facade.CompilerSession(JSON.stringify({
  input: {css: '.probe { value: theme(spacing.12); }', entry: 'input.css'}, assets: [], limits: {...limits, cache_bytes: 100000}, owner: null,
})), error => {
  const result = JSON.parse(error).error
  return result.kind === 'limit' && result.resource === 'cache_bytes'
})
let pendingBoundaryCases = 0
let pendingFeatureCases = 0
let utf16OutputCases = 0
let errorCases = 0
let variantParseCases = 0
let variantParseNullCases = 0
let variantReusedCases = 0
let variantParserLimitCases = 0
let sourceMetadataCases = 0
const variantParametersOnly = process.argv.includes('--variant-parameters')
const checkVariantParameters = process.argv.includes('--slice') || variantParametersOnly
const variantParameters = checkVariantParameters
  ? JSON.parse(await readFile(new URL('./variant-parameters-fixtures.json', import.meta.url), 'utf8'))
  : null
const variantCases = []
if (variantParameters) {
  const sha256 = value => createHash('sha256').update(typeof value === 'string' ? value : JSON.stringify(value)).digest('hex')
  variantCases.push(...variantParameters.cases)
  const referencedFixtures = new Map()
  for (const item of variantParameters.reusedCases) {
    const {file, caseId, caseSha256} = item.reference
    if (!referencedFixtures.has(file)) referencedFixtures.set(file,
      JSON.parse(await readFile(new URL(`./${file}`, import.meta.url), 'utf8')))
    const source = referencedFixtures.get(file).cases.find(item => item.id === caseId)
    assert.ok(source, `${item.id}: missing reused case ${file}:${caseId}`)
    assert.equal(sha256(source), caseSha256, `${item.id}: reused case digest`)
    assert.equal(sha256([source.input, source.candidates]), item.inputCandidatesSha256, `${item.id}: reused input digest`)
    assert.equal(sha256({expectedCss: source.expectedCss}), item.officialResultSha256, `${item.id}: reused result digest`)
    variantCases.push({...source, id: `${item.id}:reused:${file}:${caseId}`})
    variantReusedCases++
  }
  assert.equal(typeof facade.CompilerSession.prototype.parse_variant, 'function', 'WASM facade must expose the theme-aware variant parser')
  for (const [mode, css] of Object.entries(variantParameters.inputModes)) {
    const parser = new facade.CompilerSession(JSON.stringify({
      input: {css, entry: 'memory:input.css'}, assets: [], limits, owner: null,
    }))
    try {
      const before = parser.report()
      for (const item of variantParameters.parseCases.filter(item => item.mode === mode)) {
        assert.equal(sha256(item.expectedVariant), item.expectedVariantSha256, `${item.id}: parser oracle digest`)
        assert.deepEqual(JSON.parse(parser.parse_variant(item.raw)), item.expectedVariant, item.id)
        variantParseCases++
        if (item.expectedVariant === null) variantParseNullCases++
      }
      for (const [raw, resource, limit] of [
        ['x'.repeat(limits.queued_bytes + 1), 'queued_bytes', limits.queued_bytes],
        ['group-'.repeat(limits.ast_depth + 1) + 'hover', 'ast_depth', limits.ast_depth],
      ]) {
        assert.throws(() => parser.parse_variant(raw), error => { const result = JSON.parse(error).error;
          return result.kind === 'limit' && result.resource === resource && result.limit === limit
        }, `${mode}: variant parser ${resource} budget`)
        variantParserLimitCases++
      }
      assert.equal(parser.report(), before, `${mode}: variant parsing must not change committed state`)
    } finally { parser.free() }
  }
  assert.equal(variantParseCases, variantParameters.parseCases.length, 'every variant parser mode must execute')
}
if (process.argv.includes('--slice') || variantParametersOnly) {
  const cases = [...variantCases]
  if (!variantParametersOnly) {
    const utf16Boundaries = JSON.parse(await readFile(new URL('./legacy-theme-boundaries.json', import.meta.url), 'utf8'))
    for (const item of utf16Boundaries.cases) {
      const session = new facade.CompilerSession(JSON.stringify({
        input: {css: item.input, entry: 'memory:input.css'}, assets: [], limits, owner: null,
      }))
      try {
        const css = JSON.parse(session.report()).css
        assert.deepEqual(Array.from({length:css.length}, (_, index) => css.charCodeAt(index)), item.expectedCssUtf16Units, item.id)
        const staged = JSON.parse(session.prepare(JSON.stringify(item.candidates)))
        const committed = JSON.parse(session.commit(staged.token)).css
        assert.deepEqual(Array.from({length:committed.length}, (_, index) => committed.charCodeAt(index)), item.expectedCssUtf16Units, `${item.id}: committed`)
        utf16OutputCases++
      } finally { session.free() }
    }
    const staticCorpus = JSON.parse(await readFile(new URL('./static-fixtures.json', import.meta.url), 'utf8'))
    cases.push(...staticCorpus.cases.flatMap(item => [
      { id: item.id, input: staticCorpus.input, candidates: [item.root], expectedCss: item.expectedCss },
      { id: `${item.id}:important`, input: staticCorpus.input, candidates: [`${item.root}!`], expectedCss: item.expectedImportantCss },
      { id: `${item.id}:hover`, input: staticCorpus.input, candidates: [`hover:${item.root}`], expectedCss: item.expectedHoverCss },
    ]))
    cases.push(...staticCorpus.probeCases.map(item => ({ ...item, candidates: [item.root] })))
    cases.push({ ...staticCorpus.union, id: 'static:union', input: staticCorpus.input })
    for (const name of ['effects', 'variant', 'functional-spacing', 'dynamic-variant', 'colors', 'typography', 'imports', 'borders', 'backgrounds', 'gradients', 'transforms', 'theme', 'filters', 'shadows', 'prefix', 'motion', 'grid', 'layout', 'masks', 'miscellaneous', 'structural-spacing', 'effects-complete', 'arbitrary-properties', 'custom-utilities', 'custom-variants', 'parser-errors', 'apply', 'functions', 'legacy-theme']) {
      const fixture = JSON.parse(await readFile(new URL(`./${name}-fixtures.json`, import.meta.url), 'utf8'))
      cases.push(...fixture.cases)
    }
    const externalVariables = JSON.parse(await readFile(new URL('./external-variables-fixtures.json', import.meta.url), 'utf8'))
    cases.push(...externalVariables.cases.map(item => ({...item,
      buildSteps:item.batches.map((candidates,index)=>({candidates,expectedCss:item.expectedBuilds[index]})),
    })))
    const sourceDirectives = JSON.parse(await readFile(new URL('./source-directives-fixtures.json', import.meta.url), 'utf8'))
    cases.push(...sourceDirectives.cases.map(item => {
      if (item.batches) assert.equal(item.batches.length, item.expectedBuilds.length, `${item.id}: incremental oracle count`)
      return {
        ...item,
        entry: item.entry ?? '/app/input.css',
        limits: {input_bytes: 262144, asset_bytes: 262144, candidate_count: 8192,
          ast_depth: 64, css_bytes: 2097152, queued_bytes: 2097152, cache_bytes: 8388608, ...item.limits},
        ...(item.batches ? {buildSteps: item.batches.map((candidates, index) => ({candidates, expectedCss: item.expectedBuilds[index]}))} : {}),
      }
    }))
  }
  for (const item of cases) {
    const caseLimits = {...limits, ...item.limits}
    if (item.pendingFeature) {
      assert.throws(() => new facade.CompilerSession(JSON.stringify({
        input: {css: item.input, entry: item.entry ?? 'memory:input.css'}, assets: item.assets ?? [], limits: caseLimits, owner: null,
      })), error => { const result = JSON.parse(error).error;
        return result.kind === 'pending_feature' && result.feature === item.pendingFeature
      }, item.id)
      pendingFeatureCases++
      continue
    }
    if (item.expectedError) {
      const expectedError = typeof item.expectedError === 'string' ? item.expectedError : item.expectedError.message
      let session, phase = 'compile'
      try {
        assert.throws(() => {
          session = new facade.CompilerSession(JSON.stringify({
            input: {css: item.input, entry: item.entry ?? 'memory:input.css'}, assets: item.assets ?? [], limits: caseLimits, owner: null,
          }))
          phase = 'build'
          session.prepare(JSON.stringify(item.candidates))
        }, error => {
          const result = JSON.parse(error).error
          const expectedName = item.expectedErrorName ?? item.expectedError.name
          if (expectedName) assert.equal(result.kind, expectedName === 'RangeError' ? 'range_error' : 'syntax', item.id)
          if (typeof item.expectedError !== 'string') assert.equal(phase, item.expectedError.phase, item.id)
          return ['syntax','range_error'].includes(result.kind) && result.message === expectedError
        }, item.id)
      } finally { session?.free() }
      errorCases++
      continue
    }
    const session = new facade.CompilerSession(JSON.stringify({
      input: { css: item.input, entry: item.entry ?? 'memory:input.css' }, assets: item.assets ?? [], limits: caseLimits, owner: null,
    }))
    try {
      if (item.expectedSources) {
        assert.deepEqual(JSON.parse(session.source_metadata()), {
          sources: item.expectedSources, root: item.expectedRoot,
        }, `${item.id}: source metadata`)
        sourceMetadataCases++
      }
      if (item.buildSteps) {
        incrementalCases++
        for (const step of item.buildSteps) {
          const result = JSON.parse(session.prepare(JSON.stringify(step.candidates)))
          assert.equal(result.report.css, step.expectedCss, `${item.id}: incremental step`)
          assert.equal(result.report.unsupported.length, 0, `${item.id}: pending candidate`)
          session.commit(result.token)
          if (item.expectedSources) assert.deepEqual(JSON.parse(session.source_metadata()), {
            sources: item.expectedSources, root: item.expectedRoot,
          }, `${item.id}: metadata after incremental commit`)
          incrementalComparisons++
        }
        sliceCases++
        continue
      }
      const result = JSON.parse(session.prepare(JSON.stringify(item.candidates)))
      const implemented = item.expectedProperties?.length > 0 && item.expectedProperties.every(property =>
        ['font-size', 'line-height', 'letter-spacing', 'font-weight', 'outline-style', 'outline-width', 'stroke-width', 'text-decoration-thickness', 'background-image', 'background-position', 'background-size'].includes(property) || /^border(?:-(?:inline|block)(?:-(?:start|end))?|-(?:top|right|bottom|left))?-(?:style|width)$/.test(property))
      if (item.branch === 'other_interpretation' && !implemented) {
        assert.equal(result.report.css, '', item.id)
        assert.deepEqual(result.report.admitted, [], `${item.id}: admitted candidate`)
        assert.equal(result.report.unsupported.length, item.candidates.length, item.id)
        assert.ok(result.report.unsupported.every(item => item.kind === 'pending_family'), item.id)
        pendingBoundaryCases++
      } else {
        assert.equal(result.report.css, item.expectedCss, item.id)
        assert.equal(result.report.unsupported.length, 0, `${item.id}: pending candidate`)
        sliceCases++
      }
      session.commit(result.token)
      if (item.expectedSources) assert.deepEqual(JSON.parse(session.source_metadata()), {
        sources: item.expectedSources, root: item.expectedRoot,
      }, `${item.id}: metadata after commit`)
    } finally { session.free() }
  }
}
console.log(JSON.stringify({ compilerSmoke: 'passed', scannerEnabled: false,
  officialNesting: 'passed',
  cssFunctionBindingsAndLimits: 'passed',
  customUtilityBindingsAndLimits: 'passed',
  customVariantBindingsAndLimits: 'passed',
  legacyCacheLimitsAndRecovery: 'passed',
  sliceCases,
  incrementalComparisons,
  incrementalCases,
  cssComparisons: sliceCases - incrementalCases + incrementalComparisons,
  pendingBoundaryCases,
  pendingFeatureCases,
  utf16OutputCases,
  sourceMetadataCases,
  errorCases,
  variantParseCases,
  variantParseNullCases,
  variantReusedCases,
  variantParserLimitCases,
  wasmBytes: bytes.length, imports: WebAssembly.Module.imports(module).map(item => item.name),
  sharedMemory,
  fullConformance: 'pending' }))
