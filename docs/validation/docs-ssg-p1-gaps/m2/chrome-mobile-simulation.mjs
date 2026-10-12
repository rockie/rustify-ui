import { chromium, devices, expect } from '@playwright/test';
import { createHash } from 'node:crypto';
import { mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import path from 'node:path';

const evidence = path.dirname(new URL(import.meta.url).pathname);
const output = path.join(evidence, 'chrome-mobile-simulation');
mkdirSync(output, { recursive: true });
const previews = JSON.parse(readFileSync(path.join(evidence, 'recovered-previews.json'))).previews;
const matrix = JSON.parse(readFileSync(path.join(evidence, 'matrix-artifacts.json'))).sites;
const entry = name => previews.find(p => p.name === name && p.bind === '127.0.0.1').url;
const report = { observedAt: new Date().toISOString(), channel: 'chrome', physicalDevice: false, runs: [] };
const browser = await chromium.launch({ channel: 'chrome', headless: true });
report.version = browser.version();
try {
  for (const name of ['iPhone 14 Pro', 'Pixel 7']) {
    const { viewport, screen, deviceScaleFactor, isMobile, hasTouch } = devices[name];
    for (const suffix of ['root', 'subpath']) for (const script of [true, false]) {
      const full = entry(`full-${suffix}`), home = entry(`home-${suffix}`);
      const run = { profile: name, viewport, screen, deviceScaleFactor, isMobile, hasTouch,
        userAgentOverride: false, javaScriptEnabled: script, base: suffix, steps: [], screenshots: [],
        console: [], pageErrors: [], failedRequests: [], errorResponses: [] };
      report.runs.push(run);
      const context = await browser.newContext({ viewport, screen, deviceScaleFactor, isMobile, hasTouch,
        javaScriptEnabled: script, reducedMotion: 'reduce', colorScheme: 'light' });
      const page = await context.newPage();
      page.setDefaultTimeout(8000);
      page.on('console', m => run.console.push({ type: m.type(), text: m.text(), location: m.location() }));
      page.on('pageerror', e => run.pageErrors.push(e.message));
      page.on('requestfailed', r => run.failedRequests.push({ url: r.url(), error: r.failure()?.errorText }));
      page.on('response', r => {
        if (r.status() >= 400) run.errorResponses.push({ url: r.url(), status: r.status(), type: r.request().resourceType() });
      });
      async function step(label, action) {
        await action();
        run.steps.push({ label, result: 'passed', url: page.url() });
      }
      async function fits() {
        expect(await page.evaluate(() => document.documentElement.scrollWidth - innerWidth)).toBeLessThanOrEqual(0);
      }
      async function belowHeader(target) {
        await expect(target).toBeInViewport();
        expect((await target.boundingBox()).y).toBeGreaterThanOrEqual((await page.getByRole('banner').boundingBox()).height);
      }
      async function landmarks() {
        for (const role of ['banner', 'main', 'contentinfo']) await expect(page.getByRole(role)).toHaveCount(1);
        await expect(page.getByRole('heading', { level: 1 })).toHaveCount(1);
        expect(await page.getByRole('navigation').count()).toBeGreaterThan(0);
        await fits();
      }
      async function shot(label) {
        if (!script) return;
        const file = `${name.replaceAll(' ', '-')}-${suffix}-${label}.png`;
        await page.screenshot({ path: path.join(output, file), fullPage: true });
        run.screenshots.push(file);
      }
      try {
        await step('Fixed production fingerprints', async () => {
          for (const kind of ['full', 'home']) {
            const site = matrix.find(m => m.name === `${kind}-${suffix}`);
            const response = await context.request.get(entry(site.name) + 'rustify-docs-manifest.json');
            expect(response.status()).toBe(200);
            expect(createHash('sha256').update(await response.body()).digest('hex')).toBe(site.manifest_sha256);
          }
        });
        await step('Homepage and touch reading entry', async () => {
          expect((await page.goto(full)).status()).toBe(200);
          await landmarks();
          await page.getByRole('link', { name: 'Get started', exact: true }).tap();
          await expect(page).toHaveURL(full + 'guide/');
          await page.locator('article').getByRole('link', { name: 'setup', exact: true }).tap();
          await expect(page).toHaveURL(full + 'guide/setup/#install-the-cli');
          await landmarks();
        });
        await step('Touch contents and ordinary anchor', async () => {
          const toc = page.locator('details.rd-toc-narrow');
          await toc.locator('summary').tap();
          await toc.getByRole('link', { name: 'Check the version' }).tap();
          if (script) await expect(toc).not.toHaveAttribute('open', '');
          await belowHeader(page.locator('h3#check-the-version'));
          if (!script) await toc.locator('summary').tap();
        });
        await step('Copy and code/table scroll containment', async () => {
          expect(await page.locator('.rd-table').evaluate(n => getComputedStyle(n).overflowX)).toBe('auto');
          expect(await page.locator('.rd-code-pre').first().evaluate(n => getComputedStyle(n).overflowX)).toBe('auto');
          if (script) {
            await context.grantPermissions(['clipboard-read', 'clipboard-write']);
            const figure = page.locator('figure.rd-code', { hasText: 'src/example.rs' });
            await figure.locator('[data-rd-copy]').tap();
            await expect(figure.locator('[data-rd-copy]')).toHaveAttribute('data-state', 'copied');
            expect(await page.evaluate(() => navigator.clipboard.readText())).toBe('fn main() {\n    println!("hello");\n}\n');
          } else {
            await expect(page.getByRole('button', { name: 'Copy', exact: true })).toHaveCount(0);
            await expect(page.locator('[data-rd-mode-switch]')).toBeHidden();
          }
          await fits();
          await shot('setup');
        });
        await step('Theme persists through pager and sidebar', async () => {
          if (script) await page.locator('[data-rd-mode="dark"]').tap();
          await page.locator('.rd-pager a').first().tap();
          await expect(page).toHaveURL(full + 'guide/');
          if (script) await expect(page.locator('html')).toHaveAttribute('data-theme', 'dark');
          await page.locator('details.rd-sidebar-narrow > summary').tap();
          await page.locator('details.rd-sidebar-narrow').getByRole('link', { name: 'Setup', exact: true }).tap();
          await expect(page).toHaveURL(full + 'guide/setup/');
          if (script) {
            await expect(page.locator('html')).toHaveAttribute('data-theme', 'dark');
            await page.locator('[data-rd-mode="light"]').tap();
          }
        });
        await step('Header navigation and Chinese anchor', async () => {
          await page.getByRole('navigation', { name: 'Site links' }).getByRole('link', { name: 'Guide', exact: true }).tap();
          await expect(page).toHaveURL(full + 'guide/');
          await page.locator('article').getByRole('link', { name: 'reference', exact: true }).tap();
          await belowHeader(page.locator('[id="数据-api"]'));
        });
        await step('Deep 404 and return', async () => {
          expect((await page.goto(full + 'a/b/c/missing/')).status()).toBe(404);
          await landmarks();
          await page.getByRole('main').getByRole('link', { name: 'Back to the home page' }).tap();
          await expect(page).toHaveURL(full);
        });
        for (const [route, id] of [['', 'home'], ['explicit/', '入口'], ['raw/', '</h1>']]) {
          await step(`Home ${route || 'automatic'} direct/same/cross anchors`, async () => {
            await page.goto(home + route + '#' + encodeURIComponent(id));
            await landmarks();
            const h1 = page.getByRole('heading', { level: 1 });
            await expect(h1).toHaveAttribute('id', id);
            await belowHeader(h1);
            if (route === 'raw/') {
              await expect(page.locator('article')).toHaveText('Same Automatic');
              await expect(page.getByRole('link', { name: 'Lire', exact: true })).toHaveAttribute('lang', 'fr');
            }
            await shot(route ? route.slice(0, -1) : 'automatic');
            await page.locator('article').getByRole('link', { name: 'Same', exact: true }).tap();
            await belowHeader(h1);
            await page.locator('article').getByRole('link').last().tap();
            await belowHeader(page.getByRole('heading', { level: 1 }));
          });
        }
        await step('Empty home landmarks and footer navigation', async () => {
          await page.goto(home + 'empty/');
          await landmarks();
          const footer = page.getByRole('contentinfo');
          await expect(footer.getByRole('navigation', { name: 'Footer links' })).toHaveCount(1);
          await shot('empty');
          await footer.getByRole('link', { name: 'Back to the home page' }).tap();
          await expect(page).toHaveURL(home);
        });
        await step('Reduced motion and landscape layout', async () => {
          expect(await page.evaluate(() => matchMedia('(prefers-reduced-motion: reduce)').matches)).toBe(true);
          expect(await page.locator('html').evaluate(n => getComputedStyle(n).scrollBehavior)).toBe('auto');
          await page.setViewportSize({ width: screen.height, height: viewport.width });
          await page.goto(full + 'guide/setup/');
          await landmarks();
        });
        expect(run.pageErrors).toEqual([]);
        expect(run.failedRequests).toEqual([]);
        expect(run.errorResponses.filter(r => !(r.status === 404 && (r.type === 'document' && r.url.endsWith('/missing/') || r.url.endsWith('/favicon.ico'))))).toEqual([]);
        run.result = 'passed';
      } finally { await context.close(); }
    }
  }
  report.result = 'passed';
} catch (error) {
  report.result = 'failed';
  report.error = String(error.stack || error);
  process.exitCode = 1;
} finally {
  await browser.close();
  writeFileSync(path.join(output, 'results.json'), JSON.stringify(report, null, 2) + '\n');
}
console.log(JSON.stringify({ result: report.result, version: report.version, runs: report.runs.length,
  steps: report.runs.reduce((n, r) => n + r.steps.length, 0), error: report.error }));
