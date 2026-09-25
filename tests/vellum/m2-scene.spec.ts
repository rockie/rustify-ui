import { capture, expect, present, rgb, test, waitForQuiet, waitForReady } from "./support";
import { rounds } from "../tier";

test("Forma mounts in one GPU region and produces real scene pixels", async ({ page }, info) => {
    await waitForReady(page);
    const state = await page.evaluate(() => ({
        nodes: window.vellum.doc.nodes.length,
        pages: window.vellum.doc.data.pages.map(page => page.nodes.length),
        renderer: { ...window.vellum.renderer },
        regions: window.__vellum.live_regions(),
        stats: window.__vellum.stats(),
        premultipliedAlpha: (document.querySelector("#scene") as HTMLCanvasElement).getContext("webgl2")?.getContextAttributes()?.premultipliedAlpha,
    }));
    expect(state.nodes).toBe(171);
    expect(state.pages).toEqual([171, 31, 0]);
    expect(state.regions).toBe(1);
    expect(state.renderer.instanceCount).toBeGreaterThan(150);
    expect(state.renderer.backend).toBe("Makepad WebGL2");
    expect(state.stats.frames).toBeGreaterThan(0);
    expect(state.premultipliedAlpha).toBe(true);
    const pixels = await capture(page.locator("#scene"));
    const colors = new Set<string>();
    for (let i = 0; i < pixels.data.length; i += 16) colors.add(pixels.data.subarray(i, i + 3).toString("hex"));
    expect(colors.size, "The scene must contain artwork, not a blank canvas").toBeGreaterThan(100);
    console.log(`Forma scene: ${JSON.stringify(state)}`);
    await info.attach("scene-stats", { body: JSON.stringify(state, null, 2), contentType: "application/json" });
    await page.screenshot({ path: info.outputPath("forma.png") });
});

test("5,000 editable primitives render in one draw call with in-page CPU samples", async ({ page }, info) => {
    await waitForReady(page);
    await page.evaluate(() => window.vellum.actions.stressTest());
    await expect.poll(() => page.evaluate(() => window.vellum.renderer.visibleCount)).toBe(5000);
    expect(await page.evaluate(() => window.vellum.doc.nodes.length)).toBe(5000);
    expect(await page.evaluate(() => window.vellum.renderer.drawCalls)).toBe(1);
    await waitForQuiet(page);
    const count = rounds(20);
    const measurements = await page.evaluate(async count => {
        const api = window.vellum;
        const id = api.doc.nodes[0].id;
        api.select([id]);
        const samples: number[] = [];
        const started = performance.now();
        const before = window.__vellum.stats().frames;
        for (let i = 0; i < count; i++) {
            api.setProperty("w", i % 2 === 0 ? 25 : 24);
            await new Promise<void>(resolve => requestAnimationFrame(() => requestAnimationFrame(() => resolve())));
            samples.push(api.renderer.cpuMs);
        }
        const sorted = [...samples].sort((a, b) => a - b);
        return {
            samples,
            p95: sorted[Math.ceil(sorted.length * 0.95) - 1],
            elapsed: performance.now() - started,
            frames: window.__vellum.stats().frames - before,
            width: api.doc.get(id).w,
            selected: [...api.state.selection],
            drawCalls: api.renderer.drawCalls,
        };
    }, count);
    // The last edit sets 25 on an odd count, 24 on an even one.
    expect(measurements.width).toBe(count % 2 === 0 ? 24 : 25);
    expect(measurements.selected).toHaveLength(1);
    expect(measurements.frames).toBeGreaterThan(0);
    expect(measurements.drawCalls).toBe(1);
    expect(measurements.samples.every(value => Number.isFinite(value) && value >= 0)).toBe(true);
    console.log(`5,000 shapes: ${JSON.stringify(measurements)}`);
    await info.attach("stress-measurements", { body: JSON.stringify(measurements, null, 2), contentType: "application/json" });
});

test("GPU texture retention stays bounded after warming twenty zoom levels", async ({ page }, info) => {
    await waitForReady(page);
    const samples = await page.evaluate(async () => {
        const base = window.vellum.state.camera.zoom;
        const levels = Array.from({ length: 20 }, (_, index) => base * 2 ** (index / 5 - 2));
        const cycles: { ledger: number[]; retained: number[] }[] = [];
        for (let round = 0; round < 3; round++) {
            const cycle = { ledger: [] as number[], retained: [] as number[] };
            for (const zoom of levels) {
                window.vellum.zoomAt(zoom / window.vellum.state.camera.zoom);
                await new Promise<void>(resolve => requestAnimationFrame(() => requestAnimationFrame(() => requestAnimationFrame(() => resolve()))));
                while (window.__vellum.rasterStats().pending > 0) await new Promise<void>(resolve => requestAnimationFrame(() => resolve()));
                await new Promise<void>(resolve => requestAnimationFrame(() => requestAnimationFrame(() => resolve())));
                cycle.ledger.push(window.__vellum.stats().gpu_bytes);
                cycle.retained.push(window.vellum.renderer.gpuBytes);
            }
            cycles.push(cycle);
        }
        return cycles;
    });
    const warmPeak = Math.max(...samples[1].ledger);
    expect(warmPeak).toBeGreaterThan(0);
    expect(Math.max(...samples[2].ledger)).toBeLessThanOrEqual(warmPeak);
    expect(samples[2].retained).toEqual(samples[1].retained);
    console.log(`Zoom GPU ledger: ${JSON.stringify(samples)}`);
    await info.attach("zoom-retention", { body: JSON.stringify(samples), contentType: "application/json" });
});

test("premultiplied source-over blending matches two translucent layers", async ({ page }, info) => {
    await waitForReady(page);
    await page.locator("[data-page]").nth(2).click();
    await present(page);
    const canvas = page.locator("#scene");
    const background = await capture(canvas);
    await page.evaluate(() => {
        const api = window.vellum;
        api.createAtCenter("rect", { w: 200, h: 200, fill: "#ff0000", opacity: 0.5, radius: 0 });
        api.createAtCenter("rect", { w: 100, h: 100, fill: "#0000ff", opacity: 0.5, radius: 0 });
        api.select([]);
    });
    await present(page);
    const result = await capture(canvas);
    const x = Math.floor(result.width / 2), y = Math.floor(result.height / 2);
    const base = rgb(background, x, y);
    const actual = rgb(result, x, y);
    const expected = [base[0] * 0.25 + 255 * 0.25, base[1] * 0.25, base[2] * 0.25 + 255 * 0.5];
    for (let channel = 0; channel < 3; channel++) expect(Math.abs(actual[channel] - expected[channel])).toBeLessThanOrEqual(3);
    await canvas.screenshot({ path: info.outputPath("alpha-overlap.png") });
    console.log(`Premultiplied alpha: ${JSON.stringify({ base, actual, expected })}`);
});

test("rounded clipping traverses four frame ancestors", async ({ page }, info) => {
    await waitForReady(page);
    await page.locator("[data-page]").nth(2).click();
    await present(page);
    const background = await capture(page.locator("#scene"));
    await page.evaluate(() => {
        const api = window.vellum;
        const root = api.createAtCenter("frame", { w: 200, h: 200, radius: 60, clip: true, fill: "none" });
        let parent = root.id;
        for (let depth = 0; depth < 3; depth++) {
            const frame = api.createAtCenter("frame", { w: 400, h: 400, clip: true, fill: "none", parentId: parent });
            api.transaction("Place frame", [{ id: frame.id, prop: "x", value: depth === 0 ? -100 : 0 }, { id: frame.id, prop: "y", value: depth === 0 ? -100 : 0 }]);
            parent = frame.id;
        }
        const child = api.createAtCenter("rect", { w: 400, h: 400, fill: "#0000ff", parentId: parent });
        api.transaction("Place child", [{ id: child.id, prop: "x", value: 0 }, { id: child.id, prop: "y", value: 0 }]);
        api.select([]);
    });
    await present(page);
    const result = await capture(page.locator("#scene"));
    const x = Math.floor(result.width / 2), y = Math.floor(result.height / 2);
    expect(rgb(result, x, y)).toEqual([0, 0, 255]);
    expect(rgb(result, x - 95, y - 95)).toEqual(rgb(background, x - 95, y - 95));
    expect(rgb(result, x - 110, y)).toEqual(rgb(background, x - 110, y));
    await page.locator("#scene").screenshot({ path: info.outputPath("nested-round-clips.png") });
});

// The no-GPU mode is read from the address the page loads with.
test.describe(() => {
    test.use({ fresh: true });

    test("GPU-unavailable mode keeps the document and editing controls alive", async ({ page }) => {
        await waitForReady(page, "./?nogpu", false);
        await expect(page.getByRole("alert")).toContainText("GPU unavailable");
        expect(await page.evaluate(() => window.__vellum.live_regions())).toBe(0);
        const count = await page.evaluate(() => {
            const api = window.vellum;
            const node = api.createAtCenter("rect", { w: 80, h: 40 });
            api.select([node.id]);
            api.setProperty("w", 95);
            return { count: api.doc.nodes.length, width: api.doc.get(node.id).w };
        });
        expect(count).toEqual({ count: 172, width: 95 });
    });
});
