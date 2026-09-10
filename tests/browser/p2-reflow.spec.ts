import { expect, Page, test } from "@playwright/test";

import { CATALOG_SIZE, waitForReady } from "./support";

/// M8: the catalogue at 400%, without losing content.
///
/// 400% zoom of a 1280-pixel window is a 320-pixel CSS viewport, which is how
/// WCAG's reflow criterion defines the test. What it asks is that content
/// reflows rather than requiring the reader to scroll in two directions, and
/// that nothing disappears in the process.

const snapshot = (page: Page) => page.evaluate(() => window.__component_catalog.snapshot());

test.describe("M8: the catalogue reflows at 400%", () => {
    test.describe.configure({ mode: "serial" });

    test("no two-dimensional scrolling at 320 CSS pixels", async ({ page }) => {
        await waitForReady(page);
        await page.setViewportSize({ width: 320, height: 720 });
        // Give the layout a frame to settle before measuring it.
        await expect.poll(async () => (await snapshot(page)).categories).toBe(CATALOG_SIZE);

        const overflow = await page.evaluate(() => {
            const root = document.documentElement;
            const widest = [...document.querySelectorAll<HTMLElement>("body *")]
                .map((element) => ({
                    name:
                        element.getAttribute("data-testid") ??
                        `${element.tagName.toLowerCase()}.${element.className}`.slice(0, 60),
                    right: element.getBoundingClientRect().right,
                }))
                .filter((entry) => entry.right > root.clientWidth + 1)
                .sort((a, b) => b.right - a.right)
                .slice(0, 5);
            return {
                documentWidth: root.scrollWidth,
                viewportWidth: root.clientWidth,
                widest,
            };
        });

        expect(
            overflow.documentWidth,
            `widest offenders: ${JSON.stringify(overflow.widest)}`
        ).toBeLessThanOrEqual(overflow.viewportWidth + 1);
    });

    test("every category is still reachable, and the page still switches", async ({ page }) => {
        await waitForReady(page);
        await page.setViewportSize({ width: 320, height: 720 });
        await expect.poll(async () => (await snapshot(page)).categories).toBe(CATALOG_SIZE);

        // Reflow means the content moved, not that some of it went away.
        const nav = page.locator("nav.catalogue-nav li button");
        expect(await nav.count()).toBe(CATALOG_SIZE + 2);
        for (const control of ["toggle-theme", "toggle-locale", "nav-status", "nav-samples"]) {
            await expect(page.getByTestId(control), control).toBeVisible();
        }

        // And it is still a working page at that width.
        await page.getByTestId("nav-samples").click();
        await expect.poll(async () => (await snapshot(page)).path).toBe("/samples");
        await expect(page.getByTestId("samples-page")).toBeVisible();

        await page.getByTestId("nav-status").click();
        await expect.poll(async () => (await snapshot(page)).path).toBe("/status");
        await expect(page.getByTestId("status-table")).toBeVisible();
    });
});
