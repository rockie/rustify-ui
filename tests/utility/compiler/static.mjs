import assert from 'node:assert/strict'
import { createHash } from 'node:crypto'
import { readFile, writeFile } from 'node:fs/promises'
import { createRequire } from 'node:module'
import { dirname, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'

const directory = dirname(fileURLToPath(import.meta.url))
const repository = resolve(directory, '../../..')
const oracle = createRequire(resolve(directory, 'oracle/package.json'))
const tailwind = oracle('tailwindcss')
assert.equal(oracle('tailwindcss/package.json').version, '4.1.13')
const sha256 = value => createHash('sha256').update(value).digest('hex')
const banner = css => css.replace(/^\/\*! tailwindcss[^\n]*\n/, '')
const theme = await readFile(resolve(repository, 'crates/rustify-style/upstream/tailwind/theme.css'), 'utf8')
const input = '@tailwind utilities;'
const probeTheme = '@theme { --default-transition-timing-function: linear; --default-transition-duration: 77ms; --breakpoint-z: 17px; --breakpoint-a: 42rem; --spacing: 37px; --default-ring-color: purple; --default-ring-width: 7px; --default-font-family: monospace; }'

export async function staticFixtures() {
  const designs = await Promise.all([input, theme, probeTheme].map(css => tailwind.__unstable__loadDesignSystem(css)))
  const roots = [...designs[0].utilities.keys('static')].sort()
  const inventory = JSON.parse(await readFile(resolve(directory, 'inventory.json'), 'utf8'))
  assert.deepEqual(roots, inventory.utilities.filter(item => item.kind === 'static').map(item => item.name).sort())
  const cases = []
  const nodes = []
  const interned = new Map()
  const templates = []
  const rustString = value => { let hashes = '#'; while (value.includes(`"${hashes}`)) hashes += '#'; return `r${hashes}"${value}"${hashes}` }
  function intern(node) {
    const key = JSON.stringify(node)
    if (interned.has(key)) return interned.get(key)
    const children = node.nodes?.map(intern)
    let expression
    if (node.kind === 'declaration') expression = `StaticNode::Declaration(${rustString(node.property)}, ${rustString(node.value)})`
    else if (node.kind === 'rule') expression = `StaticNode::Rule(${rustString(node.selector)}, &[${children}])`
    else if (node.kind === 'at-rule') expression = `StaticNode::AtRule(${rustString(node.name)}, ${rustString(node.params)}, &[${children}])`
    else throw new Error(`unexpected static AST ${node.kind}`)
    const id = nodes.length
    nodes.push(expression)
    interned.set(key, id)
    return id
  }
  function normalize(inputNodes, globals) {
    const output = []
    for (const node of inputNodes) {
      if (node.kind === 'at-root') { globals.push(...normalize(node.nodes, [])); continue }
      if (node.kind === 'declaration') {
        if (node.value === undefined || node.property === '--tw-sort') continue
        output.push({ kind: node.kind, property: node.property, value: node.value })
      } else if (node.kind === 'rule' || node.kind === 'at-rule') {
        const children = normalize(node.nodes, globals)
        if (node.kind === 'rule' && node.selector === '&') { output.push(...children); continue }
        output.push(node.kind === 'rule' ? { kind: node.kind, selector: node.selector, nodes: children } : { kind: node.kind, name: node.name, params: node.params, nodes: children })
      } else throw new Error(`unexpected static AST ${node.kind}`)
    }
    return output
  }
  for (const root of roots) {
    const compiled = designs.map(design => {
      const candidate = design.parseCandidate(root).find(item => item.kind === 'static')
      return design.compileAstNodes(candidate, 1)
    })
    assert.equal(compiled[0].length, 1, `${root} output count`)
    const themeDependent = compiled.some(value => JSON.stringify(value) !== JSON.stringify(compiled[0]))
    const compiler = await tailwind.compile(input)
    const expectedCss = banner(compiler.build([root]))
    assert.ok(expectedCss.length, root)
    const important = await tailwind.compile(input)
    const hover = await tailwind.compile(input)
    cases.push({ id: `static:${root}`, root, themeDependent, expectedCss, expectedCssSha256: sha256(expectedCss), expectedImportantCss: banner(important.build([`${root}!`])), expectedHoverCss: banner(hover.build([`hover:${root}`])) })
    if (!themeDependent) {
      const globals = []
      const local = normalize(compiled[0][0].node.nodes, globals)
      templates.push(`(${rustString(root)}, StaticTemplate { nodes: &[${local.map(intern)}], globals: &[${globals.map(intern)}], property_order: &[${compiled[0][0].propertySort.order}], declaration_count: ${compiled[0][0].propertySort.count} })`)
    }
  }
  const compiler = await tailwind.compile(input)
  const probeCases = []
  for (const {root} of cases.filter(item => item.themeDependent)) {
    const compiler = await tailwind.compile(`${probeTheme} ${input}`)
    probeCases.push({ id: `static-theme:${root}`, root, input: `${probeTheme} ${input}`, expectedCss: banner(compiler.build([root])) })
  }
  const fixture = { schemaVersion: 1, oracle: { package: 'tailwindcss', version: '4.1.13', sourceCommit: '1334c99db8fd26c8ea065375dd9259800863f072', normalization: 'remove generated license banner only', themeIndependenceProbes: ['empty', 'fixed default theme', probeTheme] }, input, cases, probeCases, union: { candidates: roots, expectedCss: banner(compiler.build(roots)) } }
  const rust = `// Generated by tests/utility/compiler/static.mjs from Tailwind CSS 4.1.13.\n// Fixed commit: ${fixture.oracle.sourceCommit}; MIT license in LICENSE.\n// Structural AST data only. Theme-dependent static handlers are implemented in Rust.\nuse crate::compiler::static_utilities::{StaticNode, StaticTemplate};\n#[rustfmt::skip]\npub const NODES: &[StaticNode] = &[\n${nodes.map(node => `    ${node},`).join('\n')}\n];\n#[rustfmt::skip]\npub const TEMPLATES: &[(&str, StaticTemplate)] = &[\n${templates.map(node => `    ${node},`).join('\n')}\n];\n`
  return { fixture, rust }
}

export async function checkStaticFixtures() {
  const actual = await staticFixtures()
  assert.deepEqual(actual.fixture, JSON.parse(await readFile(resolve(directory, 'static-fixtures.json'), 'utf8')))
  assert.equal(actual.rust, await readFile(resolve(repository, 'crates/rustify-style/upstream/tailwind/static-data.rs'), 'utf8'))
  return { staticRoots: actual.fixture.cases.length, structuralTemplates: actual.fixture.cases.filter(item => !item.themeDependent).length }
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  if (process.argv.includes('--write')) {
    const actual = await staticFixtures()
    await writeFile(resolve(directory, 'static-fixtures.json'), `${JSON.stringify(actual.fixture, null, 2)}\n`)
    await writeFile(resolve(repository, 'crates/rustify-style/upstream/tailwind/static-data.rs'), actual.rust)
    console.log(JSON.stringify({ staticRoots: actual.fixture.cases.length, structuralTemplates: actual.fixture.cases.filter(item => !item.themeDependent).length }))
  } else console.log(JSON.stringify(await checkStaticFixtures()))
}
