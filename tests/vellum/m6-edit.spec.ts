import fs from "node:fs/promises";
import { PNG } from "pngjs";
import { differingPixels, expect, present, test, waitForReady } from "./support";
import { hasReference, openTwin } from "./twin";

test("native text session commits multiline Unicode once and restores it with undo", async ({ page }) => {
    await waitForReady(page);
    await page.locator("[data-page]").nth(2).click();
    const id = await page.evaluate(() => {
        const node = window.vellum.createAtCenter("text", { text: "Typography test", w: 340, h: 80, fontSize: 24 });
        window.vellum.fit([node.id]);
        window.vellum.actions.editText();
        return node.id;
    });
    const editor = page.getByTestId("text-editor");
    await expect(editor).toBeVisible();
    await expect(editor).toBeFocused();
    await editor.fill("Vellum · Typography\nZażółć gęślą jaźń · مرحبا");
    await page.keyboard.press("Escape");
    await expect(editor).toHaveCount(0);
    expect(await page.evaluate(id => window.vellum.doc.get(id).text, id)).toBe("Vellum · Typography\nZażółć gęślą jaźń · مرحبا");
    expect(await page.evaluate(() => window.vellum.state.editing)).toBeNull();
    await page.evaluate(() => window.vellum.actions.undo());
    expect(await page.evaluate(id => window.vellum.doc.get(id).text, id)).toBe("Typography test");
    await page.evaluate(() => window.vellum.actions.redo());
    expect(await page.evaluate(id => window.vellum.doc.get(id).text, id)).toContain("مرحبا");
    // The opening Enter must not insert a newline into the newly focused, selected textarea.
    await page.locator("#overlay").focus();
    await page.keyboard.press("Enter");
    await expect(editor).toBeFocused();
    await expect(editor).toHaveValue("Vellum · Typography\nZażółć gęślą jaźń · مرحبا");
    await page.keyboard.press("Escape");
    await expect(editor).toHaveCount(0);
    expect(await page.evaluate(id => window.vellum.doc.get(id).text, id)).toBe("Vellum · Typography\nZażółć gęślą jaźń · مرحبا");
    await present(page);
});

test("duplication, grouping, frame wrapping and component history retain original edit semantics", async ({ page }) => {
    await waitForReady(page);
    await page.locator("[data-page]").nth(2).click();
    const original = await page.evaluate(() => {
        const api = window.vellum;
        const node = api.createAtCenter("rect", { w: 300, h: 120, rotation: 30 });
        api.actions.duplicate();
        return node.id;
    });
    let nodes = await page.evaluate(() => window.vellum.doc.nodes);
    expect(nodes).toHaveLength(2);
    expect(new Set(nodes.map(node => node.id)).size).toBe(2);
    const worlds = await page.evaluate(() => Object.fromEntries(window.vellum.doc.nodes.map(node => [node.id, window.vellum.doc.world(node.id).matrix])));
    await page.evaluate(() => { window.vellum.select(window.vellum.doc.nodes.map(node => node.id)); window.vellum.actions.group(); });
    nodes = await page.evaluate(() => window.vellum.doc.nodes);
    expect(nodes.filter(node => node.type === "group")).toHaveLength(1);
    expect(nodes.filter(node => node.parentId)).toHaveLength(2);
    for (const [id, matrix] of Object.entries(worlds)) {
        const actual = await page.evaluate(id => window.vellum.doc.world(id).matrix, id);
        actual.forEach((value, index) => expect(value).toBeCloseTo(matrix[index], 7));
    }
    await page.evaluate(() => window.vellum.actions.ungroup());
    expect(await page.evaluate(() => window.vellum.doc.nodes.every(node => !node.parentId))).toBe(true);
    await page.evaluate(() => { window.vellum.select(window.vellum.doc.nodes.map(node => node.id)); window.vellum.actions.frameSelection(); });
    expect(await page.evaluate(() => window.vellum.doc.nodes.filter(node => node.type === "frame").length)).toBe(1);
    await page.evaluate(() => { window.vellum.actions.undo(); window.vellum.select(window.vellum.doc.nodes.map(node => node.id)); window.vellum.actions.component(); });
    expect(await page.evaluate(() => window.vellum.doc.nodes.some(node => node.type === "group" && node.component))).toBe(true);
    await page.evaluate(() => window.vellum.actions.undo());
    expect(await page.evaluate(() => window.vellum.doc.nodes.some(node => node.type === "group" && !node.component))).toBe(true);
    await page.evaluate(() => window.vellum.actions.undo());
    expect(await page.evaluate(() => window.vellum.doc.nodes.length)).toBe(2);
    expect(await page.evaluate(id => Boolean(window.vellum.doc.get(id)), original)).toBe(true);
});

test("component instances propagate source changes while preserving overrides across pages", async ({ page }) => {
    await waitForReady(page);
    await page.locator("[data-page]").nth(2).click();
    const ids = await page.evaluate(() => {
        const api = window.vellum;
        const source = api.createAtCenter("rect", { w: 120, h: 80, fill: "#aabbcc" });
        api.actions.component();
        (api as any).instantiate(source.id);
        return { source: source.id, instance: [...api.state.selection][0] };
    });
    expect(await page.evaluate(id => window.vellum.doc.get(id).sourceId, ids.instance)).toBe(ids.source);
    await page.evaluate(ids => { const api = window.vellum; api.select([ids.source]); api.setProperty("fill", "#ee7733"); }, ids);
    expect(await page.evaluate(id => window.vellum.doc.get(id).fill, ids.instance)).toBe("#ee7733");
    await page.evaluate(ids => { const api = window.vellum; api.select([ids.instance]); api.setProperty("fill", "#33aa88"); api.select([ids.source]); api.setProperty("fill", "#112233"); }, ids);
    expect(await page.evaluate(id => window.vellum.doc.get(id).fill, ids.instance)).toBe("#33aa88");
    await page.locator("[data-page]").nth(1).click();
    await page.evaluate(id => (window.vellum as any).instantiate(id), ids.source);
    const crossPage = await page.evaluate(() => [...window.vellum.state.selection][0]);
    expect(await page.evaluate(id => window.vellum.doc.get(id).sourceId, crossPage)).toBe(ids.source);
    await page.locator("[data-page]").nth(2).click();
    await page.evaluate(id => { window.vellum.select([id]); window.vellum.setProperty("fill", "#abcdef"); }, ids.source);
    await page.locator("[data-page]").nth(1).click();
    expect(await page.evaluate(id => window.vellum.doc.get(id).fill, crossPage)).toBe("#abcdef");
});

test("auto layout and frame constraints use the original measured positions", async ({ page }) => {
    await waitForReady(page);
    await page.locator("[data-page]").nth(2).click();
    const ids = await page.evaluate(() => {
        const api = window.vellum;
        const frame = api.createAtCenter("frame", { w: 450, h: 250, name: "Auto layout test" });
        const first = api.createAtCenter("rect", { w: 80, h: 40 });
        const second = api.createAtCenter("rect", { w: 90, h: 50 });
        api.transaction("Arrange", [{ id: first.id, prop: "parentId", value: frame.id }, { id: second.id, prop: "parentId", value: frame.id }]);
        api.select([frame.id]); api.setProperty("layout", "horizontal"); api.setProperty("gap", 20); api.setProperty("padding", 16);
        return { frame: frame.id, first: first.id, second: second.id };
    });
    expect(await page.evaluate(ids => ({ a: window.vellum.doc.get(ids.first).x, b: window.vellum.doc.get(ids.second).x, y: window.vellum.doc.get(ids.first).y }), ids)).toEqual({ a: 16, b: 116, y: 16 });
    await page.getByRole("combobox", { name: "layout", exact: true }).selectOption("vertical");
    expect(await page.evaluate(id => ({ x: window.vellum.doc.get(id).x, y: window.vellum.doc.get(id).y }), ids.second)).toEqual({ x: 16, y: 76 });
    const oldX = await page.evaluate(ids => {
        const api = window.vellum;
        api.setProperty("layout", "none"); api.select([ids.first]); api.setProperty("constraintH", "right");
        const x = api.doc.get(ids.first).x;
        api.select([ids.frame]); api.setProperty("w", 550);
        return x;
    }, ids);
    expect(await page.evaluate(id => window.vellum.doc.get(id).x, ids.first)).toBe(oldX + 100);
});

test("alignment, distribution, stacking and delete operate on the selected roots", async ({ page }) => {
    await waitForReady(page);
    await page.locator("[data-page]").nth(2).click();
    const ids = await page.evaluate(() => {
        const api = window.vellum;
        const nodes = [80, 60, 100].map(w => api.createAtCenter("rect", { w, h: 50 }));
        api.transaction("Arrange", nodes.flatMap((node, index) => [
            { id: node.id, prop: "x", value: [100, 270, 500][index] },
            { id: node.id, prop: "y", value: [100, 150, 220][index] },
        ]));
        const ids = nodes.map(node => node.id); api.select(ids); return ids;
    });
    await page.getByRole("button", { name: "Align top", exact: true }).click();
    expect(await page.evaluate(() => window.vellum.doc.nodes.map(node => node.y))).toEqual([100, 100, 100]);
    await page.evaluate(() => window.vellum.actions.selectionMenu());
    await page.getByRole("menuitem", { name: "Distribute horizontally", exact: true }).click();
    expect(await page.evaluate(() => {
        const [a, b, c] = window.vellum.doc.nodes;
        return [b.x - a.x - a.w, c.x - b.x - b.w];
    })).toEqual([130, 130]);
    await page.evaluate(id => { window.vellum.select([id]); window.vellum.actions.front(); }, ids[0]);
    expect(await page.evaluate(() => window.vellum.doc.nodes.at(-1)!.id)).toBe(ids[0]);
    await page.evaluate(() => window.vellum.actions.backward());
    expect(await page.evaluate(() => window.vellum.doc.nodes[1].id)).toBe(ids[0]);
    await page.evaluate(() => window.vellum.actions.delete());
    expect(await page.evaluate(() => window.vellum.doc.nodes.map(node => node.id))).toEqual([ids[1], ids[2]]);
    await page.evaluate(() => window.vellum.actions.undo());
    expect(await page.evaluate(() => window.vellum.doc.nodes.length)).toBe(3);
    await expect(page.getByRole("status")).toContainText("Undo: Delete layers");
});

test("rotated native text follows the full world matrix while the camera changes", async ({ page }) => {
    await waitForReady(page);
    await page.locator("[data-page]").nth(2).click();
    const id = await page.evaluate(() => {
        const api = window.vellum;
        const frame = api.createAtCenter("frame", { w: 600, h: 450, rotation: 30 });
        const text = api.createAtCenter("text", { parentId: frame.id, text: "Rotated text", w: 220, h: 72, fontSize: 24, rotation: -12 });
        api.transaction("Position text", [{ id: text.id, prop: "x", value: 50 }, { id: text.id, prop: "y", value: 60 }]);
        api.fit([text.id]); api.actions.editText();
        return text.id;
    });
    const editor = page.getByTestId("text-editor");
    await expect(editor).toBeVisible();
    for (const factor of [1, 0.65]) {
        await page.evaluate(factor => window.vellum.zoomAt(factor, 350, 200), factor);
        await present(page);
        const expected = await page.evaluate(id => {
            const api = window.vellum, node = api.doc.get(id), matrix = api.doc.world(id).matrix, camera = api.state.camera;
            const origin = document.getElementById("overlay")!.getBoundingClientRect();
            const corners = [[0, 0], [node.w, 0], [0, node.h], [node.w, node.h]].map(([x, y]) => ({ x: origin.x + (matrix[0] * x + matrix[2] * y + matrix[4]) * camera.zoom + camera.x, y: origin.y + (matrix[1] * x + matrix[3] * y + matrix[5]) * camera.zoom + camera.y }));
            const xs = corners.map(point => point.x), ys = corners.map(point => point.y);
            return { x: Math.min(...xs), y: Math.min(...ys), width: Math.max(...xs) - Math.min(...xs), height: Math.max(...ys) - Math.min(...ys) };
        }, id);
        const actual = await editor.boundingBox();
        expect(actual).not.toBeNull();
        for (const key of ["x", "y", "width", "height"] as const) expect(actual![key]).toBeCloseTo(expected[key], 0);
        await expect(editor).toBeFocused();
    }
    await editor.fill("Changed at an angle");
    await page.keyboard.press("Control+Enter");
    await expect(editor).toHaveCount(0);
    expect(await page.evaluate(id => window.vellum.doc.get(id).text, id)).toBe("Changed at an angle");
});

test("text completion precedes another transaction and native undo stays in the control", async ({ page }) => {
    await waitForReady(page);
    await page.locator("[data-page]").nth(2).click();
    const id = await page.evaluate(() => { const n = window.vellum.createAtCenter("text", { text: "Original", w: 240, h: 70 }); window.vellum.actions.editText(); return n.id; });
    const editor = page.getByTestId("text-editor");
    await expect(editor).toBeFocused();
    await editor.press("End");
    await editor.pressSequentially(" native");
    await editor.press("ControlOrMeta+z");
    await expect(editor).toHaveValue("Original");
    expect(await page.evaluate(() => window.vellum.doc.nodes.length)).toBe(1);
    await editor.fill("Committed before duplicate");
    await page.evaluate(() => window.vellum.actions.duplicate());
    await expect(editor).toHaveCount(0);
    expect(await page.evaluate(() => window.vellum.doc.nodes.map(node => node.text))).toEqual(["Committed before duplicate", "Committed before duplicate"]);
    await page.evaluate(() => window.vellum.actions.undo());
    expect(await page.evaluate(() => window.vellum.doc.nodes.length)).toBe(1);
    expect(await page.evaluate(id => window.vellum.doc.get(id).text, id)).toBe("Committed before duplicate");
    await page.evaluate(() => window.vellum.actions.undo());
    expect(await page.evaluate(id => window.vellum.doc.get(id).text, id)).toBe("Original");
});

test("composition keeps Enter and Escape in the text control until composition ends", async ({ page }) => {
    await waitForReady(page);
    await page.locator("[data-page]").nth(2).click();
    const id = await page.evaluate(() => { const n = window.vellum.createAtCenter("text", { text: "", w: 260, h: 70 }); window.vellum.actions.editText(); return n.id; });
    const editor = page.getByTestId("text-editor");
    await expect(editor).toBeFocused();
    const cdp = await page.context().newCDPSession(page);
    await cdp.send("Input.imeSetComposition", { text: "拼音输入", selectionStart: 4, selectionEnd: 4 });
    await expect(editor).toHaveValue("拼音输入");
    await editor.dispatchEvent("keydown", { key: "Enter", code: "Enter", isComposing: true, bubbles: true });
    await editor.dispatchEvent("keydown", { key: "Escape", code: "Escape", isComposing: true, bubbles: true });
    await expect(editor).toBeFocused();
    expect(await page.evaluate(() => window.vellum.state.editing)).toBe(id);
    await cdp.send("Input.insertText", { text: "拼音输入" });
    await expect(editor).toHaveValue("拼音输入");
    await page.keyboard.press("Escape");
    await expect(editor).toHaveCount(0);
    expect(await page.evaluate(id => window.vellum.doc.get(id).text, id)).toBe("拼音输入");
    await cdp.detach();
});

test("text tool and canvas double-click open the native session and blur commits once", async ({ page }) => {
    await waitForReady(page);
    await page.locator("[data-page]").nth(2).click();
    await page.keyboard.press("t");
    const area = (await page.locator("#overlay").boundingBox())!;
    await page.mouse.click(area.x + 300, area.y + 240);
    const editor = page.getByTestId("text-editor");
    await expect(editor).toBeVisible();
    await editor.fill("Created with the text tool");
    await page.keyboard.press("Escape");
    await expect(editor).toHaveCount(0);
    const point = await page.evaluate(() => {
        const api = window.vellum, node = api.doc.nodes[0], m = api.doc.world(node.id).matrix, camera = api.state.camera;
        return { id: node.id, x: (m[0] * node.w / 2 + m[2] * node.h / 2 + m[4]) * camera.zoom + camera.x, y: (m[1] * node.w / 2 + m[3] * node.h / 2 + m[5]) * camera.zoom + camera.y };
    });
    await page.mouse.dblclick(area.x + point.x, area.y + point.y);
    await expect(editor).toHaveValue("Created with the text tool");
    await editor.fill("Committed by blur");
    await page.getByRole("button", { name: "Toggle theme", exact: true }).click();
    await expect(editor).toHaveCount(0);
    expect(await page.evaluate(id => window.vellum.doc.get(id).text, point.id)).toBe("Committed by blur");
    await page.evaluate(() => window.vellum.actions.undo());
    expect(await page.evaluate(id => window.vellum.doc.get(id).text, point.id)).toBe("Created with the text tool");
});

test("SDK clipboard handles plain text, validated layers and browser refusal without losing the internal copy", async ({ page }) => {
    await waitForReady(page);
    await page.locator("[data-page]").nth(2).click();
    await page.evaluate(() => {
        (window as any).clipboardText = "External text · 中文";
        Object.defineProperty(navigator, "clipboard", { configurable: true, value: {
            writeText: async (value: string) => { (window as any).clipboardText = value; },
            readText: async () => (window as any).clipboardText,
        } });
        window.vellum.actions.paste();
    });
    await expect.poll(() => page.evaluate(() => window.vellum.doc.nodes.at(-1)?.text)).toBe("External text · 中文");
    const copied = await page.evaluate(() => {
        const n = window.vellum.createAtCenter("rect", { w: 120, h: 80, rotation: 15 });
        window.vellum.actions.copy();
        return n;
    });
    await expect.poll(() => page.evaluate(() => JSON.parse((window as any).clipboardText).format)).toBe("vellum-clipboard");
    const payload = await page.evaluate(() => JSON.parse((window as any).clipboardText));
    expect(payload.rootIds).toEqual([copied.id]);
    expect(payload.nodes).toHaveLength(1);
    await page.evaluate(() => window.vellum.actions.paste());
    await expect.poll(() => page.evaluate(() => window.vellum.doc.nodes.length)).toBe(3);
    const pasted = await page.evaluate(() => window.vellum.doc.nodes.at(-1)!);
    expect(pasted.x).toBeCloseTo(copied.x + 24, 7);
    expect(pasted.y).toBeCloseTo(copied.y + 24, 7);
    expect(pasted.rotation).toBeCloseTo(copied.rotation, 7);
    await page.evaluate(() => {
        navigator.clipboard.writeText = async () => { throw new DOMException("Test denial", "NotAllowedError"); };
        navigator.clipboard.readText = async () => { throw new DOMException("Test denial", "NotAllowedError"); };
        window.vellum.actions.copy();
    });
    await expect(page.getByRole("status")).toContainText(/browser|clipboard|internal/i);
    await page.evaluate(() => window.vellum.actions.paste());
    await expect.poll(() => page.evaluate(() => window.vellum.doc.nodes.length)).toBe(4);
    const before = await page.evaluate(() => window.vellum.doc.serialize());
    await page.evaluate(payload => {
        payload.nodes[0].parentId = payload.nodes[0].id;
        navigator.clipboard.readText = async () => JSON.stringify(payload);
        window.vellum.actions.paste();
    }, payload);
    await expect(page.getByRole("status")).toContainText(/not valid|invalid/i);
    expect(await page.evaluate(() => window.vellum.doc.serialize())).toBe(before);
});

test("external text invalidation discards only the draft and preserves the incoming value", async ({ page }) => {
    await waitForReady(page);
    await page.locator("[data-page]").nth(2).click();
    const id = await page.evaluate(() => { const n = window.vellum.createAtCenter("text", { text: "Initial", w: 240, h: 70 }); window.vellum.actions.editText(); return n.id; });
    const editor = page.getByTestId("text-editor");
    await expect(editor).toBeVisible();
    await editor.fill("Uncommitted draft");
    await page.evaluate(id => (window.__vellum as any).replaceTextExternally(id, "External update"), id);
    await expect(editor).toHaveCount(0);
    expect(await page.evaluate(id => window.vellum.doc.get(id).text, id)).toBe("External update");
    await page.evaluate(() => window.vellum.actions.editText());
    await expect(editor).toHaveValue("External update");
    await editor.fill("Next edit");
    await page.keyboard.press("Escape");
    await page.evaluate(() => window.vellum.actions.undo());
    expect(await page.evaluate(id => window.vellum.doc.get(id).text, id)).toBe("External update");
});

test("text and pen editing match twin geometry and the approved screenshot gate", async ({ page, context }, info) => {
    test.skip(!hasReference, "The optional Vellum source reference is absent.");
    await waitForReady(page);
    const twin = await context.newPage();
    await openTwin(twin);
    const results: { operation: string; differing: number; ratio: number }[] = [];
    for (const operation of ["text", "pen"]) {
        const geometry: unknown[] = [];
        for (const target of [page, twin]) {
            await target.evaluate(() => window.vellum.actions.resetStarter());
            await target.locator("[data-page]").nth(2).click();
            await target.evaluate(() => window.vellum.select([]));
            if (operation === "text") {
                await target.evaluate(() => {
                    const api = window.vellum;
                    const node = api.createAtCenter("text", { text: "Typography test", w: 340, h: 80, fontSize: 24, rotation: 15 });
                    api.fit([node.id]); api.actions.editText();
                });
                await target.locator(target === page ? '[data-testid="text-editor"]' : "#text-editor").fill("Vellum · Typography\nZażółć gęślą jaźń · مرحبا");
                await target.keyboard.press("Escape");
            } else {
                await target.keyboard.press("p");
                const box = (await target.locator("#overlay").boundingBox())!;
                await target.mouse.move(box.x + 200, box.y + 300); await target.mouse.down();
                await target.mouse.move(box.x + 240, box.y + 280); await target.mouse.up();
                await target.mouse.click(box.x + 350, box.y + 330);
                await target.mouse.click(box.x + 280, box.y + 430);
                await target.keyboard.press("Enter");
                await target.mouse.dblclick(box.x + 350, box.y + 330);
                await target.mouse.move(box.x + 350, box.y + 330); await target.mouse.down();
                await target.mouse.move(box.x + 370, box.y + 350, { steps: 4 }); await target.mouse.up();
                await target.keyboard.press("Escape");
            }
            geometry.push(await target.evaluate(() => window.vellum.doc.nodes.map(node => Object.fromEntries(
                ["type", "name", "text", "x", "y", "w", "h", "rotation", "points", "pathW", "pathH"].filter(key => key in node && (key !== "text" || node.type === "text")).map(key => [key, node[key]])
            ))));
            await target.evaluate(() => {
                window.vellum.select([]);
                for (const id of ["toast", "welcome-tip"]) { const el = document.getElementById(id); if (el) el.style.display = "none"; }
            });
            await present(target);
            await target.evaluate(() => {
                for (const id of ["engine-label", "performance"]) { const el = document.getElementById(id); if (el) el.textContent = "Renderer"; }
            });
        }
        const normalized = (value: unknown) => JSON.parse(JSON.stringify(value, (_, item) => typeof item === "number" ? Math.round(item * 1e6) / 1e6 : item));
        expect(normalized(geometry[0])).toEqual(normalized(geometry[1]));
        const actual = PNG.sync.read(await page.screenshot({ path: info.outputPath(`actual-${operation}.png`) }));
        const reference = PNG.sync.read(await twin.screenshot({ path: info.outputPath(`reference-${operation}.png`) }));
        const differing = differingPixels(actual, reference);
        const result = { operation, differing, ratio: differing / (actual.width * actual.height) };
        results.push(result);
        console.log(`Vellum edit visual: ${JSON.stringify(result)}`);
        expect(result.ratio).toBeLessThanOrEqual(0.02);
    }
    await fs.writeFile("test-results/vellum/m6-edit-visual.json", JSON.stringify({ maxRatio: 0.02, results }, null, 2));
    await twin.close();
});

test("quick insert, presets and design token downloads use the editor controls", async ({ page }) => {
    await waitForReady(page);
    await page.locator("[data-page]").nth(2).click();
    await page.getByRole("button", { name: "Assets", exact: true }).click();
    await page.locator('[data-asset="button"]').click();
    expect(await page.evaluate(() => window.vellum.doc.nodes.map(node => node.type))).toEqual(["frame", "text"]);
    expect(await page.evaluate(() => window.vellum.doc.nodes[1].parentId)).toBe(await page.evaluate(() => window.vellum.doc.nodes[0].id));
    await page.evaluate(() => window.vellum.select([]));
    await page.locator('[data-preset="phone"]').click();
    expect(await page.evaluate(() => { const n = window.vellum.doc.get([...window.vellum.state.selection][0]); return { w: n.w, h: n.h }; })).toEqual({ w: 390, h: 844 });
    await page.evaluate(() => window.vellum.actions.tokens());
    const downloadPromise = page.waitForEvent("download");
    await page.getByRole("button", { name: "Export JSON", exact: true }).click();
    const download = await downloadPromise;
    expect(download.suggestedFilename()).toMatch(/\.json$/);
    const data = JSON.parse(await fs.readFile((await download.path())!, "utf8"));
    expect(data).toEqual(await page.evaluate(() => (window.vellum.doc.data as any).tokens));
});
