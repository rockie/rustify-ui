const {chromium}=require('@playwright/test');
const fs=require('node:fs');const path=require('node:path');
(async()=>{
 const out=path.join(__dirname,'slider-confirm');fs.mkdirSync(out,{recursive:true});const results=[];
 for(const [name,engine] of [['chromium',chromium]]){
  const browser=await engine.launch();
  try{
   for(const width of name==='chromium'?[1440,390]:[1440]){
    const context=await browser.newContext({viewport:{width,height:900},deviceScaleFactor:1,reducedMotion:'reduce'});
    const page=await context.newPage();page.setDefaultTimeout(30000);const errors=[];page.on('pageerror',e=>errors.push(e.message));
    await page.goto('http://127.0.0.1:4176/');await page.getByTestId('status').waitFor({state:'attached'});
    await page.getByTestId('nav-slider').click();await page.evaluate(()=>document.fonts.ready);
    const input=page.getByTestId('example').getByRole('slider');await input.scrollIntoViewIfNeeded();
    for(const theme of ['light','dark']){
     if(theme==='dark')await page.getByTestId('toggle-theme').click();
     await input.press('Home');await input.press('ArrowRight');await page.getByTestId('category-name').click();
     const geometry=await input.evaluate(n=>{const r=n.getBoundingClientRect(),p=n.parentElement.getBoundingClientRect(),s=getComputedStyle(n),t=getComputedStyle(n,'::-moz-range-thumb');return{input:r.toJSON(),parent:p.toJSON(),margin:s.margin,overflow:s.overflow,mozThumb:{height:t.height,width:t.width,border:t.borderWidth,boxSizing:t.boxSizing},value:n.value};});
     const file=`slider-${name}-${width}-${theme}.png`;await page.screenshot({path:path.join(out,file),animations:'disabled'});
     results.push({name,version:browser.version(),width,theme,file,geometry,errors});
    }
    await context.close();
   }
  }finally{await browser.close();}
 }
 fs.writeFileSync(path.join(out,'geometry.json'),JSON.stringify(results,null,2));console.log(JSON.stringify(results));
})().catch(e=>{console.error(e);process.exit(1)});
