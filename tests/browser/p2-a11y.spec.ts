import { expect, Page, test } from "@playwright/test";

import { CATALOG_SIZE, sharedPage } from "./support";

/// M8: what the accessibility tree says, and that the keyboard can leave.
///
/// The VoiceOver record covers three things: that every control is announced
/// with its name, role, value and state; that nothing is announced twice; and
/// that the keyboard is never trapped. The first is `p2-semantics`. The other
/// two are checked here, because a browser can read its own accessibility tree
/// and can press Tab - what it cannot do is hear the result, which is what
/// keeps the record open.

/// The accessibility tree as Playwright renders it: one line per node, the
/// role first and the accessible name in quotes after it when there is one.
///
/// `page.accessibility` was removed from Playwright; this is what replaced it,
/// and it says the thing that matters here - a line with a role and no name is
/// a control with nothing to call it.
async function ariaLines(page: Page): Promise<string[]> {
    // The scope, not the whole document: what the host page puts around a
    // scope is the host's business, and V1's fixture deliberately has some.
    const snapshot = await page.locator("[data-rustify-scope]").first().ariaSnapshot();
    return snapshot.split("\n").filter((line) => line.trim().startsWith("- "));
}

/// The roles a person operates. A `generic` or a `paragraph` with no name is
/// scenery; a button with no name is a control nobody can ask for.
const OPERABLE = [
    "button",
    "checkbox",
    "combobox",
    "link",
    "menuitem",
    "radio",
    "slider",
    "switch",
    "tab",
    "textbox",
];

test.describe("M8: the accessibility tree, and the way out of it", () => {
    test.describe.configure({ mode: "serial" });
    const shared = sharedPage();

    test("every control a person can operate has something to call it", async () => {
        const page = shared.page;
        const lines = await ariaLines(page);
        // `- button "press me"` has a name; `- button:` or `- button` has not.
        const nameless = lines.filter((line) => {
            const body = line.trim().slice(2).trim();
            const role = body.split(/[\s:]/)[0];
            return OPERABLE.includes(role) && !body.includes('"');
        });
        expect(
            nameless,
            "a control with no accessible name is one nobody can ask for"
        ).toEqual([]);

        // And the check is worth something only if it found the controls at
        // all: an empty tree would pass it silently.
        const named = lines.filter((line) =>
            OPERABLE.some((role) => line.trim().startsWith(`- ${role} "`))
        );
        expect(named.length, "the tree has controls in it").toBeGreaterThan(10);
    });

    test("the eighteen categories are eighteen different things to say", async () => {
        const page = shared.page;
        const names = await page
            .locator("nav.catalogue-nav li button")
            .evaluateAll((buttons) => buttons.map((button) => button.textContent?.trim() ?? ""));
        // Eighteen categories plus the status and samples pages.
        expect(names).toHaveLength(CATALOG_SIZE + 2);
        expect(new Set(names).size, `duplicates in ${JSON.stringify(names)}`).toBe(names.length);
    });

    test("tabbing forward from the top comes back round rather than sticking", async () => {
        const page = shared.page;
        await page.evaluate(() => (document.activeElement as HTMLElement | null)?.blur());

        // A trap shows up two ways: focus stops moving, or it cycles among a
        // few elements without ever returning to where it started. Pressing
        // Tab far more times than the page has controls settles both.
        const visited: string[] = [];
        let returned = false;
        for (let n = 0; n < 300; n += 1) {
            await page.keyboard.press("Tab");
            const here = await page.evaluate(() => {
                const element = document.activeElement as HTMLElement | null;
                if (!element || element === document.body) {
                    return "";
                }
                return (
                    element.getAttribute("data-testid") ??
                    `${element.tagName.toLowerCase()}:${element.textContent?.trim().slice(0, 24)}`
                );
            });
            if (visited.length > 0 && here === visited[0]) {
                returned = true;
                break;
            }
            visited.push(here);
        }

        expect(visited.length, "the keyboard moved at all").toBeGreaterThan(5);
        expect(
            returned,
            `focus never came back round; it visited ${visited.length} places, last ${JSON.stringify(
                visited.slice(-3)
            )}`
        ).toBe(true);
    });

    test("an open layer keeps the keyboard, and Escape gives it back", async () => {
        const page = shared.page;
        await page.getByTestId("nav-dialog").click();
        await expect(page.getByTestId("category-name")).toHaveText("dialog");

        const opener = page
            .getByTestId("example")
            .getByRole("button", { name: "open the dialog" });
        await opener.focus();
        await page.keyboard.press("Enter");
        const dialog = page.getByRole("dialog", { name: "a modal dialog" });
        await expect(dialog).toBeVisible();

        // Twenty tab stops. Each one is either inside the layer or outside the
        // scope altogether: a modal makes the rest of **its own scope** inert,
        // and the host page around it is not the SDK's to make inert. What may
        // never happen is focus landing on the scope's own content behind the
        // modal, which is what `aria-modal` promises and what `p2-semantics`
        // cannot catch - a broken trap passes an Escape test too.
        const escaped: string[] = [];
        for (let n = 0; n < 20; n += 1) {
            await page.keyboard.press("Tab");
            const where = await page.evaluate(() => {
                const active = document.activeElement as HTMLElement | null;
                if (!active || active === document.body) {
                    return "outside";
                }
                const layer = document.querySelector('[role="dialog"]');
                if (layer?.contains(active)) {
                    return "layer";
                }
                const scope = document.querySelector("[data-rustify-scope]");
                if (scope?.contains(active)) {
                    return `BEHIND: ${active.getAttribute("data-testid") ?? active.tagName}`;
                }
                return "outside";
            });
            if (where.startsWith("BEHIND")) {
                escaped.push(`stop ${n}: ${where}`);
            }
        }
        expect(escaped, "the keyboard reached the scope behind the modal").toEqual([]);

        // Focus has wandered onto the host page by now, so it is put back in
        // the layer before Escape: what is under test is where the *layer*
        // sends focus when it closes, not what the host page was doing.
        await dialog.getByRole("button").first().focus();
        await page.keyboard.press("Escape");
        await expect(dialog).toBeHidden();
        await expect(opener).toBeFocused();
    });
});
