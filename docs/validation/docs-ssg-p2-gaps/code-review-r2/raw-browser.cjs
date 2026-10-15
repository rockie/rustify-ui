const { chromium, expect } = require(process.cwd() + '/node_modules/@playwright/test');
const fs = require('node:fs');
const assert = require('node:assert/strict');
const [url, wantedJSON, output] = process.argv.slice(2);
const wanted = JSON.parse(wantedJSON);
(async () => {
  const browser = await chromium.launch({ executablePath: process.env.RUSTIFY_CHROMIUM || undefined });
  const context = await browser.newContext({ viewport: { width: 1280, height: 900 }, reducedMotion: 'reduce' });
  const page = await context.newPage();
  const audit = { url, wanted, browser: browser.version(), console: [], pageErrors: [], failures: [], csp: [], views: [] };
  page.on('console', msg => { if (['error','warning'].includes(msg.type())) audit.console.push(msg.text()); });
  page.on('pageerror', error => audit.pageErrors.push(String(error)));
  page.on('requestfailed', request => { if (!request.failure()?.errorText.includes('ERR_ABORTED')) audit.failures.push({url:request.url(),error:request.failure()}); });
  await page.exposeFunction('__reviewCSP', violation => audit.csp.push(violation));
  await page.addInitScript(() => document.addEventListener('securitypolicyviolation', e => window.__reviewCSP({ directive:e.effectiveDirective, uri:e.blockedURI })));
  try {
    await page.goto(url);
    const panel = page.locator('.rd-dev-panel');
    await expect(panel).toBeVisible({timeout: 30000});
    audit.allSuggestions = await page.locator('.rd-dev-suggestion').allTextContents();
    const group = panel.locator('.rd-dev-list > section').filter({ has: page.getByRole('heading', { name: 'content/gaps.md', exact: true }) });
    const repairs = group.locator('.rd-dev-suggestion');
    await expect(repairs).toHaveCount(wanted.length);
    const shown = await repairs.allTextContents();
    assert.deepEqual(shown.sort(), wanted.map(x=>x[3].replaceAll('`','')).sort());
    assert(shown.every(x=>x.includes('r#"a"   b   "c"#.to_string()')));
    for (const width of [1280,375]) {
      await page.setViewportSize({width,height:900});
      for (const theme of ['light','dark']) {
        await page.locator(`[data-rd-mode="${theme}"]`).click();
        await expect(page.locator('html')).toHaveAttribute('data-theme',theme);
        const view = await page.evaluate(() => ({theme:document.documentElement.dataset.theme,width:innerWidth,pageOverflow:document.documentElement.scrollWidth-innerWidth,repairs:[...document.querySelectorAll('.rd-dev-suggestion')].map(x=>({text:x.textContent,whiteSpace:getComputedStyle(x).whiteSpace,overflow:x.scrollWidth-x.clientWidth,codeWhiteSpace:[...x.querySelectorAll('code')].map(c=>getComputedStyle(c).whiteSpace)}))}));
        assert(view.pageOverflow<=0);
        assert(view.repairs.every(x=>x.overflow<=0 && x.whiteSpace==='pre-wrap' && x.codeWhiteSpace.every(v=>v==='pre-wrap')));
        audit.views.push(view);
      }
    }
    await page.screenshot({path:output+'.png',fullPage:true});
    const close = panel.getByRole('button',{name:'Hide problems'});
    await close.click();
    const marker=page.locator('.rd-dev-marker');
    await expect(marker).toBeFocused();
    await page.keyboard.press('Enter');
    await expect(close).toBeFocused();
    await page.keyboard.press('Escape');
    await expect(marker).toBeFocused();
    assert.deepEqual(audit.console,[]);assert.deepEqual(audit.pageErrors,[]);assert.deepEqual(audit.failures,[]);assert.deepEqual(audit.csp,[]);
    audit.passed=true;
    console.log('raw default: exact DOM text, 4 viewport/theme combinations, no overflow, hide/reopen/Escape, console and CSP passed');
  } finally {
    fs.writeFileSync(output+'.json',JSON.stringify(audit,null,2)+'\n');
    await context.close();await browser.close();
  }
})().catch(error=>{console.error(error);process.exitCode=1;});
