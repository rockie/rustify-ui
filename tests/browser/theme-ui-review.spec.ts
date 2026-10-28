import { test, expect, waitForReady } from "./support";

test("picking an unannotated control explains the missing mapping without activating it", async ({ page }) => {
    await waitForReady(page);
    await page.getByTestId("renderer-dom").click();
    const create = page.getByTestId("studio-preview").getByTestId("preview-primary");
    const before = await create.textContent();
    await page.getByTestId("inspector-toggle").click();
    await create.click();
    await expect(create).toHaveText(before!);
    await expect(page.getByTestId("inspector-unannotated")).toBeVisible();
    await expect(page.getByTestId("inspector-outline")).toHaveAttribute("data-annotated", "false");
    await page.keyboard.press("Escape");
    await create.click();
    await expect(create).not.toHaveText(before!);
});

test("picking a declared action follows its own bounds and exposes its editor source", async ({ page }) => {
    await waitForReady(page);
    await page.getByTestId("renderer-dom").click();
    await page.getByTestId("inspector-toggle").click();
    const button = page.getByTestId("studio-preview").getByTestId("cards-submit");
    await button.click();
    const outline = page.getByTestId("inspector-outline");
    await expect(page.getByTestId("inspector-panel").locator(".inspector-tokens").first().getByRole("button", { name: "primary-foreground", exact: true })).toBeVisible();
    await expect.poll(async () => {
        const [control, selection] = await Promise.all([button.boundingBox(), outline.boundingBox()]);
        return control && selection ? Math.abs(control.width - selection.width) : Infinity;
    }).toBeLessThan(2);
    await expect(page.getByTestId("source-token")).toHaveText("primary");
    await expect(page.getByTestId("token-source")).toBeInViewport();
    await expect(page.getByTestId("token-source").getByRole("heading")).toBeInViewport({ ratio: 1 });
    await expect(page.getByTestId("inspector-source")).toContainText("primary");
});

test("a local opposite appearance and its dialog preserve the root theme", async ({ page }) => {
    await page.setViewportSize({ width: 390, height: 844 });
    await waitForReady(page);
    await page.getByTestId("mobile-preview").click();
    await page.getByTestId("renderer-dom").click();
    const preview = page.getByTestId("studio-preview");
    await preview.getByTestId("local-appearance").click();
    await preview.getByTestId("local-appearance-list").getByRole("option", { name: "Opposite to root", exact: true }).click();
    await expect(preview).toHaveAttribute("data-theme", "light");
    const local = preview.getByTestId("local-boundary");
    await expect(local).toHaveAttribute("data-theme", "dark");
    await preview.getByTestId("open-preview-dialog").click();
    const layer = preview.getByTestId("preview-dialog");
    await expect(layer).toHaveAttribute("aria-modal", "true");
    const box = await layer.locator('[data-name="Dialog"]').boundingBox();
    expect(box!.x).toBeGreaterThanOrEqual(15);
    expect(box!.x + box!.width).toBeLessThanOrEqual(375);
    const confirm = page.getByTestId("dialog-confirm");
    await expect(confirm).toHaveCSS("background-color", "rgb(83, 177, 253)");
    await expect(confirm).toHaveCSS("color", "rgb(12, 17, 29)");
    expect((await confirm.boundingBox())!.height).toBe(32);
    await page.keyboard.press("Escape");
    await expect(preview.getByTestId("open-preview-dialog")).toBeFocused();
    await preview.getByTestId("local-appearance").click();
    await preview.getByTestId("local-appearance-list").getByRole("option", { name: "Inherit root", exact: true }).click();
    await preview.getByTestId("open-preview-dialog").click();
    await expect(confirm).toHaveCSS("background-color", "rgb(21, 112, 239)");
    await expect(confirm).toHaveCSS("color", "rgb(255, 255, 255)");
    expect((await confirm.boundingBox())!.height).toBe(32);
    await page.keyboard.press("Escape");
    await preview.getByTestId("local-appearance").click();
    await preview.getByTestId("local-appearance-list").getByRole("option", { name: "Opposite to root", exact: true }).click();
    await page.getByTestId("mobile-edit").click();
    await page.getByTestId("mode-dark").click();
    await page.getByTestId("mobile-preview").click();
    await expect(preview).toHaveAttribute("data-theme", "dark");
    await expect(local).toHaveAttribute("data-theme", "light");
});

test("invalid numeric text stays visible while the valid theme remains unchanged", async ({ page }) => {
    await waitForReady(page);
    await page.getByTestId("editor-controls").click();
    const before = await page.evaluate(() => window.__theme_studio.snapshot().values.light.radius);
    const radius = page.getByTestId("edit-radius");
    await radius.fill("abc");
    await radius.press("Enter");
    await expect(radius).toHaveValue("abc");
    await expect(radius).toHaveAttribute("aria-invalid", "true");
    expect(await page.evaluate(() => window.__theme_studio.snapshot().values.light.radius)).toBe(before);
    await radius.press("Escape");
    await expect(radius).toHaveValue(before);
    await expect(radius).toHaveAttribute("aria-invalid", "false");
});

test("repeating the selected token value does not resolve its source again", async ({ page }) => {
    await waitForReady(page);
    await page.getByTestId("renderer-dom").click();
    await page.getByTestId("editor-colors").click();
    await page.getByTestId("color-token-primary").click();
    const counts = await page.evaluate(async () => {
        const api = window.__theme_studio;
        const frame = () => new Promise<void>(resolve => requestAnimationFrame(() => resolve()));
        api.edit("primary", "rgb(119 40 160)");
        for (let i = 0; i < 10; i++) await frame();
        const before = api.snapshot();
        for (let i = 0; i < 120; i++) api.edit("primary", "rgb(119 40 160)");
        await frame();
        await frame();
        return { before, after: api.snapshot() };
    });
    expect(counts.after.sdk_resolve_count).toBe(counts.before.sdk_resolve_count);
    expect(counts.after.resolve_count).toBe(counts.before.resolve_count);
    expect(counts.after.publish_count).toBe(counts.before.publish_count);
    expect(counts.after.history).toEqual(counts.before.history);
});

test.describe("editor disposal", () => {
    test.use({ fresh: true });
    test("queued editor layout work ends with its owner", async ({ page }) => {
        await waitForReady(page);
        await page.getByTestId("renderer-dom").click();
        await page.evaluate(() => {
            const request = window.requestAnimationFrame.bind(window);
            const cancel = window.cancelAnimationFrame.bind(window);
            const callbacks = new Map<number, FrameRequestCallback>();
            let id = 1_000_000;
            window.requestAnimationFrame = callback => {
                callbacks.set(++id, callback);
                return id;
            };
            window.cancelAnimationFrame = key => {
                callbacks.delete(key);
                cancel(key);
            };
            (window as any).__heldEditorFrames = { callbacks, request, cancel };
            window.__theme_studio.edit("radius", "12px");
        });
        // Let reactive effects enqueue their work while the repaint stays held.
        await page.waitForTimeout(50);
        expect(await page.evaluate(() => (window as any).__heldEditorFrames.callbacks.size)).toBeGreaterThan(0);
        expect(await page.evaluate(() => window.__theme_studio.dispose())).toBe(true);
        const errors = await page.evaluate(() => {
            const held = (window as any).__heldEditorFrames;
            window.requestAnimationFrame = held.request;
            window.cancelAnimationFrame = held.cancel;
            const errors: string[] = [];
            for (const callback of held.callbacks.values()) {
                try { callback(performance.now()); }
                catch (error) { errors.push(String(error)); }
            }
            return errors;
        });
        expect(errors).toEqual([]);
    });
});

test("external font faces provide real light, medium, semibold and bold weights", async ({ page }) => {
    await waitForReady(page);
    const weights = await page.evaluate(async () => {
        await Promise.all([300, 400, 500, 600, 700].map(weight => document.fonts.load(`${weight} 16px "IBM Plex Sans"`)));
        return Array.from(document.fonts).filter(face => face.family.includes("IBM Plex Sans") && face.status === "loaded").map(face => face.weight).sort();
    });
    expect(weights).toEqual(["300", "400", "500", "600", "700"]);
});

test("the command bar keeps actions visible at 200 percent text size", async ({ page }) => {
    await waitForReady(page);
    await page.evaluate(() => document.documentElement.style.fontSize = "200%");
    const bar = page.locator(".studio-editor-top");
    for (const viewport of [{ width: 720, height: 900 }, { width: 1280, height: 720 }]) {
        await page.setViewportSize(viewport);
        await expect.poll(() => bar.evaluate(node => node.getBoundingClientRect().height)).toBeLessThan(viewport.height * .4);
        for (const id of ["mode-dark", "theme-undo", "theme-save", "theme-export"]) {
            const button = page.getByTestId(id);
            const [bounds, toolbar] = await Promise.all([button.boundingBox(), bar.boundingBox()]);
            expect(bounds!.y).toBeGreaterThanOrEqual(toolbar!.y);
            expect(bounds!.y + bounds!.height).toBeLessThanOrEqual(toolbar!.y + toolbar!.height + 1);
        }
        expect(await bar.evaluate(node => node.scrollHeight - node.clientHeight)).toBeLessThan(2);
    }
    await page.evaluate(() => document.documentElement.style.fontSize = "");
});

test("fine adjustment and export retain their content at 200 percent text size", async ({ page }) => {
    await waitForReady(page);
    await page.setViewportSize({ width: 720, height: 900 });
    await page.evaluate(() => document.documentElement.style.fontSize = "200%");
    try {
        await page.getByTestId("editor-colors").click();
        await page.getByTestId("color-token-primary").click();
        const editor = page.locator("#studio-editor");
        await expect.poll(() => editor.evaluate(node => node.scrollWidth - node.clientWidth)).toBeLessThan(2);
        const source = await page.getByTestId("source-effective").boundingBox();
        const column = await editor.boundingBox();
        expect(source!.x).toBeGreaterThanOrEqual(column!.x);
        expect(source!.x + source!.width).toBeLessThanOrEqual(column!.x + column!.width);
        const renderers = page.locator(".studio-renderer-buttons");
        expect(await renderers.evaluate(node => node.scrollWidth - node.clientWidth)).toBeLessThan(2);
        await page.getByTestId("theme-export").click();
        const format = page.getByTestId("export-kind");
        await expect(format).toContainText("Theme JSON v2");
        expect(await format.locator("span").first().evaluate(node => node.scrollWidth - node.clientWidth)).toBeLessThan(2);
        await page.keyboard.press("Escape");
        await page.getByTestId("renderer-dom").click();
        const surface = page.getByTestId("dom-preview").getByTestId("preview-surface");
        await surface.scrollIntoViewIfNeeded();
        await expect.poll(() => surface.evaluate(node => node.scrollWidth - node.clientWidth)).toBeLessThan(2);
        const panel = await surface.boundingBox();
        for (const control of [surface.getByTestId("preview-primary"), surface.getByTestId("class-override"), surface.getByRole("button", { name: "Disabled", exact: true })]) {
            const bounds = await control.boundingBox();
            expect(bounds!.x).toBeGreaterThanOrEqual(panel!.x);
            expect(bounds!.x + bounds!.width).toBeLessThanOrEqual(panel!.x + panel!.width + 1);
        }
        const cards = surface.getByTestId("metrics-row").locator(":scope > div");
        expect(await cards.evaluateAll(nodes => nodes.every(node => node.scrollWidth <= node.clientWidth + 1))).toBe(true);
    } finally {
        await page.evaluate(() => document.documentElement.style.fontSize = "");
    }
});

test("the inspector reserves space beside the preview scroll area", async ({ page }) => {
    await page.setViewportSize({ width: 1440, height: 900 });
    await waitForReady(page);
    await page.getByTestId("renderer-dom").click();
    await page.getByTestId("inspector-toggle").click();
    const create = page.getByTestId("studio-preview").getByTestId("preview-primary");
    await create.click();
    const [panel, control] = await Promise.all([page.getByTestId("inspector-panel").boundingBox(), create.boundingBox()]);
    expect(control!.y + control!.height).toBeLessThanOrEqual(panel!.y);
    await expect(page.getByTestId("inspector-outline").locator("span")).toBeHidden();
    await page.keyboard.press("Escape");
});

test("the command bar stays available on the mobile preview panel", async ({ page }) => {
    await page.setViewportSize({ width: 390, height: 844 });
    await waitForReady(page);
    await page.getByTestId("mobile-preview").click();
    for (const id of ["mode-dark", "theme-save", "theme-export"]) await expect(page.getByTestId(id)).toBeInViewport();
    await page.getByTestId("theme-export").click();
    await expect(page.getByTestId("export-source")).toBeVisible();
    await page.keyboard.press("Escape");
});

test("shell buttons share the same height across action and ghost variants", async ({ page }) => {
    await page.setViewportSize({ width: 1440, height: 900 });
    await waitForReady(page);
    const heights = await Promise.all(["locale-en", "locale-zh", "mode-light", "theme-export", "preview-focus", "inspector-toggle", "renderer-dom"].map(async id => (await page.getByTestId(id).boundingBox())!.height));
    expect(Math.max(...heights) - Math.min(...heights)).toBeLessThan(1);
    await expect(page.getByTestId("locale-zh")).toHaveCSS("margin-left", "-1px");
    await page.setViewportSize({ width: 390, height: 844 });
    await expect(page.getByTestId("mobile-preview")).toHaveCSS("margin-left", "-1px");
});

test("a selection row keeps its token, source and color controls together", async ({ page }) => {
    await waitForReady(page);
    await page.getByTestId("editor-colors").click();
    await page.getByTestId("color-token-primary").click();
    await expect.poll(() => page.getByTestId("color-selection").evaluate(node => !!node.parentElement?.parentElement?.previousElementSibling?.querySelector('[data-testid="color-token-primary"]'))).toBe(true);
    await expect.poll(() => page.getByTestId("color-selection").evaluate(node => !!node.parentElement?.parentElement?.nextElementSibling?.querySelector('[data-testid="color-token-primary-foreground"]'))).toBe(true);
    await expect(page.getByTestId("source-token")).toHaveText("primary");
    await expect(page.locator(".ce-token-mode").first()).toHaveCSS("font-weight", "400");
    await expect(page.locator(".ce-group").filter({ has: page.getByTestId("color-token-primary") }).first().locator(":scope > h3")).toHaveCSS("text-transform", "uppercase");
    await page.getByTestId("color-value").fill("#9420aa");
    await page.getByTestId("color-value").press("Enter");
    await expect(page.getByTestId("source-effective")).toHaveText("#9420aa");
});

test("mobile selection keeps the group heading below a bordered workspace tab", async ({ page }) => {
    await page.setViewportSize({ width: 390, height: 844 });
    await waitForReady(page);
    await page.getByTestId("editor-colors").click();
    await page.getByTestId("color-token-primary").click();
    const group = page.locator(".ce-group").filter({ has: page.getByTestId("color-token-primary") }).first();
    await expect(group.locator(":scope > h3")).toBeInViewport({ ratio: 1 });
    await expect(page.locator(".studio-mobile-tabs")).toHaveCSS("border-bottom-color", "rgb(234, 236, 240)");
    await expect(page.locator(".studio-mobile-tabs")).toHaveCSS("border-bottom-width", "1px");
});

test("diagnostic summaries share labels and the shell font in Chinese", async ({ page }) => {
    await waitForReady(page);
    await page.locator(".studio-diagnostics > summary").click();
    await expect(page.getByTestId("font-diagnostics").locator("summary")).toHaveText("Fonts");
    await expect(page.locator(".theme-diagnostics details").nth(1).locator("summary")).toHaveText("Text contrast pairs");
    await page.getByTestId("locale-zh").click();
    const summary = page.locator(".studio-preview-diagnostics > summary");
    await expect(summary).toHaveCSS("font-weight", "600");
    await expect(summary).toHaveCSS("font-synthesis", "weight");
    expect(await summary.evaluate(node => getComputedStyle(node).fontFamily)).toContain("IBM Plex Sans");
});

test("inspector clips the actual rectangle instead of closing it at the scroll edge", async ({ page }) => {
    await waitForReady(page);
    await page.getByTestId("renderer-dom").click();
    await page.getByTestId("inspector-toggle").click();
    const card = page.getByTestId("dom-preview").getByTestId("scene-cards").getByTestId("cards-form");
    await card.scrollIntoViewIfNeeded();
    await card.hover({ position: { x: 8, y: 8 } });
    const outline = page.getByTestId("inspector-outline");
    await expect(outline).toBeVisible();
    const clip = await outline.evaluate(node => getComputedStyle(node).clipPath);
    expect(clip).toContain("inset(");
    expect(clip).not.toBe("inset(0px)");
    const [actual, selected] = await Promise.all([card.boundingBox(), outline.boundingBox()]);
    expect(Math.abs(actual!.height - selected!.height)).toBeLessThan(2);
});

test("anchored export selectors stay in place when output content changes", async ({ page }) => {
    await waitForReady(page);
    await page.getByTestId("theme-export").click();
    const selector = page.getByTestId("export-kind");
    const before = await selector.boundingBox();
    await selector.click();
    await page.getByTestId("export-kind-list").getByRole("option", { name: "Compatible JSON v1 · lossy", exact: true }).click();
    await expect.poll(async () => Math.abs((await selector.boundingBox())!.y - before!.y)).toBeLessThan(1);
    await page.keyboard.press("Escape");
});
