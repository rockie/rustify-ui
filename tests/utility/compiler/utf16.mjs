import assert from 'node:assert/strict'
import {createHash} from 'node:crypto'
import {readFile, writeFile} from 'node:fs/promises'
import {createRequire} from 'node:module'
import {dirname, resolve} from 'node:path'
import {fileURLToPath} from 'node:url'

const directory = dirname(fileURLToPath(import.meta.url))
const root = resolve(directory, '../../..')
const oracle = createRequire(resolve(directory, 'oracle/package.json'))
const {compile, __unstable__loadDesignSystem: loadDesignSystem} = oracle('tailwindcss')
const oracleVersion = oracle('tailwindcss/package.json').version
assert.equal(oracleVersion, '4.1.13')
const sourceCommit = '1334c99db8fd26c8ea065375dd9259800863f072'
const sourcePaths = [
  'packages/tailwindcss/src/css-parser.ts',
  'packages/tailwindcss/src/value-parser.ts',
  'packages/tailwindcss/src/theme.ts',
  'packages/tailwindcss/src/utils/escape.ts',
  'packages/tailwindcss/src/utils/variables.ts',
  'packages/tailwindcss/src/utils/to-key-path.ts',
  'packages/tailwindcss/src/compat/plugin-functions.ts',
  'packages/tailwindcss/src/compat/apply-config-to-theme.ts',
  'packages/tailwindcss/src/css-functions.ts',
  'packages/tailwindcss/src/index.ts',
]
const toUnits = text => Array.from({length: text.length}, (_, index) => text.charCodeAt(index))
function fromUnits(units) {
  let text = ''
  for (let start = 0; start < units.length; start += 8192) text += String.fromCharCode(...units.slice(start, start + 8192))
  return text
}
const high = String.fromCharCode(0xd800)
const low = String.fromCharCode(0xdc00)
const pair = String.fromCharCode(0xd83d, 0xde00)
const samples = [
  ['high', high], ['low', low], ['pair', pair],
  ['high-high', high + String.fromCharCode(0xd801)],
  ['low-low', low + String.fromCharCode(0xdc01)], ['low-high', low + high],
  ['replacement', '\ufffd'], ['private-use', '\ue000'], ['literal-escape', '\\ud800'],
]
const modes = [
  ['ordinary', ''], ['static', 'static'], ['reference', 'reference'],
  ['inline', 'inline'], ['default', 'default'], ['prefix', 'prefix(tw)'],
  ['default-prefix', 'default prefix(tw)'],
]
const tailwind = '@tailwind utilities;'
// Escaping here only constructs authored input; all expected text comes from the oracle.
const authoredName = text => text.replaceAll('\\', '\\\\')

export async function utf16Fixtures() {
  const provenance = JSON.parse(await readFile(resolve(root, 'crates/rustify-style/upstream/tailwind/PROVENANCE.json'), 'utf8'))
  assert.equal(provenance.commit, sourceCommit)
  assert.equal(provenance.version, oracleVersion)
  const sources = await Promise.all(sourcePaths.map(async path => ({
    path,
    sha256: createHash('sha256').update(await readFile(resolve(root, 'ref/tailwindcss', path))).digest('hex'),
  })))
  for (const source of sources) if (provenance.sourceDigests[source.path]) assert.equal(source.sha256, provenance.sourceDigests[source.path], source.path)
  const cases = []
  async function add(layer, name, input, candidates = [], {observeTheme = false, ...metadata} = {}) {
    const id = `utf16:${layer}:${name}`
    assert.ok(!cases.some(item => item.id === id), `duplicate case ${id}`)
    const item = {id, inputUtf16Units: toUnits(input), candidatesUtf16Units: candidates.map(toUnits), ...metadata}
    assert.equal(fromUnits(item.inputUtf16Units), input)
    assert.deepEqual(item.candidatesUtf16Units.map(fromUnits), candidates)
    // This is the actual JS JSON transport, independent of a Rust scalar-only Value.
    const transported = JSON.parse(JSON.stringify({input, candidates}))
    assert.equal(transported.input, input)
    assert.deepEqual(transported.candidates, candidates)
    let phase = 'compile'
    try {
      const compiled = await compile(input)
      phase = 'build'
      const css = compiled.build(candidates).replace(/^\/\*! tailwindcss[^\n]*\n/, '')
      item.expectedCssUtf16Units = toUnits(css)
      assert.equal(JSON.parse(JSON.stringify(structuredClone({css}))).css, css)
    } catch (error) {
      item.expectedError = {phase, nameUtf16Units: toUnits(error.name), messageUtf16Units: toUnits(error.message)}
    }
    if (observeTheme && !item.expectedError) {
      // These are raw public Theme.entries() observations, not inferred GPU bindings.
      const designSystem = await loadDesignSystem(input)
      const entries = [...designSystem.theme.entries()].map(([key, value]) => [key, value.value])
      const transportedMap = JSON.parse(JSON.stringify(Object.fromEntries(entries)))
      for (const [key, value] of entries) assert.equal(transportedMap[key], value)
      item.expectedThemeEntries = entries.map(([key, value]) => ({keyUtf16Units: toUnits(key), valueUtf16Units: toUnits(value)}))
    }
    cases.push(item)
    return item
  }

  const boundaries = JSON.parse(await readFile(resolve(directory, 'legacy-theme-boundaries.json'), 'utf8'))
  for (const boundary of boundaries.cases) {
    const item = await add('boundary', boundary.id, boundary.input, boundary.candidates, {legacyBoundaryId: boundary.id})
    assert.deepEqual(item.expectedCssUtf16Units, boundary.expectedCssUtf16Units)
  }

  for (const [name, text] of samples) {
    for (const [context, input] of [
      ['selector', `.${text} { color: red; } ${tailwind}`],
      ['property', `.p { custom-${text}: 101px; } ${tailwind}`],
      ['value', `.p { value: ${text}; } ${tailwind}`],
      ['quoted', `.p { content: "${text}"; value: url('${text}'); } ${tailwind}`],
      ['function', `.p { value: outer(${text}, inner(${text})); } ${tailwind}`],
      ['custom-value', `.p { --custom: { ${text}; nested: [${text}]; }; } ${tailwind}`],
      ['license-comment', `/*! license ${text} */ .p { color: red; } ${tailwind}`],
      ['ordinary-comment', `/* ${text} */ .p { value: a/* ${text} */b; } ${tailwind}`],
      ['custom-comment', `.p { --custom: a/* ${text} */b; } ${tailwind}`],
      ['at-rule', `@media (custom: ${text}) { .p { value: ${text}; } } ${tailwind}`],
    ]) await add('authored', `${context}:${name}`, input)
  }
  await add('authored', 'bom-crlf-high', `\ufeff.${high}\r\n{\r\n  value:\t${low};\r\n}\r\n${tailwind}`)
  await add('authored', 'escaped-literal-unit', `.p { value: \\${high}; content: "\\${low}"; } ${tailwind}`)

  for (const [mode, options] of modes) for (const [name, text] of samples) {
    const token = authoredName(text)
    const prefix = options.includes('prefix') ? 'tw:' : ''
    const theme = `@theme ${options} { --spacing-${token}: 23px; --color-${token}: #123456; --custom-${token}: ${text}; --spacing-card: ${text}; }`
    await add('theme', `${mode}:${name}`, `${theme} .p { direct: --theme(--custom-${text}); primitive: theme(padding.card); } ${tailwind}`, [
      `${prefix}p-${text}`, `${prefix}bg-${text}`, `${prefix}[value:--theme(--custom-${text})]`,
    ], {observeTheme: true})
  }

  const escapes = [
    ['high', '\\d800'], ['high-upper', '\\D800'], ['high-six-digit', '\\00d800'],
    ['high-space', '\\d800 '], ['high-tab', '\\d800\t'], ['high-newline', '\\d800\n'],
    ['high-cr', '\\d800\r'], ['high-form-feed', '\\d800\f'], ['low', '\\dc00'],
    ['pair', '\\d83d\\de00'], ['pair-spaces', '\\d83d \\de00 '],
    ['astral', '\\1f600'], ['largest-scalar', '\\10ffff'],
    ['single-hex-no-space', '\\a'], ['single-hex-space', '\\a '],
    ['single-hex-before-suffix', '\\ax'], ['single-hex-space-before-suffix', '\\a x'],
    ['zero', '\\0 '], ['surrogate-followed-hex-letter', '\\d800a'],
    ['invalid-codepoint', '\\110000'], ['invalid-largest', '\\ffffff'],
  ]
  for (const [name, escaped] of escapes) {
    await add('escape', `theme-key:${name}`, `@theme static { --custom-${escaped}: 101px; } ${tailwind}`, [], {observeTheme: true})
    await add('escape', `used-variable:${name}`, `@theme { --color-${escaped}: red; } .p { color: var(--color-${escaped}); } ${tailwind}`, [], {observeTheme: true})
  }
  await add('escape', 'authored-key-stays-escaped', `.p { --custom-\\d800: ${high}; value: var(--custom-\\d800); } ${tailwind}`)
  await add('escape', 'used-variable-invalid-without-theme', `.p { value: var(--custom-\\110000); } ${tailwind}`)
  await add('escape', 'external-candidate-invalid', `@theme { --custom-card: red; } ${tailwind}`, ['--custom-\\110000'])

  for (const [mode, options] of modes) {
    const input = `@theme ${options} { --custom-\\d800: 101px; --custom-\ufffd: 103px; --custom-\ue000: 107px; --custom-\\\\ud800: 109px; --custom-\\D800: 113px; --custom-${low}: 127px; } .p { h: --theme(--custom-${high}); l: --theme(--custom-${low}); r: --theme(--custom-\ufffd); p: --theme(--custom-\ue000); e: --theme(--custom-\\ud800); } ${tailwind}`
    const item = await add('map', `collision:${mode}`, input, [], {observeTheme: true})
    assert.equal(item.expectedThemeEntries.length, 5)
    const prefix = options.includes('prefix') ? '--tw-custom-' : '--custom-'
    const entries = item.expectedThemeEntries.map(({keyUtf16Units, valueUtf16Units}) => [fromUnits(keyUtf16Units), fromUnits(valueUtf16Units)])
    assert.equal(new Set(entries.map(([key]) => key)).size, 5)
    assert.equal(new Map(entries).get(prefix + high), '113px')
  }
  await add('map', 'literal-and-escaped-pair-alias', `@theme static { --custom-\\d83d\\de00: 101px; --custom-${pair}: 103px; } ${tailwind}`, [], {observeTheme: true})
  await add('map', 'decoded-low-high-remain-two-units', `@theme static { --custom-\\dc00\\d800: 101px; --custom-${low + high}: 103px; } ${tailwind}`, [], {observeTheme: true})
  await add('map', 'reset-and-default-identity', `@theme { --spacing-${high}: 101px; --spacing-\ufffd: 103px; } @theme default { --spacing-${high}: 107px; } @theme { --spacing-\ufffd: initial; } ${tailwind}`, [`p-${high}`, 'p-\ufffd'], {observeTheme: true})

  const paths = ['0', '1', '2', '3', '99', '01', 'length', '0.0', '1.0', 'DEFAULT']
  for (const [name, text] of samples) for (const path of paths) {
    await add('primitive', `${name}:${path}`, `@theme { --spacing-card: ${text}; } .p { value: theme(padding.card.${path}, FALLBACK); } ${tailwind}`)
  }
  for (const [mode, options] of modes) for (const index of ['0', '1']) {
    await add('primitive', `${mode}:pair:${index}`, `@theme ${options} { --spacing-card: ${pair}; } .p { value: theme(padding.card[${index}]); } ${tailwind}`)
  }
  await add('primitive', 'tuple-base-high', `@theme { --text-card: ${pair}; --text-card--line-height: ${low}; } .p { a: theme(fontSize.card.0.0); b: theme(fontSize.card.1.lineHeight.0); } ${tailwind}`)
  await add('primitive', 'font-string-index', `@theme { --font-card: ${pair}; } .p { value: theme(fontFamily.card.0); } ${tailwind}`)
  for (const [name, text] of samples) {
    const token = authoredName(text)
    for (const [style, path] of [['bracket', `padding[${text}]`], ['dot', `padding.${text}`], ['quoted', `"padding[${text}]"`]]) {
      await add('path', `${style}:${name}`, `@theme { --spacing-${token}: 101px; } .p { value: theme(${path}, FALLBACK); } ${tailwind}`)
    }
  }
  await add('path', 'quoted-css-escape-stays-literal', '@theme { --spacing-\\d800: 101px; } .p { value: theme("padding[\\d800]", FALLBACK); } @tailwind utilities;')
  await add('path', 'fallback-high', `.p { value: theme(missing, ${high}); } ${tailwind}`)
  await add('path', 'alpha-high', `@theme { --color-card: ${high}; } .p { value: theme(colors.card / .5); } ${tailwind}`)
  await add('path', 'at-rule-primitive-low', `@theme { --spacing-card: ${pair}; } @media (value: theme(padding.card.1)) { .p { display: block; } } ${tailwind}`)

  for (const [name, text] of samples) {
    for (const [context, candidates] of [
      ['arbitrary-value', [`[value:${text}]`]],
      ['arbitrary-property', [`[custom-${text}:101px]`]],
      ['arbitrary-quoted', [`[content:'${text}']`]],
      ['arbitrary-function', [`[value:outer(${text})]`]],
      ['arbitrary-selector', [`[&_${text}]:block`]],
      ['arbitrary-at-rule', [`[@media(custom:${text})]:block`]],
      ['arbitrary-variable', [`[value:var(--custom-${text})]`]],
    ]) await add('candidate', `${context}:${name}`, tailwind, candidates)
    await add('candidate', `primitive-index:${name}`, `@theme { --spacing-card: ${text}; } ${tailwind}`, ['[value:theme(padding.card.0)]', '[value:theme(padding.card.1)]'])
  }
  await add('candidate', 'source-utf16-order', tailwind, [`[value:${pair}]`, '[value:\ue000]', `[value:${high}]`, `[value:${low}]`])
  await add('candidate', 'source-number-order', tailwind, [`[custom-${high}10:1]`, `[custom-${high}2:1]`, `[custom-${high}02:1]`])
  await add('candidate', 'variant-theme-key', `@theme { --breakpoint-${high}: 700px; } ${tailwind}`, [`${high}:block`, `min-${high}:block`])

  for (const [name, text] of samples.slice(0, 6)) {
    for (const [context, input] of [
      ['missing-theme', `.p { value: theme(padding[${text}].missing); } ${tailwind}`],
      ['missing-css-theme', `.p { value: --theme(--missing-${text}); } ${tailwind}`],
      ['invalid-prefix', `@theme prefix(${text}) { --spacing-card: 101px; } ${tailwind}`],
      ['invalid-namespace', `@theme { --custom-${text}-*: ${text}; } ${tailwind}`],
      ['invalid-theme-child', `@theme { property-${text}: ${text}; } ${tailwind}`],
      ['missing-closing-rule', `.${text} { color: red;`],
      ['invalid-declaration', `.p { property-${text}; } ${tailwind}`],
      ['unterminated-string', `.p { content: "${text};\n } ${tailwind}`],
      ['alpha-message', `.p { value: --alpha(${text} /); } ${tailwind}`],
    ]) await add('error', `${context}:${name}`, input)
    await add('error', `candidate-missing-theme:${name}`, tailwind, [`[value:theme(padding[${text}].missing)]`])
  }
  assert.equal(cases.filter(item => item.legacyBoundaryId).length, 4)
  return {
    schemaVersion: 1, sourceCommit, oracleVersion, sources,
    purpose: 'Official compiler UTF-16 oracle matrix. Compiler migration acceptance remains tracked by the implementation gates.',
    unitsFormat: 'Exact UTF-16 code units. Theme observations are raw Theme.entries(), not GPU variable bindings.',
    cases,
  }
}

function summary(fixtures) {
  const layers = {}
  const phases = {}
  for (const item of fixtures.cases) {
    const layer = item.id.split(':')[1]
    layers[layer] = (layers[layer] ?? 0) + 1
    if (item.expectedError) phases[item.expectedError.phase] = (phases[item.expectedError.phase] ?? 0) + 1
  }
  return {cases: fixtures.cases.length, cssCases: fixtures.cases.filter(item => !item.expectedError).length, errors: phases, sourceFiles: fixtures.sources.length, themeObservationCases: fixtures.cases.filter(item => item.expectedThemeEntries).length, layers}
}
if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  const actual = await utf16Fixtures()
  const path = resolve(directory, 'utf16-fixtures.json')
  if (process.argv.includes('--write')) await writeFile(path, JSON.stringify(actual, null, 2) + '\n')
  else assert.deepEqual(actual, JSON.parse(await readFile(path, 'utf8')))
  console.log(JSON.stringify(summary(actual)))
}
