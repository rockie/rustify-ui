# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: docs-ssg/dev.spec.ts >> axe finds no serious or critical problem with the panel open or hidden, in both modes
- Location: tests/docs-ssg/dev.spec.ts:600:5

# Error details

```
Error: expect(locator).toBeVisible() failed

Locator: getByRole('dialog')
Expected: visible
Timeout: 10000ms
Error: element(s) not found

Call log:
  - Expect "toBeVisible" getByRole('dialog') with timeout 10000ms
  - waiting for getByRole('dialog')

```

```yaml
- link "Skip to content":
  - /url: "#content"
- banner:
  - link "Fixture Docs":
    - /url: /docs/
  - group "Theme":
    - button "System" [pressed]
    - button "Light"
    - button "Dark"
- navigation "Menu"
- main:
  - heading "Building your site" [level=1]
  - article:
    - paragraph: "The first dependency build takes longer. Keep this page open: it updates when the site is ready."
  - region "1 error · nothing has built yet":
    - paragraph: 1 error · nothing has built yet
    - text: revision 1
    - region "Problems":
      - heading "content/guide/setup.md" [level=2]
      - list:
        - listitem:
          - text: "31:1 Error: broken-link"
          - paragraph:
            - text: there is no page
            - code: content/guide/missing.md
- contentinfo:
  - navigation "Footer links":
    - link "Back to the home page":
      - /url: /docs/
- paragraph
- status: 1 error · nothing has built yet · revision 1
```

# Test source

```ts
  509 |     await expect(marker).toBeFocused();
  510 | 
  511 |     site.edit("content/guide/setup.md", `${BROKEN} `, "");
  512 |     await expect(marker).toBeHidden();
  513 |     await expect(panel).toBeHidden();
  514 |     // Focus was on the marker: it continues from the page's content.
  515 |     await expect(page.locator("main")).toBeFocused();
  516 |     await expect(page.getByRole("status").filter({ hasText: "Problems fixed" })).toHaveCount(1);
  517 |     expect(await kept(page), "the fixed page was the last good one").toBe(true);
  518 |     expect(before).toBeGreaterThan(0);
  519 | });
  520 | 
  521 | test("while nothing has built, every page says so, and becomes the site once it builds", async ({ page }) => {
  522 |     site.edit("content/guide/setup.md", "Run `rustify-docs --version`", `${BROKEN} Run \`rustify-docs --version\``);
  523 |     const server = await dev();
  524 |     const response = await page.goto(`${server.url}guide/setup/`);
  525 |     expect(response?.status()).toBe(500);
  526 |     await expect(page.getByRole("heading", { level: 1 })).toHaveText("The site does not build yet");
  527 |     // The problems are the page: listed in its column, with nothing to hide them.
  528 |     const list = page.getByRole("region", { name: "1 error · nothing has built yet" });
  529 |     await expect(list).toBeVisible();
  530 |     await expect(page.locator("main .rd-dev")).toBeVisible();
  531 |     await expect(list.getByRole("button")).toHaveCount(0);
  532 |     await expect(list.locator(".rd-dev-code")).toHaveText("content/guide/missing.md");
  533 |     await list.locator(".rd-dev-list").focus();
  534 |     await page.keyboard.press("Escape");
  535 |     await expect(list).toBeVisible();
  536 |     site.edit("content/guide/setup.md", `${BROKEN} `, "");
  537 |     await expect(page.getByRole("heading", { level: 1 })).toHaveText("Setup");
  538 |     await expect(page.locator(".rd-dev")).toBeHidden();
  539 | });
  540 | 
  541 | test("a restarted dev brings an open page up to date", async ({ page }) => {
  542 |     const port = String(await freePort());
  543 |     const first = await dev(["--port", port]);
  544 |     await page.goto(`${first.url}guide/setup/`);
  545 |     await expect(page.locator(".rd-dev")).toHaveAttribute("data-revision", /\d+/);
  546 |     await first.stop();
  547 |     site.edit("content/guide/setup.md", "Once installed, the same choice applies here:", "After a restart, the same choice applies here:");
  548 |     await dev(["--port", port]);
  549 |     await expect(page.getByText("After a restart")).toBeVisible({ timeout: 15_000 });
  550 | });
  551 | 
  552 | test("a panic in a build is one more problem, and the next build is served", async ({ page }) => {
  553 |     const server = await dev(["--port", "0"], { RUSTIFY_DOCS_FAULT: "panic:PANIC-HERE" });
  554 |     await page.goto(`${server.url}guide/setup/`);
  555 |     await mark(page);
  556 |     site.edit("content/guide/setup.md", "Once installed,", "PANIC-HERE Once installed,");
  557 |     const panel = page.getByRole("dialog", { name: "1 error · showing the last good version" });
  558 |     await expect(panel).toContainText("internal");
  559 |     await expect(panel).toContainText("stopped inside rustify-docs");
  560 |     await expect(panel.locator(".rd-dev-suggestion")).toContainText("save again");
  561 |     expect(await kept(page)).toBe(true);
  562 |     site.edit("content/guide/setup.md", "PANIC-HERE Once installed,", "Recovered. Once installed,");
  563 |     await expect(page.getByText("Recovered. Once installed")).toBeVisible();
  564 |     await expect(page.locator(".rd-dev")).toBeHidden();
  565 |     expect(server.log).toContain("stopped inside rustify-docs");
  566 | });
  567 | 
  568 | test("the panel speaks the page's language", async ({ page }) => {
  569 |     const server = await dev();
  570 |     await page.goto(`${server.url}reference/api/`);
  571 |     site.edit("content/reference/api.md", "返回[指南](../guide/index.md)。", `返回[指南](../guide/index.md)。${BROKEN}`);
  572 |     const panel = page.getByRole("dialog", { name: "1 个错误 · 显示上一个可用版本" });
  573 |     await expect(panel).toBeVisible();
  574 |     await expect(page.locator(".rd-dev")).toHaveAttribute("lang", "zh-CN");
  575 |     await expect(panel.getByRole("button", { name: "收起问题" })).toBeVisible();
  576 | });
  577 | 
  578 | test("on a phone the panel is a sheet along the bottom and nothing scrolls sideways", async ({ browser }) => {
  579 |     const server = await dev();
  580 |     for (const width of [375, 320]) {
  581 |         const context = await browser.newContext({ viewport: { width, height: 640 }, reducedMotion: "reduce" });
  582 |         const page = await context.newPage();
  583 |         await page.goto(`${server.url}guide/setup/`);
  584 |         if (width === 375) {
  585 |             site.edit("content/guide/setup.md", "Run `rustify-docs --version`", `${BROKEN} Run \`rustify-docs --version\``);
  586 |         }
  587 |         const panel = page.getByRole("dialog");
  588 |         await expect(panel).toBeVisible();
  589 |         const box = (await panel.boundingBox())!;
  590 |         expect(box.x).toBe(0);
  591 |         expect(box.width).toBe(width);
  592 |         expect(box.y + box.height).toBeCloseTo(640, 0);
  593 |         expect(box.height).toBeLessThanOrEqual(320);
  594 |         expect(await page.evaluate(() => getComputedStyle(document.querySelector(".rd-dev-panel")!).animationName)).toBe("none");
  595 |         expect(await page.evaluate(() => document.documentElement.scrollWidth - window.innerWidth)).toBeLessThanOrEqual(0);
  596 |         await context.close();
  597 |     }
  598 | });
  599 | 
  600 | test("axe finds no serious or critical problem with the panel open or hidden, in both modes", async ({ browser }) => {
  601 |     const server = await dev();
  602 |     site.edit("content/guide/setup.md", "Run `rustify-docs --version`", `${BROKEN} Run \`rustify-docs --version\``);
  603 |     const found: string[] = [];
  604 |     for (const scheme of ["light", "dark"] as const) {
  605 |         const context = await browser.newContext({ colorScheme: scheme });
  606 |         const page = await context.newPage();
  607 |         await page.goto(`${server.url}guide/setup/`);
  608 |         const panel = page.getByRole("dialog");
> 609 |         await expect(panel).toBeVisible();
      |                             ^ Error: expect(locator).toBeVisible() failed
  610 |         for (const state of ["open", "hidden"]) {
  611 |             if (state === "hidden") await panel.getByRole("button").click();
  612 |             // Colours are measured once the panel's entrance has finished.
  613 |             await page.waitForFunction(() => document.getAnimations().every((animation) => animation.playState === "finished"));
  614 |             const { violations } = await new AxeBuilder({ page })
  615 |                 .include(".rd-dev")
  616 |                 .withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"])
  617 |                 .analyze();
  618 |             for (const violation of violations) {
  619 |                 for (const node of violation.nodes) {
  620 |                     const detail = (node.any[0]?.message || node.failureSummary || "").replace(/\s+/g, " ");
  621 |                     found.push(`${scheme} ${state} ${violation.impact} ${violation.id}: ${node.target.join(" ")}: ${detail}`);
  622 |                 }
  623 |             }
  624 |         }
  625 |         await context.close();
  626 |     }
  627 |     await test.info().attach("axe-violations.txt", { body: found.join("\n") || "none", contentType: "text/plain" });
  628 |     expect(found.filter((line) => / (serious|critical) /.test(line))).toEqual([]);
  629 | });
  630 | 
```