import AxeBuilder from "@axe-core/playwright";
import { expect, test } from "@playwright/test";
import { ChildProcess, execFileSync, spawn } from "node:child_process";
import { cpSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";

// A production site with optional navigation and metadata absent, served
// by the same candidate as the main fixture at the project's base.
let root: string;
let child: ChildProcess;
let url: string;
let base: string;

test.beforeAll(async () => {
    base = test.info().project.name === "docs-root" ? "/" : "/rustify-ui/";
    root = mkdtempSync(path.join(tmpdir(), "rustify-docs-home-"));
    cpSync("tests/docs-ssg/fixtures/home", root, { recursive: true });
    const config = path.join(root, "rustify-docs.toml");
    writeFileSync(config, readFileSync(config, "utf8").replaceAll("/docs/", base));
    const bin = path.resolve(process.env.RUSTIFY_DOCS_BIN || "target/docs-cli/rustify-docs");
    execFileSync(bin, ["build", "--config", path.join(root, "rustify-docs.toml")]);
    child = spawn(bin, ["preview", "--dir", path.join(root, "dist"), "--port", "0"]);
    url = await new Promise<string>((resolve, reject) => {
        let output = "";
        let error = "";
        child.stderr!.on("data", chunk => error += chunk);
        child.stdout!.on("data", chunk => {
            output += chunk;
            const match = /^preview: (\S+)/m.exec(output);
            if (match) resolve(match[1]);
        });
        child.on("exit", code => reject(new Error(`preview exited ${code}: ${output}${error}`)));
    });
});

test.afterAll(async () => {
    if (child && child.exitCode === null && child.signalCode === null) {
        const exited = new Promise(resolve => child.once("exit", resolve));
        child.kill();
        await exited;
    }
    if (root) rmSync(root, { recursive: true, force: true });
});

for (const script of [true, false]) {
    test(`home anchors and empty navigation work with script ${script}`, async ({ browser }) => {
        const context = await browser.newContext({ javaScriptEnabled: script, reducedMotion: "reduce" });
        const page = await context.newPage();
        const errors: string[] = [];
        page.on("pageerror", error => errors.push(error.message));
        page.on("requestfailed", request => errors.push(`${request.url()}: ${request.failure()?.errorText}`));
        page.on("response", response => {
            if (response.status() >= 400 && !response.url().endsWith("missing/")) errors.push(`${response.url()}: ${response.status()}`);
        });
        for (const width of [1280, 768, 375, 320]) {
            await page.setViewportSize({ width, height: 720 });
            for (const [route, id] of [["", "home"], ["explicit/", "入口"], ["raw/", "</h1>"]]) {
                await page.goto(`${url}${route}#${encodeURIComponent(id)}`);
                const title = page.getByRole("heading", { level: 1 });
                await expect(title).toHaveCount(1);
                await expect(title).toHaveAttribute("id", id);
                if (route === "raw/") {
                    await expect(page.locator("article")).toHaveText("Same Automatic");
                    await expect(page.getByRole("navigation", { name: "Start reading" }).getByRole("link", { name: "Lire" })).toHaveAttribute("lang", "fr");
                }
                await expect(title).toBeInViewport();
                if (script && [1280, 375].includes(width)) {
                    await page.screenshot({ path: test.info().outputPath(`home-${route ? route.slice(0, -1) : "automatic"}-${width}.png`), fullPage: true });
                }
                const box = (await title.boundingBox())!;
                expect(box.y).toBeGreaterThanOrEqual((await page.getByRole("banner").boundingBox())!.height);
                await page.locator("article").getByRole("link", { name: "Same", exact: true }).click();
                await expect(title).toBeInViewport();
                const cross = page.locator("article").getByRole("link").last();
                await cross.click();
                await expect(page.getByRole("heading", { level: 1 })).toBeInViewport();
                expect(await page.locator("html").evaluate(node => node.scrollWidth)).toBeLessThanOrEqual(width);
            }
            for (const route of ["empty/", "guide/", "missing/"]) {
                const response = await page.goto(`${url}${route}`);
                expect(response?.status()).toBe(route === "missing/" ? 404 : 200);
                await expect(page.getByRole("banner")).toHaveCount(1);
                await expect(page.getByRole("main")).toHaveCount(1);
                await expect(page.getByRole("heading", { level: 1 })).toHaveCount(1);
                const footer = page.getByRole("contentinfo");
                await expect(footer).toHaveCount(1);
                if (script && route === "empty/" && [1280, 375].includes(width)) {
                    await page.screenshot({ path: test.info().outputPath(`home-empty-${width}.png`), fullPage: true });
                }
                await expect(footer.getByRole("navigation", { name: "Footer links" })).toHaveCount(1);
                const home = footer.getByRole("link", { name: "Back to the home page" });
                await expect(home).toHaveAttribute("href", base);
                await home.click();
                await expect(page).toHaveURL(url);
            }
        }
        await page.goto(`${url}explicit/`);
        await expect(page.getByRole("contentinfo").getByRole("navigation")).toHaveAttribute("lang", "en");
        await page.goto(url);
        await expect(page.getByRole("navigation", { name: "Start reading" })).toBeVisible();
        await page.getByRole("navigation", { name: "Start reading" }).getByRole("link", { name: "Read" }).click();
        await expect(page.locator("h2#raw")).toBeVisible();
        expect(errors).toEqual([]);
        await context.close();
    });
}

test("the empty home and its footer pass accessibility checks in both modes", async ({ browser }) => {
    for (const colorScheme of ["light", "dark"] as const) {
        const context = await browser.newContext({ colorScheme });
        const page = await context.newPage();
        await page.goto(`${url}empty/`);
        const result = await new AxeBuilder({ page }).withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"]).analyze();
        expect(result.violations).toEqual([]);
        await context.close();
    }
});
