import { test, expect, waitForReady } from "./support";

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
