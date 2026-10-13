# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: docs-ssg/islands.spec.ts >> 200% reflow and text scaling keeps loading, ready, failure and bounded source usable in light
- Location: tests/docs-ssg/islands.spec.ts:629:9

# Error details

```
Error: expect(received).toBe(expected) // Object.is equality

Expected: 144
Received: 208
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
                  - button "Increment" [disabled]
                  - status [ref=e51]: "2"
                  - textbox "Counter value" [disabled] [ref=e52]: "2"
            - alert [ref=e53]:
              - generic [ref=e54]: Interactive example
              - generic [ref=e55]: This example could not start. Reload the page to try again.
              - button "Reload page" [ref=e56] [cursor=pointer]
          - heading "Failing callback" [level=2] [ref=e57]
          - generic [ref=e58]:
            - group "Interactive example" [ref=e59]:
              - generic [ref=e62]:
                - button "Change before failure" [disabled]
                - status [ref=e63]: "0"
                - button "Crash this example" [disabled]
            - alert [ref=e64]:
              - generic [ref=e65]: Interactive example
              - generic [ref=e66]: This example could not start. Reload the page to try again.
              - button "Reload page" [ref=e67] [cursor=pointer]
            - generic [ref=e68]:
              - paragraph [ref=e69]: src/demos.rs
              - generic [ref=e70]:
                - generic [ref=e71]:
                  - tablist [ref=e72]:
                    - tab "Function" [ref=e73] [cursor=pointer]
                    - tab "Full file" [selected] [ref=e74] [cursor=pointer]
                  - button "Copy" [ref=e75] [cursor=pointer]
                - tabpanel "Full file" [ref=e80]:
                  - code [ref=e82]:
                    - generic [ref=e84]:
                      - text: use
                      - generic [ref=e85]: "leptos::"
                      - generic [ref=e86]: "prelude::"
                      - text: "*;"
                    - generic [ref=e88]:
                      - text: use
                      - generic [ref=e89]: "rustify_components::"
                      - generic [ref=e90]: "{Button, TextField"
                      - generic [ref=e91]: "}"
                      - text: ;
                    - generic [ref=e93]:
                      - generic [ref=e94]:
                        - generic [ref=e95]: pub fn
                        - text: counter
                      - generic [ref=e96]:
                        - generic [ref=e97]: "(initial: i32, label: String, children: AnyView"
                        - generic [ref=e98]: )
                      - generic [ref=e100]: "-> impl IntoView"
                      - generic [ref=e102]: "{"
                    - generic [ref=e107]:
                      - text: let value =
                      - generic [ref=e108]: "RwSignal::"
                      - text: new
                      - generic [ref=e109]: (initial
                      - generic [ref=e110]: )
                      - text: ;
                    - generic [ref=e114]:
                      - text: view!
                      - generic [ref=e115]: "{"
                    - generic [ref=e120]:
                      - text: <div data-counter=
                      - generic [ref=e121]: "\"\""
                      - text: class=
                      - generic [ref=e122]: "\"min-h-24 space-y-3\""
                      - text: ">"
                    - generic [ref=e127]:
                      - text: <p>
                      - generic [ref=e128]: "{label"
                      - generic [ref=e129]: "}"
                      - text: </p>
                    - generic [ref=e134]:
                      - text: <div class=
                      - generic [ref=e135]: "\"flex flex-wrap items-center gap-3\""
                      - text: ">"
                    - generic [ref=e140]:
                      - text: <Button test_id=
                      - generic [ref=e141]: "\"counter-increment\""
                      - text: on_click=move || value.update
                      - generic [ref=e142]:
                        - text: (
                        - generic [ref=e143]: "|"
                        - generic [ref=e145]: value|
                        - generic [ref=e147]: "*value += 1"
                      - generic [ref=e148]: )
                      - text: ">"
                    - generic [ref=e149]: "\"Increment\""
                    - generic [ref=e155]: </Button>
                    - generic [ref=e164]:
                      - text: <output data-testid=
                      - generic [ref=e165]: "\"counter-value\""
                      - text: ">"
                      - generic [ref=e166]:
                        - text: "{move || value.get"
                        - generic [ref=e167]: (
                        - generic [ref=e168]: )
                      - generic [ref=e169]: "}"
                      - text: </output>
                    - generic [ref=e174]:
                      - text: <TextField test_id=
                      - generic [ref=e175]: "\"counter-input\""
                      - text: value=
                      - generic [ref=e176]: "Signal::"
                      - text: derive
                      - generic [ref=e177]:
                        - text: (move || value.get
                        - generic [ref=e178]: (
                        - generic [ref=e179]: )
                        - text: .to_string
                        - generic [ref=e180]: (
                        - generic [ref=e181]: )
                      - generic [ref=e182]: )
                    - generic [ref=e187]:
                      - text: "on_change=move |input: String|"
                      - generic [ref=e188]: "{"
                    - generic [ref=e194]:
                      - text: if let Ok
                      - generic [ref=e195]: (input
                      - generic [ref=e196]: )
                      - text: = input.parse
                      - generic [ref=e197]: (
                      - generic [ref=e198]: )
                      - generic [ref=e199]:
                        - text: "{ value.set"
                        - generic [ref=e200]: (input
                        - generic [ref=e201]: )
                        - text: ;
                      - generic [ref=e202]: "}"
                    - generic [ref=e203]: "}"
                    - generic [ref=e213]:
                      - text: aria_label=
                      - generic [ref=e214]: "\"Counter value\""
                    - generic [ref=e215]: />
                    - generic [ref=e220]: </div>
                    - generic [ref=e229]:
                      - generic [ref=e230]: "{children"
                      - generic [ref=e231]: "}"
                    - generic [ref=e232]: </div>
                    - generic [ref=e237]: "}"
                    - generic [ref=e242]: "}"
                    - generic [ref=e246]: /// The function and its containing file are displayed verbatim by the docs.
                    - generic [ref=e250]:
                      - generic [ref=e251]:
                        - generic [ref=e252]: pub fn
                        - text: counter_demo
                      - generic [ref=e253]:
                        - generic [ref=e254]: (
                        - generic [ref=e255]: )
                      - generic [ref=e257]: "-> impl IntoView"
                      - generic [ref=e259]: "{"
                    - generic [ref=e264]:
                      - text: counter
                      - generic [ref=e265]: (
                    - generic [ref=e266]: 7,
                    - generic [ref=e275]:
                      - generic [ref=e276]: "\"Résumé — a real registered counter\""
                      - text: .to_string
                      - generic [ref=e277]: (
                      - generic [ref=e278]: )
                      - text: ","
                    - generic [ref=e283]:
                      - text: view!
                      - generic [ref=e284]:
                        - text: "{ <p>"
                        - generic [ref=e285]: "\"This child is part of the same view on both targets.\""
                        - text: </p>
                      - generic [ref=e286]: "}"
                      - text: .into_any
                      - generic [ref=e287]: (
                      - generic [ref=e288]: )
                      - text: ","
                    - generic [ref=e289]: )
                    - generic [ref=e294]: "}"
                    - generic [ref=e299]:
                      - generic [ref=e300]:
                        - generic [ref=e301]: pub fn
                        - text: async_after_attach_demo
                      - generic [ref=e302]:
                        - generic [ref=e303]: (
                        - generic [ref=e304]: )
                      - generic [ref=e306]: "-> impl IntoView"
                      - generic [ref=e308]: "{"
                    - generic [ref=e310]: "// No asynchronous initial data: the action starts only after a reader clicks."
                    - generic [ref=e318]:
                      - text: let load =
                      - generic [ref=e319]: "Action::"
                      - text: new
                      - generic [ref=e320]:
                        - text: (
                        - generic [ref=e321]: "|"
                        - generic [ref=e324]:
                          - text: "_: &"
                          - generic [ref=e325]: (
                          - generic [ref=e326]: )
                          - text: "|"
                        - generic [ref=e327]:
                          - text: async
                          - generic [ref=e328]:
                            - text: "{"
                            - generic [ref=e329]: "\"Loaded after attachment\""
                            - text: .to_string
                            - generic [ref=e330]: (
                            - generic [ref=e331]: )
                          - generic [ref=e332]: "}"
                      - generic [ref=e333]: )
                      - text: ;
                    - generic [ref=e337]:
                      - text: view!
                      - generic [ref=e338]: "{"
                    - generic [ref=e343]:
                      - text: <div data-async-after-attach=
                      - generic [ref=e344]: "\"\""
                      - text: class=
                      - generic [ref=e345]: "\"space-y-3\""
                      - text: ">"
                    - generic [ref=e350]:
                      - text: <Button test_id=
                      - generic [ref=e351]: "\"async-start\""
                      - text: on_click=move ||
                      - generic [ref=e352]:
                        - text: "{ load.dispatch"
                        - generic [ref=e353]:
                          - text: (
                          - generic [ref=e354]: (
                          - generic [ref=e355]: )
                        - generic [ref=e356]: )
                        - text: ;
                      - generic [ref=e357]: "}"
                      - text: ">"
                    - generic [ref=e358]: "\"Load browser data\""
                    - generic [ref=e364]: </Button>
                    - generic [ref=e373]:
                      - text: <output data-testid=
                      - generic [ref=e374]: "\"async-result\""
                      - text: ">"
                    - generic [ref=e379]:
                      - generic [ref=e380]:
                        - text: "{move || load.value"
                        - generic [ref=e381]: (
                        - generic [ref=e382]: )
                        - text: .get
                        - generic [ref=e383]: (
                        - generic [ref=e384]: )
                        - text: .unwrap_or_else
                        - generic [ref=e385]:
                          - text: (
                          - generic [ref=e386]: "|"
                          - generic [ref=e388]: "|"
                          - generic [ref=e390]:
                            - generic [ref=e391]: "\"Not loaded\""
                            - text: .to_string
                            - generic [ref=e392]: (
                            - generic [ref=e393]: )
                        - generic [ref=e394]: )
                      - generic [ref=e395]: "}"
                    - generic [ref=e396]: </output>
                    - generic [ref=e401]: </div>
                    - generic [ref=e406]: "}"
                    - generic [ref=e411]: "}"
                    - generic [ref=e416]:
                      - generic [ref=e417]:
                        - generic [ref=e418]: pub fn
                        - text: failure_demo
                      - generic [ref=e419]:
                        - generic [ref=e420]: (
                        - generic [ref=e421]: )
                      - generic [ref=e423]: "-> impl IntoView"
                      - generic [ref=e425]: "{"
                    - generic [ref=e430]:
                      - text: let value =
                      - generic [ref=e431]: "RwSignal::"
                      - text: new
                      - generic [ref=e432]: (0
                      - generic [ref=e433]: )
                      - text: ;
                    - generic [ref=e437]:
                      - text: view!
                      - generic [ref=e438]: "{"
                    - generic [ref=e443]:
                      - text: <div data-failure-demo=
                      - generic [ref=e444]: "\"\""
                      - text: class=
                      - generic [ref=e445]: "\"space-y-3\""
                      - text: ">"
                    - generic [ref=e450]:
                      - text: <Button test_id=
                      - generic [ref=e451]: "\"failure-increment\""
                      - text: on_click=move || value.update
                      - generic [ref=e452]:
                        - text: (
                        - generic [ref=e453]: "|"
                        - generic [ref=e455]: value|
                        - generic [ref=e457]: "*value += 1"
                      - generic [ref=e458]: )
                      - text: ">"
                    - generic [ref=e459]: "\"Change before failure\""
                    - generic [ref=e465]: </Button>
                    - generic [ref=e474]:
                      - text: <output data-testid=
                      - generic [ref=e475]: "\"failure-value\""
                      - text: ">"
                      - generic [ref=e476]:
                        - text: "{move || value.get"
                        - generic [ref=e477]: (
                        - generic [ref=e478]: )
                      - generic [ref=e479]: "}"
                      - text: </output>
                    - generic [ref=e484]:
                      - text: <Button test_id=
                      - generic [ref=e485]: "\"failure-crash\""
                      - text: on_click=
                      - generic [ref=e486]: "|"
                      - generic [ref=e488]: "|"
                      - generic [ref=e490]:
                        - text: panic!
                        - generic [ref=e491]:
                          - text: (
                          - generic [ref=e492]: "\"docs fixture callback failure\""
                        - generic [ref=e493]: )
                        - text: ">"
                    - generic [ref=e494]: "\"Crash this example\""
                    - generic [ref=e500]: </Button>
                    - generic [ref=e505]: </div>
                    - generic [ref=e510]: "}"
                    - generic [ref=e515]: "}"
                    - generic [ref=e520]:
                      - generic [ref=e521]:
                        - generic [ref=e522]: pub fn
                        - text: catalog_proxy
                      - generic [ref=e523]:
                        - generic [ref=e524]: (
                        - generic [ref=e525]: )
                      - generic [ref=e527]: "-> impl IntoView"
                      - generic [ref=e529]: "{"
                    - generic [ref=e534]:
                      - text: let scope =
                      - generic [ref=e535]: "rustify_ui::"
                      - generic [ref=e536]: "id::"
                      - text: next
                      - generic [ref=e537]:
                        - text: (
                        - generic [ref=e538]: "\"catalog\""
                      - generic [ref=e539]: )
                      - text: .unwrap_or_else
                      - generic [ref=e540]:
                        - text: (
                        - generic [ref=e541]: "|"
                        - generic [ref=e543]: "|"
                        - generic [ref=e545]:
                          - generic [ref=e546]: "\"catalog\""
                          - text: .to_string
                          - generic [ref=e547]: (
                          - generic [ref=e548]: )
                      - generic [ref=e549]: )
                      - text: ;
                    - generic [ref=e553]:
                      - text: view!
                      - generic [ref=e554]:
                        - text: "{ <crate"
                        - generic [ref=e555]: "::"
                        - generic [ref=e556]: "catalog::"
                        - text: CatalogView scope=scope />
                      - generic [ref=e557]: "}"
                    - generic [ref=e558]: "}"
        - navigation "Previous / Next" [ref=e562]:
          - link "Previous Client-only counter" [ref=e563] [cursor=pointer]:
            - /url: /rustify-ui/client-only/
            - generic [ref=e564]: Previous
            - generic [ref=e565]: Client-only counter
          - link "Next Catalog release proxy" [ref=e566] [cursor=pointer]:
            - /url: /rustify-ui/proxy/
            - generic [ref=e567]: Next
            - generic [ref=e568]: Catalog release proxy
  - contentinfo [ref=e569]:
    - navigation "Footer links" [ref=e570]:
      - link "Back to the home page" [ref=e571] [cursor=pointer]:
        - /url: /rustify-ui/
  - paragraph [ref=e572]
```

# Test source

```ts
  569 |     expect(audit.pageErrors).toEqual([]);
  570 |     expect(audit.failures).toEqual([]);
  571 |     expect(audit.responses).toEqual([]);
  572 |     expect(await page.evaluate(() => window.__docs_island_csp)).toEqual([]);
  573 | });
  574 | 
  575 | test("the startup deadline remains terminal when the real wasm completes late", async ({ page, audit }) => {
  576 |     // Playwright virtualizes the actual performance clock and timer callbacks;
  577 |     // this does not claim a thirty-second physical network measurement.
  578 |     await page.clock.install({ time: new Date("2026-01-01T00:00:00Z") });
  579 |     const held = await openPaused(page, "counter/");
  580 |     await page.clock.pauseAt((await page.evaluate(() => Date.now())) + 1000);
  581 |     const before = await page.evaluate(() => performance.now());
  582 |     await page.clock.runFor(30001);
  583 |     expect((await page.evaluate(() => performance.now())) - before).toBe(30001);
  584 |     await restored(page, 4);
  585 |     expect(await page.evaluate(() => window.__rustify_docs_islands?.failure())).toBe("Error: island startup deadline exceeded");
  586 |     held.release();
  587 |     await expect.poll(() => page.evaluate(() => window.__rustify_docs_islands?.stats()?.aborted)).toBe(true);
  588 |     await page.clock.runFor(100);
  589 |     await restored(page, 4);
  590 |     expect(await page.evaluate(() => window.__rustify_docs_islands?.stats())).toMatchObject({ tasks: 0, timers: 0, observers: 0, aborted: true });
  591 |     expect(audit.console.filter(message => /^Interactive examples failed/.test(message.text))).toHaveLength(1);
  592 |     expect(audit.console.filter(message => !/^Interactive examples failed/.test(message.text))).toEqual([]);
  593 |     expect(audit.pageErrors).toEqual([]);
  594 |     expect(audit.failures).toEqual([]);
  595 |     expect(audit.responses).toEqual([]);
  596 |     expect(await page.evaluate(() => window.__docs_island_csp)).toEqual([]);
  597 |     await page.clock.resume();
  598 | });
  599 | 
  600 | test("the release proxy attaches every real category, then opens floating controls with focus return", async ({ page, audit }) => {
  601 |     const held = await openPaused(page, "proxy/");
  602 |     await expect(page.locator("[data-category]")).toHaveCount(24);
  603 |     held.release();
  604 |     await ready(page, 4);
  605 |     await identity(page);
  606 |     const category = (name: string) => page.locator(`[data-category="${name}"]`);
  607 |     await category("text field").getByRole("textbox", { name: "Editable text", exact: true }).fill("Attached text");
  608 |     await expect(category("text field").locator('[data-testid$="-text-result"]')).toHaveText("Attached text");
  609 |     await category("menu").getByRole("button", { name: "Open menu", exact: true }).click();
  610 |     await expect(page.getByRole("menu", { name: "Fixture actions" })).toBeVisible();
  611 |     await page.getByRole("menuitem", { name: "Copy", exact: true }).click();
  612 |     await expect(page.getByRole("menu", { name: "Fixture actions" })).toHaveCount(0);
  613 |     await expect(category("menu").getByRole("button", { name: "Open menu", exact: true })).toBeFocused();
  614 |     await category("dialog").getByRole("button", { name: "Open dialog", exact: true }).click();
  615 |     await expect(page.getByRole("dialog", { name: "Fixture dialog" })).toBeVisible();
  616 |     await page.keyboard.press("Escape");
  617 |     await expect(page.getByRole("dialog", { name: "Fixture dialog" })).toHaveCount(0);
  618 |     await expect(category("dialog").getByRole("button", { name: "Open dialog", exact: true })).toBeFocused();
  619 |     const peers = page.locator('[data-counter]');
  620 |     await peers.first().getByTestId("counter-increment").click();
  621 |     await expect(peers.nth(0).getByTestId("counter-value")).toHaveText("3");
  622 |     await expect(peers.nth(1).getByTestId("counter-value")).toHaveText("2");
  623 |     await expect(peers.nth(2).getByTestId("counter-value")).toHaveText("2");
  624 |     await healthy(page, audit);
  625 | });
  626 | 
  627 | for (const colorScheme of ["light", "dark"] as const) {
  628 |     for (const layout of [{ name: "375px", width: 375, scaleText: false }, { name: "200% reflow and text scaling", width: 640, scaleText: true }]) {
  629 |         test(`${layout.name} keeps loading, ready, failure and bounded source usable in ${colorScheme}`, async ({ page, audit }) => {
  630 |             await page.setViewportSize({ width: layout.width, height: 800 });
  631 |             await page.emulateMedia({ colorScheme, reducedMotion: "reduce" });
  632 |             const held = await openPaused(page, "failure/");
  633 |             if (layout.scaleText) {
  634 |                 // The repository's zoom checks use half the CSS viewport and
  635 |                 // twice the root text size, exercising both reflow and text.
  636 |                 const transitions = await page.evaluate(async () => {
  637 |                     document.documentElement.style.fontSize = "200%";
  638 |                     const scaling = document.getAnimations().filter((animation): animation is CSSTransition =>
  639 |                         animation instanceof CSSTransition && Number.isFinite(animation.effect?.getTiming().iterations));
  640 |                     const observed = scaling.map(animation => ({ property: animation.transitionProperty, duration: animation.effect?.getTiming().duration }));
  641 |                     await Promise.all(scaling.map(animation => animation.finished));
  642 |                     await new Promise<void>(resolve => requestAnimationFrame(() => requestAnimationFrame(() => resolve())));
  643 |                     return observed;
  644 |                 });
  645 |                 await test.info().attach("font-scaling-transitions.json", { body: JSON.stringify(transitions, null, 2), contentType: "application/json" });
  646 |                 await captureOriginals(page);
  647 |             }
  648 |             await page.screenshot({ path: test.info().outputPath(`loading-${colorScheme}-${layout.width}.png`), fullPage: true });
  649 |             held.release();
  650 |             await ready(page, 2);
  651 |             await identity(page);
  652 |             const source = page.locator(".rd-demo-source");
  653 |             await source.getByRole("tab", { name: "Full file", exact: true }).click();
  654 |             const pre = source.locator('.rd-code:not([hidden]) .rd-code-pre');
  655 |             await pre.focus();
  656 |             await expect(pre).toBeFocused();
  657 |             const dimensions = await pre.evaluate(node => ({
  658 |                 client: node.clientHeight, scroll: node.scrollHeight,
  659 |                 limit: 24 * parseFloat(getComputedStyle(document.documentElement).fontSize),
  660 |             }));
  661 |             expect(dimensions.client).toBeLessThanOrEqual(dimensions.limit);
  662 |             expect(dimensions.scroll).toBeGreaterThan(dimensions.client);
  663 |             await page.getByTestId("failure-crash").click();
  664 |             await restored(page, 2);
  665 |             const status = await page.evaluate(() => window.__docs_island_probe!.blocks.map(snapshot => ({
  666 |                 before: snapshot.statusHeight,
  667 |                 after: document.querySelector(`[data-rustify-island="${snapshot.key}"] [data-rd-island-status]`)!.getBoundingClientRect().height,
  668 |             })));
> 669 |             for (const height of status) expect(height.after).toBe(height.before);
      |                                                               ^ Error: expect(received).toBe(expected) // Object.is equality
  670 |             expect(await page.evaluate(() => document.documentElement.scrollWidth - innerWidth)).toBeLessThanOrEqual(0);
  671 |             await expect(source.getByRole("tab", { name: "Full file", exact: true })).toBeEnabled();
  672 |             await expect(islands(page).last().getByRole("button", { name: "Reload page", exact: true })).toBeInViewport();
  673 |             await page.screenshot({ path: test.info().outputPath(`failure-${colorScheme}-${layout.width}.png`), fullPage: true });
  674 |             expect(audit.console.filter(message => !/^Interactive examples failed/.test(message.text) && !/docs fixture callback failure/.test(message.text))).toEqual([]);
  675 |             expect(audit.console.filter(message => /^Interactive examples failed/.test(message.text))).toHaveLength(1);
  676 |             expect(audit.pageErrors.filter(error => !/^RuntimeError: unreachable$/.test(error))).toEqual([]);
  677 |             expect(audit.failures).toEqual([]);
  678 |             expect(audit.responses).toEqual([]);
  679 |             expect(await page.evaluate(() => window.__docs_island_csp)).toEqual([]);
  680 |         });
  681 |     }
  682 | }
  683 | 
```