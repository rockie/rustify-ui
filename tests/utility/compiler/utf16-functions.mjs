import assert from 'node:assert/strict'
import {createHash} from 'node:crypto'
import {createRequire} from 'node:module'
import {readFile, writeFile} from 'node:fs/promises'
import {dirname, resolve} from 'node:path'
import {fileURLToPath} from 'node:url'
const dir = dirname(fileURLToPath(import.meta.url))
const require = createRequire(resolve(dir, 'oracle/package.json'))
const {compile} = require('tailwindcss')
assert.equal(require('tailwindcss/package.json').version, '4.1.13')
const units = text => Array.from({length: text.length}, (_, i) => text.charCodeAt(i))
const root = resolve(dir, '../../..')
const sourceCommit = '1334c99db8fd26c8ea065375dd9259800863f072'
const provenance = JSON.parse(await readFile(resolve(root, 'crates/rustify-style/upstream/tailwind/PROVENANCE.json'), 'utf8'))
assert.equal(provenance.commit, sourceCommit)
const sourcePaths = ['packages/tailwindcss/src/css-functions.ts', 'packages/tailwindcss/src/utils/segment.ts', 'packages/tailwindcss/src/compat/plugin-functions.ts', 'packages/tailwindcss/src/utils/to-key-path.ts']
const sources = await Promise.all(sourcePaths.map(async path => ({path, sha256: createHash('sha256').update(await readFile(resolve(root, 'ref/tailwindcss', path))).digest('hex')})))
for (const source of sources) if (provenance.sourceDigests[source.path]) assert.equal(source.sha256, provenance.sourceDigests[source.path], source.path)
const cases = []
for (const [name, text] of [['high', String.fromCharCode(0xd800)], ['low', String.fromCharCode(0xdc00)], ['pair', '😀'], ['replacement', '�'], ['private', '\ue000']]) {
  for (const [kind, expression] of [
    ['alpha', `--alpha(${text} / .5)`],
    ['alpha-full', `--alpha(${text} / 100%)`],
    ['spacing', `--spacing(${text})`],
    ['fallback', `--theme(--missing, outer(${text}, var(--other)))`],
    ['legacy-fallback', `theme(missing, ${text})`],
    ['nested', `--alpha(--theme(--missing, ${text}) / .5)`],
    ['fallback-var', `--theme(--color-card, ${text})`],
  ]) {
    const themeCss = '@theme reference { --spacing: 4px; --color-card: var(--external, initial); }'
    const input = `${themeCss} .p { value: ${expression}; } @tailwind utilities;`
    const compiler = await compile(input)
    const css = compiler.build([]).replace(/^\/\*! tailwindcss[^\n]*\n/, '')
    const value = css.match(/  value: ([\s\S]*);\n}/)[1]
    cases.push({id: `${kind}:${name}`, themeCss, valueUtf16Units: units(expression), expectedValueUtf16Units: units(value)})
  }
}
for (const [name, text, escaped] of [['high', String.fromCharCode(0xd800), '\\d800'], ['low', String.fromCharCode(0xdc00), '\\dc00'], ['low-high', String.fromCharCode(0xdc00, 0xd800), '\\dc00\\d800']]) {
  for (const path of [`padding.${text}`, `padding[${text}]`, `"padding[${text}]"`, `colors.${text}`]) {
    const themeCss = `@theme { --spacing-${escaped}: 101px; --color-${escaped}: red; }`
    const expression = `theme(${path}, FALLBACK)`
    const compiler = await compile(`${themeCss} .p { value: ${expression}; } @tailwind utilities;`)
    const css = compiler.build([]).replace(/^\/\*! tailwindcss[^\n]*\n/, '')
    const value = css.match(/  value: ([\s\S]*);\n}/)[1]
    cases.push({id: `path:${name}:${path.slice(0, path.indexOf(text))}`, themeCss, valueUtf16Units: units(expression), expectedValueUtf16Units: units(value)})
  }
}
for (const [mode, options] of [['ordinary', ''], ['static', 'static'], ['reference', 'reference'], ['inline', 'inline'], ['default', 'default'], ['prefix', 'prefix(tw)'], ['default-prefix', 'default prefix(tw)']]) {
  const high = String.fromCharCode(0xd800)
  for (const [name, expression] of [['legacy-key', `theme(padding[${high}])`], ['css-key', `--theme(--color-${high})`], ['css-key-inline', `--theme(--color-${high} inline)`], ['css-key-fallback', `--theme(--color-${high}, FALLBACK)`]]) {
    const themeCss = `@theme ${options} { --spacing-\\d800: 101px; --color-\\d800: red; }`
    const compiler = await compile(`${themeCss} .p { value: ${expression}; } @tailwind utilities;`)
    const css = compiler.build([]).replace(/^\/\*! tailwindcss[^\n]*\n/, '')
    const value = css.match(/  value: ([^\n]*);\n}/)[1]
    cases.push({id: `options:${mode}:${name}`, themeCss, valueUtf16Units: units(expression), expectedValueUtf16Units: units(value)})
  }
}
const result = {sourceCommit, oracleVersion:'4.1.13', sources, cases}
const path = resolve(dir, 'utf16-function-fixtures.json')
if (process.argv.includes('--write')) await writeFile(path, JSON.stringify(result, null, 2) + '\n')
else assert.deepEqual(result, JSON.parse(await readFile(path, 'utf8')))
console.log(JSON.stringify({cases:cases.length}))
