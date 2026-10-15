// Real browser checks for the same project created by author-acceptance.py.
const { chromium } = require(process.cwd() + '/node_modules/playwright');
const fs = require('node:fs');
const path = require('node:path');
const readline = require('node:readline');
const assert = require('node:assert/strict');
(async () => {
  const [url, out] = process.argv.slice(2);
  fs.mkdirSync(out, { recursive: true });
  const browser = await chromium.launch({ executablePath: process.env.RUSTIFY_CHROMIUM || undefined });
  const page = await browser.newPage({ viewport: { width: 1280, height: 900 } });
  const audit = { console: [], pageErrors: [], failures: [], responses: [], csp: [] };
  page.on('console', msg => { if (msg.type() === 'error') audit.console.push(msg.text()); });
  page.on('pageerror', error => audit.pageErrors.push(String(error)));
  page.on('requestfailed', request => { if (!(request.url().includes('/__rustify-docs/events') && request.failure()?.errorText === 'net::ERR_ABORTED')) audit.failures.push([request.url(), request.failure()]); });
  page.on('response', response => { if (response.status() >= 400) audit.responses.push([response.url(), response.status()]); });
  await page.exposeBinding('__gaps2Csp', (_, message) => audit.csp.push(message));
  await page.addInitScript(() => document.addEventListener('securitypolicyviolation', event => window.__gaps2Csp(`${event.violatedDirective} ${event.blockedURI}`)));
  const reply = value => process.stdout.write(JSON.stringify(value) + '\n');
  await page.goto(url);
  await page.waitForFunction(() => [...document.querySelectorAll('[data-rustify-island]')].every(node => node.getAttribute('data-rd-island-state') === 'ready'));
  await page.getByTestId('counter-input').first().fill('41');
  await page.getByTestId('counter-value').first().waitFor();
  assert.equal(await page.getByTestId('counter-value').first().textContent(), '41');
  await page.evaluate(() => { window.__gaps2Original = document.querySelector('article'); });
  reply({ ready: true });
  const commands = readline.createInterface({ input: process.stdin });
  try {
    for await (const line of commands) {
      const command = JSON.parse(line);
      if (command.action === 'show') {
        const panel = page.locator('.rd-dev-panel');
        await panel.waitFor({ state: 'visible' });
        await page.waitForFunction(revision => Number(document.querySelector('.rd-dev')?.getAttribute('data-revision')) === revision, command.revision);
        const item = page.locator('.rd-dev-item').filter({ has: page.locator('.rd-dev-rule', { hasText: command.rule }) });
        assert.equal(await item.count(), 1);
        const read = () => item.evaluate(node => ({ message: node.querySelector('.rd-dev-message').textContent, messageCodes: [...node.querySelectorAll('.rd-dev-message code')].map(code => code.textContent), suggestion: node.querySelector('.rd-dev-suggestion')?.textContent || null, suggestionCodes: [...node.querySelectorAll('.rd-dev-suggestion code')].map(code => code.textContent) }));
        const actual = await read();
        assert.deepEqual(actual, command.expected);
        assert.equal(await page.evaluate(() => window.__gaps2Original === document.querySelector('article')), true);
        assert.equal(await page.getByTestId('counter-value').first().textContent(), '41');
        const views = [];
        for (const width of [1280, 375]) {
          for (const theme of ['light', 'dark']) {
            await page.setViewportSize({ width, height: 900 });
            await page.locator(`[data-rd-mode="${theme}"]`).click();
            await page.waitForFunction(theme => document.documentElement.getAttribute('data-theme') === theme, theme);
            const layout = await page.evaluate(() => ({ page: document.documentElement.scrollWidth - innerWidth, prose: [...document.querySelectorAll('.rd-dev-message, .rd-dev-suggestion')].map(node => node.scrollWidth - node.clientWidth), fonts: [...document.querySelectorAll('.rd-dev-item code')].map(node => getComputedStyle(node).fontFamily) }));
            assert.ok(layout.page <= 0 && layout.prose.every(extra => extra <= 0));
            assert.deepEqual(await read(), command.expected);
            const screenshot = `${command.label}-${width}-${theme}.png`;
            await page.screenshot({ path: path.join(out, screenshot) });
            views.push({ width, theme, actual: await read(), layout, screenshot });
          }
        }
        const hide = page.getByRole('button', { name: 'Hide problems' });
        await hide.click();
        assert.equal(await page.locator('.rd-dev-marker').evaluate(node => document.activeElement === node), true);
        await page.keyboard.press('Enter');
        assert.equal(await hide.evaluate(node => document.activeElement === node), true);
        await page.keyboard.press('Escape');
        assert.equal(await page.locator('.rd-dev-marker').evaluate(node => document.activeElement === node), true);
        await page.locator('.rd-dev-marker').click();
        fs.writeFileSync(path.join(out, command.label + '.json'), JSON.stringify({ command, actual, views }, null, 2) + '\n');
        reply({ passed: command.label, views: views.length });
      } else if (command.action === 'clear') {
        await page.waitForFunction(revision => Number(document.querySelector('.rd-dev')?.getAttribute('data-revision')) === revision && document.querySelector('h1') && document.querySelectorAll('.rd-dev-item[data-severity="error"]').length === 0, command.revision);
        await page.waitForFunction(() => document.querySelectorAll('[data-rustify-island]').length > 0 && [...document.querySelectorAll('[data-rustify-island]')].every(node => node.getAttribute('data-rd-island-state') === 'ready'));
        await page.screenshot({ path: path.join(out, 'cleared.png') });
        const before = await page.locator('.rd-dev-marker').count();
        fs.writeFileSync(path.join(out, 'audit.json'), JSON.stringify(audit, null, 2) + '\n');
        for (const values of Object.values(audit)) assert.deepEqual(values, []);
        reply({ passed: 'clear', audit, marker: before });
      } else if (command.action === 'stop') {
        reply({ stopped: true }); break;
      }
    }
  } finally { await browser.close(); }
})().catch(error => { process.stderr.write(String(error.stack || error) + '\n'); process.exitCode = 1; });
