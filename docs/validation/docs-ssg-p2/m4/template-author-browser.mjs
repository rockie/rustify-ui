import assert from 'node:assert/strict';
import { chromium } from '@playwright/test';
import { spawn, execFileSync } from 'node:child_process';
import { readFileSync, writeFileSync } from 'node:fs';
import { createHash } from 'node:crypto';
import path from 'node:path';
const repo=process.cwd();
const state=JSON.parse(readFileSync('target/docs-ssg/m4-template-author-state.json','utf8'));
const out=path.resolve('docs/validation/docs-ssg-p2/m4');
const lock=readFileSync(path.join(state.site,'Cargo.lock'));
const report={scope:'actual installed local Rtest/Ctest template; not GitHub install or human UAT',state,steps:[],requests:[],errors:[],expectedStreamClosures:[],csp:[],resources:[],sources:[]};
const env={...process.env,CARGO_NET_OFFLINE:'true',RUSTIFY_DOCS_CARGO:'mbx'};
let active;
async function start(command){
 const child=spawn(state.installed_bin,[command,'--port','0'],{cwd:state.site,env,detached:true});
 let log='',stdout=''; child.stderr.on('data',b=>log+=b);
 const url=await new Promise((resolve,reject)=>{child.stdout.on('data',b=>{stdout+=b;const m=/^(?:preview|dev): (\S+)\n/m.exec(stdout);if(m)resolve(m[1]);});child.on('error',reject);child.on('exit',code=>reject(new Error(`${command} exited ${code}: ${stdout}${log}`)));});
 const server={child,url,log:()=>log,command}; active=server; return server;
}
async function stop(server){
 if(!server)return;
 writeFileSync(path.join(out,`template-${server.command}-browser-r1.log`),server.log());
 if(server.child.exitCode===null&&server.child.signalCode===null){const done=new Promise(resolve=>server.child.once('exit',resolve));process.kill(-server.child.pid,'SIGTERM');await done;}
 active=undefined;
}
async function ready(page,count){await page.waitForFunction(n=>document.querySelectorAll('[data-rustify-island][data-rd-island-state="ready"]').length===n,count,{timeout:600000});await page.evaluate(()=>document.fonts.ready);assert.equal(await page.locator('[style]').count(),0);assert.equal(await page.evaluate(()=>{const ids=[...document.querySelectorAll('[id]')].map(n=>n.id);return ids.length===new Set(ids).size}),true);}
async function source(page,original){
 const snippets=await page.locator('.rd-demo-source template.rd-code-raw').evaluateAll(nodes=>nodes.map(n=>n.content.textContent));
 for(let i=1;i<snippets.length;i+=2)assert.equal(snippets[i],original);
 report.sources.push({url:page.url(),groups:snippets.length/2,file_sha256:createHash('sha256').update(original).digest('hex')});
}
async function input(page,initial,after){assert.deepEqual(await page.getByTestId('counter-value').allTextContents(),initial);await page.getByTestId('counter-increment').first().click();assert.deepEqual(await page.getByTestId('counter-value').allTextContents(),after);}
async function watch(page){
 await page.exposeBinding('__author_csp',(_,value)=>report.csp.push(value));
 await page.addInitScript(()=>document.addEventListener('securitypolicyviolation',e=>window.__author_csp(`${e.violatedDirective} ${e.blockedURI}`)));
 page.on('request',r=>report.requests.push(r.url()));
 page.on('requestfailed',r=>{const text=`${r.url()} ${r.failure()?.errorText}`;if(new URL(r.url()).pathname==='/__rustify-docs/events'&&r.failure()?.errorText==='net::ERR_ABORTED')report.expectedStreamClosures.push(text);else report.errors.push(text);});
 page.on('console',m=>{if(m.type()==='error')report.errors.push(m.text())});page.on('pageerror',e=>report.errors.push(String(e)));
 page.on('response',r=>{if(r.status()>=400)report.errors.push(`${r.url()} ${r.status()}`);if(/\/(?:app\.js|app_bg\.wasm)$/.test(r.url()))report.resources.push({url:r.url(),status:r.status(),mime:r.headers()['content-type']});});
}
const browser=await chromium.launch({executablePath:process.env.RUSTIFY_CHROMIUM||undefined}); report.chromium=browser.version();
try{
 let server=await start('preview'); let context=await browser.newContext({viewport:{width:1280,height:900},serviceWorkers:'block'});let page=await context.newPage();await watch(page);
 let release;const held=new Promise(resolve=>release=resolve);await page.route('**/assets/islands/**/app_bg.wasm',async r=>{await held;await r.continue()});
 await page.goto(server.url,{waitUntil:'domcontentloaded'});await page.waitForFunction(()=>document.querySelectorAll('[data-rustify-island]').length===2);
 await page.evaluate(()=>{window.__author_nodes=[...document.querySelectorAll('fieldset[data-rd-island-controls]')].map(field=>{const root=field.firstElementChild,nodes=[root],walk=document.createTreeWalker(root,NodeFilter.SHOW_ALL);while(walk.nextNode())nodes.push(walk.currentNode);return{field,root,nodes,ids:[...root.querySelectorAll('[id]')].map(n=>[n,n.id])}})});
 release(); await ready(page,2);
 const identity=await page.evaluate(()=>window.__author_nodes.map(t=>({sameRoot:t.field.firstElementChild===t.root,total:t.nodes.length,retained:t.nodes.filter(n=>n.isConnected&&t.root.contains(n)).length,ids:t.ids.every(([n,id])=>n.id===id)})));
 assert.equal(identity.every(t=>t.sameRoot&&t.ids&&t.total===t.retained),true);report.steps.push({name:'unmodified template original SSR node/ID survival',identity});
 const original=readFileSync(path.join(state.site,'src/demos.rs'),'utf8');await source(page,original);await input(page,['2','7'],['3','7']);
 await page.getByTestId('counter-input').first().fill('20');assert.deepEqual(await page.getByTestId('counter-value').allTextContents(),['20','7']);
 await page.screenshot({path:path.join(out,'template-baseline-desktop-r1.png')});report.steps.push({name:'unmodified two counters independent input/source',passed:true});
 await context.close();await stop(server);
 server=await start('dev');context=await browser.newContext({viewport:{width:1280,height:900},serviceWorkers:'block'});page=await context.newPage();await watch(page);await page.goto(server.url);await ready(page,2);await source(page,original);
 const readme=readFileSync(path.join(state.site,'README.md'),'utf8');const rust=[...readme.matchAll(/```rust\n([\s\S]*?)\n```/g)].map(m=>m[1]);
 assert.equal(rust.length,5);assert(rust[0].includes('double_counter_demo'));assert(rust[3].includes('examples_page'));assert(rust[4].includes('pages {'));
 const authored=original+'\n'+rust[0]+'\n\n'+rust[3]+'\n';writeFileSync(path.join(state.site,'src/demos.rs'),authored);
 let lib=readFileSync(path.join(state.site,'src/lib.rs'),'utf8');const old=/demos \{\n\s+CounterDemo => crate::demos::counter_demo,\n\s+\}/;assert(old.test(lib));lib=lib.replace(old,rust[1].replaceAll('\n','\n        '));const end=lib.indexOf('\n    }\n}',lib.indexOf('rustify_docs::site!'));assert(end>=0);lib=lib.slice(0,end)+'\n        '+rust[4].replaceAll('\n','\n        ')+lib.slice(end);writeFileSync(path.join(state.site,'src/lib.rs'),lib);
 const home=path.join(state.site,'content/index.md');writeFileSync(home,readFileSync(home,'utf8')+'\n```rustify-demo\nname: DoubleCounterDemo\n```\n');
 const nav=[...readme.matchAll(/```toml\n([\s\S]*?)\n```/g)].map(m=>m[1]).find(s=>s.includes('routes = ["/examples/"]'));assert(nav);const cfg=path.join(state.site,'rustify-docs.toml');writeFileSync(cfg,readFileSync(cfg,'utf8')+'\n'+nav+'\n');
 await ready(page,3);await source(page,authored);assert.deepEqual(await page.getByTestId('counter-value').allTextContents(),['2','7','14']);await page.getByTestId('counter-increment').nth(2).click();assert.deepEqual(await page.getByTestId('counter-value').allTextContents(),['2','7','15']);
 report.steps.push({name:'README function→registration→fence→real new handler',passed:true});
 await page.goto(new URL('examples/',server.url).href);await ready(page,2);await source(page,authored);assert.equal(await page.locator('h1').innerText(),'Examples');assert.equal(await page.locator('#paired-counters').innerText(),'Two independent counters');await input(page,['7','7'],['8','7']);assert.equal(await page.locator('footer a').innerText(),'Home');assert.equal(await page.locator('footer a').getAttribute('href'),'/');
 assert.equal(await page.locator('.rd-sidebar-wide').getByRole('link',{name:'Examples',exact:true}).count(),1);await page.screenshot({path:path.join(out,'template-readme-page-footer-r1.png')});report.steps.push({name:'README custom page/context two independent demos/footer/nav',passed:true});
 await context.close();await stop(server);await browser.close();
 for(const command of ['check','build']){const began=Date.now();let status=0,stdout='',stderr='';try{stdout=execFileSync(state.installed_bin,[command,'--format','json'],{cwd:state.site,env,encoding:'utf8',stdio:['ignore','pipe','pipe']});}catch(e){status=e.status;stdout=e.stdout?.toString()||'';stderr=e.stderr?.toString()||'';}writeFileSync(path.join(out,`template-readme-${command}-r1.json`),stdout);writeFileSync(path.join(out,`template-readme-${command}-r1.log`),JSON.stringify({command,status,offline:true,elapsed_ms:Date.now()-began})+'\n'+stderr);assert.equal(status,0,stdout+'\n'+stderr);report.steps.push({name:`authored README ${command} offline`,passed:true,elapsed_ms:Date.now()-began});}
 assert.deepEqual(readFileSync(path.join(state.site,'Cargo.lock')),lock);assert.deepEqual(report.errors,[]);assert.deepEqual(report.csp,[]);assert(report.resources.some(r=>r.url.endsWith('/app_bg.wasm')&&r.mime.split(';')[0]==='application/wasm'));report.passed=true;
}catch(e){report.passed=false;report.failure=String(e);console.error(e);process.exitCode=1;}
finally{await stop(active);await browser.close();writeFileSync(path.join(out,'template-author-browser-r1.json'),JSON.stringify(report,null,2)+'\n');console.log(JSON.stringify({passed:report.passed,failure:report.failure,steps:report.steps.map(s=>s.name)},null,2));}
