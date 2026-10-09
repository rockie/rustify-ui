import assert from 'node:assert/strict'
import { createHash } from 'node:crypto'
import { readFile, writeFile } from 'node:fs/promises'
import { createRequire } from 'node:module'
import { dirname, resolve, posix } from 'node:path'
import { fileURLToPath } from 'node:url'

const directory = dirname(fileURLToPath(import.meta.url))
const oracle = createRequire(resolve(directory, 'oracle/package.json'))
const { compile } = oracle('tailwindcss')
assert.equal(oracle('tailwindcss/package.json').version, '4.1.13')
const sha256 = text => createHash('sha256').update(text).digest('hex')
const types = ['number', 'integer', 'ratio', 'percentage', 'color', 'length', 'url', 'position', 'bg-size', 'line-width', 'image', 'family-name', 'generic-name', 'absolute-size', 'relative-size', 'angle', 'vector']
const declarations = '--spacing: .25rem; --probe-card: 13px; --probe-2: 17px; --probe-1_5: 19px; --probe-0: 0; --probe-alias: var(--probe-card); --other-card: 23px; --probe-card--size: 29px; --probe-card--line-height: 31; --probe-card--nested--value: 37; --probe-orphan--size: 41px; --color-brand: #123456; --breakpoint-sm: 40rem;'
const themes = { empty: '', base: `@theme { ${declarations} }`, inline: `@theme inline { ${declarations} }`, reference: `@theme reference { ${declarations} }`, default: `@theme default { ${declarations} }`, static: `@theme static { ${declarations} }`, prefix: `@theme prefix(tw) { ${declarations} }`, prefixInline: `@theme inline prefix(tw) { ${declarations} }`, prefixReference: `@theme reference prefix(tw) { ${declarations} }`, reset: `@theme { ${declarations} } @theme { --probe-*: initial; }`, defaultPriority: `@theme { ${declarations} } @theme default { --probe-card: 99px; --spacing: 1rem; }` }
const values = ['', 'card', 'alias', 'orphan', 'unknown', 'red', 'brand', '0', '0.25', '1.5', '2', '01', '1.0', '.5', '-1', '1e+21', 'Infinity', '0%', '50%', '0.25%', '01%', '-1%', '1/2', '0/0', '2/3', '2.5/3', '01/2', '2/-3', '[0]', '[2]', '[1.5]', '[-2]', '[1e2]', '[50%]', '[0.25%]', '[13px]', '[10/9]', '[1.5/2.5]', '[var(--probe)]', '(--probe)', '[calc(1px+2px)]', '[min(1,2)]', '[rgb(1_2_3)]', '[red]', '[#123456]', '[url(card.svg)]', '[linear-gradient(red,blue)]', '[image-set(url(a.svg)_1x)]', '[left_13px_top_17px]', '[cover]', '[auto_13px]', '[thin_13px]', '[IBM_Plex_Sans]', '[serif]', '[medium]', '[larger]', '[13deg]', '[1_2_3]', '[nonsense:var(--probe)]', '[integer:var(--probe)]', '[length:var(--probe)]', '[color:13px]', '[number:red]', '[* :var(--probe)]']
const modifiers = ['2', '0', '1.5', '50%', 'card', 'unknown', 'closest-side', '[13px]', '[var(--probe)]', '(--probe)', '[integer:var(--probe)]', '[nonsense:var(--probe)]', '[]', '[;]']

export async function customUtilityFixtures() {
  const cases = []
  async function add(id, input, candidates, assets) {
    const warnings = []
    const warn = console.warn; console.warn = (...args) => warnings.push(args.join(' '))
    try {
      const options = assets ? { base: '/app', loadStylesheet: async (uri, base) => {
        const id = posix.normalize(posix.join(base, uri)); const asset = assets.find(asset => asset.id === id) ?? assets.find(asset => asset.id === uri)
        assert.ok(asset, `missing oracle asset ${uri} from ${base}`)
        return { path: asset.id, base: posix.dirname(asset.id), content: asset.content }
      } } : undefined
      const compiler = await compile(input, options)
      const expectedCss = compiler.build(candidates).replace(/^\/\*! tailwindcss[^\n]*\n/, '')
      cases.push({ id, input, candidates, ...(assets ? { entry: '/app/input.css', assets } : {}), expectedCss, expectedCssSha256: sha256(expectedCss), ...(warnings.length ? { expectedWarnings: warnings } : {}) })
    } catch (error) {
      cases.push({ id, input, candidates, ...(assets ? { entry: '/app/input.css', assets } : {}), expectedError: error.message, expectedErrorName: error.name, ...(warnings.length ? { expectedWarnings: warnings } : {}) })
    } finally { console.warn = warn }
  }
  async function utility(id, mode, body, candidates, name = 'sample-*') { await add(id, `${themes[mode]} @utility ${name} { ${body} } @tailwind utilities;`, candidates) }
  const specs = [...types.map(type => `[${type}]`), '*', '[*]', 'number', 'integer', 'ratio', 'percentage', 'color', 'length', 'nonsense', '[nonsense]', '"card"', "'closest-side'", 'number,[length]', '--probe', '--probe-*', '--probe- *', '--probe-\\*', '--probe --size', '--probe-*--size', '--probe- * --line-height', '--probe-*--nested-*--value', '--missing,--other', '--probe,number,[*]']
  for (const spec of specs) {
    for (const mode of ['empty', 'base', 'inline', 'reference', 'default']) for (const value of values) await utility(`custom:value:${spec}:${mode}:${value || '<none>'}`, mode, `--result: --value(${spec});`, [`sample${value ? `-${value}` : ''}`])
    for (const mode of ['prefix', 'prefixInline', 'prefixReference', 'reset', 'defaultPriority', 'static']) await utility(`custom:value:${spec}:${mode}:union`, mode, `--result: --value(${spec});`, ['sample-card', 'sample-2', 'sample-[13px]', 'sample-2/3', 'sample-(--probe)', 'tw:sample-card', 'tw:hover:sample-[integer:var(--probe)]!'])
  }
  for (const spec of ['number', 'integer', 'ratio', 'percentage', '[*]', '[length]', '[integer]', '[nonsense]', '--probe', '--probe-*--size', '"closest-side"', '--probe,number,[*]']) for (const value of ['card', '2', '1.5', '50%', '[13px]', '[var(--probe)]', '[integer:var(--probe)]', '[nonsense:var(--probe)]']) for (const modifier of modifiers) for (const mode of ['base', 'inline', 'reference']) await utility(`custom:modifier:${spec}:${mode}:${value}/${modifier}`, mode, `--result: --value(--probe,number,percentage,[*]); --modifier-result: --modifier(${spec});`, [`sample-${value}/${modifier}`])
  for (const expected of [...types, 'nonsense']) for (const hint of [...types, 'nonsense']) for (const mode of ['base', 'reference']) await utility(`custom:forced-hint:${expected}:${hint}:${mode}`, mode, `--result: --value([${expected}]);`, [`sample-[${hint}:var(--probe)]`])
  const inferenceEdges = ['[1.]', '[.5]', '[+2]', '[-2]', '[1e+21]', '[1e309]', '[Infinity]', '[calc(1+2)]', '[CALC(1+2)]', '[foo_calc(1+2)]', '[1px]', '[1Q]', '[1q]', '[1cqi]', '[1svmin]', '[1deg]', '[1DEG]', '[1_2_3]', '[1__2_3]', '[_1_2_3]', '[1_2_3_]', '[1/2]', '[_1/2]', '[1/2_]', '[1_/_2]', '[1s/2]', '[1sss/ss2]', '[1s/ss2]', '[1.5/2.5]', '[url(card.svg),var(--probe)]', '[url(card.svg),_var(--probe)]', '[var(--probe),url(card.svg)]', '[var(--probe)_center]', '[center_var(--probe)]', '[13px_auto]', '[auto_13px_17px]', '[cover,bogus]', '[cover,bogus_bogus_bogus]', '[13px,bogus]', '[serif,var(--probe)]', '[1foo]', '[var(--probe),serif]', '[#]', '[CurrentColor]', '[RGB(1_2_3)]']
  for (const type of types) for (const value of inferenceEdges) await utility(`custom:inference-edge:${type}:${value}`, 'empty', `--result: --value([${type}]);`, [`sample-${value}`])
  await add('custom:mixed-inline-base-reference-nested', '@theme inline reference { --probe-card:13px; } @theme reference { --probe-card--size:17px; } @utility sample-* { width:--value(--probe); height:--value(--probe-*--size); } @tailwind utilities;', ['sample-card'])
  await add('custom:mixed-reference-base-inline-nested', '@theme reference { --probe-card:13px; } @theme inline reference { --probe-card--size:17px; } @utility sample-* { width:--value(--probe); height:--value(--probe-*--size); } @tailwind utilities;', ['sample-card'])
  await add('custom:media-reference-transparent', '@media reference { @theme { --probe-card:13px; } @utility sample-* { width:--value(--probe); } } @tailwind utilities;', ['sample-card'])
  const bodies = {
    noValueFunction: 'color: red;', modifierOnly: 'color: red; opacity: --modifier(number);', fallbackDeclarations: '--result: --value(integer); --result: --value(--probe); --result: --value([integer]); color: red;',
    sameDeclarationAlternatives: '--result: --value(integer,--probe,[integer]); color: red;', multipleFunctions: '--result: calc(--value(number) * --value(number));', mixedFunctionFailure: '--result: calc(--value(number) * --value("card")); color: red;',
    modifierFirstStop: '--result: calc(--modifier(number) * --value(number)); color: red;', valueFirstStop: '--result: calc(--value(number) * --modifier(number)); color: red;',
    ratioAndNumber: '--ratio: --value(ratio); --number: --value(number); --literal: --value("2"); color: red;', ratioAndModifier: '--ratio: --value(ratio); --modifier-result: --modifier(number);',
    ratioMultipleFn: '--result: calc(--value(ratio) * --value(number)); color: red;', ratioArbitrary: '--ratio: --value([ratio]); --number: --value([number]); --modifier-result: --modifier(number);',
    spacing: 'margin: --spacing(--value(number)); margin-top: --spacing(--modifier(number));', alpha: 'color: --alpha(--value(--color,[color]) / --modifier(percentage, [*]));',
    nested: 'color:red; &:hover { --result: --value(integer); .child { width: --value(--probe,[length]); } } @media (width > 300px) { opacity: --modifier(number); }',
    emptyNested: '&:hover { --result: --value(integer); } @media (width > 300px) { --modifier-result: --modifier(integer); }',
    importantAuthored: 'width: --value(--probe,[length]) !important; --result: --value(number);', quotedFunctions: 'content:"--value(integer)"; --result: --value(integer);',
    emptyValue: '--empty: ; --result: --value(integer);', sortSentinel: '--tw-sort: width; color: --value([color]); width: 13px; height: 17px;',
  }
  for (const [name, body] of Object.entries(bodies)) for (const mode of ['empty', 'base', 'inline', 'reference']) for (const value of ['', 'unknown', 'card', '2', '2/3', '2/foo', '[2]', '[13px]', '[red]', '[var(--probe)]', '[integer:var(--probe)]', '2/[var(--probe)]']) await utility(`custom:behavior:${name}:${mode}:${value || '<none>'}`, mode, body, [`sample${value ? `-${value}` : ''}`])
  for (const name of ['sample', '-sample', 'sample-1/2', 'sample-50%', 'sample.Name', 'sample_Name', 'sample-fooBar', 'sample--name']) for (const mode of ['base', 'inline', 'reference', 'prefix']) await utility(`custom:static:${name}:${mode}`, mode, 'width:13px; color:var(--color-brand); &:hover { color:red; }', [name, `hover:${name}!`, `tw:${name}`, `${name}/foo`, `-${name}`], name)
  for (const name of ['', 'Sample', '1sample', '--sample', '~sample', '@sample', 'sample|', 'sample*', 'sample*name', 'sample-*-*', 'sample *', 'sample-*x']) await utility(`custom:invalid-name:${name || '<empty>'}`, 'empty', 'color:red;', ['sample'], name)
  for (const input of ['@utility sample {}', '@utility sample;', '@utility sample { /* comment */ }', '.host { @utility sample { color:red; } }', '@media screen { @utility sample { color:red; } }', '@layer utilities { @utility sample { color:red; } }', '@utility sample-* { }']) await add(`custom:definition:${input}`, `${input} @tailwind utilities;`, ['sample', 'sample-2'])
  for (const body of ['color:red;', 'width: --value(integer);', '--result: --value(integer); &:hover { color:red; }']) {
    await add(`custom:negative-explicit:${body}`, `@utility sample-* { ${body} } @utility -sample-* { --result: calc(--value(number,[*]) * -1); } @tailwind utilities;`, ['sample-2', '-sample-2', '-sample-[13px]', '-sample-card', 'hover:-sample-2!'])
  }
  for (const input of [
    '@utility sample { width:13px; color:red; } @utility sample { width:17px; }',
    '@utility sample-* { --result:--value(integer); } @utility sample-* { --result:--value([*]); }',
    '@utility sample-* { color:red; } @utility sample-* { --result:--value(integer); } @utility sample-* { color:blue; }',
    '@utility block { color:red; }', '@utility w-* { color:--value([color]); }', '@utility rounded { border-radius:13px; }',
    '@utility text-sm { font-size:13px; } @theme { --text-sm:17px; }', '@utility container { padding-inline:13px; } @theme { --breakpoint-sm:40rem; }',
  ]) await add(`custom:registry:${input}`, `${input} @tailwind utilities;`, ['sample', 'sample-2', 'sample-[red]', 'sample-unknown', 'block', 'w-[red]', 'w-[13px]', 'rounded', 'text-sm', 'container'])
  await add('custom:sorted-union', `${themes.base} @utility alpha { width:13px; color:red; } @utility beta { width:13px; } @utility gamma-* { --tw-sort: height; color:--value([color]); } @utility sample-* { width:--value(--probe,number,[*]); &:hover { color:red; } } @tailwind utilities;`, ['gamma-[red]', 'beta', 'alpha', 'sample-card', 'hover:sample-[13px]!', 'sm:sample-2', 'group-hover:sample-card'])
  await add('custom:utility-layer', `${themes.base} @utility sample-* { width:--value(--probe,[length]); } @layer utilities { @tailwind utilities; }`, ['sample-card', 'sample-[13px]!'])
  const assets = [{ id:'/app/utilities.css', content:`${themes.base} @utility sample-* { width:--value(--probe,[length]); } @utility static-sample { color:red; }` }]
  for (const importKind of ['@import "utilities.css";', '@reference "utilities.css";', '@import "utilities.css" reference;', '@import "utilities.css" layer(utilities);', '@import "utilities.css" screen;']) await add(`custom:import:${importKind}`, `${importKind} @tailwind utilities;`, ['sample-card', 'static-sample', 'hover:sample-[13px]!'], assets)
  for (const mode of ['base', 'inline', 'reference', 'prefix']) {
    for (const body of [
      '@property --custom-length { syntax:"<length>"; inherits:false; initial-value:13px; } width:--value(--probe,[length]);',
      '@property --custom-length { syntax:"<length>"; inherits:true; initial-value:--value([length]); } width:17px;',
      '@keyframes custom-motion { from { opacity:--value(number); } to { opacity:1; } } animation:custom-motion 1s;',
      'color:--alpha(--value(--color,[color]) / --modifier(percentage,[*]));',
      '&:hover { color:--alpha(--value(--color,[color]) / --modifier(percentage,[*])); } @media (width > 300px) { width:--value([length]); }',
      'width:--spacing(--value(number)); height:--theme(--missing,17px); padding:theme(--spacing);',
    ]) await utility(`custom:resources:${mode}:${body}`, mode, body, ['sample-card', 'sample-2', 'sample-[13px]', 'sample-brand/50%', 'sample-[red]/[var(--opacity)]', 'sample-brand/100%', 'sample-[red]/100%', 'hover:sample-brand/[var(--opacity)]!', 'hover:sample-[13px]!', 'tw:sample-card', 'tw:hover:sample-brand/[var(--opacity)]!'])
    await add(`custom:theme-keyframe-liveness:${mode}`, `${themes[mode]} @theme { --animate-motion:custom-motion 1s; @keyframes custom-motion { from { opacity:0; } to { opacity:1; } } } @utility sample-* { animation:--value(--animate); } @tailwind utilities;`, ['sample-motion', 'hover:sample-motion!', 'tw:sample-motion'])
  }
  for (const [name, body] of [['sample', 'width:--spacing(2);'], ['sample-*', 'width:--spacing(--value(number));'], ['sample-*', 'width:--theme(--missing);'], ['sample', 'width:theme(--missing);'], ['sample-*', 'color:--alpha(red);'], ['sample-*', 'width:--spacing(1,2);']]) for (const used of [false,true]) await add(`custom:template-functions:${name}:${body}:${used}`, `${themes.base} @utility ${name} { ${body} } @tailwind utilities;`, used ? [name.endsWith('-*') ? 'sample-2' : 'sample'] : [])
  await add('custom:authored-and-template-property-fallback-order', '@property --custom-length { syntax:"<length>"; inherits:true; initial-value:13px; } @utility sample { @property --custom-length { syntax:"<length>"; inherits:false; initial-value:17px; } width:var(--custom-length); } @tailwind utilities;', ['sample', 'hover:sample!'])
  await add('custom:resource-only-static-important', '@utility sample { @property --custom-raw { syntax:"*"; inherits:false; } @keyframes custom-motion { from { opacity:0; } to { opacity:1; } } } @tailwind utilities important;', ['sample', 'hover:sample!'])
  for (const candidate of ['probe-child-(--value)', 'probe-calc(1)', 'probe-(--value)']) await add(`custom:variable-exact-root:${candidate}`, '@utility probe-* { color:red; } @tailwind utilities;', [candidate])
  await add('custom:sort-marker-does-not-use-theme-variable', '@theme { --color-brand:#123456; } @utility probe { --tw-sort:var(--color-brand); } @tailwind utilities;', ['probe'])
  await add('custom:reference-body-does-not-use-theme-variable', '@theme { --color-brand:#123456; } @utility probe { @media reference { color:var(--color-brand); } } @tailwind utilities;', ['probe'])
  await add('custom:reference-context-does-not-affect-property-sorting', '@utility zulu { color:red; @media reference { width:13px; } } @utility alpha { color:blue; } @tailwind utilities;', ['zulu', 'alpha'])
  const ids = new Set(); for (const item of cases) { assert.ok(!ids.has(item.id), `duplicate ${item.id}`); ids.add(item.id) }
  const sourcePaths = ['src/index.ts','src/utilities.ts','src/utilities.test.ts','src/compile.ts','src/candidate.ts','src/theme.ts','src/ast.ts','src/at-import.ts','src/utils/infer-data-type.ts','src/utils/is-color.ts','src/utils/math-operators.ts','src/value-parser.ts','src/design-system.ts','src/css-functions.ts','src/utils/variables.ts']
  const sources = await Promise.all(sourcePaths.map(async path => ({ path:`packages/tailwindcss/${path}`, sha256:sha256(await readFile(resolve(directory, '../../../ref/tailwindcss/packages/tailwindcss', path))) })))
  return { schemaVersion:1, sourceCommit:'1334c99db8fd26c8ea065375dd9259800863f072', oracleVersion:'4.1.13', fullFamilyConformance:'pending', bareValueDataTypes:['number','integer','ratio','percentage'], arbitraryDataTypes:types, sources, cases }
}
export async function checkCustomUtilityFixtures() {
  const actual=await customUtilityFixtures(); assert.deepEqual(actual, JSON.parse(await readFile(resolve(directory,'custom-utilities-fixtures.json'),'utf8')))
  return { cases:actual.cases.length, cssCases:actual.cases.filter(item=>Object.hasOwn(item,'expectedCss')).length, errorCases:actual.cases.filter(item=>Object.hasOwn(item,'expectedError')).length, warningCases:actual.cases.filter(item=>item.expectedWarnings?.length).length }
}
if(process.argv[1] && resolve(process.argv[1])===fileURLToPath(import.meta.url)) {
  if(process.argv.includes('--write')) { const actual=await customUtilityFixtures(); await writeFile(resolve(directory,'custom-utilities-fixtures.json'),JSON.stringify(actual,null,2)+'\n'); console.log(JSON.stringify({cases:actual.cases.length,cssCases:actual.cases.filter(item=>Object.hasOwn(item,'expectedCss')).length,errorCases:actual.cases.filter(item=>Object.hasOwn(item,'expectedError')).length})) }
  else console.log(JSON.stringify(await checkCustomUtilityFixtures()))
}
