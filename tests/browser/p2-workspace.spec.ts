import { expect, Page, test } from "@playwright/test";

import { sharedPage } from "./support";

/// M5 V7: three panels, ten views over the objects, and every command in one
/// place.
///
/// The sizes are read from the application's own snapshot rather than from the
/// pixels, because what is under test is the arithmetic a person's drag ends
/// up in - and a hundred drags measured through the harness would be measuring
/// the harness.

const snapshot = (page: Page) => page.evaluate(() => window.__property_workbench.snapshot());
const workspace = async (page: Page) => (await snapshot(page)).workspace;

/// Drags a divider by `delta` pixels, in the page, one pointer sequence.
async function dragDivider(page: Page, index: number, delta: number) {
    await page.evaluate(
        ([index, delta]) => {
            const divider = document.querySelector(
                `[data-testid="divider-${index}"]`
            ) as HTMLElement;
            const box = divider.getBoundingClientRect();
            const from = box.left + box.width / 2;
            const event = (type: string, x: number) =>
                divider.dispatchEvent(
                    new PointerEvent(type, {
                        bubbles: true,
                        clientX: x,
                        clientY: box.top + box.height / 2,
                        pointerId: 1,
                        isPrimary: true,
                    })
                );
            event("pointerdown", from);
            event("pointermove", from + (delta as number));
            event("pointerup", from + (delta as number));
        },
        [index, delta] as const
    );
}

test.describe("M5 V7: a workspace of three panels", () => {
    test.describe.configure({ mode: "serial" });
    const shared = sharedPage();

    test("three panels, each no smaller than the application declared", async () => {
        const page = shared.page;
        const { panels } = await workspace(page);
        expect(panels).toHaveLength(3);
        // The minimums the workbench declares: a list of names, a region worth
        // drawing, and a form of twenty fields.
        for (const [index, min] of [220, 320, 360].entries()) {
            expect(panels[index], `panel ${index}`).toBeGreaterThanOrEqual(min);
        }
    });

    test("a hundred drags never take a panel below its minimum", async () => {
        const page = shared.page;
        const before = (await workspace(page)).panels as number[];
        const total = before.reduce((sum, size) => sum + size, 0);
        const mins = [220, 320, 360];

        for (let round = 0; round < 100; round += 1) {
            // Alternating dividers and far larger than the room available,
            // which is what a person dragging quickly produces.
            await dragDivider(page, round % 2, round % 3 === 0 ? 400 : -350);
            const { panels } = await workspace(page);
            for (const [index, min] of mins.entries()) {
                expect(panels[index], `round ${round}, panel ${index}`).toBeGreaterThanOrEqual(
                    min
                );
            }
            // A drag moves room between two panels; it never invents or loses
            // any, so the row is the width it was.
            expect(
                (panels as number[]).reduce((sum: number, size: number) => sum + size, 0),
                `round ${round}`
            ).toBeCloseTo(total, 2);
        }
    });

    test("a divider is reachable and movable from the keyboard", async () => {
        const page = shared.page;
        const divider = page.getByTestId("divider-0");
        await expect(divider).toHaveAttribute("role", "separator");
        await expect(divider).toHaveAttribute("aria-label", "resize panel 1");

        await divider.focus();
        const before = (await workspace(page)).panels[0];
        await page.keyboard.press("ArrowRight");
        expect((await workspace(page)).panels[0]).toBe(before + 16);
        await page.keyboard.press("ArrowLeft");
        expect((await workspace(page)).panels[0]).toBe(before);
    });

    test("ten views over one set of objects, and switching keeps the selection", async () => {
        const page = shared.page;
        expect(await workspace(page)).toMatchObject({ tabs: 10, tab: "all" });
        const strip = page.getByTestId("object-views");
        await expect(strip.getByRole("tab")).toHaveCount(10);

        const selected = (await snapshot(page)).selected;
        await strip.getByRole("tab", { name: "locked" }).click();
        expect(await workspace(page)).toMatchObject({ tab: "locked" });
        // A view is what you are looking at, not where things are: the object
        // showing is still the object showing.
        expect(await snapshot(page)).toMatchObject({ selected });

        await strip.getByRole("tab", { name: "all", exact: true }).click();
        expect(await workspace(page)).toMatchObject({ tab: "all" });
    });

    test("closing the view showing moves to its neighbour, and one view cannot be closed", async () => {
        const page = shared.page;
        const strip = page.getByTestId("object-views");
        await strip.getByRole("tab", { name: "blue" }).click();
        expect(await workspace(page)).toMatchObject({ tab: "blue" });

        await page.getByTestId("close-blue").click();
        // The right-hand neighbour, and one fewer view.
        expect(await workspace(page)).toMatchObject({ tab: "green", tabs: 9 });

        // The first one has no close control at all, so there is always
        // somewhere to be.
        await expect(page.getByTestId("close-all")).toHaveCount(0);
    });

    test("closing a view that is not showing leaves the selection alone", async () => {
        const page = shared.page;
        const strip = page.getByTestId("object-views");
        await strip.getByRole("tab", { name: "all", exact: true }).click();
        const before = await workspace(page);

        await page.getByTestId("close-amber").click();
        expect(await workspace(page)).toMatchObject({ tab: before.tab, tabs: before.tabs - 1 });
    });

    test("the palette finds a command, runs it once, and refuses one that cannot run", async () => {
        const page = shared.page;
        await page.getByTestId("open-commands").click();
        const palette = page.getByTestId("command-palette");
        await expect(palette).toBeVisible();
        await expect(page.getByTestId("command-search")).toBeFocused();

        // Every word has to appear: "next object" is one command, not
        // everything that mentions an object.
        await page.getByTestId("command-search").fill("next object");
        await expect(palette.getByRole("option")).toHaveCount(1);

        const before = (await snapshot(page)).selected;
        await page.keyboard.press("Enter");
        await expect(palette).toBeHidden();
        expect((await snapshot(page)).selected).toBe(before + 1);

        // One that cannot run stays in the list, says why, and does nothing.
        await page.getByTestId("open-commands").click();
        await page.getByTestId("command-search").fill("save");
        const blocked = page.getByTestId("command-save");
        await expect(blocked).toHaveAttribute("aria-disabled", "true");
        await expect(blocked).toContainText("there is nothing to save");
        await blocked.click({ force: true });
        await expect(palette).toBeVisible();
        expect((await snapshot(page)).form).toMatchObject({ saves: 0 });

        await page.keyboard.press("Escape");
        await expect(palette).toBeHidden();
    });

    test("after a hundred adjustments the region still hits where it says it drew", async () => {
        const page = shared.page;
        await expect
            .poll(async () => (await snapshot(page)).region, { timeout: 30_000 })
            .toBe("ready");
        const region = page.getByTestId("workbench-gpu");
        await region.scrollIntoViewIfNeeded();

        // P1's V4 asks whether the geometry a region *reports* and the
        // geometry a pointer *hits* are the same thing. The workbench reports
        // two controls rather than the geometry fixture's twenty anchors, so
        // this is that question asked twenty times across resizes: ten rounds,
        // each moving the region's panel and then aiming at both controls.
        for (let round = 0; round < 10; round += 1) {
            await dragDivider(page, 1, round % 2 === 0 ? 60 : -60);
            await dragDivider(page, 0, round % 2 === 0 ? -40 : 40);

            const before = await snapshot(page);
            const box = (await region.boundingBox())!;
            const controls = before.controls!;

            // The checkbox the region drew: a click on the rectangle it
            // reported must reach the control, not the space beside it.
            await page.mouse.click(
                box.x + controls.locked.x + controls.locked.width / 2,
                box.y + controls.locked.y + controls.locked.height / 2
            );
            await expect
                .poll(async () => (await snapshot(page)).locked)
                .toBe(!before.locked);

            // And the slider, aimed at a known fraction of the track.
            const target = Math.round((round % 2 === 0 ? 0.25 : 0.75) * 100 / 5) * 5;
            await page.mouse.click(
                box.x + controls.size.x + (controls.size.width * target) / 100,
                box.y + controls.size.y + controls.size.height / 2
            );
            await expect
                .poll(async () => (await snapshot(page)).size, { timeout: 10_000 })
                .toBeGreaterThan(0);
        }
    });

    test("a right-click inside the region opens a menu anchored to it", async () => {
        const page = shared.page;
        await expect
            .poll(async () => (await snapshot(page)).region, { timeout: 30_000 })
            .toBe("ready");
        const region = page.getByTestId("workbench-gpu");
        await region.scrollIntoViewIfNeeded();
        await region.click({ button: "right" });

        expect(await workspace(page)).toMatchObject({ menu: true });
        const menu = page.getByRole("menu", { name: "what can be done here" });
        await expect(menu).toBeVisible();

        const before = (await snapshot(page)).selected;
        await menu.getByRole("menuitem", { name: "select the next object" }).click();
        expect((await snapshot(page)).selected).toBe(before + 1);
        expect(await workspace(page)).toMatchObject({ menu: false });
    });
});
