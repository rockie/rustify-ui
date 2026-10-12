const fs = require('node:fs');
const path = require('node:path');
const { spawn } = require('node:child_process');
const [repoArg, rootArg, binaryArg] = process.argv.slice(2);
const repo = path.resolve(repoArg), root = path.resolve(rootArg), binary = path.resolve(binaryArg);
const { chromium } = require(path.join(repo, 'node_modules/playwright'));

async function server(name, mode = 'preview') {
  const dir = path.join(root, 'cases', name);
  const args = mode === 'preview' ? ['preview', '--dir', path.join(dir, 'dist'), '--port', '0'] : ['dev', '--config', path.join(dir, 'rustify-docs.toml'), '--port', '0'];
  const child = spawn(binary, args, { cwd: repo });
  let log = '';
  const url = await new Promise((resolve, reject) => {
    const timer = setTimeout(() => { child.kill(); reject(new Error('No address: ' + log)); }, 15000);
    const received = (chunk) => {
      log += chunk.toString();
      const address = log.match(/http:\/\/127\.0\.0\.1:\d+\/docs\//);
      if (address) { clearTimeout(timer); resolve(address[0]); }
    };
    child.stdout.on('data', received);
    child.stderr.on('data', received);
    child.once('error', reject);
    child.once('exit', (code) => { clearTimeout(timer); reject(new Error('Server stopped ' + code + ': ' + log)); });
  });
  return { url, child, close: () => { child.kill(); fs.writeFileSync(path.join(root, name + '-server.log'), log); } };
}

(async () => {
  const browser = await chromium.launch({ executablePath: process.env.RUSTIFY_CHROMIUM || undefined });
  const results = { browser: await browser.version(), cases: [] };
  fs.mkdirSync(path.join(root, 'screenshots'), { recursive: true });
  try {
    for (const name of ['header-links', 'home-resource', 'css-external', 'css-local', 'svg-fragment', 'toc-percent', 'watch-resource']) {
      const host = await server(name, name === 'watch-resource' ? 'dev' : 'preview');
      const page = await browser.newPage({ viewport: { width: 1440, height: 900 } });
      const errors = [], consoleErrors = [], failed = [];
      const result = { name, entry: host.url, errors, consoleErrors, failed };
      page.on('pageerror', e => errors.push(e.message));
      page.on('console', m => { if (m.type() === 'error') consoleErrors.push(m.text()); });
      page.on('response', r => { if (r.status() >= 400) failed.push({ url: r.url(), status: r.status() }); });
      await page.addInitScript(() => {
        window.reviewCsp = [];
        document.addEventListener('securitypolicyviolation', e => window.reviewCsp.push({ blocked: e.blockedURI, directive: e.effectiveDirective }));
      });
      try {
        result.status = (await page.goto(host.url)).status();
        await page.waitForTimeout(200);
        result.csp = await page.evaluate(() => window.reviewCsp);
        if (name === 'header-links') {
          result.topCount = await page.getByRole('link', { name: 'Top link visible', exact: true }).count();
          result.socialCount = await page.getByRole('link', { name: 'Social link visible', exact: true }).count();
        }
        if (name === 'home-resource') {
          result.href = await page.getByRole('link', { name: 'Download manual', exact: true }).getAttribute('href');
          const response = await Promise.all([page.waitForNavigation(), page.getByRole('link', { name: 'Download manual', exact: true }).click()]);
          result.downloadStatus = response[0].status();
          result.downloadUrl = page.url();
        }
        if (name === 'svg-fragment') {
          const box = () => page.locator('article svg use').evaluate(el => { const b = el.getBBox(); return { width: b.width, height: b.height, href: el.getAttribute('href') }; });
          result.before = await box();
          await page.screenshot({ path: path.join(root, 'screenshots/svg-fragment-before.png') });
          // Browser-only counterexample: the same published file renders once its fragment is restored.
          await page.locator('article svg use').evaluate(el => el.setAttribute('href', el.getAttribute('href') + '#mark'));
          await page.waitForTimeout(100);
          result.withFragmentRestored = await box();
        }
        if (name === 'toc-percent') {
          result.tocHrefs = await page.locator('[data-rd-toc-link]').evaluateAll(els => els.map(e => e.getAttribute('href')));
          result.activeToc = await page.locator('[data-rd-toc-link][aria-current]').count();
          await page.setViewportSize({ width: 375, height: 812 });
          const menu = page.locator('details.rd-sidebar-narrow');
          await menu.locator('summary').click();
          await menu.locator('summary').press('Escape');
          result.menuStillOpenAfterEscape = await menu.evaluate(e => e.open);
        }
        if (name === 'watch-resource') {
          result.before = await page.locator('article img').getAttribute('src');
          const image = path.join(root, 'cases/watch-resource/images/mark.svg');
          fs.writeFileSync(image, fs.readFileSync(image, 'utf8').replace('red', 'blue'));
          await page.waitForTimeout(1600);
          // Reloading rules out a missing client-side navigation: the server itself is still serving the old generation.
          await page.reload();
          result.afterResourceOnlySave = await page.locator('article img').getAttribute('src');
          const markdown = path.join(root, 'cases/watch-resource/content/index.md');
          fs.appendFileSync(markdown, '\nA content save to trigger the watcher.\n');
          await page.waitForTimeout(1000);
          await page.reload();
          result.afterContentSave = await page.locator('article img').getAttribute('src');
        }
        await page.screenshot({ path: path.join(root, 'screenshots', name + '.png'), fullPage: true });
        results.cases.push(result);
      } finally { await page.close(); host.close(); }
    }
  } finally { await browser.close(); }
  fs.writeFileSync(path.join(root, 'browser-results.json'), JSON.stringify(results, null, 2));
  process.stdout.write(JSON.stringify(results, null, 2) + '\n');
})().catch(e => { console.error(e); process.exitCode = 1; });
