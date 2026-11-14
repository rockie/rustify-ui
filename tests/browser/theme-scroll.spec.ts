import { test, expect, waitForReady } from "./support";

for (const width of [1440, 768, 390]) {
    test(`workbench scroll stays inside its panels at ${width}px`, async ({ page }) => {
        await page.setViewportSize({ width, height: 1000 });
        await waitForReady(page);
        if (width === 390) await page.getByTestId("mobile-preview").click();
        await page.evaluate(() => document.fonts.ready);
        const checkPage = async () => {
            expect(await page.evaluate(() => document.documentElement.scrollHeight - innerHeight)).toBeLessThanOrEqual(1);
            await page.evaluate(() => window.scrollTo(0, document.documentElement.scrollHeight));
            expect(await page.evaluate(() => window.scrollY)).toBe(0);
        };
        await checkPage();
        const preview = page.locator(".studio-previews");
        await preview.evaluate(node => { node.scrollTop = node.scrollHeight; });
        expect(await preview.evaluate(node => node.scrollTop)).toBeGreaterThan(0);
        await expect(page.getByTestId("host-probe").getByRole("button")).toBeInViewport();
        const compare = page.getByTestId("compare-scope");
        await compare.locator("summary").first().focus();
        await page.keyboard.press("Enter");
        await expect(compare).toHaveAttribute("open", "");
        await checkPage();
        await page.keyboard.press("Enter");
        await expect(compare).not.toHaveAttribute("open");
        if (width === 390) await page.getByTestId("mobile-edit").click();
        const editor = width === 390 ? page.getByTestId("editor-shell") : page.getByTestId("studio-editor");
        await editor.evaluate(node => { node.scrollTop = node.scrollHeight; });
        await expect(page.locator(".editor-note")).toBeInViewport();
        await checkPage();
    });
}

for (const width of [1440, 390]) {
    test(`wheel over GPU scrolls the preview at ${width}px and preserves clicks`, async ({ page }) => {
        await page.setViewportSize({ width, height: 900 });
        await waitForReady(page);
        if (width === 390) await page.getByTestId("mobile-preview").click();
        await page.getByTestId(width === 390 ? "renderer-gpu" : "renderer-compare").click();
        await expect.poll(() => page.evaluate(() => window.__theme_studio.snapshot().gpu_state)).toBe("Ready");
        const pane = page.locator(".studio-previews");
        const canvas = page.getByTestId("theme-gpu");
        const box = (await canvas.boundingBox())!;
        await page.mouse.move(box.x + 60, box.y + 80);
        const initial = await pane.evaluate(node => node.scrollTop);
        await page.mouse.wheel(0, 480);
        await expect.poll(() => pane.evaluate(node => node.scrollTop), { timeout: 3000 }).toBeGreaterThan(initial);
        await page.mouse.wheel(0, -480);
        await expect.poll(() => pane.evaluate(node => node.scrollTop)).toBe(initial);

        // Reach the controls with real wheel input, then use their reported
        // canvas coordinates to check hit testing after the parent scrolled.
        await page.mouse.wheel(0, 4000);
        await expect.poll(() => pane.evaluate(node => node.scrollTop)).toBeGreaterThan(initial + 480);
        const state = await page.evaluate(() => window.__theme_studio.snapshot());
        const button = state.samples.find((sample: any) => sample.id === "control.button")!;
        const [x, y, w, h] = button.rect;
        await canvas.click({ position: { x: x + w / 2, y: y + h / 2 } });
        await expect.poll(() => page.evaluate(() => window.__theme_studio.snapshot().controls.clicks)).toBe(state.controls.clicks + 1);
    });
}
