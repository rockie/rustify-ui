import { test, expect, waitForReady } from "./support";

test("theme sources distinguish boundaries and survive floating projection", async ({ page }) => {
    await waitForReady(page);
    const preview = page.getByTestId("studio-preview");
    await expect(preview).toHaveAttribute("data-rustify-theme-source", "root", { timeout: 2000 });
    const boundary = preview.getByTestId("local-boundary");
    await expect(boundary).toHaveAttribute("data-rustify-theme-source", /^boundary-\d+$/);
    const source = (await boundary.getAttribute("data-rustify-theme-source"))!;
    const sources = await page.locator("[data-rustify-theme-boundary]").evaluateAll(nodes => nodes.map(node => node.getAttribute("data-rustify-theme-source")));
    expect(new Set(sources).size).toBe(sources.length);
    await preview.getByTestId("open-preview-menu").click();
    await expect(preview.getByTestId("preview-menu")).toBeVisible();
    expect(await preview.getByTestId("preview-menu").evaluate(node => node.closest("[data-rustify-theme-source]")?.getAttribute("data-rustify-theme-source"))).toBe(source);
    await page.keyboard.press("Escape");
    await page.evaluate(() => window.__theme_studio.dispose());
    await expect(preview).not.toHaveAttribute("data-rustify-theme-source");
    await page.evaluate(() => window.__theme_studio.mount());
    await expect(preview).toHaveAttribute("data-rustify-theme-source", "root");
    expect(await preview.getByTestId("local-boundary").getAttribute("data-rustify-theme-source")).not.toBe(source);
});

test("picking opens floating content, resolves its local source, and consumes the first Escape", async ({ page }) => {
    await waitForReady(page);
    await page.getByTestId("renderer-dom").click();
    await page.getByTestId("inspector-toggle").click();
    const preview = page.getByTestId("studio-preview");
    await preview.getByTestId("open-preview-menu").click();
    const menu = preview.getByTestId("preview-menu");
    await expect(menu).toBeVisible();
    await menu.getByRole("menuitem").first().hover();
    await expect(page.getByTestId("inspector-source")).toContainText("Provider sample");
    await expect(page.getByTestId("inspector-inherited")).toContainText("Provider sample");
    await expect(page.getByTestId("inspector-boundary-note")).toContainText("editor changes the root theme");
    await expect(page.getByTestId("inspector-part")).toContainText("menu-item");
    await expect(page.getByTestId("inspector-panel").locator(".inspector-tokens").first().getByRole("button", { name: "primary", exact: true })).toBeVisible();
    await expect(page.getByTestId("inspector-contrast")).toContainText("primary-foreground / primary");
    await page.keyboard.press("Escape");
    await expect(page.getByTestId("inspector-panel")).toBeHidden();
    await expect(menu).toBeVisible();
    await page.keyboard.press("Escape");
    await expect(menu).toHaveCount(0);
});

test("part token discovery includes inactive rules and refreshes after stylesheet changes", async ({ page }) => {
    await waitForReady(page);
    await page.getByTestId("renderer-dom").click();
    await page.getByTestId("inspector-toggle").click();
    const preview = page.getByTestId("studio-preview");
    await preview.getByTestId("open-preview-menu").click();
    const item = preview.getByTestId("preview-menu").getByRole("menuitem").first();
    await item.hover();
    const buttons = page.getByTestId("inspector-panel").locator(".inspector-tokens").first();
    await expect(buttons.getByRole("button", { name: "chart-5", exact: true })).toHaveCount(0);
    await page.evaluate(() => {
        const sheet = new CSSStyleSheet();
        sheet.replaceSync('@layer gallery-test { @media (min-width: 9000px) { :where(.rustify-menu-item[data-state="closed"]:focus-visible)::before { color: var(--chart-5); background: var(--non-theme-token); } } }');
        document.adoptedStyleSheets = [...document.adoptedStyleSheets, sheet];
    });
    await item.dispatchEvent("mousemove");
    await expect(buttons.getByRole("button", { name: "chart-5", exact: true })).toBeVisible();
    await expect(buttons.getByRole("button", { name: "non-theme-token", exact: true })).toHaveCount(0);
    await page.evaluate(() => { document.adoptedStyleSheets = document.adoptedStyleSheets.slice(0, -1); });
    await page.keyboard.press("Escape");
    await page.keyboard.press("Escape");
});

async function openGallery(page: import('@playwright/test').Page) {
    await waitForReady(page);
    await page.getByTestId('renderer-dom').click();
    await page.getByTestId('preview-scene').click();
    await page.getByTestId('preview-scene-list').getByRole('option', { name: 'Components', exact: true }).click();
    const gallery = page.getByTestId('dom-preview').getByTestId('component-gallery');
    await expect(gallery).toBeVisible();
    return gallery;
}

test('component gallery groups real samples and keeps their state local', async ({ page }) => {
    const failures: string[] = [];
    page.on('pageerror', error => failures.push(error.message));
    const gallery = await openGallery(page);
    await expect(gallery.locator('[data-gallery-entry]')).toHaveCount(43);
    await expect(gallery.locator('.gallery-section')).toHaveCount(8);
    expect(await gallery.locator('[data-rustify-issue]').count()).toBe(0);
    const before = await page.evaluate(() => window.__theme_studio.snapshot());
    const buttons = gallery.locator('[data-gallery-entry="Button"]');
    await buttons.getByRole('button', { name: 'Create', exact: true }).click();
    await expect(buttons.locator('output')).toHaveText('1 created');
    await buttons.getByLabel('Variant', { exact: true }).selectOption({ label: 'outline' });
    await expect(buttons.getByRole('button', { name: 'Create', exact: true })).toHaveAttribute('data-variant', 'outline');
    await buttons.getByLabel('Disabled', { exact: true }).check();
    await expect(buttons.getByRole('button', { name: 'Create', exact: true })).toBeDisabled();
    const checkbox = gallery.locator('[data-gallery-entry="Checkbox"]');
    await checkbox.getByLabel('Receive updates').check();
    await expect(checkbox.getByLabel('Receive updates')).toBeChecked();
    await gallery.getByLabel('Show parts', { exact: true }).check();
    await expect(gallery).toHaveAttribute('data-show-parts', 'true');
    await checkbox.getByText('Structure & primitives', { exact: true }).click();
    await expect(checkbox.locator('.gallery-structure code')).toHaveText('rustify_components::primitives::checkbox');
    const after = await page.evaluate(() => window.__theme_studio.snapshot());
    expect(after.document).toEqual(before.document);
    expect(after.revision).toBe(before.revision);
    expect(failures).toEqual([]);
});

test('local contrast diagnostics locate the boundary frame and keep root editing separate', async ({ page }) => {
    const gallery = await openGallery(page);
    await gallery.getByLabel('Low contrast', { exact: true }).check();
    const diagnostics = page.getByTestId('preview-diagnostics');
    await expect(diagnostics).toContainText('Gallery local theme');
    const source = await gallery.getByTestId('gallery-boundary').getAttribute('data-rustify-theme-source');
    await diagnostics.locator(":scope > summary").click();
    const group = diagnostics.locator(`[data-diagnostic-source="${source}"]`);
    await expect(group).toContainText('card-foreground / card');
    await group.getByTestId('contrast-locate-card-foreground-on-card').click();
    await page.getByTestId('inspector-toggle').click();
    await gallery.getByTestId('gallery-local-card').hover();
    await expect(page.getByTestId('inspector-source')).toContainText('Gallery local theme');
    await expect(page.getByTestId('inspector-source')).toContainText('Boundary override');
    await expect(page.getByTestId('inspector-root-value')).toContainText('Root theme');
    await expect(page.getByTestId('inspector-contrast')).toContainText('Below 4.5:1');
    await page.getByTestId('locale-zh').click();
    await expect(page.getByTestId('inspector-source')).toContainText('陈列局部主题');
    await expect(page.getByTestId('inspector-boundary-note')).toContainText('编辑器修改根主题');
    await page.keyboard.press('Escape');
    await page.getByTestId('renderer-gpu').click();
    await expect(page.getByTestId('gallery-gpu-note')).toHaveText('组件陈列仅 DOM；GPU 显示通用主题样本。');
    await diagnostics.locator(':scope > summary').click();
});

test('gallery layout keeps samples and controls within desktop and narrow previews', async ({ page }) => {
    const gallery = await openGallery(page);
    const duplicates = await page.locator('[id]').evaluateAll(nodes => {
        const ids = nodes.map(node => node.id);
        return ids.filter((id,index) => ids.indexOf(id) !== index);
    });
    expect.soft(duplicates).toEqual([]);
    for (const width of [1440, 768, 390]) {
        await page.setViewportSize({ width, height: 900 });
        if (width === 390) await page.getByTestId('mobile-preview').click();
        await gallery.getByRole('heading', { name: 'Components', exact: true }).scrollIntoViewIfNeeded();
        expect.soft(await gallery.evaluate(node => node.scrollWidth - node.clientWidth)).toBeLessThanOrEqual(1);
        expect.soft(await gallery.locator('.gallery-sample').evaluateAll(nodes => nodes.filter(node => node.scrollWidth - node.clientWidth > 1).map(node=>({name:node.getAttribute("data-gallery-entry"),overflow:node.scrollWidth-node.clientWidth})))).toEqual([]);
        await page.screenshot({ path: `docs/validation/dom-components-p4/m5-gallery-${width}.png` });
    }
    await page.setViewportSize({ width: 1440, height: 900 });
});


test('gallery notifications name their own title and description across preview scopes', async ({ page }) => {
    const gallery = await openGallery(page);
    await gallery.locator('[data-gallery-entry="Toast"]').getByRole('button', { name: 'Show notification', exact: true }).click();
    const notice = page.getByTestId('studio-preview').locator('[data-rustify-part="toast-root"]');
    await expect(notice).toBeVisible();
    await expect(notice).toHaveAccessibleName('Draft saved');
    await expect(notice).toHaveAccessibleDescription('Your local preview is up to date.');
    await notice.getByRole('button', { name: 'dismiss this message', exact: true }).click();
    await expect(notice).toBeHidden();
});

test('gallery picking identifies menu palettes, native navigation, toolbar parts and context layers', async ({ page }) => {
    const gallery = await openGallery(page);
    const menu = gallery.locator('[data-gallery-entry="DropdownMenu"]');
    await menu.getByLabel('Palette', { exact: true }).selectOption({ label: 'Blue' });
    await page.getByTestId('inspector-toggle').click();
    await menu.locator('[aria-haspopup="menu"]').first().click();
    const surface = page.getByTestId('studio-preview').locator('[data-rustify-part="menu-content"]');
    await surface.getByRole('menuitem', { name: 'Duplicate', exact: true }).hover();
    await expect(page.getByTestId('inspector-part')).toContainText('menu-item');
    await expect(page.getByTestId('inspector-part')).toContainText('blue');
    await expect(page.getByTestId('inspector-part')).toContainText('9');
    await expect(page.getByTestId('inspector-inherited')).toContainText('Root theme');
    await page.keyboard.press('Escape');
    await page.keyboard.press('Escape');
    await page.getByTestId('inspector-toggle').click();
    const navigation = gallery.locator('[data-gallery-entry="NavigationMenu"]');
    await navigation.getByRole('link', { name: 'Reference', exact: true }).hover();
    await expect(page.getByTestId('inspector-part')).toContainText('navigation-menu-link');
    await expect(page.getByTestId('inspector-source')).toContainText('Root theme');
    await gallery.locator('[data-gallery-entry="Toolbar"]').getByRole('button', { name: 'Save', exact: true }).hover();
    await expect(page.getByTestId('inspector-part')).toContainText('toolbar-button');
    const context = gallery.locator('[data-gallery-entry="ContextMenu"]');
    await context.locator('[data-rustify-part="context-menu-trigger"]').click({ button: 'right' });
    await surface.getByRole('menuitem', { name: 'Copy', exact: true }).hover();
    await expect(page.getByTestId('inspector-inherited')).toContainText('Root theme');
    await page.keyboard.press('Escape');
    await expect(surface).toBeVisible();
    await page.keyboard.press('Escape');
    await expect(surface).toHaveCount(0);
});

for (const complete of [false, true]) {
    for (const mode of ['light', 'dark'] as const) {
        test(`gallery matrix ${complete ? 'complete' : 'representative'} in ${mode}`, complete ? { tag: '@evidence' } : {}, async ({ page }, info) => {
            const { appearance, openSurface, closeSurface } = await import('./gallery-matrix');
            const { mkdir, writeFile } = await import('node:fs/promises');
            test.setTimeout(600_000);
            const gallery = await openGallery(page);
            await page.getByTestId(`mode-${mode}`).click();
            const failures: string[] = [];
            page.on('pageerror', error => failures.push(error.message));
            const all = await gallery.locator('[data-gallery-entry]').evaluateAll(nodes => nodes.map(node => node.getAttribute('data-gallery-entry')!));
            const names = complete ? all : ['Button', 'Checkbox', 'Dialog', 'DropdownMenu', 'NavigationMenu', 'Toolbar', 'Toast', 'Text'];
            const rows: unknown[] = [];
            const checkText = (texts: Awaited<ReturnType<typeof appearance>>['text'], label: string) => {
                for (const text of texts) {
                    if (text.reference) {
                        expect.soft(text.reference.matches, `${label}: ${text.part} ${text.reference.rule}`).toBe(true);
                    } else {
                        expect.soft(text.ratio, `${label}: ${text.part} ${text.text}`).toBeGreaterThanOrEqual(4.5);
                    }
                }
            };
            for (const name of names) {
                await test.step(name, async () => {
                    const sample = gallery.locator(`[data-gallery-entry="${name}"]`);
                    await sample.scrollIntoViewIfNeeded();
                    const inspect = async (state: string) => {
                        const surface = await openSurface(page, sample, name);
                        const result = await appearance(surface);
                        if (name === 'Select') {
                            const trigger = await appearance(sample.locator('[data-rustify-part="select-trigger"]'));
                            result.visual += trigger.visual;
                            result.geometry += trigger.geometry;
                            result.text.push(...trigger.text);
                            result.edges.push(...trigger.edges);
                        }
                        rows.push({ name, state, text: result.text, edges: result.edges });
                        if (complete || state === 'default') {
                            await surface.screenshot({ path: `docs/validation/dom-components-p4/gallery-${complete ? 'complete' : 'representative'}-${mode}-${name}-${state.replace(/[^a-z0-9]+/gi, '-')}.png`, animations: 'disabled' });
                        }
                        checkText(result.text, `${name} ${state}`);
                        for (const edge of result.edges) expect.soft(edge.ratio, `${name} ${state}: ${edge.kind}`).toBeGreaterThanOrEqual(3);
                        if (['DropdownMenu', 'ContextMenu', 'Menubar'].includes(name)) {
                            await surface.getByRole('menuitem', { exact: true }).first().hover();
                            const highlighted = await appearance(surface);
                            rows.push({ name, state: `${state} highlighted`, text: highlighted.text });
                            checkText(highlighted.text, `${name} ${state} highlighted`);
                            result.visual += highlighted.visual;
                        }
                        return result;
                    };
                    await inspect('default');
                    await closeSurface(page, name);
                    const checked = sample.locator('.gallery-demo input[type="checkbox"]:not(.gallery-control input)').first();
                    if (await checked.count()) { await checked.check(); await inspect('checked'); await closeSurface(page, name); }
                    const stateTarget: Record<string, string> = {
                        Toggle: 'button[data-rustify-part]', ToggleGroup: 'button[data-rustify-part]',
                        Tabs: '[role="tab"]', Accordion: '[aria-expanded]', Collapsible: '[aria-expanded]',
                    };
                    if (stateTarget[name]) {
                        const target = sample.locator(stateTarget[name]).last();
                        const before = await appearance(sample.locator('.gallery-demo'));
                        await target.click();
                        const after = await inspect('selected or expanded');
                        expect.soft(after.visual, `${name} accepted state`).not.toEqual(before.visual);
                    }
                    const controls = sample.locator('.gallery-control select').filter({ has: page.locator('option') });
                    for (let index = 0; index < await controls.count(); index++) {
                        const control = controls.nth(index);
                        const label = (await control.getAttribute('aria-label'))!;
                        if (label === 'Palette') continue;
                        const initial = await control.inputValue();
                        const options = await control.locator('option').evaluateAll(nodes => nodes.map(node => ({ value: (node as HTMLOptionElement).value, label: node.textContent! })));
                        const signatures: { label: string, value: string }[] = [];
                        for (const option of complete ? options : options.slice(0, 2)) {
                            await control.selectOption(option.value);
                            const result = await inspect(`${label}=${option.label}`);
                            signatures.push({ label: option.label, value: /variant/i.test(label) ? result.visual + result.geometry : result.geometry });
                            await closeSurface(page, name);
                        }
                        expect.soft(new Set(signatures.map(row => row.value)).size, `${name} ${label}: declared appearances must differ (${signatures.map(row => row.label).join(', ')})`).toBe(signatures.length);
                        await control.selectOption(initial);
                    }
                    for (const label of ['Disabled', 'Loading', 'Indeterminate', 'Empty']) {
                        const toggle = sample.getByLabel(label, { exact: true });
                        if (!await toggle.count()) continue;
                        const initial = await toggle.isChecked();
                        const before = await appearance(sample.locator('.gallery-demo'));
                        await toggle.setChecked(!initial);
                        const after = await appearance(sample.locator('.gallery-demo'));
                        expect.soft(after.visual, `${name} ${label} changes rendered state`).not.toEqual(before.visual);
                        rows.push({ name, state: label, text: after.text });
                        if (label === 'Empty') await expect(sample.getByRole('button', { name: /Restore/ })).toBeVisible();
                        await toggle.setChecked(initial);
                    }
                    const palette = sample.getByLabel('Palette', { exact: true });
                    if (await palette.count()) {
                        await palette.selectOption({ label: 'Blue' });
                        await inspect('local Blue'); await closeSurface(page, name);
                        await sample.getByLabel('High contrast', { exact: true }).check();
                        await inspect('local Blue high contrast'); await closeSurface(page, name);
                        await sample.getByLabel('High contrast', { exact: true }).uncheck();
                        await palette.selectOption({ label: 'Inherit' });
                    }
                    if (['DropdownMenu', 'ContextMenu', 'Menubar', 'NavigationMenu', 'Toolbar'].includes(name)) {
                        const surface = await openSurface(page, sample, name);
                        const target = name === 'NavigationMenu' ? surface.locator('.rustify-navigation-menu')
                            : name === 'Toolbar' ? surface.locator('.rustify-toolbar') : surface;
                        const before = await target.evaluate(node => getComputedStyle(node).borderRadius);
                        await target.evaluate(node => node.classList.add('rounded-sm'));
                        const radius = await target.evaluate(node => ({ actual: getComputedStyle(node).borderRadius,
                            expected: getComputedStyle(node).getPropertyValue('--rustify-radius-sm').trim() }));
                        expect.soft(radius.actual, `${name} caller utility`).toEqual(radius.expected);
                        expect.soft(radius.actual, `${name} caller utility changes the skin`).not.toEqual(before);
                        await inspect('caller rounded-sm');
                        await target.evaluate(node => node.classList.remove('rounded-sm'));
                        await closeSurface(page, name);
                    }
                    const target = sample.locator('.gallery-demo :is(button:not(:disabled),a[href],input:not(:disabled),textarea:not(:disabled),[tabindex="0"]):not(.gallery-controls *, .gallery-control:has(select,input[type="checkbox"]) *)').filter({ visible: true }).first();
                    if (await target.count() && (complete || ['Button', 'NavigationMenu', 'Toolbar', 'Text'].includes(name))) {
                        const control = target;
                        await page.mouse.move(0, 0);
                        await page.getByTestId('renderer-dom').focus();
                        const rest = await appearance(sample.locator('.gallery-demo'));
                        if (['Button', 'Toggle', 'NavigationMenu', 'Toolbar', 'Text'].includes(name)) {
                            await control.hover();
                            expect.soft((await appearance(sample.locator('.gallery-demo'))).visual, `${name} hover`).not.toEqual(rest.visual);
                            await page.mouse.move(0, 0);
                        }
                        await page.keyboard.press('Shift');
                        await control.focus();
                        const focused = await appearance(sample.locator('.gallery-demo'));
                        expect.soft(focused.visual, `${name} focus`).not.toEqual(rest.visual);
                        for (const edge of focused.edges) expect.soft(edge.ratio, `${name} ${edge.kind}`).toBeGreaterThanOrEqual(3);
                        rows.push({ name, state: 'focus', text: focused.text, edges: focused.edges });
                        await closeSurface(page, name);
                    }
                    if (name === 'Form') {
                        await sample.getByRole('button', { name: 'Save locally', exact: true }).click();
                        await expect(sample.locator('[aria-invalid="true"]').first()).toBeVisible();
                        await inspect('invalid');
                        await sample.getByRole('textbox', { name: 'Name', exact: true }).fill('Ada');
                        await sample.getByRole('button', { name: 'Save locally', exact: true }).click();
                        await expect(sample.getByRole('status')).toContainText('Saved in this sample.');
                    }
                    await sample.screenshot({ path: `docs/validation/dom-components-p4/gallery-${complete ? 'complete' : 'representative'}-${mode}-${name}.png`, animations: 'disabled' });
                });
            }
            expect(failures).toEqual([]);
            await mkdir('docs/validation/dom-components-p4', { recursive: true });
            await writeFile(`docs/validation/dom-components-p4/gallery-${complete ? 'complete' : 'representative'}-${mode}.json`, JSON.stringify(rows, null, 2) + '\n');
            await info.attach('gallery-contrast.json', { body: JSON.stringify(rows), contentType: 'application/json' });
        });
    }
}
