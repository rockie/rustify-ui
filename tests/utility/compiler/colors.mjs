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
const roots = ['bg', 'text', 'border', 'border-x', 'border-y', 'border-s', 'border-e', 'border-t', 'border-r', 'border-b', 'border-l', 'outline', 'decoration', 'accent', 'caret', 'fill', 'stroke', 'divide', 'placeholder']
const colorProperties = new Set(['background-color', 'color', 'border-color', 'border-inline-color', 'border-block-color', 'border-inline-start-color', 'border-inline-end-color', 'border-top-color', 'border-right-color', 'border-bottom-color', 'border-left-color', 'outline-color', 'text-decoration-color', 'accent-color', 'caret-color', 'fill', 'stroke'])
const primaryNamespaces = ['background-color', 'text-color', 'border-color', 'outline-color', 'text-decoration-color', 'accent-color', 'caret-color', 'fill', 'stroke', 'divide-color']
const declarations = '--color-brand: #123456; --color-aliased: var(--color-brand); --color-missing: var(--missing-color); --color-current: red; --color-inherit: blue; --color-transparent: green; --color-1_5: purple; --opacity-muted: .35; --opacity-1_5: .65;'
const primary = primaryNamespaces.map((key, i) => `--${key}-brand: #${(0x234567 + i * 0x111111).toString(16)};`).join(' ')
const themes = {
  empty: '',
  base: `@theme { ${declarations} }`,
  primary: `@theme { ${declarations} ${primary} }`,
  inline: `@theme inline { ${declarations} ${primary} }`,
  reference: `@theme reference { ${declarations} ${primary} }`,
  divideFallback: '@theme { --border-color-brand: #456789; --color-brand: #123456; }',
  secondary: '@theme { --background-image-card: linear-gradient(red, blue); --text-card: 17px; --text-card--line-height: 1.5; --border-width-card: 7px; --outline-width-card: 9px; --text-decoration-thickness-card: 3px; --stroke-width-card: 2; --leading-tight: 1.25; --spacing: 4px; }',
  decorationPriority: '@theme { --text-decoration-thickness-brand: 3px; --text-decoration-color-brand: #123456; --color-2: red; }',
}
const arbitrary = ['[#123456]', '[#12345680]', '[red]', '[ReBeccAPurPle]', '[CanvasText]', '[currentColor]', '[inherit]', '[rgb(10_20_30)]', '[rgba(10,20,30,.4)]', '[hsl(120_30%_40%)]', '[hsla(120,30%,40%,.4)]', '[hwb(120_30%_40%)]', '[oklch(.5_.2_30)]', '[oklab(.5_.2_.3)]', '[lab(40_20_30)]', '[lch(40_20_30)]', '[color(display-p3_1_0_0)]', '[light-dark(red,blue)]', '[color-mix(in_oklab,red_50%,transparent)]', '[var(--color-brand)]', '(--color-brand)', '(color:--color-brand)', '[color:var(--probe)]', '[nonsense:red]', '[13px]', '[50%]', '[2]', '[length:var(--probe)]', '[color:13px]', '[calc(1rem_+_2px)]']
const alphas = ['0', '0.25', '1.5', '25', '50', '100', '125', '50.0', '01', '-25', '.5', 'unknown', 'muted', '[.35]', '[35%]', '[var(--opacity-muted)]', '(--opacity-muted)', '[calc(.5_*_2)]', '[0x1]', '[Infinity]', '[1e-9]', '[1e20]', '[.29]', '[0o1]', '[0b1]', '[-.5]', '[.5_]', '[]', '[;]', '(bad)', '25/foo']
export async function colorFixtures() {
  const cases = []
  const designs = new Map()
  async function add(id, mode, candidates) {
    const input = `${themes[mode]} @tailwind utilities;`.trim()
    const compiler = await tailwind.compile(input)
    const expectedCss = banner(compiler.build(candidates))
    if (!designs.has(mode)) designs.set(mode, await tailwind.__unstable__loadDesignSystem(input))
    const design = designs.get(mode)
    let color = false, other = false
    const expectedProperties = new Set()
    function inspect(nodes) {
      for (const node of nodes) {
        if (node.kind === 'at-root') continue
        if (node.kind === 'declaration' && node.property !== '--tw-sort' && node.value !== undefined) {
          if (!node.property.startsWith('--')) expectedProperties.add(node.property)
          if (colorProperties.has(node.property)) color = true
          else if (!node.property.startsWith('--')) other = true
        }
        if (node.nodes) inspect(node.nodes)
      }
    }
    for (const candidate of candidates) for (const parsed of design.parseCandidate(candidate)) for (const compiled of design.compileAstNodes(parsed, 1)) inspect([compiled.node])
    const branch = color ? 'color' : other ? 'other_interpretation' : 'invalid'
    assert.ok(!(color && other), `${id}: mixed interpretations require a separate fixture`)
    cases.push({ id, input, candidates, branch, expectedProperties: [...expectedProperties].sort(), expectedCss, expectedCssSha256: sha256(expectedCss) })
  }
  for (const root of roots) {
    for (const value of ['brand', 'aliased', 'missing', 'current', 'inherit', 'transparent', '1.5', 'unknown']) await add(`color:${root}-${value}:primary`, 'primary', [`${root}-${value}`])
    for (const value of arbitrary) await add(`color:${root}-${value}:base`, 'base', [`${root}-${value}`])
    for (const alpha of alphas) {
      await add(`color:${root}-brand/${alpha}:primary`, 'primary', [`${root}-brand/${alpha}`])
      await add(`color:${root}-[#123456]/${alpha}:base`, 'base', [`${root}-[#123456]/${alpha}`])
    }
    for (const mode of ['empty', 'base', 'inline', 'reference']) for (const value of ['brand', 'current', 'transparent', '[var(--color-brand)]']) await add(`color:${root}-${value}/50:${mode}`, mode, [`${root}-${value}/50`])
    await add(`color:${root}:missing-value`, 'primary', [root])
    await add(`color:${root}:negative`, 'primary', [`-${root}-brand`])
    await add(`color:${root}:important`, 'primary', [`${root}-brand/50!`])
    await add(`color:${root}:hover`, 'primary', [`hover:${root}-brand/50`])
    await add(`color:${root}:secondary`, 'secondary', [`${root}-card`])
  }
  for (const candidate of ['divide-brand', 'divide-brand/50', 'placeholder-brand/50']) await add(`color:${candidate}:fallback`, 'divideFallback', [candidate])
  for (const candidate of ['decoration-brand', 'decoration-brand/50', 'decoration-2', 'decoration-2/50']) await add(`color:${candidate}:priority`, 'decorationPriority', [candidate])
  for (const candidate of ['bg-[url(test.png)]', 'bg-[linear-gradient(red,blue)]', 'bg-[cover]', 'bg-[center]', 'text-[small]', 'text-[larger]', 'text-card/none', 'text-card/tight', 'text-card/[2]', 'text-card/4', 'text-card/unknown', 'stroke-2/unknown', 'stroke-card/unknown', 'stroke-[13px]/50', 'stroke-[2]/50', 'stroke-[50%]/[.5]', 'border-[13px]/50', 'outline-[13px]/50', 'decoration-[13px]/50', 'bg-[url(test.png)]/50', 'text-[13px]/unknown', 'text-[13px]/tight']) await add(`color:${candidate}:other-branch`, 'secondary', [candidate])
  await add('color:sorted-union', 'primary', roots.map(root => `${root}-brand/25`))
  assert.equal(new Set(cases.map(item => item.id)).size, cases.length)
  const inventory = JSON.parse(await readFile(resolve(directory, 'inventory.json'), 'utf8'))
  const sources = ['packages/tailwindcss/src/utilities.ts', 'packages/tailwindcss/src/utilities.test.ts', 'packages/tailwindcss/src/theme.ts', 'packages/tailwindcss/src/utils/infer-data-type.ts', 'packages/tailwindcss/src/utils/is-color.ts', 'packages/tailwindcss/src/ast.ts'].map(path => { const source = inventory.sources.find(item => item.path === path); assert.ok(source?.sha256, path); return { path, sha256: source.sha256 } })
  return { schemaVersion: 1, sourceCommit: '1334c99db8fd26c8ea065375dd9259800863f072', oracleVersion: '4.1.13', colorBranchRoots: roots, fullFamilyConformance: 'pending', branchBoundaries: ['background image/position/size', 'text font size and line height', 'border/outline width', 'decoration thickness priority', 'stroke width'], sources, cases }
}
export async function checkColorFixtures() {
  const actual = await colorFixtures()
  assert.deepEqual(actual, JSON.parse(await readFile(resolve(directory, 'colors-fixtures.json'), 'utf8')))
  return { colorBranchRoots: roots.length, cases: actual.cases.length, branches: Object.fromEntries(['color', 'invalid', 'other_interpretation'].map(branch => [branch, actual.cases.filter(item => item.branch === branch).length])) }
}
if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  if (process.argv.includes('--write')) { const actual = await colorFixtures(); await writeFile(resolve(directory, 'colors-fixtures.json'), JSON.stringify(actual, null, 2) + '\n'); console.log(JSON.stringify({ colorBranchRoots: roots.length, cases: actual.cases.length })) }
  else console.log(JSON.stringify(await checkColorFixtures()))
}
