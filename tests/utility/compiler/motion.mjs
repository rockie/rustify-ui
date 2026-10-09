import assert from 'node:assert/strict'
import { createHash } from 'node:crypto'
import { readFile, writeFile } from 'node:fs/promises'
import { createRequire } from 'node:module'
import { dirname, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'

const directory = dirname(fileURLToPath(import.meta.url))
const oracle = createRequire(resolve(directory, 'oracle/package.json'))
const tailwind = oracle('tailwindcss')
assert.equal(oracle('tailwindcss/package.json').version, '4.1.13')
const sha256 = text => createHash('sha256').update(text).digest('hex')
const roots = ['animate', 'transition', 'delay', 'duration', 'ease']
const staticRoots = ['animate-none', 'transition-none', 'transition-all', 'transition-colors', 'transition-opacity', 'transition-shadow', 'transition-transform', 'transition-discrete', 'transition-normal', 'duration-initial', 'ease-initial', 'ease-linear']
const declarations = '--animate-card: card 2s ease infinite; --animate-alias: var(--animate-card); --animate-1_5: card 1s; --animate-1\\/2: card 500ms; --transition-property-card: opacity, transform; --transition-property-alias: var(--transition-property-card); --transition-property-1_5: width; --transition-property-1\\/2: height; --transition-delay-card: 175ms; --transition-delay-alias: var(--transition-delay-card); --transition-delay-1_5: 225ms; --transition-delay-1\\/2: 275ms; --transition-duration-card: 325ms; --transition-duration-alias: var(--transition-duration-card); --transition-duration-1_5: 375ms; --transition-duration-1\\/2: 425ms; --ease-card: cubic-bezier(.1,.2,.3,.4); --ease-alias: var(--ease-card); --ease-1_5: steps(2, end); --ease-1\\/2: ease-in; --default-transition-duration: 150ms; --default-transition-timing-function: cubic-bezier(.4,0,.2,1);'
const keyframes = '@keyframes card { from { opacity: 0; } 50% { transform: translateX(10px); } to { opacity: 1; } } @keyframes unused { to { opacity: .5; } }'
const themes = {
  empty: '', base: `@theme { ${declarations} ${keyframes} }`, inline: `@theme inline { ${declarations} ${keyframes} }`, reference: `@theme reference { ${declarations} ${keyframes} }`, default: `@theme default { ${declarations} ${keyframes} }`,
  bare: `@theme { ${declarations} --animate: card 3s; --transition-property: width; --transition-delay: 525ms; --transition-duration: 625ms; --ease: ease-out; ${keyframes} }`,
  reset: `@theme { ${declarations} ${keyframes} } @theme { --animate-*: initial; --transition-*: initial; --ease-*: initial; --default-transition-*: initial; }`,
  defaultPriority: `@theme { ${declarations} ${keyframes} } @theme default { --animate-card: card 9s; --transition-property-card: height; --transition-delay-card: 9s; --transition-duration-card: 9s; --ease-card: ease-in; --default-transition-duration: 9s; --default-transition-timing-function: linear; }`,
  collision: `@theme { ${declarations} --animate-none: card 5s; --transition-property-none: width; --transition-property-all: height; --transition-property-colors: border-width; --transition-property-opacity: padding; --transition-property-shadow: margin; --transition-property-transform: top; --transition-property-discrete: left; --transition-property-normal: right; --transition-duration-initial: 725ms; --ease-initial: ease-in; --ease-linear: ease-out; ${keyframes} }`,
  dependencies: '@theme { --time: 175ms; --curve: cubic-bezier(.1,.2,.3,.4); --anim-name: card; --animate-card: var(--anim-name) var(--time) var(--curve); --transition-delay-card: var(--time); --transition-duration-card: var(--time); --ease-card: var(--curve); --default-transition-duration: var(--time); --default-transition-timing-function: var(--curve); @keyframes card { to { opacity: 0; } } }',
  staticTheme: `@theme static { ${declarations} ${keyframes} }`,
  prefix: `@theme prefix(tw) { ${declarations} ${keyframes} }`, prefixInline: `@theme prefix(tw) inline { ${declarations} ${keyframes} }`, prefixReference: `@theme prefix(tw) reference { ${declarations} ${keyframes} }`,
}
const values = ['', 'card', 'alias', '1.5', '1/2', 'missing', 'none', 'initial', 'linear', 'all', 'colors', 'opacity', 'shadow', 'transform', 'discrete', 'normal', '0', '1', '75', '123', '200', '1000', '01', '1.0', '0.5', '.5', '-1', '1e3', '1e+21', '1e21', 'Infinity', '[300ms]', '[.25s]', '[0]', '[var(--probe)]', '(--probe)', '[length:13px]', '[time:300ms]', '[number:2]', '[color:red]', '[nonsense:foo]', '[calc(1s_+_2s)]', '[opacity,transform]', '[cubic-bezier(.1,.2,.3,.4)]', '[steps(3,jump-end)]', '[linear(0,.25_50%,1)]', '[card_2s_ease_infinite]', '[none]', '[inherit]', '[initial]']
const modifiers = ['0', '1', '50', 'foo', 'card', '[var(--probe)]', '(--probe)', '[.5]', '[]', '[;]', '(bad)', '1/foo']

export async function motionFixtures() {
  const cases = []
  async function add(id, mode, candidates) {
    const input = `${themes[mode]} @tailwind utilities;`.trim()
    const compiler = await tailwind.compile(input)
    const expectedCss = compiler.build(candidates).replace(/^\/\*! tailwindcss[^\n]*\n/, '')
    cases.push({ id, input, candidates, expectedCss, expectedCssSha256: sha256(expectedCss) })
  }
  for (const root of roots) {
    for (const mode of ['empty', 'base', 'inline', 'reference', 'default', 'bare', 'reset', 'defaultPriority', 'collision', 'dependencies']) for (const value of values) await add(`motion:${root}-${value || '<default>'}:${mode}`, mode, [`${root}${value ? `-${value}` : ''}`])
    for (const mode of ['base', 'bare', 'inline', 'reference', 'collision']) for (const value of ['', 'card', '1/2', '75', '[300ms]', '[var(--probe)]']) for (const modifier of modifiers) await add(`motion:${root}-${value || '<default>'}/${modifier}:${mode}`, mode, [`${root}${value ? `-${value}` : ''}/${modifier}`])
    for (const mode of ['prefix', 'prefixInline', 'prefixReference']) for (const candidate of [`tw:${root}-card`, `tw:${root}-alias`, `tw:${root}-1/2`, `tw:${root}-[300ms]`, `tw:hover:${root}-card!`, `${root}-card`]) await add(`motion:${candidate}:${mode}`, mode, [candidate])
    for (const value of ['card', '75', '[300ms]']) await add(`motion:-${root}-${value}:negative`, 'base', [`-${root}-${value}`])
    await add(`motion:${root}:hover-important`, 'base', [`hover:${root}-card!`])
    for (const fraction of ['1/0', '0/0', '2/3', 'card/2', '1/[2]', '1/(--probe)']) await add(`motion:${root}-${fraction}:fraction`, 'base', [`${root}-${fraction}`])
  }
  for (const root of ['delay', 'duration']) for (let number = 0; number <= 1000; number += 5) await add(`motion:${root}-${number}:integer`, 'empty', [`${root}-${number}`])
  for (const root of staticRoots) {
    for (const mode of ['empty', 'base', 'inline', 'reference', 'collision', 'staticTheme']) await add(`motion:static:${root}:${mode}`, mode, [root])
    for (const mode of ['prefix', 'prefixInline', 'prefixReference']) await add(`motion:static:tw:${root}:${mode}`, mode, [`tw:${root}`])
    for (const candidate of [`-${root}`, `${root}/foo`, `hover:${root}!`]) await add(`motion:static:${candidate}:boundary`, 'collision', [candidate])
  }
  const themeCss = await readFile(resolve(directory, '../../../ref/tailwindcss/packages/tailwindcss/theme.css'), 'utf8')
  const defaultMotion = themeCss.slice(themeCss.indexOf('--ease-in:'), themeCss.indexOf('--blur-xs:'))
  const defaults = [...themeCss.matchAll(/--default-transition-(?:duration|timing-function)\s*:[^;]+;/g)].map(match => match[0]).join(' ')
  themes.upstreamDefault = `@theme default { ${defaultMotion} ${defaults} }`
  for (const candidate of ['animate-spin', 'animate-ping', 'animate-pulse', 'animate-bounce', 'animate-none', 'ease-in', 'ease-out', 'ease-in-out', 'ease-linear', 'transition', 'transition-all', 'transition-none', 'duration-150', 'delay-150']) await add(`motion:${candidate}:upstream-default`, 'upstreamDefault', [candidate])
  await add('motion:all-sorted-union', 'collision', [...staticRoots, 'animate-card', 'transition-card', 'delay-card', 'duration-card', 'ease-card', 'duration-[275ms]', 'ease-[steps(2,end)]', 'hover:animate-card!', 'hover:duration-card!'])
  await add('motion:upstream-default-sorted-union', 'upstreamDefault', ['animate-bounce', 'animate-spin', 'animate-pulse', 'animate-ping', 'transition-all', 'duration-150', 'delay-75', 'ease-in-out'])
  await add('motion:raw-animation-keyframe-use', 'base', ['animate-[card_2s_ease_infinite]', 'animate-[unused_1s]', 'animate-none'])
  const ids = new Set(); for (const item of cases) { assert.ok(!ids.has(item.id), `duplicate ${item.id}`); ids.add(item.id) }
  const inventory = JSON.parse(await readFile(resolve(directory, 'inventory.json'), 'utf8'))
  const sources = ['packages/tailwindcss/src/utilities.ts', 'packages/tailwindcss/src/utilities.test.ts', 'packages/tailwindcss/src/theme.ts', 'packages/tailwindcss/src/ast.ts', 'packages/tailwindcss/src/utils/infer-data-type.ts'].map(path => { const source = inventory.sources.find(item => item.path === path); assert.ok(source?.sha256, path); return { path, sha256: source.sha256 } })
  sources.push({ path: 'packages/tailwindcss/theme.css', sha256: sha256(themeCss) })
  return { schemaVersion: 1, sourceCommit: '1334c99db8fd26c8ea065375dd9259800863f072', oracleVersion: '4.1.13', functionalRoots: roots, staticRegressionRoots: staticRoots, fullFamilyConformance: 'pending', sources, cases }
}

export async function checkMotionFixtures() {
  const actual = await motionFixtures()
  assert.deepEqual(actual, JSON.parse(await readFile(resolve(directory, 'motion-fixtures.json'), 'utf8')))
  return { functionalRoots: roots.length, staticRegressionRoots: staticRoots.length, cases: actual.cases.length }
}
if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  if (process.argv.includes('--write')) { const actual = await motionFixtures(); await writeFile(resolve(directory, 'motion-fixtures.json'), JSON.stringify(actual, null, 2) + '\n'); console.log(JSON.stringify({ functionalRoots: roots.length, cases: actual.cases.length })) }
  else console.log(JSON.stringify(await checkMotionFixtures()))
}
