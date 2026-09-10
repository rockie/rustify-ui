import { expect, Page, test } from "@playwright/test";

import { sharedPage } from "./support";

/// M8: composing Chinese in the two places the plan names.
///
/// §9.4 asks for real pinyin in the property form and the command palette, and
/// a real input method needs a person - that record stays. What a browser *can*
/// settle is the mechanics underneath it: that a composition in flight is not
/// treated as a value, that keys pressed during one belong to the composition
/// and not to the control, and that what the application ends up with is what
/// the composition committed.
///
/// P1 checked this for the region's own text control. These are the two DOM
/// controls P2 added, and neither had been checked.

const snapshot = (page: Page) => page.evaluate(() => window.__property_workbench.snapshot());

/// One composition, as the browser reports it: a start, the pinyin being
/// composed, and an end carrying what was chosen.
async function compose(page: Page, testId: string, steps: string[], final: string) {
    await page.evaluate(
        async ({ testId, steps, final }) => {
            const field = document.querySelector(
                `[data-testid="${testId}"]`
            ) as HTMLInputElement | HTMLTextAreaElement;
            field.focus();
            field.dispatchEvent(
                new CompositionEvent("compositionstart", { bubbles: true, data: "" })
            );
            for (const step of steps) {
                field.value = step;
                field.dispatchEvent(
                    new CompositionEvent("compositionupdate", { bubbles: true, data: step })
                );
                field.dispatchEvent(
                    new InputEvent("input", { bubbles: true, isComposing: true, data: step })
                );
                await new Promise((resolve) => setTimeout(resolve, 10));
            }
            field.value = final;
            field.dispatchEvent(
                new CompositionEvent("compositionend", { bubbles: true, data: final })
            );
            field.dispatchEvent(
                new InputEvent("input", { bubbles: true, isComposing: false, data: final })
            );
        },
        { testId, steps, final }
    );
}

test.describe("M8: a composition in the property form", () => {
    test.describe.configure({ mode: "serial" });
    const shared = sharedPage(async (page) => {
        await page.getByTestId("object-7").click();
        await expect.poll(async () => (await snapshot(page)).selected).toBe(7);
    });

    test("what the composition committed is what the application has", async () => {
        const page = shared.page;
        await compose(page, "name-input", ["s", "sh", "shu", "shux"], "属性");
        await expect.poll(async () => (await snapshot(page)).name).toBe("属性");
    });

    test("a composition that is abandoned leaves the value it started from", async () => {
        const page = shared.page;
        const before = (await snapshot(page)).name;
        await page.evaluate(() => {
            const field = document.querySelector(
                '[data-testid="name-input"]'
            ) as HTMLInputElement;
            field.focus();
            field.dispatchEvent(
                new CompositionEvent("compositionstart", { bubbles: true, data: "" })
            );
            field.value = "gongzuo";
            field.dispatchEvent(
                new CompositionEvent("compositionupdate", { bubbles: true, data: "gongzuo" })
            );
            field.dispatchEvent(
                new InputEvent("input", { bubbles: true, isComposing: true, data: "gongzuo" })
            );
        });
        // Mid-composition, the pinyin on screen is not the value: the
        // application still has what it had.
        expect((await snapshot(page)).name).toBe(before);

        // Abandoning it puts the field back where it was and commits nothing,
        // which is what an input method does when the user presses Escape.
        await page.evaluate((restored) => {
            const field = document.querySelector(
                '[data-testid="name-input"]'
            ) as HTMLInputElement;
            field.value = restored;
            field.dispatchEvent(
                new CompositionEvent("compositionend", { bubbles: true, data: restored })
            );
            field.dispatchEvent(
                new InputEvent("input", { bubbles: true, isComposing: false, data: restored })
            );
        }, before ?? "");
        await expect.poll(async () => (await snapshot(page)).name).toBe(before);
    });

    test("ten thousand characters of Chinese and emoji survive the field", async () => {
        const page = shared.page;
        // The same sample V9 puts through the clipboard, through the form
        // instead: a value the field has to hold, not just carry.
        const unit = [..."属性工作台🙂🌍"];
        const out: string[] = [];
        while (out.length < 10_000) {
            out.push(...unit);
        }
        const long = out.slice(0, 10_000).join("");
        await page.getByTestId("notes-input").fill(long);
        await expect.poll(async () => (await snapshot(page)).notes).toBe(long);
    });
});

test.describe("M8: a composition in the command palette", () => {
    test.describe.configure({ mode: "serial" });
    const shared = sharedPage();

    test("a command is found by what the composition committed", async () => {
        const page = shared.page;
        await page.getByTestId("open-commands").click();
        const palette = page.getByTestId("command-palette");
        await expect(palette).toBeVisible();
        await expect(page.getByTestId("command-search")).toBeFocused();

        // Mid-composition the search has pinyin in it, and the list is not
        // filtered by a value the user has not chosen yet.
        await page.evaluate(() => {
            const field = document.querySelector(
                '[data-testid="command-search"]'
            ) as HTMLInputElement;
            field.dispatchEvent(
                new CompositionEvent("compositionstart", { bubbles: true, data: "" })
            );
            field.value = "xia";
            field.dispatchEvent(
                new CompositionEvent("compositionupdate", { bubbles: true, data: "xia" })
            );
            field.dispatchEvent(
                new InputEvent("input", { bubbles: true, isComposing: true, data: "xia" })
            );
        });

        // Enter during a composition chooses a candidate; it must not run a
        // command. Nothing has been chosen, so the palette is still open.
        await page.keyboard.press("Enter");
        await expect(palette).toBeVisible();

        // The composition finishes on a word the palette does not match, and
        // the list says so rather than running something.
        await page.evaluate(() => {
            const field = document.querySelector(
                '[data-testid="command-search"]'
            ) as HTMLInputElement;
            field.value = "属性";
            field.dispatchEvent(
                new CompositionEvent("compositionend", { bubbles: true, data: "属性" })
            );
            field.dispatchEvent(
                new InputEvent("input", { bubbles: true, isComposing: false, data: "属性" })
            );
        });
        await expect(page.getByTestId("command-empty")).toBeVisible();

        await page.keyboard.press("Escape");
        await expect(palette).toBeHidden();
    });

    test("and the search still works with the keyboard afterwards", async () => {
        const page = shared.page;
        await page.getByTestId("open-commands").click();
        await page.getByTestId("command-search").fill("next object");
        await expect(page.getByTestId("command-palette").getByRole("option")).toHaveCount(1);
        const before = (await snapshot(page)).selected;
        await page.keyboard.press("Enter");
        await expect(page.getByTestId("command-palette")).toBeHidden();
        expect((await snapshot(page)).selected).toBe((before ?? 0) + 1);
    });
});
