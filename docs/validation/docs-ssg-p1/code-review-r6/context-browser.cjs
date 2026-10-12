// From the repository root: node <this-file> <R6 evidence directory>
const fs = require('node:fs');
const path = require('node:path');
const { spawn, spawnSync } = require('node:child_process');
const { chromium } = require(path.resolve('node_modules/playwright'));
const out = path.resolve(process.argv[2]);
const cases = JSON.parse(fs.readFileSync(path.join(out, 'structural-cli.json')));
const binaries = {
    base: JSON.parse(fs.readFileSync(path.join(out, 'baseline-build.json'))).binary,
    target: path.resolve('target/docs-cli/rustify-docs'),
};
function command(binary, root, cmd, extra = []) {
    const args = [cmd, '--config', path.join(root, 'rustify-docs.toml'), '--format', 'json', ...extra];
    const done = spawnSync(binary, args, { encoding: 'utf8' });
    if (done.error) throw done.error;
    return { command: [binary, ...args], exit: done.status, report: JSON.parse(done.stdout) };
}
async function preview(binary, dir) {
    const child = spawn(binary, ['preview', '--dir', dir, '--port', '0']);
    let log = '';
    const url = await new Promise((resolve, reject) => {
        const timer = setTimeout(() => { child.kill(); reject(Error(log)); }, 10000);
        const read = data => {
            log += data.toString();
            const found = log.match(/http:\/\/127\.0\.0\.1:\d+\/\S*/);
            if (found) { clearTimeout(timer); resolve(found[0]); }
        };
        child.stdout.on('data', read);
        child.stderr.on('data', read);
        child.once('error', reject);
    });
    return { url, async stop() {
        const exited = new Promise(resolve => child.once('exit', resolve));
        child.kill();
        await exited;
    } };
}
(async () => {
    const browser = await chromium.launch({ executablePath: process.env.RUSTIFY_CHROMIUM || undefined });
    const result = { browser: browser.version(), binaries, cases: [] };
    try {
        for (const fixture of cases) {
            const source = path.join(fixture.root, 'content/index.md');
            for (const [version, binary] of Object.entries(binaries)) {
                fs.writeFileSync(source, fixture.source);
                const checked = command(binary, fixture.root, 'check');
                const dir = path.join(fixture.root, `${version}-out`);
                const built = command(binary, fixture.root, 'build', ['--out', dir]);
                const viewOnly = built.exit !== 0;
                if (viewOnly) {
                    fs.writeFileSync(source, fixture.source.replace('[Target](#target)', 'Target'));
                    const viewed = command(binary, fixture.root, 'build', ['--out', dir]);
                    if (viewed.exit !== 0) throw Error(JSON.stringify(viewed));
                }
                const host = await preview(binary, dir);
                const page = await browser.newPage();
                const errors = [];
                page.on('pageerror', error => errors.push(error.message));
                page.on('console', message => { if (message.type() === 'error') errors.push(message.text()); });
                page.on('requestfailed', request => errors.push(`${request.url()}: ${request.failure()?.errorText}`));
                try {
                    await page.goto(host.url);
                    const actual = await page.evaluate(() => {
                        const node = document.getElementById('target');
                        return node ? { name: node.tagName, namespace: node.namespaceURI, isBody: node === document.body } : null;
                    });
                    if (!viewOnly && actual) await page.getByRole('link', { name: 'Target', exact: true }).click();
                    result.cases.push({ name: fixture.name, version, checked, built, viewOnly, actual, entry: host.url, hash: new URL(page.url()).hash, errors });
                } finally { await page.close(); await host.stop(); }
            }
            fs.writeFileSync(source, fixture.source);
        }
    } finally {
        await browser.close();
        fs.writeFileSync(path.join(out, 'context-browser.json'), JSON.stringify(result, null, 2) + '\n');
    }
    console.log(JSON.stringify(result.cases.map(item => ({ name: item.name, version: item.version, exit: item.checked.exit, actual: item.actual, errors: item.errors }))));
})().catch(error => { console.error(error); process.exitCode = 1; });
