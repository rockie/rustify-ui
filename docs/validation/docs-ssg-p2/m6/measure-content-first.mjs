// M4 measure-content-first.mjs, adapted only to canonical installed C and M6
// evidence. Same new→exact homepage H1→two RAF endpoint, 0 wasm and 60 s gate.
// Run only in Root's exclusive runtime window:
// node docs/validation/docs-ssg-p2/m6/measure-content-first.mjs
import { chromium } from '@playwright/test';
import { spawn, execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { mkdtempSync, readFileSync, writeFileSync, existsSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';
import assert from 'node:assert/strict';

const out = path.resolve('docs/validation/docs-ssg-p2/m6');
const stateFile = 'target/docs-ssg/m6-installed-c.json';
const stateBytes = readFileSync(stateFile), pair = JSON.parse(stateBytes);
const hash = bytes => createHash('sha256').update(bytes).digest('hex');
const resultFile = path.join(out, 'content-first-c4-r1.json');
assert(!existsSync(resultFile), 'choose a new evidence filename before repeating; preserve this report');
assert.equal(pair.candidate_sha, 'd301344eeb27e22907c59a10805ae0cebf8dee99', 'install the frozen C4 before this measurement');
assert.equal(hash(readFileSync(pair.binary)), pair.binary_sha256, 'canonical installed binary changed');
const work = mkdtempSync(path.join(tmpdir(), 'rustify-m6-content-first-'));
const name = 'M6 content trial', site = path.join(work, name);
const calls = path.join(work, 'compiler-calls'), compiler = path.join(work, 'forbidden-compiler');
writeFileSync(calls, '');
writeFileSync(compiler, `#!${process.execPath}\nrequire('node:fs').appendFileSync(${JSON.stringify(calls)},JSON.stringify(process.argv.slice(2))+'\\n');process.exit(19);\n`, { mode: 0o755 });
const env = { ...process.env, ...pair.transport_environment,
  PATH: `${pair.process_path_prepend}:${process.env.PATH}`,
  CARGO_NET_OFFLINE: 'true', RUSTIFY_DOCS_CARGO: compiler };
const buildInfo = execFileSync(pair.binary, ['--build-info'], { env, encoding: 'utf8', timeout: 5000 }).trim();
assert.equal(buildInfo, pair.source_identity.trim(), 'under-test source identity differs from canonical C');
assert.match(buildInfo, new RegExp(`^source: ${pair.candidate_sha}$`, 'm'));

const begun = Date.now(), deadline = begun + 60000;
const remaining = () => { const left = deadline - Date.now(); assert(left > 0, 'new→homepage exceeded the 60 s deadline'); return left; };
const report = { scope: 'actual canonical installed CLI content template new→painted homepage; unchanged M4 gate, no Rust site compiler',
  candidate_C: pair.candidate_sha, framework_R: pair.framework_revision,
  installed_identity_input: { path: stateFile, sha256: hash(stateBytes) }, binary_sha256: pair.binary_sha256,
  source_identity: buildInfo, work, site, started_at_ms: begun, deadline_ms: 60000, requests: [], errors: [], csp: [] };
let child, browser, context, log = '';

async function stopDev() {
  if (!child?.pid || child.exitCode !== null || child.signalCode !== null) return;
  const exited = new Promise(resolve => child.once('exit', resolve));
  const kill = signal => { try { process.kill(-child.pid, signal); } catch (error) { if (error.code !== 'ESRCH') throw error; } };
  const escalation = setTimeout(() => kill('SIGKILL'), 2000);
  try { kill('SIGTERM'); await exited; } finally { clearTimeout(escalation); }
}

// Covers async setup and the two-RAF probe too; Playwright operation timeouts
// alone cannot bound a pending page.evaluate() paint promise.
const watchdog = setTimeout(() => {
  report.deadline_failure = 'new→homepage reached the unchanged 60 s deadline';
  if (browser) void browser.close().catch(() => {});
  void stopDev().catch(error => { report.dev_cleanup_failure = String(error); });
}, Math.max(1, deadline - Date.now()));

try {
  report.created = execFileSync(pair.binary, ['new', site], { env, encoding: 'utf8', timeout: remaining() });
  child = spawn(pair.binary, ['dev', '--port', '0'], { cwd: site, env, detached: true });
  child.stderr.on('data', bytes => { log += bytes; });
  const url = await new Promise((resolve, reject) => {
    const timer = setTimeout(() => reject(new Error('dev address was not announced before the 60 s deadline')), remaining());
    let text = '';
    const finish = (error, value) => { clearTimeout(timer); if (error) reject(error); else resolve(value); };
    child.stdout.on('data', bytes => { text += bytes; const match = /^dev: (\S+)\n/m.exec(text); if (match) finish(null, match[1]); });
    child.once('error', error => finish(error));
    child.once('exit', code => finish(new Error(`dev exit ${code}: ${log}`)));
  });
  report.url = url;
  browser = await chromium.launch({ timeout: remaining() });
  report.chromium = browser.version();
  context = await browser.newContext({ viewport: { width: 1280, height: 720 }, serviceWorkers: 'block' });
  const page = await context.newPage();
  page.on('request', request => report.requests.push(request.url()));
  page.on('pageerror', error => report.errors.push(String(error)));
  page.on('console', message => { if (message.type() === 'error') report.errors.push(message.text()); });
  await page.exposeBinding('__content_csp', (_, value) => report.csp.push(value));
  await page.addInitScript(() => document.addEventListener('securitypolicyviolation', event => window.__content_csp(event.violatedDirective + ' ' + event.blockedURI)));
  await page.goto(url, { timeout: remaining() });
  await page.waitForFunction(name => document.querySelector('h1')?.textContent === name, name, { timeout: remaining() });
  // Unchanged M4 paint probe; this is deliberately not replaced with load/FCP.
  await page.evaluate(() => new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve))));
  report.home_at_ms = Date.now();
  clearTimeout(watchdog);
  report.new_to_home_ms = report.home_at_ms - begun;
  assert.equal(report.deadline_failure, undefined);
  assert(report.new_to_home_ms <= 60000);
  assert.equal(await page.locator('[data-rustify-island]').count(), 0);
  assert.equal(report.requests.filter(url => /\.wasm(?:[?#]|$)/.test(url)).length, 0);
  assert.equal(readFileSync(calls, 'utf8'), '');
  assert.equal(hash(readFileSync(stateFile)), report.installed_identity_input.sha256, 'canonical C changed during measurement');
  await page.screenshot({ path: path.join(out, 'content-first-c4-r1.png'), timeout: 5000 });
  assert.deepEqual(report.errors, []);
  assert.deepEqual(report.csp, []);
  report.passed = true;
} catch (error) {
  report.passed = false; report.failure = String(error); process.exitCode = 1;
} finally {
  clearTimeout(watchdog);
  try { if (context) await context.close(); if (browser) await browser.close(); }
  catch (error) { report.passed = false; report.cleanup_failure = String(error); process.exitCode = 1; }
  finally {
    try { await stopDev(); }
    catch (error) { report.passed = false; report.dev_cleanup_failure = String(error); process.exitCode = 1; }
    report.compiler_calls = readFileSync(calls, 'utf8');
    writeFileSync(path.join(out, 'content-first-c4-r1.log'), log);
    writeFileSync(resultFile, JSON.stringify(report, null, 2) + '\n');
    console.log(JSON.stringify({ passed: report.passed, new_to_home_ms: report.new_to_home_ms, failure: report.failure }));
  }
}
