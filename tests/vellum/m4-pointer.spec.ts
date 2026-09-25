import { mkdir, writeFile } from "node:fs/promises";
import path from "node:path";
import type { Page } from "@playwright/test";
import { PNG } from "pngjs";
import { differingPixels, expect, present, test, waitForReady } from "./support";
import { hasReference, openTwin } from "./twin";
import { EVIDENCE } from "../tier";

async function playground(page: Page, reference = false) {
    if (reference) await openTwin(page); else await waitForReady(page);
    await page.locator("[data-page]").nth(2).click();
    await page.evaluate(() => {
        for (const id of ["toast", "welcome-tip"]) {
            const element = document.getElementById(id);
            if (element) element.style.display = "none";
        }
    });
    await present(page);
    return (await page.locator("#overlay").boundingBox())!;
}

async function drag(page: Page, from: { x: number; y: number }, to: { x: number; y: number }, modifiers: string[] = [], moveModifiers: string[] = []) {
    for (const key of modifiers) await page.keyboard.down(key);
    await page.mouse.move(from.x, from.y);
    await page.mouse.down();
    for (const key of moveModifiers) await page.keyboard.down(key);
    await page.mouse.move(to.x, to.y, { steps: 5 });
    await page.mouse.up();
    for (const key of [...modifiers, ...moveModifiers].reverse()) await page.keyboard.up(key);
    await present(page);
}

async function corner(page: Page, id: string, right: boolean, bottom: boolean) {
    const box = (await page.locator("#overlay").boundingBox())!;
    return page.evaluate(({ id, box, right, bottom }) => {
        const api = window.vellum, n = api.doc.get(id), m = api.doc.world(id).matrix, c = api.state.camera;
        const x = right ? n.w : 0, y = bottom ? n.h : 0;
        return { x: box.x + (m[0] * x + m[2] * y + m[4]) * c.zoom + c.x, y: box.y + (m[1] * x + m[3] * y + m[5]) * c.zoom + c.y };
    }, { id, box, right, bottom });
}

test("rectangle creation, dragging, undo, redo, resize and rotation follow the original smoke", async ({ page }) => {
    const box = await playground(page);
    const origin = { x: box.x + 200, y: box.y + 180 };
    await page.keyboard.press("r");
    await drag(page, origin, { x: origin.x + 180, y: origin.y + 100 });
    expect(await page.evaluate(() => window.vellum.doc.nodes.length)).toBe(1);
    const initial = await page.evaluate(() => window.vellum.doc.nodes[0]);
    expect(initial.w).toBe(180);
    expect(initial.h).toBe(100);
    await drag(page, { x: origin.x + 70, y: origin.y + 40 }, { x: origin.x + 120, y: origin.y + 70 });
    expect(await page.evaluate(id => window.vellum.doc.get(id).x, initial.id)).toBe(initial.x + 50);
    await page.keyboard.press("Control+z");
    await present(page);
    expect(await page.evaluate(id => window.vellum.doc.get(id).x, initial.id)).toBe(initial.x);
    await page.keyboard.press("Control+Shift+z");
    await present(page);
    expect(await page.evaluate(id => window.vellum.doc.get(id).x, initial.id)).toBe(initial.x + 50);
    await page.evaluate(() => window.vellum.setProperty("w", 240));
    await present(page);
    const handle = await corner(page, initial.id, true, true);
    await drag(page, handle, { x: handle.x + 60, y: handle.y + 30 });
    expect(await page.evaluate(id => window.vellum.doc.get(id).w, initial.id)).toBe(300);
    await page.evaluate(() => window.vellum.setProperty("rotation", 30));
    expect(await page.evaluate(id => window.vellum.doc.world(id).box.w, initial.id)).toBeGreaterThan(300);
});

async function geometry(page: Page) {
    return page.evaluate(() => {
        const api = window.vellum;
        const round = (value: number) => Math.round(value * 1e6) / 1e6;
        const node = (n: ReturnType<typeof api.doc.get>) => ({ type: n.type, x: round(n.x), y: round(n.y), w: round(n.w), h: round(n.h), rotation: round(n.rotation), pointCount: n.points?.length ?? 0 });
        return {
            camera: Object.fromEntries(Object.entries(api.state.camera).map(([key, value]) => [key, round(value)])),
            nodes: api.doc.nodes.map(node),
            selected: [...api.state.selection].map(id => api.doc.get(id)).filter(Boolean).map(node),
        };
    });
}

const scenarios = ["create", "alt-shift-move", "alt-shift-resize", "shift-rotate", "marquee", "wheel-and-space-pan"] as const;
for (const scenario of scenarios) {
    test(`twin geometry: ${scenario}`, async ({ page, context }, info) => {
        test.skip(!hasReference, "The optional Vellum source reference is absent.");
        const twin = await context.newPage();
        for (const target of [page, twin]) {
            const box = await playground(target, target === twin);
            const center = { x: box.x + box.width / 2, y: box.y + box.height / 2 };
            if (scenario === "create") {
                await target.keyboard.press("r");
                await drag(target, { x: box.x + 200, y: box.y + 180 }, { x: box.x + 380, y: box.y + 280 });
            } else if (scenario === "wheel-and-space-pan") {
                await target.mouse.move(center.x, center.y);
                await target.mouse.wheel(30, 45);
                await present(target);
                await target.keyboard.down("Control");
                await target.mouse.wheel(0, -35);
                await target.keyboard.up("Control");
                await present(target);
                await drag(target, center, { x: center.x + 40, y: center.y - 25 }, ["Space"]);
            } else if (scenario === "marquee") {
                await target.evaluate(() => {
                    const api = window.vellum;
                    for (const delta of [-200, 140, 350]) {
                        const node = api.createAtCenter("rect", { w: 80, h: 60 });
                        api.select([node.id]); api.setProperty("x", node.x + delta);
                    }
                    api.select([]);
                });
                await present(target);
                await drag(target, { x: center.x - 270, y: center.y - 70 }, { x: center.x + 230, y: center.y + 80 });
            } else {
                const node = await target.evaluate(() => window.vellum.createAtCenter("rect", { w: 180, h: 100 }));
                await present(target);
                if (scenario === "alt-shift-move") {
                    await drag(target, center, { x: center.x + 50, y: center.y + 30 }, ["Alt"], ["Shift"]);
                } else if (scenario === "alt-shift-resize") {
                    const handle = await corner(target, node.id, true, true);
                    await drag(target, handle, { x: handle.x + 40, y: handle.y + 30 }, ["Alt", "Shift"]);
                } else {
                    await drag(target, { x: center.x, y: center.y - 75 }, { x: center.x + 75, y: center.y + 20 }, ["Shift"]);
                }
            }
        }
        const actual = await geometry(page), reference = await geometry(twin);
        expect(actual).toEqual(reference);
        if (scenario === "marquee") expect(actual.selected).toHaveLength(2);
        if (scenario === "alt-shift-move") expect(actual.nodes).toHaveLength(2);
        await info.attach("twin-geometry", { body: JSON.stringify({ scenario, actual, reference }, null, 2), contentType: "application/json" });
        // The independent overlay snapshots keep later shell work out of the gesture evidence.
        const overlays: PNG[] = [];
        for (const [name, target] of [["actual", page], ["reference", twin]] as const) {
            const png = await target.evaluate(() => (document.getElementById("overlay") as HTMLCanvasElement).toDataURL("image/png"));
            const bytes = Buffer.from(png.split(",")[1], "base64");
            overlays.push(PNG.sync.read(bytes));
            await writeFile(info.outputPath(`${name}-${scenario}-overlay.png`), bytes);
        }
        expect(differingPixels(overlays[0], overlays[1]), "Gesture overlays must agree at RGB threshold 24").toBe(0);
        await twin.close();
    });
}

test("Escape, pointercancel and window blur restore a pending drag", async ({ page }) => {
    const box = await playground(page);
    const center = { x: box.x + box.width / 2, y: box.y + box.height / 2 };
    const node = await page.evaluate(() => window.vellum.createAtCenter("rect", { w: 180, h: 100 }));
    await present(page);
    for (const reason of ["Escape", "pointercancel", "blur"]) {
        await page.mouse.move(center.x, center.y);
        await page.mouse.down();
        await page.mouse.move(center.x + 50, center.y + 30, { steps: 3 });
        await present(page);
        expect(await page.evaluate(id => window.vellum.doc.get(id).x, node.id)).toBe(node.x + 50);
        if (reason === "Escape") await page.keyboard.press("Escape");
        else if (reason === "blur") await page.evaluate(() => window.dispatchEvent(new Event("blur")));
        else await page.locator("#overlay").dispatchEvent("pointercancel", { pointerId: 1, pointerType: "mouse" });
        await page.mouse.up();
        await present(page);
        expect(await page.evaluate(id => ({ x: window.vellum.doc.get(id).x, y: window.vellum.doc.get(id).y, gesture: window.vellum.state.gesture, guides: window.vellum.state.guides, marquee: window.vellum.state.marquee }), node.id))
            .toEqual({ x: node.x, y: node.y, gesture: null, guides: [], marquee: null });
    }
});

test("canvas keyboard edits ignore form controls and use original nudge and zoom steps", async ({ page }) => {
    await playground(page);
    const node = await page.evaluate(() => window.vellum.createAtCenter("rect", { w: 180, h: 100 }));
    await page.keyboard.press("ArrowRight");
    await page.keyboard.press("Shift+ArrowDown");
    expect(await page.evaluate(id => ({ x: window.vellum.doc.get(id).x, y: window.vellum.doc.get(id).y }), node.id)).toEqual({ x: node.x + 1, y: node.y + 10 });
    await page.evaluate(() => document.getElementById("layer-search")!.classList.remove("hidden"));
    const search = page.locator("#search-layers");
    await search.focus();
    await page.keyboard.press("r");
    expect(await page.evaluate(() => window.vellum.state.tool)).toBe("select");
    await page.locator("#overlay").focus();
    await page.keyboard.press("Shift+0");
    expect(await page.evaluate(() => window.vellum.state.camera.zoom)).toBe(1);
    await page.keyboard.press("+");
    expect(await page.evaluate(() => window.vellum.state.camera.zoom)).toBeCloseTo(1.25);
    await page.keyboard.press("Delete");
    expect(await page.evaluate(() => window.vellum.doc.nodes.length)).toBe(0);
    await page.keyboard.press("Control+z");
    expect(await page.evaluate(() => window.vellum.doc.nodes.length)).toBe(1);
});

test("pan presentation latency is measured from input to an SDK frame in the page", { tag: EVIDENCE }, async ({ page }, info) => {
    await waitForReady(page);
    const box = (await page.locator("#overlay").boundingBox())!;
    await page.mouse.move(box.x + 500, box.y + 350);
    await page.mouse.down({ button: "middle" });
    const samples = await page.evaluate(async () => {
        const overlay = document.getElementById("overlay")!, box = overlay.getBoundingClientRect();
        const frame = () => new Promise<void>(resolve => requestAnimationFrame(() => resolve()));
        const event = (type: string, x: number, y: number, buttons: number) => overlay.dispatchEvent(new PointerEvent(type, { bubbles: true, pointerId: 1, pointerType: "mouse", button: 1, buttons, clientX: box.x + x, clientY: box.y + y }));
        const durations: number[] = [];
        for (let index = 0; index < 30; index++) {
            const before = window.__vellum.stats().frames;
            const start = performance.now();
            event("pointermove", 500 + (index + 1) * 2, 350 + (index + 1), 4);
            while (window.__vellum.stats().frames <= before) {
                if (performance.now() - start > 2000) throw new Error("Pan did not produce an SDK frame");
                await frame();
            }
            durations.push(performance.now() - start);
            await frame();
        }
        return durations;
    });
    await page.mouse.up({ button: "middle" });
    const sorted = [...samples].sort((a, b) => a - b);
    const p95 = sorted[Math.ceil(sorted.length * 0.95) - 1];
    const result = { samples, p95, unit: "ms", criterion: "input dispatch to SDK presentation frame count observed in requestAnimationFrame" };
    console.log(`Pan presentation: ${JSON.stringify(result)}`);
    const directory = path.resolve(__dirname, "../../test-results/vellum");
    await mkdir(directory, { recursive: true });
    await writeFile(path.join(directory, "m4-pan.json"), JSON.stringify(result, null, 2));
    await info.attach("pan-presentation", { body: JSON.stringify(result), contentType: "application/json" });
    expect(p95, "ADR-2 requires a camera ownership reassessment above 50 ms").toBeLessThanOrEqual(50);
});

test("pen handles, finishing and anchor edits match the original path geometry", async ({ page, context }, info) => {
    test.skip(!hasReference, "The optional Vellum source reference is absent.");
    const twin = await context.newPage();
    const paths = [];
    for (const target of [page, twin]) {
        const box = await playground(target, target === twin);
        await target.keyboard.press("p");
        await drag(target, { x: box.x + 350, y: box.y + 250 }, { x: box.x + 380, y: box.y + 230 });
        await target.mouse.click(box.x + 500, box.y + 350);
        await target.mouse.click(box.x + 350, box.y + 450);
        await target.keyboard.press("Enter");
        await present(target);
        expect(await target.evaluate(() => window.vellum.doc.nodes[0].points.length)).toBe(3);
        await target.mouse.dblclick(box.x + 500, box.y + 350);
        await present(target);
        expect(await target.evaluate(() => window.vellum.state.pathEdit)).toBeTruthy();
        await drag(target, { x: box.x + 500, y: box.y + 350 }, { x: box.x + 520, y: box.y + 370 });
        paths.push(await target.evaluate(() => {
            const n = window.vellum.doc.nodes[0];
            return { x: n.x, y: n.y, w: n.w, h: n.h, pathW: n.pathW, pathH: n.pathH, points: n.points, closed: n.closed };
        }));
        await target.keyboard.press("Escape");
        expect(await target.evaluate(() => window.vellum.state.pathEdit)).toBeNull();
    }
    expect(paths[0]).toEqual(paths[1]);
    await info.attach("twin-path", { body: JSON.stringify(paths), contentType: "application/json" });
    await twin.close();
});

test("two touch pointers pinch around the same world point as the reference", async ({ page, context }, info) => {
    test.skip(!hasReference, "The optional Vellum source reference is absent.");
    const twin = await context.newPage();
    const states = [];
    for (const target of [page, twin]) {
        const box = await playground(target, target === twin);
        const session = await context.newCDPSession(target);
        const first = { x: box.x + 450, y: box.y + 350, id: 1 };
        const second = { x: box.x + 550, y: box.y + 350, id: 2 };
        await session.send("Input.dispatchTouchEvent", { type: "touchStart", touchPoints: [first] });
        await session.send("Input.dispatchTouchEvent", { type: "touchStart", touchPoints: [first, second] });
        await session.send("Input.dispatchTouchEvent", { type: "touchMove", touchPoints: [{ ...first, x: first.x - 20, y: first.y + 15 }, { ...second, x: second.x + 40, y: second.y + 15 }] });
        await session.send("Input.dispatchTouchEvent", { type: "touchEnd", touchPoints: [] });
        await present(target);
        states.push(await geometry(target));
        await session.detach();
    }
    expect(states[0]).toEqual(states[1]);
    expect(states[0].camera.zoom).toBe(1.6);
    await info.attach("twin-pinch", { body: JSON.stringify(states), contentType: "application/json" });
    await twin.close();
});

test("hover, live marquee, snapping guides, pen preview and rulers match the original overlay", async ({ page, context }, info) => {
    test.skip(!hasReference, "The optional Vellum source reference is absent.");
    const twin = await context.newPage();
    const captures = new Map<string, PNG[]>();
    for (const target of [page, twin]) {
        const box = await playground(target, target === twin);
        const cx = box.x + box.width / 2, cy = box.y + box.height / 2;
        const ids = await target.evaluate(() => {
            const api = window.vellum;
            return [-120, 120].map(dx => {
                const node = api.createAtCenter("rect", { w: 80, h: 60 });
                api.select([node.id]); api.setProperty("x", node.x + dx);
                return node.id;
            });
        });
        const captureOverlay = async (name: string) => {
            await present(target);
            const png = await target.evaluate(() => (document.getElementById("overlay") as HTMLCanvasElement).toDataURL());
            const bytes = Buffer.from(png.split(",")[1], "base64");
            const pair = captures.get(name) ?? [];
            pair.push(PNG.sync.read(bytes)); captures.set(name, pair);
            await writeFile(info.outputPath(`${target === twin ? "reference" : "actual"}-${name}.png`), bytes);
        };
        await target.evaluate(() => window.vellum.select([]));
        await target.mouse.move(cx - 120, cy);
        await captureOverlay("hover");
        await target.mouse.move(cx - 220, cy - 80);
        await target.mouse.down();
        await target.mouse.move(cx + 180, cy + 80, { steps: 4 });
        await captureOverlay("marquee");
        await target.mouse.up();
        await target.evaluate(id => window.vellum.select([id]), ids[0]);
        await target.mouse.move(cx - 120, cy);
        await target.mouse.down();
        await target.mouse.move(cx + 117, cy, { steps: 5 });
        expect(await target.evaluate(() => window.vellum.state.guides.length)).toBeGreaterThan(0);
        await captureOverlay("snap-guides");
        await target.keyboard.press("Escape");
        await target.mouse.up();
        await target.evaluate(() => { window.vellum.select([]); window.vellum.actions.rulers(); });
        await captureOverlay("rulers");
        await target.keyboard.press("p");
        await drag(target, { x: cx - 160, y: cy + 150 }, { x: cx - 130, y: cy + 130 });
        await target.mouse.click(cx, cy + 230);
        await target.mouse.move(cx + 160, cy + 160);
        await captureOverlay("pen-preview");
        await target.keyboard.press("Escape");
    }
    for (const [name, [actual, reference]] of captures) {
        expect(differingPixels(actual, reference), `${name} must agree at RGB threshold 24`).toBe(0);
    }
    await twin.close();
});
