import { createHash } from "node:crypto";
import { mkdir, readFile, writeFile } from "node:fs/promises";
import path from "node:path";
import { expect, test, type Page, type TestInfo } from "@playwright/test";
import { PNG } from "pngjs";
import { differingPixels, present, rgb, waitForQuiet, waitForReady } from "./support";
import { hasReference } from "./twin";

// Exercise installed Chrome's own GPU selection, without SwiftShader launch overrides.
test.use({ channel: "chrome", headless: false, launchOptions: { args: [] }, viewport: { width: 1600, height: 1000 }, deviceScaleFactor: 1 });
test.skip(process.env.VELLUM_HEADED_PROBE !== "1", "Enable the optional installed-Chrome UI walkthrough with VELLUM_HEADED_PROBE=1.");

const reportPath = path.resolve(__dirname, "../../test-results/vellum/m8-ui.json");
const fixturePath = path.resolve(__dirname, "fixtures/forma.vellum");
const referenceUrl = "http://127.0.0.1:4180/";
type Theme = "light" | "dark";
type Camera = { x: number; y: number; zoom: number };

function hash(text: string) { return createHash("sha256").update(text).digest("hex"); }
function canonical(value: unknown): string {
    const sorted = (value: any): any => Array.isArray(value) ? value.map(sorted)
        : value && typeof value === "object" ? Object.fromEntries(Object.keys(value).sort().map(key => [key, sorted(value[key])])) : value;
    return JSON.stringify(sorted(value));
}

async function writeSection(section: string, value: unknown, info: TestInfo) {
    const report = JSON.parse(await readFile(reportPath, "utf8"));
    report[section] = value;
    await writeFile(reportPath, JSON.stringify(report, null, 2));
    await info.attach(section, { body: JSON.stringify(value, null, 2), contentType: "application/json" });
}

function monitor(page: Page) {
    const pageErrors: string[] = [], consoleErrors: string[] = [], messages: string[] = [];
    page.on("pageerror", error => pageErrors.push(error.message));
    page.on("console", message => {
        messages.push(`${message.type()}: ${message.text()}`);
        if (message.type() === "error" || /content security policy|violates.*directive/i.test(message.text())) consoleErrors.push(message.text());
    });
    return { pageErrors, consoleErrors, messages };
}

async function graphics(page: Page) {
    return page.evaluate(() => {
        const gl = (document.getElementById("scene") as HTMLCanvasElement)?.getContext("webgl2");
        const debug = gl?.getExtension("WEBGL_debug_renderer_info");
        return {
            userAgent: navigator.userAgent,
            dpr: devicePixelRatio,
            vendor: debug ? gl!.getParameter(debug.UNMASKED_VENDOR_WEBGL) : null,
            renderer: debug ? gl!.getParameter(debug.UNMASKED_RENDERER_WEBGL) : null,
            backend: window.vellum.renderer.backend,
        };
    });
}

function difference(actual: Buffer, reference: Buffer) {
    const a = PNG.sync.read(actual), b = PNG.sync.read(reference);
    const differing = differingPixels(a, b), total = a.width * a.height;
    return { differing, total, ratio: differing / total, width: a.width, height: a.height };
}

test.beforeAll(async () => {
    await mkdir(path.dirname(reportPath), { recursive: true });
    await writeFile(reportPath, JSON.stringify({
        recordedAt: new Date().toISOString(),
        browser: { channel: "chrome", headed: true, extraLaunchArgs: [] },
        scope: "Automated UI events in headed installed Chrome; screenshots require human visual review.",
        manualEvidence: {
            physicalTrackpad: "not-tested",
            osPinyinInputMethod: "not-tested",
            composition: "Existing m6-edit.spec.ts uses CDP automation, not an OS IME. Not repeated by this headed walkthrough.",
        },
        ui: { status: "not-run" },
        backendComparison: { status: hasReference ? "not-run" : "skipped-local-reference-absent" },
    }, null, 2));
});

test("headed UI edits and WebGL context recovery preserve document and visible pixels", async ({ page, browser }, info) => {
    test.setTimeout(240_000);
    const events = monitor(page);
    const evidence: Record<string, any> = { status: "incomplete", browserVersion: browser.version(), events, screenshots: {} };
    const screenshot = async (name: string) => {
        const file = info.outputPath(`${name}.png`);
        evidence.screenshots[name] = file;
        return page.screenshot({ path: file, animations: "disabled" });
    };
    try {
        await waitForReady(page);
        await page.bringToFront();
        evidence.graphics = await graphics(page);
        for (const theme of ["light", "dark"] as const) {
            if (await page.locator(".vellum").first().getAttribute("data-theme") !== theme) {
                await page.getByRole("button", { name: "Toggle theme", exact: true }).click();
            }
            await expect(page.locator(".vellum").first()).toHaveAttribute("data-theme", theme);
            await present(page);
            await screenshot(`ui-${theme}`);
        }

        await page.locator("[data-page]").nth(2).click();
        const nodes = await page.evaluate(() => {
            const api = window.vellum;
            const frame = api.createAtCenter("frame", { name: "Headed first frame", w: 400, h: 300, rotation: 30 });
            const child = api.createAtCenter("rect", { name: "Native dragged layer", w: 80, h: 60, rotation: 15 });
            return { frame: frame.id, child: child.id, world: api.doc.world(child.id).matrix };
        });
        await page.evaluate(() => {
            (window as any).__m8Drag = [];
            for (const type of ["dragstart", "drop"]) document.addEventListener(type, event => {
                (window as any).__m8Drag.push({ type, shift: (event as DragEvent).shiftKey, trusted: event.isTrusted });
            }, { capture: true, once: true });
        });
        const source = (await page.locator(`[data-layer="${nodes.child}"]`).boundingBox())!;
        const target = (await page.locator(`[data-layer="${nodes.frame}"]`).boundingBox())!;
        await page.mouse.move(source.x + source.width / 2, source.y + source.height / 2);
        await page.mouse.down();
        // macOS Chrome suppresses initiation when Shift is held before dragstart.
        await page.mouse.move(source.x + source.width / 2, source.y + source.height / 2 + 8, { steps: 3 });
        await page.keyboard.down("Shift");
        try {
            await page.mouse.move(target.x + target.width / 2, target.y + target.height / 2, { steps: 5 });
            await page.mouse.move(target.x + target.width / 2 + 1, target.y + target.height / 2);
            await page.mouse.up();
        } finally { await page.keyboard.up("Shift"); }
        expect(await page.evaluate(id => window.vellum.doc.get(id).parentId, nodes.child)).toBe(nodes.frame);
        const world = await page.evaluate(id => window.vellum.doc.world(id).matrix, nodes.child);
        for (let i = 0; i < 6; i++) expect(world[i]).toBeCloseTo(nodes.world[i], 7);
        const dragEvents = await page.evaluate(() => (window as any).__m8Drag);
        expect(dragEvents).toEqual([{ type: "dragstart", shift: false, trusted: true }, { type: "drop", shift: true, trusted: true }]);
        evidence.layerDrag = { events: dragEvents, beforeWorld: nodes.world, afterWorld: world, parentId: nodes.frame };
        await screenshot("native-shift-reparent");

        await page.locator("#overlay").focus();
        await page.keyboard.press("t");
        await page.locator("#overlay").click({ position: { x: 80, y: 90 } });
        const textEditor = page.getByTestId("text-editor");
        await expect(textEditor).toBeFocused();
        const textId = await page.evaluate(() => window.vellum.state.editing as string);
        const unicode = "画布 🌍 é\nمرحبا — Zażółć";
        await textEditor.fill(unicode);
        await page.keyboard.press("Escape");
        await expect(textEditor).toHaveCount(0);
        expect(await page.evaluate(id => window.vellum.doc.get(id).text, textId)).toBe(unicode);
        await page.locator("#overlay").focus();
        await page.keyboard.press("Enter");
        await expect(textEditor).toHaveValue(unicode);
        await page.keyboard.press("ControlOrMeta+Enter");
        await expect(textEditor).toHaveCount(0);
        evidence.unicode = { id: textId, text: unicode, sessions: "T + pointer opens; Escape commits; Enter reopens; primary-modifier Enter commits", input: "Playwright native textarea input; no OS IME claim" };
        await screenshot("unicode-text-committed");

        await page.evaluate(() => window.vellum.createAtCenter("frame", { name: "Headed second frame", w: 360, h: 240, fill: "#55aa88" }));
        await page.locator(`[data-layer="${nodes.frame}"] .layer-name`).click();
        await page.locator("#present").click();
        await expect(page.locator("#presentation-title")).toHaveText("Headed first frame");
        await page.keyboard.press("ArrowRight");
        await expect(page.locator("#presentation-title")).toHaveText("Headed second frame");
        await page.keyboard.press("ArrowLeft");
        await expect(page.locator("#presentation-title")).toHaveText("Headed first frame");
        await page.keyboard.press("ArrowRight");
        await expect(page.locator("#presentation-title")).toHaveText("Headed second frame");
        await screenshot("presentation-keyboard");
        await page.keyboard.press("Escape");
        await expect(page.locator("#presentation")).not.toBeVisible();
        const focused = await page.evaluate(() => document.activeElement?.id);
        expect(focused).toMatch(/present|overlay/);
        evidence.presentation = { keys: ["ArrowRight", "ArrowLeft", "ArrowRight", "Escape"], focusAfterClose: focused };

        await page.evaluate(() => {
            window.vellum.createAtCenter("rect", { name: "Context recovery pixel", w: 120, h: 120, fill: "#2bc4a3", radius: 0 });
            window.vellum.select([]);
        });
        await page.mouse.move(1, 1);
        await present(page);
        const before = await page.evaluate(() => ({ document: window.vellum.doc.serialize(), camera: window.vellum.state.camera, stats: window.__vellum.stats(), renderer: { ...window.vellum.renderer } }));
        const beforePath = info.outputPath("context-before.png");
        const beforePixels = await page.locator("#scene").screenshot({ path: beforePath, animations: "disabled" });
        const beforeImage = PNG.sync.read(beforePixels);
        const center = { x: Math.floor(beforeImage.width / 2), y: Math.floor(beforeImage.height / 2) };
        expect(rgb(beforeImage, center.x, center.y)).toEqual([43, 196, 163]);
        const supported = await page.evaluate(() => {
            const canvas = document.getElementById("scene") as HTMLCanvasElement;
            const extension = canvas.getContext("webgl2")?.getExtension("WEBGL_lose_context");
            if (!extension) return false;
            (window as any).__m8Context = extension;
            (window as any).__m8ContextEvents = [];
            for (const type of ["webglcontextlost", "webglcontextrestored"]) canvas.addEventListener(type, event => {
                (window as any).__m8ContextEvents.push({ type, time: performance.now(), prevented: event.defaultPrevented });
            }, { once: true });
            extension.loseContext();
            return true;
        });
        expect(supported, "Installed Chrome must expose WEBGL_lose_context for the recovery probe").toBe(true);
        await expect(page.locator("#engine-label")).toHaveText("Restoring");
        await expect.poll(() => page.evaluate(() => window.__vellum.live_regions())).toBe(0);
        expect(await page.evaluate(() => window.vellum.doc.serialize())).toBe(before.document);
        await screenshot("context-lost");
        await page.evaluate(() => (window as any).__m8Context.restoreContext());
        await page.waitForFunction(frames => window.vellum.ready && window.__vellum.live_regions() === 1 && window.__vellum.stats().frames > frames, before.stats.frames);
        await expect(page.locator("#engine-label")).toHaveText("Makepad WebGL2");
        await present(page);
        const after = await page.evaluate(() => ({ document: window.vellum.doc.serialize(), camera: window.vellum.state.camera, stats: window.__vellum.stats(), renderer: { ...window.vellum.renderer }, events: (window as any).__m8ContextEvents, diagnostics: window.__vellum.diagnostics() }));
        expect(after.document).toBe(before.document);
        expect(after.camera).toEqual(before.camera);
        expect(after.stats.gpu_bytes).toBeGreaterThan(0);
        expect(after.events.map((event: any) => event.type)).toEqual(["webglcontextlost", "webglcontextrestored"]);
        const afterPath = info.outputPath("context-restored.png");
        const afterPixels = await page.locator("#scene").screenshot({ path: afterPath, animations: "disabled" });
        expect(rgb(PNG.sync.read(afterPixels), center.x, center.y)).toEqual([43, 196, 163]);
        evidence.contextRecovery = { before: { ...before, document: undefined, documentSha256: hash(before.document) }, after: { ...after, document: undefined, documentSha256: hash(after.document) }, center, centerRgb: [43, 196, 163], difference: difference(afterPixels, beforePixels), screenshots: { before: beforePath, after: afterPath } };
        expect(events.pageErrors).toEqual([]);
        expect(events.consoleErrors.filter(error => !/CONTEXT_LOST_WEBGL|WebGL context lost/i.test(error))).toEqual([]);
        evidence.status = "passed-automated-headed-walkthrough";
    } finally {
        await writeSection("ui", evidence, info);
    }
});

async function captureState(page: Page) {
    return page.evaluate(() => {
        const api = window.vellum;
        const area = document.getElementById("canvas-area")!.getBoundingClientRect();
        return { backend: api.renderer.backend, gpuError: api.renderer.gpuError ?? null, adapter: api.renderer.adapterInfo ?? null,
            camera: { ...api.state.camera }, viewport: { width: innerWidth, height: innerHeight, dpr: devicePixelRatio },
            canvasArea: { x: area.x, y: area.y, width: area.width, height: area.height },
            pageId: api.doc.page.id, pageName: api.doc.page.name, nodes: api.doc.nodes.length, document: api.doc.serialize() };
    });
}

async function prepareComparison(page: Page, index: number, theme: Theme, camera?: Camera) {
    await page.bringToFront();
    await page.locator("[data-page]").nth(index).click();
    await page.evaluate(({ theme, camera }) => {
        const api = window.vellum;
        const scope = document.querySelector(".vellum") ?? document.documentElement;
        if (scope.getAttribute("data-theme") !== theme) api.actions.theme();
        api.select([]); api.fit();
        // Only the original's mutable automation state is assigned a camera; Rust is the source.
        if (camera) { Object.assign(api.state.camera, camera); api.state.hover = null; }
        if (api.actions.blankBadges) api.actions.blankBadges();
        for (const id of ["welcome-tip", "toast"]) {
            const element = document.getElementById(id); if (element) element.style.display = "none";
        }
        api.render();
    }, { theme, camera });
    await present(page);
    await waitForQuiet(page);
    await page.evaluate(() => {
        for (const id of ["engine-label", "performance"]) {
            const element = document.getElementById(id); if (element) element.textContent = "Renderer";
        }
    });
}

test("headed Chrome records Rust WebGL2 versus original WebGPU on six matched views", async ({ page, context, browser }, info) => {
    test.skip(!hasReference, "The optional local Vellum source is absent; UI/context recovery still runs.");
    test.setTimeout(300_000);
    const twin = await context.newPage();
    const events = { rust: monitor(page), reference: monitor(twin) };
    const evidence: Record<string, any> = { status: "incomplete", browserVersion: browser.version(), referenceUrl, events, visualGate: null,
        note: "WebGPU comparison is measured evidence for human review. The formal 2% visual gate uses the Canvas reference in visual.spec.ts.", results: [] };
    try {
        await waitForReady(page);
        const fixture = await readFile(fixturePath, "utf8");
        await page.evaluate(async text => { await (window.vellum as any).importDocument(new File([text], "forma.vellum", { type: "application/json" })); }, fixture);
        // Import the exact Rust document, including retained unknown/default fields, into the reference.
        const shared = await page.evaluate(() => window.vellum.doc.serialize());
        await twin.bringToFront();
        await twin.goto(referenceUrl);
        await twin.waitForFunction(() => window.vellum?.ready);
        await twin.evaluate(async text => { await (window.vellum as any).importDocument(new File([text], "shared.vellum", { type: "application/json" })); }, shared);
        await twin.evaluate(() => document.fonts.ready.then(() => undefined));
        evidence.fixture = { path: fixturePath, sourceSha256: hash(fixture), sharedDocumentSha256: hash(canonical(JSON.parse(shared))) };
        evidence.graphics = await graphics(page);
        expect(await twin.evaluate(() => window.vellum.renderer.backend), "A Canvas fallback cannot stand in for the original WebGPU backend").toBe("WebGPU");
        expect(await page.evaluate(() => window.vellum.renderer.backend)).toBe("Makepad WebGL2");
        for (const index of [0, 1, 2]) for (const theme of ["light", "dark"] as const) {
            await prepareComparison(page, index, theme);
            const rust = await captureState(page);
            await prepareComparison(twin, index, theme, rust.camera);
            const original = await captureState(twin);
            expect(original.camera).toEqual(rust.camera);
            expect(original.viewport).toEqual(rust.viewport);
            expect(original.canvasArea).toEqual(rust.canvasArea);
            const documentHash = hash(canonical(JSON.parse(rust.document)));
            expect(hash(canonical(JSON.parse(original.document))), "Both screenshots must use identical document data").toBe(documentHash);
            const paths = {
                rust: info.outputPath(`rust-${index}-${theme}.png`), original: info.outputPath(`original-webgpu-${index}-${theme}.png`),
                rustScene: info.outputPath(`rust-scene-${index}-${theme}.png`), originalScene: info.outputPath(`original-webgpu-scene-${index}-${theme}.png`),
            };
            await page.bringToFront(); await present(page);
            const actual = await page.screenshot({ path: paths.rust, animations: "disabled" });
            const actualScene = await page.locator("#canvas-area").screenshot({ path: paths.rustScene, animations: "disabled" });
            await twin.bringToFront(); await present(twin);
            const expected = await twin.screenshot({ path: paths.original, animations: "disabled" });
            const expectedScene = await twin.locator("#canvas-area").screenshot({ path: paths.originalScene, animations: "disabled" });
            const result = { page: index, theme, rust: { ...rust, document: undefined, documentSha256: documentHash },
                reference: { ...original, document: undefined, documentSha256: documentHash },
                fullPage: difference(actual, expected), scene: difference(actualScene, expectedScene), screenshots: paths };
            evidence.results.push(result);
            console.log(`M8 headed matched backends: ${JSON.stringify(result)}`);
        }
        expect(events.rust.pageErrors).toEqual([]); expect(events.rust.consoleErrors).toEqual([]);
        expect(events.reference.pageErrors).toEqual([]); expect(events.reference.consoleErrors).toEqual([]);
        evidence.status = "measured-six-matched-views-awaiting-human-review";
    } finally {
        await writeSection("backendComparison", evidence, info);
        await twin.close();
    }
});
