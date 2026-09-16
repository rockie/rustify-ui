import { createHash } from "node:crypto";
import { mkdir, readFile, writeFile } from "node:fs/promises";
import path from "node:path";
import { expect, test, type Page } from "@playwright/test";
import { PNG } from "pngjs";
import { differingPixels, present, waitForQuiet } from "./support";
import { hasReference } from "./twin";

// This probe needs Chrome's actual GPU selection, without the suite's SwiftShader overrides.
test.use({ channel: "chrome", headless: false, launchOptions: { args: [] } });
test.skip(!hasReference || process.env.VELLUM_WEBGPU_PROBE !== "1", "Run the optional headed Chrome probe with VELLUM_WEBGPU_PROBE=1 and the local reference source.");

const reference = "http://127.0.0.1:4180/";
const fixture = path.resolve(__dirname, "fixtures/forma.vellum");
const reportPath = path.resolve(__dirname, "../../test-results/vellum/m3-webgpu.json");
type Theme = "light" | "dark";
type Camera = { x: number; y: number; zoom: number };

function monitor(page: Page) {
    const errors: string[] = [];
    const consoleMessages: string[] = [];
    page.on("pageerror", error => errors.push(error.message));
    page.on("console", message => {
        consoleMessages.push(`${message.type()}: ${message.text()}`);
        if (message.type() === "error") errors.push(message.text());
    });
    return { errors, consoleMessages };
}

async function initialize(page: Page, url: string, documentName: string, pageId: string) {
    await page.bringToFront();
    await page.goto(url);
    await page.waitForFunction(() => window.vellum?.ready);
    await page.locator("#file-input").setInputFiles(fixture);
    await expect.poll(() => page.evaluate(() => ({ name: window.vellum.doc.data.name, pageId: window.vellum.doc.data.pageId }))).toEqual({ name: documentName, pageId });
    await page.evaluate(() => document.fonts.ready.then(() => undefined));
    await page.bringToFront();
    await present(page);
    await waitForQuiet(page);
}

async function prepare(page: Page, index: number, theme: Theme, camera?: Camera) {
    await page.bringToFront();
    await page.locator("[data-page]").nth(index).click();
    await page.evaluate(({ theme, camera }) => {
        const api = window.vellum;
        if (document.documentElement.dataset.theme !== theme) api.actions.theme();
        api.select([]);
        api.state.hover = null;
        api.fit();
        if (camera) Object.assign(api.state.camera, camera);
        for (const id of ["welcome-tip", "toast"]) {
            const element = document.getElementById(id);
            if (element) element.style.display = "none";
        }
        api.render();
    }, { theme, camera });
    await present(page);
    await waitForQuiet(page);
    await page.evaluate(() => {
        for (const id of ["engine-label", "performance"]) {
            const element = document.getElementById(id);
            if (element) element.textContent = "Renderer";
        }
    });
}

async function state(page: Page) {
    return page.evaluate(() => {
        const api = window.vellum;
        const area = document.getElementById("canvas-area")!.getBoundingClientRect();
        return {
            backend: api.renderer.backend,
            gpuError: api.renderer.gpuError ?? null,
            adapter: api.renderer.adapterInfo ?? null,
            camera: { ...api.state.camera },
            viewport: { width: window.innerWidth, height: window.innerHeight, dpr: window.devicePixelRatio },
            canvasArea: { x: area.x, y: area.y, width: area.width, height: area.height },
            pageId: api.doc.data.pageId,
            pageName: api.doc.page.name,
            nodes: api.doc.nodes.length,
            document: api.doc.serialize(),
            userAgent: navigator.userAgent,
            secureContext: window.isSecureContext,
            navigatorGpu: "gpu" in navigator,
        };
    });
}

function difference(actual: Buffer, reference: Buffer) {
    const a = PNG.sync.read(actual), b = PNG.sync.read(reference);
    const differing = differingPixels(a, b);
    const total = a.width * a.height;
    return { differing, total, ratio: differing / total, width: a.width, height: a.height };
}

function hash(text: string) {
    return createHash("sha256").update(text).digest("hex");
}

test("headed Chrome records original WebGPU versus Canvas rendering on identical starter pages", async ({ browser, page: gpu, context }, info) => {
    test.skip(!hasReference, "The optional Vellum source reference is absent.");
    test.setTimeout(300_000);
    const goldenText = await readFile(fixture, "utf8");
    const golden = JSON.parse(goldenText) as { name: string; pageId: string };
    const canvas = await context.newPage();
    const gpuEvents = monitor(gpu), canvasEvents = monitor(canvas);
    const report = {
        browser: { channel: "chrome", version: browser.version(), headed: true, launchArgs: [] as string[] },
        referenceUrl: reference,
        fixture: { path: fixture, sha256: hash(goldenText) },
        threshold: 24,
        visualGate: null,
        status: "incomplete",
        results: [] as Record<string, unknown>[],
        console: { webgpu: gpuEvents, canvas: canvasEvents },
    };
    try {
        await initialize(gpu, reference, golden.name, golden.pageId);
        await initialize(canvas, `${reference}?canvas`, golden.name, golden.pageId);
        for (const index of [0, 1, 2]) {
            for (const theme of ["light", "dark"] as const) {
                await prepare(gpu, index, theme);
                const gpuState = await state(gpu);
                await prepare(canvas, index, theme, gpuState.camera);
                const canvasState = await state(canvas);
                expect(canvasState.camera).toEqual(gpuState.camera);
                expect(canvasState.viewport).toEqual(gpuState.viewport);
                expect(canvasState.canvasArea).toEqual(gpuState.canvasArea);
                expect(hash(canvasState.document), "Both renderers must use the same imported document").toBe(hash(gpuState.document));

                const paths = {
                    webgpu: info.outputPath(`original-webgpu-${index}-${theme}.png`),
                    canvas: info.outputPath(`original-canvas-${index}-${theme}.png`),
                    webgpuScene: info.outputPath(`original-webgpu-scene-${index}-${theme}.png`),
                    canvasScene: info.outputPath(`original-canvas-scene-${index}-${theme}.png`),
                };
                await gpu.bringToFront();
                await present(gpu);
                const gpuFull = await gpu.screenshot({ path: paths.webgpu, animations: "disabled" });
                const gpuScene = await gpu.locator("#canvas-area").screenshot({ path: paths.webgpuScene, animations: "disabled" });
                await canvas.bringToFront();
                await present(canvas);
                const canvasFull = await canvas.screenshot({ path: paths.canvas, animations: "disabled" });
                const canvasScene = await canvas.locator("#canvas-area").screenshot({ path: paths.canvasScene, animations: "disabled" });
                const { document: gpuDocument, ...webgpu } = gpuState;
                const { document: canvasDocument, ...canvas2d } = canvasState;
                const result = {
                    page: index,
                    theme,
                    webgpu: { ...webgpu, documentSha256: hash(gpuDocument) },
                    canvas: { ...canvas2d, documentSha256: hash(canvasDocument) },
                    fullPage: difference(gpuFull, canvasFull),
                    scene: difference(gpuScene, canvasScene),
                    screenshots: paths,
                };
                report.results.push(result);
                console.log(`Original WebGPU / Canvas: ${JSON.stringify(result)}`);
            }
        }
        const gpuBackend = await gpu.evaluate(() => window.vellum.renderer.backend);
        const canvasBackend = await canvas.evaluate(() => window.vellum.renderer.backend);
        report.status = gpuBackend !== "WebGPU" || canvasBackend !== "Canvas 2D"
            ? "backend-unavailable"
            : gpuEvents.errors.length || canvasEvents.errors.length ? "runtime-errors" : "measured";
        expect(gpuBackend, "The probe cannot validate WebGPU when the original renderer falls back; inspect m3-webgpu.json for the adapter/error").toBe("WebGPU");
        expect(canvasBackend).toBe("Canvas 2D");
        expect(gpuEvents.errors).toEqual([]);
        expect(canvasEvents.errors).toEqual([]);
    } finally {
        await mkdir(path.dirname(reportPath), { recursive: true });
        const json = JSON.stringify(report, null, 2);
        await writeFile(reportPath, json);
        await info.attach("headed-chrome-webgpu-comparison", { body: json, contentType: "application/json" });
        await info.attach("original-webgpu-console", { body: gpuEvents.consoleMessages.join("\n"), contentType: "text/plain" });
        await info.attach("original-canvas-console", { body: canvasEvents.consoleMessages.join("\n"), contentType: "text/plain" });
        await canvas.close();
    }
});
