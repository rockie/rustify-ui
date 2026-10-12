const fs = require('node:fs');
const path = require('node:path');
const { spawn } = require('node:child_process');
const { chromium } = require(path.resolve('node_modules/playwright'));
const work = process.argv[2];
const binary = path.resolve('target/docs-cli/rustify-docs');
const sleep = ms => new Promise(resolve => setTimeout(resolve, ms));
async function server(root, mode) {
  const args = mode === 'dev' ? ['dev', '--config', path.join(root, 'rustify-docs.toml'), '--port', '0'] : ['preview', '--dir', path.join(root, 'dist'), '--port', '0'];
  const child = spawn(binary, args);
  let log = '';
  const url = await new Promise((resolve, reject) => {
    const timer = setTimeout(() => { child.kill(); reject(new Error(log)); }, 15000);
    const read = chunk => {
      log += chunk.toString();
      const match = log.match(/http:\/\/127\.0\.0\.1:\d+\/docs\//);
      if (match) { clearTimeout(timer); resolve(match[0]); }
    };
    child.stdout.on('data', read); child.stderr.on('data', read);
    child.on('error', reject);
    child.on('exit', code => { clearTimeout(timer); reject(new Error('server exit ' + code + '\n' + log)); });
  });
  return { url, close: () => { child.kill(); fs.writeFileSync(path.join(root, 'server.log'), log); } };
}
(async () => {
  const browser = await chromium.launch({ executablePath: process.env.RUSTIFY_CHROMIUM || undefined });
  const results = { browser: browser.version(), cases: [] };
  try {
    const root = path.join(work, 'boundaries', 'watch-symlink');
    for (const dir of ['content', 'static', 'images', 'brand']) fs.mkdirSync(path.join(root, dir), { recursive: true });
    fs.writeFileSync(path.join(root, 'rustify-docs.toml'), 'schema_version=1\n[site]\nname="Watch fixture"\nurl="https://example.net/docs/"\nbase="/docs/"\n');
    const svg = '<svg xmlns="http://www.w3.org/2000/svg" width="40" height="40"><rect width="40" height="40" fill="red"/></svg>';
    const target = path.join(root, 'brand', 'real.svg');
    fs.writeFileSync(target, svg);
    fs.symlinkSync(target, path.join(root, 'images', 'mark.svg'));
    fs.writeFileSync(path.join(root, 'content', 'index.md'), '---\ntitle: Watch\ndescription: fixture\n---\n# Watch\n\n![Mark](../images/mark.svg)\n');
    const host = await server(root, 'dev');
    const page = await browser.newPage();
    const errors = []; page.on('pageerror', e => errors.push(e.message));
    try {
      await page.goto(host.url);
      const before = await page.locator('article img').getAttribute('src');
      fs.writeFileSync(target, svg.replace('red', 'blue'));
      await sleep(2000);
      await page.reload();
      const afterRealTargetSave = await page.locator('article img').getAttribute('src');
      const afterBytes = await (await page.request.get(new URL(afterRealTargetSave, host.url).href)).text();
      fs.appendFileSync(path.join(root, 'content', 'index.md'), '\nA save of the Markdown.\n');
      await sleep(1200);
      await page.reload();
      const afterContentSave = await page.locator('article img').getAttribute('src');
      const fixedBytes = await (await page.request.get(new URL(afterContentSave, host.url).href)).text();
      results.cases.push({ name: 'watch-symlink', entry: host.url, before, afterRealTargetSave, targetSaveServesRed: afterBytes.includes('red'), afterContentSave, contentSaveServesBlue: fixedBytes.includes('blue'), errors });
      await page.screenshot({ path: path.join(work, 'watch-symlink.png') });
    } finally { await page.close(); host.close(); }
    const guardRoot = path.join(work, 'boundaries', 'css-unwatched-guard');
    const guard = await server(guardRoot, 'preview');
    const second = await browser.newPage();
    const failed = [], consoleErrors = [];
    second.on('response', r => { if (r.status() >= 400) failed.push({ url: r.url(), status: r.status() }); });
    second.on('console', m => { if (m.type() === 'error') consoleErrors.push(m.text()); });
    try {
      await second.goto(guard.url);
      await sleep(400);
      const background = await second.locator('article div').evaluate(e => getComputedStyle(e).backgroundImage);
      results.cases.push({ name: 'css-unwatched-guard', entry: guard.url, background, failed, consoleErrors });
    } finally { await second.close(); guard.close(); }
  } finally { await browser.close(); }
  fs.writeFileSync(path.join(work, 'extra-browser.json'), JSON.stringify(results, null, 2) + '\n');
  process.stdout.write(JSON.stringify(results, null, 2) + '\n');
})().catch(e => { console.error(e); process.exitCode = 1; });
