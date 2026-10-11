// Run from the repository root: body-production.cjs BEFORE_BINARY AFTER_BINARY OUTPUT_DIR
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const { spawn, spawnSync } = require('node:child_process');
const { createHash } = require('node:crypto');
const { chromium } = require(path.resolve('node_modules/playwright'));
const before = path.resolve(process.argv[2]);
const after = path.resolve(process.argv[3]);
const output = path.resolve(process.argv[4]);
const work = fs.mkdtempSync(path.join(require('node:os').tmpdir(), 'rustify-docs-r5-body-fix-'));
const body = '---\ndescription: Body anchor\n---\n# Body anchor\n\n<body id="target">\n\nText\n\n</body>\n';
const links = '\n[Target](#target) [Other target](other.md#target)\n';
const hash = file => createHash('sha256').update(fs.readFileSync(file)).digest('hex');
function command(binary, root, name) {
    const args = [name, '--config', path.join(root, 'rustify-docs.toml'), '--format', 'json'];
    const done = spawnSync(binary, args, { encoding: 'utf8' });
    assert.equal(done.error, undefined);
    return { command: [binary, ...args], exit: done.status, report: JSON.parse(done.stdout), stderr: done.stderr };
}
function tree(dir, prefix = '') {
    return fs.readdirSync(dir).sort().flatMap(name => {
        const file = path.join(dir, name), relative = `${prefix}${name}`;
        return fs.statSync(file).isDirectory() ? tree(file, `${relative}/`) : [[relative, hash(file)]];
    });
}
async function preview(root) {
    const child = spawn(after, ['preview', '--dir', path.join(root, 'dist'), '--port', '0']);
    let log = '';
    const url = await new Promise((resolve, reject) => {
        const timer = setTimeout(() => { child.kill(); reject(Error(log)); }, 10000);
        const read = data => {
            log += data.toString();
            const found = log.match(/http:\/\/127\.0\.0\.1:\d+\/\S*/);
            if (found) { clearTimeout(timer); resolve(found[0]); }
        };
        child.stdout.on('data', read); child.stderr.on('data', read);
        child.on('error', reject);
        child.on('exit', code => { clearTimeout(timer); reject(Error(`preview ${code}: ${log}`)); });
    });
    return { url, async close() { const exited = new Promise(resolve => child.once('exit', resolve)); child.kill(); await exited; } };
}
(async () => {
    fs.mkdirSync(output, { recursive: true });
    const browser = await chromium.launch({ executablePath: process.env.RUSTIFY_CHROMIUM || undefined });
    const result = { before, after, binary_hashes: { before: hash(before), after: hash(after) }, browser: browser.version(), work, cases: [] };
    try {
        for (const base of ['/', '/docs/']) {
            for (const variant of ['reported', 'merged-and-template']) {
                const name = `${base === '/' ? 'root' : 'subpath'}-${variant}`;
                const root = path.join(work, name);
                fs.mkdirSync(path.join(root, 'content'), { recursive: true });
                fs.writeFileSync(path.join(root, 'rustify-docs.toml'), `schema_version=1\n[site]\nname="Body anchors"\nurl="https://example.net${base}"\nbase="${base}"\n[[nav.groups]]\nlabel="Pages"\nitems=["index.md","other.md"]\n`);
                const markup = variant === 'reported' ? body : `${body}\n<body id="ignored">\n\n<template id="holder"><body id="ghost"><span id="hidden">Hidden</span></template>\n`;
                fs.writeFileSync(path.join(root, 'content/index.md'), markup + links);
                fs.writeFileSync(path.join(root, 'content/other.md'), markup);
                const red = command(before, root, 'check');
                const redBuild = command(before, root, 'build');
                assert.equal(red.exit, 1);
                assert.equal(red.report.summary.errors, 2);
                assert.equal(redBuild.exit, 1);
                assert.equal(redBuild.report.summary.errors, 2);
                const checked = command(after, root, 'check');
                const built = command(after, root, 'build');
                assert.equal(checked.exit, 0, JSON.stringify(checked));
                assert.equal(built.exit, 0, JSON.stringify(built));
                const manifest = JSON.parse(fs.readFileSync(path.join(root, 'dist/rustify-docs-manifest.json'), 'utf8'));
                assert.equal(manifest.cli_version, '0.1.2');
                const good = tree(path.join(root, 'dist'));
                const invalid = variant === 'reported' ? 'absent' : 'ghost';
                fs.writeFileSync(path.join(root, 'content/index.md'), `${markup}${links}\n[Missing](#${invalid}) [Other missing](other.md#${invalid})\n`);
                const refused = command(after, root, 'build');
                assert.equal(refused.exit, 1);
                assert.equal(refused.report.summary.errors, 2);
                assert.deepEqual(tree(path.join(root, 'dist')), good);
                const host = await preview(root);
                const page = await browser.newPage({ viewport: { width: base === '/' ? 1280 : 375, height: 900 } });
                const errors = [];
                page.on('pageerror', error => errors.push(error.message));
                page.on('console', message => { if (message.type() === 'error') errors.push(message.text()); });
                page.on('response', response => { if (response.status() >= 400) errors.push(`${response.status()} ${response.url()}`); });
                page.on('requestfailed', request => errors.push(`${request.url()}: ${request.failure()?.errorText}`));
                try {
                    await page.goto(host.url);
                    const actual = await page.evaluate(() => ({
                        target: document.getElementById('target') === document.body,
                        ignored: document.getElementById('ignored'),
                        ghost: document.getElementById('ghost'),
                        hidden: document.getElementById('hidden'),
                        ids: [...document.querySelectorAll('article [id]')].map(node => node.id),
                        stored: document.querySelector('#holder')?.content.getElementById('hidden')?.tagName || null,
                    }));
                    assert.deepEqual(actual, { target: true, ignored: null, ghost: null, hidden: null, ids: variant === 'reported' ? ['body-anchor'] : ['body-anchor', 'holder'], stored: variant === 'reported' ? null : 'SPAN' });
                    await page.locator('article').getByRole('link', { name: 'Target', exact: true }).click();
                    assert.equal(new URL(page.url()).hash, '#target');
                    await page.locator('article').getByRole('link', { name: 'Other target', exact: true }).click();
                    assert.equal(new URL(page.url()).pathname, `${base}other/`);
                    assert.equal(new URL(page.url()).hash, '#target');
                    assert.equal(await page.locator('body').getAttribute('id'), 'target');
                    assert.equal(await page.evaluate(() => document.documentElement.scrollWidth > innerWidth), false);
                    await page.screenshot({ path: path.join(output, `${name}.png`), fullPage: true });
                    assert.deepEqual(errors, []);
                    result.cases.push({ name, base, entry: host.url, red, redBuild, checked, built, refused, output_preserved: true, actual, errors });
                } finally { await page.close(); await host.close(); }
            }
        }
    } finally {
        await browser.close();
        fs.writeFileSync(path.join(output, 'body-production.json'), JSON.stringify(result, null, 2) + '\n');
    }
    console.log(JSON.stringify({ browser: result.browser, products: result.cases.length, work }));
})().catch(error => { console.error(error); process.exitCode = 1; });
