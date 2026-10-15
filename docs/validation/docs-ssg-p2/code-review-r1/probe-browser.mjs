import { chromium } from '/Users/rockie/Documents/gh-rockie/rustify-ui/node_modules/@playwright/test/index.mjs';
import { writeFile } from 'node:fs/promises';

const browser = await chromium.launch({ headless: true });
const page = await browser.newPage({ viewport: { width: 1280, height: 900 } });
const failures = [];
page.on('pageerror', error => failures.push(String(error)));
page.on('console', message => { if (message.type() === 'error') failures.push(message.text()); });
page.on('requestfailed', request => failures.push(`${request.url()}: ${request.failure()?.errorText}`));
try {
  await page.goto('http://127.0.0.1:50434/');
  await page.waitForFunction(() => window.__rustify_docs_islands?.state() === 'ready');
  const demo = page.locator('[data-rustify-island]').nth(1);
  await demo.locator('[data-testid="counter-increment"]').click();
  const optional = await demo.locator('output').evaluate(node => ({
    text: node.textContent, class: node.className, background: getComputedStyle(node).backgroundColor,
  }));
  await page.evaluate(async () => {
    const moduleURL = document.querySelector('[data-rd-islands-module]').src;
    const app = await import(new URL('app.js', moduleURL).href);
    app.mount_branches();
  });
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
  await page.locator('#open-b').click();
  await page.waitForTimeout(350);
  const closeB = await state();
  const result = { browser: browser.version(), url: page.url(), optional, onlyA, both, closeB, failures };
  await writeFile(new URL('./probe-browser.json', import.meta.url), JSON.stringify(result, null, 2));
  await page.screenshot({ path: new URL('./probe-browser.png', import.meta.url).pathname, fullPage: true });
  console.log(JSON.stringify(result, null, 2));
} finally {
  await browser.close();
}
