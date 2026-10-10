import AxeBuilder from "@axe-core/playwright";
import { BrowserContext, expect, Page, test } from "@playwright/test";

/// The default reading theme in a browser: the colour mode from the first
/// frame, the mode switch, copying, code groups, the table of contents,
/// the narrow-screen menus, a page with script disabled, contrast and
/// landmarks. The site is the content fixture (brand primary #175cd3 /
/// #53b1fd, logo static/logo.svg), built and served by
/// tests/docs-ssg/serve.sh.

function base(): string {
    return test.info().project.name === "docs-root" ? "/" : "/rustify-ui/";
}

const modeKey = () => `rustify-docs:mode:${base()}`;

/// WCAG relative-luminance contrast between two computed CSS colours.
async function contrast(page: Page, selector: string, against?: string): Promise<number> {
    return page.evaluate(
        ([selector, against]) => {
            const parse = (value: string) => {
                const canvas = document.createElement("canvas");
                canvas.width = canvas.height = 1;
                const context = canvas.getContext("2d")!;
                context.fillStyle = value;
                context.fillRect(0, 0, 1, 1);
                const [r, g, b, a] = context.getImageData(0, 0, 1, 1).data;
                return { r, g, b, a: a / 255 };
            };
            const luminance = ({ r, g, b }: { r: number; g: number; b: number }) => {
                const channel = (c: number) => {
                    const v = c / 255;
                    return v <= 0.03928 ? v / 12.92 : ((v + 0.055) / 1.055) ** 2.4;
                };
                return 0.2126 * channel(r) + 0.7152 * channel(g) + 0.0722 * channel(b);
            };
            const element = document.querySelector(selector) as HTMLElement;
            const background = (start: HTMLElement | null) => {
                for (let node = start; node; node = node.parentElement) {
                    const color = parse(getComputedStyle(node).backgroundColor);
                    if (color.a > 0.99) return color;
                }
                return parse(getComputedStyle(document.body).backgroundColor);
            };
            const fg = parse(getComputedStyle(element).color);
            const bg = against ? background(document.querySelector(against) as HTMLElement) : background(element);
            const [l1, l2] = [luminance(fg), luminance(bg)].sort((a, b) => b - a);
            return (l1 + 0.05) / (l2 + 0.05);
        },
        [selector, against ?? null] as const,
    );
}

async function noHorizontalScroll(page: Page): Promise<void> {
    const overflow = await page.evaluate(() => document.documentElement.scrollWidth - window.innerWidth);
    expect(overflow, "the page scrolls sideways").toBeLessThanOrEqual(0);
}

test.describe("colour mode", () => {
    test("the first frame follows the system, and a stored choice before any paint", async ({ browser }) => {
        const dark = await browser.newContext({ colorScheme: "dark" });
        const page = await dark.newPage();
        await page.goto(base());
        expect(await page.evaluate(() => document.documentElement.dataset.theme)).toBe("system");
        expect(await page.evaluate(() => getComputedStyle(document.body).backgroundColor)).toBe("rgb(12, 17, 29)");
        await dark.close();

        const stored = await browser.newContext({ colorScheme: "light" });
        await stored.addInitScript(
            ([key]) => {
                localStorage.setItem(key, "dark");
                // What the page looked like when it first had a body.
                document.addEventListener("DOMContentLoaded", () => {
                    (window as unknown as { __first: string }).__first = getComputedStyle(document.body).backgroundColor;
                });
            },
            [modeKey()],
        );
        const second = await stored.newPage();
        await second.goto(`${base()}guide/`);
        expect(await second.evaluate(() => (window as unknown as { __first: string }).__first)).toBe("rgb(12, 17, 29)");
        await stored.close();
    });

    test("the switch changes, marks and keeps the mode across pages", async ({ page }) => {
        await page.goto(`${base()}guide/`);
        const dark = page.locator('[data-rd-mode="dark"]');
        await expect(page.locator("[data-rd-mode-switch]")).toBeVisible();
        await dark.focus();
        await page.keyboard.press("Enter");
        await expect(dark).toHaveAttribute("aria-pressed", "true");
        await expect(page.locator("html")).toHaveAttribute("data-theme", "dark");
        await page.locator(".rd-pager a").first().click();
        await expect(page.locator("html")).toHaveAttribute("data-theme", "dark");
        await expect(page.locator('[data-rd-mode="dark"]')).toHaveAttribute("aria-pressed", "true");
        // The brand's dark primary colours links in dark mode.
        expect(await page.locator(".rd-prose a").first().evaluate((a) => getComputedStyle(a).color)).toBe("rgb(83, 177, 253)");
    });

    test("refused storage leaves the system mode and a switch that still works", async ({ browser }) => {
        const context = await browser.newContext({ colorScheme: "light" });
        await context.addInitScript(() => {
            Object.defineProperty(window, "localStorage", {
                get() {
                    throw new DOMException("denied", "SecurityError");
                },
            });
        });
        const page = await context.newPage();
        const errors: string[] = [];
        page.on("pageerror", (error) => errors.push(error.message));
        await page.goto(base());
        await page.locator('[data-rd-mode="dark"]').click();
        await expect(page.locator("html")).toHaveAttribute("data-theme", "dark");
        expect(errors).toEqual([]);
        await context.close();
    });
});

test.describe("code", () => {
    async function clipboard(context: BrowserContext) {
        await context.grantPermissions(["clipboard-read", "clipboard-write"]);
    }

    test("copy hands over the block's text exactly and says so", async ({ page, context, browserName }) => {
        test.skip(browserName !== "chromium", "clipboard permissions are a Chromium automation feature");
        await clipboard(context);
        await page.goto(`${base()}guide/setup/`);
        const figure = page.locator("figure.rd-code", { hasText: "src/example.rs" });
        const button = figure.locator("[data-rd-copy]");
        await button.click();
        expect(await page.evaluate(() => navigator.clipboard.readText())).toBe('fn main() {\n    println!("hello");\n}\n');
        await expect(button).toHaveAttribute("data-state", "copied");
        await expect(button).toContainText("Copied");
        await expect(page.locator("#rd-status")).toHaveText("Copied");
    });

    test("a refused clipboard says so and selects the code for copying by hand", async ({ page }) => {
        await page.addInitScript(() => {
            Object.defineProperty(navigator, "clipboard", {
                value: { writeText: () => Promise.reject(new DOMException("denied", "NotAllowedError")) },
            });
        });
        await page.goto(`${base()}guide/setup/`);
        const figure = page.locator("figure.rd-code", { hasText: "src/example.rs" });
        const button = figure.locator("[data-rd-copy]");
        await button.click();
        await expect(button).toHaveAttribute("data-state", "failed");
        await expect(page.locator("#rd-status")).toContainText("Copy failed");
        expect(await page.evaluate(() => String(window.getSelection()))).toContain('println!("hello");');
    });

    test("a code group shows one tab, switches its namesakes and remembers across pages", async ({ page }) => {
        await page.goto(`${base()}guide/setup/`);
        const groups = page.locator('.rd-code-group[data-rd-group="install"]');
        await expect(groups).toHaveCount(2);
        const first = groups.nth(0);
        await expect(first.locator('[role="tablist"]')).toBeVisible();
        await expect(first.locator("figure.rd-code").nth(1)).toBeHidden();
        await first.getByRole("tab", { name: "Config" }).click();
        for (const group of [first, groups.nth(1)]) {
            await expect(group.getByRole("tab", { name: "Config" })).toHaveAttribute("aria-selected", "true");
            await expect(group.locator("figure.rd-code").nth(0)).toBeHidden();
            await expect(group.locator("figure.rd-code").nth(1)).toBeVisible();
        }
        // Arrow keys move between tabs.
        await first.getByRole("tab", { name: "Config" }).focus();
        await page.keyboard.press("ArrowLeft");
        await expect(first.getByRole("tab", { name: "Cargo" })).toBeFocused();
        await expect(groups.nth(1).getByRole("tab", { name: "Cargo" })).toHaveAttribute("aria-selected", "true");
        await page.keyboard.press("End");
        await page.goto(`${base()}guide/`);
        const other = page.locator('.rd-code-group[data-rd-group="install"]');
        await expect(other.getByRole("tab", { name: "Config" })).toHaveAttribute("aria-selected", "true");
        await expect(other.getByRole("tabpanel")).toContainText("schema_version");
    });

    test("highlighting spans stay inline, block scopes included", async ({ page }) => {
        await page.goto(`${base()}guide/setup/`);
        const displays = await page.evaluate(() =>
            [...document.querySelectorAll(".rd-code-pre .hl-meta.hl-block, .rd-code-pre [class^='hl-']")].map((span) => getComputedStyle(span).display),
        );
        expect(displays.length).toBeGreaterThan(5);
        expect(new Set(displays)).toEqual(new Set(["inline"]));
    });
});

test.describe("navigation", () => {
    test("the sidebar marks the page, and the contents follow the reader to the last section", async ({ page }) => {
        await page.setViewportSize({ width: 1280, height: 600 });
        await page.goto(`${base()}guide/`);
        await expect(page.locator('.rd-sidebar-wide a[aria-current="page"]')).toHaveText("Guide");
        // One navigation and one table of contents in the accessibility
        // tree, with their links: a closed <details> would hide them in
        // WebKit.
        await expect(page.getByRole("navigation", { name: "Menu" })).toHaveCount(1);
        await expect(page.getByRole("navigation", { name: "Menu" }).getByRole("link", { name: "Setup" })).toBeVisible();
        await expect(page.getByRole("navigation", { name: "On this page" })).toHaveCount(1);
        const toc = page.locator(".rd-toc-wide");
        await expect(toc).toBeVisible();
        await page.evaluate(() => {
            const top = document.getElementById("steps-1")!.getBoundingClientRect().top;
            window.scrollBy(0, top - 80);
        });
        await expect(toc.locator('a[aria-current="location"]')).toHaveAttribute("href", "#steps-1");
        await page.evaluate(() => window.scrollTo(0, document.documentElement.scrollHeight));
        await expect(toc.locator('a[aria-current="location"]')).toHaveAttribute("href", "#check-your-install");
    });

    test("on a phone the menu and the contents are panels that close on a link and on Escape", async ({ page }) => {
        await page.setViewportSize({ width: 375, height: 740 });
        await page.goto(`${base()}guide/setup/`);
        await noHorizontalScroll(page);
        const menu = page.locator("details.rd-sidebar-narrow");
        await expect(menu).not.toHaveAttribute("open", "");
        await menu.locator("summary").click();
        await expect(menu.getByRole("link", { name: "Guide", exact: true })).toBeVisible();
        await page.keyboard.press("Escape");
        await expect(menu).not.toHaveAttribute("open", "");
        await expect(menu.locator("summary")).toBeFocused();
        const toc = page.locator("details.rd-toc-narrow");
        await toc.locator("summary").click();
        await toc.getByRole("link", { name: "Check the version" }).click();
        await expect(toc).not.toHaveAttribute("open", "");
        await expect(page.locator("h3#check-the-version")).toBeInViewport();
    });

    test("the columns follow the width, and nothing scrolls sideways down to 320 px or at 200 %", async ({ page }) => {
        for (const [width, sidebar, toc] of [
            [1280, true, true],
            [1024, true, false],
            [768, true, false],
            [640, false, false],
            [375, false, false],
            [320, false, false],
        ] as const) {
            await page.setViewportSize({ width, height: 800 });
            await page.goto(`${base()}guide/setup/`);
            await noHorizontalScroll(page);
            await expect(page.locator(".rd-sidebar-wide"), `${width}: sidebar`).toBeVisible({ visible: sidebar });
            await expect(page.locator(".rd-sidebar-narrow > summary"), `${width}: menu`).toBeVisible({ visible: !sidebar });
            await expect(page.locator(".rd-toc-wide"), `${width}: contents`).toBeVisible({ visible: toc });
            await expect(page.locator(".rd-toc-narrow > summary"), `${width}: contents panel`).toBeVisible({ visible: !toc });
        }
        await page.goto(base());
        await noHorizontalScroll(page);
    });
});

test.describe("without script", () => {
    test.use({ javaScriptEnabled: false });

    test("everything reads, every code tab is shown and nothing pretends to be a control", async ({ page }) => {
        const wasm: string[] = [];
        page.on("request", (request) => {
            if (request.url().endsWith(".wasm")) wasm.push(request.url());
        });
        await page.goto(`${base()}guide/setup/`);
        await expect(page.locator("[data-rd-mode-switch]")).toBeHidden();
        await expect(page.locator("[data-rd-copy]").first()).toBeHidden();
        const group = page.locator(".rd-code-group").first();
        await expect(group.locator('[role="tablist"]')).toBeHidden();
        await expect(group.locator("figure.rd-code")).toHaveCount(2);
        for (const figure of await group.locator("figure.rd-code").all()) {
            await expect(figure).toBeVisible();
        }
        await expect(page.locator(".hl-source").first()).toBeVisible();
        await page.getByRole("link", { name: "Install the CLI" }).click();
        await expect(page).toHaveURL(/#install-the-cli$/);
        expect(wasm).toEqual([]);
    });
});

test.describe("reading quality", () => {
    test("one h1, landmarks and the page's language", async ({ page }) => {
        for (const [path, lang] of [
            ["", "en"],
            ["guide/setup/", "en"],
            ["reference/api/", "zh-CN"],
            ["no/such/page/", "en"],
        ] as const) {
            await page.goto(`${base()}${path}`);
            await expect(page.locator("html")).toHaveAttribute("lang", lang);
            await expect(page.locator("h1")).toHaveCount(1);
            await expect(page.getByRole("banner")).toHaveCount(1);
            await expect(page.getByRole("main")).toHaveCount(1);
        }
    });

    test("text, links, alerts and actions reach AA contrast in both modes", async ({ browser }) => {
        for (const scheme of ["light", "dark"] as const) {
            const context = await browser.newContext({ colorScheme: scheme });
            const page = await context.newPage();
            await page.goto(`${base()}guide/`);
            const checks: [string, number][] = [
                [".rd-prose p", 4.5],
                [".rd-prose a", 4.5],
                [".rd-sidebar-wide a:not([aria-current])", 4.5],
                ['.rd-sidebar-wide a[aria-current="page"]', 4.5],
                [".rd-toc-wide a", 4.5],
                [".rd-pager-label", 4.5],
                [".markdown-alert-tip .markdown-alert-title", 4.5],
                [".markdown-alert-important .markdown-alert-title", 4.5],
                [".markdown-alert-caution .markdown-alert-title", 4.5],
                [".markdown-alert-tip p:not(.markdown-alert-title)", 4.5],
                [".rd-code-tabs [role=tab]:not([aria-selected=true])", 4.5],
            ];
            for (const [selector, minimum] of checks) {
                expect(await contrast(page, selector), `${scheme}: ${selector}`).toBeGreaterThanOrEqual(minimum);
            }
            await page.goto(base());
            expect(await contrast(page, ".rd-action-primary"), `${scheme}: primary action`).toBeGreaterThanOrEqual(4.5);
            expect(await contrast(page, ".rd-tagline"), `${scheme}: tagline`).toBeGreaterThanOrEqual(4.5);
            expect(await contrast(page, ".rd-features p"), `${scheme}: feature text`).toBeGreaterThanOrEqual(4.5);
            await page.locator(".rd-action-primary").hover();
            expect(await contrast(page, ".rd-action-primary"), `${scheme}: primary action, hovered`).toBeGreaterThanOrEqual(4.5);
            await context.close();
        }
    });

    test("focus is an opaque ring with a gap, and the logo shows in both modes", async ({ browser, browserName }) => {
        // Safari moves focus to links with Option-Tab; Tab alone skips them.
        const tab = browserName === "webkit" ? "Alt+Tab" : "Tab";
        for (const scheme of ["light", "dark"] as const) {
            const context = await browser.newContext({ colorScheme: scheme });
            const page = await context.newPage();
            await page.goto(`${base()}guide/`);
            await page.keyboard.press(tab);
            await page.keyboard.press(tab);
            expect(await page.evaluate(() => document.activeElement?.className)).toBe("rd-brand");
            const ring = await page.evaluate(() => {
                const style = getComputedStyle(document.activeElement!);
                return { width: style.outlineWidth, style: style.outlineStyle, offset: style.outlineOffset, color: style.outlineColor };
            });
            expect(ring).toMatchObject({ width: "3px", style: "solid", offset: "2px" });
            expect(ring.color).toBe(scheme === "light" ? "rgb(23, 92, 211)" : "rgb(83, 177, 253)");
            const logo = page.locator(".rd-brand img");
            await expect(logo).toBeVisible();
            expect(await logo.evaluate((img) => (img as HTMLImageElement).naturalWidth)).toBeGreaterThan(0);
            await context.close();
        }
    });

    test("axe finds no serious or critical WCAG A/AA violation, wide and narrow, in both modes", async ({ browser }) => {
        const found: string[] = [];
        for (const scheme of ["light", "dark"] as const) {
            for (const width of [1280, 375]) {
                const context = await browser.newContext({ colorScheme: scheme, viewport: { width, height: 800 } });
                const page = await context.newPage();
                for (const path of ["", "guide/", "guide/setup/", "reference/api/", "no/such/page/"]) {
                    await page.goto(`${base()}${path}`);
                    const { violations } = await new AxeBuilder({ page })
                        .withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"])
                        .analyze();
                    for (const violation of violations) {
                        const line = `${scheme} ${width}px /${path} ${violation.impact} ${violation.id}: ${violation.nodes.map((node) => node.target.join(" ")).join(", ")}`;
                        found.push(line);
                    }
                }
                await context.close();
            }
        }
        await test.info().attach("axe-violations.txt", { body: found.join("\n") || "none", contentType: "text/plain" });
        expect(found.filter((line) => / (serious|critical) /.test(line))).toEqual([]);
    });

    test("reduced motion turns smooth scrolling off", async ({ browser }) => {
        const context = await browser.newContext({ reducedMotion: "reduce" });
        const page = await context.newPage();
        await page.goto(`${base()}guide/`);
        expect(await page.evaluate(() => getComputedStyle(document.documentElement).scrollBehavior)).toBe("auto");
        await context.close();
    });
});
