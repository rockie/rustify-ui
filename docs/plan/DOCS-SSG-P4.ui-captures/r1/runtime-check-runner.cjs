const {chromium,expect}=require('/Users/rockie/Documents/gh-rockie/rustify-ui/node_modules/@playwright/test');
const AxeBuilder=require('/Users/rockie/Documents/gh-rockie/rustify-ui/node_modules/@axe-core/playwright').default;
const fs=require('node:fs');
const root='/Users/rockie/Documents/gh-rockie/rustify-ui';
const out=root+'/docs/plan/DOCS-SSG-P4.ui-captures/r1';
const views=[{width:1280,height:900},{width:375,height:812},{width:320,height:812}];
const sbase='http://127.0.0.1:4205/rustify-ui/';
const obase='http://127.0.0.1:4199/rustify-ui/';
const report={startedAt:new Date().toISOString(),tool:{playwright:'1.63.0',axe:require(root+'/node_modules/axe-core/package.json').version},checks:[],flows:[],errors:[]};
const state=(p,s)=>expect(p.locator('dialog[data-rd-search]')).toHaveAttribute('data-state',s);
const input=p=>p.locator('[data-rd-search-input]');
let browser;
function save(){fs.writeFileSync(out+'/runtime-checks.json',JSON.stringify(report,null,2));}
async function fresh(base,route,vp,theme,js=true){
 const c=await browser.newContext({viewport:vp,deviceScaleFactor:1,colorScheme:theme,reducedMotion:'reduce',javaScriptEnabled:js});
 const p=await c.newPage();p.__js=js;
 p.on('pageerror',e=>report.errors.push({url:p.url(),type:'pageerror',text:String(e)}));
 await p.goto(new URL(route,base).href,{waitUntil:'networkidle'});
 if(js)await p.locator(`[data-rd-mode="${theme}"]`).click();return {p,c};
}
async function check(p,label,axe=false){
 const facts=await p.evaluate(()=>{
  const d=document.querySelector('dialog[data-rd-search]');const ds=d&&getComputedStyle(d);const a=document.activeElement;const ac=getComputedStyle(a);
  const rect=d?.getBoundingClientRect();const note=document.querySelector('.rd-untranslated');const ns=note&&getComputedStyle(note);
  const list=d?.querySelector('[data-rd-search-results]');
  return {url:location.href,viewport:{width:innerWidth,height:innerHeight,dpr:devicePixelRatio},htmlLang:document.documentElement.lang,theme:document.documentElement.dataset.theme,pageOverflow:Math.max(document.documentElement.scrollWidth,document.body.scrollWidth)-document.documentElement.clientWidth,scrollX,dialog:d?.open?{x:rect.x,y:rect.y,width:rect.width,height:rect.height,padding:ds.padding,maxWidth:ds.maxWidth,maxHeight:ds.maxHeight,bg:ds.backgroundColor,foreground:ds.color,border:ds.borderColor,radius:ds.borderRadius,rootFont:getComputedStyle(document.documentElement).fontSize,optionCount:d.querySelectorAll('[role=option]').length,selection:d.querySelector('[aria-selected=true]')?.id,count:d.querySelector('[data-rd-search-count]')?.textContent,list:{clientHeight:list.clientHeight,scrollHeight:list.scrollHeight,scrollTop:list.scrollTop,overflowY:getComputedStyle(list).overflowY,maxHeight:getComputedStyle(list).maxHeight}}:null,focus:{tag:a.tagName,id:a.id,text:(a.textContent||'').slice(0,60),outline:ac.outline,visible:a.matches(':focus-visible')},note:note?{text:note.textContent,bg:ns.backgroundColor,pageBg:getComputedStyle(document.body).backgroundColor,codeBg:getComputedStyle(document.documentElement).getPropertyValue('--rd-code-bg'),font:ns.fontSize,padding:ns.padding,lang:note.lang}:null,sourceLangs:[...document.querySelectorAll('.rd-search-title,.rd-search-path,.rd-search-excerpt,.rd-search-source')].slice(0,12).map(e=>({class:e.className,lang:e.lang,text:e.textContent.slice(0,100)})),scrollRegions:[...document.querySelectorAll('pre,table')].slice(0,8).map(e=>({tag:e.tagName,clientWidth:e.clientWidth,scrollWidth:e.scrollWidth,overflowX:getComputedStyle(e).overflowX})),motion:{reduce:matchMedia('(prefers-reduced-motion: reduce)').matches,scrollBehavior:getComputedStyle(document.documentElement).scrollBehavior}};
 });
 let scan=null;if(axe){const a=await new AxeBuilder({page:p}).withTags(['wcag2a','wcag2aa','wcag21a','wcag21aa','wcag22aa']).analyze();scan={violations:a.violations,incomplete:a.incomplete,passes:a.passes.length};}
 report.checks.push({label,at:new Date().toISOString(),facts,scan});save();console.log(JSON.stringify({label,overflow:facts.pageOverflow,violations:scan?.violations.length??null}));return facts;
}
async function main(){
 browser=await chromium.launch({headless:true});report.tool.browser=browser.version();save();
 for(const vp of views)for(const theme of ['light','dark']){
  const {p,c}=await fresh(sbase,'page-0001/',vp,theme);await p.locator('[data-rd-search-open]').click();await state(p,'idle');await check(p,`U1-idle-${vp.width}-${theme}`);
  await input(p).fill('ThemeScope');await state(p,'results');await check(p,`U1-results-${vp.width}-${theme}`,true);
  await input(p).fill('AbsolutelyNoMatchingWordHere');await state(p,'empty');await check(p,`U1-empty-${vp.width}-${theme}`);await c.close();
 }
 for(const [vp,theme]of [[views[0],'dark'],[views[1],'light']])for(const kind of ['module','index']){
  const {p,c}=await fresh(sbase,'page-0001/',vp,theme);let fail=true;
  await c.route(kind==='module'?/\/assets\/search\.[a-f0-9]+\.js(?:\?|$)/:/\/assets\/search\.en\.[a-f0-9]+\.json$/,async r=>{if(!fail)return r.continue();fail=false;return kind==='module'?r.fulfill({status:503,contentType:'text/javascript',body:'unavailable'}):r.fulfill({json:{schema_version:99,pages:[]}});});
  await p.locator('[data-rd-search-open]').click();await input(p).fill('CustomOnlyToken');await state(p,'error');await check(p,`U2-${kind}-error-${vp.width}-${theme}`,kind==='module');
  await p.keyboard.press('Tab');const retryFocus=await p.locator('[data-rd-search-retry]').evaluate(e=>document.activeElement===e);await p.keyboard.press('Enter');await state(p,'results');const facts=await check(p,`U2-${kind}-retry-${vp.width}-${theme}`);report.flows.push({label:`keyboard-retry-${kind}-${vp.width}-${theme}`,retryFocus,resultFocus:facts.focus,query:await input(p).inputValue()});await c.close();
 }
 for(const vp of views)for(const theme of ['light','dark'])for(const route of ['','zh/','zh/quickstart/']){
  const {p,c}=await fresh(obase,route,vp,theme);await check(p,`U3-${route||'en-home'}-${vp.width}-${theme}`,true);await c.close();
 }
 for(const [vp,theme]of [[views[0],'light'],[views[2],'dark']]){
  const {p,c}=await fresh(obase,'zh/architecture/',vp,theme);await check(p,`U3-fallback-${vp.width}-${theme}`,true);await c.close();
  const {p:q,c:d}=await fresh(obase,'zh/__ui_review_missing__/deep/',vp,theme);await check(q,`U3-404-${vp.width}-${theme}`,true);await q.locator('[data-rd-search-open]').click();await input(q).fill('快速开始');await state(q,'results');await q.keyboard.press('Enter');await q.waitForURL('**/zh/quickstart/**');await check(q,`U3-404-destination-${vp.width}-${theme}`);await d.close();
 }
 for(const route of ['zh/quickstart/','zh/__ui_review_missing__/deep/']){
  const {p,c}=await fresh(obase,route,views[1],'light',false);await check(p,`U3-noscript-${route}`);const disabled=await p.locator('[data-rd-search-open]').isDisabled();const noScript=await p.locator('noscript').allTextContents();report.flows.push({label:`noscript-${route}`,disabled,noScript});await c.close();
 }
 {
  const {p,c}=await fresh(sbase,'',views[0],'light');await p.locator('.rd-brand').focus();await p.keyboard.press('Control+k');await input(p).fill('SearchCorpusToken');await state(p,'results');await p.keyboard.press('ArrowDown');await p.keyboard.press('Tab');const before=await check(p,'UI-01-before-keyboard-more');await p.keyboard.press('Enter');await expect(p.locator('[role=option]')).toHaveCount(40);await p.screenshot({path:out+'/M1-show-more-after-1280-light.png'});const after=await check(p,'UI-01-after-keyboard-more');await p.keyboard.press('Enter');const secondEnter=await check(p,'UI-01-second-enter');report.flows.push({label:'UI-01-show-more-focus',before,after,secondEnter,screenshot:'M1-show-more-after-1280-light.png'});await c.close();
 }
 {
  const {p,c}=await fresh(sbase,'zh/page-0001/',views[1],'dark');await p.locator('[data-rd-search-open]').click();await input(p).fill('FallbackOnlyToken');await state(p,'results');const source=await check(p,'U1-fallback-source-lang');await p.keyboard.press('Enter');await p.waitForURL('**/zh/page-0051/');const destination=await check(p,'U1-fallback-destination');report.flows.push({label:'zh-fallback-navigation',source,destination});await c.close();
 }
 report.finishedAt=new Date().toISOString();save();await browser.close();
}
main().catch(async e=>{report.errors.push({type:'harness',text:String(e)});save();console.error(e);if(browser)await browser.close();process.exitCode=1;});
