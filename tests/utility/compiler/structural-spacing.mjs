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
const roots = ['border-spacing', 'border-spacing-x', 'border-spacing-y', 'space-x', '-space-x', 'space-y', '-space-y']
const staticRoots = [...roots.map(root => `${root}-px`), 'space-x-reverse', 'space-y-reverse']
const declarations = '--spacing: .25rem; --spacing-card: 13px; --spacing-alias: var(--spacing-card); --spacing-1_5: 17px; --spacing-1\\/2: 19px; --border-spacing-card: 23px; --border-spacing-alias: var(--border-spacing-card); --border-spacing-1_5: 29px; --border-spacing-1\\/2: 31px; --space-card: 37px; --space-alias: var(--space-card); --space-1_5: 41px; --space-1\\/2: 43px;'
const collisions = '--spacing-px: 47px; --border-spacing-px: 53px; --space-px: 59px; --space-reverse: 61px;'
const themes = {
  empty: '', base: `@theme { ${declarations} }`, inline: `@theme inline { ${declarations} }`, reference: `@theme reference { ${declarations} }`, default: `@theme default { ${declarations} }`,
  bare: `@theme { ${declarations} --border-spacing: 67px; --space: 71px; }`,
  reset: `@theme { ${declarations} } @theme { --border-spacing-*: initial; --space-*: initial; --spacing: initial; --spacing-*: initial; }`,
  spacingOnly: '@theme { --spacing: .5rem; --spacing-card: 73px; --spacing-alias: var(--spacing-card); --spacing-1_5: 79px; --spacing-1\\/2: 83px; }',
  specializedOnly: '@theme { --border-spacing-card: 89px; --space-card: 97px; }',
  defaultPriority: `@theme { ${declarations} } @theme default { --spacing: 1rem; --border-spacing-card: 101px; --space-card: 103px; }`,
  collision: `@theme { ${declarations} ${collisions} }`, staticTheme: `@theme static { ${declarations} ${collisions} }`,
  prefix: `@theme prefix(tw) { ${declarations} ${collisions} }`, prefixInline: `@theme prefix(tw) inline { ${declarations} ${collisions} }`, prefixReference: `@theme prefix(tw) reference { ${declarations} ${collisions} }`,
}
const values = ['', 'card', 'alias', '1.5', '1/2', 'missing', 'px', 'reverse', 'auto', 'full', '0', '0.25', '0.5', '0.75', '1', '12', '99', '01', '1.0', '.5', '-1', '1e3', '1e+21', '1e21', 'Infinity', 'NaN', '0.1', '[13px]', '[13px_17px]', '[0]', '[var(--probe)]', '(--probe)', '[length:13px]', '[number:2]', '[percentage:13%]', '[nonsense:foo]', '[calc(1px+2px)]', '[calc(var(--probe)*2)]', '[50%]', '[none]', '[inherit]', '[theme(spacing.2)]']
const modifiers = ['0', '1', '2', 'foo', 'card', '[var(--probe)]', '(--probe)', '[.5]', '[]', '[;]', '(bad)', '1/foo']

export async function structuralSpacingFixtures() {
  const cases = []
  async function add(id, mode, candidates) {
    const input = `${themes[mode]} @tailwind utilities;`.trim()
    const compiler = await tailwind.compile(input)
    const expectedCss = compiler.build(candidates).replace(/^\/\*! tailwindcss[^\n]*\n/, '')
    cases.push({ id, input, candidates, expectedCss, expectedCssSha256: sha256(expectedCss) })
  }
  const inventory = JSON.parse(await readFile(resolve(directory, 'inventory.json'), 'utf8'))
  const selected = item => item.name.includes('border-spacing') || /^-?space-[xy](?:-|$)/.test(item.name)
  assert.deepEqual(inventory.utilities.filter(item => item.kind === 'functional' && selected(item)).map(item => item.name).sort(), [...roots].sort())
  assert.deepEqual(inventory.utilities.filter(item => item.kind === 'static' && selected(item)).map(item => item.name).sort(), [...staticRoots].sort())
  for (const root of roots) {
    for (const mode of ['empty', 'base', 'inline', 'reference', 'default', 'bare', 'reset', 'spacingOnly', 'specializedOnly', 'defaultPriority', 'collision']) for (const value of values) await add(`structural-spacing:root=${root}:value=${value || '<default>'}:${mode}`, mode, [`${root}${value ? `-${value}` : ''}`])
    for (const mode of ['base', 'inline', 'reference', 'spacingOnly']) for (const value of ['', 'card', '1', '1/2', '[13px]', '[var(--probe)]']) for (const modifier of modifiers) await add(`structural-spacing:${root}-${value || '<default>'}/${modifier}:${mode}`, mode, [`${root}${value ? `-${value}` : ''}/${modifier}`])
    for (const mode of ['prefix', 'prefixInline', 'prefixReference']) for (const candidate of [`tw:${root}`, `tw:${root}-card`, `tw:${root}-1/2`, `tw:${root}-[13px]`, `tw:hover:${root}-card!`, `${root}-card`]) await add(`structural-spacing:${candidate}:${mode}`, mode, [candidate])
    for (const mode of ['base', 'spacingOnly']) for (let number = 0; number <= 8; number += .25) await add(`structural-spacing:${root}-${number}:${mode}:quarter`, mode, [`${root}-${number}`])
    for (const fraction of ['1/0', '0/0', '2/3', '0.5/2', '1/-2', 'card/2', '1/[2]', '1/(--probe)']) await add(`structural-spacing:${root}-${fraction}:fraction`, 'base', [`${root}-${fraction}`])
    for (const candidate of [`hover:${root}-card!`, `[&>img]:${root}-[13px]`, `supports-[display:grid]:${root}-1!`, `*:hover:${root}-card`, `before:${root}-[var(--probe)]!`]) await add(`structural-spacing:${candidate}:nested-variant`, 'base', [candidate])
    if (!root.startsWith('-')) await add(`structural-spacing:-${root}:negative-boundary`, 'base', [`-${root}-card`, `-${root}-[13px]`])
  }
  for (const root of staticRoots) {
    for (const mode of ['empty', 'base', 'inline', 'reference', 'collision', 'staticTheme']) await add(`structural-spacing:static:${root}:${mode}`, mode, [root])
    for (const mode of ['prefix', 'prefixInline', 'prefixReference']) await add(`structural-spacing:static:tw:${root}:${mode}`, mode, [`tw:${root}`])
    for (const candidate of [`${root}/foo`, `hover:${root}!`, `[&>img]:${root}!`]) await add(`structural-spacing:static:${candidate}:boundary`, 'collision', [candidate])
  }
  const themeCss = await readFile(resolve(directory, '../../../ref/tailwindcss/packages/tailwindcss/theme.css'), 'utf8')
  const spacing = themeCss.match(/--spacing\s*:[^;]+;/)?.[0]
  assert.ok(spacing)
  themes.upstreamDefault = `@theme default { ${spacing} }`
  for (const root of roots) for (const value of ['0', '.25', '0.25', '1.5', '2', '99', 'px', '1/2']) await add(`structural-spacing:${root}-${value}:upstream-default`, 'upstreamDefault', [`${root}-${value}`])
  await add('structural-spacing:all-resources-sorted-union', 'collision', [...staticRoots, ...roots.map(root => `${root}-card`), 'gap-x-2', 'gap-y-2', 'gap-2', 'space-x-2', 'space-y-2', 'border-spacing-2', 'hover:space-x-card!', 'hover:space-y-card!', 'border-spacing-x-1/2', '-space-x-0', '-space-y-2'])
  await add('structural-spacing:raw-values-sorted-union', 'reference', ['space-x-[13px]', '-space-x-[var(--probe)]', 'space-y-[17px]', '-space-y-[19px]', 'border-spacing-[23px]', 'border-spacing-x-[29px]', 'border-spacing-y-[31px]', 'space-x-reverse', 'space-y-reverse'])
  await add('structural-spacing:resource-order-x-before-y', 'base', ['space-y-2', 'space-x-2'])
  await add('structural-spacing:resource-order-y-before-x', 'base', ['space-x-2', 'space-y-2'])
  const ids = new Set(); for (const item of cases) { assert.ok(!ids.has(item.id), `duplicate ${item.id}`); ids.add(item.id) }
  const sources = ['packages/tailwindcss/src/utilities.ts', 'packages/tailwindcss/src/utilities.test.ts', 'packages/tailwindcss/src/theme.ts', 'packages/tailwindcss/src/utils/infer-data-type.ts', 'packages/tailwindcss/src/compile.ts', 'packages/tailwindcss/src/property-order.ts', 'packages/tailwindcss/src/ast.ts'].map(path => { const source = inventory.sources.find(item => item.path === path); assert.ok(source?.sha256, path); return { path, sha256: source.sha256 } })
  sources.push({ path: 'packages/tailwindcss/theme.css', sha256: sha256(themeCss) })
  return { schemaVersion: 1, sourceCommit: '1334c99db8fd26c8ea065375dd9259800863f072', oracleVersion: '4.1.13', functionalRoots: roots, staticRegressionRoots: staticRoots, fullFamilyConformance: 'pending', defaultTheme: 'explicit fixed CSS spacing token; no implicit full theme', sources, cases }
}

export async function checkStructuralSpacingFixtures() {
  const actual = await structuralSpacingFixtures()
  assert.deepEqual(actual, JSON.parse(await readFile(resolve(directory, 'structural-spacing-fixtures.json'), 'utf8')))
  return { functionalRoots: roots.length, staticRegressionRoots: staticRoots.length, cases: actual.cases.length }
}
if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  if (process.argv.includes('--write')) { const actual = await structuralSpacingFixtures(); await writeFile(resolve(directory, 'structural-spacing-fixtures.json'), JSON.stringify(actual, null, 2) + '\n'); console.log(JSON.stringify({ functionalRoots: roots.length, staticRegressionRoots: staticRoots.length, cases: actual.cases.length })) }
  else console.log(JSON.stringify(await checkStructuralSpacingFixtures()))
}
