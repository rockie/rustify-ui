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
const roots = ['font', 'text', 'leading', 'tracking', '-tracking', 'font-stretch']
const declarations = '--font-card: ui-sans-serif, system-ui, sans-serif; --font-card--font-feature-settings: "cv01"; --font-card--font-variation-settings: "wght" 650; --font-weight-card: 650; --font-weight-bold: 700; --font-weight-500: 500; --font-1_5: monospace; --font-weight-1_5: 350; --text-card: 17px; --text-card--line-height: 1.5; --text-card--letter-spacing: .02em; --text-card--font-weight: 600; --text-1_5: 15px; --leading-tight: 1.25; --leading-card: 1.375; --leading-4: 19px; --leading-1_5: 1.5; --tracking-card: .025em; --tracking-wide: .05em; --tracking-1_5: .015em; --spacing: 4px; --spacing-card: 23px; --color-brand: #123456;'
const themes = {
  empty: '',
  base: `@theme { ${declarations} }`,
  inline: `@theme inline { ${declarations} }`,
  reference: `@theme reference { ${declarations} }`,
  noSpacing: '@theme { --text-card: 17px; --text-card--line-height: 1.5; --leading-card: 1.375; }',
  defaults: '@theme { --font: serif; --font-weight: 600; --text: 16px; --leading: 1.3; --tracking: .03em; --font-stretch: 100%; --spacing: 4px; }',
  aliases: '@theme { --font-source: serif; --font-card: var(--font-source); --font-card--font-feature-settings: var(--features); --font-card--font-variation-settings: var(--variation); --features: "cv01"; --variation: "wght" 700; --text-source: 20px; --text-card: var(--text-source); --text-card--line-height: var(--leading-card); --text-card--letter-spacing: var(--tracking-card); --text-card--font-weight: var(--font-weight-card); --leading-card: 1.5; --tracking-card: .02em; --font-weight-card: 600; }',
  mixed: '@theme { --font-card: serif; --text-card: 17px; --text-card--line-height: 1.5; } @theme inline { --font-card--font-feature-settings: "cv01"; --text-card--letter-spacing: .02em; } @theme reference { --font-card--font-variation-settings: "wght" 500; --text-card--font-weight: 650; }',
  mixedBase: '@theme inline { --font-card: serif; --text-card: 17px; } @theme { --font-card--font-feature-settings: "cv01"; --text-card--line-height: 1.5; } @theme reference { --font-card--font-variation-settings: "wght" 500; --text-card--letter-spacing: .02em; --text-card--font-weight: 650; }',
  colorPriority: '@theme { --color-card: #123456; --text-card: 17px; --text-card--line-height: 1.5; --leading-card: 1.375; --spacing: 4px; }',
  namespaceOverlap: '@theme { --font-weight-card: 650; --font-size-card: 22px; --text-color-card: red; --text-decoration-color-card: blue; --text-decoration-thickness-card: 2px; --text-indent-card: 3px; --text-shadow-card: 0 0 2px red; --text-underline-offset-card: 4px; }',
  fractions: '@theme { --font-card: serif; --font-card--font-feature-settings: "cv01"; --text-card: 17px; --leading-card\\/wide: 1.75; --tracking-card\\/wide: .05em; --leading-1\\/2: 2px; --tracking-1\\/2: .02em; --spacing-card\\/wide: 23px; }',
  reset: `@theme { ${declarations} } @theme { --font-*: initial; --text-*: initial; --leading-*: initial; --tracking-*: initial; --spacing-*: initial; }`,
  whitespace: '@theme { --font-card:\r\n  serif,\t  monospace; --font-card--font-feature-settings: normal /* feature */; --text-card: 17px; --text-card--line-height: 1.5/* leading */; --tracking-card: .02em; }',
  nonePriority: '@theme { --text-card: 17px; --leading-none: 1.75; --spacing: 4px; }',
  namespaceReset: '@theme static { --font-card: serif; --font-weight-card: 650; --font-size-card: 22px; --text-card: 17px; --text-color-card: red; --text-decoration-color-card: blue; --text-decoration-thickness-card: 2px; --text-indent-card: 3px; --text-shadow-card: 0 0 2px red; --text-underline-offset-card: 4px; --inset-card: 5px; --inset-shadow-card: 0 0 2px red; --inset-ring-card: 3px; } @theme { --font-*: initial; --text-*: initial; --inset-*: initial; }',
  defaultPriority: '@theme { --font-card: serif; --text-card: 17px; --leading-card: 1.375; --tracking-card: .02em; } @theme default { --font-card: monospace; --text-card: 20px; --leading-card: 2; --tracking-card: .04em; }',
}
const values = {
  font: ['card', 'bold', '500', '1.5', 'unknown', 'weight-card', 'size-card', '["arial_rounded"]', '[ui-sans-serif]', '[sans-serif]', '[Arial,serif]', '[var(--font-card)]', '(--font-card)', '(family-name:--font-card)', '[family-name:var(--font-card)]', '[generic-name:var(--font-card)]', '[number:var(--font-weight-card)]', '[100]', '[100.5]', '[.5]', '[1e2]', '[calc(100_+_200)]', '[nonsense:serif]', '[length:20px]', '[url(test.png)]', '[0foo]', '[var(--missing)]'],
  text: ['card', '1.5', 'unknown', 'brand', '[10px]', '[50%]', '[xx-small]', '[small]', '[medium]', '[xxx-large]', '[larger]', '[smaller]', '[length:var(--text-card)]', '(length:--text-card)', '[size:var(--text-card)]', '[percentage:var(--text-card)]', '[absolute-size:var(--text-card)]', '[relative-size:var(--text-card)]', '[clamp(1rem,var(--probe),3rem)]', '[calc(1rem_+_2px)]', '[13dvmin]', '[13Q]', '[13PX]', '[0]', '[2]', '[var(--text-card)]', '(--text-card)', '[color:var(--text-card)]', '[number:13px]', '[nonsense:13px]', '[var(--missing)]'],
  leading: ['card', 'tight', '4', '0', '0.25', '1.5', '2.75', '50.0', '01', '.5', '-1', '1e+21', 'unknown', '1/2', '[13px]', '[1.5]', '[var(--leading-card)]', '(--leading-card)', '[color:red]', '[length:var(--leading-card)]', '[calc(1_+_2)]'],
  tracking: ['card', 'wide', '1.5', 'unknown', '0', '1/2', '[.02em]', '[var(--tracking-card)]', '(--tracking-card)', '[length:var(--tracking-card)]', '[color:red]', '[number:2]', '[calc(.02em_+_.01em)]'],
  '-tracking': ['card', 'wide', '1.5', 'unknown', '0', '1/2', '[.02em]', '[var(--tracking-card)]', '(--tracking-card)', '[length:var(--tracking-card)]', '[color:red]', '[number:2]', '[calc(.02em_+_.01em)]'],
  'font-stretch': ['49%', '50%', '50.0%', '050%', '50.5%', '100%', '200%', '201%', '0x64%', '1e2%', '100', 'unknown', '[120%]', '[20%]', '[var(--stretch)]', '(--stretch)', '[color:red]', '[length:13px]', 'normal'],
}
const modifiers = ['card', 'tight', '4', '0', '0.25', '1.5', '2.75', '50.0', '01', '.5', 'none', 'unknown', '[2]', '[13px]', '[var(--leading-card)]', '(--leading-card)', '[]', '[;]', '(bad)', '1/2']
export async function typographyFixtures() {
  const cases = []
  const designs = new Map()
  async function add(id, mode, candidates) {
    const input = `${themes[mode]} @tailwind utilities;`.trim()
    const compiler = await tailwind.compile(input)
    const expectedCss = banner(compiler.build(candidates))
    if (!designs.has(mode)) designs.set(mode, await tailwind.__unstable__loadDesignSystem(input))
    const expectedProperties = new Set()
    function inspect(nodes) {
      for (const node of nodes) {
        if (node.kind === 'at-root') continue
        if (node.kind === 'declaration' && !node.property.startsWith('--') && node.value !== undefined) expectedProperties.add(node.property)
        if (node.nodes) inspect(node.nodes)
      }
    }
    const design = designs.get(mode)
    for (const candidate of candidates) for (const parsed of design.parseCandidate(candidate)) for (const compiled of design.compileAstNodes(parsed, 1)) inspect([compiled.node])
    const branch = expectedProperties.has('text-shadow') ? 'other_interpretation_pending' : 'typography'
    cases.push({ id, input, candidates, branch, expectedProperties: [...expectedProperties].sort(), expectedCss, expectedCssSha256: sha256(expectedCss) })
  }
  for (const root of roots) {
    for (const mode of ['empty', 'base', 'inline', 'reference']) for (const value of values[root]) await add(`typography:${root}-${value}:${mode}`, mode, [`${root}-${value}`])
    for (const mode of ['base', 'defaults']) {
      await add(`typography:${root}:bare:${mode}`, mode, [root])
      await add(`typography:${root}:bare-modifier:${mode}`, mode, [`${root}/card`])
    }
    for (const value of ['card', '[var(--probe)]']) for (const modifier of modifiers) await add(`typography:${root}-${value}/${modifier}:base`, 'base', [`${root}-${value}/${modifier}`])
    for (const mode of ['aliases', 'mixed', 'mixedBase', 'colorPriority', 'namespaceOverlap', 'fractions', 'reset', 'defaultPriority']) for (const value of ['card', 'card/wide', '1/2']) await add(`typography:${root}-${value}:${mode}`, mode, [`${root}-${value}`])
    await add(`typography:${root}:important`, 'base', [`${root}-${root === 'font-stretch' ? '[120%]' : 'card'}!`])
    await add(`typography:${root}:hover`, 'base', [`hover:${root}-${root === 'font-stretch' ? '[120%]' : 'card'}`])
    if (!root.startsWith('-')) await add(`typography:${root}:negative`, 'base', [`-${root}-card`])
  }
  for (const mode of ['base', 'noSpacing', 'colorPriority']) for (const value of ['card', '[13px]', '[length:var(--text-card)]']) for (const modifier of modifiers) await add(`typography:text-${value}/${modifier}:${mode}:size-modifier`, mode, [`text-${value}/${modifier}`])
  for (const value of ['weight-card', 'size-card', 'color-card', 'decoration-color-card', 'decoration-thickness-card', 'indent-card', 'shadow-card', 'underline-offset-card']) for (const root of ['font', 'text']) await add(`typography:${root}-${value}:namespaceOverlap`, 'namespaceOverlap', [`${root}-${value}`])
  for (const candidate of ['font-card', 'text-card', 'text-card/[2]', 'tracking-card']) await add(`typography:${candidate}:whitespace`, 'whitespace', [candidate])
  for (const candidate of ['text-card/none', 'text-[13px]/none', 'leading-none']) await add(`typography:${candidate}:none-priority`, 'nonePriority', [candidate])
  await add('typography:namespace-reset-static-tokens', 'namespaceReset', [])
  await add('typography:namespace-reset-weight-color', 'namespaceReset', ['font-card', 'text-card'])
  const defaultsPath = resolve(directory, '../../../ref/tailwindcss/packages/tailwindcss/theme.css')
  const defaultCss = await readFile(defaultsPath, 'utf8')
  const defaultDesign = await tailwind.__unstable__loadDesignSystem(defaultCss)
  // Theme.css is supplied explicitly; the Rust compiler must not inject implicit defaults.
  const defaultDeclarations = [...defaultCss.matchAll(/(--(?:font|font-weight|text|leading|tracking)-[\w-]+)\s*:\s*([^;]+);/g)].map(match => `${match[1]}: ${match[2]};`).join('\n')
  const defaultInput = `@theme default { ${defaultDeclarations} } @tailwind utilities;`
  for (const root of ['font', 'text', 'leading', 'tracking', '-tracking']) {
    const namespaces = root === 'font' ? ['--font', '--font-weight'] : [`--${root === 'text' ? 'text' : root.replace(/^-/, '')}`]
    for (const name of defaultDesign.theme.keysInNamespaces(namespaces)) {
      const compiler = await tailwind.compile(defaultInput)
      const candidates = [`${root}-${name}`]
      const expectedCss = banner(compiler.build(candidates))
      cases.push({ id: `typography:${root}-${name}:upstream-default`, input: defaultInput, candidates, branch: 'typography', expectedCss, expectedCssSha256: sha256(expectedCss) })
    }
  }
  for (const generic of ['serif', 'sans-serif', 'monospace', 'cursive', 'fantasy', 'system-ui', 'ui-serif', 'ui-sans-serif', 'ui-monospace', 'ui-rounded', 'math', 'emoji', 'fangsong']) await add(`typography:font-[${generic}]:generic-name`, 'empty', [`font-[${generic}]`])
  for (const size of ['xx-small', 'x-small', 'small', 'medium', 'large', 'x-large', 'xx-large', 'xxx-large', 'larger', 'smaller']) await add(`typography:text-[${size}]:keyword-size`, 'base', [`text-[${size}]`])
  for (const value of ['[-.5]', '[+500]', '[1.]', '[.5]', '[1e999]', '[Arial,var(--font-card)]', '[var(--font-card),serif]', '[var(--font-card),var(--missing)]', '[serif,123]', '["1font"]', '[font_weight]', '[sqrt(4)]']) await add(`typography:font-${value}:inference`, 'base', [`font-${value}`])
  for (const percentage of ['50%', '200%', '49%', '201%', '+100%', '-100%', '0b1100100%', '0o144%', '0X64%', '100.00%', '100.5%', '1e2%', 'Infinity%', 'NaN%', '%', '.5%', '100%/foo', '[50%]/[2]']) await add(`typography:font-stretch-${percentage}:number-boundary`, 'empty', [`font-stretch-${percentage}`])
  for (const unit of ['cm', 'mm', 'Q', 'in', 'pc', 'pt', 'px', 'em', 'ex', 'ch', 'rem', 'lh', 'rlh', 'vw', 'vh', 'vmin', 'vmax', 'vb', 'vi', 'svw', 'svh', 'lvw', 'lvh', 'dvw', 'dvh', 'cqw', 'cqh', 'cqi', 'cqb', 'cqmin', 'cqmax']) await add(`typography:text-[13${unit}]:length-unit`, 'empty', [`text-[13${unit}]`])
  for (let percentage = 50; percentage <= 200; percentage++) await add(`typography:font-stretch-${percentage}%:integer-range`, 'empty', [`font-stretch-${percentage}%`])
  for (const fn of ['calc', 'min', 'max', 'clamp', 'mod', 'rem', 'sin', 'cos', 'tan', 'asin', 'acos', 'atan', 'atan2', 'pow', 'sqrt', 'hypot', 'log', 'exp', 'round']) for (const root of ['font', 'text']) await add(`typography:${root}-[${fn}(2)]:math-inference`, 'empty', [`${root}-[${fn}(2)]`])
  for (const multiplier of ['1e+21', '1e21', '1000000000000000000000', '1e+22', 'Infinity', '-0', '0.00', '0.5', '0.3']) await add(`typography:text-card/${multiplier}:multiplier-boundary`, 'base', [`text-card/${multiplier}`])
  await add('typography:sorted-union', 'base', ['font-card', 'font-bold', 'font-[350]', 'text-card', 'text-card/4', 'text-[13px]', 'leading-card', 'leading-[2]', 'tracking-wide', '-tracking-card', 'font-stretch-[120%]', 'hover:text-card!', 'text-brand/50'])
  await add('typography:resources-dedup', 'base', ['font-bold', 'font-[350]', 'leading-4', 'leading-none', 'tracking-wide', '-tracking-card'])
  assert.equal(new Set(cases.map(item => item.id)).size, cases.length)
  const inventory = JSON.parse(await readFile(resolve(directory, 'inventory.json'), 'utf8'))
  const sources = ['packages/tailwindcss/src/utilities.ts', 'packages/tailwindcss/src/utilities.test.ts', 'packages/tailwindcss/src/theme.ts', 'packages/tailwindcss/src/utils/infer-data-type.ts', 'packages/tailwindcss/src/ast.ts'].map(path => { const source = inventory.sources.find(item => item.path === path); assert.ok(source?.sha256, path); return { path, sha256: source.sha256 } })
  sources.push({ path: 'packages/tailwindcss/theme.css', sha256: sha256(defaultCss) })
  return { schemaVersion: 1, sourceCommit: '1334c99db8fd26c8ea065375dd9259800863f072', oracleVersion: '4.1.13', functionalRoots: roots, fullFamilyConformance: 'pending', themeDefaults: 'explicit input only', sources, cases }
}
export async function checkTypographyFixtures() {
  const actual = await typographyFixtures()
  assert.deepEqual(actual, JSON.parse(await readFile(resolve(directory, 'typography-fixtures.json'), 'utf8')))
  return { functionalRoots: roots.length, cases: actual.cases.length }
}
if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  if (process.argv.includes('--write')) { const actual = await typographyFixtures(); await writeFile(resolve(directory, 'typography-fixtures.json'), JSON.stringify(actual, null, 2) + '\n'); console.log(JSON.stringify({ functionalRoots: roots.length, cases: actual.cases.length })) }
  else console.log(JSON.stringify(await checkTypographyFixtures()))
}
