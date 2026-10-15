const { chromium } = require(process.cwd() + '/node_modules/playwright');
const { spawn } = require('node:child_process');
const fs = require('node:fs');
const path = require('node:path');
const os = require('node:os');
const assert = require('node:assert/strict');
const out = __dirname;
const identity = JSON.parse(fs.readFileSync(path.join(out, 'installed-before.json')));
const site = fs.mkdtempSync(path.join(os.tmpdir(), 'rustify-gaps2-content-browser-'));
fs.mkdirSync(path.join(site, 'content'));
const good = '---\ndescription: Home\n---\n# Home\n';
const config = "schema_version=1\n[site]\nname='Review'\nurl='https://example.com/'\n";
fs.writeFileSync(path.join(site, 'rustify-docs.toml'), config);
fs.writeFileSync(path.join(site, 'content/index.md'), good);
fs.writeFileSync(path.join(site, 'content/a`b`c.md'), '---\ndescription: Target\n---\n# Target\n');
const child = spawn(identity.entry, ['dev', '--port', '0'], { cwd: site, detached: true });
let stdout = '', stderr = '';
child.stdout.on('data', b => stdout += b);
child.stderr.on('data', b => stderr += b);
const pause = ms => new Promise(r => setTimeout(r, ms));
async function until(get, check, label) { for (let n = 0; n < 200; n++) { const value = await get(); if (check(value)) return value; await pause(100); } throw Error('timeout: ' + label); }
(async () => {
 let browser;
 const results = { binary: identity, site, cases: [], audit: { console: [], pageErrors: [], failures: [], csp: [], responses: [] } };
 try {
  const url = await until(() => /^dev: (\S+)/m.exec(stdout)?.[1], Boolean, 'dev address');
  browser = await chromium.launch({ executablePath: process.env.RUSTIFY_CHROMIUM || undefined });
  results.browserVersion = browser.version(); results.url = url;
  const page = await browser.newPage({ viewport: { width: 1280, height: 900 } });
  page.on('console', m => { if (m.type() === 'error') results.audit.console.push(m.text()); });
  page.on('pageerror', e => results.audit.pageErrors.push(String(e)));
  page.on('requestfailed', r => { if (!(r.url().includes('/__rustify-docs/events') && r.failure()?.errorText === 'net::ERR_ABORTED')) results.audit.failures.push({ url: r.url(), error: r.failure() }); });
  page.on('response', r => { if (r.status() >= 400) results.audit.responses.push({ url: r.url(), status: r.status() }); });
  await page.exposeBinding('__reviewCsp', (_, m) => results.audit.csp.push(m));
  await page.addInitScript(() => document.addEventListener('securitypolicyviolation', e => window.__reviewCsp(e.violatedDirective + ' ' + e.blockedURI)));
  await page.goto(url); await page.getByRole('heading', { name: 'Home', exact: true }).waitFor();
  const read = async rule => {
   const node = page.locator('.rd-dev-item').filter({ has: page.locator('.rd-dev-rule', { hasText: rule }) });
   if (await node.count() !== 1) return null;
   return node.evaluate(n => ({ message: n.querySelector('.rd-dev-message').textContent, messageCodes: [...n.querySelectorAll('.rd-dev-message code')].map(c => c.textContent), suggestion: n.querySelector('.rd-dev-suggestion')?.textContent, suggestionCodes: [...n.querySelectorAll('.rd-dev-suggestion code')].map(c => c.textContent) }));
  };
  const cases = [
   { label: 'anchor', rule: 'broken-anchor', markdown: good + '\n[Target](a%60b%60c.md#a%60b)\n', config, message: 'content/a`b`c.md has no heading #a`b', codes: ['content/a`b`c.md', '#a`b'] },
   { label: 'yaml', rule: 'embed-payload', markdown: good + '\n```rustify-component\nname: Steps\nun`known: 1\n```\n', config, message: 'unknown field un`known; a demo takes only name, a component takes name, props and children', codes: ['un`known', 'name', 'name', 'props', 'children'] },
   { label: 'empty-language', rule: 'config-lang', markdown: good, config: config + "lang=''\n", message: ' is not a language tag', codes: [''] },
   { label: 'frontmatter-at-line', rule: 'lang', markdown: '---\ndescription: Home\nlang: "a` at line `b"\n---\n# Home\n', config, message: 'lang: `` a`', codes: [] }
  ];
  for (const entry of cases) {
   fs.writeFileSync(path.join(site, 'content/index.md'), entry.markdown);
   fs.writeFileSync(path.join(site, 'rustify-docs.toml'), entry.config);
   const actual = await until(() => read(entry.rule), v => v?.message === entry.message, entry.label);
   assert.deepEqual(actual.messageCodes, entry.codes);
   const views = [];
   for (const width of [1280, 375]) for (const theme of ['light', 'dark']) {
    await page.setViewportSize({ width, height: 900 });
    await page.locator(`[data-rd-mode="${theme}"]`).click();
    await until(() => page.locator('html').getAttribute('data-theme'), t => t === theme, 'theme');
    const layout = await page.evaluate(() => ({ page: document.documentElement.scrollWidth - innerWidth, prose: [...document.querySelectorAll('.rd-dev-message, .rd-dev-suggestion')].map(n => n.scrollWidth - n.clientWidth) }));
    assert.ok(layout.page <= 0 && layout.prose.every(x => x <= 0));
    const screenshot = `${entry.label}-${width}-${theme}.png`;
    await page.screenshot({ path: path.join(out, screenshot) });
    views.push({ width, theme, actual: await read(entry.rule), layout, screenshot });
   }
   results.cases.push({ ...entry, actual, views });
  }
  const hide = page.getByRole('button', { name: 'Hide problems' });
  await hide.click(); assert.ok(await page.locator('.rd-dev-marker').evaluate(n => document.activeElement === n));
  await page.keyboard.press('Enter'); assert.ok(await hide.evaluate(n => document.activeElement === n));
  await page.keyboard.press('Escape'); assert.ok(await page.locator('.rd-dev-marker').evaluate(n => document.activeElement === n));
  fs.writeFileSync(path.join(site, 'content/index.md'), good);
  await until(() => page.locator('.rd-dev').evaluate(n => ({ hidden: n.hidden, revision: Number(n.getAttribute('data-revision')) })), s => s.hidden && s.revision >= 6, 'recover');
  await page.screenshot({ path: path.join(out, 'content-recovered.png') });
  results.recovered = true;
  for (const values of Object.values(results.audit)) assert.deepEqual(values, []);
  results.observationsConfirmed = true;
 } finally {
  fs.writeFileSync(path.join(out, 'content-browser.json'), JSON.stringify(results, null, 2) + '\n');
  fs.writeFileSync(path.join(out, 'content-dev.stderr'), stderr);
  if (browser) await browser.close();
  if (child.exitCode === null) process.kill(-child.pid, 'SIGTERM');
 }
 process.stdout.write(JSON.stringify({ browser: results.browserVersion, cases: results.cases.map(c => ({ label: c.label, text: c.actual.message, codes: c.actual.messageCodes })), recovered: results.recovered, audit: results.audit }) + '\n');
})().catch(error => { process.stderr.write(error.stack + '\n'); process.exitCode = 1; if (child.exitCode === null) { try { process.kill(-child.pid, 'SIGTERM'); } catch {} } });
