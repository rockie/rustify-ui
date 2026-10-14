# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: docs-ssg/dev.spec.ts >> semantic diagnostics locate the source, preserve the home, and clear on repair
- Location: tests/docs-ssg/dev.spec.ts:146:5

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
      - heading "content/index.md" [level=2]
      - list:
        - listitem:
          - text: "4:7 Error: lang"
          - paragraph:
            - text: "lang:"
            - code: english
            - text: is not a language tag
          - paragraph:
            - text: use a language tag such as
            - code: en
            - text: ","
            - code: zh-CN
            - text: or
            - code: fr
- contentinfo:
  - navigation "Footer links":
    - link "Back to the home page":
      - /url: /docs/
- paragraph
- status: 1 error · nothing has built yet · revision 1
```

# Test source

```ts
  59  | }
  60  | 
  61  | /// A running `dev`.
  62  | class Dev {
  63  |     log = "";
  64  | 
  65  |     private constructor(
  66  |         readonly child: ChildProcess,
  67  |         readonly url: string,
  68  |     ) {}
  69  | 
  70  |     static start(site: Site, args: string[] = ["--port", "0"], env: Record<string, string> = {}): Promise<Dev> {
  71  |         const child = spawn(bin, ["dev", ...args], { cwd: site.root, env: { ...process.env, ...env } });
  72  |         let log = "";
  73  |         child.stderr!.on("data", (chunk) => (log += chunk));
  74  |         return new Promise((resolve, reject) => {
  75  |             let out = "";
  76  |             child.stdout!.on("data", (chunk) => {
  77  |                 out += chunk;
  78  |                 const match = /^dev: (\S+)\n/m.exec(out);
  79  |                 if (match) {
  80  |                     const dev = new Dev(child, match[1]);
  81  |                     child.stderr!.on("data", (more) => (dev.log += more));
  82  |                     dev.log = log;
  83  |                     resolve(dev);
  84  |                 }
  85  |             });
  86  |             child.on("exit", (code) => reject(new Error(`dev exited ${code}: ${out}${log}`)));
  87  |         });
  88  |     }
  89  | 
  90  |     async stop(): Promise<void> {
  91  |         if (this.child.exitCode !== null || this.child.signalCode !== null) return;
  92  |         const exited = new Promise((resolve) => this.child.on("exit", resolve));
  93  |         this.child.kill();
  94  |         await exited;
  95  |     }
  96  | }
  97  | 
  98  | async function freePort(): Promise<number> {
  99  |     return new Promise((resolve) => {
  100 |         const server = createServer();
  101 |         server.listen(0, "127.0.0.1", () => {
  102 |             const port = (server.address() as { port: number }).port;
  103 |             server.close(() => resolve(port));
  104 |         });
  105 |     });
  106 | }
  107 | 
  108 | let site: Site;
  109 | const running: Dev[] = [];
  110 | 
  111 | async function dev(args?: string[], env?: Record<string, string>): Promise<Dev> {
  112 |     const started = await Dev.start(site, args, env);
  113 |     running.push(started);
  114 |     return started;
  115 | }
  116 | 
  117 | test.beforeEach(() => {
  118 |     site = new Site();
  119 | });
  120 | 
  121 | test.afterEach(async () => {
  122 |     while (running.length) await running.pop()!.stop();
  123 |     site.remove();
  124 | });
  125 | 
  126 | /// The build result the page's panel last took in.
  127 | async function seen(page: Page): Promise<number> {
  128 |     return Number(await page.locator(".rd-dev").getAttribute("data-revision"));
  129 | }
  130 | 
  131 | async function nextResult(page: Page, after: number): Promise<void> {
  132 |     await expect.poll(() => seen(page)).toBeGreaterThan(after);
  133 | }
  134 | 
  135 | /// Marks the document, so a reload shows as the mark's absence.
  136 | async function mark(page: Page): Promise<void> {
  137 |     await page.evaluate(() => ((window as unknown as { __kept: boolean }).__kept = true));
  138 | }
  139 | 
  140 | async function kept(page: Page): Promise<boolean> {
  141 |     return page.evaluate(() => (window as unknown as { __kept?: boolean }).__kept === true);
  142 | }
  143 | 
  144 | const BROKEN = "[Broken](missing.md)";
  145 | 
  146 | test("semantic diagnostics locate the source, preserve the home, and clear on repair", async ({ page }) => {
  147 |     const original = site.read("content/index.md");
  148 |     const server = await dev();
  149 |     await page.goto(server.url);
  150 |     await mark(page);
  151 |     for (const [source, rule, location] of [
  152 |         ["---\ntitle: Home\nlayout: home\nlang: english\n---\n# Home\n", "lang", "4:7"],
  153 |         ["---\ntitle: Home\nlayout: home\n---\nIntro\n\n# Home\n", "multiple-h1", "7:1"],
  154 |         ["---\ntitle: Home\nlayout: home\n---\n<div><h1>Home</h1></div>\n", "multiple-h1", "5:6"],
  155 |         ["---\ntitle: Home\nlayout: home\n---\n<H1>Home</H1>\n", "multiple-h1", "5:1"],
  156 |     ]) {
  157 |         site.write("content/index.md", source);
  158 |         const panel = page.getByRole("dialog");
> 159 |         await expect(panel).toBeVisible();
      |                             ^ Error: expect(locator).toBeVisible() failed
  160 |         await expect(panel).toContainText(rule);
  161 |         await expect(panel.locator(".rd-dev-location")).toHaveText(location);
  162 |         await expect(panel.locator(".rd-dev-suggestion")).not.toBeEmpty();
  163 |         await expect(page.locator("h1")).toHaveText("Fixture Docs");
  164 |         expect(await kept(page)).toBe(true);
  165 |         site.write("content/index.md", original);
  166 |         await expect(panel).toBeHidden();
  167 |     }
  168 | });
  169 | 
  170 | test("a new site is readable at once: the home page and its two pages", async ({ page }) => {
  171 |     const target = path.join(site.dir, "new-site");
  172 |     execFileSync(bin, ["new", target]);
  173 |     const child = spawn(bin, ["dev", "--port", "0"], { cwd: target });
  174 |     const url = await new Promise<string>((resolve, reject) => {
  175 |         let out = "";
  176 |         child.stdout!.on("data", (chunk) => {
  177 |             out += chunk;
  178 |             const match = /^dev: (\S+)\n/m.exec(out);
  179 |             if (match) resolve(match[1]);
  180 |         });
  181 |         child.on("exit", (code) => reject(new Error(`dev exited ${code}`)));
  182 |     });
  183 |     try {
  184 |         await page.goto(url);
  185 |         await expect(page.getByRole("heading", { level: 1 })).toBeVisible();
  186 |         await page.locator(".rd-action-primary").click();
  187 |         await expect(page.getByRole("heading", { level: 1 })).toBeVisible();
  188 |         const first = page.url();
  189 |         const sidebar = page.locator(".rd-sidebar-wide a:not([aria-current])").first();
  190 |         await sidebar.click();
  191 |         await expect(page).not.toHaveURL(first);
  192 |         await expect(page.getByRole("heading", { level: 1 })).toBeVisible();
  193 |         // The template has no `site.url` yet: a warning, which waits behind
  194 |         // a marker rather than opening the panel.
  195 |         const marker = page.getByRole("button", { name: "1 warning" });
  196 |         await expect(marker).toBeVisible();
  197 |         await expect(page.getByRole("dialog")).toBeHidden();
  198 |         await marker.click();
  199 |         await expect(page.getByRole("dialog")).toContainText("rustify-docs.toml");
  200 |     } finally {
  201 |         child.kill();
  202 |     }
  203 | });
  204 | 
  205 | test("a save reloads the page it changed in place and leaves the others", async ({ context }) => {
  206 |     const server = await dev();
  207 |     const setup = await context.newPage();
  208 |     await setup.goto(`${server.url}guide/setup/`);
  209 |     await setup.getByRole("tab", { name: "Config" }).first().click();
  210 |     await setup.evaluate(() => window.scrollTo(0, 600));
  211 |     await expect.poll(() => setup.evaluate(() => window.scrollY)).toBe(600);
  212 |     await mark(setup);
  213 |     const api = await context.newPage();
  214 |     await api.goto(`${server.url}reference/api/`);
  215 |     await expect(api.locator(".rd-dev")).toHaveAttribute("data-revision", /\d+/);
  216 |     await mark(api);
  217 |     const before = await seen(api);
  218 | 
  219 |     site.edit("content/guide/setup.md", "Once installed, the same choice applies here:", "Once installed, the same choice still applies here:");
  220 |     await expect(setup.getByText("the same choice still applies here")).toBeVisible();
  221 |     expect(await kept(setup), "the changed page reloaded").toBe(false);
  222 |     await expect.poll(() => setup.evaluate(() => window.scrollY)).toBe(600);
  223 |     await expect(setup.getByRole("tab", { name: "Config" }).first()).toHaveAttribute("aria-selected", "true");
  224 |     await nextResult(api, before);
  225 |     expect(await kept(api), "an unchanged page stayed").toBe(true);
  226 | 
  227 |     // The configuration changes every page.
  228 |     site.edit("rustify-docs.toml", "Fixture Docs", "Fixture Handbook");
  229 |     await expect(api.locator(".rd-brand")).toContainText("Fixture Handbook");
  230 |     expect(await kept(api)).toBe(false);
  231 | });
  232 | 
  233 | test("a real symlink target save refreshes its image and survives deletion and recovery", async ({ context }) => {
  234 |     mkdirSync(path.join(site.root, "brand"));
  235 |     mkdirSync(path.join(site.root, "images"));
  236 |     const svg = (fill: string) => `<svg xmlns="http://www.w3.org/2000/svg" width="40" height="40"><rect width="40" height="40" fill="${fill}"/></svg>`;
  237 |     site.write("brand/real.svg", svg("red"));
  238 |     symlinkSync("../brand/real.svg", path.join(site.root, "images/mark.svg"));
  239 |     site.edit("content/guide/index.md", "Read the setup page first.", "Read the setup page first. ![Mark](../../images/mark.svg)");
  240 |     const server = await dev();
  241 |     const page = await context.newPage();
  242 |     const errors: string[] = [];
  243 |     page.on("pageerror", error => errors.push(error.message));
  244 |     await page.goto(`${server.url}guide/`);
  245 |     const image = page.getByRole("img", { name: "Mark", exact: true });
  246 |     const original = await image.getAttribute("src");
  247 |     const api = await context.newPage();
  248 |     await api.goto(`${server.url}reference/api/`);
  249 |     await expect(api.locator(".rd-dev")).toHaveAttribute("data-revision", /\d+/);
  250 |     await mark(api);
  251 |     const before = await seen(api);
  252 |     site.write("brand/replacement.svg", svg("blue"));
  253 |     renameSync(path.join(site.root, "brand/replacement.svg"), path.join(site.root, "brand/real.svg"));
  254 |     await expect(image).not.toHaveAttribute("src", original!);
  255 |     const blue = await image.getAttribute("src");
  256 |     expect(await (await page.request.get(new URL(blue!, page.url()).href)).text()).toContain('fill="blue"');
  257 |     await nextResult(api, before);
  258 |     expect(await kept(api), "the page without the image stays mounted").toBe(true);
  259 |     await mark(page);
```