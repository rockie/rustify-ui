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
const banner = css => css.replace(/^\/\*! tailwindcss[^\n]*\n/, '')
const sha256 = text => createHash('sha256').update(text).digest('hex')
const roots = ['bg-linear', '-bg-linear', 'bg-conic', '-bg-conic', 'bg-radial', 'from', 'via', 'to']
const declaration = '--color-brand: #123456; --color-alias: var(--color-brand); --color-missing: var(--missing); --color-current: red; --color-inherit: blue; --color-transparent: green; --color-1_5: purple; --background-color-brand: #234567; --gradient-color-stop-positions-card: 37%; --gradient-color-stop-positions-brand: 41%; --gradient-color-stop-positions-1_5: 51%; --opacity-muted: .35; --opacity-1_5: .65;'
const themes = {
  empty: '', base: `@theme { ${declaration} }`, inline: `@theme inline { ${declaration} }`, reference: `@theme reference { ${declaration} }`, default: `@theme default { ${declaration} }`,
  positions: '@theme { --gradient-color-stop-positions-brand: 25%; --gradient-color-stop-positions-card: 37%; --gradient-color-stop-positions-1_5: 51%; }',
  colors: '@theme { --color-brand: #123456; --color-0: red; --gradient-color-stop-positions-brand: 25%; --opacity-muted: .35; }',
  ignored: '@theme { --gradient-position-card: 45deg; --gradient-color-stop-positions-45: 25%; --background-image-linear-card: url(card.png); --background-image-linear-45: url(angle.png); --background-image-conic-card: url(conic.png); --background-image-radial-card: url(radial.png); --color-brand: #123456; }',
  reset: `@theme { ${declaration} } @theme { --color-*: initial; --background-color-*: initial; --gradient-color-stop-positions-*: initial; }`,
  defaultPriority: `@theme { ${declaration} } @theme default { --color-brand: blue; --gradient-color-stop-positions-card: 11%; }`,
  prefix: `@theme prefix(tw) { ${declaration} }`, prefixInline: `@theme prefix(tw) inline { ${declaration} }`, prefixReference: `@theme prefix(tw) reference { ${declaration} }`,
}
const gradientValues = ['', '0', '30', '45', '90', '360', '450', '01', '45.0', '0.25', '-45', '1e+21', '1e21', 'Infinity', 'card', 'unknown', 'to-t', 'to-tr', 'to-r', 'to-br', 'to-b', 'to-bl', 'to-l', 'to-tl', 'to-top', '[45deg]', '[-45deg]', '[.5rad]', '[1e2grad]', '[.5turn]', '[45DEG]', '[calc(45deg_+_5deg)]', '[angle:var(--probe)]', '[angle:red]', '[color:45deg]', '[to_bottom]', '[circle_at_center]', '[in_hsl_longer_hue]', '[red]', '[var(--probe)]', '(--probe)', '[url(test.png)]']
const stopValues = ['', 'brand', 'alias', 'missing', 'card', '1.5', 'current', 'inherit', 'transparent', 'unknown', '0', '50', '0%', '25%', '50%', '100%', '150%', '50.0%', '01%', '.5%', '2.5%', '-5%', '1e+21%', '[#123456]', '[red]', '[currentColor]', '[13px]', '[50%]', '[calc(1rem_+_2px)]', '[0]', '[var(--color-brand)]', '(--color-brand)', '[color:var(--probe)]', '[length:var(--probe)]', '[percentage:var(--probe)]', '[angle:var(--probe)]', '[length:red]', '[nonsense:13px]', '[url(test.png)]']
const interpolation = ['oklab', 'oklch', 'srgb', 'hsl', 'lab', 'lch', 'srgb-linear', 'xyz', 'longer', 'shorter', 'increasing', 'decreasing', 'unknown', '50', '[in_hsl_longer_hue]', '[longer]', '[var(--probe)]', '(--probe)', '[]', '[;]', '(bad)', 'oklab/foo']
const alpha = ['0', '0.25', '1.5', '25', '50', '100', '125', '50.0', '01', '-25', '.5', 'unknown', 'muted', '[.35]', '[35%]', '[var(--opacity-muted)]', '(--opacity-muted)', '[calc(.5_*_2)]', '[0x1]', '[Infinity]', '[1e-9]', '[1e20]', '[.29]', '[0o1]', '[0b1]', '[-.5]', '[]', '[;]', '(bad)', '25/foo']
export async function gradientFixtures() {
  const cases = []
  async function add(id, mode, candidates) {
    const input = `${themes[mode]} @tailwind utilities;`.trim()
    const compiler = await tailwind.compile(input)
    const expectedCss = banner(compiler.build(candidates))
    cases.push({ id, input, candidates, ...(mode.startsWith('prefix') ? { branch: 'pending_feature', expectedPendingFeature: '@theme prefix(' } : {}), expectedCss, expectedCssSha256: sha256(expectedCss) })
  }
  for (const root of roots) {
    const stop = ['from', 'via', 'to'].includes(root)
    const values = stop ? stopValues : gradientValues
    for (const mode of ['empty', 'base', 'inline', 'reference', 'default']) for (const value of values) await add(`gradient:${root}${value ? `-${value}` : ''}:${mode}`, mode, [`${root}${value ? `-${value}` : ''}`])
    for (const value of stop ? ['brand', 'card', '25%', '[#123456]', '[13px]', '[var(--color-brand)]'] : ['', '45', 'to-r', '[45deg]', '[circle_at_center]']) for (const modifier of stop ? alpha : interpolation) await add(`gradient:${root}${value ? `-${value}` : ''}/${modifier}:modifier`, 'base', [`${root}${value ? `-${value}` : ''}/${modifier}`])
    for (const mode of ['positions', 'colors', 'ignored', 'reset', 'defaultPriority']) for (const value of ['', 'brand', 'card', '1.5', '45', '0%', '[var(--color-brand)]']) await add(`gradient:${root}${value ? `-${value}` : ''}:${mode}`, mode, [`${root}${value ? `-${value}` : ''}`])
    for (const mode of ['prefix', 'prefixInline', 'prefixReference']) for (const candidate of [`tw:${root}-${stop ? 'brand' : '45'}`, `tw:${root}-[${stop ? '13px' : '45deg'}]`, `tw:hover:${root}-[${stop ? '13px' : '45deg'}]!`, `${root}-${stop ? 'brand' : '45'}`]) await add(`gradient:${candidate}:${mode}`, mode, [candidate])
    await add(`gradient:${root}:important`, 'base', [`${root}-${stop ? 'brand' : '[45deg]'}!`])
    await add(`gradient:${root}:hover`, 'base', [`hover:${root}-${stop ? 'brand' : '[45deg]'}`])
    if (!root.startsWith('-')) await add(`gradient:${root}:negative`, 'base', [`-${root}-${stop ? 'brand' : '[45deg]'}`])
  }
  for (const root of ['from', 'via', 'to']) for (const unit of ['cm', 'mm', 'Q', 'in', 'pc', 'pt', 'px', 'em', 'ex', 'ch', 'rem', 'lh', 'rlh', 'vw', 'vh', 'vmin', 'vmax', 'vb', 'vi', 'svw', 'svh', 'lvw', 'lvh', 'dvw', 'dvh', 'cqw', 'cqh', 'cqi', 'cqb', 'cqmin', 'cqmax']) await add(`gradient:${root}-[13${unit}]:length-unit`, 'empty', [`${root}-[13${unit}]`])
  for (const root of ['from', 'via', 'to']) for (let percent = 0; percent <= 100; percent++) await add(`gradient:${root}-${percent}%:integer-percent`, 'empty', [`${root}-${percent}%`])
  for (const root of ['bg-linear', '-bg-linear']) for (const unit of ['deg', 'rad', 'grad', 'turn']) for (const number of ['45', '-45', '+45', '.5', '1e2', '1.', '0', 'calc(45)']) await add(`gradient:${root}-[${number}${unit}]:angle-syntax`, 'empty', [`${root}-[${number}${unit}]`])
  const themeCss = await readFile(resolve(directory, '../../../ref/tailwindcss/packages/tailwindcss/theme.css'), 'utf8')
  const selectedDefaults = [...themeCss.matchAll(/--color-(?:red|blue|green)-500\s*:[^;]+;/g)].map(match => match[0]).join(' ')
  themes.upstreamDefault = `@theme default { ${selectedDefaults} }`
  for (const root of ['from', 'via', 'to']) for (const color of ['red', 'blue', 'green']) await add(`gradient:${root}-${color}-500:upstream-default`, 'upstreamDefault', [`${root}-${color}-500`])
  await add('gradient:linear-color-position-union', 'base', ['bg-linear-to-r/longer', 'from-brand/50', 'from-25%', 'via-[#ff0000]', 'via-card', 'to-current', 'to-100%', 'via-none', 'hover:from-brand!', 'bg-linear-[45deg]'])
  await add('gradient:conic-radial-resource-order', 'reference', ['bg-conic-45/hsl', '-bg-conic-90/shorter', 'bg-radial-[circle_at_center]', 'from-[color:var(--probe)]', 'via-brand/50', 'to-transparent', 'via-[13px]', 'to-[50%]'])
  assert.equal(new Set(cases.map(item => item.id)).size, cases.length)
  const inventory = JSON.parse(await readFile(resolve(directory, 'inventory.json'), 'utf8'))
  const sources = ['packages/tailwindcss/src/utilities.ts', 'packages/tailwindcss/src/utilities.test.ts', 'packages/tailwindcss/src/theme.ts', 'packages/tailwindcss/src/utils/infer-data-type.ts', 'packages/tailwindcss/src/ast.ts'].map(path => { const source = inventory.sources.find(item => item.path === path); assert.ok(source?.sha256, path); return { path, sha256: source.sha256 } })
  sources.push({ path: 'packages/tailwindcss/theme.css', sha256: sha256(themeCss) })
  return { schemaVersion: 1, sourceCommit: '1334c99db8fd26c8ea065375dd9259800863f072', oracleVersion: '4.1.13', functionalRoots: roots, fullFamilyConformance: 'pending', sources, cases }
}
export async function checkGradientFixtures() {
  const actual = await gradientFixtures()
  assert.deepEqual(actual, JSON.parse(await readFile(resolve(directory, 'gradients-fixtures.json'), 'utf8')))
  return { functionalRoots: roots.length, cases: actual.cases.length, pendingFeatureCases: actual.cases.filter(item => item.branch === 'pending_feature').length }
}
if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  if (process.argv.includes('--write')) { const actual = await gradientFixtures(); await writeFile(resolve(directory, 'gradients-fixtures.json'), JSON.stringify(actual, null, 2) + '\n'); console.log(JSON.stringify({ functionalRoots: roots.length, cases: actual.cases.length })) }
  else console.log(JSON.stringify(await checkGradientFixtures()))
}
