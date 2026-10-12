import { chromium } from '@playwright/test';
import { spawn, execFileSync } from 'node:child_process';
import { createHash } from 'node:crypto';
import { readFileSync, writeFileSync } from 'node:fs';
import path from 'node:path';
import os from 'node:os';
import { fileURLToPath } from 'node:url';

const evidence = path.dirname(fileURLToPath(import.meta.url));
const root = path.resolve(evidence, '../../../..');
const binary = path.join(os.tmpdir(), 'rustify-docs-verify/cli-install/bin/rustify-docs');
const info = execFileSync(binary, ['--build-info'], { encoding: 'utf8' });
if (!info.includes('source: 1b81de25f23a25702f61dbfd3d0ecdfe323d4d44')) throw new Error(info);
const dir = path.join(root, 'target/docs-ssg/http/root/dist');
const manifest = JSON.parse(readFileSync(path.join(dir, 'rustify-docs-manifest.json'), 'utf8'));
if (manifest.cli_version !== '0.1.4') throw new Error('Unexpected production');
for (const file of manifest.files) {
    const bytes = readFileSync(path.join(dir, file.path));
    if (bytes.length !== file.bytes || createHash('sha256').update(bytes).digest('hex') !== file.sha256) throw new Error(file.path);
}
const server = spawn(binary, ['preview', '--dir', dir, '--port', '0']);
let browser;
try {
    const url = await new Promise((resolve, reject) => {
        let text = '';
        server.stdout.on('data', chunk => {
            text += chunk;
            const found = text.match(/^preview: (\S+)/m);
            if (found) resolve(found[1]);
        });
        server.on('exit', code => reject(new Error(`preview exit ${code}`)));
    });
    browser = await chromium.launch({ channel: 'chrome' });
    const context = await browser.newContext({ viewport: { width: 1280, height: 900 } });
    const page = await context.newPage();
    const errors = [], failed = [], responses = [];
    page.on('pageerror', error => errors.push(error.message));
    page.on('requestfailed', request => failed.push(request.url()));
    page.on('response', response => { if (response.status() >= 400) responses.push({ url: response.url(), status: response.status() }); });
    await page.goto(url);
    await page.getByRole('contentinfo').waitFor();
    await page.screenshot({ path: path.join(evidence, 'home-wide.png'), fullPage: true });
    await page.getByRole('link', { name: 'Get started', exact: true }).click();
    await page.setViewportSize({ width: 393, height: 852 });
    await page.goto(url + 'guide/setup/#install-the-cli');
    await page.getByRole('button', { name: 'Dark', exact: true }).click();
    await page.screenshot({ path: path.join(evidence, 'content-narrow-dark.png'), fullPage: true });
    const result = { observed_at: new Date().toISOString(), channel: 'chrome', version: browser.version(), build_info: info, url, build_id: manifest.build_id, page_errors: errors, failed_requests: failed, error_responses: responses, physical_device: false };
    writeFileSync(path.join(evidence, 'visual-runtime.json'), JSON.stringify(result, null, 2) + '\n');
    if (errors.length || failed.length) throw new Error(JSON.stringify(result));
    console.log(JSON.stringify({ version: result.version, page_errors: errors.length, failed_requests: failed.length }));
} finally {
    if (browser) await browser.close();
    const exited = new Promise(resolve => server.once('exit', resolve));
    server.kill();
    await exited;
}
