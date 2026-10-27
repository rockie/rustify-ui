// Observe real pointer/focus events; this is a bug reproduction, not an acceptance capture.
const {chromium}=require('@playwright/test');
const fs=require('node:fs');
(async()=>{
 const browser=await chromium.launch();const page=await browser.newPage({viewport:{width:1440,height:900}});
 await page.goto('http://127.0.0.1:4181/');await page.locator('#status[data-status=ready]').waitFor({state:'attached'});await page.evaluate(()=>document.fonts.ready);
 await page.getByTestId('editor-controls').click();await page.getByTestId('choose-font-sans').scrollIntoViewIfNeeded();
 await page.evaluate(()=>{
  window.__focusProbe=[];
  const record=e=>{const n=document.querySelector('[data-testid="choose-font-sans"]');window.__focusProbe.push({event:e.type,target:e.target?.dataset?.testid,rect:n.getBoundingClientRect().toJSON(),expanded:n.getAttribute('aria-expanded'),sources:document.querySelectorAll('[data-testid="studio-editor"] [data-testid="token-source"]').length});};
  for(const event of ['pointerdown','focusin','pointerup','click'])document.addEventListener(event,record,true);
 });
 const before=await page.getByTestId('choose-font-sans').boundingBox();await page.getByTestId('choose-font-sans').click();await page.waitForTimeout(100);
 const result={before,after:await page.getByTestId('choose-font-sans').boundingBox(),expanded:await page.getByTestId('choose-font-sans').getAttribute('aria-expanded'),events:await page.evaluate(()=>window.__focusProbe)};
 if (result.expanded !== 'true') { await page.getByTestId('choose-font-sans').press('Enter'); result.keyboardOpened=await page.getByTestId('choose-font-sans-list').isVisible(); }
 await browser.close();fs.writeFileSync(process.argv[2],JSON.stringify(result,null,2));console.log(JSON.stringify(result));
})();
