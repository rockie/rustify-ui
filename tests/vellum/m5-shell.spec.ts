import { expect, present, test, waitForReady } from "./support";

test("page switching and the width inspector preserve the original smoke behavior", async ({ page }) => {
    await waitForReady(page);
    expect(await page.locator("[data-page]").count()).toBe(3);
    await page.locator("[data-page]").nth(2).click();
    expect(await page.evaluate(() => window.vellum.doc.nodes.length)).toBe(0);
    const node = await page.evaluate(() => window.vellum.createAtCenter("rect", { w: 180, h: 100 }));
    const width = page.getByRole("spinbutton", { name: "w", exact: true });
    await expect(width).toBeVisible();
    await width.fill("240");
    await width.press("Tab");
    expect(await page.evaluate(id => window.vellum.doc.get(id).w, node.id)).toBe(240);
    const view = await page.evaluate(() => {
        window.vellum.zoomAt(1.25, 160, 120);
        return { camera: window.vellum.state.camera, selection: [...window.vellum.state.selection] };
    });
    await page.locator("[data-page]").nth(0).click();
    expect(await page.evaluate(() => window.vellum.doc.nodes.length)).toBe(171);
    await page.locator("[data-page]").nth(2).click();
    expect(await page.evaluate(id => window.vellum.doc.get(id).w, node.id)).toBe(240);
    expect(await page.evaluate(() => ({ camera: window.vellum.state.camera, selection: [...window.vellum.state.selection] }))).toEqual(view);
});

test("theme, command palette and responsive panels preserve the original smoke behavior", async ({ page }, info) => {
    await waitForReady(page);
    await page.getByRole("button", { name: "Toggle theme", exact: true }).click();
    await expect(page.locator(".vellum").first()).toHaveAttribute("data-theme", "light");
    await page.keyboard.press("Control+k");
    await page.getByRole("textbox", { name: "Search commands" }).fill("rulers");
    await page.keyboard.press("Enter");
    await expect(page.locator("#modal-backdrop")).toBeHidden();
    expect(await page.evaluate(() => (window.vellum as any).options.rulers)).toBe(true);
    await page.getByRole("button", { name: "Toggle theme", exact: true }).click();
    await expect(page.locator(".vellum").first()).toHaveAttribute("data-theme", "dark");
    await page.setViewportSize({ width: 1280, height: 800 });
    await present(page);
    expect(await page.evaluate(() => document.documentElement.scrollWidth)).toBe(1280);
    await page.getByRole("button", { name: "Toggle layers panel" }).click();
    await expect(page.locator("#left-panel")).toBeHidden();
    await page.getByRole("button", { name: "Toggle layers panel" }).click();
    await expect(page.locator("#left-panel")).toBeVisible();
    await page.locator("#overlay").focus();
    await page.keyboard.press("Tab");
    await expect(page.locator("#right-panel")).toBeHidden();
    await page.keyboard.press("Tab");
    await expect(page.locator("#right-panel")).toBeVisible();
    await page.screenshot({ path: info.outputPath("responsive-dark.png") });
});

test("focused inspector drafts preview live, commit one undo step and reject invalid hex", async ({ page }) => {
    await waitForReady(page);
    await page.locator("[data-page]").nth(2).click();
    const node = await page.evaluate(() => window.vellum.createAtCenter("rect", { w: 180, h: 100, fill: "#b8a2e2" }));
    const width = page.getByRole("spinbutton", { name: "w", exact: true });
    await width.focus();
    await width.press("ControlOrMeta+a");
    await width.pressSequentially("350", { delay: 40 });
    await expect(width).toBeFocused();
    await expect(width).toHaveValue("350");
    expect(await page.evaluate(id => window.vellum.doc.get(id).w, node.id)).toBe(350);
    await width.press("Tab");
    await page.evaluate(() => window.vellum.actions.undo());
    expect(await page.evaluate(id => window.vellum.doc.get(id).w, node.id)).toBe(180);
    const hex = page.getByRole("textbox", { name: "fill hex color", exact: true });
    await hex.fill("not-a-color");
    await hex.press("Tab");
    expect(await page.evaluate(id => window.vellum.doc.get(id).fill, node.id)).toBe("#b8a2e2");
    await expect(hex).toHaveValue("B8A2E2");
    await expect(page.getByRole("status")).toContainText("3- or 6-digit hex");
    await hex.fill("a3f");
    await hex.press("Tab");
    expect(await page.evaluate(id => window.vellum.doc.get(id).fill, node.id)).toBe("#aa33ff");
    await page.getByRole("combobox", { name: "fillType", exact: true }).selectOption("linear");
    await expect(page.getByRole("textbox", { name: "fill2 hex color" })).toBeVisible();
    const angle = page.getByRole("spinbutton", { name: "gradientAngle" });
    await angle.fill("45"); await angle.press("Tab");
    expect(await page.evaluate(id => window.vellum.doc.get(id).gradientAngle, node.id)).toBe(45);
});

test("typography, layout and prototype controls bind through accessible fields", async ({ page }) => {
    await waitForReady(page);
    await page.locator("[data-page]").nth(2).click();
    const nodes = await page.evaluate(() => {
        const api = window.vellum;
        const frame = api.createAtCenter("frame", { name: "Destination", w: 400, h: 300 });
        const text = api.createAtCenter("text", { name: "Sample", text: "Typography", w: 200, h: 80 });
        return { frame: frame.id, text: text.id };
    });
    await page.getByRole("combobox", { name: "fontFamily", exact: true }).selectOption("Georgia");
    await page.getByRole("combobox", { name: "fontWeight", exact: true }).selectOption("700");
    await page.getByRole("combobox", { name: "direction", exact: true }).selectOption("rtl");
    await page.getByRole("combobox", { name: "textCase", exact: true }).selectOption("upper");
    await page.getByRole("button", { name: "Italic", exact: true }).click();
    await page.getByRole("button", { name: "Underline", exact: true }).click();
    expect(await page.evaluate(id => window.vellum.doc.get(id), nodes.text)).toMatchObject({ fontFamily: "Georgia", fontWeight: 700, direction: "rtl", textCase: "upper", fontStyle: "italic", textDecoration: "underline" });
    await page.getByRole("button", { name: "Prototype", exact: true }).click();
    await page.getByRole("combobox", { name: "prototypeTarget", exact: true }).selectOption(nodes.frame);
    expect(await page.evaluate(id => window.vellum.doc.get(id).prototypeTarget, nodes.text)).toBe(nodes.frame);
    await page.getByRole("button", { name: "Design", exact: true }).click();
    await page.evaluate(id => window.vellum.select([id]), nodes.frame);
    await page.getByRole("combobox", { name: "layout", exact: true }).selectOption("horizontal");
    await expect(page.getByRole("spinbutton", { name: "gap", exact: true })).toBeVisible();
    await page.getByRole("combobox", { name: "layoutAlign", exact: true }).selectOption("center");
    expect(await page.evaluate(id => window.vellum.doc.get(id), nodes.frame)).toMatchObject({ layout: "horizontal", layoutAlign: "center" });
});

test("layer tree searches descendants, toggles visibility and lock, and renames safely", async ({ page }) => {
    await waitForReady(page);
    await page.locator("[data-page]").nth(2).click();
    const nodes = await page.evaluate(() => {
        const api = window.vellum;
        const frame = api.createAtCenter("frame", { name: "Parent", w: 400, h: 300 });
        const child = api.createAtCenter("rect", { name: "Needle <img src=x>", parentId: frame.id, w: 80, h: 60 });
        const sibling = api.createAtCenter("rect", { name: "Sibling", w: 70, h: 50 });
        api.select([]);
        return { frame: frame.id, child: child.id, sibling: sibling.id };
    });
    await page.getByRole("button", { name: "Search layers", exact: true }).click();
    const search = page.getByRole("textbox", { name: "Find a layer" });
    await search.fill("Needle");
    const child = page.locator(`[data-layer="${nodes.child}"]`);
    await expect(child).toBeVisible();
    await expect(page.locator(`[data-layer="${nodes.frame}"]`)).toBeVisible();
    await expect(page.locator(`[data-layer="${nodes.sibling}"]`)).toHaveCount(0);
    await expect(child.locator("img")).toHaveCount(0);
    await child.locator(".layer-name").click();
    await child.locator("[data-lock]").click();
    expect(await page.evaluate(id => window.vellum.doc.get(id).locked, nodes.child)).toBe(true);
    await child.locator("[data-visibility]").click();
    expect(await page.evaluate(id => window.vellum.doc.get(id).visible, nodes.child)).toBe(false);
    await search.fill("");
    const parent = page.locator(`[data-layer="${nodes.frame}"]`);
    if (await parent.getAttribute("aria-expanded") !== "true") await parent.locator("[data-expand]").click();
    await child.locator(".layer-name").dblclick();
    await expect(page.getByRole("heading", { name: "Rename layer", exact: true })).toBeVisible();
    await page.locator("#prompt-value").fill("Renamed & safe");
    await page.getByRole("button", { name: "Save", exact: true }).click();
    await expect(child.locator(".layer-name")).toHaveText("Renamed & safe");
    await page.locator(`[data-layer="${nodes.sibling}"] .layer-name`).click({ modifiers: ["Shift"] });
    expect(await page.evaluate(() => [...window.vellum.state.selection])).toEqual([nodes.child, nodes.sibling]);
});

test("native layer drag with Shift reparents while preserving world geometry", async ({ page }) => {
    await waitForReady(page);
    await page.locator("[data-page]").nth(2).click();
    const nodes = await page.evaluate(() => {
        const api = window.vellum;
        const frame = api.createAtCenter("frame", { name: "Rotated parent", w: 400, h: 300, rotation: 30 });
        const child = api.createAtCenter("rect", { name: "Dragged layer", w: 80, h: 60, rotation: 15 });
        return { frame: frame.id, child: child.id, world: api.doc.world(child.id).matrix };
    });
    const source = await page.locator(`[data-layer="${nodes.child}"]`).boundingBox();
    const target = await page.locator(`[data-layer="${nodes.frame}"]`).boundingBox();
    expect(source).not.toBeNull(); expect(target).not.toBeNull();
    await page.evaluate(() => {
        (window as any).layerDragEvents = [];
        for (const type of ["dragstart", "drop"]) document.addEventListener(type, event => {
            (window as any).layerDragEvents.push({ type, shift: (event as DragEvent).shiftKey });
        }, { capture: true, once: true });
    });
    await page.mouse.move(source!.x + source!.width / 2, source!.y + source!.height / 2);
    await page.mouse.down();
    // Chromium on macOS suppresses native drag initiation when Shift is already held.
    await page.mouse.move(source!.x + source!.width / 2, source!.y + source!.height / 2 + 8, { steps: 3 });
    await page.keyboard.down("Shift");
    await page.mouse.move(target!.x + target!.width / 2, target!.y + target!.height / 2, { steps: 5 });
    await page.mouse.move(target!.x + target!.width / 2 + 1, target!.y + target!.height / 2);
    await page.mouse.up();
    await page.keyboard.up("Shift");
    expect(await page.evaluate(() => (window as any).layerDragEvents)).toEqual([{ type: "dragstart", shift: false }, { type: "drop", shift: true }]);
    expect(await page.evaluate(id => window.vellum.doc.get(id).parentId, nodes.child)).toBe(nodes.frame);
    const matrix = await page.evaluate(id => window.vellum.doc.world(id).matrix, nodes.child);
    for (let index = 0; index < 6; index++) expect(matrix[index]).toBeCloseTo(nodes.world[index], 7);
    await page.evaluate(() => window.vellum.actions.undo());
    expect(await page.evaluate(id => window.vellum.doc.get(id).parentId, nodes.child)).toBeNull();
});

test("SDK menu and modal layers keep keyboard focus, Escape priority and background inert", async ({ page }) => {
    await waitForReady(page);
    const menuButton = page.getByRole("button", { name: "Vellum menu", exact: true });
    await menuButton.click();
    const menu = page.getByRole("menu");
    await expect(menu).toBeVisible();
    await expect(menu.locator("button:not([disabled])").first()).toBeFocused();
    await page.keyboard.press("Escape");
    await expect(menuButton).toBeFocused();
    await menuButton.click();
    await expect(menu).toBeVisible();
    await expect(menu.locator("button:not([disabled])").first()).toBeFocused();
    await page.keyboard.press("End");
    expect(await page.evaluate(() => document.activeElement?.getAttribute("data-menu-action"))).toBe("help");
    await page.keyboard.press("Enter");
    await expect(page.getByRole("heading", { name: "A few keys. Endless possibilities." })).toBeVisible();
    await expect(menu).toBeHidden();
    expect(await page.locator(".rustify-layer").count()).toBe(1);
    expect(await page.locator("#right-panel").evaluate(element => Boolean(element.closest("[inert]")))).toBe(true);
    const focusable = page.locator("#modal button:not([disabled]), #modal input:not([disabled]), #modal select:not([disabled]), #modal textarea:not([disabled])");
    await focusable.last().focus(); await page.keyboard.press("Tab");
    await expect(focusable.first()).toBeFocused();
    await page.keyboard.press("Shift+Tab");
    await expect(focusable.last()).toBeFocused();
    await page.keyboard.press("Escape");
    await expect(page.locator("#modal-backdrop")).toBeHidden();
    expect(await page.locator("#right-panel").evaluate(element => Boolean(element.closest("[inert]")))).toBe(false);
    await expect(page.locator(".rustify-layer")).toHaveCount(0);
    await page.getByRole("button", { name: "Keyboard shortcuts", exact: true }).click();
    await page.keyboard.press("Escape");
    await expect(page.getByRole("button", { name: "Keyboard shortcuts", exact: true })).toBeFocused();
});

test("document and page dialogs apply names and keep a new document undoable", async ({ page }) => {
    await waitForReady(page);
    await page.locator("#file-name").click();
    await page.locator("#prompt-value").fill("Named in the dialog");
    await page.getByRole("button", { name: "Save", exact: true }).click();
    await expect(page.locator("#file-name")).toHaveText("Named in the dialog");
    await page.getByRole("button", { name: "Add page", exact: true }).click();
    await page.locator("#prompt-value").fill("New page");
    await page.getByRole("button", { name: "Save", exact: true }).click();
    expect(await page.evaluate(() => window.vellum.doc.data.pages.length)).toBe(4);
    expect(await page.evaluate(() => window.vellum.doc.page.name)).toBe("New page");
    await page.locator("[data-page]").last().dblclick();
    await page.locator("#prompt-value").fill("Renamed page");
    await page.getByRole("button", { name: "Save", exact: true }).click();
    expect(await page.evaluate(() => window.vellum.doc.page.name)).toBe("Renamed page");
    await page.keyboard.press("Control+n");
    await expect(page.getByRole("heading", { name: "Start with a clean canvas." })).toBeVisible();
    await page.getByRole("button", { name: "New document", exact: true }).click();
    expect(await page.evaluate(() => window.vellum.doc.data.pages.length)).toBe(1);
    await page.evaluate(() => window.vellum.actions.undo());
    expect(await page.evaluate(() => ({ name: window.vellum.doc.data.name, page: window.vellum.doc.page.name, pages: window.vellum.doc.data.pages.length }))).toEqual({ name: "Named in the dialog", page: "Renamed page", pages: 4 });
});

test("zoom and selection menus support keyboard navigation and outside dismissal", async ({ page }) => {
    await waitForReady(page);
    await page.getByTitle("Zoom options", { exact: true }).click();
    await expect(page.getByRole("menuitem", { name: "Zoom to fit" })).toBeVisible();
    await expect(page.getByRole("menuitem", { name: "Zoom to fit" })).toBeFocused();
    await page.keyboard.press("Home");
    await page.keyboard.press("ArrowDown");
    await page.keyboard.press("ArrowDown");
    await page.keyboard.press("Enter");
    expect(await page.evaluate(() => window.vellum.state.camera.zoom)).toBe(1);
    await expect(page.getByRole("menu")).toHaveCount(0);
    await page.locator("[data-page]").nth(2).click();
    const id = await page.evaluate(() => window.vellum.createAtCenter("rect", { w: 120, h: 80 }).id);
    await page.locator(`[data-layer="${id}"]`).click({ button: "right" });
    await expect(page.getByRole("menuitem", { name: "Frame selection" })).toBeVisible();
    await page.getByRole("button", { name: "Toggle theme", exact: true }).click();
    await expect(page.getByRole("menu")).toHaveCount(0);
    expect(await page.evaluate(() => [...window.vellum.state.selection])).toEqual([id]);
});

test("tokens, CSS, settings and export dialogs expose their original controls", async ({ page }) => {
    await waitForReady(page);
    await page.keyboard.press("Control+k");
    await page.getByRole("textbox", { name: "Search commands" }).fill("Edit design tokens");
    await page.keyboard.press("Enter");
    await expect(page.getByRole("heading", { name: "Design tokens", exact: true })).toBeVisible();
    const before = await page.evaluate(() => (window.vellum.doc.data as any).tokens.colors[0].value);
    await page.locator('[data-token-name="0"]').fill("Updated style");
    await page.locator('[data-token-color="0"]').fill("#123456");
    await page.getByRole("button", { name: "Apply tokens", exact: true }).click();
    expect(await page.evaluate(() => (window.vellum.doc.data as any).tokens.colors[0])).toMatchObject({ name: "Updated style", value: "#123456" });
    expect(await page.evaluate(before => window.vellum.doc.data.pages.flatMap(page => page.nodes).some(node => [node.fill, node.fill2, node.stroke, node.shadowColor].some(value => value?.toLowerCase() === before.toLowerCase())), before)).toBe(false);
    await page.evaluate(() => {
        const node = window.vellum.doc.nodes.find(node => node.type === "rect")!;
        window.vellum.select([node.id]);
    });
    await page.locator('[data-action="inspectCSS"]').click();
    await expect(page.getByRole("heading", { name: "Inspect CSS", exact: true })).toBeVisible();
    await expect(page.locator("#modal .token-code")).toContainText("position: absolute");
    await page.keyboard.press("Escape");
    await page.getByRole("button", { name: "Editor settings", exact: true }).click();
    await expect(page.getByRole("heading", { name: "A workspace that feels like yours." })).toBeVisible();
    await page.getByRole("checkbox", { name: "Canvas dot grid" }).check();
    expect(await page.evaluate(() => (window.vellum as any).options.grid)).toBe(true);
    await page.keyboard.press("Escape");
    await page.locator("#share").click();
    await expect(page.getByRole("button", { name: "Download .vellum document" })).toBeVisible();
    await expect(page.getByRole("button", { name: "PNG image · 2×" })).toBeVisible();
    await expect(page.getByRole("button", { name: "SVG vector" })).toBeVisible();
    await page.keyboard.press("Escape");
    await expect(page.locator("#share")).toBeFocused();
});
