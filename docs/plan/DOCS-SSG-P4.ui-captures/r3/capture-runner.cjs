const { chromium, expect } = require('/Users/rockie/Documents/gh-rockie/rustify-ui/node_modules/@playwright/test');
const fs = require('node:fs');
const path = require('node:path');
const cp = require('node:child_process');
const crypto = require('node:crypto');
const root = '/Users/rockie/Documents/gh-rockie/rustify-ui';
const out = path.join(root, 'docs/plan/DOCS-SSG-P4.ui-captures/r3');
fs.mkdirSync(out, {recursive:true});
const hash = x => crypto.createHash('sha256').update(x).digest('hex');
const targetFile = path.join(root,'docs/plan/DOCS-SSG-P4.ui-target.md');
const target = fs.readFileSync(targetFile,'utf8').replace(/\r\n/g,'\n');
const sections = ['单元','规范要点'].map(name => {
  const start=target.indexOf(`## ${name}\n`); let end=target.indexOf('\n## ', start+4);
  if(end<0)end=target.length;
  return target.slice(start,end);
});
const normativeBytes = sections.map(x=>`${Buffer.byteLength(x)}:${x}`).join('');
const baseline = {
  head:cp.execFileSync('git',['rev-parse','HEAD'],{cwd:root,encoding:'utf8'}).trim(),
  branch:cp.execFileSync('git',['branch','--show-current'],{cwd:root,encoding:'utf8'}).trim(),
  dirtyDiffSha256:hash(cp.execFileSync('git',['diff','--binary','HEAD'],{cwd:root})),
  status:cp.execFileSync('git',['status','--short'],{cwd:root,encoding:'utf8'}),
  targetFileSha256:hash(fs.readFileSync(targetFile)),
  targetFingerprintAlgorithm:'manual-sha256-normative-sections-v1',
  targetFingerprint:hash(normativeBytes),
  fingerprintMethod:'unique level-2 单元 and 规范要点 sections, including headings; LF; each UTF-8 segment prefixed by decimal byte length and colon, concatenated in that order; no local attachments in these sections. Manual fingerprint, not a claim of Phoenix engine compatibility.',
  candidate:'1bbc54b096c6656e9ad2ad48b5acd43cb7ee844a',
  framework:'740610db164b3f396b0fcc551f5a5ef8f15da716',
  cli:cp.execFileSync(path.join(root,'target/docs-ssg/p4/install-1bbc54b0/bin/rustify-docs'),['--build-info'],{encoding:'utf8'}),
  productDiffFromCandidate:cp.execFileSync('git',['diff','--name-only','1bbc54b096c6656e9ad2ad48b5acd43cb7ee844a','--','crates','site','rustify-docs.toml','docs/zh','tests/docs-ssg','xtask','README.md','docs/quickstart.md','docs/architecture.md','docs/navigation.md','docs/workspace.md','docs/forms.md','docs/i18n.md','docs/data.md','docs/themes.md','docs/utility.md','docs/compatibility.md','docs/components.md','docs/vellum.md'],{cwd:root,encoding:'utf8'})
};
const views=[{width:1280,height:900},{width:375,height:812},{width:320,height:812}];
const sbase='http://127.0.0.1:4205/rustify-ui/';
const obase='http://127.0.0.1:4199/rustify-ui/';
const manifests={U1:[],U2:[],U3:[]};
const operations={U1:[],U2:[],U3:[]};
if(['official','hosts','u2-extra','r2-extra'].includes(process.argv[2])){
  Object.assign(baseline,JSON.parse(fs.readFileSync(path.join(out,'baseline.json'),'utf8')));
  for(const unit of ['U1','U2','U3']){
    manifests[unit]=JSON.parse(fs.readFileSync(path.join(out,`${unit}-capture.json`),'utf8'));
    operations[unit]=[fs.readFileSync(path.join(out,`${unit}-operations.md`),'utf8').replace(/^# .*?\n\n/,'')];
  }
}
let browser;
function save(){
  fs.writeFileSync(path.join(out,'baseline.json'),JSON.stringify(baseline,null,2));
  for(const unit of Object.keys(manifests)){
    fs.writeFileSync(path.join(out,`${unit}-capture.json`),JSON.stringify(manifests[unit],null,2));
    fs.writeFileSync(path.join(out,`${unit}-operations.md`),`# ${unit} 本轮实际操作\n\n`+operations[unit].join('\n\n'));
  }
}
const waitState=(p,state)=>expect(p.locator('dialog[data-rd-search]')).toHaveAttribute('data-state',state,{timeout:15000});
const inp=p=>p.locator('[data-rd-search-input]');
async function fresh(base,route,view,theme,js=true){
  const context=await browser.newContext({viewport:view,deviceScaleFactor:1,colorScheme:theme,reducedMotion:'reduce',javaScriptEnabled:js});
  const p=await context.newPage();
  p.__reviewJs=js;
  const append=x=>fs.appendFileSync(path.join(out,'browser-raw.jsonl'),JSON.stringify({at:new Date().toISOString(),...x})+'\n');
  p.on('console',m=>append({type:'console',level:m.type(),text:m.text(),location:m.location()}));
  p.on('pageerror',e=>append({type:'pageerror',text:String(e),url:p.url()}));
  context.on('requestfailed',r=>append({type:'requestfailed',url:r.url(),error:r.failure()}));
  context.on('request',r=>append({type:'request',url:r.url(),method:r.method()}));
  context.on('response',r=>{if(r.status()>=400)append({type:'http-error',url:r.url(),status:r.status()});});
  if(js)await p.addInitScript(()=>document.addEventListener('securitypolicyviolation',e=>console.error('UI-REVIEW-CSP',e.violatedDirective,e.blockedURI)));
  await p.goto(new URL(route,base).href,{waitUntil:'networkidle'});
  if(js){await p.locator(`[data-rd-mode="${theme}"]`).click(); await expect(p.locator('html')).toHaveAttribute('data-theme',theme);}
  return {p,context};
}
async function shot(unit,state,p,theme,action,interception='none'){
  const vp=p.viewportSize();
  const filename=`${unit}-${state}-${vp.width}-${theme}.png`;
  const absolute=path.join(out,filename);
  if(fs.existsSync(absolute)){console.log(JSON.stringify({preserved:filename}));return;}
  if(p.__reviewJs!==false)await p.evaluate(async()=>{await document.fonts.ready;await new Promise(r=>requestAnimationFrame(()=>requestAnimationFrame(r)));});
  await p.screenshot({path:absolute,fullPage:false,animations:'disabled'});
  const observation=await p.evaluate(()=>{
    const d=document.querySelector('dialog[data-rd-search]');
    const active=document.activeElement;
    return {title:document.title,htmlLang:document.documentElement.lang,theme:document.documentElement.dataset.theme,dialogOpen:d?.open||false,dialogState:d?.dataset.state||null,input:d?.querySelector('input')?.value||'',status:d?.querySelector('[data-rd-search-status]')?.textContent||'',total:d?.querySelector('[data-rd-search-count]')?.textContent||'',options:d?.querySelectorAll('[role=option]').length||0,selected:d?.querySelector('[aria-selected=true]')?.id,activeDescendant:d?.querySelector('input')?.getAttribute('aria-activedescendant'),focused:active?{tag:active.tagName,id:active.id,text:(active.textContent||'').slice(0,100),searchInput:active.matches('[data-rd-search-input]')}:null};
  });
  const record={unit,state,file:filename,url:p.url(),viewport:vp,dpr:1,theme,at:new Date().toISOString(),tool:`Playwright 1.63.0 / Chromium ${baseline.browser}`,interception,action,observation,sha256:hash(fs.readFileSync(absolute))};
  manifests[unit].push(record);operations[unit].push(`- ${filename}：${action}\n  - 实测记录：${JSON.stringify(observation)}`);save();
  console.log(JSON.stringify({captured:filename,url:p.url()}));
}
async function main(){
  browser=await chromium.launch({headless:true});baseline.browser=browser.version();save();
  const which=process.argv[2]||'all';

  if(which==='r2-extra'){
    {
      const {p,context}=await fresh(sbase,'',views[0],'light');
      await p.locator('.rd-brand').focus();await p.keyboard.press('Control+k');await inp(p).fill('SearchCorpusToken');await waitState(p,'results');await p.keyboard.press('ArrowDown');await p.keyboard.press('Tab');
      await shot('U1','more-20-focus',p,'light','快捷键打开后查询SearchCorpusToken；ArrowDown选择第二项，Tab到Show more。');
      for(const n of [40,60,80,98]){await p.keyboard.press('Enter');await expect(p.locator('[role=option]')).toHaveCount(n);await shot('U1',`more-${n}-focus`,p,'light',`仅真实按Enter，结果增至${n}组；记录实际焦点与所选项。`);}
      await p.keyboard.press('Tab');await p.keyboard.press('Shift+Tab');await shot('U1','more-final-keyboard',p,'light','末批后真实Tab和Shift+Tab，记录键盘焦点仍可继续操作。');await context.close();
    }
    for(const [base,label]of [[sbase,'subpath'],['http://127.0.0.1:4204/','root'],['http://127.0.0.1:4206/rustify-ui/','meta']]){
      const {p,context}=await fresh(base,'page-0001/',views[0],'light');await p.locator('[data-rd-search-open]').click();await inp(p).fill('ThemeScope');await waitState(p,'results');await p.keyboard.press('ArrowDown');await p.keyboard.press('ArrowUp');const before=p.url();await p.keyboard.press('Enter');await p.waitForURL(url=>url.href!==before);await p.waitForLoadState('networkidle');await shot('U1',`${label}-activation-a2`,p,'light','输入ThemeScope，真实ArrowDown/ArrowUp后Enter激活，记录真实目标URL与关闭状态。');await context.close();
    }
    {
      const {p,context}=await fresh(sbase,'zh/page-0001/',views[1],'dark');await p.locator('[data-rd-search-open]').click();await inp(p).fill('FallbackOnlyToken');await waitState(p,'results');await p.keyboard.press('Enter');await p.waitForURL('**/zh/page-0051/');await shot('U1','zh-fallback-destination',p,'dark','中文搜索缺译词FallbackOnlyToken后真实Enter；导航仍在zh/page-0051/。');await context.close();
    }
    for(const [vp,theme]of [[views[0],'light'],[views[2],'dark']]){
      const {p,context}=await fresh(obase,'zh/__ui_review_missing__/deep/',vp,theme);await shot('U3','404-closed',p,theme,'普通访问中文深404，尚未打开搜索。');await p.locator('[data-rd-search-open]').click();await inp(p).fill('快速开始');await waitState(p,'results');await p.keyboard.press('Enter');await p.waitForURL('**/zh/quickstart/**');await shot('U3','404-destination',p,theme,'中文深404搜快速开始后真实Enter，导航到中文quickstart。');await context.close();
    }
  }

  if(which==='u2-extra'){
    {
      const {p,context}=await fresh(sbase,'page-0001/',views[1],'light');
      let release;const held=new Promise(r=>release=r);
      await context.route(/\/assets\/search\.[a-f0-9]+\.js(?:\?|$)/,async route=>{await held;await route.continue();});
      await p.locator('[data-rd-search-open]').click();
      await shot('U2','supp-open-before-module-01',p,'light','点击 Search；首次 module HTTP 响应仍被扣留、尚未执行，输入已获焦。','hold first search module response');
      await inp(p).fill('CustomOnlyToken');await inp(p).press('ControlOrMeta+A');await inp(p).pressSequentially('ReachableCustomToken');await waitState(p,'loading');
      await shot('U2','supp-loading-edit-02',p,'light','module仍未返回，在loading输入CustomOnlyToken，再真实全选并键入ReachableCustomToken。','hold first search module response');
      release();await waitState(p,'results');
      await shot('U2','supp-loading-result-03',p,'light','释放module响应后，显示loading期间最终输入ReachableCustomToken的结果。','release held module; real Worker/index');
      await inp(p).fill('SearchCorpusToken');await waitState(p,'results');for(let i=0;i<19;i++)await p.keyboard.press('ArrowDown');
      await shot('U2','supp-longlist-04',p,'light','改搜SearchCorpusToken，连续真实按ArrowDown19次至首批末项；列表滚动，输入、关闭与Show more仍在视口内。');
      await context.close();
    }
    {
      const {p,context}=await fresh(sbase,'page-0001/',views[1],'light');let fail=true;
      await context.route(/\/assets\/search\.[a-f0-9]+\.js(?:\?|$)/,async route=>{if(!fail)return route.continue();fail=false;return route.fulfill({status:503,contentType:'text/javascript',body:'unavailable'});});
      await p.locator('[data-rd-search-open]').click();await inp(p).fill('CustomOnlyToken');await waitState(p,'error');await p.keyboard.press('Tab');
      await shot('U2','supp-retry-focus-05',p,'light','module503后，输入获焦；真实按Tab到Retry，拍焦点帧。','first module HTTP503');
      await p.keyboard.press('Enter');await waitState(p,'results');
      await shot('U2','supp-keyboard-retry-06',p,'light','真实按Enter激活Retry；输入恢复焦点且保留查询，结果显示。');
      await p.keyboard.press('Shift+Tab');
      await shot('U2','supp-close-focus-07',p,'light','从输入真实按Shift+Tab，焦点到Close。');
      await p.keyboard.press('Enter');
      await shot('U2','supp-close-return-08',p,'light','真实按Enter激活Close，对话框关闭，焦点返回页头Search。');await context.close();
    }
    {
      const {p,context}=await fresh(sbase,'page-0001/',views[0],'dark');let first=true;let release;const held=new Promise(r=>release=r);
      await context.route(/\/assets\/search\.en\.[a-f0-9]+\.json$/,async route=>{if(first){first=false;await held;}await route.continue().catch(()=>{});});
      await p.locator('[data-rd-search-open]').click();await inp(p).fill('CustomOnlyToken');await waitState(p,'loading');await expect.poll(()=>first).toBe(false);await p.keyboard.press('Escape');await p.locator('[data-rd-search-open]').click();await inp(p).fill('ReachableCustomToken');await waitState(p,'results');
      await shot('U2','supp-new-result-before-old-09',p,'dark','旧首个JSON仍被扣留；关闭并重开后，新查询ReachableCustomToken结果已显示。','first old-session JSON still held');
      release();await p.waitForTimeout(250);
      await shot('U2','supp-old-released-after-new-10',p,'dark','新查询结果已显示之后才释放旧JSON，等250ms；仍显示ReachableCustomToken当前结果。','release old-session JSON after new query results');await context.close();
    }
  }
  if(which==='hosts')for(const [base,state]of [['http://127.0.0.1:4204/','root-results'],['http://127.0.0.1:4206/rustify-ui/','meta-results']]){
    const {p,context}=await fresh(base,'page-0001/',views[0],'light');
    await p.locator('[data-rd-search-open]').click();await inp(p).fill('ThemeScope');await waitState(p,'results');
    await shot('U1',state,p,'light','从真实根路径/仅meta CSP静态托管打开搜索并输入 ThemeScope。');await context.close();
  }
  if(which==='all'||which==='search'){
    for(const vp of views)for(const theme of ['light','dark']){
      const {p,context}=await fresh(sbase,'page-0001/',vp,theme);
      await p.locator('[data-rd-search-open]').click();await waitState(p,'idle');
      await shot('U1','idle',p,theme,'点击页头 Search，等候空查询 idle；输入获焦。');
      await inp(p).fill('ThemeScope');await waitState(p,'results');
      await shot('U1','results',p,theme,'输入 ThemeScope，等候真实当前语言结果。');
      await inp(p).fill('AbsolutelyNoMatchingWordHere');await waitState(p,'empty');
      await shot('U1','empty',p,theme,'将查询改为不存在的词，等候 empty。');
      await context.close();
    }
    {
      const {p,context}=await fresh(sbase,'',views[0],'light');
      await p.locator('.rd-brand').focus();await p.keyboard.press('Control+k');await waitState(p,'idle');
      await shot('U1','keyboard-open-01',p,'light','品牌链接获得焦点后，真实按 Control+K 打开搜索。');
      await inp(p).fill('SearchCorpusToken');await waitState(p,'results');await p.keyboard.press('ArrowDown');
      await shot('U1','keyboard-select-02',p,'light','输入 SearchCorpusToken 后真实按 ArrowDown；输入仍获焦，结果选择已变化。');
      await p.keyboard.press('Tab');await shot('U1','keyboard-more-focus-03',p,'light','真实按 Tab，焦点到 Show more。');
      await p.keyboard.press('Enter');await expect(p.locator('[role=option]')).toHaveCount(40);
      await shot('U1','keyboard-more-04',p,'light','真实按 Enter 显示更多；可见 DOM 结果从20增加为40。');
      await p.keyboard.press('Escape');await shot('U1','keyboard-close-05',p,'light','真实按 Escape 关闭，活动元素为原品牌链接。');
      await p.keyboard.press('Meta+k');await inp(p).fill('ReachableCustomToken');await waitState(p,'results');await p.keyboard.press('Enter');
      await expect(p).toHaveURL(sbase+'#custom-section');
      await shot('U1','keyboard-destination-06',p,'light','真实按 Meta+K 打开，输入 ReachableCustomToken 并 Enter；导航到定制页 #custom-section，对话框关闭。');
      await context.close();
    }
    {
      const {p,context}=await fresh(sbase,'zh/page-0001/',views[1],'dark');await p.locator('[data-rd-search-open]').click();
      await inp(p).fill('主题切换');await waitState(p,'results');await shot('U1','zh-translated',p,'dark','中文页输入主题切换。');
      await inp(p).fill('FallbackOnlyToken');await waitState(p,'results');await shot('U1','zh-fallback',p,'dark','中文界面输入 FallbackOnlyToken，查看缺译标签与中文目标地址。');
      await context.close();
    }
    for(const [vp,theme]of [[views[0],'dark'],[views[1],'light']]){
      let first=true;let release;const held=new Promise(r=>release=r);
      const {p,context}=await fresh(sbase,'page-0001/',vp,theme);
      await context.route(/\/assets\/search\.en\.[a-f0-9]+\.json$/,async route=>{if(first){first=false;await held;}await route.continue();});
      await p.locator('[data-rd-search-open]').click();await inp(p).fill('CustomOnlyToken');await waitState(p,'loading');
      await shot('U2','loading',p,theme,'延迟首次索引响应，输入 CustomOnlyToken；保留输入并显示 loading。','hold first en search JSON response');
      await p.keyboard.press('Escape');
      if(vp.width===1280)await shot('U2','closed-01',p,theme,'索引仍延迟时真实按 Escape，焦点回到 Search。','hold first en search JSON response');
      await p.locator('[data-rd-search-open]').click();await waitState(p,'results');
      if(vp.width===1280)await shot('U2','reopen-02',p,theme,'重新点击 Search，重取索引后显示保留查询结果。','first JSON request still held; new session uses real next response');
      release();await inp(p).fill('ReachableCustomToken');await waitState(p,'results');
      if(vp.width===1280)await shot('U2','late-response-03',p,theme,'释放旧请求，再输入 ReachableCustomToken；当前结果仍为新查询。','release held first JSON response');
      await context.close();
      for(const kind of ['module','index']){
        const {p,context}=await fresh(sbase,'page-0001/',vp,theme);let fail=true;
        const regex=kind==='module'?/\/assets\/search\.[a-f0-9]+\.js(?:\?|$)/:/\/assets\/search\.en\.[a-f0-9]+\.json$/;
        await context.route(regex,async route=>{if(!fail)return route.continue();fail=false; if(kind==='module')return route.fulfill({status:503,contentType:'text/javascript',body:'unavailable'});return route.fulfill({json:{schema_version:99,pages:[]}});});
        await p.locator('[data-rd-search-open]').click();await inp(p).fill('CustomOnlyToken');await waitState(p,'error');
        await shot('U2',`${kind}-error`,p,theme,'首次请求故障后输入 CustomOnlyToken，等候 error；查看 Retry。',kind==='module'?'first search module HTTP 503':'first search JSON schema_version=99');
        await p.locator('[data-rd-search-retry]').click();await waitState(p,'results');
        await shot('U2',`${kind}-retry`,p,theme,'点击 Retry，恢复真实同源资源；保留 CustomOnlyToken 并显示结果。','retry uses real resource; only first request was faulted');
        await context.close();
      }
    }
  }
  if(which==='all'||which==='official'){
    for(const vp of views)for(const theme of ['light','dark'])for(const [route,state]of [['','en-home'],['zh/','zh-home'],['zh/quickstart/','zh-quickstart']]){
      const {p,context}=await fresh(obase,route,vp,theme);await shot('U3',state,p,theme,'从官方构建入口普通导航到页面，使用站内主题按钮设置主题。');await context.close();
    }
    for(const [vp,theme]of [[views[0],'light'],[views[2],'dark']])for(const [route,state]of [['zh/architecture/','fallback'],['zh/__ui_review_missing__/deep/','404-search']]){
      const {p,context}=await fresh(obase,route,vp,theme);
      if(state==='404-search'){await p.locator('[data-rd-search-open]').click();await inp(p).fill('快速开始');await waitState(p,'results');}
      await shot('U3',state,p,theme,state==='fallback'?'普通导航到中文缺译 architecture 页。':'访问中文不存在的深路径，点击搜索并输入快速开始，显示真实官方站中文结果。');await context.close();
    }
    for(const [route,state]of [['zh/quickstart/','noscript-quickstart'],['zh/__ui_review_missing__/deep/','noscript-404']]){
      const {p,context}=await fresh(obase,route,views[1],'light',false);await shot('U3',state,p,'light','新建禁用 JavaScript 的浏览器上下文，普通打开页面；未改界面代码。','browser JavaScript disabled');await context.close();
    }
    {
      const {p,context}=await fresh(obase,'zh/quickstart/',views[1],'light');
      await p.locator('.rd-language-switch summary').click();await shot('U3','language-open',p,'light','点击中文 quickstart 页的语言菜单 summary。');await p.keyboard.press('Escape');
      await p.locator('.rd-sidebar-narrow summary').click();await shot('U3','navigation-open',p,'light','真实按 Escape 关闭语言菜单，再点击窄屏导航 summary。');await context.close();
    }
  }
  save();await browser.close();
}
main().catch(async e=>{console.error(e);save();if(browser)await browser.close();process.exitCode=1;});
