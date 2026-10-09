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
const effects = ['blur', 'brightness', 'contrast', 'grayscale', 'hue-rotate', 'invert', 'saturate', 'sepia']
const roots = ['filter', 'backdrop-filter', ...effects.flatMap(root => [root, `backdrop-${root}`]), '-hue-rotate', '-backdrop-hue-rotate', 'drop-shadow', 'backdrop-opacity']
const staticRoots = ['blur-none', 'backdrop-blur-none', 'drop-shadow-none']
assert.equal(roots.length, 22)
const tokens = effects.map(root => `--${root}-card: ${root === 'blur' ? '13px' : root === 'hue-rotate' ? '17deg' : '.35'}; --${root}-alias: var(--${root}-card); --${root}-1_5: ${root === 'blur' ? '23px' : root === 'hue-rotate' ? '31deg' : '.65'}; --${root}-1\\/2: ${root === 'blur' ? '27px' : root === 'hue-rotate' ? '41deg' : '.75'};`).join(' ')
const backdropTokens = [...effects, 'opacity'].map(root => `--backdrop-${root}-card: ${root === 'blur' ? '29px' : root === 'hue-rotate' ? '47deg' : '.85'};`).join(' ')
const declaration = `${tokens} ${backdropTokens} --opacity-card: .45; --opacity-muted: .35; --color-brand: #123456; --color-alias: var(--color-brand); --color-1_5: red; --drop-shadow-color-brand: #234567; --drop-shadow-card: 0 2px 3px rgb(10 20 30 / .4); --drop-shadow-alias: var(--drop-shadow-card); --drop-shadow-1_5: 1px 2px blue; --drop-shadow-1\\/2: 0 1px red; --drop-shadow-multi: 0 1px red, 2px 3px var(--color-brand);`
const namespaces = [...effects, ...effects.map(root => `backdrop-${root}`), 'drop-shadow', 'backdrop-opacity']
const themes = {
  empty: '', base: `@theme { ${declaration} }`, inline: `@theme inline { ${declaration} }`, reference: `@theme reference { ${declaration} }`, default: `@theme default { ${declaration} }`,
  fallback: `@theme { ${tokens} --opacity-card: .45; --drop-shadow-card: 0 1px red; --color-brand: #123456; }`,
  bare: `@theme { ${namespaces.map(namespace => `--${namespace}: ${namespace.includes('drop-shadow') ? '0 1px red' : namespace.includes('blur') ? '3px' : namespace.includes('hue-rotate') ? '13deg' : '.3'};`).join(' ')} }`,
  reset: `@theme { ${declaration} } @theme { ${namespaces.map(namespace => `--${namespace}-*: initial;`).join(' ')} }`,
  defaultPriority: `@theme { ${declaration} } @theme default { --blur-card: 1px; --backdrop-blur-card: 2px; --drop-shadow-card: 0 1px yellow; --color-brand: green; }`,
  collision: '@theme { --blur-none: 9px; --backdrop-blur-none: 10px; --drop-shadow-none: 0 1px red; --drop-shadow-current: 0 1px blue; --drop-shadow-inherit: 0 1px purple; --drop-shadow-transparent: 0 1px green; --drop-shadow-brand: 0 1px red; --drop-shadow-color-brand: blue; --color-brand: green; --drop-shadow-color-card: yellow; --blur-x-card: 2px; --filter-card: blur(7px); --backdrop-filter-card: blur(9px); }',
  prefix: `@theme prefix(tw) { ${declaration} }`, prefixInline: `@theme prefix(tw) inline { ${declaration} }`, prefixReference: `@theme prefix(tw) reference { ${declaration} }`,
}
const values = ['', 'card', 'alias', '1.5', 'unknown', 'none', 'x-card', '0', '1', '25', '50', '100', '150', '01', '1.0', '.5', '0.5', '0.25', '0.3', '-1', '1e+21', '1e21', 'Infinity', '[13px]', '[45deg]', '[.35]', '[50%]', '[var(--probe)]', '(--probe)', '[calc(.5_*_2)]', '[color:red]', '[length:var(--probe)]', '[angle:var(--probe)]', '[nonsense:red]', '[red]', '[blur(2px)_contrast(50%)]']
const modifiers = ['0', '0.25', '25', '100', '125', '01', '25.0', '-25', 'card', 'muted', 'foo', '[.35]', '[35%]', '[var(--opacity-muted)]', '(--opacity-muted)', '[calc(.5_*_2)]', '[]', '[;]', '(bad)', '25/foo']
const shadows = ['0_0_red', '0_1px_red', 'red_0_1px', '0_1px', '0_1px_2px', '0_1px_2px_3px_red', '0_1px_currentcolor', '0_1px_currentColor', '0_1px_CurrentColor', '0_1px_var(--color-brand)', '0_1px_rgb(1_2_3_/_50%)', '0_1px_color-mix(in_oklab,red_50%,blue)', '0_1px_red,2px_3px_blue', '0_1px_red,2px_3px_var(--color-brand)', 'inset_0_1px_red', 'inherit', 'initial', 'revert', 'unset', 'var(--probe)', 'calc(1px_+_2px)_3px_red', '+1px_2px_red', '.5px_2px_red', '-.5px_2px_red', '1.5px_2px_red', '1px', 'red', 'red_blue_0_1px', 'red0_0_1px', '0__1px___red']
const alpha = ['0', '0.25', '1.5', '25', '50', '100', '125', '01', '50.0', '-25', 'muted', '[.35]', '[35%]', '[var(--opacity-muted)]', '(--opacity-muted)', '[calc(.5_*_2)]', '[0x1]', '[Infinity]', '[1e-9]', '[1e20]', '[.29]', '[0o1]', '[0b1]', '[-.5]', 'unknown']

export async function filterFixtures() {
  const cases = []
  async function add(id, mode, candidates) {
    const input = `${themes[mode]} @tailwind utilities;`.trim()
    const compiler = await tailwind.compile(input)
    const expectedCss = banner(compiler.build(candidates))
    cases.push({ id, input, candidates, ...(mode.startsWith('prefix') ? { branch: 'pending_feature', expectedPendingFeature: '@theme prefix(' } : {}), expectedCss, expectedCssSha256: sha256(expectedCss) })
  }
  for (const root of roots) {
    for (const mode of ['empty', 'base', 'inline', 'reference', 'default']) for (const value of values) await add(`filter:root=${root}:${value || '<default>'}:${mode}`, mode, [`${root}${value ? `-${value}` : ''}`])
    for (const mode of ['fallback', 'bare', 'reset', 'defaultPriority', 'collision']) for (const value of ['', 'card', 'alias', '1.5', 'none', '0', '1', 'current', 'inherit', 'transparent', '[var(--probe)]']) await add(`filter:root=${root}:${value || '<default>'}:${mode}`, mode, [`${root}${value ? `-${value}` : ''}`])
    for (const value of ['', 'card', '1', '[13px]']) for (const modifier of modifiers) await add(`filter:root=${root}:${value || '<default>'}/${modifier}:modifier`, 'base', [`${root}${value ? `-${value}` : ''}/${modifier}`])
    for (const mode of ['prefix', 'prefixInline', 'prefixReference']) for (const candidate of [`tw:${root}-card`, `tw:${root}-[13px]`, `tw:hover:${root}-[13px]!`, `${root}-card`]) await add(`filter:${candidate}:${mode}`, mode, [candidate])
    await add(`filter:${root}:important`, 'base', [`${root}-[13px]!`])
    await add(`filter:${root}:hover`, 'base', [`hover:${root}-[13px]`])
    if (!roots.includes(`-${root}`) && !root.startsWith('-')) await add(`filter:${root}:negative-invalid`, 'base', [`-${root}-card`])
  }
  for (const root of ['brightness', 'backdrop-brightness', 'contrast', 'backdrop-contrast', 'grayscale', 'backdrop-grayscale', 'invert', 'backdrop-invert', 'saturate', 'backdrop-saturate', 'sepia', 'backdrop-sepia']) for (let i = 0; i <= 200; i++) await add(`filter:${root}-${i}:integer-percent`, 'empty', [`${root}-${i}`])
  for (let i = 0; i <= 400; i++) await add(`filter:backdrop-opacity-${i / 4}:quarter-percent`, 'empty', [`backdrop-opacity-${i / 4}`])
  for (const value of ['', 'card', 'alias', 'multi', 'brand', 'current', 'inherit', 'transparent', '[red]', '[color:var(--probe)]', '[0_1px_red]', '[0_1px_var(--color-brand)]', '[0_1px_red,2px_3px_blue]']) for (const modifier of alpha) for (const mode of ['base', 'bare', 'inline', 'reference', 'collision']) await add(`filter:drop-shadow:${value || '<default>'}/${modifier}:${mode}:alpha`, mode, [`drop-shadow${value ? `-${value}` : ''}/${modifier}`])
  for (const shadow of shadows) for (const modifier of ['', '25', '[.35]', '[var(--opacity-muted)]']) await add(`filter:drop-shadow-[${shadow}]${modifier ? `/${modifier}` : ''}:shadow-parser`, 'base', [`drop-shadow-[${shadow}]${modifier ? `/${modifier}` : ''}`])
  for (const root of ['blur', 'backdrop-blur', 'drop-shadow', 'backdrop-opacity', 'hue-rotate', 'backdrop-hue-rotate']) for (const value of ['1/2', '1/0', '0/0', '0.5/2', 'card/2', '1/[2]', '1/(--probe)']) await add(`filter:${root}-${value}:fraction-theme`, 'base', [`${root}-${value}`])
  for (const root of staticRoots) for (const mode of ['empty', 'collision']) await add(`filter:static:${root}:${mode}`, mode, [root])
  const themeCss = await readFile(resolve(directory, '../../../ref/tailwindcss/packages/tailwindcss/theme.css'), 'utf8')
  const selectedDefaults = [...themeCss.matchAll(/--(?:blur|drop-shadow)-[a-z0-9-]+\s*:[^;]+;/g)].map(match => match[0]).join(' ')
  themes.upstreamDefault = `@theme default { ${selectedDefaults} }`
  for (const root of ['blur', 'backdrop-blur']) for (const value of ['xs', 'sm', 'md', 'lg', 'xl', '2xl', '3xl']) await add(`filter:${root}-${value}:upstream-default`, 'upstreamDefault', [`${root}-${value}`])
  for (const value of ['xs', 'sm', 'md', 'lg', 'xl', '2xl']) await add(`filter:drop-shadow-${value}:upstream-default`, 'upstreamDefault', [`drop-shadow-${value}`])
  await add('filter:all-resources-sorted-union', 'base', ['filter', 'blur-card', 'brightness-50', 'contrast-[.35]', 'grayscale', '-hue-rotate-45', 'invert-card', 'saturate-100', 'sepia-0', 'drop-shadow-multi/[var(--opacity-muted)]', 'hover:drop-shadow-brand!', 'drop-shadow-none', 'filter-[var(--probe)]'])
  await add('filter:backdrop-order-prefix-union', 'reference', ['backdrop-filter', 'backdrop-blur-card', 'backdrop-brightness-50', 'backdrop-contrast-[.35]', 'backdrop-grayscale', '-backdrop-hue-rotate-45', 'backdrop-invert-card', 'backdrop-saturate-100', 'backdrop-sepia-0', 'backdrop-opacity-0.25', 'hover:backdrop-filter-[blur(2px)]!'])
  const ids = new Set(); for (const item of cases) { assert.ok(!ids.has(item.id), `duplicate ${item.id}`); ids.add(item.id) }
  const inventory = JSON.parse(await readFile(resolve(directory, 'inventory.json'), 'utf8'))
  const sources = ['packages/tailwindcss/src/utilities.ts', 'packages/tailwindcss/src/utilities.test.ts', 'packages/tailwindcss/src/theme.ts', 'packages/tailwindcss/src/utils/infer-data-type.ts', 'packages/tailwindcss/src/utils/replace-shadow-colors.ts', 'packages/tailwindcss/src/ast.ts'].map(path => { const source = inventory.sources.find(item => item.path === path); assert.ok(source?.sha256, path); return { path, sha256: source.sha256 } })
  sources.push({ path: 'packages/tailwindcss/theme.css', sha256: sha256(themeCss) })
  return { schemaVersion: 1, sourceCommit: '1334c99db8fd26c8ea065375dd9259800863f072', oracleVersion: '4.1.13', functionalRoots: roots, staticRegressionRoots: staticRoots, fullFamilyConformance: 'pending', sources, cases }
}

export async function checkFilterFixtures() {
  const actual = await filterFixtures()
  assert.deepEqual(actual, JSON.parse(await readFile(resolve(directory, 'filters-fixtures.json'), 'utf8')))
  return { functionalRoots: roots.length, staticRegressionRoots: staticRoots.length, cases: actual.cases.length, pendingFeatureCases: actual.cases.filter(item => item.branch === 'pending_feature').length }
}
if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  if (process.argv.includes('--write')) { const actual = await filterFixtures(); await writeFile(resolve(directory, 'filters-fixtures.json'), JSON.stringify(actual, null, 2) + '\n'); console.log(JSON.stringify({ functionalRoots: roots.length, cases: actual.cases.length })) }
  else console.log(JSON.stringify(await checkFilterFixtures()))
}
