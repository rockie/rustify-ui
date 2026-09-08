import { expect, test } from "@playwright/test";
import { capture, differingPixels, litPixels, settle, waitForReady } from "./support";

const snapshot = (page: import("@playwright/test").Page) =>
    page.evaluate(() => window.__property_workbench.snapshot());

test.describe("M3 V2: one authoritative state behind a DOM panel and a GPU view", () => {
    test("a thousand objects with stable ids, the first one selected", async ({ page }) => {
        await waitForReady(page);
        expect(await snapshot(page)).toMatchObject({
            count: 1000,
            position: 1,
            selected: 1,
            name: "object-0001",
            color: "2e90fa",
            first_ids: "1,2,3,4",
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

    test("a hundred objects change in one update and the GPU shows the result", async ({ page }) => {
        await waitForReady(page);
        const region = page.getByTestId("workbench-gpu");
        const before = await settle(region);
        expect((await snapshot(page)).first_colors).toBe("2e90fa,12b76a,f79009,f04438");
        await page.getByTestId("recolour-batch").click();
        // One controlled model update, one new projection: the first four
        // objects each moved one step along the palette.
        expect(await snapshot(page)).toMatchObject({
            count: 1000,
            first_colors: "12b76a,f79009,f04438,7a5af8",
        });
        await expect
            .poll(async () => differingPixels(before, await capture(region)), { timeout: 5_000 })
            .toBeGreaterThan(20);
    });

    test("reordering keeps the selection on the object, not on the position", async ({ page }) => {
        await waitForReady(page);
        await page.getByTestId("select-next").click();
        await expect(page.getByTestId("selected-id")).toHaveText("2");
        expect(await snapshot(page)).toMatchObject({ position: 2, first_ids: "1,2,3,4" });
        await page.getByTestId("reverse-batch").click();
        // Object 2 is now second from the end of the reversed run.
        expect(await snapshot(page)).toMatchObject({
            selected: 2,
            name: "object-0002",
            position: 99,
            first_ids: "100,99,98,97",
        });
    });

    test("adding and removing objects keeps every other id where it was", async ({ page }) => {
        await waitForReady(page);
        await page.getByTestId("add-objects").click();
        expect(await snapshot(page)).toMatchObject({ count: 1010, selected: 1, position: 1 });
        await expect(page.getByTestId("object-count")).toHaveText("1010");
        await page.getByTestId("remove-objects").click();
        expect(await snapshot(page)).toMatchObject({
            count: 1000,
            selected: 1,
            position: 1,
            first_ids: "1,2,3,4",
        });
    });

    test("a list that names one object twice is refused, not guessed at", async ({ page }) => {
        await waitForReady(page);
        const region = page.getByTestId("workbench-gpu");
        const before = await settle(region);
        expect(await page.evaluate(() => window.__property_workbench.inject_duplicate_id())).toBe(true);
        await expect(page.getByTestId("rejected-binding")).toHaveText(
            "object 1 appears twice; the grid kept the last unambiguous list"
        );
        // The grid still shows the last list it could read unambiguously.
        await page.waitForTimeout(500);
        expect(differingPixels(before, await capture(region))).toBe(0);
    });

    test("ten thousand GPU actions arrive once each, in order", async ({ page }) => {
        test.setTimeout(600_000);
        await waitForReady(page);
        const region = page.getByTestId("workbench-gpu");
        await settle(region);
        const box = (await region.boundingBox())!;
        const rounds = 10_000;
        // Driven inside the page: ten thousand round trips through the test
        // harness would measure the harness, not the runtime. The events are
        // the ones the region's own listeners receive.
        const result = await page.evaluate(
            async (args) => {
                const api = window.__property_workbench;
                const canvas = document.querySelector('[data-testid="workbench-gpu"]')!;
                const fire = (type: string, buttons: number) =>
                    canvas.dispatchEvent(
                        new PointerEvent(type, {
                            bubbles: true,
                            cancelable: true,
                            clientX: args.x,
                            clientY: args.y,
                            pointerId: 1,
                            pointerType: "mouse",
                            button: 0,
                            buttons,
                            isPrimary: true,
                        })
                    );
                for (let i = 0; i < args.rounds; i++) {
                    fire("pointerdown", 1);
                    fire("pointerup", 0);
                    if (i % 25 === 0) {
                        await new Promise((r) => setTimeout(r, 0));
                    }
                }
                await new Promise((r) => setTimeout(r, 3_000));
                return api.snapshot();
            },
            { x: box.x + box.width * 0.77, y: box.y + box.height * 0.085, rounds }
        );
        // Not one lost, not one delivered twice.
        expect(result.accepted).toBe(rounds);
        // And they were applied in order: each one advanced the selection by
        // one object, so the ring of a thousand lands back where it started.
        expect(result.position).toBe((rounds % 1000) + 1);
        expect(result.count).toBe(1000);
    });

    test("deleting the selected object drops it and clears the selection", async ({ page }) => {
        await waitForReady(page);
        await page.getByTestId("delete-selected").click();
        expect(await snapshot(page)).toMatchObject({
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
