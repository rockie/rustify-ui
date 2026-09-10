import { expect, Page, test } from "@playwright/test";

import { sharedPage } from "./support";

/// M2 V4: every control has a name, a role, a value and a state, and a
/// keyboard can reach all of them.
///
/// The queries are scoped to the live example rather than the page: the state
/// matrix renders the same component three more times, and a name that matches
/// four elements is a name a test cannot rely on - which is also true of a
/// screen reader user's experience of it.

const snapshot = (page: Page) => page.evaluate(() => window.__component_catalog.snapshot());

/// Category, and what a reader finds on its page.
const NAMED: [string, string, string][] = [
    ["button", "button", "press me"],
    ["label", "textbox", "a named field"],
    ["link", "link", "a link to somewhere"],
    ["icon", "img", "done"],
    ["icon", "img", "close"],
    ["text-field", "textbox", "a text field"],
    ["text-area", "textbox", "notes"],
    ["checkbox", "checkbox", "a checkbox"],
    ["radio", "radiogroup", "a choice of three"],
    ["radio", "radio", "first"],
    ["radio", "radio", "third"],
    ["switch", "switch", "a switch"],
    ["select", "combobox", "a size"],
    ["slider", "slider", "a size"],
    ["progress", "progressbar", "how far along"],
    ["progress", "progressbar", "working"],
    ["loading", "status", "loading"],
    ["loading", "button", "start or stop"],
    ["tooltip", "button", "hover or focus me"],
    ["menu", "button", "open the menu"],
    ["dialog", "button", "open the dialog"],
    ["tabs", "tablist", "three tabs"],
    ["tabs", "tab", "second"],
    ["scroll-area", "group", "a list that does not fit"],
];

async function open(page: Page, category: string) {
    await page.getByTestId(`nav-${category}`).click();
    await expect(page.getByTestId("category-name")).toHaveText(category.replace(/-/g, " "));
}

const example = (page: Page) => page.getByTestId("example");

test.describe("M2 V4: name, role, value, state", () => {
    test.describe.configure({ mode: "serial" });
    const shared = sharedPage();

    test("twenty-four things a reader can find by what they are and what they are called", async () => {
        const page = shared.page;
        for (const [category, role, name] of NAMED) {
            await open(page, category);
            await expect(
                example(page).getByRole(role as never, { name, exact: true }),
                `${category}: ${role} "${name}"`
            ).toBeVisible();
        }
    });

    test("a control's value and state are in the tree, not only in the pixels", async () => {
        const page = shared.page;
        await open(page, "switch");
        const control = example(page).getByRole("switch", { name: "a switch" });
        const before = (await snapshot(page)).on;
        await expect(control).toHaveAttribute("aria-checked", String(before));
        await control.click();
        await expect(control).toHaveAttribute("aria-checked", String(!before));
        await control.click();

        await open(page, "slider");
        const slider = example(page).getByRole("slider", { name: "a size" });
        await expect(slider).toHaveValue(String((await snapshot(page)).size));

        await open(page, "progress");
        // A quantity nobody knows carries no value: "in progress" and "nothing
        // done" are different things to hear.
        await expect(
            example(page).getByRole("progressbar", { name: "how far along" })
        ).toHaveAttribute("aria-valuenow", "35");
        await expect(
            example(page).getByRole("progressbar", { name: "working" })
        ).not.toHaveAttribute("aria-valuenow", /.*/);
    });

    test("the arrows move within a tab strip and the strip is one tab stop", async () => {
        const page = shared.page;
        await open(page, "tabs");
        const strip = example(page).getByRole("tablist", { name: "three tabs" });
        const first = strip.getByRole("tab", { name: "first" });
        await expect(first).toHaveAttribute("tabindex", "0");
        await expect(strip.getByRole("tab", { name: "second" })).toHaveAttribute(
            "tabindex",
            "-1"
        );

        await first.focus();
        await page.keyboard.press("ArrowRight");
        expect(await snapshot(page)).toMatchObject({ tab: 1 });
        await expect(strip.getByRole("tab", { name: "second" })).toBeFocused();
        await page.keyboard.press("End");
        expect(await snapshot(page)).toMatchObject({ tab: 2 });
        // The ends are joined: a strip is a loop to anyone driving it.
        await page.keyboard.press("ArrowRight");
        expect(await snapshot(page)).toMatchObject({ tab: 0 });
        await expect(
            example(page).getByRole("tabpanel", { name: "first" })
        ).toBeVisible();
    });

    test("a menu takes the keyboard, moves within itself, and gives it back", async () => {
        const page = shared.page;
        await open(page, "menu");
        const trigger = example(page).getByRole("button", { name: "open the menu" });
        await expect(trigger).toHaveAttribute("aria-expanded", "false");
        await trigger.click();
        await expect(trigger).toHaveAttribute("aria-expanded", "true");

        const menu = page.getByRole("menu", { name: "what can be done" });
        await expect(menu).toBeVisible();
        // The keyboard lands on the first item a person can actually run.
        await expect(menu.getByRole("menuitem", { name: "the first command" })).toBeFocused();
        await page.keyboard.press("ArrowDown");
        await expect(menu.getByRole("menuitem", { name: /the second command/ })).toBeFocused();
        // A command that cannot run stays in the list and says why.
        const blocked = menu.getByRole("menuitem", { name: /the third command/ });
        await expect(blocked).toHaveAttribute("aria-disabled", "true");
        await expect(blocked).toContainText("nothing is selected");

        const before = (await snapshot(page)).actions;
        // Forced, because `aria-disabled` is what a menu item wears - it stays
        // reachable so it can say why it cannot run - and the click has to be
        // refused by the component rather than by the test harness.
        await blocked.click({ force: true });
        expect(await snapshot(page)).toMatchObject({ actions: before });

        await page.keyboard.press("Escape");
        await expect(menu).toBeHidden();
        // And focus is back where it was opened from.
        await expect(trigger).toBeFocused();
    });

    test("a modal dialog is named by its own title and hands focus back", async () => {
        const page = shared.page;
        await open(page, "dialog");
        const trigger = example(page).getByRole("button", { name: "open the dialog" });
        await trigger.click();

        const dialog = page.getByRole("dialog", { name: "a modal dialog" });
        await expect(dialog).toBeVisible();
        await expect(dialog).toHaveAttribute("aria-modal", "true");
        await expect(dialog).toContainText("the rest of the scope is inert");

        await page.keyboard.press("Escape");
        await expect(dialog).toBeHidden();
        await expect(trigger).toBeFocused();
    });

    test("a listbox says which option is current and Enter chooses it", async () => {
        const page = shared.page;
        await open(page, "select");
        const trigger = example(page).getByRole("combobox", { name: "a size" });
        await expect(trigger).toHaveAttribute("aria-expanded", "false");

        await trigger.focus();
        await page.keyboard.press("ArrowDown");
        const list = page.getByRole("listbox");
        await expect(list).toBeVisible();
        // It opens on what is chosen, so the arrows continue from there.
        await expect(list).toHaveAttribute("aria-activedescendant", /medium$/);
        await page.keyboard.press("ArrowDown");
        await expect(list).toHaveAttribute("aria-activedescendant", /large$/);
        await page.keyboard.press("Enter");
        await expect(list).toBeHidden();
        expect(await snapshot(page)).toMatchObject({ chooser: "large" });
        await expect(trigger).toBeFocused();

        await trigger.click();
        await page.getByTestId("option-medium").click();
        expect(await snapshot(page)).toMatchObject({ chooser: "medium" });
    });

    test("a tooltip describes the control it belongs to, on focus as well as hover", async () => {
        const page = shared.page;
        await open(page, "tooltip");
        const trigger = example(page).getByRole("button", { name: "hover or focus me" });
        await expect(trigger).not.toHaveAttribute("aria-describedby", /.*/);

        await trigger.hover();
        const tip = page.getByRole("tooltip");
        await expect(tip).toBeVisible();
        // The description is attached to the control, not to a wrapper no
        // reader announces.
        const describedBy = await trigger.getAttribute("aria-describedby");
        expect(describedBy).not.toBeNull();
        await expect(tip).toHaveAttribute("id", describedBy!);

        await page.getByTestId("nav-tooltip").hover();
        await expect(tip).toBeHidden();
        await expect(trigger).not.toHaveAttribute("aria-describedby", /.*/);

        // And from the keyboard, which is what the imported version had no
        // path for at all.
        await trigger.focus();
        await expect(page.getByRole("tooltip")).toBeVisible();
        await page.getByTestId("nav-tooltip").focus();
        await expect(page.getByRole("tooltip")).toBeHidden();
    });

    test("thirty rounds of theme and size later, everything is still where it says", async () => {
        const page = shared.page;
        const sizes = [
            { width: 1280, height: 720 },
            { width: 1024, height: 800 },
            { width: 900, height: 640 },
        ];
        for (let round = 0; round < 30; round += 1) {
            await page.setViewportSize(sizes[round % sizes.length]);
            await page.getByTestId("toggle-theme").click();
        }
        expect(await snapshot(page)).toMatchObject({ theme: "light" });
        await page.setViewportSize(sizes[0]);

        for (const [category, role, name] of NAMED) {
            await open(page, category);
            await expect(
                example(page).getByRole(role as never, { name, exact: true }),
                `${category}: ${role} "${name}"`
            ).toBeVisible();
        }
    });

    test("something that is not there fails quickly rather than hanging", async () => {
        const page = shared.page;
        const started = Date.now();
        await expect(
            expect(page.getByTestId("no-such-control")).toBeVisible({ timeout: 5_000 })
        ).rejects.toThrow();
        expect(Date.now() - started).toBeLessThan(8_000);
    });
});
