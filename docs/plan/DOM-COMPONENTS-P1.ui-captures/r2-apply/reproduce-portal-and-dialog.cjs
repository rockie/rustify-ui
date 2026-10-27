const {chromium}=require('@playwright/test');
const fs=require('node:fs');
(async()=>{
 const browser=await chromium.launch();const page=await browser.newPage({viewport:{width:1440,height:900}});
 await page.goto('http://127.0.0.1:4181/');await page.locator('#status[data-status=ready]').waitFor({state:'attached'});await page.evaluate(()=>document.fonts.ready);
 const dialog=[];
 async function sample(label){dialog.push(await page.evaluate(label=>{
  const node=document.querySelector('[data-testid="data-tools-dialog"]'),selector=document.querySelector('[data-testid="export-kind"]');
  const ancestors=[];for(let p=node;p;p=p.parentElement){const s=getComputedStyle(p);ancestors.push({tag:p.tagName,cls:p.className,rect:p.getBoundingClientRect().toJSON(),display:s.display,alignItems:s.alignItems,justifyContent:s.justifyContent,position:s.position,scrollTop:p.scrollTop,overflowY:s.overflowY});}
  return {label,selector:selector.getBoundingClientRect().toJSON(),active:document.activeElement?.dataset.testid,ancestors};
 },label));}
 await page.getByTestId('theme-export').click();await sample('before');
 await page.getByTestId('export-kind').click();await sample('opened');
 await page.getByTestId('export-kind-list').getByRole('option',{name:'Compatible JSON v1 · lossy',exact:true}).click();await page.waitForTimeout(200);await sample('after');
 await page.keyboard.press('Escape');
 await page.getByTestId('renderer-dom').click();await page.getByTestId('inspector-toggle').click();await page.getByTestId('inspector-panel').locator('details > summary').click();
 await page.evaluate(()=>{
  window.__oldColor=document.querySelector('[data-testid="color-value"]');window.__selectionEvents=[];
  const record=e=>window.__selectionEvents.push({event:e.type,target:e.target?.dataset?.testid,active:document.activeElement?.dataset?.testid,oldConnected:window.__oldColor.isConnected,same:window.__oldColor===document.querySelector('[data-testid="color-value"]')});
  document.addEventListener('focusin',record,true);document.addEventListener('focusout',record,true);
  new MutationObserver(()=>record({type:'mutation'})).observe(document.querySelector('[data-testid="studio-editor"]'),{childList:true,subtree:true});
 });
 await page.getByTestId('inspect-token-chart-3').click();await page.waitForTimeout(500);
 const selection=await page.evaluate(()=>({active:document.activeElement?.dataset?.testid,same:window.__oldColor===document.querySelector('[data-testid="color-value"]'),oldConnected:window.__oldColor.isConnected,events:window.__selectionEvents}));
 const result={dialog,selection};fs.writeFileSync(process.argv[2],JSON.stringify(result,null,2));console.log(JSON.stringify({dialog:dialog.map(x=>({label:x.label,selectorY:x.selector.y,ancestors:x.ancestors.slice(0,3)})),selection}));await browser.close();
})();
