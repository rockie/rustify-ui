import assert from 'node:assert/strict'
import { readFile, writeFile } from 'node:fs/promises'
import { createRequire } from 'node:module'
import { dirname, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'
const directory = dirname(fileURLToPath(import.meta.url))
const oracle = createRequire(resolve(directory, 'oracle/package.json'))
const {compile} = oracle('tailwindcss')
assert.equal(oracle('tailwindcss/package.json').version, '4.1.13')
const banner = css => css.replace(/^\/\*! tailwindcss[^\n]*\n/, '')
const classes = ['ring', 'ring-0', 'ring-2', 'ring-[3px]', 'ring-[length:var(--probe)]', 'ring-[#123456]', 'ring-current', 'ring-inherit', 'ring-transparent', 'ring-inset', 'shadow-none', 'shadow-inherit', 'shadow-[#123456]', 'shadow-[0_2px_4px_#123456]', 'shadow-[0_2px_4px]', 'shadow-[inset_0_2px_4px_rgb(10_20_30)]', 'content-["probe"]', 'before:content-["probe"]', 'after:content-[attr(data-label)]', 'before:w-[137px]!', 'after:content-none', 'before:hover:content-["probe"]']
const radiusRoots = ['rounded', 'rounded-s', 'rounded-e', 'rounded-t', 'rounded-r', 'rounded-b', 'rounded-l', 'rounded-ss', 'rounded-se', 'rounded-ee', 'rounded-es', 'rounded-tl', 'rounded-tr', 'rounded-br', 'rounded-bl']
classes.push(...radiusRoots.map(root => `${root}-[8px]`))
const theme = '@theme { --radius: 11px; --radius-card: 13px; --default-ring-width: 7px; --default-ring-color: purple; --shadow-card: 0 2px 4px #123456; --color-brand: #aabbcc; } '
export async function effectsFixtures() {
 const cases=[]
 for (const candidate of classes) {
   const input='@tailwind utilities;'
   const compiler=await compile(input)
   cases.push({id:`effect:${candidate}`, input, candidates:[candidate], expectedCss:banner(compiler.build([candidate]))})
 }
 for (const candidate of ['ring', 'ring-brand', 'shadow-card', 'shadow-brand', ...radiusRoots, ...radiusRoots.map(root => `${root}-card`)]) {
   const input=theme+'@tailwind utilities;'
   const compiler=await compile(input)
   cases.push({id:`effect-theme:${candidate}`, input, candidates:[candidate], expectedCss:banner(compiler.build([candidate]))})
 }
 const compiler=await compile('@tailwind utilities;')
 cases.push({id:'effect:union', input:'@tailwind utilities;', candidates:classes, expectedCss:banner(compiler.build(classes))})
 return {schemaVersion:1, sourceCommit:'1334c99db8fd26c8ea065375dd9259800863f072', oracleVersion:'4.1.13', cases}
}
export async function checkEffectsFixtures() {
 const actual=await effectsFixtures()
 assert.deepEqual(actual,JSON.parse(await readFile(resolve(directory,'effects-fixtures.json'),'utf8')))
 return {cases:actual.cases.length}
}
if(process.argv[1] && resolve(process.argv[1])===fileURLToPath(import.meta.url)) {
 if(process.argv.includes('--write')) { const actual=await effectsFixtures();await writeFile(resolve(directory,'effects-fixtures.json'),JSON.stringify(actual,null,2)+'\n');console.log(JSON.stringify({cases:actual.cases.length})) }
 else console.log(JSON.stringify(await checkEffectsFixtures()))
}
