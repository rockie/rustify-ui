import { expect, Page } from "@playwright/test";
import { rounds } from "../tier";
import { capture, differingPixels, settle, test, waitForReady } from "./support";

const snapshot = (page: Page) => page.evaluate(() => window.__property_workbench.snapshot());

const scopeStyle = (page: Page) =>
    page.evaluate(() => {
        const scope = document.querySelector("[data-rustify-scope]") as HTMLElement;
        const style = getComputedStyle(scope);
        return {
            theme: scope.getAttribute("data-theme"),
            background: style.getPropertyValue("--background").trim(),
            foreground: style.getPropertyValue("--foreground").trim(),
            radius: style.getPropertyValue("--radius").trim(),
        };
    });

const panelColours = (page: Page) =>
    page.evaluate(() => {
        const panel = document.querySelector(".workbench .panel") as HTMLElement;
        const style = getComputedStyle(panel);
        return { color: style.color, background: style.backgroundColor };
    });

const hostColours = (page: Page) =>
    page.evaluate(() => {
        const style = getComputedStyle(document.body);
        return {
            color: style.color,
            background: style.backgroundColor,
            theme: document.documentElement.getAttribute("data-theme"),
        };
    });

test.describe("M6 V7: one table of values for both halves of a scope", () => {
    test("switching the theme moves the DOM and the GPU together", async ({ page }) => {
        await waitForReady(page);
        const region = page.getByTestId("workbench-gpu");
        const light = await settle(region);
        expect(await snapshot(page)).toMatchObject({ theme: "light" });
        expect(await scopeStyle(page)).toMatchObject({
            theme: "light",
            background: "#f9fafb",
            foreground: "#1d2939",
            radius: "6px",
        });
        const lightPanel = await panelColours(page);

        await page.getByTestId("toggle-theme").click();
        await expect.poll(async () => (await snapshot(page)).theme).toBe("dark");
        expect(await scopeStyle(page)).toMatchObject({
            theme: "dark",
            background: "#0c111d",
            foreground: "#f9fafb",
        });
        // The panel actually changed, not only the tokens behind it.
        expect(await panelColours(page)).not.toEqual(lightPanel);
        // And so did what the region draws.
        await expect
            .poll(async () => differingPixels(light, await capture(region)), { timeout: 5_000 })
            .toBeGreaterThan(1_000);
    });

    test("the host page keeps its own appearance across twenty switches", async ({ page }) => {
        test.setTimeout(300_000);
        await waitForReady(page);
        const before = await hostColours(page);

        // An even number, so the run ends on the theme it started with.
        const switches = 2 * rounds(10);
        for (let round = 0; round < switches; round++) {
            await page.getByTestId("toggle-theme").click();
            await expect
                .poll(async () => (await snapshot(page)).theme)
                .toBe(round % 2 === 0 ? "dark" : "light");
        }
        // Twenty switches end where they started.
        expect(await snapshot(page)).toMatchObject({ theme: "light" });
        expect(await scopeStyle(page)).toMatchObject({ theme: "light", background: "#f9fafb" });
        // The tokens went on the scope, not on the document: the page around it
        // never carried a theme at all.
        expect(await hostColours(page)).toEqual(before);
        expect(before.theme).toBeNull();
    });

    test("the controls are still found by role and name after a switch", async ({ page }) => {
        await waitForReady(page);
        await page.getByTestId("toggle-theme").click();
        await expect.poll(async () => (await snapshot(page)).theme).toBe("dark");
        // A theme is a set of values, not a different set of controls.
        await expect(page.getByRole("textbox", { name: "name" })).toHaveCount(1);
        await expect(page.getByRole("button", { name: "next" })).toHaveCount(1);
        await expect(page.getByRole("status", { name: "find result" })).toHaveCount(1);
        await expect(page.getByRole("button", { name: "switch theme" })).toHaveCount(1);
    });
});
