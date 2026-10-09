import assert from 'node:assert/strict'
import { createHash } from 'node:crypto'
import { readFile, writeFile } from 'node:fs/promises'
import { createRequire } from 'node:module'
import { dirname, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'

const directory = dirname(fileURLToPath(import.meta.url))
const repository = resolve(directory, '../../..')
const require = createRequire(resolve(directory, 'oracle/package.json'))
const ts = require('typescript')
const hash = value => createHash('sha256').update(value).digest('hex')
const unknown = reason => ({ known: false, reason })
const known = value => ({ known: true, value })

// This evaluates a deliberately small syntax grammar, never upstream code.
function evaluate(node, environment) {
  if (!node) return unknown('missing expression')
  if (ts.isParenthesizedExpression(node) || ts.isAsExpression(node) || ts.isTypeAssertionExpression(node) || ts.isSatisfiesExpression(node)) return evaluate(node.expression, environment)
  if (ts.isStringLiteralLike(node)) return known(node.text)
  if (ts.isNumericLiteral(node)) return known(Number(node.text))
  if (node.kind === ts.SyntaxKind.TrueKeyword) return known(true)
  if (node.kind === ts.SyntaxKind.FalseKeyword) return known(false)
  if (node.kind === ts.SyntaxKind.NullKeyword) return known(null)
  if (ts.isIdentifier(node)) return environment.get(node.text) ?? unknown(`unbound identifier: ${node.text}`)
  if (ts.isPropertyAccessExpression(node)) return environment.get(node.getText()) ?? unknown(`unresolved property: ${node.getText()}`)
  if (ts.isArrayLiteralExpression(node)) {
    const values = []
    for (const element of node.elements) {
      const value = evaluate(ts.isSpreadElement(element) ? element.expression : element, environment)
      if (!value.known) return value
      if (ts.isSpreadElement(element)) {
        if (!Array.isArray(value.value)) return unknown('non-array spread')
        values.push(...value.value)
      } else values.push(value.value)
    }
    return known(values)
  }
  if (ts.isPrefixUnaryExpression(node)) {
    const operand = evaluate(node.operand, environment)
    if (!operand.known || typeof operand.value !== 'number') return unknown('non-numeric unary operand')
    if (node.operator === ts.SyntaxKind.MinusToken) return known(-operand.value)
    if (node.operator === ts.SyntaxKind.PlusToken) return operand
  }
  if (ts.isBinaryExpression(node)) {
    const left = evaluate(node.left, environment), right = evaluate(node.right, environment)
    if (!left.known || !right.known) return unknown('unresolved binary operand')
    if (typeof left.value === 'number' && typeof right.value === 'number') {
      if (node.operatorToken.kind === ts.SyntaxKind.LessThanLessThanToken) return known(left.value << right.value)
      if (node.operatorToken.kind === ts.SyntaxKind.BarToken) return known(left.value | right.value)
    }
    if (node.operatorToken.kind === ts.SyntaxKind.PlusToken && ['string', 'number'].includes(typeof left.value) && ['string', 'number'].includes(typeof right.value)) return known(left.value + right.value)
  }
  if (ts.isTemplateExpression(node)) {
    let value = node.head.text
    for (const span of node.templateSpans) {
      const part = evaluate(span.expression, environment)
      if (!part.known || !['string', 'number', 'boolean'].includes(typeof part.value)) return unknown('unresolved template substitution')
      value += String(part.value) + span.literal.text
    }
    return known(value)
  }
  return unknown(`unsupported syntax: ${ts.SyntaxKind[node.kind]}`)
}

function callShape(node) {
  if (!ts.isCallExpression(node)) return null
  let expression = node.expression
  const modifiers = []
  while (ts.isCallExpression(expression) || ts.isPropertyAccessExpression(expression)) {
    if (ts.isPropertyAccessExpression(expression)) modifiers.unshift(expression.name.text)
    expression = expression.expression
  }
  if (!ts.isIdentifier(expression) || !['test', 'it', 'describe'].includes(expression.text)) return null
  const builder = ts.isPropertyAccessExpression(node.expression) && ['each', 'for'].includes(node.expression.name.text)
  const parameterCall = ts.isCallExpression(node.expression) && ts.isPropertyAccessExpression(node.expression.expression) && ['each', 'for'].includes(node.expression.expression.name.text) ? node.expression : null
  return { base: expression.text, modifiers, kind: builder ? 'parameter_builder' : 'registration', parameterCall }
}

function bind(pattern, value, environment) {
  if (ts.isIdentifier(pattern)) { environment.set(pattern.text, known(value)); return true }
  if (ts.isArrayBindingPattern(pattern) && Array.isArray(value)) {
    return pattern.elements.every((part, i) => ts.isOmittedExpression(part) || (!part.dotDotDotToken && !part.initializer && i < value.length && bind(part.name, value[i], environment)))
  }
  return false
}

function assertionKind(node) {
  if (!ts.isCallExpression(node)) return null
  if (ts.isIdentifier(node.expression) && node.expression.text === 'assert') return { kind: 'assert', matcher: 'assert' }
  if (!ts.isPropertyAccessExpression(node.expression)) return null
  const matcher = node.expression.name.text
  let root = node.expression.expression
  while (ts.isCallExpression(root) || ts.isPropertyAccessExpression(root)) root = root.expression
  if (!ts.isIdentifier(root) || root.text !== 'expect') return null
  if (/^to[A-Z]/.test(matcher)) return { kind: 'matcher', matcher }
  if (['hasAssertions', 'assertions'].includes(matcher)) return { kind: 'assertion_count', matcher }
  return null
}

export function analyzeSource(path, content, importedValues = new Map()) {
  const ast = ts.createSourceFile(path, content, ts.ScriptTarget.Latest, true)
  const locator = node => ({ path, line: ast.getLineAndCharacterOfPosition(node.getStart(ast)).line + 1, start: node.getStart(ast), end: node.end, sha256: hash(node.getText(ast)) })
  const id = node => `${path}:${locator(node).line}:${node.getStart(ast)}-${node.end}`
  const calls = [], byNode = new Map(), instances = [], unresolved = [], allAssertions = new Map()
  function collect(node) {
    const assertion = assertionKind(node)
    if (assertion) allAssertions.set(id(node), { id: id(node), ...locator(node), ...assertion, argumentCount: node.arguments.length, occurrenceStatus: 'pending_runtime', fixture: null, validation: 'pending' })
    const shape = callShape(node)
    if (shape) {
      const entry = { id: id(node), ...locator(node), call: node.expression.getText(ast), shape: shape.kind, base: shape.base, modifiers: shape.modifiers, parameterBuilderId: shape.parameterCall ? id(shape.parameterCall) : null, assertionTemplates: [], instances: [] }
      calls.push(entry); byNode.set(node, entry)
      if (shape.kind === 'registration' && shape.base !== 'describe') {
        const callback = node.arguments.find(argument => ts.isArrowFunction(argument) || ts.isFunctionExpression(argument))
        if (callback) {
          function assertions(child, controls = []) {
            if (child !== callback && (ts.isFunctionDeclaration(child) || ts.isArrowFunction(child) || ts.isFunctionExpression(child))) controls = [...controls, { kind: 'callback_or_helper', ...locator(child) }]
            if (ts.isIterationStatement(child, false) || ts.isIfStatement(child) || ts.isConditionalExpression(child)) controls = [...controls, { kind: ts.SyntaxKind[child.kind], ...locator(child) }]
            const assertion = assertionKind(child)
            if (assertion) {
              entry.assertionTemplates.push({ id: id(child), ...locator(child), ...assertion, argumentCount: child.arguments.length, controls, occurrenceStatus: 'pending_runtime', fixture: null, validation: 'pending', ...(assertion.matcher.includes('InlineSnapshot') ? { inlineSnapshotArguments: child.arguments.map(argument => ({ ...locator(argument), bytes: Buffer.byteLength(argument.getText(ast)) })) } : {}) })
            }
            ts.forEachChild(child, nested => assertions(nested, controls))
          }
          assertions(callback)
        }
      }
    }
    ts.forEachChild(node, collect)
  }
  collect(ast)
  function invalidateArrayBindings(node, environment) {
    let runtimeCall = false
    function visit(child) {
      if (ts.isArrowFunction(child) || ts.isFunctionExpression(child) || ts.isFunctionDeclaration(child)) return
      if (ts.isCallExpression(child) || ts.isTaggedTemplateExpression(child)) runtimeCall = true
      ts.forEachChild(child, visit)
    }
    visit(node)
    if (runtimeCall) for (const [name, result] of environment) if (result.known && Array.isArray(result.value)) environment.set(name, unknown(`array may be mutated by runtime call: ${name}`))
  }
  function recordUnresolved(node, reason, contexts) {
    unresolved.push({ ...locator(node), reason, contexts, status: 'unresolved' })
  }
  function walk(node, environment, contexts, suites, blocked = [], inheritedModifiers = []) {
    if (ts.isSourceFile(node) || ts.isBlock(node)) {
      const local = new Map(environment)
      for (const statement of node.statements) walk(statement, local, contexts, suites, blocked, inheritedModifiers)
      return
    }
    if (ts.isVariableStatement(node)) {
      for (const declaration of node.declarationList.declarations) {
        const result = evaluate(declaration.initializer, environment)
        if (declaration.initializer) invalidateArrayBindings(declaration.initializer, environment)
        if (ts.isIdentifier(declaration.name)) environment.set(declaration.name.text, result)
      }
      return
    }
    if (ts.isForOfStatement(node)) {
      const table = evaluate(node.expression, environment)
      const pattern = ts.isVariableDeclarationList(node.initializer) ? node.initializer.declarations[0]?.name : node.initializer
      if (!table.known || !Array.isArray(table.value)) {
        const reason = table.known ? 'for-of iterable is not a static array' : table.reason
        recordUnresolved(node, reason, contexts)
        walk(node.statement, new Map(environment), contexts, suites, [...blocked, reason], inheritedModifiers)
      } else {
        table.value.forEach((value, row) => {
          const local = new Map(environment)
          if (!pattern || !bind(pattern, value, local)) {
            recordUnresolved(node, 'unsupported for-of binding', contexts)
            walk(node.statement, local, contexts, suites, [...blocked, 'unsupported for-of binding'], inheritedModifiers)
          } else walk(node.statement, local, [...contexts, { kind: 'registration_loop', sourceId: id(node), row, value }], suites, blocked, inheritedModifiers)
        })
      }
      return
    }
    if (ts.isExpressionStatement(node)) {
      const shape = callShape(node.expression)
      if (shape?.kind === 'registration') {
        const call = node.expression, entry = byNode.get(call)
        const effectiveModifiers = [...inheritedModifiers, ...shape.modifiers]
        const callback = call.arguments.find(argument => ts.isArrowFunction(argument) || ts.isFunctionExpression(argument))
        const name = evaluate(call.arguments[0], environment)
        let rows = [null], rowFailure = null
        if (shape.parameterCall) {
          const table = evaluate(shape.parameterCall.arguments[0], environment)
          if (table.known && Array.isArray(table.value)) rows = table.value
          else { rowFailure = table.known ? 'parameter table is not a static array' : table.reason; rows = [null] }
        }
        rows.forEach((row, index) => {
          const local = new Map(environment), failures = [...blocked]
          if (shape.modifiers.includes('for')) failures.push('test.for argument semantics not implemented')
          const next = shape.parameterCall ? [...contexts, { kind: shape.base === 'describe' ? 'suite_parameter' : 'test_parameter', sourceId: entry.id, row: index, ...(rowFailure ? { unresolved: rowFailure } : { value: row }) }] : contexts
          if (rowFailure) failures.push(rowFailure)
          if (!name.known || typeof name.value !== 'string') failures.push(name.known ? 'test name is not a string' : name.reason)
          if (!callback) failures.push('missing callback')
          const bindings = {}
          if (shape.parameterCall && !rowFailure && callback) {
            const values = Array.isArray(row) ? row : [row]
            callback.parameters.forEach((parameter, i) => {
              if (i < values.length && bind(parameter.name, values[i], local)) {
                if (ts.isIdentifier(parameter.name)) bindings[parameter.name.text] = values[i]
              } else failures.push('unsupported or missing parameter binding')
            })
          }
          if (shape.base === 'describe') {
            if (callback) {
              // Suite parameters shadow outer declarations even when unresolved.
              if (!shape.parameterCall || rowFailure) for (const parameter of callback.parameters) if (ts.isIdentifier(parameter.name)) local.set(parameter.name.text, unknown('unresolved suite callback parameter'))
              walk(callback.body, local, next, [...suites, { sourceId: entry.id, name: name.known ? name.value : null }], failures, effectiveModifiers)
            }
          } else {
            const instance = { id: `${entry.id}#${hash(JSON.stringify(next)).slice(0, 16)}`, sourceTestId: entry.id, name: name.known ? name.value : null, suites, contexts: next, parameterBindings: bindings, registrationExpansion: failures.length ? 'unresolved' : 'static_expanded', unresolvedReasons: [...new Set(failures)], upstreamMode: effectiveModifiers.includes('skip') ? 'skip' : effectiveModifiers.includes('todo') ? 'todo' : effectiveModifiers.includes('only') ? 'only' : 'run', execution: 'pending', scopeReview: 'pending', assertionIds: entry.assertionTemplates.map(assertion => assertion.id), fixture: null }
            instances.push(instance); entry.instances.push(instance.id)
          }
        })
        return
      }
      // Runtime statements can mutate previously literal bindings. Do not execute them.
      const written = new Set()
      function baseIdentifier(child) {
        while (ts.isPropertyAccessExpression(child) || ts.isElementAccessExpression(child)) child = child.expression
        return ts.isIdentifier(child) ? child.text : null
      }
      function writes(child) {
        if (ts.isBinaryExpression(child) && child.operatorToken.kind >= ts.SyntaxKind.FirstAssignment && child.operatorToken.kind <= ts.SyntaxKind.LastAssignment) written.add(baseIdentifier(child.left))
        if (ts.isPostfixUnaryExpression(child) || ts.isPrefixUnaryExpression(child)) if ([ts.SyntaxKind.PlusPlusToken, ts.SyntaxKind.MinusMinusToken].includes(child.operator)) written.add(baseIdentifier(child.operand))
        if (ts.isCallExpression(child)) {
          if (ts.isPropertyAccessExpression(child.expression)) written.add(baseIdentifier(child.expression.expression))
          // Passing an array to an unknown function may mutate it.
          for (const argument of child.arguments) if (ts.isIdentifier(argument) && Array.isArray(environment.get(argument.text)?.value)) written.add(argument.text)
        }
        ts.forEachChild(child, writes)
      }
      writes(node)
      for (const name of written) if (name !== null && environment.has(name)) environment.set(name, unknown(`binding may have been mutated: ${name}`))
      invalidateArrayBindings(node, environment)
      return
    }
    if (ts.isFunctionDeclaration(node) || ts.isClassDeclaration(node) || ts.isImportDeclaration(node)) return
    if (ts.isIfStatement(node) || ts.isForStatement(node) || ts.isWhileStatement(node) || ts.isDoStatement(node) || ts.isSwitchStatement(node) || ts.isTryStatement(node)) {
      recordUnresolved(node, 'unsupported registration control flow', contexts)
      ts.forEachChild(node, child => walk(child, new Map(environment), contexts, suites, [...blocked, 'unsupported registration control flow'], inheritedModifiers))
    }
  }
  walk(ast, new Map(importedValues), [], [])
  for (const entry of calls) {
    if (entry.shape === 'parameter_builder') entry.registrationIds = calls.filter(call => call.parameterBuilderId === entry.id).map(call => call.id)
    if (entry.shape === 'registration' && entry.base === 'describe') entry.descendantTestIds = instances.filter(test => test.suites.some(suite => suite.sourceId === entry.id)).map(test => test.id)
    if (entry.shape === 'registration' && entry.base !== 'describe' && !entry.instances.length) {
      const instance = { id: `${entry.id}#unresolved`, sourceTestId: entry.id, name: null, suites: [], contexts: [], parameterBindings: {}, registrationExpansion: 'unresolved', unresolvedReasons: ['registration context not statically visited'], upstreamMode: entry.modifiers.includes('skip') ? 'skip' : 'run', execution: 'pending', scopeReview: 'pending', assertionIds: entry.assertionTemplates.map(assertion => assertion.id), fixture: null }
      instances.push(instance); entry.instances.push(instance.id)
    }
  }
  const ownedAssertions = new Set(calls.flatMap(call => call.assertionTemplates.map(assertion => assertion.id)))
  return { calls, instances, unresolved, unownedAssertions: [...allAssertions.values()].filter(assertion => !ownedAssertions.has(assertion.id)) }
}

function enumValues(ast) {
  const values = new Map()
  for (const statement of ast.statements) {
    if (!ts.isEnumDeclaration(statement)) continue
    let previous = -1
    for (const member of statement.members) {
      const result = member.initializer ? evaluate(member.initializer, values) : known(previous + 1)
      if (!result.known || typeof result.value !== 'number') break
      previous = result.value
      values.set(`${statement.name.text}.${member.name.getText(ast)}`, result)
    }
  }
  return values
}

export async function generateLedger(sourceRoot = resolve(repository, 'ref/tailwindcss')) {
  const inventoryBytes = await readFile(resolve(directory, 'inventory.json'))
  const inventory = JSON.parse(inventoryBytes)
  assert.equal(inventory.baseline.version, '4.1.13')
  assert.equal(inventory.baseline.commit, '1334c99db8fd26c8ea065375dd9259800863f072')
  assert.equal(hash(inventory.sources.map(source => `${source.path}\0${source.sha256}\n`).join('')), inventory.baseline.sourceDigest)
  const content = new Map(await Promise.all(inventory.sources.map(async source => {
    const bytes = await readFile(resolve(sourceRoot, source.path))
    assert.equal(hash(bytes), source.sha256, `upstream source changed: ${source.path}`)
    return [source.path, bytes.toString('utf8')]
  })))
  const sourceCalls = [], expandedTests = [], unresolvedContexts = [], snapshots = [], unownedAssertions = []
  for (const source of inventory.sources) {
    const text = content.get(source.path)
    if (source.path.endsWith('.snap')) {
      const ast = ts.createSourceFile(source.path, text, ts.ScriptTarget.Latest, true)
      for (const statement of ast.statements) {
        const assignment = ts.isExpressionStatement(statement) ? statement.expression : null
        if (!assignment || !ts.isBinaryExpression(assignment) || assignment.operatorToken.kind !== ts.SyntaxKind.EqualsToken || !ts.isElementAccessExpression(assignment.left) || assignment.left.expression.getText(ast) !== 'exports') continue
        const key = evaluate(assignment.left.argumentExpression, new Map()), value = evaluate(assignment.right, new Map())
        assert(key.known && typeof key.value === 'string' && value.known && typeof value.value === 'string', `unresolved snapshot export: ${source.path}`)
        snapshots.push({ id: `${source.path}#${hash(key.value)}`, path: source.path, kind: 'snapshot_export', sourceKind: 'snapshot', sourceSha256: source.sha256, line: ast.getLineAndCharacterOfPosition(assignment.getStart(ast)).line + 1, start: assignment.getStart(ast), end: assignment.end, sha256: hash(assignment.getText(ast)), key: key.value, valueSha256: hash(value.value), valueBytes: Buffer.byteLength(value.value), status: 'pending', matchedAssertions: [] })
      }
      continue
    }
    if (!/\.test\.[cm]?tsx?$/.test(source.path)) continue
    const ast = ts.createSourceFile(source.path, text, ts.ScriptTarget.Latest, true), imports = new Map()
    for (const statement of ast.statements) {
      if (!ts.isImportDeclaration(statement) || !ts.isStringLiteral(statement.moduleSpecifier) || !statement.moduleSpecifier.text.startsWith('.') || !statement.importClause?.namedBindings || !ts.isNamedImports(statement.importClause.namedBindings)) continue
      const importPath = resolve(dirname(resolve(sourceRoot, source.path)), `${statement.moduleSpecifier.text}.ts`)
      const relativePath = importPath.slice(resolve(sourceRoot).length + 1)
      if (!content.has(relativePath)) continue
      const enums = enumValues(ts.createSourceFile(relativePath, content.get(relativePath), ts.ScriptTarget.Latest, true))
      for (const item of statement.importClause.namedBindings.elements) {
        const imported = item.propertyName?.text ?? item.name.text
        for (const [name, value] of enums) if (name.startsWith(`${imported}.`)) imports.set(`${item.name.text}${name.slice(imported.length)}`, value)
      }
    }
    const analysis = analyzeSource(source.path, text, imports)
    sourceCalls.push(...analysis.calls); expandedTests.push(...analysis.instances); unresolvedContexts.push(...analysis.unresolved)
    unownedAssertions.push(...analysis.unownedAssertions)
  }
  const actualCalls = new Map(sourceCalls.map(call => [call.id, call]))
  assert.equal(actualCalls.size, sourceCalls.length, 'duplicate source call IDs')
  assert.deepEqual([...actualCalls.keys()].sort(), inventory.tests.map(test => test.id).sort(), 'ledger must retain every original source call')
  const originalCalls = inventory.tests.map(original => {
    const call = actualCalls.get(original.id)
    assert.equal(call.sha256, original.sha256)
    const parserOnly = original.path.endsWith('/css-parser.test.ts') && original.containsJsPluginOrConfig
    return { ...original, sourceSha256: inventory.sources.find(source => source.path === original.path).sha256, ...call, scopeReview: 'pending', suggestedScope: parserOnly ? 'in_scope' : null, scopeEvidence: parserOnly ? 'CSS parser assertion; @plugin text does not establish JavaScript execution' : null }
  })
  for (const call of originalCalls) if (call.parameterBuilderId) assert(actualCalls.get(call.parameterBuilderId)?.registrationIds.includes(call.id), 'parameter builder must link its registration')
  for (const instance of expandedTests) {
    const call = actualCalls.get(instance.sourceTestId)
    let counter = 0
    for (const assertion of call.assertionTemplates.filter(assertion => assertion.matcher === 'toMatchSnapshot')) {
      counter++
      const testFile = instance.sourceTestId.split(':')[0]
      const snapshotPath = `${dirname(testFile)}/__snapshots__/${testFile.slice(testFile.lastIndexOf('/') + 1)}.snap`
      const name = [...instance.suites.map(suite => suite.name), instance.name]
      if (name.some(part => part === null) || instance.registrationExpansion !== 'static_expanded' || instance.contexts.some(context => context.kind !== 'registration_loop')) continue
      // Named snapshots, property matchers and conditional calls need execution.
      if (assertion.argumentCount || assertion.controls.length) continue
      const key = `${name.join(' > ')} ${counter}`
      const snapshot = snapshots.find(snapshot => snapshot.path === snapshotPath && snapshot.key === key)
      if (snapshot) snapshot.matchedAssertions.push({ expandedTestId: instance.id, sourceAssertionId: assertion.id, matching: 'static_default_key_pending_runtime' })
    }
  }
  const testCalls = originalCalls.filter(call => call.role === 'test')
  const summary = {
    sourcesVerified: inventory.sources.length, originalSourceCalls: originalCalls.length, originalTestCalls: testCalls.length,
    originalParameterizationCalls: originalCalls.filter(call => call.role === 'parameterization').length,
    originalSuiteCalls: originalCalls.filter(call => call.role === 'suite').length,
    parameterBuilders: originalCalls.filter(call => call.shape === 'parameter_builder').length,
    expandedTests: expandedTests.length, staticallyExpandedTests: expandedTests.filter(test => test.registrationExpansion === 'static_expanded').length,
    unresolvedTests: expandedTests.filter(test => test.registrationExpansion === 'unresolved').length,
    upstreamSkippedTests: expandedTests.filter(test => test.upstreamMode === 'skip').length,
    assertionTemplates: originalCalls.reduce((count, call) => count + call.assertionTemplates.length, 0),
    assertionTemplateEdges: expandedTests.reduce((count, test) => count + test.assertionIds.length, 0),
    controlledAssertionTemplates: originalCalls.reduce((count, call) => count + call.assertionTemplates.filter(assertion => assertion.controls.length).length, 0),
    externalSnapshotExports: snapshots.length, matchedExternalSnapshotExports: snapshots.filter(snapshot => snapshot.matchedAssertions.length).length,
    suggestedScopeCorrections: originalCalls.filter(call => call.suggestedScope !== null).length,
    validatedTests: 0, validatedAssertions: 0, adjudicatedExclusions: 0,
  }
  assert.equal(summary.originalTestCalls, 1125)
  assert.equal(summary.externalSnapshotExports, 13)
  assert.equal(new Set(expandedTests.map(test => test.id)).size, expandedTests.length, 'duplicate expanded IDs')
  summary.unownedAssertionTemplates = unownedAssertions.length
  const compare = expandedTests.filter(test => test.sourceTestId.includes('/utils/compare.test.ts:8:'))
  const parser = expandedTests.filter(test => test.sourceTestId.includes('/css-parser.test.ts:'))
  assert.equal(compare.length, 12); assert(compare.every(test => test.registrationExpansion === 'static_expanded'))
  assert.equal(parser.length, 136); assert(parser.every(test => test.registrationExpansion === 'static_expanded'))
  return {
    schemaVersion: 1, baseline: { ...inventory.baseline, inventorySha256: hash(inventoryBytes) },
    meaning: 'Static source-call and assertion conversion ledger. Expansion is not upstream execution, oracle comparison, Rust conformance, or scope adjudication. Assertion occurrences and all validation remain pending.',
    status: 'pending', summary, sourceCalls: originalCalls, expandedTests, unresolvedContexts, unownedAssertionTemplates: unownedAssertions, externalSnapshots: snapshots,
  }
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  const args = process.argv.slice(2)
  assert(args.every(argument => ['--check', '--write'].includes(argument)) && args.length <= 1, 'usage: node tests/utility/compiler/upstream-test-ledger.mjs [--check|--write]')
  const ledger = await generateLedger()
  const output = `${JSON.stringify(ledger, null, 2)}\n`
  if (args.includes('--check')) assert.equal(await readFile(resolve(directory, 'upstream-test-ledger.json'), 'utf8'), output)
  else if (args.includes('--write')) await writeFile(resolve(directory, 'upstream-test-ledger.json'), output)
  console.log(JSON.stringify(ledger.summary))
}
