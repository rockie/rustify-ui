# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: docs-ssg/islands.spec.ts >> no script keeps real SSR views and both source panels readable with native controls disabled
- Location: tests/docs-ssg/islands.spec.ts:250:1

# Error details

```
Error: expect(locator).toHaveText(expected) failed

Locator:  locator('[data-rustify-island]').first().locator('noscript')
Expected: "Enable JavaScript to interact"
Received: ""
Timeout:  10000ms

Call log:
  - Expect "toHaveText" locator('[data-rustify-island]').first().locator('noscript') with timeout 10000ms
  - waiting for locator('[data-rustify-island]').first().locator('noscript')
    24 × locator resolved to <noscript>…</noscript>
       - unexpected value ""

```

# Test source

```ts
  165 |         expect(block.sameIds, block.key).toBe(true);
  166 |         expect(block.sameNativeValues, block.key).toBe(true);
  167 |         expect(block.sameSource, block.key).toBe(true);
  168 |         expect(block.statusHeight, block.key).toBe(block.initialStatusHeight);
  169 |         expect(block.sourceTop, block.key).toBe(block.initialSourceTop);
  170 |     }
  171 |     expect(await page.evaluate(() => {
  172 |         const ids = [...document.querySelectorAll("[id]")].map(node => node.id);
  173 |         return ids.length === new Set(ids).size;
  174 |     })).toBe(true);
  175 | }
  176 | 
  177 | async function openPaused(page: Page, route: string, file = "app_bg.wasm") {
  178 |     const held = await holdAsset(page, file);
  179 |     await page.goto(url(route), { waitUntil: "commit" });
  180 |     await held.seen;
  181 |     await expect(page.locator('[data-rustify-island][data-rd-island-state="loading"]')).toHaveCount(await islands(page).count());
  182 |     await captureOriginals(page);
  183 |     return held;
  184 | }
  185 | 
  186 | async function restored(page: Page, count: number) {
  187 |     await expect(page.locator('[data-rustify-island][data-rd-island-state="failed"]')).toHaveCount(count);
  188 |     expect(await page.evaluate(() => window.__rustify_docs_islands?.state())).toBe("failed");
  189 |     const result = await page.evaluate(() => ({
  190 |         blocks: window.__docs_island_probe!.blocks.map(snapshot => {
  191 |             const block = [...document.querySelectorAll("[data-rustify-island]")].find(block => block.getAttribute("data-rustify-island") === snapshot.key)!;
  192 |             return {
  193 |                 key: snapshot.key,
  194 |                 original: block.querySelector("fieldset[data-rd-island-controls]")!.firstElementChild!.isEqualNode(snapshot.original),
  195 |                 sameSource: block.querySelector(".rd-demo-source") === snapshot.source,
  196 |             };
  197 |         }),
  198 |         navigation: window.__docs_island_probe!.navigation.every(node => node.isConnected),
  199 |     }));
  200 |     await test.info().attach("restored-ssr-trees.json", { body: JSON.stringify(result, null, 2), contentType: "application/json" });
  201 |     expect(result.navigation).toBe(true);
  202 |     for (const block of result.blocks) {
  203 |         expect(block.original, block.key).toBe(true);
  204 |         expect(block.sameSource, block.key).toBe(true);
  205 |     }
  206 |     for (const island of await islands(page).all()) {
  207 |         await expect(canvas(island)).toHaveJSProperty("disabled", true);
  208 |         await expect(canvas(island)).toHaveJSProperty("inert", true);
  209 |         await expect(canvas(island)).toHaveAttribute("aria-busy", "false");
  210 |         await expect(island.locator("[data-rd-island-status]")).toHaveAttribute("role", "alert");
  211 |         await expect(island.locator("[data-rd-island-error]")).toHaveText("This example could not start. Reload the page to try again.");
  212 |         await expect(island.locator("[data-rd-island-error]")).toBeVisible();
  213 |         await expect(island.locator("[data-rd-island-reload]")).toBeEnabled();
  214 |         await expect(island.locator("[data-rd-island-loading]")).toBeHidden();
  215 |     }
  216 | }
  217 | 
  218 | async function sourceCopy(page: Page) {
  219 |     const source = page.locator(".rd-demo-source");
  220 |     await expect(source).toHaveCount(1);
  221 |     await expect(source.locator("[data-rd-source-path]")).toHaveText("src/demos.rs");
  222 |     const file = fixtureSource();
  223 |     const originalFunction = file.match(/(?:\/\/\/[^\r\n]*\r?\n)*pub fn counter_demo\(\)[\s\S]*?^\}(?:\r?\n|$)/m)?.[0];
  224 |     expect(originalFunction, "the fixture's direct function is present").toBeTruthy();
  225 |     const expected = [originalFunction!, file];
  226 |     const raw = await source.locator("template.rd-code-raw").evaluateAll(nodes => nodes.map(node => (node as HTMLTemplateElement).content.textContent));
  227 |     expect(raw).toEqual(expected);
  228 |     for (const [index, label] of ["Function", "Full file"].entries()) {
  229 |         const tab = source.getByRole("tab", { name: label, exact: true });
  230 |         await tab.click();
  231 |         await expect(tab).toHaveAttribute("aria-selected", "true");
  232 |         await expect(source.locator(`[data-rd-tab-panel="${index}"]`)).toBeVisible();
  233 |         await source.locator(".rd-code-tabs > [data-rd-copy]").click();
  234 |         expect(await page.evaluate(() => navigator.clipboard.readText())).toBe(expected[index]);
  235 |         await expect(source.locator(".rd-code-tabs > [data-rd-copy]")).toHaveAttribute("data-state", "copied");
  236 |     }
  237 |     await source.getByRole("tab", { name: "Full file", exact: true }).press("Home");
  238 |     await expect(source.getByRole("tab", { name: "Function", exact: true })).toBeFocused();
  239 |     await expect(source.getByRole("tab", { name: "Function", exact: true })).toHaveAttribute("aria-selected", "true");
  240 | }
  241 | 
  242 | test("a page without interactive embeds loads no island JavaScript or wasm", async ({ page, audit }) => {
  243 |     await page.goto(url("./"));
  244 |     await expect(page.getByRole("heading", { level: 1 })).toHaveText("Component fixture");
  245 |     await expect(islands(page)).toHaveCount(0);
  246 |     expect(audit.requests.filter(request => /\/assets\/islands\//.test(new URL(request).pathname) || new URL(request).pathname.endsWith(".wasm"))).toEqual([]);
  247 |     await healthy(page, audit);
  248 | });
  249 | 
  250 | test("no script keeps real SSR views and both source panels readable with native controls disabled", async ({ browser }) => {
  251 |     const context = await browser.newContext({ javaScriptEnabled: false, reducedMotion: "reduce" });
  252 |     const page = await context.newPage();
  253 |     const audit = await watch(page);
  254 |     try {
  255 |         await page.goto(url("counter/"));
  256 |         await expect(islands(page)).toHaveCount(4);
  257 |         for (const [index, island] of (await islands(page).all()).entries()) {
  258 |             await expect(island).toHaveAttribute("data-rd-island-state", "no-script");
  259 |             await expect(canvas(island)).toHaveAttribute("disabled", "");
  260 |             await expect(canvas(island)).toHaveJSProperty("disabled", true);
  261 |             await expect(island.getByTestId("counter-increment")).toBeDisabled();
  262 |             await expect(island.getByTestId("counter-input")).toBeDisabled();
  263 |             await expect(island.getByTestId("counter-input")).toHaveValue(index === 3 ? "7" : "2");
  264 |             await expect(island.getByTestId("counter-value")).toHaveText(index === 3 ? "7" : "2");
> 265 |             await expect(island.locator("noscript")).toHaveText("Enable JavaScript to interact");
      |                                                      ^ Error: expect(locator).toHaveText(expected) failed
  266 |         }
  267 |         await expect(page.locator(".rd-demo-source .rd-code")).toHaveCount(2);
  268 |         await expect(page.locator(".rd-demo-source .rd-code-tabs")).toBeHidden();
  269 |         for (const panel of await page.locator(".rd-demo-source .rd-code").all()) await expect(panel).toBeVisible();
  270 |         expect(await page.locator("template.rd-code-raw").last().evaluate(node => (node as HTMLTemplateElement).content.textContent)).toBe(fixtureSource());
  271 |         await page.goto(url("client-only/"));
  272 |         await expect(page.locator("[data-rustify-client-message]")).toHaveText("This counter starts in the browser.");
  273 |         await expect(page.getByTestId("counter-increment")).toHaveCount(0);
  274 |         expect((await page.locator("[data-rustify-client-only]").boundingBox())!.height).toBeGreaterThanOrEqual(192);
  275 |         await page.goto(url("proxy/"));
  276 |         await expect(page.locator("[data-category]")).toHaveCount(24);
  277 |         await expect(page.getByRole("textbox", { name: "Multiline text" })).toHaveValue("\nRésumé </textarea> & <line>\nCRLF\nCR\nlast");
  278 |         await expect(page.getByRole("textbox", { name: "Multiline text" })).toBeDisabled();
  279 |         expect(audit.requests.filter(request => new URL(request).pathname.endsWith(".wasm"))).toEqual([]);
  280 |         expect(audit.failures).toEqual([]);
  281 |         expect(audit.responses).toEqual([]);
  282 |         expect(audit.console).toEqual([]);
  283 |         expect(audit.pageErrors).toEqual([]);
  284 |         await test.info().attach("no-script-browser-audit.json", { body: JSON.stringify(audit, null, 2), contentType: "application/json" });
  285 |     } finally {
  286 |         await context.close();
  287 |     }
  288 | });
  289 | 
  290 | test("loading gates native and captured input while source controls remain usable", async ({ page, context, audit }) => {
  291 |     await context.grantPermissions(["clipboard-read", "clipboard-write"]);
  292 |     const held = await openPaused(page, "counter/");
  293 |     try {
  294 |         const island = islands(page).first();
  295 |         await expect(canvas(island)).toHaveJSProperty("disabled", false);
  296 |         await expect(canvas(island)).toHaveJSProperty("inert", true);
  297 |         await expect(canvas(island)).toHaveAttribute("aria-busy", "true");
  298 |         await expect(island.locator("[data-rd-island-loading]")).toBeVisible();
  299 |         const result = await island.getByTestId("counter-increment").evaluate(button => {
  300 |             let delivered = 0;
  301 |             const names = ["click", "pointerdown", "keydown", "beforeinput", "input", "change", "submit"];
  302 |             for (const name of names) button.addEventListener(name, () => delivered++);
  303 |             const accepted = names.map(name => button.dispatchEvent(new Event(name, { bubbles: true, cancelable: true })));
  304 |             (button as HTMLElement).focus();
  305 |             return { accepted, delivered, focused: document.activeElement === button };
  306 |         });
  307 |         expect(result).toEqual({ accepted: Array(7).fill(false), delivered: 0, focused: false });
  308 |         const box = (await island.getByTestId("counter-increment").boundingBox())!;
  309 |         await page.mouse.click(box.x + box.width / 2, box.y + box.height / 2);
  310 |         await expect(island.getByTestId("counter-value")).toHaveText("2");
  311 |         await sourceCopy(page);
  312 |         held.release();
  313 |         await ready(page, 4);
  314 |         await identity(page);
  315 |         await healthy(page, audit);
  316 |     } finally {
  317 |         held.release();
  318 |     }
  319 | });
  320 | 
  321 | test("attachment retains original SSR nodes and isolates three identical counters", async ({ page, audit }) => {
  322 |     const held = await openPaused(page, "counter/");
  323 |     held.release();
  324 |     await ready(page, 4);
  325 |     await identity(page);
  326 |     const counters = islands(page).filter({ has: page.locator('[data-counter]') });
  327 |     await expect(counters).toHaveCount(4);
  328 |     await counters.nth(0).getByTestId("counter-increment").click();
  329 |     await expect(counters.nth(0).getByTestId("counter-value")).toHaveText("3");
  330 |     await counters.nth(1).getByTestId("counter-input").fill("19");
  331 |     await expect(counters.nth(1).getByTestId("counter-value")).toHaveText("19");
  332 |     await counters.nth(2).getByTestId("counter-increment").focus();
  333 |     await page.keyboard.press("Enter");
  334 |     await expect(counters.nth(2).getByTestId("counter-value")).toHaveText("3");
  335 |     await expect(counters.nth(0).getByTestId("counter-value")).toHaveText("3");
  336 |     await expect(counters.nth(1).getByTestId("counter-value")).toHaveText("19");
  337 |     await expect(counters.nth(3).getByTestId("counter-value")).toHaveText("7");
  338 |     expect((await page.getByTestId("counter-input").all()).length).toBe(4);
  339 |     await healthy(page, audit);
  340 | });
  341 | 
  342 | test("an asynchronous action starts after attachment with supported hydration globals", async ({ page, audit }) => {
  343 |     const held = await openPaused(page, "async/");
  344 |     await expect(page.getByTestId("async-result")).toHaveText("Not loaded");
  345 |     held.release();
  346 |     await ready(page, 1);
  347 |     await identity(page);
  348 |     expect(await page.evaluate(() => ["__RESOLVED_RESOURCES", "__INCOMPLETE_CHUNKS", "__SERIALIZED_ERRORS"].map(name => Array.isArray((window as unknown as Record<string, unknown>)[name])))).toEqual([true, true, true]);
  349 |     await page.getByTestId("async-start").click();
  350 |     await expect(page.getByTestId("async-result")).toHaveText("Loaded after attachment");
  351 |     await healthy(page, audit);
  352 | });
  353 | 
  354 | test("client-only attachment preserves the placeholder and mounts typed props and child content", async ({ page, audit }) => {
  355 |     const held = await openPaused(page, "client-only/");
  356 |     const placeholder = page.locator("[data-rustify-client-only]");
  357 |     const original = await placeholder.elementHandle();
  358 |     const preserved = await page.evaluateHandle(() => {
  359 |         const snapshot = window.__docs_island_probe!.blocks[0];
  360 |         const message = snapshot.root.querySelector("[data-rustify-client-message]")!;
  361 |         return snapshot.nodes.filter(node => !message.contains(node));
  362 |     });
  363 |     const height = (await placeholder.boundingBox())!.height;
  364 |     await expect(page.locator("[data-rustify-client-message]")).toBeVisible();
  365 |     held.release();
```