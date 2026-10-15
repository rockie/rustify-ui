import { chromium } from '/Users/rockie/Documents/gh-rockie/rustify-ui/node_modules/@playwright/test/index.mjs';
import { spawn } from 'node:child_process';
import { readFile, writeFile } from 'node:fs/promises';
import { createHash } from 'node:crypto';

const root = new URL('./', import.meta.url).pathname;
const cli = '/var/folders/cv/pn7p1v7s737cqlt605jyl7lm0000gn/T/rustify-m6-pair4-installed-9pziu1hu/installed/bin/rustify-docs';
const child = spawn(cli, ['dev', '--config', `${root}probe-site/rustify-docs.toml`, '--port', '0'], {
  env: { ...process.env, RUSTIFY_DOCS_CARGO: 'mbx', CARGO_NET_OFFLINE: 'true' },
});
let out = '', log = '';
child.stdout.on('data', b => { out += b; process.stdout.write(b); });
child.stderr.on('data', b => { log += b; });
const waitUntil = async (predicate, timeout = 120000) => {
  const start = Date.now();
  while (!predicate()) {
    if (Date.now() - start > timeout) throw new Error(`deadline exceeded\n${log}`);
    await new Promise(resolve => setTimeout(resolve, 100));
  }
};
const abort = new AbortController();
let browser;
const states = [];
try {
  await waitUntil(() => /dev: (http:\/\/[^\s]+)/.test(out));
  const url = out.match(/dev: (http:\/\/[^\s]+)/)[1];
  const response = await fetch(`${url}__rustify-docs/events?path=/&good=0`, { signal: abort.signal });
  const reader = response.body.getReader();
  const capture = (async () => {
    const decoder = new TextDecoder();
    let buffer = '';
    for (;;) {
      const { value, done } = await reader.read();
      if (done) return;
      buffer += decoder.decode(value, { stream: true });
      let end;
      while ((end = buffer.indexOf('\n\n')) >= 0) {
        const message = buffer.slice(0, end); buffer = buffer.slice(end + 2);
        const data = message.match(/^data: (.*)$/m);
        if (data) states.push(JSON.parse(data[1]));
      }
    }
  })().catch(error => { if (!abort.signal.aborted) throw error; });
  await waitUntil(() => states.some(state => !state.building && state.ok && state.good > 0));
  const initial = structuredClone(states.at(-1));
  console.log('initial generation ready');
  browser = await chromium.launch({ headless: true });
  const page = await browser.newPage();
  const observe = async () => {
    await page.goto(url);
    await page.waitForFunction(() => window.__rustify_docs_islands?.state() === 'ready');
    const demo = page.locator('[data-rustify-island]').nth(1);
    await demo.locator('[data-testid="counter-increment"]').click();
    const view = await demo.locator('output').evaluate(node => ({ text: node.textContent, class: node.className, background: getComputedStyle(node).backgroundColor }));
    const moduleURL = await page.locator('[data-rd-islands-module]').getAttribute('src');
    const wasm = await (await page.request.get(new URL('app_bg.wasm', new URL(moduleURL, url)).href)).body();
    return { ...view, moduleURL, wasmSHA256: createHash('sha256').update(wasm).digest('hex') };
  };
  const before = await observe();
  const manifest = `${root}payload-v1/payload.txt`;
  const original = await readFile(manifest, 'utf8');
  await writeFile(`${root}duplicate-initial.txt`, original);
  const changed = "14\n";
  await writeFile(manifest, changed);
  await writeFile(`${root}duplicate-changed.txt`, changed);
  const sequence = initial.sequence;
  await new Promise(resolve => setTimeout(resolve, 3000));
  const afterManifest = structuredClone(states.at(-1));
  const manifestOnly = await observe();
  console.log('build-input-only save observed');
  const source = `${root}probe-site/src/demos.rs`;
  await writeFile(source, (await readFile(source, 'utf8')) + '\n// Force the author source to be rebuilt for the control observation.\n');
  await waitUntil(() => states.at(-1)?.sequence > afterManifest.sequence && !states.at(-1).building);
  const afterForced = structuredClone(states.at(-1));
  const forced = await observe();
  const result = { url, browser: browser.version(), initial, before, afterManifest, manifestOnly, afterForced, forced, states };
  await writeFile(`${root}probe-build-input.json`, JSON.stringify(result, null, 2));
  console.log(JSON.stringify({ before, manifestOnly, forced, generations: [initial.compile?.generation, afterManifest.compile?.generation, afterForced.compile?.generation] }, null, 2));
  abort.abort();
  await capture;
} finally {
  abort.abort();
  if (browser) await browser.close();
  child.kill('SIGINT');
  await writeFile(`${root}probe-build-input.log`, log);
}
