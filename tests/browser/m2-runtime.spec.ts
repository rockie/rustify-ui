import { expect } from "@playwright/test";
import { capture, differingPixels, settle, test, waitForReady } from "./support";
import { EVIDENCE } from "../tier";

// Every region count here is five at rest and three with one counter scope
// gone: the page's own mount is B0 (one region) since P3 M6, and the two
// counter scopes these checks are about (two regions each) are mounted by
// `waitForReady`.
test.describe("M2 V1: mounting", () => {
    test("a missing container fails and leaves the running scopes alone", async ({ page }) => {
        await waitForReady(page);
        const error = await page.evaluate(() => {
            try {
                window.__fusion_basic.mount("not-a-container");
                return null;
            } catch (e) {
                return String(e);
            }
        });
        expect(error).toContain("container not found");
        expect(await page.evaluate(() => window.__fusion_basic.live_regions())).toBe(5);
        await page.getByTestId("scope-a-dom-increment").click();
        await expect(page.getByTestId("scope-a-dom-count")).toHaveText("1");
    });

    test("mounting over a live scope fails and leaves that scope alone", async ({ page }) => {
        await waitForReady(page);
        await page.getByTestId("scope-a-dom-increment").click();
        await expect(page.getByTestId("scope-a-dom-count")).toHaveText("1");
        const error = await page.evaluate(() => {
            try {
                window.__fusion_basic.mount("scope-a");
                return null;
            } catch (e) {
                return String(e);
            }
        });
        expect(error).toContain("already mounted");
        expect(await page.evaluate(() => window.__fusion_basic.live_regions())).toBe(5);
        await expect(page.getByTestId("scope-a-dom-count")).toHaveText("1");
        await page.getByTestId("scope-a-dom-increment").click();
        await expect(page.getByTestId("scope-a-dom-count")).toHaveText("2");
    });

    test("a disposed handle is spent: the second dispose reports nothing to do", async ({ page }) => {
        await waitForReady(page);
        expect(await page.evaluate(() => window.__fusion_basic.dispose("scope-a"))).toBe(true);
        expect(await page.evaluate(() => window.__fusion_basic.dispose("scope-a"))).toBe(false);
        expect(await page.evaluate(() => window.__fusion_basic.live_regions())).toBe(3);
    });
});

test.describe("M2: region state", () => {
    test("every live region reports itself ready", async ({ page }) => {
        await waitForReady(page);
        expect(await page.evaluate(() => window.__fusion_basic.region_states())).toEqual({
            "scope-a-gpu-1": "ready",
            "scope-a-gpu-2": "ready",
            "scope-b-gpu-1": "ready",
            "scope-b-gpu-2": "ready",
        });
        await page.evaluate(() => window.__fusion_basic.dispose("scope-a"));
        expect(await page.evaluate(() => window.__fusion_basic.region_states())).toEqual({
            "scope-b-gpu-1": "ready",
            "scope-b-gpu-2": "ready",
        });
    });

    test("a region denied a GPU context says so and keeps its DOM half working", async ({ page }) => {
        await waitForReady(page);
        expect(await page.evaluate(() => window.__fusion_basic.dispose("scope-a"))).toBe(true);
        await page.evaluate(() => {
            const original = HTMLCanvasElement.prototype.getContext;
            Object.assign(window, {
                __restore_get_context: () => {
                    HTMLCanvasElement.prototype.getContext = original;
                },
            });
            HTMLCanvasElement.prototype.getContext = function (this: HTMLCanvasElement, type: string, ...rest: unknown[]) {
                return type === "webgl2" ? null : (original as Function).call(this, type, ...rest);
            } as typeof original;
        });
        await page.evaluate(() => window.__fusion_basic.mount("scope-a"));
        await expect(page.getByTestId("scope-a-gpu-1-error")).toHaveText(
            "no WebGL2 context for the region canvas"
        );
        expect(await page.evaluate(() => window.__fusion_basic.region_states())).toEqual({
            "scope-a-gpu-1": "failed",
            "scope-a-gpu-2": "failed",
            "scope-b-gpu-1": "ready",
            "scope-b-gpu-2": "ready",
        });
        expect(await page.evaluate(() => window.__fusion_basic.live_regions())).toBe(3);
        // Six, for two regions: a region asks the canvas three times before it
        // gives up, and the host records every refusal it saw. The record the
        // application reads tells the three apart - two `GpuInitRetry` and one
        // `GpuInitFailed` per region - because only the last one is a failure.
        expect(await page.evaluate(() => window.__fusion_basic.errors())).toHaveLength(6);
        await page.getByTestId("scope-a-dom-increment").click();
        await expect(page.getByTestId("scope-a-dom-count")).toHaveText("1");
        await expect(page.getByTestId("scope-b-dom-count")).toHaveText("0");
        await page.evaluate(() => (window as unknown as { __restore_get_context(): void }).__restore_get_context());
    });
});

test.describe("M2 V3: teardown and host coexistence", () => {
    test("closing one scope leaves the other fully operable, DOM and GPU", async ({ page }) => {
        await waitForReady(page);
        const b1 = page.getByTestId("scope-b-gpu-1");
        const b2 = page.getByTestId("scope-b-gpu-2");
        await settle(b1);
        expect(await page.evaluate(() => window.__fusion_basic.dispose("scope-a"))).toBe(true);
        const before1 = await settle(b1);
        const before2 = await settle(b2);
        const box = (await b1.boundingBox())!;
        await page.mouse.click(box.x + box.width / 2, box.y + box.height / 2 + 24);
        await expect(page.getByTestId("scope-b-dom-count")).toHaveText("1");
        await expect
            .poll(async () => differingPixels(before1, await capture(b1)), { timeout: 5_000 })
            .toBeGreaterThan(20);
        await expect
            .poll(async () => differingPixels(before2, await capture(b2)), { timeout: 5_000 })
            .toBeGreaterThan(20);
    });

    test("events aimed at a disposed scope produce no callback and no error", async ({ page }) => {
        const failures: string[] = [];
        page.on("pageerror", (error) => failures.push(String(error)));
        await waitForReady(page);
        const box = (await page.getByTestId("scope-a-gpu-1").boundingBox())!;
        expect(await page.evaluate(() => window.__fusion_basic.dispose("scope-a"))).toBe(true);
        await expect(page.getByTestId("scope-a-gpu-1")).toHaveCount(0);
        // Pointer, keyboard, resize and a full timer window aimed at where the
        // scope used to be.
        await page.mouse.click(box.x + box.width / 2, box.y + box.height / 2 + 24);
        await page.keyboard.press("Tab");
        await page.setViewportSize({ width: 1000, height: 800 });
        await page.waitForTimeout(500);
        expect(failures).toEqual([]);
        expect(await page.evaluate(() => window.__fusion_basic.live_regions())).toBe(3);
        await expect(page.getByTestId("scope-b-dom-count")).toHaveText("0");
    });

    test("idle regions do not wake the runtime", async ({ page }) => {
        await waitForReady(page);
        await settle(page.getByTestId("scope-a-gpu-1"));
        const before = await page.evaluate(() => window.__fusion_basic.stats().pumps);
        await page.waitForTimeout(3_000);
        const after = await page.evaluate(() => window.__fusion_basic.stats().pumps);
        expect(after).toBe(before);
    });

    test("an embedded region takes no page-level state", async ({ page }) => {
        await waitForReady(page);
        const page_state = await page.evaluate(() => ({
            active: document.activeElement?.tagName ?? null,
            title: document.title,
            hash: location.hash,
            history: history.length,
            body_overflow: document.body.style.overflow,
            hidden_textareas: document.querySelectorAll("textarea.cx_webgl_textinput").length,
        }));
        expect(page_state).toEqual({
            active: "BODY",
            title: "fusion-basic",
            hash: "",
            history: page_state.history,
            body_overflow: "",
            hidden_textareas: 0,
        });
        await page.getByTestId("scope-a-dom-increment").click();
        await expect(page.getByTestId("scope-a-dom-count")).toHaveText("1");
        expect(await page.evaluate(() => document.title)).toBe("fusion-basic");
    });

    test("the host page keeps its link, scrolling and text selection", async ({ page }) => {
        await waitForReady(page);
        await page.evaluate(() => window.__fusion_basic.dispose("scope-a"));
        await page.getByTestId("native-link").click();
        expect(await page.evaluate(() => location.hash)).toBe("#anchor");
        await page.setViewportSize({ width: 800, height: 400 });
        await page.evaluate(() => window.scrollTo(0, 200));
        expect(await page.evaluate(() => window.scrollY)).toBeGreaterThan(0);
        const selected = await page.evaluate(() => {
            const heading = document.querySelector("h1")!;
            const range = document.createRange();
            range.selectNodeContents(heading);
            const selection = getSelection()!;
            selection.removeAllRanges();
            selection.addRange(range);
            return selection.toString();
        });
        expect(selected).toBe("fusion-basic");
    });

    test("repeated mount and dispose returns every browser resource", { tag: EVIDENCE }, async ({ page }) => {
        test.setTimeout(600_000);
        const failures: string[] = [];
        page.on("pageerror", (error) => failures.push(String(error)));
        await waitForReady(page);
        const { regions, timers, animation_frames, tasks, errors } = await page.evaluate(() =>
            window.__fusion_basic.stats()
        );
        expect({ regions, timers, animation_frames, tasks, errors }).toEqual({
            regions: 5,
            timers: 0,
            animation_frames: 0,
            tasks: 0,
            errors: 0,
        });
        // The leak sample is the tail: a hundred rounds that have to add
        // nothing. Linear memory never shrinks, so even 8 KiB held per round
        // would show up as a dozen more pages.
        //
        // The warm-up is what the tail has to come after, and four hundred is
        // measured rather than guessed: sampled every fifty rounds over a
        // thousand, this build's working set takes its last step between
        // rounds three hundred and three hundred and fifty and the next six
        // hundred and fifty add nothing. It settled by round one hundred
        // before each fixture kept a way back to its first state, and the
        // warm-up was three hundred then. A warm-up that ends before the
        // working set does measures the working set and calls it a leak.
        const after = await page.evaluate(async () => {
            const api = window.__fusion_basic;
            const round = async () => {
                api.dispose("scope-a");
                api.mount("scope-a");
                await new Promise((r) => setTimeout(r, 20));
            };
            for (let i = 0; i < 400; i++) {
                await round();
            }
            const warm = api.stats();
            for (let i = 0; i < 100; i++) {
                await round();
            }
            api.dispose("scope-a");
            await new Promise((r) => setTimeout(r, 500));
            return {
                warm,
                stats: api.stats(),
                errors: api.errors(),
                canvases: document.querySelectorAll("canvas").length,
            };
        });
        expect(after.errors).toEqual([]);
        expect(after.canvases).toBe(3);
        expect(after.stats).toMatchObject({
            regions: 3,
            timers: 0,
            animation_frames: 0,
            tasks: 0,
            errors: 0,
        });
        expect(after.stats.memory).toBe(after.warm.memory);
        expect(failures).toEqual([]);
        await page.evaluate(() => window.__fusion_basic.mount("scope-a"));
        await expect(page.getByTestId("scope-a-gpu-1")).toHaveCount(1);
        await page.getByTestId("scope-a-dom-increment").click();
        await expect(page.getByTestId("scope-a-dom-count")).toHaveText("1");
        const region = page.getByTestId("scope-a-gpu-1");
        const before = await settle(region);
        await page.getByTestId("scope-a-dom-increment").click();
        await expect
            .poll(async () => differingPixels(before, await capture(region)), { timeout: 5_000 })
            .toBeGreaterThan(20);
    });
});

test.describe("M7 V8: what a trap in the shared module reaches", () => {
    // Its own page: a trap kills the instance, and a shared page's instance
    // is the next check's.
    test.use({ fresh: true });

    test("every mount in the runtime is dead, and the page says which", async ({ page }) => {
        await waitForReady(page);
        // Two scopes, two regions each: the blast radius claim is about all of
        // them, so all of them have to be there to begin with.
        await expect(page.getByTestId("scope-a-dom-count")).toHaveCount(1);
        await expect(page.getByTestId("scope-b-dom-count")).toHaveCount(1);
        expect(await page.evaluate(() => window.__fusion_basic.live_regions())).toBe(5);

        await page.evaluate(() =>
            window.__fusion_basic.hooks.runtime.enter_fatal(new Error("injected trap"))
        );

        // Not one scope: every one of them. A trap is a property of the wasm
        // module, and every mount in this runtime shares it.
        await expect(page.getByTestId("status")).toHaveAttribute("data-status", "fatal");
        for (const id of [
            "scope-a-dom-count",
            "scope-a-dom-increment",
            "scope-a-gpu-1",
            "scope-a-gpu-2",
            "scope-b-dom-count",
            "scope-b-dom-increment",
            "scope-b-gpu-1",
            "scope-b-gpu-2",
        ]) {
            await expect(page.getByTestId(id), id).toHaveCount(0);
        }

        // The notice says what was lost and what to do, in that order.
        const notice = page.getByRole("alert");
        await expect(notice).toContainText("RuntimeFatal");
        await expect(notice).toContainText("reload the page");
        await expect(notice).toContainText("unsaved in-memory state is lost");

        // And the application's own entry points are gone: nothing the page
        // still holds can call back into the module that trapped.
        expect(await page.evaluate(() => "__fusion_basic" in window)).toBe(false);
    });
});
