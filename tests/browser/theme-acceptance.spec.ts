import { test, expect, waitForReady, waitForQuiet, allowRegionStart, REGION_START_MS } from "./support";
import type { Page, TestInfo } from "@playwright/test";
import { readFile, writeFile } from "node:fs/promises";

const snapshot = (page: Page) => page.evaluate(() => window.__theme_studio.snapshot());
const preview = (page: Page) => page.getByTestId("studio-preview");

async function select(page: Page, id: string, label: string) {
    await page.getByTestId(id).click();
    await page.getByTestId(`${id}-list`).getByRole("option", { name: label, exact: true }).click();
}

async function color(page: Page, token: string, value: string) {
    await page.getByTestId("editor-colors").click();
    await page.getByTestId("color-search").fill("");
    await page.getByTestId(`color-token-${token}`).click();
    await page.getByTestId("color-value").fill(value);
    await page.getByTestId("color-value").press("Enter");
}

async function field(page: Page, token: string, value: string) {
    await page.getByTestId(`edit-${token}`).fill(value);
    await page.getByTestId(`edit-${token}`).press("Enter");
}

async function drawn(page: Page) {
    await page.getByTestId("theme-gpu").scrollIntoViewIfNeeded();
    await expect.poll(async () => {
        const state = await snapshot(page);
        return state.gpu_state === "Ready" && state.drawn_revision === state.revision && state.samples.length > 50
            && state.samples.find((sample: any) => sample.id === "font.sans.latin")?.layout_size_px?.[0] > 0;
    }, { timeout: REGION_START_MS }).toBe(true);
    return snapshot(page);
}

async function evidence(page: Page, info: TestInfo, name: string, gpu = false) {
    await waitForQuiet(page);
    await page.evaluate(() => {
        document.querySelector(".studio-previews")?.scrollTo(0, 0);
        document.querySelector("#studio-editor")?.scrollTo(0, 0);
        document.querySelector(".studio-editor-content")?.scrollTo(0, 0);
    });
    const path = info.outputPath(`${name}.png`);
    await page.screenshot({ path, animations: "disabled" });
    await info.attach(name, { path, contentType: "image/png" });
    if (gpu) {
        // The board is taller than these viewports. Scroll its real parent to
        // the reported samples so clipping cannot leave a blank font/shadow
        // area in an element screenshot that extends beyond the viewport.
        for (const [kind, id] of [["font", "font.sans.latin"], ["shadow", "shadow.shadow-lg"]]) {
            const state = await snapshot(page), sample = state.samples.find((sample: any) => sample.id === id);
            expect(sample, id).toBeDefined();
            await page.getByTestId("theme-gpu").evaluate((canvas, y: number) => {
                const pane = document.querySelector(".studio-previews") as HTMLElement;
                pane.scrollTop += canvas.getBoundingClientRect().top + y - pane.getBoundingClientRect().top - 80;
            }, sample.rect[1]);
            const path = info.outputPath(`${name}-gpu-${kind}.png`);
            await page.screenshot({ path, animations: "disabled" });
            await info.attach(`${name}-gpu-${kind}`, { path, contentType: "image/png" });
        }
    }
    await info.attach(`${name}-state`, {
        body: JSON.stringify({ viewport: page.viewportSize(), state: await snapshot(page), environment: await page.evaluate(() => {
            const canvas = document.querySelector('[data-testid="theme-gpu"]') as HTMLCanvasElement;
            const gl = canvas.getContext("webgl2");
            const extension = gl?.getExtension("WEBGL_debug_renderer_info");
            return { userAgent: navigator.userAgent, rootFont: getComputedStyle(document.documentElement).fontSize,
                gpu: gl ? { version: gl.getParameter(gl.VERSION), renderer: extension ? gl.getParameter(extension.UNMASKED_RENDERER_WEBGL) : gl.getParameter(gl.RENDERER) } : null };
        }) }, null, 2), contentType: "application/json",
    });
}

async function audit(page: Page) {
    const pageErrors: string[] = [], consoleErrors: string[] = [], failed: string[] = [];
    page.on("pageerror", error => pageErrors.push(error.message));
    page.on("console", message => { if (message.type() === "error") consoleErrors.push(message.text()); });
    page.on("requestfailed", request => failed.push(`${request.url()} ${request.failure()?.errorText}`));
    page.on("response", response => { if (response.status() >= 400) failed.push(`${response.status()} ${response.url()}`); });
    await page.addInitScript(() => {
        (window as any).__themeAcceptanceCsp = [];
        document.addEventListener("securitypolicyviolation", event => (window as any).__themeAcceptanceCsp.push({ directive: event.violatedDirective, blocked: event.blockedURI }));
    });
    // A shared page has already loaded; this installs the same observer for
    // edits on that document without reloading its GPU region.
    await page.evaluate(() => {
        if ((window as any).__themeAcceptanceCsp) return;
        (window as any).__themeAcceptanceCsp = [];
        document.addEventListener("securitypolicyviolation", event => (window as any).__themeAcceptanceCsp.push({ directive: event.violatedDirective, blocked: event.blockedURI }));
    });
    return { pageErrors, consoleErrors, failed, async healthy() {
        expect(pageErrors).toEqual([]); expect(consoleErrors).toEqual([]); expect(failed).toEqual([]);
        expect(await page.evaluate(() => (window as any).__themeAcceptanceCsp)).toEqual([]);
        const diagnostics=await page.evaluate(() => window.__theme_studio.diagnostics());
        // Makepad's Window sends its native title on creation. The embedded
        // host rejects that operation and preserves this page's title.
        const rejectedTitle="a region asked to set the document title; only the host page can";
        expect(diagnostics.entries.filter((entry:any)=>entry.severity==="error" && !(entry.kind==="UnsupportedCapability" && entry.detail===rejectedTitle)),JSON.stringify(diagnostics)).toEqual([]);
        expect(diagnostics.entries.filter((entry:any)=>entry.detail===rejectedTitle)).toHaveLength(1);
        expect(await page.title()).toBe("Rustify Theme Studio");
    } };
}

test.describe("V6 complete theme journey", () => {
    test.use({ fresh: true });
    test("reference preset → edit → overlays → both renderers → save/reload → export → independent import", async ({ page, browser }, info) => {
        test.setTimeout(300_000);
        const log = await audit(page);
        await page.setViewportSize({ width: 1440, height: 900 });
        await waitForReady(page);
        await page.getByTestId("preset-search").fill("Graphite");
        await select(page, "preset-select", "Built-in · Graphite");
        await page.getByTestId("renderer-compare").click();
        await drawn(page);
        const canvas = await page.getByTestId("theme-gpu").elementHandle();
        const regions = await page.evaluate(() => [...(window.__theme_studio as any).hooks.regions.keys()]);
        await color(page, "primary", "rgb(24 132 112 / .65)");
        await color(page, "shadow-color", "#123456");
        await page.getByTestId("editor-controls").click();
        await select(page, "choose-font-sans", "Noto Serif");
        for (const [token, value] of Object.entries({ "letter-spacing": "0.06em", radius: ".625rem", spacing: "5px", "layout-gap": "24px", "shadow-opacity": ".35", "shadow-blur": "12px", "shadow-spread": "1px", "shadow-offset-x": "3px", "shadow-offset-y": "8px" })) await field(page, token, value);
        const edited = await drawn(page);
        expect(edited.samples.find((sample: any) => sample.id === "font.sans.latin")).toMatchObject({ font: "Noto Serif", letter_spacing_em: .06 });
        expect(edited.samples.find((sample: any) => sample.id === "radius.lg").radius_px).toBe(10);
        await expect.poll(() => preview(page).getByTestId("preview-primary").evaluate(node => getComputedStyle(node).backgroundColor)).toBe("rgba(24, 132, 112, 0.65)");
        await preview(page).getByTestId("open-preview-menu").click();
        const menu = preview(page).getByTestId("preview-menu");
        await expect(menu).toBeVisible();
        await menu.getByRole("menuitem", { name: "Duplicate", exact: true }).focus();
        await page.keyboard.press("Enter");
        await expect(preview(page).getByTestId("local-card").getByRole("status")).toHaveText("duplicate");
        await preview(page).getByTestId("open-preview-dialog").click();
        const dialog = preview(page).getByTestId("preview-dialog").locator('[data-name="Dialog"]');
        await expect(dialog).toBeVisible();
        expect(await dialog.evaluate(node => getComputedStyle(node).boxShadow)).toContain("12px");
        await preview(page).getByTestId("dialog-confirm").click();
        await preview(page).getByTestId("preview-dialog-close").click();
        await page.getByTestId("theme-undo").click();
        expect((await drawn(page)).document).not.toEqual(edited.document);
        await page.getByTestId("theme-redo").click();
        expect((await drawn(page)).document).toEqual(edited.document);
        await evidence(page, info, "desktop-light", true);
        await page.getByTestId("mode-dark").click(); await drawn(page);
        await evidence(page, info, "desktop-dark", true);
        await page.getByTestId("mode-light").click(); await drawn(page);

        // The second scope has independent mode state. Switching it cannot
        // mutate the first preview's computed theme or the host control.
        await page.getByTestId("compare-scope").locator("summary").first().click();
        const styles = async () => Promise.all([preview(page), page.getByTestId("host-probe")].map(node => node.evaluate(element => {
            const s = getComputedStyle(element); return [s.backgroundColor, s.color, s.fontFamily, s.borderRadius];
        })));
        const first = await styles();
        await page.getByTestId("compare-light").click();
        await expect(page.getByTestId("studio-compare")).toHaveAttribute("data-theme", "light");
        expect(await styles()).toEqual(first);
        await page.getByTestId("compare-dark").click();
        await expect(page.getByTestId("studio-compare")).toHaveAttribute("data-theme", "dark");
        expect(await styles()).toEqual(first);
        expect(await page.evaluate(() => [...(window.__theme_studio as any).hooks.regions.keys()])).toEqual(regions);
        expect(await canvas!.evaluate(node => node.isConnected && node === document.querySelector('[data-testid="theme-gpu"]'))).toBe(true);

        await page.getByTestId("theme-save").click();
        await page.getByTestId("theme-name").fill("Acceptance 中文");
        await page.getByTestId("theme-name-confirm").click();
        await expect(page.getByTestId("storage-status")).toContainText("Saved on this device");
        const saved = (await snapshot(page)).document;
        expect((await snapshot(page)).modified).toBe(false);
        allowRegionStart(info); await page.reload();
        await expect(page.getByTestId("status")).toHaveAttribute("data-status", "ready", { timeout: REGION_START_MS });
        expect((await drawn(page)).document).toEqual(saved);
        await page.getByTestId("theme-export").click();
        const jsonDownload = page.waitForEvent("download"); await page.getByTestId("export-download").click();
        const json = await readFile((await (await jsonDownload).path())!, "utf8");
        expect(JSON.parse(json)).toEqual(saved);
        await select(page, "export-kind", "Tailwind v4 CSS");
        const css = await page.getByTestId("export-source").inputValue();
        const cssDownload = page.waitForEvent("download"); await page.getByTestId("export-download").click();
        expect(await readFile((await (await cssDownload).path())!, "utf8")).toBe(css);
        expect(css).toContain("@font-face"); expect(css).toContain("--color-primary");
        await page.keyboard.press("Escape");

        // An independent application has its own browser storage. Export
        // compilation/consumption outside this site is covered separately by
        // theme-export-runtime; here the user completes the JSON import UI.
        const context = await browser.newContext({ baseURL: info.project.use.baseURL, viewport: { width: 1440, height: 900 } });
        try {
            const imported = await context.newPage(); const importedLog = await audit(imported);
            allowRegionStart(info); await imported.goto("./");
            await expect(imported.getByTestId("status")).toHaveAttribute("data-status", "ready", { timeout: REGION_START_MS });
            expect((await snapshot(imported)).document).not.toEqual(saved);
            await imported.getByTestId("theme-import").click();
            await imported.getByTestId("import-source").fill(json); await imported.getByTestId("import-review").click();
            await expect(imported.getByTestId("import-preview")).toContainText("Acceptance 中文");
            await imported.getByTestId("import-apply").click();
            await imported.getByTestId("renderer-compare").click();
            expect((await drawn(imported)).document).toEqual(saved);
            await expect.poll(() => preview(imported).getByTestId("preview-primary").evaluate(node => getComputedStyle(node).backgroundColor)).toBe("rgba(24, 132, 112, 0.65)");
            await evidence(imported, info, "independent-json-import", true);
            await importedLog.healthy();
        } finally { await context.close(); }
        await log.healthy();
    });
});

for (const viewport of [{ name: "desktop", width: 1440, height: 900 }, { name: "tablet", width: 768, height: 1024 }, { name: "mobile", width: 390, height: 844 }]) {
    test(`V6 ${viewport.name}: bilingual modes, keyboard editing, readable editor and recoverable error states`, async ({ page }, info) => {
        const log = await audit(page);
        await page.setViewportSize({ width: viewport.width, height: viewport.height });
        await waitForReady(page);
        await page.getByTestId("locale-en").click();
        await page.getByTestId("preset-search").fill("no-theme-matches-v6");
        await expect(page.getByTestId("preset-search-empty")).toBeVisible();
        await page.getByTestId("preset-search-clear").click();
        await page.getByTestId("editor-colors").focus(); await page.keyboard.press("ArrowRight");
        await expect(page.getByTestId("editor-controls")).toBeFocused();
        const radius = page.getByTestId("edit-radius");
        await radius.focus(); await page.keyboard.press("ControlOrMeta+a"); await page.keyboard.type(".75rem"); await page.keyboard.press("Enter");
        expect((await snapshot(page)).document.styles.light.radius).toBe(".75rem");
        await page.getByTestId("locale-zh").click();
        await expect(page.locator("html")).toHaveAttribute("lang", "zh");
        await expect(page.getByTestId("editor-controls")).toHaveText("字体与其他");
        await page.getByTestId("mode-dark").focus(); await page.keyboard.press("Enter");
        if (viewport.name === "mobile") await page.getByTestId("mobile-preview").click();
        await page.getByTestId("renderer-compare").click(); await drawn(page);
        await expect(page.getByRole("heading", { name: "GPU 主题样本", exact: true })).toBeVisible();
        await expect(page.getByTestId("gpu-state")).toHaveText("已就绪");
        await expect(preview(page)).toHaveAttribute("data-theme", "dark");
        await expect(page.getByTestId("dom-preview").getByTestId("scene-cards")).toContainText("本地");
        await evidence(page, info, `${viewport.name}-zh-dark-preview`, viewport.name === "desktop");
        if (viewport.name === "mobile") await page.getByTestId("preview-back-edit").click();
        await page.getByTestId("locale-en").click(); await page.getByTestId("mode-light").click();
        await color(page, "background", "#ffffff"); await color(page, "foreground", "#ffffff");
        await page.locator(".studio-diagnostics > summary").click();
        await expect(page.getByTestId("contrast-foreground-on-background")).toContainText("1.000:1 · Below text thresholds");
        const shell = page.getByTestId("editor-shell");
        expect(await shell.evaluate(node => {
            const luminance = (color: string) => {
                const channels = color.match(/[\d.]+/g)!.slice(0, 3).map(value => {
                    const s = Number(value) / 255; return s <= .04045 ? s / 12.92 : ((s + .055) / 1.055) ** 2.4;
                });
                return channels[0] * .2126 + channels[1] * .7152 + channels[2] * .0722;
            };
            const foreground = luminance(getComputedStyle(node).color);
            const background = luminance(getComputedStyle(document.getElementById("studio-editor")!).backgroundColor);
            return (Math.max(foreground, background) + .05) / (Math.min(foreground, background) + .05);
        })).toBeGreaterThanOrEqual(4.5);
        await page.getByTestId("theme-import").click();
        const before = await snapshot(page);
        await page.getByTestId("import-source").fill('{"schema_version":999}'); await page.getByTestId("import-review").click();
        await expect(page.getByTestId("import-error")).toBeVisible();
        expect((await snapshot(page)).document).toEqual(before.document);
        await evidence(page, info, `${viewport.name}-import-failure`);
        await page.keyboard.press("Escape");
        await page.getByTestId("theme-undo").click(); await page.getByTestId("theme-undo").click();
        if (viewport.name === "mobile") await evidence(page, info, "mobile-en-light-editor");
        if (viewport.name === "mobile") await page.getByTestId("mobile-preview").click();
        await drawn(page); await evidence(page, info, `${viewport.name}-en-light-preview`);
        expect(await page.evaluate(() => document.documentElement.scrollWidth <= window.innerWidth + 1)).toBe(true);
        await log.healthy();
    });
}

test("V6 200% reflow/text zoom completes keyboard edits, preview and modal export", async ({ page }, info) => {
    const log = await audit(page);
    await page.setViewportSize({ width: 1440, height: 900 }); await waitForReady(page);
    const original = await page.evaluate(() => ({ style: document.documentElement.style.fontSize, px: parseFloat(getComputedStyle(document.documentElement).fontSize) }));
    try {
        // The same WCAG technique as p2-zoom: half the window width plus
        // doubled root text. This is not Chromium's UI zoom or pinch scaling.
        await page.setViewportSize({ width: 720, height: 900 });
        await page.evaluate(() => { document.documentElement.style.fontSize = "200%"; });
        await expect.poll(() => page.evaluate(() => parseFloat(getComputedStyle(document.documentElement).fontSize))).toBe(original.px * 2);
        await page.getByTestId("editor-colors").focus(); await page.keyboard.press("End");
        await expect(page.getByTestId("editor-controls")).toBeFocused();
        const field = page.getByTestId("edit-radius");
        await field.focus(); await page.keyboard.press("ControlOrMeta+a"); await page.keyboard.type(".5rem"); await page.keyboard.press("Enter");
        await page.getByTestId("renderer-compare").focus(); await page.keyboard.press("Enter");
        const state = await drawn(page);
        expect(state.samples.find((sample: any) => sample.id === "radius.lg").radius_px).toBe(original.px);
        await expect.poll(() => preview(page).getByTestId("radius-lg").evaluate(node => getComputedStyle(node).borderRadius)).toBe(`${original.px}px`);
        await page.getByTestId("theme-export").focus(); await page.keyboard.press("Enter");
        await expect(page.getByTestId("export-source")).toBeVisible();
        await page.getByTestId("export-select").focus(); await page.keyboard.press("Enter");
        expect(await page.getByTestId("export-source").evaluate((node: HTMLTextAreaElement) => node.selectionEnd - node.selectionStart)).toBe((await page.getByTestId("export-source").inputValue()).length);
        await evidence(page, info, "200-percent-export");
        await page.keyboard.press("Escape"); await drawn(page);
        await evidence(page, info, "200-percent-preview", true);
        await log.healthy();
    } finally {
        await page.evaluate(value => { document.documentElement.style.fontSize = value; }, original.style);
    }
});

test("Chinese composition commits once and never invokes theme history while composing", async ({ page }, info) => {
    await waitForReady(page);
    await page.getByTestId("editor-controls").click();
    const field = page.getByTestId("edit-font-sans"), before = await snapshot(page);
    await field.fill("");
    await field.evaluate(node => {
        (window as any).__themeComposition = [];
        for (const kind of ["compositionstart", "compositionupdate", "compositionend"]) {
            node.addEventListener(kind, event => (window as any).__themeComposition.push({ type: event.type, trusted: event.isTrusted, text: (event as CompositionEvent).data }));
        }
    });
    const cdp = await page.context().newCDPSession(page);
    await cdp.send("Input.imeSetComposition", { text: "中文字体", selectionStart: 4, selectionEnd: 4 });
    await field.press("Enter");
    expect((await snapshot(page)).document).toEqual(before.document);
    expect((await snapshot(page)).history).toEqual(before.history);
    await expect(field).toHaveValue("中文字体");
    await cdp.send("Input.insertText", { text: "中文字体" });
    await field.press("Enter");
    const committed = await snapshot(page);
    expect(committed.document.styles.light["font-sans"]).toBe("中文字体");
    expect(committed.document.styles.dark["font-sans"]).toBe("中文字体");
    expect(committed.history.undo).toBe(before.history.undo + 1);
    const events = await page.evaluate(() => (window as any).__themeComposition);
    expect(events[0].type).toBe("compositionstart");
    expect(events.at(-1).type).toBe("compositionend");
    expect(events.filter((event: any) => event.type === "compositionupdate").length).toBeGreaterThan(0);
    expect(events.filter((event: any) => event.type !== "compositionend").every((event: any) => event.trusted)).toBe(true);
    await page.getByTestId("theme-undo").click();
    expect((await snapshot(page)).document).toEqual(before.document);
    await page.getByTestId("theme-save").click();
    await page.getByTestId("theme-name").fill("");
    await cdp.send("Input.imeSetComposition", { text: "中文主题", selectionStart: 4, selectionEnd: 4 });
    await cdp.send("Input.insertText", { text: "中文主题" });
    await page.getByTestId("theme-name-confirm").click();
    expect((await snapshot(page)).document.name).toBe("中文主题");
    expect(await page.evaluate(() => JSON.parse(localStorage.getItem("rustify-ui.theme-studio.v1")!).draft.name)).toBe("中文主题");
    await info.attach("cdp-composition", { body: JSON.stringify(events), contentType: "application/json" });
    await cdp.detach();
});

test("actual Chrome 200% page zoom preserves editing, GPU comparison and export", async ({ playwright, headless }, info) => {
    // A temporary profile is required for Chrome's settings API. This changes
    // browser zoom, not CSS font size, device emulation or pinch scaling.
    const context = await playwright.chromium.launchPersistentContext("", {
        channel: "chromium", executablePath: process.env.RUSTIFY_CHROMIUM || undefined,
        headless, viewport: null, deviceScaleFactor: undefined,
        args: ["--window-size=1440,1000"],
    });
    try {
        const settings = await context.newPage();
        await settings.goto("chrome://settings/appearance");
        const page = await context.newPage();
        const log = await audit(page);
        await page.goto(info.project.use.baseURL!);
        await expect(page.getByTestId("status")).toHaveAttribute("data-status", "ready", { timeout: REGION_START_MS });
        await drawn(page);
        const dimensions = () => page.evaluate(() => ({ width: innerWidth, height: innerHeight, dpr: devicePixelRatio, root: getComputedStyle(document.documentElement).fontSize, visualScale: visualViewport!.scale }));
        const before = await dimensions();
        await settings.evaluate(() => new Promise<void>(resolve => (window as any).chrome.settingsPrivate.setDefaultZoom(2, resolve)));
        await expect.poll(async () => (await dimensions()).dpr).toBe(before.dpr * 2);
        const zoomed = await dimensions();
        expect(Math.abs(zoomed.width - before.width / 2)).toBeLessThanOrEqual(1);
        expect(zoomed.root).toBe(before.root);
        expect(zoomed.visualScale).toBe(1);
        await page.bringToFront();
        await color(page, "primary", "#8844aa");
        await page.getByTestId("renderer-compare").click();
        const state = await drawn(page);
        expect(state.document.styles.light.primary).toBe("#8844aa");
        expect(state.region_count).toBe(1);
        expect(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth + 1)).toBe(true);
        const cdp = await context.newCDPSession(page);
        const captureZoom = async (name: string) => {
            // Capture the browser view directly: a CSS-sized clip would be
            // scaled again by browser zoom and crop the compositor surface.
            const { data } = await cdp.send("Page.captureScreenshot", { format: "png", fromSurface: false });
            const path = info.outputPath(`${name}.png`);
            await writeFile(path, Buffer.from(data, "base64"));
            await info.attach(name, { path, contentType: "image/png" });
        };
        await page.evaluate(() => document.querySelector(".studio-previews")?.scrollTo(0, 0));
        await captureZoom("actual-200-percent-preview");
        await page.getByTestId("theme-export").focus(); await page.keyboard.press("Enter");
        await page.getByTestId("export-select").click();
        expect(await page.getByTestId("export-source").evaluate((node: HTMLTextAreaElement) => node.selectionEnd - node.selectionStart)).toBe((await page.getByTestId("export-source").inputValue()).length);
        await captureZoom("actual-200-percent-export");
        await page.keyboard.press("Escape");
        await expect(page.getByTestId("theme-export")).toBeFocused();
        await log.healthy();
        await info.attach("actual-browser-zoom", { body: JSON.stringify({ before, zoomed, method: "Chrome settingsPrivate.setDefaultZoom(2) in a temporary profile" }), contentType: "application/json" });
    } finally { await context.close(); }
});

test.describe("V6 isolated failures preserve the document", () => {
    test.use({ fresh: true });
    test("DOM edits remain usable during GPU loss and the restored region draws the latest revision", async ({ page }, info) => {
        const log = await audit(page); await waitForReady(page); await page.getByTestId("renderer-compare").click(); await drawn(page);
        const canvas = await page.getByTestId("theme-gpu").elementHandle();
        const regions = await page.evaluate(() => [...(window.__theme_studio as any).hooks.regions.keys()]);
        await page.getByTestId("locale-zh").click();
        await page.getByTestId("theme-gpu").evaluate((canvas: HTMLCanvasElement) => {
            const extension = canvas.getContext("webgl2")!.getExtension("WEBGL_lose_context");
            if (!extension) throw new Error("WEBGL_lose_context is required for this recovery check");
            (window as any).__themeAcceptanceLoss = extension; extension.loseContext();
        });
        await expect.poll(async () => (await snapshot(page)).gpu_state).toBe("Lost");
        await expect(page.getByTestId("gpu-state")).toHaveText("正在恢复 GPU…");
        await page.getByTestId("locale-en").click();
        await expect(page.getByTestId("gpu-state")).toHaveText("Restoring GPU…");
        await color(page, "primary", "#00a88a");
        await expect.poll(() => preview(page).getByTestId("preview-primary").evaluate(node => getComputedStyle(node).backgroundColor)).toBe("rgb(0, 168, 138)");
        const accepted = await snapshot(page); expect(accepted.drawn_revision).toBeLessThan(accepted.revision);
        allowRegionStart(info); await page.evaluate(() => { (window as any).__themeAcceptanceLoss.restoreContext(); delete (window as any).__themeAcceptanceLoss; });
        const restored = await drawn(page);
        expect(restored.document).toEqual(accepted.document); expect(restored.region_count).toBe(1);
        expect(restored.samples.find((sample: any) => sample.id === "color.primary").rgba).toEqual([0, 168 / 255, 138 / 255, 1]);
        const restoredRegions=await page.evaluate(() => [...(window.__theme_studio as any).hooks.regions.keys()]);
        expect(restoredRegions).toHaveLength(1);
        expect(restoredRegions).not.toEqual(regions);
        expect(await canvas!.evaluate(node => node.isConnected && node === document.querySelector('[data-testid="theme-gpu"]'))).toBe(true);
        expect(log.consoleErrors.filter(message => !/^\[rustify\] region \d+: WebGL context lost$/.test(message))).toEqual([]);
        expect(log.pageErrors).toEqual([]); expect(log.failed).toEqual([]);
        expect(await page.evaluate(() => (window as any).__themeAcceptanceCsp)).toEqual([]);
        await evidence(page, info, "gpu-recovered", true);
    });

    test("a missing packaged font is diagnosed, DOM edits persist, and a repaired deployment reload restores glyphs", async ({ page }, info) => {
        test.setTimeout(180_000);
        const log = await audit(page), font = "**/makepad_widgets/resources/IBMPlexSans-Text.ttf";
        await page.route(font, route => route.fulfill({ status: 404, contentType: "text/plain", body: "V6 missing packaged font" }));
        try {
            await waitForReady(page);
            await expect.poll(() => page.evaluate(() => window.__theme_studio.diagnostics().entries.filter((entry: any) => entry.kind === "AssetLoadFailed" && entry.asset?.endsWith("IBMPlexSans-Text.ttf")).length)).toBeGreaterThan(0);
            await page.getByTestId("renderer-compare").click();
            await expect(page.getByTestId("gpu-asset-warning")).toContainText("Reload to retry; your saved draft stays on this device.");
            await page.getByTestId("locale-zh").click();
            await expect(page.getByTestId("gpu-asset-warning")).toContainText("预览资源加载失败，字体样本可能使用回退字形。刷新页面可重试；已保存草稿仍保留在本机。");
            await page.getByTestId("locale-en").click();
            await color(page, "primary", "#8844aa");
            await expect.poll(() => preview(page).getByTestId("preview-primary").evaluate(node => getComputedStyle(node).backgroundColor)).toBe("rgb(136, 68, 170)");
            const draft = (await snapshot(page)).document;
            expect(await page.evaluate(() => JSON.parse(localStorage.getItem("rustify-ui.theme-studio.v1")!).draft)).toEqual(draft);
            await info.attach("missing-font-diagnostics", { body: JSON.stringify(await page.evaluate(() => window.__theme_studio.diagnostics()), null, 2), contentType: "application/json" });
            const initialConsoleErrors = log.consoleErrors.length, initialFailures = log.failed.length;
            await page.unroute(font); allowRegionStart(info); await page.reload();
            await expect(page.getByTestId("status")).toHaveAttribute("data-status", "ready", { timeout: REGION_START_MS });
            await page.getByTestId("renderer-compare").click(); const restored = await drawn(page);
            await expect(page.getByTestId("gpu-asset-warning")).toHaveCount(0);
            expect(restored.document).toEqual(draft);
            const faces = await page.evaluate(async () => {
                const faces = await document.fonts.load('15px "IBM Plex Sans"', "Recovered font");
                return faces.map(face => ({ family: face.family, status: face.status }));
            });
            expect(faces.map(face => ({ ...face, family: face.family.replace(/^["']|["']$/g, "") }))).toEqual([{ family: "IBM Plex Sans", status: "loaded" }]);
            expect(restored.samples.find((sample: any) => sample.id === "font.sans.latin").layout_size_px[0]).toBeGreaterThan(0);
            expect((await page.evaluate(() => window.__theme_studio.diagnostics())).entries.filter((entry: any) => entry.kind === "AssetLoadFailed")).toEqual([]);
            expect(log.pageErrors).toEqual([]);
            expect(log.failed.filter(message => !message.includes("IBMPlexSans-Text.ttf"))).toEqual([]);
            expect(log.consoleErrors.slice(initialConsoleErrors)).toEqual([]); expect(log.failed.slice(initialFailures)).toEqual([]);
            expect(await page.evaluate(() => (window as any).__themeAcceptanceCsp)).toEqual([]);
            await evidence(page, info, "font-recovered", true);
        } finally { await page.unroute(font); }
    });

    test("the existing host fatal boundary releases regions and restart restores the persisted draft, discarding uncommitted input", async ({ page }, info) => {
        test.setTimeout(180_000);
        const log = await audit(page); await waitForReady(page); await page.getByTestId("renderer-compare").click(); await drawn(page);
        await color(page, "primary", "#226688");
        await page.getByTestId("theme-save").click(); await page.getByTestId("theme-name").fill("Before fatal 中文"); await page.getByTestId("theme-name-confirm").click();
        await expect(page.getByTestId("storage-status")).toContainText("Saved on this device");
        const saved = (await snapshot(page)).document;
        const previous = await page.evaluate(() => window.__theme_studio.diagnostics());
        await page.getByTestId("color-value").fill("oklch(");
        expect((await snapshot(page)).document).toEqual(saved);
        // This enters the real loader recovery boundary. Theme Studio exposes
        // no deliberate wasm-trap export, so this does not claim to trigger one.
        await waitForQuiet(page);
        await page.evaluate(() => { (window as any).__themeAcceptanceDead = window.__theme_studio; (window.__theme_studio as any).hooks.runtime.enter_fatal(new Error("V6 host fatal boundary")); });
        await expect(page.getByTestId("status")).toHaveAttribute("data-status", "fatal");
        await expect(page.getByTestId("fatal-text")).toContainText("restart to restore the last saved draft");
        for (const id of ["studio-editor", "studio-preview", "studio-compare"]) {
            await expect(page.getByTestId(id)).not.toHaveAttribute("data-rustify-scope");
            await expect(page.getByTestId(id)).not.toHaveAttribute("data-rustify-theme-owner");
            await expect(page.getByTestId(id)).not.toHaveAttribute("data-rustify-theme-runtime");
        }
        expect(await page.evaluate(() => (window as any).__themeAcceptanceDead.hooks.regions.size)).toBe(0);
        expect(await page.evaluate(() => typeof window.__theme_studio)).toBe("undefined");
        allowRegionStart(info); await page.getByTestId("fatal-restart").click();
        await expect(page.getByTestId("status")).toHaveAttribute("data-status", "ready", { timeout: REGION_START_MS });
        const restored = await drawn(page), next = await page.evaluate(() => window.__theme_studio.diagnostics());
        expect(restored.document).toEqual(saved); expect(restored.modified).toBe(false); expect(restored.region_count).toBe(1);
        expect(next.runtime).toBe(previous.runtime + 1); expect(next.build).toBe(previous.build);
        await expect(page.getByTestId("color-value")).not.toHaveValue("oklch(");
        await color(page, "primary", "#338899"); expect((await drawn(page)).document.styles.light.primary).toBe("#338899");
        await evidence(page, info, "fatal-restarted", true); await log.healthy();
    });
});
