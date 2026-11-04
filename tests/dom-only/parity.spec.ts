import { test, expect, Page } from '@playwright/test';

const part = (page: Page, name: string) => page.getByTestId(name);
async function policy(page: Page, component: string, value: 'accept' | 'reject') {
    await page.evaluate(({ component, value }) => (window as any).__parity.policy(component, value), { component, value });
}
async function snapshot(page: Page, component: string) {
    return page.evaluate(component => (window as any).__parity.snapshot(component), component);
}
async function activeOption(page: Page, option: string) {
    await expect(part(page, 'select')).toBeFocused();
    await expect(part(page, option)).toHaveAttribute('data-highlighted', '');
    await expect(part(page, 'select')).toHaveAttribute('aria-activedescendant', (await part(page, option).getAttribute('id'))!);
}
const errors = new WeakMap<Page, string[]>();
test.beforeEach(async ({ page }) => {
    const failures: string[] = [];
    errors.set(page, failures);
    page.on('pageerror', error => failures.push(error.message));
    page.on('console', message => { if (message.type() === 'error') failures.push(message.text()); });
    page.on('requestfailed', request => failures.push(`${request.url()}: ${request.failure()?.errorText}`));
    await page.goto('/parity.html');
    await expect(page.locator('#status')).toHaveAttribute('data-status', 'ready');
});
test.afterEach(async ({ page }) => { expect(errors.get(page)).toEqual([]); });

for (const acceptance of ['accept', 'reject'] as const) {
    const accept = acceptance === 'accept';
    for (const choice of [
        { name: 'checkbox', target: 'checkbox', before: 'Unchecked', requested: 'Checked', aria: 'aria-checked', initialAria: 'false' },
        { name: 'checkbox-mixed', target: 'checkbox-mixed', before: 'Indeterminate', requested: 'Checked', aria: 'aria-checked', initialAria: 'mixed' },
        { name: 'checkbox-group', target: 'checkbox-group-b', before: '["a"]', requested: '["a", "b"]', aria: 'aria-checked', initialAria: 'false' },
        { name: 'radio-group', target: 'radio-group-b', before: 'Some("a")', requested: 'Some("b")', aria: 'aria-checked', initialAria: 'false' },
        { name: 'switch', target: 'switch', before: 'false', requested: 'true', aria: 'aria-checked', initialAria: 'false' },
        { name: 'toggle', target: 'toggle', before: 'false', requested: 'true', aria: 'aria-pressed', initialAria: 'false' },
        { name: 'toggle-group', target: 'toggle-group-b', before: '["a"]', requested: '["a", "b"]', aria: 'aria-pressed', initialAria: 'false' },
    ]) {
        test(`${choice.name}: ${acceptance} one keyboard request preserves application state and focus`, async ({ page }) => {
            await policy(page, choice.name, acceptance);
            const target = part(page, choice.target);
            await target.focus();
            await page.keyboard.press('Space');
            await expect.poll(() => snapshot(page, choice.name)).toEqual({
                value: accept ? choice.requested : choice.before, requests: [choice.requested], focus: choice.target, open: false,
            });
            await expect(target).toHaveAttribute(choice.aria, accept ? 'true' : choice.initialAria);
            if (['checkbox', 'checkbox-mixed', 'checkbox-group', 'radio-group', 'switch'].includes(choice.name)) {
                expect(await target.evaluate((node: HTMLInputElement) => node.checked)).toBe(accept);
            }
            if (choice.name === 'checkbox-mixed') {
                expect(await target.evaluate((node: HTMLInputElement) => node.indeterminate)).toBe(!accept);
            }
        });
    }
    for (const range of [false, true]) {
        test(`${range ? 'range' : 'slider'}: ${acceptance} one arrow request keeps named values and focus`, async ({ page }) => {
            const name = range ? 'range' : 'slider', target = range ? 'range-start' : 'slider';
            const values = (number: number) => range
                ? `[ThumbValue { id: "start", value: ${number}.0 }, ThumbValue { id: "end", value: 70.0 }]`
                : `[ThumbValue { id: "single", value: ${number}.0 }]`;
            await policy(page, name, acceptance);
            await part(page, target).focus();
            await page.keyboard.press('ArrowRight');
            await expect.poll(() => snapshot(page, name)).toEqual({ value: values(accept ? 21 : 20), requests: [values(21)], focus: target, open: false });
            await expect(part(page, target)).toHaveAttribute('aria-valuenow', accept ? '21' : '20');
            if (range) await expect(part(page, 'range-end')).toHaveAttribute('aria-valuenow', '70');
        });
    }
    test(`select: ${acceptance} selection emits one value request and closes to its trigger`, async ({ page }) => {
        await policy(page, 'select', acceptance);
        await part(page, 'select').focus();
        await page.keyboard.press('ArrowDown');
        await expect(part(page, 'select-content')).toBeVisible();
        await page.keyboard.press('ArrowDown');
        await activeOption(page, 'select-b');
        await page.keyboard.press('Enter');
        await expect.poll(() => snapshot(page, 'select')).toEqual({ value: accept ? 'Some("b")' : 'Some("a")', requests: ['Some("b")'], focus: 'select', open: false });
        await expect(part(page, 'select')).toHaveText(accept ? 'Banana' : 'Apricot');
        await expect(part(page, 'select-content')).toHaveCount(0);
        expect((await snapshot(page, 'select-open')).requests).toEqual(['true', 'false']);
    });
    test(`select: ${acceptance} opening emits one request and retains the application expanded state`, async ({ page }) => {
        await policy(page, 'select-open', acceptance);
        await part(page, 'select').focus();
        await page.keyboard.press('ArrowDown');
        await expect.poll(async () => (await snapshot(page, 'select-open')).requests).toEqual(['true']);
        await expect(part(page, 'select')).toHaveAttribute('aria-expanded', String(accept));
        await expect(part(page, 'select')).toBeFocused();
        if (accept) await activeOption(page, 'select-a');
        expect((await snapshot(page, 'select')).requests).toEqual([]);
        if (!accept) await expect(part(page, 'select-content')).toHaveCount(0);
    });
    test(`select: ${acceptance} closing records one request and follows the application open state`, async ({ page }) => {
        await part(page, 'select').focus();
        await page.keyboard.press('ArrowDown');
        await activeOption(page, 'select-a');
        await policy(page, 'select-open', acceptance);
        await page.keyboard.press('Escape');
        await expect.poll(async () => (await snapshot(page, 'select-open')).requests).toEqual(['true', 'false']);
        await expect(part(page, 'select')).toHaveAttribute('aria-expanded', String(!accept));
        await expect(part(page, 'select')).toBeFocused();
        if (!accept) await activeOption(page, 'select-a');
        if (accept) await expect(part(page, 'select-content')).toHaveCount(0);
        else await expect(part(page, 'select-content')).toBeVisible();
        expect((await snapshot(page, 'select')).requests).toEqual([]);
    });
    for (const field of [
        { name: 'text-field', before: 'Accepted', after: 'Changed', formName: 'title', numeric: false },
        { name: 'textarea', before: 'Notes', after: 'Revised notes', formName: 'notes', numeric: false },
        { name: 'number-field', before: '12', after: '15', formName: 'quantity', numeric: true },
        { name: 'color-field', before: '#112233', after: '#abcdef', formName: 'color', numeric: false },
    ]) {
        test(`${field.name}: ${acceptance} one edit commits the application value and native form entry`, async ({ page }) => {
            await policy(page, field.name, acceptance);
            const target = part(page, field.name);
            await target.fill(field.after);
            if (field.name === 'number-field' || field.name === 'color-field') await page.keyboard.press('Enter');
            const value = accept ? field.after : field.before;
            await expect.poll(() => snapshot(page, field.name)).toEqual({
                value: field.numeric ? value + '.0' : JSON.stringify(value),
                requests: [field.numeric ? field.after + '.0' : JSON.stringify(field.after)], focus: field.name, open: false,
            });
            // After editing ends, both the editor and native form entry must expose the accepted value.
            await part(page, 'outside').focus();
            await expect(target).toHaveValue(value);
            expect(await page.evaluate(name => (window as any).__parity.formData()[name], field.formName)).toBe(value);
            await expect(part(page, 'outside')).toBeFocused();
            expect((await snapshot(page, field.name)).requests).toHaveLength(1);
        });
    }
    test(`form and field: ${acceptance} native reset is a single application request`, async ({ page }) => {
        await part(page, 'text-field').fill('Changed');
        await policy(page, 'form', acceptance);
        await part(page, 'form-reset').focus();
        await page.keyboard.press('Enter');
        await expect.poll(() => snapshot(page, 'form')).toEqual({ value: String(accept), requests: ['true'], focus: 'form-reset', open: false });
        await expect(part(page, 'text-field')).toHaveValue(accept ? 'Accepted' : 'Changed');
        await expect(part(page, 'text-field')).toHaveAccessibleName('Title');
        await expect(part(page, 'text-field')).toHaveAccessibleDescription('Application controlled');
        expect(await page.evaluate(() => (window as any).__parity.formData().title)).toBe(accept ? 'Accepted' : 'Changed');
        await expect(part(page, 'form-submits')).toHaveText('0');
    });
    test(`hover card: ${acceptance} focus opening keeps focus on the native link`, async ({ page }) => {
        await policy(page, 'hover-card', acceptance);
        await part(page, 'hover-card').focus();
        await expect.poll(() => snapshot(page, 'hover-card')).toEqual({ value: String(accept), requests: ['true'], focus: 'hover-card', open: accept });
        if (accept) await expect(part(page, 'hover-card-content')).toBeVisible();
        else await expect(part(page, 'hover-card-content')).toHaveCount(0);
        await expect(part(page, 'hover-card')).toHaveAttribute('href', '#parity');
    });
}
