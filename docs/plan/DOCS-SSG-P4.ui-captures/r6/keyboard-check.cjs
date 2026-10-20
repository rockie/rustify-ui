const {chromium,expect}=require('/Users/rockie/Documents/gh-rockie/rustify-ui/node_modules/@playwright/test');
const fs=require('node:fs');
const out=__dirname;
const records=[];
const base='http://127.0.0.1:4199/rustify-ui/';
let browser;
async function observed(p,label,shot=false){
 const facts=await p.evaluate(()=>{const a=document.activeElement,s=getComputedStyle(a),d=document.querySelector('dialog[data-rd-search]');return {url:location.href,viewport:{width:innerWidth,height:innerHeight,dpr:devicePixelRatio},overflow:Math.max(document.documentElement.scrollWidth,document.body.scrollWidth)-innerWidth,focus:{tag:a.tagName,id:a.id,text:(a.textContent||a.value||'').slice(0,80),visible:a.matches(':focus-visible'),outline:s.outline},dialog:d.open,state:d.dataset.state,query:d.querySelector('input').value,count:d.querySelector('[data-rd-search-count]').textContent,options:d.querySelectorAll('[role=option]').length};});
 const file=shot?`M2-${label}.png`:null;if(file)await p.screenshot({path:out+'/'+file});
 records.push({label,at:new Date().toISOString(),facts,file});fs.writeFileSync(out+'/keyboard-checks.json',JSON.stringify(records,null,2));return facts;
}
async function main(){
 browser=await chromium.launch();
 const c=await browser.newContext({viewport:{width:375,height:812},deviceScaleFactor:1,reducedMotion:'reduce'}),p=await c.newPage();
 await p.goto(base+'zh/quickstart/');await p.keyboard.press('Tab');await observed(p,'skip-focus',true);await p.keyboard.press('Enter');await observed(p,'skip-destination');
 await p.locator('.rd-language-switch summary').focus();await p.keyboard.press('Enter');await observed(p,'language-open',true);await p.keyboard.press('Escape');await observed(p,'language-escape',true);
 await p.locator('[data-rd-search-open]').focus();await p.keyboard.press('Enter');await p.locator('[data-rd-search-input]').fill('ThemeScope');await expect(p.locator('dialog[data-rd-search]')).toHaveAttribute('data-state','results');
 for(let i=0;i<4;i++){await p.keyboard.press('Tab');await observed(p,'tab-cycle-'+i,i===0);}await p.keyboard.press('Shift+Tab');await observed(p,'reverse-tab');
 await p.keyboard.press('Escape');await observed(p,'search-escape',true);
 await p.keyboard.press('Enter');await expect(p.locator('dialog[data-rd-search]')).toHaveAttribute('data-state','results');await observed(p,'reopen-preserved',true);
 await p.keyboard.press('Shift+Tab');await observed(p,'close-focus',true);await p.keyboard.press('Enter');await observed(p,'close-return',true);
 await p.goto(base);await p.getByRole('main').getByRole('link',{name:'Quick start',exact:true}).click();await p.waitForURL('**/quickstart/');await observed(p,'home-primary');
 await p.goto(base);await p.getByRole('main').getByRole('link',{name:'Components',exact:true}).click();await p.waitForURL('**/components/**');await observed(p,'home-secondary');
 await p.goto(base+'zh/quickstart/');await p.locator('.rd-language-switch summary').click();await p.locator('.rd-language-switch').getByRole('link',{name:'English',exact:true}).click();await p.waitForURL('**/quickstart/');await observed(p,'language-link');
 await p.goto(base+'zh/quickstart/');await p.locator('.rd-toc-narrow summary').click();const section=p.locator('.rd-toc-narrow a').first();const href=await section.getAttribute('href');await section.click();await observed(p,'section-anchor',true);records.push({label:'section-href',href});
 const pre=p.locator('pre').first();await pre.scrollIntoViewIfNeeded();await pre.hover();const initial=await pre.evaluate(e=>({clientWidth:e.clientWidth,scrollWidth:e.scrollWidth,left:e.scrollLeft}));await p.mouse.wheel(900,0);await expect.poll(()=>pre.evaluate(e=>e.scrollLeft)).toBeGreaterThan(0);await observed(p,'code-horizontal',true);records.push({label:'code-scroll',initial,after:await pre.evaluate(e=>({left:e.scrollLeft,clientWidth:e.clientWidth,scrollWidth:e.scrollWidth}))});
 await c.close();
 const nc=await browser.newContext({viewport:{width:375,height:812},javaScriptEnabled:false}),n=await nc.newPage();await n.goto(base+'zh/quickstart/');await n.locator('.rd-sidebar-narrow summary').click();await n.locator('.rd-sidebar-narrow a[href$="/zh/architecture/"]').click();await n.waitForURL('**/zh/architecture/');await observed(n,'noscript-navigation',true);await nc.close();
 const sc=await browser.newContext({viewport:{width:375,height:812}}),s=await sc.newPage();await s.goto('http://127.0.0.1:4205/rustify-ui/page-0001/');let failed=false;
 await sc.route(/\/assets\/search\.[a-f0-9]+\.js(?:\?|$)/,r=>{if(failed)return r.continue();failed=true;return r.fulfill({status:503,contentType:'text/javascript',body:'unavailable'});});
 await s.locator('[data-rd-search-open]').click();await s.locator('[data-rd-search-input]').fill('CustomOnlyToken');await expect(s.locator('dialog[data-rd-search]')).toHaveAttribute('data-state','error');await s.locator('[data-rd-search-input]').fill('ReachableCustomToken');await s.keyboard.press('Tab');await observed(s,'edited-error-retry-focus',true);await s.keyboard.press('Enter');await expect(s.locator('dialog[data-rd-search]')).toHaveAttribute('data-state','results');await observed(s,'edited-error-retry-result',true);await sc.close();
 fs.writeFileSync(out+'/keyboard-checks.json',JSON.stringify(records,null,2));await browser.close();
}
main().catch(async e=>{records.push({type:'harness-error',error:String(e)});fs.writeFileSync(out+'/keyboard-checks.json',JSON.stringify(records,null,2));console.error(e);if(browser)await browser.close();process.exitCode=1;});
