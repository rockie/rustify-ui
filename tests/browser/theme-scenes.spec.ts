import { test, expect, waitForReady } from "./support";
import type { ElementHandle, Locator, Page } from "@playwright/test";

const snapshot = (page: Page) => page.evaluate(() => window.__theme_studio.snapshot());
const dom = (page: Page) => page.getByTestId("dom-preview");

async function select(page: Page, trigger: Locator, listId: string, label: string) {
    await trigger.click();
    await page.getByTestId(`${listId}-list`).getByRole("option", { name: label, exact: true }).click();
}

async function scene(page: Page, name: string, id: string) {
    await select(page, page.getByTestId("preview-scene"), "preview-scene", name);
    await expect(dom(page).getByTestId(`scene-${id}`)).toBeVisible();
    return dom(page).getByTestId(`scene-${id}`);
}

async function rendered(page: Page) {
    await page.getByTestId("renderer-compare").click();
    await page.getByTestId("theme-gpu").scrollIntoViewIfNeeded();
    await expect.poll(async () => {
        const state = await snapshot(page);
        return state.gpu_state === "Ready" && state.drawn_revision === state.revision && state.samples.length > 50;
    }).toBe(true);
}

async function regionIds(page: Page) {
    return page.evaluate(() => [...(window.__theme_studio as any).hooks.regions.keys()]);
}

async function localState(page: Page) {
    const canvas = await page.getByTestId("theme-gpu").elementHandle();
    if (!canvas) throw new Error("the mounted GPU canvas is missing");
    return { state: await snapshot(page), regions: await regionIds(page), canvas };
}

async function unchangedTheme(page: Page, before: Awaited<ReturnType<typeof localState>>) {
    const after = await snapshot(page);
    expect(after.document).toEqual(before.state.document);
    expect(after.history).toEqual(before.state.history);
    expect(after.document_revision).toBe(before.state.document_revision);
    expect(after.resolve_count).toBe(before.state.resolve_count);
    expect(after.revision).toBe(before.state.revision);
    expect(after.region_count).toBe(1);
    expect(await regionIds(page)).toEqual(before.regions);
    await sameCanvas(before.canvas);
}

async function sameCanvas(canvas: ElementHandle<HTMLElement | SVGElement>) {
    expect(await canvas.evaluate(node => node.isConnected && node === document.querySelector('[data-testid="theme-gpu"]'))).toBe(true);
}

test.describe("local preview scenes", () => {
    test.beforeEach(async ({ page }) => {
        await waitForReady(page);
        await page.getByTestId("locale-en").click();
        await rendered(page);
        await page.getByTestId("renderer-dom").click();
    });

    test("Cards validates the form, cancels and confirms a local profile, and performs menu actions", async ({ page }) => {
        const before = await localState(page);
        const cards = await scene(page, "Cards", "cards");
        await expect(cards.getByTestId("cards-clear-draft")).toBeDisabled();
        await cards.getByTestId("cards-actions").click();
        const duplicate = page.getByTestId("cards-menu").getByTestId("menu-item-duplicate");
        await expect(duplicate).toBeDisabled();
        await expect(duplicate).toContainText("Enter a name first");
        await page.keyboard.press("Escape");
        await cards.getByTestId("cards-submit").click();
        await expect(cards.getByTestId("cards-error")).toHaveText("Enter a name.");
        await cards.getByTestId("cards-name").fill("Avery");
        await cards.getByTestId("cards-email").fill("incomplete@host");
        await cards.getByTestId("cards-submit").click();
        await expect(cards.getByTestId("cards-error")).toHaveText("Enter a valid email address.");
        await cards.getByTestId("cards-email").fill("avery@example.test");
        await cards.getByTestId("cards-submit").click();
        await expect(cards.getByTestId("cards-error")).toHaveText("Confirm the local-demo notice.");
        await cards.getByTestId("cards-terms").check();
        await cards.getByTestId("cards-submit").click();
        await expect(page.getByTestId("cards-dialog")).toBeVisible();
        await page.getByTestId("cards-cancel").click();
        await expect(page.getByTestId("cards-dialog")).toHaveCount(0);
        await expect(cards.getByTestId("cards-success")).toBeEmpty();
        await cards.getByTestId("cards-submit").click();
        await page.getByTestId("cards-confirm").click();
        await expect(cards.getByTestId("cards-success")).toHaveText("Local profile created for Avery");
        await cards.getByTestId("cards-notifications").click();
        await expect(cards.getByTestId("cards-notifications")).toHaveAttribute("aria-checked", "false");
        await expect(cards.getByTestId("cards-notification-status")).toContainText("paused");
        await cards.getByTestId("cards-actions").click();
        await page.getByTestId("cards-menu").getByRole("menuitem", { name: "Duplicate local profile", exact: true }).click();
        await expect(cards.getByTestId("cards-name")).toHaveValue("Avery (copy)");
        await expect(cards.getByTestId("cards-action-status")).toContainText("copy is ready");
        await cards.getByTestId("cards-actions").click();
        await page.getByTestId("cards-menu").getByRole("menuitem", { name: "Clear form", exact: true }).click();
        await expect(cards.getByTestId("cards-name")).toHaveValue("");
        await expect(cards.getByTestId("cards-email")).toHaveValue("");
        await expect(cards.getByTestId("cards-terms")).not.toBeChecked();
        await expect(cards.getByTestId("cards-success")).toBeEmpty();
        await expect(cards.getByTestId("cards-clear-draft")).toBeDisabled();
        await unchangedTheme(page, before);
    });

    test("Dashboard changes real chart values and restores an empty result", async ({ page }) => {
        const before = await localState(page);
        const dashboard = await scene(page, "Dashboard", "dashboard");
        await expect(dashboard.getByTestId("dashboard-total")).toHaveText("150");
        await expect(dashboard.getByTestId("dashboard-bars").locator("rect")).toHaveCount(7);
        await select(page, dashboard.getByTestId("dashboard-period"), "dashboard-period", "Last 30 days");
        await expect(dashboard.getByTestId("dashboard-total")).toHaveText("450");
        await expect(dashboard.getByTestId("dashboard-data").locator("td")).toHaveText(["42", "58", "37", "76", "65", "90", "82"]);
        await expect(dashboard.getByTestId("dashboard-bars").locator("rect").first()).toHaveAttribute("height", "42");
        await select(page, dashboard.getByTestId("dashboard-period"), "dashboard-period", "Last 90 days");
        await expect(dashboard.getByTestId("dashboard-total")).toHaveText("572");
        await dashboard.getByTestId("dashboard-empty-toggle").check();
        await expect(dashboard.getByTestId("dashboard-total")).toHaveText("0");
        await expect(dashboard.getByTestId("dashboard-empty")).toBeVisible();
        await expect(dashboard.getByTestId("dashboard-bars")).toHaveCount(0);
        await dashboard.getByTestId("dashboard-empty-toggle").uncheck();
        await expect(dashboard.getByTestId("dashboard-total")).toHaveText("572");
        await expect(dashboard.getByTestId("dashboard-data").locator("tr")).toHaveCount(7);
        await unchangedTheme(page, before);
    });

    test("Application saves valid settings and keeps the previous project after an invalid save", async ({ page }) => {
        const before = await localState(page);
        const application = await scene(page, "Application", "application");
        await application.getByTestId("application-settings").click();
        await application.getByTestId("application-project-name").fill("   ");
        await application.getByTestId("application-save").click();
        await expect(application.getByTestId("application-save-status")).toHaveText("Enter a project name before saving.");
        await application.getByTestId("application-project-name").fill("  Studio review  ");
        await application.getByTestId("application-digest").click();
        await expect(application.getByTestId("application-digest")).toHaveAttribute("aria-checked", "false");
        await application.getByTestId("application-save").click();
        await expect(application.getByTestId("application-save-status")).toHaveText("Saved in this preview: Studio review");
        await application.getByTestId("application-projects").click();
        await expect(application.getByTestId("application-project").getByRole("heading", { name: "Studio review", exact: true })).toBeVisible();
        await application.getByTestId("application-edit-project").click();
        await expect(application.getByTestId("application-digest")).toHaveAttribute("aria-checked", "false");
        await application.getByTestId("application-project-name").fill("");
        await application.getByTestId("application-save").click();
        await application.getByTestId("application-projects").click();
        await expect(application.getByTestId("application-project")).toContainText("Studio review");
        await unchangedTheme(page, before);
    });

    test("Marketing uses keyboard billing tabs and records the selected plan's billing period", async ({ page }) => {
        const before = await localState(page);
        const marketing = await scene(page, "Marketing", "marketing");
        await expect(marketing.getByTestId("marketing-price-pro")).toHaveText("$32/ month");
        await marketing.getByTestId("tab-monthly").focus();
        await page.keyboard.press("ArrowRight");
        await expect(marketing.getByTestId("tab-yearly")).toHaveAttribute("aria-selected", "true");
        await expect(marketing.getByTestId("marketing-price-pro")).toHaveText("$24/ month");
        await marketing.getByTestId("marketing-choose-pro").click();
        await expect(marketing.getByTestId("marketing-status")).toHaveText("Selected locally: Pro · yearly");
        await marketing.getByTestId("tab-monthly").click();
        await expect(marketing.getByTestId("marketing-price-pro")).toHaveText("$32/ month");
        await expect(marketing.getByTestId("marketing-status")).toHaveText("Selected locally: Pro · yearly");
        await marketing.getByTestId("marketing-clear").click();
        await expect(marketing.getByTestId("marketing-status")).toBeEmpty();
        await expect(marketing.getByTestId("marketing-clear")).toHaveCount(0);
        await unchangedTheme(page, before);
    });

    test("Mail searches, reads and archives locally, then restores all messages", async ({ page }) => {
        const before = await localState(page);
        const mail = await scene(page, "Mail", "mail");
        await expect(mail.getByTestId("mail-message")).toContainText("Select a message");
        await mail.getByTestId("mail-search").fill("no-such-message-6947");
        await expect(mail.getByTestId("mail-empty")).toBeVisible();
        await mail.getByTestId("mail-search").fill("SAM");
        await expect(mail.getByTestId("mail-list").locator('[data-testid^="mail-open-"]')).toHaveCount(1);
        await mail.getByTestId("mail-open-2").click();
        await expect(mail.getByTestId("mail-message")).toContainText("Review the dashboard");
        await expect(mail.getByTestId("mail-message")).toContainText("No message was sent online.");
        await mail.getByTestId("mail-archive").click();
        await expect(mail.getByTestId("mail-status")).toContainText("archived");
        await expect(mail.getByTestId("mail-empty")).toBeVisible();
        await mail.getByTestId("mail-restore").click();
        await expect(mail.getByTestId("mail-search")).toHaveValue("");
        await expect(mail.getByTestId("mail-list").locator('[data-testid^="mail-open-"]')).toHaveCount(3);
        for (const id of [1, 2, 3]) {
            await mail.getByTestId(`mail-open-${id}`).click();
            await mail.getByTestId("mail-archive").click();
        }
        await expect(mail.getByTestId("mail-empty")).toBeVisible();
        await mail.getByTestId("mail-restore").click();
        await expect(mail.getByTestId("mail-list").locator('[data-testid^="mail-open-"]')).toHaveCount(3);
        await unchangedTheme(page, before);
    });

    test("Typography updates three actual font samples and handles empty multilingual text", async ({ page }) => {
        const before = await localState(page);
        const typography = await scene(page, "Typography", "typography");
        const text = "Theme 中文 cafe\u0301 fi 👩‍👩‍👧‍👦";
        await typography.getByTestId("typography-input").fill(text);
        await select(page, typography.getByTestId("typography-size"), "typography-size", "48px");
        const families: string[] = [];
        for (const slot of ["sans", "serif", "mono"]) {
            const sample = typography.getByTestId(`typography-${slot}`);
            await expect(sample).toHaveText(text);
            const style = await sample.evaluate(node => ({ family: getComputedStyle(node).fontFamily, size: getComputedStyle(node).fontSize }));
            expect(style.size).toBe("48px");
            families.push(style.family);
        }
        expect(new Set(families).size).toBe(3);
        await typography.getByTestId("typography-input").fill("");
        await expect(typography.getByTestId("typography-empty-note")).toContainText("Enter text");
        for (const slot of ["sans", "serif", "mono"]) await expect(typography.getByTestId(`typography-${slot}`)).toBeEmpty();
        await page.getByTestId("locale-zh").click();
        await expect(typography.getByTestId("typography-empty-note")).toHaveText("请输入文本，比较三种字体槽位。");
        await typography.getByTestId("typography-input").fill("再看一次 中文");
        await expect(typography.getByTestId("typography-serif")).toHaveText("再看一次 中文");
        await unchangedTheme(page, before);
    });

    test("Color Palette shows every semantic color, changes formats and exposes exact copy text", async ({ page }) => {
        const before = await localState(page);
        const palette = await scene(page, "Color palette", "palette");
        await expect(palette.locator("article[data-theme-tokens]")).toHaveCount(35);
        const authors = before.state.document.styles.light;
        for (const token of Object.keys(authors).filter(token => paletteTokens.has(token))) {
            await expect(palette.getByTestId(`palette-${token}`)).toHaveAttribute("data-theme-tokens", token);
            await expect(palette.getByTestId(`palette-value-${token}`)).not.toBeEmpty();
        }
        await expect(palette.getByTestId("palette-value-primary")).toHaveText(/^#[0-9a-f]{6,8}$/i);
        for (const [label, prefix] of [["RGB", /^rgb\(/], ["HSL", /^hsl\(/], ["OKLCH", /^oklch\(/]] as const) {
            await select(page, palette.getByTestId("palette-format"), "palette-format", label);
            await expect(palette.getByTestId("palette-value-primary")).toHaveText(prefix);
        }
        await palette.getByTestId("palette-copy-token-primary").click();
        await expect(palette.getByTestId("palette-copy-text")).toHaveValue("--primary");
        await expect(palette.getByTestId("palette-copy-status")).toHaveText(/Copied to clipboard\.|Clipboard unavailable\./);
        const value = await palette.getByTestId("palette-value-primary").innerText();
        await palette.getByTestId("palette-copy-value-primary").click();
        await expect(palette.getByTestId("palette-copy-text")).toHaveValue(value);
        await expect(palette.getByTestId("palette-copy-text")).toHaveAttribute("readonly", "");
        await expect(palette.getByTestId("palette-copy-status")).toHaveText(/Copied to clipboard\.|Clipboard unavailable\./);
        await unchangedTheme(page, before);
    });

    test("renderer and preview width switches keep the same region, and focus mode restores keyboard focus", async ({ page }) => {
        const before = await localState(page);
        await page.getByTestId("renderer-gpu").click();
        await expect(page.getByTestId("dom-preview")).toBeHidden();
        await expect(page.getByTestId("gpu-preview")).toBeVisible();
        await rendered(page);
        await expect(page.getByTestId("dom-preview")).toBeVisible();
        await expect(page.getByTestId("gpu-preview")).toBeVisible();
        await select(page, page.getByTestId("preview-width"), "preview-width", "390 px");
        await expect(page.getByTestId("preview-workbench")).toHaveAttribute("data-width", "mobile");
        await expect.poll(async () => page.getByTestId("dom-preview").evaluate(node => getComputedStyle(node).width)).toBe("390px");
        expect((await snapshot(page)).preferences.preview_width).toBe("mobile");
        await page.getByTestId("preview-focus").click();
        await expect(page.locator(".studio-previews")).toHaveAttribute("data-fullscreen", "true");
        await expect(page.getByTestId("preview-workbench")).toBeFocused();
        await page.keyboard.press("Escape");
        await expect(page.locator(".studio-previews")).toHaveAttribute("data-fullscreen", "false");
        await expect(page.getByTestId("preview-focus")).toBeFocused();
        await select(page, page.getByTestId("preview-width"), "preview-width", "Responsive");
        await expect(page.getByTestId("preview-workbench")).toHaveAttribute("data-width", "responsive");
        await unchangedTheme(page, before);
    });
});

const paletteTokens = new Set([
    "background", "foreground", "card", "card-foreground", "popover", "popover-foreground",
    "primary", "primary-foreground", "secondary", "secondary-foreground", "muted", "muted-foreground",
    "accent", "accent-foreground", "destructive", "destructive-foreground", "border", "input", "ring",
    "chart-1", "chart-2", "chart-3", "chart-4", "chart-5", "sidebar", "sidebar-foreground",
    "sidebar-primary", "sidebar-primary-foreground", "sidebar-accent", "sidebar-accent-foreground",
    "sidebar-border", "sidebar-ring", "success", "warning", "shadow-color",
]);

test.describe("narrow workbench panels", () => {
    test.use({ viewport: { width: 390, height: 844 } });
    test("mobile navigation reveals the same preview and returns to the editor", async ({ page }) => {
        await waitForReady(page);
        await expect(page.getByTestId("editor-shell")).toBeVisible();
        await expect(page.getByTestId("preview-workbench")).toBeHidden();
        await expect(page.getByTestId("editor-separator")).toBeHidden();
        await page.getByTestId("mobile-preview").click();
        await expect(page.getByTestId("editor-shell")).toBeHidden();
        await expect(page.getByTestId("preview-workbench")).toBeVisible();
        await rendered(page);
        const canvas = await localState(page);
        await scene(page, "Mail", "mail");
        await expect(dom(page).getByTestId("mail-open-1")).toBeVisible();
        await page.getByTestId("preview-back-edit").click();
        await expect(page.getByTestId("editor-shell")).toBeVisible();
        await expect(page.getByTestId("preview-workbench")).toBeHidden();
        await page.getByTestId("mobile-preview").click();
        await expect(dom(page).getByTestId("scene-mail")).toBeVisible();
        await unchangedTheme(page, canvas);
    });
});
