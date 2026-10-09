import assert from 'node:assert/strict'
import { createHash } from 'node:crypto'
import { readFile, readdir, writeFile } from 'node:fs/promises'
import { createRequire } from 'node:module'
import { dirname, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'

const directory = dirname(fileURLToPath(import.meta.url))
const repository = resolve(directory, '../../..')
const oracle = createRequire(resolve(directory, 'oracle/package.json'))
const { compile, __unstable__loadDesignSystem: loadDesignSystem } = oracle('tailwindcss')
assert.equal(oracle('tailwindcss/package.json').version, '4.1.13')
const sha256 = value => createHash('sha256').update(typeof value === 'string' ? value : JSON.stringify(value)).digest('hex')
const banner = css => css.replace(/^\/\*! tailwindcss[^\n]*\n/, '')
const base = '@tailwind utilities;'
const declarations = '--breakpoint-card: 40rem; --breakpoint-short: 20rem; --breakpoint-long: 80rem; --breakpoint-zero: 0; --breakpoint-pixel: 640px; --breakpoint-decimal: 40.9rem; --breakpoint-call: calc(100rem - 1px); --breakpoint-clamp: clamp(20rem, 30vw, 60rem); --breakpoint-variable: var(--screen); --container-card: 23rem; --container-short: 13rem; --container-long: 47rem; --container-pixel: 768px; --container-zero: 0; --container-decimal: 23.9rem; --container-call: calc(100rem - 2px); --container-variable: var(--container);'
const collisions = '--breakpoint-hover: 11rem; --breakpoint-group: 12rem; --breakpoint-group-hover: 13rem; --breakpoint-aria: 14rem; --breakpoint-aria-checked: 15rem; --breakpoint-max: 16rem; --breakpoint-min: 17rem; --breakpoint-not: 18rem; --breakpoint-nth-last: 19rem; --breakpoint-dark: 21rem; --container-min-card: 22rem; --container-max-card: 24rem; --container-aria: 26rem;'
const inputModes = {
  base,
  themed: `@theme { ${declarations} }\n${base}`,
  inline: `@theme inline { ${declarations} }\n${base}`,
  reference: `@theme reference { ${declarations} }\n${base}`,
  static: `@theme static { ${declarations} }\n${base}`,
  prefix: `@theme prefix(tw) { ${declarations} }\n${base}`,
  prefixReference: `@theme prefix(tw) reference { ${declarations} }\n${base}`,
  collision: `@theme { ${declarations} ${collisions} }\n${base}`,
  collisionChildren: `@theme { ${declarations} --breakpoint-hover: 11rem; --breakpoint-aria: 14rem; --breakpoint-aria-checked: 15rem; --breakpoint-nth-last: 19rem; --breakpoint-dark: 21rem; }\n${base}`,
  reset: `@theme { ${declarations} } @theme { --breakpoint-*: initial; --container-*: initial; }\n${base}`,
  defaultPriority: `@theme { ${declarations} } @theme default { --breakpoint-card: 99rem; --container-card: 101rem; }\n${base}`,
}
const functionalValue = (root, value) => `${root}${value ? root === '@' ? value : `-${value}` : ''}`
const modifierValues = ['', 'card', 'card/', 'card//tail', 'card/other', '/tail', '[card_name]', '[var(--name)]', '(--name)', '(--name,fallback)', '(length:--name)', '(bad)', '[]', '[_]', '()', '[;]', '[a/b]', '[😀]', '[\ue000]']
const commonValues = ['', '-', 'probe', '[]', '[_]', '[ ]', '()', '(--probe)', '(--probe,fallback)', '(bad)', '(length:--probe)', '[var(--probe)]', '[length:var(--probe)]', '[foo;bar]', '[foo}bar]', 'foo-[bar]', 'foo[bar]', 'foo-(--probe)', 'foo(--probe)', '[probe', '(--probe', 'probe]', 'probe)']
const nthValues = ['0', '2', '02', '2.0', '2.5', '-2', '1e3', '1e+21', 'Infinity', '[2n+1]', '[2n+1_of_.card]', '[odd]', '[number:2]', '[calc(1+2)]']
const attributeValues = ['checked', 'state', '[modal]', '[value=Hello_world]', '[value="Hello_world"]', "[value='Hello_world']", '[value=Hello_i]', '[value=Hello_I]', '[value=Hello_s]', '[value=Hello_S]', '[value=Hello_x]', '[value=a=b]', '[value~=a_b]', '[value_=_"a"]', '[value_^_=_"a"]', '[value=var(--probe)]', '[value=theme(--probe)]', '[value=a\\_b]', '[value="},;(["]']
const supportsValues = ['grid', '[display]', '[display:grid]', '[--probe]', '[--probe:var(--value)]', '[selector(:has(*))]', '[font-tech(color-COLRv1)]', '[font-format(opentype)]', '[var(--probe)]', '[not(display:grid)]', '[(display:grid)and(color:red)]', '[(display:grid)or(color:red)]', '[((display:grid)_and_(color:red))]', '[display:func(a,b)]', '[length:var(--probe)]']
const breakpointValues = ['card', 'short', 'long', 'zero', 'pixel', 'decimal', 'call', 'clamp', 'variable', 'missing', 'min-card', 'max-card', '[0]', '[-1rem]', '[640px]', '[40.9rem]', '[40.1rem]', '[calc(100%-1rem)]', '[calc(100%-2rem)]', '[clamp(20rem,30vw,60rem)]', '[var(--screen)]', '[length:40rem]']
const arbitrary = ['[]', '[_]', '[ ]', '[p]', '[>p]', '[+p]', '[~p]', '[&>p]', '[&:is(:hover,:focus)]', '[&:hover,&:focus]', '[&::before]', '[&:where(::before)]', '[&[data-value="},;(["]]', '[&:has(>p)]', '[@media(width>500px)]', '[@media_(width>500px)]', '[@media_screen,print]', '[@media_not_print]', '[@supports(display:grid)]', '[@supports_not_(display:grid)]', '[@container_card_(width>20rem)]', '[@container_card_not_(width>20rem)]', '[@starting-style]', '[@layer_probe]', '[@unknown_probe]', '[@media(width>500px){&:hover}]', '[&;p]', '[&}p]', '[&/*;*/:hover]', '[&:is(.a,.b)/foo]', '[p]/card']
const sourcePaths = ['packages/tailwindcss/src/variants.ts', 'packages/tailwindcss/src/variants.test.ts', 'packages/tailwindcss/src/candidate.ts', 'packages/tailwindcss/src/candidate.test.ts', 'packages/tailwindcss/src/compile.ts', 'packages/tailwindcss/src/theme.ts', 'packages/tailwindcss/src/utils/segment.ts', 'packages/tailwindcss/src/utils/decode-arbitrary-value.ts', 'packages/tailwindcss/src/utils/compare-breakpoints.ts', 'packages/tailwindcss/theme.css']

export async function variantParameterFixtures() {
  const sourceCommit = '1334c99db8fd26c8ea065375dd9259800863f072'
  const provenance = JSON.parse(await readFile(resolve(repository, 'crates/rustify-style/upstream/tailwind/PROVENANCE.json'), 'utf8'))
  assert.equal(provenance.commit, sourceCommit)
  const sources = await Promise.all(sourcePaths.map(async path => {
    const sha = sha256(await readFile(resolve(repository, 'ref/tailwindcss', path), 'utf8'))
    if (provenance.sourceDigests[path]) assert.equal(sha, provenance.sourceDigests[path], path)
    return { path, sha256: sha }
  }))
  const themeCss = await readFile(resolve(repository, 'ref/tailwindcss/packages/tailwindcss/theme.css'), 'utf8')
  const defaultBreakpoints = [...themeCss.matchAll(/--breakpoint-[a-z0-9-]+\s*:[^;]+;/g)].map(match => match[0])
  const defaultContainers = [...themeCss.matchAll(/--container-[a-z0-9-]+\s*:[^;]+;/g)].map(match => match[0])
  assert.equal(defaultBreakpoints.length, 5)
  assert.equal(defaultContainers.length, 13)
  const inputs = { ...inputModes, upstreamDefault: `@theme default { ${[...defaultBreakpoints, ...defaultContainers].join(' ')} }\n${base}` }
  const designs = new Map()
  const designFor = async input => {
    if (!designs.has(input)) designs.set(input, await loadDesignSystem(input))
    return designs.get(input)
  }
  const registries = []
  for (const [mode, input] of Object.entries(inputs)) {
    const design = await designFor(input)
    const roots = [...design.variants.entries()].map(([root, { kind, order, compounds, compoundsWith }]) => ({ root, kind, order, compounds, compoundsWith }))
    registries.push({ mode, inputSha256: sha256(input), roots, rootsSha256: sha256(roots) })
  }
  const baseRoots = registries.find(item => item.mode === 'base').roots
  const functionalRoots = baseRoots.filter(item => item.kind === 'functional').map(item => item.root)
  const compoundRoots = baseRoots.filter(item => item.kind === 'compound').map(item => item.root)
  const staticRoots = baseRoots.filter(item => item.kind === 'static').map(item => item.root)
  const parseCases = [], parseByKey = new Map(), requests = new Map()
  function request(id, input, candidates, tags = []) {
    const key = sha256([input, candidates])
    if (!requests.has(key)) requests.set(key, { id, input, candidates, tags: [...tags] })
    else for (const tag of tags) if (!requests.get(key).tags.includes(tag)) requests.get(key).tags.push(tag)
  }
  async function variant(mode, raw, tag = 'parameter') {
    const key = JSON.stringify([mode, raw])
    if (!parseByKey.has(key)) {
      const expectedVariant = (await designFor(inputs[mode])).parseVariant(raw)
      const item = { id: `variant-parse:${mode}:${raw}`, mode, raw, expectedVariant: structuredClone(expectedVariant), expectedVariantSha256: sha256(expectedVariant) }
      parseByKey.set(key, item)
      parseCases.push(item)
    }
    const prefix = mode.startsWith('prefix') ? 'tw:' : ''
    request(`variant-parameters:${mode}:${raw}`, inputs[mode], [`${prefix}${raw}:flex`], [tag])
  }
  for (const root of staticRoots) {
    for (const raw of [root, `${root}-probe`, `${root}-[]`, `${root}-(--probe)`, ...modifierValues.map(modifier => `${root}/${modifier}`)]) await variant('base', raw, 'static-no-value-or-modifier')
    request(`variant-parameters:static-existing:${root}`, base, [`${root}:w-[137px]`], ['static-existing'])
  }
  for (const root of functionalRoots) {
    const values = [...new Set([...commonValues, ...(root.startsWith('nth') ? nthValues : root === 'aria' || root === 'data' ? attributeValues : root === 'supports' ? supportsValues : breakpointValues)])]
    for (const value of values) for (const mode of ['base', 'themed']) await variant(mode, functionalValue(root, value), 'functional-value')
    const modifierBranches = root.startsWith('nth') ? ['2', '[2n+1]', '(--probe)'] : root === 'aria' || root === 'data' ? ['state', '[state=open]', '(--probe)'] : root === 'supports' ? ['grid', '[selector(:has(*))]', '(--probe)'] : ['card', '[40rem]', '(--probe)']
    for (const value of ['', ...modifierBranches]) for (const modifier of modifierValues) await variant('themed', `${functionalValue(root, value)}/${modifier}`, 'functional-modifier')
    for (const mode of ['inline', 'reference', 'static', 'prefix', 'prefixReference', 'reset', 'defaultPriority']) for (const value of ['card', 'variable', '[var(--probe)]', '(--probe)', '[length:40rem]']) await variant(mode, functionalValue(root, value), 'theme-options')
  }
  const compoundChildren = [...baseRoots.map(item => item.root), ...arbitrary, 'aria-checked', 'data-[state=open]', 'nth-2', 'nth-[2n+1]', 'supports-[display:grid]', 'min-card', 'max-card', '@card', '@min-card', '@max-card', 'group-hover', 'not-hover', 'group-not-hover', 'not-not-hover', 'group-group-hover', 'has-has-[>p]']
  for (const root of compoundRoots) {
    for (const raw of [root, `${root}-`, `${root}/card`, ...compoundChildren.map(child => `${root}-${child}`)]) await variant('themed', raw, 'compound-compatibility-and-nesting')
    for (const child of ['hover', 'focus', 'data-[state=open]', '[&:hover,&:focus]', '[>p]', 'not-hover', 'group-hover']) for (const modifier of modifierValues) await variant('themed', `${root}-${child}/${modifier}`, 'compound-modifier')
    for (const mode of ['prefix', 'prefixReference']) for (const raw of [`${root}-hover/card`, `${root}-data-[state=open]/[card_name]`, `${root}-hover/`, `${root}-hover/card/`, `${root}-hover/card//tail`]) await variant(mode, raw, 'prefixed-relation')
  }
  for (const outer of compoundRoots) for (const inner of compoundRoots) for (const child of ['focus', '[>p]', 'supports-grid', '@card']) await variant('themed', `${outer}-${inner}-${child}`, 'nested-style-relative-and-conditional-compatibility')
  for (const raw of ['not-not-not-focus', 'group-not-not-focus', 'not-group-not-focus', 'peer-has-not-focus', 'in-group-has-focus', 'has-peer-not-focus', 'not-not-not-@card', 'not-group-group-focus', 'group-group-group-focus']) await variant('themed', raw, 'deep-compound-boundaries')
  for (const raw of arbitrary) await variant('base', raw, 'arbitrary-selector-and-conditional')
  for (const raw of ['', '-', '--', '/', 'missing', 'missing-[probe]', '[', ']', '[&:hover', '&:hover]', '@missing/probe/other', 'tw:hover']) await variant('base', raw, 'unknown-and-partial-parser-boundaries')
  for (const [name, whitespace] of [['space', ' '], ['tab', '\t'], ['u0085', '\u0085'], ['u2028', '\u2028'], ['u2029', '\u2029'], ['nbsp', '\u00a0'], ['bom', '\ufeff']]) {
    for (const raw of [`[${whitespace}]`, `[${whitespace}&:hover${whitespace}]`, `[${whitespace}>p]`, `aria-[${whitespace}]`, `data-[${whitespace}state${whitespace}]`, `nth-[${whitespace}]`, `supports-[${whitespace}]`, `min-[${whitespace}]`, `@min-[${whitespace}]`, `group-hover/[${whitespace}]`, `peer-focus/[${whitespace}card${whitespace}]`, `group-hover/(--name${whitespace})`, `data-(--probe${whitespace})`]) await variant('themed', raw, `js-trim-and-preserved-whitespace:${name}`)
  }
  for (const registry of registries) for (const { root } of registry.roots) await variant(registry.mode, root, 'all-registry-roots')
  for (const declaration of defaultBreakpoints) {
    const name = declaration.match(/--breakpoint-([^:]+):/)[1]
    for (const raw of [name, `min-${name}`, `max-${name}`, `min-${name}/card`, `not-${name}`, `group-${name}`]) await variant('upstreamDefault', raw, 'all-default-breakpoint-tokens')
  }
  for (const declaration of defaultContainers) {
    const name = declaration.match(/--container-([^:]+):/)[1]
    for (const root of ['@', '@min', '@max']) for (const modifier of ['', '/card', '/[card_name]', '/']) await variant('upstreamDefault', `${functionalValue(root, name)}${modifier}`, 'all-default-container-tokens')
  }
  request('variant-parameters:all-default-token-union', inputs.upstreamDefault, [...defaultBreakpoints.flatMap(declaration => { const name = declaration.match(/--breakpoint-([^:]+):/)[1]; return [name, `min-${name}`, `max-${name}`] }), ...defaultContainers.flatMap(declaration => { const name = declaration.match(/--container-([^:]+):/)[1]; return ['@', '@min', '@max'].map(root => `${functionalValue(root, name)}/card`) })].map(raw => `${raw}:flex`), ['all-default-token-ordering'])
  for (const raw of ['hover', 'hover/card', 'hover-probe', 'group', 'group-hover', 'group-hover-probe', 'not', 'not-hover', 'aria', 'aria-checked', 'aria-checked/card', 'nth-last', 'nth-last-2', 'min', 'min-card', 'max', 'max-card', 'dark', 'not-dark', 'group-dark', '@min-card', '@max-card']) await variant('collision', raw, 'theme-root-collision')
  // Keep compound parents intact while theme registration changes child rule metadata.
  for (const root of compoundRoots) for (const child of ['hover', 'aria', 'aria-checked', 'nth-last', 'nth-last-2', 'dark', 'supports-grid', 'group-hover']) await variant('collisionChildren', `${root}-${child}`, 'theme-child-compounds-and-inherited-order')
  // Source compile/apply rejection is distinct from parseVariant accepting a shape.
  for (const raw of ['hover', 'group-hover', 'group-before', 'not-before', 'not-hover/card', 'group', 'in', 'has-[>p]', 'group-[>p]', 'not-[@media_screen,print]', 'not-[@container_card_not_(width>20rem)]', 'nth', 'nth-2', 'nth-02', 'nth-[number:2]', 'data-[]', 'supports', 'supports-[display:grid]', '@min-card', '@card/', '@card/card/', '@card/card//tail', 'missing']) {
    request(`variant-parameters:authored:${raw}`, `@theme { ${declarations} }\n.probe { @variant ${raw} { color: red; } }\n${base}`, [], ['authored-variant-error-phase'])
    request(`variant-parameters:apply:${raw}`, `@theme { ${declarations} }\n.probe { @apply ${raw}:flex; }\n${base}`, [], ['apply-error-phase'])
  }
  for (const mode of ['base', 'themed', 'collision', 'collisionChildren', 'prefix', 'upstreamDefault']) {
    const prefix = mode === 'prefix' ? 'tw:' : ''
    const raws = ['data-z', 'data-a', 'data-[z]', 'data-[a]', 'data-[a_b]', 'data-[a ]', 'data-[😀]', 'data-[\ue000]', 'aria-z', 'aria-a', 'nth-2', 'nth-10', 'nth-[2n+1]', 'supports-grid', 'supports-[display:grid]', 'group-hover', 'group-hover/a', 'group-hover/z', 'group-hover/[😀]', 'group-hover/[\ue000]', 'not-hover', 'group-not-focus', 'peer-focus', 'in-focus', 'has-[>p]', 'min-card', 'max-card', '@card', '@min-card', '@max-card', '@card/a', '@card/z', 'min-[40.9rem]', 'min-[40.1rem]', 'max-[40.9rem]', 'max-[40.1rem]', 'min-[640px]', 'min-[calc(100%-1rem)]', 'min-[calc(100%-2rem)]', '@min-[23.9rem]', '@min-[23.1rem]', '@max-[23.9rem]', '@max-[23.1rem]', '[&:hover]', '[&:focus]', '[😀]', '[\ue000]', 'hover:focus', 'focus:hover', 'before:hover', 'hover:before']
    const candidates = raws.map(raw => `${prefix}${raw}:flex`)
    request(`variant-parameters:union:${mode}`, inputs[mode], candidates, ['ordering-union'])
    request(`variant-parameters:reversed-union:${mode}`, inputs[mode], [...candidates].reverse(), ['ordering-permutation'])
    for (const offset of [1, 7, 19]) request(`variant-parameters:rotated-union:${mode}:${offset}`, inputs[mode], [...candidates.slice(offset), ...candidates.slice(0, offset)], ['ordering-permutation'])
    if (prefix) request('variant-parameters:prefix-position', inputs[mode], ['group-hover:tw:flex', 'hover:tw:flex', 'tw:group-hover:flex', 'tw:hover:flex', 'tw:tw:hover:flex', 'hover:flex'], ['prefix-position'])
  }
  for (const mode of ['themed', 'prefix']) for (const raw of ['min-card', 'max-card', '@card', 'group-hover/card', 'not-hover']) {
    const prefix = mode === 'prefix' ? 'tw:' : ''
    request(`variant-parameters:stacked:${mode}:${raw}`, inputs[mode], [`${prefix}${raw}:hover:flex`, `${prefix}hover:${raw}:flex`, `${prefix}${raw}:before:flex!`, `${prefix}before:${raw}:flex!`], ['stacking-and-important'])
  }
  // Index only requested pairs. Existing large corpora are read one at a time.
  const existing = new Map()
  const files = (await readdir(directory)).filter(file => /-fixtures\.json$/.test(file) && file !== 'variant-parameters-fixtures.json').sort()
  for (const file of files) {
    const fixture = JSON.parse(await readFile(resolve(directory, file), 'utf8'))
    for (const item of fixture.cases ?? []) {
      if (typeof item.input !== 'string' || !Array.isArray(item.candidates) || item.assets || item.options || item.files || item.expectedError || typeof item.expectedCss !== 'string') continue
      const key = sha256([item.input, item.candidates])
      if (requests.has(key) && !existing.has(key)) existing.set(key, { file, item })
    }
  }
  const cases = [], reusedCases = []
  for (const [key, item] of requests) {
    let phase = 'compile', result
    try {
      const compiler = await compile(item.input)
      phase = 'build'
      result = { expectedCss: banner(compiler.build(item.candidates)) }
    } catch (error) {
      result = { expectedError: { phase, name: error.name, message: error.message } }
    }
    const officialResultSha256 = sha256(result)
    if (existing.has(key)) {
      const { file, item: sourceCase } = existing.get(key)
      assert.deepEqual(result, { expectedCss: sourceCase.expectedCss }, `${file}:${sourceCase.id}: official reuse mismatch`)
      reusedCases.push({ id: item.id, tags: item.tags, reference: { file, caseId: sourceCase.id, caseSha256: sha256(sourceCase) }, inputCandidatesSha256: key, officialResultSha256 })
    } else cases.push({ ...item, ...result, inputCandidatesSha256: key, officialResultSha256 })
  }
  return {
    schemaVersion: 1, sourceCommit, oracleVersion: '4.1.13', fullVariantConformance: 'pending',
    contract: 'Real fixed-version designSystem.parseVariant JSON shapes and published compile/build results; invalid parser results and apply failures remain distinct. No Rust conformance pass is implied.',
    exclusions: [ { feature: 'lone UTF-16 code units', status: 'covered_in_separate_text_matrix', fixture: 'utf16-fixtures.json' }, { feature: 'execute JavaScript plugins/configuration', status: 'out_of_scope' } ],
    defaultTheme: 'Explicit fixed CSS subset: all five breakpoints and thirteen container tokens; no implicit full theme.',
    sources, inputModes: inputs, registries, functionalRoots, compoundRoots, staticRoots, parseCases, cases, reusedCases,
  }
}

const counts = fixture => ({ registries: fixture.registries.length, baseRegistryRoots: fixture.registries.find(item => item.mode === 'base').roots.length, staticRoots: fixture.staticRoots.length, functionalRoots: fixture.functionalRoots.length, compoundRoots: fixture.compoundRoots.length, parseCases: fixture.parseCases.length, parseNull: fixture.parseCases.filter(item => item.expectedVariant === null).length, cssCases: fixture.cases.filter(item => item.expectedCss !== undefined).length, errors: fixture.cases.filter(item => item.expectedError).length, compileErrors: fixture.cases.filter(item => item.expectedError?.phase === 'compile').length, buildErrors: fixture.cases.filter(item => item.expectedError?.phase === 'build').length, reusedCases: fixture.reusedCases.length })
export async function checkVariantParameterFixtures() {
  const actual = await variantParameterFixtures()
  assert.deepEqual(actual, JSON.parse(await readFile(resolve(directory, 'variant-parameters-fixtures.json'), 'utf8')))
  return counts(actual)
}
if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  if (process.argv.includes('--write')) { const actual = await variantParameterFixtures(); await writeFile(resolve(directory, 'variant-parameters-fixtures.json'), JSON.stringify(actual, null, 2) + '\n'); console.log(JSON.stringify(counts(actual))) }
  else console.log(JSON.stringify(await checkVariantParameterFixtures()))
}
