import { test, expect, Page } from '@playwright/test';
import { EVIDENCE } from '../tier';
import { mkdir, readFile, readdir, writeFile } from 'node:fs/promises';
import { resolve } from 'node:path';
import { gzipSync } from 'node:zlib';
import { execFileSync } from 'node:child_process';
import { platform, release } from 'node:os';

const phase = process.env.PANEL_BENCH_PHASE;
const directory = 'docs/validation/dom-components-p3';
const warmups = 5, samples = 40;
const viewport = { width: 1440, height: 900 };
const percentile = (values: number[], p: number) => [...values].sort((a, b) => a - b)[Math.ceil(values.length * p) - 1];
const snapshot = (page: Page) => page.evaluate(() => (window as any).__panel_bench.snapshot());

test('panel native-input frame workloads', { tag: EVIDENCE }, async ({ page, browser }) => {
    test.setTimeout(240_000);
    expect(['baseline', 'current']).toContain(phase);
    const errors: string[] = [];
    page.on('pageerror', error => errors.push(error.message));
    page.on('console', message => { if (message.type() === 'error') errors.push(message.text()); });
    page.on('requestfailed', request => errors.push(`${request.url()}: ${request.failure()?.errorText}`));
    await page.setViewportSize(viewport);
    await page.goto('panels.html');
    await expect(page.locator('#status')).toHaveAttribute('data-status', 'ready');
    await page.evaluate(() => (window as any).__panel_bench.policy(false));
    await page.getByTestId('bench-dialog-open').click();
    await page.getByRole('tab', { name: 'Details', exact: true }).click();
    await expect.poll(() => snapshot(page)).toMatchObject({ requests: 2, accepts: 0, dialog: false, active: 'summary' });
    await page.evaluate(() => (window as any).__panel_bench.policy(true));
    const workloads: Record<string, any> = {};
    for (const control of ['dialog', 'tabs', 'tooltip', 'toast', 'progress', 'scroll']) {
        const before = await page.evaluate(() => (window as any).__panel_bench.stats());
        const start = await snapshot(page);
        const rows: any[] = [];
        for (let index = 0; index < warmups + samples; index++) {
            // Setup does not belong to the measured input (replacement starts with a visible toast).
            if (control === 'toast' && index === 0) await page.getByTestId('bench-toast-show').click();
            if (control === 'tooltip') await page.getByTestId('bench-progress-update').focus();
            const expected = control === 'dialog' || control === 'tooltip' ? index % 2 === 0
                : control === 'tabs' ? (index % 2 === 0 ? 'details' : 'summary')
                : control === 'progress' ? index + 1 : control === 'toast' ? `Saved ${index + 2}` : index;
            // Focus is the tooltip input. Alternating focus/Escape requires closing setup only on open samples.
            if (control === 'tooltip' && expected === false) await page.getByTestId('bench-tooltip-trigger').focus();
            await page.evaluate(({ control, expected }) => {
                const bench = (window as any).__panel_bench;
                bench.pending = null;
                const type = control === 'scroll' ? 'wheel' : control === 'tooltip' ? (expected ? 'focusin' : 'keydown') : control === 'dialog' && !expected ? 'keydown' : 'click';
                const scroll = document.querySelector('[data-testid="bench-scroll"]')!;
                const oldScroll = scroll.scrollTop;
                document.addEventListener(type, () => {
                    const at = performance.now();
                    const initial = bench.snapshot();
                    const observe = () => {
                        const state = bench.snapshot();
                        const dialog = document.querySelector('[role="dialog"]');
                        const tooltip = document.querySelector('[role="tooltip"]');
                        const toast = document.querySelector('[data-name="ToastMessage"]');
                        const good = control === 'dialog' ? state.dialog === expected && !!dialog === expected
                            : control === 'tabs' ? state.active === expected && !(document.querySelector(`[data-testid="panel-${expected}"]`) as HTMLElement)?.hidden
                            : control === 'tooltip' ? state.tooltip === expected && !!tooltip === expected
                            : control === 'toast' ? toast?.textContent === expected && state.requests > initial.requests
                            : control === 'progress' ? state.progress === expected && Math.abs(Number(document.querySelector('[role="progressbar"]')?.getAttribute('aria-valuenow')) - Number(expected)) < 1e-9
                            : scroll.scrollTop !== oldScroll;
                        if (good) bench.pending = { latency_ms: performance.now() - at, input_ms: at, dom_frame_ms: performance.now(), state, request_delta: state.requests - initial.requests, accept_delta: state.accepts - initial.accepts, scroll_top: scroll.scrollTop };
                        else if (performance.now() - at < 5000) requestAnimationFrame(observe);
                        else bench.pending = { error: 'accepted result did not reach DOM', control, expected, state };
                    };
                    requestAnimationFrame(observe);
                }, { capture: true, once: true });
            }, { control, expected });
            if (control === 'dialog') {
                if (expected) await page.getByTestId('bench-dialog-open').click();
                else await page.keyboard.press('Escape');
            } else if (control === 'tabs') await page.getByRole('tab', { name: expected === 'details' ? 'Details' : 'Summary', exact: true }).click();
            else if (control === 'tooltip') {
                if (expected) await page.getByTestId('bench-tooltip-trigger').focus();
                else await page.keyboard.press('Escape');
            } else if (control === 'toast') await page.getByTestId('bench-toast-show').click();
            else if (control === 'progress') await page.getByTestId('bench-progress-update').click();
            else { await page.getByTestId('bench-scroll').hover(); await page.mouse.wheel(0, index % 2 === 0 ? 120 : -120); }
            await page.waitForFunction(() => (window as any).__panel_bench.pending !== null);
            const row = await page.evaluate(() => (window as any).__panel_bench.pending);
            expect(row.error, JSON.stringify(row)).toBeUndefined();
            expect(row.request_delta).toBe(control === 'scroll' ? 0 : 1);
            expect(row.accept_delta).toBe(control === 'scroll' ? 0 : 1);
            if (index >= warmups) rows.push(row);
        }
        if (control === 'dialog') await page.keyboard.press('Escape');
        if (control === 'tooltip') { await page.keyboard.press('Escape'); await page.getByTestId('bench-progress-update').focus(); }
        if (control === 'toast') {
            await page.getByTestId('bench-toaster-toast-dismiss').click();
            await expect(page.getByTestId('bench-toaster-toast')).toHaveCount(0);
        }
        // Existing replaced-toast timers run to their due time before the idle comparison.
        await page.waitForTimeout(1200);
        const end = await snapshot(page);
        const after = await page.evaluate(() => (window as any).__panel_bench.stats());
        expect(after).toEqual(before);
        workloads[control] = { p50_ms: percentile(rows.map(row => row.latency_ms), .5), p95_ms: percentile(rows.map(row => row.latency_ms), .95),
            requests: end.requests - start.requests, accepts: end.accepts - start.accepts, idle: { before, after }, raw: rows };
    }
    expect(errors).toEqual([]);
    await mkdir(directory, { recursive: true });
    await page.screenshot({ path: `${directory}/${phase}-panel.png`, fullPage: true });
    const product = process.env.DOM_ONLY_DIR || 'target/dom/dom-consumer';
    const sizes: Record<string, any> = {};
    for (const file of (await readdir(product, { recursive: true })).filter(file => /\.(wasm|css|js)$/.test(file)).sort()) {
        const bytes = await readFile(resolve(product, file));
        sizes[file] = { bytes: bytes.length, gzip: gzipSync(bytes).length };
    }
    const parameters = { warmups, samples, viewport, browser: browser.version(), os: { platform: platform(), release: release() },
        environment: await page.evaluate(() => ({ user_agent: navigator.userAgent, dpr: devicePixelRatio, visibility: document.visibilityState })),
        rust: execFileSync('rustc', ['--version'], { encoding: 'utf8' }).trim(), node: process.version };
    const report: any = { phase, collected_at: new Date().toISOString(), source: execFileSync('git', ['rev-parse', 'HEAD'], { encoding: 'utf8' }).trim(),
        worktree: execFileSync('git', ['status', '--short'], { encoding: 'utf8' }), product, parameters, sizes, workloads };
    if (phase === 'current') {
        const baseline = JSON.parse(await readFile(`${directory}/baseline.json`, 'utf8'));
        expect(parameters).toEqual(baseline.parameters);
        report.delta = Object.fromEntries(Object.entries(workloads).map(([name, row]) => [name, { p50_ms: row.p50_ms - baseline.workloads[name].p50_ms, p95_ms: row.p95_ms - baseline.workloads[name].p95_ms }]));
    }
    await mkdir(directory, { recursive: true });
    await writeFile(`${directory}/${phase}.json`, JSON.stringify(report, null, 2) + '\n');
});
