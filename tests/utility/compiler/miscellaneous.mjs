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
const roots = ['line-clamp', 'list', 'list-image', 'indent', '-indent', 'align', 'z', '-z', 'aspect', 'cursor', 'will-change', 'contain', '@container', 'max-w-screen']
const namespaces = ['line-clamp', 'list-style-type', 'list-style-image', 'text-indent', 'spacing', 'z-index', 'aspect', 'cursor', 'breakpoint']
const declarations = '--spacing: .25rem; --line-clamp: 13; --line-clamp-card: 17; --list-style-type: square; --list-style-type-card: upper-roman; --list-style-image: url(default.svg); --list-style-image-card: url(card.svg); --text-indent: 19px; --text-indent-card: 23px; --spacing-card: 29px; --z-index: 31; --z-index-card: 37; --aspect: 3 / 5; --aspect-card: 7 / 11; --cursor: crosshair; --cursor-card: url(card.svg), pointer; --breakpoint: 41rem; --breakpoint-card: 43rem; --align-card: bottom; --will-change-card: opacity; --contain-card: paint; --container-card: 47rem; --max-width-screen-card: 53rem; --line-clamp-alias: var(--line-clamp-card); --aspect-alias: var(--aspect-card); --text-indent-alias: var(--text-indent-card); --z-index-alias: var(--z-index-card);'
const collisionDeclarations = namespaces.map((namespace, index) => `--${namespace}-2: ${index + 61}px; --${namespace}-1_5: ${index + 71}px; --${namespace}-2\\/3: ${index + 81}px; --${namespace}-none: ${index + 91}px;`).join(' ')
const themes = {
  empty: '', base: `@theme { ${declarations} }`, inline: `@theme inline { ${declarations} }`, reference: `@theme reference { ${declarations} }`, default: `@theme default { ${declarations} }`, staticTheme: `@theme static { ${declarations} }`,
  collision: `@theme { ${declarations} ${collisionDeclarations} }`, reset: `@theme { ${declarations} } @theme { ${namespaces.map(namespace => `--${namespace}-*: initial;`).join(' ')} }`,
  defaultPriority: `@theme { ${declarations} } @theme default { --spacing: 1rem; --line-clamp-card: 99; --text-indent-card: 101px; --cursor-card: wait; --breakpoint-card: 103rem; }`,
  noSpacing: '@theme { --spacing: initial; --text-indent-card: 107px; --spacing-2: 109px; }',
  prefix: `@theme prefix(tw) { ${declarations} }`, prefixInline: `@theme prefix(tw) inline { ${declarations} }`, prefixReference: `@theme prefix(tw) reference { ${declarations} }`,
}
const values = ['', 'card', 'alias', 'unknown', 'auto', 'none', 'normal', 'px', 'square', 'disc', 'decimal', 'video', 'pointer', 'size', 'paint', 'scroll', '0', '0.25', '1.5', '2', '45', '50', '125', '01', '1.0', '.5', '-1', '1e+21', 'Infinity', '0%', '50%', '0.25%', '01%', '-1%', '[13px]', '[13]', '[50%]', '[0.25%]', '[13px_17px]', '[var(--probe)]', '(--probe)', '(color:--probe)', '(length:--probe)', '[color:var(--probe)]', '[length:var(--probe)]', '[number:var(--probe)]', '[percentage:var(--probe)]', '[nonsense:var(--probe)]', '[image:var(--probe)]', '[url:var(--probe)]', '[red]', '[#123456]', '[color:13px]', '[length:red]', '[calc(1px+2px)]', '[0]', '[size]', '[normal]', '[unset]', '[10/9]', '[3_/_5]', '[url(cursor.svg),_pointer]', '["a_b"]']
const modifiers = ['card', 'unknown', '2', '3', '0', '01', '-1', '1.5', '[name]', '[a_b]', '[var(--name)]', '(--name)', '(bad)', '[]', '[;]']

export async function miscellaneousFixtures() {
  const inventory = JSON.parse(await readFile(resolve(directory, 'inventory.json'), 'utf8'))
  assert.deepEqual(inventory.utilities.filter(item => item.kind === 'functional' && roots.includes(item.name)).map(item => item.name).sort(), [...roots].sort())
  const staticRoots = inventory.utilities.filter(item => item.kind === 'static' && /^(?:line-clamp|list|indent|-indent|align|z|aspect|cursor|will-change|contain)-/.test(item.name)).map(item => item.name)
  const cases = []
  async function add(id, mode, candidates) {
    const input = `${themes[mode]} @tailwind utilities;`.trim()
    const compiler = await tailwind.compile(input)
    const expectedCss = compiler.build(candidates).replace(/^\/\*! tailwindcss[^\n]*\n/, '')
    cases.push({ id, input, candidates, expectedCss, expectedCssSha256: sha256(expectedCss) })
  }
  for (const root of roots) {
    for (const mode of ['empty', 'base', 'inline', 'reference', 'default']) for (const value of values) await add(`misc:${root}:${value || '<default>'}:${mode}`, mode, [`${root}${value ? `-${value}` : ''}`])
    for (const mode of ['collision', 'reset', 'defaultPriority', 'noSpacing']) for (const value of ['', 'card', 'alias', '2', '1.5', '2/3', 'none', 'px', '[var(--probe)]']) await add(`misc:${root}:${value || '<default>'}:${mode}`, mode, [`${root}${value ? `-${value}` : ''}`])
    for (const mode of ['base', 'inline', 'reference', 'collision']) for (const value of ['', 'card', '2', '[13px]', '[var(--probe)]']) for (const modifier of modifiers) await add(`misc:${root}:${value || '<default>'}/${modifier}:${mode}:modifier`, mode, [`${root}${value ? `-${value}` : ''}/${modifier}`])
    for (const mode of ['prefix', 'prefixInline', 'prefixReference']) for (const candidate of [`tw:${root}`, `tw:${root}-card`, `tw:${root}-2`, `tw:${root}-[var(--probe)]`, `tw:hover:${root}-[13px]!`, `tw:${root}-2/3`, `${root}-card`]) await add(`misc:${candidate}:${mode}`, mode, [candidate])
    for (const value of ['1/2', '0/0', '2/3', '2/[3]', '2/(--probe)', '2/3/4', '01/2', '2/-3', '-2/3', '2.5/3']) await add(`misc:${root}-${value}:fraction`, 'base', [`${root}-${value}`])
    if (!root.startsWith('-')) await add(`misc:${root}:negative-boundary`, 'base', [`-${root}`, `-${root}-card`, `-${root}-[13px]`])
    await add(`misc:${root}:hover-important`, 'base', [`hover:${root}-[var(--probe)]!`])
  }
  for (const root of ['indent', '-indent']) for (const mode of ['base', 'noSpacing', 'inline', 'reference']) for (let number = 0; number <= 8; number += .25) await add(`misc:${root}-${number}:${mode}:spacing`, mode, [`${root}-${number}`])
  for (const root of ['z', '-z', 'line-clamp']) for (let number = 0; number <= 50; number++) await add(`misc:${root}-${number}:integer`, 'empty', [`${root}-${number}`])
  for (const root of staticRoots) {
    for (const mode of ['empty', 'base', 'inline', 'reference', 'collision', 'staticTheme']) await add(`misc:static:${root}:${mode}`, mode, [root])
    for (const mode of ['prefix', 'prefixInline', 'prefixReference']) await add(`misc:static:tw:${root}:${mode}`, mode, [`tw:${root}`])
    for (const candidate of [`-${root}`, `${root}/foo`, `hover:${root}!`]) await add(`misc:static:${candidate}:boundary`, 'base', [candidate])
  }
  const themeCss = await readFile(resolve(directory, '../../../ref/tailwindcss/packages/tailwindcss/theme.css'), 'utf8')
  const defaultTokens = [...themeCss.matchAll(/--(?:spacing|aspect-[a-z0-9-]+|breakpoint-[a-z0-9-]+)\s*:[^;]+;/g)].map(match => match[0])
  assert.equal(defaultTokens.length, 7)
  themes.upstreamDefault = `@theme default { ${defaultTokens.join(' ')} }`
  for (const root of roots) await add(`misc:${root}:upstream-default`, 'upstreamDefault', ['', 'video', 'sm', 'md', 'lg', 'xl', '2xl', '0', '1.5', '2', '1/2', 'px', '[var(--probe)]'].map(value => `${root}${value ? `-${value}` : ''}`))
  await add('misc:source-order-union', 'base', [...staticRoots, ...roots.flatMap(root => [`${root}-card`, `${root}-[var(--probe)]`, `hover:${root}-[13px]!`]), '@container/sidebar', '@container-normal/[a_b]', 'aspect-2/3', 'max-w-screen-card/ignored'])
  await add('misc:spacing-and-fraction-namespace-collisions', 'collision', ['indent-2', '-indent-2', 'indent-2/3', '-indent-2/3', 'z-2/3', '-z-2/3', 'aspect-2/3', 'line-clamp-2/3', 'list-2/3', 'cursor-2/3', 'max-w-screen-2/3', 'align-card', 'will-change-card', 'contain-card'])
  const ids = new Set(); for (const item of cases) { assert.ok(!ids.has(item.id), `duplicate ${item.id}`); ids.add(item.id) }
  const sources = ['packages/tailwindcss/src/utilities.ts', 'packages/tailwindcss/src/utilities.test.ts', 'packages/tailwindcss/src/compat/legacy-utilities.ts', 'packages/tailwindcss/src/theme.ts', 'packages/tailwindcss/src/utils/infer-data-type.ts', 'packages/tailwindcss/src/property-order.ts', 'packages/tailwindcss/src/ast.ts'].map(path => { const source = inventory.sources.find(item => item.path === path); assert.ok(source?.sha256, path); return { path, sha256: source.sha256 } })
  sources.push({ path: 'packages/tailwindcss/theme.css', sha256: sha256(themeCss) })
  return { schemaVersion: 1, sourceCommit: '1334c99db8fd26c8ea065375dd9259800863f072', oracleVersion: '4.1.13', functionalRoots: roots, staticRegressionRoots: staticRoots, fullFamilyConformance: 'pending', defaultTheme: 'explicit fixed CSS theme subset: spacing, aspect-video and five breakpoints; no implicit full theme', sources, cases }
}

export async function checkMiscellaneousFixtures() {
  const actual = await miscellaneousFixtures()
  assert.deepEqual(actual, JSON.parse(await readFile(resolve(directory, 'miscellaneous-fixtures.json'), 'utf8')))
  return { functionalRoots: roots.length, staticRegressionRoots: actual.staticRegressionRoots.length, cases: actual.cases.length }
}
if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  if (process.argv.includes('--write')) { const actual = await miscellaneousFixtures(); await writeFile(resolve(directory, 'miscellaneous-fixtures.json'), JSON.stringify(actual, null, 2) + '\n'); console.log(JSON.stringify({ functionalRoots: roots.length, staticRegressionRoots: actual.staticRegressionRoots.length, cases: actual.cases.length })) }
  else console.log(JSON.stringify(await checkMiscellaneousFixtures()))
}
