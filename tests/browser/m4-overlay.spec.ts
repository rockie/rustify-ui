import { expect, Page } from "@playwright/test";
import { anchorRect, geometry, mountGeometry, test } from "./support";
import { rounds } from "../tier";

const box = (page: Page, testId: string) =>
    page.evaluate((testId) => {
        const element = document.querySelector(`[data-testid="${testId}"]`);
        if (element === null) {
            return null;
        }
        const rect = element.getBoundingClientRect();
        return { x: rect.left, y: rect.top, width: rect.width, height: rect.height };
    }, testId);

const focused = (page: Page) =>
    page.evaluate(() => {
        const active = document.activeElement as HTMLElement | null;
        return active?.dataset.testid ?? active?.tagName.toLowerCase() ?? null;
    });

/// Arms the menu and picks an anchor, so the menu opens on that anchor.
async function openMenuOn(page: Page, index: number) {
    await page.getByTestId("arm-menu").click();
    const anchor = (await geometry(page)).anchors[index];
    const rect = await anchorRect(page, anchor);
    await page.mouse.click(rect.x + rect.width / 2, rect.y + rect.height / 2);
    await expect.poll(async () => (await geometry(page)).menu).toBe(index);
    return anchor;
}

test.describe("M4 V4 / R10: layers of one scope", () => {
    test("a menu opened from a GPU anchor sits on it, and stays on it while the page scrolls", async ({
        page,
    }) => {
        test.setTimeout(300_000);
        await mountGeometry(page);
        const anchor = await openMenuOn(page, 7);

        const check = async (label: string) => {
            const expected = await anchorRect(page, anchor);
            const menu = (await box(page, "geometry-menu"))!;
            expect(Math.abs(menu.x - expected.x), `${label} left`).toBeLessThanOrEqual(1);
            expect(
                Math.abs(menu.y - (expected.y + expected.height)),
                `${label} top`
            ).toBeLessThanOrEqual(1);
        };
        await check("when opened");

        // Both levels of scrolling move the anchor without changing it.
        for (const [inner, outer] of [
            [40, 30],
            [0, 120],
            [75, 0],
        ]) {
            await page.evaluate(([inner, outer]) => {
                (document.querySelector('[data-testid="geometry-inner"]') as HTMLElement).scrollTop = inner;
                (document.querySelector('[data-testid="geometry-outer"]') as HTMLElement).scrollTop = outer;
            }, [inner, outer]);
            await page.waitForTimeout(100);
            await check(`after scrolling ${inner}/${outer}`);
        }

        // And it is drawn above the region, not behind it.
        const menu = (await box(page, "geometry-menu"))!;
        const on_top = await page.evaluate(
            (point) => {
                const hit = document.elementFromPoint(point.x, point.y);
                return hit?.closest('[data-testid="geometry-menu"]') !== null;
            },
            { x: menu.x + menu.width / 2, y: menu.y + 4 }
        );
        expect(on_top).toBe(true);
    });

    test("a modal over the region takes a hundred clicks and passes none of them down", async ({
        page,
    }) => {
        test.setTimeout(300_000);
        await mountGeometry(page);
        await openMenuOn(page, 7);
        await page.getByTestId("menu-open-dialog").click();
        await expect.poll(async () => (await geometry(page)).dialog).toBe(true);

        const before = (await geometry(page)).hits;
        const anchor = (await geometry(page)).anchors[7];
        const rect = await anchorRect(page, anchor);
        const dialog = (await box(page, "geometry-dialog"))!;
        for (let click = 0; click < rounds(100); click++) {
            // Half on the dialog itself, half on the control it covers.
            if (click % 2 === 0) {
                await page.mouse.click(dialog.x + dialog.width / 2, dialog.y + 4);
            } else {
                await page.mouse.click(rect.x + rect.width / 2, rect.y + rect.height / 2);
            }
        }
        expect((await geometry(page)).hits).toBe(before);
        expect((await geometry(page)).dialog).toBe(true);
    });

    test("escape closes one layer at a time and hands focus back to what opened it", async ({
        page,
    }) => {
        await mountGeometry(page);
        await openMenuOn(page, 7);
        await page.getByTestId("menu-open-dialog").click();
        await expect.poll(async () => (await geometry(page)).dialog).toBe(true);
        // A modal layer takes focus; the rest of the scope cannot be reached.
        expect(await focused(page)).toBe("dialog-input");

        await page.keyboard.press("Escape");
        await expect.poll(async () => (await geometry(page)).dialog).toBe(false);
        // Only the top one: the menu is still open, and holds focus again.
        expect((await geometry(page)).menu).toBe(7);
        expect(await focused(page)).toBe("menu-open-dialog");

        await page.keyboard.press("Escape");
        await expect.poll(async () => (await geometry(page)).menu).toBeNull();
    });

    test("an open layer owns escape, and the application gets it back afterwards", async ({
        page,
    }) => {
        await mountGeometry(page);
        await page.getByTestId("focus-01").focus();
        // With nothing on the stack the key is the application's own command.
        await page.keyboard.press("Escape");
        await expect.poll(async () => (await geometry(page)).commands).toBe(1);

        await openMenuOn(page, 7);
        await page.getByTestId("menu-open-dialog").click();
        await expect.poll(async () => (await geometry(page)).dialog).toBe(true);

        // Two layers, two presses, two closes - and the command underneath
        // ran neither time.
        await page.keyboard.press("Escape");
        await expect.poll(async () => (await geometry(page)).dialog).toBe(false);
        await page.keyboard.press("Escape");
        await expect.poll(async () => (await geometry(page)).menu).toBeNull();
        expect((await geometry(page)).commands).toBe(1);

        await page.getByTestId("focus-01").focus();
        await page.keyboard.press("Escape");
        await expect.poll(async () => (await geometry(page)).commands).toBe(2);
    });

    test("a layer whose trigger is gone hands focus to the scope instead", async ({ page }) => {
        await mountGeometry(page);
        await page.getByTestId("anchor-host").click();
        await expect.poll(async () => (await geometry(page)).anchored).toBe(true);
        expect(await focused(page)).toBe("anchor-host");
        // The layer holds the keyboard when it closes, so it has one to hand
        // back.
        await page.getByTestId("anchor-layer-button").focus();

        // The element the layer is anchored to - and the control it was opened
        // from - leaves the document while the layer is open. Removed without
        // a real click, which would move focus by itself.
        await page.evaluate(() =>
            (document.querySelector('[data-testid="drop-anchor-host"]') as HTMLElement).click()
        );
        await expect.poll(async () => (await geometry(page)).anchored).toBe(false);
        await expect(page.getByTestId("anchor-layer")).toHaveCount(0);
        expect(await focused(page)).toBe("geometry");
    });

    test("a layer that no longer holds focus does not take it back", async ({ page }) => {
        await mountGeometry(page);
        await page.getByTestId("anchor-host").click();
        await expect.poll(async () => (await geometry(page)).anchored).toBe(true);

        // The user has moved on to another control; closing the layer must not
        // pull the keyboard back to where the layer was opened from.
        await page.getByTestId("focus-03").focus();
        await page.evaluate(() =>
            (document.querySelector('[data-testid="drop-anchor-host"]') as HTMLElement).click()
        );
        await expect.poll(async () => (await geometry(page)).anchored).toBe(false);
        expect(await focused(page)).toBe("focus-03");
    });

    test("a modal keeps the tab sequence inside itself, and gives it back after", async ({
        page,
    }) => {
        await mountGeometry(page);
        await page.getByTestId("focus-01").focus();
        await openMenuOn(page, 7);
        await page.getByTestId("menu-open-dialog").click();
        await expect.poll(async () => (await geometry(page)).dialog).toBe(true);

        const seen: (string | null)[] = [];
        for (let step = 0; step < 8; step++) {
            await page.keyboard.press("Tab");
            seen.push(await focused(page));
        }
        // Nothing behind the modal is reachable: the scope's content is inert.
        expect(seen.filter((id) => id?.startsWith("focus-"))).toEqual([]);

        await page.getByTestId("dialog-close").click();
        await expect.poll(async () => (await geometry(page)).dialog).toBe(false);
        await page.keyboard.press("Escape");
        await expect.poll(async () => (await geometry(page)).menu).toBeNull();
        // With every layer gone the scope answers the keyboard again.
        await page.getByTestId("focus-01").focus();
        await page.keyboard.press("Tab");
        expect(await focused(page)).toBe("focus-02");
    });
});

test.describe("M4 V4: focus belongs to whoever had it", () => {
    test("mounting a scope and starting its region take no focus from the host page", async ({
        page,
    }) => {
        await mountGeometry(page);
        await page.getByTestId("native-link").focus();
        expect(await focused(page)).toBe("native-link");

        // A second scope starts, with its own regions, while the host page has
        // the keyboard.
        await page.evaluate(() => window.__fusion_basic.dispose("scope-b"));
        await page.evaluate(() => window.__fusion_basic.mount("scope-b"));
        await page.waitForTimeout(500);
        expect(await focused(page)).toBe("native-link");

        // And nothing in the page is a hidden text box of Makepad's: the
        // regions do not compete for focus with a proxy of their own. Named
        // by its class rather than counted as "every textarea": B0 has one of
        // its own, and a control the application put there is not a proxy.
        expect(
            await page.evaluate(
                () => document.querySelectorAll("textarea.cx_webgl_textinput").length
            )
        ).toBe(0);
    });
});

test.describe("M4 V4: twenty focus items", () => {
    test("tab and shift-tab walk them in order, skipping the disabled one", async ({ page }) => {
        test.setTimeout(300_000);
        await mountGeometry(page);
        const ids = Array.from({ length: 20 }, (_, n) => `focus-${String(n + 1).padStart(2, "0")}`);
        // The disabled item is not in the sequence at all.
        const expected = ids.filter((id) => id !== "focus-07");

        await page.getByTestId(ids[0]).focus();
        const forward: (string | null)[] = [ids[0]];
        for (let step = 1; step < expected.length; step++) {
            await page.keyboard.press("Tab");
            forward.push(await focused(page));
        }
        expect(forward).toEqual(expected);

        const backward: (string | null)[] = [];
        for (let step = 1; step < expected.length; step++) {
            await page.keyboard.press("Shift+Tab");
            backward.push(await focused(page));
        }
        expect(backward).toEqual([...expected].reverse().slice(1));

        // The read-only item can be reached and read, and refuses the edit.
        await page.getByTestId("focus-12").focus();
        expect(await focused(page)).toBe("focus-12");
        await page.keyboard.type("edited");
        await expect(page.getByTestId("focus-12")).toHaveValue("read only");
    });
});
