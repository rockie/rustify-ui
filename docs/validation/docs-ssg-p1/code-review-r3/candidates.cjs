// Isolated CLI + Chromium reproductions, no repository writes.
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const { spawn, spawnSync } = require('node:child_process');
const { chromium } = require(path.resolve('node_modules/playwright'));
const work = process.argv[2];
const versions = { target: path.resolve('target/docs-cli/rustify-docs'), baseline: path.join(work, 'baseline-target/docs-cli/rustify-docs') };
const results = { cases: [] };
const config = 'schema_version=1\n[site]\nname="Review"\nurl="https://example.net/docs/"\nbase="/docs/"\n';
function rootFor(name) {
  const root = path.join(work, 'candidate-fixtures', name);
  for (const dir of ['content', 'static', 'brand']) fs.mkdirSync(path.join(root, dir), { recursive: true });
  fs.writeFileSync(path.join(root, 'rustify-docs.toml'), config);
  return root;
}
async function serve(binary, root, dev = false) {
  const child = spawn(binary, dev ? ['dev', '--config', path.join(root, 'rustify-docs.toml'), '--port', '0'] : ['preview', '--dir', path.join(root, 'dist'), '--port', '0']);
  let log = '';
  const url = await new Promise((resolve, reject) => {
    const timer = setTimeout(() => { child.kill(); reject(Error(log)); }, 15000);
    const read = b => { log += b.toString(); const m = log.match(/http:\/\/127\.0\.0\.1:\d+\/docs\//); if (m) { clearTimeout(timer); resolve(m[0]); } };
    child.stdout.on('data', read); child.stderr.on('data', read);
    child.on('error', reject); child.on('exit', code => { clearTimeout(timer); reject(Error(`server ${code}: ${log}`)); });
  });
  return { url, async close() { const exit = new Promise(resolve => child.once('exit', resolve)); child.kill(); await exit; fs.writeFileSync(path.join(root, 'server.log'), log); } };
}
const sleep = ms => new Promise(resolve => setTimeout(resolve, ms));
// Watch one page revision every whole second, at most ten samples.
async function waitForRevision(page, after) {
  for (let i = 0; i < 10; i++) {
    await sleep(1000);
    const revision = Number(await page.locator('.rd-dev').getAttribute('data-revision'));
    if (revision > after) return revision;
  }
  throw Error('No next revision within 10 seconds');
}
(async () => {
  const browser = await chromium.launch({ executablePath: process.env.RUSTIFY_CHROMIUM || undefined });
  results.browser = browser.version();
  try {
    for (const [version, binary] of Object.entries(versions)) {
      const root = rootFor(`template-${version}`);
      const body = '---\ndescription: Template fixture\n---\n# Template\n\n<template id="holder"><span id="ghost">Not a document target</span></template>\n\n[Ghost](#ghost)\n';
      fs.writeFileSync(path.join(root, 'content/index.md'), body);
      const build = spawnSync(binary, ['build', '--config', path.join(root, 'rustify-docs.toml'), '--format', 'json'], { encoding: 'utf8' });
      const report = JSON.parse(build.stdout);
      const item = { name: 'template-anchor', version, exit: build.status, report };
      if (version === 'baseline') { assert.equal(build.status, 1); assert(report.diagnostics.some(d => d.rule === 'broken-anchor')); }
      if (version === 'target') {
        assert.equal(build.status, 0);
        const host = await serve(binary, root);
        const page = await browser.newPage(); const errors = [];
        page.on('pageerror', e => errors.push(e.message));
        try {
          await page.goto(host.url);
          item.entry = host.url;
          item.document = await page.evaluate(() => ({ target: document.getElementById('ghost')?.outerHTML ?? null, templateTarget: document.querySelector('template').content.getElementById('ghost')?.outerHTML ?? null, templateElement: document.getElementById('holder')?.tagName }));
          await page.locator('article a[href="#ghost"]').click();
          item.afterClick = await page.evaluate(() => ({ hash: location.hash, target: document.getElementById('ghost'), scrollY }));
          assert.equal(item.document.target, null); assert(item.document.templateTarget); assert.equal(item.afterClick.hash, '#ghost');
          item.errors = errors; assert.deepEqual(errors, []);
          await page.screenshot({ path: path.join(work, 'template-target.png') });
        } finally { await page.close(); await host.close(); }
      }
      results.cases.push(item);
    }
    for (const [version, binary] of Object.entries(versions)) {
      const root = rootFor(`attachment-watch-${version}`);
      const target = path.join(root, 'brand/real.pdf');
      fs.writeFileSync(target, 'original attachment');
      fs.symlinkSync('../brand/real.pdf', path.join(root, 'content/manual.pdf'));
      const contentFile = path.join(root, 'content/index.md');
      fs.writeFileSync(contentFile, '---\ndescription: Attachment fixture\n---\n# Attachment\n\n[Download](manual.pdf)\n');
      const host = await serve(binary, root, true);
      const page = await browser.newPage(); const errors = []; page.on('pageerror', e => errors.push(e.message));
      try {
        await page.goto(host.url);
        await page.locator('.rd-dev[data-revision]').waitFor({ state: 'attached' });
        const revision = Number(await page.locator('.rd-dev').getAttribute('data-revision'));
        const originalHref = await page.locator('article a').getAttribute('href');
        fs.unlinkSync(target);
        const deletionRevision = await waitForRevision(page, revision);
        const deletionPanel = await page.getByRole('dialog').innerText();
        assert(deletionPanel.includes('broken-link'));
        fs.writeFileSync(target, 'restored attachment');
        // Failure hypothesis: recovery never matches the replacement filter.
        // Three finite one-second observations, not open-ended monitoring.
        const observed = [];
        for (let i = 0; i < 3; i++) { await sleep(1000); observed.push(Number(await page.locator('.rd-dev').getAttribute('data-revision'))); }
        assert(observed.every(r => r === deletionRevision));
        const stillFailed = await page.getByRole('dialog').isVisible(); assert(stillFailed);
        fs.appendFileSync(contentFile, '\nA content save.\n');
        const contentSaveRevision = await waitForRevision(page, deletionRevision);
        const fixedHref = await page.locator('article a').getAttribute('href');
        assert.notEqual(fixedHref, originalHref);
        const restored = await (await page.request.get(new URL(fixedHref, host.url).href)).text(); assert.equal(restored, 'restored attachment');
        results.cases.push({ name: 'attachment-symlink-recovery', version, entry: host.url, originalHref, revision, deletionRevision, deletionPanel, observedAfterRestore: observed, stillFailed, contentSaveRevision, fixedHref, restored, errors });
        assert.deepEqual(errors, []);
        if (version === 'target') await page.screenshot({ path: path.join(work, 'attachment-after-content-save.png') });
      } finally { await page.close(); await host.close(); }
    }
  } finally { await browser.close(); }
  fs.writeFileSync(path.join(work, 'candidate-results.json'), JSON.stringify(results, null, 2) + '\n');
  console.log(JSON.stringify(results, null, 2));
})().catch(error => { console.error(error); process.exitCode = 1; });
