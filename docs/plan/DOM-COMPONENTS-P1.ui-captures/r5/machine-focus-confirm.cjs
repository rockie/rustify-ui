const{chromium}=require('@playwright/test');const fs=require('fs'),path=require('path');
const out={at:new Date().toISOString(),cases:[]};
async function state(page){return page.evaluate(()=>{const n=document.activeElement,s=getComputedStyle(n);return{testid:n.dataset.testid,focusVisible:n.matches(':focus-visible'),outline:[s.outlineStyle,s.outlineWidth,s.outlineColor,s.outlineOffset].join(' '),boxShadow:s.boxShadow,border:s.borderColor,background:s.backgroundColor}});}
async function tabTo(page,id,max=70){for(let i=1;i<=max;i++){await page.keyboard.press('Tab');if(await page.evaluate(id=>document.activeElement?.dataset.testid===id,id))return i;}throw Error('Tab did not reach '+id);}
(async()=>{const browser=await chromium.launch();try{
for(const[trigger,id,label]of[['theme-export','export-kind','export'],['theme-import','import-format','import']]){
 const ctx=await browser.newContext({viewport:{width:1440,height:900},reducedMotion:'reduce'});const page=await ctx.newPage();await page.goto('http://127.0.0.1:4181/');await page.locator('[data-status=ready]').first().waitFor({state:'attached',timeout:90000});await page.evaluate(()=>document.fonts.ready);
 const triggerTabs=await tabTo(page,trigger);await page.keyboard.press('Enter');const dialogTabs=await tabTo(page,id,15);await page.waitForTimeout(700);const focused=await state(page);const screenshot=`MC-${label}-format-pure-keyboard-1440-light.png`;await page.screenshot({path:path.join(__dirname,screenshot),animations:'disabled'});
 await page.keyboard.press('Tab');await page.waitForTimeout(700);const next=await state(page);const nextScreenshot=`MC-${label}-next-control-keyboard-1440-light.png`;await page.screenshot({path:path.join(__dirname,nextScreenshot),animations:'disabled'});
 const item={label,triggerTabs,dialogTabs,focused,next,screenshot,nextScreenshot};out.cases.push(item);
 if(label==='export'){
  await page.getByTestId('export-kind').click();await page.getByTestId('export-kind-list').getByRole('option',{name:'CSS · resolved values',exact:true}).click();
  item.css=[];for(const id2 of['export-profile','export-color-format']){await page.getByTestId(id2).focus();await page.keyboard.press('Shift+Tab');await page.keyboard.press('Tab');await page.waitForTimeout(700);const file=`MC-${id2}-keyboard-focus-1440-light.png`;item.css.push({id:id2,...await state(page),screenshot:file});await page.screenshot({path:path.join(__dirname,file),animations:'disabled'});}
 }
 await ctx.close();fs.writeFileSync(path.join(__dirname,'machine-focus-confirm.json'),JSON.stringify(out,null,2));
}
}catch(e){out.error=String(e.stack||e);process.exitCode=1;}finally{await browser.close();fs.writeFileSync(path.join(__dirname,'machine-focus-confirm.json'),JSON.stringify(out,null,2));console.log(JSON.stringify(out,null,2));}})();
