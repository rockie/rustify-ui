import { chromium } from '/Users/rockie/Documents/gh-rockie/rustify-ui/node_modules/@playwright/test/index.mjs';
import { writeFile, readFile } from 'node:fs/promises';

const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1280, height: 900 } });
const failures = [];
page.on('pageerror', error => failures.push(String(error)));
page.on('console', message => { if (message.type() === 'error') failures.push(message.text()); });
page.on('requestfailed', request => failures.push(`${request.url()}: ${request.failure()?.errorText}`));
try {
  const html = await readFile(new URL('./csr-probe/dist/index.html', import.meta.url), 'utf8');
  const moduleURL = html.match(/src="([^"]+\/islands\.js)"/)[1];
  const cssURL = html.match(/href="([^"]+\/site\.[^"]+\.css)"/)[1];
  await page.route('http://127.0.0.1:61138/standalone', route => route.fulfill({
    contentType: 'text/html',
    headers: { 'Content-Security-Policy': "default-src 'none'; script-src 'self' 'wasm-unsafe-eval'; style-src 'self'; connect-src 'self'; img-src 'self' data:; font-src 'self';" },
    body: `<!doctype html><html><head><link rel="stylesheet" href="${cssURL}"><link rel="icon" href="data:image/svg+xml,%3Csvg/%3E"></head><body></body></html>`,
  }));
  await page.goto('http://127.0.0.1:61138/standalone');
  const optional = null;
  await page.evaluate(async moduleURL => {
    const base = new URL(moduleURL, location.origin);
    const { boot } = await import(new URL('dom-loader.js', base).href);
    const started = await boot({
      glue_url: new URL('app.js', base).href,
      wasm_url: new URL('app_bg.wasm', base).href,
    });
    started.app.mount_branches();
  }, moduleURL);
  const state = () => page.evaluate(() => {
    const root = document.getElementById('branch-probe');
    const bars = {};
    for (const name of ['a', 'b']) {
      const track = root.querySelector(`[data-testid="bar-${name}"]`);
      if (track) bars[name] = {
        track: track.getBoundingClientRect().width,
        fill: track.firstElementChild.getBoundingClientRect().width,
        class: track.firstElementChild.className,
        width: getComputedStyle(track.firstElementChild).width,
      };
    }
    return {
      marker: root.getAttribute('data-rustify-geometry-scope'), bars,
      rules: [...document.adoptedStyleSheets].flatMap(sheet => [...sheet.cssRules].map(rule => rule.cssText)),
    };
  });
  await page.locator('#open-a').click();
  await page.waitForTimeout(350);
  const onlyA = await state();
  await page.locator('#open-b').click();
  await page.waitForTimeout(350);
  const both = await state();
  await page.screenshot({ path: new URL("./probe-csr-both.png", import.meta.url).pathname, fullPage: true });
  await page.locator('#open-b').click();
  await page.waitForTimeout(350);
  const closeB = await state();
  const result = { browser: browser.version(), url: page.url(), optional, onlyA, both, closeB, failures };
  await writeFile(new URL('./probe-csr.json', import.meta.url), JSON.stringify(result, null, 2));
  await page.screenshot({ path: new URL('./probe-csr-recovered.png', import.meta.url).pathname, fullPage: true });
  console.log(JSON.stringify(result, null, 2));
} finally {
  await browser.close();
}
