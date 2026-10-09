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
const roots = ['border', 'border-x', 'border-y', 'border-s', 'border-e', 'border-t', 'border-r', 'border-b', 'border-l', 'divide-x', 'divide-y', 'outline', 'outline-offset', '-outline-offset', 'stroke', 'decoration', 'underline-offset', '-underline-offset', 'opacity']
const namespaces = ['border-width', 'divide-width', 'outline-width', 'outline-offset', 'stroke-width', 'text-decoration-thickness', 'text-underline-offset', 'opacity']
const declarations = namespaces.map((key, index) => `--${key}-card: ${key === 'opacity' ? '.35' : key === 'stroke-width' ? '3' : `${index + 2}px`}; --${key}-1_5: ${key === 'opacity' ? '.65' : '15px'};`).join(' ') + ' --color-brand: #123456; --border-color-brand: #234567; --outline-color-brand: #345678; --text-decoration-color-brand: #456789; --stroke-brand: #56789a; --opacity-muted: .35;'
const themes = {
  empty: '', base: `@theme { ${declarations} }`, inline: `@theme inline { ${declarations} }`, reference: `@theme reference { ${declarations} }`,
  priority: `@theme { ${declarations} --color-card: #abcdef; --color-2: red; --color-2.5: blue; }`,
  borderFallback: '@theme { --border-width-card: 7px; }',
  defaults: '@theme { --default-border-width: 3px; --default-outline-width: 5px; --opacity: .45; --outline-offset: 4px; --text-underline-offset: 7px; --divide-width: 9px; --border-width: 11px; --outline-width: 13px; --stroke-width: 2; --text-decoration-thickness: 3px; }',
  defaultsInline: '@theme inline { --default-border-width: 3px; --default-outline-width: 5px; --opacity: .45; --outline-offset: 4px; --text-underline-offset: 7px; }',
  defaultsReference: '@theme reference { --default-border-width: 3px; --default-outline-width: 5px; --opacity: .45; --outline-offset: 4px; --text-underline-offset: 7px; }',
  aliases: '@theme { --source-width: 8px; --default-border-width: var(--source-width); --default-outline-width: var(--source-width); --border-width-card: var(--source-width); --divide-width-card: var(--source-width); --outline-width-card: var(--source-width); --outline-offset-card: var(--source-width); --stroke-width-card: var(--source-width); --text-decoration-thickness-card: var(--source-width); --text-underline-offset-card: var(--source-width); --opacity-card: var(--source-opacity); --source-opacity: .35; }',
  fractions: '@theme { --divide-width-card\\/wide: 3px; --outline-offset-card\\/wide: 4px; --text-underline-offset-card\\/wide: 5px; --opacity-card\\/wide: .35; --border-width-card\\/wide: 6px; --outline-width-card\\/wide: 7px; --stroke-width-card: 2; --text-decoration-thickness-card\\/wide: 8px; }',
  reset: `@theme { ${declarations} } @theme { ${namespaces.map(key => `--${key}-*: initial;`).join(' ')} }`,
  defaultPriority: `@theme { ${declarations} } @theme default { --border-width-card: 50px; --opacity-card: .75; }`,
  slashDefault: '@theme { --default-border-width: calc(1px / 2); --default-outline-width: calc(3px / 2); --border-width-card: calc(1px / 2); --outline-offset-card: calc(1px / 2); --opacity-card: calc(1 / 2); }',
}
const values = ['card', '1.5', 'brand', 'current', 'inherit', 'transparent', '0', '1', '2', '2.5', '0.25', '25', '50', '100', '125', '1e+21', '1e21', '50.0', '01', '-0', '-2', 'unknown', '1/2', '[2px]', '[50%]', '[2]', '[thin]', '[thin_thick]', '[medium]', '[var(--border-width-card)]', '(--border-width-card)', '(length:--border-width-card)', '[length:var(--border-width-card)]', '[number:var(--probe)]', '[percentage:var(--probe)]', '[line-width:var(--probe)]', '[color:13px]', '[nonsense:2px]', '[calc(1px_+_2px)]', '[#123456]', '[red]', '[var(--missing)]', '[inherit]']
const modifiers = ['50', 'unknown', 'muted', '[.5]', '[2]', '(--opacity-muted)', '[]', '[;]', '(bad)', 'wide', '2/foo']
export async function borderFixtures() {
  const cases = []
  async function add(id, mode, candidates) {
    const input = `${themes[mode]} @tailwind utilities;`.trim()
    const compiler = await tailwind.compile(input)
    const expectedCss = banner(compiler.build(candidates))
    cases.push({ id, input, candidates, expectedCss, expectedCssSha256: sha256(expectedCss) })
  }
  for (const root of roots) {
    for (const mode of ['empty', 'base', 'inline', 'reference']) {
      for (const value of values) await add(`border:${root}-${value}:${mode}`, mode, [`${root}-${value}`])
      await add(`border:${root}:bare:${mode}`, mode, [root])
      await add(`border:${root}:bare-modifier:${mode}`, mode, [`${root}/50`])
    }
    for (const value of ['card', '2', '[2px]']) for (const modifier of modifiers) await add(`border:${root}-${value}/${modifier}:base`, 'base', [`${root}-${value}/${modifier}`])
    for (const mode of ['priority', 'borderFallback', 'defaults', 'defaultsInline', 'defaultsReference', 'aliases', 'fractions', 'reset', 'defaultPriority', 'slashDefault']) for (const value of ['', '-card', '-card/wide', '-card/[2]', '-2', '-2/unknown']) await add(`border:${root}${value}:${mode}`, mode, [`${root}${value}`])
    await add(`border:${root}:important`, 'base', [`${root}-${root === 'opacity' ? '50' : '2'}!`])
    await add(`border:${root}:hover`, 'base', [`hover:${root}-${root === 'opacity' ? '50' : '2'}`])
    if (!root.startsWith('-')) await add(`border:${root}:negative`, 'base', [`-${root}-2`])
  }
  for (let numerator = 0; numerator <= 400; numerator++) await add(`border:opacity-${numerator / 4}:quarter-range`, 'empty', [`opacity-${numerator / 4}`])
  for (const root of ['border', 'outline', 'stroke', 'decoration', 'divide-x', 'divide-y', 'outline-offset', '-outline-offset', 'underline-offset', '-underline-offset']) for (const value of ['0', '1', '2', '3', '4', '8', '123', '0.5', '1e+22', 'Infinity', '0x2', '+2']) await add(`border:${root}-${value}:bare-number-boundary`, 'empty', [`${root}-${value}`])
  for (const unit of ['cm', 'mm', 'Q', 'in', 'pc', 'pt', 'px', 'em', 'ex', 'ch', 'rem', 'lh', 'rlh', 'vw', 'vh', 'vmin', 'vmax', 'vb', 'vi', 'svw', 'svh', 'lvw', 'lvh', 'dvw', 'dvh', 'cqw', 'cqh', 'cqi', 'cqb', 'cqmin', 'cqmax']) for (const root of ['border', 'outline', 'stroke', 'decoration']) await add(`border:${root}-[13${unit}]:length-unit`, 'empty', [`${root}-[13${unit}]`])
  await add('border:sorted-union', 'base', ['border', 'border-2', 'border-x-4', 'border-brand/50', 'border-dashed', 'outline', 'outline-2', 'outline-dotted', 'outline-offset-4', '-outline-offset-2', 'stroke-2', 'stroke-brand', 'decoration-2', 'decoration-brand', 'underline-offset-4', '-underline-offset-2', 'opacity-50', 'hover:border-4!', 'divide-x-2', 'divide-y-4', 'divide-brand', 'divide-x-reverse'])
  await add('border:divide-ordered-resources', 'base', ['divide-x-2', 'divide-y-4', 'divide-x-reverse', 'divide-y-reverse', 'divide-dashed', 'border-2'])
  assert.equal(new Set(cases.map(item => item.id)).size, cases.length)
  const inventory = JSON.parse(await readFile(resolve(directory, 'inventory.json'), 'utf8'))
  const sources = ['packages/tailwindcss/src/utilities.ts', 'packages/tailwindcss/src/utilities.test.ts', 'packages/tailwindcss/src/theme.ts', 'packages/tailwindcss/src/utils/infer-data-type.ts', 'packages/tailwindcss/src/ast.ts'].map(path => { const source = inventory.sources.find(item => item.path === path); assert.ok(source?.sha256, path); return { path, sha256: source.sha256 } })
  const upstreamTheme = await readFile(resolve(directory, '../../../ref/tailwindcss/packages/tailwindcss/theme.css'), 'utf8')
  const defaultNamespaceTokens = [...upstreamTheme.matchAll(/--(?:default-border-width|default-outline-width|border-width|divide-width|outline-width|outline-offset|stroke-width|text-decoration-thickness|text-underline-offset|opacity)(?:-[\w-]+)?\s*:/g)].map(match => match[0])
  assert.deepEqual(defaultNamespaceTokens, [])
  sources.push({ path: 'packages/tailwindcss/theme.css', sha256: sha256(upstreamTheme) })
  return { schemaVersion: 1, defaultNamespaceTokens, sourceCommit: '1334c99db8fd26c8ea065375dd9259800863f072', oracleVersion: '4.1.13', functionalRoots: roots, fullFamilyConformance: 'pending', themeDefaults: 'explicit input only; upstream theme.css has no tokens in these width/offset/opacity namespaces', sources, cases }
}
export async function checkBorderFixtures() {
  const actual = await borderFixtures()
  assert.deepEqual(actual, JSON.parse(await readFile(resolve(directory, 'borders-fixtures.json'), 'utf8')))
  return { functionalRoots: roots.length, cases: actual.cases.length }
}
if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  if (process.argv.includes('--write')) { const actual = await borderFixtures(); await writeFile(resolve(directory, 'borders-fixtures.json'), JSON.stringify(actual, null, 2) + '\n'); console.log(JSON.stringify({ functionalRoots: roots.length, cases: actual.cases.length })) }
  else console.log(JSON.stringify(await checkBorderFixtures()))
}
