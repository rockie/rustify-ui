# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: docs-ssg/islands.spec.ts >> client-only attachment preserves the placeholder and mounts typed props and child content
- Location: tests/docs-ssg/islands.spec.ts:352:1

# Error details

```
Error: expect(locator).toHaveCount(expected) failed

Locator:  locator('[data-rustify-client-host][data-rustify-mounted]')
Expected: 1
Received: 0
Timeout:  10000ms

Call log:
  - Expect "toHaveCount" locator('[data-rustify-client-host][data-rustify-mounted]') with timeout 10000ms
  - waiting for locator('[data-rustify-client-host][data-rustify-mounted]')
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
      - link "Component fixture" [ref=e5] [cursor=pointer]:
        - /url: /rustify-ui/
      - navigation "Site links" [ref=e7]:
        - list [ref=e8]:
          - listitem [ref=e9]:
            - link "Counters" [ref=e10] [cursor=pointer]:
              - /url: /rustify-ui/counter/
          - listitem [ref=e11]:
            - link "Catalog proxy" [ref=e12] [cursor=pointer]:
              - /url: /rustify-ui/proxy/
      - group "Theme" [ref=e13]:
        - button "System" [pressed] [ref=e14] [cursor=pointer]
        - button "Light" [ref=e20] [cursor=pointer]
        - button "Dark" [ref=e26] [cursor=pointer]
  - generic [ref=e31]:
    - navigation "Menu" [ref=e33]:
      - generic [ref=e34]:
        - heading "Examples" [level=2] [ref=e35]
        - list [ref=e36]:
          - listitem [ref=e37]:
            - link "Registered counters" [ref=e38] [cursor=pointer]:
              - /url: /rustify-ui/counter/
          - listitem [ref=e39]:
            - link "Static built-in components" [ref=e40] [cursor=pointer]:
              - /url: /rustify-ui/builtins/
          - listitem [ref=e41]:
            - link "Browser work after attachment" [ref=e42] [cursor=pointer]:
              - /url: /rustify-ui/async/
          - listitem [ref=e43]:
            - link "Client-only counter" [ref=e44] [cursor=pointer]:
              - /url: /rustify-ui/client-only/
          - listitem [ref=e45]:
            - link "Runtime failure recovery" [ref=e46] [cursor=pointer]:
              - /url: /rustify-ui/failure/
          - listitem [ref=e47]:
            - link "Catalog release proxy" [ref=e48] [cursor=pointer]:
              - /url: /rustify-ui/proxy/
    - main [ref=e49]:
      - generic [ref=e50]:
        - article [ref=e51]:
          - heading "Client-only counter" [level=1] [ref=e52]
          - paragraph [ref=e53]: The server publishes the declaration's fixed-size placeholder. The same typed props and child Markdown reach the real counter after attachment.
          - generic [ref=e54]:
            - group "Interactive example" [ref=e55]:
              - generic [ref=e61]:
                - paragraph [ref=e62]: Browser counter
                - generic [ref=e63]:
                  - button "Increment" [ref=e64] [cursor=pointer]
                  - status [ref=e65]: "2"
                  - textbox "Counter value" [ref=e66]: "2"
                - paragraph [ref=e68]:
                  - text: This
                  - strong [ref=e69]: browser-created child
                  - text: remains readable after mounting.
            - status [ref=e70]:
              - generic [ref=e71]: Interactive example
        - navigation "Previous / Next" [ref=e72]:
          - link "Previous Browser work after attachment" [ref=e73] [cursor=pointer]:
            - /url: /rustify-ui/async/
            - generic [ref=e74]: Previous
            - generic [ref=e75]: Browser work after attachment
          - link "Next Runtime failure recovery" [ref=e76] [cursor=pointer]:
            - /url: /rustify-ui/failure/
            - generic [ref=e77]: Next
            - generic [ref=e78]: Runtime failure recovery
  - contentinfo [ref=e79]:
    - navigation "Footer links" [ref=e80]:
      - link "Back to the home page" [ref=e81] [cursor=pointer]:
        - /url: /rustify-ui/
  - paragraph [ref=e82]
```

# Test source

```ts
  269 |         await page.goto(url("client-only/"));
  270 |         await expect(page.locator("[data-rustify-client-message]")).toHaveText("This counter starts in the browser.");
  271 |         await expect(page.getByTestId("counter-increment")).toHaveCount(0);
  272 |         expect((await page.locator("[data-rustify-client-only]").boundingBox())!.height).toBeGreaterThanOrEqual(192);
  273 |         await page.goto(url("proxy/"));
  274 |         await expect(page.locator("[data-category]")).toHaveCount(24);
  275 |         await expect(page.getByRole("textbox", { name: "Multiline text" })).toHaveValue("\nRésumé </textarea> & <line>\nCRLF\nCR\nlast");
  276 |         await expect(page.getByRole("textbox", { name: "Multiline text" })).toBeDisabled();
  277 |         expect(audit.requests.filter(request => new URL(request).pathname.endsWith(".wasm"))).toEqual([]);
  278 |         expect(audit.failures).toEqual([]);
  279 |         expect(audit.responses).toEqual([]);
  280 |         expect(audit.console).toEqual([]);
  281 |         expect(audit.pageErrors).toEqual([]);
  282 |         await test.info().attach("no-script-browser-audit.json", { body: JSON.stringify(audit, null, 2), contentType: "application/json" });
  283 |     } finally {
  284 |         await context.close();
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
> 369 |     await expect(page.locator("[data-rustify-client-host][data-rustify-mounted]")).toHaveCount(1);
      |                                                                                    ^ Error: expect(locator).toHaveCount(expected) failed
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
  385 |         await expect(page.locator("[data-rustify-scope], fieldset[data-rd-island-controls]")).toHaveCount(0);
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
```