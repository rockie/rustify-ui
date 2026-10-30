import { test, expect } from '@playwright/test';

const failures = new WeakMap<import('@playwright/test').Page, string[]>();
test.beforeEach(async ({ page }) => {
    const errors: string[] = []; failures.set(page, errors);
    page.on('pageerror', error => errors.push(error.message));
    page.on('console', message => { if (message.type() === 'error') errors.push(message.text()); });
    page.on('requestfailed', request => errors.push(`${request.url()}: ${request.failure()?.errorText}`));
    await page.goto('/fields.html');
    await expect(page.locator('#status')).toHaveAttribute('data-status', 'ready');
});
test.afterEach(({ page }) => expect(failures.get(page)).toEqual([]));

const fieldRoot = (page: import('@playwright/test').Page, id: string) => page.getByTestId(id).locator('..');
const slotStyles = (page: import('@playwright/test').Page, id: string) => page.getByTestId(id).locator('..').evaluate(node => {
    const css = getComputedStyle(node);
    return { gap: css.gap, left: css.paddingLeft, right: css.paddingRight, color: css.color };
});

test('field skins apply all sizes and variants with independent responsive slots and utility overrides', async ({ page }) => {
    for (const [size, height] of [[1, 24], [2, 32], [3, 40]]) {
        for (const variant of ['classic', 'surface', 'soft']) {
            const input = page.getByTestId(`skin-${size}-${variant}`);
            await expect(input).toHaveAttribute('data-rustify-part', 'text-field-input');
            const root = input.locator('..');
            await expect(root).toHaveAttribute('data-variant', variant);
            expect(await root.evaluate(node => node.getBoundingClientRect().height)).toBe(height);
        }
    }
    await page.setViewportSize({ width: 1440, height: 1100 });
    const wide = await slotStyles(page, 'skin-color-slot');
    expect(wide).toMatchObject({ gap: '16px', left: '12px', right: '16px' });
    expect(await slotStyles(page, 'skin-neutral-slot')).toMatchObject({ gap: '12px', left: '12px', right: '12px' });
    expect((await slotStyles(page, 'skin-neutral-slot')).color).not.toBe(wide.color);
    expect(await slotStyles(page, 'skin-utility-slot')).toMatchObject({ gap: '1px', left: '1px', right: '1px' });
    await expect(page.getByTestId('skin-typed-input')).toHaveClass(/field-utility/);
    await expect(page.getByTestId('skin-typed-input')).toHaveClass(/rustify-field-input/);
    expect(await page.getByTestId('skin-typed-input').evaluate(node => getComputedStyle(node).paddingLeft)).toBe('1px');
    expect(await fieldRoot(page, 'skin-area').evaluate(node => getComputedStyle(node).resize)).toBe('both');
    await page.screenshot({ path: 'docs/validation/dom-components-p2/m3-fields-desktop.png', fullPage: true });
    await page.setViewportSize({ width: 390, height: 844 });
    expect(await slotStyles(page, 'skin-color-slot')).toMatchObject({ gap: '4px', left: '12px', right: '8px' });
    expect(await slotStyles(page, 'skin-neutral-slot')).toMatchObject({ gap: '8px', left: '4px', right: '4px' });
    const input = page.getByTestId('skin-slot-input');
    const start = page.getByTestId('skin-neutral-slot').locator('..');
    const end = page.getByTestId('skin-color-slot').locator('..');
    expect((await start.boundingBox())!.x).toBeLessThan((await input.boundingBox())!.x);
    const color = (await slotStyles(page, 'skin-neutral-slot')).color;
    await page.getByTestId('skin-direction').click();
    expect((await start.boundingBox())!.x).toBeGreaterThan((await input.boundingBox())!.x);
    expect((await end.boundingBox())!.x).toBeLessThan((await input.boundingBox())!.x);
    expect(await slotStyles(page, 'skin-color-slot')).toMatchObject({ left: '12px', right: '8px' });
    expect((await slotStyles(page, 'skin-neutral-slot')).color).toBe(color);
    expect(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth)).toBe(true);
    await page.screenshot({ path: 'docs/validation/dom-components-p2/m3-fields-mobile-rtl.png', fullPage: true });
});

test('field themed native submit commits a draft once and exposes real constraint errors', async ({ page }) => {
    const title = page.getByTestId('skin-title');
    await title.fill('');
    await page.getByTestId('skin-submit').click();
    await expect(title).toBeFocused();
    await expect(page.getByTestId('error-title')).toHaveAttribute('data-source', 'constraint');
    await expect(page.getByTestId('skin-saves')).toHaveText('0');
    await title.fill('Ready');
    await expect(page.getByTestId('error-title')).toHaveCount(0);
    await page.getByTestId('skin-number').fill('17.5');
    const dataBefore = await page.getByTestId('skin-form').evaluate(form => Object.fromEntries(new FormData(form as HTMLFormElement)));
    expect(dataBefore.amount).toBe('12.5');
    await page.getByTestId('skin-submit').click();
    await expect(page.getByTestId('skin-saves')).toHaveText('1');
    await expect(page.getByTestId('skin-number')).toHaveValue('17.5');
    const data = await page.getByTestId('skin-form').evaluate(form => Object.fromEntries(new FormData(form as HTMLFormElement)));
    expect(data).toEqual({ title: 'Ready', amount: '17.5', color: '#336699' });
    await page.getByTestId('skin-submit').focus();
    await page.getByTestId('skin-submit').press('Enter');
    await expect(page.getByTestId('skin-saves')).toHaveText('1');
    await expect(page.getByTestId('skin-asked')).toHaveText('Blocked { first_error: "title" }|Save|Busy');
    await page.getByTestId('skin-saved').click();
    const input = page.getByTestId('skin-2-surface');
    await page.getByTestId('skin-disabled').click();
    await expect(input).toBeDisabled();
    await page.getByTestId('skin-disabled').click();
    await page.getByTestId('skin-readonly').click();
    await expect(input).toHaveJSProperty('readOnly', true);
});

test('select skins cover all trigger sizes and treatments with utility and responsive overrides', async ({ page }) => {
    await page.setViewportSize({ width: 1440, height: 1100 });
    for (const [size, height] of [[1, 24], [2, 32], [3, 40]]) {
        for (const variant of ['classic', 'surface', 'soft', 'ghost-offset', 'ghost']) {
            const trigger = page.getByTestId(`select-${size}-${variant}`);
            await expect(trigger).toHaveAttribute('data-variant', variant);
            await expect(trigger).toHaveAttribute('role', 'combobox');
            const expectedHeight = variant === 'ghost' ? [0, 24, 28, 36][size] : height;
            await expect.poll(() => trigger.evaluate(node => node.getBoundingClientRect().height), { message: `${size} ${variant}` }).toBe(expectedHeight);
            await trigger.click();
            const list = page.getByTestId(`select-${size}-${variant}-list`);
            await expect(list).toHaveAttribute('data-variant', 'solid');
            expect(await list.locator('[role="option"]').first().evaluate(node => node.getBoundingClientRect().height)).toBe(size === 1 ? 24 : 32);
            await trigger.press('Escape');
        }
    }
    const responsive = page.getByTestId('select-responsive');
    expect(await responsive.evaluate(node => node.getBoundingClientRect().height)).toBe(40);
    expect(await responsive.evaluate(node => parseFloat(getComputedStyle(node).borderRadius))).toBeGreaterThanOrEqual(20);
    const custom = page.getByTestId('select-typed');
    await expect(custom).toHaveClass(/select-utility/);
    await expect(custom).toHaveClass(/rustify-select-trigger/);
    expect(await custom.evaluate(node => ({ height: node.getBoundingClientRect().height, padding: getComputedStyle(node).paddingLeft, radius: getComputedStyle(node).borderRadius }))).toEqual({ height: 48, padding: '1px', radius: '0px' });
    await page.getByTestId('select-skins').screenshot({ path: 'docs/validation/dom-components-p2/m4-select-light.png', animations: 'disabled' });
    await page.setViewportSize({ width: 390, height: 844 });
    expect(await responsive.evaluate(node => node.getBoundingClientRect().height)).toBe(24);
    await page.getByTestId('skin-direction').click();
    await responsive.click();
    await expect(page.getByTestId('select-responsive-list')).toHaveAttribute('dir', 'rtl');
    await page.screenshot({ path: 'docs/validation/dom-components-p2/m4-select-mobile-rtl.png', fullPage: true, animations: 'disabled' });
});

test('select skins keep local popup colors, contrast and composition through actual theme changes', async ({ page }) => {
    await page.setViewportSize({ width: 1440, height: 1100 });
    const trigger = page.getByTestId('select-responsive');
    const list = page.getByTestId('select-responsive-list');
    await trigger.click();
    await expect(list).toHaveAttribute('data-caller', 'kept');
    const active = list.locator('[data-highlighted]');
    const solid = await active.evaluate(node => getComputedStyle(node).backgroundColor);
    const colors = await Promise.all([trigger, list].map(node => node.evaluate(element => getComputedStyle(element).getPropertyValue('--rustify-component-color-9'))));
    expect(colors[0]).not.toBe(colors[1]);
    expect(await list.locator('[data-rustify-part="select-label"]').evaluate(node => getComputedStyle(node).fontSize)).toBe('14px');
    await page.getByTestId('select-skin-variant').evaluate(node => (node as HTMLButtonElement).click());
    await expect(list).toHaveAttribute('data-variant', 'soft');
    expect(await active.evaluate(node => getComputedStyle(node).backgroundColor)).not.toBe(solid);
    await page.getByTestId('select-skin-variant').evaluate(node => (node as HTMLButtonElement).click());
    await page.getByTestId('select-skin-contrast').evaluate(node => (node as HTMLButtonElement).click());
    await expect(list).toHaveAttribute('data-high-contrast', '');
    expect(await active.evaluate(node => getComputedStyle(node).backgroundColor)).not.toBe(solid);
    await page.getByTestId('skin-theme').evaluate(node => (node as HTMLButtonElement).click());
    await expect(list.locator('..')).toHaveAttribute('data-theme', 'dark');
    expect(await list.evaluate(node => getComputedStyle(node).backgroundColor)).not.toBe('rgba(0, 0, 0, 0)');
    expect(await list.evaluate(node => getComputedStyle(node).backgroundColor)).toMatch(/^rgb\(/);
    await page.screenshot({ path: 'docs/validation/dom-components-p2/m4-select-dark.png', animations: 'disabled' });
    await list.getByRole('option', { name: 'Banana' }).click();
    await expect(trigger).toHaveText('Banana');
    await expect(trigger).toBeFocused();
    await expect(list).toHaveCount(0);
    const legacy = page.getByTestId('select-legacy');
    await legacy.click();
    await page.getByTestId('select-legacy-list').getByRole('option', { name: 'Apricot' }).click();
    expect(await page.getByTestId('select-skin-form').evaluate(node => Object.fromEntries(new FormData(node as HTMLFormElement)))).toEqual({ fruit: 'a' });
    await page.getByTestId('select-skin-empty').click();
    await expect(legacy).toHaveText('Choose');
    await legacy.click();
    await expect(page.getByTestId('select-legacy-list').getByText('No options')).toBeVisible();
    await expect(legacy).not.toHaveAttribute('aria-activedescendant');
    await legacy.press('Escape');
    await page.getByTestId('select-skin-submit').click();
    await expect(legacy).toBeFocused();
});
