// Development-only differential oracle for the entire cn build core surface.
import assert from 'node:assert/strict'
import {mkdtempSync,mkdirSync,writeFileSync,readFileSync,symlinkSync,rmSync,existsSync} from 'node:fs'
import {resolve,relative,dirname} from 'node:path'
import {tmpdir} from 'node:os'
import {spawnSync} from 'node:child_process'
import {fixtureRoot,commit,load,assertLockedSource,json,sha256,read} from './upstream.mjs'
assertLockedSource()
const mode=process.argv[2];if(!['--check','--write'].includes(mode))throw Error('Use --check or --write')
const upstream=await load('build'),theme=await load('theme-css'),compiler=await load('compiler')
const temp=mkdtempSync(resolve(tmpdir(),'cn-build-contract-')),cwd=resolve(temp,'project')
const files={
 'src/view.html':'<div class="p-2 p-4"></div>', 'src/component.tsx':'hover:p-2 hover:p-4', 'src/noext':'m-2 m-4',
 '.hidden/view.html':'p-2', '.git/view.html':'p-2', 'node_modules/hidden/view.html':'p-2', 'node_modules/allowed/view.html':'m-4',
 'dist/view.html':'p-2','public/.dot.html':'p-2','docs/view.mdx':'text-lg','literal[1].html':'p-2',
 'themes/main.css':'/* @theme { --shadow-comment: 0; } */ @import url("./imported.css"); @import "package/theme.css"; @theme inline { --radius-*: initial; --radius-card: 1px; --text-hero: 1px; --text-hero--line-height: 1; --color-brand: red; --font-brand: serif; --shadow-gone: initial; }',
 'themes/imported.css':'@import "./cycle.css"; @theme { --shadow-card: 1px; --shadow-gone: 1px; --font-weight-book: 550; }',
 'themes/cycle.css':'@import "./main.css"; @theme { --text-shadow-label: 1px; }',
 'themes/reset.css':'@theme { --*: initial; --blur-first: 1px; --blur-first: initial; --blur-last: 2px; --radius-card: initial; }',
 'plain.css':'/* @theme */ .x {}', 'themes/unclosed.css':'@theme {', 'themes/unclosed-unicode.css':'@theme { --shadow-odd: initial😀', 'themes/unicode.css':'@themeé { --shadow-é: 1px; }',
 'public/cr\r.html':'p-2', 'public/line.html\n':'p-4', 'tokens.txt':'p-2 p-4 '+ 'x'.repeat(8193),'safelist.txt':'m-2\uFEFFm-4',
 'extension.json':JSON.stringify({prefix:'tw',extend:{theme:{shadow:['panel']}}}),
 'config.mjs':'export default config=>({...config,prefix:"tw"})',
 '../outside.html':'p-4'
}
const links={'alias':'src','src/cycle':'..','src/broken.html':'../missing.html','node_modules/srclink':'../src'}
const globs=[['**/*.html'],['src'],['./src\\**\\*.{html,tsx}'],['node_modules/allowed/*.html','**/*.html'],['.hidden/*.html'],['.git/*.html'],['.git'],['node_modules'],['src','alias/**/*.html'],['../outside.html'],['src/**/*.html','src/**/*.html'],['literal[1].html'],['src/*.{html,tsx}'],['src/?.html'],['src/{oops']]
const matcherInputs=[['**/*.html'],['**/*.html','node_modules/allowed/*.html'],['src'],['.git'],['.hidden/*.html'],['../outside.html'],['src/{oops']]
const probeFiles=['src/view.html','src/noext','node_modules/hidden/view.html','node_modules/allowed/view.html','.hidden/view.html','.git/view.html','../outside.html','public/.dot.html']
function file(path,text){path=resolve(cwd,path);mkdirSync(dirname(path),{recursive:true});writeFileSync(path,text)}
const shorten=(s)=>s.replaceAll(cwd,'<project>')
const names=(paths)=>paths.map(path=>relative(cwd,path).replaceAll('\\','/'))
try{
 for(const [path,text]of Object.entries(files))file(path,text)
 for(const [path,target]of Object.entries(links)){const abs=resolve(cwd,path);mkdirSync(dirname(abs),{recursive:true});symlinkSync(target,abs)}
 const globCases=globs.map(patterns=>{const state={skippedUnreadable:0};try{return{patterns,expected:{files:names(upstream.expandGlobs(patterns,cwd,state)),...state}}}catch(e){return{patterns,expected:{error:shorten(e.message)}}}})
 const matchers=matcherInputs.map(patterns=>{try{const matcher=upstream.createContentMatcher(patterns,cwd);return{patterns,expected:probeFiles.map(path=>({path,matched:matcher(resolve(cwd,path))}))}}catch(e){return{patterns,expected:{error:shorten(e.message)}}}})
 const themes=[['themes/main.css'],['themes/main.css','themes/reset.css'],['themes/reset.css','themes/main.css'],['themes/unclosed.css'],['themes/unclosed-unicode.css'],['themes/unicode.css'],['themes/missing.css']].map(entries=>{try{const r=theme.themeFromCss(entries.map(path=>resolve(cwd,path)));return{entries,expected:{extension:r.extension,files:names(r.files)}}}catch(e){return{entries,expected:{error:shorten(e.message)}}}})
 const discovery=['@theme {}','/* @theme */','@themeish {}','@import "tailwindcss";','@import url( "tailwindcss/theme.css");','/* @import "tailwindcss" */','@import "./tailwindcss";','@themeé {}','@import\uFEFF"tailwindcss";','@import\u0085"tailwindcss";'].map(css=>({css,expected:theme.isTailwindCss(css)}))
 const buildCases=[]
 for(const [id,opts]of Object.entries({discovery:{content:['src/**/*.html']},tokensNoCap:{tokens:'tokens.txt',css:false,safelist:'safelist.txt'},fullSkipsInputs:{full:true,css:false,tokens:'missing.txt',safelist:'missing.txt'},configTransform:{tokens:'tokens.txt',css:false,config:'config.mjs'},longWarning:{content:['long.html'],css:false},outputExcluded:{content:['self/*.html'],css:false,out:'self/tables.html'},outputAliasExcluded:{content:['outputs/*.html'],css:false,out:'outputs/tables.html'},missingSafelist:{tokens:'tokens.txt',safelist:'missing.txt',css:false},missingImport:{tokens:'tokens.txt',css:'themes/broken.css'},outputDirectory:{full:true,css:false,out:'.'}})){
  file('long.html','class="p-2" '+ '😀'.repeat(4097));file('self/tables.html','p-4');file('outputs/real.html','m-4');file('outputs/tables.html','p-4');
  const alias=resolve(cwd,'outputs/alias.html');if(!existsSync(alias))symlinkSync('tables.html',alias)
  if(id==='missingImport')file('themes/broken.css','@import "./missing.css"; @theme {}')
  try{const r=await upstream.build({cwd,out:`contract-${id}.mjs`,...opts});buildCases.push({id,options:opts,expected:{tokens:[...r.tokens].filter(s=>s.length<100).sort(),longTokens:[...r.tokens].filter(s=>s.length>=100).map(s=>s.length),scannedFiles:r.scannedFiles,cssFiles:names(r.cssFiles),groups:Object.keys(r.config.classGroups),warnings:r.warnings,sourceSha256:sha256(r.source)}})}catch(e){buildCases.push({id,options:opts,expected:{error:shorten(e.message)}})}
 }
 const config=resolve(cwd,'coalesce.mjs');file('coalesce.mjs','export default config => { globalThis.__cnBuildCount=(globalThis.__cnBuildCount??0)+1; return config }')
 const builder=upstream.createBuilder({cwd,content:['src/**/*.html'],css:false,config:'coalesce.mjs',out:'coalesce-tables.mjs'})
 const [one,two]=await Promise.all([builder.run(),builder.run()]);const coalescing={builds:globalThis.__cnBuildCount,same:one===two,lastSame:builder.last===one,changed:one.changed}
 const plugin=await load('vite');const vite=plugin.cn({css:false,content:['src/**/*.html']});vite.configResolved({root:cwd});await vite.buildStart();const output=resolve(cwd,'cn-tables.mjs');const initial=readFileSync(output,'utf8');file('src/view.html','m-4');await vite.watchChange(resolve(cwd,'src/view.html'),{event:'delete'});const deleteIgnored=readFileSync(output,'utf8')===initial;await vite.watchChange(resolve(cwd,'src/view.html'),{event:'update'});const updateChanged=readFileSync(output,'utf8')!==initial;
 const next=await load('next');let passed;const withCn=next.withCn((phase,context)=>{passed={phase,context,outputExists:existsSync(resolve(cwd,'next-tables.mjs'))};return{test:true}},{cwd,css:false,full:true,out:'next-tables.mjs'});const nextResult=await withCn('phase-production-build',{probe:7});
 // The child executes the locked CLI byte-for-byte except its built-module
 // import is redirected to the locked TS oracle; production never uses this shim.
 const cliPath=resolve(temp,'tool/bin/cn.mjs');mkdirSync(dirname(cliPath),{recursive:true})
 writeFileSync(cliPath,read('packages/cn/bin/cn.mjs').replace('"../dist/build.js"',JSON.stringify(new URL('../../../ref/cn/packages/cn/src/build.ts',new URL('./',import.meta.url)).href)))
 writeFileSync(resolve(temp,'tool/package.json'),read('packages/cn/package.json'))
 const cliCases={help:[],version:['--version'],unknownCommand:['serve'],missingValue:['build','--out'],unknownOption:['build','--oops'],helpPriority:['bad','--version','--help'],quiet:['build','--cwd','<project>','--full','--no-css','--out','cli-quiet.mjs','--quiet'],summary:['build','--cwd','<project>','--content','src/**/*.{html,tsx},docs/*.mdx','--no-css','--out','cli-summary.mjs'],fullSummary:['build','--cwd','<project>','--full','--no-css','-o','cli-full.mjs']}
 const cli=Object.entries(cliCases).map(([id,args])=>{const r=spawnSync(process.execPath,['--import',resolve(fixtureRoot,'upstream.mjs'),cliPath,...args.map(arg=>arg.replaceAll('<project>',cwd))],{encoding:'utf8'});assert.equal(r.error,undefined);return{id,args,expected:{stdout:shorten(r.stdout),stderr:shorten(r.stderr),exitCode:r.status}}})
 const artifact={schemaVersion:1,commit,files,links,globs:globCases,matchers,themes,discovery,builds:buildCases,coalescing,cli,tools:{vite:{name:vite.name,deleteIgnored,updateChanged},next:{passed,result:nextResult}}}
 const path=resolve(fixtureRoot,'build-contract-fixtures.json');if(mode==='--write')writeFileSync(path,json(artifact));else assert.equal(readFileSync(path,'utf8'),json(artifact),'cn build contract fixture drift')
 console.log(`cn build contract ${mode}: ${globCases.length} glob, ${matchers.length} matcher, ${themes.length} CSS, ${buildCases.length} build, coalescing and Vite/Next passed`)
}finally{rmSync(temp,{recursive:true,force:true})}
