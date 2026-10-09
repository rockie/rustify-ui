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
const banner = css => css.replace(/^\/\*! tailwindcss[^\n]*\n/, '')
const shadowRoots = ['text-shadow', 'shadow', 'inset-shadow']
const ringRoots = ['ring', 'inset-ring', 'ring-offset']
const roots = [...shadowRoots, ...ringRoots]
const staticRoots = ['text-shadow-initial', 'shadow-initial', 'inset-shadow-initial', 'ring-inset']
const rawTokens = shadowRoots.map(root => `--${root}-card: 0 2px 3px rgb(10 20 30 / .4); --${root}-alias: var(--${root}-card); --${root}-1_5: 1px 2px blue; --${root}-multi: 0 1px red, 2px 3px var(--color-brand);`).join(' ')
const declaration = `${rawTokens} --opacity-muted: .35; --color-brand: #123456; --color-alias: var(--color-brand); --color-1_5: purple; --color-missing: var(--missing); --box-shadow-color-brand: #234567; --text-shadow-color-brand: #345678; --ring-color-brand: #456789; --ring-offset-color-brand: #56789a; --ring-width-card: 7px; --ring-width-brand: 9px; --ring-width-1_5: 11px; --ring-offset-width-card: 13px; --ring-offset-width-brand: 17px; --ring-offset-width-1_5: 19px;`
const themes = {
  empty: '', base: `@theme { ${declaration} }`, inline: `@theme inline { ${declaration} }`, reference: `@theme reference { ${declaration} }`, default: `@theme default { ${declaration} }`,
  bare: `@theme { ${declaration} ${shadowRoots.map(root => `--${root}: 0 1px red, 2px 3px var(--color-brand);`).join(' ')} --default-ring-width: 3px; --default-ring-color: var(--color-brand); }`,
  defaults: '@theme { --default-ring-width: 13px; --default-ring-color: #123456; --ring-width: 17px; --ring-color: #234567; --ring-offset-width: 19px; --ring-offset-color: #345678; }',
  reset: `@theme { ${declaration} } @theme { --shadow-*: initial; --inset-shadow-*: initial; --text-shadow-*: initial; --ring-width-*: initial; --ring-color-*: initial; --ring-offset-width-*: initial; --ring-offset-color-*: initial; }`,
  namespaceReset: `@theme { ${declaration} } @theme { --inset-*: initial; --text-*: initial; }`,
  defaultPriority: `@theme { ${declaration} } @theme default { --shadow-card: 0 1px blue; --inset-shadow-card: 0 1px green; --text-shadow-card: 0 1px yellow; --ring-width-card: 2px; --ring-offset-width-card: 3px; }`,
  collision: '@theme { --shadow-none: 0 1px red; --text-shadow-none: 0 1px red; --inset-shadow-none: 0 1px red; --shadow-inherit: 0 1px blue; --text-shadow-inherit: 0 1px blue; --inset-shadow-inherit: 0 1px blue; --shadow-current: 0 1px yellow; --text-shadow-current: 0 1px yellow; --inset-shadow-current: 0 1px yellow; --shadow-brand: 0 1px red; --inset-shadow-brand: 0 1px blue; --text-shadow-brand: 0 1px green; --box-shadow-color-brand: purple; --text-shadow-color-brand: yellow; --ring-color-brand: blue; --ring-width-brand: 23px; --ring-offset-color-brand: green; --ring-offset-width-brand: 31px; --ring-offset-color-2: purple; --color-brand: red; }',
  colorsOnly: '@theme { --color-brand: #123456; --color-card: blue; --color-1_5: purple; --opacity-muted: .35; }',
  prefix: `@theme prefix(tw) { ${declaration} }`, prefixInline: `@theme prefix(tw) inline { ${declaration} }`, prefixReference: `@theme prefix(tw) reference { ${declaration} }`,
}
const values = ['', 'card', 'alias', 'multi', 'brand', '1.5', 'missing', 'unknown', 'none', 'inherit', 'current', 'transparent', 'initial', 'inset', 'offset-card', 'color-brand', '0', '1', '2', '50', '01', '1.0', '.5', '0.5', '0.25', '-1', '1e+21', '1e21', 'Infinity', '[13px]', '[50%]', '[0]', '[var(--probe)]', '(--probe)', '[calc(1px_+_2px)]', '[color:var(--probe)]', '[length:var(--probe)]', '[shadow:var(--probe)]', '[percentage:var(--probe)]', '[length:red]', '[color:13px]', '[nonsense:13px]', '[red]', '[inherit]', '[currentColor]', '[#123456]', '[0_1px_red]', '[0_1px_var(--color-brand)]', '[0_1px_red,2px_3px_blue]']
const modifiers = ['0', '0.25', '1.5', '25', '50', '100', '125', '01', '50.0', '-25', 'muted', '[.35]', '[35%]', '[var(--opacity-muted)]', '(--opacity-muted)', '[calc(.5_*_2)]', '[0x1]', '[Infinity]', '[1e-9]', '[1e20]', '[.29]', '[0o1]', '[0b1]', '[-.5]', 'unknown', '[]', '[;]', '(bad)', '25/foo']
const shadows = ['0_0_red', '0_1px_red', 'red_0_1px', '0_1px', '0_1px_2px', '0_1px_2px_3px_red', '0_1px_currentcolor', '0_1px_currentColor', '0_1px_CurrentColor', '0_1px_var(--color-brand)', '0_1px_rgb(1_2_3_/_50%)', '0_1px_color-mix(in_oklab,red_50%,blue)', '0_1px_red,2px_3px_blue', '0_1px_red,2px_3px_var(--color-brand)', 'inset_0_1px_red', 'inherit', 'initial', 'revert', 'unset', 'var(--probe)', 'calc(1px_+_2px)_3px_red', '+1px_2px_red', '.5px_2px_red', '-.5px_2px_red', '1.5px_2px_red', '1px', 'red', 'red_blue_0_1px', 'red0_0_1px', '0__1px___red']

export async function shadowFixtures() {
  const cases = []
  async function add(id, mode, candidates) {
    const input = `${themes[mode]} @tailwind utilities;`.trim()
    const compiler = await tailwind.compile(input)
    const expectedCss = banner(compiler.build(candidates))
    cases.push({ id, input, candidates, ...(mode.startsWith('prefix') ? { branch: 'pending_feature', expectedPendingFeature: '@theme prefix(' } : {}), expectedCss, expectedCssSha256: sha256(expectedCss) })
  }
  for (const root of roots) {
    for (const mode of ['empty', 'base', 'inline', 'reference', 'default']) for (const value of values) await add(`shadow:root=${root}:${value || '<default>'}:${mode}`, mode, [`${root}${value ? `-${value}` : ''}`])
    for (const mode of ['bare', 'defaults', 'reset', 'namespaceReset', 'defaultPriority', 'collision', 'colorsOnly']) for (const value of ['', 'card', 'alias', 'multi', 'brand', '1.5', 'none', 'current', 'inherit', '0', '2', '[var(--probe)]']) await add(`shadow:root=${root}:${value || '<default>'}:${mode}`, mode, [`${root}${value ? `-${value}` : ''}`])
    for (const value of ['', 'card', 'alias', 'multi', 'brand', 'none', 'inherit', 'current', 'transparent', '2', '[red]', '[color:var(--probe)]', '[13px]', '[0_1px_red]', '[0_1px_var(--color-brand)]']) for (const modifier of modifiers) for (const mode of ['base', 'bare', 'inline', 'reference', 'collision']) await add(`shadow:root=${root}:${value || '<default>'}/${modifier}:${mode}:modifier`, mode, [`${root}${value ? `-${value}` : ''}/${modifier}`])
    for (const mode of ['prefix', 'prefixInline', 'prefixReference']) for (const candidate of [`tw:${root}-card`, `tw:${root}-[13px]`, `tw:hover:${root}-[13px]!`, `${root}-card`]) await add(`shadow:${candidate}:${mode}`, mode, [candidate])
    await add(`shadow:${root}:important`, 'base', [`${root}-[0_1px_red]/[var(--opacity-muted)]!`])
    await add(`shadow:${root}:hover`, 'base', [`hover:${root}-brand/50`])
    await add(`shadow:${root}:negative-invalid`, 'base', [`-${root}-card`])
  }
  for (const root of shadowRoots) for (const shadow of shadows) for (const modifier of ['', '25', '[.35]', '[var(--opacity-muted)]']) await add(`shadow:${root}-[${shadow}]${modifier ? `/${modifier}` : ''}:shadow-parser`, 'base', [`${root}-[${shadow}]${modifier ? `/${modifier}` : ''}`])
  for (const root of ringRoots) for (let width = 0; width <= 100; width++) await add(`shadow:${root}-${width}:integer-width`, 'empty', [`${root}-${width}`])
  for (const root of ringRoots) for (const unit of ['cm', 'mm', 'Q', 'in', 'pc', 'pt', 'px', 'em', 'ex', 'ch', 'rem', 'lh', 'rlh', 'vw', 'vh', 'vmin', 'vmax', 'vb', 'vi', 'svw', 'svh', 'lvw', 'lvh', 'dvw', 'dvh', 'cqw', 'cqh', 'cqi', 'cqb', 'cqmin', 'cqmax']) await add(`shadow:${root}-[13${unit}]:length-unit`, 'empty', [`${root}-[13${unit}]`])
  for (const root of roots) for (const mode of ['base', 'colorsOnly']) for (const value of ['1/2', '1/0', '0/0', '0.5/2', 'card/2', '1/[2]', '1/(--probe)']) await add(`shadow:${root}-${value}:${mode}:fraction`, mode, [`${root}-${value}`])
  for (const root of staticRoots) for (const mode of ['empty', 'collision']) await add(`shadow:static:${root}:${mode}`, mode, [root])
  const themeCss = await readFile(resolve(directory, '../../../ref/tailwindcss/packages/tailwindcss/theme.css'), 'utf8')
  const selectedDefaults = [...themeCss.matchAll(/--(?:shadow|inset-shadow|text-shadow)(?:-[a-z0-9-]+)?\s*:[^;]+;/g)].map(match => match[0]).join(' ')
  themes.upstreamDefault = `@theme default { ${selectedDefaults} }`
  for (const root of shadowRoots) for (const value of ['', '2xs', 'xs', 'sm', 'md', 'lg', 'xl', '2xl', 'inner']) await add(`shadow:${root}${value ? `-${value}` : ''}:upstream-default`, 'upstreamDefault', [`${root}${value ? `-${value}` : ''}`])
  await add('shadow:all-resources-sorted-union', 'base', ['shadow-card/25', 'inset-shadow-[0_1px_red,2px_3px_var(--color-brand)]/[var(--opacity-muted)]', 'text-shadow-brand/50', 'ring-2', 'inset-ring-card', 'ring-offset-card', 'ring-inset', 'shadow-initial', 'hover:text-shadow-card!', 'ring-brand', 'inset-ring-brand', 'ring-offset-brand'])
  await add('shadow:text-inset-raw-priority-union', 'reference', ['text-shadow', 'text-shadow-card', 'text-shadow-inherit', 'text-shadow-initial', 'inset-shadow-card', 'inset-shadow-[0_1px_red,2px_3px_blue]', 'shadow-transparent', 'ring-[13px]', 'ring-offset-[13px]', 'inset-ring-[var(--probe)]'])
  const ids = new Set(); for (const item of cases) { assert.ok(!ids.has(item.id), `duplicate ${item.id}`); ids.add(item.id) }
  const inventory = JSON.parse(await readFile(resolve(directory, 'inventory.json'), 'utf8'))
  const sources = ['packages/tailwindcss/src/utilities.ts', 'packages/tailwindcss/src/utilities.test.ts', 'packages/tailwindcss/src/theme.ts', 'packages/tailwindcss/src/utils/infer-data-type.ts', 'packages/tailwindcss/src/utils/replace-shadow-colors.ts', 'packages/tailwindcss/src/ast.ts'].map(path => { const source = inventory.sources.find(item => item.path === path); assert.ok(source?.sha256, path); return { path, sha256: source.sha256 } })
  sources.push({ path: 'packages/tailwindcss/theme.css', sha256: sha256(themeCss) })
  return { schemaVersion: 1, sourceCommit: '1334c99db8fd26c8ea065375dd9259800863f072', oracleVersion: '4.1.13', functionalRoots: roots, staticRegressionRoots: staticRoots, fullFamilyConformance: 'pending', sources, cases }
}

export async function checkShadowFixtures() {
  const actual = await shadowFixtures()
  assert.deepEqual(actual, JSON.parse(await readFile(resolve(directory, 'shadows-fixtures.json'), 'utf8')))
  return { functionalRoots: roots.length, staticRegressionRoots: staticRoots.length, cases: actual.cases.length, pendingFeatureCases: actual.cases.filter(item => item.branch === 'pending_feature').length }
}
if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  if (process.argv.includes('--write')) { const actual = await shadowFixtures(); await writeFile(resolve(directory, 'shadows-fixtures.json'), JSON.stringify(actual, null, 2) + '\n'); console.log(JSON.stringify({ functionalRoots: roots.length, cases: actual.cases.length })) }
  else console.log(JSON.stringify(await checkShadowFixtures()))
}
