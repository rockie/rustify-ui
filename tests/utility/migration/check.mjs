import assert from 'node:assert/strict'
import { createHash } from 'node:crypto'
import { readFile } from 'node:fs/promises'
import { dirname, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'

const directory = dirname(fileURLToPath(import.meta.url))
const sha256 = value => createHash('sha256').update(value).digest('hex')
const fixture = JSON.parse(await readFile(resolve(directory, 'fixtures.json'), 'utf8'))
assert.equal(fixture.baseline.captureStatus, 'captured')
assert.equal(fixture.cases.length, fixture.baseline.evidence.caseCount)
assert.equal(new Set(fixture.cases.map(item => item.id)).size, fixture.cases.length)
const output = fixture.cases.map(item => {
  assert.equal(typeof item.oldOutput, 'string')
  assert.ok(Array.isArray(item.rawFragments))
  assert.equal(typeof item.callerClass, 'string')
  return `${item.id}\t${JSON.stringify(item.oldOutput)}\n`
}).join('')
assert.equal(sha256(output), fixture.baseline.evidence.captureOutputSha256)
if (process.argv.includes('--check-capture-sources')) {
  for (const item of fixture.sources) {
    assert.equal(sha256(await readFile(resolve(directory, '../../..', item.path))), item.sha256, item.path)
  }
}
console.log(JSON.stringify({ capturedCases: fixture.cases.length, oldDependencyRequired: false, validation: 'historical-capture-integrity-only' }))
