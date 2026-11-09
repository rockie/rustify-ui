import { test, expect, REGION_START_MS } from "./support";
import type { Page, TestInfo } from "@playwright/test";
import { mkdir, writeFile } from "node:fs/promises";
import { EVIDENCE } from "../tier";
import { capture, fitsWidth, inViewport, nativeZoom, NATIVE_MISSING, NATIVE_SKIP } from "../native-zoom";

const OUT = "docs/validation/dom-components-p5";
const preview = (page: Page) => page.getByTestId("studio-preview");

async function select(page: Page, id: string, label: string) {
    await page.getByTestId(id).click();
    await page.getByTestId(`${id}-list`).getByRole("option", { name: label, exact: true }).click();
}

async function audit(page: Page) {
    const problems: string[] = [];
    page.on("pageerror", error => problems.push(`pageerror ${error.message}`));
    page.on("console", message => { if (message.type() === "error") problems.push(`console ${message.text()}`); });
    page.on("requestfailed", request => problems.push(`requestfailed ${request.url()} ${request.failure()?.errorText}`));
    page.on("response", response => { if (response.status() >= 400) problems.push(`${response.status()} ${response.url()}`); });
    await page.addInitScript(() => document.addEventListener("securitypolicyviolation", event => {
        const target = event.target as Element;
        console.error(`csp ${event.violatedDirective} ${event.blockedURI} ${target?.outerHTML?.slice(0, 160)}`);
    }));
    return problems;
}

/// The extension durations the scope's motion token drives.
const durations = (page: Page) => preview(page).evaluate(root => Object.fromEntries(["PanelTabFrame", "DropZone"].map(name => [name,
    [...root.querySelectorAll(`[data-name="${name}"]`)].map(node => getComputedStyle(node).transitionDuration)])));

/// Preset → fine edit → component gallery → Dialog, Select and CommandPalette →
/// export, the Studio's main path, with each floating surface inside the
/// window and the page never scrolling sideways.
async function studioPath(page: Page, info: TestInfo, prefix: string) {
    await page.goto("./");
    await expect(page.getByTestId("status")).toHaveAttribute("data-status", "ready", { timeout: REGION_START_MS });
    await page.getByTestId("preset-search").fill("Graphite");
    await select(page, "preset-select", "Built-in · Graphite");
    await page.getByTestId("editor-colors").click();
    await page.getByTestId("color-token-primary").click();
    await page.getByTestId("color-value").fill("#8844aa");
    await page.getByTestId("color-value").press("Enter");
    await expect.poll(() => page.evaluate(() => window.__theme_studio.snapshot().values.light.primary)).toBe("#8844aa");
    await fitsWidth(page);

    await page.getByTestId("renderer-dom").click();
    await page.getByTestId("preview-scene").click();
    await page.getByTestId("preview-scene-list").getByRole("option", { name: "Components", exact: true }).click();
    const gallery = page.getByTestId("dom-preview").getByTestId("component-gallery");
    await expect(gallery).toBeVisible();

    const dialog = gallery.locator('[data-gallery-entry="Dialog"]');
    await dialog.scrollIntoViewIfNeeded();
    const dialogTrigger = dialog.locator("[aria-haspopup]").first();
    await dialogTrigger.click();
    await expect(preview(page).locator('[data-rustify-part="dialog-content"]:visible')).toHaveCount(1);
    await inViewport(page, '[data-rustify-part="dialog-content"]:visible');
    await capture(page, `${OUT}/${prefix}-dialog.png`);
    await page.keyboard.press("Escape");
    await expect(preview(page).locator('[data-rustify-part="dialog-content"]:visible')).toHaveCount(0);
    await expect(dialogTrigger).toBeFocused();

    const choice = gallery.locator('[data-gallery-entry="Select"]');
    await choice.scrollIntoViewIfNeeded();
    const selectTrigger = choice.locator('[data-rustify-part="select-trigger"]');
    await selectTrigger.click();
    await expect(preview(page).locator('[data-rustify-part="select-content"]:visible')).toHaveCount(1);
    await inViewport(page, '[data-rustify-part="select-content"]:visible');
    await page.keyboard.press("Escape");
    await expect(selectTrigger).toBeFocused();

    const table = gallery.locator('[data-gallery-entry="DataTable"]');
    await table.scrollIntoViewIfNeeded();
    const grid = table.getByTestId("gallery-table");
    const scroll = await grid.evaluate(node => {
        const scroller = [node, ...node.querySelectorAll("*")].find(child => child.scrollWidth > child.clientWidth + 1 && /auto|scroll/.test(getComputedStyle(child).overflowX));
        return scroller ? { scrollWidth: scroller.scrollWidth, clientWidth: scroller.clientWidth } : null;
    });
    await fitsWidth(page);

    const commands = gallery.locator('[data-gallery-entry="CommandPalette"]');
    await commands.scrollIntoViewIfNeeded();
    await commands.getByRole("button", { name: "Open the command palette", exact: true }).click();
    const palette = preview(page).getByTestId("gallery-command");
    await expect(palette).toBeVisible();
    await inViewport(page, '[data-testid="gallery-command"]');
    await capture(page, `${OUT}/${prefix}-command.png`);
    await palette.getByTestId("command-search").fill("save");
    await palette.getByRole("option", { name: "Save the file", exact: true }).click();
    await expect(commands.locator("output")).toHaveText("save");
    const motion = await durations(page);

    await page.getByTestId("theme-export").focus();
    await page.keyboard.press("Enter");
    await expect(page.getByTestId("export-source")).toBeVisible();
    await page.getByTestId("export-select").click();
    expect(await page.getByTestId("export-source").evaluate((node: HTMLTextAreaElement) => node.selectionEnd - node.selectionStart))
        .toBe((await page.getByTestId("export-source").inputValue()).length);
    expect(await page.getByTestId("export-source").inputValue()).toContain("#8844aa");
    await capture(page, `${OUT}/${prefix}-export.png`);
    await page.keyboard.press("Escape");
    await expect(page.getByTestId("theme-export")).toBeFocused();
    await fitsWidth(page);
    await info.attach(`${prefix}-path`, { body: JSON.stringify({ tableScroll: scroll, motion }), contentType: "application/json" });
    return { tableScroll: scroll, motion };
}

test.describe("Studio at real Chrome 200% and reduced motion", { tag: EVIDENCE }, () => {
    test("real Chrome 200% completes preset, edit, gallery overlays and export", async ({ playwright }, info) => {
        test.skip(NATIVE_MISSING, NATIVE_SKIP);
        test.setTimeout(300_000);
        const zoom = await nativeZoom(playwright.chromium, info);
        try {
            const problems = await audit(zoom.page);
            const path = await studioPath(zoom.page, info, "native-200-studio");
            expect(path.tableScroll, "the data table scrolls inside its own region").not.toBeNull();
            expect(path.motion.PanelTabFrame.every(value => value !== "0s"), "without a motion preference tabs still transition").toBe(true);
            expect(problems).toEqual([]);
            await mkdir(OUT, { recursive: true });
            await writeFile(`${OUT}/native-200-studio.json`, JSON.stringify({ environment: zoom.environment, ...path }, null, 2));
        } finally { await zoom.close(); }
    });

    test.describe(() => {
        test.use({ reducedMotion: "reduce" });
        test("reduced motion completes the same path with no extension transition", async ({ page, browser }, info) => {
            test.setTimeout(300_000);
            const problems = await audit(page);
            const path = await studioPath(page, info, "reduced-motion-studio");
            expect(path.motion.PanelTabFrame.length).toBeGreaterThan(0);
            expect(path.motion.DropZone.length).toBeGreaterThan(0);
            for (const values of Object.values(path.motion)) expect(values.every(value => value === "0s"), JSON.stringify(path.motion)).toBe(true);
            expect(problems).toEqual([]);
            await mkdir(OUT, { recursive: true });
            await writeFile(`${OUT}/reduced-motion-studio.json`, JSON.stringify({ browser: browser.version(), executable: process.env.RUSTIFY_CHROMIUM ?? "bundled", ...path }, null, 2));
        });
    });
});
