// Run from the repository root. Browser modes use a freshly built utility-lab.
import assert from 'node:assert/strict';
import { createRequire } from 'node:module';
import { copyFileSync, mkdirSync, mkdtempSync, readFileSync, rmSync, symlinkSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join, resolve } from 'node:path';

const [mode, url = 'http://127.0.0.1:64907/'] = process.argv.slice(2);
const require = createRequire(resolve('package.json'));
if (mode === 'ci-oracle') {
  const root = mkdtempSync(join(tmpdir(), 'utility-review-ci-'));
  const oracle = join(root, 'tests/utility/compiler/oracle');
  mkdirSync(oracle, { recursive: true });
  copyFileSync('tests/utility/compiler/oracle/package.json', join(oracle, 'package.json'));
  const dependencies = JSON.parse(readFileSync('package.json', 'utf8')).devDependencies;
  for (const name of Object.keys(dependencies)) {
    const destination = join(root, 'node_modules', name);
    mkdirSync(join(destination, '..'), { recursive: true });
    symlinkSync(resolve('node_modules', name), destination);
  }
  try {
    const clean = createRequire(join(oracle, 'package.json'));
    try { clean('tailwindcss/package.json'); assert.fail('Expected a missing oracle dependency'); }
    catch (error) {
      assert.equal(error.code, 'MODULE_NOT_FOUND');
      console.log(JSON.stringify({ code: error.code, message: error.message.split('\n')[0] }));
    }
  } finally { rmSync(root, { recursive: true }); }
} else {
  assert.ok(['animation', 'history'].includes(mode), 'mode: animation | history | ci-oracle');
  const { chromium } = require('playwright');
  const browser = await chromium.launch({ channel: 'chrome', headless: true });
  try {
    const page = await browser.newPage();
    const errors = [];
    page.on('pageerror', error => errors.push(error.message));
    await page.goto(url);
    await page.locator('#status[data-status="ready"]').waitFor();
    const result = await page.evaluate(async mode => {
      const api = window.__utility_lab;
      if (mode === 'animation') {
        api.dispose('scope-b');
        api.mount('scope-b', 10000, '@theme { --animate-card: card 1s linear infinite; @keyframes card { to { opacity: 0; } } } @tailwind utilities;');
        await api.ensure('scope-b', ['animate-card']);
        const sample = document.getElementById('scope-b-sample');
        sample.className = 'animate-card';
        const actual = { name: getComputedStyle(sample).animationName, count: sample.getAnimations().length };
        const root = document.createElement('div');
        const control = document.createElement('div');
        control.className = 'animate-card';
        root.append(control);
        document.querySelector('main').append(root);
        const sheet = new CSSStyleSheet();
        sheet.replaceSync(':root { --animate-card: card 1s linear infinite; } .animate-card { animation: var(--animate-card); } @keyframes card { to { opacity: 0; } }');
        document.adoptedStyleSheets = [...document.adoptedStyleSheets, sheet];
        const reference = { name: getComputedStyle(control).animationName, count: control.getAnimations().length };
        document.adoptedStyleSheets = document.adoptedStyleSheets.filter(value => value !== sheet);
        root.remove();
        return { actual, reference, scopedCss: api.report('scope-b').css };
      }
      await api.ensure('scope-b', ['w-[12345px]']);
      const before = api.report('scope-b');
      const activeMatches = document.getElementById('scope-b').querySelectorAll('.w-\\[12345px\\]').length;
      const after = await api.replace('scope-b', '@tailwind utilities;');
      const retained = { hadCandidate: before.admitted.includes('w-[12345px]'), activeMatches,
        stillRetained: after.admitted.includes('w-[12345px]'), beforeRevision: before.revision, afterRevision: after.revision };
      const host = document.createElement('div');
      host.id = 'scope-review-quota';
      document.querySelector('main').append(host);
      const limits = { input_bytes: 65536, asset_bytes: 65536, candidate_count: 1024, ast_depth: 32,
        css_bytes: 262144, queued_bytes: 65536, cache_bytes: 1048576 };
      api.mount(host.id, 10000, undefined, limits);
      await api.flush(host.id);
      const baseline = api.retained(host.id);
      api.dispose(host.id);
      host.replaceChildren();
      api.mount(host.id, 10000, undefined, { ...limits, candidate_count: baseline + 1 });
      await api.flush(host.id);
      await api.ensure(host.id, ['w-[12345px]']);
      let replacementFailure, failure;
      try { await api.replace(host.id, '@tailwind utilities;'); }
      catch (error) { replacementFailure = JSON.parse(String(error)); }
      try { await api.ensure(host.id, ['w-[12346px]']); }
      catch (error) { failure = JSON.parse(String(error)); }
      return { retained, quota: { baseline, limit: baseline + 1, count: api.retained(host.id), replacementFailure, failure } };
    }, mode);
    assert.deepEqual(errors, []);
    if (mode === 'animation') {
      assert.equal(result.actual.count, 0);
      assert.equal(result.reference.count, 1);
    } else {
      assert.equal(result.retained.activeMatches, 0);
      assert.equal(result.retained.stillRetained, true);
      assert.equal(result.quota.failure.error.resource, 'candidate_count');
    }
    console.log(JSON.stringify({ ...result, pageErrors: errors }, null, 2));
  } finally { await browser.close(); }
}
