// Live-page evidence only; execute after independent judgments are saved.
const { chromium } = require('@playwright/test');
const fs = require('node:fs');
const path = require('node:path');
const OUT = __dirname;
const result = { at: new Date().toISOString(), inspector: [], semantics: {}, errors: [] };
async function opened(browser, url, width, height) {
  const context = await browser.newContext({ viewport: { width, height }, deviceScaleFactor: 1,
    reducedMotion: 'reduce', locale: 'en-US', colorScheme: 'light' });
  const page = await context.newPage();
  page.on('pageerror', error => result.errors.push(error.message));
  await page.goto(url);
  await page.locator('#status[data-status=ready],[data-testid=status][data-status=ready]').first().waitFor({ state: 'attached', timeout: 90000 });
  await page.evaluate(() => document.fonts.ready);
  await page.waitForLoadState('networkidle');
  return { context, page };
}
const tid = (page, id) => page.getByTestId(id);
(async () => {
  const browser = await chromium.launch();
  try {
    for (const [width, height, mode] of [[1440, 900, 'light'], [1440, 900, 'dark'], [390, 844, 'light']]) {
      const { context, page } = await opened(browser, 'http://127.0.0.1:4181/', width, height);
      if (width === 390) await tid(page, 'mobile-preview').click();
      await tid(page, 'renderer-dom').click();
      if (mode === 'dark') await tid(page, 'mode-dark').click();
      await tid(page, 'inspector-toggle').click();
      await page.waitForTimeout(800);
      const target = tid(page, 'studio-preview').getByTestId('preview-primary');
      await target.scrollIntoViewIfNeeded();
      await target.click();
      await page.waitForTimeout(700);
      const observed = await target.evaluate(node => {
        const r = node.getBoundingClientRect(), s = getComputedStyle(node);
        const overlays = [...document.querySelectorAll('body *')].flatMap(element => {
          const c = getComputedStyle(element), b = element.getBoundingClientRect();
          if (c.position !== 'fixed' || b.width <= 0 || b.height <= 0) return [];
          if (Math.abs(b.left-r.left)>10 || Math.abs(b.top-r.top)>10 || Math.abs(b.width-r.width)>20 || Math.abs(b.height-r.height)>20) return [];
          return [{ tag: element.tagName, testid: element.dataset.testid || null, className: element.className,
            border: c.border, outline: c.outline, shadow: c.boxShadow,
            rect: { x: b.x, y: b.y, width: b.width, height: b.height } }];
        });
        return { target: { text: node.textContent, outline: s.outline, border: s.border,
          rect: { x: r.x, y: r.y, width: r.width, height: r.height } }, overlays,
          panelText: document.querySelector('[data-testid=inspector-panel]')?.textContent || null };
      });
      const file = `MC-inspector-unmapped-${width}-${mode}.png`;
      await page.screenshot({ path: path.join(OUT, file), animations: 'disabled' });
      await page.keyboard.press('Escape');
      observed.afterEscapePressed = await tid(page, 'inspector-toggle').getAttribute('aria-pressed');
      result.inspector.push({ viewport: `${width}x${height}`, mode, file, ...observed });
      await context.close();
    }
    {
      const { context, page } = await opened(browser, 'http://127.0.0.1:4176/', 1280, 900);
      const host = await page.locator('body').elementHandle();
      const styles = element => { const s=getComputedStyle(element); return { fontFamily:s.fontFamily,
        fontSize:s.fontSize, color:s.color, background:s.backgroundColor, margin:s.margin, padding:s.padding }; };
      const before = await host.evaluate(styles);
      await page.evaluate(() => window.__component_catalog.mount_layout_fixtures());
      await tid(page, 'layout-typography').waitFor();
      result.semantics = await tid(page, 'catalog-layout-fixture').evaluate(root => ({
        headings: [...root.querySelectorAll('h1,h2,h3,h4,h5,h6')].map(n => ({ tag:n.tagName, text:n.textContent })),
        nativeTypography: [...root.querySelectorAll('strong,em,q,blockquote,code,kbd')].map(n => ({ tag:n.tagName, text:n.textContent })),
        resetElements: [...root.querySelectorAll('[class*="reset"]')].map(n => ({ tag:n.tagName,
          testid:n.dataset.testid || null, classes:n.className, childTags:[...n.children].map(c=>c.tagName), text:n.textContent })),
        ellipsis: [...root.querySelectorAll('*')].flatMap(n => { const s=getComputedStyle(n);
          return s.textOverflow==='ellipsis' ? [{ tag:n.tagName, testid:n.dataset.testid || null, fullText:n.textContent,
            scrollWidth:n.scrollWidth, clientWidth:n.clientWidth, textNodes:[...n.childNodes].filter(x=>x.nodeType===3).map(x=>x.textContent) }] : []; })
      }));
      result.semantics.hostBodyBefore = before;
      result.semantics.hostBodyAfter = await host.evaluate(styles);
      result.semantics.fonts = await page.evaluate(() => ({ loaded:document.fonts.status,
        shellFamily:getComputedStyle(document.querySelector('h1') || document.body).fontFamily }));
      await context.close();
    }
    result.chromium = browser.version();
  } catch (error) { result.error = String(error.stack || error); process.exitCode = 1; }
  finally { await browser.close(); fs.writeFileSync(path.join(OUT, 'machine-extra.json'), JSON.stringify(result,null,2)); }
})();
