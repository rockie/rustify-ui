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
const timing = (rows: any[]) => ({ p50_ms: percentile(rows.map(row => row.latency_ms), .5), p95_ms: percentile(rows.map(row => row.latency_ms), .95) });
const difference = (before: Record<string, unknown>, after: Record<string, unknown>) => Object.fromEntries(Object.keys(before)
    .filter(key => typeof before[key] === 'number' && typeof after[key] === 'number').map(key => [key, (after[key] as number) - (before[key] as number)]));
async function stableStats(page: Page) {
    await page.evaluate(async () => { for (let i = 0; i < 3; i++) await new Promise<void>(resolve => requestAnimationFrame(() => resolve())); });
    return page.evaluate(() => (window as any).__panel_bench.stats());
}
type NewControl = 'notification_migration' | 'fifo_slot' | 'scroll_track';
async function captureNew(page: Page, control: NewControl, target: string, type: string, expected: boolean | number) {
    await page.evaluate(({ control, target, type, expected }) => {
        const bench = (window as any).__panel_bench;
        bench.new_pending = null;
        const element = (name: string) => document.querySelector(`[data-testid="${name}"]`) as HTMLElement | null;
        const counts = () => element('feedback-close-requests')!.textContent!.split(',').map(Number);
        const capture = (event: Event) => {
            if (!(event.target as Element)?.closest(`[data-testid="${target}"]`)) return;
            document.removeEventListener(type, capture, true);
            const at = performance.now(), initial = control === 'fifo_slot' ? counts() : [];
            const oldScroll = control === 'scroll_track' ? element('content-scroll-viewport')!.scrollTop : 0;
            let frames = 0;
            const observe = () => {
                frames++;
                let good = false, actual: Record<string, any> = {};
                if (control === 'notification_migration') {
                    const host = element('feedback-host-dialog'), toast = element('feedback-layer-toast')!;
                    const popup = element('feedback-review-popup')!, options = element('feedback-layer-options')!;
                    const sameNodes = Object.entries(bench.new_nodes).every(([name, node]) => element(name) === node);
                    const hosted = !!host && host.contains(toast) && host.contains(popup) && host.contains(options);
                    actual = { host_open: !!host, hosted, same_nodes: sameNodes, focused: document.activeElement === element(target),
                        focus_counts: element('feedback-layer-focus-counts')!.textContent!.split(',').map(Number),
                        close_requests: Number(element('feedback-layer-close-requests')!.textContent) };
                    good = !!host === expected && sameNodes && actual.focused && !popup.hidden && !options.hidden
                        && (expected ? hosted : !toast.closest('[data-testid="feedback-host-dialog"]'));
                } else if (control === 'fifo_slot') {
                    const first = element('feedback-toast-0')!, second = element('feedback-toast-1')!, next = element('feedback-toast-2')!;
                    actual = { first_hidden: first.hidden, second_visible: !second.hidden, next_visible: !next.hidden,
                        request_delta: counts().map((count, index) => count - initial[index]) };
                    good = actual.first_hidden && actual.second_visible && actual.next_visible && actual.request_delta[0] === 1;
                } else {
                    const viewport = element('content-scroll-viewport')!, track = element('content-scroll-y')!, thumb = element('content-thumb-y')!;
                    const t = track.getBoundingClientRect(), h = thumb.getBoundingClientRect();
                    const projected = (t.height - h.height) * viewport.scrollTop / (viewport.scrollHeight - viewport.clientHeight);
                    actual = { scroll_top: viewport.scrollTop, old_scroll_top: oldScroll, native_scroll_delta: viewport.scrollTop - oldScroll,
                        thumb_offset: h.top - t.top, projected_offset: projected, pointer_id: (event as PointerEvent).pointerId,
                        captured: track.hasPointerCapture((event as PointerEvent).pointerId) };
                    good = Number(expected) * actual.native_scroll_delta > 0 && Math.abs(actual.thumb_offset - projected) < 2;
                }
                if (good) bench.new_pending = { latency_ms: performance.now() - at, input_ms: at, dom_frame_ms: performance.now(), frames,
                    input_event: type, trusted: event.isTrusted, expected, ...actual };
                else if (performance.now() - at < 5000) requestAnimationFrame(observe);
                else bench.new_pending = { error: 'native input did not reach the new capability DOM', control, expected, ...actual };
            };
            requestAnimationFrame(observe);
        };
        document.addEventListener(type, capture, true);
    }, { control, target, type, expected });
}
async function newResult(page: Page) {
    await page.waitForFunction(() => (window as any).__panel_bench.new_pending !== null);
    const row = await page.evaluate(() => (window as any).__panel_bench.new_pending);
    expect(row.error, JSON.stringify(row)).toBeUndefined();
    expect(row.trusted).toBe(true);
    return row;
}
async function currentOnly(page: Page) {
    const workloads: Record<string, any> = {};
    const overallBefore = await stableStats(page);

    await page.getByTestId('feedback-mode-layers').click();
    await page.getByTestId('feedback-layer-show').click();
    await page.getByTestId('feedback-review-open').focus();await page.keyboard.press('Enter');
    await page.getByTestId('feedback-layer-select').click();
    await expect(page.getByTestId('feedback-layer-options')).toBeVisible();
    await page.evaluate(() => {
        (window as any).__panel_bench.new_nodes = Object.fromEntries(['feedback-layer-toast', 'feedback-layer-viewport', 'feedback-review-popup', 'feedback-layer-options', 'feedback-layer-note', 'feedback-layer-select']
            .map(name => [name, document.querySelector(`[data-testid="${name}"]`)]));
    });
    const migrationRows: any[] = [];
    let migrationBefore: any;
    for (let index = 0; index < warmups + samples; index++) {
        if (index === warmups) {
            // The measured loop starts with the same open descendants on either side of each modal transition.
            await page.getByTestId('feedback-layer-select').press('F2');
            await expect(page.getByTestId('feedback-host-dialog')).toHaveCount(0);
            migrationBefore = await stableStats(page);
        }
        const expected = index < warmups ? index % 2 === 0 : (index - warmups) % 2 === 0;
        await captureNew(page, 'notification_migration', 'feedback-layer-select', 'keydown', expected);
        await page.getByTestId('feedback-layer-select').press('F2');
        const row = await newResult(page);
        expect(row.close_requests).toBe(0);
        if (index >= warmups) migrationRows.push(row);
    }
    const migrationAfter = await stableStats(page);
    expect(migrationAfter).toEqual(migrationBefore);
    workloads.notification_migration = { ...timing(migrationRows), input: 'trusted F2 on the open notification Select; existing Popover and Select migrate into/out of the modal',
        input_events: migrationRows.length, idle: { before: migrationBefore, after: migrationAfter, delta: difference(migrationBefore, migrationAfter) }, raw: migrationRows };
    await page.evaluate(() => { delete (window as any).__panel_bench.new_nodes; });
    await page.keyboard.press('Escape');
    await page.getByTestId('feedback-review-close').click();
    await page.getByTestId('feedback-layer-toast-close').focus();await page.keyboard.press('Enter');
    await page.getByTestId('feedback-mode-queue').click();

    const queueTools = page.getByTestId('feedback-notice-tools');
    await queueTools.locator(':scope > summary').click();
    await page.getByTestId('feedback-notice-duration').fill('5000');
    const prepareQueue = async () => {
        await page.getByTestId('feedback-notices-clear').focus();await page.keyboard.press('Enter');
        for (const slot of [0, 1, 2]) { await page.getByTestId(`feedback-notice-show-${slot}`).focus();await page.keyboard.press('Enter'); }
        await expect(page.getByTestId('feedback-toast-0')).toBeVisible();
        await expect(page.getByTestId('feedback-toast-1')).toBeVisible();
        await expect(page.getByTestId('feedback-toast-2')).toBeHidden();
        await page.getByTestId('feedback-toast-close-0').focus();
    };
    const fifoRows: any[] = [];
    let fifoBefore: any;
    for (let index = 0; index < warmups + samples; index++) {
        await prepareQueue();
        if (index === warmups) fifoBefore = await stableStats(page);
        await captureNew(page, 'fifo_slot', 'feedback-toast-close-0', 'click', true);
        await page.keyboard.press('Enter');
        const row = await newResult(page);
        expect(row.request_delta).toEqual([1, 0, 0, 0]);
        if (index >= warmups) fifoRows.push(row);
    }
    await prepareQueue();
    const fifoAfter = await stableStats(page);
    expect(fifoAfter).toEqual(fifoBefore);
    workloads.fifo_slot = { ...timing(fifoRows), input: 'trusted keyboard-generated Close click; third FIFO notice takes the released slot with the second still visible',
        input_events: fifoRows.length, setup: 'clear and reopen three notices outside timing; two visible slots; 5000 ms reading time',
        idle: { before: fifoBefore, after: fifoAfter, delta: difference(fifoBefore, fifoAfter) }, raw: fifoRows };
    await page.getByTestId('feedback-notices-clear').focus();await page.keyboard.press('Enter');

    await page.getByTestId('content-tools').locator(':scope > summary').click();
    await expect(page.getByTestId('content-scroll')).toHaveAttribute('data-y-overflow', '');
    const track = page.getByTestId('content-scroll-y');
    await track.scrollIntoViewIfNeeded();
    const scrollRows: any[] = [];
    let scrollBefore: any;
    for (let index = 0; index < warmups + samples; index++) {
        if (index === warmups) scrollBefore = await stableStats(page);
        const box = (await track.boundingBox())!, direction = index % 2 === 0 ? 1 : -1;
        await page.mouse.move(box.x + box.width / 2, box.y + box.height * (direction > 0 ? .8 : .2));
        await captureNew(page, 'scroll_track', 'content-scroll-y', 'pointerdown', direction);
        await page.mouse.down();
        let row: any;
        try { row = await newResult(page); expect(row.captured).toBe(true); }
        finally { await page.mouse.up(); }
        expect(await track.evaluate((element, pointer: number) => element.hasPointerCapture(pointer), row.pointer_id)).toBe(false);
        if (index >= warmups) scrollRows.push({ ...row, capture_released: true });
    }
    const scrollAfter = await stableStats(page);
    expect(scrollAfter).toEqual(scrollBefore);
    workloads.scroll_track = { ...timing(scrollRows), input: 'trusted pointerdown alternates 80%/20% of the vertical track; native scrollTop and projected thumb reach a frame before pointerup',
        input_events: scrollRows.length, idle: { before: scrollBefore, after: scrollAfter, delta: difference(scrollBefore, scrollAfter) }, raw: scrollRows };
    await page.getByTestId('content-outside').focus();await page.mouse.move(2, 2);
    await page.getByTestId('content-tools').locator(':scope > summary').click();
    await page.getByTestId('feedback-mode-layers').click();await page.getByTestId('feedback-mode-queue').click();
    // The final remount drops the queue announcements and all per-session timers.
    const overallAfter = await stableStats(page);
    expect(overallAfter).toEqual(overallBefore);
    return { warmups, samples, comparison: 'new capabilities only; no matching baseline or latency delta', workloads,
        cleanup: { before: overallBefore, after: overallAfter, delta: difference(overallBefore, overallAfter),
            coverage: 'runtime tasks/timers/observers, DOM nodes and geometry rules here; detailed observer/listener removal and cancellation in content.spec.ts and feedback.spec.ts' } };
}

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
    const tabs = page.getByRole('tablist', { name: 'Entry views', exact: true });
    await tabs.getByRole('tab', { name: 'Details', exact: true }).click();
    await expect.poll(() => snapshot(page)).toMatchObject({ requests: 2, accepts: 0, dialog: false, active: 'summary' });
    await page.evaluate(() => (window as any).__panel_bench.policy(true));
    const workloads: Record<string, any> = {};
    for (const control of ['dialog', 'tabs', 'tooltip', 'toast', 'progress', 'scroll']) {
        if (control === 'toast' && phase === 'current') {
            // The live region retains one latest announcement, independently of visible messages.
            await page.getByTestId('bench-toast-show').click();
            await page.getByTestId('bench-toaster-toast-dismiss').click();
            await expect(page.getByTestId('bench-toaster-toast')).toHaveCount(0);
        }
        const before = await page.evaluate(() => (window as any).__panel_bench.stats());
        const start = await snapshot(page);
        const rows: any[] = [];
        for (let index = 0; index < warmups + samples; index++) {
            // Setup does not belong to the measured input (replacement starts with a visible toast).
            if (control === 'toast' && index === 0) await page.getByTestId('bench-toast-show').click();
            if (control === 'tooltip') await page.getByTestId('bench-progress-update').focus();
            const expected = control === 'dialog' || control === 'tooltip' ? index % 2 === 0
                : control === 'tabs' ? (index % 2 === 0 ? 'details' : 'summary')
                : control === 'progress' ? index + 1 : control === 'toast' ? `Saved ${index + (phase === 'current' ? 3 : 2)}` : index;
            // Focus is the tooltip input. Alternating focus/Escape requires closing setup only on open samples.
            if (control === 'tooltip' && expected === false) {
                await page.getByTestId('bench-tooltip-trigger').focus();
                await expect(page.getByTestId('bench-tooltip')).toBeVisible();
            }
            await page.evaluate(({ control, expected, current }) => {
                const bench = (window as any).__panel_bench;
                bench.pending = null;
                const waitReady = current && expected === true && (control === 'dialog' || control === 'tooltip');
                const type = control === 'scroll' ? 'wheel' : control === 'tooltip' ? (expected ? 'focusin' : 'keydown') : control === 'dialog' && !expected ? 'keydown' : 'click';
                const scroll = document.querySelector('[data-testid="bench-scroll"]')!;
                const oldScroll = scroll.scrollTop;
                document.addEventListener(type, () => {
                    const at = performance.now();
                    const initial = bench.snapshot();
                    let accepted: any = null;
                    let acceptedNode: Element | null = null;
                    const observe = () => {
                        const state = bench.snapshot();
                        const dialog = document.querySelector('[data-testid="bench-dialog"][role="dialog"], [data-testid="bench-dialog"] [role="dialog"]');
                        const tooltip = document.querySelector('[data-testid="bench-tooltip"][role="tooltip"], [data-testid="bench-tooltip"] [role="tooltip"]');
                        const toast = document.querySelector('[data-testid="bench-toaster"] [data-name="ToastMessage"]');
                        const panel = control === 'tabs' ? document.querySelector(`[data-testid="panel-${expected}"]`) as HTMLElement | null : null;
                        const good = control === 'dialog' ? state.dialog === expected && !!dialog === expected
                            : control === 'tabs' ? state.active === expected && !!panel && !panel.hidden
                            : control === 'tooltip' ? state.tooltip === expected && !!tooltip === expected
                            : control === 'toast' ? toast?.textContent === expected && state.requests > initial.requests
                            : control === 'progress' ? state.progress === expected && Math.abs(Number(document.querySelector('[data-testid="bench-progress"]')?.getAttribute('aria-valuenow')) - Number(expected)) < 1e-9
                            : scroll.scrollTop !== oldScroll;
                        if (!accepted && good) {
                            accepted = { latency_ms: performance.now() - at, input_ms: at, dom_frame_ms: performance.now(), state, request_delta: state.requests - initial.requests, accept_delta: state.accepts - initial.accepts, scroll_top: scroll.scrollTop };
                            acceptedNode = control === 'dialog' ? dialog : tooltip;
                        }
                        if (accepted) {
                            if (!waitReady) { bench.pending = accepted; return; }
                            const ready = acceptedNode?.isConnected && !acceptedNode.closest('[hidden], [inert]')
                                && getComputedStyle(acceptedNode).visibility === 'visible'
                                && Array.from(acceptedNode.getClientRects()).some(rect => rect.width > 0 && rect.height > 0);
                            if (ready) {
                                const readyAt = performance.now();
                                bench.pending = { ...accepted, ready_latency_ms: readyAt - at, ready_frame_ms: readyAt };
                                return;
                            }
                        }
                        if (performance.now() - at < 5000) requestAnimationFrame(observe);
                        else bench.pending = { error: accepted ? 'accepted DOM did not become ready' : 'accepted result did not reach DOM', control, expected, state, accepted };
                    };
                    requestAnimationFrame(observe);
                }, { capture: true, once: true });
            }, { control, expected, current: phase === 'current' });
            if (control === 'dialog') {
                if (expected) await page.getByTestId('bench-dialog-open').click();
                else await page.keyboard.press('Escape');
            } else if (control === 'tabs') await tabs.getByRole('tab', { name: expected === 'details' ? 'Details' : 'Summary', exact: true }).click();
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
        expect(after, `${control}: resources after the workload`).toEqual(before);
        workloads[control] = { ...timing(rows),
            requests: end.requests - start.requests, accepts: end.accepts - start.accepts, idle: { before, after }, raw: rows };
    }
    const newOnly = phase === 'current' ? await currentOnly(page) : undefined;
    expect(errors).toEqual([]);
    await mkdir(directory, { recursive: true });
    await page.screenshot({ path: `${directory}/${phase}-panel.png`, fullPage: true });
    const product = process.env.DOM_ONLY_DIR || 'target/dom/dom-consumer';
    const sizes: Record<string, any> = {};
    const imageSizes: Record<string, any> = {};
    for (const file of (await readdir(product, { recursive: true })).filter(file => /\.(wasm|css|js|svg|png|jpe?g|gif|webp|avif)$/.test(file)).sort()) {
        const bytes = await readFile(resolve(product, file));
        const bucket = /\.(wasm|css|js)$/.test(file) ? sizes : imageSizes;
        bucket[file] = { bytes: bytes.length, gzip: gzipSync(bytes).length };
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
        report.new_only = { ...newOnly, open_ready: {
            endpoint: 'same accepted role node connected, with a nonempty rect, no hidden/inert ancestor and visible computed visibility on an animation frame; not paint presentation or autofocus completion',
            input_pacing: 'current Dialog/Tooltip open samples wait for ready before the next input; baseline releases after logical DOM acceptance. Matched latency_ms/dom_frame_ms retain that first logical DOM endpoint.',
            source: 'opening rows from the existing matched workload, excluding its five warmups; no additional rounds or baseline delta',
            workloads: Object.fromEntries(['dialog', 'tooltip'].map(name => {
                const rows = workloads[name].raw.filter((row: any) => typeof row.ready_latency_ms === 'number');
                return [name, { samples: rows.length, p50_ms: percentile(rows.map((row: any) => row.ready_latency_ms), .5), p95_ms: percentile(rows.map((row: any) => row.ready_latency_ms), .95) }];
            })),
        }, image_resources: {
            scope: 'local URL media fixtures; separate from matched wasm/CSS/runtime sizes and latency deltas',
            files: imageSizes, count: Object.keys(imageSizes).length, bytes: Object.values(imageSizes).reduce((sum, file) => sum + file.bytes, 0),
            gzip: Object.values(imageSizes).reduce((sum, file) => sum + file.gzip, 0) } };
    }
    await mkdir(directory, { recursive: true });
    await writeFile(`${directory}/${phase}.json`, JSON.stringify(report, null, 2) + '\n');
});
