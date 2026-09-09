import { expect, Page, test } from "@playwright/test";
import { settle, waitForReady } from "./support";

const snapshot = (page: Page) => page.evaluate(() => window.__property_workbench.snapshot());

const focused = (page: Page) =>
    page.evaluate(() => {
        const active = document.activeElement as HTMLElement | null;
        return active?.dataset.testid ?? active?.tagName.toLowerCase() ?? null;
    });

test.describe("M5 V6: every meaningful control has one semantic entry", () => {
    test("twenty controls are found by role and name", async ({ page }) => {
        await waitForReady(page);
        const found: string[] = [];
        const seen = async (description: string, locator: ReturnType<Page["getByRole"]>) => {
            await expect(locator, description).toHaveCount(1);
            found.push(description);
        };

        await seen("properties heading", page.getByRole("heading", { name: "properties" }));
        await seen("object properties region", page.getByRole("region", { name: "object properties" }));
        await seen("name field", page.getByRole("textbox", { name: "name" }));
        for (const colour of ["2e90fa", "12b76a", "f79009", "f04438", "7a5af8", "475467"]) {
            await seen(`colour ${colour}`, page.getByRole("button", { name: `colour ${colour}` }));
        }
        await seen("previous", page.getByRole("button", { name: "previous" }));
        await seen("next", page.getByRole("button", { name: "next" }));
        await seen("delete selected", page.getByRole("button", { name: "delete selected" }));
        await seen("recolour 100", page.getByRole("button", { name: "recolour 100" }));
        await seen("reverse 100", page.getByRole("button", { name: "reverse 100" }));
        await seen("add 10", page.getByRole("button", { name: "add 10" }));
        await seen("remove 10", page.getByRole("button", { name: "remove 10" }));
        await seen("go to object", page.getByRole("textbox", { name: "go to object" }));
        await seen("go", page.getByRole("button", { name: "go", exact: true }));
        await seen("find result", page.getByRole("status", { name: "find result" }));
        await seen("object count", page.getByRole("status", { name: "object count" }));

        expect(found.length).toBeGreaterThanOrEqual(20);
        expect(new Set(found).size).toBe(found.length);
    });

    test("the region's pixels are not a second, mute copy of the panel", async ({ page }) => {
        await waitForReady(page);
        // Everything the region draws that means something is a control in the
        // panel; the canvas itself is decoration.
        await expect(page.getByTestId("workbench-gpu")).toHaveAttribute("aria-hidden", "true");
    });

    test("the DOM path and the pointer path select the same object", async ({ page }) => {
        await waitForReady(page);
        const region = page.getByTestId("workbench-gpu");
        await settle(region);
        const box = (await region.boundingBox())!;

        // Pointer: a real click on a cell of the grid.
        await page.mouse.click(box.x + box.width * 0.5, box.y + box.height * 0.7);
        await expect(page.getByTestId("selected-id")).not.toHaveText("1");
        const picked = (await snapshot(page)).selected!;

        // Keyboard and DOM: the same object, reached by name.
        await page.getByRole("button", { name: "previous" }).click();
        expect((await snapshot(page)).selected).not.toBe(picked);
        await page.getByRole("textbox", { name: "go to object" }).fill(String(picked));
        await page.getByRole("button", { name: "go", exact: true }).click();

        expect(await snapshot(page)).toMatchObject({ selected: picked });
        await expect(page.getByRole("status", { name: "find result" })).toHaveText(
            `selected object ${picked}`
        );
    });

    test("looking for an object answers, or says it could not, within five seconds", async ({
        page,
    }) => {
        await waitForReady(page);
        // One that exists: answered without waiting.
        expect(await page.evaluate(() => window.__property_workbench.lookup_object(3))).toBe("found");

        // One that has been deleted: a different answer from one that never
        // existed, and just as final.
        await page.getByTestId("selected-id").waitFor();
        await page.getByRole("button", { name: "delete selected" }).click();
        expect(await page.evaluate(() => window.__property_workbench.lookup_object(1))).toBe(
            "disposed"
        );
        expect(await page.evaluate(() => window.__property_workbench.await_object(1))).toBe(
            "disposed"
        );

        // One that never existed: the wait ends at the deadline and says so.
        const started = Date.now();
        expect(
            await page.evaluate(() => window.__property_workbench.await_object(999_999, 1_000))
        ).toBe("timeout");
        expect(Date.now() - started).toBeLessThan(5_000);
        expect(await page.evaluate(() => window.__property_workbench.lookup_object(999_999))).toBe(
            "not_found"
        );
    });
});

test.describe("M5 V6: five journeys with the keyboard alone", () => {
    test("select, edit, run a command, read the result, and undo the selection", async ({
        page,
    }) => {
        test.setTimeout(300_000);
        await waitForReady(page);
        await settle(page.getByTestId("workbench-gpu"));

        // 1. Select an object without a pointer.
        await page.getByRole("textbox", { name: "go to object" }).focus();
        await page.keyboard.type("42");
        await page.keyboard.press("Tab");
        expect(await focused(page)).toBe("find-object");
        await page.keyboard.press("Enter");
        expect(await snapshot(page)).toMatchObject({ selected: 42 });

        // 2. Edit a property of it.
        await page.getByRole("textbox", { name: "name" }).focus();
        await page.keyboard.press("ControlOrMeta+a");
        await page.keyboard.type("renamed with the keyboard");
        expect(await snapshot(page)).toMatchObject({
            selected: 42,
            name: "renamed with the keyboard",
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
        await expect(page.getByRole("status", { name: "find result" })).toHaveText("no object 999999");
        // The failed lookup left the selection alone.
        expect(await snapshot(page)).toMatchObject({ selected: 42 });

        // 5. Move the selection and get back to where it was.
        await page.getByRole("button", { name: "next" }).focus();
        await page.keyboard.press("Enter");
        expect(await snapshot(page)).toMatchObject({ selected: 43 });
        await page.getByRole("button", { name: "previous" }).focus();
        await page.keyboard.press("Enter");
        expect(await snapshot(page)).toMatchObject({
            selected: 42,
            name: "renamed with the keyboard",
        });
    });
});
