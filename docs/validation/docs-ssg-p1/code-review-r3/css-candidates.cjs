// Check reviewer CSS candidates against both fixed binaries and real browser.
const assert = require('node:assert/strict'); const fs = require('node:fs'); const path = require('node:path');
const { spawn, spawnSync } = require('node:child_process'); const { chromium } = require(path.resolve('node_modules/playwright'));
const work = process.argv[2]; const binaries = { target: path.resolve('target/docs-cli/rustify-docs'), baseline: path.join(work, 'baseline-target/docs-cli/rustify-docs') };
const svg = '<svg xmlns="http://www.w3.org/2000/svg" width="40" height="40"><rect width="40" height="40" fill="red"/></svg>';
const cases = { 'uppercase-underscore': {base:'/', class:"[background-image:URL('/assets/bg_one.svg')]", file:'static/assets/bg one.svg'}, 'entity-paren-equals': {base:'/docs/',class:'[background-image:url&#40=/bg.svg)]',file:'content/=/bg.svg'}, 'entity-semicolon-control': {base:'/docs/',class:'[background-image:url&#40;=/bg.svg)]',file:'content/=/bg.svg'} };
async function preview(binary, root) {
 const child=spawn(binary,['preview','--dir',path.join(root,'dist'),'--port','0']); let log='';
 const url=await new Promise((resolve,reject)=>{ const timer=setTimeout(()=>{child.kill();reject(Error(log));},15000); const read=b=>{log+=b.toString();const m=log.match(/http:\/\/127\.0\.0\.1:\d+\/[^\s]*/);if(m){clearTimeout(timer);resolve(m[0]);}}; child.stdout.on('data',read);child.stderr.on('data',read);child.on('error',reject);child.on('exit',code=>{clearTimeout(timer);reject(Error(`server ${code} ${log}`));}); });
 return {url, async close(){const exit=new Promise(r=>child.once('exit',r));child.kill();await exit;}};
}
(async()=>{const browser=await chromium.launch({executablePath:process.env.RUSTIFY_CHROMIUM||undefined});const result={browser:browser.version(),cases:[]};try{
 for(const [name,test] of Object.entries(cases))for(const [version,binary]of Object.entries(binaries)){
  const root=path.join(work,'candidate-fixtures',`${name}-${version}`); fs.mkdirSync(path.join(root,'content'),{recursive:true});fs.mkdirSync(path.dirname(path.join(root,test.file)),{recursive:true});fs.writeFileSync(path.join(root,test.file),svg);
  fs.writeFileSync(path.join(root,'rustify-docs.toml'),`schema_version=1\n[site]\nname="CSS"\nurl="https://example.net${test.base}"\nbase="${test.base}"\n`);
  const source=`---\ndescription: CSS fixture\n---\n# CSS\n\n<div class="h-10 w-10 ${test.class}">Hello</div>\n`;fs.writeFileSync(path.join(root,'content/index.md'),source);
  const done=spawnSync(binary,['build','--config',path.join(root,'rustify-docs.toml'),'--format','json'],{encoding:'utf8'});const item={name,version,exit:done.status,report:JSON.parse(done.stdout)};
  if(done.status===0){const host=await preview(binary,root);const page=await browser.newPage();const errors=[];page.on('pageerror',e=>errors.push(e.message));page.on('response',r=>{if(r.status()>=400)errors.push(`${r.status()} ${r.url()}`);});try{await page.goto(host.url);item.entry=host.url;item.actual=await page.locator('article div').evaluate(e=>({class:e.className,background:getComputedStyle(e).backgroundImage}));item.decodedSource=await page.evaluate(source=>{const doc=new DOMParser().parseFromString(source,'text/html');return doc.querySelector('div').className;},source);item.errors=errors;if(name==='uppercase-underscore'&&version==='baseline')assert(item.actual.background.includes('bg%20one.svg'));if(name==='entity-paren-equals'&&version==='target'){assert.equal(item.actual.background,'none');assert(!item.actual.class.includes('url('));}if(name==='entity-semicolon-control'&&version==='target')assert.notEqual(item.actual.background,'none');await page.screenshot({path:path.join(work,`${name}-${version}.png`)});}finally{await page.close();await host.close();}}
  if(name==='uppercase-underscore'&&version==='target'){assert.equal(done.status,1);assert(item.report.diagnostics.some(d=>d.rule==='missing-image'));}result.cases.push(item);
 }
}finally{await browser.close();}fs.writeFileSync(path.join(work,'css-candidate-results.json'),JSON.stringify(result,null,2)+'\n');console.log(JSON.stringify(result,null,2));})().catch(e=>{console.error(e);process.exitCode=1;});
