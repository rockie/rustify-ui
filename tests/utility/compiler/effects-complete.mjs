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
const radiusRoots = ['rounded', 'rounded-s', 'rounded-e', 'rounded-t', 'rounded-r', 'rounded-b', 'rounded-l', 'rounded-ss', 'rounded-se', 'rounded-ee', 'rounded-es', 'rounded-tl', 'rounded-tr', 'rounded-br', 'rounded-bl']
const roots = [...radiusRoots, 'content']
const staticRoots = [...radiusRoots.flatMap(root => [`${root}-none`, `${root}-full`]), 'content-none']
const declarations = '--radius: 11px; --radius-card: 13px; --radius-alias: var(--radius-card); --radius-1_5: 17px; --radius-1_5\\/foo: 19px; --radius-1\\/2: 23px; --radius-card\\/foo: 29px; --radius-escaped\\/key: 31px; --radius-zero: 0; --radius-calc: calc(2px + 3px); --radius-empty: ; --content: "bare"; --content-card: "named"; --content-1\\/2: "fraction"; --content-none: "collision"; --spacing: .25rem;'
const collision = '--radius-none: 37px; --radius-full: 41px; --radius-none\\/foo: 43px; --radius-full\\/foo: 47px; --radius-card\\/2: 53px;'
const themes = {
  empty: '', base: `@theme { ${declarations} }`, inline: `@theme inline { ${declarations} }`, reference: `@theme reference { ${declarations} }`, default: `@theme default { ${declarations} }`,
  staticTheme: `@theme static { ${declarations} ${collision} }`, collision: `@theme { ${declarations} ${collision} }`,
  reset: `@theme { ${declarations} ${collision} } @theme { --radius-*: initial; --content-*: initial; }`,
  defaultPriority: `@theme { ${declarations} } @theme default { --radius: 59px; --radius-card: 61px; --radius-1\\/2: 67px; }`,
  inlineReference: `@theme inline reference { ${declarations} }`,
  prefix: `@theme prefix(tw) { ${declarations} ${collision} }`, prefixInline: `@theme prefix(tw) inline { ${declarations} ${collision} }`, prefixReference: `@theme prefix(tw) reference { ${declarations} ${collision} }`,
}
const values = ['', 'card', 'alias', 'zero', 'calc', 'empty', 'missing', 'none', 'full', '1.5', '1_5', '1/2', '1.5/foo', 'card/foo', 'card/2', 'escaped/key', '0', '1', '2', '01', '1.0', '.5', '-1', '50%', '1e+21', 'Infinity', '[13px]', '[13px_17px]', '[var(--probe)]', '(--probe)', '(length:--probe)', '(color:--probe)', '[length:13px]', '[color:#123456]', '[number:13]', '[percentage:13%]', '[nonsense:foo]', '[calc(1px+2px)]', '[50%]', '[0]', '[none]', '[inherit]', '["hello_world"]', "['hello_world']", '[attr(data-label)]', '[counter(items)]', '[url(icon.svg)]', '[normal]', '[var(--label,_"fallback")]']
const modifiers = ['0', '1', '2', 'foo', 'key', 'card', 'missing', '[13px]', '[var(--probe)]', '(--probe)', '[.5]', '[]', '[;]', '(bad)', 'foo/bar']

export async function effectsCompleteFixtures() {
  const cases = []
  async function add(id, mode, candidates) {
    const input = `${themes[mode]} @tailwind utilities;`.trim()
    const compiler = await tailwind.compile(input)
    const expectedCss = compiler.build(candidates).replace(/^\/\*! tailwindcss[^\n]*\n/, '')
    cases.push({ id, input, candidates, expectedCss, expectedCssSha256: sha256(expectedCss) })
  }
  for (const root of roots) {
    for (const mode of ['empty', 'base', 'inline', 'reference', 'default', 'staticTheme', 'collision', 'reset', 'defaultPriority', 'inlineReference']) for (const value of values) await add(`effects-complete:root=${root}:value=${value || '<default>'}:${mode}`, mode, [`${root}${value ? `-${value}` : ''}`])
    for (const mode of ['base', 'inline', 'reference', 'collision']) for (const value of ['', 'card', '1.5', '1/2', 'none', 'full', '[13px]', '[var(--probe)]']) for (const modifier of modifiers) await add(`effects-complete:modifier:root=${root}:value=${value || '<default>'}/${modifier}:${mode}`, mode, [`${root}${value ? `-${value}` : ''}/${modifier}`])
    for (const mode of ['prefix', 'prefixInline', 'prefixReference']) for (const candidate of [`tw:${root}`, `tw:${root}-card`, `tw:${root}-1/2`, `tw:${root}-card/foo`, `tw:${root}-card/[13px]`, `tw:${root}-[13px]`, `tw:hover:${root}-card!`, `tw:before:${root}-[var(--probe)]!`, `${root}-card`]) await add(`effects-complete:${candidate}:${mode}`, mode, [candidate])
    for (const mode of ['base', 'inline', 'reference']) for (const candidate of [`hover:${root}-card!`, `before:${root}-[13px]`, `after:${root}-[var(--probe)]!`, `before:hover:${root}-["probe"]`, `hover:after:${root}-none!`]) await add(`effects-complete:${candidate}:${mode}`, mode, [candidate])
    for (const candidate of [`-${root}-card`, `-${root}-[13px]`, `${root}-[]`, `${root}-[;]`, `${root}-(bad)`, `${root}-[var(--probe)`, `${root}-card!foo`, `${root}-card!!`, `!${root}-card!`, `${root}-card/foo/bar`]) await add(`effects-complete:invalid:${candidate}`, 'base', [candidate])
  }
  for (const root of staticRoots) {
    for (const mode of ['empty', 'base', 'inline', 'reference', 'collision', 'staticTheme']) await add(`effects-complete:static:${root}:${mode}`, mode, [root])
    for (const mode of ['prefix', 'prefixInline', 'prefixReference']) await add(`effects-complete:static:tw:${root}:${mode}`, mode, [`tw:${root}`])
    for (const candidate of [`-${root}`, `${root}/foo`, `${root}/[var(--probe)]`, `before:${root}`, `after:${root}!`, `hover:${root}!`]) await add(`effects-complete:static:${candidate}:boundary`, 'collision', [candidate])
  }
  const themeCss = await readFile(resolve(directory, '../../../ref/tailwindcss/packages/tailwindcss/theme.css'), 'utf8')
  const defaultRadius = [...themeCss.matchAll(/--radius-[a-z0-9-]+\s*:[^;]+;/g)].map(match => match[0])
  assert.equal(defaultRadius.length, 8)
  const legacyRadius = themeCss.match(/--radius\s*:[^;]+;/)?.[0]
  assert.equal(legacyRadius, '--radius: 0.25rem;')
  themes.upstreamDefault = `@theme default { ${defaultRadius.join(' ')} } @theme default inline reference { ${legacyRadius} }`
  for (const root of radiusRoots) for (const value of ['', 'xs', 'sm', 'md', 'lg', 'xl', '2xl', '3xl', '4xl', 'none', 'full', 'missing', '1/2', '[13px]']) await add(`effects-complete:${root}-${value || '<default>'}:upstream-default`, 'upstreamDefault', [`${root}${value ? `-${value}` : ''}`])
  for (const mode of ['empty', 'base', 'inline', 'reference', 'collision', 'prefix']) {
    const prefix = mode === 'prefix' ? 'tw:' : ''
    await add(`effects-complete:sorted-union:${mode}`, mode, [...staticRoots, ...roots.map(root => `${root}-card`), ...radiusRoots.map(root => `${root}-1/2`), 'rounded-card/[13px]', 'rounded-card/foo', 'hover:rounded-card!', 'before:content-["probe"]!', 'after:content-[attr(data-label)]', 'content-["label"]', 'before:content-none', 'after:rounded-[17px]!'].map(candidate => prefix + candidate))
    await add(`effects-complete:pseudo-resource-order:${mode}`, mode, ['before:content-["probe"]', 'after:content-[attr(data-label)]!', 'content-["label"]!', 'after:content-none', 'before:rounded-card', 'rounded-[13px_17px]', 'before:hover:content-[var(--label)]'].map(candidate => prefix + candidate))
  }
  const ids = new Set(); for (const item of cases) { assert.ok(!ids.has(item.id), `duplicate ${item.id}`); ids.add(item.id) }
  const inventory = JSON.parse(await readFile(resolve(directory, 'inventory.json'), 'utf8'))
  assert.deepEqual([...roots].sort(), inventory.utilities.filter(item => item.kind === 'functional' && (item.name.startsWith('rounded') || item.name === 'content')).map(item => item.name).sort())
  const sources = ['packages/tailwindcss/src/utilities.ts', 'packages/tailwindcss/src/utilities.test.ts', 'packages/tailwindcss/src/variants.ts', 'packages/tailwindcss/src/candidate.ts', 'packages/tailwindcss/src/theme.ts', 'packages/tailwindcss/src/property-order.ts', 'packages/tailwindcss/src/ast.ts'].map(path => { const source = inventory.sources.find(item => item.path === path); assert.ok(source?.sha256, path); return { path, sha256: source.sha256 } })
  sources.push({ path: 'packages/tailwindcss/theme.css', sha256: sha256(themeCss) })
  return { schemaVersion: 1, sourceCommit: '1334c99db8fd26c8ea065375dd9259800863f072', oracleVersion: '4.1.13', functionalRoots: roots, staticRegressionRoots: staticRoots, fullFamilyConformance: 'pending', defaultTheme: 'explicit fixed theme subset: eight radius scale tokens and legacy bare radius; no implicit full theme', sources, cases }
}
export async function checkEffectsCompleteFixtures() {
  const actual = await effectsCompleteFixtures()
  assert.deepEqual(actual, JSON.parse(await readFile(resolve(directory, 'effects-complete-fixtures.json'), 'utf8')))
  return { functionalRoots: roots.length, staticRegressionRoots: staticRoots.length, cases: actual.cases.length }
}
if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  if (process.argv.includes('--write')) { const actual = await effectsCompleteFixtures(); await writeFile(resolve(directory, 'effects-complete-fixtures.json'), JSON.stringify(actual, null, 2) + '\n'); console.log(JSON.stringify({ functionalRoots: roots.length, staticRegressionRoots: staticRoots.length, cases: actual.cases.length })) }
  else console.log(JSON.stringify(await checkEffectsCompleteFixtures()))
}
