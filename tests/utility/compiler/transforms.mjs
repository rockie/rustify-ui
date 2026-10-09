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
const positiveRoots = ['origin', 'perspective-origin', 'perspective', 'translate', 'translate-x', 'translate-y', 'translate-z', 'scale', 'scale-x', 'scale-y', 'scale-z', 'rotate', 'rotate-x', 'rotate-y', 'rotate-z', 'skew', 'skew-x', 'skew-y', 'transform']
const negativeRoots = positiveRoots.filter(root => /^(translate|scale|rotate|skew)/.test(root)).map(root => `-${root}`)
const roots = [...positiveRoots, ...negativeRoots]
const staticRoots = [
  ...['origin', 'perspective-origin'].flatMap(root => ['center', 'top', 'top-right', 'right', 'bottom-right', 'bottom', 'bottom-left', 'left', 'top-left'].map(value => `${root}-${value}`)),
  'perspective-none', 'translate-none', 'translate-full', '-translate-full', 'translate-x-full', '-translate-x-full', 'translate-y-full', '-translate-y-full', 'translate-3d',
  ...['translate', 'translate-x', 'translate-y', 'translate-z'].flatMap(root => [`${root}-px`, `-${root}-px`]),
  'scale-none', 'scale-3d', 'rotate-none', 'transform-cpu', 'transform-gpu', 'transform-none', 'transform-flat', 'transform-3d', 'transform-content', 'transform-border', 'transform-fill', 'transform-stroke', 'transform-view', 'backface-visible', 'backface-hidden',
]
assert.equal(roots.length, 34)
assert.equal(staticRoots.length, 50)
const namespaces = ['transform-origin', 'perspective-origin', 'perspective', 'translate', 'scale', 'rotate', 'skew']
const declaration = '--spacing: .25rem; --spacing-card: 2rem; --spacing-1_5: 3rem; --translate-card: 17px; --translate-alias: var(--spacing-card); --translate-1_5: 23px; --translate-1\\/2: 41%; --scale-card: 1.7; --scale-alias: var(--scale-card); --scale-1_5: 1.3; --scale-1\\/2: .5; --rotate-card: 37deg; --rotate-alias: var(--rotate-card); --rotate-1_5: 13deg; --rotate-1\\/2: 23deg; --skew-card: 11deg; --skew-alias: var(--skew-card); --skew-1_5: 17deg; --skew-1\\/2: 31deg; --transform-origin-card: 13px 27px; --transform-origin-alias: var(--transform-origin-card); --transform-origin-1_5: 17% 37%; --transform-origin-1\\/2: top left; --perspective-origin-card: 17% 23%; --perspective-origin-alias: var(--perspective-origin-card); --perspective-origin-1_5: 11px 21px; --perspective-origin-1\\/2: center; --perspective-card: 333px; --perspective-alias: var(--perspective-card); --perspective-1_5: 777px; --perspective-1\\/2: 555px;'
const themes = {
  empty: '', base: `@theme { ${declaration} }`, inline: `@theme inline { ${declaration} }`, reference: `@theme reference { ${declaration} }`, default: `@theme default { ${declaration} }`,
  bare: `@theme { ${namespaces.map((namespace, index) => `--${namespace}: ${index + 1}px;`).join(' ')} --spacing: .5rem; }`,
  spacingOnly: '@theme { --spacing: .25rem; --spacing-card: 7px; --spacing-1_5: 13px; }',
  reset: `@theme { ${declaration} } @theme { ${namespaces.map(namespace => `--${namespace}-*: initial;`).join(' ')} --spacing-*: initial; }`,
  defaultPriority: `@theme { ${declaration} } @theme default { ${namespaces.map(namespace => `--${namespace}-card: 1px;`).join(' ')} --spacing: 1rem; }`,
  collision: '@theme { --transform-origin-top: 1px; --perspective-origin-top: 2px; --perspective-none: 3px; --translate-px: 4px; --translate-full: 5px; --translate-x-full: 6px; --translate-z-full: 7px; --translate-3d: 8px; --scale-none: 9; --scale-3d: 10; --rotate-none: 11deg; --rotate-x-card: 12deg; --skew-x-card: 13deg; --transform-card: scale(2); --transform-gpu: scale(3); }',
  prefix: `@theme prefix(tw) { ${declaration} }`, prefixInline: `@theme prefix(tw) inline { ${declaration} }`, prefixReference: `@theme prefix(tw) reference { ${declaration} }`,
}
const values = ['', 'card', 'alias', '1.5', 'unknown', '0', '1', '45', '50', '100', '150', '01', '1.0', '.5', '0.5', '0.25', '0.3', '-1', '1e+21', '1e21', 'Infinity', 'px', 'full', 'none', '3d', 'top', '[13px]', '[45deg]', '[2]', '[50%]', '[var(--probe)]', '(--probe)', '[calc(1rem_+_2px)]', '[length:var(--probe)]', '[angle:var(--probe)]', '[number:var(--probe)]', '[vector:var(--probe)]', '[nonsense:red]', '[red]', '[1_2_3]', '[x_45deg]', '[1_2_3_45deg]', '[50px_100px]', '[scaleZ(2)_rotateY(45deg)]']
const modifiers = ['2', '0', 'foo', '[2]', '(--probe)', '[]', '[;]', '(bad)', '2/foo']

export async function transformFixtures() {
  const cases = []
  async function add(id, mode, candidates) {
    const input = `${themes[mode]} @tailwind utilities;`.trim()
    const compiler = await tailwind.compile(input)
    const expectedCss = banner(compiler.build(candidates))
    cases.push({ id, input, candidates, ...(mode.startsWith('prefix') ? { branch: 'pending_feature', expectedPendingFeature: '@theme prefix(' } : {}), expectedCss, expectedCssSha256: sha256(expectedCss) })
  }
  for (const root of roots) {
    for (const mode of ['empty', 'base', 'inline', 'reference', 'default']) for (const value of values) await add(`transform:root=${root}:${root}${value ? `-${value}` : ''}:${mode}`, mode, [`${root}${value ? `-${value}` : ''}`])
    for (const mode of ['bare', 'spacingOnly', 'reset', 'defaultPriority', 'collision']) for (const value of ['', 'card', 'alias', '1.5', '0', '1', 'px', 'full', 'none', '3d', 'top', 'x-card', '[var(--probe)]']) await add(`transform:root=${root}:${root}${value ? `-${value}` : ''}:${mode}`, mode, [`${root}${value ? `-${value}` : ''}`])
    for (const value of ['', 'card', '1', '[13px]']) for (const modifier of modifiers) await add(`transform:root=${root}:${root}${value ? `-${value}` : ''}/${modifier}:modifier`, 'base', [`${root}${value ? `-${value}` : ''}/${modifier}`])
    for (const mode of ['prefix', 'prefixInline', 'prefixReference']) for (const candidate of [`tw:${root}-card`, `tw:${root}-[13px]`, `tw:hover:${root}-[13px]!`, `${root}-card`]) await add(`transform:${candidate}:${mode}`, mode, [candidate])
    await add(`transform:${root}:important`, 'base', [`${root}-[13px]!`])
    await add(`transform:${root}:hover`, 'base', [`hover:${root}-[13px]`])
    if (!root.startsWith('-') && !negativeRoots.includes(`-${root}`)) await add(`transform:${root}:negative-invalid`, 'base', [`-${root}-card`])
  }
  for (const root of ['translate', '-translate', 'translate-x', '-translate-x', 'translate-y', '-translate-y', 'translate-z', '-translate-z', 'scale-x', '-scale-x', 'rotate-x', '-rotate-x', 'skew', '-skew', 'origin', 'perspective']) for (const mode of ['empty', 'base', 'spacingOnly']) for (const fraction of ['1/2', '1/0', '0/0', '0/2', '1/3', '2/3', '01/2', '1/02', '1.0/2', '0.5/2', 'card/2', '1/[2]', '1/(--probe)', '[13px]/2']) await add(`transform:${root}-${fraction}:${mode}:fraction`, mode, [`${root}-${fraction}`])
  for (const sign of ['', '-']) for (const vector of ['1_2_3', '.5_+2_-3', '1e2_2_3', '1_2', '1_2_3_4', '1__2___3', '1_2_3deg', 'calc(1)_2_3', 'var(--probe)', '1._2_3']) for (const hint of ['', 'vector:', 'angle:', 'nonsense:']) await add(`transform:${sign}rotate-[${hint}${vector}]:vector`, 'empty', [`${sign}rotate-[${hint}${vector}]`])
  for (const root of ['rotate', '-rotate', 'rotate-x', '-rotate-x', 'skew-y', '-skew-y']) for (const unit of ['deg', 'rad', 'grad', 'turn']) for (const number of ['45', '-45', '+45', '.5', '1e2', '1.', '0', 'calc(45)']) await add(`transform:${root}-[${number}${unit}]:angle`, 'empty', [`${root}-[${number}${unit}]`])
  for (const root of ['translate-x', '-translate-x']) for (let i = 0; i <= 40; i++) await add(`transform:${root}-${i / 4}:quarter-spacing`, 'spacingOnly', [`${root}-${i / 4}`])
  for (const root of ['scale', '-scale', 'scale-z', '-scale-z']) for (let i = 0; i <= 200; i++) await add(`transform:${root}-${i}:integer-scale`, 'empty', [`${root}-${i}`])
  for (const root of staticRoots) for (const mode of ['empty', 'collision']) await add(`transform:static:${root}:${mode}`, mode, [root])
  const themeCss = await readFile(resolve(directory, '../../../ref/tailwindcss/packages/tailwindcss/theme.css'), 'utf8')
  const selectedDefaults = [...themeCss.matchAll(/--(?:spacing\s*|perspective-[a-z-]+)\s*:[^;]+;/g)].map(match => match[0]).join(' ')
  themes.upstreamDefault = `@theme default { ${selectedDefaults} }`
  for (const candidate of ['perspective-dramatic', 'perspective-near', 'perspective-normal', 'perspective-midrange', 'perspective-distant', 'translate-4', '-translate-z-0.5', 'translate-x-1/2']) await add(`transform:${candidate}:upstream-default`, 'upstreamDefault', [candidate])
  await add('transform:all-resources-sorted-union', 'base', ['translate-4', 'translate-z-card', '-translate-x-1/2', 'scale-50', '-scale-z-card', 'rotate-x-45', '-rotate-y-card', 'skew-card', 'skew-x-13', 'transform', 'transform-gpu', 'origin-card', 'perspective-card', 'hover:rotate-[vector:1_2_3]!', 'scale-[2_1.5_3]'])
  await add('transform:reference-inline-resource-union', 'reference', ['transform-[scaleZ(2)]!', 'rotate-[1_2_3]', 'rotate-card', '-skew-y-card', 'translate-3d', 'translate-x-alias', 'scale-alias', 'origin-alias', 'perspective-origin-alias'])
  const ids = new Set(); for (const item of cases) { assert.ok(!ids.has(item.id), `duplicate ${item.id}`); ids.add(item.id) }
  const inventory = JSON.parse(await readFile(resolve(directory, 'inventory.json'), 'utf8'))
  const sources = ['packages/tailwindcss/src/utilities.ts', 'packages/tailwindcss/src/utilities.test.ts', 'packages/tailwindcss/src/theme.ts', 'packages/tailwindcss/src/utils/infer-data-type.ts', 'packages/tailwindcss/src/ast.ts'].map(path => { const source = inventory.sources.find(item => item.path === path); assert.ok(source?.sha256, path); return { path, sha256: source.sha256 } })
  sources.push({ path: 'packages/tailwindcss/theme.css', sha256: sha256(themeCss) })
  return { schemaVersion: 1, sourceCommit: '1334c99db8fd26c8ea065375dd9259800863f072', oracleVersion: '4.1.13', functionalRoots: roots, staticRegressionRoots: staticRoots, fullFamilyConformance: 'pending', sources, cases }
}

export async function checkTransformFixtures() {
  const actual = await transformFixtures()
  assert.deepEqual(actual, JSON.parse(await readFile(resolve(directory, 'transforms-fixtures.json'), 'utf8')))
  return { functionalRoots: roots.length, staticRegressionRoots: staticRoots.length, cases: actual.cases.length, pendingFeatureCases: actual.cases.filter(item => item.branch === 'pending_feature').length }
}
if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  if (process.argv.includes('--write')) { const actual = await transformFixtures(); await writeFile(resolve(directory, 'transforms-fixtures.json'), JSON.stringify(actual, null, 2) + '\n'); console.log(JSON.stringify({ functionalRoots: roots.length, cases: actual.cases.length })) }
  else console.log(JSON.stringify(await checkTransformFixtures()))
}
