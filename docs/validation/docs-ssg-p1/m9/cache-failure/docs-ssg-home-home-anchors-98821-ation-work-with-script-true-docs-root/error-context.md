# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: docs-ssg/home.spec.ts >> home anchors and empty navigation work with script true
- Location: tests/docs-ssg/home.spec.ts:47:9

# Error details

```
Error: expect(locator).toHaveAttribute(expected) failed

Locator:  getByRole('heading', { level: 1 })
Expected: "home"
Received: ""
Timeout:  10000ms

Call log:
  - Expect "toHaveAttribute" getByRole('heading', { level: 1 }) with timeout 10000ms
  - waiting for getByRole('heading', { level: 1 })
    24 × locator resolved to <h1 class="rd-home-title">Home</h1>
       - unexpected value "null"

```

```yaml
- heading "Home" [level=1]
```

# Test source

```ts
  1   | import AxeBuilder from "@axe-core/playwright";
  2   | import { expect, test } from "@playwright/test";
  3   | import { ChildProcess, execFileSync, spawn } from "node:child_process";
  4   | import { cpSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
  5   | import { tmpdir } from "node:os";
  6   | import path from "node:path";
  7   | 
  8   | // A production site with optional navigation and metadata absent, served
  9   | // by the same candidate as the main fixture at the project's base.
  10  | let root: string;
  11  | let child: ChildProcess;
  12  | let url: string;
  13  | let base: string;
  14  | 
  15  | test.beforeAll(async () => {
  16  |     base = test.info().project.name === "docs-root" ? "/" : "/rustify-ui/";
  17  |     root = mkdtempSync(path.join(tmpdir(), "rustify-docs-home-"));
  18  |     cpSync("tests/docs-ssg/fixtures/home", root, { recursive: true });
  19  |     const config = path.join(root, "rustify-docs.toml");
  20  |     writeFileSync(config, readFileSync(config, "utf8").replaceAll("/docs/", base));
  21  |     const bin = path.resolve(process.env.RUSTIFY_DOCS_BIN || "target/docs-cli/rustify-docs");
  22  |     execFileSync(bin, ["build", "--config", path.join(root, "rustify-docs.toml")]);
  23  |     child = spawn(bin, ["preview", "--dir", path.join(root, "dist"), "--port", "0"]);
  24  |     url = await new Promise<string>((resolve, reject) => {
  25  |         let output = "";
  26  |         let error = "";
  27  |         child.stderr!.on("data", chunk => error += chunk);
  28  |         child.stdout!.on("data", chunk => {
  29  |             output += chunk;
  30  |             const match = /^preview: (\S+)/m.exec(output);
  31  |             if (match) resolve(match[1]);
  32  |         });
  33  |         child.on("exit", code => reject(new Error(`preview exited ${code}: ${output}${error}`)));
  34  |     });
  35  | });
  36  | 
  37  | test.afterAll(async () => {
  38  |     if (child && child.exitCode === null && child.signalCode === null) {
  39  |         const exited = new Promise(resolve => child.once("exit", resolve));
  40  |         child.kill();
  41  |         await exited;
  42  |     }
  43  |     if (root) rmSync(root, { recursive: true, force: true });
  44  | });
  45  | 
  46  | for (const script of [true, false]) {
  47  |     test(`home anchors and empty navigation work with script ${script}`, async ({ browser }) => {
  48  |         const context = await browser.newContext({ javaScriptEnabled: script, reducedMotion: "reduce" });
  49  |         const page = await context.newPage();
  50  |         const errors: string[] = [];
  51  |         page.on("pageerror", error => errors.push(error.message));
  52  |         page.on("requestfailed", request => errors.push(`${request.url()}: ${request.failure()?.errorText}`));
  53  |         page.on("response", response => {
  54  |             if (response.status() >= 400 && !response.url().endsWith("missing/")) errors.push(`${response.url()}: ${response.status()}`);
  55  |         });
  56  |         for (const width of [1280, 768, 375, 320]) {
  57  |             await page.setViewportSize({ width, height: 720 });
  58  |             for (const [route, id] of [["", "home"], ["explicit/", "入口"], ["raw/", "</h1>"]]) {
  59  |                 await page.goto(`${url}${route}#${encodeURIComponent(id)}`);
  60  |                 const title = page.getByRole("heading", { level: 1 });
  61  |                 await expect(title).toHaveCount(1);
> 62  |                 await expect(title).toHaveAttribute("id", id);
      |                                     ^ Error: expect(locator).toHaveAttribute(expected) failed
  63  |                 if (route === "raw/") {
  64  |                     await expect(page.locator("article")).toHaveText("Same Automatic");
  65  |                     await expect(page.getByRole("navigation", { name: "Start reading" }).getByRole("link", { name: "Lire" })).toHaveAttribute("lang", "fr");
  66  |                 }
  67  |                 await expect(title).toBeInViewport();
  68  |                 if (script && [1280, 375].includes(width)) {
  69  |                     await page.screenshot({ path: test.info().outputPath(`home-${route ? route.slice(0, -1) : "automatic"}-${width}.png`), fullPage: true });
  70  |                 }
  71  |                 const box = (await title.boundingBox())!;
  72  |                 expect(box.y).toBeGreaterThanOrEqual((await page.getByRole("banner").boundingBox())!.height);
  73  |                 await page.locator("article").getByRole("link", { name: "Same", exact: true }).click();
  74  |                 await expect(title).toBeInViewport();
  75  |                 const cross = page.locator("article").getByRole("link").last();
  76  |                 await cross.click();
  77  |                 await expect(page.getByRole("heading", { level: 1 })).toBeInViewport();
  78  |                 expect(await page.locator("html").evaluate(node => node.scrollWidth)).toBeLessThanOrEqual(width);
  79  |             }
  80  |             for (const route of ["empty/", "guide/", "missing/"]) {
  81  |                 const response = await page.goto(`${url}${route}`);
  82  |                 expect(response?.status()).toBe(route === "missing/" ? 404 : 200);
  83  |                 await expect(page.getByRole("banner")).toHaveCount(1);
  84  |                 await expect(page.getByRole("main")).toHaveCount(1);
  85  |                 await expect(page.getByRole("heading", { level: 1 })).toHaveCount(1);
  86  |                 const footer = page.getByRole("contentinfo");
  87  |                 await expect(footer).toHaveCount(1);
  88  |                 if (script && route === "empty/" && [1280, 375].includes(width)) {
  89  |                     await page.screenshot({ path: test.info().outputPath(`home-empty-${width}.png`), fullPage: true });
  90  |                 }
  91  |                 await expect(footer.getByRole("navigation", { name: "Footer links" })).toHaveCount(1);
  92  |                 const home = footer.getByRole("link", { name: "Back to the home page" });
  93  |                 await expect(home).toHaveAttribute("href", base);
  94  |                 await home.click();
  95  |                 await expect(page).toHaveURL(url);
  96  |             }
  97  |         }
  98  |         await page.goto(`${url}explicit/`);
  99  |         await expect(page.getByRole("contentinfo").getByRole("navigation")).toHaveAttribute("lang", "en");
  100 |         await page.goto(url);
  101 |         await expect(page.getByRole("navigation", { name: "Start reading" })).toBeVisible();
  102 |         await page.getByRole("navigation", { name: "Start reading" }).getByRole("link", { name: "Read" }).click();
  103 |         await expect(page.locator("h2#raw")).toBeVisible();
  104 |         expect(errors).toEqual([]);
  105 |         await context.close();
  106 |     });
  107 | }
  108 | 
  109 | test("the empty home and its footer pass accessibility checks in both modes", async ({ browser }) => {
  110 |     for (const colorScheme of ["light", "dark"] as const) {
  111 |         const context = await browser.newContext({ colorScheme });
  112 |         const page = await context.newPage();
  113 |         await page.goto(`${url}empty/`);
  114 |         const result = await new AxeBuilder({ page }).withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"]).analyze();
  115 |         expect(result.violations).toEqual([]);
  116 |         await context.close();
  117 |     }
  118 | });
  119 | 
```