// Chrome's own 200% page zoom, not CSS text zoom or device emulation.
//
// Chrome keeps the default zoom in the profile as a level where the factor is
// 1.2^level, so the level is written before launch and the browser opens every
// page at 200%. `viewport: null` keeps Playwright from emulating a viewport
// over it. Only an installed Chrome named by RUSTIFY_CHROMIUM is used, so a
// run without one skips instead of measuring a different browser.
import { expect, type BrowserContext, type BrowserType, type Page, type TestInfo } from "@playwright/test";
import { execFileSync } from "node:child_process";
import { existsSync } from "node:fs";
import { mkdir, mkdtemp, rm, writeFile } from "node:fs/promises";
import { tmpdir } from "node:os";
import { join } from "node:path";

const LEVEL_200 = Math.log(2) / Math.log(1.2);
const WINDOW = { width: 1280, height: 800 };

export const NATIVE_CHROME = process.env.RUSTIFY_CHROMIUM ?? "";
export const NATIVE_MISSING = NATIVE_CHROME === "" || !existsSync(NATIVE_CHROME);
export const NATIVE_SKIP = "real Chrome 200% needs RUSTIFY_CHROMIUM set to an installed Chrome executable";

export type NativeZoom = {
    context: BrowserContext;
    page: Page;
    environment: { executable: string; version: string; window: typeof WINDOW; innerWidth: number; devicePixelRatio: number };
    close(): Promise<void>;
};

export async function nativeZoom(chromium: BrowserType, info: TestInfo, options: { reducedMotion?: "reduce" | "no-preference" } = {}): Promise<NativeZoom> {
    const profile = await mkdtemp(join(tmpdir(), "rustify-native-zoom-"));
    await mkdir(join(profile, "Default"));
    await writeFile(join(profile, "Default", "Preferences"), JSON.stringify({ partition: { default_zoom_level: { x: LEVEL_200 } } }));
    const context = await chromium.launchPersistentContext(profile, {
        executablePath: NATIVE_CHROME,
        headless: info.project.use.headless ?? true,
        // The test runner adds the project's context options; a null viewport
        // takes no scale factor of its own.
        viewport: null, deviceScaleFactor: undefined,
        baseURL: info.project.use.baseURL,
        reducedMotion: options.reducedMotion ?? "no-preference",
        args: [`--window-size=${WINDOW.width},${WINDOW.height}`],
    }).catch(async error => { await rm(profile, { recursive: true, force: true }); throw error; });
    const close = async () => { await context.close(); await rm(profile, { recursive: true, force: true }); };
    try {
        const page = context.pages()[0] ?? await context.newPage();
        await page.goto("about:blank");
        const measured = await page.evaluate(() => ({ innerWidth, devicePixelRatio }));
        expect(measured, "the profile's default zoom must give a real 200% page").toEqual({ innerWidth: WINDOW.width / 2, devicePixelRatio: 2 });
        const version = execFileSync(NATIVE_CHROME, ["--version"], { encoding: "utf8" }).trim();
        return { context, page, environment: { executable: NATIVE_CHROME, version, window: WINDOW, ...measured }, close };
    } catch (error) {
        await close();
        throw error;
    }
}

/// The page fits its zoomed window: no page-level horizontal scroll.
export async function fitsWidth(page: Page) {
    expect(await page.evaluate(() => document.documentElement.scrollWidth - innerWidth), "page-level horizontal overflow").toBeLessThanOrEqual(1);
}

/// A floating surface lies inside the visual viewport.
export async function inViewport(page: Page, selector: string) {
    const box = await page.locator(selector).last().evaluate(node => {
        const rect = node.getBoundingClientRect();
        return { left: rect.left, top: rect.top, right: rect.right, bottom: rect.bottom, width: innerWidth, height: innerHeight };
    });
    expect(box.left, `${selector} left`).toBeGreaterThanOrEqual(-1);
    expect(box.top, `${selector} top`).toBeGreaterThanOrEqual(-1);
    expect(box.right, `${selector} right`).toBeLessThanOrEqual(box.width + 1);
    expect(box.bottom, `${selector} bottom`).toBeLessThanOrEqual(box.height + 1);
}

/// The whole page as the browser shows it. `page.screenshot` sizes its clip in
/// CSS pixels, which browser zoom scales again, so it crops a zoomed page.
export async function capture(page: Page, path: string) {
    const cdp = await page.context().newCDPSession(page);
    try {
        const { data } = await cdp.send("Page.captureScreenshot", { format: "png" });
        await writeFile(path, Buffer.from(data, "base64"));
    } finally { await cdp.detach(); }
}
