const fs = require('node:fs');
const path = require('node:path');
const { spawn, spawnSync } = require('node:child_process');
const { chromium } = require(path.resolve('node_modules/playwright'));
const output = path.resolve(process.argv[2]);
const scope = JSON.parse(fs.readFileSync(path.join(output, 'scope.json')));
const binaries = {
    base: path.join(scope.workspace, 'base-target/docs-cli/rustify-docs'),
    target: path.resolve('target/docs-cli/rustify-docs'),
};
const cases = JSON.parse(fs.readFileSync(path.join(output, 'extra-cases.json')));
async function preview(binary, dir) {
    const child = spawn(binary, ['preview', '--dir', dir, '--port', '0']);
    let text = '';
    const url = await new Promise((resolve, reject) => {
        const timer = setTimeout(() => { child.kill(); reject(Error(text)); }, 10000);
        const read = data => {
            text += data.toString();
            const match = text.match(/http:\/\/127\.0\.0\.1:\d+\/\S*/);
            if (match) { clearTimeout(timer); resolve(match[0]); }
        };
        child.stdout.on('data', read);
        child.stderr.on('data', read);
        child.once('error', reject);
    });
    return { url, stop: async () => {
        const exited = new Promise(resolve => child.once('exit', resolve));
        child.kill();
        await exited;
    } };
}
(async () => {
    const browser = await chromium.launch();
    const results = { browser: browser.version(), binaries, cases: [] };
    try {
        for (const fixture of cases) {
            const source = path.join(fixture.root, 'content/index.md');
            const original = fs.readFileSync(source, 'utf8');
            for (const [version, binary] of Object.entries(binaries)) {
                fs.writeFileSync(source, original);
                const dir = path.join(fixture.root, `${version}-dist`);
                const args = ['build', '--config', path.join(fixture.root, 'rustify-docs.toml'), '--out', dir, '--format', 'json'];
                const built = spawnSync(binary, args, { encoding: 'utf8' });
                let viewOnly = false;
                if (built.status !== 0) {
                    viewOnly = true;
                    fs.writeFileSync(source, original.replace(`[Target](#${fixture.anchor})`, 'Target'));
                    const viewed = spawnSync(binary, args, { encoding: 'utf8' });
                    if (viewed.status !== 0) throw Error(viewed.stdout + viewed.stderr);
                }
                const host = await preview(binary, dir);
                const page = await browser.newPage();
                const errors = [];
                page.on('pageerror', error => errors.push(error.message));
                page.on('requestfailed', request => errors.push(request.url()));
                try {
                    await page.goto(host.url);
                    const actual = await page.evaluate(id => {
                        const node = document.getElementById(id);
                        return node ? { id: node.id, name: node.tagName, namespace: node.namespaceURI, inArticle: !!node.closest('article') } : null;
                    }, fixture.anchor);
                    if (!viewOnly && actual) {
                        await page.locator('article').getByRole('link', { name: 'Target', exact: true }).click();
                    }
                    results.cases.push({ case: fixture.case, version, build_exit: built.status, report: JSON.parse(built.stdout), entry: host.url, view_only_without_link: viewOnly, actual, hash: new URL(page.url()).hash, errors });
                    if (fixture.case === 'body-id') await page.screenshot({ path: path.join(output, `body-id-${version}.png`) });
                } finally { await page.close(); await host.stop(); }
            }
            fs.writeFileSync(source, original);
        }
    } finally {
        await browser.close();
        fs.writeFileSync(path.join(output, 'structural-browser.json'), JSON.stringify(results, null, 2) + '\n');
    }
    console.log(JSON.stringify(results.cases.map(({ case: name, version, build_exit, actual }) => ({ name, version, build_exit, actual }))));
})().catch(error => { console.error(error); process.exitCode = 1; });
