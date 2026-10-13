# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: docs-ssg/dev-components.spec.ts >> the address and first building shell precede the actual compilation, then content saves reuse its pair
- Location: tests/docs-ssg/dev-components.spec.ts:321:5

# Error details

```
Error: expect(received).toEqual(expected) // deep equality

- Expected  - 0
+ Received  + 2

  Array [
    "docs-host",
    "docs-wasm",
+   "docs-host",
+   "docs-wasm",
  ]
```

# Page snapshot

```yaml
- generic [active] [ref=f1e1]:
  - link "Skip to content" [ref=f1e2] [cursor=pointer]:
    - /url: "#content"
  - banner [ref=f1e3]:
    - generic [ref=f1e4]:
      - generic [ref=f1e5]: Reading laboratory
      - navigation "Site links" [ref=f1e7]:
        - list [ref=f1e8]:
          - listitem [ref=f1e9]:
            - link "Counters" [ref=f1e10] [cursor=pointer]:
              - /url: /docs/counter/
          - listitem [ref=f1e11]:
            - link "Catalog proxy" [ref=f1e12] [cursor=pointer]:
              - /url: /docs/proxy/
      - group "Theme" [ref=f1e13]:
        - button "System" [pressed] [ref=f1e14] [cursor=pointer]
        - button "Light" [ref=f1e20] [cursor=pointer]
        - button "Dark" [ref=f1e26] [cursor=pointer]
  - generic [ref=f1e31]:
    - generic [ref=f1e32]:
      - paragraph [ref=f1e34]: Examples in this guide
      - navigation "Menu" [ref=f1e35]:
        - generic [ref=f1e36]:
          - heading "Custom pages" [level=2] [ref=f1e37]
          - list [ref=f1e38]:
            - listitem [ref=f1e39]:
              - link "Custom page" [ref=f1e40] [cursor=pointer]:
                - /url: /docs/custom/
            - listitem [ref=f1e41]:
              - link "Static custom page" [ref=f1e42] [cursor=pointer]:
                - /url: /docs/plain/
        - generic [ref=f1e43]:
          - heading "Examples" [level=2] [ref=f1e44]
          - list [ref=f1e45]:
            - listitem [ref=f1e46]:
              - link "Registered counters" [ref=f1e47] [cursor=pointer]:
                - /url: /docs/counter/
            - listitem [ref=f1e48]:
              - link "Static built-in components" [ref=f1e49] [cursor=pointer]:
                - /url: /docs/builtins/
            - listitem [ref=f1e50]:
              - link "Browser work after attachment" [ref=f1e51] [cursor=pointer]:
                - /url: /docs/async/
            - listitem [ref=f1e52]:
              - link "Client-only counter" [ref=f1e53] [cursor=pointer]:
                - /url: /docs/client-only/
            - listitem [ref=f1e54]:
              - link "Runtime failure recovery" [ref=f1e55] [cursor=pointer]:
                - /url: /docs/failure/
            - listitem [ref=f1e56]:
              - link "Catalog release proxy" [ref=f1e57] [cursor=pointer]:
                - /url: /docs/proxy/
      - paragraph [ref=f1e59]: Each example keeps its source nearby
    - navigation "On this page" [ref=f1e61]:
      - paragraph [ref=f1e62]: On this page
      - list [ref=f1e63]:
        - listitem [ref=f1e64]:
          - link "Three independent instances" [ref=f1e65] [cursor=pointer]:
            - /url: "#three-independent-instances"
        - listitem [ref=f1e66]:
          - link "Function and full file" [ref=f1e67] [cursor=pointer]:
            - /url: "#function-and-full-file"
    - main [ref=f1e68]:
      - generic [ref=f1e69]:
        - paragraph [ref=f1e72]: Try the example, then read its implementation
        - article [ref=f1e73]:
          - heading "Registered counters" [level=1] [ref=f1e74]
          - heading "Three independent instances" [level=2] [ref=f1e75]
          - paragraph [ref=f1e76]: Each counter starts with the same props and child content. Incrementing one leaves the others at two.
          - generic [ref=f1e77]:
            - group "Interactive example" [ref=f1e78]:
              - generic [ref=f1e81]:
                - paragraph [ref=f1e82]: Shared initial props
                - generic [ref=f1e83]:
                  - button "Increment" [ref=f1e84] [cursor=pointer]
                  - status [ref=f1e85]: "2"
                  - textbox "Counter value" [ref=f1e86]: "2"
                - paragraph [ref=f1e88]:
                  - text: The same
                  - strong [ref=f1e89]: safe child Markdown
                  - text: appears in each instance.
            - status [ref=f1e90]:
              - generic [ref=f1e91]: Interactive example
          - generic [ref=f1e92]:
            - group "Interactive example" [ref=f1e93]:
              - generic [ref=f1e96]:
                - paragraph [ref=f1e97]: Shared initial props
                - generic [ref=f1e98]:
                  - button "Increment" [ref=f1e99] [cursor=pointer]
                  - status [ref=f1e100]: "2"
                  - textbox "Counter value" [ref=f1e101]: "2"
                - paragraph [ref=f1e103]:
                  - text: The same
                  - strong [ref=f1e104]: safe child Markdown
                  - text: appears in each instance.
            - status [ref=f1e105]:
              - generic [ref=f1e106]: Interactive example
          - generic [ref=f1e107]:
            - group "Interactive example" [ref=f1e108]:
              - generic [ref=f1e111]:
                - paragraph [ref=f1e112]: Shared initial props
                - generic [ref=f1e113]:
                  - button "Increment" [ref=f1e114] [cursor=pointer]
                  - status [ref=f1e115]: "2"
                  - textbox "Counter value" [ref=f1e116]: "2"
                - paragraph [ref=f1e118]:
                  - text: The same
                  - strong [ref=f1e119]: safe child Markdown
                  - text: appears in each instance.
            - status [ref=f1e120]:
              - generic [ref=f1e121]: Interactive example
          - heading "Function and full file" [level=2] [ref=f1e122]
          - paragraph [ref=f1e123]: The registered function starts at seven. Its original function and containing file appear below the view.
          - generic [ref=f1e124]:
            - group "Interactive example" [ref=f1e125]:
              - generic [ref=f1e128]:
                - paragraph [ref=f1e129]: Résumé — a real registered counter
                - generic [ref=f1e130]:
                  - button "Increment" [ref=f1e131] [cursor=pointer]
                  - status [ref=f1e132]: "7"
                  - textbox "Counter value" [ref=f1e133]: "7"
                - paragraph [ref=f1e134]: This child is part of the same view on both targets.
            - status [ref=f1e135]:
              - generic [ref=f1e136]: Interactive example
            - generic [ref=f1e137]:
              - paragraph [ref=f1e138]: src/demos.rs
              - generic [ref=f1e139]:
                - generic [ref=f1e140]:
                  - tablist [ref=f1e141]:
                    - tab "Function" [selected] [ref=f1e142] [cursor=pointer]
                    - tab "Full file" [ref=f1e143] [cursor=pointer]
                  - button "Copy" [ref=f1e144] [cursor=pointer]
                - tabpanel "Function" [ref=f1e149]:
                  - code [ref=f1e151]:
                    - generic [ref=f1e152]: /// The function and its containing file are displayed verbatim by the docs.
                    - generic [ref=f1e156]:
                      - generic [ref=f1e157]:
                        - generic [ref=f1e158]: pub fn
                        - text: counter_demo
                      - generic [ref=f1e159]:
                        - generic [ref=f1e160]: (
                        - generic [ref=f1e161]: )
                      - generic [ref=f1e163]: "-> impl IntoView"
                      - generic [ref=f1e165]: "{"
                    - generic [ref=f1e170]:
                      - text: counter
                      - generic [ref=f1e171]: (
                    - generic [ref=f1e172]: 7,
                    - generic [ref=f1e181]:
                      - generic [ref=f1e182]: "\"Résumé — a real registered counter\""
                      - text: .to_string
                      - generic [ref=f1e183]: (
                      - generic [ref=f1e184]: )
                      - text: ","
                    - generic [ref=f1e189]:
                      - text: view!
                      - generic [ref=f1e190]:
                        - text: "{ <p>"
                        - generic [ref=f1e191]: "\"This child is part of the same view on both targets.\""
                        - text: </p>
                      - generic [ref=f1e192]: "}"
                      - text: .into_any
                      - generic [ref=f1e193]: (
                      - generic [ref=f1e194]: )
                      - text: ","
                    - generic [ref=f1e195]: )
                    - generic [ref=f1e200]: "}"
        - paragraph [ref=f1e205]: Continue with the next page
        - navigation "Previous / Next" [ref=f1e206]:
          - link "Previous Static custom page" [ref=f1e207] [cursor=pointer]:
            - /url: /docs/plain/
            - generic [ref=f1e208]: Previous
            - generic [ref=f1e209]: Static custom page
          - link "Next Static built-in components" [ref=f1e210] [cursor=pointer]:
            - /url: /docs/builtins/
            - generic [ref=f1e211]: Next
            - generic [ref=f1e212]: Static built-in components
  - contentinfo [ref=f1e213]:
    - paragraph [ref=f1e215]:
      - text: Keep reading ·
      - link "Home" [ref=f1e216] [cursor=pointer]:
        - /url: /docs/
  - paragraph [ref=f1e217]
  - status [ref=f1e218]
  - button "4 warnings" [ref=f1e220] [cursor=pointer]
```

# Test source

```ts
  246 |         values: [...tree.root.querySelectorAll("input")].map(node => node.value), originalValues: tree.values,
  247 |     })));
  248 |     expect(records).toHaveLength(4);
  249 |     for (const record of records) {
  250 |         expect(record.retained).toBe(record.nodes);
  251 |         expect(record.root && record.ids && record.source && record.sourceNodes).toBe(true);
  252 |         expect(record.values).toEqual(record.originalValues);
  253 |     }
  254 |     expect(await page.evaluate(() => window.__dev_component_document)).toBe("the actual last good document");
  255 | }
  256 | 
  257 | async function settled(after = -1, ok = true): Promise<State> {
  258 |     await expect.poll(() => {
  259 |         const state = server!.state();
  260 |         return !!state && state.revision > after && !state.building && state.ok === ok;
  261 |     }, { timeout: buildTimeout }).toBe(true);
  262 |     return server!.state()!;
  263 | }
  264 | 
  265 | function compilation(state: State) {
  266 |     expect(state.compile).not.toBeNull();
  267 |     const compiled = state.compile!;
  268 |     expect(compiled.completed_at_ms).toBe(Math.max(compiled.host.completed_at_ms, compiled.wasm.completed_at_ms));
  269 |     expect(compiled.host.profile).toBe("docs-host");
  270 |     expect(compiled.wasm.profile).toBe("docs-wasm");
  271 |     expect(compiled.host.target).toBeNull();
  272 |     expect(compiled.wasm.target).toBe("wasm32-unknown-unknown");
  273 |     for (const timing of [compiled.host, compiled.wasm]) {
  274 |         expect(timing.success).toBe(true);
  275 |         expect(timing.flags).toContain("--locked");
  276 |         expect(timing.elapsed_ms).toBeGreaterThanOrEqual(0);
  277 |         expect(timing.fresh_artifacts + timing.compiled_artifacts).toBeGreaterThan(0);
  278 |     }
  279 |     return compiled;
  280 | }
  281 | 
  282 | async function layouts(page: Page, state: string) {
  283 |     const viewport = page.viewportSize()!;
  284 |     const position = await page.evaluate(() => scrollY);
  285 |     const preference = await page.locator("html").getAttribute("data-theme") || "system";
  286 |     for (const variant of [
  287 |         { name: "desktop-light", mode: "light", width: 1280, height: 900 },
  288 |         { name: "desktop-dark", mode: "dark", width: 1280, height: 900 },
  289 |         { name: "narrow-light", mode: "light", width: 375, height: 812 },
  290 |     ]) {
  291 |         await page.setViewportSize({ width: variant.width, height: variant.height });
  292 |         await page.locator(`[data-rd-mode="${variant.mode}"]`).click();
  293 |         await page.evaluate(async () => {
  294 |             await document.fonts.ready;
  295 |             await Promise.allSettled(document.getAnimations().filter(animation => animation instanceof CSSTransition).map(animation => animation.finished));
  296 |             await new Promise<void>(resolve => requestAnimationFrame(() => requestAnimationFrame(() => resolve())));
  297 |         });
  298 |         await expect(page.locator("html")).toHaveAttribute("data-theme", variant.mode);
  299 |         expect(await page.evaluate(() => document.documentElement.scrollWidth)).toBeLessThanOrEqual(variant.width);
  300 |         await test.info().attach(`${state}-${variant.name}.png`, { body: await page.screenshot(), contentType: "image/png" });
  301 |     }
  302 |     await page.setViewportSize(viewport);
  303 |     await page.locator(`[data-rd-mode="${preference}"]`).click();
  304 |     await page.evaluate(y => window.scrollTo(0, y), position);
  305 | }
  306 | 
  307 | async function healthy() {
  308 |     expect(audit.csp).toEqual([]);
  309 |     expect(audit.failures).toEqual([]);
  310 |     expect(audit.responses.filter(response => response.status >= 400)).toEqual([]);
  311 |     expect(audit.console).toEqual([]);
  312 |     expect(audit.pageErrors).toEqual([]);
  313 |     expect(server!.streamErrors).toEqual([]);
  314 |     expect(audit.requests.filter(url => new URL(url).origin !== new URL(server!.url).origin || /makepad/i.test(url))).toEqual([]);
  315 |     expect(readFileSync(path.join(site.root, "Cargo.lock"))).toEqual(site.lock);
  316 |     expect(existsSync(path.join(site.root, "dist"))).toBe(false);
  317 |     const sequences = server!.states.map(state => state.sequence);
  318 |     expect(sequences.every((value, index) => index === 0 || value > sequences[index - 1])).toBe(true);
  319 | }
  320 | 
  321 | test("the address and first building shell precede the actual compilation, then content saves reuse its pair", async ({ page }) => {
  322 |     test.setTimeout(900_000);
  323 |     server = await Dev.start(site);
  324 |     await page.goto(new URL("counter/", server.url).href);
  325 |     await expect(page.getByRole("heading", { name: "Building your site", exact: true })).toBeVisible();
  326 |     await expect(islands(page)).toHaveCount(0);
  327 |     await expect.poll(() => existsSync(site.entered)).toBe(true);
  328 |     await expect.poll(() => server!.state()?.phase).toBe("host");
  329 |     expect(server.state()!.building).toBe(true);
  330 |     expect(server.state()!.good).toBeNull();
  331 |     await layouts(page, "first-building");
  332 | 
  333 |     let releaseWasm!: () => void;
  334 |     const heldWasm = new Promise<void>(resolve => { releaseWasm = resolve; });
  335 |     await page.route("**/assets/islands/**/app_bg.wasm", async route => { await heldWasm; await route.continue(); });
  336 |     site.release();
  337 |     await expect(page.getByRole("heading", { level: 1 })).toHaveText("Registered counters", { timeout: buildTimeout });
  338 |     await expect(page.locator('[data-rustify-island][data-rd-island-state="loading"]')).toHaveCount(4);
  339 |     await captureTrees(page);
  340 |     await sourceText(page, site.read("src/demos.rs"));
  341 |     releaseWasm();
  342 |     await ready(page);
  343 |     await retained(page);
  344 |     const compiled = compilation(await settled());
  345 |     const commands = site.commands();
> 346 |     expect(commands.filter(args => args[0] === "build").map(args => args[args.indexOf("--profile") + 1])).toEqual(["docs-host", "docs-wasm"]);
      |                                                                                                           ^ Error: expect(received).toEqual(expected) // deep equality
  347 |     await page.getByTestId("counter-input").first().fill("20");
  348 |     await expect(values(page)).toHaveText(["20", "2", "2", "7"]);
  349 |     await page.getByTestId("counter-increment").first().click();
  350 |     await expect(values(page)).toHaveText(["21", "2", "2", "7"]);
  351 |     await expect(page.locator(".rd-dev")).toBeHidden();
  352 | 
  353 |     let previous = server.state()!.revision;
  354 |     site.edit("content/counter.md", "Each counter starts with the same props and child content.", "A Markdown save reuses the already compiled view and its source.");
  355 |     await expect(page.getByText("A Markdown save reuses the already compiled view and its source.", { exact: false })).toBeVisible();
  356 |     expect((await settled(previous)).compile).toEqual(compiled);
  357 |     expect(site.commands()).toEqual(commands);
  358 |     await ready(page);
  359 | 
  360 |     previous = server.state()!.revision;
  361 |     site.edit("rustify-docs.toml", 'name = "Component fixture"', 'name = "Component handbook"');
  362 |     await expect(page).toHaveTitle("Registered counters | Component handbook");
  363 |     expect((await settled(previous)).compile).toEqual(compiled);
  364 |     expect(site.commands()).toEqual(commands);
  365 |     await expect(page.locator('meta[property="og:site_name"]')).toHaveAttribute("content", "Component handbook");
  366 |     await ready(page);
  367 |     await sourceText(page, site.read("src/demos.rs"));
  368 | 
  369 |     previous = server.state()!.revision;
  370 |     const theme = JSON.parse(site.read("theme.json"));
  371 |     theme.styles.light.background = "#f2f7ff";
  372 |     site.write("theme.json", JSON.stringify(theme, null, 2) + "\n");
  373 |     await page.locator('[data-rd-mode="light"]').click();
  374 |     await expect(page.locator("body")).toHaveCSS("background-color", "rgb(242, 247, 255)");
  375 |     expect((await settled(previous)).compile).toEqual(compiled);
  376 |     expect(site.commands()).toEqual(commands);
  377 |     await ready(page);
  378 |     await sourceText(page, site.read("src/demos.rs"));
  379 |     const wasm = audit.responses.filter(response => response.url.endsWith("/app_bg.wasm"));
  380 |     expect(wasm.length).toBeGreaterThan(0);
  381 |     expect(new Set(wasm.map(response => response.url)).size).toBe(1);
  382 |     expect(wasm.every(response => response.status === 200 && response.mime.split(";")[0] === "application/wasm")).toBe(true);
  383 |     await healthy();
  384 | });
  385 | 
  386 | test("Rust saves keep the original interactive document until a complete new pair, and diagnostics recover at its scroll position", async ({ page }) => {
  387 |     test.setTimeout(900_000);
  388 |     server = await Dev.start(site);
  389 |     await page.goto(new URL("counter/", server.url).href);
  390 |     site.release();
  391 |     await ready(page);
  392 |     const firstState = await settled();
  393 |     const first = compilation(firstState);
  394 |     const original = site.read("src/demos.rs");
  395 |     const firstWasm = audit.requests.filter(url => url.endsWith("/app_bg.wasm")).at(-1)!;
  396 |     expect(firstWasm).toBeTruthy();
  397 |     await sourceText(page, original);
  398 |     await source(page).getByRole("tab", { name: "Full file", exact: true }).click();
  399 |     await page.getByTestId("counter-input").first().fill("30");
  400 |     await expect(values(page)).toHaveText(["30", "2", "2", "7"]);
  401 |     await captureTrees(page);
  402 | 
  403 |     const changed = original.replace("*value += 1", "*value += 3").replace('        7,\n        "Résumé', '        11,\n        "Résumé');
  404 |     expect(changed).not.toBe(original);
  405 |     site.hold();
  406 |     site.write("src/demos.rs", changed);
  407 |     await expect.poll(() => existsSync(site.entered)).toBe(true);
  408 |     await expect(page.locator(".rd-dev-marker")).toContainText("Building");
  409 |     await expect(page.locator(".rd-dev-marker")).toContainText("showing the last good version");
  410 |     await retained(page);
  411 |     await sourceText(page, original);
  412 |     await layouts(page, "rust-building");
  413 |     await page.getByTestId("counter-increment").first().click();
  414 |     await expect(values(page)).toHaveText(["31", "2", "2", "7"]);
  415 |     await source(page).getByRole("tab", { name: "Full file", exact: true }).click();
  416 |     const scroll = await page.evaluate(() => {
  417 |         const maximum = document.documentElement.scrollHeight - innerHeight;
  418 |         if (maximum < 300) throw new Error("the real source page must have room for a scroll-restoration check");
  419 |         const position = Math.min(500, maximum - 50);
  420 |         window.scrollTo(0, position);
  421 |         return position;
  422 |     });
  423 |     await expect.poll(() => page.evaluate(() => scrollY)).toBe(scroll);
  424 |     site.release();
  425 |     await ready(page, "11");
  426 |     await sourceText(page, changed);
  427 |     await expect(source(page).getByRole("tab", { name: "Full file", exact: true })).toHaveAttribute("aria-selected", "true");
  428 |     await expect.poll(() => page.evaluate(() => scrollY)).toBe(scroll);
  429 |     expect(await page.evaluate(() => window.__dev_component_document)).toBeUndefined();
  430 |     const second = compilation(await settled(firstState.revision));
  431 |     expect(second.completed_at_ms).toBeGreaterThan(first.completed_at_ms);
  432 |     expect(second.generation).not.toBe(first.generation);
  433 |     const secondWasm = audit.requests.filter(url => url.endsWith("/app_bg.wasm")).at(-1)!;
  434 |     expect(secondWasm).not.toBe(firstWasm);
  435 |     const group = secondWasm.slice(0, secondWasm.lastIndexOf("/") + 1);
  436 |     expect(audit.responses.some(response => response.url === `${group}app.js` && response.status === 200)).toBe(true);
  437 |     expect(audit.responses.some(response => response.url === secondWasm && response.status === 200 && response.mime.split(";")[0] === "application/wasm")).toBe(true);
  438 |     await page.getByTestId("counter-increment").first().click();
  439 |     await expect(values(page)).toHaveText(["5", "2", "2", "11"]);
  440 | 
  441 |     const bad = changed.replace("RwSignal::new(initial)", "RwSignal::new(initial + missing_counter_step)");
  442 |     const line = bad.slice(0, bad.indexOf("missing_counter_step")).split("\n").length;
  443 |     const column = bad.split("\n")[line - 1].indexOf("missing_counter_step") + 1;
  444 |     const previous = server.state()!.revision;
  445 |     await captureTrees(page);
  446 |     site.write("src/demos.rs", bad);
```