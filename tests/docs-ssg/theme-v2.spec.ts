import { expect, test, type Locator, type Page } from "@playwright/test";
import { mkdirSync, readFileSync } from "node:fs";
import { join } from "node:path";

type Expected = {
    document_id: string;
    config: { appearance: string; accent_color: string; gray_color: string; scaling: number };
    light: Record<string, string>;
    dark: Record<string, string>;
};
type Audit = { errors: string[]; requests: string[]; failures: string[] };
declare global {
    interface Window {
        __docs_v2_csp?: string[];
        __docs_v2_button?: Element;
    }
}

let expected: Expected;
const site = process.env.RUSTIFY_DOCS_V2_SITE || "tests/docs-ssg/target/theme-v2-author";
test.beforeAll(() => {
    expected = JSON.parse(readFileSync(join(site, ".rustify-docs/theme-v2-expected.json"), "utf8"));
    expect(expected.config.appearance).toBe("dark");
    expect(expected.config.scaling).toBe(110);
});

async function watch(page: Page): Promise<Audit> {
    const audit: Audit = { errors: [], requests: [], failures: [] };
    await page.addInitScript(() => {
        window.__docs_v2_csp = [];
        document.addEventListener("securitypolicyviolation", event => window.__docs_v2_csp!.push(`${event.violatedDirective} ${event.blockedURI}`));
    });
    page.on("pageerror", error => audit.errors.push(error.message));
    page.on("console", message => { if (message.type() === "error") audit.errors.push(message.text()); });
    page.on("request", request => audit.requests.push(request.url()));
    page.on("requestfailed", request => audit.failures.push(request.url()));
    page.on("response", response => { if (response.status() >= 400) audit.failures.push(`${response.status()} ${response.url()}`); });
    return audit;
}

async function clean(page: Page, audit: Audit) {
    expect(audit.errors).toEqual([]);
    expect(audit.failures).toEqual([]);
    expect(await page.evaluate(() => window.__docs_v2_csp ?? [])).toEqual([]);
    expect(audit.requests.filter(url => /\.(woff2?|ttf|otf)(?:\?|$)/.test(url))).toEqual([]);
}

async function color(node: Locator, property: string, expectedColor: string) {
    const sample = () => node.evaluate((element, [property, expectedColor]) => {
        const actual = getComputedStyle(element).getPropertyValue(property);
        const canvas = document.createElement("canvas");
        canvas.width = canvas.height = 1;
        const context = canvas.getContext("2d")!;
        const rgba = (value: string) => {
            context.clearRect(0, 0, 1, 1);
            context.fillStyle = value;
            context.fillRect(0, 0, 1, 1);
            return Array.from(context.getImageData(0, 0, 1, 1).data);
        };
        return { actual: rgba(actual), expected: rgba(expectedColor) };
    }, [property, expectedColor]);
    const target = (await sample()).expected;
    await expect.poll(async () => (await sample()).actual, { message: `${property}: ${expectedColor}` }).toEqual(target);
}

async function projection(page: Page, mode: "light" | "dark") {
    const values = expected[mode];
    const root = page.locator("html");
    const probe = page.getByTestId("docs-v2-probe");
    await expect(probe).toHaveAttribute("data-appearance", "dark");
    await expect(probe).toHaveAttribute("data-brand", "blue");
    await expect(probe).toHaveAttribute("data-gray", "slate");
    await expect(probe).toHaveAttribute("data-scaling", "110");
    const mismatches = await root.evaluate((element, values) => {
        const style = getComputedStyle(element);
        const reference = document.createElement("span");
        document.body.append(reference);
        const numbers = (value: string) => value.replace(/-?\d*\.?\d+(?:e[+-]?\d+)?/gi, number => String(Number(Number(number).toFixed(6)))).replace(/\s+/g, " ").trim();
        const normalize = (name: string, value: string, original: string) => {
            if (original === "initial") return value.trim();
            if (original.startsWith("rgb(")) {
                reference.style.color = value;
                return getComputedStyle(reference).color;
            }
            if (name.startsWith("--rustify-shadow") && original.includes("rgb(")) {
                reference.style.boxShadow = value;
                return numbers(getComputedStyle(reference).boxShadow);
            }
            return numbers(value);
        };
        const mismatches = Object.entries(values).flatMap(([name, value]) => {
            const actual = style.getPropertyValue(name);
            if (!actual.trim() && value !== "initial") return [{ name, expected: value, actual }];
            const quantity = (value: string) => {
                const match = /^(-?\d*\.?\d+(?:e[+-]?\d+)?)(px|em|ms|s)$/i.exec(value.trim());
                return match ? { value: Number(match[1]) * (match[2] === "s" ? 1000 : 1), unit: match[2] === "s" ? "ms" : match[2] } : undefined;
            };
            const actualQuantity = quantity(actual);
            const expectedQuantity = quantity(value);
            if (actualQuantity && expectedQuantity && actualQuantity.unit === expectedQuantity.unit) {
                const tolerance = Math.max(1e-4, Math.abs(expectedQuantity.value) * 1e-5);
                return Math.abs(actualQuantity.value - expectedQuantity.value) <= tolerance ? [] : [{ name, expected: value, actual }];
            }
            // CSS-wide initial makes these unregistered custom properties
            // absent. Other properties compare their complete resolved value.
            const wanted = value === "initial" ? "" : normalize(name, value, value);
            const received = normalize(name, actual, value);
            return wanted === received ? [] : [{ name, expected: wanted, actual: received }];
        });
        reference.remove();
        return mismatches;
    }, values);
    expect(mismatches).toEqual([]);
    for (const name of ["--primary", "--rustify-brand-12", "--rustify-brand-a3", "--rustify-gray-12", "--rustify-panel", "--rustify-surface"]) {
        await color(root, name, values[name]);
    }
    const button = page.getByTestId("docs-v2-button");
    await color(button, "background-color", values["--primary"]);
    await color(button, "color", values["--primary-foreground"]);
    expect(Math.abs(await button.evaluate(element => element.getBoundingClientRect().height) - parseFloat(values["--rustify-control-height-2"]))).toBeLessThanOrEqual(1 / 64);
    await expect(button).toHaveCSS("border-top-left-radius", "9999px");
    expect(Math.abs(await page.getByTestId("docs-v2-text").evaluate(element => parseFloat(getComputedStyle(element).fontSize)) - parseFloat(values["--rustify-font-size-3"]))).toBeLessThanOrEqual(1 / 64);
    expect(Math.abs(await page.getByTestId("docs-v2-heading").evaluate(element => parseFloat(getComputedStyle(element).lineHeight)) - parseFloat(values["--rustify-heading-line-height-6"]))).toBeLessThanOrEqual(1 / 64);
    const shadows = await page.getByTestId("docs-v2-shadow").evaluate((element, expected) => {
        const reference = document.createElement("span");
        reference.style.boxShadow = expected;
        document.body.append(reference);
        const shadows = { actual: getComputedStyle(element).boxShadow, expected: getComputedStyle(reference).boxShadow };
        reference.remove();
        // Tailwind composes shadows with transparent ring/inset placeholders.
        // Keep every visible layer and compare both complete theme layers.
        shadows.actual = shadows.actual.split(/,\s*(?![^()]*\))/).filter(layer => !layer.startsWith("rgba(0, 0, 0, 0) ")).join(", ");
        return shadows;
    }, values["--rustify-shadow-lg"]);
    expect(shadows.actual).toBe(shadows.expected);
}

test("source v2 remains readable without script while Docs reader CSS selects either mode", async ({ browser, baseURL }) => {
    for (const mode of ["light", "dark"] as const) {
        const context = await browser.newContext({ javaScriptEnabled: false, colorScheme: mode });
        const page = await context.newPage();
        const audit = await watch(page);
        try {
            const response = await page.goto(baseURL!);
            expect(response?.headers()["content-security-policy"]).toContain("script-src");
            await expect(page.locator("html")).toHaveAttribute("data-theme", "system");
            await expect(page.getByTestId("docs-v2-probe")).toHaveAttribute("data-mode", "light");
            // SSR cannot know the reader's OS preference. It preserves its
            // initial Light metadata; both static CSS palettes are available.
            await projection(page, mode);
            await expect(page.getByTestId("docs-v2-count")).toHaveText("0");
            await clean(page, audit);
        } finally { await context.close(); }
    }
});

test("hydrated source context follows explicit Docs reader mode and keeps authored appearance", async ({ page }, info) => {
    await page.emulateMedia({ colorScheme: "light" });
    await page.addInitScript(() => localStorage.setItem("rustify-docs:mode:/theme-v2/", "light"));
    const audit = await watch(page);
    await page.goto("./");
    await expect(page.locator('[data-rustify-island][data-rd-island-state="ready"]')).toHaveCount(1);
    const probe = page.getByTestId("docs-v2-probe");
    await expect(probe).toHaveAttribute("data-mode", "light");
    await projection(page, "light");
    await page.getByTestId("docs-v2-button").click();
    await expect(page.getByTestId("docs-v2-count")).toHaveText("1");
    await page.getByTestId("docs-v2-button").evaluate(element => window.__docs_v2_button = element);
    await page.locator('[data-rd-mode="dark"]').click();
    await expect(probe).toHaveAttribute("data-mode", "dark");
    await projection(page, "dark");
    await page.getByTestId("docs-v2-dialog-open").click();
    const layer = page.getByTestId("docs-v2-dialog");
    await expect(layer).toHaveAttribute("role", "presentation");
    const dialog = layer.locator('[data-name="Dialog"]');
    await expect(dialog).toBeVisible();
    await color(page.getByTestId("docs-v2-dialog-content"), "background-color", expected.dark["--card"]);
    await page.getByTestId("docs-v2-dialog-close").click();
    await expect(dialog).toBeHidden();
    await page.locator('[data-rd-mode="light"]').click();
    await expect(probe).toHaveAttribute("data-mode", "light");
    await page.getByTestId("docs-v2-dialog-open").click();
    await expect(dialog).toBeVisible();
    await color(page.getByTestId("docs-v2-dialog-content"), "background-color", expected.light["--card"]);
    await page.keyboard.press("Escape");
    await expect(dialog).toBeHidden();
    await page.emulateMedia({ colorScheme: "dark" });
    await page.locator('[data-rd-mode="system"]').click();
    await expect(probe).toHaveAttribute("data-mode", "dark");
    await page.emulateMedia({ colorScheme: "light" });
    await expect(probe).toHaveAttribute("data-mode", "light");
    await projection(page, "light");
    expect(await page.getByTestId("docs-v2-button").evaluate(element => element === window.__docs_v2_button)).toBe(true);
    await expect(page.getByTestId("docs-v2-count")).toHaveText("1");
    const evidence = "docs/validation/dom-components-p1/export";
    mkdirSync(evidence, { recursive: true });
    await page.screenshot({ path: join(evidence, "source-docs-v2.png"), fullPage: true });
    await info.attach("Docs full v2 reader policy", { body: JSON.stringify({ appearance: expected.config.appearance, readerModes: ["light", "dark", "system"], framework: JSON.parse(readFileSync(join(site, ".rustify-docs/theme-v2-provenance.json"), "utf8")) }, null, 2), contentType: "application/json" });
    await clean(page, audit);
});
