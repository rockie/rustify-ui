# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: docs-ssg/islands.spec.ts >> 200% reflow and text scaling keeps loading, ready, failure and bounded source usable in dark
- Location: tests/docs-ssg/islands.spec.ts:622:9

# Error details

```
Error: island-909443d1f3-1

expect(received).toBe(expected) // Object.is equality

Expected: 2113.96875
Received: 2232.96875
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
      - group "Theme" [ref=e7]:
        - button "System" [pressed] [ref=e8] [cursor=pointer]
        - button "Light" [ref=e14] [cursor=pointer]
        - button "Dark" [ref=e20] [cursor=pointer]
  - navigation "Site links" [ref=e25]:
    - list [ref=e26]:
      - listitem [ref=e27]:
        - link "Counters" [ref=e28] [cursor=pointer]:
          - /url: /rustify-ui/counter/
      - listitem [ref=e29]:
        - link "Catalog proxy" [ref=e30] [cursor=pointer]:
          - /url: /rustify-ui/proxy/
  - generic [ref=e31]:
    - group [ref=e33]:
      - generic "Menu" [ref=e34] [cursor=pointer]
    - group [ref=e36]:
      - generic "On this page" [ref=e37] [cursor=pointer]
    - main [ref=e38]:
      - generic [ref=e39]:
        - article [ref=e40]:
          - heading "Runtime failure recovery" [level=1] [ref=e41]
          - paragraph [ref=e42]: Change the first counter, then choose “Crash this example.” Every island in this page belongs to one wasm instance, so the failure restores their initial views and leaves the source and navigation available.
          - heading "A peer island" [level=2] [ref=e43]
          - generic [ref=e44]:
            - group "Interactive example" [ref=e45]:
              - generic [ref=e48]:
                - paragraph [ref=e49]: Counter
                - generic [ref=e50]:
                  - button "Increment" [ref=e51] [cursor=pointer]
                  - status [ref=e52]: "2"
                  - textbox "Counter value" [ref=e53]: "2"
            - status [ref=e54]:
              - generic [ref=e55]: Interactive example
          - heading "Failing callback" [level=2] [ref=e56]
          - generic [ref=e57]:
            - group "Interactive example" [ref=e58]:
              - generic [ref=e61]:
                - button "Change before failure" [ref=e62] [cursor=pointer]
                - status [ref=e63]: "0"
                - button "Crash this example" [ref=e64] [cursor=pointer]
            - status [ref=e65]:
              - generic [ref=e66]: Interactive example
            - generic [ref=e67]:
              - paragraph [ref=e68]: src/demos.rs
              - generic [ref=e69]:
                - generic [ref=e70]:
                  - tablist [ref=e71]:
                    - tab "Function" [selected] [ref=e72] [cursor=pointer]
                    - tab "Full file" [ref=e73] [cursor=pointer]
                  - button "Copy" [ref=e74] [cursor=pointer]
                - tabpanel "Function" [ref=e79]:
                  - code [ref=e81]:
                    - generic [ref=e83]:
                      - generic [ref=e84]:
                        - generic [ref=e85]: pub fn
                        - text: failure_demo
                      - generic [ref=e86]:
                        - generic [ref=e87]: (
                        - generic [ref=e88]: )
                      - generic [ref=e90]: "-> impl IntoView"
                      - generic [ref=e92]: "{"
                    - generic [ref=e97]:
                      - text: let value =
                      - generic [ref=e98]: "RwSignal::"
                      - text: new
                      - generic [ref=e99]: (0
                      - generic [ref=e100]: )
                      - text: ;
                    - generic [ref=e104]:
                      - text: view!
                      - generic [ref=e105]: "{"
                    - generic [ref=e110]:
                      - text: <div data-failure-demo=
                      - generic [ref=e111]: "\"\""
                      - text: class=
                      - generic [ref=e112]: "\"space-y-3\""
                      - text: ">"
                    - generic [ref=e117]:
                      - text: <Button test_id=
                      - generic [ref=e118]: "\"failure-increment\""
                      - text: on_click=move || value.update
                      - generic [ref=e119]:
                        - text: (
                        - generic [ref=e120]: "|"
                        - generic [ref=e122]: value|
                        - generic [ref=e124]: "*value += 1"
                      - generic [ref=e125]: )
                      - text: ">"
                    - generic [ref=e126]: "\"Change before failure\""
                    - generic [ref=e132]: </Button>
                    - generic [ref=e141]:
                      - text: <output data-testid=
                      - generic [ref=e142]: "\"failure-value\""
                      - text: ">"
                      - generic [ref=e143]:
                        - text: "{move || value.get"
                        - generic [ref=e144]: (
                        - generic [ref=e145]: )
                      - generic [ref=e146]: "}"
                      - text: </output>
                    - generic [ref=e151]:
                      - text: <Button test_id=
                      - generic [ref=e152]: "\"failure-crash\""
                      - text: on_click=
                      - generic [ref=e153]: "|"
                      - generic [ref=e155]: "|"
                      - generic [ref=e157]:
                        - text: panic!
                        - generic [ref=e158]:
                          - text: (
                          - generic [ref=e159]: "\"docs fixture callback failure\""
                        - generic [ref=e160]: )
                        - text: ">"
                    - generic [ref=e161]: "\"Crash this example\""
                    - generic [ref=e167]: </Button>
                    - generic [ref=e172]: </div>
                    - generic [ref=e177]: "}"
                    - generic [ref=e182]: "}"
        - navigation "Previous / Next" [ref=e186]:
          - link "Previous Client-only counter" [ref=e187] [cursor=pointer]:
            - /url: /rustify-ui/client-only/
            - generic [ref=e188]: Previous
            - generic [ref=e189]: Client-only counter
          - link "Next Catalog release proxy" [ref=e190] [cursor=pointer]:
            - /url: /rustify-ui/proxy/
            - generic [ref=e191]: Next
            - generic [ref=e192]: Catalog release proxy
  - contentinfo [ref=e193]:
    - navigation "Footer links" [ref=e194]:
      - link "Back to the home page" [ref=e195] [cursor=pointer]:
        - /url: /rustify-ui/
  - paragraph [ref=e196]
```

# Test source

```ts
  69  | const assetPattern = (file: string) => `**/assets/islands/**/${file}`;
  70  | const fixtureSource = () => readFileSync("tests/docs-ssg/fixtures/components/src/demos.rs", "utf8");
  71  | const url = (route: string) => new URL(route, String(test.info().project.use.baseURL)).href;
  72  | 
  73  | async function healthy(page: Page, audit: Audit) {
  74  |     expect(await page.evaluate(() => window.__docs_island_csp)).toEqual([]);
  75  |     expect(audit.failures).toEqual([]);
  76  |     expect(audit.responses).toEqual([]);
  77  |     expect(audit.console).toEqual([]);
  78  |     expect(audit.pageErrors).toEqual([]);
  79  |     const origin = new URL(url("./")).origin;
  80  |     expect(audit.requests.filter(request => new URL(request).origin !== origin)).toEqual([]);
  81  | }
  82  | 
  83  | async function ready(page: Page, count: number) {
  84  |     await expect(islands(page)).toHaveCount(count);
  85  |     await expect(page.locator('[data-rustify-island][data-rd-island-state="ready"]')).toHaveCount(count);
  86  |     expect(await page.evaluate(() => window.__rustify_docs_islands?.state())).toBe("ready");
  87  |     for (const island of await islands(page).all()) {
  88  |         await expect(canvas(island)).toHaveJSProperty("inert", false);
  89  |         await expect(canvas(island)).toHaveJSProperty("disabled", false);
  90  |         await expect(canvas(island)).toHaveAttribute("aria-busy", "false");
  91  |         const metadata = JSON.parse((await island.getAttribute("data-rustify-island-metadata"))!);
  92  |         await expect(canvas(island).locator(":scope > [data-rustify-scope]")).toHaveAttribute("data-rustify-mounted", metadata.seed);
  93  |         await expect(island.locator("[data-rd-island-loading]")).toBeHidden();
  94  |         await expect(island.locator("[data-rd-island-error]")).toBeHidden();
  95  |         await expect(island.locator("[data-rd-island-reload]")).toBeHidden();
  96  |     }
  97  | }
  98  | 
  99  | // Hold the real resource before WebAssembly runs; every assertion keeps the
  100 | // server's actual nodes and uses the normal loader once the route is released.
  101 | async function holdAsset(page: Page, file: string) {
  102 |     let release!: (fail?: boolean) => void;
  103 |     let observed!: (value: string) => void;
  104 |     const released = new Promise<boolean>(resolve => { release = (fail = false) => resolve(fail); });
  105 |     const seen = new Promise<string>(resolve => { observed = resolve; });
  106 |     const routeHandler = async (route: Route) => {
  107 |         observed(route.request().url());
  108 |         if (await released) {
  109 |             await route.fulfill({ status: 503, contentType: "text/plain", body: "Injected island asset failure" });
  110 |         } else {
  111 |             await route.continue();
  112 |         }
  113 |     };
  114 |     await page.route(assetPattern(file), routeHandler);
  115 |     return { seen, release, remove: () => page.unroute(assetPattern(file), routeHandler) };
  116 | }
  117 | 
  118 | async function captureOriginals(page: Page) {
  119 |     const before = await page.evaluate(() => {
  120 |         const blocks = [...document.querySelectorAll("[data-rustify-island]")].map(block => {
  121 |             const field = block.querySelector("fieldset[data-rd-island-controls]")!;
  122 |             const root = field.firstElementChild!;
  123 |             const nodes: Node[] = [root];
  124 |             const walker = document.createTreeWalker(root, NodeFilter.SHOW_ALL);
  125 |             while (walker.nextNode()) nodes.push(walker.currentNode);
  126 |             const source = block.querySelector(".rd-demo-source");
  127 |             const controls = [...root.querySelectorAll<HTMLInputElement | HTMLTextAreaElement | HTMLSelectElement>("input,textarea,select")].map(node => ({
  128 |                 node, value: node.value, checked: node instanceof HTMLInputElement ? node.checked : null,
  129 |             }));
  130 |             return {
  131 |                 key: block.getAttribute("data-rustify-island")!, root, original: root.cloneNode(true) as Element,
  132 |                 nodes, ids: [...root.querySelectorAll("[id]")].map(node => [node, node.id] as [Element, string]),
  133 |                 controls, source,
  134 |                 statusHeight: block.querySelector("[data-rd-island-status]")!.getBoundingClientRect().height,
  135 |                 sourceTop: source ? source.getBoundingClientRect().top + scrollY : null,
  136 |             };
  137 |         });
  138 |         window.__docs_island_probe = { blocks, navigation: [...document.querySelectorAll("header a, .rd-header-nav a, .rd-sidebar a, .rd-pager a")] };
  139 |         return blocks.map(block => ({ key: block.key, nodes: block.nodes.length, html: block.original.outerHTML, values: block.controls.map(control => control.value) }));
  140 |     });
  141 |     await test.info().attach("original-ssr-nodes.json", { body: JSON.stringify(before, null, 2), contentType: "application/json" });
  142 | }
  143 | 
  144 | async function identity(page: Page) {
  145 |     const actual = await page.evaluate(() => window.__docs_island_probe!.blocks.map(snapshot => {
  146 |         const block = [...document.querySelectorAll("[data-rustify-island]")].find(block => block.getAttribute("data-rustify-island") === snapshot.key)!;
  147 |         const field = block.querySelector("fieldset[data-rd-island-controls]")!;
  148 |         return {
  149 |             key: snapshot.key, nodes: snapshot.nodes.length,
  150 |             sameRoot: field.firstElementChild === snapshot.root,
  151 |             retained: snapshot.nodes.filter(node => node.isConnected && snapshot.root.contains(node)).length,
  152 |             sameIds: snapshot.ids.every(([node, id]) => node.id === id),
  153 |             sameNativeValues: snapshot.controls.every(control => control.node.value === control.value && (!(control.node instanceof HTMLInputElement) || control.node.checked === control.checked)),
  154 |             sameSource: block.querySelector(".rd-demo-source") === snapshot.source,
  155 |             statusHeight: block.querySelector("[data-rd-island-status]")!.getBoundingClientRect().height,
  156 |             initialStatusHeight: snapshot.statusHeight,
  157 |             sourceTop: snapshot.source ? snapshot.source.getBoundingClientRect().top + scrollY : null,
  158 |             initialSourceTop: snapshot.sourceTop,
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
> 169 |         expect(block.sourceTop, block.key).toBe(block.initialSourceTop);
      |                                            ^ Error: island-909443d1f3-1
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
  259 |             await expect(canvas(island)).toBeDisabled();
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
```