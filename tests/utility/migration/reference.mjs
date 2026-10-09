import assert from 'node:assert/strict'
import { readFile, writeFile } from 'node:fs/promises'
import { load, assertLockedSource, commit, json } from '../cn/upstream.mjs'

assertLockedSource()
const { cn } = await load('index')
const historical = JSON.parse(await readFile(new URL('./fixtures.json', import.meta.url), 'utf8'))
const output = {
  oracleCommit: commit,
  historicalOutputPolicy: 'The fixed cn oracle decides merge semantics. Preserve old captures as historical evidence.',
  cases: historical.cases.map(item => {
    const expected = cn(...item.rawFragments, item.callerClass)
    const oldTokens = new Set(item.oldOutput.split(' '))
    const newTokens = new Set(expected.split(' '))
    return {
      id: item.id, expected,
      changed: expected !== item.oldOutput,
      added: [...newTokens].filter(value => !oldTokens.has(value)),
      removed: [...oldTokens].filter(value => !newTokens.has(value)),
      classification: expected === item.oldOutput ? 'unchanged' : 'fixed-cn-upstream-semantics',
    }
  }),
}
const path = new URL('./cn-expected.json', import.meta.url)
if (process.argv.includes('--write')) await writeFile(path, json(output))
else assert.equal(await readFile(path, 'utf8'), json(output))
console.log(JSON.stringify({ cases: output.cases.length, changed: output.cases.filter(item => item.changed).length }))
