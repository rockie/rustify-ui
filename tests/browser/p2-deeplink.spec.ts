import { expect, Page, test } from "@playwright/test";

/// M4 V6: a URL somebody was sent.
///
/// The same checks run twice, against the same application deployed twice: at
/// the root, and under `/tools/demo/`. The difference between them is exactly
/// the thing that breaks deep links - what a relative reference resolves
/// against - so running one and assuming the other is how that ships.

const snapshot = (page: Page) => page.evaluate(() => window.__property_workbench.snapshot());

async function open(page: Page, path: string) {
    const failures: string[] = [];
    page.on("response", (response) => {
        if (response.status() >= 400) {
            failures.push(`${response.status()} ${new URL(response.url()).pathname}`);
        }
    });
    await page.goto(path);
    await expect(page.getByTestId("status")).toHaveAttribute("data-status", "ready", {
        timeout: 60_000,
    });
    return failures;
}

test.describe("M4 V6: three deep links and one that names nothing", () => {
    for (const id of [1, 42, 999]) {
        test(`objects/${id} opens on that object`, async ({ page }) => {
            const failures = await open(page, `./objects/${id}`);
            expect(await snapshot(page)).toMatchObject({
                selected: id,
                name: `object-${String(id).padStart(4, "0")}`,
                path: `/objects/${id}`,
            });
            // Nothing 404'd on the way. This is the check that catches a page
            // whose scripts resolved against the route rather than the base -
            // it boots at the root and does not boot one level down.
            expect(failures).toEqual([]);
            await expect(page.getByTestId("not-found")).toBeHidden();
        });
    }

    test("an id nothing answers to says so and keeps the address", async ({ page }) => {
        await open(page, "./objects/424242");
        await expect(page.getByTestId("not-found")).toContainText("no object 424242");
        expect(await snapshot(page)).toMatchObject({
            selected: null,
            path: "/objects/424242",
        });
        // The address is kept so it can be corrected, rather than swapped for
        // one the user did not ask for.
        expect(new URL(page.url()).pathname).toContain("/objects/424242");
    });

    test("the region and its fonts load from the deployment, not from the route", async ({
        page,
    }) => {
        const failures = await open(page, "./objects/42");
        await expect.poll(async () => (await snapshot(page)).region, { timeout: 30_000 }).toBe(
            "ready"
        );
        // A region resolves its resources against where the application is
        // deployed. Against the current path they would be fetched from
        // `/objects/42/...`, which is the bug this exists to keep fixed.
        expect(failures).toEqual([]);
        const diagnostics = await page.evaluate(() =>
            window.__property_workbench.diagnostics()
        );
        expect(
            diagnostics.entries.filter((entry) => entry.kind === "AssetLoadFailed")
        ).toEqual([]);
    });

    test("arriving somewhere adds no history entry of its own", async ({ page }) => {
        await open(page, "./objects/7");
        // The entry the user arrived on is the entry they are on: the router
        // stamps it in place rather than pushing a second one, so the first
        // back press leaves the application rather than going nowhere.
        expect(
            await page.evaluate(
                () => (history.state as { rustify?: { index?: number } } | null)?.rustify?.index
            )
        ).toBe(0);

        // And moving within the application does add one.
        await page.getByTestId("select-next").click();
        expect(
            await page.evaluate(
                () => (history.state as { rustify?: { index?: number } } | null)?.rustify?.index
            )
        ).toBe(1);
    });
});
