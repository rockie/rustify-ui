import assert from 'node:assert/strict'
import {createHash} from 'node:crypto'
import {readFile} from 'node:fs/promises'
import {createRequire} from 'node:module'
import {resolve} from 'node:path'
import {pathToFileURL} from 'node:url'

const oracle = createRequire(resolve('tests/utility/compiler/oracle/package.json'))
const tailwind = oracle('tailwindcss')
assert.equal(oracle('tailwindcss/package.json').version, '4.1.13')
const provenance = JSON.parse(await readFile('crates/rustify-style/upstream/tailwind/PROVENANCE.json','utf8'))
assert.equal(provenance.commit, '1334c99db8fd26c8ea065375dd9259800863f072')
for (const path of ['candidate.ts','design-system.ts','compile.ts','variants.ts','utilities.ts']) {
  const source = `packages/tailwindcss/src/${path}`
  assert.equal(createHash('sha256').update(await readFile(resolve('ref/tailwindcss',source))).digest('hex'),
    provenance.sourceDigests[source], source)
}
const directory = resolve('target/utility-conformance/compiler-api')
const facade = await import(pathToFileURL(resolve(directory,'style.js')))
const bytes = await readFile(resolve(directory,'style_bg.wasm'))
const exports = await facade.default({module_or_path:bytes})
assert.equal(exports.memory.buffer instanceof SharedArrayBuffer,false)
const limits = {input_bytes:262144,asset_bytes:262144,candidate_count:1024,ast_depth:32,
  css_bytes:262144,queued_bytes:262144,cache_bytes:16000000}
const config = JSON.stringify({limits})
const snapshot = value => JSON.parse(JSON.stringify(value, (_, item) =>
  typeof item === 'bigint' ? {bigint:item.toString()} : item === undefined ? {undefined:true} : item))
let candidates = 0, variants = 0, identities = 0, astBuilds = 0
const samples = ['\ud800','\udc00','😀','\ud800\ud801','\udc00\ud800','�','\\ud800']
for (const sample of samples) {
  const input = `@theme inline { --color-card${sample}: red; } @tailwind utilities;`
  const expected = await tailwind.__unstable__loadDesignSystem(input)
  const actual = new facade.DesignSystem(input,config)
  const css = new facade.CssCompiler(input,config)
  const expectedCss = await tailwind.compile(input)
  try {
    const raws = [`bg-card${sample}`,`content-['${sample}']`,`[--probe:${sample}]`,
      `[&.${sample}]:block`,`group-hover/${sample}:block`,`data-[${sample}]:block`]
    for (const raw of raws) {
      const parsed = actual.parseCandidate(raw), reference = expected.parseCandidate(raw)
      assert.deepEqual(snapshot(parsed),snapshot(reference),raw)
      assert.equal(actual.parseCandidate(raw),parsed)
      identities++
      for (let index=0; index<parsed.length; index++) {
        assert.equal(actual.printCandidate(parsed[index]),expected.printCandidate(reference[index]),raw)
        assert.deepEqual(snapshot(actual.compileAstNodes(parsed[index],0)),snapshot(expected.compileAstNodes(reference[index],0)),raw)
      }
      candidates++
    }
    assert.equal(css.build(raws),expectedCss.build(raws))
    for (const raw of [`[&.${sample}]`,`group-hover/${sample}`,`data-[${sample}]`]) {
      const parsed = actual.parseVariant(raw), reference = expected.parseVariant(raw)
      assert.deepEqual(snapshot(parsed),snapshot(reference),raw)
      assert.equal(actual.parseVariant(raw),parsed)
      if (parsed !== null) assert.equal(actual.printVariant(parsed),expected.printVariant(reference))
      variants++; identities++
    }
    const mutable = actual.parseCandidate('[--probe:old]')[0]
    const mutableReference = expected.parseCandidate('[--probe:old]')[0]
    mutable.value = mutableReference.value = sample
    assert.equal(actual.printCandidate(mutable),expected.printCandidate(mutableReference))
    assert.deepEqual(snapshot(actual.compileAstNodes(mutable,0)),snapshot(expected.compileAstNodes(mutableReference,0)))
    candidates++
    const mutableVariant = actual.parseVariant('[&.old]')
    const mutableVariantReference = expected.parseVariant('[&.old]')
    mutableVariant.selector = mutableVariantReference.selector = `&.${sample}`
    assert.equal(actual.printVariant(mutableVariant),expected.printVariant(mutableVariantReference))
    variants++
    const hover = actual.parseVariant('hover')
    assert.equal(actual.parseCandidate('hover:block')[0].variants[0],hover)
    assert.equal(actual.parseVariant('group-hover').variant,hover)
    assert.ok(actual.getVariantOrder().has(hover))
    identities += 3
  } finally { actual.free(); css.free() }
  const inputAst = [{kind:'at-rule',name:'@tailwind',params:'utilities',nodes:[]}]
  const ast = new facade.AstCompiler(inputAst,config)
  const referenceAst = await tailwind.compileAst([{kind:'at-rule',name:'@tailwind',params:'utilities',nodes:[]}])
  try {
    const batch = [`[--probe:${sample}]`,`[&.${sample}]:block`]
    assert.deepEqual(snapshot(ast.build(batch)),snapshot(referenceAst.build(batch)))
    const previous = ast.build(batch)
    assert.equal(ast.build([]),previous)
    astBuilds++; identities++
  } finally { ast.free() }
}
const distinct = new facade.DesignSystem('',config)
try {
  const values = ['\ud800','�','\\ud800'].map(value => distinct.parseCandidate(`content-['${value}']`))
  assert.equal(new Set(values).size,3)
  for (let index=0; index<values.length; index++) {
    assert.equal(values[index][0].value.value,`'${['\ud800','�','\\ud800'][index]}'`)
  }
  identities++
} finally { distinct.free() }
console.log(JSON.stringify({candidates,variants,identities,astBuilds,actualWasm:true,
  sharedMemory:false,wasmBytes:bytes.length,fullPublicValueConformance:'pending'}))
