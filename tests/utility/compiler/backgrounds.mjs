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
const roots = ['bg', 'bg-position', 'bg-size']
const declarations = '--background-image-card: url(test_file.png); --background-image-gradient: linear-gradient(red, blue); --background-image-1_5: url(dot.png); --background-image-source: url(source.png); --background-image-alias: var(--background-image-source); --background-image-missing: var(--missing); --background-image-current: url(current.png); --background-image-inherit: url(inherit.png); --background-image-transparent: url(transparent.png); --background-position-card: left top; --background-size-card: 50%; --background-position: center; --background-size: cover; --background-image: url(default.png); --color-brand: #123456; --opacity-muted: .35;'
const themes = {
  empty: '', base: `@theme { ${declarations} }`, inline: `@theme inline { ${declarations} }`, reference: `@theme reference { ${declarations} }`, default: `@theme default { ${declarations} }`,
  colorPriority: `@theme { ${declarations} --color-card: red; --background-color-card: blue; }`,
  overlap: '@theme { --background-image-size-card: url(size.png); --background-image-position-card: url(position.png); --background-image-size: url(size-default.png); --background-image-position: url(position-default.png); }',
  reset: `@theme { ${declarations} } @theme { --background-image-*: initial; }`,
  defaultPriority: `@theme { ${declarations} } @theme default { --background-image-card: url(ignored.png); }`,
  whitespace: '@theme { --background-image-card: linear-gradient(red,\r\n blue) /* image */; }',
  prefix: `@theme prefix(tw) { ${declarations} }`, prefixInline: `@theme prefix(tw) inline { ${declarations} }`, prefixReference: `@theme prefix(tw) reference { ${declarations} }`,
}
const arbitrary = ['[url(test.png)]', '[url(test_file.png)]', '[url("test_file.png")]', '[url(test.png),url(other.png)]', '[url(test.png),var(--background-image-card)]', '[linear-gradient(red,blue)]', '[radial-gradient(red,blue)]', '[conic-gradient(red,blue)]', '[repeating-linear-gradient(red,blue)]', '[repeating-radial-gradient(red,blue)]', '[repeating-conic-gradient(red,blue)]', '[image-set(url(test.png)_1x)]', '[image(url(test.png))]', '[cross-fade(url(test.png),url(other.png),50%)]', '[element(#id)]', '[var(--background-image-card)]', '(--background-image-card)', '(image:--background-image-card)', '[image:var(--background-image-card)]', '[url:var(--background-image-card)]', '[length:var(--background-size-card)]', '[size:var(--background-size-card)]', '[bg-size:var(--background-size-card)]', '[percentage:var(--background-position-card)]', '[position:var(--background-position-card)]', '[red]', '[#123456]', '[currentColor]', '[color:var(--color-brand)]', '[color:13px]', '[number:13px]', '[nonsense:url(test.png)]', '[13px]', '[50%]', '[0]', '[2]', '[auto]', '[cover]', '[contain]', '[center]', '[top]', '[left_top]', '[top_left]', '[right_13px_bottom_50%]', '[13px_50%]', '[auto_auto]', '[auto_13px]', '[cover,contain]', '[13px,50%]', '[center,var(--probe)]', '[var(--probe),center]', '[calc(1rem_+_2px)]', '[13PX]', '[url(test.png)_center]', '[foo]', '[length:red]']
const modifiers = ['50', 'muted', 'unknown', '[.5]', '[50%]', '(--opacity-muted)', 'wide', '[]', '[;]', '(bad)', '25/foo']
export async function backgroundFixtures() {
  const cases = []
  async function add(id, mode, candidates) {
    const input = `${themes[mode]} @tailwind utilities;`.trim()
    const compiler = await tailwind.compile(input)
    const expectedCss = banner(compiler.build(candidates))
    cases.push({ id, input, candidates, ...(mode.startsWith('prefix') ? { branch: 'pending_feature', expectedPendingFeature: '@theme prefix(' } : {}), expectedCss, expectedCssSha256: sha256(expectedCss) })
  }
  for (const root of roots) {
    for (const mode of ['empty', 'base', 'inline', 'reference', 'default']) {
      for (const value of ['card', 'gradient', 'alias', 'missing', '1.5', 'brand', 'current', 'inherit', 'transparent', 'unknown', '50', ...arbitrary]) await add(`background:${root}-${value}:${mode}`, mode, [`${root}-${value}`])
      await add(`background:${root}:bare:${mode}`, mode, [root])
      await add(`background:${root}:bare-modifier:${mode}`, mode, [`${root}/50`])
    }
    for (const value of ['card', '[url(test.png)]', '[13px]', '[cover]', '[var(--color-brand)]']) for (const modifier of modifiers) await add(`background:${root}-${value}/${modifier}:base`, 'base', [`${root}-${value}/${modifier}`])
    for (const mode of ['colorPriority', 'overlap', 'reset', 'defaultPriority', 'whitespace']) for (const value of ['', '-card', '-card/50', '-card/muted', '-card/unknown']) await add(`background:${root}${value}:${mode}`, mode, [`${root}${value}`])
    for (const mode of ['prefix', 'prefixInline', 'prefixReference']) for (const candidate of [`tw:${root}-card`, `tw:${root}-[url(test.png)]`, `tw:${root}-[13px]`, `tw:hover:${root}-[13px]!`, `${root}-card`, `hover:tw:${root}-card`]) await add(`background:${candidate}:${mode}`, mode, [candidate])
    await add(`background:${root}:important`, 'base', [`${root}-[13px]!`])
    await add(`background:${root}:hover`, 'base', [`hover:${root}-[13px]`])
    await add(`background:${root}:negative`, 'base', [`-${root}-[13px]`])
  }
  for (const unit of ['cm', 'mm', 'Q', 'in', 'pc', 'pt', 'px', 'em', 'ex', 'ch', 'rem', 'lh', 'rlh', 'vw', 'vh', 'vmin', 'vmax', 'vb', 'vi', 'svw', 'svh', 'lvw', 'lvh', 'dvw', 'dvh', 'cqw', 'cqh', 'cqi', 'cqb', 'cqmin', 'cqmax']) await add(`background:bg-[13${unit}]:length-unit`, 'empty', [`bg-[13${unit}]`])
  await add('background:sorted-union', 'base', ['bg-card', 'bg-alias', 'bg-[url(test.png)]', 'bg-[center]', 'bg-[13px]', 'bg-size-[cover]', 'bg-position-[left_top]', 'bg-brand/50', 'hover:bg-[50%]!', 'bg-none', 'bg-cover', 'bg-center'])
  assert.equal(new Set(cases.map(item => item.id)).size, cases.length)
  const inventory = JSON.parse(await readFile(resolve(directory, 'inventory.json'), 'utf8'))
  const sources = ['packages/tailwindcss/src/utilities.ts', 'packages/tailwindcss/src/utilities.test.ts', 'packages/tailwindcss/src/theme.ts', 'packages/tailwindcss/src/utils/infer-data-type.ts', 'packages/tailwindcss/src/ast.ts'].map(path => { const source = inventory.sources.find(item => item.path === path); assert.ok(source?.sha256, path); return { path, sha256: source.sha256 } })
  return { schemaVersion: 1, sourceCommit: '1334c99db8fd26c8ea065375dd9259800863f072', oracleVersion: '4.1.13', functionalRoots: roots, deferredGradientRoots: ['bg-linear', '-bg-linear', 'bg-conic', '-bg-conic', 'bg-radial', 'from', 'via', 'to'], fullFamilyConformance: 'pending', sources, cases }
}
export async function checkBackgroundFixtures() {
  const actual = await backgroundFixtures()
  assert.deepEqual(actual, JSON.parse(await readFile(resolve(directory, 'backgrounds-fixtures.json'), 'utf8')))
  return { functionalRoots: roots.length, cases: actual.cases.length, pendingFeatureCases: actual.cases.filter(item => item.branch === 'pending_feature').length }
}
if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  if (process.argv.includes('--write')) { const actual = await backgroundFixtures(); await writeFile(resolve(directory, 'backgrounds-fixtures.json'), JSON.stringify(actual, null, 2) + '\n'); console.log(JSON.stringify({ functionalRoots: roots.length, cases: actual.cases.length })) }
  else console.log(JSON.stringify(await checkBackgroundFixtures()))
}
