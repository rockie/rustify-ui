import assert from 'node:assert/strict'
import {readFile, writeFile} from 'node:fs/promises'
import {createRequire} from 'node:module'
import {dirname, resolve} from 'node:path'
import {fileURLToPath} from 'node:url'
const directory = dirname(fileURLToPath(import.meta.url))
const oracle = createRequire(resolve(directory, 'oracle/package.json'))
const {compile} = oracle('tailwindcss')
assert.equal(oracle('tailwindcss/package.json').version, '4.1.13')
export async function themeFixtures() {
  const cases = []
  async function add(id, input, candidates = []) {
    const compiled = await compile(input)
    cases.push({id: `theme:${id}`, input, candidates, expectedCss: compiled.build(candidates).replace(/^\/\*! tailwindcss[^\n]*\n/, '')})
  }
  const keyframes = '@keyframes spin { to { transform: rotate(360deg); } } @keyframes bounce { 0%,100% { transform: translateY(0); } 50% { transform: translateY(-10px); } }'
  for (const mode of ['', 'inline', 'reference', 'static', 'default']) {
    const theme = `@theme ${mode} { --color-brand: #123456; --color-alias: var(--color-brand); --animate-spin: spin 1s linear infinite; --animate-bounce: bounce 2s; ${keyframes} }`
    await add(`${mode}:unused`, `${theme} @tailwind utilities;`)
    await add(`${mode}:direct-css`, `${theme} .x { color: var(--color-alias); } @tailwind utilities;`)
    await add(`${mode}:arbitrary-css`, `${theme} @tailwind utilities;`, ['[color:var(--color-alias)]'])
    await add(`${mode}:literal-animation`, `${theme} .x { animation: spin 1s, bounce 2s; } @tailwind utilities;`)
    await add(`${mode}:variable-animation`, `${theme} .x { animation: var(--animate-spin); } @tailwind utilities;`)
    await add(`${mode}:nested-animation`, `@layer theme { ${theme} } @tailwind utilities;`, ['[animation:spin_1s]'])
  }
  await add('ordinary-keyframes', `@keyframes unused { to { opacity: 0; } } @theme { ${keyframes} } @tailwind utilities;`)
  await add('ordinary-animation-variable', `@theme { ${keyframes} } .x { --animate-custom: spin 1s; } @tailwind utilities;`)
  await add('keyframe-reference-variable', '@theme { --color-brand: #123456; @keyframes color { to { color: var(--color-brand); } } } @tailwind utilities;')
  await add('reference-keyframes', `@media reference { @theme { ${keyframes} } } .x { animation: spin 1s; } @tailwind utilities;`)
  await add('duplicate-keyframes', '@theme { @keyframes spin { to { opacity: 0; } } } @theme { @keyframes spin { to { opacity: 1; } } } .x { animation: spin 1s; } @tailwind utilities;')
  await add('unused-cycle', '@theme { --a: var(--b); --b: var(--a); --c: var(--a); --unused: red; } @tailwind utilities;')
  await add('used-cycle', '@theme { --a: var(--b); --b: var(--a); --c: var(--a); } .x { color: var(--c); } @tailwind utilities;')
  await add('wildcard-literal', '@theme { --font-a: serif; --font*: mono; } @theme { --font*: initial; } @tailwind utilities;', ['font-a'])
  await add('wildcard-literal-value', '@theme static { --font*: mono; } @tailwind utilities;')
  await add('inline-license', '@theme { --font-brand: IBM /*! custom */ Plex; } @tailwind utilities;', ['font-brand'])
  await add('root-license', '/*! custom */ @theme { --color-brand: red; } @tailwind utilities;', ['bg-brand'])
  const defaults = await readFile(resolve(directory, '../../../ref/tailwindcss/packages/tailwindcss/theme.css'), 'utf8')
  await add('full-default-unused', `${defaults} @tailwind utilities;`)
  await add('full-default-colors', `${defaults} @tailwind utilities;`, ['bg-red-500', 'text-blue-500', 'p-3'])
  return {schemaVersion: 1, sourceCommit: '1334c99db8fd26c8ea065375dd9259800863f072', oracleVersion: '4.1.13', cases}
}
if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  const actual = await themeFixtures()
  if (process.argv.includes('--write')) await writeFile(resolve(directory, 'theme-fixtures.json'), JSON.stringify(actual, null, 2)+'\n')
  else assert.deepEqual(actual, JSON.parse(await readFile(resolve(directory, 'theme-fixtures.json'), 'utf8')))
  console.log(JSON.stringify({cases: actual.cases.length}))
}
