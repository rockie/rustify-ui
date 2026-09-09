import { expect, Page, test } from "@playwright/test";
import { capture, differingPixels, settle, waitForReady } from "./support";

const snapshot = (page: Page) => page.evaluate(() => window.__property_workbench.snapshot());
const diagnostics = (page: Page) => page.evaluate(() => window.__property_workbench.diagnostics());
const state = (page: Page) =>
    page.evaluate(() => document.querySelector("[data-testid='workbench-gpu']") !== null);

test.describe("M7 V8: a region whose context went away", () => {
    test("comes back with the state the application still holds", async ({ page }) => {
        const failures: string[] = [];
        page.on("pageerror", (error) => failures.push(String(error)));
        await waitForReady(page);
        const region = page.getByTestId("workbench-gpu");
        await settle(region);

        // Something worth losing: a selection and an edited value.
        await page.getByRole("textbox", { name: "go to object" }).fill("42");
        await page.getByRole("button", { name: "go", exact: true }).click();
        await page.getByRole("textbox", { name: "name" }).fill("survives a lost context");
        const before = await snapshot(page);
        expect(before).toMatchObject({ selected: 42, name: "survives a lost context" });
        const drawn = await settle(region);

        expect(await page.evaluate(() => window.__property_workbench.lose_context("workbench-gpu"))).toBe(
            true
        );

        // While the context is gone the region says so, and says it plainly:
        // this is a state it comes back from, not a failure.
        await expect.poll(async () => (await snapshot(page)).region, { timeout: 10_000 }).toBe("lost");
        expect(await page.evaluate(() => window.__property_workbench.live_regions())).toBe(0);
        // Nothing the application holds was in the region.
        expect(await snapshot(page)).toMatchObject({
            selected: 42,
            name: "survives a lost context",
        });

        // The browser gives the context back, and the region is built again
        // with the state the application never stopped holding.
        expect(
            await page.evaluate(() => window.__property_workbench.restore_context("workbench-gpu"))
        ).toBe(true);
        await expect.poll(async () => (await snapshot(page)).region, { timeout: 20_000 }).toBe("ready");
        await expect.poll(async () => (await snapshot(page)).selected, { timeout: 15_000 }).toBe(42);
        expect(await snapshot(page)).toMatchObject({
            selected: 42,
            name: "survives a lost context",
            count: before.count,
        });
        expect(await state(page)).toBe(true);
        // And it draws the same picture: the projection is the state, not a
        // copy of it that could have been lost with the context.
        await expect
            .poll(async () => differingPixels(drawn, await capture(region)), { timeout: 20_000 })
            .toBeLessThan(200);

        // It is recorded as what it was: recoverable, with nothing to do.
        const log = await diagnostics(page);
        const lost = log.entries.filter((entry) => entry.kind === "GpuContextLost");
        expect(lost.length).toBe(1);
        expect(lost[0].suggestion).toContain("rebuilt with the current state");
        expect(failures).toEqual([]);
    });

    test("twenty losses, with the state edited between them, lose nothing", async ({ page }) => {
        test.setTimeout(600_000);
        const failures: string[] = [];
        page.on("pageerror", (error) => failures.push(String(error)));
        await waitForReady(page);
        const region = page.getByTestId("workbench-gpu");
        await settle(region);
        const name = page.getByRole("textbox", { name: "name" });

        for (let round = 0; round < 20; round++) {
            // Ten fields edited across the rounds, so a loss always lands on
            // an application that has moved since the last one.
            await name.fill(`round ${round}`);
            expect(await snapshot(page)).toMatchObject({ name: `round ${round}` });
            expect(
                await page.evaluate(() => window.__property_workbench.lose_context("workbench-gpu"))
            ).toBe(true);
            await expect
                .poll(async () => (await snapshot(page)).region, { timeout: 15_000 })
                .toBe("lost");
            expect(
                await page.evaluate(() => window.__property_workbench.restore_context("workbench-gpu"))
            ).toBe(true);
            await expect
                .poll(async () => (await snapshot(page)).region, { timeout: 20_000 })
                .toBe("ready");
            // The edit made before the loss is still the application's value.
            expect(await snapshot(page)).toMatchObject({ name: `round ${round}` });
            // A rebuilt region is a working region: it answers a pointer again.
            await settle(region);
        }

        const box = (await region.boundingBox())!;
        await page.mouse.click(box.x + box.width * 0.5, box.y + box.height * 0.7);
        await expect.poll(async () => (await snapshot(page)).selected).not.toBe(1);

        const log = await diagnostics(page);
        expect(log.entries.filter((entry) => entry.kind === "GpuContextLost").length).toBe(20);
        // Twenty rebuilds and no runtime failure.
        expect(await page.evaluate(() => window.__property_workbench.hooks.runtime.errors.length)).toBe(20);
        expect(failures).toEqual([]);
    });
});

test.describe("M7 V9: a bounded record that says when it is a tail", () => {
    test("the record names the runtime, the build, and what it dropped", async ({ page }) => {
        await waitForReady(page);
        const log = await diagnostics(page);
        expect(log).toMatchObject({ runtime: 1, dropped: 0, max_entries: 1000, max_bytes: 4194304 });
        // The build is the identifier the page and the wasm already agreed on.
        expect(log.build).toMatch(/^[0-9]+$/);
    });

    test("every entry carries a cause and a next step, and no user content", async ({ page }) => {
        await waitForReady(page);
        // A refusal the application asked for: mounting over an occupied
        // container.
        expect(
            await page.evaluate(() => {
                try {
                    return window.__property_workbench.mount_into("workbench");
                } catch (error) {
                    return String(error);
                }
            })
        ).toContain("already mounted");

        const secret = "a name nobody should log";
        await page.getByRole("textbox", { name: "name" }).fill(secret);
        expect(await snapshot(page)).toMatchObject({ name: secret });

        const log = await diagnostics(page);
        expect(log.count).toBeGreaterThan(0);
        const occupied = log.entries.filter((entry) => entry.kind === "OccupiedContainer");
        expect(occupied.length).toBe(1);
        expect(occupied[0].suggestion).not.toBe("");
        for (const entry of log.entries) {
            expect(entry.detail).not.toBe("");
            expect(entry.suggestion).not.toBe("");
            // What a user typed never reaches the record.
            expect(JSON.stringify(entry)).not.toContain(secret);
        }
    });
});
