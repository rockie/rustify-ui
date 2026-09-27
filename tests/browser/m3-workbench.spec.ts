import { expect, Page } from "@playwright/test";
import { rounds } from "../tier";
import { capture, differingPixels, litPixels, settle, test, waitForReady, windowListeners } from "./support";

const snapshot = (page: Page) =>
    page.evaluate(() => window.__property_workbench.snapshot());

// The region's own header row puts its buttons first, so their place depends on
// the row's padding and nothing else: an offset in CSS pixels from the region's
// left edge, not a fraction of a width that a value could change.
const NEXT_BUTTON = { dx: 100, fy: 0.06 };

const at = (
    box: { x: number; y: number; width: number; height: number },
    spot: { dx: number; fy: number }
) => ({
    x: box.x + spot.dx,
    y: box.y + box.height * spot.fy,
});

test.describe("M3 V2: one authoritative state behind a DOM panel and a GPU view", () => {
    test("a thousand objects with stable ids, the first one selected", async ({ page }) => {
        await waitForReady(page);
        expect(await snapshot(page)).toMatchObject({
            count: 1000,
            position: 1,
            selected: 1,
            name: "object-0001",
            color: "2e90fa",
            first_ids: "1,2,3,4",
        });
        await expect(page.getByTestId("object-count")).toHaveText("1000");
        await expect(page.getByTestId("selected-id")).toHaveText("1");
        expect(await page.evaluate(() => window.__property_workbench.live_regions())).toBe(1);
        // The grid is on screen without any interaction: the first projection
        // of application state has to survive the pump that creates the region,
        // otherwise the region draws its empty defaults until something else
        // changes.
        const drawn = await settle(page.getByTestId("workbench-gpu"));
        expect(litPixels(drawn)).toBeGreaterThan(10_000);
    });

    test("renaming and recolouring in the DOM changes what the GPU draws", async ({ page }) => {
        await waitForReady(page);
        const region = page.getByTestId("workbench-gpu");
        const before = await settle(region);
        await page.getByTestId("name-input").fill("renamed in the panel");
        await expect
            .poll(async () => differingPixels(before, await capture(region)), { timeout: 5_000 })
            .toBeGreaterThan(20);
        const renamed = await settle(region);
        await page.getByTestId("swatch-f04438").click();
        await expect
            .poll(async () => differingPixels(renamed, await capture(region)), { timeout: 5_000 })
            .toBeGreaterThan(20);
        expect(await snapshot(page)).toMatchObject({
            selected: 1,
            name: "renamed in the panel",
            color: "f04438",
        });
    });

    test("the GPU view moves the selection the application owns", async ({ page }) => {
        await waitForReady(page);
        const region = page.getByTestId("workbench-gpu");
        await settle(region);
        const box = (await region.boundingBox())!;
        const next = at(box, NEXT_BUTTON);
        await page.mouse.click(next.x, next.y);
        await expect(page.getByTestId("selected-id")).toHaveText("2");
        expect(await snapshot(page)).toMatchObject({ selected: 2, position: 2, name: "object-0002" });
        await page.getByTestId("select-next").click();
        await expect(page.getByTestId("selected-id")).toHaveText("3");
    });

    test("a name with quotes and backslashes reaches the application unchanged", async ({ page }) => {
        await waitForReady(page);
        const awkward = 'a"b\\c\td';
        await page.getByTestId("name-input").fill(awkward);
        await expect(page.getByTestId("name-input")).toHaveValue(awkward);
        expect(await snapshot(page)).toMatchObject({ selected: 1, name: awkward });
    });

    test("a run of selection steps from both sides lands on one expected object", async ({ page }) => {
        test.setTimeout(300_000);
        await waitForReady(page);
        const forward = rounds(25);
        // Fewer back than forward, so the run cannot land where it started.
        const back = Math.min(5, forward - 1);
        for (let round = 0; round < forward; round++) {
            await page.getByTestId("select-next").click();
        }
        await expect(page.getByTestId("selected-id")).toHaveText(String(1 + forward));
        for (let round = 0; round < back; round++) {
            await page.getByTestId("select-previous").click();
        }
        expect(await snapshot(page)).toMatchObject({
            selected: 1 + forward - back,
            position: 1 + forward - back,
            name: `object-${String(1 + forward - back).padStart(4, "0")}`,
        });
    });

    test("clicking a cell in the GPU grid selects that object in the DOM", async ({ page }) => {
        await waitForReady(page);
        const region = page.getByTestId("workbench-gpu");
        await settle(region);
        const box = (await region.boundingBox())!;
        const cell = { x: box.x + box.width * 0.5, y: box.y + box.height * 0.7 };
        await page.mouse.click(cell.x, cell.y);
        await expect(page.getByTestId("selected-id")).not.toHaveText("1");
        const picked = await snapshot(page);
        expect(picked.selected).not.toBeNull();
        expect(picked.name).toBe(`object-${String(picked.selected).padStart(4, "0")}`);
        // The same cell is the same object: identity comes from the id, not
        // from where the pointer landed.
        await page.mouse.click(cell.x, cell.y);
        expect(await snapshot(page)).toMatchObject({ selected: picked.selected });
        // And the panel edits the object the grid chose.
        await page.getByTestId("name-input").fill("picked on the GPU");
        expect(await snapshot(page)).toMatchObject({
            selected: picked.selected,
            name: "picked on the GPU",
        });
    });

    test("a hundred objects change in one update and the GPU shows the result", async ({ page }) => {
        await waitForReady(page);
        const region = page.getByTestId("workbench-gpu");
        const before = await settle(region);
        expect((await snapshot(page)).first_colors).toBe("2e90fa,12b76a,f79009,f04438");
        await page.getByTestId("recolour-batch").click();
        // One controlled model update, one new projection: the first four
        // objects each moved one step along the palette.
        expect(await snapshot(page)).toMatchObject({
            count: 1000,
            first_colors: "12b76a,f79009,f04438,7a5af8",
        });
        await expect
            .poll(async () => differingPixels(before, await capture(region)), { timeout: 5_000 })
            .toBeGreaterThan(20);
    });

    test("reordering keeps the selection on the object, not on the position", async ({ page }) => {
        await waitForReady(page);
        await page.getByTestId("select-next").click();
        await expect(page.getByTestId("selected-id")).toHaveText("2");
        expect(await snapshot(page)).toMatchObject({ position: 2, first_ids: "1,2,3,4" });
        await page.getByTestId("reverse-batch").click();
        // Object 2 is now second from the end of the reversed run.
        expect(await snapshot(page)).toMatchObject({
            selected: 2,
            name: "object-0002",
            position: 99,
            first_ids: "100,99,98,97",
        });
    });

    test("adding and removing objects keeps every other id where it was", async ({ page }) => {
        await waitForReady(page);
        await page.getByTestId("add-objects").click();
        expect(await snapshot(page)).toMatchObject({ count: 1010, selected: 1, position: 1 });
        await expect(page.getByTestId("object-count")).toHaveText("1010");
        await page.getByTestId("remove-objects").click();
        expect(await snapshot(page)).toMatchObject({
            count: 1000,
            selected: 1,
            position: 1,
            first_ids: "1,2,3,4",
        });
    });

    test("a list that names one object twice is refused, not guessed at", async ({ page }) => {
        await waitForReady(page);
        const region = page.getByTestId("workbench-gpu");
        const before = await settle(region);
        expect(await page.evaluate(() => window.__property_workbench.inject_duplicate_id())).toBe(true);
        await expect(page.getByTestId("rejected-binding")).toHaveText(
            "object 1 appears twice; the grid kept the last unambiguous list"
        );
        // The grid still shows the last list it could read unambiguously.
        await page.waitForTimeout(500);
        expect(differingPixels(before, await capture(region))).toBe(0);
    });

    test("ten thousand GPU actions arrive once each, in order", async ({ page }) => {
        test.setTimeout(600_000);
        await waitForReady(page);
        const region = page.getByTestId("workbench-gpu");
        await settle(region);
        const box = (await region.boundingBox())!;
        const actions = rounds(10_000);
        // Driven inside the page: ten thousand round trips through the test
        // harness would measure the harness, not the runtime. The events are
        // the ones the region's own listeners receive.
        const result = await page.evaluate(
            async (args) => {
                const api = window.__property_workbench;
                const canvas = document.querySelector('[data-testid="workbench-gpu"]')!;
                const fire = (type: string, buttons: number) =>
                    canvas.dispatchEvent(
                        new PointerEvent(type, {
                            bubbles: true,
                            cancelable: true,
                            clientX: args.x,
                            clientY: args.y,
                            pointerId: 1,
                            pointerType: "mouse",
                            button: 0,
                            buttons,
                            isPrimary: true,
                        })
                    );
                for (let i = 0; i < args.rounds; i++) {
                    fire("pointerdown", 1);
                    fire("pointerup", 0);
                    if (i % 25 === 0) {
                        await new Promise((r) => setTimeout(r, 0));
                    }
                }
                await new Promise((r) => setTimeout(r, 3_000));
                return api.snapshot();
            },
            { ...at(box, NEXT_BUTTON), rounds: actions }
        );
        // Not one lost, not one delivered twice, and not one refused: this is
        // the load the queue depth was chosen for.
        expect(result.accepted).toBe(actions);
        expect(result.refused).toBe(0);
        // And they were applied in order: each one advanced the selection by
        // one object, so the ring of a thousand lands back where it started.
        expect(result.position).toBe((actions % 1000) + 1);
        expect(result.count).toBe(1000);
    });

    test("the object under the pointer is the object a click there selects", async ({ page }) => {
        await waitForReady(page);
        const region = page.getByTestId("workbench-gpu");
        await settle(region);
        const box = (await region.boundingBox())!;
        const cell = { x: box.x + box.width * 0.4, y: box.y + box.height * 0.6 };
        await page.mouse.move(cell.x, cell.y);
        await expect.poll(async () => (await snapshot(page)).hovered).not.toBeNull();
        const hovered = (await snapshot(page)).hovered;
        await page.mouse.click(cell.x, cell.y);
        // Two paths through the same grid geometry have to name the same
        // object, or the highlight is pointing at something the click will not
        // pick.
        await expect(page.getByTestId("selected-id")).toHaveText(String(hovered));
        expect(await snapshot(page)).toMatchObject({ selected: hovered, hovered });
    });

    test("a pointer stream loses none of the clicks and saves interleaved with it", async ({ page }) => {
        test.setTimeout(300_000);
        await waitForReady(page);
        const region = page.getByTestId("workbench-gpu");
        const before = await settle(region);
        const box = (await region.boundingBox())!;
        const clicks = rounds(20);
        const moves_per_click = 12;
        // Driven inside the page and paced by a real timer: the stream has to
        // be a stream, and a round trip through the harness per move would set
        // the rate rather than measure it.
        const result = await page.evaluate(
            async (args) => {
                const canvas = document.querySelector('[data-testid="workbench-gpu"]')!;
                const input = document.querySelector('[data-testid="name-input"]') as HTMLInputElement;
                const pointer = (type: string, x: number, y: number, buttons: number) =>
                    canvas.dispatchEvent(
                        new PointerEvent(type, {
                            bubbles: true,
                            cancelable: true,
                            clientX: x,
                            clientY: y,
                            pointerId: 1,
                            pointerType: "mouse",
                            button: 0,
                            buttons,
                            isPrimary: true,
                        })
                    );
                const moves = args.clicks * args.moves_per_click;
                for (let i = 0; i < moves; i++) {
                    // Across the grid, a cell or so at a time.
                    pointer("pointermove", args.sweep_from + i * args.step, args.grid_y, 0);
                    if ((i + 1) % args.moves_per_click === 0) {
                        const click = (i + 1) / args.moves_per_click;
                        pointer("pointerdown", args.button_x, args.button_y, 1);
                        pointer("pointerup", args.button_x, args.button_y, 0);
                        input.value = `save-${click}`;
                        input.dispatchEvent(new Event("input", { bubbles: true }));
                    }
                    // 120 Hz.
                    await new Promise((r) => setTimeout(r, 8));
                }
                await new Promise((r) => setTimeout(r, 2_000));
                return { moves, snapshot: window.__property_workbench.snapshot() };
            },
            {
                clicks,
                moves_per_click,
                sweep_from: box.x + box.width * 0.05,
                step: (box.width * 0.9) / (clicks * moves_per_click),
                grid_y: box.y + box.height * 0.6,
                button_x: at(box, NEXT_BUTTON).x,
                button_y: at(box, NEXT_BUTTON).y,
            }
        );
        // Independent expectation: click k selected object k+1 and the save
        // that followed it renamed that object, so the last click leaves
        // object 21 selected under the twentieth name.
        expect(result.snapshot).toMatchObject({
            count: 1000,
            accepted: clicks,
            refused: 0,
            position: clicks + 1,
            selected: clicks + 1,
            name: `save-${clicks}`,
        });
        // The stream itself was collapsed on the way in - that is what keeps it
        // out of the queue the clicks use - but it did arrive.
        expect(result.snapshot.hovers).toBeGreaterThan(0);
        expect(result.snapshot.hovers).toBeLessThanOrEqual(result.moves);
        // Each save landed on the object selected at the time, and none of them
        // overwrote a neighbour.
        await page.getByTestId("select-previous").click();
        expect(await snapshot(page)).toMatchObject({
            selected: clicks,
            name: `save-${clicks - 1}`,
        });
        expect(differingPixels(before, await capture(region))).toBeGreaterThan(20);
    });

    test("deleting the selected object drops it and clears the selection", async ({ page }) => {
        await waitForReady(page);
        await page.getByTestId("delete-selected").click();
        expect(await snapshot(page)).toMatchObject({
            count: 999,
            position: 0,
            selected: null,
            name: null,
            color: "none",
        });
        await expect(page.getByTestId("selected-id")).toHaveText("none");
        await expect(page.getByTestId("name-input")).toBeDisabled();
        // The scope keeps working: the next object can be selected again.
        await page.getByTestId("select-next").click();
        await expect(page.getByTestId("selected-id")).toHaveText("2");
    });
});

test.describe("M3: a scope that closes while its region is running", () => {
    test("closing from inside a region action leaves the runtime alive", async ({ page }) => {
        const failures: string[] = [];
        page.on("pageerror", (error) => failures.push(String(error)));
        await waitForReady(page);
        const region = page.getByTestId("workbench-gpu");
        await settle(region);
        const box = (await region.boundingBox())!;
        await page.evaluate(() => window.__property_workbench.close_on_next_action());
        // The region's own "next" button: the application closes its scope
        // from inside the callback this click delivers, while the pump that
        // produced it is still on the JS stack.
        const next = at(box, NEXT_BUTTON);
        await page.mouse.click(next.x, next.y);
        await expect(page.getByTestId("workbench-gpu")).toHaveCount(0);
        await page.waitForTimeout(500);
        // The runtime is not fatal: the page still has its API and no error.
        expect(await page.evaluate(() => window.__property_workbench !== undefined)).toBe(true);
        await expect(page.getByTestId("status")).toHaveAttribute("data-status", "ready");
        const state = await page.evaluate(() => ({
            regions: window.__property_workbench.live_regions(),
            errors: window.__property_workbench.hooks.runtime.errors,
        }));
        expect(state).toEqual({ regions: 0, errors: [] });
        expect(failures).toEqual([]);
        // And it still works: the page mounts the scope again.
        await page.evaluate(() => window.__property_workbench.mount());
        await expect(page.getByTestId("workbench-gpu")).toHaveCount(1);
        await page.getByTestId("select-next").click();
        await expect(page.getByTestId("selected-id")).toHaveText("2");
    });

    test("a scope disposed while its region is still starting leaves nothing behind", async ({ page }) => {
        const failures: string[] = [];
        page.on("pageerror", (error) => failures.push(String(error)));
        page.on("console", (message) => {
            if (message.type() === "error") {
                failures.push(message.text());
            }
        });
        await waitForReady(page);
        // A region's startup is suspended on a microtask between its canvas
        // being created and its first message; the dispose lands somewhere in
        // that window, wherever the framework's own microtasks put it.
        const state = await page.evaluate(async () => {
            const api = window.__property_workbench;
            api.dispose();
            for (let ticks = 0; ticks < 8; ticks++) {
                api.mount();
                for (let tick = 0; tick < ticks; tick++) {
                    await Promise.resolve();
                }
                api.dispose();
                await new Promise((r) => setTimeout(r, 50));
            }
            await new Promise((r) => setTimeout(r, 300));
            return { regions: api.live_regions(), errors: api.hooks.runtime.errors };
        });
        expect(state).toEqual({ regions: 0, errors: [] });
        expect(failures).toEqual([]);
    });

    test("the batch a closing region produced is drawn before its Cx goes", async ({ page }) => {
        const failures: string[] = [];
        page.on("pageerror", (error) => failures.push(String(error)));
        await waitForReady(page);
        const region = page.getByTestId("workbench-gpu");
        await settle(region);
        const box = (await region.boundingBox())!;
        const observed = await page.evaluate(
            async (point) => {
                const api = window.__property_workbench;
                const host = [...api.hooks.regions.values()][0];
                const live: number[] = [];
                const original = host.new_from_wasm.bind(host);
                // Every batch the region produces is handed over here, and its
                // draw calls read uniform buffers by pointer into memory the Cx
                // owns. Nothing may be handed over after that Cx is gone.
                host.new_from_wasm = (ptr: number) => {
                    live.push(api.live_regions());
                    return original(ptr);
                };
                api.close_on_next_action();
                const canvas = document.querySelector('[data-testid="workbench-gpu"]')!;
                for (const type of ["pointerdown", "pointerup"]) {
                    canvas.dispatchEvent(
                        new PointerEvent(type, {
                            bubbles: true,
                            cancelable: true,
                            clientX: point.x,
                            clientY: point.y,
                            pointerId: 1,
                            pointerType: "mouse",
                            button: 0,
                            buttons: type === "pointerdown" ? 1 : 0,
                            isPrimary: true,
                        })
                    );
                }
                await new Promise((r) => setTimeout(r, 300));
                return { live, regions: api.live_regions() };
            },
            at(box, NEXT_BUTTON)
        );
        expect(observed.live.length).toBeGreaterThan(0);
        expect(observed.live.filter((count) => count === 0)).toEqual([]);
        expect(observed.regions).toBe(0);
        expect(failures).toEqual([]);
    });

    // A trap ends the instance, so each of these loads a page of its own.
    test.describe(() => {
        test.use({ fresh: true });

        test("a fatal drops the work the runtime had already scheduled", async ({ page }) => {
            await waitForReady(page);
            const outcome = await page.evaluate(async () => {
                const api = window.__property_workbench;
                let ran = false;
                // The SDK gives the browser its turn between action batches through
                // this same entry, so what is cancelled here is a queued action on
                // its way back into a module that has trapped.
                api.hooks.defer(() => {
                    ran = true;
                });
                const scheduled = api.stats().tasks;
                api.hooks.runtime.enter_fatal(new Error("injected trap"));
                await new Promise((r) => setTimeout(r, 200));
                return { scheduled, ran, tasks: api.stats().tasks };
            });
            expect(outcome).toEqual({ scheduled: 1, ran: false, tasks: 0 });
            await expect(page.getByTestId("status")).toHaveAttribute("data-status", "fatal");
        });

        test("a trapped runtime leaves no keyboard listener on the window", async ({ page }) => {
            await waitForReady(page);
            // The drag's Escape is heard on the window, which outlives the
            // module; no cleanup runs after a trap, so only the instance's
            // abort can take it off.
            expect((await windowListeners(page)).keydown ?? 0).toBeGreaterThan(0);
            await page.evaluate(() =>
                window.__property_workbench.hooks.runtime.enter_fatal(new Error("injected trap"))
            );
            await expect(page.getByTestId("status")).toHaveAttribute("data-status", "fatal");
            expect((await windowListeners(page)).keydown ?? 0).toBe(0);
        });

        test("a trapped runtime takes its controls off the page", async ({ page }) => {
            await waitForReady(page);
            await expect(page.getByTestId("name-input")).toHaveCount(1);
            await page.evaluate(() =>
                window.__property_workbench.hooks.runtime.enter_fatal(new Error("injected trap"))
            );
            await expect(page.getByTestId("status")).toHaveAttribute("data-status", "fatal");
            // Nothing the user can still press reaches the module that trapped.
            await expect(page.getByTestId("name-input")).toHaveCount(0);
            await expect(page.getByTestId("select-next")).toHaveCount(0);
            await expect(page.getByTestId("workbench-gpu")).toHaveCount(0);
        });
    });
});


test.describe("a reset puts the page back the way it loaded", () => {
    // The first load is what it is compared with, so it makes one.
    test.use({ fresh: true });

    /// Everything a check can leave behind that the next one could see.
    const state = (page: Page) =>
        page.evaluate(() => {
            const api = window.__property_workbench;
            return {
                url: location.href,
                snapshot: api.snapshot(),
                regions: api.live_regions(),
                scopes: document.querySelectorAll("[data-rustify-scope]").length,
                third_party: api.third_party(),
                recording: api.diagnostics().recording,
                // Deleted below; a reset forgets that it ever was.
                deleted: api.lookup_object(43),
                errors: api.hooks.runtime.errors.length,
                expanded: document.querySelectorAll('[aria-expanded="true"]').length,
                focused: (document.activeElement as HTMLElement | null)?.dataset.testid ?? document.activeElement?.tagName,
                root_style: document.documentElement.style.cssText,
                scrolled: window.scrollY,
            };
        });

    /// One drag of an object's row onto the bin, dispatched in the page, or
    /// abandoned with Escape once it is over it.
    const carry = (page: Page, object: number, escape: boolean) =>
        page.evaluate(
            ({ object, escape }) => {
                const row = document.querySelector(`[data-testid="object-${object}"]`) as HTMLElement;
                const from = row.getBoundingClientRect();
                const bin = document.querySelector('[data-testid="ungrouped-bin"]')!.getBoundingClientRect();
                const to = { x: bin.left + bin.width / 2, y: bin.top + bin.height / 2 };
                const send = (type: string, x: number, y: number) =>
                    row.dispatchEvent(
                        new PointerEvent(type, {
                            bubbles: true,
                            clientX: x,
                            clientY: y,
                            pointerId: 1,
                            isPrimary: true,
                            button: 0,
                            buttons: type === "pointerup" ? 0 : 1,
                        })
                    );
                send("pointerdown", from.left + from.width / 2, from.top + from.height / 2);
                send("pointermove", to.x, to.y);
                if (escape) {
                    window.dispatchEvent(new KeyboardEvent("keydown", { key: "Escape", bubbles: true }));
                } else {
                    send("pointerup", to.x, to.y);
                }
            },
            { object, escape }
        );

    test("the application, the address and the page read as they did on the first load", async ({
        page,
    }) => {
        test.setTimeout(240_000);
        await waitForReady(page);
        await expect.poll(async () => (await snapshot(page)).controls).not.toBeNull();
        const first = await state(page);
        // Loaded at the root; the application put the first object's address
        // in its place.
        expect(new URL(first.url).pathname).toBe("/objects/1");
        expect(first.snapshot).toMatchObject({ selected: 1, path: "/objects/1", region: "ready" });

        const region = page.getByTestId("workbench-gpu");
        const box = (await region.boundingBox())!;
        // The pointer over the grid, and a selection made on the region.
        await page.mouse.move(box.x + box.width * 0.4, box.y + box.height * 0.6);
        await expect.poll(async () => (await snapshot(page)).hovered).not.toBeNull();
        const next = at(box, NEXT_BUTTON);
        await page.mouse.click(next.x, next.y);
        await expect.poll(async () => (await snapshot(page)).selected).toBe(2);

        // Carried to the bin once, and abandoned over it once.
        await page.evaluate(() => window.scrollTo(0, 0));
        await carry(page, 2, false);
        await carry(page, 2, true);
        await expect
            .poll(async () => {
                const { drops, cancels } = (await snapshot(page)).drag;
                return { drops, cancels };
            })
            .toEqual({ drops: 1, cancels: 1 });

        // One object deleted, another renamed, recoloured, locked and resized,
        // and the list itself reordered and grown.
        const find = async (id: number) => {
            await page.getByTestId("find-object-id").fill(String(id));
            await page.getByTestId("find-object").click();
        };
        await find(43);
        await page.getByTestId("delete-selected").click();
        await find(42);
        await page.getByTestId("name-input").fill("renamed before a reset");
        await page.getByTestId("swatch-f04438").click();
        await page.getByRole("slider", { name: "size" }).focus();
        await page.keyboard.press("ArrowRight");
        await page.getByTestId("locked-input").check();
        await page.getByTestId("recolour-batch").click();
        await page.getByTestId("reverse-batch").click();
        await page.getByTestId("add-objects").click();
        await page.getByTestId("import-file").setInputFiles({
            name: "objects.txt",
            mimeType: "text/plain",
            buffer: Buffer.from("3\tfrom a file\t2\t0\t35\n", "utf8"),
        });
        const downloaded = page.waitForEvent("download");
        await page.getByTestId("export-text").click();
        await downloaded;
        await page.getByTestId("copy-notes").click();
        await expect
            .poll(async () => {
                const { imports, exports, clipboard } = (await snapshot(page)).transfer;
                return { imports, exports, answered: clipboard !== "" };
            })
            .toEqual({ imports: 1, exports: 1, answered: true });

        // The theme, an override, the third-party component and the workspace.
        await page.getByTestId("toggle-theme").click();
        await page.getByTestId("toggle-emphasis").click();
        await page.getByTestId("toggle-third-party").click();
        await page.getByTestId("wheel-propagates").click();
        await page.getByTestId("divider-0").focus();
        await page.keyboard.press("ArrowRight");
        const views = page.getByTestId("object-views");
        await views.getByRole("tab", { name: "blue" }).click();
        await page.getByTestId("close-blue").click();
        await views.getByRole("tab", { name: "locked" }).click();
        await page.evaluate(() => window.__property_workbench.start_load(40, "loaded before a reset"));
        await expect(page.getByTestId("details")).toHaveAttribute("data-state", "ready");

        // Unsaved details with a check still running and a submit waiting on
        // it, which is also a guard on leaving.
        await page.getByTestId("location").fill("shelf 99");
        await page.getByTestId("reference").fill("REF-9999");
        await page.getByTestId("save-details").click({ force: true });

        // An edit session on the region, committed by the palette taking the
        // keyboard, and the palette left open.
        const controls = (await snapshot(page)).controls!;
        const name = {
            x: box.x + controls.name.x + controls.name.width / 2,
            y: box.y + controls.name.y + controls.name.height / 2,
        };
        await page.evaluate(() => window.scrollTo(0, 0));
        await page.mouse.click(name.x, name.y);
        await expect.poll(async () => (await snapshot(page)).editing).toBe(true);
        await page.getByTestId("gpu-name-edit").fill("typed on the region");
        await page.getByTestId("open-commands").click();
        await expect(page.getByTestId("command-palette")).toBeVisible();
        await page.getByTestId("command-search").fill("theme");
        await page.mouse.move(0, 0);

        const changed = await state(page);
        expect(changed.snapshot).toMatchObject({
            count: 1009,
            theme: "dark",
            emphasis: true,
            third_party: false,
            guarded: true,
            workspace: { tabs: 9, tab: "locked", palette: true },
            drag: { propagates: false, grouped: 1 },
            form: { checks: 1, asked: "waiting" },
        });
        expect(changed.snapshot.objects).not.toBe(first.snapshot.objects);
        expect(changed.deleted).toBe("disposed");

        // Then what only the page's handle reaches, all at once so that no
        // region action can land between the last of them and the reset: a
        // second scope, an armed close, a record switched off, an error, a
        // zoomed root, a scrolled page, and an answer still on its way.
        const second = await page.evaluate(async () => {
            const api = window.__property_workbench;
            const second = api.mount_into("second-workbench");
            api.set_diagnostics(false);
            api.hooks.runtime.errors.push("left by a check");
            document.documentElement.style.fontSize = "200%";
            window.scrollTo(0, 400);
            api.start_load(1_000, "asked for before the reset");
            api.close_on_next_action();
            await (api as unknown as { reset(): Promise<void> }).reset();
            return second;
        });
        expect(second).toBeGreaterThan(0);

        await expect.poll(() => state(page), { timeout: 30_000 }).toEqual(first);
        // The answer asked for before the reset has had its time, and did not
        // land in the state put back.
        await page.waitForTimeout(1_200);
        expect(await state(page)).toEqual(first);
        // And the close the check armed went with it: the region's next
        // action selects, and the scope stays.
        await page.mouse.click(next.x, next.y);
        await expect.poll(async () => (await snapshot(page)).selected).toBe(2);
        expect(await page.evaluate(() => window.__property_workbench.live_regions())).toBe(1);
    });
});
