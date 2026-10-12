const fs = require('node:fs');
const path = require('node:path');
const assert = require('node:assert/strict');
const { spawn, spawnSync } = require('node:child_process');
const { chromium } = require(path.resolve('node_modules/playwright'));
const evidence = path.resolve(process.argv[2]);
const target = path.resolve(process.argv[3] || 'target/docs-cli/rustify-docs');
const base = path.resolve(process.argv[4] || path.join(evidence, 'base-target/docs-cli/rustify-docs'));
async function preview(binary, dir) {
  const child = spawn(binary, ['preview', '--dir', dir, '--port', '0']);
  let output = '';
  const url = await new Promise((resolve, reject) => {
    const timer = setTimeout(() => { child.kill(); reject(Error(output)); }, 10000);
    const read = value => { output += value.toString(); const found = output.match(/http:\/\/127\.0\.0\.1:\d+\/\S*/); if (found) { clearTimeout(timer); resolve(found[0]); } };
    child.stdout.on('data', read); child.stderr.on('data', read);
    child.on('error', reject); child.on('exit', code => { clearTimeout(timer); reject(Error(`preview ${code}: ${output}`)); });
  });
  return { url, async close() { const stopped = new Promise(resolve => child.once('exit', resolve)); child.kill(); await stopped; } };
}
(async () => {
  const cases = JSON.parse(fs.readFileSync(path.join(evidence, 'candidate-results.json'))).cases;
  const browser = await chromium.launch({ executablePath: process.env.RUSTIFY_CHROMIUM || undefined });
  const result = { browser: browser.version(), cases: [] };
  try {
    for (const item of cases) {
      const fixture = path.join(evidence, 'candidate-fixtures', item.case);
      const dom = `${fixture}-target-dom`;
      fs.mkdirSync(dom, { recursive: true });
      fs.cpSync(path.join(fixture, 'content'), path.join(dom, 'content'), { recursive: true });
      fs.copyFileSync(path.join(fixture, 'rustify-docs.toml'), path.join(dom, 'rustify-docs.toml'));
      fs.mkdirSync(path.join(dom, 'static'), { recursive: true });
      const markdown = path.join(dom, 'content/index.md');
      fs.writeFileSync(markdown, fs.readFileSync(markdown, 'utf8').replace(/^\[(Circle|Visible|Ghost)\].*\n/gm, ''));
      const built = spawnSync(target, ['build', '--config', path.join(dom, 'rustify-docs.toml'), '--format', 'json'], { encoding: 'utf8' });
      assert.equal(built.status, 0, built.stdout);
      for (const [version, binary, dir] of [['base', base, path.join(fixture, 'baseline-dist')], ['target-without-link', target, path.join(dom, 'dist')]]) {
        const host = await preview(binary, dir);
        const page = await browser.newPage({ viewport: { width: 1280, height: 900 } });
        const errors = [];
        page.on('pageerror', error => errors.push(error.message));
        page.on('console', message => { if (message.type() === 'error') errors.push(message.text()); });
        page.on('response', response => { if (response.status() >= 400) errors.push(`${response.status()} ${response.url()}`); });
        page.on('requestfailed', request => errors.push(`${request.url()}: ${request.failure()?.errorText}`));
        try {
          await page.goto(host.url);
          const actual = await page.evaluate(() => Object.fromEntries(['visible', 'circle', 'svg-holder', 'holder', 'ghost'].map(id => {
            const node = document.getElementById(id);
            return [id, node ? { tag: node.tagName, namespace: node.namespaceURI, templateContent: !!node.content } : null];
          })));
          if (item.case === 'svg-template-pair') assert.equal(actual.circle.namespace, 'http://www.w3.org/2000/svg');
          if (item.case.endsWith('selfclose') || item.case === 'nested-svg-template') assert.equal(actual.visible.tag, 'H2');
          if (version === 'base' && item.case === 'svg-template-selfclose') {
            await page.locator('a[href$="#visible"]').last().click();
            assert.equal(new URL(page.url()).hash, '#visible');
          }
          if (version === 'target-without-link' && ['svg-template-pair', 'svg-template-selfclose'].includes(item.case)) await page.screenshot({ path: path.join(evidence, `${item.case}.png`), fullPage: true });
          assert.deepEqual(errors, []);
          result.cases.push({ case: item.case, version, entry: host.url, actual, hash: new URL(page.url()).hash, errors });
        } finally { await page.close(); await host.close(); }
      }
    }
  } finally { await browser.close(); }
  fs.writeFileSync(path.join(evidence, 'candidate-browser-results.json'), JSON.stringify(result, null, 2) + '\n');
  console.log(JSON.stringify({ browser: result.browser, checkedProducts: result.cases.length }));
})().catch(error => { console.error(error); process.exitCode = 1; });
