import { expect, Page } from "@playwright/test";
import { capture, differingPixels, isShared, settle, test, waitForReady } from "./support";
import { rounds } from "../tier";

test.describe("M3 V2: shared state and controlled GPU components", () => {
    test("interleaved DOM and GPU actions each land exactly once", async ({ page }) => {
        test.setTimeout(300_000);
        await waitForReady(page);
        const region = page.getByTestId("scope-a-gpu-1");
        const total = rounds(20);
        for (let round = 0; round < total; round++) {
            // Into view, then measured, every round: the canvas starts below
            // the fold on this page and clicking the DOM button scrolls it
            // again, and a pointer aimed outside the viewport - or at where
            // the canvas used to be - lands on nothing at all, silently.
            await region.scrollIntoViewIfNeeded();
            const box = (await region.boundingBox())!;
            await page.mouse.click(box.x + box.width / 2, box.y + box.height / 2 + 24);
            await page.getByTestId("scope-a-dom-increment").click();
        }
        // One authoritative value: both paths write the same signal, so the
        // count is exactly the number of accepted actions.
        await expect(page.getByTestId("scope-a-dom-count")).toHaveText(String(total * 2));
        await expect(page.getByTestId("scope-b-dom-count")).toHaveText("0");
    });

    test("both regions of a scope show the current value, not a stale one", async ({ page }) => {
        await waitForReady(page);
        const first = page.getByTestId("scope-a-gpu-1");
        const second = page.getByTestId("scope-a-gpu-2");
        const beforeFirst = await settle(first);
        const beforeSecond = await settle(second);
        for (let click = 0; click < 3; click++) {
            await page.getByTestId("scope-a-dom-increment").click();
        }
        await expect(page.getByTestId("scope-a-dom-count")).toHaveText("3");
        await expect
            .poll(async () => differingPixels(beforeFirst, await capture(first)), { timeout: 5_000 })
            .toBeGreaterThan(20);
        await expect
            .poll(async () => differingPixels(beforeSecond, await capture(second)), { timeout: 5_000 })
            .toBeGreaterThan(20);
        // The two regions project the same value, so they end up identical.
        expect(differingPixels(await capture(first), await capture(second))).toBe(0);
    });
});

/// Everything a check can read back about where the page is: the application's
/// state in both halves, which scopes and regions are alive and in what state,
/// the address bar, and what the page around them holds.
const snapshot = (page: Page) =>
    page.evaluate(() => {
        const fusion = window.__fusion_basic;
        const text = (id: string) => document.querySelector(`[data-testid="${id}"]`)?.textContent ?? null;
        return {
            b0: fusion.b0_state(),
            counters: { a: text("scope-a-dom-count"), b: text("scope-b-dom-count") },
            regions: fusion.region_states(),
            live_regions: fusion.live_regions(),
            live_components: fusion.live_components(),
            routes: fusion.routes(),
            url: location.pathname + location.search + location.hash,
            history_state: history.state,
            url_owner: document.documentElement.getAttribute("data-rustify-url-owner"),
            errors: fusion.errors(),
            scopes: [...document.querySelectorAll("[data-rustify-scope]")].map((element) => element.id),
            containers: Object.fromEntries(
                [...document.querySelectorAll("section > div[id]")].map((element) => [
                    element.id,
                    element.childElementCount,
                ])
            ),
            canvases: document.querySelectorAll("canvas").length,
            inline_styles: document.querySelectorAll("[style]").length,
            scroll: [scrollX, scrollY],
            selection: getSelection()?.toString() ?? "",
            active: document.activeElement?.tagName ?? null,
            probes: Object.keys(window).filter((name) => name.startsWith("__probe")),
            native_get_context: HTMLCanvasElement.prototype.getContext.toString().includes("[native code]"),
        };
    });

/// The page's own reset, which the shared page runs before every check.
const reset = (page: Page) =>
    page.evaluate(() => (window.__fusion_basic as unknown as { reset(): Promise<void> }).reset());

/// The canvases of the scopes a reset keeps.
const KEPT_CANVASES = "#b0 canvas, #scope-a canvas, #scope-b canvas";

test.describe("the page a check starts on", () => {
    test("a reset puts back the page as it loads, and keeps its regions", async ({ page, browser }, info) => {
        test.setTimeout(300_000);

        // What a load looks like, on a page of its own that nothing else has
        // touched.
        const context = await browser.newContext({
            baseURL: info.project.use.baseURL,
            viewport: page.viewportSize(),
        });
        const loaded = await context.newPage();
        await waitForReady(loaded);
        const expected = await snapshot(loaded);
        await context.close();

        // The page under test is the one the checks share, where
        // `waitForReady` does not load anything: a reset that left something
        // behind shows up here and nowhere else.
        expect(isShared(page), "the check runs on the shared page").toBe(true);
        await waitForReady(page);
        expect(await snapshot(page)).toEqual(expected);
        await page.evaluate((selector) => {
            for (const canvas of document.querySelectorAll(selector)) {
                (canvas as unknown as { kept: boolean }).kept = true;
            }
        }, KEPT_CANVASES);

        // The counters, from the DOM half and the GPU half.
        const first = page.getByTestId("scope-a-gpu-1");
        const second = page.getByTestId("scope-a-gpu-2");
        await page.getByTestId("scope-a-dom-increment").click();
        await page.getByTestId("scope-b-dom-increment").click();
        await first.scrollIntoViewIfNeeded();
        const box = (await first.boundingBox())!;
        await page.mouse.click(box.x + box.width / 2, box.y + box.height / 2 + 24);
        await expect(page.getByTestId("scope-a-dom-count")).toHaveText("2");
        await expect(page.getByTestId("scope-b-dom-count")).toHaveText("1");
        // The press left its button focused, and the region's twin shows the
        // same count without it.
        await page.getByTestId("scope-a-dom-increment").hover();
        await expect
            .poll(async () => differingPixels(await settle(first), await settle(second)), { timeout: 10_000 })
            .toBeGreaterThan(0);

        // B0's one state, field by field.
        await page.getByTestId("b0-bump").click();
        await page.getByTestId("b0-title").fill("edited");
        await page.getByTestId("b0-visible").click();
        await page.getByTestId("b0-size").focus();
        await page.keyboard.press("ArrowRight");
        await expect
            .poll(async () => page.evaluate(() => window.__fusion_basic.b0_state()))
            .toMatchObject({ count: 1, title: "edited", visible: false, size: 45 });

        // A scope the page does not load with, and what a check does to it.
        await page.evaluate(() => window.__fusion_basic.mount_geometry("geometry"));
        await expect
            .poll(async () => page.evaluate(() => window.__fusion_basic.geometry().state), { timeout: 60_000 })
            .toBe("ready");
        await page.evaluate(() => {
            (document.querySelector('[data-testid="geometry-gpu"]') as HTMLElement).style.width = "300px";
            (document.querySelector('[data-testid="geometry-inner"]') as HTMLElement).scrollTop = 40;
        });

        // An owner of the address bar that has moved, an entry the page pushed
        // itself, and a guard that refuses to leave.
        await page.evaluate(() => {
            window.__fusion_basic.mount_owner("route-owner");
            window.__fusion_basic.mount_guest("route-guest");
        });
        await page.getByTestId("route-owner-one").click();
        await expect.poll(async () => new URL(page.url()).pathname).toBe("/one");
        await page.evaluate(() => {
            history.pushState({ theirs: true }, "", "?pushed#here");
            window.__fusion_basic.set_guard(true);
        });
        expect(await page.evaluate(() => document.documentElement.getAttribute("data-rustify-url-owner"))).toBe(
            "1:route-owner"
        );

        // And the page around the application.
        await page.evaluate(() => {
            window.scrollTo(0, 300);
            const range = document.createRange();
            range.selectNodeContents(document.querySelector("h1")!);
            getSelection()!.addRange(range);
            document.body.style.overflow = "hidden";
            window.__fusion_basic.hooks.runtime.errors.push("a check's error");
            const original = HTMLCanvasElement.prototype.getContext;
            HTMLCanvasElement.prototype.getContext = function (this: HTMLCanvasElement, ...args: unknown[]) {
                return (original as Function).apply(this, args);
            } as typeof original;
            const probe = new AbortController();
            addEventListener("popstate", () => {}, { signal: probe.signal });
            Object.assign(window, { __probe_listening: probe, __probe: [] });
        });
        await page.getByTestId("focus-01").focus();
        expect(await snapshot(page)).not.toEqual(expected);

        await reset(page);
        await waitForReady(page);
        expect(await snapshot(page)).toEqual(expected);

        // The regions are the ones that were there: nothing started again.
        const kept = await page.evaluate(
            (selector) =>
                [...document.querySelectorAll(selector)].map(
                    (canvas) => (canvas as unknown as { kept?: boolean }).kept === true
                ),
            KEPT_CANVASES
        );
        expect(kept).toEqual([true, true, true, true, true]);
        // And they draw what they were handed back. The two regions of a scope
        // show one value and are the same picture - unless the one whose
        // button was pressed still shows that press as focus.
        await expect
            .poll(async () => differingPixels(await settle(first), await settle(second)), { timeout: 10_000 })
            .toBe(0);
    });
});
