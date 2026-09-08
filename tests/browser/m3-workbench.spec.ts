import { expect, test } from "@playwright/test";
import { capture, differingPixels, litPixels, settle, waitForReady } from "./support";

const snapshot = (page: import("@playwright/test").Page) =>
    page.evaluate(() => window.__property_workbench.snapshot());

test.describe("M3 V2: one authoritative state behind a DOM panel and a GPU view", () => {
    test("a thousand objects with stable ids, the first one selected", async ({ page }) => {
        await waitForReady(page);
        expect(await snapshot(page)).toEqual({
            count: 1000,
            position: 1,
            selected: 1,
            name: "object-0001",
            color: "2e90fa",
        });
        await expect(page.getByTestId("object-count")).toHaveText("1000");
        await expect(page.getByTestId("selected-id")).toHaveText("1");
        expect(await page.evaluate(() => window.__property_workbench.live_regions())).toBe(1);
        // The grid is on screen without any interaction: the first projection
        // of application state has to survive the pump that creates the region,
        // otherwise the region draws its empty defaults until something else
        // changes.
        const drawn = await settle(page.getByTestId("workbench-gpu"));
        expect(litPixels(drawn)).toBeGreaterThan(10_000);
    });

    test("renaming and recolouring in the DOM changes what the GPU draws", async ({ page }) => {
        await waitForReady(page);
        const region = page.getByTestId("workbench-gpu");
        const before = await settle(region);
        await page.getByTestId("name-input").fill("renamed in the panel");
        await expect
            .poll(async () => differingPixels(before, await capture(region)), { timeout: 5_000 })
            .toBeGreaterThan(20);
        const renamed = await settle(region);
        await page.getByTestId("swatch-f04438").click();
        await expect
            .poll(async () => differingPixels(renamed, await capture(region)), { timeout: 5_000 })
            .toBeGreaterThan(20);
        expect(await snapshot(page)).toMatchObject({
            selected: 1,
            name: "renamed in the panel",
            color: "f04438",
        });
    });

    test("the GPU view moves the selection the application owns", async ({ page }) => {
        await waitForReady(page);
        const region = page.getByTestId("workbench-gpu");
        await settle(region);
        const box = (await region.boundingBox())!;
        // The region puts swatch, name, position and the two buttons in one
        // header row above the grid.
        const next = { x: box.x + box.width * 0.77, y: box.y + box.height * 0.085 };
        await page.mouse.click(next.x, next.y);
        await expect(page.getByTestId("selected-id")).toHaveText("2");
        expect(await snapshot(page)).toMatchObject({ selected: 2, position: 2, name: "object-0002" });
        await page.getByTestId("select-next").click();
        await expect(page.getByTestId("selected-id")).toHaveText("3");
    });

    test("a run of selection steps from both sides lands on one expected object", async ({ page }) => {
        test.setTimeout(300_000);
        await waitForReady(page);
        const rounds = 25;
        for (let round = 0; round < rounds; round++) {
            await page.getByTestId("select-next").click();
        }
        await expect(page.getByTestId("selected-id")).toHaveText(String(1 + rounds));
        for (let round = 0; round < 5; round++) {
            await page.getByTestId("select-previous").click();
        }
        expect(await snapshot(page)).toMatchObject({
            selected: 1 + rounds - 5,
            position: 1 + rounds - 5,
            name: `object-${String(1 + rounds - 5).padStart(4, "0")}`,
        });
    });

    test("clicking a cell in the GPU grid selects that object in the DOM", async ({ page }) => {
        await waitForReady(page);
        const region = page.getByTestId("workbench-gpu");
        await settle(region);
        const box = (await region.boundingBox())!;
        const cell = { x: box.x + box.width * 0.5, y: box.y + box.height * 0.7 };
        await page.mouse.click(cell.x, cell.y);
        await expect(page.getByTestId("selected-id")).not.toHaveText("1");
        const picked = await snapshot(page);
        expect(picked.selected).not.toBeNull();
        expect(picked.name).toBe(`object-${String(picked.selected).padStart(4, "0")}`);
        // The same cell is the same object: identity comes from the id, not
        // from where the pointer landed.
        await page.mouse.click(cell.x, cell.y);
        expect(await snapshot(page)).toMatchObject({ selected: picked.selected });
        // And the panel edits the object the grid chose.
        await page.getByTestId("name-input").fill("picked on the GPU");
        expect(await snapshot(page)).toMatchObject({
            selected: picked.selected,
            name: "picked on the GPU",
        });
    });

    test("deleting the selected object drops it and clears the selection", async ({ page }) => {
        await waitForReady(page);
        await page.getByTestId("delete-selected").click();
        expect(await snapshot(page)).toEqual({
            count: 999,
            position: 0,
            selected: null,
            name: null,
            color: "none",
        });
        await expect(page.getByTestId("selected-id")).toHaveText("none");
        await expect(page.getByTestId("name-input")).toBeDisabled();
        // The scope keeps working: the next object can be selected again.
        await page.getByTestId("select-next").click();
        await expect(page.getByTestId("selected-id")).toHaveText("2");
    });
});
