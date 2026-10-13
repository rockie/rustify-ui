# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: docs-ssg/islands.spec.ts >> no script keeps real SSR views and both source panels readable with native controls disabled
- Location: tests/docs-ssg/islands.spec.ts:250:1

# Error details

```
Error: expect(locator).toBeDisabled() failed

Locator:  locator('[data-rustify-island]').first().locator('fieldset[data-rd-island-controls]')
Expected: disabled
Received: enabled
Timeout:  10000ms

Call log:
  - Expect "toBeDisabled" locator('[data-rustify-island]').first().locator('fieldset[data-rd-island-controls]') with timeout 10000ms
  - waiting for locator('[data-rustify-island]').first().locator('fieldset[data-rd-island-controls]')
    24 × locator resolved to <fieldset disabled class="rd-demo-canvas" data-rd-island-controls="" aria-labelledby="rd-demo-label-island-f271cfadea-0">…</fieldset>
       - unexpected value "enabled"

```

```yaml
- group "Interactive example":
  - paragraph: Shared initial props
  - button "Increment" [disabled]
  - status: "2"
  - textbox "Counter value" [disabled]: "2"
  - paragraph
```

# Test source

```ts
  159 |         };
  160 |     }));
  161 |     await test.info().attach("attached-node-identity.json", { body: JSON.stringify(actual, null, 2), contentType: "application/json" });
  162 |     for (const block of actual) {
  163 |         expect(block.sameRoot, block.key).toBe(true);
  164 |         expect(block.retained, block.key).toBe(block.nodes);
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
> 259 |             await expect(canvas(island)).toBeDisabled();
      |                                          ^ Error: expect(locator).toBeDisabled() failed
  260 |             await expect(island.getByTestId("counter-increment")).toBeDisabled();
  261 |             await expect(island.getByTestId("counter-input")).toHaveValue(index === 3 ? "7" : "2");
  262 |             await expect(island.getByTestId("counter-value")).toHaveText(index === 3 ? "7" : "2");
  263 |             await expect(island.locator("noscript")).toHaveText("Enable JavaScript to interact");
  264 |         }
  265 |         await expect(page.locator(".rd-demo-source .rd-code")).toHaveCount(2);
  266 |         await expect(page.locator(".rd-demo-source .rd-code-tabs")).toBeHidden();
  267 |         for (const panel of await page.locator(".rd-demo-source .rd-code").all()) await expect(panel).toBeVisible();
  268 |         expect(await page.locator("template.rd-code-raw").last().evaluate(node => (node as HTMLTemplateElement).content.textContent)).toBe(fixtureSource());
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
```