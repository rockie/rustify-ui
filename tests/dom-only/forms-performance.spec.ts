import { test, expect, Page } from '@playwright/test';
import { EVIDENCE } from '../tier';
import { mkdir, readFile, writeFile } from 'node:fs/promises';
import { resolve } from 'node:path';
import { gzipSync } from 'node:zlib';

const warmups = 5;
const samples = 40;
const viewport = { width: 1440, height: 900 };
const percentile = (values: number[], p: number) => [...values].sort((a, b) => a - b)[Math.ceil(values.length * p) - 1];
const api = (page: Page) => page.evaluate(() => (window as any).__form_bench.snapshot());
const phase = process.env.FORM_BENCH_PHASE;
const directory = 'docs/validation/dom-components-p2';

test.describe('scalar form controls', { tag: EVIDENCE }, () => {
    test('accepted values, refusal and fixed page-timestamp workload', async ({ page, browser }) => {
        test.setTimeout(180_000);
        expect(['baseline', 'current']).toContain(phase);
        const errors: string[] = [];
        page.on('pageerror', error => errors.push(error.message));
        page.on('console', message => { if (message.type() === 'error') errors.push(message.text()); });
        page.on('requestfailed', request => errors.push(`${request.url()}: ${request.failure()?.errorText}`));
        await page.setViewportSize(viewport);
        await page.goto('forms.html');
        await expect(page.locator('#status')).toHaveAttribute('data-status', 'ready');
        const check = page.getByTestId('bench-checkbox');
        const text = page.getByTestId('bench-text');
        const slider = page.getByTestId('bench-slider');
        const select = page.getByTestId('bench-select');

        // The driver uses browser input; it never calls a component handler.
        await page.evaluate(() => (window as any).__form_bench.policy(false));
        await check.click();
        await expect(check).not.toBeChecked();
        await text.fill('refused');
        await expect(text).toHaveValue('initial');
        await slider.focus();
        await page.keyboard.press('ArrowRight');
        if (phase === 'baseline') await expect(slider).toHaveValue('50');
        else await expect(slider).toHaveAttribute('aria-valuenow', '50');
        await select.click();
        await page.getByRole('option', { name: 'Beta', exact: true }).click();
        await expect(select).toContainText('Alpha');
        await expect.poll(() => api(page)).toMatchObject({ requests: 4, accepts: 0, text: 'initial', slider: 50, selected: 'alpha' });
        await page.evaluate(() => (window as any).__form_bench.policy(true, true));
        const readonly = await api(page);
        await check.click();
        await slider.focus();
        await page.keyboard.press('ArrowRight');
        expect(await api(page)).toEqual(readonly);
        await page.evaluate(() => (window as any).__form_bench.policy(true, false, true));
        await expect(check).toBeDisabled();
        await expect(text).toBeDisabled();
        await expect(slider).toBeDisabled();
        await expect(select).toBeDisabled();
        await page.evaluate(() => (window as any).__form_bench.reset());

        const workloads: Record<string, unknown> = {};
        for (const control of ['checkbox', 'text', 'slider', 'select']) {
            await page.evaluate(() => (window as any).__form_bench.reset());
            await expect(text).toHaveValue('initial');
            const before = await page.evaluate(() => (window as any).__form_bench.stats());
            const rows: any[] = [];
            for (let index = 0; index < warmups + samples; index++) {
                const expected = control === 'checkbox' ? index % 2 === 0
                    : control === 'text' ? `sample-${index}`
                    : control === 'slider' ? 51 + index
                    : index % 2 === 0 ? 'beta' : 'alpha';
                if (control === 'select') await select.click();
                if (control === 'slider') await slider.focus();
                await page.evaluate(({ control, expected }) => {
                    const bench = (window as any).__form_bench;
                    const eventType = control === 'slider' ? 'keydown' : control === 'text' ? 'input' : 'click';
                    const before = bench.snapshot();
                    bench.pending = null;
                    document.addEventListener(eventType, () => {
                        const at = performance.now();
                        const observe = () => {
                            const state = bench.snapshot();
                            const element = document.querySelector(`[data-testid="bench-${control}"]`) as HTMLInputElement;
                            const accepted = control === 'checkbox' ? state.checked : control === 'select' ? state.selected : state[control];
                            const displayed = control === 'checkbox' ? element.checked === expected
                                : control === 'select' ? element.textContent?.includes(expected === 'alpha' ? 'Alpha' : 'Beta')
                                : control === 'slider' && element.tagName !== 'INPUT' ? element.getAttribute('aria-valuenow') === String(expected)
                                : element.value === String(expected);
                            if (state.requests === before.requests + 1 && accepted === expected && displayed) {
                                const row = { input_ms: at, dom_frame_ms: performance.now(), latency_ms: performance.now() - at, state };
                                bench.pending = row;
                                bench.measurements.push(row);
                            } else if (performance.now() - at < 5000) requestAnimationFrame(observe);
                            else bench.pending = { error: 'accepted value did not reach DOM', state, expected };
                        };
                        requestAnimationFrame(observe);
                    }, { capture: true, once: true });
                }, { control, expected });
                if (control === 'checkbox') await check.click();
                if (control === 'text') await text.fill(String(expected));
                if (control === 'slider') await page.keyboard.press('ArrowRight');
                if (control === 'select') await page.getByRole('option', { name: expected === 'beta' ? 'Beta' : 'Alpha', exact: true }).click();
                await page.waitForFunction(() => (window as any).__form_bench.pending !== null);
                const row = await page.evaluate(() => (window as any).__form_bench.pending);
                expect(row.error, JSON.stringify({ control, index, row })).toBeUndefined();
                if (index >= warmups) rows.push(row);
            }
            const counts = await api(page);
            expect(counts.requests).toBe(warmups + samples);
            expect(counts.accepts).toBe(warmups + samples);
            await page.evaluate(() => new Promise(resolve => setTimeout(resolve, 250)));
            const after = await page.evaluate(() => (window as any).__form_bench.stats());
            expect(after).toEqual(before);
            const values = rows.map(row => row.latency_ms);
            workloads[control] = { p50_ms: percentile(values, .5), p95_ms: percentile(values, .95), counts, idle: { before, after }, raw: rows };
        }
        expect(errors).toEqual([]);
        const product = process.env.PRIMITIVES_DIR!;
        const sizes: Record<string, { bytes: number; gzip: number }> = {};
        for (const file of ['pkg/app_bg.wasm', 'rustify.css', 'rustify-runtime.css']) {
            const bytes = await readFile(resolve(product, file));
            sizes[file] = { bytes: bytes.length, gzip: gzipSync(bytes).length };
        }
        const environment = await page.evaluate(() => ({ user_agent: navigator.userAgent, dpr: devicePixelRatio, visibility: document.visibilityState }));
        const parameters = { warmups, samples, viewport, browser_version: browser.version(), executable: process.env.RUSTIFY_CHROMIUM || null, environment };
        const report: any = { phase, collected_at: new Date().toISOString(), product, parameters, sizes, workloads };
        if (phase === 'current') {
            const baseline = JSON.parse(await readFile(`${directory}/baseline.json`, 'utf8'));
            expect(parameters).toEqual(baseline.parameters);
            report.baseline = { collected_at: baseline.collected_at, sizes: baseline.sizes };
            report.delta = Object.fromEntries(Object.keys(workloads).map(control => {
                const old = baseline.workloads[control], now = workloads[control] as any;
                return [control, { p50_ms: now.p50_ms - old.p50_ms, p95_ms: now.p95_ms - old.p95_ms }];
            }));
        }
        await mkdir(directory, { recursive: true });
        await writeFile(`${directory}/${phase}.json`, JSON.stringify(report, null, 2) + '\n');
        console.log(JSON.stringify({ ...report, workloads: Object.fromEntries(Object.entries(workloads).map(([name, row]: [string, any]) => [name, { ...row, raw: undefined }])) }));
    });
});

test.describe('additional form capabilities', { tag: EVIDENCE }, () => {
    test('current-only three-state, named range and composed field page timestamps', async ({ page }) => {
        test.skip(phase !== 'current', 'Additional capabilities have no matching old implementation.');
        const errors: string[] = [];
        page.on('pageerror', error => errors.push(error.message));
        page.on('console', message => { if (message.type() === 'error') errors.push(message.text()); });
        page.on('requestfailed', request => errors.push(request.url()));
        await page.setViewportSize(viewport);
        await page.goto('forms.html');
        await expect(page.locator('#status')).toHaveAttribute('data-status', 'ready');
        await expect(page.getByTestId('new-check')).toHaveAttribute('aria-checked', 'mixed');
        const stats = () => page.evaluate(() => ({ ...((window as any).__form_bench.stats()), geometryRules: document.adoptedStyleSheets.reduce((sum, sheet) => sum + sheet.cssRules.length, 0) }));
        const workloads: Record<string, unknown> = {};
        for (const kind of ['check', 'range', 'text']) {
            const target = kind === 'range' ? 'new-range-low' : `new-${kind}`;
            const control = page.getByTestId(target);
            await control.scrollIntoViewIfNeeded();
            const before = await stats(); const rows: any[] = [];
            for (let index = 0; index < warmups + samples; index++) {
                const expected = kind === 'check' ? (index % 2 === 0 ? 'true' : 'false') : kind === 'range' ? String(11 + index) : `part-${index}`;
                await page.evaluate(({ target, kind }) => {
                    (window as any).__form_new = null;
                    const type = kind === 'range' ? 'keydown' : kind === 'text' ? 'input' : 'click';
                    const capture = (event: Event) => {
                        if (!(event.target as Element)?.closest(`[data-testid="${target}"]`)) return;
                        document.removeEventListener(type, capture, true);
                        const started = performance.now();
                        requestAnimationFrame(() => {
                            const element = document.querySelector(`[data-testid="${target}"]`)!;
                            (window as any).__form_new = { latency_ms: performance.now() - started, actual: kind === 'range' ? element.getAttribute('aria-valuenow') : kind === 'check' ? element.getAttribute('aria-checked') : (element as HTMLInputElement).value };
                        });
                    };
                    document.addEventListener(type, capture, true);
                }, { target, kind });
                if (kind === 'check') await control.click();
                else if (kind === 'range') await control.press('ArrowRight');
                else await control.fill(expected);
                await expect.poll(() => page.evaluate(() => (window as any).__form_new?.actual)).toBe(expected);
                const row = await page.evaluate(() => (window as any).__form_new);
                if (index >= warmups) rows.push(row);
            }
            const count = Number(await page.getByTestId(`new-${kind}-count`).textContent());
            expect(count).toBe(warmups + samples);
            const commits = kind === 'range' ? Number(await page.getByTestId('new-range-commits').textContent()) : 0;
            if (kind === 'range') expect(commits).toBe(warmups + samples);
            const after = await stats(); expect(after).toEqual(before);
            const values = rows.map(row => row.latency_ms);
            workloads[kind] = { p50_ms: percentile(values, .5), p95_ms: percentile(values, .95), requests: count - warmups, accepts: count - warmups, commits: kind === 'range' ? commits - warmups : 0, idle: { before, after }, raw: rows };
        }
        const rawProduct: Record<string, unknown> = {};
        for (const file of ['pkg/app_bg.wasm', 'primitives.css', 'rustify-runtime.css']) {
            const bytes = await readFile(resolve('target/dom/dom-primitives', file));
            rawProduct[file] = { bytes: bytes.length, gzip: gzipSync(bytes).length };
        }
        expect(errors).toEqual([]);
        const report = { phase: 'current-only', collected_at: new Date().toISOString(), parameters: { warmups, samples, viewport }, rawProduct, workloads };
        await writeFile(`${directory}/current-only.json`, JSON.stringify(report, null, 2) + '\n');
        console.log(JSON.stringify({ ...report, workloads: Object.fromEntries(Object.entries(workloads).map(([key, row]: [string, any]) => [key, { ...row, raw: undefined }])) }));
    });
});
