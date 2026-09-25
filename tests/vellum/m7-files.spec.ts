import fs from "node:fs/promises";
import path from "node:path";
import { PNG } from "pngjs";
import { expect, present, test, waitForReady } from "./support";
import { hasReference, openTwin } from "./twin";
import { rounds } from "../tier";

const emptyDocument = { format: "vellum", version: 1, name: "Empty import", pageId: "empty", pages: [{ id: "empty", name: "Empty", nodes: [] }], assets: {} };
const imageSvg = '<svg xmlns="http://www.w3.org/2000/svg" width="80" height="60"><rect width="80" height="60" fill="#339966"/></svg>';

async function importDocument(page, value: object) {
    await page.evaluate(async value => {
        await (window.vellum as any).importDocument(new File([JSON.stringify(value)], "import.vellum", { type: "application/json" }));
    }, value);
}

async function storedDocument(page) {
    return page.evaluate(() => new Promise<any>((resolve, reject) => {
        const request = indexedDB.open("vellum-editor", 1);
        request.onerror = () => reject(request.error);
        request.onsuccess = () => {
            const db = request.result;
            const read = db.transaction("documents").objectStore("documents").get("current");
            read.onsuccess = () => { db.close(); resolve(read.result ? JSON.parse(read.result) : null); };
            read.onerror = () => { db.close(); reject(read.error); };
        };
    }));
}

// These reload the page to read storage back, or replace the browser's storage
// with init scripts and prototype stubs that stay in place.
test.describe(() => {
    test.use({ fresh: true });

    test("debounced local save restores the document and options after reload", async ({ page }) => {
        await waitForReady(page);
        await page.locator("[data-page]").nth(2).click();
        const id = await page.evaluate(() => window.vellum.createAtCenter("rect", { name: "Persisted rectangle", w: 123, h: 87 }).id);
        await page.evaluate(() => { window.vellum.actions.theme(); window.vellum.actions.grid(); window.vellum.actions.rulers(); window.vellum.actions.snap(); });
        await page.locator("#dismiss-tip").click();
        await expect.poll(async () => (await storedDocument(page))?.pages.flatMap(page => page.nodes).some(node => node.id === id)).toBe(true);
        await expect(page.locator("#save-indicator")).toContainText("Saved locally");
        await waitForReady(page, "./", false);
        expect(await page.evaluate(id => window.vellum.doc.get(id)?.name ?? null, id)).toBe("Persisted rectangle");
        expect(await page.evaluate(() => (window.vellum as any).options)).toMatchObject({ theme: "light", grid: true, rulers: true, snap: false });
        expect(await page.evaluate(() => [...window.vellum.state.selection])).toEqual([]);
        await expect(page.locator("#welcome-tip")).toHaveCount(0);
    });

    test("IndexedDB open failure falls back to localStorage and preserves edits", async ({ page }) => {
        await page.addInitScript(() => { IDBFactory.prototype.open = () => { throw new DOMException("Injected storage denial", "SecurityError"); }; });
        await waitForReady(page);
        await page.locator("[data-page]").nth(2).click();
        const id = await page.evaluate(() => window.vellum.createAtCenter("ellipse", { name: "Fallback saved" }).id);
        await page.evaluate(() => (window.vellum as any).save());
        expect(await page.evaluate(() => JSON.parse(localStorage.getItem("vellum-document")!).pages.at(-1).nodes.length)).toBe(1);
        await page.evaluate(() => window.vellum.actions.settings());
        await expect(page.locator("#modal")).toContainText("localStorage");
        await waitForReady(page, "./", false);
        expect(await page.evaluate(id => window.vellum.doc.get(id)?.name, id)).toBe("Fallback saved");
    });

    test("quota failure shows an unsaved state while portable export remains available", async ({ page }) => {
        await waitForReady(page);
        await page.evaluate(() => {
            IDBObjectStore.prototype.put = () => { throw new DOMException("Injected quota failure", "QuotaExceededError"); };
            window.vellum.createAtCenter("rect", { name: "Still exportable" });
        });
        await expect(page.locator("#save-indicator")).toContainText("Save failed", { timeout: 10_000 });
        await expect(page.locator("#toast")).toContainText(/full|unavailable/);
        const downloadPromise = page.waitForEvent("download");
        await page.evaluate(() => window.vellum.actions.saveFile());
        const download = await downloadPromise;
        const saved = JSON.parse(await fs.readFile((await download.path())!, "utf8"));
        expect(saved.pages[0].nodes.some(node => node.name === "Still exportable")).toBe(true);
    });

    test("an aborted IndexedDB write and an unavailable fallback both report save failure", async ({ page }) => {
        await waitForReady(page);
        await page.evaluate(() => {
            const put = IDBObjectStore.prototype.put;
            IDBObjectStore.prototype.put = function (...args) {
                const request = put.apply(this, args as [any, IDBValidKey?]);
                this.transaction.abort();
                return request;
            };
            window.vellum.createAtCenter("rect", { name: "Aborted write" });
        });
        await expect(page.locator("#save-indicator")).toContainText("Save failed", { timeout: 10_000 });
        await page.addInitScript(() => {
            IDBFactory.prototype.open = () => { throw new DOMException("Injected denial", "SecurityError"); };
            Storage.prototype.setItem = () => { throw new DOMException("Injected quota failure", "QuotaExceededError"); };
        });
        await waitForReady(page);
        await page.evaluate(() => window.vellum.createAtCenter("rect", { name: "No local store" }));
        await expect(page.locator("#save-indicator")).toContainText("Save failed", { timeout: 10_000 });
        expect(await page.evaluate(() => window.vellum.doc.nodes.at(-1)?.name)).toBe("No local store");
    });
});

test("an unfinished pointer transaction is never persisted as a completed edit", async ({ page }) => {
    await waitForReady(page);
    await page.locator("[data-page]").nth(2).click();
    await page.evaluate(() => (window.vellum as any).save());
    await page.locator("#overlay").focus();
    await page.keyboard.press("r");
    const area = (await page.locator("#overlay").boundingBox())!;
    await page.mouse.move(area.x + 100, area.y + 100);
    await page.mouse.down();
    await page.mouse.move(area.x + 300, area.y + 250, { steps: 4 });
    await page.waitForTimeout(800);
    expect((await storedDocument(page)).pages.at(-1).nodes).toHaveLength(0);
    await page.mouse.up();
    await expect.poll(async () => (await storedDocument(page)).pages.at(-1).nodes.length).toBe(1);
});

// An init script, and runtime traps that spend the instance's restarts.
test.describe(() => {
    test.use({ fresh: true });

    test("malformed saved data opens a usable starter and explains the recovery", async ({ page }) => {
        await page.addInitScript(() => {
            IDBFactory.prototype.open = () => { throw new DOMException("Injected denial", "SecurityError"); };
            localStorage.setItem("vellum-document", "{broken");
        });
        await waitForReady(page);
        expect(await page.evaluate(() => window.vellum.doc.nodes.length)).toBe(171);
        await expect(page.locator("#toast")).toContainText("could not be restored");
        await page.evaluate(() => window.vellum.createAtCenter("rect", { name: "Recovered editor" }));
        expect(await page.evaluate(() => window.vellum.doc.nodes.at(-1)?.name)).toBe("Recovered editor");
    });

    test("runtime traps recover saved work through three SDK restarts", async ({ page }) => {
        await waitForReady(page);
        await page.locator("[data-page]").nth(2).click();
        const id = await page.evaluate(async () => {
            const node = window.vellum.createAtCenter("rect", { name: "Survives restart" });
            await (window.vellum as any).save();
            return node.id;
        });
        for (let round = 0; round < 3; round++) {
            await page.evaluate(() => {
                window.vellum.createAtCenter("rect", { name: "Unsaved draft" });
                (window.__vellum as any).hooks.runtime.enter_fatal(new Error("injected Vellum recovery trap"));
            });
            await expect(page.locator("#status")).toHaveAttribute("data-status", "fatal");
            await expect(page.getByTestId("fatal-text")).toContainText("unsaved changes were lost");
            await page.waitForTimeout(700);
            expect((await storedDocument(page)).pages.at(-1).nodes.some(node => node.name === "Unsaved draft")).toBe(false);
            await page.getByTestId("fatal-restart").click();
            await page.waitForFunction(() => window.vellum?.ready);
            expect(await page.evaluate(id => window.vellum.doc.get(id)?.name, id)).toBe("Survives restart");
            expect(await page.evaluate(() => window.vellum.doc.nodes.some(node => node.name === "Unsaved draft"))).toBe(false);
            expect(await page.evaluate(() => window.__vellum.live_regions())).toBe(1);
        }
        await page.evaluate(() => (window.__vellum as any).hooks.runtime.enter_fatal(new Error("restart limit")));
        await expect(page.locator("#status")).toHaveAttribute("data-status", "fatal");
        await expect(page.getByTestId("fatal-restart")).toHaveCount(0);
        await expect(page.getByRole("button", { name: /reload/i })).toBeVisible();
    });

    test("a runtime trap aborts a storage write before its transaction commits", async ({ page }) => {
        await waitForReady(page);
        await page.locator("[data-page]").nth(2).click();
        await page.evaluate(() => (window.vellum as any).save());
        await page.evaluate(() => {
            const put = IDBObjectStore.prototype.put;
            IDBObjectStore.prototype.put = function (...args) {
                const request = put.apply(this, args as [any, IDBValidKey?]);
                IDBObjectStore.prototype.put = put;
                (window.__vellum as any).hooks.runtime.enter_fatal(new Error("trap during storage write"));
                return request;
            };
            window.vellum.createAtCenter("rect", { name: "Interrupted storage write" });
            void (window.vellum as any).save().catch(() => undefined);
        });
        await expect(page.locator("#status")).toHaveAttribute("data-status", "fatal");
        await page.waitForTimeout(700);
        expect((await storedDocument(page)).pages.at(-1).nodes).toHaveLength(0);
        await page.getByTestId("fatal-restart").click();
        await page.waitForFunction(() => window.vellum?.ready);
        expect(await page.evaluate(() => window.vellum.doc.nodes.length)).toBe(0);
    });
});

test("image import embeds pixels and undoing document replacement restores assets", async ({ page }) => {
    await waitForReady(page);
    await page.locator("[data-page]").nth(2).click();
    await page.evaluate(async source => {
        await (window.vellum as any).importImage(new File([source], "test.svg", { type: "image/svg+xml" }), { x: 120, y: 90 });
    }, imageSvg);
    const node = await page.evaluate(() => window.vellum.doc.nodes.at(-1)!);
    expect(node).toMatchObject({ type: "image", w: 80, h: 60, x: 80, y: 60 });
    const source = await page.evaluate(id => JSON.parse(window.vellum.doc.serialize()).assets[id], node.assetId);
    expect(source).toMatch(/^data:image\/svg\+xml/);
    await importDocument(page, emptyDocument);
    expect(await page.evaluate(() => window.vellum.doc.nodes.length)).toBe(0);
    await page.evaluate(() => window.vellum.actions.undo());
    expect(await page.evaluate(id => JSON.parse(window.vellum.doc.serialize()).assets[id], node.assetId)).toBe(source);
    expect(await page.evaluate(id => window.vellum.doc.get(id)?.assetId, node.id)).toBe(node.assetId);
    await present(page);
    const pixel = await page.evaluate(async id => {
        const canvas = await (window.vellum.renderer as any).exportCanvas([id], 1);
        return [...canvas.getContext("2d").getImageData(20, 20, 1, 1).data];
    }, node.id);
    expect(pixel).toEqual([51, 153, 102, 255]);
});

test("document picker uses SDK files and portable downloads preserve unknown fields", async ({ page }, info) => {
    await waitForReady(page);
    const imported = { ...emptyDocument, name: "Portable: / document", futureFlag: { kept: true }, pages: [{ id: "empty", name: "Empty", extraPage: 17, nodes: [] }] };
    const chooserPromise = page.waitForEvent("filechooser");
    await page.evaluate(() => window.vellum.actions.openFile());
    const chooser = await chooserPromise;
    await chooser.setFiles({ name: "portable.vellum", mimeType: "application/json", buffer: Buffer.from(JSON.stringify(imported)) });
    await expect(page.locator("#file-name")).toHaveText(imported.name);
    const downloadPromise = page.waitForEvent("download");
    await page.evaluate(() => window.vellum.actions.saveFile());
    const download = await downloadPromise;
    expect(download.suggestedFilename()).not.toMatch(/[:/]/);
    const destination = info.outputPath("portable.vellum");
    await download.saveAs(destination);
    const result = JSON.parse(await fs.readFile(destination, "utf8"));
    expect(result.futureFlag).toEqual({ kept: true });
    expect(result.pages[0].extraPage).toBe(17);
    await info.attach("portable-document", { path: destination, contentType: "application/json" });
});

test("cyclic and injected documents are rejected without changing the current document", async ({ page }) => {
    await waitForReady(page);
    const before = await page.evaluate(() => window.vellum.doc.serialize());
    for (const kind of ["cycle", "injection", "malformed"]) {
        await page.evaluate(async ({ before, kind }) => {
            const value = JSON.parse(before);
            const node = value.pages[0].nodes[0];
            if (kind === "cycle") node.parentId = node.id;
            if (kind === "injection") node.id = '\"><img src=x onerror=alert(1)>';
            try { await (window.vellum as any).importDocument(new File([kind === "malformed" ? "{broken" : JSON.stringify(value)], "bad.vellum", { type: "application/json" })); }
            catch (error) { return String(error); }
            return null;
        }, { before, kind });
        expect(await page.evaluate(() => window.vellum.doc.serialize())).toBe(before);
        await expect(page.locator("#toast")).toBeVisible();
    }
});

test("file limits and invalid image decodes fail before any document mutation", async ({ page }) => {
    await waitForReady(page);
    const before = await page.evaluate(() => window.vellum.doc.serialize());
    const cases = await page.evaluate(async () => {
        const api = window.vellum as any;
        const attempt = async (run: () => Promise<any>) => { try { await run(); return "resolved"; } catch (error) { return String(error); } };
        return [
            await attempt(() => api.importDocument(new File([new Uint8Array(80 * 1024 * 1024 + 1)], "huge.vellum"))),
            await attempt(() => api.importImage(new File([new Uint8Array(25 * 1024 * 1024 + 1)], "huge.png", { type: "image/png" }))),
            await attempt(() => api.importImage(new File(["bad image"], "bad.png", { type: "image/png" }))),
            await attempt(() => api.importImage(new File(['<svg xmlns="http://www.w3.org/2000/svg" width="10000" height="10000"/>'], "huge.svg", { type: "image/svg+xml" }))),
        ];
    });
    expect(cases).toHaveLength(4);
    expect(await page.evaluate(() => window.vellum.doc.serialize())).toBe(before);
});

test("canvas file drops place images at the pointer and accept document files", async ({ page }) => {
    await waitForReady(page);
    await page.locator("[data-page]").nth(2).click();
    const world = await page.locator("#overlay").evaluate((element, source) => {
        const rect = element.getBoundingClientRect();
        const camera = window.vellum.state.camera;
        const data = new DataTransfer();
        data.items.add(new File([source], "dropped.svg", { type: "image/svg+xml" }));
        element.dispatchEvent(new DragEvent("dragover", { bubbles: true, cancelable: true, dataTransfer: data }));
        element.dispatchEvent(new DragEvent("drop", { bubbles: true, cancelable: true, dataTransfer: data, clientX: rect.left + 200, clientY: rect.top + 220 }));
        return { x: (200 - camera.x) / camera.zoom, y: (220 - camera.y) / camera.zoom };
    }, imageSvg);
    await expect.poll(() => page.evaluate(() => window.vellum.doc.nodes.length)).toBe(1);
    const node = await page.evaluate(() => window.vellum.doc.nodes[0]);
    expect(node.x + node.w / 2).toBeCloseTo(world.x, 6);
    expect(node.y + node.h / 2).toBeCloseTo(world.y, 6);
    await page.locator("#overlay").evaluate((element, value) => {
        const data = new DataTransfer();
        data.items.add(new File([JSON.stringify(value)], "dropped.vellum", { type: "application/json" }));
        element.dispatchEvent(new DragEvent("drop", { bubbles: true, cancelable: true, dataTransfer: data }));
    }, emptyDocument);
    await expect(page.locator("#file-name")).toHaveText("Empty import");
});

test("image picker scales large valid images and cancellation leaves the document untouched", async ({ page }) => {
    await waitForReady(page);
    await page.locator("[data-page]").nth(2).click();
    const chooserPromise = page.waitForEvent("filechooser");
    await page.evaluate(() => window.vellum.actions.placeImage());
    const chooser = await chooserPromise;
    await chooser.setFiles({ name: "large.svg", mimeType: "image/svg+xml", buffer: Buffer.from('<svg xmlns="http://www.w3.org/2000/svg" width="1600" height="1000"><rect width="1600" height="1000" fill="red"/></svg>') });
    await expect.poll(() => page.evaluate(() => window.vellum.doc.nodes.length)).toBe(1);
    expect(await page.evaluate(() => window.vellum.doc.nodes[0])).toMatchObject({ type: "image", w: 800, h: 500 });
    const before = await page.evaluate(() => window.vellum.doc.serialize());
    const cancelPromise = page.waitForEvent("filechooser");
    await page.evaluate(() => window.vellum.actions.openFile());
    const cancel = await cancelPromise;
    await cancel.element().evaluate(input => input.dispatchEvent(new Event("cancel")));
    expect(await page.evaluate(() => window.vellum.doc.serialize())).toBe(before);
});

// A `File.prototype.arrayBuffer` stub that stays in place, and a reload after saving.
test.describe(() => {
    test.use({ fresh: true });

    test("a file read completed after disposal cannot replace the new mounted editor", async ({ page }) => {
        await waitForReady(page);
        await page.evaluate(value => {
            const file = new File([JSON.stringify(value)], "late.vellum", { type: "application/json" });
            const buffer = new TextEncoder().encode(JSON.stringify(value)).buffer;
            const read = File.prototype.arrayBuffer;
            File.prototype.arrayBuffer = function () {
                if (this.name !== "late.vellum") return read.call(this);
                return new Promise(resolve => { (window as any).__finishLateRead = () => resolve(buffer); });
            };
            (window as any).__lateRead = (window.vellum as any).importDocument(file).catch(error => String(error));
        }, emptyDocument);
        await page.waitForFunction(() => typeof (window as any).__finishLateRead === "function");
        await page.evaluate(() => { (window.__vellum as any).dispose(); (window.__vellum as any).mount(); });
        await page.waitForFunction(() => window.vellum.ready);
        const name = await page.evaluate(() => window.vellum.doc.data.name);
        await page.evaluate(async () => { (window as any).__finishLateRead(); await (window as any).__lateRead; });
        expect(await page.evaluate(() => window.vellum.doc.data.name)).toBe(name);
        expect(await page.evaluate(() => window.__vellum.live_regions())).toBe(1);
    });

    test("local font picker embeds a font, applies it to text and restores it after reload", async ({ page }) => {
        await waitForReady(page);
        await page.locator("[data-page]").nth(2).click();
        const id = await page.evaluate(() => window.vellum.createAtCenter("text", { text: "Portable typography" }).id);
        const chooserPromise = page.waitForEvent("filechooser");
        await page.evaluate(() => window.vellum.actions.loadFont());
        const chooser = await chooserPromise;
        await chooser.setFiles(path.resolve(__dirname, "../../makepad/widgets/resources/LiberationMono-Regular.ttf"));
        await expect.poll(() => page.evaluate(id => window.vellum.doc.get(id).fontFamily, id)).toBe("LiberationMono-Regular");
        await page.evaluate(() => (window.vellum as any).save());
        await waitForReady(page, "./", false);
        const fonts = await page.evaluate(() => window.vellum.fontReady());
        expect(fonts.families).toContain("LiberationMono-Regular");
        expect(await page.evaluate(id => window.vellum.doc.get(id).fontFamily, id)).toBe("LiberationMono-Regular");
    });
});

test("font sources follow document replacement undo and redo without retaining removed faces", async ({ page }) => {
    await waitForReady(page);
    const mono = await fs.readFile(path.resolve(__dirname, "../../makepad/widgets/resources/LiberationMono-Regular.ttf"));
    const sans = await fs.readFile(path.resolve(__dirname, "../../makepad/widgets/resources/IBMPlexSans-Text.ttf"));
    const document = source => ({ ...emptyDocument, fonts: { "Recovery Font": `data:font/ttf;base64,${source.toString("base64")}` }, pages: [{ id: "empty", name: "Fonts", nodes: [{ id: "recovery_text", type: "text", text: "iiiiWWWW", fontFamily: "Recovery Font", fontSize: 32, x: 0, y: 0, w: 400, h: 70, rotation: 0, opacity: 1 }] }] });
    await importDocument(page, document(mono));
    const first = await page.evaluate(() => window.vellum.textLayout("recovery_text").widths[0]);
    await importDocument(page, document(sans));
    const second = await page.evaluate(() => window.vellum.textLayout("recovery_text").widths[0]);
    expect(Math.abs(first - second)).toBeGreaterThan(1);
    await page.evaluate(() => window.vellum.actions.undo());
    await expect.poll(() => page.evaluate(() => window.vellum.textLayout("recovery_text").widths[0])).toBeCloseTo(first, 4);
    await page.evaluate(() => window.vellum.actions.redo());
    await expect.poll(() => page.evaluate(() => window.vellum.textLayout("recovery_text").widths[0])).toBeCloseTo(second, 4);
    await importDocument(page, emptyDocument);
    expect((await page.evaluate(() => window.vellum.fontReady())).families).not.toContain("Recovery Font");
    await page.evaluate(() => window.vellum.actions.undo());
    await expect.poll(() => page.evaluate(() => window.vellum.textLayout("recovery_text").widths[0])).toBeCloseTo(second, 4);
});

// A runtime trap, and `EventTarget.prototype` wrappers that stay in place.
test.describe(() => {
    test.use({ fresh: true });

    test("local font faces leave the browser when their runtime fails", async ({ page }) => {
        await waitForReady(page);
        const bytes = await fs.readFile(path.resolve(__dirname, "../../makepad/widgets/resources/LiberationMono-Regular.ttf"));
        await page.evaluate(async dataUrl => {
            await window.vellum.importFont({ name: "Restart Font.ttf", dataUrl });
            await (window.vellum as any).save();
        }, `data:font/ttf;base64,${bytes.toString("base64")}`);
        expect(await page.evaluate(() => [...document.fonts].filter(face => face.family === "Restart Font").length)).toBe(1);
        await page.evaluate(() => (window.__vellum as any).hooks.runtime.enter_fatal(new Error("font cleanup")));
        await expect(page.locator("#status")).toHaveAttribute("data-status", "fatal");
        expect(await page.evaluate(() => [...document.fonts].filter(face => face.family === "Restart Font").length)).toBe(0);
        await page.getByTestId("fatal-restart").click();
        await page.waitForFunction(() => window.vellum?.ready);
        expect(await page.evaluate(() => [...document.fonts].filter(face => face.family === "Restart Font").length)).toBe(1);
    });

    test("SDK picker releases its listeners after both cancel and successful selection", async ({ page }) => {
        await waitForReady(page);
        await page.evaluate(() => {
            const add = EventTarget.prototype.addEventListener;
            const remove = EventTarget.prototype.removeEventListener;
            (window as any).__pickerListeners = 0;
            EventTarget.prototype.addEventListener = function (name, callback, options) {
                if (this instanceof HTMLInputElement && this.type === "file" && ["change", "cancel"].includes(name)) (window as any).__pickerListeners++;
                return add.call(this, name, callback, options);
            };
            EventTarget.prototype.removeEventListener = function (name, callback, options) {
                if (this instanceof HTMLInputElement && this.type === "file" && ["change", "cancel"].includes(name)) (window as any).__pickerListeners--;
                return remove.call(this, name, callback, options);
            };
        });
        const picks = rounds(4);
        for (let index = 0; index < picks; index++) {
            const choice = page.waitForEvent("filechooser");
            await page.evaluate(() => window.vellum.actions.openFile());
            const chooser = await choice;
            expect(await page.evaluate(() => (window as any).__pickerListeners)).toBe(2);
            if (index === picks - 1) {
                await chooser.setFiles({ name: "opened.vellum", mimeType: "application/json", buffer: Buffer.from(JSON.stringify(emptyDocument)) });
                await expect(page.locator("#file-name")).toHaveText("Empty import");
            } else {
                await chooser.element().evaluate(input => input.dispatchEvent(new Event("cancel")));
            }
            expect(await page.evaluate(() => (window as any).__pickerListeners)).toBe(0);
        }
    });
});

test("SVG and PNG exports contain real vector, text and image content with bounded dimensions", async ({ page }, info) => {
    await waitForReady(page);
    await importDocument(page, emptyDocument);
    const frame = await page.evaluate(() => {
        const api = window.vellum;
        const frame = api.createAtCenter("frame", { x: 0, y: 0, w: 550, h: 250, name: "Export frame", fill: "#eeeeee" });
        const text = api.createAtCenter("text", { parentId: frame.id, w: 200, h: 40, text: "Exported text", fill: "#000000" });
        const vector = api.createAtCenter("path", { parentId: frame.id, w: 80, h: 80, points: [{ x: 0, y: 0 }, { x: 80, y: 80 }, { x: 0, y: 80 }], pathW: 80, pathH: 80, closed: true, fill: "#ff0000" });
        api.transaction("Position export content", [[frame.id, 0, 0], [text.id, 40, 40], [vector.id, 260, 40]].flatMap(([id, x, y]) => [{ id: String(id), prop: "x", value: x }, { id: String(id), prop: "y", value: y }]));
        return frame.id;
    });
    await page.evaluate(async ({ source, frame }) => {
        await (window.vellum as any).importImage(new File([source], "test.svg", { type: "image/svg+xml" }), { x: 400, y: 120 });
        const image = window.vellum.doc.nodes.at(-1)!;
        window.vellum.transaction("Attach image", [{ id: image.id, prop: "parentId", value: frame }]);
    }, { source: imageSvg, frame });
    const exported = await page.evaluate(async frame => {
        const api = window.vellum as any;
        const svg = api.exportSVG([frame]);
        const xml = new DOMParser().parseFromString(svg, "image/svg+xml");
        const canvas = await api.renderer.exportCanvas([frame], 1);
        return { svg, valid: !xml.querySelector("parsererror"), text: !!xml.querySelector("text"), path: !!xml.querySelector("path"), image: !!xml.querySelector("image"), width: canvas.width, height: canvas.height, pixel: [...canvas.getContext("2d").getImageData(10, 10, 1, 1).data], imagePixel: [...canvas.getContext("2d").getImageData(400, 120, 1, 1).data] };
    }, frame);
    expect(exported).toMatchObject({ valid: true, text: true, path: true, image: true, width: 550, height: 250, pixel: [238, 238, 238, 255], imagePixel: [51, 153, 102, 255] });
    await info.attach("exported-svg", { body: exported.svg, contentType: "image/svg+xml" });
    const pngPromise = page.waitForEvent("download");
    await page.evaluate(frame => (window.vellum as any).doExport("PNG", 1, [frame]), frame);
    const png = await pngPromise;
    expect(png.suggestedFilename()).toBe("Export frame@1x.png");
    const decoded = PNG.sync.read(await fs.readFile((await png.path())!));
    expect([decoded.width, decoded.height]).toEqual([550, 250]);
    const rejected = await page.evaluate(async frame => {
        const api = window.vellum as any;
        const failures: string[] = [];
        for (const scale of [0, -1, 100]) {
            try { await api.renderer.exportCanvas([frame], scale); failures.push("accepted"); }
            catch (error) { failures.push(String(error)); }
        }
        return failures;
    }, frame);
    expect(rejected.every(value => value !== "accepted")).toBe(true);
});

test("frame presentation navigates, follows prototype links and returns keyboard focus", async ({ page }) => {
    await waitForReady(page);
    await importDocument(page, emptyDocument);
    const ids = await page.evaluate(() => {
        const api = window.vellum;
        const first = api.createAtCenter("frame", { name: "First frame", x: 0, y: 0, w: 400, h: 300, fill: "#ff0000" });
        const second = api.createAtCenter("frame", { name: "Second frame", x: 500, y: 0, w: 400, h: 300, fill: "#00ff00" });
        const link = api.createAtCenter("rect", { name: "Next frame link", parentId: first.id, x: 100, y: 100, w: 200, h: 100, prototypeTarget: second.id });
        api.createAtCenter("frame", { name: "Hidden frame", visible: false, x: 1000, y: 0, w: 400, h: 300 });
        api.transaction("Position presentation frames", [[first.id, 0, 0], [second.id, 500, 0], [link.id, 100, 100]].flatMap(([id, x, y]) => [{ id: String(id), prop: "x", value: x }, { id: String(id), prop: "y", value: y }]));
        api.select([link.id]);
        return { first: first.id, second: second.id };
    });
    await page.locator("#present").click();
    await expect(page.locator("#presentation")).toBeVisible();
    await expect(page.locator("#presentation-title")).toHaveText("First frame");
    await expect(page.locator("#presentation-count")).toHaveText("1 / 2");
    await expect.poll(() => page.locator("#presentation-canvas").evaluate((canvas: HTMLCanvasElement) => canvas.getContext("2d")!.getImageData(10, 10, 1, 1).data[0])).toBe(255);
    await page.locator("#presentation-canvas").click();
    await expect(page.locator("#presentation-title")).toHaveText("Second frame");
    await page.keyboard.press("ArrowRight");
    await expect(page.locator("#presentation-title")).toHaveText("First frame");
    await page.keyboard.press("ArrowLeft");
    await expect(page.locator("#presentation-title")).toHaveText("Second frame");
    await page.locator("#prev-frame").click();
    await expect(page.locator("#presentation-title")).toHaveText("First frame");
    await page.keyboard.press("Escape");
    await expect(page.locator("#presentation")).not.toBeVisible();
    expect(await page.evaluate(() => document.activeElement?.id)).toMatch(/present|overlay/);
    expect(await page.evaluate(ids => [window.vellum.doc.get(ids.first).x, window.vellum.doc.get(ids.second).x], ids)).toEqual([0, 500]);
});

// Twin comparisons open the twin in the test's own context and keep the page
// they compare there too.
test.describe(() => {
    test.use({ fresh: true });

    test("reference and Rust portable documents open in both directions", async ({ page, context }, info) => {
        test.skip(!hasReference, "Reference checkout is not available");
        const reference = await context.newPage();
        await openTwin(reference);
        await waitForReady(page);
        const original = await reference.evaluate(() => (window as any).vellum.doc.serialize());
        await importDocument(page, JSON.parse(original));
        const actual = await page.evaluate(() => window.vellum.doc.serialize());
        const rust = JSON.parse(actual);
        expect(rust.pages.map(page => page.nodes.length)).toEqual([171, 31, 0]);
        const reparsed = await reference.evaluate(async text => {
            const api = (window as any).vellum;
            await api.importDocument(new File([text], "rust.vellum", { type: "application/json" }));
            return api.doc.data;
        }, actual);
        expect(reparsed.pages.map(page => page.nodes.length)).toEqual([171, 31, 0]);
        for (let index = 0; index < rust.pages.length; index++) {
            expect(reparsed.pages[index]).toEqual(rust.pages[index]);
        }
        await fs.writeFile(info.outputPath("rust-roundtrip.vellum"), actual);
        await reference.close();
    });
});
