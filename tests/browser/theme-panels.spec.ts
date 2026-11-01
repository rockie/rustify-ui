import { test, expect, waitForReady } from './support';
import type { Page } from '@playwright/test';
import { readFile } from 'node:fs/promises';

test.use({ fresh: true });
const failures = new WeakMap<Page, string[]>();
const snapshot = (page: Page) => page.evaluate(() => window.__theme_studio.snapshot());

async function mobileSection(page: Page, section: 'edit' | 'preview') {
    const button = page.getByTestId(`mobile-${section}`);
    if (await button.isVisible()) await button.click();
}

async function review(page: Page, document: unknown) {
    await mobileSection(page, 'edit');
    await page.getByTestId('theme-import').click();
    await expect(page.getByTestId('data-tools-dialog').getByRole('dialog')).toHaveAccessibleName('Import theme');
    await page.getByTestId('import-source').fill(JSON.stringify(document));
    await page.getByTestId('import-review').click();
    await expect(page.getByTestId('import-preview')).toBeVisible();
}

test.beforeEach(async ({ page }) => {
    const errors: string[] = []; failures.set(page, errors);
    page.on('pageerror', error => errors.push(error.message));
    page.on('console', message => { if (message.type() === 'error') errors.push(message.text()); });
    page.on('requestfailed', request => errors.push(`${request.url()}: ${request.failure()?.errorText}`));
    await waitForReady(page);
    await page.getByTestId('locale-en').click();
    await page.getByTestId('renderer-dom').click();
});
test.afterEach(({ page }) => expect(failures.get(page)).toEqual([]));

test('review cancellation and nested selector Escape preserve the theme exported by the existing panel', async ({ page }) => {
    const before = await snapshot(page);
    await page.getByTestId('theme-export').click();
    const exportedBefore = await page.getByTestId('export-source').inputValue();
    await page.keyboard.press('Escape');
    const candidate = structuredClone(before.document);
    candidate.name = 'Unaccepted panel import';
    candidate.overrides.light.primary = '#164e63';
    await review(page, candidate);
    expect((await snapshot(page)).document).toEqual(before.document);

    const format = page.getByTestId('import-format');
    await format.focus(); await format.press('Enter');
    await expect(page.getByTestId('import-format-list')).toBeVisible();
    await page.keyboard.press('Escape');
    await expect(page.getByTestId('import-format-list')).toHaveCount(0);
    await expect(page.getByTestId('data-tools-dialog')).toBeVisible();
    await expect(format).toBeFocused();
    await expect(page.getByTestId('import-preview')).toContainText('Unaccepted panel import');
    await page.getByTestId('import-cancel').click();
    await expect(page.getByTestId('theme-import')).toBeFocused();
    expect((await snapshot(page)).document).toEqual(before.document);
    expect((await snapshot(page)).history).toEqual(before.history);

    await page.getByTestId('theme-export').click();
    await expect(page.getByTestId('data-tools-dialog').getByRole('dialog')).toHaveAccessibleName('Export theme');
    const source = page.getByTestId('export-source');
    expect(await source.inputValue()).toBe(exportedBefore);
    const download = page.waitForEvent('download');
    await page.getByTestId('export-download').click();
    const artifact = await download;
    expect(artifact.suggestedFilename()).toMatch(/\.json$/);
    expect(await readFile((await artifact.path())!, 'utf8')).toBe(exportedBefore);
    await page.keyboard.press('Escape');
    await expect(page.getByTestId('theme-export')).toBeFocused();
    expect((await snapshot(page)).document).toEqual(before.document);
    expect((await snapshot(page)).history).toEqual(before.history);
});

test('an applied import reaches the existing local themed dialog without changing its root appearance', async ({ page }) => {
    await page.setViewportSize({ width: 390, height: 844 });
    await mobileSection(page, 'preview');
    const preview = page.getByTestId('studio-preview');
    const local = preview.getByTestId('local-boundary');
    await preview.getByTestId('local-appearance').click();
    await preview.getByTestId('local-appearance-list').getByRole('option', { name: 'Opposite to root', exact: true }).click();
    await expect(preview).toHaveAttribute('data-theme', 'light');
    await expect(local).toHaveAttribute('data-theme', 'dark');
    const before = await snapshot(page);
    const candidate = structuredClone(before.document);
    candidate.name = 'Local panel import';
    candidate.overrides.light.primary = '#164e63';
    candidate.overrides.dark.primary = '#67e8f9';
    await review(page, candidate);
    await page.getByTestId('import-apply').click();
    await expect.poll(async () => (await snapshot(page)).document).toEqual(candidate);
    await mobileSection(page, 'preview');
    await expect(preview).toHaveAttribute('data-theme', 'light');
    await expect(local).toHaveAttribute('data-theme', 'dark');
    expect((await snapshot(page)).mode).toBe(before.mode);

    await preview.getByTestId('open-preview-dialog').click();
    const dialog = preview.getByTestId('preview-dialog');
    await expect(dialog.getByRole('dialog')).toHaveAccessibleName('Theme preview');
    await expect(dialog.getByRole('dialog')).not.toHaveAttribute('aria-modal');
    await expect(dialog).toHaveAttribute('data-theme', 'dark');
    const confirm = page.getByTestId('dialog-confirm');
    await expect(confirm).toHaveCSS('background-color', 'rgb(103, 232, 249)');
    await expect.poll(() => dialog.locator('[data-name="Dialog"]').evaluate(element => {
        const box = element.getBoundingClientRect();
        return Math.max(-box.left, box.right - innerWidth, -box.top, box.bottom - innerHeight);
    })).toBeLessThanOrEqual(1);
    await page.screenshot({ path: 'docs/validation/dom-components-p3/m7-studio-local-panel-390.png' });
    await confirm.click();
    await expect(dialog).toHaveCount(0);
    await expect(preview.getByTestId('open-preview-dialog')).toBeFocused();
    await expect(preview.getByTestId('local-card').getByRole('status')).toHaveText('Confirmed');
    expect((await snapshot(page)).document).toEqual(candidate);

    await mobileSection(page, 'edit');
    await page.getByTestId('theme-export').click();
    expect(JSON.parse(await page.getByTestId('export-source').inputValue())).toEqual(candidate);
    await page.keyboard.press('Escape');
    await expect(page.getByTestId('theme-export')).toBeFocused();
    await test.info().attach('existing-studio-panel-result', {
        body: JSON.stringify({ document: candidate, root: 'light', local: 'dark', viewport: page.viewportSize() }, null, 2),
        contentType: 'application/json',
    });
});
