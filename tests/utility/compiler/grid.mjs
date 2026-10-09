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
const placement = ['col', 'col-start', 'col-end', 'row', 'row-start', 'row-end']
const spans = ['col-span', 'row-span']
const tracks = ['grid-cols', 'grid-rows', 'auto-cols', 'auto-rows']
const roots = [...placement.flatMap(root => [root, `-${root}`]), ...spans, ...tracks]
const staticRoots = ['col-auto', 'col-span-full', 'col-start-auto', 'col-end-auto', 'row-auto', 'row-span-full', 'row-start-auto', 'row-end-auto', 'grid-flow-row', 'grid-flow-col', 'grid-flow-dense', 'grid-flow-row-dense', 'grid-flow-col-dense', 'auto-cols-auto', 'auto-cols-min', 'auto-cols-max', 'auto-cols-fr', 'auto-rows-auto', 'auto-rows-min', 'auto-rows-max', 'auto-rows-fr', 'grid-cols-none', 'grid-cols-subgrid', 'grid-rows-none', 'grid-rows-subgrid']
const namespaces = { col: 'grid-column', 'col-start': 'grid-column-start', 'col-end': 'grid-column-end', row: 'grid-row', 'row-start': 'grid-row-start', 'row-end': 'grid-row-end', 'grid-cols': 'grid-template-columns', 'grid-rows': 'grid-template-rows', 'auto-cols': 'grid-auto-columns', 'auto-rows': 'grid-auto-rows' }
const rawValue = root => placement.includes(root) ? '7 / 11' : 'minmax(13px, 1fr) 2fr'
const declarations = Object.entries(namespaces).map(([root, namespace]) => `--${namespace}-card: ${rawValue(root)}; --${namespace}-alias: var(--${namespace}-card); --${namespace}-1_5: ${placement.includes(root) ? '13' : '17px 1fr'}; --${namespace}-1\\/2: ${placement.includes(root) ? '17' : '23px 1fr'}; --${namespace}-dependent: var(--track);`).join(' ') + ' --track: 29px; --col-span-card: 19; --row-span-card: 23;'
const collisionTokens = ' --grid-column-auto: 2 / 3; --grid-column-span-full: 3 / 4; --grid-column-span-card: 5 / 6; --grid-column-start-auto: 7; --grid-column-end-auto: 8; --grid-row-auto: 2 / 3; --grid-row-span-full: 3 / 4; --grid-row-span-card: 5 / 6; --grid-row-start-auto: 7; --grid-row-end-auto: 8; --grid-auto-columns-auto: 31px; --grid-auto-columns-min: 37px; --grid-auto-columns-max: 41px; --grid-auto-columns-fr: 43px; --grid-auto-rows-auto: 31px; --grid-auto-rows-min: 37px; --grid-auto-rows-max: 41px; --grid-auto-rows-fr: 43px; --grid-template-columns-none: 47px; --grid-template-columns-subgrid: 53px; --grid-template-rows-none: 47px; --grid-template-rows-subgrid: 53px;'
const themes = {
  empty: '', base: `@theme { ${declarations} }`, inline: `@theme inline { ${declarations} }`, reference: `@theme reference { ${declarations} }`, default: `@theme default { ${declarations} }`,
  bare: `@theme { ${declarations} ${Object.entries(namespaces).map(([root, namespace]) => `--${namespace}: ${rawValue(root)};`).join(' ')} }`,
  reset: `@theme { ${declarations} } @theme { --grid-*: initial; }`,
  partialReset: `@theme { ${declarations} } @theme { --grid-column-*: initial; --grid-template-*: initial; }`,
  defaultPriority: `@theme { ${declarations} } @theme default { ${Object.values(namespaces).map(namespace => `--${namespace}-card: 99;`).join(' ')} }`,
  collision: `@theme { ${declarations} ${collisionTokens} }`, staticTheme: `@theme static { ${declarations} ${collisionTokens} }`,
  prefix: `@theme prefix(tw) { ${declarations} ${collisionTokens} }`, prefixInline: `@theme prefix(tw) inline { ${declarations} ${collisionTokens} }`, prefixReference: `@theme prefix(tw) reference { ${declarations} ${collisionTokens} }`,
}
const values = ['', 'card', 'alias', 'dependent', '1.5', '1/2', 'unknown', 'auto', 'full', 'min', 'max', 'fr', 'none', 'subgrid', 'span-card', 'span-full', 'start-card', 'end-card', '0', '1', '12', '99', '01', '1.0', '0.5', '.5', '-1', '1e3', '1e+21', '1e21', 'Infinity', '[13]', '[13px]', '[2fr]', '[0]', '[var(--probe)]', '(--probe)', '[integer:13]', '[length:13px]', '[number:2]', '[nonsense:foo]', '[calc(1_+_2)]', '[span_3/span_3]', '[1_/_-1]', '[minmax(13px,1fr)_2fr]', '[repeat(3,minmax(0,1fr))]', '[[left]_1fr_[right]]', '[none]', '[subgrid]', '[inherit]']
const modifiers = ['0', '1', '2', 'foo', 'card', '[var(--probe)]', '(--probe)', '[.5]', '[]', '[;]', '(bad)', '1/foo']

export async function gridFixtures() {
  const cases = []
  async function add(id, mode, candidates) {
    const input = `${themes[mode]} @tailwind utilities;`.trim()
    const compiler = await tailwind.compile(input)
    const expectedCss = compiler.build(candidates).replace(/^\/\*! tailwindcss[^\n]*\n/, '')
    cases.push({ id, input, candidates, expectedCss, expectedCssSha256: sha256(expectedCss) })
  }
  for (const root of roots) {
    for (const mode of ['empty', 'base', 'inline', 'reference', 'default', 'bare', 'reset', 'partialReset', 'defaultPriority', 'collision']) for (const value of values) await add(`grid:root=${root}:value=${value || '<default>'}:${mode}`, mode, [`${root}${value ? `-${value}` : ''}`])
    for (const mode of ['base', 'bare', 'inline', 'reference', 'collision']) for (const value of ['', 'card', '1/2', '12', '[13px]', '[var(--probe)]']) for (const modifier of modifiers) await add(`grid:${root}-${value || '<default>'}/${modifier}:${mode}`, mode, [`${root}${value ? `-${value}` : ''}/${modifier}`])
    for (const mode of ['prefix', 'prefixInline', 'prefixReference']) for (const candidate of [`tw:${root}-card`, `tw:${root}-1/2`, `tw:${root}-[13px]`, `tw:hover:${root}-card!`, `${root}-card`]) await add(`grid:${candidate}:${mode}`, mode, [candidate])
    for (let number = 0; number <= 32; number++) await add(`grid:${root}-${number}:integer`, 'empty', [`${root}-${number}`])
    await add(`grid:${root}:hover-important`, 'base', [`hover:${root}-card!`])
    for (const fraction of ['1/0', '0/0', '2/3', 'card/2', '1/[2]', '1/(--probe)']) await add(`grid:${root}-${fraction}:fraction`, 'base', [`${root}-${fraction}`])
  }
  for (const root of staticRoots) {
    for (const mode of ['empty', 'base', 'inline', 'reference', 'collision', 'staticTheme']) await add(`grid:static:${root}:${mode}`, mode, [root])
    for (const mode of ['prefix', 'prefixInline', 'prefixReference']) await add(`grid:static:tw:${root}:${mode}`, mode, [`tw:${root}`])
    for (const candidate of [`-${root}`, `${root}/foo`, `hover:${root}!`]) await add(`grid:static:${candidate}:boundary`, 'collision', [candidate])
  }
  const themeCss = await readFile(resolve(directory, '../../../ref/tailwindcss/packages/tailwindcss/theme.css'), 'utf8')
  assert.equal([...themeCss.matchAll(/--grid-(?:column|row|template|auto)(?:-[a-z0-9-]+)?\s*:/g)].length, 0, 'fixed CSS default theme has no grid tokens')
  await add('grid:all-sorted-union', 'collision', [...staticRoots, ...roots.map(root => `${root}-card`), 'col-span-3', 'row-span-5', 'grid-cols-12', 'grid-rows-7', 'hover:grid-cols-card!', 'hover:-col-start-card!'])
  await add('grid:raw-tracks-and-placement-union', 'reference', ['grid-cols-[[left]_1fr_[right]]', 'grid-rows-[repeat(2,minmax(0,1fr))]', 'col-[1_/_-1]', 'row-[span_3/span_3]', '-col-start-[var(--probe)]', 'row-span-[var(--probe)]', 'auto-cols-[minmax(13px,1fr)]', 'auto-rows-[2fr]'])
  const ids = new Set(); for (const item of cases) { assert.ok(!ids.has(item.id), `duplicate ${item.id}`); ids.add(item.id) }
  const inventory = JSON.parse(await readFile(resolve(directory, 'inventory.json'), 'utf8'))
  const sources = ['packages/tailwindcss/src/utilities.ts', 'packages/tailwindcss/src/utilities.test.ts', 'packages/tailwindcss/src/theme.ts', 'packages/tailwindcss/src/utils/infer-data-type.ts', 'packages/tailwindcss/src/property-order.ts'].map(path => { const source = inventory.sources.find(item => item.path === path); assert.ok(source?.sha256, path); return { path, sha256: source.sha256 } })
  sources.push({ path: 'packages/tailwindcss/theme.css', sha256: sha256(themeCss) })
  return { schemaVersion: 1, sourceCommit: '1334c99db8fd26c8ea065375dd9259800863f072', oracleVersion: '4.1.13', functionalRoots: roots, staticRegressionRoots: staticRoots, fullFamilyConformance: 'pending', defaultTheme: 'fixed CSS theme contains no grid namespace tokens; numeric fallbacks are native utilities', sources, cases }
}

export async function checkGridFixtures() {
  const actual = await gridFixtures()
  assert.deepEqual(actual, JSON.parse(await readFile(resolve(directory, 'grid-fixtures.json'), 'utf8')))
  return { functionalRoots: roots.length, staticRegressionRoots: staticRoots.length, cases: actual.cases.length }
}
if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  if (process.argv.includes('--write')) { const actual = await gridFixtures(); await writeFile(resolve(directory, 'grid-fixtures.json'), JSON.stringify(actual, null, 2) + '\n'); console.log(JSON.stringify({ functionalRoots: roots.length, cases: actual.cases.length })) }
  else console.log(JSON.stringify(await checkGridFixtures()))
}
