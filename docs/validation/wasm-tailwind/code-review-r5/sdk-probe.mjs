import {createRequire} from 'node:module';
import {resolve} from 'node:path';
import {readFileSync,writeFileSync} from 'node:fs';
import {spawn} from 'node:child_process';
import assert from 'node:assert/strict';
const require=createRequire(resolve('package.json'));
const {chromium}=require('playwright');
const directory=process.argv[2];
const cases=JSON.parse(readFileSync(directory+'/candidate-cases.json','utf8')).filter(x=>['outside-override','multi-inherit','alias-mixed','external-disable-name','external-disable-cyclic'].includes(x.id));
const browser=await chromium.launch({channel:'chrome',headless:true});
try {
 for(const [name,base] of [['root','/'],['deep','/tools/utility-lab/']]) {
  const args=['serve','--example','utility-lab','--release','--port','0'];
  if(base!=='/')args.push('--base',base);
  const server=spawn('target/debug/xtask',args);let logs='';
  const url=await new Promise((ok,no)=>{
   const timer=setTimeout(()=>no(new Error('serve timeout')),10000);
   server.once('error',e=>{clearTimeout(timer);no(e)});
   server.stdout.on('data',data=>{logs+=data.toString();const m=logs.match(/at (http:\/\/[^\s]+)/);if(m){clearTimeout(timer);ok(m[1]);}});
   server.stderr.on('data',data=>{logs+=data.toString()});
  });
  const page=await browser.newPage();const errors=[],requests=[],badResponses=[];
  try {
   page.on('pageerror',e=>errors.push(e.message));page.on('console',m=>{if(m.type()==='error')errors.push(m.text())});
   page.on('requestfailed',r=>requests.push({url:r.url(),error:r.failure()}));
   page.on('response',r=>{if(r.status()>=400)badResponses.push({url:r.url(),status:r.status()})});
   await page.addInitScript(()=>{window.__r5Csp=[];document.addEventListener('securitypolicyviolation',e=>window.__r5Csp.push(e.violatedDirective))});
   await page.goto(url);await page.locator('#status[data-status="ready"]').waitFor();
   const results=await page.evaluate(async cases=>{
    const api=window.__utility_lab,id='r5-direct';const root=document.createElement('div');root.id=id;document.querySelector('main').append(root);
    api.mount(id,10000,':root { --v: linear; --a: var(--v); } @keyframes linear { to { opacity: .3; } } .motion { animation-name: var(--v); animation-duration: 10s; } .alias { animation-name: var(--a); animation-duration: 10s; } @tailwind utilities;');
    const frame=document.getElementById(id+'-frame');const motion=document.createElement('div');motion.className='motion';frame.append(motion);
    const alias=document.createElement('div');alias.className='alias';frame.append(alias);
    const inline=document.createElement('div');inline.style.animationName='var(--v)';inline.style.animationDuration='10s';frame.append(inline);
    const plain=document.createElement('div');plain.style.transitionTimingFunction='var(--v)';frame.append(plain);
    const report=await api.ensure(id,[]);await api.flush(id);const baseline=api.report(id);const owner=root.dataset.rustifyStyleOwner;
    const read=e=>{const s=getComputedStyle(e);return {variable:s.getPropertyValue('--v').trim(),name:s.animationName,timing:s.transitionTimingFunction,count:e.getAnimations().length}};
    const before=[motion,alias,inline,plain].map(read);
    inline.style.setProperty('--v','none');const local=[motion,alias,inline].map(read);inline.style.removeProperty('--v');
    root.style.setProperty('--v','none');const inherited=[motion,alias,inline].map(read);root.style.removeProperty('--v');
    const denied=[];
    for(const c of cases){let error=null;try{await api.replace(id,c.css+' @tailwind utilities;')}catch(e){error=JSON.parse(String(e))}const after=api.report(id);denied.push({id:c.id,error,revision:after.revision,css:after.css,animation:read(motion)})}
    const final=api.report(id);api.dispose(id);root.remove();
    return {owner,before,local,inherited,denied,baseline,final};
   },cases);
   const csp=await page.evaluate(()=>window.__r5Csp);
   const manifests=await page.evaluate(async()=>({main:await(await fetch('build-manifest.json')).json(),worker:await(await fetch('utility/build-manifest.json')).json()}));
   writeFileSync(directory+'/sdk-'+name+'.json',JSON.stringify({url,browser:browser.version(),results,manifests,errors,requests,badResponses,csp},null,2)+'\n');
   const n='rustify-'+results.owner+'-linear';assert.equal(results.before[0].name,n);assert.equal(results.before[1].name,n);assert.equal(results.before[2].name,n);assert.equal(results.before[3].variable,n);assert.equal(results.before[3].timing,'ease');
   assert.deepEqual(results.before.map(x=>x.count),[1,1,1,0]);assert.deepEqual(results.local.map(x=>x.count),[1,1,0]);assert.deepEqual(results.inherited.map(x=>x.count),[0,0,0]);
   for(const d of results.denied){assert.equal(d.error.kind,'compile');assert.equal(d.error.error.kind,'unsupported_feature');assert.equal(d.revision,results.baseline.revision);assert.equal(d.css,results.baseline.css);assert.equal(d.animation.count,1)}
   assert.deepEqual(errors,[]);assert.deepEqual(requests,[]);assert.deepEqual(badResponses,[]);assert.deepEqual(csp,[]);
   console.log(name+' SDK: inline/name/alias overrides and all 5 rejection rollbacks passed; Chrome '+browser.version());
  } finally {await page.close();server.kill('SIGTERM');writeFileSync(directory+'/serve-'+name+'.log',logs)}
 }
}finally{await browser.close()}
