import assert from 'node:assert/strict'
import { analyzeSource, generateLedger } from './upstream-test-ledger.mjs'

const analyze = source => analyzeSource('synthetic.test.ts', source)
let checks = 0
function check(name, run) { run(); checks++; console.log(`ok ${name}`) }

check('each builder is not a registration; suite and test rows form a product', () => {
  const result = analyze(`describe.each(['Unix', 'Windows'])('line %s', line => {
    const table = [['a', 1], ['b', 2]] as const
    it.each(table)('%s', (input, expected) => { expect(input.length).toBe(expected) })
  })`)
  assert.equal(result.calls.filter(call => call.shape === 'parameter_builder').length, 2)
  assert(result.calls.filter(call => call.shape === 'parameter_builder').every(call => call.registrationIds.length === 1))
  assert.equal(result.calls.find(call => call.base === 'describe' && call.shape === 'registration').descendantTestIds.length, 4)
  assert.equal(result.instances.length, 4)
  assert(result.instances.every(test => test.registrationExpansion === 'static_expanded'))
  assert.deepEqual(result.instances.map(test => test.parameterBindings), [{ input: 'a', expected: 1 }, { input: 'b', expected: 2 }, { input: 'a', expected: 1 }, { input: 'b', expected: 2 }])
})

check('literal let tables and prefix loops expand without executing callbacks', () => {
  const result = analyze(`let table = ['a', 'b']; it.each(table)('%s', value => { throw new Error('never execute') });
    const prefixes = ['border', 'border-x']; for (let prefix of prefixes) { test(\`\${prefix}-*\`, () => { expect(prefix).toBe('border') }) }`)
  assert.equal(result.instances.length, 4)
  assert.deepEqual(result.instances.slice(2).map(test => test.name), ['border-*', 'border-x-*'])
})

check('unknown table and registration loop remain unresolved and retain source calls', () => {
  const result = analyze(`test.each(loadTable())('%s', value => { expect(value).toBe(1) });
    for (const prefix of loadPrefixes()) { test(prefix, () => { expect(prefix).toBe('x') }) }`)
  assert.equal(result.calls.length, 3)
  assert.equal(result.instances.length, 2)
  assert(result.instances.every(test => test.registrationExpansion === 'unresolved'))
  assert.equal(result.unresolved.length, 1)
})

check('skip and only modifiers propagate from suites', () => {
  const result = analyze(`describe.skip('suite', () => { test('a', () => { expect(1).toBe(1) }) });
    describe.only('suite', () => { test.skip('b', () => {}); test('c', () => {}) })`)
  assert.deepEqual(result.instances.map(test => test.upstreamMode), ['skip', 'skip', 'only'])
})

check('array mutations invalidate static tables', () => {
  for (const mutation of ['table.push(2)', 'table[0] = 2', 'mutate(table)', 'table = [2]', 'const output = mutate(table)', 'const alias = table; alias.push(2)', 'mutateGlobalState()']) {
    const result = analyze(`let table = [1]; ${mutation}; test.each(table)('%s', value => {})`)
    assert.equal(result.instances[0].registrationExpansion, 'unresolved', mutation)
  }
})

check('assert and assertion-count controls are retained; unowned helpers are visible', () => {
  const result = analyze(`function helper() { expect(1).toBe(1) }
    test('guards', () => { assert(flag); expect.hasAssertions(); expect.assertions(2) })`)
  assert.deepEqual(result.calls[0].assertionTemplates.map(assertion => assertion.kind), ['assert', 'assertion_count', 'assertion_count'])
  assert.equal(result.unownedAssertions.length, 1)
  assert.equal(result.unownedAssertions[0].validation, 'pending')
})

check('suite parameters do not inherit shadowed literal declarations', () => {
  const result = analyze(`const table = [1]; describe('suite', table => { test.each(table)('%s', value => {}) })`)
  assert.equal(result.instances[0].registrationExpansion, 'unresolved')
})

check('unsupported for argument semantics and control flow are not guessed', () => {
  const result = analyze(`test.for([[1, 2]])('%s', row => {});
    if (flag) test('conditional', () => {});`)
  assert.equal(result.instances.length, 2)
  assert(result.instances.every(test => test.registrationExpansion === 'unresolved'))
})

check('assertion locators preserve runtime multiplicity uncertainty', () => {
  const result = analyze(`test('assertions', () => {
    expect(1).not.toBe(2)
    for (const value of values()) expect(value).toEqual(1)
    function helper() { expect(2).toBe(2) }
    if (flag) expect('css').toMatchInlineSnapshot('css')
  })`)
  const assertions = result.calls[0].assertionTemplates
  assert.equal(assertions.length, 4)
  assert.deepEqual(assertions.map(assertion => assertion.controls.length), [0, 1, 1, 1])
  assert(assertions.every(assertion => assertion.validation === 'pending' && assertion.occurrenceStatus === 'pending_runtime'))
})

const ledger = await generateLedger()
check('fixed source edges, compare, parser and snapshots remain complete and pending', () => {
  assert.equal(ledger.summary.originalSourceCalls, 1215)
  assert.equal(ledger.summary.originalTestCalls, 1125)
  assert.equal(ledger.summary.matchedExternalSnapshotExports, 13)
  assert.equal(ledger.summary.validatedAssertions, 0)
  assert.equal(new Set(ledger.expandedTests.map(test => test.id)).size, ledger.expandedTests.length)
  assert.equal(ledger.expandedTests.filter(test => test.sourceTestId.includes('/utils/compare.test.ts:8:')).length, 12)
  assert.equal(ledger.expandedTests.filter(test => test.sourceTestId.includes('/css-parser.test.ts:')).length, 136)
  assert(ledger.sourceCalls.every(call => call.status === 'pending' && call.scopeReview === 'pending'))
})
console.log(JSON.stringify({ checks, summary: ledger.summary }))
