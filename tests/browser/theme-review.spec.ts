import { test, expect, waitForReady } from "./support";

async function snapshot(page: import("@playwright/test").Page) {
    await expect.poll(() => page.evaluate(() => {
        const s = window.__theme_studio.snapshot();
        return s.gpu_state === "Ready" && s.drawn_revision === s.revision && s.samples.length > 50;
    })).toBe(true);
    return page.evaluate(() => window.__theme_studio.snapshot());
}

test("tracking of optional ligatures matches same-font DOM widths", async ({ page }, info) => {
    await page.setViewportSize({ width: 1440, height: 3600 });
    await waitForReady(page);
    await page.evaluate(async () => {
        window.__theme_studio.edit("font-sans", "IBM Plex Sans");
        await document.fonts.load('15px "IBM Plex Sans"');
    });
    const widths = [];
    for (const spacing of [0, 0.2, 0.5]) {
        await page.evaluate(spacing => window.__theme_studio.edit("letter-spacing", `${spacing}em`), spacing);
        await expect.poll(async () => {
            const s = await snapshot(page);
            return s.samples.find((s: any) => s.id === "font.sans.latin").letter_spacing_em;
        }).toBe(spacing);
        const state = await snapshot(page);
        const font = state.samples.find((s: any) => s.id === "font.sans.latin");
        const dom = await page.evaluate(({ spacing, size }) => {
            const node = document.createElement("span");
            node.textContent = "Rustify · ffi fi · e\u0301";
            Object.assign(node.style, { fontFamily: '"IBM Plex Sans"', fontSize: `${size}px`, letterSpacing: `${spacing}em`, whiteSpace: "pre", position: "absolute" });
            document.body.append(node);
            const width = node.getBoundingClientRect().width;
            node.remove();
            return width;
        }, { spacing, size: font.font_size_px });
        widths.push({ spacing, gpu: font.layout_size_px[0], dom });
        expect(Math.abs(dom - font.layout_size_px[0]), JSON.stringify(widths)).toBeLessThan(0.1);
    }
    const lines = await page.evaluate(() => {
        const node = document.createElement("div");
        node.textContent = "fi fi";
        Object.assign(node.style, { fontFamily: '"IBM Plex Sans"', fontSize: "16px", letterSpacing: "0.5em", width: "35px", lineHeight: "20px", position: "absolute" });
        document.body.append(node);
        const lines = node.getBoundingClientRect().height / 20;
        node.remove();
        return lines;
    });
    expect(lines).toBe(2);
    await info.attach("tracking-widths", { body: JSON.stringify({ widths, lines }), contentType: "application/json" });
});

for (const width of [1440, 390]) {
    test(`large spacing keeps GPU dropdown and tabs scrollable at ${width}px`, async ({ page }, info) => {
        await page.setViewportSize({ width, height: 900 });
        await waitForReady(page);
        const errors: string[] = [];
        page.on("pageerror", error => errors.push(error.message));
        await page.getByTestId("editor-controls").click();
        await page.getByTestId("edit-spacing").fill("64px");
        await page.getByTestId("edit-spacing").press("Enter");
        if (width === 390) await page.getByTestId("mobile-preview").click();
        await page.getByTestId(width === 390 ? "renderer-gpu" : "renderer-compare").click();
        const initial = await snapshot(page);
        const board = page.getByTestId("gpu-board");
        const canvas = page.getByTestId("theme-gpu");
        expect(await canvas.evaluate(node => node.clientHeight)).toBeLessThanOrEqual(4096);
        expect(await board.evaluate(node => node.scrollHeight > node.clientHeight)).toBe(true);
        await board.focus();
        await page.keyboard.press("End");
        await expect.poll(async () => {
            const s = await snapshot(page);
            return s.samples.find((s: any) => s.id === "control.tabs").visible_rect[3];
        }).toBeGreaterThan(0);
        const click = async (id: string, fraction = 0.5) => {
            const state = await snapshot(page);
            const sample = state.samples.find((s: any) => s.id === id);
            const [x, y, w, h] = sample.visible_rect;
            expect(w).toBeGreaterThan(0); expect(h).toBeGreaterThan(0);
            const box = (await canvas.boundingBox())!;
            await page.locator(".studio-previews").evaluate((node, targetY) => { node.scrollTop += targetY - 450; }, box.y + y + h / 2);
            await canvas.click({ position: { x: x + w * fraction, y: y + h / 2 } });
        };
        await click("control.dropdown");
        await expect(page.getByTestId("gpu-dropdown-menu")).toBeVisible();
        await page.getByTestId("gpu-dropdown-menu").getByRole("menuitem", { name: "Usage", exact: true }).click();
        await expect.poll(() => page.evaluate(() => window.__theme_studio.snapshot().controls.index)).toBe(1);
        await click("control.tabs", 0.85);
        await expect.poll(() => page.evaluate(() => window.__theme_studio.snapshot().controls.index)).toBe(2);
        await page.getByTestId("inspector-toggle").click();
        await click("control.tabs");
        await expect(page.getByTestId("inspector-outline")).toContainText("muted");
        if (width === 390) {
            await expect(page.getByTestId("mobile-edit")).toHaveAttribute("aria-pressed", "true");
            await page.getByTestId("mobile-preview").click();
            await click("control.tabs");
            await page.getByTestId("mobile-preview").click();
        }
        await expect(page.getByTestId("inspector-outline")).toBeVisible();
        expect((await snapshot(page)).region_count).toBe(initial.region_count);
        expect(errors).toEqual([]);
        await page.screenshot({ path: info.outputPath(`spacing-${width}.png`) });
        await page.getByTestId("inspector-toggle").click();
        await page.evaluate(() => window.__theme_studio.edit("spacing", ".25rem"));
        await expect.poll(() => board.evaluate(node => node.scrollTop)).toBe(0);
        await expect.poll(async () => (await snapshot(page)).samples.find((s: any) => s.id === "control.tabs").visible_rect[3]).toBeGreaterThan(0);
    });
}
