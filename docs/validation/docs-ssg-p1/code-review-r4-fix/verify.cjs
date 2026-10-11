// Run from the repository root: verify.cjs BEFORE_BINARY AFTER_BINARY OUTPUT_DIR
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const { spawn, spawnSync } = require('node:child_process');
const { createHash } = require('node:crypto');
const { chromium } = require(path.resolve('node_modules/playwright'));
const before = path.resolve(process.argv[2]);
const after = path.resolve(process.argv[3]);
const output = path.resolve(process.argv[4]);
const work = fs.mkdtempSync(path.resolve('target/docs-ssg/r4-fix/products-'));
const contexts = [
    ['foreignObject', '<svg><foreignObject>', '</foreignObject></svg>'],
    ['desc', '<svg><desc>', '</desc></svg>'],
    ['title', '<svg><title>', '</title></svg>'],
    ['math-text', '<math><mtext>', '</mtext></math>'],
    ['math-annotation', '<math><annotation-xml encoding="TEXT/HTML">', '</annotation-xml></math>'],
];
const cases = [
    ['svg-template-selfclose', '<svg><template/></svg>', [], true],
    ['svg-template-pair', '<svg><template id="svg-holder"><circle id="circle" r="10"/></template></svg>', ['svg-holder', 'circle'], true],
    ['nested-svg-template', '<template id="holder"><svg><template/></svg></template>', ['holder'], true],
    ['html-template-control', '<template id="holder"><span id="ghost">Hidden</span><template><b id="deep">Hidden</b></template></template>', ['holder'], false],
    ['html-template-selfclose', '<template id="holder"/><span id="ghost">Hidden</span></template>', ['holder'], false],
    ...contexts.map(([name, open, close]) => [name, `${open}<template id="holder"><svg><template><circle id="ghost" r="10"/></template></svg></template>${close}`, ['holder'], false]),
];
const hash = file => createHash('sha256').update(fs.readFileSync(file)).digest('hex');
function command(binary, root, name, extra = []) {
    const done = spawnSync(binary, [name, '--config', path.join(root, 'rustify-docs.toml'), '--format', 'json', ...extra], { encoding: 'utf8' });
    assert.equal(done.error, undefined);
    return { exit: done.status, report: JSON.parse(done.stdout), stderr: done.stderr };
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
        const read = data => { log += data.toString(); const found = log.match(/http:\/\/127\.0\.0\.1:\d+\/\S*/); if (found) { clearTimeout(timer); resolve(found[0]); } };
        child.stdout.on('data', read); child.stderr.on('data', read);
        child.on('error', reject); child.on('exit', code => { clearTimeout(timer); reject(Error(`preview ${code}: ${log}`)); });
    });
    return { url, async close() { const exited = new Promise(resolve => child.once('exit', resolve)); child.kill(); await exited; } };
}
(async () => {
    fs.mkdirSync(output, { recursive: true });
    const browser = await chromium.launch({ executablePath: process.env.RUSTIFY_CHROMIUM || undefined });
    const result = { before, after, binary_hashes: { before: hash(before), after: hash(after) }, browser: browser.version(), work, cases: [] };
    try {
        for (const base of ['/', '/docs/']) {
            for (const [name, markup, ids, regression] of cases) {
                const root = path.join(work, `${base === '/' ? 'root' : 'subpath'}-${name}`);
                fs.mkdirSync(path.join(root, 'content'), { recursive: true });
                fs.writeFileSync(path.join(root, 'rustify-docs.toml'), `schema_version=1\n[site]\nname="Anchors"\nurl="https://example.net${base}"\nbase="${base}"\n[[nav.groups]]\nlabel="Pages"\nitems=["index.md","other.md"]\n`);
                const body = `---\ndescription: Namespace anchors\n---\n# Anchors\n\n${markup}\n\n## Visible\n\n${[...ids, 'visible'].map(id => `[${id}](#${id}) [Other ${id}](other.md#${id})`).join('\n')}\n`;
                fs.writeFileSync(path.join(root, 'content/index.md'), body);
                fs.writeFileSync(path.join(root, 'content/other.md'), body);
                const baseline = command(before, root, 'build', ['--out', path.join(root, 'before-dist')]);
                if (regression) assert.equal(baseline.exit, 1, `${name}: expected a pre-fix failure`);
                const checked = command(after, root, 'check');
                const built = command(after, root, 'build');
                assert.equal(checked.exit, 0, `${name}: ${JSON.stringify(checked)}`);
                assert.equal(built.exit, 0, `${name}: ${JSON.stringify(built)}`);
                const good = tree(path.join(root, 'dist'));
                const invalid = markup.includes('ghost') ? 'ghost' : 'absent';
                fs.writeFileSync(path.join(root, 'content/index.md'), `${body}\n[Ghost](#${invalid}) [Other ghost](other.md#${invalid})\n`);
                const refused = command(after, root, 'build');
                assert.equal(refused.exit, 1);
                assert.equal(refused.report.diagnostics.filter(d => d.rule === 'broken-anchor').length, 2);
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
                    const actual = await page.locator('article [id]').evaluateAll(nodes => nodes.map(node => ({ id: node.id, namespace: node.namespaceURI, content: 'content' in node })));
                    assert.deepEqual(actual.map(node => node.id), ['anchors', ...ids, 'visible'], name);
                    if (name === 'svg-template-pair') assert.equal(actual.find(node => node.id === 'svg-holder').content, false);
                    if (name === 'foreignObject') assert.equal(actual.find(node => node.id === 'holder').content, true);
                    assert.equal(await page.locator('#ghost').count(), 0);
                    await page.locator('article').getByRole('link', { name: 'visible', exact: true }).click();
                    assert.equal(new URL(page.url()).hash, '#visible');
                    await page.locator('article').getByRole('link', { name: 'Other visible', exact: true }).click();
                    assert.equal(new URL(page.url()).pathname, `${base}other/`);
                    assert.equal(new URL(page.url()).hash, '#visible');
                    if (ids.includes('circle')) {
                        await page.locator('article').getByRole('link', { name: 'circle', exact: true }).click();
                        assert.equal(new URL(page.url()).hash, '#circle');
                    }
                    assert.equal(await page.evaluate(() => document.documentElement.scrollWidth > innerWidth), false);
                    if (['svg-template-selfclose', 'foreignObject'].includes(name)) await page.screenshot({ path: path.join(output, `${base === '/' ? 'root' : 'subpath'}-${name}.png`), fullPage: true });
                    assert.deepEqual(errors, []);
                    result.cases.push({ name, base, entry: host.url, baseline, checked, built, refused, output_preserved: true, actual, errors });
                } finally { await page.close(); await host.close(); }
            }
        }
    } finally {
        await browser.close();
        fs.writeFileSync(path.join(output, 'verification.json'), JSON.stringify(result, null, 2) + '\n');
    }
    console.log(JSON.stringify({ browser: result.browser, products: result.cases.length, work }));
})().catch(error => { console.error(error); process.exitCode = 1; });
