import assert from 'node:assert/strict'
import {createHash} from 'node:crypto'
import {readFile, writeFile} from 'node:fs/promises'
import {createRequire} from 'node:module'
import {dirname, resolve} from 'node:path'
import {fileURLToPath} from 'node:url'

const directory = dirname(fileURLToPath(import.meta.url))
const root = resolve(directory, '../../..')
const require = createRequire(resolve(directory, 'oracle/package.json'))
const {compile} = require('tailwindcss')
assert.equal(require('tailwindcss/package.json').version, '4.1.13')
const units = text => Array.from({length:text.length}, (_, index) => text.charCodeAt(index))
const samples = [['high','\ud800'],['low','\udc00'],['pair','😀'],['high-high','\ud800\ud801'],
  ['low-low','\udc00\udc01'],['low-high','\udc00\ud800'],['replacement','�'],['literal-escape','\\ud800']]
const definitions = [
  ['colors', value => `--color-brand: ${value};`, ['bg-brand','bg-brand/50']],
  ['backgrounds', value => `--background-image-brand: url("${value}");`, ['bg-brand']],
  ['borders', value => `--divide-width-brand: ${value};`, ['divide-x-brand']],
  ['effects', value => `--radius-brand: ${value};`, ['rounded-brand']],
  ['typography-font', value => `--font-brand: "${value}";`, ['font-brand']],
  ['typography-size', value => `--text-brand: ${value}; --text-brand--line-height: ${value};`, ['text-brand']],
  ['functional', value => `--spacing: ${value}; --spacing-brand: ${value};`, ['p-2','-m-brand','size-brand']],
  ['grid', value => `--grid-template-columns-brand: ${value};`, ['grid-cols-brand']],
  ['layout', value => `--aspect-brand: ${value};`, ['aspect-brand']],
  ['motion', value => `--animate-brand: ${value} 1s; --ease-brand: ${value};`, ['animate-brand','ease-brand']],
  ['transforms', value => `--translate-brand: ${value}; --rotate-brand: ${value};`, ['translate-x-brand','-rotate-brand']],
  ['structural-spacing', value => `--spacing: ${value}; --spacing-brand: ${value};`, ['space-x-brand','-space-y-2']],
  ['shadows', value => `--shadow-brand: 0 1px ${value};`, ['shadow-brand','shadow-brand/50']],
  ['filters-blur', value => `--blur-brand: ${value};`, ['blur-brand','backdrop-blur-brand']],
  ['filters-shadow', value => `--drop-shadow-brand: 0 1px ${value};`, ['drop-shadow-brand','drop-shadow-brand/50']],
  ['gradients', value => `--color-brand: ${value};`, ['from-brand','via-brand/50','to-brand']],
  ['masks', value => `--color-brand: ${value};`, ['mask-x-from-brand/50']],
  ['miscellaneous', value => `--breakpoint-brand: ${value};`, ['max-w-screen-brand']],
  ['static-transition', value => `--default-transition-duration: ${value}; --default-transition-timing-function: ${value};`, ['transition']],
  ['static-container', value => `--breakpoint-brand: ${value};`, ['container']],
]
export async function familyFixtures() {
  const inventory = JSON.parse(await readFile(resolve(directory,'inventory.json'),'utf8'))
  const provenance = JSON.parse(await readFile(resolve(root,'crates/rustify-style/upstream/tailwind/PROVENANCE.json'),'utf8'))
  assert.equal(provenance.commit,'1334c99db8fd26c8ea065375dd9259800863f072')
  assert.equal(provenance.version,'4.1.13')
  const sourcePaths = ['packages/tailwindcss/src/utilities.ts','packages/tailwindcss/src/theme.ts',
    'packages/tailwindcss/src/utils/infer-data-type.ts','packages/tailwindcss/src/utils/replace-shadow-colors.ts',
    'packages/tailwindcss/src/utils/is-color.ts','packages/tailwindcss/src/utils/compare-breakpoints.ts']
  const sources = []
  for (const path of sourcePaths) {
    const sha256 = createHash('sha256').update(await readFile(resolve(root,'ref/tailwindcss',path))).digest('hex')
    const locked = inventory.sources.find(source => source.path === path)
    assert.equal(sha256,locked?.sha256,path)
    if (provenance.sourceDigests[path]) assert.equal(sha256,provenance.sourceDigests[path],path)
    sources.push({path,sha256})
  }
  const cases = []
  for (const [family, theme, candidates] of definitions) for (const mode of ['ordinary','inline','reference']) {
    for (const [sample, value] of samples) {
      const input = `@theme ${mode === 'ordinary' ? '' : mode} { ${theme(value)} } @tailwind utilities;`
      const compiler = await compile(input)
      const baseline = compiler.build([])
      // Each selected class must independently produce CSS; empty outputs cannot pass the fixture.
      for (const candidate of candidates) {
        const isolated = (await compile(input)).build([candidate])
        assert.notEqual(isolated, baseline, `${family}:${mode}:${sample}:${candidate}`)
        assert.ok(isolated.includes(value), `missing original units ${family}:${mode}:${sample}:${candidate}`)
      }
      const expected = compiler.build(candidates).replace(/^\/\*! tailwindcss[^\n]*\n/, '')
      cases.push({id:`family:${family}:${mode}:${sample}`,family,mode,sample,
        inputUtf16Units:units(input),candidates,expectedCssUtf16Units:units(expected)})
    }
  }
  // String.replace's replacement-string tokens are observable inside opaque CSS.
  for (const [family,namespace,candidate] of [
    ['shadows-replacement','shadow','shadow-brand/50'],
    ['filters-replacement','drop-shadow','drop-shadow-brand/50'],
  ]) for (const [sample,unit] of samples) for (const token of ['$$','$&','$`',"$'",'$1']) {
    const value = `x${unit}${token}y`
    const input = `@theme inline { --${namespace}-brand: 0 1px ${value}; } @tailwind utilities;`
    const compiler = await compile(input)
    const baseline = compiler.build([])
    const expected = compiler.build([candidate]).replace(/^\/\*! tailwindcss[^\n]*\n/, '')
    assert.notEqual(expected,baseline.replace(/^\/\*! tailwindcss[^\n]*\n/, ''),`${family}:${sample}:${token}`)
    cases.push({id:`family:${family}:${sample}:${token}`,family,mode:'inline',sample,
      inputUtf16Units:units(input),candidates:[candidate],expectedCssUtf16Units:units(expected)})
  }
  return {schemaVersion:1,sourceCommit:'1334c99db8fd26c8ea065375dd9259800863f072',oracleVersion:'4.1.13',
    families:[...definitions.map(([family])=>family),'shadows-replacement','filters-replacement'],sources,cases}
}
if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  const actual = await familyFixtures()
  const file = resolve(directory,'utf16-family-fixtures.json')
  if (process.argv.includes('--write')) await writeFile(file,JSON.stringify(actual)+'\n')
  else assert.deepEqual(actual,JSON.parse(await readFile(file,'utf8')))
  console.log(JSON.stringify({families:actual.families.length,cases:actual.cases.length,oracleVersion:actual.oracleVersion}))
}
