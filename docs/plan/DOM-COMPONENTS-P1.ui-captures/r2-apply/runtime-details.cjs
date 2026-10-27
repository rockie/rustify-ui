// Focused live checks that supplement the frozen capture matrix.
const {chromium}=require('@playwright/test');
const fs=require('node:fs');
const path=require('node:path');
(async()=>{
 const browser=await chromium.launch();
 const context=await browser.newContext({viewport:{width:1440,height:900},deviceScaleFactor:1,reducedMotion:'reduce',locale:'en-US'});
 const page=await context.newPage();const errors=[];
 page.on('pageerror',e=>errors.push(e.message));page.on('requestfailed',r=>errors.push(`${r.url()}: ${r.failure()?.errorText}`));
 await page.goto('http://127.0.0.1:4181/');await page.locator('#status[data-status=ready]').waitFor({state:'attached'});await page.evaluate(()=>document.fonts.ready);
 const before=await page.evaluate(()=>({faces:[...document.fonts].filter(f=>f.family.includes('IBM Plex Sans')).map(f=>({weight:f.weight,status:f.status})),fontRequests:performance.getEntriesByType('resource').filter(e=>e.name.includes('/public/fonts/ibm-plex-sans/')).map(e=>e.name),titleSize:getComputedStyle(document.querySelector('[data-testid="current-theme-name"]')).fontSize}));
 const candidate=await page.evaluate(()=>structuredClone(window.__theme_studio.snapshot().document));candidate.name='Review candidate';candidate.config.accent_color='tomato';
 await page.getByTestId('theme-import').click();await page.getByTestId('import-source').fill(JSON.stringify(candidate,null,2));await page.getByTestId('import-review').click();
 const brand=page.getByTestId('import-config').locator('div').filter({has:page.locator('dt',{hasText:'Brand color'})});
 const text=await brand.innerText();if(!text.includes('Tomato')||await brand.locator('.tools-change-label').count()!==1)throw Error(`Missing current→imported change: ${text}`);
 const captureDir=process.env.UI_APPLY_OUT||path.join(__dirname,'confirm-shell');fs.mkdirSync(captureDir,{recursive:true});
 const screenshot='U3-import-v2-config-review-1440-light.png';await page.screenshot({path:path.join(captureDir,screenshot),animations:'disabled'});
 const config=await page.getByTestId('import-config').innerText();await page.keyboard.press('Escape');
 await page.setViewportSize({width:768,height:1024});await page.waitForTimeout(200);
 const narrow=await page.evaluate(()=>({titleSize:getComputedStyle(document.querySelector('[data-testid="current-theme-name"]')).fontSize,headingSize:getComputedStyle(document.querySelector('[data-testid="editor-shell"] h1')).fontSize}));
 if(narrow.titleSize!=='16px'||narrow.headingSize!=='16px')throw Error(JSON.stringify(narrow));
 const report={at:new Date().toISOString(),captureDir:path.relative(__dirname,captureDir),before,narrow,config,screenshot,errors};fs.writeFileSync(path.join(__dirname,'runtime-details.json'),JSON.stringify(report,null,2));await browser.close();console.log(JSON.stringify(report));
})();
