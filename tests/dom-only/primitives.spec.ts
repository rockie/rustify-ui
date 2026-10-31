import { test, expect } from '@playwright/test';

const browserFailures = new WeakMap<import('@playwright/test').Page, string[]>();
test.beforeEach(({ page }) => {
    const failures: string[] = [];
    browserFailures.set(page, failures);
    page.on('pageerror', error => failures.push(error.message));
    page.on('console', message => { if (message.type() === 'error') failures.push(message.text()); });
    page.on('requestfailed', request => failures.push(`${request.url()}: ${request.failure()?.errorText}`));
});
test.afterEach(({ page }) => expect(browserFailures.get(page)).toEqual([]));


// This project serves the independent product, never the skinned consumer.
test('loads only structural styles and preserves a single native target', async ({ page }) => {
    const errors: string[] = [];
    const requests: string[] = [];
    page.on('pageerror', error => errors.push(error.message));
    page.on('console', message => { if (message.type() === 'error') errors.push(message.text()); });
    page.on('request', request => requests.push(request.url()));
    page.on('requestfailed', request => errors.push(request.url()));
    await page.goto('/');
    await expect(page.locator('#status')).toHaveAttribute('data-status', 'ready');
    expect(requests.some(url => /\/(rustify|sdk|theme-v4)\.css/.test(url))).toBe(false);
    const css = await (await page.request.get('/primitives.css')).text();
    expect(css).not.toMatch(/--(?:primary|accent|rustify-space)|font-family|box-shadow|:root/);
    const target = page.getByTestId('cancel-target');
    await expect(target).toHaveAttribute('data-rustify-part', 'trigger');
    expect(await target.evaluate(node => node.tagName)).toBe('BUTTON');
    await target.click();
    await expect(page.getByTestId('child-calls')).toHaveText('1');
    await expect(page.getByTestId('requests')).toHaveText('0');
    await expect(page.getByRole('img', { name: 'Status' })).toHaveCount(1);
    const hidden = page.getByTestId('hidden-text');
    expect(await hidden.evaluate(node => getComputedStyle(node.parentElement!).clipPath)).toBe('inset(50%)');
    expect(errors).toEqual([]);
});

test('portal retains local direction, has no theme tokens and cleans up its wrapper', async ({ page }) => {
    const errors: string[] = [];
    page.on('pageerror', error => errors.push(error.message));
    page.on('console', message => { if (message.type() === 'error') errors.push(message.text()); });
    page.on('requestfailed', request => errors.push(request.url()));
    await page.goto('/');
    await expect(page.locator('#status')).toHaveAttribute('data-status', 'ready');
    const root = page.locator('[data-rustify-overlay]');
    const before = await root.evaluate(node => node.childElementCount);
    const idle = await page.evaluate(() => (window as any).__primitives.stats());
    for (let round = 0; round < 3; round++) {
        await page.getByTestId('portal-toggle').click();
        const portal = page.locator('[data-rustify-part="portal"]');
        await expect(portal).toHaveAttribute('dir', 'rtl');
        await expect(root.getByTestId('portal-content')).toBeVisible();
        expect(await portal.evaluate(node => getComputedStyle(node).getPropertyValue('--primary'))).toBe('');
        await page.getByTestId('direction-toggle').click();
        await expect(portal).toHaveAttribute('dir', 'ltr');
        await page.getByTestId('direction-toggle').click();
        await page.getByTestId('portal-toggle').click();
        await expect(portal).toHaveCount(0);
        expect(await root.evaluate(node => node.childElementCount)).toBe(before);
    }
    expect(await page.evaluate(() => (window as any).__primitives.stats())).toEqual(idle);
    await page.evaluate(() => (window as any).__primitives.dispose());
    await expect(page.getByTestId('portal-content')).toHaveCount(0);
    expect(errors).toEqual([]);
});

const choiceValue = (page: import('@playwright/test').Page, id: string) => page.getByTestId(id).locator('[data-value]');
const choiceRequests = (page: import('@playwright/test').Page, id: string) => page.getByTestId(id).locator('[data-requests]');

test('choice requests remain separate from accepted native state and FormData', async ({ page }) => {
    const errors: string[] = [];
    page.on('pageerror', error => errors.push(error.message));
    await page.goto('/');
    await expect(page.locator('#status')).toHaveAttribute('data-status', 'ready');
    const mixed = page.getByTestId('choice-mixed');
    await expect(mixed).toHaveAttribute('aria-checked', 'mixed');
    expect(await mixed.evaluate(node => (node as HTMLInputElement).indeterminate)).toBe(true);
    await mixed.click();
    await expect(mixed).toBeChecked();
    expect(await mixed.evaluate(node => (node as HTMLInputElement).indeterminate)).toBe(false);
    await page.getByTestId('choice-policy').selectOption('refuse');
    await mixed.click();
    await expect(mixed).toBeChecked();
    await expect(choiceRequests(page, 'choice-mixed-state')).toHaveText('2');
    await page.getByTestId('choice-one-b').click();
    await expect(page.getByTestId('choice-one-a')).toBeChecked();
    await expect(page.getByTestId('choice-one-b')).not.toBeChecked();
    await expect(page.getByTestId('choice-two-a')).toBeChecked();
    await page.getByTestId('choice-peer-b').click();
    await expect(page.getByTestId('choice-peer-a')).toBeChecked();
    await expect(page.getByTestId('choice-peer-b')).not.toBeChecked();
    await page.getByTestId('choice-switch').click();
    await expect(page.getByTestId('choice-switch')).not.toBeChecked();
    await page.getByTestId('choice-policy').selectOption('delay');
    await page.getByTestId('choice-one-b').click();
    await expect(page.getByTestId('choice-one-a')).toBeChecked();
    await page.getByTestId('choice-apply').click();
    await expect(page.getByTestId('choice-one-b')).toBeChecked();
    await expect(page.getByTestId('choice-two-a')).toBeChecked();
    const entries = await page.evaluate(() => [...new FormData(document.getElementById('choices-form') as HTMLFormElement).entries()]);
    expect(entries).toEqual([['consent', 'yes'], ['peer', 'a'], ['features', 'b'], ['format', 'bold']]);
    expect(errors).toEqual([]);
});

test('choice readonly moves radio focus without requesting and reordered multi-values follow declaration', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('#status')).toHaveAttribute('data-status', 'ready');
    await page.getByTestId('choice-readonly').click();
    await page.getByTestId('choice-one-a').focus();
    await page.keyboard.press('ArrowRight');
    await expect(page.getByTestId('choice-one-b')).toBeFocused();
    await expect(page.getByTestId('choice-one-a')).toBeChecked();
    await expect(choiceRequests(page, 'choice-one-state')).toHaveText('0');
    await page.getByTestId('choice-readonly').click();
    await page.getByTestId('choice-one-b').click();
    await page.evaluate(() => (document.querySelector('[data-testid="choice-remove-b"]') as HTMLButtonElement).click());
    await expect(page.getByTestId('choice-one-c')).toBeFocused();
    await expect(choiceValue(page, 'choice-one-state')).toHaveText('Some("b")');
    await page.getByTestId('choice-reverse').click();
    await page.getByTestId('choice-check-a').click();
    await expect(choiceValue(page, 'choice-checks-state')).toHaveText('["c", "b", "a"]');
    await page.getByTestId('choice-disabled').click();
    await expect(page.getByTestId('choice-mixed')).toBeDisabled();
    await expect(page.getByTestId('choice-disabled')).toBeFocused();
    expect(await page.evaluate(() => [...new FormData(document.getElementById('choices-form') as HTMLFormElement).entries()])).toEqual([]);
});

test('choice Slot target cancellation retains caller attributes and does not request', async ({ page }) => {
    const errors: string[] = [];
    page.on('pageerror', error => errors.push(error.message));
    await page.goto('/');
    await expect(page.locator('#status')).toHaveAttribute('data-status', 'ready');
    await page.getByTestId('choice-cancel').click();
    await expect(page.getByTestId('choice-cancel')).not.toBeChecked();
    expect(errors).toEqual([]);
});

test('choice structural observers disconnect on scope disposal', async ({ page }) => {
    await page.addInitScript(() => {
        const original = window.MutationObserver;
        const active = new Set<MutationObserver>();
        window.MutationObserver = class extends original {
            observe(target: Node, options?: MutationObserverInit) { super.observe(target, options); active.add(this); }
            disconnect() { active.delete(this); super.disconnect(); }
        };
        (window as any).__choiceObservers = () => active.size;
    });
    await page.goto('/');
    await expect(page.locator('#status')).toHaveAttribute('data-status', 'ready');
    expect(await page.evaluate(() => (window as any).__choiceObservers())).toBeGreaterThan(0);
    await page.evaluate(() => (window as any).__primitives.dispose());
    await expect.poll(() => page.evaluate(() => (window as any).__choiceObservers())).toBe(0);
});

async function sliderForm(page: import('@playwright/test').Page) {
    return page.locator('#slider-form').evaluate(form => [...new FormData(form as HTMLFormElement).entries()]);
}

test('slider refuses previews, commits the final keyboard request once and accepts delayed values', async ({ page }) => {
    await page.goto('/');
    const left = page.getByTestId('slider-left');
    await page.getByTestId('slider-policy').selectOption('refuse');
    await left.focus();
    await page.keyboard.down('ArrowRight');
    await page.keyboard.down('ArrowRight');
    await page.keyboard.down('ArrowRight');
    await expect(left).toHaveAttribute('aria-valuenow', '20');
    await expect(page.getByTestId('slider-requests')).toHaveText('3');
    await page.keyboard.up('ArrowRight');
    await expect(page.getByTestId('slider-commits')).toHaveText('1');
    await expect(page.getByTestId('slider-committed')).toHaveText('left=23,right=80');
    expect(await sliderForm(page)).toEqual([['range', '20'], ['range', '80']]);
    await page.getByTestId('slider-policy').selectOption('delay');
    await left.press('PageUp');
    await expect(left).toHaveAttribute('aria-valuenow', '20');
    await expect(page.getByTestId('slider-committed')).toHaveText('left=30,right=80');
    await page.getByTestId('slider-apply').click();
    await expect(left).toHaveAttribute('aria-valuenow', '30');
    expect(await sliderForm(page)).toEqual([['range', '30'], ['range', '80']]);
});

test('slider crossing preserves DOM focus identity while form values sort numerically', async ({ page }) => {
    await page.goto('/');
    const left = page.getByTestId('slider-left');
    await left.press('End');
    await expect(left).toBeFocused();
    await expect(left).toHaveAttribute('aria-valuenow', '100');
    const order = await page.getByTestId('slider-group').getByRole('slider').evaluateAll(thumbs => thumbs.map(thumb => thumb.getAttribute('data-testid')));
    expect(order).toEqual(['slider-left', 'slider-right']);
    expect(await sliderForm(page)).toEqual([['range', '80'], ['range', '100']]);
    await left.press('Home');
    await page.getByTestId('slider-preserve').click();
    await left.press('End');
    await expect(left).toHaveAttribute('aria-valuenow', '75');
    await expect(left).toHaveAttribute('aria-valuemax', '75');
    await page.getByTestId('slider-rtl').click();
    await left.press('ArrowRight');
    await expect(left).toHaveAttribute('aria-valuenow', '74');
    await page.getByTestId('slider-readonly').click();
    await left.press('ArrowLeft');
    await expect(left).toHaveAttribute('aria-valuenow', '74');
});

test('slider pointer capture requests accepted geometry and Escape cancels without committing', async ({ page }) => {
    await page.goto('/');
    const group = page.getByTestId('slider-group');
    const track = group.locator('[data-rustify-part="slider-track"]');
    const box = (await track.boundingBox())!;
    const left = page.getByTestId('slider-left');
    await left.scrollIntoViewIfNeeded();
    const actual = (await track.boundingBox())!;
    const y = actual.y + actual.height / 2;
    await page.mouse.move(actual.x + actual.width * .2, y);
    await page.mouse.down();
    await page.mouse.move(actual.x + actual.width * .45, y);
    await expect(left).toHaveAttribute('aria-valuenow', '45');
    await page.keyboard.press('Escape');
    await expect(page.getByTestId('slider-cancels')).toHaveText('1');
    await expect(page.getByTestId('slider-commits')).toHaveText('0');
    await expect(left).toHaveAttribute('aria-valuenow', '20');
    await page.mouse.up();
    await expect(page.getByTestId('slider-commits')).toHaveText('0');
    expect(box.width).toBeGreaterThan(0);
});

test('slider configuration errors retain the last valid display and recover when repaired', async ({ page }) => {
    await page.goto('/');
    const left = page.getByTestId('slider-left');
    await page.getByTestId('slider-invalid-step').click();
    await expect(page.getByTestId('slider-group')).toHaveAttribute('data-rustify-issue', 'invalid-config');
    await expect(left).toHaveAttribute('aria-valuenow', '20');
    await expect(left).toBeDisabled();
    expect(await sliderForm(page)).toEqual([]);
    await page.getByTestId('slider-invalid-step').click();
    await expect(left).toBeEnabled();
    await left.press('ArrowRight');
    await expect(left).toHaveAttribute('aria-valuenow', '21');
});

test('slider returning to a refused starting value does not commit a stale pointer request', async ({ page }) => {
    await page.goto('/');
    await page.getByTestId('slider-policy').selectOption('refuse');
    const left = page.getByTestId('slider-left');
    await left.scrollIntoViewIfNeeded();
    const track = (await page.getByTestId('slider-group').locator('[data-rustify-part="slider-track"]').boundingBox())!;
    const y = track.y + track.height / 2;
    await page.mouse.move(track.x + track.width * .2, y);
    await page.mouse.down();
    await page.mouse.move(track.x + track.width * .6, y);
    await expect(page.getByTestId('slider-last')).toHaveText('left=60,right=80');
    await page.mouse.move(track.x + track.width * .2, y);
    await page.mouse.up();
    await expect(page.getByTestId('slider-last')).toHaveText('left=20,right=80');
    await expect(page.getByTestId('slider-commits')).toHaveText('0');
    await expect(left).toHaveAttribute('aria-valuenow', '20');
});

test('slider disabling or removing the active thumb cancels its session before blur can commit', async ({ page }) => {
    await page.goto('/');
    const left = page.getByTestId('slider-left');
    await left.focus();
    await page.keyboard.down('ArrowRight');
    await expect(left).toHaveAttribute('aria-valuenow', '21');
    await page.getByTestId('slider-disable-left').dispatchEvent('click');
    await expect(page.getByTestId('slider-cancels')).toHaveText('1');
    await expect(page.getByTestId('slider-commits')).toHaveText('0');
    await page.keyboard.up('ArrowRight');
    await page.getByTestId('slider-disable-left').click();
    await left.focus();
    await page.keyboard.down('ArrowRight');
    await page.getByTestId('slider-remove-left').dispatchEvent('click');
    await expect(left).toHaveCount(0);
    await expect(page.getByTestId('slider-cancels')).toHaveText('2');
    await expect(page.getByTestId('slider-commits')).toHaveText('0');
    await page.keyboard.up('ArrowRight');
    expect(await sliderForm(page)).toEqual([['range', '80']]);
});

test('slider scope disposal releases a live native pointer capture and geometry resources', async ({ page }) => {
    await page.goto('/');
    const left = page.getByTestId('slider-left');
    await left.scrollIntoViewIfNeeded();
    await page.evaluate(() => {
        document.addEventListener('pointerdown', event => {
            (window as any).__sliderCapture = { element: event.target, pointer: event.pointerId };
        }, { once: true });
    });
    const box = (await left.boundingBox())!;
    await page.mouse.move(box.x + box.width / 2, box.y + box.height / 2);
    await page.mouse.down();
    expect(await page.evaluate(() => { const { element, pointer } = (window as any).__sliderCapture; return element.hasPointerCapture(pointer); })).toBe(true);
    await page.evaluate(() => (window as any).__primitives.dispose());
    expect(await page.evaluate(() => { const { element, pointer } = (window as any).__sliderCapture; return element.hasPointerCapture(pointer); })).toBe(false);
    await page.mouse.up();
    await expect(page.locator('[data-rustify-part="slider-root"]')).toHaveCount(0);
    await expect.poll(() => page.evaluate(() => (window as any).__primitives.stats())).toMatchObject({ tasks: 0, timers: 0, observers: 0, errors: 0 });
});

test('slider caches the track rectangle between moves and invalidates it on viewport resize', async ({ page }) => {
    await page.goto('/');
    const left = page.getByTestId('slider-left');
    await left.scrollIntoViewIfNeeded();
    const track = page.getByTestId('slider-group').locator('[data-rustify-part="slider-track"]');
    const box = (await track.boundingBox())!;
    await track.evaluate(node => {
        const original = node.getBoundingClientRect.bind(node);
        (window as any).__sliderRectReads = 0;
        node.getBoundingClientRect = () => { (window as any).__sliderRectReads++; return original(); };
    });
    const y = box.y + box.height / 2;
    await page.mouse.move(box.x + box.width * .2, y);
    await page.mouse.down();
    await page.evaluate(() => new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve))));
    await page.mouse.move(box.x + box.width * .3, y);
    const reads = await page.evaluate(() => (window as any).__sliderRectReads);
    await page.mouse.move(box.x + box.width * .4, y, { steps: 5 });
    expect(await page.evaluate(() => (window as any).__sliderRectReads)).toBe(reads);
    const viewport = page.viewportSize()!;
    const resized = page.evaluate(() => new Promise<void>(resolve => window.addEventListener('resize', () => resolve(), { once: true })));
    await page.setViewportSize({ ...viewport, width: viewport.width + 10 });
    await resized;
    await page.mouse.move(box.x + box.width * .5, y);
    expect(await page.evaluate(() => (window as any).__sliderRectReads)).toBeGreaterThan(reads);
    await page.mouse.up();
});

test('slider vertical direction and inversion share normalized keyboard requests', async ({ page }) => {
    await page.goto('/');
    const vertical = page.getByTestId('slider-vertical-thumb');
    await vertical.press('ArrowUp');
    await expect(page.getByTestId('slider-vertical-last')).toHaveText('vertical=36');
    await expect(vertical).toHaveAttribute('aria-valuenow', '35');
    await expect(vertical).toHaveAttribute('aria-orientation', 'vertical');
    await vertical.press('PageDown');
    await expect(page.getByTestId('slider-vertical-last')).toHaveText('vertical=25');
    await page.getByTestId('slider-inverted-thumb').press('ArrowUp');
    await expect(page.getByTestId('slider-vertical-last')).toHaveText('vertical=34');
});

const nativeValues = (page: import('@playwright/test').Page) => page.getByTestId('native-form').evaluate(form => Object.fromEntries(new FormData(form as HTMLFormElement)));
async function fieldsReady(page: import('@playwright/test').Page) {
    await page.goto('/');
    await expect(page.locator('#status')).toHaveAttribute('data-status', 'ready');
}

test('field controlled edits preserve caller cancellation, native attributes and delayed acceptance', async ({ page }) => {
    await fieldsReady(page);
    const input = page.locator('input[data-testid="field-text"]');
    await expect(input).toHaveAttribute('autocomplete', 'nickname');
    expect(await input.evaluate(node => (node as HTMLInputElement).labels?.[0]?.textContent)).toBe('Title');
    await page.getByTestId('field-policy').selectOption('refuse');
    await input.fill('declined');
    await expect(input).toHaveValue('accepted');
    await expect(page.getByTestId('field-requests')).toHaveText('text:declined');
    await input.evaluate(node => {
        (node as HTMLInputElement).value = 'cancel';
        node.dispatchEvent(new InputEvent('input', { bubbles: true, cancelable: true }));
    });
    await expect(input).toHaveValue('accepted');
    await expect(page.getByTestId('field-requests')).toHaveText('text:declined');
    await page.getByTestId('field-policy').selectOption('delay');
    await input.fill('later');
    await expect(input).toHaveValue('accepted');
    expect((await nativeValues(page)).title).toBe('accepted');
    await page.getByTestId('field-apply').click();
    await expect(input).toHaveValue('later');
    expect((await nativeValues(page)).title).toBe('later');
    await page.getByTestId('field-policy').selectOption('accept');
    await input.fill('hello');
    await input.evaluate(node => {
        const input = node as HTMLInputElement;
        input.setSelectionRange(2, 2);
        input.dispatchEvent(new InputEvent('input', { bubbles: true, data: null }));
    });
    expect(await input.evaluate(node => (node as HTMLInputElement).selectionStart)).toBe(2);
    const area = page.locator('textarea[data-testid="field-area"]');
    await area.fill('first\nsecond');
    await expect(area).toHaveValue('first\nsecond');
    expect((await nativeValues(page)).notes).toBe('first\nsecond');
});

test('field IME keeps composition private and delivers one final request followed by real edits', async ({ page }) => {
    await fieldsReady(page);
    const input = page.locator('input[data-testid="field-text"]');
    await input.focus();
    await input.evaluate(node => {
        const input = node as HTMLInputElement;
        input.dispatchEvent(new CompositionEvent('compositionstart', { bubbles: true }));
        input.value = 'zhong';
        input.dispatchEvent(new InputEvent('input', { bubbles: true, isComposing: true }));
    });
    await expect(input).toHaveValue('zhong');
    await expect(page.getByTestId('field-requests')).toHaveText('');
    await input.press('Enter');
    await input.press('Escape');
    await expect(page.getByTestId('field-saves')).toHaveText('0');
    await input.evaluate(node => {
        const input = node as HTMLInputElement;
        input.value = '中文';
        input.dispatchEvent(new CompositionEvent('compositionend', { bubbles: true, data: '中文' }));
        input.dispatchEvent(new InputEvent('input', { bubbles: true, data: '中文' }));
    });
    await expect(input).toHaveValue('中文');
    await expect(page.getByTestId('field-requests')).toHaveText('text:中文');
    await input.fill('中文!');
    await expect(page.getByTestId('field-requests')).toHaveText('text:中文|text:中文!');
    await page.getByTestId('field-policy').selectOption('refuse');
    await input.focus();
    await input.evaluate(node => {
        const input = node as HTMLInputElement;
        input.dispatchEvent(new CompositionEvent('compositionstart', { bubbles: true }));
        input.value = '拒绝';
        input.dispatchEvent(new CompositionEvent('compositionend', { bubbles: true }));
        input.dispatchEvent(new InputEvent('input', { bubbles: true }));
    });
    await expect(input).toHaveValue('中文!');
    await expect(page.getByTestId('field-requests')).toHaveText('text:中文|text:中文!|text:拒绝');
});

test('field drafts keep invalid and refused text out of FormData and settle once', async ({ page }) => {
    await fieldsReady(page);
    const number = page.locator('input[data-testid="field-number"]');
    await number.fill('-');
    await expect(number).toHaveValue('-');
    await expect(number).toHaveAttribute('aria-invalid', 'true');
    expect((await nativeValues(page)).number).toBe('12');
    await number.press('Tab');
    await expect(number).toHaveValue('12');
    await page.getByTestId('field-policy').selectOption('refuse');
    await number.fill('18');
    await expect(number).toHaveValue('18');
    expect((await nativeValues(page)).number).toBe('12');
    await number.press('Enter');
    await expect(number).toHaveValue('12');
    await expect(page.getByTestId('field-requests')).toHaveText('number:18');
    await page.getByTestId('field-saved').click();
    await number.fill('19');
    await number.press('Escape');
    await expect(number).toHaveValue('12');
    await expect(page.getByTestId('field-requests')).toHaveText('number:18');
    const color = page.locator('input[data-testid="field-color"]');
    await color.fill('#1f');
    await expect(color).toHaveAttribute('aria-invalid', 'true');
    expect((await nativeValues(page)).color).toBe('#112233');
    await color.press('Tab');
    await expect(color).toHaveValue('#112233');
    await page.getByTestId('field-policy').selectOption('accept');
    await number.focus();
    await number.press('ArrowUp');
    await expect(number).toHaveValue('13');
    await expect(number).toHaveAttribute('aria-valuenow', '13');
});

test('field submit flushes accepted drafts once, reports Busy and rejects reset during save', async ({ page }) => {
    await fieldsReady(page);
    const number = page.locator('input[data-testid="field-number"]');
    await page.getByTestId('field-policy').selectOption('refuse');
    await number.fill('18');
    await page.getByTestId('native-form').evaluate(form => (form as HTMLFormElement).requestSubmit());
    await expect(number).toHaveValue('12');
    await expect(page.getByTestId('field-requests')).toHaveText('number:18');
    await expect(page.getByTestId('field-saves')).toHaveText('1');
    await expect(page.getByTestId('field-asked')).toHaveText('Save');
    expect((await nativeValues(page)).number).toBe('12');
    await page.getByTestId('field-submit').click();
    await expect(page.getByTestId('field-saves')).toHaveText('1');
    await expect(page.getByTestId('field-asked')).toHaveText('Save|Busy');
    await page.getByTestId('field-reset').click();
    await expect(page.getByTestId('field-resets')).toHaveText('0');
    await expect(page.getByTestId('field-asked')).toHaveText('Save|Busy|Busy');
    await page.getByTestId('field-failed').click();
    await expect(page.getByTestId('field-failure')).toHaveText('Try again');
    await page.getByTestId('field-submit').click();
    await expect(page.getByTestId('field-saves')).toHaveText('2');
});

test('field unbound native constraints block before rules and expose real invalid targets', async ({ page }) => {
    await fieldsReady(page);
    const form = page.getByTestId('native-form');
    await form.evaluate(node => {
        (window as any).__invalidFields = [];
        for (const [id, external] of [['plain-required', false], ['external-required', true]] as const) {
            const input = document.createElement('input');
            input.id = id; input.name = id; input.required = true;
            if (external) input.setAttribute('form', node.id);
            input.addEventListener('invalid', event => (window as any).__invalidFields.push({ id: (event.target as HTMLInputElement).id, message: input.validationMessage, bubbles: event.bubbles }));
            (external ? node.parentElement! : node).append(input);
        }
    });
    await page.getByTestId('field-submit').click();
    await expect(page.locator('#plain-required')).toBeFocused();
    await expect(page.getByTestId('field-rules')).toHaveText('0');
    await expect(page.getByTestId('field-saves')).toHaveText('0');
    await expect(page.getByTestId('field-asked')).toHaveText('');
    const invalid = await page.evaluate(() => (window as any).__invalidFields);
    expect(invalid.map((entry: any) => entry.id)).toEqual(['plain-required', 'external-required']);
    expect(invalid.every((entry: any) => entry.message && !entry.bubbles)).toBe(true);
    await page.locator('#plain-required').fill('ok');
    await page.getByTestId('field-submit').click();
    await expect(page.locator('#external-required')).toBeFocused();
    await expect(page.getByTestId('field-rules')).toHaveText('0');
    await page.locator('#external-required').fill('ok');
    await page.locator('input[data-testid="field-text"]').fill('');
    await page.getByTestId('field-submit').click();
    await expect(page.locator('input[data-testid="field-text"]')).toBeFocused();
    await expect(page.getByTestId('error-text')).toHaveAttribute('data-source', 'constraint');
    await expect(page.getByTestId('field-asked')).toHaveText('Blocked { first_error: "text" }');
    await expect(page.getByTestId('field-saves')).toHaveText('0');
    await page.locator('input[data-testid="field-text"]').fill('valid');
    await page.getByTestId('field-submit').click();
    await expect(page.getByTestId('field-saves')).toHaveText('1');
});

test('field reset remains an application request and composing Draft never previews or submits', async ({ page }) => {
    await fieldsReady(page);
    const input = page.locator('input[data-testid="field-text"]');
    await input.fill('changed');
    await page.getByTestId('field-policy').selectOption('refuse');
    await page.getByTestId('field-reset').click();
    await expect(input).toHaveValue('changed');
    expect((await nativeValues(page)).title).toBe('changed');
    await page.getByTestId('field-policy').selectOption('accept');
    await page.getByTestId('field-reset').click();
    await expect(input).toHaveValue('accepted');
    const number = page.locator('input[data-testid="field-number"]');
    await number.focus();
    await number.evaluate(node => {
        const input = node as HTMLInputElement;
        input.dispatchEvent(new CompositionEvent('compositionstart', { bubbles: true }));
        input.value = 'ni';
        input.dispatchEvent(new InputEvent('input', { bubbles: true, isComposing: true }));
    });
    await number.press('Enter');
    await number.press('Escape');
    await page.getByTestId('native-form').evaluate(form => (form as HTMLFormElement).requestSubmit());
    await expect(number).toHaveValue('ni');
    await expect(page.getByTestId('field-saves')).toHaveText('0');
    await expect(page.getByTestId('field-previews')).toHaveText('');
    await number.evaluate(node => {
        const input = node as HTMLInputElement;
        input.value = '17';
        input.dispatchEvent(new CompositionEvent('compositionend', { bubbles: true }));
        input.dispatchEvent(new InputEvent('input', { bubbles: true }));
    });
    await expect(page.getByTestId('field-previews')).toHaveText('number:17');
    await number.press('Tab');
    await expect(number).toHaveValue('17');
    await expect(page.getByTestId('field-requests')).toHaveText('text:changed|number:17');
});

test('field registered choices and slider focus real controls in native form order', async ({ page }) => {
    await fieldsReady(page);
    await page.getByTestId('form-choices-clear').click();
    const submit = page.getByTestId('form-choices-submit');
    await submit.click();
    await expect(page.getByTestId('registered-consent')).toBeFocused();
    await expect(page.getByTestId('registered-error-consent')).toHaveAttribute('data-source', 'constraint');
    await page.getByTestId('registered-consent').check();
    await submit.click();
    await expect(page.getByTestId('registered-tags-a')).toBeFocused();
    await expect(page.getByTestId('registered-error-tags')).toHaveAttribute('data-source', 'constraint');
    await page.getByTestId('registered-tags-b').check();
    await submit.click();
    await expect(page.getByTestId('registered-radio-a')).toBeFocused();
    await page.getByTestId('registered-radio-a').check();
    await submit.click();
    await expect(page.getByTestId('registered-toggle')).toBeFocused();
    await expect(page.getByTestId('registered-error-toggle')).toHaveAttribute('data-source', 'constraint');
    await page.getByTestId('registered-toggle').click();
    await submit.click();
    await expect(page.getByTestId('registered-formats-a')).toBeFocused();
    await expect(page.getByTestId('registered-error-formats')).toHaveAttribute('data-source', 'constraint');
    await page.getByTestId('registered-formats-b').click();
    await submit.click();
    const slider = page.getByTestId('registered-slider');
    await expect(slider).toBeFocused();
    await expect(page.getByTestId('registered-error-amount')).toHaveAttribute('data-source', 'rule');
    await slider.press('End');
    await submit.click();
    await expect(page.getByTestId('form-choices-saves')).toHaveText('1');
    const values = await page.getByTestId('choice-registered-form').evaluate(form => Object.fromEntries(new FormData(form as HTMLFormElement)));
    expect(values).toEqual({ consent: 'on', tags: 'b', radio: 'a', toggle: 'on', formats: 'b', amount: '100' });
});

test('field scope disposal releases an active composition listener and native form bridges', async ({ page }) => {
    await fieldsReady(page);
    const input = page.locator('input[data-testid="field-text"]');
    await input.focus();
    const cdp = await page.context().newCDPSession(page);
    const documentRef = await cdp.send('Runtime.evaluate', { expression: 'document', objectGroup: 'field-listeners' });
    const parentRef = await cdp.send('Runtime.evaluate', { expression: 'document.getElementById("native-form").parentElement', objectGroup: 'field-listeners' });
    const count = async (objectId: string, type: string) => {
        const { listeners } = await cdp.send('DOMDebugger.getEventListeners', { objectId });
        return listeners.filter(listener => listener.type === type).length;
    };
    const documentId = documentRef.result.objectId!;
    const parentId = parentRef.result.objectId!;
    const idleKeys = await count(documentId, 'keydown');
    expect(await count(parentId, 'submit')).toBe(1);
    expect(await count(parentId, 'reset')).toBe(1);
    await input.dispatchEvent('compositionstart');
    expect(await count(documentId, 'keydown')).toBe(idleKeys + 1);
    await page.evaluate(() => (window as any).__primitives.dispose());
    expect(await count(documentId, 'keydown')).toBe(idleKeys);
    expect(await count(parentId, 'submit')).toBe(0);
    expect(await count(parentId, 'reset')).toBe(0);
    await expect.poll(() => page.evaluate(() => (window as any).__primitives.stats())).toMatchObject({ tasks: 0, timers: 0, observers: 0, errors: 0 });
    await expect(page.getByTestId('native-form')).toHaveCount(0);
    await cdp.detach();
});

const selectTrigger = (page: import('@playwright/test').Page) => page.getByTestId('raw-select-trigger');
const selectList = (page: import('@playwright/test').Page) => page.getByTestId('raw-select-list');
const selectShown = (page: import('@playwright/test').Page) => selectTrigger(page).locator('[data-rustify-part="select-value"]');
const selectNative = (page: import('@playwright/test').Page) => page.getByTestId('raw-select-form').evaluate(form => Object.fromEntries(new FormData(form as HTMLFormElement)));
async function activeOption(page: import('@playwright/test').Page) {
    return selectTrigger(page).evaluate(node => {
        const id = node.getAttribute('aria-activedescendant');
        const active = id && document.getElementById(id);
        return active ? { id, test: active.getAttribute('data-testid'), role: active.getAttribute('role') } : null;
    });
}

test('select trigger owns focus, mounted ARIA targets and native accepted form state', async ({ page }) => {
    await fieldsReady(page);
    const trigger = selectTrigger(page);
    await expect(trigger).toHaveClass(/host-select-trigger/);
    await expect(selectShown(page)).toHaveText('Banana');
    expect(await selectNative(page)).toEqual({ fruit: 'b' });
    await trigger.click();
    await expect(selectList(page)).toBeVisible();
    await expect(selectList(page)).toHaveAttribute('data-caller-content', 'kept');
    await expect(trigger).toBeFocused();
    await expect(trigger).toHaveAttribute('aria-expanded', 'true');
    expect(await activeOption(page)).toMatchObject({ test: 'raw-option-1-b', role: 'option' });
    const listId = await selectList(page).getAttribute('id');
    await expect(trigger).toHaveAttribute('aria-controls', listId!);
    await trigger.press('ArrowDown');
    expect(await activeOption(page)).toMatchObject({ test: 'raw-option-2-bb' });
    await expect(selectShown(page)).toHaveText('Banana');
    await trigger.press('Enter');
    await expect(selectList(page)).toHaveCount(0);
    await expect(trigger).toBeFocused();
    await expect(selectShown(page)).toHaveText('Blackberry');
    expect(await selectNative(page)).toEqual({ fruit: 'bb' });
    await expect(trigger).not.toHaveAttribute('aria-controls');
    await expect(trigger).not.toHaveAttribute('aria-activedescendant');
    await trigger.press('ArrowDown');
    await trigger.press('Home');
    expect(await activeOption(page)).toMatchObject({ test: 'raw-option-0-a' });
    await trigger.press('End');
    expect(await activeOption(page)).toMatchObject({ test: 'raw-option-4-c' });
    await trigger.press('Tab');
    await expect(selectList(page)).toHaveCount(0);
    await expect(page.getByTestId('select-submit')).toBeFocused();
});

test('select cancellation, refusal and delayed values never become a second displayed value', async ({ page }) => {
    await fieldsReady(page);
    await page.getByTestId('select-policy').selectOption('refuse');
    await selectTrigger(page).click();
    await page.getByTestId('raw-option-0-a').click();
    await expect(page.getByTestId('select-requests')).toHaveText('a');
    await expect(selectShown(page)).toHaveText('Banana');
    expect(await selectNative(page)).toEqual({ fruit: 'b' });
    await page.getByTestId('select-policy').selectOption('delay');
    await selectTrigger(page).click();
    await page.getByTestId('raw-option-4-c').click();
    await expect(selectShown(page)).toHaveText('Banana');
    await page.getByTestId('select-apply').click();
    await expect(selectShown(page)).toHaveText('Cherry');
    expect(await selectNative(page)).toEqual({ fruit: 'c' });
    await page.getByTestId('select-cancel').click();
    await selectTrigger(page).click();
    await page.getByTestId('raw-option-0-a').click();
    await expect(selectList(page)).toBeVisible();
    await expect(selectTrigger(page)).toBeFocused();
    await expect(page.getByTestId('select-requests')).toHaveText('a|c');
    await expect(selectShown(page)).toHaveText('Cherry');
    await selectTrigger(page).press('Escape');
    await page.getByTestId('select-cancel').click();
    await page.getByTestId('select-policy').selectOption('accept');
    await page.getByTestId('select-refuse-close').click();
    await selectTrigger(page).click();
    await page.getByTestId('raw-option-0-a').click();
    await expect(selectList(page)).toBeVisible();
    await expect(selectShown(page)).toHaveText('Apricot');
    await expect(page.getByTestId('raw-option-0-a')).toHaveAttribute('aria-selected', 'true');
});

test('select typeahead cycles repeated characters, resets after one second and opens upward at the last item', async ({ page }) => {
    await fieldsReady(page);
    await page.getByTestId('select-clear').click();
    const trigger = selectTrigger(page);
    await trigger.press('ArrowUp');
    expect(await activeOption(page)).toMatchObject({ test: 'raw-option-4-c' });
    await trigger.press('Escape');
    await trigger.press('b');
    expect(await activeOption(page)).toMatchObject({ test: 'raw-option-1-b' });
    await trigger.press('b');
    expect(await activeOption(page)).toMatchObject({ test: 'raw-option-2-bb' });
    await trigger.press('b');
    expect(await activeOption(page)).toMatchObject({ test: 'raw-option-1-b' });
    await page.waitForTimeout(1100);
    await trigger.press('c');
    expect(await activeOption(page)).toMatchObject({ test: 'raw-option-4-c' });
    await trigger.press('Escape');
    await trigger.press('b');
    await trigger.press('l');
    expect(await activeOption(page)).toMatchObject({ test: 'raw-option-2-bb' });
    await expect(page.getByTestId('select-requests')).toHaveText('');
    await expect(page.getByTestId('select-accepted')).toHaveText('');
});

test('select Escape and outside hooks cancel synchronously without stealing outside focus', async ({ page }) => {
    await fieldsReady(page);
    await page.getByTestId('select-cancel-escape').click();
    await selectTrigger(page).click();
    await selectTrigger(page).press('Escape');
    await expect(selectList(page)).toBeVisible();
    await expect(page.getByTestId('select-open-requests')).toHaveText('true');
    await expect(page.getByTestId('select-hooks')).toHaveText('open-focus|escape');
    await page.getByTestId('select-after').click();
    await expect(selectList(page)).toHaveCount(0);
    await expect(page.getByTestId('select-after')).toBeFocused();
    await page.getByTestId('select-cancel-outside').click();
    await selectTrigger(page).click();
    await page.getByTestId('select-after').click();
    await expect(selectList(page)).toBeVisible();
    await expect(page.getByTestId('select-after')).toBeFocused();
    await page.getByTestId('select-program-close').click();
    await expect(selectList(page)).toHaveCount(0);
    await expect(page.getByTestId('select-program-close')).toBeFocused();
    await expect(page.getByTestId('select-requests')).toHaveText('');
    expect((await page.getByTestId('select-hooks').textContent())?.split('close-focus')).toHaveLength(3);
});

test('select metadata changes preserve business values and never point at missing or duplicate options', async ({ page }) => {
    await fieldsReady(page);
    await selectTrigger(page).click();
    await page.getByTestId('select-mode').selectOption('remove');
    await expect(selectShown(page)).toHaveText('Choose fruit');
    await expect(page.getByTestId('select-accepted')).toHaveText('b');
    expect(await selectNative(page)).toEqual({ fruit: '' });
    expect(await activeOption(page)).toMatchObject({ test: 'raw-option-0-a' });
    await page.getByTestId('select-mode').selectOption('empty');
    await expect(selectTrigger(page)).not.toHaveAttribute('aria-activedescendant');
    await selectTrigger(page).press('Enter');
    await expect(page.getByTestId('select-requests')).toHaveText('');
    await page.getByTestId('select-mode').selectOption('duplicate-entries');
    await expect(page.getByTestId('raw-option-1-b')).toHaveAttribute('aria-disabled', 'true');
    await expect(page.getByTestId('raw-option-2-b')).toHaveAttribute('aria-disabled', 'true');
    await expect(selectList(page)).toHaveAttribute('data-rustify-issue', 'duplicate-entry');
    await page.getByTestId('select-mode').selectOption('duplicate-items');
    await expect(page.getByTestId('raw-option-duplicate')).toHaveAttribute('data-rustify-issue', 'duplicate-item');
    await page.getByTestId('select-mode').selectOption('missing-item');
    await expect(selectList(page)).toHaveAttribute('data-rustify-issue', 'missing-item');
    await selectTrigger(page).press('Home');
    expect(await activeOption(page)).toMatchObject({ test: 'raw-option-1-b' });
    await page.getByTestId('select-mode').selectOption('unknown-item');
    await expect(page.getByTestId('raw-option-unknown')).toHaveAttribute('data-rustify-issue', 'unknown-item');
});

test('select long lists scroll only their viewport and required failure focuses the trigger', async ({ page }) => {
    await fieldsReady(page);
    await page.setViewportSize({ width: 640, height: 280 });
    await page.getByTestId('select-mode').selectOption('long');
    await selectTrigger(page).click();
    const bounds = (await selectList(page).boundingBox())!;
    expect(bounds.y).toBeGreaterThanOrEqual(0);
    expect(bounds.y + bounds.height).toBeLessThanOrEqual(280);
    const before = await page.evaluate(() => scrollY);
    await selectTrigger(page).press('End');
    expect(await activeOption(page)).toMatchObject({ test: 'raw-option-149-item-149' });
    const viewport = selectList(page).locator('[data-rustify-part="select-viewport"]');
    expect(await viewport.evaluate(node => node.scrollTop)).toBeGreaterThan(0);
    expect(await page.evaluate(() => scrollY)).toBe(before);
    await expect(selectList(page).locator('[data-rustify-part="select-scroll-down"]')).toBeDisabled();
    await selectList(page).locator('[data-rustify-part="select-scroll-up"]').click();
    await expect(selectTrigger(page)).toBeFocused();
    await selectTrigger(page).press('Escape');
    await page.getByTestId('select-submit').click();
    await expect(selectTrigger(page)).toBeFocused();
    await expect(page.getByTestId('select-error')).toHaveAttribute('data-source', 'constraint');
    await expect(page.getByTestId('select-saves')).toHaveText('0');
});

test('select readonly inspects without selection and native target cancellation precedes defaults', async ({ page }) => {
    await fieldsReady(page);
    await page.getByTestId('select-cancel-target').click();
    await selectTrigger(page).click();
    await expect(selectList(page)).toHaveCount(0);
    await expect(page.getByTestId('select-open-requests')).toHaveText('');
    await page.getByTestId('select-cancel-target').click();
    await page.getByTestId('select-readonly').click();
    await selectTrigger(page).press('ArrowDown');
    await selectTrigger(page).press('End');
    await selectTrigger(page).press('Enter');
    await expect(selectShown(page)).toHaveText('Banana');
    await expect(page.getByTestId('select-requests')).toHaveText('');
    await page.getByTestId('select-disabled').click();
    await expect(selectTrigger(page)).toBeDisabled();
});

test('select detached trigger releases its layer even when close is refused and reattaches safely', async ({ page }) => {
    await fieldsReady(page);
    await page.getByTestId('select-refuse-close').click();
    await selectTrigger(page).click();
    await page.getByTestId('select-trigger-visible').evaluate(node => (node as HTMLButtonElement).click());
    await expect(selectList(page)).toHaveCount(0);
    await expect(page.getByTestId('select-open-requests')).toHaveText('true|false');
    await page.getByTestId('select-trigger-visible').click();
    await expect(selectList(page)).toBeVisible();
    await expect(selectTrigger(page)).toBeFocused();
    await expect(selectShown(page)).toHaveText('Banana');
});

test('select content forwards native attributes and caller cancellation precedes item selection', async ({ page }) => {
    await fieldsReady(page);
    await page.getByTestId('select-cancel-content').click();
    await selectTrigger(page).click();
    await expect(selectList(page)).toHaveAttribute('data-caller-content', 'kept');
    await page.getByTestId('raw-option-0-a').click();
    await expect(selectList(page)).toBeVisible();
    await expect(selectShown(page)).toHaveText('Banana');
    await expect(page.getByTestId('select-requests')).toHaveText('');
    await page.getByTestId('select-cancel-content').evaluate(node => (node as HTMLButtonElement).click());
    await page.getByTestId('raw-option-0-a').click();
    await expect(selectList(page)).toHaveCount(0);
    await expect(selectShown(page)).toHaveText('Apricot');
    await expect(selectTrigger(page)).toBeFocused();
});

test('select nested layers consume only their own Escape, with composing and Draft priority', async ({ page }) => {
    await fieldsReady(page);
    await page.getByTestId('layer-open').click();
    await expect(page.getByTestId('layer-first')).toBeFocused();
    const trigger = page.getByTestId('layer-select');
    await trigger.click();
    await expect(page.getByTestId('layer-list')).toBeVisible();
    const composing = await trigger.evaluate(node => {
        const event = new KeyboardEvent('keydown', { key: 'Escape', isComposing: true, bubbles: true, cancelable: true });
        node.dispatchEvent(event);
        return event.defaultPrevented;
    });
    expect(composing).toBe(false);
    await expect(page.getByTestId('layer-calls')).not.toContainText('escape');
    await trigger.press('Escape');
    await expect(page.getByTestId('layer-list')).toBeVisible();
    await expect(page.getByTestId('layer-calls')).toHaveText('outer-open-focus|inner-escape');
    await page.getByTestId('layer-cancel-escape').evaluate(node => (node as HTMLButtonElement).click());
    await trigger.press('Escape');
    await expect(page.getByTestId('layer-list')).toHaveCount(0);
    await expect(trigger).toBeFocused();
    await expect(page.getByTestId('layer-panel')).toBeVisible();
    const number = page.getByTestId('layer-number');
    await number.fill('19');
    await number.press('Escape');
    await expect(number).toHaveValue('12');
    await expect(page.getByTestId('layer-panel')).toBeVisible();
    await number.press('Escape');
    await expect(page.getByTestId('layer-panel')).toHaveCount(0);
    await expect(page.getByTestId('layer-open')).toBeFocused();
    await expect(page.getByTestId('layer-calls')).toHaveText('outer-open-focus|inner-escape|inner-escape|outer-escape|outer-close|outer-close-focus');
});

test('select nested outside dismissal preserves the parent and Layer autofocus hooks cancel defaults', async ({ page }) => {
    await fieldsReady(page);
    await page.getByTestId('layer-cancel-open-focus').click();
    await page.getByTestId('layer-open').click();
    await expect(page.getByTestId('layer-first')).not.toBeFocused();
    await page.getByTestId('layer-select').click();
    await page.getByTestId('layer-after').click();
    await expect(page.getByTestId('layer-list')).toHaveCount(0);
    await expect(page.getByTestId('layer-panel')).toBeVisible();
    await expect(page.getByTestId('layer-after')).toBeFocused();
    await page.getByTestId('layer-cancel-close-focus').click();
    await page.getByTestId('layer-program-close').click();
    await expect(page.getByTestId('layer-panel')).toHaveCount(0);
    await expect(page.getByTestId('layer-open')).not.toBeFocused();
    await expect(page.getByTestId('layer-calls')).toHaveText('outer-open-focus|outer-close-focus');
});

test('select placement reports the same RTL flip as its actual viewport coordinates', async ({ page }) => {
    await fieldsReady(page);
    await page.setViewportSize({ width: 640, height: 480 });
    await page.evaluate(() => document.styleSheets[0].insertRule('[data-testid="raw-select-trigger"] { position: fixed; right: 4px; bottom: 4px; width: 180px; }', 0));
    await page.getByTestId('select-direction').click();
    await selectTrigger(page).click();
    await expect(selectList(page)).toHaveAttribute('dir', 'rtl');
    await expect(selectList(page)).toHaveAttribute('data-side', 'top');
    await expect(selectList(page)).toHaveAttribute('data-align', 'start');
    const trigger = (await selectTrigger(page).boundingBox())!;
    const list = (await selectList(page).boundingBox())!;
    expect(list.y + list.height).toBeLessThanOrEqual(trigger.y + 1);
    expect(Math.abs(list.x + list.width - trigger.x - trigger.width)).toBeLessThan(1);
    expect(list.x).toBeGreaterThanOrEqual(0);
    await selectTrigger(page).press('Escape');
});

test('select scope disposal clears native typeahead timers, observers and content bridges', async ({ page }) => {
    await page.addInitScript(() => {
        const mutation = new Set<MutationObserver>();
        const resize = new Set<ResizeObserver>();
        const timers = new Set<number>();
        const Mutation = window.MutationObserver;
        const Resize = window.ResizeObserver;
        const timeout = window.setTimeout.bind(window);
        const clear = window.clearTimeout.bind(window);
        window.MutationObserver = class extends Mutation {
            observe(node: Node, options?: MutationObserverInit) { super.observe(node, options); mutation.add(this); }
            disconnect() { mutation.delete(this); super.disconnect(); }
        };
        window.ResizeObserver = class extends Resize {
            observe(node: Element, options?: ResizeObserverOptions) { super.observe(node, options); resize.add(this); }
            disconnect() { resize.delete(this); super.disconnect(); }
        };
        window.setTimeout = ((handler: TimerHandler, delay?: number, ...args: any[]) => {
            if (delay !== 1000 || typeof handler !== 'function') return timeout(handler, delay, ...args);
            const id = timeout(() => { timers.delete(id); handler(...args); }, delay);
            timers.add(id);
            return id;
        }) as typeof window.setTimeout;
        window.clearTimeout = (id?: number) => { if (id !== undefined) timers.delete(id); clear(id); };
        (window as any).__selectResources = () => ({ mutation: mutation.size, resize: resize.size, timers: timers.size });
    });
    await fieldsReady(page);
    const cdp = await page.context().newCDPSession(page);
    const documentRef = await cdp.send('Runtime.evaluate', { expression: 'document', objectGroup: 'select-listeners' });
    const count = async (id: string, type: string) => (await cdp.send('DOMDebugger.getEventListeners', { objectId: id })).listeners.filter(listener => listener.type === type).length;
    const idlePointer = await count(documentRef.result.objectId!, 'pointerdown');
    await selectTrigger(page).press('b');
    await expect(selectList(page)).toBeVisible();
    expect((await page.evaluate(() => (window as any).__selectResources())).timers).toBe(1);
    const bridge = await cdp.send('Runtime.evaluate', { expression: 'document.querySelector("[data-testid=raw-select-list]").parentElement', objectGroup: 'select-listeners' });
    expect(await count(bridge.result.objectId!, 'click')).toBe(1);
    expect(await count(documentRef.result.objectId!, 'pointerdown')).toBe(idlePointer + 1);
    await page.evaluate(() => (window as any).__primitives.dispose());
    expect(await count(bridge.result.objectId!, 'click')).toBe(0);
    expect(await count(documentRef.result.objectId!, 'pointerdown')).toBe(idlePointer);
    await expect.poll(() => page.evaluate(() => (window as any).__selectResources())).toEqual({ mutation: 0, resize: 0, timers: 0 });
    await expect.poll(() => page.evaluate(() => (window as any).__primitives.stats())).toMatchObject({ tasks: 0, timers: 0, observers: 0, errors: 0 });
    await cdp.detach();
});


test('slider uses real browser touch pointers for capture, accepted values and one commit', async ({ page }) => {
    await page.goto('/');
    await expect(page.locator('#status')).toHaveAttribute('data-status', 'ready');
    const group = page.getByTestId('slider-group');
    const left = page.getByTestId('slider-left');
    await left.scrollIntoViewIfNeeded();
    await group.evaluate(node => node.addEventListener('pointerdown', event => { node.setAttribute('data-observed-pointer', (event as PointerEvent).pointerType); }, { once: true }));
    const box = (await group.locator('[data-rustify-part="slider-track"]').boundingBox())!;
    const client = await page.context().newCDPSession(page);
    const point = (fraction: number) => [{ x: box.x + box.width * fraction, y: box.y + box.height / 2, id: 1 }];
    await client.send('Input.dispatchTouchEvent', { type: 'touchStart', touchPoints: point(.2) });
    await client.send('Input.dispatchTouchEvent', { type: 'touchMove', touchPoints: point(.45) });
    await expect(group).toHaveAttribute('data-observed-pointer', 'touch');
    await expect(left).toHaveAttribute('aria-valuenow', '45');
    await client.send('Input.dispatchTouchEvent', { type: 'touchEnd', touchPoints: [] });
    await expect(page.getByTestId('slider-commits')).toHaveText('1');
    expect(await sliderForm(page)).toEqual([['range', '45'], ['range', '80']]);
    await client.detach();
});

test('field keeps browser selection, clipboard paste and native undo through controlled acceptance', async ({ page, context }) => {
    await fieldsReady(page);
    await context.grantPermissions(['clipboard-read', 'clipboard-write']);
    const input = page.locator('input[data-testid="field-text"]');
    await input.focus();
    await input.press('ControlOrMeta+A');
    await page.keyboard.insertText('Native edit');
    await input.press('ControlOrMeta+A');
    expect(await input.evaluate(node => { const field = node as HTMLInputElement; return [field.selectionStart, field.selectionEnd]; })).toEqual([0, 11]);
    await input.press('ControlOrMeta+C');
    expect(await page.evaluate(() => navigator.clipboard.readText())).toBe('Native edit');
    await page.evaluate(() => navigator.clipboard.writeText('Pasted edit'));
    await input.press('ControlOrMeta+V');
    await expect(input).toHaveValue('Pasted edit');
    await input.press('ControlOrMeta+Z');
    await expect(input).toHaveValue('Native edit');
    await input.press('ControlOrMeta+Shift+Z');
    await expect(input).toHaveValue('Pasted edit');
    expect((await nativeValues(page)).title).toBe('Pasted edit');
});

for (const [kind, draft, accepted] of [['number', '18', '12'], ['color', '#abcdef', '#112233']]) {
    test(`successful reset discards the focused ${kind} draft before blur, Enter and submit`, async ({ page }) => {
        await page.goto('/');
        const editor = page.getByTestId(`field-${kind}`);
        for (const end of ['blur', 'Enter', 'submit']) {
            await editor.fill(draft);
            await page.locator('#native-form').evaluate(node => (node as HTMLFormElement).reset());
            await expect(editor).toBeFocused();
            await expect(editor).toHaveValue(accepted);
            if (end === 'blur') await editor.blur();
            else if (end === 'Enter') await editor.press('Enter');
            else await page.locator('#native-form').evaluate(node => (node as HTMLFormElement).requestSubmit());
            await expect(page.getByTestId('field-requests')).toHaveText('');
            await page.getByTestId('field-saved').click();
        }
    });
    test(`refused and busy reset retain the focused ${kind} draft`, async ({ page }) => {
        await page.goto('/');
        await page.getByTestId('field-policy').selectOption('refuse');
        const editor = page.getByTestId(`field-${kind}`);
        await editor.fill(draft);
        await page.locator('#native-form').evaluate(node => (node as HTMLFormElement).reset());
        await expect(editor).toHaveValue(draft);
        await page.getByTestId('field-policy').selectOption('accept');
        await page.locator('#native-form').evaluate(node => (node as HTMLFormElement).requestSubmit());
        const busyDraft = kind === 'number' ? '19' : '#fedcba';
        await editor.fill(busyDraft);
        await page.locator('#native-form').evaluate(node => (node as HTMLFormElement).reset());
        await expect(editor).toHaveValue(busyDraft);
    });
}

test('field input registration follows mounted owners and recovers from duplicates', async ({ page }) => {
    await page.goto('/');
    const input = page.getByTestId('lifecycle-input');
    for (let round = 0; round < 3; round++) {
        await page.getByTestId('input-toggle').click();
        await expect(input).toHaveCount(0);
        await page.getByTestId('input-toggle').click();
        await expect(input).toBeEnabled();
        await input.fill(`title${round}`);
        await expect(input).toHaveValue(`title${round}`);
    }
    await page.getByTestId('input-duplicate').click();
    await expect(input).toBeDisabled();
    await expect(input.locator('..')).toHaveAttribute('data-rustify-issue', 'multiple-inputs');
    await page.getByTestId('input-duplicate').click();
    await expect(input).toBeEnabled();
    await input.fill('recovered');
    await expect(input).toHaveValue('recovered');
    await input.evaluate(node => {
        const editor = node as HTMLInputElement;
        editor.value = 'refused';
        editor.addEventListener('input', event => event.preventDefault(), { once: true });
        editor.dispatchEvent(new Event('input', { bubbles: true, cancelable: true }));
    });
    await expect(input).toHaveValue('recovered');
    await page.getByTestId('input-duplicate').click();
    await page.getByTestId('input-toggle').click();
    const remaining = page.getByTestId('lifecycle-second');
    await expect(remaining).toBeEnabled();
    await remaining.fill('second survives');
    await expect(remaining).toHaveValue('second survives');
    await remaining.evaluate(node => {
        const editor = node as HTMLInputElement;
        editor.value = 'refused';
        editor.addEventListener('input', event => event.preventDefault(), { once: true });
        editor.dispatchEvent(new Event('input', { bubbles: true, cancelable: true }));
    });
    await expect(remaining).toHaveValue('second survives');
    await page.getByTestId('lifecycle-submit').click();
    await expect(remaining).toBeFocused();
});

test('field labels follow explicit IDs and replacement controls', async ({ page }) => {
    await page.goto('/');
    for (let round = 0; round < 3; round++) {
        for (const prefix of ['lifecycle', 'implicit']) {
            const input = page.getByTestId(`${prefix}-input`);
            const label = page.getByTestId(`${prefix}-label`);
            expect(await label.evaluate(node => (node as HTMLLabelElement).control?.id)).toBe(await input.getAttribute('id'));
            await label.click();
            await expect(input).toBeFocused();
            await expect(page.getByRole('textbox', { name: prefix === 'lifecycle' ? 'Explicit title' : 'Implicit title', exact: true })).toHaveCount(1);
        }
        await page.getByTestId('lifecycle-submit').click();
        await expect(page.getByTestId('lifecycle-input')).toBeFocused();
        await page.getByTestId('control-toggle').click();
        await page.getByTestId('control-toggle').click();
    }
    await page.getByTestId('legacy-submit').click();
    await expect(page.getByTestId('legacy-input')).toBeFocused();
    const select = page.getByRole('combobox', { name: 'Fruit', exact: true });
    await expect(select).toHaveAttribute('id', 'raw-select-trigger');
    expect(await page.getByText('Fruit', { exact: true }).evaluate(node => (node as HTMLLabelElement).control?.id)).toBe('raw-select-trigger');
});

test('conditional orphan parts retain caller content and diagnostics without aborting', async ({ page }) => {
    await page.goto('/');
    await page.getByTestId('missing-toggle').click();
    const parts = page.getByTestId('orphan-parts');
    for (const text of ['Content child', 'Viewport child', 'Text child', 'Indicator child', 'Up child', 'Down child', 'Description child']) {
        await expect(parts.getByText(text, { exact: true })).toBeVisible();
    }
    expect(await parts.locator('[data-rustify-issue]').count()).toBeGreaterThanOrEqual(9);
    await page.getByTestId('lifecycle-input').fill('still running');
    await expect(page.getByTestId('lifecycle-input')).toHaveValue('still running');
});

test('decimal slider accepts adjacent fractional steps at a nonzero minimum', async ({ page }) => {
    await page.goto('/');
    const low = page.getByTestId('decimal-low');
    await expect(low).not.toBeDisabled();
    await expect(low).toHaveAttribute('aria-valuenow', '100.2');
    await low.press('ArrowLeft');
    await expect(low).toHaveAttribute('aria-valuenow', '100.1');
    await low.press('ArrowRight');
    await expect(low).toHaveAttribute('aria-valuenow', '100.2');
    await low.press('ArrowRight');
    await expect(low).toHaveAttribute('aria-valuenow', '100.2');
});

test('Escape cancels an active nested slider gesture and closes the parent layer', async ({ page }) => {
    await page.goto('/');
    await page.getByTestId('layer-open').click();
    const slider = page.getByTestId('layer-slider');
    await slider.focus();
    await page.keyboard.down('ArrowRight');
    await page.keyboard.press('Escape');
    await page.keyboard.up('ArrowRight');
    await expect(page.getByTestId('layer-panel')).toHaveCount(0);
    await expect(page.getByTestId('layer-calls')).toContainText('slider-cancel');
    await expect(page.getByTestId('layer-calls')).not.toContainText('slider-commit');
    await page.getByTestId('layer-open').click();
    const box = await slider.boundingBox();
    expect(box).not.toBeNull();
    await page.mouse.move(box!.x + box!.width / 2, box!.y + box!.height / 2);
    await page.mouse.down();
    await page.keyboard.press('Escape');
    await page.mouse.up();
    await expect(page.getByTestId('layer-panel')).toHaveCount(0);
    await expect(page.getByTestId('layer-calls')).not.toContainText('slider-commit');
    expect((await page.getByTestId('layer-calls').innerText()).split('slider-cancel').length - 1).toBe(2);
});

test('raw form feedback distinguishes saving, failure and completion', async ({ page }) => {
    await page.goto('/');
    await page.getByTestId('field-text').fill('Valid title');
    await page.getByTestId('field-submit').click();
    await expect(page.getByTestId('field-status')).toHaveText('Saving');
    await page.getByTestId('field-submit').click();
    await expect(page.getByTestId('field-saves')).toHaveText('1');
    await page.getByTestId('field-failed').click();
    await expect(page.getByTestId('field-status')).toHaveText('Save failed');
    await page.getByTestId('field-submit').click();
    await page.getByTestId('field-saved').click();
    await expect(page.getByTestId('field-status')).toHaveText('Saved');
    await expect(page.getByTestId('field-saves')).toHaveText('2');
});

test('raw select has an opaque surface and explicit cancelled selection feedback', async ({ page }) => {
    await page.goto('/');
    const trigger = page.getByTestId('raw-select-trigger');
    await trigger.click();
    await page.getByTestId('raw-option-0-a').click();
    await expect(page.getByTestId('select-accepted')).toHaveText('a');
    await page.getByTestId('select-cancel').click();
    await trigger.click();
    const list = page.getByTestId('raw-select-list');
    await expect(list).not.toHaveCSS('background-color', 'rgba(0, 0, 0, 0)');
    await page.getByTestId('raw-option-1-b').click();
    await expect(page.getByTestId('select-result')).toHaveText('Cancelled: b');
    await expect(page.getByTestId('select-accepted')).toHaveText('a');
    await expect(page.getByTestId('select-request-count')).toHaveText('1');
    await expect(list).toBeVisible();
    await trigger.press('Escape');
    await expect(trigger).toBeFocused();
});
