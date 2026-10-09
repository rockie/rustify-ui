import assert from 'node:assert/strict'
import { readFile, writeFile } from 'node:fs/promises'
import { createRequire } from 'node:module'
import { dirname, resolve, posix } from 'node:path'
import { fileURLToPath } from 'node:url'

const directory = dirname(fileURLToPath(import.meta.url))
const oracle = createRequire(resolve(directory, 'oracle/package.json'))
const { compile } = oracle('tailwindcss')
assert.equal(oracle('tailwindcss/package.json').version, '4.1.13')
const banner = css => css.replace(/^\/\*! tailwindcss[^\n]*\n/, '')
const declarations = '--spacing: .25rem; --color-brand: #123456; --breakpoint-sm: 40rem;'
const patterns = [
  '', 'underline', 'my-cucumber', 'flex block', ' flex  block ', 'flex\tblock', 'flex\nblock',
  '{flex,block,hidden}', '{hover:,focus:,}underline', '{{hover,focus}:,}hidden',
  '{hover:,focus:,}{flex,block}', 'w-[{0..5}px]', 'w-[{-5..0}px]', 'w-[{0..-5}px]',
  'w-[{0..10..5}px]', 'w-[{0..10..-5}px]', 'w-[{10..0..5}px]', 'w-[{10..0..-5}px]',
  'w-[{00..05}px]', 'w-[{001..9}px]', 'w-[{0..5..2}px]', 'w-[{1.1..2.2}px]',
  'w-[{50,{100..900..100},950}px]', 'w-[{1,{2,3},{4..6}}px]',
  'w-[{1,2}{3,4}px]', 'w-[{,1,}px]', 'w-[{1}px]', 'w-[{}px]',
  '{flex,flex,block}', '{flex,{block,{hidden}}}', 'w-[calc({1,2}px+{3,4}px)]',
  '[--probe:{red,blue}]', '[content:\'a b\']', '[--probe:{var(--one),var(--two)}]',
  'w-[{1,2}px] bg-brand/50', 'bg-brand/{25,50,75}', '{!,}flex', '{flex,block}!',
  'w-[{+1..3}px]', 'w-[{1..3..+1}px]', 'w-[{-0..0}px]',
  'w-[{9007199254740993..9007199254740993..2}px]',
  'w-[{1000000000000000000000..1000000000000000000000000..1000000000000000000000}px]',
  'w-[{0..5..0}px]', 'w-[{1,2px]', 'flex{block,{hidden}', 'flex{block}tail{hidden',
]
// Each brace-expansion.test.ts input is embedded in an arbitrary property so
// every expanded branch appears in CSS from the locked official compiler.
const upstreamBracePatterns = [
  'a/b/c', 'a/{x,y,z}/b', '{a,b}/{x,y}', '{{xs,sm,md,lg}:,}hidden',
  'a/{0..5}/b', 'a/{-5..0}/b', 'a/{0..-5}/b', 'a/{0..10..5}/b',
  'a/{0..10..-5}/b', 'a/{10..0..5}/b', 'a/{10..0..-5}/b',
  'a/{00..05}/b', 'a{001..9}b', 'a/{0..5..2}/b', 'bg-red-{100..900..100}',
  'a{b,c,/{x,y}}/e', 'a{b,c,/{x,y},{z,w}}/e', 'a{b,c,/{x,y},{0..2}}/e',
  'bg-red-{50,{100..900..100},950}', '{1.1..2.2}',
  'a{b,c{d,e},{f,g}h}x{y,z', 'a{0..5..0}/b',
]
patterns.push(...upstreamBracePatterns.map(pattern => `[--probe:${pattern}]`))

for (const start of [-3, 0, 3]) for (const end of [-3, 0, 3]) for (const step of [undefined, -4, -2, -1, 0, 1, 2, 4])
  patterns.push(`w-[{${start}..${end}${step === undefined ? '' : `..${step}`}}px]`)

export async function sourceDirectiveFixtures() {
  const cases = []
  async function add(id, input, candidates = [], assets, batches) {
    const options = { base:'/app' }
    if (assets) options.loadStylesheet = async (uri, base) => {
      const id = posix.normalize(posix.join(base, uri))
      const asset = assets.find(asset => asset.id === id) ?? assets.find(asset => asset.id === uri)
      assert.ok(asset, `missing oracle asset ${uri} from ${base}`)
      return { path:asset.id, base:posix.dirname(asset.id), content:asset.content }
    }
    const item = { id:`source:${id}`, input, candidates }
    if (assets) { item.entry = '/app/input.css'; item.assets = assets }
    try {
      const compiled = await compile(input, options)
      item.expectedCss = banner(compiled.build(candidates))
      item.expectedSources = compiled.sources
      item.expectedRoot = compiled.root
      if (batches) {
        const incremental = await compile(input, options)
        item.batches = batches
        item.expectedBuilds = batches.map(batch => banner(incremental.build(batch)))
      }
    } catch (error) { item.expectedError = error.message }
    cases.push(item)
  }
  for (const mode of ['', 'inline', 'reference', 'static', 'default', 'prefix(tw)']) {
    const theme = `@theme ${mode} { ${declarations} }`
    for (const [index, pattern] of patterns.entries()) for (const not of ['', 'not ']) {
      const source = mode === 'prefix(tw)' ? pattern.split(' ').map(part => `tw:${part}`).join(' ') : pattern
      await add(`patterns:${mode}:${index}:${not}`, `${theme} @source ${not}inline(${JSON.stringify(source)}); @tailwind utilities;`, ['flex', 'block', 'underline', 'w-[1px]', 'hover:underline', 'tw:flex', 'tw:bg-brand'])
    }
  }
  for (const declaration of [
    '@source inline("flex");', "@source inline('block');", '@source inline("");', "@source inline('');", '@source inline();',
    '@source inline(flex);', '@source not inline(flex);', '@source inline("flex") extra;', '@source inline("flex", "block");',
    '@source INLINE("flex");', '@source not  inline("flex");', '@source not\tinline("flex");', '@source inline( "flex" );',
    '@source inline("flex") {}', '@source inline("flex") { color:red; }', '@source not inline("flex") { @source inline("block"); }',
    '@source;', '@source not;',
  ]) for (const wrapper of ['BODY', '.host { BODY }', '@media screen { BODY }', '@layer utilities { BODY }', '@supports (display:grid) { BODY }', '@theme { BODY }', '@utility probe { BODY }'])
    await add(`syntax:${declaration}:${wrapper}`, `${wrapper.replace('BODY', declaration)} @tailwind utilities;`, ['flex', 'block', 'probe'])
  for (const sources of [
    '@source inline("flex block"); @source not inline("flex");', '@source not inline("flex"); @source inline("flex block");',
    '@source inline("flex flex block"); @source inline("block hidden");', '@source not inline("{hover:,}flex"); @source inline("{hover:,focus:,}flex");',
    '@source inline("w-[{1..3}px]"); @source not inline("w-[{2..3}px]");',
  ]) for (const tailwind of ['', '@tailwind utilities;', '@tailwind utilities source(none);'])
    await add(`incremental:${sources}:${tailwind}`, `${sources} ${tailwind}`, ['flex','block','underline'], undefined, [[], ['flex'], ['block'], ['underline','w-[4px]'], [], ['flex','block']])
  for (const params of ['utilities source(none)', 'utilities ignored', 'utilities important', 'utilities source(none) ignored', 'utilities source(unquoted)', 'utilities source("./src")', 'utilities source(none) source("./src")', 'utilities source("./src") source(none)']) {
    await add(`tailwind:${params}`, `@source inline("flex"); @tailwind ${params};`, [])
    await add(`duplicate-tailwind:${params}`, `@tailwind utilities; @tailwind ${params};`, ['flex'])
  }
  const assets = [{ id:'/app/source.css', content:'@source inline("flex block"); @source not inline("hidden"); @tailwind utilities;' }]
  for (const directive of ['@import "source.css";', '@reference "source.css";', '@import "source.css" reference;', '@import "source.css" layer(utilities);', '@import "source.css" screen;'])
    await add(`import:${directive}`, `${directive} @tailwind utilities;`, ['hidden','underline'], assets)
  for (const input of ['@source "./foo/**/*.ts";', '@source not "./foo/**/*.ts";', '@source "./foo/**/*.ts"; @source not "./secret/*.ts";', '@tailwind utilities source("./src");'])
    await add(`metadata:${input}`, input, ['flex'])
  const pathAssets = [
    {id:'/app/pkg/utilities.css', content:'@tailwind utilities; @source "./component/**/*.{html,rs}"; @source not "./private/**";'},
    {id:'/app/pkg/entry.css', content:'@import "nested/root.css"; @source "../shared/**";'},
    {id:'/app/pkg/nested/root.css', content:'@tailwind utilities source("./nested-src"); @source "./local/**";'},
    {id:'/app/pkg/explicit.css', content:'@tailwind utilities source("./own-src");'},
  ]
  for (const path of ['./src', '../src/**/*.{html,rs}', '/absolute/**', '', './a b', '!./negated', './escaped\\\\path']) {
    for (const not of ['', 'not ']) await add(`path:${JSON.stringify(path)}:${not}`, `@source ${not}${JSON.stringify(path)}; @tailwind utilities;`, ['flex'])
    await add(`root:${JSON.stringify(path)}`, `@tailwind utilities source(${JSON.stringify(path)});`, ['flex'])
  }
  for (const uri of ['pkg/utilities.css', 'pkg/entry.css', 'pkg/explicit.css']) {
    for (const options of ['', 'source(none)', 'source("./override")', 'reference', 'source("../host") reference']) {
      await add(`import-base:${uri}:${options}`, `@import "${uri}" ${options}; @source "./host/**"; @tailwind utilities;`, ['flex'], pathAssets)
    }
  }
  for (const input of [
    '@media source("./outer") { @import "pkg/utilities.css"; }',
    '@media source("./outer") { @import "pkg/entry.css"; }',
    '@media source("./outer") { @import "pkg/utilities.css" source("./inner"); }',
    '@media source("./outer") { @import "pkg/explicit.css"; }',
    '@media source("./outer") { @media source("./inner") { @import "pkg/utilities.css"; } }',
    '@media source(none) { @media source("./inner") { @import "pkg/utilities.css"; } }',
    '@media source("./outer") source("./later") { @import "pkg/utilities.css"; }',
    '@import "pkg/utilities.css"; @tailwind utilities source("./ignored");',
    '@reference "pkg/utilities.css"; @tailwind utilities source("./host");',
    '@import "pkg/utilities.css" layer(utilities);',
    '@custom-variant registered { @source "./ignored/**"; @slot; } @tailwind utilities;',
    '@variant registered { @source "./ignored/**"; @slot; } @tailwind utilities;',
  ]) await add(`metadata-context:${input}`, input, ['flex'], pathAssets)
  for (const input of ['@tailwind utilities; @media important { .x { color:red; } }', '@media important { .x { color:red; } } @tailwind utilities;', '@tailwind utilities; @media important { @tailwind utilities; }', '@media reference important { @tailwind utilities; } @tailwind utilities;'])
    await add(`global-important:${input}`, input, ['flex'])
  return { schemaVersion:1, sourceCommit:'1334c99db8fd26c8ea065375dd9259800863f072', oracleVersion:'4.1.13', cases }
}
if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  const actual = await sourceDirectiveFixtures()
  if (process.argv.includes('--write')) await writeFile(resolve(directory, 'source-directives-fixtures.json'), JSON.stringify(actual, null, 2) + '\n')
  else assert.deepEqual(actual, JSON.parse(await readFile(resolve(directory, 'source-directives-fixtures.json'), 'utf8')))
  console.log(JSON.stringify({ cases:actual.cases.length, css:actual.cases.filter(item => item.expectedCss !== undefined && !item.pendingFeature).length, errors:actual.cases.filter(item => item.expectedError).length, metadataCases:actual.cases.filter(item => item.expectedSources).length, incrementalBuilds:actual.cases.reduce((sum,item) => sum + (item.batches?.length ?? 0), 0) }))
}
