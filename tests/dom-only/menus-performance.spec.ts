import { test, expect, Page } from '@playwright/test';
import { EVIDENCE } from '../tier';
import { mkdir, readFile, readdir, writeFile } from 'node:fs/promises';
import { resolve } from 'node:path';
import { gzipSync } from 'node:zlib';
import { execFileSync } from 'node:child_process';
import { platform, release } from 'node:os';

const phase = process.env.MENU_BENCH_PHASE;
const directory = 'docs/validation/dom-components-p4';
const warmups = 5, samples = 40;
const viewport = { width: 1440, height: 900 };
const percentile = (values: number[], p: number) => [...values].sort((a, b) => a - b)[Math.ceil(values.length * p) - 1];
const snapshot = (page: Page) => page.evaluate(() => (window as any).__menu_bench.snapshot());
async function stableStats(page: Page) {
    await page.evaluate(async () => { for (let i = 0; i < 3; i++) await new Promise<void>(resolve => requestAnimationFrame(() => resolve())); });
    return page.evaluate(() => ({ ...(window as any).__menu_bench.stats(), page_listeners: (window as any).__menu_listeners.size }));
}
type Action = 'open' | 'move' | 'activate';
async function newWorkloads(page: Page) {
    await page.goto('menus.html?parts');
    await expect(page.locator('#status')).toHaveAttribute('data-status', 'ready');
    const definitions = [
        { name: 'submenu_open', trigger: 'menu-trigger', prepare: [{ key: 'ArrowDown', focus: 'menu-alpha' }, { key: 'End', focus: 'menu-sub-trigger' }], key: 'ArrowRight', focus: 'menu-sub-copy', content: 'menu-sub-content', counter: 'menu-requests', expected: 1, close: [{ key: 'Escape', focus: 'menu-sub-trigger' }, { key: 'Escape', focus: 'menu-trigger' }] },
        { name: 'menubar_switch', trigger: 'menubar-file', prepare: [{ key: 'ArrowDown', focus: 'menubar-new' }], key: 'ArrowRight', focus: 'menubar-undo', content: 'menubar-edit-content', counter: 'menubar-log', expected: 1, close: [{ key: 'Escape', focus: 'menubar-edit' }] },
        { name: 'navigation_open', trigger: 'navigation-learn', prepare: [], key: 'Enter', focus: 'navigation-learn', content: 'navigation-learn-content', counter: 'navigation-log', expected: 1, close: [{ key: 'ArrowDown', focus: 'navigation-guide' }, { key: 'Escape', focus: 'navigation-learn' }] },
        { name: 'toolbar_roving', trigger: 'toolbar-save', prepare: [], key: 'ArrowRight', focus: 'toolbar-bold', content: 'toolbar', counter: 'toolbar-log', expected: 0, close: [] },
    ];
    const workloads: Record<string, unknown> = {};
    for (const definition of definitions) {
        const rows: any[] = [];
        let before: unknown;
        for (let round = 0; round < warmups + samples; round++) {
            if (round === warmups) before = await stableStats(page);
            await page.getByTestId(definition.trigger).focus();
            for (const step of definition.prepare) {
                await page.keyboard.press(step.key);
                await expect(page.getByTestId(step.focus)).toBeFocused();
            }
            await page.evaluate(definition => {
                const bench = (window as any).__menu_bench;
                bench.pending = null;
                const count = () => {
                    const value = document.querySelector(`[data-testid="${definition.counter}"]`)!.textContent!;
                    return definition.counter === 'menu-requests' ? Number(value) : value ? value.split(definition.counter === 'toolbar-log' ? ';' : ',').length : 0;
                };
                const capture = (event: KeyboardEvent) => {
                    document.removeEventListener('keydown', capture, true);
                    const start = performance.now(), initial = count();
                    const observe = () => {
                        const content = document.querySelector(`[data-testid="${definition.content}"]`);
                        const focus = document.activeElement?.getAttribute('data-testid');
                        if (content && content.getBoundingClientRect().height > 0 && !content.closest('[hidden],[inert]') && focus === definition.focus) {
                            bench.pending = { latency_ms: performance.now() - start, trusted: event.isTrusted, focus, request_delta: count() - initial };
                        } else if (performance.now() - start < 5000) requestAnimationFrame(observe);
                        else bench.pending = { error: 'new workload did not reach accepted DOM and focus', definition, focus };
                    };
                    requestAnimationFrame(observe);
                };
                document.addEventListener('keydown', capture, true);
            }, definition);
            await page.keyboard.press(definition.key);
            await page.waitForFunction(() => (window as any).__menu_bench.pending !== null);
            const row = await page.evaluate(() => (window as any).__menu_bench.pending);
            expect(row.error, JSON.stringify(row)).toBeUndefined();
            expect(row.trusted).toBe(true);
            expect(row.request_delta, definition.name).toBe(definition.expected);
            if (round >= warmups) rows.push(row);
            for (const step of definition.close) {
                await page.keyboard.press(step.key);
                await expect(page.getByTestId(step.focus)).toBeFocused();
            }
            await page.getByTestId('menu-outside').focus();
        }
        const after = await stableStats(page);
        expect(after, definition.name).toEqual(before);
        workloads[definition.name] = { input: definition.key, endpoint: 'accepted DOM and focus on animation frame',
            p50_ms: percentile(rows.map(row => row.latency_ms), .5), p95_ms: percentile(rows.map(row => row.latency_ms), .95),
            raw: rows, cleanup: { before, after } };
    }
    return workloads;
}
async function measure(page: Page, action: Action) {
    await page.evaluate(action => {
        const bench = (window as any).__menu_bench;
        bench.pending = null;
        const capture = (event: KeyboardEvent) => {
            document.removeEventListener('keydown', capture, true);
            const at = performance.now(), initial = bench.snapshot();
            const observe = () => {
                const state = bench.snapshot();
                const focused = document.activeElement?.getAttribute('data-testid');
                const menu = document.querySelector('[data-testid="bench-menu"]');
                const good = action === 'open' ? state.open && focused === 'menu-item-bench-open'
                    : action === 'move' ? state.open && focused === 'menu-item-bench-copy'
                    : !state.open && !menu && state.last === 'bench-copy' && state.commands === initial.commands + 1;
                if (good) bench.pending = { latency_ms: performance.now() - at, input_ms: at, dom_frame_ms: performance.now(),
                    trusted: event.isTrusted, focused, state, request_delta: state.requests - initial.requests,
                    accept_delta: state.accepts - initial.accepts, command_delta: state.commands - initial.commands };
                else if (performance.now() - at < 5000) requestAnimationFrame(observe);
                else bench.pending = { error: 'menu input did not reach accepted DOM and focus', action, state, focused };
            };
            requestAnimationFrame(observe);
        };
        document.addEventListener('keydown', capture, true);
    }, action);
    await page.keyboard.press(action === 'move' ? 'ArrowDown' : 'Enter');
    await page.waitForFunction(() => (window as any).__menu_bench.pending !== null);
    const row = await page.evaluate(() => (window as any).__menu_bench.pending);
    expect(row.error, JSON.stringify(row)).toBeUndefined();
    expect(row.trusted).toBe(true);
    expect(row.request_delta).toBe(action === 'move' ? 0 : 1);
    expect(row.accept_delta).toBe(action === 'move' ? 0 : 1);
    expect(row.command_delta).toBe(action === 'activate' ? 1 : 0);
    return row;
}

test('menu native-input frame workloads', { tag: EVIDENCE }, async ({ page, browser }) => {
    test.setTimeout(180_000);
    expect(['baseline', 'current']).toContain(phase);
    const errors: string[] = [];
    page.on('pageerror', error => errors.push(error.message));
    page.on('console', message => { if (message.type() === 'error') errors.push(message.text()); });
    page.on('requestfailed', request => errors.push(`${request.url()}: ${request.failure()?.errorText}`));
    await page.addInitScript(() => {
        const add = EventTarget.prototype.addEventListener, remove = EventTarget.prototype.removeEventListener;
        const listeners = new Set<{ target: EventTarget, type: string, listener: any, capture: boolean }>();
        EventTarget.prototype.addEventListener = function(type: string, listener: any, options: any) {
            const capture = typeof options === 'boolean' ? options : !!options?.capture;
            if ((this === document || this === window) && listener && !options?.signal?.aborted
                && ![...listeners].some(row => row.target === this && row.type === type && row.listener === listener && row.capture === capture)) {
                const row = { target: this, type, listener, capture };
                listeners.add(row);
                if (options?.signal) add.call(options.signal, 'abort', () => listeners.delete(row), { once: true });
            }
            return add.call(this, type, listener, options);
        };
        EventTarget.prototype.removeEventListener = function(type: string, listener: any, options: any) {
            const capture = typeof options === 'boolean' ? options : !!options?.capture;
            for (const row of listeners) if (row.target === this && row.type === type && row.listener === listener && row.capture === capture) listeners.delete(row);
            return remove.call(this, type, listener, options);
        };
        (window as any).__menu_listeners = listeners;
    });
    await page.setViewportSize(viewport);
    await page.goto('menus.html');
    await expect(page.locator('#status')).toHaveAttribute('data-status', 'ready');
    const trigger = page.getByTestId('bench-menu-open');
    await page.evaluate(() => (window as any).__menu_bench.policy(false));
    await trigger.focus(); await page.keyboard.press('Enter');
    await expect.poll(() => snapshot(page)).toMatchObject({ requests: 1, accepts: 0, commands: 0, open: false });
    await page.evaluate(() => (window as any).__menu_bench.policy(true));
    await page.keyboard.press('Enter');
    await expect(page.getByTestId('menu-item-bench-open')).toBeFocused();
    await page.evaluate(() => (window as any).__menu_bench.policy(false));
    await page.keyboard.press('Escape');
    await expect.poll(() => snapshot(page)).toMatchObject({ requests: 3, accepts: 1, commands: 0, open: true });
    await page.evaluate(() => (window as any).__menu_bench.policy(true));
    await page.keyboard.press('Escape');
    await expect(page.getByTestId('bench-menu')).toHaveCount(0);
    const rows: Record<Action, any[]> = { open: [], move: [], activate: [] };
    let before: any;
    const start = await snapshot(page);
    for (let index = 0; index < warmups + samples; index++) {
        await trigger.focus();
        if (index === warmups) before = await stableStats(page);
        for (const action of ['open', 'move', 'activate'] as const) {
            const row = await measure(page, action);
            if (index >= warmups) rows[action].push(row);
        }
    }
    const after = await stableStats(page), end = await snapshot(page);
    expect(after).toEqual(before);
    expect(end.requests - start.requests).toBe(2 * (warmups + samples));
    expect(end.accepts - start.accepts).toBe(2 * (warmups + samples));
    expect(end.commands - start.commands).toBe(warmups + samples);
    expect(errors).toEqual([]);
    const workloads = Object.fromEntries(Object.entries(rows).map(([name, raw]) => [name, {
        p50_ms: percentile(raw.map(row => row.latency_ms), .5), p95_ms: percentile(raw.map(row => row.latency_ms), .95), raw,
    }]));
    const product = process.env.DOM_ONLY_DIR || 'target/dom/dom-consumer';
    const sizes: Record<string, { bytes: number, gzip: number }> = {};
    for (const file of (await readdir(product, { recursive: true })).filter(file => /\.(wasm|css)$/.test(file)).sort()) {
        const bytes = await readFile(resolve(product, file));
        sizes[file] = { bytes: bytes.length, gzip: gzipSync(bytes).length };
    }
    const parameters = { warmups, samples, viewport, browser: browser.version(), os: { platform: platform(), release: release() },
        environment: await page.evaluate(() => ({ user_agent: navigator.userAgent, dpr: devicePixelRatio, visibility: document.visibilityState })),
        rust: execFileSync('rustc', ['--version'], { encoding: 'utf8' }).trim(), node: process.version,
        load: { items: 6, enabled_commands: 3, input: 'trusted Enter / ArrowDown / Enter', endpoint: 'accepted DOM and focus on animation frame' } };
    const report: any = { phase, collected_at: new Date().toISOString(), source: execFileSync('git', ['rev-parse', 'HEAD'], { encoding: 'utf8' }).trim(),
        worktree: execFileSync('git', ['status', '--short'], { encoding: 'utf8' }), product, parameters, sizes, workloads, cleanup: { before, after } };
    if (phase === 'current') {
        const baseline = JSON.parse(await readFile(`${directory}/baseline.json`, 'utf8'));
        expect(parameters).toEqual(baseline.parameters);
        report.delta = Object.fromEntries(Object.entries(workloads).map(([name, row]) => [name, {
            p50_ms: row.p50_ms - baseline.workloads[name].p50_ms, p95_ms: row.p95_ms - baseline.workloads[name].p95_ms,
        }]));
        report.size_delta = Object.fromEntries(Object.entries(sizes).map(([file, size]) => [file, {
            bytes: size.bytes - (baseline.sizes[file]?.bytes || 0), gzip: size.gzip - (baseline.sizes[file]?.gzip || 0),
        }]));
    }
    await mkdir(directory, { recursive: true });
    await trigger.focus(); await page.keyboard.press('Enter');
    await expect(page.getByTestId('menu-item-bench-open')).toBeFocused();
    await page.screenshot({ path: `${directory}/${phase}-menu.png`, fullPage: true });
    await page.keyboard.press('Escape');
    if (phase === 'current') report.new_only = await newWorkloads(page);
    expect(errors).toEqual([]);
    await writeFile(`${directory}/${phase}.json`, JSON.stringify(report, null, 2) + '\n');
});
