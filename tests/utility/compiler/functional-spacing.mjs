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
const groups = [
  { roots: ['inset', 'inset-x', 'inset-y', 'start', 'end', 'top', 'right', 'bottom', 'left'], negative: true, fractions: true },
  { roots: ['m', 'mx', 'my', 'ms', 'me', 'mt', 'mr', 'mb', 'ml'], negative: true, fractions: false },
  { roots: ['p', 'px', 'py', 'ps', 'pe', 'pt', 'pr', 'pb', 'pl'], negative: false, fractions: false },
  { roots: ['scroll-m', 'scroll-mx', 'scroll-my', 'scroll-ms', 'scroll-me', 'scroll-mt', 'scroll-mr', 'scroll-mb', 'scroll-ml'], negative: true, fractions: false },
  { roots: ['scroll-p', 'scroll-px', 'scroll-py', 'scroll-ps', 'scroll-pe', 'scroll-pt', 'scroll-pr', 'scroll-pb', 'scroll-pl'], negative: false, fractions: false },
  { roots: ['size', 'w', 'min-w', 'max-w', 'h', 'min-h', 'max-h', 'basis'], negative: false, fractions: true },
  { roots: ['gap', 'gap-x', 'gap-y'], negative: false, fractions: false },
]
const roots = groups.flatMap(group => group.roots.flatMap(root => group.negative ? [root, `-${root}`] : [root]))
const namespaceValues = { inset: 11, margin: 13, padding: 17, 'scroll-margin': 19, 'scroll-padding': 23, size: 29, width: 31, 'min-width': 37, 'max-width': 41, height: 43, 'min-height': 47, 'max-height': 53, 'flex-basis': 59, gap: 61, container: 67 }
const namespaces = Object.keys(namespaceValues)
const declarations = '--spacing: 4px; --spacing-4: 7px; --spacing-card: 9px; --spacing-1_5: 15px; ' + namespaces.map(key => `--${key}-card: ${namespaceValues[key]}px;`).join(' ')
const themes = {
  empty: '',
  multiplier: '@theme { --spacing: 4px; }',
  named: `@theme { ${declarations} }`,
  inline: `@theme inline { ${declarations} }`,
  reference: `@theme reference { ${declarations} }`,
  spacingOnly: '@theme { --spacing-card: 9px; --container-card: 67px; }',
  containerOnly: '@theme { --container-card: 67px; }',
  heightOnly: '@theme { --height-card: 43px; }',
  fractionTheme: '@theme { --spacing: 4px; --spacing-1\\/2: 9px; --width-1\\/2: 11px; }',
  reset: '@theme { --spacing: 4px; --spacing-*: initial; --spacing-4: 7px; }',
  slashMultiplier: '@theme inline { --spacing: calc(4px / 2); }',
  ignoredInset: '@theme { --inset-shadow-card: 19px; --inset-ring-card: 23px; --inset-shadowned: 29px; }',
}
const suffixes = ['', '-0', '-0.25', '-1', '-1.5', '-2.75', '-0.375', '-2.50', '-.75', '-01', '-1e3', '-1e+21', '-100000000000000000000', '--1', '-unknown', '-[13px]', '-[length:17px]', '-[color:red]', '-[calc(1rem_+_2px)]', '-[var(--probe)]', '-(--probe)', '-[50%]', '-[13px]/foo', '-1/foo', '-1/[50%]', '-1/(--probe)', '-1/2', '-0/2', '-1/0', '-1.5/2', '-01/2', '-1/-2', '-1/02', '-1/2/foo', '-card', '-card/foo', '-card/[50%]', '-card/(--probe)', '-card/[]', '-card/(bad)', '-card/[;]']
export async function functionalSpacingFixtures() {
  const design = await tailwind.__unstable__loadDesignSystem('@tailwind utilities;')
  assert.equal(roots.length, 83)
  for (const root of roots) assert.ok(design.utilities.get(root).some(item => item.kind === 'functional'), root)
  const cases = []
  async function add(id, mode, candidates) {
    const input = `${themes[mode]} @tailwind utilities;`.trim()
    const compiler = await tailwind.compile(input)
    const expectedCss = banner(compiler.build(candidates))
    cases.push({ id, input, candidates, expectedCss, expectedCssSha256: sha256(expectedCss) })
  }
  for (const root of roots) {
    for (const suffix of suffixes) await add(`functional:${root}${suffix}:named`, 'named', [root + suffix])
    for (const mode of ['empty', 'multiplier', 'inline', 'reference', 'spacingOnly', 'containerOnly', 'heightOnly']) {
      for (const suffix of ['-4', '-card', '-1.5']) await add(`functional:${root}${suffix}:${mode}`, mode, [root + suffix])
    }
    for (const suffix of ['-1/2', '-1/foo', '-card/[50%]']) await add(`functional:${root}${suffix}:fractionTheme`, 'fractionTheme', [root + suffix])
    for (const suffix of ['-1/foo', '-1/[50%]', '-1/(--probe)']) await add(`functional:${root}${suffix}:slashMultiplier`, 'slashMultiplier', [root + suffix])
    for (const suffix of ['-1', '-4']) await add(`functional:${root}${suffix}:reset`, 'reset', [root + suffix])
    await add(`functional:${root}:important`, 'named', [`${root}-[13px]!`])
    await add(`functional:${root}:hover`, 'named', [`hover:${root}-4`])
  }
  for (const root of ['inset-x', '-inset-x']) for (const suffix of ['shadow-card', 'ring-card', 'shadowned']) await add(`functional:${root}-${suffix}:ignored`, 'ignoredInset', [`${root}-${suffix}`])
  for (const suffix of ['--0', '-1e21', '-1e+22', '-1e+100', '-Infinity', '-NaN', '-1e309', '-1000000000000000100', '-9007199254740992', '-0.0000001', '-+1', '-1/1e+21', '-1/1e21', '-1/Infinity', '-1/NaN']) await add(`functional:w${suffix}:numeric`, 'multiplier', [`w${suffix}`])
  await add('functional:sorted-union', 'named', roots.flatMap(root => [`${root}-4`, `${root}-[13px]`, `${root}-1/2`]))
  await add('functional:size-sort-and-multiple-interpretations', 'named', ['size-4', 'w-4', 'h-4', 'size-[17px]', 'w-full'])
  assert.equal(new Set(cases.map(item => item.id)).size, cases.length)
  const inventory = JSON.parse(await readFile(resolve(directory, 'inventory.json'), 'utf8'))
  const sourcePaths = ['packages/tailwindcss/src/utilities.ts', 'packages/tailwindcss/src/utilities.test.ts', 'packages/tailwindcss/src/theme.ts', 'packages/tailwindcss/src/candidate.ts', 'packages/tailwindcss/src/compile.ts', 'packages/tailwindcss/src/utils/infer-data-type.ts']
  const sources = sourcePaths.map(path => { const source = inventory.sources.find(item => item.path === path); assert.ok(source?.sha256, path); return { path, sha256: source.sha256 } })
  return { schemaVersion: 1, sourceCommit: '1334c99db8fd26c8ea065375dd9259800863f072', oracleVersion: '4.1.13', groups, functionalRoots: roots, boundaryCoverage: ['missing value', 'arbitrary value and ignored type hint', 'CSS variable shorthand', 'theme namespace precedence', 'inline/reference/reset modes', 'dot-to-underscore token keys', 'escaped fraction theme keys', 'canonical quarter multipliers', 'negative named/arbitrary/bare values', 'fractions with nonnegative integer numerator/denominator', 'modifiers with named and arbitrary theme values', 'important', 'hover', 'sorted union', 'size property sort override', 'ignored overlapping inset namespaces'], sourceLocators: { functionalUtility: 'packages/tailwindcss/src/utilities.ts:383', spacingUtility: 'packages/tailwindcss/src/utilities.ts:498', themeResolve: 'packages/tailwindcss/src/theme.ts:166', numericInteger: 'packages/tailwindcss/src/utils/infer-data-type.ts:346', spacingMultiplier: 'packages/tailwindcss/src/utils/infer-data-type.ts:356', modifier: 'packages/tailwindcss/src/candidate.ts:512', propertySort: 'packages/tailwindcss/src/compile.ts:333' }, sources, cases }
}
export async function checkFunctionalSpacingFixtures() {
  const actual = await functionalSpacingFixtures()
  assert.deepEqual(actual, JSON.parse(await readFile(resolve(directory, 'functional-spacing-fixtures.json'), 'utf8')))
  return { functionalRoots: actual.functionalRoots.length, cases: actual.cases.length }
}
if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  if (process.argv.includes('--write')) { const actual = await functionalSpacingFixtures(); await writeFile(resolve(directory, 'functional-spacing-fixtures.json'), JSON.stringify(actual, null, 2) + '\n'); console.log(JSON.stringify({ functionalRoots: actual.functionalRoots.length, cases: actual.cases.length })) }
  else console.log(JSON.stringify(await checkFunctionalSpacingFixtures()))
}
