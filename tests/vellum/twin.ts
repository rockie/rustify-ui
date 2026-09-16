import { existsSync } from "node:fs";
import path from "node:path";
import type { Page } from "@playwright/test";
import { expect, present, waitForQuiet } from "./support";

export const hasReference = existsSync(path.resolve(__dirname, "../../ref/Vellum-main/index.html"));

export async function openTwin(page: Page) {
    await page.goto("http://127.0.0.1:4180/?canvas");
    await page.waitForFunction(() => window.vellum?.ready);
    await expect.poll(() => page.evaluate(() => window.vellum.renderer.backend)).toBe("Canvas 2D");
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
