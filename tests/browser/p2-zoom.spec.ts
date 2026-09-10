import { expect, Page, test } from "@playwright/test";

import { settle, waitForReady } from "./support";

/// M8: B1's five journeys at 200%.
///
/// Zoom is tested the way WCAG's reflow criterion defines it: a viewport half
/// as wide is what 200% zoom of a 1280-pixel window produces, and doubling the
/// root font size is the text-scaling half. Both together, because a layout can
/// survive one and not the other.
///
/// What the plan asks for here is that the journeys can still be *completed* -
/// not that nothing scrolls sideways. Whether the page reflows without
/// two-dimensional scrolling is asked of the catalogue at 400%, in
/// `p2-reflow.spec.ts`, which is where the requirement puts it.

const snapshot = (page: Page) => page.evaluate(() => window.__property_workbench.snapshot());

/// The page as a person zoomed to 200% sees it.
async function zoomed(page: Page) {
    await page.setViewportSize({ width: 640, height: 720 });
    await page.evaluate(() => {
        document.documentElement.style.fontSize = "200%";
    });
}

test.describe("M8: the five journeys at 200%", () => {
    test("select, edit, run a command, read the result, and undo the selection", async ({
        page,
    }) => {
        test.setTimeout(300_000);
        await waitForReady(page);
        await zoomed(page);
        await settle(page.getByTestId("workbench-gpu"));

        // The same five journeys as `m5-semantics`, driven the same way. If
        // zoom broke one, it would break here and nowhere else.

        // 1. Select an object without a pointer.
        await page.getByRole("textbox", { name: "go to object" }).focus();
        await page.keyboard.type("42");
        await page.keyboard.press("Tab");
        await page.keyboard.press("Enter");
        expect(await snapshot(page)).toMatchObject({ selected: 42 });

        // 2. Edit a property of it.
        await page.getByRole("textbox", { name: "name" }).focus();
        await page.keyboard.press("ControlOrMeta+a");
        await page.keyboard.type("renamed at 200 percent");
        expect(await snapshot(page)).toMatchObject({
            selected: 42,
            name: "renamed at 200 percent",
        });

        // 3. Run a command that changes many objects at once.
        await page.getByRole("button", { name: "recolour 100" }).focus();
        await page.keyboard.press("Enter");
        expect((await snapshot(page)).first_colors).toBe("12b76a,f79009,f04438,7a5af8");

        // 4. Read the result of a lookup that fails.
        await page.getByRole("textbox", { name: "go to object" }).focus();
        await page.keyboard.press("ControlOrMeta+a");
        await page.keyboard.type("999999");
        await page.keyboard.press("Tab");
        await page.keyboard.press("Enter");
        await expect(page.getByRole("status", { name: "find result" })).toHaveText(
            "no object 999999"
        );
        expect(await snapshot(page)).toMatchObject({ selected: 42 });

        // 5. Move the selection and get back to where it was.
        await page.getByRole("button", { name: "next" }).focus();
        await page.keyboard.press("Enter");
        expect(await snapshot(page)).toMatchObject({ selected: 43 });
        await page.getByRole("button", { name: "previous" }).focus();
        await page.keyboard.press("Enter");
        expect(await snapshot(page)).toMatchObject({
            selected: 42,
            name: "renamed at 200 percent",
        });
    });

    test("the region is still drawn, and still says where it drew", async ({ page }) => {
        test.setTimeout(120_000);
        await waitForReady(page);
        await zoomed(page);
        await settle(page.getByTestId("workbench-gpu"));

        // A zoom changes the region's canvas size, which is a resize as far as
        // the region is concerned. It has to come back with a report, or a
        // caller aiming at anything it drew is aiming at the old layout.
        await expect
            .poll(async () => (await snapshot(page)).controls !== null, { timeout: 30_000 })
            .toBe(true);
        const controls = (await snapshot(page)).controls!;
        for (const [name, rect] of Object.entries(controls)) {
            if (typeof rect === "number") {
                continue;
            }
            expect(rect.width, `${name} has width`).toBeGreaterThan(0);
            expect(rect.height, `${name} has height`).toBeGreaterThan(0);
        }
        // And what it reported is inside the canvas it was given.
        const canvas = (await page.getByTestId("workbench-gpu").boundingBox())!;
        expect(controls.groups.x + controls.groups.width).toBeLessThanOrEqual(canvas.width + 1);
    });
});
