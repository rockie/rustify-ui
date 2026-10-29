import type { ConsoleMessage, Locator, Page, Request } from "@playwright/test";
import { test, expect, waitForReady } from "./support";

type CatalogLayout = typeof window.__component_catalog & {
    mount_layout_fixtures(): void;
    dispose_layout_fixtures(): void;
};

type Evidence = {
    errors: string[];
    onError: (error: Error) => void;
    onConsole: (message: ConsoleMessage) => void;
    onRequest: (request: Request) => void;
    sheets: number;
    host: string[];
};
const evidence = new WeakMap<Page, Evidence>();
const root = (page: Page) => page.locator("#catalog-layout-fixture");
const sample = (page: Page, id: string) => root(page).getByTestId(id);
const number = (node: Locator, property: string) => node.evaluate((element, property) =>
    parseFloat(getComputedStyle(element).getPropertyValue(property)), property);

async function hostStyles(page: Page): Promise<string[]> {
    return page.getByTestId("host-button").evaluate(element => {
        const css = getComputedStyle(element);
        return [css.color, css.backgroundColor, css.fontSize, css.borderRadius, css.padding, css.display];
    });
}

test.beforeEach(async ({ page }) => {
    await waitForReady(page);
    await page.setViewportSize({ width: 1280, height: 900 });
    const errors: string[] = [];
    const onError = (error: Error) => errors.push(error.message);
    const onConsole = (message: ConsoleMessage) => { if (message.type() === "error") errors.push(message.text()); };
    const onRequest = (request: Request) => errors.push(`${request.method()} ${request.url()}: ${request.failure()?.errorText}`);
    page.on("pageerror", onError);
    page.on("console", onConsole);
    page.on("requestfailed", onRequest);
    evidence.set(page, {
        errors, onError, onConsole, onRequest,
        sheets: await page.evaluate(() => document.adoptedStyleSheets.length),
        host: await hostStyles(page),
    });
    await page.evaluate(() => {
        const state = window as typeof window & {
            __layoutFixtureCsp: string[];
            __layoutFixtureCspListener: (event: SecurityPolicyViolationEvent) => void;
        };
        state.__layoutFixtureCsp = [];
        state.__layoutFixtureCspListener = event => state.__layoutFixtureCsp.push(event.violatedDirective);
        document.addEventListener("securitypolicyviolation", state.__layoutFixtureCspListener);
        (window.__component_catalog as CatalogLayout).mount_layout_fixtures();
    });
    await expect(sample(page, "layout-body")).toBeVisible();
});

for (const width of [1280, 390]) {
    test(`Code ghost keeps neutral foreground and explicit gray roles at ${width}px`, async ({ page }) => {
        await page.setViewportSize({ width, height: 900 });
        const matrix = sample(page, "type-code-matrix");
        const neutral = await root(page).evaluate(element => {
            const style = document.createElement("span").style;
            style.color = getComputedStyle(element).getPropertyValue("--foreground");
            return style.color;
        });
        const color = (selector: string) => matrix.locator(selector).evaluate(element => getComputedStyle(element).color);
        const ghost = '.rustify-code[data-variant="ghost"]';
        expect(await color(`${ghost}[data-color="indigo"]:not([data-high-contrast])`)).toBe(neutral);
        expect(await color(`${ghost}[data-color="indigo"][data-high-contrast]`)).toBe(neutral);
        expect(await color(`${ghost}[data-color="gray"]:not([data-high-contrast])`)).toBe(
            await color('.rustify-code[data-variant="soft"][data-color="gray"]:not([data-high-contrast])'));
        expect(await color(`${ghost}[data-color="gray"][data-high-contrast]`)).toBe(
            await color('.rustify-code[data-variant="outline"][data-color="gray"][data-high-contrast]'));
        const plain = await matrix.locator(`${ghost}:not([data-color])`).evaluateAll(elements => elements.map(element => getComputedStyle(element).color));
        expect(plain).toHaveLength(9);
        expect(plain.every(color => color === neutral)).toBe(true);
        const surfaces = await matrix.locator(ghost).evaluateAll(elements => elements.map(element => {
            const css = getComputedStyle(element);
            return [css.backgroundColor, css.boxShadow, css.borderStyle];
        }));
        expect(surfaces.every(([background, shadow, border]) => background === "rgba(0, 0, 0, 0)" && shadow === "none" && border === "none")).toBe(true);
        for (const variant of ["solid", "soft", "outline"]) {
            expect(await color(`.rustify-code[data-variant="${variant}"][data-color="indigo"]:not([data-high-contrast])`)).not.toBe(neutral);
        }
    });
}

test.afterEach(async ({ page }) => {
    const state = evidence.get(page);
    if (!state) return;
    try {
        expect(await page.evaluate(() => (window as any).__layoutFixtureCsp)).toEqual([]);
        expect(await hostStyles(page)).toEqual(state.host);
        // Screenshot caret restoration may leave an empty style attribute.
        // Check CSSOM declarations rather than the presence of that attribute.
        const inlineDeclarations = await root(page).locator("[style]").evaluateAll(elements => elements
            .filter(element => (element as HTMLElement).style.length > 0)
            .map(element => ({ tag: element.tagName, testId: element.getAttribute("data-testid"), css: (element as HTMLElement).style.cssText })));
        expect(inlineDeclarations, "component descendants must keep geometry in external CSSOM rules").toEqual([]);
        await page.evaluate(() => {
            (window.__component_catalog as CatalogLayout).dispose_layout_fixtures();
            const state = window as any;
            document.removeEventListener("securitypolicyviolation", state.__layoutFixtureCspListener);
        });
        await expect.poll(() => page.evaluate(() => document.adoptedStyleSheets.length)).toBe(state.sheets);
        await expect(root(page)).not.toHaveAttribute("data-rustify-scope");
        expect(state.errors).toEqual([]);
    } finally {
        page.off("pageerror", state.onError);
        page.off("console", state.onConsole);
        page.off("requestfailed", state.onRequest);
        evidence.delete(page);
    }
});

test("layout and typography switch at each Themes breakpoint, including both sides", async ({ page }) => {
    for (const width of [375, 519, 520, 767, 768, 1023, 1024, 1279, 1280, 1639, 1640]) {
        await page.setViewportSize({ width, height: 900 });
        const index = [0, 520, 768, 1024, 1280, 1640].filter(minimum => minimum <= width).length - 1;
        await expect(sample(page, "layout-responsive-width")).toHaveCSS("width", `${[10, 20, 30, 40, 50, 60][index]}px`);
        await expect(sample(page, "type-responsive")).toHaveCSS("font-size", `${[14, 16, 18, 20, 24, 28][index]}px`);
        await expect(sample(page, "type-responsive")).toHaveCSS("font-weight", width >= 1280 ? "700" : width >= 768 ? "500" : "400");
        await expect(sample(page, "type-responsive")).toHaveCSS("text-align", width >= 1024 ? "center" : "left");
        await expect(sample(page, "type-responsive")).toHaveCSS("white-space", width >= 1640 ? "nowrap" : "normal");
        await expect(sample(page, "layout-flex")).toHaveCSS("flex-direction", width >= 1280 ? "row-reverse" : width >= 768 ? "column" : "row");
        await expect(sample(page, "layout-sparse-display")).toHaveCSS("display", width >= 1024 && width < 1280 ? "none" : "block");
        await expect(sample(page, "layout-container").locator(".rustify-container-inner")).toHaveCSS("max-width", `${width >= 1280 ? 1136 : width >= 1024 ? 880 : width >= 768 ? 688 : 448}px`);
        await expect(sample(page, "layout-section")).toHaveCSS("padding-top", `${width >= 1640 ? 80 : width >= 1024 ? 64 : width >= 520 ? 40 : 24}px`);
        await expect(sample(page, "layout-section")).toHaveCSS("padding-bottom", `${width >= 1640 ? 80 : width >= 1024 ? 64 : width >= 520 ? 40 : 24}px`);
    }
});

test("Kbd rounding follows its font size across all nine sizes and theme scaling", async ({ page }) => {
    const matrix = sample(page, "type-kbd-matrix");
    for (const scale of [100, 110, 95]) {
        await sample(page, `layout-scale-${scale}`).click();
        await expect.poll(() => number(matrix.locator(".fixture-kbd-row:not(.fixture-prop) kbd").first(), "border-radius"))
            .toBeCloseTo(16.8 * scale / 100, 2);
        const rows = await matrix.locator(".fixture-kbd-row:not(.fixture-prop)").evaluateAll(elements => elements.map(row =>
            Array.from(row.querySelectorAll("kbd"), element => {
                const css = getComputedStyle(element);
                return { font: parseFloat(css.fontSize), radius: parseFloat(css.borderRadius) };
            })));
        expect(rows).toHaveLength(9);
        for (const row of rows) {
            expect(row).toHaveLength(4);
            for (const key of row) expect(key.radius / key.font).toBeCloseTo(0.35, 3);
        }
    }
});

test("shared layout fields, Container projection and caller utilities keep their meanings", async ({ page }) => {
    const box = sample(page, "layout-shared");
    const expected: Record<string, string> = {
        "box-sizing": "border-box", width: "160px", "min-width": "80px", "max-width": "240px",
        height: "90px", "min-height": "40px", "max-height": "180px", position: "relative",
        top: "4px", right: "8px", bottom: "12px", left: "16px",
        "padding-top": "8px", "padding-right": "12px", "padding-bottom": "16px", "padding-left": "12px",
        "margin-top": "24px", "margin-right": "32px", "margin-bottom": "40px", "margin-left": "48px",
        "overflow-x": "auto", "overflow-y": "scroll", "flex-basis": "100px", "flex-grow": "2", "flex-shrink": "0.5",
        "grid-column-start": "3", "grid-column-end": "5", "grid-row-start": "2", "grid-row-end": "4",
        "align-self": "start", "justify-self": "end",
    };
    for (const [property, value] of Object.entries(expected)) {
        await expect(box, `layout-shared ${property}`).toHaveCSS(property, value);
    }
    await expect(sample(page, "layout-box-default")).toHaveCSS("display", "block");
    expect(await sample(page, "layout-box-default").evaluate(element => element.tagName)).toBe("SPAN");
    const flex = sample(page, "layout-flex");
    await expect(flex).toHaveCSS("display", "flex");
    await expect(flex).toHaveCSS("justify-content", "space-between");
    await expect(flex).toHaveCSS("align-items", "center");
    await expect(flex).toHaveCSS("flex-wrap", "wrap");
    await expect(flex).toHaveCSS("column-gap", "8px");
    await expect(flex).toHaveCSS("row-gap", "16px");
    const grid = sample(page, "layout-grid");
    await expect(grid).toHaveCSS("display", "grid");
    expect(await grid.evaluate(element => getComputedStyle(element).gridAutoFlow)).toMatch(/^(row )?dense$/);
    await expect(grid).toHaveCSS("align-content", "space-around");
    await expect(grid).toHaveCSS("justify-items", "center");
    await expect(grid).toHaveCSS("column-gap", "12px");
    await expect(grid).toHaveCSS("row-gap", "16px");
    expect(await grid.evaluate(element => getComputedStyle(element).gridTemplateColumns.split(" ").length)).toBe(3);
    const outer = sample(page, "layout-container-projection");
    const inner = outer.locator(".rustify-container-inner");
    await expect(inner).toHaveCSS("width", "240px");
    await expect(inner).toHaveCSS("height", "72px");
    await expect(inner).toHaveCSS("min-width", "120px");
    await expect(inner).toHaveCSS("max-width", "280px");
    await expect(inner).toHaveCSS("min-height", "40px");
    await expect(inner).toHaveCSS("max-height", "90px");
    expect(await number(outer, "width")).toBeGreaterThan(240);
    await expect(sample(page, "layout-class-override")).toHaveCSS("width", "80px");
    await expect(sample(page, "layout-class-override")).toHaveCSS("padding", "0px");
});

test("Inset consumes only nearest padding and Reset preserves the native event", async ({ page }) => {
    const inset = sample(page, "layout-inset");
    await expect(inset).toHaveCSS("margin-left", "-8px");
    await expect(inset).toHaveCSS("margin-top", "-8px");
    await expect(inset).toHaveCSS("padding-top", "8px");
    await expect(inset).toHaveCSS("padding-right", "0px");
    await expect(sample(page, "layout-inset-nested")).toHaveCSS("margin-left", "0px");
    const nearest = await sample(page, "layout-inset-nearest").boundingBox();
    const outer = await sample(page, "layout-inset-outer").boundingBox();
    const inside = await inset.boundingBox();
    expect(inside!.x).toBeCloseTo(nearest!.x, 1);
    expect(nearest!.x - outer!.x).toBeCloseTo(32, 1);
    const reset = sample(page, "layout-reset");
    expect(await reset.evaluate(element => element.tagName)).toBe("BUTTON");
    await expect(reset).toHaveAttribute("type", "button");
    await expect(reset).toHaveAccessibleName("Native reset action");
    await reset.click();
    await expect(sample(page, "layout-reset-count")).toHaveText("1");
    await reset.focus();
    await page.keyboard.press("Enter");
    await expect(sample(page, "layout-reset-count")).toHaveText("2");
    await expect(reset).toHaveCSS("box-sizing", "border-box");
    await expect(page.locator("input.fixture-reset-outline")).toHaveCSS("outline-offset","-1px");
    await expect(page.locator("textarea.fixture-reset-outline")).toHaveCSS("outline-offset","-1px");
    expect(await hostStyles(page)).toEqual(evidence.get(page)!.host);
});

test("theme scaling applies once while absolute values and utility overrides stay exact", async ({ page }) => {
    for (const [scale, padding] of [[110, 17.6], [95, 15.2], [100, 16]] as const) {
        await sample(page, `layout-scale-${scale}`).click();
        await expect.poll(() => number(sample(page, "layout-scaled-space"), "padding-left")).toBeCloseTo(padding, 2);
        await expect(sample(page, "layout-absolute")).toHaveCSS("width", "37px");
        await expect(sample(page, "layout-absolute")).toHaveCSS("height", "19px");
        await expect(sample(page, "layout-absolute")).toHaveCSS("padding-left", "3px");
        await expect(sample(page, "type-class-override")).toHaveCSS("font-size", "14px");
        await expect.poll(() => number(sample(page, "type-margin"), "margin-left")).toBeCloseTo(8 * scale / 100, 2);
    }
});

test("Inset selects each side and restores other margins across responsive changes", async ({ page }, info) => {
    const cases: [string, boolean[]][] = [
        ["all", [true, true, true, true]], ["x", [false, true, false, true]],
        ["y", [true, false, true, false]], ["top", [true, true, false, true]],
        ["bottom", [false, true, true, true]], ["left", [true, false, true, true]],
        ["right", [true, true, true, false]],
    ];
    for (const [side, edges] of cases) {
        for (const [edge, property] of ["margin-top", "margin-right", "margin-bottom", "margin-left"].entries()) {
            await expect(sample(page, `layout-inset-${side}`)).toHaveCSS(property, edges[edge] ? "-13px" : "3px");
        }
    }
    for (const width of [390, 519, 520, 767, 768, 1023, 1024, 1280]) {
        await page.setViewportSize({width, height:900});
        const side = width >= 1024 ? "left" : width >= 768 ? "bottom" : width >= 520 ? "right" : "top";
        const edges = cases.find(([name]) => name === side)![1];
        for (const [edge, property] of ["margin-top", "margin-right", "margin-bottom", "margin-left"].entries()) {
            await expect(sample(page, "layout-inset-responsive")).toHaveCSS(property, edges[edge] ? "-13px" : "3px");
        }
    }
    await sample(page, "layout-inset-sides").screenshot({path:info.outputPath("inset-sides-desktop.png")});
    await page.setViewportSize({width:390, height:900});
    await sample(page, "layout-inset-sides").screenshot({path:info.outputPath("inset-sides-mobile.png")});
});

test("invalid CSS remains local, reports its prop and accepts a valid replacement", async ({ page }) => {
    const node = sample(page, "layout-invalid-prop");
    await expect(node).toHaveCSS("width", "240px");
    await expect(node).toHaveCSS("box-sizing","border-box");
    await expect(node).toHaveCSS("padding","2px 8px");
    await expect(node).toHaveCSS("margin-top","4px");
    await sample(page, "layout-invalid").click();
    await expect(node).toHaveText("A bad prop keeps this view alive.");
    await expect.poll(() => number(node, "width")).toBeGreaterThan(240);
    await expect(sample(page, "layout-prop-diagnostics")).toContainText("width");
    await expect(sample(page, "layout-prop-diagnostics")).toContainText("12px;display:none");
    const diagnostics = await page.evaluate(() => window.__component_catalog.diagnostics());
    expect(diagnostics.entries.some((entry: any) => entry.kind === "InvalidContainer" && entry.field === "width")).toBe(true);
    await sample(page, "layout-valid").click();
    await expect(node).toHaveCSS("width", "240px");
    await expect(sample(page, "layout-prop-diagnostics")).toBeEmpty();
    await expect(sample(page, "layout-reset")).toBeEnabled();
});

test("eight typography components and Link retain native semantics and real text", async ({ page }, info) => {
    for (const [id, tag] of [
        ["type-paragraph", "P"], ["type-label", "LABEL"], ["type-heading", "H3"], ["type-default-heading", "H1"],
        ["type-blockquote", "BLOCKQUOTE"], ["type-code", "CODE"], ["type-kbd", "KBD"],
        ["type-em", "EM"], ["type-quote", "Q"], ["type-strong", "STRONG"], ["type-link", "A"],
    ]) expect(await sample(page, id).evaluate(element => element.tagName)).toBe(tag);
    await expect(sample(page, "type-heading")).toHaveCSS("font-size", "60px");
    await expect(sample(page, "type-default-heading")).toHaveCSS("font-size", "24px");
    await expect(sample(page, "type-paragraph")).toHaveCSS("line-height", "24px");
    await expect(sample(page, "type-paragraph")).toHaveCSS("text-wrap-style", "pretty");
    const trim = await sample(page, "type-paragraph").evaluate(element => [getComputedStyle(element, "::before").content, getComputedStyle(element, "::after").content, getComputedStyle(element, "::before").marginBottom]);
    expect(trim.slice(0, 2)).toEqual(['""', '""']);
    expect(parseFloat(trim[2])).toBeLessThan(0);
    await expect(sample(page, "type-strong")).toHaveCSS("font-weight", "700");
    expect(await sample(page, "type-quote").evaluate(element => getComputedStyle(element, "::before").content)).toBe("open-quote");
    await expect(sample(page, "type-label")).toHaveAttribute("for", "layout-name");
    await sample(page, "type-label").click();
    await expect(root(page).getByRole("textbox", { name: "Your name" })).toBeFocused();
    await expect(sample(page, "type-link")).toHaveAttribute("href", "#layout-target");
    await expect(sample(page, "type-link")).toHaveAttribute("target", "_blank");
    await expect(sample(page, "type-link")).toHaveAttribute("rel", "noreferrer");
    await expect(sample(page, "type-link")).toHaveCSS("text-decoration-line", "underline");
    await expect(sample(page, "type-disabled-link")).not.toHaveAttribute("href");
    for (const id of ["type-truncate", "type-long-link"]) {
        await expect(sample(page, id)).toHaveCSS("text-overflow", "ellipsis");
        await expect(sample(page, id)).toHaveCSS("white-space", "nowrap");
        expect(await sample(page, id).evaluate(element => element.scrollWidth > element.clientWidth)).toBe(true);
    }
    const desktop = process.env.RUSTIFY_UI_EVIDENCE_OUT ? `${process.env.RUSTIFY_UI_EVIDENCE_OUT}/dom-layout-desktop.png` : "docs/validation/dom-components-p1/dom-layout-desktop.png";
    await sample(page, "layout-body").screenshot({ path: desktop });
    await info.attach("DOM layout and type at 1280px", { path: desktop, contentType: "image/png" });
    await page.setViewportSize({ width: 375, height: 900 });
    const paragraph = sample(page, "type-paragraph");
    expect(await paragraph.evaluate(element => element.scrollWidth <= element.clientWidth + 1)).toBe(true);
    const mobile = process.env.RUSTIFY_UI_EVIDENCE_OUT ? `${process.env.RUSTIFY_UI_EVIDENCE_OUT}/dom-layout-mobile.png` : "docs/validation/dom-components-p1/dom-layout-mobile.png";
    await sample(page, "layout-typography").screenshot({ path: mobile });
    await info.attach("DOM typography at 375px", { path: mobile, contentType: "image/png" });
});
