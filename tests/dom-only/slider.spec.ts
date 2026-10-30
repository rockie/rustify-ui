import { test, expect } from '@playwright/test';
import { mkdir } from 'node:fs/promises';

test('themed scalar slider paints accepted geometry and keeps the form consumer controlled', async ({ page }, info) => {
    const errors: string[] = [];
    page.on('pageerror', error => errors.push(error.message));
    page.on('console', message => { if (message.type() === 'error') errors.push(message.text()); });
    page.on('requestfailed', request => errors.push(request.url()));
    await page.goto('/forms.html');
    await expect(page.locator('#status')).toHaveAttribute('data-status', 'ready');
    const thumb = page.getByTestId('bench-slider');
    const root = page.getByTestId('bench-slider-root');
    const track = root.locator('[data-rustify-part="slider-track"]');
    const range = root.locator('[data-rustify-part="slider-range"]');
    await expect(thumb).toHaveAccessibleName('Amount');
    for (const [key, value] of [['Home', '0'], ['End', '100']] as const) {
        await thumb.press(key);
        await expect(thumb).toHaveAttribute('aria-valuenow', value);
        await expect(page.getByTestId('accepted-slider')).toHaveText(value);
        const rail = (await track.boundingBox())!;
        await expect.poll(async () => (await range.boundingBox())!.width).toBeCloseTo(value === '0' ? 0 : rail.width, 1);
        const fill = (await range.boundingBox())!;
        expect(fill.y + fill.height / 2).toBeCloseTo(rail.y + rail.height / 2, 1);
    }
    await page.evaluate(() => (window as any).__form_bench.policy(false));
    await thumb.press('ArrowLeft');
    await expect(thumb).toHaveAttribute('aria-valuenow', '100');
    await page.evaluate(() => (window as any).__form_bench.reset());
    await expect(thumb).toHaveAttribute('aria-valuenow', '50');
    await expect.poll(async () => {
        const rail = (await track.boundingBox())!, handle = (await thumb.boundingBox())!;
        return Math.abs(handle.x + handle.width / 2 - (rail.x + rail.width / 2));
    }).toBeLessThan(1);
    expect(await root.locator('[style]').count()).toBe(0);
    await mkdir('docs/validation/dom-components-p2', { recursive: true });
    const path = 'docs/validation/dom-components-p2/m2-slider.png';
    await root.screenshot({ path });
    await info.attach('Themed scalar slider', { path, contentType: 'image/png' });
    expect(errors).toEqual([]);
});
