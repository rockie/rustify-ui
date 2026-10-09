import assert from 'node:assert/strict'
import { createHash } from 'node:crypto'
import { readFile, writeFile } from 'node:fs/promises'
import { createRequire } from 'node:module'
import { dirname, resolve, posix } from 'node:path'
import { fileURLToPath } from 'node:url'

const directory = dirname(fileURLToPath(import.meta.url))
const repository = resolve(directory, '../../..')
const oracle = createRequire(resolve(directory, 'oracle/package.json'))
const tailwind = oracle('tailwindcss')
assert.equal(oracle('tailwindcss/package.json').version, '4.1.13')
const sourceCommit = '1334c99db8fd26c8ea065375dd9259800863f072'
const sha256 = value => createHash('sha256').update(typeof value === 'string' || Buffer.isBuffer(value) ? value : JSON.stringify(value)).digest('hex')
// JSON cannot represent bigint, undefined or shared references. Preserve their
// observable distinctions explicitly rather than silently dropping properties.
const snapshot = value => JSON.parse(JSON.stringify(value, (_, item) =>
  typeof item === 'bigint' ? { bigint: item.toString() } : item === undefined ? { undefined: true } : item))
const constants = object => Object.fromEntries(Object.entries(object).filter(([, value]) => typeof value === 'number'))
const sourcePaths = [
  'packages/tailwindcss/src/index.ts', 'packages/tailwindcss/src/index.test.ts',
  'packages/tailwindcss/src/ast.ts', 'packages/tailwindcss/src/css-parser.ts',
  'packages/tailwindcss/src/at-import.ts', 'packages/tailwindcss/src/design-system.ts',
  'packages/tailwindcss/src/intellisense.ts', 'packages/tailwindcss/src/sort.ts',
  'packages/tailwindcss/src/candidate.ts', 'packages/tailwindcss/src/compile.ts',
  'packages/tailwindcss/src/theme.ts', 'packages/tailwindcss/src/utilities.ts',
  'packages/tailwindcss/src/source-maps/source.ts',
  'packages/tailwindcss/src/source-maps/source-map.ts',
  'packages/tailwindcss/src/source-maps/source-map.test.ts',
  'packages/tailwindcss/src/source-maps/line-table.ts',
]
const declarations = '--spacing: .25rem; --color-brand: #123456; --color-alias: var(--color-brand); --breakpoint-card: 40rem; --container-card: 23rem;'
const theme = `@theme { ${declarations} }`
const batches = [[], ['flex', 'w-[137px]'], ['hover:underline', 'bg-brand/50'], ['unknown'], [], ['w-[137px]', 'flex'], ['--color-brand']]

function sourceMapSnapshot(map) {
  const sourceReferences = map.mappings.map(mapping => {
    if (!mapping.originalPosition) return null
    const index = map.sources.indexOf(mapping.originalPosition.source)
    assert.notEqual(index, -1, 'decoded mapping source is not in sources')
    return index
  })
  return { decoded: snapshot(map), sourceReferences }
}

function optionsFor(spec, calls) {
  const options = { ...spec.options }
  if (spec.assets) options.loadStylesheet = async (id, base) => {
    calls.push({ id, base })
    const resolved = posix.normalize(posix.join(base, id))
    const asset = spec.assets.find(asset => asset.id === resolved || asset.id === id)
    assert.ok(asset, `missing oracle asset ${id} from ${base}`)
    return { path: asset.path ?? asset.id, base: asset.base ?? posix.dirname(asset.id), content: asset.content }
  }
  return options
}

async function compileCase(spec) {
  const warnings = [], loadCalls = []
  const originalWarn = console.warn
  console.warn = (...args) => warnings.push(args.join(' '))
  try {
    const compiler = await tailwind.compile(spec.input, optionsFor(spec, loadCalls))
    const item = { ...spec, expectedSources: snapshot(compiler.sources), expectedRoot: compiler.root, expectedFeatures: compiler.features,
      expectedInitialSourceMap: sourceMapSnapshot(compiler.buildSourceMap()), expectedBuilds: [] }
    for (const candidates of spec.batches) {
      const css = compiler.build(candidates)
      item.expectedBuilds.push({ css, cssSha256: sha256(css), sourceMap: sourceMapSnapshot(compiler.buildSourceMap()) })
    }
    return { ...item, expectedLoadCalls: loadCalls, expectedWarnings: warnings }
  } catch (error) {
    return { ...spec, expectedError: error.message, expectedErrorName: error.name, expectedLoadCalls: loadCalls, expectedWarnings: warnings }
  } finally { console.warn = originalWarn }
}

const declaration = (property, value, important = false) => ({ kind: 'declaration', property, value, important })
const rule = (selector, nodes) => ({ kind: 'rule', selector, nodes })
const atRule = (name, params = '', nodes = []) => ({ kind: 'at-rule', name, params, nodes })

async function astCase(spec) {
  const input = structuredClone(spec.inputAst), loadCalls = []
  const compiler = await tailwind.compileAst(input, optionsFor(spec, loadCalls))
  const result = { ...spec, inputAst: snapshot(spec.inputAst), expectedSources: snapshot(compiler.sources), expectedRoot: compiler.root, expectedFeatures: compiler.features, expectedBuilds: [] }
  const outputs = []
  for (const candidates of spec.batches) {
    const ast = compiler.build(candidates)
    result.expectedBuilds.push({ ast: snapshot(ast), sameAsInput: ast === input, sameAsPrevious: outputs.length > 0 && ast === outputs.at(-1) })
    outputs.push(ast)
  }
  result.expectedLoadCalls = loadCalls
  return result
}

async function designCase(mode) {
  const prefix = mode === 'prefix' ? 'tw:' : ''
  const input = `@theme ${mode === 'prefix' ? 'prefix(tw)' : mode === 'inline' ? 'inline' : ''} { ${declarations} } @utility probe { width:13px; } @custom-variant active (&[data-active]); @tailwind utilities important;`
  const design = await tailwind.__unstable__loadDesignSystem(input)
  const candidates = ['flex', 'w-[137px]', 'bg-brand/50', 'hover:underline!', 'group-hover/card:probe', 'active:probe', '[--probe:var(--color-brand)]', '-m-2', 'w-1/2', 'missing'].map(raw => `${prefix}${raw}`)
  const candidateResults = candidates.map(raw => {
    const parsed = design.parseCandidate(raw)
    return { raw, parsed: snapshot(parsed), printed: parsed.map(candidate => design.printCandidate(candidate)), compiled: parsed.map(candidate => ({
      flagsNone: snapshot(design.compileAstNodes(candidate, 0)), flagsRespectImportant: snapshot(design.compileAstNodes(candidate, 1)),
    })) }
  })
  const variantResults = ['hover', 'group-hover/card', 'data-[state=open]', 'card', '@card', 'active', '[&>p]', 'not-hover', 'missing'].map(raw => {
    const parsed = design.parseVariant(raw)
    return { raw, parsed: snapshot(parsed), printed: parsed === null ? null : design.printVariant(parsed) }
  })
  const classList = design.getClassList()
  const variants = design.getVariants().map(({ selectors, ...metadata }) => ({ ...metadata,
    selectorProbes: [{}, { value: metadata.values[0] ?? 'card' }, { value: '[state=open]', modifier: 'named' }].map(options => ({ options, selectors: selectors(options) })),
  }))
  const classOrderInput = [...candidates].reverse().concat(candidates[0], '--color-brand')
  const classOrder = snapshot(design.getClassOrder(classOrderInput))
  const css = design.candidatesToCss(candidates)
  const variantOrder = [...design.getVariantOrder()].map(([variant, order]) => ({ variant: snapshot(variant), order }))
  const resolveResults = ['--spacing', '--color-brand', '--color-brand / 50%', '--color-alias', '--missing'].flatMap(path =>
    [undefined, false, true].map(forceInline => ({ path, ...(forceInline === undefined ? {} : { forceInline }), result: snapshot(design.resolveThemeValue(path, forceInline)) })))
  const trackingDesign = await tailwind.__unstable__loadDesignSystem(input)
  const beforeTracking = snapshot([...trackingDesign.theme.entries()])
  const tracked = `color:var(${mode === 'prefix' ? '--tw-color-brand' : '--color-brand'}); width:var(${mode === 'prefix' ? '--tw-spacing' : '--spacing'}); missing:var(--missing);`
  trackingDesign.trackUsedVariables(tracked)
  const afterTracking = snapshot([...trackingDesign.theme.entries()])
  trackingDesign.trackUsedVariables(tracked)
  const repeatedTracking = snapshot([...trackingDesign.theme.entries()])
  assert.notDeepEqual(afterTracking, beforeTracking, 'tracking probe did not mark a variable')
  assert.deepEqual(repeatedTracking, afterTracking)
  return { id: `design:${mode}`, input, publicEntries: Object.keys(design).map(name => ({ name, type: typeof design[name] })),
    important: design.important, candidateResults, variantResults, classOrderInput, expectedClassOrder: classOrder,
    expectedCandidatesToCss: css, expectedInvalidCandidates: [...design.invalidCandidates],
    expectedClassList: { count: classList.length, sha256: sha256(classList), representatives: classList.filter(([name]) => /^(?:-?m-|w-|bg-brand|flex$|probe$)/.test(name)) },
    expectedVariants: variants, expectedVariantOrder: variantOrder, resolveResults, trackUsedVariables: { input: tracked, beforeTracking, afterTracking, repeatedTracking },
  }
}

async function generate() {
  const inventory = JSON.parse(await readFile(resolve(directory, 'inventory.json'), 'utf8'))
  assert.equal(inventory.baseline.commit, sourceCommit)
  const sources = await Promise.all(sourcePaths.map(async path => {
    const bytes = await readFile(resolve(repository, 'ref/tailwindcss', path))
    const actual = sha256(bytes)
    assert.equal(actual, inventory.sources.find(source => source.path === path)?.sha256, path)
    return { path, sha256: actual }
  }))
  const features = constants(tailwind.Features), polyfills = constants(tailwind.Polyfills)
  assert.deepEqual(features, { None: 0, AtApply: 1, AtImport: 2, JsPluginCompat: 4, ThemeFunction: 8, Utilities: 16, Variants: 32 })
  assert.deepEqual(polyfills, { None: 0, AtProperty: 1, ColorMix: 2, All: 3 })
  const specs = []
  const add = (id, input, options = {}, assets, buildBatches = batches) => specs.push({ id, input, options, ...(assets ? { assets } : {}), batches: buildBatches })
  for (const base of [undefined, '', '/discover', 'relative/root']) for (const from of [undefined, '', '/maps/input.css', 'input.css']) {
    const options = { ...(base === undefined ? {} : { base }), ...(from === undefined ? {} : { from }) }
    add(`options:base=${base ?? '<omitted>'}:from=${from ?? '<omitted>'}`, '@source "./src/**"; @tailwind utilities source("./root");', options)
  }
  const assets = [
    { id: '/discover/pkg.css', path: '/locations/包😀.css', base: '/virtual/nested', content: '@source "./inside/**"; @import "leaf.css"; @utility imported { color:red; }' },
    { id: '/virtual/nested/leaf.css', path: '/locations/leaf.css', base: '/discovery/leaf', content: '@theme { --color-brand:#123456; } @utility leaf { width:17px; }' },
  ]
  for (const from of [undefined, '/maps/input.css']) for (const directive of ['@import', '@reference']) {
    add(`options:loader:${directive}:${from ?? '<omitted>'}`, `${directive} "pkg.css"; @source "./host/**"; @tailwind utilities source("./root");`, { base: '/discover', ...(from ? { from } : {}) }, assets, [[], ['imported', 'leaf', 'bg-brand'], [], ['imported']])
  }
  const featureCases = [
    ['none', '.a { color:red; }', 0], ['utilities', '@tailwind utilities;', 16],
    ['apply', '.a { @apply underline; }', 1],
    ['theme', `${theme} .a { color:theme(--color-brand); }`, 8],
    ['variant', '.a { @variant dark { color:red; } }', 32],
    ['legacy-variant', '@variant dark (&:is(.dark, .dark *));', 0],
    ['custom-variant-definition', '@custom-variant active (&[data-active]);', 0],
    ['theme-only', theme, 0],
  ]
  for (const [id, input, expectedMask] of featureCases) {
    add(`features:${id}`, input)
    specs.at(-1).assertFeatures = expectedMask
  }
  add('features:import', '@import "pkg.css";', { base: '/discover' }, assets)
  add('features:reference-utilities', '@reference "utilities.css";', { base: '/discover' }, [{ id: '/discover/utilities.css', content: '@tailwind utilities;' }])
  specs.at(-1).assertFeatures = 2
  add('features:combined', '@import "theme.css"; .a { color:theme(--color-brand); @apply underline; @variant hover { color:red; } } @tailwind utilities;', { base: '/discover' }, [{ id: '/discover/theme.css', content: theme }])
  specs.at(-1).assertFeatures = 1 | 2 | 8 | 16 | 32
  const polyfillInputs = {
    authored: '@property --probe { syntax:"<color>"; inherits:false; initial-value:red; } .a { color:color-mix(in oklab, currentColor 50%, transparent); } @tailwind utilities;',
    generated: `${theme} @tailwind utilities;`,
    custom: `${theme} @utility probe { @property --custom { syntax:"<length>"; inherits:true; initial-value:13px; } color:color-mix(in oklab, var(--color-brand) 50%, transparent); } @tailwind utilities;`,
  }
  for (const [name, input] of Object.entries(polyfillInputs)) for (const [flag, polyfill] of Object.entries(polyfills))
    add(`polyfills:${name}:${flag}`, input, { polyfills: polyfill, from: '/maps/polyfills.css' }, undefined, [[], ['ring-[3px]', 'bg-brand/50', 'probe'], [], ['hover:probe!']])
  const mapInputs = {
    authored: '.a { color:red; --multiline: one\n two; }',
    unicode: '/*! 中文😀\né */\n.卡片😀 { content:"é😀"; --字: "😀"; }\n@tailwind utilities;',
    crlfBom: '\ufeff/*! license 😀 */\r\n.é { color:red; }\r\n@tailwind utilities;',
    apply: '.按钮😀 {\n @apply underline;\n &:hover { @apply flex; }\n}\n@tailwind utilities;',
    customUtility: '@utility probe {\n width:13px;\n &:hover { color:red; }\n}\n@tailwind utilities;',
    customVariant: '@custom-variant active {\n &[data-active] { @slot; }\n}\n@utility probe { width:13px; }\n@tailwind utilities;',
    dependency: '@theme { --color-brand:#123456; --animate-probe:probe 1s; @keyframes probe { from { opacity:0; } to { opacity:1; } } } @tailwind utilities;',
  }
  for (const [name, input] of Object.entries(mapInputs)) for (const from of [undefined, `/maps/${name}😀.css`])
    add(`source-map:${name}:${from ? 'from' : 'no-from'}`, input, from ? { from } : {}, undefined, [[], ['flex', 'probe', 'active:probe', 'bg-brand', 'animate-probe', 'before:content-["😀"]'], [], ['hover:probe!']])
  add('warnings:unsupported-bare-type', '@utility sample-* { --result: --value(*); } @tailwind utilities;', {}, undefined, [[], ['sample', 'sample-2'], [], ['sample-2']])
  add('errors:missing-stylesheet-loader', '@import "missing.css";', { base: '/discover', from: '/maps/error.css' })
  const cases = []
  for (const spec of specs) {
    const item = await compileCase(spec)
    if (spec.assertFeatures !== undefined) assert.equal(item.expectedFeatures, spec.assertFeatures, spec.id)
    if (!spec.id.startsWith('errors:')) assert.ok(!Object.hasOwn(item, 'expectedError'), `${spec.id}: ${item.expectedError}`)
    cases.push(item)
  }
  const astSpecs = [
    { id: 'ast:passthrough-empty', inputAst: [] },
    { id: 'ast:passthrough-rule', inputAst: [rule('.a', [declaration('color', 'red')])] },
    { id: 'ast:passthrough-comment-context', inputAst: [{ kind: 'comment', value: 'license 😀' }, { kind: 'context', context: { reference: false, base: '/ast' }, nodes: [rule('.a', [declaration('--probe', undefined)])] }] },
    { id: 'ast:utilities', inputAst: [atRule('@tailwind', 'utilities')] },
    { id: 'ast:apply', inputAst: [rule('.a', [atRule('@apply', 'underline')])] },
    { id: 'ast:inline-candidates', inputAst: [atRule('@source', 'inline("flex block")'), atRule('@tailwind', 'utilities')] },
    { id: 'ast:theme-variable-candidates', inputAst: [atRule('@theme', '', [declaration('--color-brand', '#123456')]), atRule('@tailwind', 'utilities')], batches: [[], ['--color-brand'], ['flex'], [], ['--color-brand']] },
    { id: 'ast:source-options', inputAst: [atRule('@source', '"./src/**"'), atRule('@tailwind', 'utilities source("./root")')], options: { base: '/discover', from: '/maps/ast.css' } },
    { id: 'ast:import', inputAst: [atRule('@import', '"pkg.css"'), atRule('@tailwind', 'utilities')], options: { base: '/discover', from: '/maps/ast.css' }, assets },
  ]
  const astCases = []
  for (const spec of astSpecs) astCases.push(await astCase({ ...spec, options: spec.options ?? {}, batches: spec.batches ?? batches }))
  const designs = []
  for (const mode of ['base', 'inline', 'prefix']) designs.push(await designCase(mode))
  const coveredMethods = ['candidatesToCss', 'getClassOrder', 'getClassList', 'getVariants', 'parseCandidate', 'parseVariant', 'compileAstNodes', 'printCandidate', 'printVariant', 'getVariantOrder', 'resolveThemeValue', 'trackUsedVariables']
  for (const design of designs) assert.deepEqual(design.publicEntries.filter(item => item.type === 'function').map(item => item.name).sort(), [...coveredMethods].sort())
  return { schemaVersion: 1, sourceCommit, oracleVersion: '4.1.13', oracleNodeEnv: 'production', implementationStatus: 'pending', fullApiConformance: 'pending',
    normalization: 'Preserve complete official CSS including license banner, AST property presence, decoded map fields/order and map source identity indexes; bigint/undefined use explicit JSON markers. No function is serialized.',
    sources, publicExports: Object.keys(tailwind), features, polyfills,
    excluded: ['JavaScript plugin/config execution; JsPluginCompat constant is retained but user JavaScript is not executed'],
    designSystemMethods: coveredMethods, compileCases: cases, compileAstCases: astCases, designSystemCases: designs,
  }
}

export async function publicApiFixtures() {
  const previous = process.env.NODE_ENV
  process.env.NODE_ENV = 'production'
  try { return await generate() }
  finally { if (previous === undefined) delete process.env.NODE_ENV; else process.env.NODE_ENV = previous }
}

function counts(fixture) {
  return { compileCases: fixture.compileCases.length, compileBuilds: fixture.compileCases.reduce((count, item) => count + (item.expectedBuilds?.length ?? 0), 0),
    errorCases: fixture.compileCases.filter(item => Object.hasOwn(item, 'expectedError')).length,
    warningCases: fixture.compileCases.filter(item => item.expectedWarnings.length > 0).length,
    compileAstCases: fixture.compileAstCases.length, compileAstBuilds: fixture.compileAstCases.reduce((count, item) => count + item.expectedBuilds.length, 0),
    designSystemCases: fixture.designSystemCases.length, designSystemMethods: fixture.designSystemMethods.length, implementationStatus: fixture.implementationStatus }
}

export async function checkPublicApiFixtures() {
  const actual = await publicApiFixtures()
  assert.deepEqual(actual, JSON.parse(await readFile(resolve(directory, 'public-api-fixtures.json'), 'utf8')))
  return counts(actual)
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  if (process.argv.includes('--write')) {
    const actual = await publicApiFixtures()
    await writeFile(resolve(directory, 'public-api-fixtures.json'), `${JSON.stringify(actual, null, 2)}\n`)
    console.log(JSON.stringify(counts(actual)))
  } else console.log(JSON.stringify(await checkPublicApiFixtures()))
}
