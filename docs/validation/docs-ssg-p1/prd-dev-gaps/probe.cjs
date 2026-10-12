// Reproduce PRD coverage differences without changing product sources.
const fs = require('node:fs');
const path = require('node:path');
const os = require('node:os');
const { spawn, spawnSync } = require('node:child_process');
const { chromium } = require('playwright');

const root = process.cwd();
const evidence = __dirname;
const bin = path.resolve(process.env.RUSTIFY_DOCS_BIN || 'target/docs-cli/rustify-docs');
const work = fs.mkdtempSync(path.join(os.tmpdir(), 'rustify-prd-gaps-'));
const results = { binary: bin, work, cases: [] };

function command(args, cwd = root) {
  const r = spawnSync(bin, args, { cwd, encoding: 'utf8', timeout: 30000 });
  return { args, exit: r.status, stdout: r.stdout, stderr: r.stderr };
}

async function preview(dir) {
  const child = spawn(bin, ['preview', '--dir', dir, '--port', '0'], { stdio: ['ignore', 'pipe', 'pipe'] });
  let log = '';
  const url = await new Promise((resolve, reject) => {
    const timeout = setTimeout(() => reject(new Error(`preview timeout: ${log}`)), 10000);
    child.on('exit', code => { clearTimeout(timeout); reject(new Error(`preview exit ${code}: ${log}`)); });
    function read(chunk) {
      log += chunk;
      const match = log.match(/http:\/\/127\.0\.0\.1:\d+[^\s]*/);
      if (match) { clearTimeout(timeout); resolve(match[0]); }
    }
    child.stdout.on('data', read);
    child.stderr.on('data', read);
  }).catch(error => { child.kill(); throw error; });
  return { child, url, log: () => log };
}

async function inspect(browser, name, site, base, paths, after) {
  const config = path.join(site, 'rustify-docs.toml');
  const checked = command(['check', '--config', config, '--format', 'json']);
  const out = path.join(site, 'dist');
  const built = command(['build', '--config', config, '--out', out, '--format', 'json']);
  if (checked.exit !== 0 || built.exit !== 0) throw new Error(`${name}: ${JSON.stringify({ checked, built })}`);
  const server = await preview(out);
  const context = await browser.newContext({ viewport: { width: 1280, height: 720 } });
  const page = await context.newPage();
  const errors = { console: [], pageerror: [], failedRequests: [] };
  page.on('console', event => { if (event.type() === 'error') errors.console.push(event.text()); });
  page.on('pageerror', error => errors.pageerror.push(error.message));
  page.on('requestfailed', request => errors.failedRequests.push({ url: request.url(), error: request.failure() }));
  const record = { name, base, checked, built, url: server.url, pages: [], errors };
  try {
    for (const route of paths) {
      const response = await page.goto(new URL(route, server.url).href);
      const dom = await page.evaluate(() => ({
        h1: Array.from(document.querySelectorAll('h1')).map(node => ({ text: node.textContent, id: node.id })),
        footerElements: Array.from(document.querySelectorAll('footer')).map(node => ({ inMain: !!node.closest('main'), text: node.textContent })),
        navElements: document.querySelectorAll('nav').length,
        ids: Array.from(document.querySelectorAll('[id]')).map(node => node.id),
      }));
      record.pages.push({ route, status: response.status(), ...dom,
        roles: { banner: await page.getByRole('banner').count(), navigation: await page.getByRole('navigation').count(), main: await page.getByRole('main').count(), contentinfo: await page.getByRole('contentinfo').count() },
      });
    }
    if (after) await after(page, record, server.url);
    await page.screenshot({ path: path.join(evidence, name + '-1280.png'), fullPage: true });
    await page.setViewportSize({ width: 375, height: 720 });
    await page.screenshot({ path: path.join(evidence, name + '-375.png'), fullPage: true });
    record.previewLog = server.log();
    results.cases.push(record);
    fs.writeFileSync(path.join(evidence, 'probe-results.json'), JSON.stringify(results, null, 2) + '\n');
    console.log(JSON.stringify({ name, pages: record.pages.map(p => ({ route: p.route, h1: p.h1.length, roles: p.roles })), anchor: record.anchor }));
  } finally {
    await context.close();
    server.child.kill();
    await new Promise(resolve => server.child.once('exit', resolve));
  }
}

(async () => {
  const browser = await chromium.launch({ executablePath: process.env.RUSTIFY_CHROMIUM || undefined });
  results.browser = browser.version();
  try {
    const template = path.join(work, 'template');
    const created = command(['new', template]);
    if (created.exit !== 0) throw new Error(JSON.stringify(created));
    results.new = created;
    await inspect(browser, 'template-landmarks', template, '/', ['', 'guide/getting-started/', 'x/y/']);

    for (const base of ['/', '/rustify-ui/']) {
      for (const explicit of [false, true]) {
        const name = (base === '/' ? 'root' : 'subpath') + (explicit ? '-home-explicit' : '-home-auto');
        const site = path.join(work, name);
        fs.cpSync(path.join(root, 'tests/docs-ssg/fixtures/content'), site, { recursive: true });
        const config = path.join(site, 'rustify-docs.toml');
        fs.writeFileSync(config, fs.readFileSync(config, 'utf8').replace('https://example.com/docs/', 'https://example.com' + base).replace('base = "/docs/"', 'base = "' + base + '"'));
        const id = explicit ? 'welcome' : 'fixture-docs';
        const home = path.join(site, 'content/index.md');
        let source = fs.readFileSync(home, 'utf8');
        if (explicit) source = source.replace('# Fixture Docs\n', '# Fixture Docs {#welcome}\n');
        source += '\n[Same-page home heading](#' + id + ')\n';
        fs.writeFileSync(home, source);
        const guide = path.join(site, 'content/guide/setup.md');
        fs.appendFileSync(guide, '\n[Cross-page home heading](../index.md#' + id + ')\n');
        await inspect(browser, name, site, base, ['', 'guide/setup/'], async (page, record, url) => {
          await page.getByRole('link', { name: 'Cross-page home heading' }).click();
          await page.waitForLoadState('load');
          await page.locator('h1').first().waitFor({ state: 'visible' });
          record.anchor = await page.evaluate(target => ({ url: location.href, target, exists: document.getElementById(target) !== null, h1: Array.from(document.querySelectorAll('h1')).map(n => ({ text: n.textContent, id: n.id })) }), id);
        });
      }
    }

    const raw = path.join(work, 'raw-heading');
    fs.cpSync(template, raw, { recursive: true });
    fs.writeFileSync(path.join(raw, 'content/guide/raw.md'), '---\ntitle: Raw heading\ndescription: Heading semantics probe.\n---\n\n# Markdown title\n\n<h1 id="second">Second title</h1>\n\n<h4 id="skipped">Skipped heading</h4>\n\nText.\n');
    await inspect(browser, 'raw-heading', raw, '/', ['guide/raw/']);
  } finally { await browser.close(); }
})().catch(error => { console.error(error); process.exitCode = 1; });
