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
const roots = ['order', '-order', 'flex', 'grow', 'shrink', 'columns', 'object', 'flex-grow', 'flex-shrink']
const staticRoots = ['flex', 'inline-flex', 'flex-auto', 'flex-initial', 'flex-none', 'order-first', 'order-last', 'order-none', 'columns-auto', 'flex-row', 'flex-row-reverse', 'flex-col', 'flex-col-reverse', 'flex-wrap', 'flex-nowrap', 'flex-wrap-reverse', 'object-contain', 'object-cover', 'object-fill', 'object-none', 'object-scale-down', 'object-top', 'object-top-left', 'object-top-right', 'object-bottom', 'object-bottom-left', 'object-bottom-right', 'object-left', 'object-right', 'object-center', 'object-left-top', 'object-right-top', 'object-left-bottom', 'object-right-bottom']
const declarations = '--order-card: 7; --order-alias: var(--order-card); --order-1_5: 13; --order-1\\/2: 17; --columns-card: 19rem; --columns-alias: var(--columns-card); --columns-1_5: 23rem; --columns-1\\/2: 29rem; --container-card: 31rem; --container-container: 37rem; --container-alias: 41rem; --container-1_5: 43rem; --object-position-card: 13% 17%; --object-position-alias: var(--object-position-card); --object-position-1_5: left 19px top 23px; --object-position-1\\/2: right 29px bottom 31px; --grow: 7; --grow-card: 9; --shrink: 11; --shrink-card: 13; --flex-card: 2 1 17px; --flex: 3 1 19px; --flex-grow-card: 17; --flex-shrink-card: 19; --spacing: .25rem; --flex-basis-card: 47px;'
const collision = '--order-first: 53; --order-last: 59; --order-none: 61; --columns-auto: 67rem; --container-auto: 71rem; --object-position-contain: 3% 7%; --object-position-cover: 11% 13%; --object-position-fill: 17% 19%; --object-position-none: 23% 29%; --object-position-scale-down: 31% 37%; --object-position-top: 41% 43%; --object-position-top-left: 47% 53%; --object-position-center: 59% 61%; --object-position-left-top: 67% 71%; --flex-auto: 5 7 13px; --flex-initial: 7 11 17px; --flex-none: 11 13 19px;'
const themes = {
  empty: '', base: `@theme { ${declarations} }`, inline: `@theme inline { ${declarations} }`, reference: `@theme reference { ${declarations} }`, default: `@theme default { ${declarations} }`,
  bare: `@theme { ${declarations} --order: 73; --columns: 79rem; --container: 83rem; --object-position: left 89px top 97px; }`,
  reset: `@theme { ${declarations} } @theme { --order-*: initial; --columns-*: initial; --container-*: initial; --object-*: initial; }`,
  containerOnly: '@theme { --container: 103rem; --container-card: 107rem; --container-alias: 109rem; --container-1_5: 113rem; --container-1\\/2: 127rem; }',
  defaultPriority: `@theme { ${declarations} } @theme default { --order-card: 131; --columns-card: 137rem; --container-card: 139rem; --object-position-card: 149% 151%; }`,
  collision: `@theme { ${declarations} ${collision} }`, staticTheme: `@theme static { ${declarations} ${collision} }`,
  prefix: `@theme prefix(tw) { ${declarations} ${collision} }`, prefixInline: `@theme prefix(tw) inline { ${declarations} ${collision} }`, prefixReference: `@theme prefix(tw) reference { ${declarations} ${collision} }`,
}
const values = ['', 'card', 'alias', 'container', '1.5', '1/2', 'missing', 'auto', 'initial', 'none', 'first', 'last', 'grow', 'shrink', 'center', 'top', 'top-left', 'left-top', 'contain', 'cover', 'fill', 'scale-down', '0', '1', '12', '99', '01', '1.0', '0.5', '.5', '-1', '1e3', '1e+21', '1e21', 'Infinity', '[13]', '[13px]', '[0]', '[var(--probe)]', '(--probe)', '[integer:13]', '[length:13px]', '[number:2]', '[position:13%_17%]', '[nonsense:foo]', '[calc(1_+_2)]', '[1_1_17px]', '[13rem_3]', '[left_13px_top_17px]', '[50%_75%]', '[none]', '[inherit]']
const modifiers = ['0', '1', '2', 'foo', 'card', '[var(--probe)]', '(--probe)', '[.5]', '[]', '[;]', '(bad)', '1/foo']

export async function layoutFixtures() {
  const cases = []
  async function add(id, mode, candidates) {
    const input = `${themes[mode]} @tailwind utilities;`.trim()
    const compiler = await tailwind.compile(input)
    const expectedCss = compiler.build(candidates).replace(/^\/\*! tailwindcss[^\n]*\n/, '')
    cases.push({ id, input, candidates, expectedCss, expectedCssSha256: sha256(expectedCss) })
  }
  for (const root of roots) {
    for (const mode of ['empty', 'base', 'inline', 'reference', 'default', 'bare', 'reset', 'containerOnly', 'defaultPriority', 'collision']) for (const value of values) await add(`layout:root=${root}:value=${value || '<default>'}:${mode}`, mode, [`${root}${value ? `-${value}` : ''}`])
    for (const mode of ['base', 'bare', 'inline', 'reference', 'collision']) for (const value of ['', 'card', '1/2', '12', '[13px]', '[var(--probe)]']) for (const modifier of modifiers) await add(`layout:root=${root}:value=${value || '<default>'}/${modifier}:${mode}`, mode, [`${root}${value ? `-${value}` : ''}/${modifier}`])
    for (const mode of ['prefix', 'prefixInline', 'prefixReference']) for (const candidate of [`tw:${root}`, `tw:${root}-card`, `tw:${root}-1/2`, `tw:${root}-[13px]`, `tw:hover:${root}-card!`, `${root}-card`]) await add(`layout:${candidate}:${mode}`, mode, [candidate])
    for (let number = 0; number <= 32; number++) await add(`layout:${root}-${number}:integer`, 'empty', [`${root}-${number}`])
    for (const fraction of ['1/0', '0/0', '2/3', '0.5/2', '1/-2', 'card/2', '1/[2]', '1/(--probe)']) await add(`layout:${root}-${fraction}:fraction`, 'base', [`${root}-${fraction}`])
    await add(`layout:${root}:hover-important`, 'base', [`hover:${root}-[var(--probe)]!`])
    if (!root.startsWith('-')) await add(`layout:-${root}:negative-boundary`, 'base', [`-${root}-card`, `-${root}-[13]`])
  }
  for (const root of staticRoots) {
    for (const mode of ['empty', 'base', 'inline', 'reference', 'collision', 'staticTheme']) await add(`layout:static:${root}:${mode}`, mode, [root])
    for (const mode of ['prefix', 'prefixInline', 'prefixReference']) await add(`layout:static:tw:${root}:${mode}`, mode, [`tw:${root}`])
    for (const candidate of [`-${root}`, `${root}/foo`, `hover:${root}!`]) await add(`layout:static:${candidate}:boundary`, 'collision', [candidate])
  }
  for (const mode of ['base', 'inline', 'reference', 'prefix']) for (const value of ['card', '1/2', 'auto', 'full']) await add(`layout:basis-${value}:${mode}:existing-regression`, mode, [`${mode === 'prefix' ? 'tw:' : ''}basis-${value}`])
  const themeCss = await readFile(resolve(directory, '../../../ref/tailwindcss/packages/tailwindcss/theme.css'), 'utf8')
  const defaultColumns = [...themeCss.matchAll(/--container-[a-z0-9-]+\s*:[^;]+;/g)].map(match => match[0])
  assert.equal(defaultColumns.length, 13)
  themes.upstreamDefault = `@theme default { ${defaultColumns.join(' ')} }`
  for (const value of ['3xs', '2xs', 'xs', 'sm', 'md', 'lg', 'xl', '2xl', '3xl', '4xl', '5xl', '6xl', '7xl', '0', '1', 'auto', 'unknown']) await add(`layout:columns-${value}:upstream-default`, 'upstreamDefault', [`columns-${value}`])
  await add('layout:all-sorted-union', 'collision', [...staticRoots, ...roots.map(root => `${root}-card`), 'grow', 'shrink', 'flex-grow', 'flex-shrink', 'flex-1/2', 'columns-4', 'basis-card', 'basis-1/2', 'hover:order-card!', 'hover:object-card!'])
  await add('layout:raw-values-sorted-union', 'reference', ['flex-[1_1_17px]', 'grow-[2]', 'shrink-[3]', 'flex-grow-[4]', 'flex-shrink-[5]', '-order-[var(--probe)]', 'columns-[13rem_3]', 'object-[left_13px_top_17px]', 'basis-[29px]'])
  const ids = new Set(); for (const item of cases) { assert.ok(!ids.has(item.id), `duplicate ${item.id}`); ids.add(item.id) }
  const inventory = JSON.parse(await readFile(resolve(directory, 'inventory.json'), 'utf8'))
  const sources = ['packages/tailwindcss/src/utilities.ts', 'packages/tailwindcss/src/compat/legacy-utilities.ts', 'packages/tailwindcss/src/utilities.test.ts', 'packages/tailwindcss/src/theme.ts', 'packages/tailwindcss/src/utils/infer-data-type.ts', 'packages/tailwindcss/src/property-order.ts'].map(path => { const source = inventory.sources.find(item => item.path === path); assert.ok(source?.sha256, path); return { path, sha256: source.sha256 } })
  sources.push({ path: 'packages/tailwindcss/theme.css', sha256: sha256(themeCss) })
  return { schemaVersion: 1, sourceCommit: '1334c99db8fd26c8ea065375dd9259800863f072', oracleVersion: '4.1.13', functionalRoots: roots, staticRegressionRoots: staticRoots, existingFunctionalRegressionRoots: ['basis'], existingStaticRegressionRoots: ['basis-auto', 'basis-full'], fullFamilyConformance: 'pending', defaultTheme: 'explicit fixed CSS theme subset: thirteen container tokens; no implicit full theme', sources, cases }
}

export async function checkLayoutFixtures() {
  const actual = await layoutFixtures()
  assert.deepEqual(actual, JSON.parse(await readFile(resolve(directory, 'layout-fixtures.json'), 'utf8')))
  return { functionalRoots: roots.length, staticRegressionRoots: staticRoots.length, cases: actual.cases.length }
}
if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  if (process.argv.includes('--write')) { const actual = await layoutFixtures(); await writeFile(resolve(directory, 'layout-fixtures.json'), JSON.stringify(actual, null, 2) + '\n'); console.log(JSON.stringify({ functionalRoots: roots.length, cases: actual.cases.length })) }
  else console.log(JSON.stringify(await checkLayoutFixtures()))
}
