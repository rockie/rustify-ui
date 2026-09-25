import { expect, Page } from "@playwright/test";
import { capture, differingPixels, settle, test, waitForReady } from "./support";

const snapshot = (page: Page) => page.evaluate(() => window.__property_workbench.snapshot());

// The name field the region draws, just right of the swatch.
// Offsets from the region's own left edge, in CSS pixels. The header puts its
// buttons first and its values after them, so these do not move when a value
// does; they are pixels rather than fractions because the row is laid out from
// the left, not stretched across the width.
/// Where the region says it drew its two pieces of text, in its own local
/// pixels. Asked for rather than guessed: a pixel offset written down here is
/// a test that breaks the first time the panel around the region changes
/// width, which is exactly what R25 says not to build.
async function labelPoint(page: Page, which: "name" | "notes") {
    const region = page.getByTestId("workbench-gpu");
    const box = (await region.boundingBox())!;
    await expect
        .poll(async () => (await snapshot(page)).controls, { timeout: 15_000 })
        .not.toBeNull();
    const rect = (await snapshot(page)).controls![which];
    return {
        x: box.x + rect.x + rect.width / 2,
        y: box.y + rect.y + rect.height / 2,
    };
}

/// Clicks the name the region draws, which asks the application for a real
/// text control over that rectangle.
async function startEditing(page: Page) {
    const at = await labelPoint(page, "name");
    await page.mouse.click(at.x, at.y);
    await expect.poll(async () => (await snapshot(page)).editing).toBe(true);
    return page.getByTestId("gpu-name-edit");
}

/// One IME composition, as the browser reports it: a start, the text being
/// composed, and an end carrying the final value.
async function compose(page: Page, steps: string[], final: string) {
    await page.evaluate(
        async ({ steps, final }) => {
            const field = document.querySelector('[data-testid="gpu-name-edit"]') as HTMLInputElement;
            field.dispatchEvent(new CompositionEvent("compositionstart", { bubbles: true, data: "" }));
            for (const step of steps) {
                field.value = step;
                field.dispatchEvent(new CompositionEvent("compositionupdate", { bubbles: true, data: step }));
                field.dispatchEvent(new InputEvent("input", { bubbles: true, isComposing: true, data: step }));
                await new Promise((r) => setTimeout(r, 10));
            }
            field.value = final;
            field.dispatchEvent(new CompositionEvent("compositionend", { bubbles: true, data: final }));
            field.dispatchEvent(new InputEvent("input", { bubbles: true, isComposing: false, data: final }));
        },
        { steps, final }
    );
}

/// Clicks the one line of notes the region shows, which asks for a control
/// that can hold all of them.
async function startEditingNotes(page: Page) {
    const at = await labelPoint(page, "notes");
    await page.mouse.click(at.x, at.y);
    await expect.poll(async () => (await snapshot(page)).editing).toBe(true);
    return page.getByTestId("gpu-notes-edit");
}

test.describe("M5 V5: the browser's own text control edits the region's text", () => {
    test("the control takes the rectangle the region drew the name into", async ({ page }) => {
        await waitForReady(page);
        const region = page.getByTestId("workbench-gpu");
        await settle(region);
        const field = await startEditing(page);

        // The region hands over a rectangle; the control covers it.
        const placed = await page.evaluate(() => {
            const canvas = document.querySelector('[data-testid="workbench-gpu"]')!;
            const input = document.querySelector('[data-testid="gpu-name-edit"]')!;
            const c = canvas.getBoundingClientRect();
            const i = input.getBoundingClientRect();
            return { dx: i.left - c.left, dy: i.top - c.top, width: i.width, height: i.height };
        });
        expect(placed.width).toBeGreaterThan(10);
        expect(placed.height).toBeGreaterThan(6);
        // It opens with the value the application holds, selected, and focused.
        await expect(field).toHaveValue("object-0001");
        expect(
            await page.evaluate(
                () => document.activeElement?.getAttribute("data-testid") ?? null
            )
        ).toBe("gpu-name-edit");
    });

    test("enter commits once and the region draws the new value", async ({ page }) => {
        await waitForReady(page);
        const region = page.getByTestId("workbench-gpu");
        const before = await settle(region);
        const field = await startEditing(page);
        await field.fill("committed by enter");
        await page.keyboard.press("Enter");

        await expect.poll(async () => (await snapshot(page)).editing).toBe(false);
        expect(await snapshot(page)).toMatchObject({
            selected: 1,
            name: "committed by enter",
            invalidated: 0,
        });
        await expect(page.getByTestId("name-input")).toHaveValue("committed by enter");
        await expect
            .poll(async () => differingPixels(before, await capture(region)), { timeout: 5_000 })
            .toBeGreaterThan(20);
    });

    test("escape abandons the edit and the value is untouched", async ({ page }) => {
        await waitForReady(page);
        await settle(page.getByTestId("workbench-gpu"));
        const field = await startEditing(page);
        await field.fill("typed but abandoned");
        await page.keyboard.press("Escape");

        await expect.poll(async () => (await snapshot(page)).editing).toBe(false);
        expect(await snapshot(page)).toMatchObject({ selected: 1, name: "object-0001" });
    });

    test("keys pressed while composing belong to the composition", async ({ page }) => {
        await waitForReady(page);
        await settle(page.getByTestId("workbench-gpu"));
        await startEditing(page);

        await page.evaluate(() => {
            const field = document.querySelector('[data-testid="gpu-name-edit"]') as HTMLInputElement;
            field.value = "ni hao";
            field.dispatchEvent(new CompositionEvent("compositionstart", { bubbles: true, data: "" }));
        });
        // Neither key ends anything while the composition is open.
        await page.keyboard.press("Enter");
        await page.keyboard.press("Escape");
        expect(await snapshot(page)).toMatchObject({ editing: true, name: "object-0001" });

        // The composition finishes, and only then does Enter commit - once.
        await page.evaluate(() => {
            const field = document.querySelector('[data-testid="gpu-name-edit"]') as HTMLInputElement;
            field.value = "你好";
            field.dispatchEvent(new CompositionEvent("compositionend", { bubbles: true, data: "你好" }));
            field.dispatchEvent(new InputEvent("input", { bubbles: true, data: "你好" }));
        });
        expect(await snapshot(page)).toMatchObject({ editing: true, name: "object-0001" });
        await page.keyboard.press("Enter");
        await expect.poll(async () => (await snapshot(page)).editing).toBe(false);
        expect(await snapshot(page)).toMatchObject({ name: "你好", invalidated: 0 });
    });

    test("a composition produces one value, not one per event", async ({ page }) => {
        await waitForReady(page);
        await settle(page.getByTestId("workbench-gpu"));
        await startEditing(page);
        // The intermediate states of a composition are not proposals.
        await compose(page, ["n", "ni", "nih", "niha", "nihao"], "你好世界");
        expect(await snapshot(page)).toMatchObject({ editing: true, name: "object-0001" });
        await page.keyboard.press("Enter");
        await expect.poll(async () => (await snapshot(page)).editing).toBe(false);
        expect(await snapshot(page)).toMatchObject({ name: "你好世界" });
    });

    test("a value that moves underneath the session ends it instead of overwriting", async ({
        page,
    }) => {
        await waitForReady(page);
        await settle(page.getByTestId("workbench-gpu"));
        const field = await startEditing(page);
        await field.fill("a draft nobody asked for");

        // The application changes the same field from somewhere else. Done
        // without touching focus: leaving the control would be a commit, and
        // this is about the value moving, not about the user leaving.
        await page.evaluate(() => {
            const input = document.querySelector('[data-testid="name-input"]') as HTMLInputElement;
            input.value = "changed from the panel";
            input.dispatchEvent(new Event("input", { bubbles: true }));
        });
        await expect.poll(async () => (await snapshot(page)).editing).toBe(false);
        expect(await snapshot(page)).toMatchObject({
            name: "changed from the panel",
            invalidated: 1,
        });
        await expect(page.getByTestId("gpu-name-edit")).toHaveCount(0);
    });

    test("losing the control commits exactly once", async ({ page }) => {
        await waitForReady(page);
        await settle(page.getByTestId("workbench-gpu"));
        const field = await startEditing(page);
        await field.fill("committed by leaving");
        await page.getByTestId("select-next").focus();

        await expect.poll(async () => (await snapshot(page)).editing).toBe(false);
        expect(await snapshot(page)).toMatchObject({
            selected: 1,
            name: "committed by leaving",
            invalidated: 0,
        });
    });

    test("several lines are edited in a control that can hold them", async ({ page }) => {
        await waitForReady(page);
        await settle(page.getByTestId("workbench-gpu"));
        // The region has room for one line; the value has two.
        expect(await snapshot(page)).toMatchObject({ notes: "note 1\nsecond line" });

        const field = await startEditingNotes(page);
        await expect(field).toHaveValue("note 1\nsecond line");
        const box = (await field.boundingBox())!;
        expect(box.height).toBeGreaterThan(40);

        // Enter is a new line here, not a commit.
        await field.fill("first\nsecond");
        await page.keyboard.press("End");
        await page.keyboard.press("Enter");
        await page.keyboard.type("third");
        expect((await snapshot(page)).editing).toBe(true);

        // Leaving it is the commit, and every line survives.
        await page.getByTestId("select-next").focus();
        await expect.poll(async () => (await snapshot(page)).editing).toBe(false);
        expect(await snapshot(page)).toMatchObject({
            selected: 1,
            notes: "first\nsecond\nthird",
            invalidated: 0,
        });
        await expect(page.getByTestId("notes-input")).toHaveValue("first\nsecond\nthird");
    });
});

