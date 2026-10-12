# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: docs-ssg/theme.spec.ts >> reading quality >> one h1, landmarks and the page's language
- Location: tests/docs-ssg/theme.spec.ts:357:9

# Error details

```
Error: expect(locator).toHaveCount(expected) failed

Locator:  getByRole('contentinfo')
Expected: 1
Received: 0
Timeout:  10000ms

Call log:
  - Expect "toHaveCount" getByRole('contentinfo') with timeout 10000ms
  - waiting for getByRole('contentinfo')
    24 × locator resolved to 0 elements
       - unexpected value "0"

```

# Page snapshot

```yaml
- generic [active] [ref=e1]:
  - link "Skip to content" [ref=e2] [cursor=pointer]:
    - /url: "#content"
  - banner [ref=e3]:
    - generic [ref=e4]:
      - link "Fixture Docs" [ref=e5] [cursor=pointer]:
        - /url: /
      - navigation "Site links" [ref=e7]:
        - list [ref=e8]:
          - listitem [ref=e9]:
            - link "Guide" [ref=e10] [cursor=pointer]:
              - /url: /guide/
          - listitem [ref=e11]:
            - link "GitHub" [ref=e12] [cursor=pointer]:
              - /url: https://github.com/example/fixture
        - list [ref=e13]:
          - listitem [ref=e14]:
            - link "Mastodon" [ref=e15] [cursor=pointer]:
              - /url: https://mastodon.example/@fixture
      - group "Theme" [ref=e16]:
        - button "System" [pressed] [ref=e17] [cursor=pointer]
        - button "Light" [ref=e23] [cursor=pointer]
        - button "Dark" [ref=e29] [cursor=pointer]
  - main [ref=e34]:
    - generic [ref=e35]:
      - heading "Fixture Docs" [level=1] [ref=e36]
      - paragraph [ref=e37]: Markdown in, a static site out.
      - paragraph [ref=e38]:
        - link "Get started" [ref=e39] [cursor=pointer]:
          - /url: /guide/
        - link "Reference" [ref=e40] [cursor=pointer]:
          - /url: /reference/api/
    - list [ref=e41]:
      - listitem [ref=e42]:
        - heading "Static" [level=2] [ref=e43]
        - paragraph [ref=e44]: Every page is complete HTML.
      - listitem [ref=e45]:
        - heading "Checked" [level=2] [ref=e46]
        - paragraph [ref=e47]: Every link is resolved before a build is published.
    - article [ref=e48]:
      - paragraph [ref=e49]:
        - text: Start with the
        - link "guide" [ref=e50] [cursor=pointer]:
          - /url: /guide/
        - text: or read the
        - link "API reference" [ref=e51] [cursor=pointer]:
          - /url: /reference/api/
        - text: .
  - paragraph [ref=e52]
```

# Test source

```ts
  269 |                 await noHorizontalScroll(page);
  270 |                 const row = page.getByRole("navigation", { name: "Site links" });
  271 |                 await expect(row, `${width} ${path}`).toHaveCount(1);
  272 |                 await expect(row.getByRole("link", { name: "Guide" })).toBeVisible();
  273 |                 await row.getByRole("link", { name: "Mastodon" }).scrollIntoViewIfNeeded();
  274 |                 await expect(row.getByRole("link", { name: "Mastodon" })).toBeInViewport();
  275 |                 await noHorizontalScroll(page);
  276 |             }
  277 |         }
  278 |         expect(errors).toEqual([]);
  279 |     });
  280 | });
  281 | 
  282 | test.describe("unusual content", () => {
  283 |     test("a heading id with % neither breaks the contents nor the narrow menus", async ({ page }) => {
  284 |         const errors: string[] = [];
  285 |         page.on("pageerror", (error) => errors.push(error.message));
  286 |         // Short enough that the heading reaches the line before the page
  287 |         // ends, where the last section would be marked instead.
  288 |         await page.setViewportSize({ width: 1280, height: 320 });
  289 |         await page.goto(`${base()}guide/`);
  290 |         const toc = page.locator(".rd-toc-wide");
  291 |         await expect(toc.locator('a[href="#100%25"]')).toHaveText("All done");
  292 |         await page.evaluate(() => {
  293 |             const top = document.getElementById("100%")!.getBoundingClientRect().top;
  294 |             window.scrollBy(0, top - 80);
  295 |         });
  296 |         await expect(toc.locator('a[aria-current="location"]')).toHaveAttribute("href", "#100%25");
  297 |         await page.setViewportSize({ width: 375, height: 740 });
  298 |         const menu = page.locator("details.rd-sidebar-narrow");
  299 |         await menu.locator("summary").click();
  300 |         await expect(menu).toHaveAttribute("open", "");
  301 |         await page.keyboard.press("Escape");
  302 |         await expect(menu).not.toHaveAttribute("open", "");
  303 |         const contents = page.locator("details.rd-toc-narrow");
  304 |         await contents.locator("summary").click();
  305 |         await contents.getByRole("link", { name: "All done" }).click();
  306 |         await expect(page.locator('[id="100%"]')).toBeInViewport();
  307 |         expect(errors).toEqual([]);
  308 |     });
  309 | 
  310 |     test("an SVG symbol keeps its fragment, and a class's background loads from the site", async ({ page }) => {
  311 |         const failed: string[] = [];
  312 |         page.on("response", (response) => {
  313 |             if (response.status() >= 400) failed.push(`${response.status()} ${response.url()}`);
  314 |         });
  315 |         await page.goto(`${base()}guide/`);
  316 |         const mark = page.getByRole("img", { name: "The fixture's mark" });
  317 |         await expect(mark.locator("use")).toHaveAttribute("href", new RegExp(`^${base()}assets/files/sprite\\.[0-9a-f]{10}\\.svg#mark$`));
  318 |         await expect.poll(() => mark.locator("use").evaluate((use) => {
  319 |             const box = (use as SVGUseElement).getBBox();
  320 |             return `${box.width}x${box.height}`;
  321 |         })).toBe("40x40");
  322 |         const banner = page.getByRole("img", { name: "A banner" });
  323 |         const image = await banner.evaluate((element) => getComputedStyle(element).backgroundImage);
  324 |         expect(image).toMatch(new RegExp(`assets/files/banner\\.[0-9a-f]{10}\\.svg`));
  325 |         const url = image.slice(image.indexOf("(") + 1, image.lastIndexOf(")")).replace(/^"|"$/g, "");
  326 |         const loaded = await page.evaluate(async (url) => (await fetch(url)).status, url);
  327 |         expect(loaded).toBe(200);
  328 |         expect(failed).toEqual([]);
  329 |     });
  330 | });
  331 | 
  332 | test.describe("without script", () => {
  333 |     test.use({ javaScriptEnabled: false });
  334 | 
  335 |     test("everything reads, every code tab is shown and nothing pretends to be a control", async ({ page }) => {
  336 |         const wasm: string[] = [];
  337 |         page.on("request", (request) => {
  338 |             if (request.url().endsWith(".wasm")) wasm.push(request.url());
  339 |         });
  340 |         await page.goto(`${base()}guide/setup/`);
  341 |         await expect(page.locator("[data-rd-mode-switch]")).toBeHidden();
  342 |         await expect(page.locator("[data-rd-copy]").first()).toBeHidden();
  343 |         const group = page.locator(".rd-code-group").first();
  344 |         await expect(group.locator('[role="tablist"]')).toBeHidden();
  345 |         await expect(group.locator("figure.rd-code")).toHaveCount(2);
  346 |         for (const figure of await group.locator("figure.rd-code").all()) {
  347 |             await expect(figure).toBeVisible();
  348 |         }
  349 |         await expect(page.locator(".hl-source").first()).toBeVisible();
  350 |         await page.getByRole("link", { name: "Install the CLI" }).click();
  351 |         await expect(page).toHaveURL(/#install-the-cli$/);
  352 |         expect(wasm).toEqual([]);
  353 |     });
  354 | });
  355 | 
  356 | test.describe("reading quality", () => {
  357 |     test("one h1, landmarks and the page's language", async ({ page }) => {
  358 |         for (const [path, lang] of [
  359 |             ["", "en"],
  360 |             ["guide/setup/", "en"],
  361 |             ["reference/api/", "zh-CN"],
  362 |             ["no/such/page/", "en"],
  363 |         ] as const) {
  364 |             await page.goto(`${base()}${path}`);
  365 |             await expect(page.locator("html")).toHaveAttribute("lang", lang);
  366 |             await expect(page.locator("h1")).toHaveCount(1);
  367 |             await expect(page.getByRole("banner")).toHaveCount(1);
  368 |             await expect(page.getByRole("main")).toHaveCount(1);
> 369 |             await expect(page.getByRole("contentinfo")).toHaveCount(1);
      |                                                         ^ Error: expect(locator).toHaveCount(expected) failed
  370 |             expect(await page.getByRole("navigation").count()).toBeGreaterThan(0);
  371 |             await expect(page.getByRole("contentinfo").getByRole("link")).toHaveAttribute("href", base());
  372 |         }
  373 |     });
  374 | 
  375 |     test("text, links, alerts and actions reach AA contrast in both modes", async ({ browser }) => {
  376 |         for (const scheme of ["light", "dark"] as const) {
  377 |             const context = await browser.newContext({ colorScheme: scheme });
  378 |             const page = await context.newPage();
  379 |             await page.goto(`${base()}guide/`);
  380 |             const checks: [string, number][] = [
  381 |                 [".rd-prose p", 4.5],
  382 |                 [".rd-prose a", 4.5],
  383 |                 [".rd-sidebar-wide a:not([aria-current])", 4.5],
  384 |                 ['.rd-sidebar-wide a[aria-current="page"]', 4.5],
  385 |                 [".rd-toc-wide a", 4.5],
  386 |                 [".rd-pager-label", 4.5],
  387 |                 [".markdown-alert-tip .markdown-alert-title", 4.5],
  388 |                 [".markdown-alert-important .markdown-alert-title", 4.5],
  389 |                 [".markdown-alert-caution .markdown-alert-title", 4.5],
  390 |                 [".markdown-alert-tip p:not(.markdown-alert-title)", 4.5],
  391 |                 [".rd-code-tabs [role=tab]:not([aria-selected=true])", 4.5],
  392 |             ];
  393 |             for (const [selector, minimum] of checks) {
  394 |                 expect(await contrast(page, selector), `${scheme}: ${selector}`).toBeGreaterThanOrEqual(minimum);
  395 |             }
  396 |             await page.goto(base());
  397 |             expect(await contrast(page, ".rd-action-primary"), `${scheme}: primary action`).toBeGreaterThanOrEqual(4.5);
  398 |             expect(await contrast(page, ".rd-tagline"), `${scheme}: tagline`).toBeGreaterThanOrEqual(4.5);
  399 |             expect(await contrast(page, ".rd-site-footer a"), `${scheme}: footer link`).toBeGreaterThanOrEqual(4.5);
  400 |             expect(await contrast(page, ".rd-features p"), `${scheme}: feature text`).toBeGreaterThanOrEqual(4.5);
  401 |             await page.locator(".rd-action-primary").hover();
  402 |             expect(await contrast(page, ".rd-action-primary"), `${scheme}: primary action, hovered`).toBeGreaterThanOrEqual(4.5);
  403 |             await context.close();
  404 |         }
  405 |     });
  406 | 
  407 |     test("focus is an opaque ring with a gap, and the logo shows in both modes", async ({ browser, browserName }) => {
  408 |         // Safari moves focus to links with Option-Tab; Tab alone skips them.
  409 |         const tab = browserName === "webkit" ? "Alt+Tab" : "Tab";
  410 |         for (const scheme of ["light", "dark"] as const) {
  411 |             const context = await browser.newContext({ colorScheme: scheme });
  412 |             const page = await context.newPage();
  413 |             await page.goto(`${base()}guide/`);
  414 |             await page.keyboard.press(tab);
  415 |             await page.keyboard.press(tab);
  416 |             expect(await page.evaluate(() => document.activeElement?.className)).toBe("rd-brand");
  417 |             const ring = await page.evaluate(() => {
  418 |                 const style = getComputedStyle(document.activeElement!);
  419 |                 return { width: style.outlineWidth, style: style.outlineStyle, offset: style.outlineOffset, color: style.outlineColor };
  420 |             });
  421 |             expect(ring).toMatchObject({ width: "3px", style: "solid", offset: "2px" });
  422 |             expect(ring.color).toBe(scheme === "light" ? "rgb(23, 92, 211)" : "rgb(83, 177, 253)");
  423 |             const logo = page.locator(".rd-brand img");
  424 |             await expect(logo).toBeVisible();
  425 |             expect(await logo.evaluate((img) => (img as HTMLImageElement).naturalWidth)).toBeGreaterThan(0);
  426 |             await context.close();
  427 |         }
  428 |     });
  429 | 
  430 |     test("axe finds no serious or critical WCAG A/AA violation, wide and narrow, in both modes", async ({ browser }) => {
  431 |         const found: string[] = [];
  432 |         for (const scheme of ["light", "dark"] as const) {
  433 |             for (const width of [1280, 375]) {
  434 |                 const context = await browser.newContext({ colorScheme: scheme, viewport: { width, height: 800 } });
  435 |                 const page = await context.newPage();
  436 |                 for (const path of ["", "guide/", "guide/setup/", "reference/api/", "no/such/page/"]) {
  437 |                     await page.goto(`${base()}${path}`);
  438 |                     const { violations } = await new AxeBuilder({ page })
  439 |                         .withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"])
  440 |                         .analyze();
  441 |                     for (const violation of violations) {
  442 |                         const line = `${scheme} ${width}px /${path} ${violation.impact} ${violation.id}: ${violation.nodes.map((node) => node.target.join(" ")).join(", ")}`;
  443 |                         found.push(line);
  444 |                     }
  445 |                 }
  446 |                 await context.close();
  447 |             }
  448 |         }
  449 |         await test.info().attach("axe-violations.txt", { body: found.join("\n") || "none", contentType: "text/plain" });
  450 |         expect(found.filter((line) => / (serious|critical) /.test(line))).toEqual([]);
  451 |     });
  452 | 
  453 |     test("reduced motion turns smooth scrolling off", async ({ browser }) => {
  454 |         const context = await browser.newContext({ reducedMotion: "reduce" });
  455 |         const page = await context.newPage();
  456 |         await page.goto(`${base()}guide/`);
  457 |         expect(await page.evaluate(() => getComputedStyle(document.documentElement).scrollBehavior)).toBe("auto");
  458 |         await context.close();
  459 |     });
  460 | });
  461 | 
```