import { test, expect } from '@playwright/test';

test('public guide consumer completes safe confirmation, auxiliary choice and persistent feedback', async ({ page }) => {
    const failures: string[] = [];
    page.on('pageerror', error => failures.push(error.message));
    page.on('console', message => { if (message.type() === 'error') failures.push(message.text()); });
    page.on('requestfailed', request => failures.push(request.url()));
    await page.addInitScript(() => document.addEventListener('securitypolicyviolation', event => console.error(`${event.violatedDirective}: ${event.blockedURI}`)));
    await page.goto('/');
    await expect(page.locator('html')).toHaveAttribute('data-ready', 'true');
    const note = page.getByRole('textbox', { name: 'Note', exact: true });
    await note.fill('Retain this address review until confirmed.');
    const clear = page.getByRole('button', { name: 'Clear note', exact: true }).first();
    await clear.click();
    const confirm = page.getByRole('alertdialog', { name: 'Clear this note?' });
    await expect(confirm).toBeVisible();
    await expect(confirm.getByRole('button', { name: 'Keep note' })).toBeFocused();
    await page.keyboard.press('Escape');
    await expect(confirm).toHaveCount(0);
    await expect(note).toHaveValue('Retain this address review until confirmed.');
    await expect(clear).toBeFocused();

    await page.getByRole('button', { name: 'Delivery options', exact: true }).click();
    const speed = page.getByRole('combobox', { name: 'Delivery speed' });
    await speed.focus(); await speed.press('Enter');
    await page.keyboard.press('End'); await page.keyboard.press('Enter');
    await expect(speed).toHaveText('Express');
    await page.getByRole('button', { name: 'Done', exact: true }).click();
    await expect(page.getByRole('status').filter({ hasText: /^express$/ })).toHaveText('express');

    await page.setViewportSize({ width: 390, height: 844 });
    await clear.focus(); await clear.press('Enter');
    await expect(confirm.getByRole('button', { name: 'Keep note' })).toBeFocused();
    await page.keyboard.press('Tab');
    await expect(confirm.getByRole('button', { name: 'Clear note', exact: true })).toBeFocused();
    await page.keyboard.press('Enter');
    await expect(confirm).toHaveCount(0);
    await expect(note).toHaveValue('');
    const result = page.locator('[data-rustify-part="callout-root"]');
    await expect(result).toHaveText('The local note was cleared.');
    await expect(page.locator('[data-rustify-part="toast-root"]')).toContainText('The local note was cleared.');
    await expect.poll(() => page.locator('body').evaluate(node => node.scrollWidth - innerWidth)).toBeLessThanOrEqual(1);
    await page.screenshot({ path: 'docs/validation/dom-components-p3/m7-public-consumer-390.png' });
    await expect(page.locator('[data-rustify-part="toast-root"]')).toHaveCount(0, { timeout: 5000 });
    await expect(result).toHaveText('The local note was cleared.');
    await expect(page.getByRole('status').filter({ hasText: /^express$/ })).toHaveText('express');
    expect(failures).toEqual([]);
});
