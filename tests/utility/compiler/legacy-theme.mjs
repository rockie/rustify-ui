import assert from 'node:assert/strict'
import {execFileSync} from 'node:child_process'
import {createHash} from 'node:crypto'
import {readFile, writeFile} from 'node:fs/promises'
import {createRequire} from 'node:module'
import {dirname, resolve} from 'node:path'
import {fileURLToPath} from 'node:url'
const directory = dirname(fileURLToPath(import.meta.url))
const oracle = createRequire(resolve(directory, 'oracle/package.json'))
const {compile} = oracle('tailwindcss')
const defaults = oracle('tailwindcss/defaultTheme')
assert.equal(oracle('tailwindcss/package.json').version, '4.1.13')
const marker = '__RUSTIFY_THEME_DEPENDENCY__'
function expression(key, value) {
  if (key === 'colors') return {from: 'color', base: {}}
  if (typeof value !== 'function') return {base: value}
  const calls = []
  const result = value({theme(path) { calls.push(path); return {[marker]: path} }})
  assert.equal(calls.length, 1, `unexpected dynamic default shape: ${key}`)
  assert.equal(result[marker], calls[0], key)
  const {[marker]: from, ...base} = result
  return {from, base}
}
const expressions = Object.fromEntries(Object.entries(defaults).map(([key, value]) => [key, expression(key, value)]))
function node(value) {
  if (typeof value === 'string') return `Node::Text(${JSON.stringify(value)})`
  if (Array.isArray(value)) return `Node::List(&[${value.map(node).join(',')}])`
  assert.equal(typeof value, 'object')
  return `Node::Map(&[${Object.entries(value).filter(([,v]) => typeof v !== 'function').map(([key,value]) => `(${JSON.stringify(key)},${node(value)})`).join(',')}])`
}
export function legacyThemeData() {
  const source = '// Generated immutable Tailwind CSS 4.1.13 default-theme data.\n// Functions remain namespace dependencies evaluated by the Rust resolver.\nuse super::Node;\npub(super) const DEFAULTS: &[(&str, Node, Option<&str>)] = &[\n' + Object.entries(expressions).map(([key, {base, from}]) => `(${JSON.stringify(key)},${node(base)},${from ? `Some(${JSON.stringify(from)})` : 'None'}),`).join('\n') + '\n];\n'
  return execFileSync('rustfmt', ['--edition', '2021', '--emit', 'stdout'], {input: source, encoding: 'utf8'})
}
function expanded(key, seen = new Set()) {
  assert.ok(!seen.has(key), key)
  seen.add(key)
  const {base, from} = expressions[key]
  return from && expressions[from] ? {...base, ...expanded(from, seen)} : base
}
function leafPaths(value, parts, target) {
  target.add(parts.join('.').replaceAll('.[', '['))
  if (Array.isArray(value)) value.forEach((value, i) => leafPaths(value, [...parts, String(i)], target))
  else if (value && typeof value === 'object') for (const [key, child] of Object.entries(value)) {
    if (typeof child === 'function') continue
    leafPaths(child, [...parts, key.includes('.') || key.includes('/') ? `[${key}]` : key], target)
  }
}
export async function legacyThemeFixtures() {
  const paths = new Set()
  for (const key of Object.keys(expressions)) leafPaths(expanded(key), [key], paths)
  for (const key of ['colors', 'accentColor', 'backgroundColor', 'borderColor', 'boxShadowColor', 'caretColor', 'divideColor', 'fill', 'gradientColorStops', 'outlineColor', 'placeholderColor', 'ringColor', 'ringOffsetColor', 'stroke', 'textColor', 'textDecorationColor']) for (const [name, value] of Object.entries(oracle('tailwindcss/colors'))) leafPaths(value, [key, name], paths)
  for (const path of ['fontSize.xs.1.lineHeight', 'fontSize.xs.0', 'fontSize.xs.1', 'fontFamily.sans.0', 'fontFamily.sans.1', 'spacing[0.5]', 'spacing.0.5', 'spacing.0_5', 'colors.blue.500', 'colors.blue-500', 'colors[blue-500]', 'colors.brand.card', 'colors.brand-card', 'colors.brand.500', 'colors.brand', 'colors.brand.DEFAULT', 'borderColor.DEFAULT', 'ringColor.DEFAULT', 'transitionDuration.DEFAULT', 'transitionTimingFunction.DEFAULT', 'unknown.x', 'unknown', 'container', 'container.x', 'fontSize.card', 'fontSize.card.1.fontWeight', 'fontSize.card[1].lineHeight', 'fontFamily.card', 'fontFamily.card.1.fontFeatureSettings', 'fontFamily.card.1.fontVariationSettings', 'custom.camelCase', 'custom[1.5]', 'custom.1.5', 'custom.card', 'custom.card.1.lineHeight', 'custom.card.DEFAULT', 'custom.DEFAULT', 'spacing.unknown', 'padding.card', 'margin.card', 'size.card', 'width.card', 'maxWidth.card', 'textColor.brand', 'divideColor.brand', 'ringColor.brand', 'backdropBlur.card', 'backdropBrightness.card', 'divideWidth.card', 'ringOpacity.card']) paths.add(path)
  const custom = '--color-brand: rgb(1 2 3 / <alpha-value>); --color-brand-card: #123456; --color-brand-500: #abcdef; --spacing-card: 13px; --spacing-0_5: 17px; --spacing-auto: 19px; --text-card: 23px; --text-card--line-height: 29px; --text-card--font-weight: 700; --text-xs: 31px; --text-xs--line-height: 37px; --font-card: Card, serif; --font-card--font-feature-settings: "cv11"; --font-card--font-variation-settings: "wght" 700; --custom-camel-case: CAMEL; --custom-1_5: DECIMAL; --custom-card: CARD; --custom-card--line-height: EXTRA; --custom: DEFAULT; --blur-card: 41px; --brightness-card: 43%; --border-width-card: 47px; --opacity-card: .53; --default-transition-duration: 59ms; --default-transition-timing-function: ease-in;'
  const cssDefaults = await readFile(resolve(directory, '../../../ref/tailwindcss/packages/tailwindcss/theme.css'), 'utf8')
  const themes = {empty: '', custom: `@theme { ${custom} }`, default: `@theme default { ${custom} }`, inline: `@theme inline { ${custom} }`, reference: `@theme reference { ${custom} }`, prefix: `@theme prefix(tw) { ${custom} }`, cssDefaults, reset: `${cssDefaults} @theme { --*: initial; }`}
  const namespaceMap = {animation:'animate', aspectRatio:'aspect', borderRadius:'radius', boxShadow:'shadow', colors:'color', containers:'container', fontFamily:'font', fontSize:'text', letterSpacing:'tracking', lineHeight:'leading', maxWidth:'container', screens:'breakpoint', transitionTimingFunction:'ease'}
  const kebab = value => value.replaceAll('.', '_').replace(/([a-z])([A-Z])/g, (_, a, b) => `${a}-${b.toLowerCase()}`)
  const collisionPaths = new Set()
  const collisionTokens = []
  for (const namespace of Object.keys(expressions)) {
    if(namespace === 'container') continue
    const values = expanded(namespace)
    let keys = Object.keys(values).filter(key => /^[a-zA-Z0-9_.%-]+$/.test(key) && key !== '__BARE_VALUE__').slice(0, 3)
    if(keys.length === 0) keys = ['card']
    for(const key of keys) {
      const name = `${kebab(namespaceMap[namespace] ?? namespace)}${key === 'DEFAULT' ? '' : `-${kebab(key)}`}`
      const value = `CUSTOM_${collisionTokens.length}`
      collisionTokens.push(`--${name}: ${value};`)
      collisionPaths.add(`${namespace}${key.includes('.') ? `[${key}]` : `.${key}`}`)
    }
    collisionPaths.add(`${namespace}.card`)
  }
  Object.assign(themes, {collision:`@theme { ${collisionTokens.join(' ')} }`, collisionDefault:`@theme default { ${collisionTokens.join(' ')} }`})
  const cases = []
  const all = [...paths]
  const collision = [...collisionPaths]
  for (const [mode, theme] of Object.entries(themes)) for (let start = 0; start < (mode.startsWith('collision') ? collision : all).length; start += 70) {
    const batch = (mode.startsWith('collision') ? collision : all).slice(start, start + 70)
    const input = `${theme}\n${batch.map((path, i) => `.p${i} { value: theme(${JSON.stringify(path)}, FALLBACK); }`).join('\n')}\n@tailwind utilities;`
    const compiled = await compile(input)
    cases.push({id: `legacy-theme:${mode}:${start}`, input, candidates: [], paths: batch, ...(mode === 'cssDefaults' ? {limits: {cache_bytes: 4194304}} : {}), expectedCss: compiled.build([]).replace(/^\/\*! tailwindcss[^\n]*\n/, '')})
  }
  for (const value of ['theme(colors.brand / .5)', 'theme(colors.brand, red)', 'theme("colors.brand" / 50%)', 'theme(spacing[0.5])', 'theme(fontSize.xs.1.lineHeight)', 'theme(unknown)', 'theme(fontSize)', 'theme(fontSize.xs.1)', 'theme(transitionDuration.DEFAULT)']) for (const mode of ['empty','custom','default']) {
    const input = `${themes[mode]} .probe { value: ${value}; } @tailwind utilities;`
    try { const compiled = await compile(input); cases.push({id:`legacy-theme:expression:${mode}:${value}`, input, candidates:[], expectedCss:compiled.build([]).replace(/^\/\*! tailwindcss[^\n]*\n/, '')}) }
    catch(error) { cases.push({id:`legacy-theme:expression:${mode}:${value}`,input,candidates:[],expectedError:error.message}) }
  }
  for(const path of ['colors.brand', 'colors.brand-card', 'padding.card', 'margin.card', 'fontSize.card', 'fontSize.card.1.lineHeight', 'spacing[0.5]', 'custom.camelCase', 'backdropBlur.card', 'transitionDuration.DEFAULT', 'unknown']) for(const mode of ['custom','default','reference','prefix']) {
    const candidates = [`${mode === 'prefix' ? 'tw:' : ''}[value:theme(${path},FALLBACK)]`]
    const input = `${themes[mode]} @tailwind utilities;`
    const compiled = await compile(input)
    cases.push({id:`legacy-theme:candidate:${mode}:${path}`, input, candidates, expectedCss:compiled.build(candidates).replace(/^\/\*! tailwindcss[^\n]*\n/, '')})
  }
  const boundaryPaths = ['colors[brand]500', 'colors[brand].500', 'colors[brand][500]', 'colors[brand].[500]', 'colors..brand', 'fontSize.card[0]', 'fontSize.card[1][lineHeight]', 'fontSize.card.1.unknown', 'fontSize.card.2', 'fontSize.xs.01', 'fontSize.xs.0.0', 'fontSize.xs.0.99', 'fontFamily.sans.0.0', 'fontFamily.sans.7', 'fontFamily.sans.99', 'spacing.4.0', 'spacing.4.99', 'spacing.4.01', 'spacing.4.length', 'padding[a.b]', 'padding.a.b', 'padding[a b]', 'padding[a_b]', 'margin[a.b]', 'width[a.b]', 'colors[quoted"key]', 'colors[back\\slash]', 'textColor[quoted"key]', 'textColor[back\\slash]', 'fontFamily.card.DEFAULT', 'fontSize.card.DEFAULT', 'blur.DEFAULT', 'borderColor.DEFAULT', 'ringColor.DEFAULT', 'color.DEFAULT', 'colors.DEFAULT', 'custom.DEFAULT']
  const escapedTokens = '--spacing-a\\.b: 61px; --spacing-a\\ b: 67px; --spacing-a_b: 71px; --color-quoted\\"key: red; --color-back\\\\slash: blue; --color-brand-500: #123456; --blur: 73px; --color: black;'
  for(const mode of ['', 'default', 'prefix(tw)', 'default prefix(tw)']) {
    const theme = `@theme ${mode} { ${custom} ${escapedTokens} }`
    const input = `${theme} ${boundaryPaths.map((path,i)=>`.boundary${i} { value: theme(${JSON.stringify(path)}, FALLBACK); }`).join(' ')} @tailwind utilities;`
    const compiled = await compile(input)
    cases.push({id:`legacy-theme:boundary:${mode || 'plain'}`, input, candidates:[], paths:boundaryPaths, expectedCss:compiled.build([]).replace(/^\/\*! tailwindcss[^\n]*\n/, '')})
  }
  for(const mode of ['prefix(tw)', 'default prefix(tw)']) {
    const input = `@theme ${mode} { ${collisionTokens.join(' ')} } ${collision.map((path,i)=>`.collision${i} { value: theme(${JSON.stringify(path)}, FALLBACK); }`).join(' ')} @tailwind utilities;`
    const compiled = await compile(input)
    cases.push({id:`legacy-theme:prefix-collision:${mode}`, input, candidates:[], expectedCss:compiled.build([]).replace(/^\/\*! tailwindcss[^\n]*\n/, '')})
  }
  for(const key of ['--spacing-\\a', '--spacing-\\a b', '--spacing-\\31 card', '--spacing-\\000031card']) {
    const input = `@theme { ${key}: 79px; } .p { value: theme(padding.a, FALLBACK); } .q { value: theme(padding[1card], FALLBACK); } @tailwind utilities;`
    const compiled = await compile(input)
    cases.push({id:`legacy-theme:escape-codepoint:${key}`, input, candidates:[], expectedCss:compiled.build([]).replace(/^\/\*! tailwindcss[^\n]*\n/, '')})
  }
  for(const key of ['--spacing-\\110000', '--spacing-\\ffffff', '--spacing-\\\\110000', '--custom-\\\\110000']) for(const prefix of ['', 'prefix(tw)']) {
    const input = `@theme ${prefix} { ${key}: 83px; } .p { value: theme(${key.startsWith('--custom') ? 'custom' : 'padding'}.card, FALLBACK); } @tailwind utilities;`
    try { const compiled = await compile(input); cases.push({id:`legacy-theme:invalid-codepoint:${prefix}:${key}`,input,candidates:[],expectedCss:compiled.build([]).replace(/^\/\*! tailwindcss[^\n]*\n/, '')}) }
    catch(error) { cases.push({id:`legacy-theme:invalid-codepoint:${prefix}:${key}`,input,candidates:[],expectedError:error.message,expectedErrorName:error.name}) }
  }
  for(const input of ['.p { value: var(--foo-\\110000); } @tailwind utilities;', '@theme { --spacing: 4px; } .p { value: var(--foo-\\110000); } @tailwind utilities;', '@theme { --spacing-card: 漢字; } .p { value: theme(padding.card.0); } .q { value: theme(padding.card.1); } @tailwind utilities;', '@theme { --spacing-\\d7ff: 89px; --spacing-\\e000: 97px; } .p { value: theme(padding.card,FALLBACK); } @tailwind utilities;']) {
    try { const compiled = await compile(input); cases.push({id:`legacy-theme:tracking-boundary:${cases.length}`,input,candidates:[],expectedCss:compiled.build([]).replace(/^\/\*! tailwindcss[^\n]*\n/, '')}) }
    catch(error) { cases.push({id:`legacy-theme:tracking-boundary:${cases.length}`,input,candidates:[],expectedError:error.message,expectedErrorName:error.name}) }
  }
  const sourcePaths = ['src/compat/default-theme.ts', 'src/compat/colors.ts', 'src/compat/plugin-functions.ts', 'src/compat/apply-config-to-theme.ts', 'src/compat/apply-compat-hooks.ts', 'src/compat/config/create-compat-config.ts', 'src/utils/to-key-path.ts', 'theme.css']
  const sources = []
  for(const path of sourcePaths) sources.push({path:`packages/tailwindcss/${path}`, sha256:createHash('sha256').update(await readFile(resolve(directory, '../../../ref/tailwindcss/packages/tailwindcss',path))).digest('hex')})
  return {schemaVersion:1, sourceCommit:'1334c99db8fd26c8ea065375dd9259800863f072', oracleVersion:'4.1.13', defaultNamespaces:Object.keys(expressions).length, dynamicNamespaces:Object.values(expressions).filter(value=>value.from).length, paths:all.length, collisionPaths:collision.length, sources,cases}
}
export async function legacyThemeBoundaries() {
  const inputs = [
    '@theme static { --custom-\\d800: 101px; } @tailwind utilities;',
    '@theme static { --custom-\\dc00: 103px; } @tailwind utilities;',
    '@theme { --spacing-card: 😀; } .p { value: theme(padding.card.0); } @tailwind utilities;',
    '@theme { --spacing-card: 😀; } .p { value: theme(padding.card.1); } @tailwind utilities;',
  ]
  const cases = []
  for(const [index,input] of inputs.entries()) {
    const compiled = await compile(input)
    const css = compiled.build([]).replace(/^\/\*! tailwindcss[^\n]*\n/, '')
    cases.push({id:`legacy-theme:utf16:${index}`,input,candidates:[],status:'frozen-utf16-output',expectedCssUtf16Units:Array.from({length:css.length},(_,i)=>css.charCodeAt(i))})
  }
  return {schemaVersion:1,sourceCommit:'1334c99db8fd26c8ea065375dd9259800863f072',oracleVersion:'4.1.13',coverageStatus:'frozen',reason:'Official outputs containing non-scalar UTF-16 code units are compared by their original units in native and WASM tests; broader UTF-16 input coverage is tracked separately.',cases}
}
if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  const data = legacyThemeData()
  const fixtures = await legacyThemeFixtures()
  const boundaries = await legacyThemeBoundaries()
  const boundaryPath = resolve(directory, 'legacy-theme-boundaries.json')
  const dataPath = resolve(directory, '../../../crates/rustify-style/upstream/tailwind/legacy-theme-data.rs')
  if(process.argv.includes('--write')) { await writeFile(dataPath,data); await writeFile(resolve(directory,'legacy-theme-fixtures.json'), JSON.stringify(fixtures,null,2)+'\n'); await writeFile(boundaryPath,JSON.stringify(boundaries,null,2)+'\n') }
  else { assert.equal(data,await readFile(dataPath,'utf8')); assert.deepEqual(fixtures,JSON.parse(await readFile(resolve(directory,'legacy-theme-fixtures.json'),'utf8'))); assert.deepEqual(boundaries,JSON.parse(await readFile(boundaryPath,'utf8'))) }
  console.log(JSON.stringify({namespaces:fixtures.defaultNamespaces,dynamic:fixtures.dynamicNamespaces,paths:fixtures.paths,cases:fixtures.cases.length,errors:fixtures.cases.filter(item=>item.expectedError).length}))
}
