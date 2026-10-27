const {chromium}=require('@playwright/test');
const fs=require('node:fs');const path=require('node:path');
(async()=>{
 const out=process.env.UI_APPLY_OUT||path.join(__dirname,'confirm-inspector');fs.mkdirSync(out,{recursive:true});
 const browser=await chromium.launch();const errors=[];const cases=[];
 try{
  const context=await browser.newContext({viewport:{width:1440,height:900},deviceScaleFactor:1,reducedMotion:'reduce',locale:'en-US'});
  const page=await context.newPage();page.on('pageerror',e=>errors.push(e.message));
  await page.goto('http://127.0.0.1:4181/');await page.locator('#status[data-status=ready]').waitFor({state:'attached'});await page.evaluate(()=>document.fonts.ready);
  await page.getByTestId('renderer-dom').click();await page.getByTestId('inspector-toggle').click();await page.waitForTimeout(800);
  await page.getByTestId('dom-preview').getByTestId('cards-submit').click();
  async function record(label,file){
   await page.evaluate(()=>new Promise(r=>requestAnimationFrame(()=>requestAnimationFrame(r))));
   const state=await page.evaluate(()=>{
    const source=document.querySelector('[data-testid="token-source"]'),heading=source.querySelector('h3').getBoundingClientRect(),port=document.querySelector('#studio-editor').getBoundingClientRect();
    return {token:source.querySelector('[data-testid="source-token"]').textContent,heading:heading.toJSON(),editor:port.toJSON(),active:document.activeElement?.dataset?.testid,visible:heading.top>=port.top&&heading.bottom<=port.bottom};
   });
   if(!state.visible||state.active!=='color-value')throw Error(JSON.stringify(state));
   await page.screenshot({path:path.join(out,file),animations:'disabled'});cases.push({label,file,...state});
  }
  await record('DOM primary','U5-inspector-picked-1440-light.png');
  await page.getByTestId('renderer-compare').click();await page.getByTestId('theme-gpu').scrollIntoViewIfNeeded();
  await page.waitForFunction(()=>{const s=window.__theme_studio.snapshot();return s.gpu_state==='Ready'&&s.drawn_revision===s.revision&&s.samples.length>50;});
  const sample=await page.evaluate(()=>window.__theme_studio.snapshot().samples.find(s=>s.id==='color.chart-3'));
  const [x,y,w,h]=sample.visible_rect;
  const point=await page.getByTestId('theme-gpu').evaluate((node,local)=>{
   const scale=node.getBoundingClientRect().height/node.clientHeight;
   for(let parent=node.parentElement;parent;parent=parent.parentElement){
    if(parent.scrollHeight>parent.clientHeight+1&&/auto|scroll/.test(getComputedStyle(parent).overflowY)){
     const port=parent.getBoundingClientRect();parent.scrollTop+=node.getBoundingClientRect().top+local.y*scale-(port.top+port.height/2);
    }
   }
   const r=node.getBoundingClientRect();return {x:r.left+local.x*r.width/node.clientWidth,y:r.top+local.y*scale};
  },{x:x+w/2,y:y+h/2});
  if(await page.evaluate(p=>document.elementFromPoint(p.x,p.y)?.dataset.testid,point)!=='theme-gpu')throw Error('GPU sample is not in the visible canvas');
  await page.mouse.click(point.x,point.y);
  await record('GPU chart-3','U5-inspector-gpu-picked-1440-light.png');
  const report={at:new Date().toISOString(),chromium:browser.version(),cases,errors};fs.writeFileSync(path.join(out,'source-check.json'),JSON.stringify(report,null,2));console.log(JSON.stringify(report));
 }finally{await browser.close();}
})();
