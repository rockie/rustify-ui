import { expect, Page } from "@playwright/test";
import { B0 } from "./loads";
import { capture, differingPixels, isShared, litPixels, settle, test, waitForQuiet } from "./support";

/// M6 · the page as it loads is B0, and B0 is what the load definition says.
///
/// A budget quoted against a load nobody checked is a number about whatever
/// happened to be on the page. So the first check here is the definition
/// itself - ten DOM controls, twenty GPU controls in one region, one scope,
/// one shared state - measured on the page rather than read back from a
/// constant the application also wrote.
///
/// `waitForReady` is not used: it mounts the two counter scopes the
/// phase-one checks were written for, and the point of this file is the page
/// with nothing on it but B0. A shared page is loaded and put back already,
/// and may have those scopes below B0: the checks that count what is on the
/// page run on a page of their own, and the ones that use B0 do not mind.
async function openB0(page: Page) {
    if (!isShared(page)) {
        await page.goto("./");
    }
    await expect(page.getByTestId("status")).toHaveAttribute("data-status", "ready", {
        timeout: 120_000,
    });
    await expect.poll(async () => (await b0(page)).region, { timeout: 60_000 }).toBe("ready");
    await waitForQuiet(page);
}

const b0 = (page: Page) => page.evaluate(() => window.__fusion_basic.b0_state());

/// The form controls a person can operate inside the B0 scope.
const domControls = (page: Page) =>
    page.evaluate(() =>
        document.querySelectorAll("#b0 input, #b0 textarea, #b0 select, #b0 button").length
    );

test.describe("M6 · B0 is the load it is named after", () => {
    // What a load puts on the page and what it fetches: only a load can say.
    test.describe(() => {
        test.use({ fresh: true });

        test("ten DOM controls, twenty GPU controls, one region, one scope", async ({ page }) => {
            await openB0(page);

            expect(await domControls(page)).toBe(B0.domControls);

            // Every GPU control is counted by the rectangle it drew, not by the
            // list it is named in: a control that laid out to nothing is not on
            // screen, whatever the region meant to draw.
            const state = await b0(page);
            const names = await page.evaluate(() => window.__fusion_basic.b0_controls());
            expect(names).toHaveLength(B0.gpuControls);
            expect(state.controls.map((control: { name: string }) => control.name)).toEqual(names);
            for (const control of state.controls) {
                expect(control.width, control.name).toBeGreaterThan(0);
                expect(control.height, control.name).toBeGreaterThan(0);
            }

            expect(await page.evaluate(() => window.__fusion_basic.live_regions())).toBe(B0.regions);
            expect(await page.evaluate(() => window.__fusion_basic.live_components())).toBe(B0.scopes);

            // And they are drawn: twenty controls that reported a rectangle and
            // left the canvas empty would pass everything above.
            expect(litPixels(await capture(page.getByTestId("b0-gpu")))).toBeGreaterThan(200);
        });

        test("the first screen takes one font, and it is the Latin one", async ({ page }) => {
            await openB0(page);
            const fonts = await page.evaluate(() =>
                performance
                    .getEntriesByType("resource")
                    .map((entry) => entry.name)
                    .filter((name) => name.endsWith(".ttf") || name.endsWith(".woff2"))
                    .map((name) => name.slice(name.lastIndexOf("/") + 1))
            );
            expect(fonts).toEqual(["IBMPlexSans-Text.ttf"]);
        });
    });

    test("one state, two halves: each answers for what the other did", async ({ page }) => {
        await openB0(page);
        const region = page.getByTestId("b0-gpu");
        const before = await settle(region);

        // DOM to GPU: the count the region draws is the one the DOM button
        // raised.
        await page.getByTestId("b0-bump").click();
        await expect(page.getByTestId("b0-count")).toHaveText("1");
        await expect
            .poll(async () => (await b0(page)).count, { timeout: 10_000 })
            .toBe(1);
        await expect
            .poll(async () => differingPixels(before, await capture(region)), { timeout: 10_000 })
            .toBeGreaterThan(10);

        // GPU to DOM: a real pointer on the rectangle the region reported for
        // its own button.
        await clickControl(page, "bump_first");
        await expect(page.getByTestId("b0-count")).toHaveText("2");

        // A value on a range, from the DOM side, read back from the state
        // both halves project from.
        await page.getByTestId("b0-size").focus();
        await page.keyboard.press("ArrowRight");
        expect((await b0(page)).size).toBe(45);

        // Text the application owns, typed into the control that asks for it.
        await page.getByTestId("b0-title").fill("");
        await page.getByTestId("b0-title").type("edited");
        expect((await b0(page)).title).toBe("edited");
    });

    test("a request the region makes is answered by the application, once", async ({ page }) => {
        await openB0(page);
        // The chooser does not choose: it asks, and what it shows afterwards
        // is the answer. Two entries, so a second ask comes back to the first.
        expect((await b0(page)).palette).toBe("slate");
        await clickControl(page, "chooser_palette");
        await expect.poll(async () => (await b0(page)).palette).toBe("amber");
        await clickControl(page, "chooser_palette");
        await expect.poll(async () => (await b0(page)).palette).toBe("slate");

        // A checkbox in the region and one in the DOM over the same field.
        expect((await b0(page)).visible).toBe(true);
        await clickControl(page, "check_visible");
        await expect.poll(async () => (await b0(page)).visible).toBe(false);
        await page.getByTestId("b0-visible").click();
        await expect.poll(async () => (await b0(page)).visible).toBe(true);
    });
});

/// Puts a real pointer on the middle of the rectangle the region reported for
/// one of its controls.
///
/// The canvas is measured again on every call: a DOM change above it moves
/// the canvas, and a click aimed at where it used to be lands on something
/// else.
async function clickControl(page: Page, name: string) {
    const state = await b0(page);
    const control = state.controls.find((entry: { name: string }) => entry.name === name);
    expect(control, name).toBeDefined();
    // Into view first, then measured: the region sits below the fold on this
    // page, and a pointer aimed outside the viewport lands nowhere at all -
    // silently, because there is nothing there to refuse it.
    const canvasLocator = page.getByTestId("b0-gpu");
    await canvasLocator.scrollIntoViewIfNeeded();
    const canvas = (await canvasLocator.boundingBox())!;
    await page.mouse.click(
        canvas.x + control.x + control.width / 2,
        canvas.y + control.y + control.height / 2
    );
}
