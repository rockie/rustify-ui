import {createRequire} from 'node:module';
import {resolve} from 'node:path';
import {writeFileSync} from 'node:fs';
const require=createRequire(resolve('package.json'));
const {chromium}=require('playwright');
const url=process.argv[2],output=process.argv[3];
const cases=[
 {id:'fixed',css:':root { --ease: linear; } @keyframes card { to { opacity: 0; } } @keyframes linear { to { opacity: .5; } } .motion { animation: card 1s var(--ease) infinite; } @tailwind utilities;'},
 {id:'var-before',css:':root { --x: linear; } @keyframes ease { to { opacity: .3; } } .motion { animation: var(--x) 10s ease infinite; } @tailwind utilities;'},
 {id:'var-name',css:':root { --name: linear; } @keyframes linear { to { opacity: .3; } } .motion { animation: 10s ease var(--name) infinite; } @tailwind utilities;'},
 {id:'var-fallback',css:':root { --x: ease; } @keyframes linear { to { opacity: .3; } } .motion { animation: var(--x, linear) 10s linear infinite; } @tailwind utilities;'},
 {id:'time-calc',css:'@keyframes card { to { opacity: .3; } } .motion { animation: calc(10s) linear card infinite; } @tailwind utilities;'},
 {id:'time-min',css:'@keyframes card { to { opacity: .3; } } .motion { animation: min(10s, 12s) linear card infinite; } @tailwind utilities;'},
 {id:'iteration',css:'@keyframes infinite { to { opacity: .3; } } .motion { animation: 2 10s infinite linear; } @tailwind utilities;'},
 {id:'iteration-before-name',css:'@keyframes infinite { to { opacity: .3; } } .motion { animation: 10s infinite 2 linear; } @tailwind utilities;'},
 {id:'shared-role',css:':root { --ease: linear; } @keyframes card { to { opacity: 0; } } @keyframes linear { to { opacity: .5; } } .motion { animation: card 10s var(--ease) infinite; } .other { animation-name: var(--ease); animation-duration: 10s; } @tailwind utilities;'},
];
const browser=await chromium.launch({channel:'chrome',headless:true});
try {
const page=await browser.newPage();const errors=[],requests=[];
page.on('pageerror',e=>errors.push(e.message));page.on('console',m=>{if(m.type()==='error')errors.push(m.text())});page.on('requestfailed',r=>requests.push(r.url()));
await page.addInitScript(()=>{window.__r3Csp=[];document.addEventListener('securitypolicyviolation',e=>window.__r3Csp.push(e.violatedDirective))});
await page.goto(url);await page.locator('#status[data-status="ready"]').waitFor();
const result=await page.evaluate(async cases=>{
 const api=window.__utility_lab;
 const read=e=>{const s=getComputedStyle(e);const anim=e.getAnimations();return {name:s.animationName,timing:s.animationTimingFunction,duration:s.animationDuration,iteration:s.animationIterationCount,count:anim.length,lastOpacity:anim[0]?.effect.getKeyframes().at(-1)?.opacity};};
 const rows=[];
 for(const c of cases){
  const id='r3-'+c.id;const root=document.createElement('div');root.id=id;document.querySelector('main').append(root);
  api.mount(id,10000,c.css);await api.flush(id);
  const frame=document.getElementById(id+'-frame');const element=document.createElement('div');element.className='motion';frame.append(element);
  const report=await api.ensure(id,[]);const owner=root.dataset.rustifyStyleOwner;const actual=read(element);const other=document.createElement('div');other.className='other';frame.append(other);const actualOther=c.id==='shared-role'?read(other):null;
  api.dispose(id);root.remove();
  const ref=document.createElement('div');ref.id=id+'-reference';const child=document.createElement('div');child.className='motion';ref.append(child);document.querySelector('main').append(ref);
  const sheet=new CSSStyleSheet();sheet.replaceSync(c.css.replaceAll(':root', '#'+ref.id).replace('@tailwind utilities;',''));
  document.adoptedStyleSheets=[...document.adoptedStyleSheets,sheet];const expected=read(child);const refOther=document.createElement('div');refOther.className='other';ref.append(refOther);const referenceOther=c.id==='shared-role'?read(refOther):null;
  document.adoptedStyleSheets=document.adoptedStyleSheets.filter(s=>s!==sheet);ref.remove();
  rows.push({...c,owner,actual,actualOther,reference:expected,referenceOther,scopedCss:report.css});
 }
 return rows;
},cases);
const csp=await page.evaluate(()=>window.__r3Csp);const manifests=await page.evaluate(async()=>({main:await(await fetch('build-manifest.json')).json(),worker:await(await fetch('utility/build-manifest.json')).json()}));
writeFileSync(output,JSON.stringify({url,cases:result,manifests,errors,requests,csp},null,2)+'\n');
console.log(JSON.stringify({url,cases:result.map(({id,actual,reference})=>({id,actual,reference})),errors,requests,csp},null,2));
}finally{await browser.close();}
