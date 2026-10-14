# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: docs-ssg/dev.spec.ts >> paired SVG template preserves document anchors and cross-page navigation
- Location: tests/docs-ssg/dev.spec.ts:414:9

# Error details

```
Error: expect(received).toEqual(expected) // deep equality

- Expected  - 1
+ Received  + 3

- Array []
+ Array [
+   "http://127.0.0.1:64471/__rustify-docs/events?path=%2Fdocs%2F&good=0&session=1913-18de3830ec5f7688: net::ERR_ABORTED",
+ ]
```

# Page snapshot

```yaml
- generic [active] [ref=f2e1]:
  - link "Skip to content" [ref=f2e2] [cursor=pointer]:
    - /url: "#content"
  - banner [ref=f2e3]:
    - generic [ref=f2e4]:
      - link "Fixture Docs" [ref=f2e5] [cursor=pointer]:
        - /url: /docs/
      - navigation "Site links" [ref=f2e7]:
        - list [ref=f2e8]:
          - listitem [ref=f2e9]:
            - link "Guide" [ref=f2e10] [cursor=pointer]:
              - /url: /docs/guide/
          - listitem [ref=f2e11]:
            - link "GitHub" [ref=f2e12] [cursor=pointer]:
              - /url: https://github.com/example/fixture
        - list [ref=f2e13]:
          - listitem [ref=f2e14]:
            - link "Mastodon" [ref=f2e15] [cursor=pointer]:
              - /url: https://mastodon.example/@fixture
      - group "Theme" [ref=f2e16]:
        - button "System" [pressed] [ref=f2e17] [cursor=pointer]
        - button "Light" [ref=f2e23] [cursor=pointer]
        - button "Dark" [ref=f2e29] [cursor=pointer]
  - generic [ref=f2e34]:
    - navigation "Menu" [ref=f2e36]:
      - generic [ref=f2e37]:
        - heading "Guide" [level=2] [ref=f2e38]
        - list [ref=f2e39]:
          - listitem [ref=f2e40]:
            - link "Guide" [ref=f2e41] [cursor=pointer]:
              - /url: /docs/guide/
          - listitem [ref=f2e42]:
            - link "Setup" [ref=f2e43] [cursor=pointer]:
              - /url: /docs/guide/setup/
          - listitem [ref=f2e44]:
            - link "Later · Draft" [ref=f2e45] [cursor=pointer]:
              - /url: /docs/guide/later/
      - generic [ref=f2e46]:
        - heading "Reference" [level=2] [ref=f2e47]
        - list [ref=f2e48]:
          - listitem [ref=f2e49]:
            - link "API 参考" [ref=f2e50] [cursor=pointer]:
              - /url: /docs/reference/api/
      - generic [ref=f2e51]:
        - heading "Anchors" [level=2] [ref=f2e52]
        - list [ref=f2e53]:
          - listitem [ref=f2e54]:
            - link "Foreign anchors" [ref=f2e55] [cursor=pointer]:
              - /url: /docs/
          - listitem [ref=f2e56]:
            - link "Foreign anchors" [ref=f2e57] [cursor=pointer]:
              - /url: /docs/other/
    - navigation "On this page" [ref=f2e59]:
      - paragraph [ref=f2e60]: On this page
      - list [ref=f2e61]:
        - listitem [ref=f2e62]:
          - link "Visible" [ref=f2e63] [cursor=pointer]:
            - /url: "#visible"
    - main [ref=f2e64]:
      - generic [ref=f2e65]:
        - article [ref=f2e66]:
          - heading "Foreign anchors" [level=1] [ref=f2e67]
          - paragraph [ref=f2e68]
          - heading "Visible" [level=2] [ref=f2e70]
          - paragraph [ref=f2e71]:
            - link "Visible" [ref=f2e72] [cursor=pointer]:
              - /url: "#visible"
            - link "Other visible" [ref=f2e73] [cursor=pointer]:
              - /url: /docs/other/#visible
          - paragraph [ref=f2e74]:
            - link "svg-holder" [ref=f2e75] [cursor=pointer]:
              - /url: "#svg-holder"
            - link "Other svg-holder" [ref=f2e76] [cursor=pointer]:
              - /url: /docs/other/#svg-holder
            - link "circle" [ref=f2e77] [cursor=pointer]:
              - /url: "#circle"
            - link "Other circle" [ref=f2e78] [cursor=pointer]:
              - /url: /docs/other/#circle
        - navigation "Previous / Next" [ref=f2e79]:
          - link "Previous Foreign anchors" [ref=f2e80] [cursor=pointer]:
            - /url: /docs/
            - generic [ref=f2e81]: Previous
            - generic [ref=f2e82]: Foreign anchors
  - contentinfo [ref=f2e83]:
    - navigation "Footer links" [ref=f2e84]:
      - link "Back to the home page" [ref=f2e85] [cursor=pointer]:
        - /url: /docs/
  - paragraph [ref=f2e86]
  - status [ref=f2e87]
```

# Test source

```ts
  335 |         expect(background, classes[i]).not.toBe("none");
  336 |         const url = background.match(/^url\("(.*)"\)$/)![1];
  337 |         const response = await page.request.get(url);
  338 |         expect(response.ok(), classes[i]).toBe(true);
  339 |         expect(await response.text()).toContain(`fill="${[1, 2, 3].includes(i) ? "blue" : "red"}"`);
  340 |     }
  341 |     expect(errors).toEqual([]);
  342 |     await test.info().attach("resources.png", { body: await page.screenshot(), contentType: "image/png" });
  343 | });
  344 | 
  345 | test("template contents are rejected as anchors while their holder and ordinary IDs navigate", async ({ page }) => {
  346 |     site.write("rustify-docs.toml", `${site.read("rustify-docs.toml")}\n[[nav.groups]]\nlabel = "Anchors"\nitems = ["index.md", "other.md"]\n`);
  347 |     const body = '---\ndescription: Template anchors\n---\n# Templates\n\n<template id="holder"><span id="ghost">Hidden</span><template id="nested"><b id="deep">Hidden</b></template></template>\n\n<p id="visible">Visible</p>\n\n[Holder](#holder) [Visible](#visible) [Other holder](other.md#holder)\n';
  348 |     site.write("content/index.md", `${body}\n[Ghost](#ghost) [Deep](other.md#deep)\n`);
  349 |     site.write("content/other.md", body);
  350 |     const errors: string[] = [];
  351 |     page.on("pageerror", (error) => errors.push(error.message));
  352 |     const server = await dev();
  353 |     await page.goto(server.url);
  354 |     await expect(page.getByRole("region", { name: "2 errors · nothing has built yet" })).toContainText("broken-anchor");
  355 |     site.write("content/index.md", body);
  356 |     await expect(page.getByRole("heading", { name: "Templates", exact: true })).toBeVisible();
  357 |     expect(await page.evaluate(() => ({
  358 |         holder: document.getElementById("holder")?.tagName,
  359 |         ghost: document.getElementById("ghost"),
  360 |         nested: document.getElementById("nested"),
  361 |         deep: document.getElementById("deep"),
  362 |         stored: document.querySelector<HTMLTemplateElement>("#holder")!.content.getElementById("ghost")?.tagName,
  363 |     }))).toEqual({ holder: "TEMPLATE", ghost: null, nested: null, deep: null, stored: "SPAN" });
  364 |     await page.getByRole("link", { name: "Visible", exact: true }).click();
  365 |     expect(new URL(page.url()).hash).toBe("#visible");
  366 |     await page.getByRole("link", { name: "Holder", exact: true }).click();
  367 |     expect(new URL(page.url()).hash).toBe("#holder");
  368 |     await page.getByRole("link", { name: "Other holder", exact: true }).click();
  369 |     await expect(page).toHaveURL(/\/other\/#holder$/);
  370 |     expect(await page.locator("#holder").count()).toBe(1);
  371 |     expect(errors).toEqual([]);
  372 | });
  373 | 
  374 | for (const base of ["/", "/docs/"]) {
  375 |     test(`body anchors at ${base} navigate on the same and other pages and recover from ghost links`, async ({ page }) => {
  376 |         site.edit("rustify-docs.toml", 'url = "https://example.com/docs/"', `url = "https://example.com${base}"`);
  377 |         site.edit("rustify-docs.toml", 'base = "/docs/"', `base = "${base}"`);
  378 |         site.write("rustify-docs.toml", `${site.read("rustify-docs.toml")}\n[[nav.groups]]\nlabel = "Anchors"\nitems = ["index.md", "other.md"]\n`);
  379 |         const body = '---\ndescription: Body anchor\n---\n# Body anchor\n\n<body id="target">\n\nText\n\n</body>\n\n<template id="holder"><body id="ghost"><span id="hidden">Hidden</span></template>\n\n[Target](#target) [Other target](other.md#target)\n';
  380 |         site.write("content/index.md", body);
  381 |         site.write("content/other.md", body);
  382 |         const errors: string[] = [];
  383 |         page.on("pageerror", (error) => errors.push(error.message));
  384 |         page.on("console", (message) => { if (message.type() === "error") errors.push(message.text()); });
  385 |         page.on("requestfailed", (request) => errors.push(`${request.url()}: ${request.failure()?.errorText}`));
  386 |         const server = await dev();
  387 |         await page.goto(server.url);
  388 |         await expect(page.getByRole("heading", { name: "Body anchor", exact: true })).toBeVisible();
  389 |         expect(await page.evaluate(() => ({
  390 |             target: document.getElementById("target") === document.body,
  391 |             ghost: document.getElementById("ghost"),
  392 |             hidden: document.getElementById("hidden"),
  393 |             stored: document.querySelector<HTMLTemplateElement>("#holder")!.content.getElementById("hidden")?.tagName,
  394 |         }))).toEqual({ target: true, ghost: null, hidden: null, stored: "SPAN" });
  395 |         await page.locator("article").getByRole("link", { name: "Target", exact: true }).click();
  396 |         expect(new URL(page.url()).hash).toBe("#target");
  397 |         await page.locator("article").getByRole("link", { name: "Other target", exact: true }).click();
  398 |         await expect(page).toHaveURL(new RegExp(`${base}other/#target$`));
  399 |         expect(await page.locator("body").getAttribute("id")).toBe("target");
  400 |         site.write("content/index.md", `${body}\n[Ghost](#ghost) [Other ghost](other.md#ghost)\n`);
  401 |         await expect(page.getByRole("dialog", { name: "2 errors · showing the last good version" })).toContainText("broken-anchor");
  402 |         site.write("content/index.md", body);
  403 |         await expect(page.getByRole("dialog", { name: /errors ·/ })).toHaveCount(0);
  404 |         expect(errors).toEqual([]);
  405 |         await test.info().attach("body-anchors.png", { body: await page.screenshot(), contentType: "image/png" });
  406 |     });
  407 | }
  408 | 
  409 | for (const [name, markup, ids] of [
  410 |     ["self-closing SVG template", '<svg><template id="svg-holder"/></svg>', ["svg-holder"]],
  411 |     ["paired SVG template", '<svg><template id="svg-holder"><circle id="circle" r="10"/></template></svg>', ["svg-holder", "circle"]],
  412 |     ["SVG inside an HTML template", '<template id="holder"><svg><template id="ghost"/></svg></template>', ["holder"]],
  413 | ] as const) {
  414 |     test(`${name} preserves document anchors and cross-page navigation`, async ({ page }) => {
  415 |         site.write("rustify-docs.toml", `${site.read("rustify-docs.toml")}\n[[nav.groups]]\nlabel = "Anchors"\nitems = ["index.md", "other.md"]\n`);
  416 |         const body = `---\ndescription: Foreign anchors\n---\n# Foreign anchors\n\n${markup}\n\n## Visible\n\n[Visible](#visible) [Other visible](other.md#visible)\n\n${ids.map((id) => `[${id}](#${id}) [Other ${id}](other.md#${id})`).join("\n")}\n`;
  417 |         site.write("content/index.md", body);
  418 |         site.write("content/other.md", body);
  419 |         const errors: string[] = [];
  420 |         page.on("pageerror", (error) => errors.push(error.message));
  421 |         page.on("console", (message) => { if (message.type() === "error") errors.push(message.text()); });
  422 |         page.on("requestfailed", (request) => errors.push(`${request.url()}: ${request.failure()?.errorText}`));
  423 |         const server = await dev();
  424 |         await page.goto(server.url);
  425 |         await expect(page.getByRole("heading", { name: "Visible", exact: true })).toBeVisible();
  426 |         expect(await page.locator("article [id]").evaluateAll((nodes) => nodes.map((node) => node.id))).toEqual(["foreign-anchors", ...ids, "visible"]);
  427 |         if (ids.some((id) => id === "svg-holder")) {
  428 |             expect(await page.locator("#svg-holder").evaluate((node) => ({ namespace: node.namespaceURI, content: "content" in node }))).toEqual({ namespace: "http://www.w3.org/2000/svg", content: false });
  429 |         }
  430 |         await page.locator("article").getByRole("link", { name: "Visible", exact: true }).click();
  431 |         expect(new URL(page.url()).hash).toBe("#visible");
  432 |         await page.locator("article").getByRole("link", { name: `Other ${ids[0]}`, exact: true }).click();
  433 |         await expect(page).toHaveURL(new RegExp(`/other/#${ids[0]}$`));
  434 |         expect(await page.locator(`[id="${ids[0]}"]`).count()).toBe(1);
> 435 |         expect(errors).toEqual([]);
      |                        ^ Error: expect(received).toEqual(expected) // deep equality
  436 |         await test.info().attach("foreign-anchors.png", { body: await page.screenshot(), contentType: "image/png" });
  437 |     });
  438 | }
  439 | 
  440 | test("HTML templates at foreign integration points recover from ghost links", async ({ page }) => {
  441 |     site.write("rustify-docs.toml", `${site.read("rustify-docs.toml")}\n[[nav.groups]]\nlabel = "Anchors"\nitems = ["index.md", "other.md"]\n`);
  442 |     const contexts = [
  443 |         ["<svg><foreignObject>", "</foreignObject></svg>"],
  444 |         ["<svg><desc>", "</desc></svg>"],
  445 |         ["<svg><title>", "</title></svg>"],
  446 |         ["<math><mtext>", "</mtext></math>"],
  447 |         ['<math><annotation-xml encoding="text/html">', "</annotation-xml></math>"],
  448 |     ];
  449 |     const body = `---\ndescription: Integration anchors\n---\n# Integration anchors\n\n${contexts.map(([open, close], i) => `${open}<template id="holder-${i}"><svg><template><circle id="ghost-${i}"/></template></svg></template>${close}`).join("\n\n")}\n\n## Visible\n\n[Visible](#visible) [Other holder](other.md#holder-0)\n`;
  450 |     const invalid = contexts.map((_, i) => `[Ghost ${i}](#ghost-${i}) [Other ghost ${i}](other.md#ghost-${i})`).join("\n");
  451 |     site.write("content/index.md", `${body}\n${invalid}\n`);
  452 |     site.write("content/other.md", body);
  453 |     const errors: string[] = [];
  454 |     page.on("pageerror", (error) => errors.push(error.message));
  455 |     const server = await dev();
  456 |     await page.goto(server.url);
  457 |     await expect(page.getByRole("region", { name: "10 errors · nothing has built yet" })).toContainText("broken-anchor");
  458 |     site.write("content/index.md", body);
  459 |     await expect(page.getByRole("heading", { name: "Visible", exact: true })).toBeVisible();
  460 |     for (let i = 0; i < contexts.length; i++) {
  461 |         expect(await page.locator(`#holder-${i}`).evaluate((node) => ({ namespace: node.namespaceURI, stored: (node as HTMLTemplateElement).content.querySelector("circle")?.id }))).toEqual({ namespace: "http://www.w3.org/1999/xhtml", stored: `ghost-${i}` });
  462 |         expect(await page.locator(`#ghost-${i}`).count()).toBe(0);
  463 |     }
  464 |     await page.locator("article").getByRole("link", { name: "Visible", exact: true }).click();
  465 |     expect(new URL(page.url()).hash).toBe("#visible");
  466 |     await page.locator("article").getByRole("link", { name: "Other holder", exact: true }).click();
  467 |     await expect(page).toHaveURL(/\/other\/#holder-0$/);
  468 |     expect(errors).toEqual([]);
  469 | });
  470 | 
  471 | test("a failed build keeps the page readable under a panel that hides to a marker and clears on the fix", async ({ page }) => {
  472 |     site.write("content/guide/setup.md", `${site.read("content/guide/setup.md")}\n\n## Content\n\nThe author's heading keeps its own anchor.\n`);
  473 |     const server = await dev();
  474 |     await page.goto(`${server.url}guide/setup/`);
  475 |     await expect(page.locator("h2#content")).toHaveText("Content");
  476 |     await expect(page.locator("main")).toHaveAttribute("id", "content-1");
  477 |     await mark(page);
  478 |     const before = await seen(page);
  479 |     site.edit("content/guide/setup.md", "Run `rustify-docs --version`", `${BROKEN} Run \`rustify-docs --version\``);
  480 | 
  481 |     const panel = page.getByRole("dialog", { name: "1 error · showing the last good version" });
  482 |     await expect(panel).toBeVisible();
  483 |     await expect(panel).toContainText("content/guide/setup.md");
  484 |     await expect(panel.locator(".rd-dev-location")).toHaveText(/^\d+:\d+$/);
  485 |     await expect(panel).toContainText("broken-link");
  486 |     expect(await kept(page), "the last good page stays").toBe(true);
  487 |     await expect(page.locator("article").getByText("Broken", { exact: true })).toHaveCount(0);
  488 |     expect(await page.evaluate(() => document.activeElement === document.body), "focus stays where it was").toBe(true);
  489 |     await expect(page.getByRole("status").filter({ hasText: "1 error" })).toHaveCount(1);
  490 | 
  491 |     await panel.getByRole("button", { name: "Hide problems" }).click();
  492 |     await expect(panel).toBeHidden();
  493 |     const marker = page.getByRole("button", { name: "1 error · showing the last good version" });
  494 |     await expect(marker).toBeVisible();
  495 |     await expect(marker).toBeFocused();
  496 |     await expect(marker).toHaveAttribute("aria-expanded", "false");
  497 | 
  498 |     // The same mistake saved again: a new result, the marker stays.
  499 |     const failed = await seen(page);
  500 |     site.write("content/guide/setup.md", `${site.read("content/guide/setup.md")}\n`);
  501 |     await nextResult(page, failed);
  502 |     await expect(marker).toBeVisible();
  503 |     await expect(panel).toBeHidden();
  504 | 
  505 |     await page.keyboard.press("Enter");
  506 |     await expect(panel).toBeVisible();
  507 |     await expect(panel.getByRole("button", { name: "Hide problems" })).toBeFocused();
  508 |     await page.keyboard.press("Escape");
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
```