# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: docs-ssg/official.spec.ts >> the system first frame and mode controls persist across static and island pages
- Location: tests/docs-ssg/official.spec.ts:259:64

# Error details

```
Error: expect(locator).toHaveAttribute(expected) failed

Locator:  locator('html')
Expected: "dark"
Received: "system"
Timeout:  10000ms

Call log:
  - Expect "toHaveAttribute" locator('html') with timeout 10000ms
  - waiting for locator('html')
    24 × locator resolved to <html lang="en" data-theme="system" data-rustify-scope="" data-rd-base="/rustify-ui/">…</html>
       - unexpected value "system"

```

```yaml
- document:
  - link "Skip to content":
    - /url: "#content"
  - banner
  - navigation "Menu"
  - navigation "On this page"
  - main
  - contentinfo
  - paragraph
```

# Test source

```ts
  171 |         case "dialog": {
  172 |             const trigger = field.getByRole("button", { name: "Open dialog", exact: true }); await trigger.click(); const dialog = field.getByRole("dialog", { name: "Confirm an action", exact: true }); await expect(dialog).toBeVisible();
  173 |             await page.keyboard.press("Tab"); expect(await dialog.evaluate(node => node.contains(document.activeElement))).toBe(true);
  174 |             await page.keyboard.press("Escape"); await expect(dialog).toBeHidden(); await expect(trigger).toBeFocused();
  175 |             await trigger.click(); await dialog.getByRole("button", { name: "Confirm", exact: true }).click(); await expect(state).toHaveText("Confirmed"); await expect(dialog).toBeHidden(); await expect(trigger).toBeFocused(); break;
  176 |         }
  177 |         case "tabs": await field.getByRole("tab", { name: "Overview", exact: true }).press("ArrowRight"); await expect(field.getByRole("tab", { name: "Details", exact: true })).toBeFocused(); await page.keyboard.press("Enter"); await expect(state).toHaveText("details"); await expect(field.getByRole("tabpanel", { name: "Details", exact: true })).toBeVisible(); break;
  178 |         case "scroll area": {
  179 |             await field.getByRole("button", { name: "Add item", exact: true }).click(); await expect(state).toHaveText("9");
  180 |             const area = field.getByRole("group", { name: "Example items", exact: true }); await area.focus(); await expect(area).toBeFocused(); await area.press("End"); await expect.poll(() => area.evaluate(node => node.scrollTop)).toBeGreaterThan(0); break;
  181 |         }
  182 |         case "data table": {
  183 |             const grid = field.getByRole("grid", { name: "Example components", exact: true }); const cell = grid.getByRole("gridcell").filter({ hasText: /^Button$/ }); await cell.focus(); await page.keyboard.press("Space"); await expect(state).toHaveText("Button"); await page.keyboard.press("Enter"); await expect(field.getByText("Activated: Button", { exact: true })).toBeVisible(); break;
  184 |         }
  185 |         case "tree": {
  186 |             const tree = field.getByRole("tree", { name: "Example documentation tree", exact: true }); const parent = tree.getByRole("treeitem").filter({ hasText: /Components$/ });
  187 |             await parent.focus(); await page.keyboard.press("ArrowDown"); const item = tree.getByRole("treeitem").filter({ hasText: /Button$/ }); await expect(item).toBeFocused(); await page.keyboard.press("Enter"); await expect(state).toHaveText("button");
  188 |             await page.keyboard.press("ArrowLeft"); await expect(parent).toBeFocused(); await page.keyboard.press("ArrowLeft"); await expect(parent).toHaveAttribute("aria-expanded", "false"); break;
  189 |         }
  190 |         case "toast": await field.getByRole("button", { name: "Show notification", exact: true }).click(); await expect(state).toHaveText("The example action completed."); await field.locator('[data-name="ToastDismiss"]').click(); await expect(state).toHaveText("None"); break;
  191 |         case "number field": await field.getByLabel("Amount", { exact: true }).press("ArrowUp"); await expect(state).toHaveText("13"); break;
  192 |         case "color field": await field.getByLabel("Color", { exact: true }).fill("ff000080"); await field.getByLabel("Color", { exact: true }).press("Enter"); await expect(state).toHaveText("#ff000080"); await expect(field.getByTestId("color-demo-swatch")).toHaveCSS("background-color", "rgba(255, 0, 0, 0.5)"); break;
  193 |         case "toggle group": {
  194 |             const toolbar = field.getByRole("toolbar", { name: "Text styles", exact: true }); await toolbar.getByRole("button", { name: "Bold", exact: true }).press("Space"); await expect(state).toHaveText("italic, bold"); await toolbar.getByRole("button", { name: "Bold", exact: true }).press("ArrowRight"); await expect(toolbar.getByRole("button", { name: "Italic", exact: true })).toBeFocused(); break;
  195 |         }
  196 |         default: throw new Error(`CATALOG ${name} needs an actual interaction probe`);
  197 |     }
  198 | }
  199 | 
  200 | test("home has one English title, the original README introduction and two first-viewport ordinary actions", async ({ page, audit }) => {
  201 |     expect(categories).toHaveLength(24); expect(guides).toHaveLength(12);
  202 |     expect(configuredRoutes).toEqual(categories.map(entry => `/${entry.route}`)); expect(configuredGuides).toEqual(guides.map(entry => entry.file));
  203 |     await page.setViewportSize({ width: 1280, height: 900 }); await open(page, "./");
  204 |     await expect(page.locator("h1")).toHaveCount(1); await expect(page.locator("h1")).toHaveText("Rustify UI");
  205 |     await expect(page.locator("[data-official-home]")).toHaveCount(1); await expect(page.locator(".rd-sidebar")).toHaveCount(0);
  206 |     expect((await page.locator(".rd-home-prose p").allTextContents()).map(text => text.replace(/\s+/g, " ").trim())).toEqual(introParagraphs);
  207 |     const quickstart = page.getByRole("navigation", { name: "Start reading", exact: true }).getByRole("link", { name: "Quick start", exact: true });
  208 |     const components = page.getByRole("navigation", { name: "Start reading", exact: true }).getByRole("link", { name: "Components", exact: true });
  209 |     await expect(quickstart).toHaveAttribute("href", new URL(url("quickstart/")).pathname); await expect(components).toHaveAttribute("href", new URL(url(categories[0].route)).pathname);
  210 |     for (const action of [quickstart, components]) { const box = (await action.boundingBox())!; expect(box.y).toBeGreaterThanOrEqual(0); expect(box.y + box.height).toBeLessThanOrEqual(900); }
  211 |     await expect(page.locator(".rd-home-prose").getByRole("link", { name: "SDK quick start", exact: true })).toHaveAttribute("href", new URL(url("quickstart/")).pathname);
  212 |     await expect(page.locator(".rd-home-prose").getByRole("link", { name: "repository quick start", exact: true })).toHaveAttribute("href", repoQuickstart);
  213 |     await expect(island(page)).toHaveCount(0); expect(wasm(audit)).toEqual([]); await healthy(page, audit);
  214 |     await quickstart.click(); await expect(page.locator("h1")).toHaveText(guides[0].title); expect(wasm(audit)).toEqual([]);
  215 | });
  216 | 
  217 | test("the ordinary sidebar contains the exact twelve guides and CATALOG pages in configured order", async ({ page, audit }) => {
  218 |     await open(page, categories[0].route); await attached(page, categories[0].demo);
  219 |     const links = page.locator(".rd-sidebar-wide a");
  220 |     expect(await links.evaluateAll(nodes => nodes.map(node => node.getAttribute("href")))).toEqual([...guides.map(entry => new URL(url(entry.route)).pathname), ...categories.map(entry => new URL(url(entry.route)).pathname)]);
  221 |     await expect(links).toHaveCount(36);
  222 |     await expect(page.locator(".rd-sidebar-wide a[aria-current=page]")).toHaveAttribute("href", new URL(url(categories[0].route)).pathname);
  223 |     await healthy(page, audit);
  224 | });
  225 | 
  226 | for (const guide of guides) test(`guide ${guide.route} is English, reachable and requests no wasm`, async ({ page, audit }) => {
  227 |     await open(page, guide.route); await expect(page.locator("h1")).toHaveCount(1); await expect(page.locator("h1")).toHaveText(guide.title);
  228 |     await expect(page.locator(".rd-sidebar-wide a[aria-current=page]")).toHaveAttribute("href", new URL(url(guide.route)).pathname);
  229 |     await expect(island(page)).toHaveCount(0); expect(wasm(audit)).toEqual([]); await healthy(page, audit);
  230 | });
  231 | 
  232 | for (const entry of categories) test(`CATALOG ${entry.name} uses its actual function, source and live control`, async ({ page, audit }) => {
  233 |     await open(page, entry.route); await expect(page.locator("h1")).toHaveCount(1); await expect(page.locator("h1")).toHaveText(entry.title);
  234 |     await expect(page.locator("[data-component-category]")).toHaveAttribute("data-component-category", entry.name);
  235 |     const field = await attached(page, entry.demo); const source = await sourceText(page, entry);
  236 |     await source.getByRole("tab", { name: "Full file", exact: true }).click(); await expect(source.getByRole("tab", { name: "Full file", exact: true })).toHaveAttribute("aria-selected", "true");
  237 |     await source.getByRole("tab", { name: "Function", exact: true }).click(); await expect(source.getByRole("tab", { name: "Function", exact: true })).toHaveAttribute("aria-selected", "true");
  238 |     await exercise(page, field, entry.name); expect(new Set(wasm(audit)).size).toBe(1);
  239 |     expect(audit.responses.filter(response => /\.wasm$/.test(response.url)).every(response => response.status === 200 && response.mime.split(";")[0] === "application/wasm")).toBe(true);
  240 |     await expect(page.locator("[data-component-category]").getByRole("link", { name: "component catalogue example", exact: true })).toHaveAttribute("href", "https://github.com/rockie/rustify-ui/tree/main/examples/component-catalog");
  241 |     await healthy(page, audit);
  242 | });
  243 | 
  244 | test("without script every official demo keeps its source and native disabled initial view", async ({ browser }) => {
  245 |     test.setTimeout(180_000); const context = await browser.newContext({ javaScriptEnabled: false, viewport: { width: 375, height: 812 } });
  246 |     const page = await context.newPage(); const audit = await watch(page);
  247 |     try {
  248 |         for (const entry of categories) {
  249 |             await open(page, entry.route); const field = island(page).locator("fieldset[data-rd-island-controls]"); await expect(field).toHaveJSProperty("disabled", true);
  250 |             await expect(island(page).locator("noscript span")).toBeVisible(); await expect(field.locator("[data-demo-state]")).not.toBeEmpty(); await sourceText(page, entry);
  251 |             expect(await field.locator("input,button,textarea,select").evaluateAll(nodes => nodes.every(node => node.matches(":disabled")))).toBe(true);
  252 |             await fits(page);
  253 |         }
  254 |         expect(wasm(audit)).toEqual([]); await healthy(page, audit);
  255 |         await test.info().attach("official-no-script-audit.json", { body: JSON.stringify({ ...audit, expected404: [] }, null, 2), contentType: "application/json" });
  256 |     } finally { await context.close(); }
  257 | });
  258 | 
  259 | for (const preference of ["light", "dark", "system"] as const) test(`the ${preference} first frame and mode controls persist across static and island pages`, async ({ page, audit }) => {
  260 |     await page.emulateMedia({ colorScheme: "dark" });
  261 |     await page.addInitScript(({ key, preference }) => {
  262 |         localStorage.setItem(key, preference); document.addEventListener("DOMContentLoaded", () => { window.__official_first_theme = { mode: document.documentElement.getAttribute("data-theme"), background: getComputedStyle(document.body).backgroundColor }; });
  263 |     }, { key: `rustify-docs:mode:${base()}`, preference });
  264 |     await open(page, "./"); await expect(page.locator("html")).toHaveAttribute("data-theme", preference);
  265 |     expect(await page.evaluate(() => window.__official_first_theme?.mode)).toBe(preference);
  266 |     const initial = await page.evaluate(() => window.__official_first_theme!.background);
  267 |     expect(await page.locator("body").evaluate(node => getComputedStyle(node).backgroundColor)).toBe(initial);
  268 |     await page.locator('[data-rd-mode="light"]').press("Enter"); const light = await page.locator("body").evaluate(node => getComputedStyle(node).backgroundColor);
  269 |     await page.locator('[data-rd-mode="dark"]').press("Enter"); const dark = await page.locator("body").evaluate(node => getComputedStyle(node).backgroundColor); expect(light).not.toBe(dark);
  270 |     await page.getByRole("navigation", { name: "Start reading", exact: true }).getByRole("link", { name: "Components", exact: true }).click(); await attached(page, categories[0].demo);
> 271 |     await expect(page.locator("html")).toHaveAttribute("data-theme", "dark"); await expect(page.locator('[data-rd-mode="dark"]')).toHaveAttribute("aria-pressed", "true"); await healthy(page, audit);
      |                                        ^ Error: expect(locator).toHaveAttribute(expected) failed
  272 | });
  273 | 
  274 | for (const layout of [{ name: "375px", width: 375, scaleText: false }, { name: "320px", width: 320, scaleText: false }, { name: "200% layout and text emulation", width: 640, scaleText: true }]) test(`${layout.name} retains reading controls, ordered narrow navigation and bounded long source`, async ({ page, audit }) => {
  275 |     await page.setViewportSize({ width: layout.width, height: 800 }); await page.emulateMedia({ reducedMotion: "reduce" });
  276 |     const entry = categories.find(entry => entry.name === "data table")!;
  277 |     for (const route of ["./", guides[0].route, entry.route]) {
  278 |         await open(page, route);
  279 |         if (layout.scaleText) await page.evaluate(async () => { document.documentElement.style.fontSize = "200%"; await Promise.all(document.getAnimations().filter(animation => animation instanceof CSSTransition).map(animation => animation.finished)); });
  280 |         if (route === entry.route) {
  281 |             await attached(page, entry.demo); const source = await sourceText(page, entry); await source.getByRole("tab", { name: "Full file", exact: true }).click();
  282 |             const pre = source.locator('.rd-code:not([hidden]) .rd-code-pre'); await pre.focus(); await expect(pre).toBeFocused();
  283 |             const size = await pre.evaluate(node => ({ client: node.clientHeight, content: node.scrollHeight, limit: 24 * parseFloat(getComputedStyle(document.documentElement).fontSize) })); expect(size.client).toBeLessThanOrEqual(size.limit); expect(size.content).toBeGreaterThan(size.client);
  284 |         }
  285 |         if (route !== "./") {
  286 |             const details = page.locator(".rd-sidebar-narrow"); await details.locator(":scope > summary").click(); await expect(details).toHaveAttribute("open", "");
  287 |             expect(await details.locator("a").evaluateAll(nodes => nodes.map(node => node.getAttribute("href")))).toEqual([...guides.map(entry => new URL(url(entry.route)).pathname), ...categories.map(entry => new URL(url(entry.route)).pathname)]);
  288 |             await details.locator(":scope > summary").click();
  289 |         }
  290 |         await fits(page); await page.evaluate(() => document.fonts.ready);
  291 |     }
  292 |     await healthy(page, audit);
  293 | });
  294 | 
  295 | test("deep not-found retains same-base ordinary navigation", async ({ page, audit }) => {
  296 |     const missing = url("unavailable/deeper/page/"); audit.expected404.add(missing); const response = await page.goto(missing); expect(response?.status()).toBe(404);
  297 |     await expect(page.locator("h1")).toHaveText("Page not found"); await expect(page.locator("html")).toHaveAttribute("lang", "en"); await expect(island(page)).toHaveCount(0); expect(wasm(audit)).toEqual([]);
  298 |     const home = page.getByRole("contentinfo").getByRole("link", { name: "Back to the home page", exact: true }); await expect(home).toHaveAttribute("href", base()); await home.click(); await expect(page).toHaveURL(url("./")); await healthy(page, audit);
  299 | });
  300 | 
  301 | test("one bounded official home, category and guide capture batch records desktop light/dark and narrow reading", async ({ page, audit }) => {
  302 |     test.setTimeout(120_000); const entry = categories.find(entry => entry.name === "data table")!;
  303 |     for (const [kind, route] of [["home", "./"], ["category", entry.route], ["guide", guides[0].route]] as const) {
  304 |         for (const variant of [{ name: "desktop-light", width: 1280, height: 900, mode: "light" }, { name: "desktop-dark", width: 1280, height: 900, mode: "dark" }, { name: "narrow-light", width: 375, height: 812, mode: "light" }]) {
  305 |             await page.setViewportSize({ width: variant.width, height: variant.height }); await open(page, route);
  306 |             if (kind === "category") await attached(page, entry.demo);
  307 |             await page.locator(`[data-rd-mode="${variant.mode}"]`).click(); await page.evaluate(async () => { await document.fonts.ready; await Promise.allSettled(document.getAnimations().filter(animation => animation instanceof CSSTransition).map(animation => animation.finished)); window.scrollTo(0, 0); });
  308 |             await fits(page); await test.info().attach(`official-${kind}-${variant.name}.png`, { body: await page.screenshot(), contentType: "image/png" });
  309 |         }
  310 |     }
  311 |     await healthy(page, audit);
  312 | });
  313 | 
```