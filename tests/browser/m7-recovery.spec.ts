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

test.describe("M7 V8: a region that cannot start at all", () => {
    test("says so within two seconds and leaves the DOM half working", async ({ page }) => {
        await waitForReady(page);
        await page.evaluate(() => window.__property_workbench.dispose());

        // The machine this runs on has WebGL2, so the denial has to be made:
        // the canvas answers null for that context and for nothing else.
        await page.evaluate(() => {
            const original = HTMLCanvasElement.prototype.getContext;
            HTMLCanvasElement.prototype.getContext = function (
                this: HTMLCanvasElement,
                type: string,
                ...rest: unknown[]
            ) {
                return type === "webgl2" ? null : (original as Function).call(this, type, ...rest);
            } as typeof original;
        });

        // Measured inside the page. A round trip through the test harness
        // costs more than the budget being measured.
        const elapsed = await page.evaluate(async () => {
            const started = performance.now();
            window.__property_workbench.mount();
            for (;;) {
                if (document.querySelector('[data-testid="workbench-gpu-error"]')) {
                    return performance.now() - started;
                }
                if (performance.now() - started > 5_000) {
                    return -1;
                }
                await new Promise((resolve) => requestAnimationFrame(resolve));
            }
        });
        expect(elapsed).toBeGreaterThanOrEqual(0);
        expect(elapsed).toBeLessThan(2_000);

        await expect(page.getByTestId("workbench-gpu-error")).toHaveText(
            "no WebGL2 context for the region canvas"
        );
        expect(await snapshot(page)).toMatchObject({ region: "failed" });
        // The panel beside it is untouched: a region is a part of a page, not
        // the page.
        await page.getByRole("button", { name: "next" }).click();
        expect(await snapshot(page)).toMatchObject({ selected: 2 });

        const log = await diagnostics(page);
        const failed = log.entries.filter((entry) => entry.kind === "GpuInitFailed");
        expect(failed.length).toBe(1);
        expect(failed[0].suggestion).toContain("use the DOM path");
    });
});

test.describe("M7 V9 / NFR-4: a capability a region does not get", () => {
    test("is refused, recorded once, and does not move the page", async ({ page }) => {
        await waitForReady(page);
        const url = page.url();
        const refusals = async () =>
            (await diagnostics(page)).entries.filter(
                (entry) => entry.kind === "UnsupportedCapability"
            );

        // One refusal happens before the application asks for anything:
        // Makepad names its window on start, and an embedded region has no
        // document title to name. It was always refused; until now it was
        // refused silently.
        const atBoot = await refusals();
        expect(atBoot).toHaveLength(1);
        expect(atBoot[0].detail).toContain("set the document title");

        // The region asks the host to open a link. It is not the page, so it
        // does not get to.
        await page.getByRole("button", { name: "ask the region to open a link" }).click();
        await expect.poll(async () => (await refusals()).length, { timeout: 10_000 }).toBe(2);

        const entry = (await refusals())[1];
        expect(entry.detail).toContain("open a URL");
        expect(entry.suggestion).toContain("do it from the host page");
        expect(entry.region).not.toBeNull();

        // The page did not move, and nothing else opened.
        expect(page.url()).toBe(url);
        expect(await snapshot(page)).toMatchObject({ region: "ready" });

        // Asked ten more times, it is still those two entries: a region in a
        // loop cannot fill the record with one mistake.
        for (let round = 0; round < 10; round++) {
            await page.getByRole("button", { name: "ask the region to open a link" }).click();
        }
        await page.waitForTimeout(500);
        expect(await refusals()).toHaveLength(2);
        expect(page.url()).toBe(url);
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

    test("filling it past its ceiling keeps the newest and says how many went", async ({ page }) => {
        test.setTimeout(300_000);
        await waitForReady(page);
        const before = await diagnostics(page);
        expect(before.dropped).toBe(0);

        // Every one of these is refused, so nothing is created and the only
        // thing that grows is the record.
        const refusals = await page.evaluate(() => {
            let refused = 0;
            for (let round = 0; round < 1_100; round++) {
                if (window.__property_workbench.mount_over("workbench") !== "mounted") {
                    refused += 1;
                }
            }
            return refused;
        });
        expect(refusals).toBe(1_100);

        const log = await diagnostics(page);
        // The ceiling held, and what went over it was counted rather than
        // quietly forgotten.
        expect(log.count).toBe(log.max_entries);
        expect(log.dropped).toBeGreaterThanOrEqual(1_100 - log.max_entries);
        expect(log.bytes).toBeLessThanOrEqual(log.max_bytes);
        // What is kept is the newest end of the record.
        expect(log.entries[log.entries.length - 1].kind).toBe("OccupiedContainer");
        // And the page is unharmed: nothing was mounted, nothing was lost.
        expect(await page.evaluate(() => window.__property_workbench.live_regions())).toBe(1);
        expect(await snapshot(page)).toMatchObject({ region: "ready" });
    });

    test("every entry carries a cause and a next step, and no user content", async ({ page }) => {
        await waitForReady(page);
        // A refusal the application asked for: mounting over an occupied
        // container.
        expect(await page.evaluate(() => window.__property_workbench.mount_over("workbench"))).toContain(
            "already mounted"
        );

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
