import { expect, Page, test } from "@playwright/test";

import { sharedPage } from "./support";

/// M1's exit conditions, on the third example: it boots under the release
/// policy, its product carries nothing inline, a theme change reaches the DOM
/// and the GPU in the same breath, and a host page's own controls look exactly
/// the same before and after a scope is mounted beside them.

const snapshot = (page: Page) =>
    page.evaluate(() => window.__component_catalog.snapshot());

async function ready(page: Page) {
    const violations: string[] = [];
    page.on("console", (message) => {
        if (/Content Security Policy|Refused to/i.test(message.text())) {
            violations.push(message.text());
        }
    });
    await page.goto("./");
    await expect(page.getByTestId("status")).toHaveAttribute("data-status", "ready", {
        timeout: 60_000,
    });
    return violations;
}

test.describe("M1 V1: the catalogue starts under the policy it will be deployed with", () => {
    test("boots with no policy violation and draws its region", async ({ page }) => {
        const violations = await ready(page);
        expect(await snapshot(page)).toMatchObject({ categories: 18, theme: "light" });
        await expect
            .poll(async () => (await snapshot(page)).region, { timeout: 30_000 })
            .toBe("ready");
        expect(violations).toEqual([]);
        // The one thing a strict policy cannot police for us: a page that is
        // clean only because nothing rendered.
        await expect(page.getByTestId("catalogue")).toBeVisible();
        await expect(page.getByTestId("nav-button")).toBeVisible();
    });

    test("the product contains no inline script, no inline style and no dynamic JS", async ({
        request,
    }) => {
        const index = await (await request.get("./index.html")).text();
        // An inline <script> has a body; a <script src> does not.
        expect(index).not.toMatch(/<script(?![^>]*\bsrc=)[^>]*>[\s\S]*?<\/script>/);
        expect(index).not.toMatch(/<style[\s>]/);
        expect(index).not.toMatch(/\sstyle="/);
        for (const name of ["app.js", "loader.js", "rustify.css"]) {
            const body = await (await request.get(`./${name}`)).text();
            expect(body).not.toContain("new Function");
            expect(body).not.toMatch(/\beval\(/);
        }
    });

    test("the stylesheet the page loads is the one the components were compiled against", async ({
        request,
    }) => {
        const css = await (await request.get("./rustify.css")).text();
        // The classes the catalogue actually uses have rules; a missing one
        // means the product was built from a stale stylesheet.
        for (const cls of [
            "rui\\:bg-card",
            "rui\\:bg-success",
            "rui\\:bg-warning",
            "rui\\:text-muted-foreground",
        ]) {
            expect(css).toContain(`.${cls}`);
        }
        // And nothing that resets or restyles the host page.
        expect(css).not.toContain("@layer base");
        expect(css).not.toMatch(/^\s*(html|body|input|button|a|\*)\s*\{/m);
    });
});

test.describe("M1 V1: one theme, both halves", () => {
    test("a switch moves the panel and the region together, from either side", async ({ page }) => {
        await ready(page);
        await expect.poll(async () => (await snapshot(page)).region).toBe("ready");
        const region = page.getByTestId("catalogue-region");
        const before = await region.screenshot();

        await page.getByTestId("toggle-theme").click();
        expect(await snapshot(page)).toMatchObject({ theme: "dark" });
        // The DOM half moved: the scope root carries the new theme.
        await expect(page.locator("[data-rustify-scope]").first()).toHaveAttribute(
            "data-theme",
            "dark"
        );
        // And the GPU half moved with it, in pixels rather than in a promise.
        await expect
            .poll(async () => (await region.screenshot()).equals(before), { timeout: 15_000 })
            .toBe(false);

        // The region asks for the same switch, and the answer comes back to
        // both halves: a request from the GPU side is still the application's
        // to grant. The pointer goes where the region says it drew the button,
        // not where this test thinks the layout put it.
        const box = (await region.boundingBox())!;
        const button = (await snapshot(page)).button!;
        expect(button).not.toBeNull();
        await page.mouse.click(
            box.x + button.x + button.width / 2,
            box.y + button.y + button.height / 2
        );
        await expect.poll(async () => (await snapshot(page)).theme).toBe("light");
        await expect(page.locator("[data-rustify-scope]").first()).toHaveAttribute(
            "data-theme",
            "light"
        );
    });

    test("the host page keeps its own appearance", async ({ page }) => {
        await ready(page);
        // Nothing the SDK writes lands on the document: the theme goes on the
        // scope root, and the stylesheet has no rule for a bare element.
        const documentTheme = await page.evaluate(() =>
            document.documentElement.getAttribute("data-theme")
        );
        expect(documentTheme).toBeNull();
        const bodyBefore = await page.evaluate(() => getComputedStyle(document.body).background);
        await page.getByTestId("toggle-theme").click();
        await expect.poll(async () => (await snapshot(page)).theme).toBe("dark");
        expect(await page.evaluate(() => getComputedStyle(document.body).background)).toBe(
            bodyBefore
        );
    });
});

test.describe("M1: the catalogue answers for all eighteen categories", () => {
    test("every category has a page, and the status table has a row for each", async ({ page }) => {
        await ready(page);
        await page.getByTestId("nav-status").click();
        const rows = page.locator('[data-testid^="status-row-"]');
        await expect(rows).toHaveCount(18);

        // A category page shows that category's own answers, not the first
        // one's: the nav is what changes the page.
        await page.getByTestId("nav-slider").click();
        await expect(page.getByTestId("category-name")).toHaveText("slider");
        expect(await snapshot(page)).toMatchObject({ path: "/slider" });
        await page.getByTestId("nav-tabs").click();
        await expect(page.getByTestId("category-name")).toHaveText("tabs");
        // Every category has a DOM component now; what differs is what a
        // region draws of one, and the cell says which rather than nothing.
        await expect(page.getByTestId("presentation-dom")).toHaveText("yes");
        await expect(page.getByTestId("presentation-gpu")).toHaveText("partial");
    });

    test("the language switch changes the words and nothing else", async ({ page }) => {
        await ready(page);
        await page.getByTestId("nav-status").click();
        const heading = page.getByTestId("status-page").getByRole("heading");
        const english = await heading.first().textContent();
        const rows = await page.locator('[data-testid^="status-row-"]').count();

        await page.getByTestId("toggle-locale").click();
        expect(await snapshot(page)).toMatchObject({ locale: "zh-CN" });
        expect(await heading.first().textContent()).not.toBe(english);
        // The table is the same table: a language is words, not a different
        // set of answers.
        expect(await page.locator('[data-testid^="status-row-"]').count()).toBe(rows);
    });
});

test.describe("M1 V1: the host page's own controls are not ours", () => {
    /// The decisive comparison: the same markup, on a page that loads our
    /// stylesheets and on one that loads nothing. Rust/UI's own CSS restyles
    /// `input[type="range"]` through a bare element selector, which is exactly
    /// what this catches - and what the scoping rule exists to prevent.
    const MARKUP = `
        <section class="dark">
            <input type="range" data-testid="host-range" min="0" max="10" value="5">
            <input type="text" data-testid="host-text" value="host">
            <button type="button" data-testid="host-button">host button</button>
            <a href="#host" data-testid="host-link">host link</a>
        </section>`;

    const PROPERTIES = [
        "appearance",
        "height",
        "width",
        "font",
        "color",
        "background-color",
        "border",
        "padding",
        "margin",
        "cursor",
        "text-decoration",
    ];

    const styles = (page: Page) =>
        page.evaluate((properties) => {
            const out: Record<string, Record<string, string>> = {};
            for (const id of ["host-range", "host-text", "host-button", "host-link"]) {
                const element = document.querySelector(`[data-testid="${id}"]`)!;
                const computed = getComputedStyle(element);
                out[id] = Object.fromEntries(
                    properties.map((property) => [property, computed.getPropertyValue(property)])
                );
            }
            return out;
        }, PROPERTIES);

    test("they compute the same with our stylesheets as without them", async ({ page, browser }) => {
        await ready(page);
        // The same allowance the other region checks give it. Waiting for the
        // region is a precondition here, not the thing under test, and a cold
        // boot behind thirty other tests is slower than one on its own.
        await expect
            .poll(async () => (await snapshot(page)).region, { timeout: 30_000 })
            .toBe("ready");
        const withUs = await styles(page);

        // The bare page carries the example's own stylesheet and not ours, so
        // a difference is the SDK's doing rather than the application's: what
        // is under test is `runtime.css` and `rustify.css`, not `app.css`.
        const bare = await browser.newPage();
        await bare.goto(new URL("./app.css", page.url()).href);
        const appCss = await bare.evaluate(() => document.body.textContent ?? "");
        await bare.setContent(
            `<!DOCTYPE html><html><head><style>${appCss}</style></head><body>${MARKUP}</body></html>`
        );
        const without = await styles(bare);
        await bare.close();

        expect(withUs).toEqual(without);
    });

    test("a host `dark` class does not darken a scope, and two scopes keep their own themes", async ({
        page,
    }) => {
        await ready(page);
        // The host section carries `class="dark"`, the convention a Tailwind
        // page uses. The scope inside the page is light because its own root
        // says so, and for no other reason.
        expect(await snapshot(page)).toMatchObject({ theme: "light" });
        await expect(page.locator("[data-rustify-scope]").first()).toHaveAttribute(
            "data-theme",
            "light"
        );

        await page.evaluate(() => window.__component_catalog.mount_second());
        await expect(page.locator("[data-rustify-scope]")).toHaveCount(2);
        // The first scope switches; the second keeps what it had.
        await page.getByTestId("toggle-theme").first().click();
        await expect(page.locator("[data-rustify-scope]").first()).toHaveAttribute(
            "data-theme",
            "dark"
        );
        await expect(page.locator("[data-rustify-scope]").nth(1)).toHaveAttribute(
            "data-theme",
            "light"
        );
        expect(await page.evaluate(() => window.__component_catalog.dispose_second())).toBe(true);
    });
});

/// M2 V2: the eighteen categories are components now, not descriptions of
/// them. Every page carries the component itself, bound to a value the region
/// above it draws with the GPU half - and, for the nine where the states mean
/// something, the same component again as disabled, read-only and invalid.
///
/// One page for the whole block. A cold catalogue costs about seven seconds to
/// download, compile and boot, and these are short checks; the price is that
/// each test has to leave the page as it found it, or read a delta.
test.describe("M2 V2: a component per category", () => {
    test.describe.configure({ mode: "serial" });
    const shared = sharedPage();

    const CATEGORIES = [
        "button",
        "label",
        "link",
        "icon",
        "text-field",
        "text-area",
        "checkbox",
        "radio",
        "switch",
        "select",
        "slider",
        "progress",
        "loading",
        "tooltip",
        "menu",
        "dialog",
        "tabs",
        "scroll-area",
    ];

    /// The eight with a state beyond the ordinary one. A matrix is only worth
    /// drawing where the states differ: a tooltip has one, and a component
    /// rendered as "invalid" that has no invalid state shows the same picture
    /// twice under two names.
    const WITH_STATES = [
        "button",
        "link",
        "text-field",
        "text-area",
        "checkbox",
        "radio",
        "switch",
        "slider",
    ];

    async function open(page: Page, category: string) {
        await page.getByTestId(`nav-${category}`).click();
        await expect(page.getByTestId("category-name")).toHaveText(category.replace(/-/g, " "));
    }

    test("every category shows a working example of itself", async () => {
        const page = shared.page;
        for (const category of CATEGORIES) {
            await open(page, category);
            // A component of this crate, not a paragraph about one: every one
            // of them carries `data-name`.
            const named = page.locator('[data-testid="example"] [data-name]');
            expect(await named.count(), category).toBeGreaterThan(0);
            const matrix = page.getByTestId("state-matrix");
            await expect(matrix, category).toBeAttached({
                attached: WITH_STATES.includes(category),
            });
        }
    });

    test("a disabled control asks for nothing and a read-only one puts itself back", async () => {
        const page = shared.page;
        await open(page, "checkbox");
        const before = await snapshot(page);

        // The live one is an ordinary control: it asks, and the application
        // grants.
        await page.getByTestId("default-checkbox").click();
        const after = await snapshot(page);
        expect(after.actions).toBe(before.actions + 1);
        expect(after.checked).toBe(!before.checked);

        // The other two share that value and that counter, which is what
        // makes this a measurement rather than a guess.
        await page.getByTestId("disabled-checkbox").click({ force: true });
        expect(await snapshot(page)).toMatchObject({
            actions: after.actions,
            checked: after.checked,
        });

        // A read-only checkbox has no HTML state to be in, so it takes the
        // click and puts the box back where the application has it.
        await page.getByTestId("read-only-checkbox").click();
        expect(await snapshot(page)).toMatchObject({
            actions: after.actions,
            checked: after.checked,
        });
        expect(await page.getByTestId("read-only-checkbox").isChecked()).toBe(after.checked);

        // Put the page back for whatever runs next.
        await page.getByTestId("default-checkbox").click();
        expect(await snapshot(page)).toMatchObject({ checked: before.checked });
    });

    test("a read-only field keeps the application's value, not the keystrokes", async () => {
        const page = shared.page;
        await open(page, "text-field");
        const before = await snapshot(page);

        // A text field has a read-only state in HTML, so the browser refuses
        // the keystrokes itself.
        await expect(page.getByTestId("read-only-text-field")).toHaveJSProperty(
            "readOnly",
            true
        );
        // And an input event that arrives anyway - a script, an extension, a
        // test - is still not a change: the control is put back in step with
        // what the application holds.
        await page.evaluate(() => {
            const field = document.querySelector(
                '[data-testid="read-only-text-field"]'
            ) as HTMLInputElement;
            field.value = "typed into a read-only field";
            field.dispatchEvent(new Event("input", { bubbles: true }));
        });
        expect(await snapshot(page)).toMatchObject({
            actions: before.actions,
            text: before.text,
        });
        await expect(page.getByTestId("read-only-text-field")).toHaveValue(before.text);

        await expect(page.getByTestId("disabled-text-field")).toBeDisabled();

        // The live one does take them, which is what makes the two different.
        await page.getByTestId("default-text-field").fill("a new value");
        expect(await snapshot(page)).toMatchObject({ text: "a new value" });
        await page.getByTestId("default-text-field").fill(before.text);
    });

    test("one value, two halves: the region's control moves the DOM's", async () => {
        const page = shared.page;
        await open(page, "checkbox");
        await expect.poll(async () => (await snapshot(page)).region).toBe("ready");
        // The rectangle is cleared on a page change and reported again once
        // the region has drawn this page's control, so waiting for it is
        // waiting for the right control rather than the last one.
        await expect
            .poll(async () => (await snapshot(page)).control, { timeout: 15_000 })
            .not.toBeNull();
        const before = await snapshot(page);

        // A bounding box is in the viewport, and so is a mouse click. The nav
        // is long enough to have scrolled the region off the top, and a click
        // at a negative y lands nowhere.
        const region = page.getByTestId("catalogue-region");
        await region.scrollIntoViewIfNeeded();
        const box = (await region.boundingBox())!;
        const control = before.control!;
        await page.mouse.click(
            box.x + control.x + control.width / 2,
            box.y + control.y + control.height / 2
        );

        await expect.poll(async () => (await snapshot(page)).checked).toBe(!before.checked);
        expect((await snapshot(page)).actions).toBe(before.actions + 1);
        // And the DOM half is showing the same value, because there is one.
        await expect(page.getByTestId("default-checkbox")).toBeChecked({
            checked: !before.checked,
        });

        await page.getByTestId("default-checkbox").click();
        expect(await snapshot(page)).toMatchObject({ checked: before.checked });
    });

    test("the state matrix shows one value in each of its states", async () => {
        const page = shared.page;
        await open(page, "switch");
        const held = (await snapshot(page)).on;
        for (const state of ["default", "disabled", "read-only"]) {
            await expect(page.getByTestId(`${state}-switch`)).toHaveAttribute(
                "aria-checked",
                String(held)
            );
        }
        await expect(page.getByTestId("disabled-switch")).toBeDisabled();
        await expect(page.getByTestId("read-only-switch")).toHaveAttribute(
            "aria-readonly",
            "true"
        );
    });

    test("five kinds of component make one interface", async () => {
        const page = shared.page;
        await open(page, "radio");
        const names = await page.evaluate(() =>
            Array.from(
                new Set(
                    Array.from(document.querySelectorAll("[data-rustify-scope] [data-name]")).map(
                        (element) => (element as HTMLElement).dataset.name
                    )
                )
            )
        );
        // Button, Panel, Row, Chip, RadioGroup, RadioItem, Label, Switch: the
        // page is composed of the crate's own components rather than of markup
        // that happens to look like them.
        expect(names.length).toBeGreaterThanOrEqual(5);
        expect(names).toContain("Button");
        expect(names).toContain("RadioGroup");

        // And the mixed interface works: a choice in the group is the
        // application's, and the counter moves once.
        const before = await snapshot(page);
        await page.getByTestId("default-radio-third").click();
        expect(await snapshot(page)).toMatchObject({
            chosen: 2,
            actions: before.actions + 1,
        });
        await page.getByTestId(`default-radio-first`).click();
        expect(await snapshot(page)).toMatchObject({ chosen: 0 });
    });

    test("the region draws a control for every category it claims one for", async () => {
        const page = shared.page;
        await expect.poll(async () => (await snapshot(page)).region, { timeout: 30_000 }).toBe(
            "ready"
        );
        // The fourteen the catalogue says a region draws. A shader that does
        // not compile, a widget type that was never registered and a slot that
        // never becomes visible all look the same from here - an empty
        // rectangle - and all three fail this.
        const DRAWN = [
            "button",
            "icon",
            "checkbox",
            "radio",
            "switch",
            "select",
            "slider",
            "progress",
            "loading",
            "tabs",
            "scroll-area",
        ];
        for (const category of DRAWN) {
            await open(page, category);
            await expect
                .poll(async () => (await snapshot(page)).control, {
                    message: category,
                    timeout: 15_000,
                })
                .not.toBeNull();
        }
    });
});


