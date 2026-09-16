import { readFileSync } from "node:fs";
import path from "node:path";
import { PNG } from "pngjs";
import { capture, differingPixels, expect, present, rgb, test, waitForReady } from "./support";
import { hasReference, openTwin } from "./twin";

const font = `data:font/ttf;base64,${readFileSync(path.resolve(__dirname, "../../makepad/widgets/resources/LiberationMono-Regular.ttf")).toString("base64")}`;

function imageSource(colors: number[][]) {
    const png = new PNG({ width: 300, height: 100 });
    for (let y = 0; y < png.height; y++) {
        for (let x = 0; x < png.width; x++) {
            const color = colors[Math.floor(x / 100) % colors.length];
            const offset = (y * png.width + x) * 4;
            png.data.set([...color, 255], offset);
        }
    }
    return `data:image/png;base64,${PNG.sync.write(png).toString("base64")}`;
}

async function playground(page: import("@playwright/test").Page) {
    await waitForReady(page);
    await page.locator("[data-page]").nth(2).click();
    await present(page);
}

async function textPixels(page: import("@playwright/test").Page, width: number, height: number) {
    const box = (await page.locator("#scene").boundingBox())!;
    return PNG.sync.read(await page.screenshot({ clip: {
        x: box.x + (box.width - width) / 2,
        y: box.y + (box.height - height) / 2,
        width, height,
    } }));
}

test("typography properties bind and Unicode layout matches the browser reference", async ({ page, context }, info) => {
    test.skip(!hasReference, "The optional Vellum source reference is absent.");
    await playground(page);
    const twin = await context.newPage();
    await openTwin(twin);
    const nodes = await page.evaluate(() => {
        const api = window.vellum;
        const first = api.createAtCenter("text", { text: "Vellum · Typography\nZażółć gęślą jaźń · مرحبا", w: 340, h: 80, fontSize: 24 });
        api.select([first.id]);
        for (const [prop, value] of Object.entries({ fontSize: 32, fontWeight: 700, letterSpacing: 1.2, lineHeight: 160, textAlign: "center", direction: "rtl", textDecoration: "underline" })) api.setProperty(prop, value);
        const values = [api.doc.get(first.id)];
        for (const [text, textCase, width] of [
            ["👨‍👩‍👧‍👦 é 🏳️‍🌈\n\n日本語の長い文章", "none", 55],
            ["straße istanbul ΣΊΣΥΦΟΣ", "upper", 110],
            ["éCOLE DÉJÀ vu مرحبا", "title", 120],
            ["WORDS\u00a0AND\u2003SPACES\n", "lower", 85],
        ] as const) values.push(api.createAtCenter("text", { text, textCase, w: width, h: 500, fontSize: 23, letterSpacing: 0.7 }));
        api.select([]);
        return values;
    });
    expect(nodes[0]).toMatchObject({ fontSize: 32, fontWeight: 700, letterSpacing: 1.2, lineHeight: 1.6, textAlign: "center", direction: "rtl", textDecoration: "underline" });
    const actual = await page.evaluate(nodes => nodes.map(node => window.vellum.textLayout(node.id)), nodes);
    const expected = await twin.evaluate(async nodes => {
        const source = "/src/renderer.js";
        const { layoutText, displayText, fontSpec } = await import(source);
        return nodes.map(node => ({ ...layoutText(node), displayText: displayText(node), fontSpec: fontSpec(node) }));
    }, nodes);
    for (let index = 0; index < nodes.length; index++) {
        expect(actual[index]).toMatchObject(expected[index]);
        expect(actual[index].widths.every((value: number) => Number.isFinite(value))).toBe(true);
    }
    await info.attach("unicode-layout", { body: JSON.stringify({ actual, expected }), contentType: "application/json" });
    await twin.close();
});

test("editing a new history branch cannot reuse stale text pixels", async ({ page }) => {
    await playground(page);
    const node = await page.evaluate(() => window.vellum.createAtCenter("text", { text: "AAAA", fontSize: 72, fontFamily: "Arial", w: 400, h: 130, fill: "#ffffff" }));
    await present(page);
    await page.evaluate(id => { window.vellum.select([id]); window.vellum.setProperty("text", "BBBB"); }, node.id);
    await present(page);
    await page.evaluate(() => window.vellum.actions.undo());
    await present(page);
    await page.evaluate(() => { window.vellum.setProperty("text", "CCCC"); window.vellum.select([]); });
    await present(page);
    const branch = await textPixels(page, 400, 130);
    await page.evaluate(id => {
        const api = window.vellum;
        api.select([id]); api.actions.delete();
        api.createAtCenter("text", { text: "CCCC", fontSize: 72, fontFamily: "Arial", w: 400, h: 130, fill: "#ffffff" });
        api.select([]);
    }, node.id);
    await present(page);
    const fresh = await textPixels(page, 400, 130);
    expect(differingPixels(branch, fresh), "Branch C must render identically to a fresh C node").toBe(0);
});

test("styled text raster stays within the approved visual tolerance at unit zoom", async ({ page, context }, info) => {
    test.skip(!hasReference, "The optional Vellum source reference is absent.");
    await playground(page);
    const twin = await context.newPage();
    await openTwin(twin);
    await twin.locator("[data-page]").nth(2).click();
    for (const target of [page, twin]) {
        await target.evaluate(() => {
            window.vellum.createAtCenter("text", {
                text: "Zażółć gęślą jaźń\nمرحبا بالعالم\nCafé 👨‍👩‍👧‍👦", w: 500, h: 240,
                fontFamily: "Arial", fontSize: 30, fontWeight: 700, fontStyle: "italic",
                textCase: "upper", textAlign: "center", direction: "rtl", letterSpacing: 1.2,
                lineHeight: 1.6, textDecoration: "underline", fill: "#f0c090", fillOpacity: 0.8,
            });
            window.vellum.select([]);
        });
        await present(target);
    }
    const captureText = async (target: import("@playwright/test").Page, name: string) => {
        const bounds = await target.locator("#scene").boundingBox();
        if (!bounds) throw new Error("The scene has no visible bounds");
        const png = await target.screenshot({
            animations: "disabled",
            path: info.outputPath(name),
            clip: {
                x: bounds.x + (bounds.width - 500) / 2,
                y: bounds.y + (bounds.height - 240) / 2,
                width: 500,
                height: 240,
            },
        });
        return PNG.sync.read(png);
    };
    const actual = await captureText(page, "text-crop-actual.png");
    const expected = await captureText(twin, "text-crop-reference.png");
    const count = differingPixels(actual, expected);
    await page.locator("#scene").screenshot({ path: info.outputPath("text-actual.png") });
    await twin.locator("#scene").screenshot({ path: info.outputPath("text-reference.png") });
    const comparison = { differing: count, total: actual.width * actual.height, ratio: count / (actual.width * actual.height), maxRatio: 0.02 };
    console.log(`Styled text comparison: ${JSON.stringify(comparison)}`);
    await info.attach("styled-text-comparison", { body: JSON.stringify(comparison), contentType: "application/json" });
    // Canvas readback uses software antialiasing; the reference may rasterize on the GPU.
    // Layout is checked exactly above; rendered pixels use the approved screenshot gate.
    expect(comparison.ratio, "Styled text must satisfy the same visual gate as the full editor").toBeLessThanOrEqual(comparison.maxRatio);
    await twin.close();
});

test("local font loading works under strict CSP and invalidates existing glyph pixels", async ({ page }, info) => {
    await playground(page);
    const node = await page.evaluate(() => window.vellum.createAtCenter("text", { text: "WWW iii 123", fontSize: 48, fontFamily: "Arial", w: 600, h: 100, fill: "#ffffff" }));
    const before = await page.evaluate(id => window.vellum.textLayout(id), node.id);
    await present(page);
    const oldPixels = await capture(page.locator("#scene"));
    const imported = await page.evaluate(dataUrl => window.vellum.importFont({ name: "Vellum Test Mono.ttf", dataUrl }), font);
    expect(imported.family).toBe("Vellum Test Mono");
    const state = await page.evaluate(async id => ({
        font: await window.vellum.fontReady(),
        node: window.vellum.doc.get(id),
        layout: window.vellum.textLayout(id),
        stored: JSON.parse(window.vellum.doc.serialize()).fonts,
        loaded: document.fonts.check('48px "Vellum Test Mono"'),
    }), node.id);
    expect(state.font.pending).toBe(0);
    expect(state.font.families).toContain("Vellum Test Mono");
    expect(state.loaded).toBe(true);
    expect(state.node.fontFamily).toBe("Vellum Test Mono");
    expect(state.stored["Vellum Test Mono"]).toBe(font);
    expect(state.layout.widths).not.toEqual(before.widths);
    await present(page);
    expect(differingPixels(oldPixels, await capture(page.locator("#scene")))).toBeGreaterThan(100);
    await page.evaluate(() => window.vellum.loadStoredFonts());
    expect(await page.evaluate(() => window.vellum.fontReady())).toMatchObject({ pending: 0, errors: [] });
    await info.attach("font-measurement", { body: JSON.stringify({ before, after: state.layout }), contentType: "application/json" });
});

test("decoded images cover the box and replacing an asset refreshes the same GPU key", async ({ page }) => {
    await playground(page);
    const before = await capture(page.locator("#scene"));
    const stripes = imageSource([[255, 0, 0], [0, 255, 0], [0, 0, 255]]);
    const replacement = imageSource([[255, 0, 255]]);
    await page.evaluate(source => window.vellum.setAsset({ id: "cover-test", source }), stripes);
    const node = await page.evaluate(() => {
        const node = window.vellum.createAtCenter("image", { assetId: "cover-test", w: 100, h: 100, radius: 20 });
        window.vellum.select([]); return node;
    });
    await present(page);
    const image = await capture(page.locator("#scene"));
    const x = Math.floor(image.width / 2), y = Math.floor(image.height / 2);
    expect(rgb(image, x - 40, y)).toEqual([0, 255, 0]);
    expect(rgb(image, x + 40, y)).toEqual([0, 255, 0]);
    expect(rgb(image, x - 49, y - 49)).toEqual(rgb(before, x - 49, y - 49));
    await page.evaluate(source => window.vellum.setAsset({ id: "cover-test", source }), replacement);
    await present(page);
    expect(await page.evaluate(id => window.vellum.doc.get(id).version, node.id)).toBe(node.version);
    expect(rgb(await capture(page.locator("#scene")), x, y)).toEqual([255, 0, 255]);
});

test("the browser raster LRU evicts pixels at the 64 MiB budget", async ({ page }, info) => {
    await playground(page);
    await page.evaluate(() => {
        const api = window.vellum;
        for (let index = 0; index < 6; index++) api.createAtCenter("text", { text: `Cache ${index}`, w: 2044, h: 1800, fontSize: 120 });
        api.select([]);
    });
    await present(page);
    const stats = await page.evaluate(() => window.__vellum.rasterStats());
    expect(stats.bytes).toBeLessThanOrEqual(64 * 1024 * 1024);
    expect(stats.bytes).toBeGreaterThan(0);
    expect(stats.evictions).toBeGreaterThan(0);
    expect(stats.totalGenerated).toBeGreaterThanOrEqual(6);
    await info.attach("raster-budget", { body: JSON.stringify(stats), contentType: "application/json" });
});

test("resolution boundaries record actual raster work and page presentation time", async ({ page }, info) => {
    await waitForReady(page);
    const samples = await page.evaluate(async () => {
        const samples = [];
        for (const zoom of [0.49, 0.51, 0.99, 1.01, 1.99, 2.01, 3.99, 4.01]) {
            const beforeFrame = window.__vellum.stats().frames;
            const started = performance.now();
            window.vellum.zoomAt(zoom / window.vellum.state.camera.zoom);
            const rafIntervals: number[] = [];
            let previous = started;
            let elapsedMs = 0;
            for (let frame = 0; frame < 60; frame++) {
                await new Promise<void>(resolve => requestAnimationFrame(() => resolve()));
                const current = performance.now();
                rafIntervals.push(current - previous);
                previous = current;
                if (window.__vellum.stats().frames > beforeFrame) { elapsedMs = current - started; break; }
            }
            if (!elapsedMs) throw new Error("Zoom did not produce a presentation frame");
            const batches = [window.__vellum.rasterStats()];
            while (window.__vellum.rasterStats().pending > 0) {
                if (performance.now() - started > 30_000) throw new Error("Zoom refinement did not finish");
                await new Promise<void>(resolve => requestAnimationFrame(() => resolve()));
                batches.push(window.__vellum.rasterStats());
            }
            await new Promise<void>(resolve => requestAnimationFrame(() => requestAnimationFrame(() => resolve())));
            samples.push({ zoom, elapsedMs, settledMs: performance.now() - started, rafIntervals, batches, frames: window.__vellum.stats().frames - beforeFrame, cpuMs: window.vellum.renderer.cpuMs, ...window.__vellum.rasterStats() });
        }
        return samples;
    });
    expect(samples.map(sample => sample.lastResolution)).toEqual([0.5, 1, 1, 2, 2, 4, 4, 4]);
    expect(samples.some(sample => sample.lastGenerated > 0)).toBe(true);
    expect(samples.some(sample => sample.deferred > 0), "Cold zoom rasters should be spread across presentation frames").toBe(true);
    expect(samples.every(sample => sample.pending === 0)).toBe(true);
    expect(samples.every(sample => sample.lastProjectMs >= 0 && sample.elapsedMs > 0)).toBe(true);
    console.log(`Raster resolution samples: ${JSON.stringify(samples)}`);
    await info.attach("resolution-boundaries", { body: JSON.stringify(samples), contentType: "application/json" });
});
