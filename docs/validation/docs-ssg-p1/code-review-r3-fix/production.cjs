// Run from the repository root with the docs-cli binary and an evidence directory.
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const { tmpdir } = require('node:os');
const { spawn, spawnSync } = require('node:child_process');
const { chromium } = require(path.resolve('node_modules/playwright'));
const binary = path.resolve(process.argv[2] || 'target/docs-cli/rustify-docs');
const output = path.resolve(process.argv[3] || 'docs/validation/docs-ssg-p1/code-review-r3-fix');
const values = ["URL('/assets/bg_one.svg')", "url('/assets/bg_one.svg')", "URL('/assets/bg\\_one.svg')", "url('/assets/bg\\_one.svg')", "URL('/assets/bg%20one.svg')", "url('/assets/bg%20one.svg')", 'url&#40=/bg.svg)', 'url&#40;=/bg.svg)', 'url(&#34=/bg.svg&#34)', 'url(&#34;=/bg.svg&#34;)'];
const svg = color => `<svg xmlns="http://www.w3.org/2000/svg" width="40" height="40"><rect width="40" height="40" fill="${color}"/></svg>`;
const template = '<template id="holder"><span id="ghost">Hidden</span><template id="nested"><b id="deep">Hidden</b></template></template>\n<p id="visible">Visible</p>';
const front = '---\ndescription: Resource and anchor verification\n---\n';
function build(root) {
    const done = spawnSync(binary, ['build', '--config', path.join(root, 'rustify-docs.toml'), '--format', 'json'], { encoding: 'utf8' });
    return { exit: done.status, report: JSON.parse(done.stdout) };
}
function tree(dir, prefix = '') {
    return fs.readdirSync(dir).sort().flatMap(name => {
        const file = path.join(dir, name), relative = `${prefix}${name}`;
        return fs.statSync(file).isDirectory() ? tree(file, `${relative}/`) : [[relative, fs.readFileSync(file).toString('base64')]];
    });
}
async function preview(root) {
    const child = spawn(binary, ['preview', '--dir', path.join(root, 'dist'), '--port', '0']);
    let log = '';
    const url = await new Promise((resolve, reject) => {
        const timer = setTimeout(() => { child.kill(); reject(Error(log)); }, 15000);
        const read = data => { log += data.toString(); const found = log.match(/http:\/\/127\.0\.0\.1:\d+\/\S*/); if (found) { clearTimeout(timer); resolve(found[0]); } };
        child.stdout.on('data', read); child.stderr.on('data', read);
        child.on('error', reject); child.on('exit', code => { clearTimeout(timer); reject(Error(`preview ${code}: ${log}`)); });
    });
    return { url, async close() { const exited = new Promise(resolve => child.once('exit', resolve)); child.kill(); await exited; } };
}
(async () => {
    fs.mkdirSync(output, { recursive: true });
    const browser = await chromium.launch({ executablePath: process.env.RUSTIFY_CHROMIUM || undefined });
    const result = { browser: browser.version(), binary, cases: [] };
    try {
        for (const base of ['/', '/docs/']) {
            const label = base === '/' ? 'root' : 'subpath';
            const root = fs.mkdtempSync(path.join(tmpdir(), 'docs-r3-production-'));
            for (const dir of ['content/=', 'static/assets']) fs.mkdirSync(path.join(root, dir), { recursive: true });
            fs.writeFileSync(path.join(root, 'rustify-docs.toml'), `schema_version=1\n[site]\nname="Fixtures"\nurl="https://example.net${base}"\nbase="${base}"\n[[nav.groups]]\nlabel="Pages"\nitems=["index.md","other.md"]\n`);
            fs.writeFileSync(path.join(root, 'static/assets/bg one.svg'), svg('red'));
            fs.writeFileSync(path.join(root, 'static/assets/bg_one.svg'), svg('blue'));
            fs.writeFileSync(path.join(root, 'content/=/bg.svg'), svg('red'));
            const body = `${front}# Fixtures\n\n${values.map((value, i) => `<div id="resource-${i}" class="w-10 h-10 [background-image:${value}]">${i}</div>`).join('\n')}\n\n${template}\n\n[Holder](#holder) [Visible](#visible) [Other visible](other.md#visible)\n`;
            fs.writeFileSync(path.join(root, 'content/index.md'), body);
            fs.writeFileSync(path.join(root, 'content/other.md'), `${front}# Other\n\n${template}\n`);
            const built = build(root); assert.equal(built.exit, 0); assert.equal(built.report.diagnostics.filter(d => d.severity === 'error').length, 0);
            const before = tree(path.join(root, 'dist'));
            fs.writeFileSync(path.join(root, 'content/index.md'), `${body}\n[Ghost](#ghost) [Deep](other.md#deep)\n`);
            const refused = build(root); assert.equal(refused.exit, 1);
            assert.equal(refused.report.diagnostics.filter(d => d.rule === 'broken-anchor').length, 2);
            assert.deepEqual(tree(path.join(root, 'dist')), before);
            const host = await preview(root), page = await browser.newPage({ viewport: { width: 1280, height: 900 } });
            const errors = [], resources = [];
            page.on('pageerror', error => errors.push(error.message));
            page.on('console', message => { if (message.type() === 'error') errors.push(message.text()); });
            page.on('response', response => { if (response.status() >= 400) errors.push(`${response.status()} ${response.url()}`); });
            page.on('requestfailed', request => errors.push(`${request.url()}: ${request.failure()?.errorText}`));
            try {
                const response = await page.goto(host.url);
                assert(response.headers()['content-security-policy'].includes("style-src 'self'"));
                for (let i = 0; i < values.length; i++) {
                    const actual = await page.locator(`#resource-${i}`).evaluate(element => ({ class: element.className, background: getComputedStyle(element).backgroundImage }));
                    assert.notEqual(actual.background, 'none');
                    const url = actual.background.match(/^url\("(.*)"\)$/)[1], fetched = await page.request.get(url);
                    assert(fetched.ok()); assert.equal(await fetched.text(), svg([1, 2, 3].includes(i) ? 'blue' : 'red'));
                    resources.push({ input: values[i], ...actual, url, status: fetched.status() });
                }
                const anchors = await page.evaluate(() => ({ holder: document.getElementById('holder')?.tagName, ghost: document.getElementById('ghost'), nested: document.getElementById('nested'), deep: document.getElementById('deep'), stored: document.querySelector('#holder').content.getElementById('ghost')?.tagName }));
                assert.deepEqual(anchors, { holder: 'TEMPLATE', ghost: null, nested: null, deep: null, stored: 'SPAN' });
                for (const width of [1280, 375]) { await page.setViewportSize({ width, height: 900 }); assert(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth)); await page.screenshot({ path: path.join(output, `production-${label}-${width}.png`), fullPage: true }); }
                await page.getByRole('link', { name: 'Visible', exact: true }).click(); assert.equal(new URL(page.url()).hash, '#visible');
                await page.getByRole('link', { name: 'Holder', exact: true }).click(); assert.equal(new URL(page.url()).hash, '#holder');
                await page.getByRole('link', { name: 'Other visible', exact: true }).click(); assert(new URL(page.url()).pathname.endsWith('/other/')); assert.equal(new URL(page.url()).hash, '#visible');
                assert.deepEqual(errors, []);
                result.cases.push({ base, entry: host.url, build: built, invalidAnchors: refused, previousOutputPreserved: true, anchors, resources, errors });
            } finally { await page.close(); await host.close(); fs.rmSync(root, { recursive: true, force: true }); }
        }
    } finally { await browser.close(); }
    fs.writeFileSync(path.join(output, 'production-results.json'), JSON.stringify(result, null, 2) + '\n');
    console.log(JSON.stringify({ browser: result.browser, bases: result.cases.length, loadedResources: result.cases.reduce((sum, item) => sum + item.resources.length, 0) }));
})().catch(error => { console.error(error); process.exitCode = 1; });
