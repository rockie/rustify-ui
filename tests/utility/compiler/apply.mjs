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
const sha256 = value => createHash('sha256').update(value).digest('hex')
const banner = value => value.replace(/^\/\*! tailwindcss[^\n]*\n/, '')
const declarations = '--spacing: .25rem; --width-card: 13px; --width-alias: var(--width-card); --color-brand: #123456; --color-alias: var(--color-brand); --opacity-muted: .35; --breakpoint-card: 40rem; --radius-card: 7px; --shadow-card: 0 1px 3px #123456; --animate-card: application-motion 1s linear; --animate-alias: var(--animate-card); --application-card: 17px; @keyframes application-motion { to { opacity:0; } }'
const themes = { empty:'', base:`@theme { ${declarations} }`, inline:`@theme inline { ${declarations} }`, reference:`@theme reference { ${declarations} }`, default:`@theme default { ${declarations} }`, static:`@theme static { ${declarations} }`, prefix:`@theme prefix(tw) { ${declarations} }`, prefixInline:`@theme prefix(tw) inline { ${declarations} }`, prefixReference:`@theme prefix(tw) reference { ${declarations} }` }
const prefixFor = mode => mode.startsWith('prefix') ? 'tw:' : ''
const property = name => `@property --${name} { syntax:"*"; inherits:false; initial-value:initial; }`
const frames = name => `@keyframes ${name} { from { opacity:0; } to { opacity:1; } }`
const resource = name => `${property(name)} ${frames(name)}`
const variants = '@custom-variant application (&:hover, &:focus); @custom-variant application-media (@media (width > 13px)); @custom-variant application-pseudo (&::before); @custom-variant application-discard { color:blue; }'

export async function applyFixtures() {
  const cases = [], reusedCases = [], signatures = new Set()
  function optionsFor(assets) {
    return assets ? { base:'/app', loadStylesheet:async (uri, base) => {
      const id = posix.normalize(posix.join(base, uri))
      const asset = assets.find(asset => asset.id === id) ?? assets.find(asset => asset.id === uri)
      assert.ok(asset, `missing oracle asset ${uri} from ${base}`)
      return { path:asset.id, base:posix.dirname(asset.id), content:asset.content }
    } } : undefined
  }
  async function result(input, candidates, assets, batches) {
    const warnings = [], warn = console.warn
    console.warn = (...args) => warnings.push(args.join(' '))
    let phase = 'compile'
    try {
      const compiler = await compile(input, optionsFor(assets)); phase = 'build'
      const buildSteps = batches ? batches.map(candidates => { const expectedCss = banner(compiler.build(candidates)); return { candidates, expectedCss, expectedCssSha256:sha256(expectedCss) } }) : undefined
      const expectedCss = buildSteps ? buildSteps.at(-1).expectedCss : banner(compiler.build(candidates))
      return { expectedCss, expectedCssSha256:sha256(expectedCss), ...(buildSteps ? { buildSteps } : {}), ...(warnings.length ? { expectedWarnings:warnings } : {}) }
    } catch (error) {
      return { expectedError:error.message, expectedErrorName:error.name, expectedErrorPhase:phase, ...(warnings.length ? { expectedWarnings:warnings } : {}) }
    } finally { console.warn = warn }
  }
  async function add(id, input, candidates = [], assets, batches) {
    const signature = JSON.stringify([input, candidates, assets, batches])
    if (signatures.has(signature)) return
    signatures.add(signature)
    cases.push({ id:`apply:${id}`, input, candidates, ...(assets ? { entry:'/app/input.css', assets } : {}), ...await result(input, candidates, assets, batches) })
  }
  async function themed(id, mode, body, candidates = [], tailwind = '@tailwind utilities;') {
    await add(id, `${themes[mode]} ${variants} ${tailwind} ${body}`.trim(), candidates)
  }
  const candidates = ['block','flex','underline','w-card','w-alias','w-[13px]','w-(--probe)','w-[length:var(--probe)]','p-2','p-1/2','-inset-2','bg-brand','bg-alias/50','bg-brand/muted','bg-brand/[var(--alpha)]','bg-[#123456]/35','bg-(color:--probe)/50','[color:red]','[color:red]/50','[--probe:13px]','rounded-card','border-2','opacity-50','content-["a"]','before:content-["b"]','shadow-card','ring-2','transform','translate-x-full','rotate-13','filter','blur-[3px]','animate-card','animate-alias','animate-[other_1s_linear]','hover:bg-brand','card:w-card','group-hover:w-[13px]','peer-focus:underline','in-hover:flex','has-[p]:underline','not-hover:block','supports-[display:grid]:block','[&>p]:w-[13px]','[@media(width>13px)]:w-[17px]','application:w-card','group-application:w-card','not-application:w-card','application-discard:block','application-pseudo:content-["a"]','block!','!block','unknown-utility','unknown:w-card','unknown:also-unknown:hover:block','unknown:bg-missing','group','peer','bg-brand/nope','w-card/50','[color:red]/[]','w-[','hover:','block/mark','', '!important']
  const wrappers = { style:'.probe { BODY }', root:'BODY', nested:'.probe { &:hover { BODY } }', media:'@media screen { .probe { BODY } }', layer:'@layer components { .probe { BODY } }' }
  for (const mode of Object.keys(themes)) for (const candidate of candidates) for (const [wrapperName, wrapper] of Object.entries(wrappers)) {
    const applied = candidate ? prefixFor(mode) + candidate : ''
    await themed(`ordinary:${mode}:${wrapperName}:${candidate || '<empty>'}`, mode, wrapper.replace('BODY', `@apply ${applied};`))
  }
  for (const mode of ['empty','base','reference','prefix']) for (const important of ['', 'important']) for (const candidate of ['block','block!','!block','hover:block','hover:block!','animate-card','before:content-["a"]']) {
    await themed(`global-important:${mode}:${important || 'normal'}:${candidate}`, mode, `.probe { @apply ${prefixFor(mode)}${candidate}; }`, [prefixFor(mode)+'block'], `@tailwind utilities ${important};`)
  }
  const orders = [
    ['w-[13px]','w-[17px]'], ['w-[17px]','w-[13px]'], ['block','flex'], ['flex','block'], ['content-["a"]','content-["b"]'], ['content-["b"]','content-["a"]'],
    ['w-card','w-card','hover:w-[17px]','w-card!'], ['bg-brand/50','underline','p-2','before:content-["a"]','animate-card','transform'], ['unknown-first','unknown-second'], ['unknown-second','unknown-first'], ['hover:unknown','unknown:hover:block'],
  ]
  for (const mode of Object.keys(themes)) for (const order of orders) for (const shape of ['single','split','authored-interleaved']) {
    const parts = order.map(candidate => prefixFor(mode)+candidate)
    const body = shape === 'single' ? `@apply ${parts.join(' ')};` : shape === 'split' ? parts.map(candidate=>`@apply ${candidate};`).join(' ') : `color:red; @apply ${parts[0]}; color:blue; @apply ${parts.slice(1).join(' ')}; color:green;`
    await themed(`ordering:${mode}:${shape}:${order.join(',')}`, mode, `.probe { ${body} }`)
  }
  for (const whitespace of [' ', '  ', '\t', '\n', '\r\n', '\f', '\u00a0']) for (const margin of ['', ' ', '\n']) await add(`whitespace:${JSON.stringify(whitespace)}:${JSON.stringify(margin)}`, `@tailwind utilities; .probe { @apply${whitespace}${margin}flex${whitespace}underline${margin}; }`)
  for (const value of ['flex /* middle */ underline','/* start */ flex','flex /* end */','"flex"', "'flex'", 'flex !important','flex!important','flex,underline','flex;','flex{}','flex(underline)','flex\\ underline','flex\\:underline']) await add(`parameter-syntax:${value}`, `@tailwind utilities; .probe { @apply ${value}; }`)
  for (const body of ['@apply flex;', '.child { @apply flex; }', '@media screen { @apply flex; }', '@variant hover { @apply flex; }']) for (const context of ['@keyframes motion { from { BODY } }','@theme { @keyframes motion { to { BODY } } }','@utility unused { @keyframes motion { to { BODY } } }']) await themed(`keyframes-error:${context}:${body}`, 'base', context.replace('BODY',body))
  for (const name of ['@font-face','@page','@unknown probe','@supports (display:grid)']) await themed(`other-context:${name}`, 'base', `${name} { @apply w-card underline; }`)
  for (const body of ['@apply flex;','@apply unknown;','.probe { @apply flex; }']) await add(`theme-context:${body}`, `@theme { --color-brand:red; ${body} } @tailwind utilities;`)
  const staticTemplates = {
    basic:'@apply flex flex-col underline;', nested:'@apply bg-brand; &:hover { @apply bg-brand/50; } &:disabled { @apply opacity-50; }', important:'@apply w-card! hover:bg-brand!;',
    rawResources:`${resource('application-raw')} @apply before:content-["a"] transform;`, variant:'@variant hover { @apply w-card; }', variantBefore:'@variant before { @apply bg-brand/50; }',
    unknown:'@apply unknown-utility;', unknownVariant:'@apply unknown:block;', keyframes:'@keyframes forbidden { to { @apply flex; } }', empty:'@apply;', repeated:'@apply flex flex; @apply flex;',
  }
  for (const mode of ['empty','base','inline','reference']) for (const [name, template] of Object.entries(staticTemplates)) for (const use of ['unused','authored','candidate','both','important']) {
    const body = `@utility application-static { ${template} } ${use === 'authored' || use === 'both' || use === 'important' ? `.probe { @apply application-static${use === 'important' ? '!' : ''}; }` : ''}`
    await themed(`custom-static:${mode}:${name}:${use}`, mode, body, use === 'candidate' || use === 'both' ? ['application-static','hover:application-static'] : [])
  }
  const functionalTemplates = {
    integer:'--application-result: --value(integer); @apply flex;', theme:'width:--value(--width-*,[length]); @apply underline;', fraction:'width:calc(--value(ratio) * 100%); @apply flex;', modifier:'width:--value(integer); opacity:--modifier(number,[number]); @apply underline;',
    nested:'width:--value(--width-*,[length]); &:hover { @apply bg-brand/50; }', variant:'@variant hover { width:--value(integer); @apply flex; }', applyArgument:'@apply w-[--value(integer)];', unknown:'width:--value(integer); @apply unknown-utility;',
  }
  for (const mode of ['empty','base','inline','reference','prefix']) for (const [name, template] of Object.entries(functionalTemplates)) for (const value of ['13','card','alias','1/2','[17px]','13/50','13/[.5]','unknown','']) for (const use of ['unused','authored','candidate']) {
    // Prefix all CSS @apply arguments as well as external candidates. Values and variant declarations stay unchanged.
    const prefixed = template.replace(/@apply ([^;]+);/g, (_, args) => `@apply ${args.split(/\s+/).map(candidate => prefixFor(mode)+candidate).join(' ')};`)
    const candidate = `${prefixFor(mode)}application-functional${value ? '-'+value : ''}`
    await themed(`custom-functional:${mode}:${name}:${value || '<empty>'}:${use}`, mode, `@utility application-functional-* { ${prefixed} } ${use === 'authored' ? `.probe { @apply ${candidate}; }` : ''}`, use === 'candidate' ? [candidate] : [])
  }
  const dependencyGraphs = {
    forward:'@utility application-a { @apply application-b; } @utility application-b { @apply application-c; } @utility application-c { @apply flex; }',
    reverse:'@utility application-c { @apply flex; } @utility application-b { @apply application-c; } @utility application-a { @apply application-b; }',
    diamond:'@utility application-a { @apply application-b application-c; } @utility application-b { @apply application-d; } @utility application-c { @apply application-d; } @utility application-d { @apply flex; }',
    nested:'@utility application-a { &:hover { @apply application-b; } } @utility application-b { &:disabled { @apply application-c; } } @utility application-c { @apply flex; }',
    variants:'@utility application-a { @apply hover:application-b!; } @utility application-b { @apply focus:application-c; } @utility application-c { @apply flex; }',
    self:'@utility application-a { @apply application-a; }', selfVariant:'@utility application-a { @apply hover:application-a; }', pair:'@utility application-a { @apply application-b; } @utility application-b { @apply application-a; }',
    pairNested:'@utility application-a { .child { @apply hover:application-b; } } @utility application-b { .child { @apply focus:application-a; } }',
    triple:'@utility application-a { @apply application-b; } @utility application-b { @apply application-c; } @utility application-c { @apply application-a; }',
    duplicate:'@utility application-a { @apply application-b; } @utility application-a { width:13px; } @utility application-b { @apply underline; }',
    duplicateCycle:'@utility application-a { @apply application-b; } @utility application-a { @apply application-c; } @utility application-b { @apply application-c; } @utility application-c { @apply application-a; }',
    functionalSelf:'@utility application-a-* { width:--value(integer); @apply application-a-13; }', functionalStatic:'@utility application-a-* { width:--value(integer); @apply application-b; } @utility application-b { @apply application-a-13; }',
    sameRoot:'@utility application-a { @apply application-a-13; } @utility application-a-* { width:--value(integer); }',
    nativeCollision:'@utility flex { @apply block; } @utility application-a { @apply flex; }', nativeCycle:'@utility flex { @apply flex; }',
    arbitrary:'@utility application-a { @apply [--application-a:13px]; }', functionalChain:'@utility application-a-* { width:--value(integer); @apply application-b-13; } @utility application-b-* { opacity:--value(number); @apply flex; }',
  }
  for (const [name, graph] of Object.entries(dependencyGraphs)) for (const authoredFirst of [false,true]) for (const used of [false,true]) {
    const consumer = '.probe { @apply application-a; }'
    await themed(`dependency:${name}:${authoredFirst}:${used}`, 'base', authoredFirst ? `${consumer} ${graph}` : `${graph} ${consumer}`, used ? ['application-a','application-a-13','application-b'] : [])
    await themed(`dependency-unused:${name}:${authoredFirst}:${used}`, 'base', graph, used ? ['application-a','application-a-13','application-b'] : [])
  }
  for (const mode of ['empty','base','prefix']) for (const excluded of ['flex','hover:flex','w-card','unknown']) for (const applied of [excluded, 'underline']) {
    const prefix = prefixFor(mode)
    await themed(`source-exclusion:${mode}:${excluded}:${applied}`, mode, `@source not inline("${prefix}${excluded}"); .probe { @apply ${prefix}${applied}; }`, [prefix+excluded])
  }
  for (const candidate of ['block','tw:block','hover:tw:block','tw:hover:block','tweak:block','tw:unknown','tw:unknown:block','tw:unknown:also-unknown:block','tw:group-application:w-card','tw:not-application:w-card']) await themed(`prefix-errors:${candidate}`, 'prefix', `.probe { @apply ${candidate}; }`)
  for (const position of ['before','after']) for (const inner of ['before:content-["a"]','shadow-card','transform','animate-card','[--raw:13px]']) for (const wrapper of ['.probe { BODY }','@utility application-resource { BODY }']) for (const important of [false,true]) {
    const body = position === 'before' ? `${resource('application-outer')} @slot;` : `@slot; ${resource('application-outer')}`
    const input = `@custom-variant outer { ${body} } ${wrapper.replace('BODY', `@apply outer:${inner}${important ? '!' : ''};`)}`
    await themed(`resource-position:${position}:${inner}:${wrapper}:${important}`, 'base', input, ['application-resource'])
  }
  for (const variant of ['hover','before','application','application-discard']) for (const shape of ['applicationOutside','applyOutside','unusedUtility']) {
    const input = shape === 'applicationOutside' ? `.probe { @variant ${variant} { @apply shadow-card animate-card; } }` : shape === 'applyOutside' ? `@utility application-probe { @variant ${variant} { @apply shadow-card animate-card; } } .probe { @apply application-probe!; }` : `@utility application-probe { @variant ${variant} { @apply shadow-card animate-card; } }`
    await themed(`variant-interaction:${variant}:${shape}`, 'base', input)
  }
  for (const duplicateSlots of [false,true]) for (const used of [false,true]) {
    const input = `@custom-variant resources { ${resource('application-duplicated')} ${duplicateSlots ? '&:hover { @slot; } &:focus { @slot; }' : '@slot;'} } @utility application-a { @apply resources:block; } @utility application-b { @apply resources:flex; } .one { @apply application-a; } .two { @apply application-b; }`
    await themed(`resource-duplicates:${duplicateSlots}:${used}`, 'base', input, used ? ['application-a','application-b'] : [])
  }
  const assets = [
    { id:'/app/theme.css', content:`${themes.base} ${variants} @utility application-imported { @apply w-card before:content-["imported"] animate-card; } .imported { @apply underline; } @tailwind utilities;` },
    { id:'/app/utilities.css', content:'@tailwind utilities;' },
    { id:'/app/nested/entry.css', content:'@reference "../theme.css"; .nested { @apply application-imported; }' },
    { id:'/app/unknown.css', content:'@utility application-broken { @apply unknown-imported; }' },
  ]
  for (const directive of ['@import "theme.css";', '@reference "theme.css";', '@import "theme.css" reference;', '@import "theme.css" layer(components);','@import "theme.css" screen;','@import "theme.css" supports(display:grid);','@import "nested/entry.css";','@reference "nested/entry.css";']) for (const candidate of ['application-imported','hover:application-imported','application-imported!','bg-brand/50','unknown']) for (const used of [false,true]) await add(`import:${directive}:${candidate}:${used}`, `${directive} .probe { @apply ${candidate}; } @tailwind utilities;`, used ? ['application-imported','w-card'] : [], assets)
  for (const directive of ['@reference "unknown.css";', '@import "unknown.css";', '@import "unknown.css" reference;']) await add(`import-unused-error:${directive}`, `${directive} .probe { @apply flex; } @tailwind utilities;`, [], assets)
  for (const important of ['', 'important']) await add(`import-important:${important || 'normal'}`, `@import "utilities.css" ${important}; .normal { @apply flex; } .important { @apply flex!; }`, ['flex'], assets)
  for (const mode of ['empty','base','reference','prefix']) for (const tailwind of ['', '@tailwind utilities;','@layer utilities { @tailwind utilities; }']) {
    await themed(`without-utilities:${mode}:${tailwind || 'absent'}`, mode, `.probe { @apply ${prefixFor(mode)}${mode === 'empty' ? 'flex' : 'w-card'}; }`, [prefixFor(mode)+'w-[19px]'], tailwind)
    await themed(`authored-theme-used:${mode}:${tailwind || 'absent'}`, mode, `.probe { @apply ${prefixFor(mode)}animate-alias ${prefixFor(mode)}bg-alias/50; }`, [], tailwind)
  }
  for (const mode of ['base','reference','prefix']) for (const reverse of [false,true]) {
    const prefix = prefixFor(mode)
    const input = `${themes[mode]} ${variants} @utility application-a { @apply ${prefix}before:content-["a"] ${prefix}w-card; } .probe { @apply ${prefix}application-a ${prefix}animate-card; } @tailwind utilities;`
    const candidates = ['application-a','application-a!','shadow-card','hover:bg-brand','unknown'].map(candidate => prefix+candidate)
    const ordered = reverse ? candidates.toReversed() : candidates
    const batches = [[], [ordered[0]], [ordered[1],ordered[2]], [ordered[0]], [ordered[3],ordered[4]], []]
    await add(`incremental:${mode}:${reverse}`, input, [...new Set(batches.flat())], undefined, batches)
  }
  for (const applied of ['flex','unknown-generated','animate-card','before:content-["generated"]']) for (const use of ['unused','authoredVariant','authoredApply','candidate']) {
    const definitions = `@custom-variant generated { @apply ${applied}; @slot; }`
    const consumer = use === 'authoredVariant' ? '.probe { @variant generated { width:13px; } }' : use === 'authoredApply' ? '.probe { @apply generated:block; }' : ''
    await themed(`generated-apply:${applied}:${use}`, 'base', `${definitions} ${consumer}`, use === 'candidate' ? ['generated:block'] : [])
  }
  for (const errorFirst of [false,true]) for (const error of ['unknown-first','unknown:hover:block','application-a']) {
    const definitions = '@utility application-a { @apply application-b; } @utility application-b { @apply application-a; }'
    const first = `.first { @apply ${error}; }`, second = '.second { @apply unknown-second; }'
    await themed(`error-order:${errorFirst}:${error}`, 'base', `${error === 'application-a' ? definitions : ''} ${errorFirst ? first+' '+second : second+' '+first}`)
  }
  for (const syntax of ['@apply flex { color:red; }','@apply flex() { color:red; }','@APPLY flex;','@Apply flex;','@apply-flex;','@applyflex;','@apply flex','@apply;']) await add(`at-rule-boundary:${syntax}`, `@tailwind utilities; .probe { ${syntax} }`)
  for (const mode of ['base','inline','reference','prefix']) for (const body of [
    'width:--spacing(2); @apply w-card;', 'color:--alpha(var(--color-brand) / 50%); @apply bg-brand/50;', '@media (width >= theme(--breakpoint-card)) { @apply w-card; }',
    '@apply [width:--spacing(2)];', '@apply [color:--alpha(var(--color-brand)_/_50%)];', '@apply [width:--spacing(bad)];',
  ]) {
    const prefixed = body.replace(/@apply ([^;]+);/g,(_, args)=>`@apply ${args.split(/\s+/).map(candidate=>prefixFor(mode)+candidate).join(' ')};`)
    await themed(`functions-order:${mode}:${body}`, mode, `.probe { ${prefixed} }`)
  }
  for (const excluded of ['{flex,underline}','{hover:,focus:,}flex','w-{card,[13px]}']) for (const applied of ['flex','hover:flex','w-card','w-[13px]']) await themed(`brace-exclusion:${excluded}:${applied}`, 'base', `@source not inline("${excluded}"); .probe { @apply ${applied}; }`)
  for (const [fixture, id, reason] of [
    ['custom-utilities-fixtures.json','custom:value:[number]:empty:<none>','functional arbitrary value parsing already frozen; new cases add apply substitution'],
    ['custom-variants-fixtures.json','custom-variant:selector:hover:empty:sample','CSS-native custom variant registration already frozen; new cases add apply and variant interaction'],
    ['prefix-fixtures.json','prefix::1','theme prefix rewriting already frozen; new cases add strict apply prefix failures'],
    ['imports-fixtures.json','import:relative-theme','relative theme import resolution already frozen; new cases add imported apply and reference behavior'],
    ['motion-fixtures.json','motion:animate-card:empty','animation candidate rejection without a theme already frozen; new cases add strict apply errors'],
    ['functions-fixtures.json','functions::authored:0','authored CSS function expansion already frozen; apply runs after this phase'],
  ]) {
    const raw = await readFile(resolve(directory,fixture),'utf8'), stored = JSON.parse(raw).cases.find(item => item.id === id)
    assert.ok(stored, `missing reference ${fixture}:${id}`)
    const actual = await result(stored.input, stored.candidates, stored.assets, stored.buildSteps?.map(step => step.candidates))
    for (const key of ['expectedCss','expectedCssSha256','expectedError','expectedErrorName','expectedErrorPhase','buildSteps','expectedWarnings']) if (Object.hasOwn(stored,key)) assert.deepEqual(actual[key],stored[key],`${fixture}:${id}:${key}`)
    reusedCases.push({ fixture, id, fixtureSha256:sha256(raw), reason })
  }
  const ids = new Set(); for (const item of cases) { assert.ok(!ids.has(item.id), `duplicate ${item.id}`); ids.add(item.id) }
  const paths = ['src/apply.ts','src/index.ts','src/index.test.ts','src/compile.ts','src/ast.ts','src/variants.ts','src/candidate.ts','src/design-system.ts','src/utilities.ts','src/utilities.test.ts','src/theme.ts','src/at-import.ts','src/css-functions.ts','src/utils/segment.ts','src/utils/default-map.ts','src/property-order.ts']
  const sources = await Promise.all(paths.map(async path => ({ path:`packages/tailwindcss/${path}`, sha256:sha256(await readFile(resolve(directory,'../../../ref/tailwindcss/packages/tailwindcss',path))) })))
  return { schemaVersion:1, sourceCommit:'1334c99db8fd26c8ea065375dd9259800863f072', oracleVersion:'4.1.13', fullFamilyConformance:'pending', mechanism:'CSS-native @apply dependency sorting and candidate AST substitution', incrementalSchema:'buildSteps executes additive official build calls; top candidates is their union and expectedCss is the final step', excludedBoundaries:['JavaScript plugin/config execution; CSS @source not inline exclusion remains in scope','lone UTF-16 code units: separate text migration'], sources, reusedCases, cases }
}
function counts(actual) {
  return { cases:actual.cases.length, cssCases:actual.cases.filter(item => Object.hasOwn(item,'expectedCss')).length, errorCases:actual.cases.filter(item => Object.hasOwn(item,'expectedError')).length, compileErrors:actual.cases.filter(item => item.expectedErrorPhase === 'compile').length, buildErrors:actual.cases.filter(item => item.expectedErrorPhase === 'build').length, incrementalCases:actual.cases.filter(item => item.buildSteps).length, reusedCases:actual.reusedCases.length }
}
export async function checkApplyFixtures() {
  const actual = await applyFixtures(); assert.deepEqual(actual, JSON.parse(await readFile(resolve(directory,'apply-fixtures.json'),'utf8')))
  return counts(actual)
}
if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  if (process.argv.includes('--write')) { const actual=await applyFixtures(); await writeFile(resolve(directory,'apply-fixtures.json'),JSON.stringify(actual,null,2)+'\n'); console.log(JSON.stringify(counts(actual))) }
  else console.log(JSON.stringify(await checkApplyFixtures()))
}
