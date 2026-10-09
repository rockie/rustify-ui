import {createRequire} from 'node:module';
import {resolve} from 'node:path';
import {writeFileSync} from 'node:fs';
const require = createRequire(resolve('package.json'));
const {chromium}=require('playwright');
const browser=await chromium.launch({channel:'chrome',headless:true});
try {
const page=await browser.newPage(); const errors=[],requests=[],csp=[];
page.on('pageerror', e=>errors.push(e.message));
page.on('console', m=>{if(m.type()==='error')errors.push(m.text())});
page.on('requestfailed', r=>requests.push(r.url()));
await page.addInitScript(()=>{window.__probeCsp=[];document.addEventListener('securitypolicyviolation', e=>window.__probeCsp.push(e.violatedDirective))});
await page.goto(process.argv[2]); await page.locator('#status[data-status="ready"]').waitFor();
const result=await page.evaluate(async()=>{
 const api=window.__utility_lab;
 const mount=async(id,css)=>{const root=document.createElement('div');root.id=id;document.querySelector('main').append(root);api.mount(id,10000,css);await api.flush(id);return root;};
 const read=e=>({name:getComputedStyle(e).animationName,timing:getComputedStyle(e).animationTimingFunction,count:e.getAnimations().length});
 const keyword=await mount('keyword-motion',':root { --ease: linear; } @keyframes card { to { opacity: 0; } } @keyframes linear { to { opacity: 0.5; } } .main-motion { animation: card 1s var(--ease) infinite; } .other-motion { animation-name: linear; animation-duration: 2s; } @tailwind utilities;');
 const a=document.createElement('div');a.className='main-motion';const b=document.createElement('div');b.className='other-motion';keyword.append(a,b);await api.ensure('keyword-motion',[]);
 const motionBefore=read(a),motionOther=read(b),motionCss=api.report('keyword-motion').css;
 const reference=document.createElement('div');reference.id='motion-reference';const ref=document.createElement('div');ref.className='main-motion';reference.append(ref);document.querySelector('main').append(reference);
 const sheet=new CSSStyleSheet();sheet.replaceSync('@scope (#motion-reference) { :scope { --ease: linear; } .main-motion { animation: card 1s var(--ease) infinite; } } @keyframes card { to { opacity: 0; } } @keyframes linear { to { opacity: 0.5; } }');document.adoptedStyleSheets=[...document.adoptedStyleSheets,sheet];
 const motionReference=read(ref);document.adoptedStyleSheets=document.adoptedStyleSheets.filter(s=>s!==sheet);reference.remove();
 const id='rebuild-recovery';const root=await mount(id,'@tailwind utilities;');await api.ensure(id,['w-[246px]']);
 const retainedBefore=api.retained(id),reportBefore=api.report(id);
 const original=CSSStyleSheet.prototype.replaceSync;CSSStyleSheet.prototype.replaceSync=function(){throw new DOMException('r2 injected install failure','SyntaxError')};
 let installFailure;try{await api.rebuild(id,['w-[247px]'])}catch(e){installFailure=JSON.parse(String(e))}finally{CSSStyleSheet.prototype.replaceSync=original};
 const retainedFailure=api.retained(id),reportFailure=api.report(id);
 const rebuilt=await api.rebuild(id,['w-[247px]']);const retainedAfter=api.retained(id);const replay=await api.retry(id);
 const oldPost=Worker.prototype.postMessage;Worker.prototype.postMessage=function(msg,...args){if(msg.operation==='ack')return;return oldPost.call(this,msg,...args)};
 let ackFailure;try{await api.rebuild(id,['w-[248px]'])}catch(e){ackFailure=JSON.parse(String(e))}finally{Worker.prototype.postMessage=oldPost};
 const reportAck=api.report(id);const ackRetry=await api.retry(id);
 api.dispose(id);api.dispose('keyword-motion');
 return {keyword:{motionBefore,motionOther,motionReference,motionCss},recovery:{retainedBefore,reportBefore,installFailure,retainedFailure,reportFailure,rebuilt,retainedAfter,replay,ackFailure,reportAck,ackRetry}};
});
csp.push(...await page.evaluate(()=>window.__probeCsp));
const output={...result,errors,requests,csp};writeFileSync(process.argv[3],JSON.stringify(output,null,2)+'\n');
console.log(JSON.stringify({keyword:result.keyword,recovery:{installFailure:result.recovery.installFailure,preserved:result.recovery.reportBefore.css===result.recovery.reportFailure.css,rebuildAdmitted:result.recovery.rebuilt.admitted.filter(c=>c.includes('24')),retryAdmitted:result.recovery.replay.admitted.filter(c=>c.includes('24')),ackFailure:result.recovery.ackFailure,ackRetry:result.recovery.ackRetry.admitted.filter(c=>c.includes('24'))},errors,requests,csp},null,2));
}finally{await browser.close()}
