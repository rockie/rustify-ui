import { expect, test } from "@playwright/test";
import { capture, differingPixels, litPixels, settle, waitForReady } from "./support";

test.describe("M1 probe 1: Leptos CSR and Makepad in one wasm, no isolation", () => {
    test("boots without cross-origin isolation and draws into the region", async ({ page }) => {
        await waitForReady(page);
        const isolated = await page.evaluate(() => ({
            crossOriginIsolated: globalThis.crossOriginIsolated,
            shared: typeof SharedArrayBuffer !== "undefined",
        }));
        expect(isolated.crossOriginIsolated).toBe(false);
        expect(isolated.shared).toBe(false);
        const region = page.getByTestId("scope-a-gpu-1");
        const pixels = await settle(region);
        expect(litPixels(pixels)).toBeGreaterThan(50);
    });

    test("a DOM button changes the GPU label", async ({ page }) => {
        await waitForReady(page);
        const region = page.getByTestId("scope-a-gpu-1");
        const before = await settle(region);
        await page.getByTestId("scope-a-dom-increment").click();
        await expect(page.getByTestId("scope-a-dom-count")).toHaveText("1");
        await expect
            .poll(async () => differingPixels(before, await capture(region)), { timeout: 5_000 })
            .toBeGreaterThan(20);
    });

    test("a GPU button changes the DOM count", async ({ page }) => {
        await waitForReady(page);
        const region = page.getByTestId("scope-a-gpu-1");
        await settle(region);
        const box = (await region.boundingBox())!;
        await page.mouse.click(box.x + box.width / 2, box.y + box.height / 2 + 24);
        await expect(page.getByTestId("scope-a-dom-count")).toHaveText("1");
    });
});

test.describe("M1 probe 2: two scopes, four regions, symmetric teardown", () => {
    test("regions in different scopes do not share state", async ({ page }) => {
        await waitForReady(page);
        expect(await page.evaluate(() => window.__fusion_basic.live_regions())).toBe(4);
        const a2 = page.getByTestId("scope-a-gpu-2");
        const b1 = page.getByTestId("scope-b-gpu-1");
        const a2Before = await settle(a2);
        const b1Before = await settle(b1);
        await page.getByTestId("scope-b-dom-increment").click();
        await expect(page.getByTestId("scope-b-dom-count")).toHaveText("1");
        await expect(page.getByTestId("scope-a-dom-count")).toHaveText("0");
        await expect
            .poll(async () => differingPixels(b1Before, await capture(b1)), { timeout: 5_000 })
            .toBeGreaterThan(20);
        await page.waitForTimeout(300);
        expect(differingPixels(a2Before, await capture(a2))).toBe(0);
    });

    test("disposing one scope leaves the other usable and releases its regions", async ({ page }) => {
        await waitForReady(page);
        const b1 = page.getByTestId("scope-b-gpu-1");
        await settle(b1);
        expect(await page.evaluate(() => window.__fusion_basic.dispose("scope-a"))).toBe(true);
        await expect(page.getByTestId("scope-a-gpu-1")).toHaveCount(0);
        expect(await page.evaluate(() => window.__fusion_basic.live_regions())).toBe(2);
        const before = await settle(b1);
        await page.getByTestId("scope-b-dom-increment").click();
        await expect(page.getByTestId("scope-b-dom-count")).toHaveText("1");
        await expect
            .poll(async () => differingPixels(before, await capture(b1)), { timeout: 5_000 })
            .toBeGreaterThan(20);
        const errors: string[] = [];
        page.on("pageerror", (error) => errors.push(String(error)));
        await page.waitForTimeout(500);
        expect(errors).toEqual([]);
    });

    test("disposing right after mounting leaves no region and no late callback", async ({ page }) => {
        await waitForReady(page);
        const errors: string[] = [];
        page.on("pageerror", (error) => errors.push(String(error)));
        for (const delay of [0, 5, 50]) {
            const live = await page.evaluate(async (delay) => {
                const api = window.__fusion_basic;
                api.dispose("scope-a");
                await new Promise((r) => setTimeout(r, 100));
                api.mount("scope-a");
                if (delay > 0) {
                    await new Promise((r) => setTimeout(r, delay));
                }
                api.dispose("scope-a");
                await new Promise((r) => setTimeout(r, 300));
                return api.live_regions();
            }, delay);
            expect(live).toBe(2);
        }
        expect(errors).toEqual([]);
        await page.evaluate(() => window.__fusion_basic.mount("scope-a"));
        await expect(page.getByTestId("scope-a-gpu-1")).toHaveCount(1);
    });

    test("mount and dispose repeat without leaking regions", async ({ page }) => {
        test.setTimeout(180_000);
        await waitForReady(page);
        const live = () => page.evaluate(() => window.__fusion_basic.live_regions());
        for (let i = 0; i < 20; i++) {
            expect(await page.evaluate(() => window.__fusion_basic.dispose("scope-a"))).toBe(true);
            await expect.poll(live, { timeout: 5_000 }).toBe(2);
            await page.evaluate(() => window.__fusion_basic.mount("scope-a"));
            await expect(page.getByTestId("scope-a-gpu-1")).toHaveCount(1);
            await expect.poll(live, { timeout: 5_000 }).toBe(4);
        }
        const region = page.getByTestId("scope-a-gpu-1");
        const pixels = await settle(region);
        expect(litPixels(pixels)).toBeGreaterThan(50);
    });
});

test.describe("M1 probe 3: static message bridge under a strict CSP", () => {
    test("no dynamic JS execution is needed and the bridge hash matches", async ({ page }) => {
        const violations: string[] = [];
        page.on("console", (message) => {
            if (message.text().includes("Content Security Policy")) {
                violations.push(message.text());
            }
        });
        await waitForReady(page);
        expect(violations).toEqual([]);
        const bridge = await page.evaluate(async () => {
            const module = await import("./rustify_makepad/message_bridge.js");
            return { hash: module.SCHEMA_HASH, hasFactory: typeof module.create_message_classes === "function" };
        });
        expect(bridge.hasFactory).toBe(true);
        expect(bridge.hash).toMatch(/^\d+$/);
        const manifest = await (await page.request.get("./build-manifest.json")).json();
        expect(manifest.schema_hash).toBe(bridge.hash);
        const shipped = await (await page.request.get("./makepad_wasm_bridge/wasm_bridge.js")).text();
        expect(shipped).not.toContain("new Function");
        expect(shipped).not.toContain("eval(");
    });
});

declare global {
    interface Window {
        __fusion_basic: {
            mount(container_id: string): number;
            dispose(container_id: string): boolean;
            live_regions(): number;
        };
    }
}
