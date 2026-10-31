import { test, expect, waitForReady } from './support';
import type { Page } from '@playwright/test';
import { writeFile } from 'node:fs/promises';

const sample = (page: Page) => page.getByTestId('dom-preview').getByTestId('form-samples');
const failures = new WeakMap<Page, string[]>();
test.beforeEach(async ({ page }) => {
    const errors: string[] = []; failures.set(page, errors);
    page.on('pageerror', error => errors.push(error.message));
    page.on('console', message => { if (message.type() === 'error') errors.push(message.text()); });
    page.on('requestfailed', request => errors.push(`${request.url()}: ${request.failure()?.errorText}`));
    await page.setViewportSize({ width: 1440, height: 1000 });
    await waitForReady(page);
    await page.getByTestId('locale-en').click();
    await page.getByTestId('renderer-dom').click();
    await page.getByTestId('preview-scene').click();
    await page.getByTestId('preview-scene-list').getByRole('option', { name: 'Cards', exact: true }).click();
    await expect(sample(page)).toBeVisible();
});
test.afterEach(({ page }) => expect(failures.get(page)).toEqual([]));

async function save(page: Page) {
    const button = sample(page).getByTestId('sample-save');
    await button.focus();
    await button.press('Enter');
}

async function region(page: Page) {
    const trigger = sample(page).getByTestId('sample-region');
    await trigger.click();
    const id = await trigger.getAttribute('aria-controls');
    await page.locator(`#${id}`).getByRole('option', { name: 'East Asia', exact: true }).click();
}

test('form examples associate visible guidance, error sources and incomplete drafts', async ({ page }) => {
    const root = sample(page);
    await save(page);
    await expect(root.getByTestId('sample-name')).toHaveAccessibleDescription(/Input requirement/);
    await root.getByTestId('sample-name').fill('A');
    await save(page);
    await expect(root.getByTestId('sample-name')).toHaveAccessibleDescription(/Form rule/);
    await root.getByTestId('sample-amount').fill('-');
    await expect(root.getByTestId('sample-amount')).toHaveAccessibleDescription(/Number incomplete/);
    expect(await root.getByTestId('sample-form').evaluate(node => new FormData(node as HTMLFormElement).get('amount'))).toBe('12');
    await root.getByTestId('sample-amount').press('Escape');
    await expect(root.getByTestId('sample-amount')).toHaveValue('12');
    await expect(root.getByTestId('sample-amount-draft-help')).toBeHidden();
    await expect(root.getByTestId('sample-amount')).not.toHaveAccessibleDescription(/Number incomplete/);
    for (const finish of ['Enter', 'submit']) {
        await root.getByTestId('sample-amount').fill('-');
        if (finish === 'Enter') await root.getByTestId('sample-amount').press('Enter');
        else await root.getByTestId('sample-form').evaluate(node => (node as HTMLFormElement).requestSubmit());
        await expect(root.getByTestId('sample-amount')).toHaveValue('12');
        await expect(root.getByTestId('sample-amount-draft-help')).toBeHidden();
        await expect(root.getByTestId('sample-amount')).not.toHaveAccessibleDescription(/Number incomplete/);
    }
    await root.getByTestId('sample-amount').fill('-');
    await root.getByTestId('form-states-details').locator('summary').click();
    await expect(root.getByTestId('parts-component')).toBeVisible();
    await expect(root.getByTestId('sample-amount')).toHaveValue('12');
    await expect(root.getByTestId('sample-amount-draft-help')).toBeHidden();
});

test('state examples associate visible labels and help in both presentations', async ({ page }) => {
    const root = sample(page);
    await root.getByTestId('form-states-details').locator('summary').click();
    const tool = root.getByTestId('form-states');
    for (const presentation of ['themes', 'primitives']) {
        await tool.getByTestId('parts-presentation').selectOption(presentation);
        for (const [part, label] of [['Checkbox', 'Receive email updates'], ['Switch', 'Enable notifications'], ['TextField', 'Display name'], ['TextArea', 'Notes'], ['NumberField', 'Quantity'], ['ColorField', 'Colour']]) {
            await tool.getByTestId('parts-component').selectOption(part);
            await expect(tool.getByTestId('parts-label')).toHaveText(label);
            await expect(tool.getByTestId('parts-control')).toHaveAccessibleName(label);
            await expect(tool.getByTestId('parts-control')).toHaveAccessibleDescription(/.+/);
        }
        await tool.getByTestId('parts-state').selectOption('invalid');
        await expect(tool.getByTestId('parts-control')).toHaveAccessibleDescription(/Invalid appearance demonstration/);
        await tool.getByTestId('parts-component').selectOption('CheckboxCards');
        await expect(tool.getByTestId('parts-example').getByRole('checkbox').first()).toHaveAccessibleDescription(/Invalid appearance demonstration/);
        await tool.getByTestId('parts-state').selectOption('normal');
        await tool.getByTestId('parts-component').selectOption('ToggleGroup');
        await expect(tool.getByTestId('parts-example').getByRole('button', { name: 'Email', exact: true })).toHaveAccessibleDescription(/including none/);
    }
});

test('themed range endpoints and soft switch retain their visible boundaries', async ({ page }) => {
    await sample(page).getByTestId('form-states-details').locator('summary').click();
    const tool = sample(page).getByTestId('form-states');
    await tool.getByTestId('parts-component').selectOption('RangeSlider');
    for (const rtl of [false, true]) {
        await tool.getByTestId('parts-rtl').setChecked(rtl);
        for (const key of ['End', 'Home']) {
            const thumb = tool.getByTestId('parts-start');
            await thumb.focus(); await thumb.press(key);
            await expect(thumb).toBeFocused();
            expect(await thumb.evaluate(node => {
                const box = node.getBoundingClientRect();
                let parent = node.parentElement;
                while (parent) {
                    const css = getComputedStyle(parent), clip = parent.getBoundingClientRect();
                    if (css.overflowX !== 'visible' && (box.left < clip.left || box.right > clip.right)) return false;
                    parent = parent.parentElement;
                }
                return box.width === box.height;
            })).toBe(true);
        }
    }
    await tool.getByTestId('parts-component').selectOption('Switch');
    await tool.getByTestId('parts-variant').selectOption('soft');
    await expect(tool.getByTestId('parts-control')).not.toHaveCSS('border-color', 'rgba(0, 0, 0, 0)');
});

test('complete local form exposes constraints, grouped values, Draft completion and application retry', async ({ page }) => {
    const form = sample(page).getByTestId('sample-form');
    const name = form.getByTestId('sample-name');
    const revision = await page.evaluate(() => window.__theme_studio.snapshot().revision);
    await save(page);
    await expect(name).toBeFocused();
    await expect(form.getByTestId('sample-name-error')).toHaveAttribute('data-source', 'constraint');
    await name.fill('A');
    await save(page);
    await expect(form.getByTestId('sample-name-error')).toHaveAttribute('data-source', 'rule');
    await expect(form.getByTestId('sample-name-error')).toHaveText('Use at least two characters.');
    await name.fill('Avery');
    await form.getByRole('checkbox', { name: 'Guides', exact: true }).check();
    await form.getByRole('checkbox', { name: 'Activity feed', exact: true }).check();
    await form.getByRole('radio', { name: /Team/ }).check();
    await region(page);
    await form.getByTestId('sample-consent').check();
    const amount = form.getByTestId('sample-amount');
    await amount.fill('17.5');
    expect(await form.evaluate(node => new FormData(node as HTMLFormElement).get('amount'))).toBe('12');
    await save(page);
    await expect(form.getByTestId('sample-save-status')).toHaveText('Saved in this preview only · 1');
    await expect(amount).toHaveValue('17.5');
    const native = await form.evaluate(node => [...new FormData(node as HTMLFormElement).entries()]);
    expect(native).toEqual(expect.arrayContaining([['name', 'Avery'], ['topics', 'updates'], ['topics', 'guides'], ['channels', 'mail'], ['channels', 'feed'], ['plan', 'team'], ['amount', '17.5'], ['region', 'east'], ['consent', 'on']]));
    const data = form.getByTestId('sample-form-data');
    expect(JSON.parse((await data.textContent())!)).toEqual(native);
    await form.getByTestId('sample-fail').click();
    await save(page);
    await expect(form.getByTestId('sample-name-error')).toHaveAttribute('data-source', 'application');
    await expect(name).toBeFocused();
    await name.fill('Avery again');
    await save(page);
    await expect(form.getByTestId('sample-save-status')).toHaveText('Saved in this preview only · 3');
    await form.getByTestId('sample-reset').click();
    await expect(name).toHaveValue('');
    await expect(form.getByTestId('sample-consent')).toHaveAttribute('aria-checked', 'mixed');
    await expect(form.getByTestId('sample-region')).toHaveText('Choose a region');
    expect(await page.evaluate(() => window.__theme_studio.snapshot().revision)).toBe(revision);
});

test('states tool separates refused and delayed requests and exercises native selection cancellation', async ({ page }) => {
    const root = sample(page);
    await root.getByTestId('form-states-details').locator('summary').click();
    const tool = root.getByTestId('form-states');
    await tool.getByTestId('parts-policy').selectOption('refuse');
    await tool.getByTestId('parts-control').click();
    await expect(tool.getByTestId('parts-control')).toHaveAttribute('aria-checked', 'mixed');
    await expect(tool.getByTestId('parts-request')).toHaveText('Checked');
    await expect(tool.getByTestId('parts-accepted')).toHaveText('Indeterminate');
    await tool.getByTestId('parts-policy').selectOption('delay');
    await tool.getByTestId('parts-control').click();
    await expect(tool.getByTestId('parts-count')).toHaveText('2');
    await tool.getByTestId('parts-apply').click();
    await expect(tool.getByTestId('parts-control')).toBeChecked();
    await expect(tool.getByTestId('parts-accepted')).toHaveText('Checked');
    await tool.getByTestId('parts-presentation').selectOption('primitives');
    await expect(tool.getByTestId('parts-control')).not.toHaveClass(/rustify-choice-native/);
    await tool.getByTestId('parts-component').selectOption('Select');
    await expect(tool.getByTestId('parts-count')).toHaveText('0');
    await expect(tool.getByTestId('parts-request')).toHaveText('—');
    await tool.getByTestId('parts-cancel').check();
    await tool.getByTestId('parts-control').click();
    await page.getByTestId('parts-select-list').getByRole('option', { name: 'Activity feed', exact: true }).click();
    await expect(tool.getByTestId('parts-count')).toHaveText('0');
    await expect(page.getByTestId('parts-select-list')).toBeVisible();
    await tool.getByTestId('parts-cancel').uncheck();
    await tool.getByTestId('parts-control').click();
    await page.getByTestId('parts-select-list').getByRole('option', { name: 'Activity feed', exact: true }).click();
    await expect(tool.getByTestId('parts-request')).toHaveText('"b"');
    await expect(tool.getByTestId('parts-accepted')).toHaveText('"a"');
    await tool.getByTestId('parts-apply').click();
    await expect(tool.getByTestId('parts-accepted')).toHaveText('"b"');
    await expect(tool.getByTestId('parts-control')).toContainText('Activity feed');
});

test('all public form examples mount in both presentations and retain authored input through language and mode changes', async ({ page }) => {
    const root = sample(page);
    await root.getByTestId('form-states-details').locator('summary').click();
    const tool = root.getByTestId('form-states');
    const parts = ['Checkbox', 'Switch', 'Toggle', 'RadioGroup', 'CheckboxGroup', 'ToggleGroup', 'RadioCards', 'CheckboxCards', 'Slider', 'RangeSlider', 'TextField', 'TextArea', 'NumberField', 'ColorField', 'Select'];
    for (const mode of ['themes', 'primitives']) {
        await tool.getByTestId('parts-presentation').selectOption(mode);
        for (const part of parts) {
            await tool.getByTestId('parts-component').selectOption(part);
            await expect(tool.getByTestId('parts-control')).toBeVisible();
            if (part === 'RangeSlider') {
                await expect(tool.getByRole('slider')).toHaveCount(2);
            }
        }
    }
    await root.getByTestId('sample-name').fill('保留 Avery');
    await page.getByTestId('locale-zh').click();
    await expect(root.getByTestId('sample-name')).toHaveValue('保留 Avery');
    await expect(root.getByTestId('sample-region')).toHaveText('请选择地区');
    await page.getByTestId('mode-dark').click();
    await expect(root.getByTestId('sample-name')).toHaveValue('保留 Avery');
    await page.setViewportSize({ width: 390, height: 844 });
    await page.getByTestId('mobile-preview').click();
    await root.getByTestId('sample-name').scrollIntoViewIfNeeded();
    expect(await root.evaluate(node => node.scrollWidth <= node.clientWidth)).toBe(true);
    await page.screenshot({ path: 'docs/validation/dom-components-p2/m5-form-mobile.png', animations: 'disabled' });
    // A narrower layout probes reflow; it does not claim browser UI zoom.
    await page.setViewportSize({ width: 720, height: 450 });
    await root.getByTestId('sample-name').scrollIntoViewIfNeeded();
    expect(await root.evaluate(node => node.scrollWidth <= node.clientWidth)).toBe(true);
});

test('themes form matrix renders every declared size and treatment with local styles and state', async ({ page }, info) => {
    const root = sample(page);
    await root.getByTestId('form-states-details').locator('summary').click();
    const tool = root.getByTestId('form-states');
    const observations: unknown[] = [];
    const controls = ['Checkbox', 'Switch', 'RadioGroup', 'CheckboxGroup', 'RadioCards', 'CheckboxCards', 'Slider', 'RangeSlider', 'TextField', 'TextArea', 'NumberField', 'ColorField', 'Select'];
    for (const part of controls) {
        await tool.getByTestId('parts-component').selectOption(part);
        const variants = await tool.getByTestId('parts-variant').locator('option').evaluateAll(nodes => nodes.map(node => (node as HTMLOptionElement).value));
        for (const size of ['1', '2', '3']) {
            await tool.getByTestId('parts-size').selectOption(size);
            for (const variant of variants) {
                await tool.getByTestId('parts-variant').selectOption(variant);
                const control = tool.getByTestId('parts-control');
                await expect(control).toBeVisible();
                observations.push(await control.evaluate((node, setting) => {
                    const boundary = node.closest('[data-name]') ?? node;
                    const css = getComputedStyle(boundary); const box = boundary.getBoundingClientRect();
                    return { ...setting, width: box.width, height: box.height, background: css.backgroundColor, border: css.borderColor, radius: css.borderRadius };
                }, { part, size, variant }));
            }
        }
        await tool.getByTestId('parts-color').selectOption('red');
        await tool.getByTestId('parts-state').selectOption('disabled');
        await expect(tool.getByTestId('parts-control')).toHaveAttribute('data-disabled', '');
        await tool.getByTestId('parts-state').selectOption('invalid');
        await expect.poll(() => tool.getByTestId('parts-control').evaluate(node => node.matches('[aria-invalid="true"], [data-invalid]') || !!node.querySelector('[aria-invalid="true"], [data-invalid]'))).toBe(true);
        await tool.getByTestId('parts-state').selectOption('normal');
    }
    await tool.getByTestId('parts-radius').selectOption('none');
    await expect(tool.getByTestId('parts-control')).toHaveCSS('border-radius', '0px');
    await tool.getByTestId('parts-radius').selectOption('full');
    await tool.getByTestId('parts-contrast').check();
    await tool.getByTestId('parts-rtl').check();
    await tool.getByTestId('parts-control').click();
    const list = page.getByTestId('parts-select-list');
    await expect(list).toHaveAttribute('dir', 'rtl');
    await tool.getByTestId('parts-content-variant').selectOption('soft');
    await expect(list).toHaveAttribute('data-variant', 'soft');
    await tool.getByTestId('parts-control').press('Escape');
    const name = root.getByTestId('sample-name');
    await name.fill('Scale check');
    const height = () => name.evaluate(node => node.closest('.rustify-field')!.getBoundingClientRect().height);
    const originalHeight = await height();
    await page.getByTestId('studio-config-scaling').click();
    await page.getByRole('option', { name: '110%', exact: true }).click();
    await expect.poll(height).toBeGreaterThan(originalHeight);
    await expect(name).toHaveValue('Scale check');
    await tool.screenshot({ path: 'docs/validation/dom-components-p2/m5-parts-desktop.png', animations: 'disabled' });
    await page.setViewportSize({ width: 768, height: 1000 });
    expect(await root.evaluate(node => node.scrollWidth <= node.clientWidth)).toBe(true);
    await root.getByTestId('sample-name').scrollIntoViewIfNeeded();
    await page.screenshot({ path: 'docs/validation/dom-components-p2/m5-form-tablet.png', animations: 'disabled' });
    await info.attach('form-themes-matrix.json', { body: JSON.stringify(observations, null, 2), contentType: 'application/json' });
    await writeFile('docs/validation/dom-components-p2/m5-form-matrix.json', JSON.stringify(observations, null, 2) + '\n');
});

test('default form and shell expose measured text, control edges and keyboard focus contrast', async ({ page }, info) => {
    const form = sample(page);
    const name = form.getByTestId('sample-name');
    const observations: unknown[] = [];
    for (const mode of ['light', 'dark']) {
        await page.getByTestId(`mode-${mode}`).click();
        await form.getByTestId('sample-consent').click();
        const choice = form.getByTestId('sample-consent');
        await choice.click();
        const boundary = await choice.evaluate(node => {
            const probe = document.createElement('span');
            probe.style.color = getComputedStyle(node).getPropertyValue('--rustify-choice-boundary');
            node.parentElement!.append(probe);
            const color = getComputedStyle(probe).color;
            probe.remove();
            return color;
        });
        await expect(choice).toHaveCSS('border-top-color', boundary);
        await name.focus();
        await page.keyboard.press('Tab');
        await page.keyboard.press('Shift+Tab');
        await expect(name).toBeFocused();
        await expect.poll(() => name.evaluate(node => node.matches(':focus-visible') && parseFloat(getComputedStyle(node.closest('.rustify-field')!).outlineWidth) >= 2)).toBe(true);
        const rows = await page.evaluate(() => {
            const canvas = document.createElement('canvas'); canvas.width=1;canvas.height=1;
            const ctx = canvas.getContext('2d')!;
            const rgba = (value: string) => { ctx.clearRect(0,0,1,1);ctx.fillStyle=value;ctx.fillRect(0,0,1,1);return [...ctx.getImageData(0,0,1,1).data].map(value=>value/255); };
            const over = (front: number[], back: number[]) => [0,1,2].map(at=>front[at]*front[3]+back[at]*(1-front[3])).concat(1);
            const background = (element: Element) => {
                const ancestors: Element[]=[];for(let node:Element|null=element;node;node=node.parentElement)ancestors.unshift(node);
                return ancestors.reduce((color,node)=>over(rgba(getComputedStyle(node).backgroundColor),color),[1,1,1,1]);
            };
            const luminance = (color: number[]) => color.slice(0,3).map(channel=>channel<=.04045?channel/12.92:((channel+.055)/1.055)**2.4).reduce((sum,channel,index)=>sum+channel*[.2126,.7152,.0722][index],0);
            const ratio = (a:number[],b:number[]) => {const x=luminance(a),y=luminance(b);return(Math.max(x,y)+.05)/(Math.min(x,y)+.05);};
            const input=document.querySelector('[data-testid="dom-preview"] [data-testid="sample-name"]')!;
            const field=input.closest('.rustify-field')!;
            const choice=document.querySelector('[data-testid="dom-preview"] [data-testid="sample-consent"]')!;
            const shell=document.querySelector('[data-testid="locale-en"]')!;
            const probes = [
                {name:'shell text',element:shell,color:getComputedStyle(shell).color,minimum:4.5},
                {name:'field text',element:input,color:getComputedStyle(input).color,minimum:4.5},
                {name:'field boundary',element:field,color:getComputedStyle(field).getPropertyValue('--rustify-field-edge'),minimum:3},
                {name:'field keyboard focus',element:field,color:getComputedStyle(field).outlineColor,minimum:3},
                {name:'unchecked choice boundary',element:choice.parentElement!,color:getComputedStyle(choice).borderColor,minimum:3},
            ];
            return probes.map(probe=>({...probe,element:undefined,ratio:ratio(over(rgba(probe.color),background(probe.element)),background(probe.element))}));
        });
        await info.attach('default-form-contrast.json', { body: JSON.stringify({mode,rows},null,2),contentType:'application/json' });
        observations.push({ mode, rows });
        for (const row of rows) expect(row.ratio, `${mode} ${row.name}`).toBeGreaterThanOrEqual(row.minimum);
    }
    await writeFile('docs/validation/dom-components-p2/m5-form-contrast.json', JSON.stringify(observations, null, 2) + '\n');
});
