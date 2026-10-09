import assert from 'node:assert/strict'
import { createHash } from 'node:crypto'
import { readFile, writeFile } from 'node:fs/promises'
import { createRequire } from 'node:module'
import { dirname, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'

const directory = dirname(fileURLToPath(import.meta.url))
const oracle = createRequire(resolve(directory, 'oracle/package.json'))
const tailwind = oracle('tailwindcss')
assert.equal(oracle('tailwindcss/package.json').version, '4.1.13')
const sha256 = text => createHash('sha256').update(text).digest('hex')
const edgeRoots = ['x', 'y', 't', 'r', 'b', 'l'].flatMap(edge => ['from', 'to'].map(stop => `mask-${edge}-${stop}`))
const gradientStopRoots = ['linear', 'radial', 'conic'].flatMap(kind => ['from', 'to'].map(stop => `mask-${kind}-${stop}`))
const stopRoots = [...edgeRoots, ...gradientStopRoots]
const constructors = ['mask-linear', '-mask-linear', 'mask-radial', 'mask-radial-at', 'mask-conic', '-mask-conic']
const roots = ['mask', 'mask-size', 'mask-position', ...edgeRoots, ...constructors, ...gradientStopRoots]
const declarations = '--spacing: .25rem; --color-brand: #123456; --background-color-brand: #234567; --color-alias: var(--color-brand); --color-2: #345678; --color-1_5: #456789; --color-unknown: var(--missing); --opacity-muted: .35; --mask: url(mask.svg); --mask-card: url(card.svg); --mask-size: 13px; --mask-size-card: 17px; --mask-position: top; --mask-position-card: left; --mask-linear: 19deg; --mask-linear-card: 23deg; --mask-radial: 29px; --mask-radial-card: 31px; --mask-radial-at-card: left; --mask-conic: 37deg; --mask-conic-card: 41deg; --gradient-color-stop-positions-card: 43%; --gradient-color-stop-positions-50: 47%;'
const themes = {
  empty: '', base: `@theme { ${declarations} }`, inline: `@theme inline { ${declarations} }`, reference: `@theme reference { ${declarations} }`, default: `@theme default { ${declarations} }`,
  reset: `@theme { ${declarations} } @theme { --color-*: initial; --background-color-*: initial; --opacity-*: initial; --spacing: initial; }`,
  defaultPriority: `@theme { ${declarations} } @theme default { --spacing: 1rem; --color-brand: red; --background-color-brand: blue; --opacity-muted: .75; }`,
  noSpacing: '@theme { --color-brand: #123456; --background-color-brand: #234567; --opacity-muted: .35; }',
  collision: `@theme { ${declarations} --color-current: red; --color-inherit: blue; --color-transparent: green; --color-0: yellow; --color-50: orange; --color-50\\%: purple; --background-color-2: red; --background-color-alias: var(--color-brand); --background-color-brand: blue; }`,
  staticTheme: `@theme static { ${declarations} }`,
  prefix: `@theme prefix(tw) { ${declarations} }`, prefixInline: `@theme prefix(tw) inline { ${declarations} }`, prefixReference: `@theme prefix(tw) reference { ${declarations} }`,
}
const values = ['', 'card', 'brand', 'alias', 'unknown', 'current', 'inherit', 'transparent', 'none', 'auto', 'cover', 'contain', 'top', 'center', '0', '0.25', '1.5', '2', '45', '50', '125', '01', '1.0', '.5', '-1', '1e+21', 'Infinity', '0%', '2%', '50%', '125%', '0.25%', '01%', '-1%', '[13px]', '[50%]', '[0.25%]', '[13px_17px]', '[var(--probe)]', '(--probe)', '(color:--probe)', '(length:--probe)', '[color:var(--probe)]', '[length:var(--probe)]', '[percentage:var(--probe)]', '[nonsense:var(--probe)]', '[size:13px_17px]', '[position:13px_17px]', '[bg-size:13px_17px]', '[image:var(--probe)]', '[url:var(--probe)]', '[red]', '[#123456]', '[color:13px]', '[length:red]', '[rgb(calc(1+2)_0_0)]', '[calc(1px+2px)]', '[0]', '[cover]', '[contain]', '[center]', '[13deg]', '[3rad]', '[left_13px_top_17px]', '[url(mask.svg)]', '[linear-gradient(black,transparent)]', '[image-set(url(a.svg)_1x,url(b.svg)_2x)]', '[96px_at_top,black_40%,transparent_80%,black_90%]']
const modifiers = ['0', '0.25', '1.5', '25', '50', '100', '125', '01', '-25', 'muted', 'unknown', '[.35]', '[35%]', '[var(--opacity-muted)]', '(--opacity-muted)', '[calc(.5*2)]', '[Infinity]', '[]', '[;]', '(bad)']

export async function maskFixtures() {
  const cases = []
  async function add(id, mode, candidates) {
    const input = `${themes[mode]} @tailwind utilities;`.trim()
    const compiler = await tailwind.compile(input)
    const expectedCss = compiler.build(candidates).replace(/^\/\*! tailwindcss[^\n]*\n/, '')
    cases.push({ id, input, candidates, expectedCss, expectedCssSha256: sha256(expectedCss) })
  }
  const inventory = JSON.parse(await readFile(resolve(directory, 'inventory.json'), 'utf8'))
  const actualRoots = inventory.utilities.filter(item => item.kind === 'functional' && (item.name === 'mask' || item.name.startsWith('mask-') || item.name.startsWith('-mask-'))).map(item => item.name)
  assert.deepEqual([...roots].sort(), actualRoots.sort())
  const staticRoots = inventory.utilities.filter(item => item.kind === 'static' && item.name.startsWith('mask-')).map(item => item.name)
  assert.equal(staticRoots.length, 56)
  for (const root of roots) {
    for (const mode of ['empty', 'base', 'inline', 'reference', 'default']) for (const value of values) await add(`mask:root=${root}:value=${value || '<default>'}:${mode}`, mode, [`${root}${value ? `-${value}` : ''}`])
    for (const mode of ['reset', 'defaultPriority', 'noSpacing', 'collision']) for (const value of ['brand', 'alias', 'current', 'inherit', 'transparent', '0', '1.5', '2', '50%', 'card', '[var(--probe)]', '[color:var(--probe)]']) await add(`mask:root=${root}:value=${value}:${mode}`, mode, [`${root}-${value}`])
    for (const mode of ['prefix', 'prefixInline', 'prefixReference']) for (const candidate of [`tw:${root}-brand`, `tw:${root}-2`, `tw:${root}-[13px]`, `tw:hover:${root}-[color:var(--probe)]!`, `${root}-brand`]) await add(`mask:${candidate}:${mode}`, mode, [candidate])
    for (const value of ['1/2', '0/0', '2/3', '2/[3]', '2/(--probe)']) await add(`mask:${root}-${value}:fraction`, 'base', [`${root}-${value}`])
    if (!root.startsWith('-')) await add(`mask:-${root}:negative-boundary`, 'base', [`-${root}-brand`, `-${root}-[13px]`])
    await add(`mask:${root}:hover-important`, 'base', [`hover:${root}-[var(--probe)]!`])
  }
  for (const root of stopRoots) {
    for (const mode of ['base', 'inline', 'reference']) for (const value of ['brand', 'alias', '2', '50%', '[red]', '[color:var(--probe)]', '[13px]', '[var(--probe)]']) for (const modifier of modifiers) await add(`mask:${root}-${value}/${modifier}:${mode}:alpha`, mode, [`${root}-${value}/${modifier}`])
    for (const mode of ['base', 'noSpacing']) for (let number = 0; number <= 8; number += .25) await add(`mask:${root}-${number}:${mode}:spacing`, mode, [`${root}-${number}`])
    for (let number = 0; number <= 100; number += 5) await add(`mask:${root}-${number}%:percentage`, 'empty', [`${root}-${number}%`])
    for (const value of ['[0.0%]', '[0.25%]', '[01%]', '[-1%]', '[calc(1%+2%)]', '[percentage:var(--probe)]', '[length:13%]', '[color:13%]']) await add(`mask:${root}-${value}:percentage-type-boundary`, 'base', [`${root}-${value}`])
  }
  for (const root of roots.filter(root => !stopRoots.includes(root))) for (const value of ['', 'card', '2', '[13px]', '[var(--probe)]']) for (const modifier of modifiers) await add(`mask:${root}-${value || '<default>'}/${modifier}:non-stop-modifier`, 'base', [`${root}${value ? `-${value}` : ''}/${modifier}`])
  for (const root of staticRoots) {
    for (const mode of ['empty', 'base', 'inline', 'reference', 'staticTheme']) await add(`mask:static:${root}:${mode}`, mode, [root])
    for (const mode of ['prefix', 'prefixInline', 'prefixReference']) await add(`mask:static:tw:${root}:${mode}`, mode, [`tw:${root}`])
    for (const candidate of [`-${root}`, `${root}/foo`, `hover:${root}!`]) await add(`mask:static:${candidate}:boundary`, 'base', [candidate])
  }
  const themeCss = await readFile(resolve(directory, '../../../ref/tailwindcss/packages/tailwindcss/theme.css'), 'utf8')
  const colorTokens = [...themeCss.matchAll(/--color-([a-z0-9-]+)\s*:[^;]+;/g)]
  assert.equal(colorTokens.length, 244)
  const spacingToken = themeCss.match(/--spacing\s*:[^;]+;/)?.[0]
  assert.ok(spacingToken)
  themes.upstreamDefault = `@theme default { ${spacingToken} ${colorTokens.map(match => match[0]).join(' ')} }`
  for (const root of stopRoots) {
    for (let offset = 0; offset < colorTokens.length; offset += 32) for (const alpha of ['', '/50']) await add(`mask:${root}:upstream-default-colors:${offset}${alpha}`, 'upstreamDefault', colorTokens.slice(offset, offset + 32).map(match => `${root}-${match[1]}${alpha}`))
    for (const value of ['0', '.25', '0.25', '1.5', '2', '50%', '0.25%']) await add(`mask:${root}-${value}:upstream-default-spacing`, 'upstreamDefault', [`${root}-${value}`])
  }
  await add('mask:all-resources-sorted-union', 'base', [...staticRoots, 'mask-x-from-brand/50', 'mask-y-to-50%', 'mask-t-from-2', 'mask-r-to-[13px]', 'mask-b-from-[color:var(--probe)]', 'mask-l-to-[var(--probe)]', 'mask-linear-45', '-mask-conic-45', 'mask-linear-from-brand', 'mask-linear-to-50%', 'mask-radial-[40px_80px]', 'mask-radial-at-[top_left]', 'mask-radial-from-2', 'mask-radial-to-brand/50', 'mask-conic-from-[13px]', 'mask-conic-to-[red]/[var(--opacity-muted)]', 'hover:mask-x-from-brand/50!'])
  await add('mask:arbitrary-source-inference-union', 'reference', ['mask-[50%]', 'mask-[13px]', 'mask-[13px_17px]', 'mask-[rgb(calc(1+2)_0_0)]', 'mask-[var(--probe)]', 'mask-[url(mask.svg)]', 'mask-size-[13px_17px]', 'mask-position-[13px]', 'mask-linear-from-[rgb(calc(1+2)_0_0)]', 'mask-linear-from-[nonsense:red]', '-mask-linear-[calc(1deg+2deg)]'])
  const ids = new Set(); for (const item of cases) { assert.ok(!ids.has(item.id), `duplicate ${item.id}`); ids.add(item.id) }
  const sources = ['packages/tailwindcss/src/utilities.ts', 'packages/tailwindcss/src/utilities.test.ts', 'packages/tailwindcss/src/theme.ts', 'packages/tailwindcss/src/utils/infer-data-type.ts', 'packages/tailwindcss/src/utils/math-operators.ts', 'packages/tailwindcss/src/property-order.ts', 'packages/tailwindcss/src/ast.ts'].map(path => { const source = inventory.sources.find(item => item.path === path); assert.ok(source?.sha256, path); return { path, sha256: source.sha256 } })
  sources.push({ path: 'packages/tailwindcss/theme.css', sha256: sha256(themeCss) })
  return { schemaVersion: 1, sourceCommit: '1334c99db8fd26c8ea065375dd9259800863f072', oracleVersion: '4.1.13', functionalRoots: roots, staticRegressionRoots: staticRoots, fullFamilyConformance: 'pending', defaultTheme: 'explicit fixed CSS theme subset: all 244 colors and spacing; no implicit full theme', sources, cases }
}

export async function checkMaskFixtures() {
  const actual = await maskFixtures()
  assert.deepEqual(actual, JSON.parse(await readFile(resolve(directory, 'masks-fixtures.json'), 'utf8')))
  return { functionalRoots: roots.length, staticRegressionRoots: 56, cases: actual.cases.length }
}
if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  if (process.argv.includes('--write')) { const actual = await maskFixtures(); await writeFile(resolve(directory, 'masks-fixtures.json'), JSON.stringify(actual, null, 2) + '\n'); console.log(JSON.stringify({ functionalRoots: roots.length, cases: actual.cases.length })) }
  else console.log(JSON.stringify(await checkMaskFixtures()))
}
