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
const banner = css => css.replace(/^\/\*! tailwindcss[^\n]*\n/, '')
const declarations = '--spacing: .25rem; --width-card: 13px; --width-alias: var(--width-card); --color-brand: #123456; --opacity-muted: .35; --breakpoint-sm: 40rem;'
const themes = { empty:'', base:`@theme { ${declarations} }`, inline:`@theme inline { ${declarations} }`, reference:`@theme reference { ${declarations} }`, default:`@theme default { ${declarations} }`, static:`@theme static { ${declarations} }`, prefix:`@theme prefix(tw) { ${declarations} }`, prefixInline:`@theme prefix(tw) inline { ${declarations} }`, prefixReference:`@theme prefix(tw) reference { ${declarations} }` }
const selectors = [
  ['hover', '&:hover'], ['list', '&:hover, &:focus'], ['is', '&:is(:hover,:focus)'], ['attribute', '&[data-on="true"]'], ['ancestor', '.host &'], ['child', '& > p'], ['relative', '> p'], ['bare', 'p'], ['pseudo', '&::before'], ['nested-pseudo', '&:where(::before)'], ['mixed-pseudo', '&:hover, &::after'],
  ['media', '@media (width > 300px)'], ['supports', '@supports (display: grid)'], ['container', '@container (width > 20rem)'], ['media-list', '@media (hover: hover), @media (pointer: fine)'], ['mixed', '@media (width > 300px), &:hover'], ['mixed-reversed', '&:hover, @supports (display: grid)'], ['layer', '@layer utilities'], ['starting', '@starting-style'], ['font-face', '@font-face'], ['keyframes', '@keyframes custom-motion'], ['supports-selector', '@supports selector(:has(p))'], ['unknown-at-rule', '@unknown probe'],
  ['empty', ''], ['comma', ','], ['trailing', '&:hover,'], ['leading', ', &:hover'], ['double-comma', '&:hover,, &:focus'], ['function-comma', '&:is(.one,.two), &:not(.three,.four)'], ['quoted-comma', '&[data-on="a,b"], &:hover'], ['amp-less-at-rule', '@media screen'],
]
const bodies = {
  rootSlot:'@slot;', styleSlot:'&:hover { @slot; }', listSlot:'&:hover, &:focus { @slot; }', atRuleSlot:'@media (width > 300px) { @slot; }', supportsSlot:'@supports (display: grid) { @slot; }', containerSlot:'@container (width > 20rem) { @slot; }',
  nestedSlot:'@media (width > 300px) { &:hover { @slot; } }', siblingSlots:'&:hover { @slot; } &:focus { @slot; }', duplicateRootSlots:'@slot; @slot;', duplicateNestedSlots:'&:hover { @slot; @slot; }',
  slotParameters:'&:hover { @slot probe; }', slotBody:'&:hover { @slot { color: red; } }', noSlot:'&:hover { color: red; }', declarationsOnly:'color: red; width: 17px;', declarationsAndSlot:'color: red; &:hover { @slot; }', emptyRule:'&:hover { } @slot;',
  pseudoSlot:'&::before { @slot; }', pseudoAndStyle:'&::before { @slot; } &:hover { @slot; }', mixedSiblings:'@media (width > 300px) { @slot; } &:hover { @slot; }', layerSlot:'@layer utilities { @slot; }', startingSlot:'@starting-style { @slot; }', unknownSlot:'@unknown probe { @slot; }',
  authoredImportant:'color: red !important; &:hover { @slot; }', themeValues:'color: var(--color-brand); width: var(--width-card); &:hover { @slot; }',
  resources:'@property --custom-variant-length { syntax:"<length>"; inherits:false; initial-value:13px; } @keyframes custom-variant-motion { from { opacity:0; } to { opacity:1; } } &:hover { width:var(--custom-variant-length); @slot; }',
  resourceSlot:'@property --custom-variant-raw { syntax:"*"; inherits:true; @slot; } &:hover { @slot; }', nestedDefinition:'&:hover { @custom-variant inner (&:focus); @slot; }', commentOnly:'/* @slot; */', empty:'',
}
const variantForms = ['sample', 'sample/mark', 'sample/[]', 'sample-unknown', 'group-sample', 'group-sample/mark', 'peer-sample', 'peer-sample/mark', 'in-sample', 'not-sample', 'has-sample', 'not-group-sample', 'group-not-sample', 'peer-not-sample', 'has-not-sample', 'not-has-sample', 'sample:hover', 'hover:sample', 'sample:focus', 'group-hover:sample', 'sample:sample']

export async function customVariantFixtures() {
  const cases = []
  function optionsFor(assets) {
    return assets ? { base:'/app', loadStylesheet:async (uri, base) => {
      const id = posix.normalize(posix.join(base, uri)); const asset = assets.find(asset => asset.id === id) ?? assets.find(asset => asset.id === uri)
      assert.ok(asset, `missing oracle asset ${uri} from ${base}`)
      return { path:asset.id, base:posix.dirname(asset.id), content:asset.content }
    } } : undefined
  }
  async function add(id, input, candidates, assets, batches) {
    const warnings = []; const warn = console.warn; console.warn = (...args) => warnings.push(args.join(' '))
    const base = { id, input, candidates, ...(assets ? { entry:'/app/input.css', assets } : {}) }
    let phase = 'compile'
    try {
      const compiler = await compile(input, optionsFor(assets)); phase = 'build'
      const buildSteps = batches ? batches.map(candidates => { const expectedCss = banner(compiler.build(candidates)); return { candidates, expectedCss, expectedCssSha256:sha256(expectedCss) } }) : undefined
      const expectedCss = buildSteps ? buildSteps.at(-1).expectedCss : banner(compiler.build(candidates))
      cases.push({ ...base, expectedCss, expectedCssSha256:sha256(expectedCss), ...(buildSteps ? { buildSteps } : {}), ...(warnings.length ? { expectedWarnings:warnings } : {}) })
    } catch (error) {
      cases.push({ ...base, expectedError:error.message, expectedErrorName:error.name, expectedErrorPhase:phase, ...(warnings.length ? { expectedWarnings:warnings } : {}) })
    } finally { console.warn = warn }
  }
  async function definition(id, mode, declaration, candidates) { await add(id, `${themes[mode]} ${declaration} @tailwind utilities;`.trim(), candidates) }
  for (const [key, selector] of selectors) {
    for (const mode of ['empty','base','inline','reference','default']) for (const form of variantForms) await definition(`custom-variant:selector:${key}:${mode}:${form}`, mode, `@custom-variant sample (${selector});`, [`${form}:w-[13px]`, `${form}:w-card!`])
    for (const mode of ['prefix','prefixInline','prefixReference']) await definition(`custom-variant:selector:${key}:${mode}:union`, mode, `@custom-variant sample (${selector});`, ['tw:sample:w-card', 'tw:group-sample/mark:bg-brand/50!', 'tw:not-sample:w-[17px]', 'tw:hover:sample:w-[19px]!', 'sample:w-[13px]'])
    for (const form of ['sample','group-sample','not-sample']) await definition(`custom-variant:legacy-selector:${key}:${form}`, 'base', `@variant sample (${selector});`, [`${form}:w-card`, `${form}:bg-brand/50!`])
  }
  for (const [key, body] of Object.entries(bodies)) {
    for (const mode of ['empty','base','inline','reference']) for (const form of variantForms) await definition(`custom-variant:body:${key}:${mode}:${form}`, mode, `@custom-variant sample { ${body} }`, [`${form}:w-[13px]`, `${form}:bg-brand/50!`])
    for (const mode of ['prefix','prefixInline','prefixReference','static']) await definition(`custom-variant:body:${key}:${mode}:union`, mode, `@custom-variant sample { ${body} }`, ['sample:w-card', 'before:sample:w-[17px]!', 'tw:sample:w-card!', 'tw:group-sample:w-[19px]', 'tw:not-sample:bg-brand/50'])
    for (const form of ['sample','group-sample','not-sample']) await definition(`custom-variant:legacy-body:${key}:${form}`, 'base', `@variant sample { ${body} }`, [`${form}:w-card`, `${form}:w-[17px]!`])
  }
  for (const name of ['sample','a','1','1sample','sample_Name','sample-fooBar','a--b','a__b','@sample','@1sample','Sample','_sample','-sample','sample_','sample-','@','sample.foo','sample/foo','sample:foo','sample*','sample$','☃','sample☃','']) {
    for (const atRule of ['@custom-variant','@variant']) for (const syntax of ['selector','body']) await definition(`custom-variant:name:${name || '<empty>'}:${atRule}:${syntax}`, 'empty', syntax === 'selector' ? `${atRule} ${name} (&:hover);` : `${atRule} ${name} { &:hover { @slot; } }`, [`${name}:w-[13px]`, `group-${name}:w-[13px]`])
  }
  for (const declaration of ['@custom-variant sample;', '@custom-variant sample {}', '@custom-variant sample (&:hover) {}', '@custom-variant sample (&:hover) { @slot; }', '@custom-variant sample &:hover;', '@custom-variant sample "&:hover";', '@custom-variant sample (&:hover) ignored;', '@custom-variant sample (&:hover) (&:focus);', '@custom-variant sample (@media);', '@custom-variant sample (&);', '@variant sample;', '@variant sample {}', '@variant sample (&:hover) {}']) await definition(`custom-variant:declaration-boundary:${declaration}`, 'empty', declaration, ['sample:w-[13px]', 'group-sample:w-[17px]', 'not-sample:w-[19px]!'])
  for (const wrapper of ['.host { BODY }', '@media screen { BODY }', '@layer utilities { BODY }', '@supports (display:grid) { BODY }', '@theme { BODY }', '@utility sample { BODY }']) for (const declaration of ['@custom-variant sample (&:hover);', '@custom-variant sample { &:hover { @slot; } }', '@variant sample (&:hover);', '@variant sample { &:hover { @slot; } }']) await definition(`custom-variant:nested:${wrapper}:${declaration}`, 'base', wrapper.replace('BODY', declaration), ['sample:w-card'])
  for (const name of ['hover','before','dark','sm','group','peer','in','not','has','aria','@','@max']) for (const declaration of [`@custom-variant ${name} (&[data-probe]);`, `@custom-variant ${name} { &:focus { @slot; } }`]) await definition(`custom-variant:override:${declaration}`, 'base', declaration, [`${name}:w-card`, `${name}-hover:w-[13px]`, `group-${name}:w-[17px]!`, `hover:${name}:bg-brand`, `not-${name}:w-[19px]`, 'hover:w-[23px]', 'group-hover:w-[29px]'])
  for (const definitions of [
    '@custom-variant alpha (&:hover); @custom-variant beta (&:focus);', '@custom-variant beta (&:focus); @custom-variant alpha (&:hover);',
    '@custom-variant sample (&:hover); @custom-variant sample (&:focus);', '@custom-variant sample { &:hover { @slot; } } @custom-variant sample (@media (width > 300px));',
    '@custom-variant sample (&:hover); @custom-variant sample { @slot; }', '@custom-variant hover (&:focus); @custom-variant sample (&:hover);', '@custom-variant sample (&:hover); @custom-variant hover (&:focus);',
  ]) await definition(`custom-variant:order:${definitions}`, 'base', definitions, ['beta:w-card', 'alpha:w-card', 'sample:w-card', 'hover:w-card', 'focus:w-card', 'group-sample:w-[13px]', 'not-sample:w-[17px]!'])
  for (const input of [
    '@variant sample { color:red; } @custom-variant sample (&:hover);', '@custom-variant sample (&:hover); @variant sample { color:red; }', '.host { @variant sample { color:red; } } @custom-variant sample (&:hover);',
    '@variant hover { color:red; }', '.host { @variant hover { color:red; } }', '@variant unknown { color:red; }', '@variant group-sample { width:13px; } @custom-variant sample (&::before);',
    '@variant sample { &:hover { @slot; } }', '@variant sample { content:"@slot"; }', '@variant sample { @media screen { @slot; } }', '.host { @variant sample { @slot; } } @custom-variant sample (&:hover);',
  ]) await definition(`custom-variant:legacy-application:${input}`, 'base', input, ['sample:w-card', 'hover:w-[17px]!'])
  const assets = [
    { id:'/app/variants.css', content:`${themes.base} @custom-variant sample (&:hover, &:focus); @custom-variant alpha (@media (width > 300px));` },
    { id:'/app/nested/variants.css', content:'@import "more.css"; @custom-variant beta (&[data-on]);' },
    { id:'/app/nested/more.css', content:'@custom-variant sample { &:focus { @slot; } }' },
  ]
  for (const directive of ['@import "variants.css";', '@reference "variants.css";', '@import "variants.css" reference;', '@import "variants.css" layer(utilities);', '@import "variants.css" screen;', '@import "nested/variants.css";', '@reference "nested/variants.css";']) await add(`custom-variant:import:${directive}`, `${directive} @tailwind utilities;`, ['sample:w-card', 'alpha:w-[13px]', 'group-sample:bg-brand/50!', 'not-alpha:w-[17px]', 'beta:w-[19px]'], assets)
  for (const input of ['@import "variants.css"; @custom-variant sample (&:active);', '@custom-variant sample (&:active); @import "variants.css";', '@reference "variants.css"; @custom-variant sample { &:focus { @slot; } }']) await add(`custom-variant:import-override:${input}`, `${input} @tailwind utilities;`, ['sample:w-card', 'group-sample:w-[17px]'], assets)
  for (const mode of ['base','inline','reference','prefix']) for (const declaration of ['@custom-variant sample (&:hover, &:focus);', '@custom-variant sample { @media screen { &:hover { @slot; } } }']) {
    const prefix = mode === 'prefix' ? 'tw:' : ''
    const input = `${themes[mode]} ${declaration} @custom-variant alpha (&[data-on]); @tailwind utilities;`
    const candidates = ['alpha:w-card', 'sample:w-card', 'hover:sample:w-[17px]!', 'group-sample:w-[19px]', 'not-sample:bg-brand/50', 'sample:w-[13px]', 'unknown:w-[29px]'].map(candidate => prefix + candidate)
    await add(`custom-variant:sorted-union:${mode}:${declaration}`, input, candidates)
    await add(`custom-variant:reverse-sorted-union:${mode}:${declaration}`, input, candidates.toReversed())
    const batches = [[], [candidates[5]], [candidates[1], candidates[0]], [candidates[1]], [candidates[2], candidates[3], candidates[4], candidates[6]], []]
    await add(`custom-variant:incremental:${mode}:${declaration}`, input, [...new Set(batches.flat())], undefined, batches)
  }
  const property = name => `@property --${name} { syntax:"*"; inherits:false; initial-value:initial; }`
  const frames = name => `@keyframes ${name} { from { opacity:0; } to { opacity:1; } }`
  const resource = name => `${property(name)} ${frames(name)}`
  const applicationBodies = {
    hover:'@variant hover { width:13px; color:var(--color-brand); }',
    before:'@variant before { color:var(--color-brand); content:"probe"; }',
    nested:'@variant hover { @variant before { width:13px; color:var(--color-brand); } }',
    sibling:'@variant hover { width:13px; } @variant focus { color:var(--color-brand); }',
    unknown:'@variant unknown-application { width:13px; }',
    unknownNested:'@variant hover { @variant unknown-application { width:13px; } }',
    functional:'@variant hover { width:--value(integer); }',
  }
  for (const mode of ['empty','base','inline','reference','prefix','prefixInline','prefixReference']) {
    const prefix = mode.startsWith('prefix') ? 'tw:' : ''
    for (const [bodyName, body] of Object.entries(applicationBodies)) {
      const functional = bodyName === 'functional'
      const candidate = functional ? 'application-probe-17' : 'application-probe'
      const declaration = `@utility application-probe${functional ? '-*' : ''} { ${body} }`
      for (const [use, candidates] of Object.entries({ unused:[], used:[candidate], important:[`${candidate}!`], hover:[`hover:${candidate}`], union:[candidate, `${candidate}!`, `hover:${candidate}`, 'w-[19px]'] })) {
        await definition(`custom-variant:application:utility:${bodyName}:${mode}:${use}`, mode, declaration, candidates.map(candidate => prefix + candidate))
      }
    }
  }
  const applicationOrders = {
    outerUnknown:'.host { @variant unknown-outer { @variant unknown-inner { color:red; } } }',
    reversedUnknown:'.host { @variant unknown-inner { @variant unknown-outer { color:red; } } }',
    siblings:'.host { @variant unknown-first { color:red; } @variant unknown-second { color:blue; } }',
    reversedSiblings:'.host { @variant unknown-second { color:blue; } @variant unknown-first { color:red; } }',
    unusedUtilityFirst:'@utility application-probe { @variant unknown-utility { color:red; } } .host { @variant unknown-authored { color:blue; } }',
    authoredFirst:'.host { @variant unknown-authored { color:blue; } } @utility application-probe { @variant unknown-utility { color:red; } }',
    incompatibleOuter:'@custom-variant multi (&:hover, &:focus); .host { @variant not-multi { @variant unknown-inner { color:red; } } }',
    incompatibleInner:'@custom-variant multi (&:hover, &:focus); .host { @variant hover { @variant not-multi { color:red; } } }',
  }
  for (const [name, input] of Object.entries(applicationOrders)) for (const used of [false,true]) await definition(`custom-variant:application:preorder:${name}:${used ? 'used' : 'unused'}`, 'base', input, used ? ['application-probe'] : [])
  for (const inner of ['unknown-inner','before','hover']) for (const wrapper of ['.host { BODY }','@utility application-probe { BODY }']) for (const used of [false,true]) {
    const input = `@custom-variant discard { color:blue; } ${wrapper.replace('BODY', `@variant discard { @variant ${inner} { color:red; } }`)}`
    await definition(`custom-variant:application:discard:${inner}:${wrapper}:${used ? 'used' : 'unused'}`, 'base', input, used ? ['application-probe!'] : [])
  }
  for (const generated of ['unknown-generated','before','hover']) for (const original of ['', '@variant hover { color:red; }', '@variant unknown-original { color:red; }']) for (const wrapper of ['.host { BODY }','@utility application-probe { BODY }']) {
    await definition(`custom-variant:application:generated:${generated}:${original || 'plain'}:${wrapper}`, 'base', `@custom-variant generated { @variant ${generated} { @slot; } } ${wrapper.replace('BODY', `@variant generated { width:13px; ${original} }`)}`, ['application-probe!'])
  }
  for (const outerPosition of ['before','after']) for (const innerPosition of ['before','after']) for (const inner of ['resource','before']) for (const wrapper of ['.host { BODY }','@utility application-probe { BODY }']) for (const use of ['unused','used','important']) {
    const place = (position, name) => position === 'before' ? `${resource(name)} @slot;` : `@slot; ${resource(name)}`
    const definitions = `@custom-variant outer { ${place(outerPosition, 'application-outer')} } @custom-variant inner { ${place(innerPosition, 'application-inner')} }`
    const input = `${definitions} ${wrapper.replace('BODY', `@variant outer { @variant ${inner === 'resource' ? 'inner' : 'before'} { color:red; } }`)}`
    await definition(`custom-variant:application:resource-position:${outerPosition}:${innerPosition}:${inner}:${wrapper}:${use}`, 'base', input, use === 'unused' ? [] : [`application-probe${use === 'important' ? '!' : ''}`])
  }
  for (const mode of ['base','inline','reference','prefix']) for (const position of ['before','after']) {
    const prefix = mode === 'prefix' ? 'tw:' : ''
    const body = position === 'before' ? `${resource('application-outer')} &:hover { @slot; }` : `&:hover { @slot; } ${resource('application-outer')}`
    const declarations = `@custom-variant outer { ${body} } @utility application-probe { @variant before { color:var(--color-brand); } } @utility application-second { @variant outer { color:red; } }`
    for (const candidates of [['outer:application-probe'], ['outer:application-probe!'], ['application-second','outer:application-probe'], ['outer:application-probe','application-second']]) await definition(`custom-variant:application:candidate-resource:${mode}:${position}:${candidates.join(',')}`, mode, declarations, candidates.map(candidate => prefix + candidate))
    for (const builtin of ['shadow-sm','transform','content-["probe"]','before:content-["probe"]']) for (const important of [false,true]) {
      const shadowTheme = builtin === 'shadow-sm' ? '@theme { --shadow-sm: 0 1px 3px 0 rgb(0 0 0 / 0.1), 0 1px 2px -1px rgb(0 0 0 / 0.1); }' : ''
      const input = `${shadowTheme} @custom-variant outer { ${body} }`
      await definition(`custom-variant:application:builtin-resource:${mode}:${position}:${builtin}:${important}`, mode, input, [`${prefix}outer:${builtin}${important ? '!' : ''}`])
    }
  }
  for (const count of [1,2]) for (const important of [false,true]) for (const reversed of [false,true]) {
    const declarations = `@custom-variant resources { ${resource('application-shared')} &:hover { @slot; } } @utility application-probe { @variant resources { color:red; } } @utility application-second { @variant resources { color:blue; } }`
    const candidates = ['application-probe','application-second'].slice(0,count).map(candidate => `${candidate}${important ? '!' : ''}`)
    await definition(`custom-variant:application:resource-duplicates:${count}:${important}:${reversed}`, 'base', declarations, reversed ? candidates.toReversed() : candidates)
  }
  for (const mode of ['base','inline','reference','prefix']) for (const slot of ['@slot; @slot;','&:hover { @slot; } &:focus { @slot; }','&:hover { @slot; @slot; }']) for (const important of [false,true]) {
    const prefix = mode === 'prefix' ? 'tw:' : ''
    const input = `@custom-variant duplicate { ${slot} } @utility application-probe { @variant duplicate { @variant before { color:color-mix(in oklab, var(--color-brand) 50%, transparent); width:13px; color:var(--color-brand); } } }`
    await definition(`custom-variant:application:duplicate-slot-alias:${mode}:${slot}:${important}`, mode, input, [`${prefix}application-probe${important ? '!' : ''}`])
  }
  for (const variant of ['hover','before','outer']) for (const wrapper of ['.host { BODY }','@utility application-probe { BODY }']) {
    const input = `@custom-variant outer { &:hover { @slot; } } ${wrapper.replace('BODY', `@variant ${variant} { ${resource('application-authored')} color:red; }`)}`
    await definition(`custom-variant:application:authored-resources:${variant}:${wrapper}`, 'base', input, ['application-probe!'])
  }
  const applicationAssets = [{ id:'/app/applications.css', content:`${themes.base} @custom-variant resources { ${resource('application-imported')} &:hover { @slot; } } @utility application-probe { @variant resources { color:var(--color-brand); } } @utility application-second { @variant before { content:"probe"; } }` }]
  for (const directive of ['@import "applications.css";', '@reference "applications.css";', '@import "applications.css" reference;', '@import "applications.css" layer(utilities);']) for (const use of ['unused','used','important','union']) {
    const candidates = use === 'unused' ? [] : use === 'union' ? ['application-probe','application-second','hover:application-probe!'] : [`application-probe${use === 'important' ? '!' : ''}`]
    await add(`custom-variant:application:import:${directive}:${use}`, `${directive} @tailwind utilities;`, candidates, applicationAssets)
  }
  for (const mode of ['base','prefix']) for (const reversed of [false,true]) {
    const prefix = mode === 'prefix' ? 'tw:' : ''
    const input = `${themes[mode]} @custom-variant resources { ${resource('application-incremental')} &:hover { @slot; } } @utility application-probe { @variant resources { color:red; } } @utility application-second { @variant resources { color:blue; } } @tailwind utilities;`
    const candidates = ['application-probe','application-second','application-probe!'].map(candidate => prefix + candidate)
    const ordered = reversed ? candidates.toReversed() : candidates
    const batches = [[], [ordered[0]], [ordered[1]], [ordered[0]], [ordered[2]], []]
    await add(`custom-variant:application:incremental:${mode}:${reversed}`, input, [...new Set(batches.flat())], undefined, batches)
  }
  const ids = new Set(); for (const item of cases) { assert.ok(!ids.has(item.id), `duplicate ${item.id}`); ids.add(item.id) }
  const paths = ['src/index.ts','src/variants.ts','src/candidate.ts','src/ast.ts','src/compile.ts','src/theme.ts','src/at-import.ts','src/css-parser.ts','src/design-system.ts','src/utils/segment.ts','src/utils/is-valid-arbitrary.ts','src/utilities.ts','theme.css']
  const sources = await Promise.all(paths.map(async path => ({ path:`packages/tailwindcss/${path}`, sha256:sha256(await readFile(resolve(directory, '../../../ref/tailwindcss/packages/tailwindcss', path))) })))
  return { schemaVersion:1, sourceCommit:'1334c99db8fd26c8ea065375dd9259800863f072', oracleVersion:'4.1.13', fullFamilyConformance:'pending', mechanism:'CSS-native custom variant registration and root legacy variant declaration/application', incrementalSchema:'buildSteps executes additive official build calls; top candidates is their union and expectedCss is the final step', sources, cases }
}
export async function checkCustomVariantFixtures() {
  const actual = await customVariantFixtures(); assert.deepEqual(actual, JSON.parse(await readFile(resolve(directory,'custom-variants-fixtures.json'),'utf8')))
  return { cases:actual.cases.length, cssCases:actual.cases.filter(item => Object.hasOwn(item,'expectedCss')).length, errorCases:actual.cases.filter(item => Object.hasOwn(item,'expectedError')).length, incrementalCases:actual.cases.filter(item => item.buildSteps).length }
}
if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  if (process.argv.includes('--write')) { const actual = await customVariantFixtures(); await writeFile(resolve(directory,'custom-variants-fixtures.json'),JSON.stringify(actual,null,2)+'\n'); console.log(JSON.stringify({cases:actual.cases.length,cssCases:actual.cases.filter(item => Object.hasOwn(item,'expectedCss')).length,errorCases:actual.cases.filter(item => Object.hasOwn(item,'expectedError')).length,incrementalCases:actual.cases.filter(item => item.buildSteps).length})) }
  else console.log(JSON.stringify(await checkCustomVariantFixtures()))
}
