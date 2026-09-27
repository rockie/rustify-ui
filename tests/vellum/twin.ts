import { existsSync } from "node:fs";
import path from "node:path";
import type { Page } from "@playwright/test";
import { expect, present, waitForQuiet } from "./support";

/// What port 4180 serves, if anything: the original JavaScript Vellum from
/// `ref/`, or a build of this example from before a change (a worktree named
/// by `VELLUM_BASELINE_DIR`), which is what a refactor is compared against.
export type TwinSource = "original" | "baseline";

function twinSource(): TwinSource | null {
    const original = existsSync(path.resolve(__dirname, "../../ref/Vellum-main/index.html"));
    const baseline = process.env.VELLUM_BASELINE_DIR;
    switch (process.env.VELLUM_TWIN) {
        case "original":
            return original ? "original" : null;
        case "baseline":
            if (!baseline) {
                throw new Error("VELLUM_TWIN=baseline needs VELLUM_BASELINE_DIR, a worktree with a release build of the example");
            }
            return "baseline";
        case undefined:
        case "":
            return original ? "original" : null;
        default:
            throw new Error(`VELLUM_TWIN=${process.env.VELLUM_TWIN}: use original or baseline`);
    }
}

export const twin = twinSource();

/// Whether any twin is served. Comparisons that only make sense against the
/// original implementation check `twin === "original"` instead.
export const hasReference = twin !== null;

export async function openTwin(page: Page) {
    if (twin === "baseline") {
        // The same renderer as the page under test; it is the same example.
        await page.goto("http://127.0.0.1:4180/");
        await page.waitForFunction(() => window.vellum?.ready);
        await expect.poll(() => page.evaluate(() => window.vellum.renderer.backend), { timeout: 60_000 }).toBe("Makepad WebGL2");
        await expect.poll(() => page.evaluate(() => window.vellum.renderer.instanceCount), { timeout: 60_000 }).toBeGreaterThan(150);
    } else {
        await page.goto("http://127.0.0.1:4180/?canvas");
        await page.waitForFunction(() => window.vellum?.ready);
        await expect.poll(() => page.evaluate(() => window.vellum.renderer.backend)).toBe("Canvas 2D");
    }
    await waitForQuiet(page);
}

export async function prepareVisual(page: Page, pageIndex: number, theme: "light" | "dark") {
    await page.evaluate(() => window.vellum.actions.resetStarter());
    await page.locator("[data-page]").nth(pageIndex).click();
    await page.evaluate(theme => {
        const scope = document.querySelector(".vellum") ?? document.documentElement;
        if (scope.getAttribute("data-theme") !== theme) window.vellum.actions.theme();
        window.vellum.select([]);
        window.vellum.fit();
        if (window.vellum.actions.blankBadges) window.vellum.actions.blankBadges();
        for (const id of ["toast", "welcome-tip"]) {
            const element = document.getElementById(id);
            if (element) element.style.display = "none";
        }
    }, theme);
    await present(page);
    await waitForQuiet(page);
    await page.evaluate(() => {
        for (const id of ["engine-label", "performance"]) {
            const element = document.getElementById(id);
            if (element) element.textContent = "Renderer";
        }
    });
}
