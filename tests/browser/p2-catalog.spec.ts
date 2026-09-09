import { expect, Page, test } from "@playwright/test";

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
        // Nine categories are not in P1 and the catalogue says so rather than
        // leaving the cell empty.
        await expect(page.getByTestId("presentation-dom")).toHaveText("no");
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
        await expect.poll(async () => (await snapshot(page)).region).toBe("ready");
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
