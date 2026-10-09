import assert from 'node:assert/strict'
import {readFile, writeFile} from 'node:fs/promises'
import {createRequire} from 'node:module'
import {dirname, resolve} from 'node:path'
import {fileURLToPath} from 'node:url'
const directory = dirname(fileURLToPath(import.meta.url))
const oracle = createRequire(resolve(directory, 'oracle/package.json'))
const {compile} = oracle('tailwindcss')
assert.equal(oracle('tailwindcss/package.json').version, '4.1.13')
export async function functionFixtures() {
  const cases = []
  async function add(id, input, candidates = []) {
    try {
      const compiled = await compile(input)
      cases.push({id: `functions:${id}`, input, candidates, expectedCss: compiled.build(candidates).replace(/^\/\*! tailwindcss[^\n]*\n/, '')})
    } catch (error) { cases.push({id: `functions:${id}`, input, candidates, expectedError: error.message}) }
  }
  const expressions = ['--alpha(#123456 / .5)', '--alpha(currentColor / 35%)', '--alpha(var(--color-brand) / var(--opacity-muted))', '--alpha(red / 1)', '--spacing(3)', '--spacing(-1)', '--spacing(calc(1 + 2))', '--theme(--color-brand)', '--theme(--color-brand inline)', '--theme(--color-brand / 50%)', '--theme(--missing, red, blue)', '--theme(--missing, initial)', '--theme(--color-brand, blue)', 'theme(--color-brand)', 'theme("--color-brand")', 'theme(--color-brand / 50%)', 'theme(--missing, red, blue)', 'calc(--spacing(3) + --spacing(2))', '--alpha(--theme(--color-brand) / .5)']
  for (const mode of ['', 'inline', 'reference', 'static', 'prefix(tw)']) {
    const theme = `@theme ${mode} { --color-brand: #123456; --spacing: 4px; --breakpoint-card: 700px; --opacity-muted: .35; }`
    for (let i = 0; i < expressions.length; i++) {
      const value = expressions[i]
      await add(`${mode}:authored:${i}`, `${theme} .probe { value: ${value}; } @tailwind utilities;`)
      await add(`${mode}:candidate:${i}`, `${theme} @tailwind utilities;`, [`${mode.startsWith('prefix') ? 'tw:' : ''}[value:${value.replaceAll(' ', '_')}]`])
    }
    await add(`${mode}:media`, `${theme} @media (width >= --theme(--breakpoint-card)) { .probe { color: --theme(--color-brand); } } @tailwind utilities;`)
    await add(`${mode}:supports`, `${theme} @supports (color: --theme(--color-brand)) { .probe { color: red; } } @tailwind utilities;`)
    await add(`${mode}:theme-dependency`, `@theme ${mode} { --color-brand: red; --color-alias: --theme(--color-brand); --spacing: 4px; --space-custom: --spacing(3); } @tailwind utilities;`, [`${mode.startsWith('prefix') ? 'tw:' : ''}bg-alias`])
  }
  for (const value of ['--alpha()', '--alpha(red)', '--alpha(red /)', '--alpha(red / .5, blue)', '--spacing()', '--spacing(3, 4)', '--theme(color)', '--theme(--missing)', 'theme(--missing)']) {
    await add(`invalid:${value}`, `@theme { --spacing: 4px; } .probe { value: ${value}; } @tailwind utilities;`)
    await add(`invalid-candidate:${value}`, '@theme { --spacing: 4px; } @tailwind utilities;', [`[value:${value.replaceAll(' ', '_')}]`])
  }
  await add('spacing-without-theme', '.probe { padding: --spacing(3); } @tailwind utilities;')
  for (const mode of ['', 'reference', 'inline']) await add(`${mode}:nested-fallback`, `@theme ${mode} { --font-alias: --theme(--font-missing, initial); } .probe { font-family: --theme(--font-alias, Potato Sans, sans-serif); } @tailwind utilities;`)
  for (const mode of ['', 'inline', 'reference', 'static', 'prefix(tw)']) {
    const prefix = mode.startsWith('prefix') ? 'tw:' : ''
    const theme = `@theme ${mode} { --spacing: 4px; --color-brand: #123456; --breakpoint-card: 700px; --animate-probe: probe 1s; @keyframes probe { from { margin-left: --spacing(2); color: --theme(--color-brand); } } }`
    await add(`${mode}:live-keyframes`, `${theme} @tailwind utilities;`, [`${prefix}animate-probe`])
    await add(`${mode}:unused-keyframes`, `${theme} @tailwind utilities;`)
    await add(`${mode}:nested-structural`, `${theme} @tailwind utilities;`, [`${prefix}divide-x-[--spacing(3)]`, `${prefix}drop-shadow-[0_2px_--spacing(3)_--theme(--color-brand)]`])
    await add(`${mode}:function-variant`, `${theme} @tailwind utilities;`, [`${prefix}supports-[color:--theme(--color-brand)]:block`, `${prefix}[@media(width>=--theme(--breakpoint-card))]:block`])
    await add(`${mode}:property-function`, `${theme} @property --custom { syntax: "*"; inherits: false; initial-value: --spacing(3); } @tailwind utilities;`)
    await add(`${mode}:custom-media-container`, `${theme} @custom-media --card (width > --theme(--breakpoint-card)); @container (width > --theme(--breakpoint-card)) { .probe { color: --theme(--color-brand); } } @tailwind utilities;`)
    await add(`${mode}:recursive-alias`, `@theme ${mode} { --color-brand: #123456; --color-alias: --theme(--color-brand); --color-nested: --theme(--color-alias); } @tailwind utilities;`, [`${prefix}bg-nested`])
  }
  await add('quoted-function-text', '.probe { content: "--theme(--missing)"; value: url("theme(--missing)"); } @tailwind utilities;')
  await add('invalid-unused-keyframes', '@theme { @keyframes probe { from { color: --theme(--missing); } } } @tailwind utilities;')
  await add('ignored-reference-functions', '@theme reference { --color-brand: --theme(--missing); } @tailwind utilities;')
  await add('authored-property-inherits', '@property --custom { syntax: "*"; inherits: true; initial-value: --alpha(red / .5); } @tailwind utilities;')
  await add('authored-property-duplicate', '@property --custom { syntax: "*"; inherits: false; initial-value: red; } @property --custom { syntax: "*"; inherits: true; initial-value: blue; } @tailwind utilities;')
  await add('nested-property-does-not-fallback', '@media (width > 100px) { @property --custom { syntax: "*"; inherits: false; initial-value: --alpha(red / .5); } } @tailwind utilities;')
  for (const input of ['.x{content:"theme(--foo)";}', '.x{--x:pretheme(--foo);}',
    '@theme reference { --foo:theme(--a,red); } .x{color:blue;}',
    '@theme reference { --spacing:--spacing(2); } .x{color:blue;}',
    '@custom-variant foo { color:theme(--x,red); @slot; }',
    '@theme { --foo:theme(--a,red); } .x{color:blue;}',
    '@theme { --foo: red; } .x{color:blue;}',
    '@media reference { .x { content:"theme(--foo)"; } }',
    '/*! theme(--foo) */ .x{color:blue;}']) {
    await add(`feature-fastpath:${input}`, input, ['flex'])
  }
  return {schemaVersion: 1, sourceCommit: '1334c99db8fd26c8ea065375dd9259800863f072', oracleVersion: '4.1.13', cases}
}
if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  const actual = await functionFixtures()
  if (process.argv.includes('--write')) await writeFile(resolve(directory, 'functions-fixtures.json'), JSON.stringify(actual, null, 2)+'\n')
  else assert.deepEqual(actual, JSON.parse(await readFile(resolve(directory, 'functions-fixtures.json'), 'utf8')))
  console.log(JSON.stringify({cases: actual.cases.length, errors: actual.cases.filter(item => item.expectedError).length}))
}
