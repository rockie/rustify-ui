// Reproduce and check the real Theme Studio Inspector in an isolated browser.
const { chromium, expect } = require('@playwright/test');
const fs = require('node:fs');
const path = require('node:path');
const assert = require('node:assert/strict');

const phase = process.argv[2];
assert(['before', 'after'].includes(phase));
const out = path.join(__dirname, process.argv[3] || phase);
fs.mkdirSync(out, { recursive: false });
const records = [];
const url = 'http://127.0.0.1:4181/';
const cases = phase === 'before' ? [[1440, 900, 'light']] :
    [[1440, 900, 'light'], [1440, 900, 'dark'], [390, 844, 'light']];

(async () => {
    const browser = await chromium.launch({ headless: true,
        ...(process.env.RUSTIFY_CHROMIUM ? { executablePath: process.env.RUSTIFY_CHROMIUM } : {}) });
    try {
        for (const [width, height, theme] of cases) {
            const ctx = await browser.newContext({ viewport: { width, height },
                deviceScaleFactor: 1, reducedMotion: 'reduce', locale: 'en-US' });
            const page = await ctx.newPage();
            const health = { consoleErrors: [], pageErrors: [], failedRequests: [], csp: [] };
            const record = { phase, url, viewport: { width, height }, dpr: 1, theme, health, picks: [] };
            records.push(record);
            page.on('console', m => { if (m.type() === 'error') health.consoleErrors.push(m.text()); });
            page.on('pageerror', e => health.pageErrors.push(e.message));
            page.on('requestfailed', r => health.failedRequests.push(`${r.url()} ${r.failure()?.errorText}`));
            page.on('response', r => { if (r.status() >= 400) health.failedRequests.push(`${r.status()} ${r.url()}`); });
            await page.addInitScript(() => {
                window.__inspectCheckCsp = [];
                document.addEventListener('securitypolicyviolation', e => window.__inspectCheckCsp.push(e.violatedDirective));
            });
            await page.goto(url);
            await page.locator('#status[data-status=ready]').waitFor({ state: 'attached', timeout: 90000 });
            await page.evaluate(() => document.fonts.ready);
            await page.waitForLoadState('networkidle');
            await page.waitForTimeout(800);
            const manifest = await ctx.request.get(url + 'build-manifest.json');
            record.buildManifest = await manifest.json();
            if (width <= 600) await page.getByTestId('mobile-preview').click();
            await page.getByTestId('renderer-dom').click();
            if (theme === 'dark') await page.getByTestId('mode-dark').click();
            const preview = page.getByTestId('studio-preview');
            const outline = page.getByTestId('inspector-outline');
            for (const [kind, targetId] of [['mapped', 'cards-submit'], ['unmapped', 'preview-primary']]) {
                await page.getByTestId('inspector-toggle').click();
                await page.waitForTimeout(500);
                const target = preview.getByTestId(targetId);
                await target.scrollIntoViewIfNeeded();
                const before = await target.textContent();
                await target.click();
                if (width <= 600 && kind === 'mapped') {
                    await expect(page.getByTestId('mobile-edit')).toHaveAttribute('aria-pressed', 'true');
                    await page.getByTestId('mobile-preview').click();
                }
                await expect(outline).toBeVisible();
                await expect(outline).toHaveAttribute('data-annotated', String(kind === 'mapped'));
                if (kind === 'unmapped') {
                    await expect(target).toHaveText(before);
                    await expect(page.getByTestId('inspector-unannotated')).toBeVisible();
                } else {
                    await expect(page.getByTestId('inspector-source')).toContainText('primary');
                }
                await page.getByTestId('inspector-panel').evaluate(n => n.scrollIntoView({ block: 'start' }));
                await page.waitForTimeout(500);
                const style = await outline.evaluate(n => {
                    const s = getComputedStyle(n), r = n.getBoundingClientRect();
                    return { borderColor: s.borderColor, borderStyle: s.borderStyle, borderWidth: s.borderWidth,
                        outline: s.outline, boxShadow: s.boxShadow, clipPath: s.clipPath,
                        rect: { x: r.x, y: r.y, width: r.width, height: r.height } };
                });
                const expected = phase === 'before' && kind === 'unmapped' ?
                    ['rgb(71, 84, 103)', 'dashed'] : ['rgb(21, 112, 239)', 'solid'];
                assert.deepEqual([style.borderColor, style.borderStyle], expected);
                const file = `${kind}-${width}-${theme}.png`;
                const at = new Date().toISOString();
                await page.screenshot({ path: path.join(out, file) });
                record.picks.push({ kind, targetId, at, file, style,
                    panel: await page.getByTestId('inspector-panel').innerText(), targetText: before });
                await page.keyboard.press('Escape');
                await expect(outline).toBeHidden();
                await expect(page.getByTestId('inspector-panel')).toBeHidden();
                if (kind === 'unmapped') {
                    await target.click();
                    await expect(target).not.toHaveText(before);
                    record.normalActionAfterEscape = await target.textContent();
                }
            }
            health.csp = await page.evaluate(() => window.__inspectCheckCsp);
            assert(Object.values(health).every(v => v.length === 0), JSON.stringify(health));
            await ctx.close();
        }
    } finally {
        fs.writeFileSync(path.join(out, 'results.json'), JSON.stringify({ phase, chromium: browser.version(), records }, null, 2) + '\n');
        await browser.close();
    }
    console.log(JSON.stringify({ phase, contexts: records.length, screenshots: records.reduce((n, r) => n + r.picks.length, 0), passed: true }));
})().catch(e => { console.error(e); process.exitCode = 1; });
