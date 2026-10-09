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
export async function importFixtures() {
  const cases = []
  const assets = [
    { id: '/app/theme.css', content: '@theme { --spacing: 4px; --color-brand: #123456; }' },
    { id: '/app/parts/body.css', content: '@import "../theme.css"; .custom { color: red; } @tailwind utilities;' },
    { id: '/app/plain.css', content: '.imported { color: red; }' },
    { id: 'package/theme.css', content: '@theme { --spacing: 7px; --color-brand: #abcdef; }' },
    { id: '/app/reference.css', content: '@theme { --spacing: 4px; --color-brand: #123456; } .hidden { color: red; } @tailwind utilities;' },
  ]
  async function add(id, input, candidates = ['p-3', 'bg-brand'], bundle = assets) {
    const loaded = []
    const item = { id: `import:${id}`, input, entry: '/app/input.css', assets: bundle, candidates, loaded }
    try {
    const compiler = await compile(input, { base: '/app', loadStylesheet: async (uri, base) => {
      const path = posix.normalize(posix.join(base, uri))
      const asset = bundle.find(item => item.id === path) ?? bundle.find(item => item.id === uri)
      assert.ok(asset, `missing oracle virtual asset ${uri} from ${base}`)
      loaded.push({ uri, base, path: asset.id })
      return { path: asset.id, base: posix.dirname(asset.id), content: asset.content }
    } })
    item.expectedCss = banner(compiler.build(candidates))
    } catch (error) { item.expectedError = error.message }
    cases.push(item)
  }
  await add('relative-theme', '@import "./theme.css"; @tailwind utilities;')
  await add('bare-package', '@import "package/theme.css"; @tailwind utilities;')
  await add('nested-relative', '@import "parts/body.css";')
  await add('duplicate', '@import "plain.css"; @import "./plain.css"; @tailwind utilities;')
  await add('empty-layer-after-theme-pruning', '@import "theme.css" layer(components); @tailwind utilities;', ['flex'])
  await add('empty-layer', '@layer utilities {} @tailwind utilities;', ['flex'])
  for (const layer of ['layer', 'layer(utilities)']) for (const condition of ['', ' screen', ' supports(display:grid)', ' supports((display:grid)) print'])
    await add(`${layer}${condition}`, `@import "plain.css" ${layer}${condition}; @tailwind utilities;`, ['flex'])
  await add('media-with-theme', '@import "parts/body.css" screen and (width > 500px);')
  await add('reference-theme', '@reference "theme.css"; @tailwind utilities;')
  await add('reference-css-hidden', '@reference "reference.css"; @tailwind utilities;')
  await add('import-reference', '@import "reference.css" reference; @tailwind utilities;')
  await add('reference-under-layer', '@import "reference.css" layer(utilities) reference; @tailwind utilities;')
  await add('reference-then-local-theme', '@reference "theme.css"; @theme { --color-brand: #abcdef; } @tailwind utilities;')
  await add('local-theme-then-reference', '@theme { --color-brand: #abcdef; } @reference "theme.css"; @tailwind utilities;')
  await add('import-reference-with-media', '@import "reference.css" screen reference; @tailwind utilities;')
  await add('media-reference-input', '@media reference { @theme { --spacing: 4px; --color-brand: #123456; } .hidden { color:red; } } @tailwind utilities;')
  await add('ordinary-nested', '@media screen { @import "plain.css"; } @tailwind utilities;', ['flex'])
  for (const uri of ['"https://example.test/a.css"', '"http://example.test/a.css"', '"data:text/css,body{}"', 'url("https://example.test/a.css")', 'url(local.css)', 'unquoted.css'])
    await add(`retained-${uri}`, `@import ${uri}; @tailwind utilities;`, ['flex'])
  for (const uri of ['"https://example.test/a.css"', 'url(local.css)', 'unquoted.css'])
    await add(`retained-reference-${uri}`, `@reference ${uri}; @tailwind utilities;`, ['flex'])
  for (const option of ['theme(inline)', 'theme(reference)', 'theme(default)', 'theme(static)',
    'theme(inline reference)', 'theme(static default)', 'theme(unknown)', 'theme()',
    'prefix(tw)', 'prefix()', 'prefix(TW)', 'prefix(tw) prefix(app)',
    'theme(prefix(tw))', 'theme(inline) prefix(tw)', 'prefix(tw) theme(inline)',
    'theme(reference) important', 'source(none)', 'source(unquoted)', 'important']) {
    for (const placement of ['import', 'media']) {
      const candidates = ['p-3', 'bg-brand', 'tw:p-3', 'tw:bg-brand', 'app:p-3', 'app:bg-brand']
      const input = placement === 'import'
        ? `@import "theme.css" ${option}; @tailwind utilities;`
        : `@media ${option} { @theme { --spacing: 4px; --color-brand: #123456; } } @tailwind utilities;`
      await add(`option:${placement}:${option}`, input, candidates)
    }
  }
  for (const option of ['theme(inline)', 'theme(reference)', 'prefix(tw)', 'source(none)', 'source(unquoted)', 'important', 'important screen', 'screen important', 'reference important']) {
    await add(`body-option:${option}`, `@import "parts/body.css" ${option};`, ['flex', 'p-3', 'bg-brand', 'tw:p-3', 'tw:bg-brand'])
    await add(`media-option:${option}`, `@media ${option} { .probe { color: red; } @theme { --spacing: 4px; } @tailwind utilities; }`, ['flex', 'p-3', 'tw:p-3'])
  }
  for (const outer of ['theme(inline)', 'theme(default)', 'theme(static)', 'prefix(tw)', 'important', 'reference', 'screen']) {
    for (const inner of ['theme(reference)', 'theme(default)', 'prefix(app)', 'important', 'reference', 'print']) {
      await add(`nested:${outer}:${inner}`, `@media ${outer} { @media ${inner} { @theme { --spacing: 4px; --color-brand: #123456; } .probe { color: red; } @tailwind utilities; } }`, ['p-3', 'bg-brand', 'tw:p-3', 'app:p-3'])
    }
  }
  await add('important-outside-utility', '@media important { .probe { color: red; } } @tailwind utilities;', ['flex'])
  await add('important-after-utility', '@tailwind utilities; @media important { .probe { color: red; } }', ['flex'])
  await add('first-tailwind-ignores-source-on-duplicate', '@tailwind utilities; @tailwind utilities source(unquoted);', ['flex'])
  await add('source-none-import-utility', '@import "parts/body.css" source(none);', ['p-3'])
  for (const input of [
    '@theme { @media theme(inline) { --x:red; } } @tailwind utilities;',
    '@custom-variant foo { @media important { @slot; } } @tailwind utilities;',
    '@custom-variant foo { @media theme(reference) { color:red; @slot; } } @tailwind utilities;',
    '@variant foo { @media important { @slot; } } @tailwind utilities;',
    '@utility foo { @media important { color:red; } } @tailwind utilities;',
    '@custom-variant foo { @source inline("hidden"); @slot; } @tailwind utilities;',
    '@custom-variant foo { @tailwind utilities source(unquoted); @slot; } @tailwind utilities;',
    '@variant foo { @source inline("hidden"); @slot; } @tailwind utilities;',
  ]) await add(`media-traversal:${input}`, input, ['flex'])
  return { schemaVersion: 1, sourceCommit: '1334c99db8fd26c8ea065375dd9259800863f072', oracleVersion: '4.1.13', cases }
}
if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  const actual = await importFixtures()
  if (process.argv.includes('--write')) await writeFile(resolve(directory, 'imports-fixtures.json'), JSON.stringify(actual, null, 2) + '\n')
  else assert.deepEqual(actual, JSON.parse(await readFile(resolve(directory, 'imports-fixtures.json'), 'utf8')))
  console.log(JSON.stringify({ cases: actual.cases.length }))
}
