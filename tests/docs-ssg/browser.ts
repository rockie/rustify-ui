import { BrowserContext, Page } from "@playwright/test";

export function firstFontFamily(node: Element) {
    return getComputedStyle(node).fontFamily.split(",")[0].trim().replace(/^['"]|['"]$/g, "");
}

export async function grantClipboard(context: BrowserContext) {
    const permissions = context.browser()?.browserType().name() === "chromium"
        ? ["clipboard-read", "clipboard-write"] : ["clipboard-read"];
    await context.grantPermissions(permissions);
}

export async function capture(page: Page, options: Parameters<Page["screenshot"]>[0]) {
    // WebKit's Playwright screenshot preparation inserts an inline body {}
    // stylesheet to synchronize animations, violating this fixture's CSP.
    // Keep its functional/CSP assertions; Chromium supplies the captures.
    if (page.context().browser()?.browserType().name() !== "webkit") await page.screenshot(options);
}
