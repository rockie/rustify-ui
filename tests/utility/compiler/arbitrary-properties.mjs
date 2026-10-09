import assert from 'node:assert/strict'
import { createHash } from 'node:crypto'
import { readFile, writeFile } from 'node:fs/promises'
import { createRequire } from 'node:module'
import { dirname, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'

const directory = dirname(fileURLToPath(import.meta.url))
const upstream = resolve(directory, '../../../ref/tailwindcss')
const oracle = createRequire(resolve(directory, 'oracle/package.json'))
const tailwind = oracle('tailwindcss')
assert.equal(oracle('tailwindcss/package.json').version, '4.1.13')
const sha256 = text => createHash('sha256').update(text).digest('hex')
const properties = ['color', 'background-color', 'border-color', 'fill', 'stroke', 'width', 'opacity', 'content', '--probe', '-webkit-text-fill-color', 'unknown-property']
const values = ['red', '#123456', 'rgb(13_17_19)', 'oklch(.5_.2_30)', 'transparent', 'currentColor', 'inherit', 'none', '0', '13px', '50%', 'var(--probe)', 'var(--probe,_red)', 'var(--my_color)', '(--probe)', '(color:--probe)', 'color:var(--probe)', 'length:13px', 'calc(13px+17px)', 'calc(1/2)', '"hello_world"', "'hello\\_world'", 'attr(data-label)', 'url(icon_name.svg)', '_']
const declarations = '--opacity-muted: .35; --opacity-alias: var(--opacity-muted); --opacity-full: 1; --opacity-percent: 35%; --opacity-zero: 0; --opacity-half: .5; --opacity-50: .75; --opacity-1_5: .65; --opacity-empty: ; --opacity-calculated: calc(.5 * 2); --source-opacity: .45; --opacity-source: var(--source-opacity); --probe: red; --my_color: #123456;'
const themes = {
  empty: '', base: `@theme { ${declarations} }`, inline: `@theme inline { ${declarations} }`, reference: `@theme reference { ${declarations} }`, default: `@theme default { ${declarations} }`, staticTheme: `@theme static { ${declarations} }`,
  reset: `@theme { ${declarations} } @theme { --opacity-*: initial; }`, defaultPriority: `@theme { ${declarations} } @theme default { --opacity-muted: .95; --opacity-50: .25; }`,
  prefix: `@theme prefix(tw) { ${declarations} }`, prefixInline: `@theme prefix(tw) inline { ${declarations} }`, prefixReference: `@theme prefix(tw) reference { ${declarations} }`,
}
const modifiers = ['0', '.25', '0.25', '1', '1.5', '25', '50', '75', '100', '125', '-0', '-25', '+25', '01', '1.0', '0.5', '0.1', '1e+21', '1e21', '1e+22', 'Infinity', 'NaN', '0x10', 'muted', 'alias', 'full', 'percent', 'zero', 'half', 'empty', 'calculated', 'source', 'unknown', '[.35]', '[35%]', '[1]', '[100%]', '[0]', '[-.25]', '[-25%]', '[125%]', '[calc(.5*2)]', '[var(--opacity-muted)]', '[var(--opacity-muted,_.5)]', '(--opacity-muted)', '(--my_opacity)', '[Infinity]', '[NaN]', '[0x10]', '[number:.5]', '[percentage:50%]', '[".5"]', '[]', '[_]', '()', '(bad)', '[;]', '[}]', '50/foo']
const invalidCandidates = ['', '[]', '[color]', '[color:]', '[:red]', '[Color:red]', '[1color:red]', '[color:red', 'color:red]', '[color:red;display:block]', '[color:red}]', '[color:red]/', '[color:red]/[]', '[color:red]/[_]', '[color:red]/()', '[color:red]/(bad)', '[color:red]/[;]', '[color:red]/50/25', '[color:red]!!', '![color:red]!', '[color:red]!suffix', 'unknown:[color:red]', '-[color:red]']

export async function arbitraryPropertiesFixtures() {
  const cases = []
  async function add(id, mode, candidates) {
    const input = `${themes[mode]} @tailwind utilities;`.trim()
    const compiler = await tailwind.compile(input)
    const expectedCss = compiler.build(candidates).replace(/^\/\*! tailwindcss[^\n]*\n/, '')
    cases.push({ id, input, candidates, expectedCss, expectedCssSha256: sha256(expectedCss) })
  }
  for (const property of properties) {
    for (const mode of ['empty', 'base', 'inline', 'reference', 'default']) for (const value of values) {
      const candidate = `[${property}:${value}]`
      await add(`arbitrary-property:property=${property}:value=${value}:${mode}`, mode, [candidate])
    }
    for (const mode of ['empty', 'base', 'inline', 'reference', 'default', 'reset', 'defaultPriority']) for (const value of ['red', 'var(--probe)', '13px', 'color:var(--probe)']) for (const modifier of modifiers) {
      const candidate = `[${property}:${value}]/${modifier}`
      await add(`arbitrary-property:alpha:property=${property}:value=${value}:modifier=${modifier}:${mode}`, mode, [candidate])
    }
    for (const mode of ['prefix', 'prefixInline', 'prefixReference']) for (const candidate of [`tw:[${property}:red]`, `tw:[${property}:red]/50`, `tw:[${property}:var(--probe)]/muted`, `tw:[${property}:red]/[100%]`, `tw:hover:[${property}:red]/[.5]!`, `tw:before:[${property}:13px]/(--opacity-muted)!`, `[${property}:red]/50`]) await add(`arbitrary-property:${candidate}:${mode}`, mode, [candidate])
    for (const mode of ['empty', 'base', 'inline', 'reference']) for (const candidate of [`hover:[${property}:red]/50!`, `before:[${property}:red]/muted`, `after:[${property}:13px]/[.5]!`, `[&>p]:[${property}:red]/[35%]`, '[@media(width>=123px)]:[&_p]:' + `[${property}:var(--probe)]/50!`]) await add(`arbitrary-property:${candidate}:${mode}`, mode, [candidate])
  }
  for (let numerator = 0; numerator <= 500; numerator++) for (const property of ['color', 'width']) await add(`arbitrary-property:quarter:${property}:${numerator / 4}`, 'empty', [`[${property}:red]/${numerator / 4}`])
  for (const property of ['-webkit-appearance', '--my_color', 'fooBar', 'foo_bar', '-', '--', 'a b', 'color;color']) for (const value of ['red', 'var(--my_color)']) await add(`arbitrary-property:property-boundary:${property}:${value}`, 'empty', [`[${property}:${value}]`, `[${property}:${value}]/50`])
  for (const candidate of invalidCandidates) for (const mode of ['empty', 'base', 'prefix']) await add(`arbitrary-property:syntax-boundary:${candidate || '<empty>'}:${mode}`, mode, [candidate])
  for (const candidate of ['[color:red]/[calc(1/2)]', '[color:red]/(--opacity-muted)', '[color:red]/(color:--opacity-muted)', '[color:var(--probe)]/number:.5', '[color:(--probe)]/50', '[color:color:var(--probe)]/50', '[color:red]/[var(--opacity-muted]', '[color:var(--probe]', '[color:";"]/[";"]']) await add(`arbitrary-property:typehint-shorthand:${candidate}`, 'base', [candidate])
  const themeCss = await readFile(resolve(upstream, 'packages/tailwindcss/theme.css'), 'utf8')
  assert.deepEqual([...themeCss.matchAll(/--opacity(?:-[\w-]+)?\s*:/g)], [])
  for (const mode of ['empty', 'base', 'inline', 'reference', 'staticTheme', 'prefix']) {
    const prefix = mode === 'prefix' ? 'tw:' : ''
    const candidates = ['[color:red]/50', '[color:red]/muted', '[width:13px]/[.5]', '[opacity:.5]/[100%]', '[background-color:var(--probe)]/alias', '[--probe:red]/0', '[content:"probe"]/[35%]', 'before:[content:"probe"]/muted!', 'hover:[color:red]/100!', '[fill:color:var(--probe)]/50', '[unknown-property:calc(1/2)]/25', '[color:red]/[]'].map(candidate => prefix + candidate)
    await add(`arbitrary-property:sorted-union:${mode}`, mode, candidates)
    await add(`arbitrary-property:reverse-arrival-sorted-union:${mode}`, mode, candidates.toReversed())
    await add(`arbitrary-property:property-order:${mode}`, mode, ['[z-index:1]/50', '[display:block]/50', '[width:13px]/50', '[opacity:.5]/50', '[color:red]/50', '[--probe:red]/50', '[unknown-property:red]/50', '[background-color:red]/50'].map(candidate => prefix + candidate))
  }
  const ids = new Set(); for (const item of cases) { assert.ok(!ids.has(item.id), `duplicate ${item.id}`); ids.add(item.id) }
  const paths = ['packages/tailwindcss/src/compile.ts', 'packages/tailwindcss/src/candidate.ts', 'packages/tailwindcss/src/candidate.test.ts', 'packages/tailwindcss/src/utilities.ts', 'packages/tailwindcss/src/theme.ts', 'packages/tailwindcss/src/variants.ts', 'packages/tailwindcss/src/utils/infer-data-type.ts', 'packages/tailwindcss/src/utils/decode-arbitrary-value.ts', 'packages/tailwindcss/src/utils/is-valid-arbitrary.ts', 'packages/tailwindcss/src/value-parser.ts', 'packages/tailwindcss/src/property-order.ts', 'packages/tailwindcss/src/ast.ts', 'packages/tailwindcss/theme.css']
  const sources = await Promise.all(paths.map(async path => ({ path, sha256: sha256(await readFile(resolve(upstream, path))) })))
  return { schemaVersion: 1, sourceCommit: '1334c99db8fd26c8ea065375dd9259800863f072', oracleVersion: '4.1.13', mechanism: 'arbitrary property compileBaseUtility applies asColor to any property with a modifier', sourceLocators: [{ path: 'packages/tailwindcss/src/compile.ts', lines: '273-287' }, { path: 'packages/tailwindcss/src/candidate.ts', lines: '285-342' }, { path: 'packages/tailwindcss/src/utilities.ts', lines: '165-178,202-226' }], defaultTheme: 'explicit input only; fixed theme.css has no opacity namespace tokens', fullFamilyConformance: 'pending', sources, cases }
}
export async function checkArbitraryPropertiesFixtures() {
  const actual = await arbitraryPropertiesFixtures()
  assert.deepEqual(actual, JSON.parse(await readFile(resolve(directory, 'arbitrary-properties-fixtures.json'), 'utf8')))
  return { properties: properties.length, cases: actual.cases.length }
}
if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  if (process.argv.includes('--write')) { const actual = await arbitraryPropertiesFixtures(); await writeFile(resolve(directory, 'arbitrary-properties-fixtures.json'), JSON.stringify(actual, null, 2) + '\n'); console.log(JSON.stringify({ properties: properties.length, cases: actual.cases.length })) }
  else console.log(JSON.stringify(await checkArbitraryPropertiesFixtures()))
}
