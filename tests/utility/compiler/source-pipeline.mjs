import assert from 'node:assert/strict'
import { createHash } from 'node:crypto'
import { readFile, writeFile, mkdtemp, rm } from 'node:fs/promises'
import { createRequire } from 'node:module'
import { dirname, resolve, posix } from 'node:path'
import { tmpdir } from 'node:os'
import { execFileSync } from 'node:child_process'
import { fileURLToPath } from 'node:url'
const directory = dirname(fileURLToPath(import.meta.url))
const root = resolve(directory, '../../..')
const oracle = createRequire(resolve(directory, 'oracle/package.json'))
const { compile } = oracle('tailwindcss')
assert.equal(oracle('tailwindcss/package.json').version, '4.1.13')
export async function sourcePipelineFixtures() {
  const inventory = JSON.parse(await readFile(resolve(root,'tests/utility/scanner/inventory.json')))
  for (const file of [...inventory.files,...(inventory.adapted_sources ?? [])]) {
    const bytes = await readFile(resolve(root,'ref/tailwindcss',file.source))
    assert.equal(createHash('sha256').update(bytes).digest('hex'),file.sha256,`fixed scanner source ${file.source}`)
  }
  const theme = '@theme { --color-brand: #123456; --color-alias: var(--color-brand); --spacing: 4px; }'
  const inputs = [
    '@tailwind utilities;',
    '@tailwind utilities source(none);',
    '@tailwind utilities source("./src");',
    '@tailwind utilities source(none); @source "./src/**/*.{html,rs}";',
    '@tailwind utilities source(none); @source "./src/**"; @source not "./src/private/**";',
    '@tailwind utilities; @source not "./src/private/**"; @source inline("block"); @source not inline("hidden");',
    '@import "pkg/utilities.css" source("../src");',
    '@reference "pkg/utilities.css"; @tailwind utilities source("./src");',
  ]
  const assets = [{id:'/app/pkg/utilities.css', content:'@tailwind utilities; @source "../shared/**";'}]
  const cases = []
  const temp = await mkdtemp(posix.join(tmpdir(), 'rustify-source-pipeline-'))
  try {
    for (const [index, body] of inputs.entries()) {
      const input = `${theme} ${body}`
      const compiled = await compile(input, {base:'/app', loadStylesheet:async (uri, base) => {
        const asset = assets.find(asset => asset.id === posix.normalize(posix.join(base, uri)))
        assert.ok(asset)
        return {path:asset.id, base:posix.dirname(asset.id), content:asset.content}
      }})
      const sources = (compiled.root === 'none' ? [] : compiled.root === null
        ? [{base:'/app',pattern:'**/*',negated:false}] : [{...compiled.root,negated:false}]).concat(compiled.sources)
      const files = [
        {path:'app/.gitignore',content:'ignored/\n'},
        {path:'app/src/main.rs',content:'view! { <div class="w-[137px] bg-brand p-2"> }'},
        {path:'app/src/view.html',content:'<p class="hidden text-brand">var(--color-alias)</p>'},
        {path:'app/src/private/secret.html',content:'<div class="w-[999px]">'},
        {path:'app/shared/sample.html',content:'<div class="h-[72px]">'},
        {path:'app/outside.html',content:'<div class="w-[159px]">'},
        {path:'app/ignored/test.html',content:'<div class="w-[666px]">'},
        {path:'app/theme.css',content:'.x {color:var(--color-brand)} .flex { --not-used: red; }'},
      ]
      const steps = [
        {operation:'scan'}, {operation:'scan'},
        {operation:'put',path:'app/src/main.rs',content:'view! { <div class="w-[147px]"> }'}, {operation:'scan'},
        {operation:'change',changed:[{content:'bg-brand var(--color-alias)',extension:'html'}]},
        {operation:'remove',path:'app/src/view.html'}, {operation:'scan'},
      ]
      const fixture = {name:`pipeline-${index}`,directories:['app/.git','app/src','app/shared'],files,sources:sources.map(source=>({...source,base:source.base.replace(/^\//,'')})),steps}
      const path = posix.join(temp,'input.json')
      await writeFile(path,JSON.stringify([fixture]))
      const [scanned] = JSON.parse(execFileSync('mbx',['run','--locked','--quiet','--manifest-path','tests/utility/scanner/oracle/Cargo.toml','--',path,'--sources'],{cwd:root,encoding:'utf8',maxBuffer:4*1024*1024}))
      cases.push({id:`pipeline:${index}`,input,entry:'/app/input.css',base:'/app',assets,files,directories:fixture.directories,
        steps:steps.map((step, i)=>({...step,...(scanned.expected[i]===null ? {} : {expectedCandidates:scanned.expected[i],expectedCss:compiled.build(scanned.expected[i]).replace(/^\/\*! tailwindcss[^\n]*\n/,'')})}))})
    }
  } finally { await rm(temp,{recursive:true,force:true}) }
  return {schemaVersion:1,sourceCommit:'1334c99db8fd26c8ea065375dd9259800863f072',oracleVersion:'4.1.13',cases}
}
if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  const actual = await sourcePipelineFixtures()
  if (process.argv.includes('--write')) await writeFile(resolve(directory,'source-pipeline-fixtures.json'),JSON.stringify(actual,null,2)+'\n')
  else assert.deepEqual(actual,JSON.parse(await readFile(resolve(directory,'source-pipeline-fixtures.json'),'utf8')))
  console.log(JSON.stringify({cases:actual.cases.length,operations:actual.cases.reduce((n,c)=>n+c.steps.length,0)}))
}
