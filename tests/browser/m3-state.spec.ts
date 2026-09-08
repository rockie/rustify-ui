import { expect, test } from "@playwright/test";
import { capture, differingPixels, settle, waitForReady } from "./support";

test.describe("M3 V2: shared state and controlled GPU components", () => {
    test("interleaved DOM and GPU actions each land exactly once", async ({ page }) => {
        test.setTimeout(300_000);
        await waitForReady(page);
        const region = page.getByTestId("scope-a-gpu-1");
        const box = (await region.boundingBox())!;
        const gpuButton = { x: box.x + box.width / 2, y: box.y + box.height / 2 + 24 };
        const rounds = 20;
        for (let round = 0; round < rounds; round++) {
            await page.mouse.click(gpuButton.x, gpuButton.y);
            await page.getByTestId("scope-a-dom-increment").click();
        }
        // One authoritative value: both paths write the same signal, so the
        // count is exactly the number of accepted actions.
        await expect(page.getByTestId("scope-a-dom-count")).toHaveText(String(rounds * 2));
        await expect(page.getByTestId("scope-b-dom-count")).toHaveText("0");
    });

    test("both regions of a scope show the current value, not a stale one", async ({ page }) => {
        await waitForReady(page);
        const first = page.getByTestId("scope-a-gpu-1");
        const second = page.getByTestId("scope-a-gpu-2");
        const beforeFirst = await settle(first);
        const beforeSecond = await settle(second);
        for (let click = 0; click < 3; click++) {
            await page.getByTestId("scope-a-dom-increment").click();
        }
        await expect(page.getByTestId("scope-a-dom-count")).toHaveText("3");
        await expect
            .poll(async () => differingPixels(beforeFirst, await capture(first)), { timeout: 5_000 })
            .toBeGreaterThan(20);
        await expect
            .poll(async () => differingPixels(beforeSecond, await capture(second)), { timeout: 5_000 })
            .toBeGreaterThan(20);
        // The two regions project the same value, so they end up identical.
        expect(differingPixels(await capture(first), await capture(second))).toBe(0);
    });
});
