const {chromium}=require('@playwright/test');
const AxeBuilder=require('@axe-core/playwright').default;
const fs=require('node:fs');
(async()=>{
 const browser=await chromium.launch();
 try{
  const context=await browser.newContext({viewport:{width:1280,height:900}});
  const page=await context.newPage();await page.goto('http://127.0.0.1:4176/');
  await page.locator('#status[data-status=ready]').waitFor({state:'attached'});
  await page.evaluate(()=>window.__component_catalog.mount_layout_fixtures());
  await page.getByTestId('layout-invalid').click();await page.getByTestId('layout-prop-diagnostics').scrollIntoViewIfNeeded();
  const style=await page.getByTestId('layout-prop-diagnostics').evaluate(n=>({color:getComputedStyle(n).color,font:getComputedStyle(n).fontSize,text:n.textContent}));
  const scan=await new AxeBuilder({page}).include('[data-testid="layout-prop-diagnostics"]').withTags(['wcag2aa']).analyze();
  const report={style,violations:scan.violations.map(v=>({id:v.id,nodes:v.nodes.map(n=>n.failureSummary)})),incomplete:scan.incomplete.map(v=>v.id)};
  fs.writeFileSync(process.argv[2],JSON.stringify(report,null,2));console.log(JSON.stringify(report));
  process.exitCode=report.violations.length?1:0;
 }finally{await browser.close();}
})();
