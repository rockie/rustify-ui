import { test, expect } from '@playwright/test';

async function ready(page: any) {
    await page.goto('./');
    await expect(page.locator('#status')).toHaveAttribute('data-status', 'ready');
}

test('failed CSSOM installation nacks preparation, preserves the installed revision and permits retry', async ({ page }) => {
    await ready(page);
    const result = await page.evaluate(async () => {
        const api = (window as any).__utility_lab;
        await api.ensure('scope-a', ['w-[137px]', 'bg-[#123456]']);
        const sample = document.getElementById('scope-a-sample')!;
        sample.className = 'w-[137px] bg-[#123456]';
        await new Promise(resolve => setTimeout(resolve, 0));
        const before = api.report('scope-a');
        const original = CSSStyleSheet.prototype.replaceSync;
        CSSStyleSheet.prototype.replaceSync = function() { throw new DOMException('injected install failure', 'SyntaxError'); };
        let failure;
        try { await api.ensure('scope-a', ['w-[179px]']); } catch (error) { failure = JSON.parse(String(error)); }
        finally { CSSStyleSheet.prototype.replaceSync = original; }
        const retained = api.report('scope-a');
        const retainedWidth = getComputedStyle(sample).width;
        const next = await api.ensure('scope-a', ['w-[179px]']);
        sample.className = 'w-[179px] bg-[#123456]';
        return { failure, before: before.revision, retained: retained.revision,
            retainedWidth, next: next.revision, width: getComputedStyle(sample).width };
    });
    expect(result.failure.kind).toBe('install');
    expect(result.retained).toBe(result.before);
    expect(result.retainedWidth).toBe('137px');
    expect(result.next).toBe(result.before + 1);
    expect(result.width).toBe('179px');
});

test('Worker timeout preserves installed CSS and explicit retry replays in a fresh epoch', async ({ page }) => {
    await ready(page);
    const result = await page.evaluate(async () => {
        const api = (window as any).__utility_lab;
        api.dispose('scope-a');
        api.mount('scope-a', 1000);
        await api.ensure('scope-a', ['w-[137px]']);
        const sample = document.getElementById('scope-a-sample')!;
        sample.className = 'w-[137px]';
        const before = api.report('scope-a').revision;
        const original = Worker.prototype.postMessage;
        Worker.prototype.postMessage = function(message: any, ...args: any[]) {
            if (message.operation === 'ack') return;
            return original.call(this, message, ...args as [any]);
        };
        let failure;
        try { await api.ensure('scope-a', ['w-[179px]']); } catch (error) { failure = JSON.parse(String(error)); }
        finally { Worker.prototype.postMessage = original; }
        const installed = api.report('scope-a');
        sample.className = 'w-[179px]';
        const width = getComputedStyle(sample).width;
        const replayed = await api.retry('scope-a');
        return { failure, before, installed: installed.revision, width, replayedWidth: getComputedStyle(sample).width,
            replayedCss: replayed.css, sheetCount: document.adoptedStyleSheets.length };
    });
    expect(result.failure.kind).toBe('timeout');
    expect(result.installed).toBe(result.before + 1);
    expect(result.width).toBe('179px');
    expect(result.replayedWidth).toBe('179px');
    expect(result.sheetCount).toBeLessThanOrEqual(2);
    expect(result.replayedCss).toContain('179px');
});

test('instance trap releases Worker, observer, owner attribute and installed stylesheet through JS abort cleanup', async ({ page }) => {
    await ready(page);
    const result = await page.evaluate(async () => {
        const api = (window as any).__utility_lab;
        await api.ensure('scope-a', ['w-[137px]']);
        const before = document.adoptedStyleSheets.length;
        try { api.trap(); } catch { /* loader attributes the actual WASM trap */ }
        return { before, after: document.adoptedStyleSheets.length,
            owners: document.querySelectorAll('[data-rustify-style-owner]').length,
            fatal: api.hooks.runtime.fatal !== null };
    });
    expect(result.before).toBeGreaterThan(0);
    expect(result.after).toBe(0);
    expect(result.owners).toBe(0);
    expect(result.fatal).toBe(true);
});

test('a second WASM instance keeps its compiler and stylesheet when the first instance traps', async ({ page }) => {
    await ready(page);
    const result = await page.evaluate(async () => {
        const api = (window as any).__utility_lab;
        const root = document.createElement('div'); root.id = 'instance-two'; document.body.append(root);
        const second = await api.second('instance-two');
        await second.app.utility_lab_ensure('instance-two', JSON.stringify(['w-[93px]']));
        const sample = document.getElementById('instance-two-sample')!; sample.className = 'w-[93px]';
        await api.ensure('scope-a', ['w-[137px]']);
        try { api.trap(); } catch { /* actual fatal path belongs to the first instance */ }
        const retained = getComputedStyle(sample).width;
        await second.app.utility_lab_ensure('instance-two', JSON.stringify(['w-[179px]']));
        sample.className = 'w-[179px]';
        const width = getComputedStyle(sample).width;
        const owners = document.querySelectorAll('[data-rustify-style-owner]').length;
        second.app.utility_lab_dispose('instance-two');
        return { retained, width, owners, after: document.adoptedStyleSheets.length };
    });
    expect(result.retained).toBe('93px');
    expect(result.width).toBe('179px');
    expect(result.owners).toBe(1);
    expect(result.after).toBe(0);
});

test('a Worker construction failure leaves the root available for a later mount', async ({ page }) => {
    await ready(page);
    const result = await page.evaluate(() => {
        const api = (window as any).__utility_lab;
        api.dispose('scope-a');
        const original = window.Worker;
        window.Worker = class { constructor() { throw new DOMException('injected Worker construction failure', 'SecurityError'); } } as any;
        let failure;
        try { api.mount('scope-a'); } catch (error) { failure = String(error); }
        finally { window.Worker = original; }
        const occupied = document.getElementById('scope-a')!.hasAttribute('data-rustify-style-owner');
        api.mount('scope-a');
        return { failure, occupied, remounted: document.getElementById('scope-a')!.hasAttribute('data-rustify-style-owner') };
    });
    expect(result.failure).toContain('injected Worker construction failure');
    expect(result.occupied).toBe(false);
    expect(result.remounted).toBe(true);
});

test('limit boundaries, apply cycles, deep nesting and remote imports fail without changing the installed revision or fetching', async ({ page }, info) => {
    const remote: string[] = [];
    page.on('request', request => { if (!request.url().startsWith('http://127.0.0.1')) remote.push(request.url()); });
    await ready(page);
    const result = await page.evaluate(async () => {
        const api = (window as any).__utility_lab;
        const fail = async (work: Promise<unknown>) => { try { await work; return null; } catch (error) { return JSON.parse(String(error)); } };
        const host = document.createElement('div');
        host.id = 'scope-limits';
        document.querySelector('main')!.append(host);
        const limits = { input_bytes: 65536, asset_bytes: 65536, candidate_count: 1024, ast_depth: 32, css_bytes: 262144, queued_bytes: 65536, cache_bytes: 1048576 };
        api.mount('scope-limits', 10000, undefined, limits);
        await api.flush('scope-limits');
        // Remount at exactly the installed count plus one: one new class fits, a second does not.
        const baseline = api.retained('scope-limits');
        api.dispose('scope-limits');
        host.replaceChildren();
        api.mount('scope-limits', 10000, undefined, { ...limits, candidate_count: baseline + 1 });
        await api.flush('scope-limits');
        const sample = document.getElementById('scope-limits-sample')!;
        const atLimit = await api.ensure('scope-limits', ['w-[141px]']);
        sample.className = 'sample w-[141px]';
        const over = await fail(api.ensure('scope-limits', ['w-[143px]']));
        const afterOver = api.report('scope-limits');
        const widthAfterOver = getComputedStyle(sample).width;
        // Configuration failures, in a scope with room to rebuild.
        api.dispose('scope-limits');
        host.replaceChildren();
        api.mount('scope-limits', 10000, undefined, limits);
        const roomy = await api.ensure('scope-limits', ['w-[141px]']);
        const configured = document.getElementById('scope-limits-sample')!;
        configured.className = 'sample w-[141px]';
        const cycle = await fail(api.replace('scope-limits', '@utility loop-a { @apply loop-b; } @utility loop-b { @apply loop-a; } @tailwind utilities;'));
        const afterCycle = api.report('scope-limits');
        const deep = await fail(api.replace('scope-limits', '.deep {'.repeat(40) + 'color: red;' + '}'.repeat(40) + ' @tailwind utilities;'));
        const afterDeep = api.report('scope-limits');
        const imported = await fail(api.replace('scope-limits', '@import "https://example.invalid/remote.css"; @tailwind utilities;'));
        const afterImport = api.report('scope-limits');
        await new Promise(resolve => setTimeout(resolve, 200));
        const width = getComputedStyle(configured).width;
        api.dispose('scope-limits');
        host.remove();
        return { baseline, atLimit: atLimit.revision, over, afterOver: afterOver.revision, widthAfterOver, roomy: roomy.revision,
            cycle, afterCycle: afterCycle.revision, deep, afterDeep: afterDeep.revision,
            imported, afterImport: afterImport.revision, width };
    });
    await info.attach('Fault results', { body: JSON.stringify(result, null, 2), contentType: 'application/json' });
    expect(result.over.kind).toBe('compile');
    expect(result.over.error).toEqual({ kind: 'limit', resource: 'candidate_count', limit: result.baseline + 1 });
    expect(result.afterOver).toBe(result.atLimit);
    for (const revision of [result.afterCycle, result.afterDeep, result.afterImport]) expect(revision).toBe(result.roomy);
    expect(result.widthAfterOver).toBe('141px');
    expect(result.width).toBe('141px');
    expect(result.cycle).not.toBeNull();
    expect(JSON.stringify(result.cycle)).toMatch(/circular|cycle/i);
    expect(result.deep).not.toBeNull();
    expect(JSON.stringify(result.deep)).toContain('ast_depth');
    expect(result.imported.error).toEqual({ kind: 'unsupported_feature', feature: 'runtime global network resources' });
    expect(remote).toEqual([]);
});
