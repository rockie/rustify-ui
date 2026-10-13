# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: docs-ssg/official.spec.ts >> CATALOG link uses its actual function, source and live control
- Location: tests/docs-ssg/official.spec.ts:232:33

# Error details

```
Error: expect(received).toBe(expected) // Object.is equality

Expected: "#rui-surface-a3376c2ec5-page-Link-demo-0-link-demo-destination-1"
Received: ""

Call Log:
- Timeout 10000ms exceeded while waiting on the predicate
```

# Page snapshot

```yaml
- generic [ref=e1]:
  - link "Skip to content" [ref=e2] [cursor=pointer]:
    - /url: "#content"
  - banner [ref=e3]:
    - generic [ref=e4]:
      - link "Rustify UI" [ref=e5] [cursor=pointer]:
        - /url: /rustify-ui/
      - navigation "Site links" [ref=e7]:
        - list [ref=e8]:
          - listitem [ref=e9]:
            - link "GitHub" [ref=e10] [cursor=pointer]:
              - /url: https://github.com/rockie/rustify-ui
      - group "Theme" [ref=e11]:
        - button "System" [pressed] [ref=e12] [cursor=pointer]
        - button "Light" [ref=e18] [cursor=pointer]
        - button "Dark" [ref=e24] [cursor=pointer]
  - generic [ref=e29]:
    - navigation "Menu" [ref=e31]:
      - generic [ref=e32]:
        - heading "Guides" [level=2] [ref=e33]
        - list [ref=e34]:
          - listitem [ref=e35]:
            - link "Quick start" [ref=e36] [cursor=pointer]:
              - /url: /rustify-ui/quickstart/
          - listitem [ref=e37]:
            - link "Architecture" [ref=e38] [cursor=pointer]:
              - /url: /rustify-ui/architecture/
          - listitem [ref=e39]:
            - link "Navigation" [ref=e40] [cursor=pointer]:
              - /url: /rustify-ui/navigation/
          - listitem [ref=e41]:
            - link "Workspace" [ref=e42] [cursor=pointer]:
              - /url: /rustify-ui/workspace/
          - listitem [ref=e43]:
            - link "Forms" [ref=e44] [cursor=pointer]:
              - /url: /rustify-ui/forms/
          - listitem [ref=e45]:
            - link "Languages, formatting and text" [ref=e46] [cursor=pointer]:
              - /url: /rustify-ui/i18n/
          - listitem [ref=e47]:
            - link "Data larger than the screen" [ref=e48] [cursor=pointer]:
              - /url: /rustify-ui/data/
          - listitem [ref=e49]:
            - link "Theme documents" [ref=e50] [cursor=pointer]:
              - /url: /rustify-ui/themes/
          - listitem [ref=e51]:
            - link "Runtime utilities" [ref=e52] [cursor=pointer]:
              - /url: /rustify-ui/utility/
          - listitem [ref=e53]:
            - link "Compatibility and capability boundaries (P1 research preview)" [ref=e54] [cursor=pointer]:
              - /url: /rustify-ui/compatibility/
          - listitem [ref=e55]:
            - link "Component catalogue" [ref=e56] [cursor=pointer]:
              - /url: /rustify-ui/components/
          - listitem [ref=e57]:
            - link "Vellum example and framework capabilities" [ref=e58] [cursor=pointer]:
              - /url: /rustify-ui/vellum/
      - generic [ref=e59]:
        - heading "Components" [level=2] [ref=e60]
        - list [ref=e61]:
          - listitem [ref=e62]:
            - link "Button" [ref=e63] [cursor=pointer]:
              - /url: /rustify-ui/components/button/
          - listitem [ref=e64]:
            - link "Label" [ref=e65] [cursor=pointer]:
              - /url: /rustify-ui/components/label/
          - listitem [ref=e66]:
            - link "Link" [ref=e67] [cursor=pointer]:
              - /url: /rustify-ui/components/link/
          - listitem [ref=e68]:
            - link "Icon" [ref=e69] [cursor=pointer]:
              - /url: /rustify-ui/components/icon/
          - listitem [ref=e70]:
            - link "Text field" [ref=e71] [cursor=pointer]:
              - /url: /rustify-ui/components/text-field/
          - listitem [ref=e72]:
            - link "Text area" [ref=e73] [cursor=pointer]:
              - /url: /rustify-ui/components/text-area/
          - listitem [ref=e74]:
            - link "Checkbox" [ref=e75] [cursor=pointer]:
              - /url: /rustify-ui/components/checkbox/
          - listitem [ref=e76]:
            - link "Radio" [ref=e77] [cursor=pointer]:
              - /url: /rustify-ui/components/radio/
          - listitem [ref=e78]:
            - link "Switch" [ref=e79] [cursor=pointer]:
              - /url: /rustify-ui/components/switch/
          - listitem [ref=e80]:
            - link "Select" [ref=e81] [cursor=pointer]:
              - /url: /rustify-ui/components/select/
          - listitem [ref=e82]:
            - link "Slider" [ref=e83] [cursor=pointer]:
              - /url: /rustify-ui/components/slider/
          - listitem [ref=e84]:
            - link "Progress" [ref=e85] [cursor=pointer]:
              - /url: /rustify-ui/components/progress/
          - listitem [ref=e86]:
            - link "Loading" [ref=e87] [cursor=pointer]:
              - /url: /rustify-ui/components/loading/
          - listitem [ref=e88]:
            - link "Tooltip" [ref=e89] [cursor=pointer]:
              - /url: /rustify-ui/components/tooltip/
          - listitem [ref=e90]:
            - link "Menu" [ref=e91] [cursor=pointer]:
              - /url: /rustify-ui/components/menu/
          - listitem [ref=e92]:
            - link "Dialog" [ref=e93] [cursor=pointer]:
              - /url: /rustify-ui/components/dialog/
          - listitem [ref=e94]:
            - link "Tabs" [ref=e95] [cursor=pointer]:
              - /url: /rustify-ui/components/tabs/
          - listitem [ref=e96]:
            - link "Scroll area" [ref=e97] [cursor=pointer]:
              - /url: /rustify-ui/components/scroll-area/
          - listitem [ref=e98]:
            - link "Data table" [ref=e99] [cursor=pointer]:
              - /url: /rustify-ui/components/data-table/
          - listitem [ref=e100]:
            - link "Tree" [ref=e101] [cursor=pointer]:
              - /url: /rustify-ui/components/tree/
          - listitem [ref=e102]:
            - link "Toast" [ref=e103] [cursor=pointer]:
              - /url: /rustify-ui/components/toast/
          - listitem [ref=e104]:
            - link "Number field" [ref=e105] [cursor=pointer]:
              - /url: /rustify-ui/components/number-field/
          - listitem [ref=e106]:
            - link "Color field" [ref=e107] [cursor=pointer]:
              - /url: /rustify-ui/components/color-field/
          - listitem [ref=e108]:
            - link "Toggle group" [ref=e109] [cursor=pointer]:
              - /url: /rustify-ui/components/toggle-group/
    - navigation "On this page" [ref=e111]:
      - paragraph [ref=e112]: On this page
      - list [ref=e113]:
        - listitem [ref=e114]:
          - link "Example" [ref=e115] [cursor=pointer]:
            - /url: "#example"
        - listitem [ref=e116]:
          - link "Capabilities" [ref=e117] [cursor=pointer]:
            - /url: "#capabilities"
        - listitem [ref=e118]:
          - link "GPU integration" [ref=e119] [cursor=pointer]:
            - /url: "#gpu-integration"
    - main [ref=e120]:
      - generic [ref=e121]:
        - heading "Link" [level=1] [ref=e122]
        - article [ref=e123]:
          - generic [ref=e124]:
            - paragraph [ref=e125]: href, exact or within matching, disabled; a disabled link keeps its place and loses its href
            - heading "Example" [level=2] [ref=e126]
            - generic [ref=e127]:
              - group "Interactive example" [ref=e128]:
                - generic [ref=e131]:
                  - generic [ref=e132]:
                    - link "Jump to the destination" [active] [ref=e133] [cursor=pointer]:
                      - /url: "#rui-surface-a3376c2ec5-page-Link-demo-0-link-demo-destination-1"
                    - button "Toggle availability" [ref=e134] [cursor=pointer]
                  - paragraph [ref=e135]: The link points to this paragraph.
                  - paragraph [ref=e136]:
                    - text: "Link:"
                    - status [ref=e137]: Available
              - status [ref=e138]:
                - generic [ref=e139]: Interactive example
              - generic [ref=e140]:
                - paragraph [ref=e141]: site/src/demos/basics.rs
                - generic [ref=e142]:
                  - generic [ref=e143]:
                    - tablist [ref=e144]:
                      - tab "Function" [selected] [ref=e145] [cursor=pointer]
                      - tab "Full file" [ref=e146] [cursor=pointer]
                    - button "Copy" [ref=e147] [cursor=pointer]
                  - tabpanel "Function" [ref=e152]:
                    - code [ref=e154]:
                      - generic [ref=e155]: /// A disabled link keeps its label and stops offering a destination.
                      - generic [ref=e159]:
                        - generic [ref=e160]:
                          - generic [ref=e161]: pub fn
                          - text: link_demo
                        - generic [ref=e162]:
                          - generic [ref=e163]: (
                          - generic [ref=e164]: )
                        - generic [ref=e166]: "-> impl IntoView"
                        - generic [ref=e168]: "{"
                      - generic [ref=e173]:
                        - text: let destination_id =
                        - generic [ref=e174]: "id::"
                        - text: next
                        - generic [ref=e175]:
                          - text: (
                          - generic [ref=e176]: "\"link-demo-destination\""
                        - generic [ref=e177]: )
                        - text: ;
                      - generic [ref=e181]:
                        - text: let disabled =
                        - generic [ref=e182]: "RwSignal::"
                        - text: new
                        - generic [ref=e183]: (false
                        - generic [ref=e184]: )
                        - text: ;
                      - generic [ref=e188]:
                        - text: view!
                        - generic [ref=e189]: "{"
                      - generic [ref=e194]:
                        - text: <div class=
                        - generic [ref=e195]: "\"space-y-4\""
                        - text: ">"
                      - generic [ref=e200]:
                        - text: <div class=
                        - generic [ref=e201]: "\"flex flex-wrap items-center gap-3\""
                        - text: ">"
                      - generic [ref=e206]:
                        - text: <Link href=format!
                        - generic [ref=e207]: (
                        - generic [ref=e208]: "\"#{destination_id}\""
                        - generic [ref=e210]: )
                        - text: disabled=disabled>
                        - generic [ref=e211]: "\"Jump to the destination\""
                        - text: </Link>
                      - generic [ref=e216]:
                        - text: <Button variant=
                        - generic [ref=e217]: "ButtonVariant::"
                        - text: Outline on_click=move || disabled.update
                        - generic [ref=e218]:
                          - text: (
                          - generic [ref=e219]: "|"
                          - generic [ref=e221]: disabled|
                          - generic [ref=e223]: "*disabled = !*disabled"
                        - generic [ref=e224]: )
                        - text: ">"
                      - generic [ref=e225]: "\"Toggle availability\""
                      - generic [ref=e231]: </Button>
                      - generic [ref=e236]: </div>
                      - generic [ref=e245]:
                        - text: <p id=destination_id tabindex=
                        - generic [ref=e246]: "\"-1\""
                        - text: ">"
                        - generic [ref=e247]: "\"The link points to this paragraph.\""
                        - text: </p>
                      - generic [ref=e252]:
                        - text: <p class=
                        - generic [ref=e253]: "\"text-sm\""
                        - text: ">"
                        - generic [ref=e254]: "\"Link: \""
                        - text: <output data-demo-state=
                        - generic [ref=e255]: "\"\""
                        - text: ">"
                        - generic [ref=e256]:
                          - text: "{move || if disabled.get"
                          - generic [ref=e257]: (
                          - generic [ref=e258]: )
                          - generic [ref=e259]:
                            - text: "{"
                            - generic [ref=e260]: "\"Unavailable\""
                          - generic [ref=e261]: "}"
                          - text: else
                          - generic [ref=e262]:
                            - text: "{"
                            - generic [ref=e263]: "\"Available\""
                          - generic [ref=e264]: "}"
                        - generic [ref=e265]: "}"
                        - text: </output></p>
                      - generic [ref=e266]: </div>
                      - generic [ref=e271]: "}"
                      - generic [ref=e276]: "}"
            - heading "Capabilities" [level=2] [ref=e280]
            - region "Component capabilities" [ref=e281]:
              - table [ref=e282]:
                - rowgroup [ref=e283]:
                  - row [ref=e284]:
                    - columnheader "Capability" [ref=e285]
                    - columnheader "Support" [ref=e286]
                    - columnheader "Details" [ref=e287]
                - rowgroup [ref=e288]:
                  - row [ref=e289]:
                    - rowheader "properties" [ref=e290]
                    - cell "yes" [ref=e291]
                    - cell "href, exact or within matching, disabled; a disabled link keeps its place and loses its href" [ref=e292]
                  - row [ref=e293]:
                    - rowheader "actions" [ref=e294]
                    - cell "yes" [ref=e295]
                    - cell "follows the link, and marks itself aria-current=page while the path is at or below its own" [ref=e296]
                  - row [ref=e297]:
                    - rowheader "theme" [ref=e298]
                    - cell "yes" [ref=e299]
                    - cell "primary and the focus ring" [ref=e300]
                  - row [ref=e301]:
                    - rowheader "input" [ref=e302]
                    - cell "yes" [ref=e303]
                    - cell "pointer and Enter, plus everything the browser gives a link" [ref=e304]
                  - row [ref=e305]:
                    - rowheader "accessibility" [ref=e306]
                    - cell "yes" [ref=e307]
                    - cell "a native anchor; the current one says so rather than only looking different" [ref=e308]
                  - row [ref=e309]:
                    - rowheader "environment" [ref=e310]
                    - cell "partial" [ref=e311]
                    - cell "the current path comes from the application until the SDK router arrives in M4; a region reports a click and the application navigates, because a region never touches history (C-5)" [ref=e312]
            - heading "GPU integration" [level=2] [ref=e313]
            - paragraph [ref=e314]:
              - text: "GPU drawing support: no. This documentation example uses the DOM component. See the"
              - link "component catalogue example" [ref=e315] [cursor=pointer]:
                - /url: https://github.com/rockie/rustify-ui/tree/main/examples/component-catalog
              - text: for the available DOM and GPU presentations.
        - link "Edit this page" [ref=e317] [cursor=pointer]:
          - /url: https://github.com/rockie/rustify-ui/edit/main/site/src/lib.rs
        - navigation "Previous / Next" [ref=e318]:
          - link "Previous Label" [ref=e319] [cursor=pointer]:
            - /url: /rustify-ui/components/label/
            - generic [ref=e320]: Previous
            - generic [ref=e321]: Label
          - link "Next Icon" [ref=e322] [cursor=pointer]:
            - /url: /rustify-ui/components/icon/
            - generic [ref=e323]: Next
            - generic [ref=e324]: Icon
  - contentinfo [ref=e325]:
    - navigation "Footer links" [ref=e326]:
      - link "Back to the home page" [ref=e327] [cursor=pointer]:
        - /url: /rustify-ui/
  - paragraph [ref=e328]
```

# Test source

```ts
  39  | const configuredGuides = [...guideItems.matchAll(/"([^"]+)"/g)].map(match => `docs/${match[1]}`);
  40  | const readme = readFileSync("README.md", "utf8");
  41  | const introduction = readme.slice(readme.indexOf("\n", readme.indexOf("# ")) + 1, readme.indexOf("\n## Quick start")).trim();
  42  | const introParagraphs = introduction.split(/\n\s*\n/).map(paragraph => paragraph
  43  |     .replace(/\[([^\]]+)\]\([^)]*\)/g, "$1").replace(/`([^`]+)`/g, "$1").replace(/\s+/g, " ").trim());
  44  | const repoQuickstart = "https://github.com/rockie/rustify-ui/blob/main/README.md#quick-start";
  45  | 
  46  | type Audit = { requests: string[]; failures: string[]; console: string[]; pageErrors: string[]; csp: string[]; responses: { url: string; status: number; mime: string }[]; expected404: Set<string>; expectedConsole: string[] };
  47  | declare global { interface Window { __official_csp(value: string): void; __official_first_theme?: { mode: string | null; background: string }; } }
  48  | 
  49  | async function watch(page: Page): Promise<Audit> {
  50  |     const audit: Audit = { requests: [], failures: [], console: [], pageErrors: [], csp: [], responses: [], expected404: new Set(), expectedConsole: [] };
  51  |     await page.exposeBinding("__official_csp", (_, value: string) => audit.csp.push(value));
  52  |     await page.addInitScript(() => document.addEventListener("securitypolicyviolation", event => window.__official_csp(`${event.violatedDirective} ${event.blockedURI}`)));
  53  |     page.on("request", request => audit.requests.push(request.url()));
  54  |     page.on("requestfailed", request => audit.failures.push(`${request.url()} ${request.failure()?.errorText}`));
  55  |     page.on("pageerror", error => audit.pageErrors.push(String(error)));
  56  |     page.on("response", response => audit.responses.push({ url: response.url(), status: response.status(), mime: response.headers()["content-type"] || "" }));
  57  |     page.on("console", message => {
  58  |         if (message.type() !== "error") return;
  59  |         if (audit.expected404.has(message.location().url) && message.text() === "Failed to load resource: the server responded with a status of 404 (Not Found)") audit.expectedConsole.push(message.text());
  60  |         else audit.console.push(message.text());
  61  |     });
  62  |     return audit;
  63  | }
  64  | 
  65  | const test = playwright.extend<{ audit: Audit }>({ audit: [async ({ page }, use, info) => {
  66  |     const audit = await watch(page);
  67  |     try { await use(audit); }
  68  |     finally { await info.attach("official-browser-audit.json", { body: JSON.stringify({ ...audit, expected404: [...audit.expected404] }, null, 2), contentType: "application/json" }); }
  69  | }, { auto: true }] });
  70  | 
  71  | const url = (route: string) => new URL(route, String(test.info().project.use.baseURL)).href;
  72  | const base = () => new URL(url("./")).pathname;
  73  | const wasm = (audit: Audit) => audit.requests.filter(request => /\.wasm(?:[?#]|$)/.test(request));
  74  | const island = (page: Page) => page.locator("[data-rustify-island]");
  75  | 
  76  | async function healthy(page: Page, audit: Audit) {
  77  |     await page.evaluate(() => document.fonts.ready);
  78  |     expect(audit.failures).toEqual([]);
  79  |     expect(audit.console).toEqual([]);
  80  |     expect(audit.pageErrors).toEqual([]);
  81  |     expect(audit.csp).toEqual([]);
  82  |     expect(audit.responses.filter(response => response.status >= 400 && !(response.status === 404 && audit.expected404.has(response.url)))).toEqual([]);
  83  |     expect(audit.requests.filter(request => new URL(request).origin !== new URL(url("./")).origin || /makepad/i.test(request))).toEqual([]);
  84  | }
  85  | 
  86  | async function open(page: Page, route: string) {
  87  |     const response = await page.goto(url(route));
  88  |     expect(response?.status()).toBe(200);
  89  |     await expect(page.locator("html")).toHaveAttribute("lang", "en");
  90  |     const policy = await page.locator('meta[http-equiv="Content-Security-Policy"]').getAttribute("content");
  91  |     expect(policy).toContain("script-src");
  92  |     expect(policy).not.toContain("frame-ancestors");
  93  |     expect(policy).not.toContain("'unsafe-inline'");
  94  |     if (test.info().project.name.endsWith("-meta")) expect(response!.headers()["content-security-policy"]).toBeUndefined();
  95  |     else expect(response!.headers()["content-security-policy"]).toContain("frame-ancestors");
  96  |     return response;
  97  | }
  98  | 
  99  | async function attached(page: Page, name: string) {
  100 |     await expect(island(page)).toHaveCount(1);
  101 |     await expect(island(page)).toHaveAttribute("data-rd-island-state", "ready");
  102 |     const metadata = JSON.parse((await island(page).getAttribute("data-rustify-island-metadata"))!);
  103 |     expect(metadata.target).toEqual({ type: "demo", name });
  104 |     expect(metadata.environment.base).toBe(base());
  105 |     expect(metadata.environment.lang).toBe("en");
  106 |     const field = island(page).locator("fieldset[data-rd-island-controls]");
  107 |     await expect(field).toHaveJSProperty("disabled", false);
  108 |     await expect(field).toHaveJSProperty("inert", false);
  109 |     await expect(field.locator(":scope > [data-rustify-scope]")).toHaveAttribute("data-rustify-mounted", metadata.seed);
  110 |     await expect(field.locator("[style]")).toHaveCount(0);
  111 |     expect(await page.evaluate(() => { const ids = [...document.querySelectorAll("[id]")].map(node => node.id); return ids.length === new Set(ids).size; })).toBe(true);
  112 |     await page.evaluate(() => document.fonts.ready);
  113 |     return field;
  114 | }
  115 | 
  116 | async function sourceText(page: Page, entry: typeof categories[number]) {
  117 |     const source = page.locator(".rd-demo-source");
  118 |     await expect(source.locator("[data-rd-source-path]")).toHaveText(entry.file);
  119 |     expect(await source.locator("template.rd-code-raw").evaluateAll(nodes => nodes.map(node => (node as HTMLTemplateElement).content.textContent))).toEqual([entry.function, entry.source]);
  120 |     return source;
  121 | }
  122 | 
  123 | async function fits(page: Page) {
  124 |     expect(await page.evaluate(() => document.documentElement.scrollWidth - innerWidth), "page-level horizontal overflow").toBeLessThanOrEqual(0);
  125 | }
  126 | 
  127 | async function exercise(page: Page, field: Locator, name: string) {
  128 |     const state = field.locator("[data-demo-state]");
  129 |     await expect(state).not.toBeEmpty();
  130 |     switch (name) {
  131 |         case "button":
  132 |             await expect(field.getByRole("button", { name: "Unavailable", exact: true })).toBeDisabled();
  133 |             await field.getByRole("button", { name: "Increment", exact: true }).focus(); await page.keyboard.press("Enter"); await expect(state).toHaveText("1"); break;
  134 |         case "label":
  135 |             await field.locator("label").click(); await expect(field.getByLabel("Project name", { exact: true })).toBeFocused();
  136 |             await field.getByLabel("Project name", { exact: true }).fill("Official reader"); await expect(state).toHaveText("Official reader"); break;
  137 |         case "link": {
  138 |             const link = field.getByRole("link", { name: "Jump to the destination", exact: true });
> 139 |             const destination = await link.getAttribute("href"); await link.click(); await expect.poll(() => new URL(page.url()).hash).toBe(destination);
      |                                                                                                                                        ^ Error: expect(received).toBe(expected) // Object.is equality
  140 |             await field.getByRole("button", { name: "Toggle availability", exact: true }).click(); await expect(state).toHaveText("Unavailable"); await expect(link).toHaveAttribute("aria-disabled", "true"); break;
  141 |         }
  142 |         case "icon": await field.getByRole("button", { name: "Change icon", exact: true }).click(); await expect(state).toHaveText("Close"); await expect(field.getByRole("img", { name: "Not selected", exact: true })).toBeVisible(); break;
  143 |         case "text field": {
  144 |             const input = field.getByLabel("Project name", { exact: true }); const hint = await input.getAttribute("aria-describedby");
  145 |             expect(hint).toBeTruthy(); await expect(field.locator(`[id="${hint}"]`)).toHaveText("Changes appear below as you type.");
  146 |             await input.fill("A real edit — résumé"); await expect(state).toHaveText("A real edit — résumé"); break;
  147 |         }
  148 |         case "text area": await field.getByLabel("Notes", { exact: true }).fill("First line\nSecond line — résumé"); await expect(state).toHaveText("First line\nSecond line — résumé"); break;
  149 |         case "checkbox": await field.getByRole("checkbox", { name: "Include this item", exact: true }).press("Space"); await expect(state).toHaveText("Excluded"); break;
  150 |         case "radio": await expect(field.getByRole("radio", { name: "Custom (unavailable)", exact: true })).toBeDisabled(); await field.getByRole("radio", { name: "Compact", exact: true }).press("Space"); await expect(state).toHaveText("compact"); break;
  151 |         case "switch": await field.getByRole("switch", { name: "Enable notifications", exact: true }).press("Space"); await expect(state).toHaveText("Enabled"); break;
  152 |         case "select": {
  153 |             const trigger = field.getByRole("combobox", { name: "Language", exact: true }); await trigger.click();
  154 |             await field.getByRole("option", { name: "JavaScript", exact: true }).click(); await expect(state).toHaveText("javascript"); await expect(field.getByRole("listbox")).toBeHidden(); await expect(trigger).toBeFocused();
  155 |             await trigger.click(); await page.keyboard.press("Escape"); await expect(field.getByRole("listbox")).toBeHidden(); await expect(trigger).toBeFocused(); break;
  156 |         }
  157 |         case "slider": await field.getByRole("slider", { name: "Value", exact: true }).press("ArrowRight"); await expect(state).toHaveText("30"); break;
  158 |         case "progress": await field.getByRole("button", { name: "Advance", exact: true }).press("Enter"); await expect(state).toHaveText("50"); await expect(field.getByRole("progressbar", { name: "Example progress", exact: true })).toHaveAttribute("aria-valuenow", "50"); break;
  159 |         case "loading": await field.getByRole("button", { name: "Toggle loading", exact: true }).click(); await expect(state).toHaveText("Ready"); break;
  160 |         case "tooltip": {
  161 |             const trigger = field.getByRole("button", { name: "Hover or focus for help", exact: true }); await trigger.focus();
  162 |             await expect(state).toHaveText("Open"); const tooltip = field.getByRole("tooltip"); await expect(tooltip).toBeVisible(); await expect(trigger).toHaveAttribute("aria-describedby", await tooltip.getAttribute("id") as string);
  163 |             await page.keyboard.press("Escape"); await expect(state).toHaveText("Closed"); await expect(trigger).toBeFocused(); break;
  164 |         }
  165 |         case "menu": {
  166 |             const trigger = field.getByRole("button", { name: "Open menu", exact: true }); await trigger.click(); await expect(field.getByRole("menu", { name: "Example actions", exact: true })).toBeVisible();
  167 |             await page.keyboard.press("End"); const item = field.getByRole("menuitem", { name: "Rename", exact: true }); await expect(item).toBeFocused(); await page.keyboard.press("Enter");
  168 |             await expect(state).toHaveText("rename"); await expect(field.getByRole("menu")).toBeHidden(); await expect(trigger).toBeFocused();
  169 |             await trigger.click(); await page.keyboard.press("Escape"); await expect(field.getByRole("menu")).toBeHidden(); await expect(trigger).toBeFocused(); break;
  170 |         }
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
  194 |             const toolbar = field.getByRole("toolbar", { name: "Text styles", exact: true }); await toolbar.getByRole("button", { name: "Bold", exact: true }).press("Space"); await expect(state).toHaveText("bold, italic"); await toolbar.getByRole("button", { name: "Bold", exact: true }).press("ArrowRight"); await expect(toolbar.getByRole("button", { name: "Italic", exact: true })).toBeFocused(); break;
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
```