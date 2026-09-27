import { expect, Page } from "@playwright/test";
import { rounds } from "../tier";
import { test, waitForReady } from "./support";

const snapshot = (page: Page) => page.evaluate(() => window.__property_workbench.snapshot());

const start = (page: Page, delay: number, outcome: string) =>
    page.evaluate(
        ({ delay, outcome }) => window.__property_workbench.start_load(delay, outcome),
        { delay, outcome }
    );

test.describe("M6 V7: four states, and only the newest answer", () => {
    test("each of the four states says what it is", async ({ page }) => {
        await waitForReady(page);
        const details = page.getByTestId("details");

        await start(page, 60, "the details");
        await expect(details).toHaveAttribute("data-state", "loading");
        await expect(details).toHaveAttribute("data-state", "ready");
        await expect(details).toHaveText("the details");

        await start(page, 40, "empty");
        await expect(details).toHaveAttribute("data-state", "empty");
        // Empty is an answer, not a wait that never ends.
        await expect(details).toHaveText("nothing to show");

        await start(page, 40, "error");
        await expect(details).toHaveAttribute("data-state", "error");
        // A failure says what to do about it.
        await expect(details).toContainText("try again");
    });

    test("an answer that arrives after a newer one does not undo it", async ({ page }) => {
        await waitForReady(page);
        const details = page.getByTestId("details");

        // A is asked first and answers last.
        await start(page, 900, "answer A");
        await start(page, 60, "answer B");
        await expect(details).toHaveText("answer B");

        // Long enough for A to arrive; it changes nothing.
        await page.waitForTimeout(1_200);
        await expect(details).toHaveText("answer B");
        expect(await snapshot(page)).toMatchObject({
            details: "ready",
            details_value: "answer B",
        });
    });

    test("twenty pairs answered backwards all end on the newer answer", async ({ page }) => {
        test.setTimeout(300_000);
        await waitForReady(page);
        const pairs = rounds(20);
        for (let round = 0; round < pairs; round++) {
            await start(page, 400, `stale ${round}`);
            await start(page, 20, `fresh ${round}`);
            await expect(page.getByTestId("details")).toHaveText(`fresh ${round}`);
        }
        await page.waitForTimeout(600);
        await expect(page.getByTestId("details")).toHaveText(`fresh ${pairs - 1}`);
    });

    test("an answer for a scope that has gone reaches nothing", async ({ page }) => {
        const failures: string[] = [];
        page.on("pageerror", (error) => failures.push(String(error)));
        await waitForReady(page);

        await start(page, 800, "for a scope that will not be there");
        await page.evaluate(() => window.__property_workbench.dispose());
        await expect(page.getByTestId("details")).toHaveCount(0);
        await page.waitForTimeout(1_200);

        // The runtime is untouched and the page still works.
        const state = await page.evaluate(() => ({
            regions: window.__property_workbench.live_regions(),
            errors: window.__property_workbench.hooks.runtime.errors,
        }));
        expect(state).toEqual({ regions: 0, errors: [] });
        expect(failures).toEqual([]);

        await page.evaluate(() => window.__property_workbench.mount());
        await expect(page.getByTestId("details")).toHaveAttribute("data-state", "loading");
        expect(await start(page, 40, "a fresh scope answers")).toBe(true);
        await expect
            .poll(async () => (await snapshot(page)).details_value, { timeout: 10_000 })
            .toBe("a fresh scope answers");
    });
});

