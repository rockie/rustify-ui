import assert from 'node:assert/strict'
import { readFile, writeFile } from 'node:fs/promises'
import { createRequire } from 'node:module'
import { dirname, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'
const directory = dirname(fileURLToPath(import.meta.url))
const oracle = createRequire(resolve(directory, 'oracle/package.json'))
const { compile } = oracle('tailwindcss')
assert.equal(oracle('tailwindcss/package.json').version, '4.1.13')
export async function externalVariableFixtures() {
  const cases = []
  const theme = '--color-brand: #123456; --color-alias: var(--color-brand); --font-a\\/b: serif; --animate-spin: spin 1s linear; @keyframes spin { to { opacity: 0; } }'
  for (const mode of ['', 'inline', 'reference', 'static', 'default', 'prefix(tw)', 'inline prefix(tw)']) {
    for (const suffix of ['@tailwind utilities;', '', '.x { color:var(--color-brand); } @tailwind utilities;']) {
      for (const variable of ['--color-brand', '--color-alias', '--animate-spin', '--missing', '--font-a\\/b', '--font-a/b', '--tw-color-brand', '--tw-color-alias', '--tw-font-a\\/b', '--tw-animate-spin']) {
        const input = `@theme ${mode} { ${theme} } ${suffix}`
        const batches = [[], [variable], [variable], ['flex', variable], ['--color-alias', '--tw-color-alias'], []]
        const compiler = await compile(input)
        cases.push({ id:`external:${mode}:${suffix}:${variable}`, input, batches,
          expectedBuilds:batches.map(batch => compiler.build(batch).replace(/^\/\*! tailwindcss[^\n]*\n/, '')) })
      }
    }
  }
  for (const input of [
    '@theme { --color-brand: red; } @source not inline("--color-brand"); @tailwind utilities;',
    '@theme { --color-brand: red; } @source inline("--color-brand"); @tailwind utilities;',
    '@theme { --a:var(--b); --b:var(--a); } @tailwind utilities;',
  ]) {
    const batches = [[], ['--color-brand', '--a'], [], ['flex']]
    const compiler = await compile(input)
    cases.push({id:`external:${input}`,input,batches,expectedBuilds:batches.map(batch => compiler.build(batch).replace(/^\/\*! tailwindcss[^\n]*\n/, ''))})
  }
  return { schemaVersion:1, sourceCommit:'1334c99db8fd26c8ea065375dd9259800863f072', oracleVersion:'4.1.13', cases }
}
if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  const actual = await externalVariableFixtures()
  if (process.argv.includes('--write')) await writeFile(resolve(directory, 'external-variables-fixtures.json'), JSON.stringify(actual,null,2)+'\n')
  else assert.deepEqual(actual,JSON.parse(await readFile(resolve(directory, 'external-variables-fixtures.json'),'utf8')))
  console.log(JSON.stringify({cases:actual.cases.length,builds:actual.cases.reduce((n,c)=>n+c.batches.length,0)}))
}
