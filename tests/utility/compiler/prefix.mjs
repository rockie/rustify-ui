import assert from 'node:assert/strict'
import {readFile, writeFile} from 'node:fs/promises'
import {createRequire} from 'node:module'
import {dirname, resolve} from 'node:path'
import {fileURLToPath} from 'node:url'
const directory = dirname(fileURLToPath(import.meta.url))
const oracle = createRequire(resolve(directory, 'oracle/package.json'))
const {compile} = oracle('tailwindcss')
assert.equal(oracle('tailwindcss/package.json').version, '4.1.13')
export async function prefixFixtures() {
  const cases = []
  async function add(id, input, candidates = []) {
    const compiled = await compile(input)
    cases.push({id: `prefix:${id}`, input, candidates, expectedCss: compiled.build(candidates).replace(/^\/\*! tailwindcss[^\n]*\n/, '')})
  }
  for (const mode of ['', 'inline', 'reference', 'static', 'default']) {
    const input = `@theme prefix(tw) ${mode} { --color-brand: #123456; --color-alias: var(--color-brand); --color-prefixed: var(--tw-color-brand); --spacing: 4px; --breakpoint-card: 700px; --font-brand: serif; --animate-spin: spin 1s linear; @keyframes spin { to { opacity: 0; } } } @tailwind utilities;`
    for (const candidates of [[], ['tw:bg-brand','tw:p-3','tw:font-brand'], ['bg-brand','hover:tw:bg-brand','tw:hover:bg-brand!'], ['tw:bg-alias','tw:bg-prefixed'], ['tw:card:w-[13px]','tw:[color:var(--tw-color-brand)]'], ['tw:[animation:var(--tw-animate-spin)]']]) await add(`${mode}:${cases.length}`, input, candidates)
  }
  await add('direct-used-prefixed', '@theme prefix(tw) { --color-brand: red; } .x { color: var(--tw-color-brand); } @tailwind utilities;')
  await add('direct-used-unprefixed', '@theme prefix(tw) { --color-brand: red; } .x { color: var(--color-brand); } @tailwind utilities;')
  await add('later-prefix', '@theme prefix(old) { --color-brand: red; } @theme prefix(new) { --color-other: blue; } @tailwind utilities;', ['old:bg-brand', 'new:bg-brand', 'new:bg-other'])
  await add('last-option', '@theme prefix(old) prefix(new) { --color-brand: red; } @tailwind utilities;', ['old:bg-brand', 'new:bg-brand'])
  await add('empty-prefix-does-not-reset', '@theme prefix(tw) { --color-brand: red; } @theme prefix() { --color-other: blue; } @tailwind utilities;', ['tw:bg-brand', 'tw:bg-other'])
  await add('last-valid-option-overrides-invalid', '@theme prefix(UP) prefix(tw) { --color-brand: red; } @tailwind utilities;', ['tw:bg-brand'])
  await add('last-empty-option-ignores-invalid', '@theme prefix(UP) prefix() { --color-brand: red; } @tailwind utilities;', ['bg-brand'])
  await add('escaped-variable', '@theme prefix(tw) { --size-1\\/2: 13px; } @tailwind utilities;', ['tw:w-1/2'])
  for (const prefix of ['UP', 'tw1', 'tw-prefix', 'é']) {
    const input = `@theme prefix(${prefix}) { --color-brand: red; } @tailwind utilities;`
    let expectedError
    try { await compile(input) } catch (error) { expectedError = error.message }
    assert.ok(expectedError)
    cases.push({id: `prefix:invalid:${prefix}`, input, candidates: [], expectedError})
  }
  return {schemaVersion: 1, sourceCommit: '1334c99db8fd26c8ea065375dd9259800863f072', oracleVersion: '4.1.13', cases}
}
if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  const actual = await prefixFixtures()
  if (process.argv.includes('--write')) await writeFile(resolve(directory, 'prefix-fixtures.json'), JSON.stringify(actual, null, 2)+'\n')
  else assert.deepEqual(actual, JSON.parse(await readFile(resolve(directory, 'prefix-fixtures.json'), 'utf8')))
  console.log(JSON.stringify({cases: actual.cases.length}))
}
