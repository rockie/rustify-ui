import { mkdir, writeFile } from "node:fs/promises";
import path from "node:path";
import { expect, present, test, waitForReady } from "./support";
import { EVIDENCE } from "../tier";

// Keep the reference smoke.py's 36 checks in one uninterrupted editor session.
test("the original 36 Vellum smoke checks pass in their original order", { tag: EVIDENCE }, async ({ page, browserErrors }, info) => {
    test.setTimeout(240_000);
    const results: { name: string; passed: true }[] = [];
    let stressScene: Record<string, unknown> | undefined;
    async function check<T>(name: string, body: () => Promise<T>): Promise<T> {
        const value = await test.step(`${results.length + 1}. ${name}`, body);
        results.push({ name, passed: true });
        return value;
    }

    try {
        await waitForReady(page);
        await check("Editor initializes with real scene graph", async () => {
            expect(await page.evaluate(() => window.vellum.doc.nodes.length)).toBe(171);
            expect(await page.evaluate(() => window.vellum.doc.data.pages.length)).toBe(3);
        });
        await check("Renderer produces scene instances", async () => {
            expect(await page.evaluate(() => window.vellum.renderer.instanceCount)).toBeGreaterThan(150);
            expect(await page.evaluate(() => window.vellum.renderer.backend)).toBe("Makepad WebGL2");
        });
        await check("Page navigation", async () => {
            await page.locator("#dismiss-tip").click();
            await page.locator("[data-page]").last().click();
            expect(await page.evaluate(() => window.vellum.doc.page.name)).toBe("Playground");
            expect(await page.evaluate(() => window.vellum.doc.nodes.length)).toBe(0);
        });

        const canvas = (await page.locator("#overlay").boundingBox())!;
        const x = canvas.x + 200, y = canvas.y + 180;
        const rectangle = await check("Rectangle tool pointer drawing", async () => {
            await page.keyboard.press("r");
            await page.mouse.move(x, y);
            await page.mouse.down();
            await page.mouse.move(x + 180, y + 100, { steps: 5 });
            await page.mouse.up();
            const nodes = await page.evaluate(() => window.vellum.doc.nodes);
            expect(nodes).toHaveLength(1);
            expect(nodes[0].type).toBe("rect");
            expect(Math.abs(nodes[0].w - 180)).toBeLessThan(1);
            return { id: nodes[0].id, originalX: nodes[0].x };
        });
        await check("Pointer dragging updates geometry", async () => {
            await page.mouse.move(x + 70, y + 40);
            await page.mouse.down();
            await page.mouse.move(x + 120, y + 70, { steps: 5 });
            await page.mouse.up();
            const current = await page.evaluate(id => window.vellum.doc.get(id).x, rectangle.id);
            expect(Math.abs(current - rectangle.originalX - 50)).toBeLessThan(1);
        });
        await check("Undo restores transform", async () => {
            await page.keyboard.press("Control+z");
            const current = await page.evaluate(id => window.vellum.doc.get(id).x, rectangle.id);
            expect(Math.abs(current - rectangle.originalX)).toBeLessThan(1);
        });
        await check("Redo reapplies transform", async () => {
            await page.keyboard.press("Control+Shift+z");
            const current = await page.evaluate(id => window.vellum.doc.get(id).x, rectangle.id);
            expect(Math.abs(current - rectangle.originalX - 50)).toBeLessThan(1);
        });
        await check("Inspector dimension binding", async () => {
            await page.locator('input[data-prop="w"]').fill("240");
            await page.locator('input[data-prop="w"]').press("Enter");
            await page.locator('input[data-prop="h"]').click();
            expect(await page.evaluate(id => window.vellum.doc.get(id).w, rectangle.id)).toBe(240);
        });
        await check("Canvas resize handles", async () => {
            const corner = await page.evaluate(id => {
                const api = window.vellum, node = api.doc.get(id), world = api.doc.world(id), camera = api.state.camera;
                return { x: (world.matrix[4] + node.w) * camera.zoom + camera.x, y: (world.matrix[5] + node.h) * camera.zoom + camera.y };
            }, rectangle.id);
            await page.mouse.move(canvas.x + corner.x, canvas.y + corner.y);
            await page.mouse.down();
            await page.mouse.move(canvas.x + corner.x + 60, canvas.y + corner.y + 30, { steps: 5 });
            await page.mouse.up();
            expect(Math.abs(await page.evaluate(id => window.vellum.doc.get(id).w, rectangle.id) - 300)).toBeLessThan(1);
        });
        await check("Rotation transforms world bounds", async () => {
            await page.evaluate(id => { window.vellum.select([id]); window.vellum.setProperty("rotation", 30); }, rectangle.id);
            expect(await page.evaluate(id => window.vellum.doc.get(id).rotation, rectangle.id)).toBe(30);
            expect(await page.evaluate(id => window.vellum.doc.world(id).box.w, rectangle.id)).toBeGreaterThan(300);
        });
        await check("Deep duplication uses distinct IDs", async () => {
            await page.evaluate(() => window.vellum.actions.duplicate());
            const ids = await page.evaluate(() => window.vellum.doc.nodes.map(node => node.id));
            expect(ids).toHaveLength(2);
            expect(new Set(ids).size).toBe(2);
        });
        await check("Grouping preserves hierarchy", async () => {
            await page.evaluate(() => { window.vellum.select(window.vellum.doc.nodes.map(node => node.id)); window.vellum.actions.group(); });
            const nodes = await page.evaluate(() => window.vellum.doc.nodes);
            expect(nodes.filter(node => node.type === "group")).toHaveLength(1);
            expect(nodes.filter(node => node.parentId)).toHaveLength(2);
        });
        await check("Ungroup preserves children", async () => {
            await page.evaluate(() => window.vellum.actions.ungroup());
            const nodes = await page.evaluate(() => window.vellum.doc.nodes);
            expect(nodes).toHaveLength(2);
            expect(nodes.every(node => !node.parentId)).toBe(true);
        });

        const text = await check("Multiline Unicode text editing commits", async () => {
            const id = await page.evaluate(() => {
                const api = window.vellum;
                const node = api.createAtCenter("text", { text: "Typography test", w: 340, h: 80, fontSize: 24 });
                api.fit([node.id]); api.actions.editText();
                return node.id;
            });
            await page.getByTestId("text-editor").fill("Vellum · Typography\nZażółć gęślą jaźń · مرحبا");
            await page.keyboard.press("Escape");
            const value = await page.evaluate(id => window.vellum.doc.get(id).text as string, id);
            expect(value).toContain("مرحبا");
            expect(value).toContain("\n");
            expect(await page.evaluate(() => Boolean(window.vellum.state.editing))).toBe(false);
            return id;
        });
        await check("Typography properties bind to the document", async () => {
            await page.evaluate(id => {
                const api = window.vellum;
                api.select([id]); api.setProperty("fontSize", 32); api.setProperty("fontWeight", 700);
                api.setProperty("letterSpacing", 1.2); api.setProperty("lineHeight", 160); api.setProperty("textAlign", "center");
                api.setProperty("direction", "rtl"); api.setProperty("textDecoration", "underline");
            }, text);
            expect(await page.evaluate(id => window.vellum.doc.get(id), text)).toMatchObject({
                fontSize: 32, fontWeight: 700, letterSpacing: 1.2, lineHeight: 1.6,
                textAlign: "center", direction: "rtl", textDecoration: "underline",
            });
        });
        const instance = await check("Components create linked instances", async () => {
            const state = await page.evaluate(id => {
                const api = window.vellum;
                api.select([id]); api.actions.component();
                const before = api.doc.nodes.length;
                (api as any).instantiate(id);
                return { before, count: api.doc.nodes.length, instance: [...api.state.selection][0] };
            }, rectangle.id);
            expect(state.count).toBe(state.before + 1);
            expect(await page.evaluate(id => window.vellum.doc.get(id).sourceId, state.instance)).toBe(rectangle.id);
            return state.instance;
        });
        await check("Main component changes propagate", async () => {
            await page.evaluate(id => { window.vellum.select([id]); window.vellum.setProperty("fill", "#ee7733"); }, rectangle.id);
            expect(await page.evaluate(id => window.vellum.doc.get(id).fill, instance)).toBe("#ee7733");
        });
        await check("Instance overrides survive source edits", async () => {
            await page.evaluate(({ instance, source }) => {
                const api = window.vellum;
                api.select([instance]); api.setProperty("fill", "#33aa88");
                api.select([source]); api.setProperty("fill", "#112233");
            }, { instance, source: rectangle.id });
            expect(await page.evaluate(id => window.vellum.doc.get(id).fill, instance)).toBe("#33aa88");
        });
        const layout = await check("Horizontal auto layout and spacing", async () => {
            const ids = await page.evaluate(() => {
                const api = window.vellum;
                const frame = api.createAtCenter("frame", { w: 450, h: 250, name: "Auto layout test" }).id;
                const a = api.createAtCenter("rect", { w: 80, h: 40 }).id;
                const b = api.createAtCenter("rect", { w: 90, h: 50 }).id;
                api.transaction("Arrange", [{ id: a, prop: "parentId", value: frame }, { id: b, prop: "parentId", value: frame }]);
                api.select([frame]); api.setProperty("layout", "horizontal"); api.setProperty("gap", 20); api.setProperty("padding", 16);
                return { frame, a, b };
            });
            expect(await page.evaluate(id => window.vellum.doc.get(id), ids.a)).toMatchObject({ x: 16, y: 16 });
            expect(await page.evaluate(id => window.vellum.doc.get(id).x, ids.b)).toBe(116);
            return ids;
        });
        await check("Vertical auto layout", async () => {
            await page.evaluate(() => window.vellum.setProperty("layout", "vertical"));
            expect(await page.evaluate(id => window.vellum.doc.get(id), layout.b)).toMatchObject({ x: 16, y: 76 });
        });
        await check("Frame resize constraints", async () => {
            const before = await page.evaluate(({ frame, a }) => {
                const api = window.vellum;
                api.setProperty("layout", "none"); api.select([a]); api.setProperty("constraintH", "right");
                const previous = api.doc.get(a).x;
                api.select([frame]); api.setProperty("w", 550);
                return previous;
            }, layout);
            expect(await page.evaluate(id => window.vellum.doc.get(id).x, layout.a)).toBe(before + 100);
        });
        await check("Pen creates editable Bézier geometry", async () => {
            await page.evaluate(() => { window.vellum.fit(); window.vellum.select([]); });
            await page.keyboard.press("p");
            await page.mouse.move(canvas.x + 200, canvas.y + 400);
            await page.mouse.down();
            await page.mouse.move(canvas.x + 240, canvas.y + 380);
            await page.mouse.up();
            await page.mouse.click(canvas.x + 350, canvas.y + 430);
            await page.mouse.click(canvas.x + 280, canvas.y + 530);
            await page.keyboard.press("Enter");
            expect(await page.evaluate(() => window.vellum.doc.nodes.some(node => node.type === "path" && node.points.length === 3 && Boolean(node.points[0].out)))).toBe(true);
        });
        const imageAsset = await check("Image import embeds image data", async () => {
            const result = await page.evaluate(async () => {
                const api = window.vellum as any;
                const before = api.doc.nodes.length;
                const svg = '<svg xmlns="http://www.w3.org/2000/svg" width="80" height="60"><rect width="80" height="60" fill="#339966"/></svg>';
                await api.importImage(new File([svg], "test.svg", { type: "image/svg+xml" }));
                const node = api.doc.nodes.at(-1);
                return { before, count: api.doc.nodes.length, node, assets: Object.keys(JSON.parse(api.doc.serialize()).assets) };
            });
            expect(result.count).toBe(result.before + 1);
            expect(result.node.type).toBe("image");
            expect(result.assets).toHaveLength(1);
            return result.node.assetId as string;
        });
        await check("Undo document replacement restores image assets", async () => {
            await page.evaluate(async () => {
                const data = { format: "vellum", version: 1, name: "Empty import", pageId: "empty", pages: [{ id: "empty", name: "Empty", nodes: [] }], assets: {} };
                await (window.vellum as any).importDocument(new File([JSON.stringify(data)], "empty.vellum", { type: "application/json" }));
                window.vellum.actions.undo();
            });
            expect(await page.evaluate(asset => window.vellum.doc.nodes.some(node => node.assetId === asset), imageAsset)).toBe(true);
            const source = await page.evaluate(asset => JSON.parse(window.vellum.doc.serialize()).assets[asset], imageAsset);
            expect(source).toMatch(/^data:image\/svg\+xml/);
        });
        await check("Portable JSON validates and round-trips", async () => {
            const data = await page.evaluate(() => {
                const api = window.vellum as any;
                const serialized = api.doc.serialize(), parsed = api.parse(serialized);
                return { parsed: parsed.pages, original: JSON.parse(serialized).pages };
            });
            expect(data.parsed).toHaveLength(3);
            expect(data.parsed).toEqual(data.original);
        });
        await check("Cyclic document input is rejected", async () => {
            expect(await page.evaluate(() => {
                const api = window.vellum as any, data = JSON.parse(api.doc.serialize());
                const node = data.pages.find(page => page.id === data.pageId).nodes[0];
                node.parentId = node.id;
                try { api.parse(JSON.stringify(data)); return false; } catch { return true; }
            })).toBe(true);
        });
        await check("HTML-injection layer IDs are rejected", async () => {
            expect(await page.evaluate(() => {
                const api = window.vellum as any, data = JSON.parse(api.doc.serialize());
                data.pages.find(page => page.id === data.pageId).nodes[0].id = '"><img src=x onerror=alert(1)>';
                try { api.parse(JSON.stringify(data)); return false; } catch { return true; }
            })).toBe(true);
        });
        await check("SVG export is well-formed and includes vector/text nodes", async () => {
            expect(await page.evaluate(() => {
                const api = window.vellum as any;
                const roots = api.doc.nodes.filter(node => !node.parentId).map(node => node.id);
                const xml = new DOMParser().parseFromString(api.exportSVG(roots), "image/svg+xml");
                return { valid: !xml.querySelector("parsererror"), text: Boolean(xml.querySelector("text")), path: Boolean(xml.querySelector("path")), image: Boolean(xml.querySelector("image")) };
            })).toEqual({ valid: true, text: true, path: true, image: true });
        });
        await check("PNG export produces real pixel content", async () => {
            const exported = await page.evaluate(async id => {
                const canvas: HTMLCanvasElement = await (window.vellum.renderer as any).exportCanvas([id], 1);
                const pixel = canvas.getContext("2d")!.getImageData(10, 10, 1, 1).data;
                return { width: canvas.width, height: canvas.height, alpha: pixel[3] };
            }, layout.frame);
            expect(exported.width).toBe(550);
            expect(exported.height).toBe(250);
            expect(exported.alpha).toBeGreaterThan(0);
        });
        await check("Frame presentation renders a preview", async () => {
            await page.evaluate(id => { window.vellum.select([id]); window.vellum.actions.present(); }, layout.frame);
            await expect(page.locator("#presentation")).toBeVisible();
            await expect.poll(() => page.locator("#presentation-canvas").evaluate((canvas: HTMLCanvasElement) => canvas.width)).toBeGreaterThan(0);
        });
        await page.locator("#close-presentation").click();
        await check("Light appearance switches semantic tokens", async () => {
            await page.locator("#theme-toggle").click();
            await expect(page.locator(".vellum").first()).toHaveAttribute("data-theme", "light");
        });
        await check("Command palette executes commands", async () => {
            const rulers = await page.evaluate(() => (window.vellum as any).options.rulers);
            await page.keyboard.press("Control+k");
            await page.locator("#command-search").fill("rulers");
            await page.keyboard.press("Enter");
            await expect(page.locator("#modal-backdrop")).toBeHidden();
            expect(await page.evaluate(() => (window.vellum as any).options.rulers)).toBe(!rulers);
        });

        await page.evaluate(() => { window.vellum.actions.resetStarter(); window.vellum.select([]); window.vellum.fit(); });
        await present(page);
        await page.locator("#toast").evaluate(element => element.classList.add("hidden"));
        const light = info.outputPath("screenshot-light.png");
        await page.screenshot({ path: light, animations: "disabled" });
        await info.attach("screenshot-light", { path: light, contentType: "image/png" });
        await check("Dark appearance restores semantic tokens", async () => {
            await page.evaluate(() => window.vellum.actions.theme());
            await present(page);
            await expect(page.locator(".vellum").first()).toHaveAttribute("data-theme", "dark");
        });
        const dark = info.outputPath("screenshot-dark.png");
        await page.screenshot({ path: dark, animations: "disabled" });
        await info.attach("screenshot-dark", { path: dark, contentType: "image/png" });
        await check("Responsive editor has no horizontal document overflow", async () => {
            await page.setViewportSize({ width: 1280, height: 800 });
            await page.evaluate(() => window.vellum.fit());
            await present(page);
            expect(await page.evaluate(() => document.documentElement.scrollWidth)).toBe(1280);
        });
        await page.setViewportSize({ width: 1600, height: 1000 });
        await present(page);
        await check("5,000-shape scene renders and remains editable", async () => {
            await page.evaluate(() => window.vellum.actions.stressTest());
            await page.waitForFunction(() => window.vellum.doc.nodes.length === 5000 && window.vellum.renderer.visibleCount === 5000, null, { timeout: 10_000 });
            expect(await page.evaluate(() => ({ nodes: window.vellum.doc.nodes.length, visible: window.vellum.renderer.visibleCount }))).toEqual({ nodes: 5000, visible: 5000 });
        });
        stressScene = await page.evaluate(() => {
            const renderer = window.vellum.renderer;
            return { backend: renderer.backend, instances: renderer.instanceCount, visibleLayers: renderer.visibleCount,
                cpuSubmissionMs: renderer.cpuMs, drawCalls: renderer.drawCalls, gpuError: renderer.gpuError, secureContext: isSecureContext };
        });
        await check("No uncaught browser errors", async () => {
            expect(browserErrors, "Uncaught browser errors and CSP console errors").toEqual([]);
            expect(await page.evaluate(() => window.__vellumCsp)).toEqual([]);
            expect(await page.evaluate(() => window.__vellum.errors())).toEqual([]);
        });
        expect(results).toHaveLength(36);
    } finally {
        const report = { mode: page.url(), passed: results.length, results, stressScene, limitations: [] };
        const text = JSON.stringify(report, null, 2);
        await writeFile(info.outputPath("results.json"), text);
        const directory = path.resolve(__dirname, "../../test-results/vellum");
        await mkdir(directory, { recursive: true });
        await writeFile(path.join(directory, "smoke.json"), text);
        await info.attach("36-check-report", { body: text, contentType: "application/json" });
        console.log(`Vellum smoke: ${JSON.stringify({ passed: report.passed, stressScene })}`);
    }
});
