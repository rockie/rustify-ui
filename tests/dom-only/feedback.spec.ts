import { test, expect, type Page, type Locator } from '@playwright/test';

type TimerRecord = { id: number; delay: number; at: number; cancelled: boolean; fired: boolean };
type Probe = { timers: Map<number, TimerRecord>; csp: string[] };
declare global {
    interface Window {
        __feedbackProbe: Probe;
        __feedbackNodes: Record<string, Element>;
        __panel_bench?: { stats(): { tasks: number; timers: number; observers: number } };
        __primitives?: { stats(): { tasks: number; timers: number; observers: number } };
    }
}
const failures = new WeakMap<Page, string[]>();
const id = (page: Page, name: string) => page.getByTestId(`feedback-${name}`);
const toast = (page: Page, index: number) => id(page, `toast-${index}`);

async function details(page: Page, name: string) {
    const element = id(page, name);
    if (!(await element.evaluate(node => (node as HTMLDetailsElement).open))) {
        await element.locator(':scope > summary').click();
    }
}
async function counters(page: Page, name: string) {
    return (await id(page, name).textContent())!.trim().split(',').map(Number);
}
async function leaveNotice(page: Page) {
    await id(page, 'outside').focus();
    await page.mouse.move(2, 2);
}
async function readingTime(page: Page, ms: number) {
    await details(page, 'notice-tools');
    await id(page, 'notice-duration').fill(String(ms));
}
async function remember(page: Page, names: string[]) {
    await page.evaluate(names => {
        window.__feedbackNodes ??= {};
        for (const name of names) {
            const node = document.querySelector(`[data-testid="feedback-${name}"]`);
            if (!node) throw new Error(`Missing identity target ${name}`);
            window.__feedbackNodes[name] = node;
        }
    }, names);
}
async function sameNodes(page: Page, names: string[]) {
    expect(await page.evaluate(names => names.every(name => window.__feedbackNodes[name] === document.querySelector(`[data-testid="feedback-${name}"]`)), names)).toBe(true);
}
async function liveShape(viewport: Locator) {
    await expect(viewport).toHaveRole('region');
    await expect(viewport).not.toHaveAttribute('aria-live');
    await expect(viewport.locator('[data-rustify-toast-announcer="polite"]')).toHaveCount(1);
    await expect(viewport.locator('[data-rustify-toast-announcer="assertive"]')).toHaveCount(1);
    for (const mode of ['polite', 'assertive']) {
        const region = viewport.locator(`[data-rustify-toast-announcer="${mode}"]`);
        await expect(region).toHaveAttribute('aria-live', mode);
        await expect(region).toHaveAttribute('aria-relevant', 'additions');
        expect(await region.evaluate(node => getComputedStyle(node).clipPath)).not.toBe('none');
    }
    expect(await viewport.locator('[data-rustify-part="toast-root"][aria-live]').count()).toBe(0);
}
async function swipe(page: Page, index: number, delta: number, vertical = 0) {
    const title = id(page, `toast-title-${index}`);
    const box = (await title.boundingBox())!;
    const x = box.x + Math.min(32, box.width / 3), y = box.y + box.height / 2;
    await page.mouse.move(x, y);
    await page.mouse.down();
    await page.mouse.move(x + delta, y + vertical, { steps: 8 });
    await page.mouse.up();
}
async function stats(page: Page) {
    return page.evaluate(() => {
        const source = window.__panel_bench ?? window.__primitives;
        if (!source) throw new Error('Consumer runtime stats are not exposed');
        return {
            ...source.stats(),
            nodes: document.querySelectorAll('*').length,
            sheets: document.adoptedStyleSheets.length,
            rules: Array.from(document.adoptedStyleSheets).reduce((count, sheet) => count + sheet.cssRules.length, 0),
        };
    });
}
async function latestPendingTimer(page: Page, since: number) {
    return page.evaluate(since => Array.from(window.__feedbackProbe.timers.values())
        .filter(timer => timer.at >= since && !timer.cancelled && !timer.fired && timer.delay > 500 && timer.delay <= 6000), since);
}

test.beforeEach(async ({ page }, info) => {
    const errors: string[] = [];
    failures.set(page, errors);
    page.on('pageerror', error => errors.push(error.message));
    page.on('console', message => { if (message.type() === 'error') errors.push(message.text()); });
    page.on('requestfailed', request => errors.push(request.url()));
    await page.addInitScript(() => {
        const nativeSet = window.setTimeout.bind(window), nativeClear = window.clearTimeout.bind(window);
        const probe: Probe = { timers: new Map(), csp: [] };
        window.__feedbackProbe = probe;
        window.addEventListener('securitypolicyviolation', event => probe.csp.push(`${event.violatedDirective}: ${event.blockedURI}`));
        // Keep native numeric handles and the browser clock: wasm APIs accept i32 timer IDs.
        window.setTimeout = ((callback: TimerHandler, delay = 0, ...parameters: unknown[]) => {
            let record: TimerRecord | undefined;
            const invoke = typeof callback === 'function' ? (...args: unknown[]) => {
                if (record) record.fired = true;
                callback.apply(window, args);
            } : callback;
            const handle = nativeSet(invoke, delay, ...parameters);
            record = { id: handle, delay: Number(delay), at: performance.now(), cancelled: false, fired: false };
            probe.timers.set(handle, record);
            return handle;
        }) as typeof window.setTimeout;
        window.clearTimeout = ((handle?: number) => {
            if (handle !== undefined) {
                const timer = probe.timers.get(handle);
                if (timer) timer.cancelled = true;
            }
            nativeClear(handle);
        }) as typeof window.clearTimeout;
    });
    await page.goto(info.project.name === 'dom-primitives' ? '/' : '/panels.html');
    await page.bringToFront();
    await expect(page.locator('#status')).toHaveAttribute('data-status', 'ready');
    await expect(id(page, 'samples')).toBeVisible();
    await id(page, 'samples').scrollIntoViewIfNeeded();
});
test.afterEach(async ({ page }) => {
    expect(failures.get(page)).toEqual([]);
    expect(await page.evaluate(() => window.__feedbackProbe.csp)).toEqual([]);
});

test('progress keeps original units, unknown state, and actionable invalid-input diagnostics', async ({ page }) => {
    const meter = id(page, 'progress');
    await expect(meter).toHaveRole('progressbar');
    await expect(meter).toHaveAttribute('aria-valuenow', '37.5');
    await expect(meter).toHaveAttribute('aria-valuemax', '150');
    await expect(meter).toHaveAttribute('aria-valuetext', '37.5 of 150 review items');
    await expect.poll(async () => id(page, 'progress-indicator').evaluate(node => {
        const root = node.parentElement!.getBoundingClientRect(), bar = node.getBoundingClientRect();
        return bar.width / root.width;
    })).toBeCloseTo(.25, 2);
    await details(page, 'progress-tools');
    await id(page, 'progress-unknown').click();
    await expect(meter).toHaveAttribute('data-state', 'indeterminate');
    await expect(meter).not.toHaveAttribute('aria-valuenow');
    await expect(meter).toHaveAttribute('aria-valuemax', '150');
    await id(page, 'progress-complete').click();
    await expect(meter).toHaveAttribute('aria-valuenow', '150');
    await expect(meter).toHaveAttribute('data-state', 'complete');
    await id(page, 'progress-bad-max').click();
    await expect(meter).toHaveAttribute('data-rustify-issue', 'max');
    await expect(meter).toHaveAttribute('aria-valuemax', '100');
    await expect(meter).not.toHaveAttribute('aria-valuenow');
    await id(page, 'progress-known').click();
    await id(page, 'progress-bad-value').click();
    await expect(meter).toHaveAttribute('data-rustify-issue', 'value');
    await expect(meter).not.toHaveAttribute('aria-valuenow');
    await id(page, 'progress-known').click();
    await expect(meter).not.toHaveAttribute('data-rustify-issue');
});

test('progress changes the painted fill without resizing its layout box and follows RTL', async ({ page }, testInfo) => {
    const meter = id(page, 'progress');
    const indicator = id(page, 'progress-indicator');
    const geometry = () => indicator.evaluate(node => {
        const root = node.parentElement!;
        return { layout: (node as HTMLElement).offsetWidth / root.clientWidth,
            fill: node.getBoundingClientRect().width / root.getBoundingClientRect().width,
            right: node.getBoundingClientRect().right - root.getBoundingClientRect().right + parseFloat(getComputedStyle(root).borderRightWidth) };
    });
    expect((await geometry()).layout).toBeCloseTo(1, 2);
    await expect.poll(async () => (await geometry()).fill).toBeCloseTo(.25, 2);
    await meter.evaluate(node => node.setAttribute('dir', 'rtl'));
    await expect.poll(async () => (await geometry()).right).toBeCloseTo(0, 1);
    await testInfo.attach('progress-rtl', { body: await meter.screenshot(), contentType: 'image/png' });
    await details(page, 'progress-tools');
    await id(page, 'progress-complete').click();
    await expect.poll(async () => (await geometry()).fill).toBeCloseTo(1, 2);
    expect((await geometry()).layout).toBeCloseTo(1, 2);
});

test('loading retains native nodes, drafts and listeners without reclaiming external focus', async ({ page }) => {
    const spinner = id(page, 'spinner-draft'), skeleton = id(page, 'skeleton-draft');
    await spinner.fill('A retained release-owner draft');
    await skeleton.fill('A retained delivery draft');
    await remember(page, ['spinner-draft', 'skeleton-draft', 'spinner-native', 'skeleton-native']);
    const bounds = await id(page, 'spinner-native').evaluate(node => { const box = node.getBoundingClientRect(); return { width: box.width, height: box.height }; });
    await spinner.focus();
    await spinner.press('F2');
    await expect(spinner).not.toBeFocused();
    await expect(id(page, 'spinner-native')).toHaveAttribute('inert', '');
    await expect(id(page, 'skeleton-native')).toHaveAttribute('inert', '');
    await expect(id(page, 'spinner-frame').getByRole('status')).toHaveCount(1);
    await expect(id(page, 'spinner-frame').getByRole('status')).toHaveAccessibleName('loading…');
    await id(page, 'outside').focus();
    await spinner.evaluate(node => (node as HTMLInputElement).focus());
    await expect(id(page, 'outside')).toBeFocused();
    await skeleton.evaluate(node => (node as HTMLInputElement).focus());
    await expect(id(page, 'outside')).toBeFocused();
    expect(await id(page, 'spinner-native').evaluate(node => { const box = node.getBoundingClientRect(); return { width: box.width, height: box.height }; })).toEqual(bounds);
    await id(page, 'loading-finish').click();
    await expect(id(page, 'loading-finish')).toBeFocused();
    await expect(id(page, 'spinner-frame').getByRole('status')).toHaveCount(0);
    await sameNodes(page, ['spinner-draft', 'skeleton-draft', 'spinner-native', 'skeleton-native']);
    await expect(spinner).toHaveValue('A retained release-owner draft');
    await expect(skeleton).toHaveValue('A retained delivery draft');
    await expect(id(page, 'spinner-native')).not.toHaveAttribute('inert');
    for (let turn = 0; turn < 10 && !(await spinner.evaluate(node => node === document.activeElement)); turn++) await page.keyboard.press('Tab');
    await expect(spinner).toBeFocused();
    await page.keyboard.press('Tab');
    await expect(id(page, 'retained-action')).toBeFocused();
    await page.keyboard.press('Space');
    await expect(id(page, 'retained-clicks')).toHaveText('1');
    await page.keyboard.press('Tab');
    await expect(skeleton).toBeFocused();
});

test('all declared loading sizes and progress variants consume current theme and caller utilities', async ({ page }, info) => {
    test.skip(info.project.name === 'dom-primitives', 'The raw API has no themed size or variant parameters.');
    await details(page, 'visual-tools');
    for (const [size, height, variant] of [[1, 4, 'classic'], [2, 6, 'surface'], [3, 8, 'soft']] as const) {
        const meter = id(page, `progress-size-${size}`);
        await expect(meter).toHaveAttribute('data-variant', variant);
        expect((await meter.boundingBox())!.height).toBe(height);
        await expect(id(page, `spinner-size-${size}`)).toHaveRole('status');
    }
    const spinSizes = await Promise.all([1, 2, 3].map(size => id(page, `spinner-size-${size}`).locator('[data-rustify-part="spinner-graphic"]').evaluate(node => node.getBoundingClientRect().width)));
    expect(spinSizes).toEqual([12, 16, 20]);
    expect((await id(page, 'progress-utility').boundingBox())!.height).toBe(40);
    const indicator = id(page, 'progress-indicator-1');
    const defaultColor = await indicator.evaluate(node => getComputedStyle(node).backgroundColor);
    await id(page, 'visual-color').click();
    await expect.poll(() => indicator.evaluate(node => getComputedStyle(node).backgroundColor)).not.toBe(defaultColor);
    const localColor = await indicator.evaluate(node => getComputedStyle(node).backgroundColor);
    await id(page, 'visual-contrast').click();
    await expect.poll(() => indicator.evaluate(node => getComputedStyle(node).backgroundColor)).not.toBe(localColor);
    await id(page, 'visual-radius').click();
    expect(await id(page, 'progress-size-1').evaluate(node => getComputedStyle(node).borderRadius)).toBe('0px');
    await details(page, 'tools');
    await id(page, 'theme').click();
    await expect(id(page, 'theme-boundary')).toHaveAttribute('data-theme', 'dark');
    await id(page, 'theme').click();
    await expect(id(page, 'theme-boundary')).toHaveAttribute('data-theme', 'light');
});

test('reduced motion keeps loading facts readable while stopping authored animation', async ({ page }, info) => {
    test.skip(info.project.name === 'dom-primitives', 'Raw parts provide structure, not the themed animations.');
    await page.emulateMedia({ reducedMotion: 'reduce' });
    await id(page, 'loading-start').click();
    await details(page, 'progress-tools');
    await id(page, 'progress-unknown').click();
    const region = id(page, 'loading');
    await expect(id(page, 'spinner-frame').getByRole('status')).toHaveCount(1);
    for (const target of [id(page, 'progress-indicator'), region.locator('[data-rustify-part="spinner-leaf"]').first(), id(page, 'skeleton-native')]) {
        await expect(target).toHaveCSS('animation-name', 'none');
    }
    await expect(id(page, 'progress')).not.toHaveAttribute('aria-valuenow');
});

test('FIFO limits visible roots and queued entries keep their full reading time', async ({ page }) => {
    await readingTime(page, 1200);
    await id(page, 'notice-show-0').click();
    await expect(id(page, 'toast-viewport').locator('[data-rustify-toast-announcer="polite"]')).toContainText('Summary saved');
    await id(page, 'notice-show-1').click();
    await id(page, 'notice-show-2').click();
    await id(page, 'notice-show-3').click();
    await leaveNotice(page);
    await expect(toast(page, 0)).toBeVisible();
    await expect(toast(page, 1)).toBeVisible();
    await expect(toast(page, 2)).toBeHidden();
    await expect(toast(page, 3)).toBeHidden();
    const viewport = id(page, 'toast-viewport');
    await liveShape(viewport);
    await expect(viewport.locator('[data-rustify-toast-announcer="polite"] > span')).toHaveCount(1);
    await expect(viewport.locator('[data-rustify-toast-announcer="polite"]')).toContainText('Attachment ready');
    await expect(viewport.locator('[data-rustify-toast-announcer="assertive"] > span')).toHaveCount(0);
    await expect(toast(page, 2)).toBeVisible({ timeout: 3000 });
    await expect(toast(page, 3)).toBeVisible();
    await page.waitForTimeout(500);
    await expect(toast(page, 2)).toBeVisible();
    await expect(viewport.locator('[data-rustify-toast-announcer="assertive"] > span')).toHaveCount(1);
    await expect(toast(page, 2)).toBeHidden({ timeout: 2000 });
    await expect(toast(page, 3)).toBeVisible();
    await page.waitForTimeout(1500);
    await expect(toast(page, 3)).toBeVisible();
    expect((await counters(page, 'close-requests')).slice(0, 3)).toEqual([1, 1, 1]);
});

test('Replace waits for a refused old root and does not spend the waiting item duration', async ({ page }) => {
    await id(page, 'mode-replace').click();
    await readingTime(page, 900);
    await id(page, 'notice-refuse').check();
    await id(page, 'notice-show-0').click();
    await id(page, 'notice-show-1').click();
    await leaveNotice(page);
    await expect(toast(page, 0)).toBeVisible();
    await expect(toast(page, 1)).toBeHidden();
    await expect.poll(() => counters(page, 'close-requests')).toEqual([1, 0, 0, 0]);
    await page.waitForTimeout(1300);
    expect(await counters(page, 'close-requests')).toEqual([1, 0, 0, 0]);
    await id(page, 'accept-first-close').click();
    await leaveNotice(page);
    await expect(toast(page, 1)).toBeVisible();
    await page.waitForTimeout(350);
    await expect(toast(page, 1)).toBeVisible();
    await expect(toast(page, 1)).toBeHidden({ timeout: 1600 });
    expect(await counters(page, 'close-requests')).toEqual([1, 1, 0, 0]);
});

test('refused expiry requests happen once until duration explicitly changes', async ({ page }) => {
    await readingTime(page, 800);
    await id(page, 'notice-refuse').check();
    await id(page, 'notice-show-0').click();
    await leaveNotice(page);
    await expect.poll(() => counters(page, 'close-requests')).toEqual([1, 0, 0, 0]);
    await page.waitForTimeout(1100);
    await expect(toast(page, 0)).toBeVisible();
    expect(await counters(page, 'close-requests')).toEqual([1, 0, 0, 0]);
    await id(page, 'notice-duration').fill('1000');
    await leaveNotice(page);
    await expect.poll(() => counters(page, 'close-requests')).toEqual([2, 0, 0, 0]);
    await page.waitForTimeout(400);
    expect(await counters(page, 'close-requests')).toEqual([2, 0, 0, 0]);
    await id(page, 'notices-clear').click();
    await expect(toast(page, 0)).toBeHidden();
});

test('viewport hover and focus compose with blur and visibility-event pauses without resetting time', async ({ page }) => {
    await readingTime(page, 1739);
    const started = await page.evaluate(() => performance.now());
    await id(page, 'notice-show-0').click();
    await id(page, 'notice-show-1').click();
    await leaveNotice(page);
    await page.waitForTimeout(350);
    await id(page, 'toast-action-0').focus();
    await toast(page, 1).hover();
    await page.waitForTimeout(1900);
    await expect(toast(page, 0)).toBeVisible();
    await expect(toast(page, 1)).toBeVisible();
    await id(page, 'outside').focus();
    await page.waitForTimeout(300);
    await expect(toast(page, 0)).toBeVisible();
    await page.evaluate(() => window.dispatchEvent(new Event('blur')));
    await page.mouse.move(2, 2);
    await page.waitForTimeout(350);
    // This isolates the DOM visibility API contract; actual OS backgrounding remains a manual check.
    await page.evaluate(() => {
        Object.defineProperty(document, 'hidden', { configurable: true, get: () => true });
        document.dispatchEvent(new Event('visibilitychange'));
        window.dispatchEvent(new Event('focus'));
    });
    await page.waitForTimeout(600);
    await expect(toast(page, 0)).toBeVisible();
    await expect(toast(page, 1)).toBeVisible();
    const resumed = await page.evaluate(() => {
        delete (document as unknown as { hidden?: boolean }).hidden;
        document.dispatchEvent(new Event('visibilitychange'));
        return performance.now();
    });
    await expect.poll(async () => (await latestPendingTimer(page, resumed)).length).toBe(1);
    const timers = await latestPendingTimer(page, resumed);
    expect(timers[0].delay).toBeGreaterThan(500);
    expect(timers[0].delay).toBeLessThan(1589);
    expect((await latestPendingTimer(page, started)).length).toBe(1);
    await expect(toast(page, 0)).toBeHidden({ timeout: 2500 });
    await expect(toast(page, 1)).toBeHidden({ timeout: 2500 });
    expect((await counters(page, 'pause-counts')).slice(0, 2).every(count => count > 0)).toBe(true);
    expect((await counters(page, 'resume-counts')).slice(0, 2).every(count => count > 0)).toBe(true);
});

test('F8, action cancellation, Escape cancellation, Close and natural Tab use actual focus targets', async ({ page }) => {
    await details(page, 'notice-tools');
    await id(page, 'notice-show-3').click();
    await id(page, 'action-cancel').check();
    await id(page, 'toast-action-3').click();
    await expect(id(page, 'action-requests')).toHaveText('1');
    await expect(id(page, 'action-result')).toHaveText('0');
    await expect(toast(page, 3)).toBeVisible();
    await id(page, 'escape-cancel').check();
    await id(page, 'toast-action-3').focus();
    await page.keyboard.press('Escape');
    await expect(toast(page, 3)).toBeVisible();
    expect(await counters(page, 'close-requests')).toEqual([0, 0, 0, 0]);
    await id(page, 'escape-cancel').uncheck();
    await id(page, 'spinner-draft').focus();
    await page.keyboard.type('a');
    await expect(id(page, 'spinner-draft')).toBeFocused();
    await page.keyboard.press('F8');
    await expect(id(page, 'toast-action-3')).toBeFocused();
    await page.keyboard.press('Tab');
    await expect(id(page, 'toast-close-3')).toBeFocused();
    await page.keyboard.press('Tab');
    expect(await id(page, 'toast-viewport').evaluate(node => node.contains(document.activeElement))).toBe(false);
    await id(page, 'spinner-draft').focus();
    await page.keyboard.press('F8');
    await expect(id(page, 'toast-action-3')).toBeFocused();
    await page.keyboard.press('Escape');
    await expect(toast(page, 3)).toBeHidden();
    await expect(id(page, 'spinner-draft')).toBeFocused();
    await id(page, 'notice-show-3').click();
    await id(page, 'action-cancel').uncheck();
    await id(page, 'toast-action-3').click();
    await expect(id(page, 'action-result')).toHaveText('1');
    await expect(toast(page, 3)).toBeHidden();
    await id(page, 'notice-show-3').click();
    await id(page, 'toast-close-3').click();
    await expect(toast(page, 3)).toBeHidden();
});

test('native pointer swipes respect direction, hook cancellation and pointer cancellation', async ({ page }) => {
    await details(page, 'notice-tools');
    await id(page, 'notice-show-3').click();
    await swipe(page, 3, -85);
    await expect(toast(page, 3)).toBeVisible();
    expect(await toast(page, 3).evaluate(node => { const value = getComputedStyle(node).translate; return value === 'none' || value.split(/\s+/).every(part => parseFloat(part) === 0); })).toBe(true);
    await id(page, 'swipe-cancel').selectOption('end');
    await swipe(page, 3, 85);
    await expect(toast(page, 3)).toBeVisible();
    await expect(id(page, 'swipe-events')).toContainText('end');
    const session = await page.context().newCDPSession(page);
    const box = (await id(page, 'toast-title-3').boundingBox())!;
    const point = { x: box.x + 24, y: box.y + box.height / 2 };
    await session.send('Input.dispatchTouchEvent', { type: 'touchStart', touchPoints: [{ ...point, id: 7 }] });
    await session.send('Input.dispatchTouchEvent', { type: 'touchMove', touchPoints: [{ x: point.x + 25, y: point.y, id: 7 }] });
    await session.send('Input.dispatchTouchEvent', { type: 'touchCancel', touchPoints: [] });
    await expect(id(page, 'swipe-events')).toContainText('cancel');
    await expect(toast(page, 3)).toBeVisible();
    await id(page, 'swipe-cancel').selectOption('');
    await swipe(page, 3, 85);
    await expect(toast(page, 3)).toBeHidden();
    expect((await counters(page, 'close-requests'))[3]).toBe(1);
    await session.detach();
});

test('localized status, viewport and dismiss labels stay separate from two announcement channels', async ({ page }) => {
    await details(page, 'tools');
    await id(page, 'locale').click();
    await id(page, 'loading-start').click();
    await expect(id(page, 'spinner-frame').getByRole('status')).toHaveAccessibleName('加载中…');
    await id(page, 'notice-show-3').click();
    const viewport = id(page, 'toast-viewport');
    await expect(viewport).toHaveAccessibleName('通知');
    await expect(id(page, 'toast-close-3')).toHaveAccessibleName('关闭这条消息');
    await liveShape(viewport);
    const polite = viewport.locator('[data-rustify-toast-announcer="polite"] > span');
    await expect(polite).toHaveCount(1);
    await expect(polite).toContainText('Pinned delivery reminder');
    await id(page, 'toast-action-3').focus();
    await page.keyboard.press('F8');
    await expect(polite).toHaveCount(1);
    // DOM channel structure is checked here; this does not claim a VoiceOver reading result.
});

async function openMigration(page: Page) {
    await id(page, 'mode-layers').click();
    await id(page, 'layer-show').click();
    await expect(id(page, 'layer-toast')).toBeVisible();
}
async function openReviewAndSelect(page: Page) {
    await id(page, 'review-open').click();
    await expect(id(page, 'review-popup')).toBeVisible();
    await id(page, 'layer-select').click();
    await expect(id(page, 'layer-options')).toBeVisible();
    await expect(id(page, 'layer-select')).toBeFocused();
}

test('a saved notice enters a modal before opening Popover and Select, then closes in descendant order', async ({ page }) => {
    await openMigration(page);
    await remember(page, ['layer-toast', 'layer-viewport']);
    await id(page, 'host-open').click();
    await expect(id(page, 'host-dialog')).toBeVisible();
    await expect.poll(() => id(page, 'layer-viewport').evaluate(node => !!node.closest('[data-testid="feedback-host-dialog"]'))).toBe(true);
    await openReviewAndSelect(page);
    expect(await id(page, 'layer-options').evaluate(node => !!node.closest('[data-testid="feedback-review-popup"]'))).toBe(true);
    await page.keyboard.press('Escape');
    await expect(id(page, 'layer-options')).toHaveCount(0);
    await expect(id(page, 'review-popup')).toBeVisible();
    await page.keyboard.press('Escape');
    await expect(id(page, 'review-popup')).toHaveCount(0);
    await expect(id(page, 'review-open')).toBeFocused();
    await expect(id(page, 'host-dialog')).toBeVisible();
    await page.keyboard.press('Escape');
    await expect(id(page, 'layer-toast')).toBeHidden();
    await expect(id(page, 'host-dialog')).toBeVisible();
    await id(page, 'host-close').click();
    await expect(id(page, 'host-dialog')).toHaveCount(0);
    await sameNodes(page, ['layer-toast', 'layer-viewport']);
});

test('a focused notification descendant pauses only its own Provider beside the existing benchmark Toaster', async ({ page }, info) => {
    test.skip(info.project.name === 'dom-primitives', 'The benchmark Toaster is an existing styled consumer composition.');
    await id(page, 'mode-layers').click();
    await details(page, 'migration-tools');
    await id(page, 'layer-persistent').uncheck();
    const started = await page.evaluate(() => performance.now());
    await id(page, 'layer-show').click();
    await expect(id(page, 'layer-toast')).toBeVisible();
    await leaveNotice(page);
    await page.getByTestId('bench-toast-show').click();
    const other = page.getByTestId('bench-toaster-toast');
    await expect(other).toHaveText(/Saved 1/);
    // Keyboard activation avoids overlapping fixed viewports affecting the ownership check.
    await id(page, 'review-open').focus();
    await page.keyboard.press('Enter');
    await expect(id(page, 'review-popup')).toBeVisible();
    await id(page, 'layer-note').focus();
    await expect(id(page, 'layer-note')).toBeFocused();
    await page.mouse.move(2, 2);
    await expect(other).toHaveCount(0, { timeout: 2500 });
    await expect(id(page, 'layer-toast')).toBeVisible();
    await expect(id(page, 'review-popup')).toBeVisible();
    await expect(id(page, 'layer-note')).toBeFocused();
    await expect.poll(async () => (await latestPendingTimer(page, started)).length).toBe(0);
    await expect(id(page, 'layer-close-requests')).toHaveText('0');
});

test('existing notification Popover and Select keep nodes, focus, state and announcements across host migration', async ({ page }) => {
    await openMigration(page);
    await id(page, 'review-open').click();
    await id(page, 'layer-note').fill('Retained saved-item note');
    await id(page, 'layer-select').click();
    await expect(id(page, 'layer-options')).toBeVisible();
    const names = ['layer-toast', 'layer-viewport', 'review-popup', 'layer-options', 'layer-note', 'layer-select'];
    await remember(page, names);
    const counts = await counters(page, 'layer-focus-counts');
    const announcer = id(page, 'layer-viewport').locator('[data-rustify-toast-announcer="polite"] > span');
    await expect(announcer).toHaveCount(1);
    await id(page, 'layer-select').press('F2');
    await expect(id(page, 'host-dialog')).toBeVisible();
    await expect(id(page, 'layer-select')).toBeFocused();
    await sameNodes(page, names);
    expect((await counters(page, 'layer-focus-counts')).slice(0, 4)).toEqual(counts.slice(0, 4));
    await expect(id(page, 'layer-note')).toHaveValue('Retained saved-item note');
    await expect(announcer).toHaveCount(1);
    await id(page, 'layer-select').press('F2');
    await expect(id(page, 'host-dialog')).toHaveCount(0);
    await expect(id(page, 'layer-select')).toBeFocused();
    await sameNodes(page, names);
    expect((await counters(page, 'layer-focus-counts')).slice(0, 4)).toEqual(counts.slice(0, 4));
    await expect(id(page, 'layer-close-requests')).toHaveText('0');
    await expect(id(page, 'layer-options')).toBeVisible();
    await page.keyboard.press('Escape');
    await id(page, 'review-close').click();
    await expect(id(page, 'review-popup')).toHaveCount(0);
    await expect(id(page, 'review-open')).toBeFocused();
});

test('a notice opening its own modal and a modal inside its existing Popover do not create a DOM cycle', async ({ page }) => {
    await openMigration(page);
    await remember(page, ['layer-toast', 'layer-viewport']);
    await id(page, 'toast-dialog-open').click();
    await expect(id(page, 'toast-dialog')).toBeVisible();
    await expect.poll(() => id(page, 'layer-viewport').evaluate(node => !!node.closest('[data-testid="feedback-toast-dialog"]'))).toBe(true);
    await id(page, 'toast-dialog-close').click();
    await expect(id(page, 'toast-dialog-open')).toBeFocused();
    await sameNodes(page, ['layer-toast', 'layer-viewport']);
    await id(page, 'review-open').click();
    await remember(page, ['review-popup']);
    await id(page, 'nested-dialog-open').click();
    await expect(id(page, 'nested-dialog')).toBeVisible();
    expect(await page.evaluate(() => {
        const modal = document.querySelector('[data-testid="feedback-nested-dialog"]')!;
        const viewport = document.querySelector('[data-testid="feedback-layer-viewport"]')!;
        const popup = document.querySelector('[data-testid="feedback-review-popup"]')!;
        return modal.contains(viewport) && !viewport.contains(modal) && popup.contains(modal) && !modal.contains(popup);
    })).toBe(true);
    await id(page, 'nested-dialog-close').click();
    await expect(id(page, 'nested-dialog-open')).toBeFocused();
    await sameNodes(page, ['layer-toast', 'layer-viewport', 'review-popup']);
    await expect(id(page, 'layer-close-requests')).toHaveText('0');
});

test('native number Draft and a composing keyboard event outrank notification descendants', async ({ page }) => {
    await openMigration(page);
    await id(page, 'review-open').click();
    const input = id(page, 'layer-estimate');
    await input.fill('12.');
    await page.keyboard.press('Escape');
    await expect(input).toHaveValue('3');
    await expect(id(page, 'draft-cancels')).toHaveText('1');
    await expect(id(page, 'review-popup')).toBeVisible();
    await expect(id(page, 'layer-toast')).toBeVisible();
    await id(page, 'layer-note').focus();
    await id(page, 'layer-note').dispatchEvent('keydown', { key: 'Escape', code: 'Escape', bubbles: true, isComposing: true });
    await expect(id(page, 'review-popup')).toBeVisible();
    await id(page, 'review-close').focus();
    await page.keyboard.press('Escape');
    await expect(id(page, 'review-popup')).toHaveCount(0);
    await expect(id(page, 'layer-toast')).toBeVisible();
    // The DOM composing flag is exercised; physical IME completion still needs manual evidence.
});

test('migration retains the remaining duration rather than restarting a timed notice', async ({ page }) => {
    await id(page, 'mode-layers').click();
    await details(page, 'migration-tools');
    await id(page, 'layer-persistent').uncheck();
    const epoch = await page.evaluate(() => performance.now());
    await id(page, 'layer-show').click();
    await leaveNotice(page);
    await page.waitForTimeout(1200);
    await id(page, 'review-open').click();
    await id(page, 'layer-note').fill('Timed notice with retained work');
    await remember(page, ['layer-toast', 'review-popup', 'layer-note']);
    await id(page, 'layer-note').press('F2');
    await expect(id(page, 'host-dialog')).toBeVisible();
    await page.waitForTimeout(700);
    await id(page, 'layer-note').press('F2');
    await expect(id(page, 'host-dialog')).toHaveCount(0);
    await sameNodes(page, ['layer-toast', 'review-popup', 'layer-note']);
    const resumed = await page.evaluate(() => performance.now());
    await id(page, 'review-close').click();
    await leaveNotice(page);
    await expect.poll(async () => (await latestPendingTimer(page, resumed)).length).toBe(1);
    const pending = await latestPendingTimer(page, resumed);
    expect(pending[0].delay).toBeGreaterThan(2000);
    expect(pending[0].delay).toBeLessThan(5400);
    expect(await page.evaluate(epoch => Array.from(window.__feedbackProbe.timers.values()).some(timer => timer.at >= epoch && timer.delay > 5500 && timer.delay <= 6000), epoch)).toBe(true);
    await expect(id(page, 'layer-toast')).toBeHidden({ timeout: 6500 });
    await expect(id(page, 'layer-close-requests')).toHaveText('1');
});

test('provider replacement releases timers, swipe geometry and overlays after repeated use', async ({ page }) => {
    await leaveNotice(page);
    const before = await stats(page);
    for (let round = 0; round < 4; round++) {
        await id(page, 'notice-show-0').click();
        await id(page, 'notice-show-3').click();
        await swipe(page, 3, 25);
        await expect(toast(page, 3)).toBeVisible();
        // Replace the Provider while its timed and swipe-enabled roots are still open.
        await id(page, 'mode-replace').click();
        await id(page, 'mode-queue').click();
    }
    await leaveNotice(page);
    await expect.poll(async () => (await stats(page)).observers).toBe(before.observers);
    await expect.poll(async () => (await stats(page)).rules).toBe(before.rules);
    expect((await stats(page)).sheets).toBe(before.sheets);
    expect((await stats(page)).nodes).toBe(before.nodes);
    await expect.poll(async () => (await stats(page)).tasks).toBe(before.tasks);
    await expect.poll(async () => (await stats(page)).timers).toBe(before.timers);
    expect(await latestPendingTimer(page, 0)).toEqual([]);
});

for (const width of [390, 768, 1440]) test(`feedback remains usable at ${width}px and captures both theme appearances`, async ({ page }, info) => {
    await page.setViewportSize({ width, height: 900 });
    await id(page, 'samples').scrollIntoViewIfNeeded();
    await id(page, 'notice-show-3').click();
    await expect(toast(page, 3)).toBeVisible();
    const root = (await toast(page, 3).boundingBox())!;
    expect(root.x).toBeGreaterThanOrEqual(0);
    expect(root.x + root.width).toBeLessThanOrEqual(width);
    await details(page, 'tools');
    await expect(toast(page, 3)).not.toHaveCSS('background-color', 'rgba(0, 0, 0, 0)');
    if (info.project.name === 'dom-only') {
        const background = await id(page, 'theme-boundary').evaluate(node => getComputedStyle(node).backgroundColor);
        await expect(toast(page, 3)).toHaveCSS('background-color', background);
    }
    await page.screenshot({ path: `docs/validation/dom-components-p3/m7-feedback-${info.project.name}-${width}-light.png`, fullPage: false });
    await id(page, 'theme').click();
    await expect(id(page, 'theme-boundary')).toHaveAttribute('data-theme', 'dark');
    await expect(toast(page, 3)).not.toHaveCSS('background-color', 'rgba(0, 0, 0, 0)');
    if (info.project.name === 'dom-only') {
        const background = await id(page, 'theme-boundary').evaluate(node => getComputedStyle(node).backgroundColor);
        await expect(toast(page, 3)).toHaveCSS('background-color', background);
    }
    await page.screenshot({ path: `docs/validation/dom-components-p3/m7-feedback-${info.project.name}-${width}-dark.png`, fullPage: false });
    // This is viewport resizing. Browser UI zoom, OS touch and assistive-technology speech are separate manual checks.
});

for (const [direction, dx, dy] of [['right', 85, 0], ['left', -85, 0], ['up', 0, -85], ['down', 0, 85]] as const) {
    test(`native ${direction} swipe consumes only the configured direction`, async ({ page }) => {
        await details(page, 'swipe-config');
        await id(page, 'swipe-direction').selectOption(direction);
        await id(page, 'notice-show-3').click();
        await expect(toast(page, 3)).toHaveAttribute('data-swipe-direction', direction);
        await swipe(page, 3, -dx, -dy);
        await expect(toast(page, 3)).toBeVisible();
        await swipe(page, 3, dx, dy);
        await expect(toast(page, 3)).toBeHidden();
        expect((await counters(page, 'close-requests'))[3]).toBe(1);
    });
}

test('notice descendants accept a real status choice and preserve outside focus when their layers close', async ({ page }) => {
    await openMigration(page);
    await id(page, 'host-open').click();
    await openReviewAndSelect(page);
    await id(page, 'layer-ready').click();
    await expect(id(page, 'layer-status')).toHaveText('ready');
    await expect(id(page, 'layer-options')).toHaveCount(0);
    await id(page, 'layer-select').click();
    await id(page, 'layer-note').click();
    await expect(id(page, 'layer-options')).toHaveCount(0);
    await expect(id(page, 'review-popup')).toBeVisible();
    await id(page, 'layer-select').click();
    await page.keyboard.press('Tab');
    await expect(id(page, 'layer-options')).toHaveCount(0);
    await expect(id(page, 'review-popup')).toBeVisible();
    await expect(id(page, 'nested-dialog-open')).toBeFocused();
    await id(page, 'host-reference').click();
    await expect(id(page, 'review-popup')).toHaveCount(0);
    await expect(id(page, 'host-reference')).toBeFocused();
    await expect(id(page, 'host-dialog')).toBeVisible();
    await expect(id(page, 'layer-toast')).toBeVisible();
});

test('browser UI zoom reaches 200 percent when the runner exposes browser accelerators', async ({ page }, info) => {
    await page.setViewportSize({ width: 1440, height: 900 });
    const modifier = process.platform === 'darwin' ? 'Meta' : 'Control';
    await page.keyboard.press(`${modifier}+Digit0`);
    const before = await page.evaluate(() => ({ dpr: devicePixelRatio, width: innerWidth }));
    for (let step = 0; step < 5; step++) await page.keyboard.press(`${modifier}+Equal`);
    const after = await page.evaluate(() => ({ dpr: devicePixelRatio, width: innerWidth }));
    const doubled = Math.abs(after.dpr / before.dpr - 2) < .05 && Math.abs(after.width / before.width - .5) < .05;
    if (!doubled) {
        info.annotations.push({ type: 'missing', description: 'The runner did not expose real browser UI zoom. Verify 200% manually; viewport resizing and CSS zoom do not replace this check.' });
        test.skip(true, 'Real browser UI zoom was unavailable; retained as a manual missing item.');
    }
    await id(page, 'samples').scrollIntoViewIfNeeded();
    await id(page, 'notice-show-3').click();
    await expect(toast(page, 3)).toBeVisible();
    const bounds = (await toast(page, 3).boundingBox())!;
    expect(bounds.x).toBeGreaterThanOrEqual(0);
    expect(bounds.x + bounds.width).toBeLessThanOrEqual(after.width);
    await expect(id(page, 'toast-close-3')).toBeInViewport();
    await page.screenshot({ path: info.outputPath('feedback-browser-ui-zoom-200.png') });
    await page.keyboard.press(`${modifier}+Digit0`);
});
