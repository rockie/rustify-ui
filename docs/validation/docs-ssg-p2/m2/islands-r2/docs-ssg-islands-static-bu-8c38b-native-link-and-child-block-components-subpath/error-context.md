# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: docs-ssg/islands.spec.ts >> static builtins need no script or wasm and retain every panel, native link and child block
- Location: tests/docs-ssg/islands.spec.ts:377:1

# Error details

```
Error: expect(locator).toHaveCount(expected) failed

Locator:  locator('[data-rustify-scope], fieldset[data-rd-island-controls]')
Expected: 0
Received: 1
Timeout:  10000ms

Call log:
  - Expect "toHaveCount" locator('[data-rustify-scope], fieldset[data-rd-island-controls]') with timeout 10000ms
  - waiting for locator('[data-rustify-scope], fieldset[data-rd-island-controls]')
    24 × locator resolved to 1 element
       - unexpected value "1"

```

# Test source

```ts
  285 |     }
  286 | });
  287 | 
  288 | test("loading gates native and captured input while source controls remain usable", async ({ page, context, audit }) => {
  289 |     await context.grantPermissions(["clipboard-read", "clipboard-write"]);
  290 |     const held = await openPaused(page, "counter/");
  291 |     try {
  292 |         const island = islands(page).first();
  293 |         await expect(canvas(island)).toHaveJSProperty("disabled", false);
  294 |         await expect(canvas(island)).toHaveJSProperty("inert", true);
  295 |         await expect(canvas(island)).toHaveAttribute("aria-busy", "true");
  296 |         await expect(island.locator("[data-rd-island-loading]")).toBeVisible();
  297 |         const result = await island.getByTestId("counter-increment").evaluate(button => {
  298 |             let delivered = 0;
  299 |             const names = ["click", "pointerdown", "keydown", "beforeinput", "input", "change", "submit"];
  300 |             for (const name of names) button.addEventListener(name, () => delivered++);
  301 |             const accepted = names.map(name => button.dispatchEvent(new Event(name, { bubbles: true, cancelable: true })));
  302 |             (button as HTMLElement).focus();
  303 |             return { accepted, delivered, focused: document.activeElement === button };
  304 |         });
  305 |         expect(result).toEqual({ accepted: Array(7).fill(false), delivered: 0, focused: false });
  306 |         const box = (await island.getByTestId("counter-increment").boundingBox())!;
  307 |         await page.mouse.click(box.x + box.width / 2, box.y + box.height / 2);
  308 |         await expect(island.getByTestId("counter-value")).toHaveText("2");
  309 |         await sourceCopy(page);
  310 |         held.release();
  311 |         await ready(page, 4);
  312 |         await identity(page);
  313 |         await healthy(page, audit);
  314 |     } finally {
  315 |         held.release();
  316 |     }
  317 | });
  318 | 
  319 | test("attachment retains original SSR nodes and isolates three identical counters", async ({ page, audit }) => {
  320 |     const held = await openPaused(page, "counter/");
  321 |     held.release();
  322 |     await ready(page, 4);
  323 |     await identity(page);
  324 |     const counters = islands(page).filter({ has: page.locator('[data-counter]') });
  325 |     await expect(counters).toHaveCount(4);
  326 |     await counters.nth(0).getByTestId("counter-increment").click();
  327 |     await expect(counters.nth(0).getByTestId("counter-value")).toHaveText("3");
  328 |     await counters.nth(1).getByTestId("counter-input").fill("19");
  329 |     await expect(counters.nth(1).getByTestId("counter-value")).toHaveText("19");
  330 |     await counters.nth(2).getByTestId("counter-increment").focus();
  331 |     await page.keyboard.press("Enter");
  332 |     await expect(counters.nth(2).getByTestId("counter-value")).toHaveText("3");
  333 |     await expect(counters.nth(0).getByTestId("counter-value")).toHaveText("3");
  334 |     await expect(counters.nth(1).getByTestId("counter-value")).toHaveText("19");
  335 |     await expect(counters.nth(3).getByTestId("counter-value")).toHaveText("7");
  336 |     expect((await page.getByTestId("counter-input").all()).length).toBe(4);
  337 |     await healthy(page, audit);
  338 | });
  339 | 
  340 | test("an asynchronous action starts after attachment with supported hydration globals", async ({ page, audit }) => {
  341 |     const held = await openPaused(page, "async/");
  342 |     await expect(page.getByTestId("async-result")).toHaveText("Not loaded");
  343 |     held.release();
  344 |     await ready(page, 1);
  345 |     await identity(page);
  346 |     expect(await page.evaluate(() => ["__RESOLVED_RESOURCES", "__INCOMPLETE_CHUNKS", "__SERIALIZED_ERRORS"].map(name => Array.isArray((window as unknown as Record<string, unknown>)[name])))).toEqual([true, true, true]);
  347 |     await page.getByTestId("async-start").click();
  348 |     await expect(page.getByTestId("async-result")).toHaveText("Loaded after attachment");
  349 |     await healthy(page, audit);
  350 | });
  351 | 
  352 | test("client-only attachment preserves the placeholder and mounts typed props and child content", async ({ page, audit }) => {
  353 |     const held = await openPaused(page, "client-only/");
  354 |     const placeholder = page.locator("[data-rustify-client-only]");
  355 |     const original = await placeholder.elementHandle();
  356 |     const preserved = await page.evaluateHandle(() => {
  357 |         const snapshot = window.__docs_island_probe!.blocks[0];
  358 |         const message = snapshot.root.querySelector("[data-rustify-client-message]")!;
  359 |         return snapshot.nodes.filter(node => !message.contains(node));
  360 |     });
  361 |     const height = (await placeholder.boundingBox())!.height;
  362 |     await expect(page.locator("[data-rustify-client-message]")).toBeVisible();
  363 |     held.release();
  364 |     await ready(page, 1);
  365 |     expect(await original!.evaluate(node => node.isConnected)).toBe(true);
  366 |     expect(await preserved.evaluate(nodes => nodes.every(node => node.isConnected))).toBe(true);
  367 |     expect((await placeholder.boundingBox())!.height).toBeGreaterThanOrEqual(height);
  368 |     await expect(page.locator("[data-rustify-client-message]")).toHaveCount(0);
  369 |     await expect(page.locator("[data-rustify-client-host][data-rustify-mounted]")).toHaveCount(1);
  370 |     await expect(page.getByTestId("counter-value")).toHaveText("2");
  371 |     await expect(page.locator("[data-rustify-children] strong")).toHaveText("browser-created child");
  372 |     await page.getByTestId("counter-increment").click();
  373 |     await expect(page.getByTestId("counter-value")).toHaveText("3");
  374 |     await healthy(page, audit);
  375 | });
  376 | 
  377 | test("static builtins need no script or wasm and retain every panel, native link and child block", async ({ browser }) => {
  378 |     const context = await browser.newContext({ javaScriptEnabled: false });
  379 |     const page = await context.newPage();
  380 |     const audit = await watch(page);
  381 |     try {
  382 |         await page.goto(url("builtins/"));
  383 |         await expect(page.getByRole("heading", { level: 1 })).toHaveText("Static built-in components");
  384 |         await expect(islands(page)).toHaveCount(0);
> 385 |         await expect(page.locator("[data-rustify-scope], fieldset[data-rd-island-controls]")).toHaveCount(0);
      |                                                                                               ^ Error: expect(locator).toHaveCount(expected) failed
  386 |         const groups = page.locator("[data-rd-static-tabs]");
  387 |         await expect(groups).toHaveCount(2);
  388 |         await expect(page.locator("[data-rd-static-panel]")).toHaveCount(4);
  389 |         for (const group of await groups.all()) {
  390 |             await expect(group.locator(":scope > [data-rd-static-tabbar]")).toBeHidden();
  391 |             await expect(group.locator(":scope > span[data-rustify-children]")).toHaveCount(1);
  392 |             for (const panel of await group.locator(":scope > [data-rd-static-panel]").all()) await expect(panel).toBeVisible();
  393 |         }
  394 |         await expect(groups.first().locator('[data-rd-static-panel="1"]')).toHaveAttribute("aria-label", "First group code");
  395 |         await expect(groups.first().locator('[data-rd-static-panel="1"] .rd-static-text')).toHaveText("A literal <tag> & raw value stays text.");
  396 |         await expect(groups.first().locator("tag")).toHaveCount(0);
  397 |         const children = groups.first().locator(":scope > span[data-rustify-children]");
  398 |         await expect(children.locator("strong")).toHaveText("checked Markdown");
  399 |         await expect(children.getByRole("link", { name: "registered counter", exact: true })).toHaveAttribute("href", new URL(url("counter/")).pathname);
  400 |         await expect(children.locator(".rd-code-pre code")).toHaveText('let text = "<tag> & raw";');
  401 |         await expect(page.locator(".rd-static-card-grid")).toHaveAttribute("data-columns", "2");
  402 |         await expect(page.locator(".rd-static-card-grid > ul > li")).toHaveCount(2);
  403 |         await expect(page.locator(".rd-static-card-grid").getByRole("link", { name: "Registered counters", exact: true })).toHaveAttribute("href", new URL(url("counter/")).pathname);
  404 |         await expect(page.locator(".rd-static-card-grid").getByRole("link", { name: "All categories", exact: true })).toHaveAttribute("href", new URL(url("proxy/")).pathname);
  405 |         await expect(page.locator(".rd-static-steps > ol > li .rd-static-step-title")).toHaveText(["Read the static example", "Open a counter"]);
  406 |         await expect(page.locator("[data-rustify-children]")).toHaveCount(4);
  407 |         for (const width of [1280, 375]) {
  408 |             await page.setViewportSize({ width, height: 800 });
  409 |             expect(await page.evaluate(() => document.documentElement.scrollWidth - innerWidth)).toBeLessThanOrEqual(0);
  410 |             await page.screenshot({ path: test.info().outputPath(`builtins-no-script-${width}.png`), fullPage: true });
  411 |         }
  412 |         expect(audit.requests.filter(request => /\/assets\/islands\//.test(new URL(request).pathname) || new URL(request).pathname.endsWith(".wasm"))).toEqual([]);
  413 |         expect(audit.failures).toEqual([]);
  414 |         expect(audit.responses).toEqual([]);
  415 |         expect(audit.console).toEqual([]);
  416 |         expect(audit.pageErrors).toEqual([]);
  417 |         await test.info().attach("builtin-no-script-browser-audit.json", { body: JSON.stringify(audit, null, 2), contentType: "application/json" });
  418 |     } finally {
  419 |         await context.close();
  420 |     }
  421 | });
  422 | 
  423 | test("static tabs use unique ARIA mappings and keyboard state stays local to each peer", async ({ page, context, audit }) => {
  424 |     await context.grantPermissions(["clipboard-read", "clipboard-write"]);
  425 |     await page.goto(url("builtins/"));
  426 |     const groups = page.locator("[data-rd-static-tabs]");
  427 |     await expect(page.locator("[data-rd-static-tabs][data-rd-static-enhanced]")).toHaveCount(2);
  428 |     const panels = await page.locator("[data-rd-static-panel]").elementHandles();
  429 |     for (const group of await groups.all()) {
  430 |         await expect(group.locator(":scope > [data-rd-static-tabbar]")).toHaveAttribute("role", "tablist");
  431 |         const tabs = await group.locator("[data-rd-static-tab]").all();
  432 |         for (const [index, tab] of tabs.entries()) {
  433 |             const panel = group.locator(`[data-rd-static-panel="${index}"]`);
  434 |             await expect(tab).toHaveAttribute("role", "tab");
  435 |             await expect(tab).toHaveAttribute("aria-controls", (await panel.getAttribute("id"))!);
  436 |             await expect(panel).toHaveAttribute("role", "tabpanel");
  437 |             await expect(panel).toHaveAttribute("aria-labelledby", (await tab.getAttribute("id"))!);
  438 |             await expect(tab).toHaveJSProperty("tabIndex", index === 0 ? 0 : -1);
  439 |             await expect(panel).toHaveJSProperty("hidden", index !== 0);
  440 |         }
  441 |     }
  442 |     expect(await page.evaluate(() => {
  443 |         const ids = [...document.querySelectorAll("[id]")].map(node => node.id);
  444 |         return ids.length === new Set(ids).size;
  445 |     })).toBe(true);
  446 |     const first = groups.nth(0).locator("[data-rd-static-tab]");
  447 |     const second = groups.nth(1).locator("[data-rd-static-tab]");
  448 |     await first.nth(0).focus();
  449 |     await page.keyboard.press("ArrowRight");
  450 |     await expect(first.nth(1)).toBeFocused();
  451 |     await expect(first.nth(1)).toHaveAttribute("aria-selected", "true");
  452 |     await expect(groups.nth(0).locator('[data-rd-static-panel="1"]')).toBeVisible();
  453 |     await expect(second.nth(0)).toHaveAttribute("aria-selected", "true");
  454 |     await second.nth(0).focus();
  455 |     await page.keyboard.press("ArrowLeft");
  456 |     await expect(second.nth(1)).toBeFocused();
  457 |     await expect(second.nth(1)).toHaveAttribute("aria-selected", "true");
  458 |     await expect(first.nth(1)).toHaveAttribute("aria-selected", "true");
  459 |     await first.nth(1).press("Home");
  460 |     await expect(first.nth(0)).toBeFocused();
  461 |     await first.nth(0).press("End");
  462 |     await expect(first.nth(1)).toBeFocused();
  463 |     await first.nth(1).press("ArrowRight");
  464 |     await expect(first.nth(0)).toBeFocused();
  465 |     await expect(first.nth(0)).toHaveAttribute("aria-selected", "true");
  466 |     await expect(second.nth(1)).toHaveAttribute("aria-selected", "true");
  467 |     await groups.first().locator(".rd-code [data-rd-copy]").click();
  468 |     expect(await page.evaluate(() => navigator.clipboard.readText())).toBe('let text = "<tag> & raw";\n');
  469 |     await expect(first.nth(0)).toHaveAttribute("aria-selected", "true");
  470 |     await expect(second.nth(1)).toHaveAttribute("aria-selected", "true");
  471 |     for (const panel of panels) expect(await panel.evaluate(node => node.isConnected && document.getElementById(node.id) === node)).toBe(true);
  472 |     expect(audit.requests.filter(request => /\/assets\/islands\//.test(new URL(request).pathname) || new URL(request).pathname.endsWith(".wasm"))).toEqual([]);
  473 |     await page.locator(".rd-static-card-grid").getByRole("link", { name: "Registered counters", exact: true }).click();
  474 |     await expect(page).toHaveURL(url("counter/"));
  475 |     await ready(page, 4);
  476 |     await healthy(page, audit);
  477 | });
  478 | 
  479 | test("a real callback trap restores every initial island and keeps source, navigation and reload alive", async ({ page, context, audit }) => {
  480 |     await context.grantPermissions(["clipboard-read", "clipboard-write"]);
  481 |     const held = await openPaused(page, "failure/");
  482 |     held.release();
  483 |     await ready(page, 2);
  484 |     await identity(page);
  485 |     await page.getByTestId("counter-increment").click();
```