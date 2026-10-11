// Run after the R2 replay; every server serves the newly built production output.
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const { execFileSync, spawn } = require('node:child_process');
const { chromium } = require(path.resolve('node_modules/playwright'));
const { PNG } = require(path.resolve('node_modules/pngjs'));
const work = process.argv[2];
const evidence = __dirname;
const binary = path.resolve('target/docs-cli/rustify-docs');

async function preview(root) {
  const child = spawn(binary, ['preview', '--dir', path.join(root, 'dist'), '--port', '0']);
  let log = '';
  const url = await new Promise((resolve, reject) => {
    const timer = setTimeout(() => { child.kill(); reject(new Error(log)); }, 15000);
    const read = chunk => {
      log += chunk.toString();
      const match = log.match(/http:\/\/127\.0\.0\.1:\d+\/docs\//);
      if (match) { clearTimeout(timer); resolve(match[0]); }
    };
    child.stdout.on('data', read);
    child.stderr.on('data', read);
    child.on('error', reject);
    child.on('exit', code => { clearTimeout(timer); reject(new Error(`preview ${code}: ${log}`)); });
  });
  return { url, async close() {
    const closed = new Promise(resolve => child.once('exit', resolve));
    child.kill();
    await closed;
  } };
}

(async () => {
  const browser = await chromium.launch({ executablePath: process.env.RUSTIFY_CHROMIUM || undefined });
  const results = { browser: browser.version(), cases: [] };
  try {
    // The review's mask is black and therefore hides its rect. Add a white
    // circular mask to prove the browser resolves the definition and draws it.
    const maskRoot = path.join(work, 'boundaries', 'css-mask-rendered');
    fs.cpSync(path.join(work, 'boundaries', 'css-fragment-only'), maskRoot, { recursive: true });
    fs.writeFileSync(path.join(maskRoot, 'content/index.md'), '---\ntitle: Mask\ndescription: fixture\n---\n# Mask\n\n<svg width="40" height="40"><defs><mask id="m"><circle cx="20" cy="20" r="15" fill="white"/></mask></defs><rect class="[mask:url(#m)]" width="40" height="40" fill="red"/></svg>\n');
    execFileSync(binary, ['build', '--config', path.join(maskRoot, 'rustify-docs.toml')], { stdio: 'pipe' });
    for (const name of ['css-content-text', 'css-entity', 'css-fragment-only', 'css-mask-rendered', 'css-query', 'css-uppercase', 'encoded-question-content', 'encoded-question-static']) {
      const root = path.join(work, 'boundaries', name);
      const host = await preview(root);
      const page = await browser.newPage({ viewport: name === 'css-content-text' ? { width: 375, height: 812 } : { width: 1280, height: 800 } });
      const errors = [];
      page.on('pageerror', error => errors.push(error.message));
      page.on('console', message => { if (message.type() === 'error') errors.push(message.text()); });
      page.on('response', response => { if (response.status() >= 400) errors.push(`${response.status()} ${response.url()}`); });
      page.on('requestfailed', request => errors.push(`${request.url()} ${request.failure()?.errorText}`));
      try {
        await page.goto(host.url);
        const computed = await page.locator('article [class]').evaluateAll(elements => elements.map(element => ({
          class: element.className.baseVal ?? element.className,
          background: getComputedStyle(element).backgroundImage,
          mask: getComputedStyle(element).maskImage,
          before: getComputedStyle(element, '::before').content,
        })));
        if (name === 'css-content-text') assert(computed.some(element => element.before.includes('url(foo)')));
        if (['css-entity', 'css-uppercase', 'css-query'].includes(name)) {
          const background = computed.find(element => element.background.includes('url('));
          assert(background, `${name} has an applied background`);
          const url = /url\("?([^"()]+)"?\)/.exec(background.background)[1];
          assert.equal((await page.request.get(url)).status(), 200);
        }
        if (name === 'css-fragment-only') {
          assert.equal(await page.locator('mask#m').count(), 1);
          assert(computed.some(element => element.mask.includes('#m')));
        }
        let maskPixels;
        if (name === 'css-mask-rendered') {
          const capture = await page.locator('article svg').screenshot();
          const png = PNG.sync.read(capture);
          const pixel = (x, y) => [...png.data.subarray((y * png.width + x) * 4, (y * png.width + x) * 4 + 3)];
          maskPixels = { center: pixel(20, 20), corner: pixel(2, 2) };
          assert.deepEqual(maskPixels.center, [255, 0, 0]);
          assert.notDeepEqual(maskPixels.corner, [255, 0, 0]);
          fs.writeFileSync(path.join(evidence, 'css-mask-rendered.png'), capture);
        }
        const downloads = [];
        if (name.startsWith('encoded-question')) {
          for (const href of await page.locator('article a').evaluateAll(elements => elements.map(element => element.href))) {
            const response = await page.request.get(href);
            assert.equal(response.status(), 200);
            downloads.push({ href, body: await response.text() });
          }
          assert(downloads.length > 0);
        }
        // Fonts and images have settled before checking the recorded errors.
        await page.evaluate(() => document.fonts.ready);
        assert.deepEqual(errors, [], name);
        assert(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth));
        if (['css-content-text', 'css-fragment-only'].includes(name)) await page.screenshot({ path: path.join(evidence, `${name}.png`) });
        results.cases.push({ name, entry: host.url, computed, downloads, maskPixels, errors });
      } finally { await page.close(); await host.close(); }
    }
  } finally { await browser.close(); }
  fs.writeFileSync(path.join(evidence, 'browser-results.json'), JSON.stringify(results, null, 2) + '\n');
  process.stdout.write(JSON.stringify(results, null, 2) + '\n');
})().catch(error => { console.error(error); process.exitCode = 1; });
