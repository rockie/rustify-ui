import { expect, Locator, Page } from "@playwright/test";

import { test } from "./support";

/// The toast, the two draft fields, the toggle group, and what the menu and
/// the dialog gained: their keyboard paths, their roles and states, and the
/// one exception to controlled values - a focused field keeps what is being
/// typed, and everything else shows what the application holds.

interface Snapshot {
    theme: string;
    locale: string;
    actions: number;
    menu: boolean;
    dialog: boolean;
    dialog_closes: number;
    command: string;
    context_menu: boolean;
    number: number;
    colour: string;
    tint: string;
    requests: string[];
    align: string[];
    styles: string[];
    tone: string;
    toast: { seq: number; message: string; tone: string } | null;
}

const snapshot = (page: Page) =>
    page.evaluate(() => window.__component_catalog.snapshot()) as Promise<Snapshot>;

async function open(page: Page, category: string) {
    await page.getByTestId(`nav-${category}`).click();
    await expect(page.getByTestId("category-name")).toHaveText(category.replace(/-/g, " "));
}

const example = (page: Page) => page.getByTestId("example");

async function toDark(page: Page) {
    await page.getByTestId("toggle-theme").click();
    await expect(page.locator("[data-rustify-scope]").first()).toHaveAttribute("data-theme", "dark");
}

const computed = (locator: Locator, property: string) =>
    locator.evaluate(
        (element, property) => getComputedStyle(element).getPropertyValue(property),
        property
    );

/// A token's value as `getComputedStyle` writes a colour.
function rgb(hex: string): string {
    const value = parseInt(hex.slice(1), 16);
    return `rgb(${(value >> 16) & 255}, ${(value >> 8) & 255}, ${value & 255})`;
}

/// The light and dark values of the tokens these checks read
/// (`crates/rustify-ui/src/theme.rs`).
const TOKENS = {
    light: { primary: "#1570ef", popover: "#ffffff", input: "#ffffff", border: "#858f9e" },
    dark: { primary: "#53b1fd", popover: "#1d2939", input: "#101828", border: "#717c8e" },
};

test.describe("number field", () => {
    const field = (page: Page) =>
        example(page).getByRole("spinbutton", { name: "a quantity", exact: true });

    test("a spinbutton that announces its bounds and the application's value", async ({ page }) => {
        await open(page, "number-field");
        const number = field(page);
        await expect(number).toHaveValue("40");
        await expect(number).toHaveAttribute("aria-valuenow", "40");
        await expect(number).toHaveAttribute("aria-valuemin", "0");
        await expect(number).toHaveAttribute("aria-valuemax", "100");
        await expect(number).not.toHaveAttribute("aria-invalid");
        await expect(page.getByTestId("invalid-number")).toHaveAttribute("aria-invalid", "true");
    });

    test("a focused draft is kept while the application refuses it, and leaving settles on what it holds", async ({
        page,
    }) => {
        await open(page, "number-field");
        const number = field(page);
        await number.fill("150");
        // Refused - it is out of bounds - and still there to be corrected.
        await expect(number).toHaveValue("150");
        expect(await snapshot(page)).toMatchObject({ number: 40, requests: ["number preview 150"] });
        await expect(number).not.toHaveAttribute("aria-invalid");

        // Leaving is the commit, and the application clamps what it was asked.
        await page.keyboard.press("Tab");
        await expect(number).not.toBeFocused();
        await expect(number).toHaveValue("100");
        expect(await snapshot(page)).toMatchObject({
            number: 100,
            requests: ["number preview 150", "number commit 150"],
        });
        await expect(number).toHaveAttribute("aria-valuenow", "100");
    });

    test("text that is not a number yet is invalid, asks for nothing, and Enter puts the value back", async ({
        page,
    }) => {
        await open(page, "number-field");
        const number = field(page);
        for (const text of ["-", "1e"]) {
            await number.fill(text);
            await expect(number).toHaveValue(text);
            await expect(number).toHaveAttribute("aria-invalid", "true");
        }
        await page.keyboard.press("Enter");
        await expect(number).toHaveValue("40");
        await expect(number).not.toHaveAttribute("aria-invalid");
        await expect(number).toBeFocused();
        expect(await snapshot(page)).toMatchObject({ number: 40, requests: [] });
    });

    test("Escape drops the draft and asks for its previews back", async ({ page }) => {
        await open(page, "number-field");
        const number = field(page);
        await number.fill("55");
        expect(await snapshot(page)).toMatchObject({ number: 55 });
        await page.keyboard.press("Escape");
        await expect(number).toHaveValue("40");
        await expect(number).toBeFocused();
        expect(await snapshot(page)).toMatchObject({
            number: 40,
            requests: ["number preview 55", "number cancel"],
        });
        // Nothing was typed since: leaving commits nothing.
        await page.keyboard.press("Tab");
        expect((await snapshot(page)).requests).toEqual(["number preview 55", "number cancel"]);
    });

    test("an untouched field follows a change made elsewhere, and a touched one outlasts it", async ({
        page,
    }) => {
        await open(page, "number-field");
        const number = field(page);
        const outside = page.getByTestId("number-outside");

        await number.focus();
        await outside.click();
        await expect(number).toBeFocused();
        await expect(number).toHaveValue("25");

        await number.fill("7");
        await outside.click();
        expect(await snapshot(page)).toMatchObject({ number: 25 });
        await expect(number).toHaveValue("7");
        await page.keyboard.press("Tab");
        await expect(number).toHaveValue("7");
        expect(await snapshot(page)).toMatchObject({ number: 7 });
    });

    test("the arrows ask for a step from the application's value, and the application clamps", async ({
        page,
    }) => {
        await open(page, "number-field");
        const number = field(page);
        await number.focus();
        await page.keyboard.press("ArrowUp");
        await expect(number).toHaveValue("45");
        await page.keyboard.press("PageUp");
        await expect(number).toHaveValue("95");
        await page.keyboard.press("ArrowUp");
        await page.keyboard.press("ArrowUp");
        await expect(number).toHaveValue("100");
        await page.keyboard.press("ArrowDown");
        await expect(number).toHaveValue("95");
        expect((await snapshot(page)).requests).toEqual([
            "number commit 45",
            "number commit 95",
            "number commit 100",
            "number commit 105",
            "number commit 95",
        ]);

        // A typed edit ends first, as Enter would end it, and the step is from
        // whatever the application made of it.
        await number.fill("12");
        await page.keyboard.press("ArrowUp");
        await expect(number).toHaveValue("17");
        expect((await snapshot(page)).requests.slice(-3)).toEqual([
            "number preview 12",
            "number commit 12",
            "number commit 17",
        ]);
    });

    test("disabled and read-only fields ask for nothing", async ({ page }) => {
        await open(page, "number-field");
        await expect(page.getByTestId("disabled-number")).toBeDisabled();
        const read = page.getByTestId("read-only-number");
        await expect(read).toHaveAttribute("aria-readonly", "true");
        await expect(read).toHaveValue("40");
        const before = await snapshot(page);
        await read.focus();
        await page.keyboard.press("ArrowUp");
        await page.keyboard.type("5");
        await page.keyboard.press("Enter");
        await page.keyboard.press("Tab");
        const after = await snapshot(page);
        expect(after).toMatchObject({ number: 40, requests: [], actions: before.actions });
    });
});

test.describe("colour field", () => {
    const colour = (page: Page) => example(page).getByRole("textbox", { name: "a colour", exact: true });
    const tint = (page: Page) =>
        example(page).getByRole("textbox", { name: "a colour with opacity", exact: true });

    test("#rgb and #rrggbb are colours, and the swatch paints what the application holds", async ({
        page,
    }) => {
        await open(page, "color-field");
        const field = colour(page);
        const swatch = page.getByTestId("default-colour-swatch");
        await expect(field).toHaveValue("#1570ef");
        await expect(swatch).toHaveAttribute("aria-hidden", "true");
        await expect.poll(() => computed(swatch, "background-color")).toBe(rgb("#1570ef"));

        await field.fill("#ABC");
        // A preview: the application shows it at once, the swatch with it.
        expect(await snapshot(page)).toMatchObject({ colour: "#aabbcc" });
        await expect.poll(() => computed(swatch, "background-color")).toBe(rgb("#aabbcc"));
        await expect(field).toHaveValue("#ABC");
        await page.keyboard.press("Enter");
        await expect(field).toHaveValue("#aabbcc");
        expect((await snapshot(page)).requests).toEqual([
            "colour preview #aabbcc",
            "colour commit #aabbcc",
        ]);
    });

    test("an opacity is a colour only where the field allows one", async ({ page }) => {
        await open(page, "color-field");
        const field = colour(page);
        await field.fill("#1f6feb80");
        await expect(field).toHaveAttribute("aria-invalid", "true");
        await page.keyboard.press("Escape");
        await expect(field).toHaveValue("#1570ef");
        await expect(field).not.toHaveAttribute("aria-invalid");

        const withAlpha = tint(page);
        await expect(withAlpha).toHaveValue("#12b76a80");
        await withAlpha.fill("#1f6feb80");
        await expect(withAlpha).not.toHaveAttribute("aria-invalid");
        await page.keyboard.press("Tab");
        expect(await snapshot(page)).toMatchObject({ colour: "#1570ef", tint: "#1f6feb80" });
        await expect
            .poll(() => computed(page.getByTestId("default-tint-swatch"), "background-color"))
            .toMatch(/^rgba\(31, 111, 235, 0\.5\d*\)$/);
        expect((await snapshot(page)).requests).toEqual([
            "tint preview #1f6feb80",
            "tint commit #1f6feb80",
        ]);
    });

    test("a draft on its way to a colour is kept, Escape takes previews back, and an untouched field follows elsewhere", async ({
        page,
    }) => {
        await open(page, "color-field");
        const field = colour(page);
        await field.fill("#1f");
        await expect(field).toHaveValue("#1f");
        await expect(field).toHaveAttribute("aria-invalid", "true");
        await field.fill("#1f6feb");
        expect(await snapshot(page)).toMatchObject({ colour: "#1f6feb" });
        await page.keyboard.press("Escape");
        await expect(field).toHaveValue("#1570ef");
        expect(await snapshot(page)).toMatchObject({
            colour: "#1570ef",
            requests: ["colour preview #1f6feb", "colour cancel"],
        });

        await page.getByTestId("colour-outside").click();
        await expect(field).toBeFocused();
        await expect(field).toHaveValue("#12b76a");
    });

    test("disabled and read-only fields ask for nothing", async ({ page }) => {
        await open(page, "color-field");
        await expect(page.getByTestId("disabled-colour")).toBeDisabled();
        const read = page.getByTestId("read-only-colour");
        await expect(read).toHaveAttribute("aria-readonly", "true");
        await read.focus();
        await page.keyboard.type("#000");
        await page.keyboard.press("Enter");
        await page.keyboard.press("Tab");
        expect(await snapshot(page)).toMatchObject({ colour: "#1570ef", requests: [] });
        await expect(page.getByTestId("invalid-colour")).toHaveAttribute("aria-invalid", "true");
    });
});

test.describe("toggle group", () => {
    test("pressed is the application's: one at a time, and it keeps one pressed", async ({ page }) => {
        await open(page, "toggle-group");
        const group = example(page).getByRole("group", { name: "alignment", exact: true });
        const button = (name: string) => group.getByRole("button", { name, exact: true });
        await expect(button("start")).toHaveAttribute("aria-pressed", "true");
        await expect(button("centre")).toHaveAttribute("aria-pressed", "false");

        await button("centre").click();
        expect(await snapshot(page)).toMatchObject({ align: ["centre"] });
        await expect(button("centre")).toHaveAttribute("aria-pressed", "true");
        await expect(button("start")).toHaveAttribute("aria-pressed", "false");

        // Pressing the pressed one asks for none, and the page refuses.
        const before = (await snapshot(page)).actions;
        await button("centre").click();
        await expect(button("centre")).toHaveAttribute("aria-pressed", "true");
        expect(await snapshot(page)).toMatchObject({ align: ["centre"], actions: before });
    });

    test("a group is one tab stop, and the arrows rove within it over what is disabled", async ({ page }) => {
        await open(page, "toggle-group");
        const align = example(page).getByRole("group", { name: "alignment", exact: true });
        const toolbar = example(page).getByRole("toolbar", { name: "text style", exact: true });
        const tool = (name: string) => toolbar.getByRole("button", { name, exact: true });
        await expect(toolbar).toHaveAttribute("aria-orientation", "horizontal");
        await expect(align.getByRole("button", { name: "start" })).toHaveAttribute("tabindex", "0");
        await expect(align.getByRole("button", { name: "end" })).toHaveAttribute("tabindex", "-1");

        // The stop is the pressed button; Tab goes from one group to the next.
        await align.getByRole("button", { name: "start" }).focus();
        await page.keyboard.press("Tab");
        await expect(tool("bold")).toBeFocused();

        await page.keyboard.press("ArrowRight");
        await expect(tool("italic")).toBeFocused();
        // Moving is not pressing.
        expect(await snapshot(page)).toMatchObject({ styles: ["bold"] });
        await page.keyboard.press("ArrowRight");
        await expect(tool("underline")).toBeFocused();
        await page.keyboard.press("ArrowRight");
        await expect(tool("bold")).toBeFocused();
        await page.keyboard.press("ArrowLeft");
        await expect(tool("underline")).toBeFocused();
        await page.keyboard.press("Home");
        await expect(tool("bold")).toBeFocused();
        await page.keyboard.press("End");
        await expect(tool("underline")).toBeFocused();
        await expect(tool("strike")).toBeDisabled();

        // Several at once: Space presses, Enter presses too.
        await page.keyboard.press("Space");
        expect(await snapshot(page)).toMatchObject({ styles: ["bold", "underline"] });
        await expect(tool("underline")).toHaveAttribute("aria-pressed", "true");
        await page.keyboard.press("Home");
        await page.keyboard.press("Enter");
        expect(await snapshot(page)).toMatchObject({ styles: ["underline"] });

        // Back into the toolbar, the keyboard lands where it left.
        await page.keyboard.press("Shift+Tab");
        await expect(align.getByRole("button", { name: "start" })).toBeFocused();
        await page.keyboard.press("Tab");
        await expect(tool("bold")).toBeFocused();
    });

    test("a disabled group presses nothing", async ({ page }) => {
        await open(page, "toggle-group");
        const disabled = page.getByTestId("disabled-align");
        for (const name of ["start", "centre", "end"]) {
            await expect(disabled.getByRole("button", { name, exact: true })).toBeDisabled();
        }
        await expect(page.getByTestId("disabled-styles-bold")).toBeDisabled();
    });
});

test.describe("toast", () => {
    const region = (page: Page) => page.getByTestId("toaster");
    const card = (page: Page) => page.getByTestId("toaster-toast");

    test("a message is shown in a polite status region, and a newer one replaces it", async ({ page }) => {
        await open(page, "toast");
        await expect(region(page)).toHaveAttribute("role", "status");
        await expect(region(page)).toHaveAttribute("aria-live", "polite");
        await expect(card(page)).toHaveCount(0);

        await page.getByTestId("default-toast-show").click();
        await expect(card(page)).toContainText("message 1");
        await expect(card(page)).toHaveAttribute("data-tone", "neutral");
        const first = (await snapshot(page)).toast!;
        expect(first).toMatchObject({ message: "message 1", tone: "neutral" });

        await example(page)
            .getByRole("group", { name: "the tone of the next message" })
            .getByRole("button", { name: "error", exact: true })
            .click();
        await page.getByTestId("default-toast-show").click();
        await expect(card(page)).toHaveCount(1);
        await expect(card(page)).toContainText("message 2");
        await expect(card(page)).toHaveAttribute("data-tone", "error");
        const second = (await snapshot(page)).toast!;
        expect(second.seq).toBeGreaterThan(first.seq);
        // The tone is how it looks: a coloured edge, nothing more.
        await expect.poll(() => computed(card(page), "border-inline-start-width")).toBe("4px");
    });

    test("a message goes on its own, a replaced one's timer leaves its successor, and dismiss takes it sooner", async ({
        page,
    }) => {
        await open(page, "toast");
        await page.getByTestId("default-toast-quick").click();
        await expect(card(page)).toContainText("message 1");
        await expect(card(page)).toHaveCount(0, { timeout: 10_000 });
        expect((await snapshot(page)).toast).toBeNull();

        await page.getByTestId("default-toast-quick").click();
        await page.getByTestId("default-toast-show").click();
        await expect(card(page)).toContainText("message 3");
        // Past the first one's time, well inside the second's.
        await page.waitForTimeout(1_800);
        await expect(card(page)).toContainText("message 3");

        const dismiss = card(page).getByRole("button", { name: "dismiss this message", exact: true });
        await dismiss.focus();
        await page.keyboard.press("Enter");
        await expect(card(page)).toHaveCount(0);
        // The keyboard stays in the scope rather than falling to the page.
        await expect(page.locator("#catalog")).toBeFocused();

        // The button's name is the SDK's, in the scope's language.
        await page.getByTestId("toggle-locale").click();
        await page.getByTestId("default-toast-show").click();
        await expect(card(page).getByRole("button", { name: "关闭这条消息", exact: true })).toBeVisible();
    });

    test("a message is not a layer: Escape leaves it, and a modal leaves it reachable", async ({ page }) => {
        await open(page, "toast");
        await page.getByTestId("default-toast-show").click();
        await expect(card(page)).toBeVisible();
        await page.getByTestId("default-toast-show").focus();
        await page.keyboard.press("Escape");
        await expect(card(page)).toBeVisible();

        await open(page, "dialog");
        await page.getByTestId("default-dialog-trigger").click();
        await expect(page.getByRole("dialog", { name: "a modal dialog" })).toBeVisible();
        expect(await region(page).evaluate((element) => element.closest("[inert]") === null)).toBe(true);
        await page.keyboard.press("Escape");
        await expect(page.getByRole("dialog")).toBeHidden();
    });
});

test.describe("menu: headings, separators, shortcuts, and staying in view", () => {
    const menu = (page: Page) => page.getByRole("menu", { name: "what can be done", exact: true });

    test("headings name the groups, a separator divides them, and the keyboard passes over both", async ({
        page,
    }) => {
        await open(page, "menu");
        await example(page).getByRole("button", { name: "open the menu" }).click();
        const list = menu(page);
        const group = list.getByRole("group", { name: "on this page", exact: true });
        await expect(group.getByRole("menuitem")).toHaveCount(2);
        await expect(
            list.getByRole("group", { name: "on a selection", exact: true }).getByRole("menuitem")
        ).toHaveCount(1);
        await expect(list.getByRole("separator")).toHaveCount(1);
        await expect(list.locator('[data-name="MenuHeading"]')).toHaveCount(2);
        await expect(list.locator('[data-name="MenuHeading"]').first()).toHaveAttribute("aria-hidden", "true");

        const first = list.getByRole("menuitem", { name: "the first command", exact: true });
        const second = list.getByRole("menuitem", { name: "the second command", exact: true });
        await expect(first).toBeFocused();
        await page.keyboard.press("ArrowDown");
        await expect(second).toBeFocused();
        // Past the separator and the heading is a command that cannot run,
        // so the next stop is the first again.
        await page.keyboard.press("ArrowDown");
        await expect(first).toBeFocused();
        await page.keyboard.press("End");
        await expect(second).toBeFocused();
        await page.keyboard.press("Enter");
        expect(await snapshot(page)).toMatchObject({ command: "second", menu: false });
    });

    test("a shortcut is shown beside its command, announced, and the page answers it", async ({ page }) => {
        await open(page, "menu");
        const trigger = example(page).getByRole("button", { name: "open the menu" });
        await trigger.click();
        const first = menu(page).getByRole("menuitem", { name: "the first command", exact: true });
        await expect(first).toHaveAttribute("aria-keyshortcuts", "Alt+Shift+F");
        const hint = menu(page).getByTestId("menu-shortcut-first");
        await expect(hint).toHaveText("Alt+Shift+F");
        await expect(hint).toHaveAttribute("aria-hidden", "true");
        await expect(menu(page).getByTestId("menu-item-third")).not.toHaveAttribute("aria-keyshortcuts");

        await page.keyboard.press("Escape");
        await expect(trigger).toBeFocused();
        await page.keyboard.press("Alt+Shift+S");
        expect(await snapshot(page)).toMatchObject({ command: "second", menu: false });
    });

    test("a context menu opens at the point, and stays inside the viewport near its corner", async ({ page }) => {
        await page.setViewportSize({ width: 800, height: 720 });
        await open(page, "menu");
        const area = page.getByTestId("default-context-area");
        const context = page.getByRole("menu", { name: "what can be done here", exact: true });

        // With room, its corner is the point.
        await area.evaluate((element) => element.scrollIntoView({ block: "center" }));
        let box = (await area.boundingBox())!;
        await page.mouse.click(box.x + 10, box.y + 10, { button: "right" });
        await expect(context).toBeVisible();
        expect(await snapshot(page)).toMatchObject({ context_menu: true });
        let placed = (await context.boundingBox())!;
        expect(Math.abs(placed.x - (box.x + 10))).toBeLessThanOrEqual(1);
        expect(Math.abs(placed.y - (box.y + 10))).toBeLessThanOrEqual(1);
        await page.keyboard.press("Escape");
        await expect(context).toBeHidden();

        // Near the bottom right corner, it moves left and opens above.
        await page.setViewportSize({ width: 800, height: Math.ceil(box.y + box.height + 8) });
        box = (await area.boundingBox())!;
        const view = await page.evaluate(() => ({
            width: document.documentElement.clientWidth,
            height: document.documentElement.clientHeight,
        }));
        const at = { x: Math.min(box.x + box.width, view.width) - 4, y: box.y + box.height - 4 };
        await page.mouse.click(at.x, at.y, { button: "right" });
        await expect(context).toBeVisible();
        placed = (await context.boundingBox())!;
        expect(placed.x).toBeGreaterThanOrEqual(0);
        expect(placed.y).toBeGreaterThanOrEqual(0);
        expect(placed.x + placed.width).toBeLessThanOrEqual(view.width + 1);
        expect(placed.y + placed.height).toBeLessThanOrEqual(view.height + 1);
        expect(placed.x).toBeLessThan(at.x);
        expect(placed.y + placed.height).toBeLessThanOrEqual(at.y + 1);
        await page.keyboard.press("Escape");
    });

    test("a menu with no room under its trigger opens over it", async ({ page }) => {
        await open(page, "menu");
        const trigger = example(page).getByRole("button", { name: "open the menu" });
        await trigger.click();
        const height = (await menu(page).boundingBox())!.height;
        await page.keyboard.press("Escape");
        const below = (await trigger.boundingBox())!;
        await page.setViewportSize({ width: 1280, height: Math.ceil(below.y + below.height + height / 2) });

        await trigger.click();
        const over = (await menu(page).boundingBox())!;
        const now = (await trigger.boundingBox())!;
        expect(over.y + over.height).toBeLessThanOrEqual(now.y + 1);
        expect(over.y).toBeGreaterThanOrEqual(0);
    });

    test("the keyboard opens the context menu against the box it is in", async ({ page }) => {
        await open(page, "menu");
        const area = page.getByTestId("default-context-area");
        await area.focus();
        await page.keyboard.press("Shift+F10");
        const context = page.getByRole("menu", { name: "what can be done here", exact: true });
        await expect(context).toBeVisible();
        await expect(context.getByRole("menuitem", { name: "a command for this point" })).toBeFocused();
        await page.keyboard.press("ArrowDown");
        await expect(context.getByRole("menuitem", { name: "a command for anywhere" })).toBeFocused();
        await page.keyboard.press("Escape");
        await expect(context).toBeHidden();
        await expect(area).toBeFocused();
    });
});

test.describe("dialog: the title bar", () => {
    test("the close button is named in the scope's language, and it and Escape make one request", async ({
        page,
    }) => {
        await open(page, "dialog");
        const trigger = page.getByTestId("default-dialog-trigger");
        const dialog = page.getByRole("dialog", { name: "a modal dialog" });

        await trigger.click();
        const close = dialog.getByRole("button", { name: "close", exact: true });
        await expect(close).toHaveAttribute("data-testid", "dialog-close");
        await expect(dialog.locator('[data-name="DialogTitleBar"]').getByRole("button")).toHaveCount(1);
        // The first stop in the dialog: always safe to press.
        await expect(close).toBeFocused();
        await page.keyboard.press("Enter");
        await expect(dialog).toBeHidden();
        await expect(trigger).toBeFocused();
        expect(await snapshot(page)).toMatchObject({ dialog: false, dialog_closes: 1 });

        await trigger.click();
        await expect(dialog).toBeVisible();
        await page.keyboard.press("Escape");
        await expect(dialog).toBeHidden();
        expect(await snapshot(page)).toMatchObject({ dialog: false, dialog_closes: 2 });

        // The application's own way out is not a close request.
        await trigger.click();
        await page.getByTestId("dialog-accept").click();
        await expect(dialog).toBeHidden();
        expect(await snapshot(page)).toMatchObject({ dialog_closes: 2 });

        await page.getByTestId("toggle-locale").click();
        await trigger.click();
        await expect(dialog.getByRole("button", { name: "关闭", exact: true })).toBeFocused();
        await page.keyboard.press("Escape");
    });
});

test.describe("light and dark", () => {
    test("the new controls draw from the scope's tokens in both themes", async ({ page }) => {
        const read = async () => {
            await open(page, "number-field");
            const number = await computed(page.getByTestId("default-number"), "background-color");
            const border = await computed(page.getByTestId("default-number"), "border-top-color");
            await open(page, "toggle-group");
            const pressed = await computed(page.getByTestId("default-align-start"), "background-color");
            await open(page, "toast");
            await page.getByTestId("default-toast-show").click();
            const toast = await computed(page.getByTestId("toaster-toast"), "background-color");
            await page.getByTestId("toaster-toast-dismiss").click();
            await open(page, "menu");
            await example(page).getByRole("button", { name: "open the menu" }).click();
            const separator = await computed(
                page.getByRole("menu", { name: "what can be done" }).getByRole("separator"),
                "background-color"
            );
            await page.keyboard.press("Escape");
            await open(page, "dialog");
            await page.getByTestId("default-dialog-trigger").click();
            const dialog = await computed(page.locator('[data-name="Dialog"]'), "background-color");
            await page.keyboard.press("Escape");
            return { number, border, pressed, toast, separator, dialog };
        };

        for (const theme of ["light", "dark"] as const) {
            if (theme === "dark") {
                await toDark(page);
            }
            const tokens = TOKENS[theme];
            expect(await read(), theme).toEqual({
                number: rgb(tokens.input),
                border: rgb(tokens.border),
                pressed: rgb(tokens.primary),
                toast: rgb(tokens.popover),
                separator: rgb(tokens.border),
                dialog: rgb(tokens.popover),
            });
        }
    });
});
