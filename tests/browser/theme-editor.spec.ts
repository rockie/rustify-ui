import { test, expect, waitForReady } from "./support";
import type { ElementHandle, Locator, Page } from "@playwright/test";

const snapshot = (page: Page) => page.evaluate(() => window.__theme_studio.snapshot());
const preview = (page: Page) => page.getByTestId("studio-preview");
const sample = (state: any, id: string) => {
    const value = state.samples.find((entry: any) => entry.id === id);
    expect(value, `the completed GPU draw must report ${id}`).toBeDefined();
    return value;
};

async function select(page: Page, trigger: Locator, id: string, label: string) {
    await trigger.click();
    await page.getByTestId(`${id}-list`).getByRole("option", { name: label, exact: true }).click();
}

async function drawn(page: Page) {
    await page.getByTestId("theme-gpu").scrollIntoViewIfNeeded();
    await expect.poll(async () => {
        const state = await snapshot(page);
        return state.gpu_state === "Ready" && state.drawn_revision === state.revision && state.samples.length > 50;
    }).toBe(true);
    const state = await snapshot(page);
    await expect(preview(page)).toHaveAttribute("data-theme", state.mode);
    return state;
}

async function twoFrames(page: Page) {
    await page.evaluate(() => new Promise<void>(resolve => requestAnimationFrame(() => requestAnimationFrame(() => resolve()))));
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

async function identity(page: Page) {
    const canvas = await page.getByTestId("theme-gpu").elementHandle();
    if (!canvas) throw new Error("the mounted GPU canvas is missing");
    return { canvas, regions: await regionIds(page) };
}

async function regionIds(page: Page) {
    return page.evaluate(() => [...(window.__theme_studio as any).hooks.regions.keys()]);
}

async function sameRegion(page: Page, before: { canvas: ElementHandle<HTMLElement | SVGElement>; regions: any[] }) {
    expect((await snapshot(page)).region_count).toBe(1);
    expect(await regionIds(page)).toEqual(before.regions);
    expect(await before.canvas.evaluate(node => node.isConnected && node === document.querySelector('[data-testid="theme-gpu"]'))).toBe(true);
}

function unchangedCore(before: any, after: any) {
    expect(after.document).toEqual(before.document);
    expect(after.document_revision).toBe(before.document_revision);
    expect(after.history).toEqual(before.history);
    expect(after.resolve_count).toBe(before.resolve_count);
    expect(after.publish_count).toBe(before.publish_count);
    expect(after.revision).toBe(before.revision);
    expect(after.drawn_revision).toBe(before.drawn_revision);
}

test.describe("theme editing through the workbench", () => {
    test.beforeEach(async ({ page }) => {
        await waitForReady(page);
        await page.getByTestId("locale-en").click();
        await page.getByTestId("renderer-compare").click();
        await drawn(page);
        await page.getByTestId("editor-colors").click();
        await page.getByTestId("color-search").fill("");
    });

    test("an authored alpha color reaches DOM and a completed GPU draw, while no-op edits reuse the core snapshot", async ({ page }) => {
        const mounted = await identity(page);
        const initial = await snapshot(page);
        const authored = "rgb(200 40 80 / .5)";
        await color(page, "primary", authored);
        const light = await drawn(page);
        expect(light.document.styles.light.primary).toBe(authored);
        expect(light.document.styles.dark.primary).toBe(initial.document.styles.dark.primary);
        expect(light.document_revision).toBe(initial.document_revision + 1);
        expect(light.history.undo).toBe(initial.history.undo + 1);
        expect(light.resolve_count).toBe(initial.resolve_count + 2);
        expect(light.revision).toBeGreaterThan(initial.revision);
        expect(sample(light, "color.primary").rgba).toEqual([200 / 255, 40 / 255, 80 / 255, .5]);
        await expect.poll(async () => preview(page).getByTestId("preview-primary").evaluate(node => getComputedStyle(node).backgroundColor)).toBe("rgba(200, 40, 80, 0.5)");
        await page.getByTestId("color-value").fill(authored);
        await page.getByTestId("color-value").press("Enter");
        await page.getByTestId("mode-light").click();
        await twoFrames(page);
        unchangedCore(light, await snapshot(page));
        await page.getByTestId("mode-dark").click();
        const dark = await drawn(page);
        expect(dark.document).toEqual(light.document);
        expect(dark.history).toEqual(light.history);
        expect(dark.document_revision).toBe(light.document_revision);
        expect(dark.resolve_count).toBe(light.resolve_count);
        expect(sample(dark, "color.primary").rgba).not.toEqual(sample(light, "color.primary").rgba);
        await page.getByTestId("mode-light").click();
        const restored = await drawn(page);
        expect(sample(restored, "color.primary").rgba).toEqual(sample(light, "color.primary").rgba);
        await page.getByTestId("color-reset-primary").click();
        const reset = await drawn(page);
        expect(reset.document.styles.light.primary).toBe(initial.document.styles.light.primary);
        expect(reset.document.styles.dark.primary).toBe(initial.document.styles.dark.primary);
        await expect(page.getByTestId("color-reset-primary")).toBeDisabled();
        await page.getByTestId("theme-undo").click();
        expect((await drawn(page)).document).toEqual(light.document);
        await sameRegion(page, mounted);
    });

    test("notation and search preserve authors, and an incomplete color never replaces the effective theme", async ({ page }) => {
        const authored = "oklch(0.72 0.22 35 / .6)";
        await color(page, "primary", authored);
        const accepted = await drawn(page);
        for (const notation of ["HEX", "RGB", "HSL", "OKLCH"]) {
            await select(page, page.getByTestId("color-format"), "color-format", notation);
            await twoFrames(page);
            unchangedCore(accepted, await snapshot(page));
        }
        await page.getByTestId("color-value").fill("oklch(");
        await expect(page.getByTestId("color-value")).toHaveAttribute("aria-invalid", "true");
        unchangedCore(accepted, await snapshot(page));
        await page.getByTestId("color-value").press("Enter");
        await expect(page.locator("#color-value-help")).toContainText("Invalid color");
        const refused = await snapshot(page);
        expect(refused.document).toEqual(accepted.document);
        expect(refused.history).toEqual(accepted.history);
        expect(refused.document_revision).toBe(accepted.document_revision);
        expect(refused.revision).toBe(accepted.revision);
        await page.getByTestId("color-value").press("Escape");
        await expect(page.getByTestId("color-value")).not.toHaveAttribute("aria-invalid", "true");
        await page.getByTestId("color-search").fill("no-matching-semantic-token");
        await expect(page.getByTestId("color-search-empty")).toBeVisible();
        await page.getByTestId("color-search-clear").click();
        await expect(page.getByTestId("color-token-primary")).toBeVisible();
        expect((await snapshot(page)).document.styles.light.primary).toBe(authored);
    });

    test("all fourteen common fields synchronize both modes and field reset restores the author value", async ({ page }) => {
        await select(page, page.getByTestId("preset-select"), "preset-select", "Built-in · Rustify Light");
        await drawn(page);
        const mounted = await identity(page);
        const initial = await snapshot(page);
        await page.getByTestId("editor-controls").click();
        await expect(page.getByTestId("common-fields-note")).toContainText("14 shared fields");
        for (const [token, family] of [["font-sans", "Noto Serif"], ["font-serif", "IBM Plex Sans"], ["font-mono", "JetBrains Mono"]]) {
            await select(page, page.getByTestId(`choose-${token}`), `choose-${token}`, family);
        }
        const values = {
            "font-size": "18px", "letter-spacing": "0.08em", "radius": "0.625rem", "spacing": "6px", "layout-gap": "28px",
            "shadow-opacity": "0.45", "shadow-blur": "11px", "shadow-spread": "-1px", "shadow-offset-x": "-2px", "shadow-offset-y": "7px",
        };
        for (const [token, value] of Object.entries(values)) await field(page, token, value);
        await page.getByTestId("edit-reduce-motion").click();
        const state = await drawn(page);
        for (const token of ["font-sans", "font-serif", "font-mono", "reduce-motion", ...Object.keys(values)]) {
            expect(state.document.styles.light[token], token).toBe(state.document.styles.dark[token]);
        }
        expect(state.document.styles.light["font-sans"]).toBe('"Noto Serif", sans-serif');
        expect(state.document.styles.light["reduce-motion"]).toBe("true");
        expect(sample(state, "font.sans.latin").font).toBe("Noto Serif");
        expect(sample(state, "font.sans.latin").letter_spacing_em).toBe(.08);
        expect(sample(state, "radius.lg").radius_px).toBe(10);
        await expect.poll(async () => preview(page).getByTestId("preview-surface").evaluate(node => getComputedStyle(node).fontFamily)).toContain('"Noto Serif"');
        await expect.poll(async () => preview(page).getByTestId("preview-surface").evaluate(node => getComputedStyle(node).letterSpacing)).toBe("1.44px");
        await page.getByTestId("reset-radius").click();
        const reset = await drawn(page);
        expect(reset.document.styles.light.radius).toBe(initial.document.styles.light.radius);
        expect(reset.document.styles.dark.radius).toBe(initial.document.styles.light.radius);
        await expect(page.getByTestId("reset-radius")).toBeDisabled();
        await sameRegion(page, mounted);
    });

    test("incomplete shared parameter drafts stay local, and keyboard tabs expose the correct panel", async ({ page }) => {
        await page.getByTestId("editor-colors").focus();
        await page.keyboard.press("ArrowRight");
        await expect(page.getByTestId("editor-controls")).toBeFocused();
        await expect(page.getByTestId("editor-controls")).toHaveAttribute("aria-selected", "true");
        await expect(page.getByTestId("theme-controls")).toBeVisible();
        const origin = await snapshot(page);
        for (const [token, draft] of [["radius", "-"], ["letter-spacing", "0."], ["font-sans", '"Unclosed author font']]) {
            const input = page.getByTestId(`edit-${token}`);
            await input.fill(draft);
            await expect(input).toHaveAttribute("aria-invalid", "true");
            unchangedCore(origin, await snapshot(page));
            await input.press("Enter");
            unchangedCore(origin, await snapshot(page));
            await expect(page.getByTestId(`error-${token}`)).not.toBeEmpty();
            await input.press("Escape");
        }
        await page.getByTestId("editor-controls").focus();
        await page.keyboard.press("Home");
        await expect(page.getByTestId("editor-colors")).toBeFocused();
        await expect(page.getByTestId("color-editor")).toBeVisible();
        await expect(page.getByTestId("theme-controls")).toBeHidden();
        unchangedCore(origin, await snapshot(page));
    });

    test("preset search and favorites preserve mode, and shared author differences survive until an explicit edit", async ({ page }) => {
        await page.getByTestId("mode-dark").click();
        await drawn(page);
        await page.getByTestId("preset-search").fill("gRApHiTE");
        await select(page, page.getByTestId("preset-select"), "preset-select", "Built-in · Graphite");
        const graphite = await drawn(page);
        expect(graphite.mode).toBe("dark");
        expect(graphite.document.id).toBe("graphite");
        expect(graphite.document.styles.light["font-sans"]).toBe("Montserrat, sans-serif");
        expect(graphite.document.styles.dark["font-sans"]).toBe("Inter, sans-serif");
        await expect(page.getByTestId("theme-modified")).toHaveText("Matches preset");
        await page.getByTestId("preset-favorite").click();
        await expect(page.getByTestId("preset-favorite")).toHaveAttribute("aria-pressed", "true");
        await page.getByTestId("preset-favorites").click();
        await page.getByTestId("preset-select").click();
        await expect(page.getByTestId("preset-select-list").getByRole("option")).toHaveCount(1);
        await page.keyboard.press("Escape");
        await page.getByTestId("preset-favorite").click();
        await expect(page.getByTestId("preset-search-empty")).toBeVisible();
        await expect(page.getByTestId("preset-select")).toBeDisabled();
        await page.getByTestId("preset-search-clear").click();
        await expect(page.getByTestId("preset-search")).toHaveValue("");
        await expect(page.getByTestId("preset-select")).toBeEnabled();
        await page.getByTestId("editor-controls").click();
        await expect(page.getByTestId("common-differences")).toContainText("font-sans");
        await field(page, "radius", "9px");
        let state = await drawn(page);
        expect(state.document.styles.light["font-sans"]).toBe(graphite.document.styles.light["font-sans"]);
        expect(state.document.styles.dark["font-sans"]).toBe(graphite.document.styles.dark["font-sans"]);
        await select(page, page.getByTestId("choose-font-sans"), "choose-font-sans", "Noto Sans");
        state = await drawn(page);
        expect(state.document.styles.light["font-sans"]).toBe('"Noto Sans", sans-serif');
        expect(state.document.styles.dark["font-sans"]).toBe(state.document.styles.light["font-sans"]);
        await expect(page.getByTestId("common-differences")).toHaveCount(0);
        await page.getByTestId("theme-reset").click();
        state = await drawn(page);
        expect(state.document).toEqual(graphite.document);
        expect(state.mode).toBe("dark");
        await expect(page.getByTestId("theme-modified")).toHaveText("Matches preset");
        await page.getByTestId("preset-search").fill("definitely-unknown-preset");
        await expect(page.getByTestId("preset-search-empty")).toBeVisible();
        await page.getByTestId("preset-search-clear").click();
    });

    test("undo and redo restore documents without switching mode, and a new edit clears the redo branch", async ({ page }) => {
        await select(page, page.getByTestId("preset-select"), "preset-select", "Built-in · Rustify Light");
        await drawn(page);
        const mounted = await identity(page);
        const initial = await snapshot(page);
        await color(page, "primary", "#f24b78");
        const first = await drawn(page);
        await page.getByTestId("editor-controls").click();
        await field(page, "spacing", "6px");
        const second = await drawn(page);
        await page.getByTestId("mode-dark").click();
        await drawn(page);
        await page.getByTestId("theme-undo").click();
        let state = await drawn(page);
        expect(state.document).toEqual(first.document);
        expect(state.mode).toBe("dark");
        await page.getByTestId("theme-redo").click();
        state = await drawn(page);
        expect(state.document).toEqual(second.document);
        expect(state.mode).toBe("dark");
        await page.getByTestId("theme-undo").click();
        await drawn(page);
        await color(page, "primary", "#005e8a");
        const branch = await drawn(page);
        expect(branch.history.redo).toBe(0);
        await expect(page.getByTestId("theme-redo")).toBeDisabled();
        await page.getByTestId("theme-reset").click();
        state = await drawn(page);
        expect(state.document).toEqual(initial.document);
        expect(state.mode).toBe("dark");
        await expect(page.getByTestId("theme-modified")).toHaveText("Matches preset");
        await page.getByTestId("theme-undo").click();
        expect((await drawn(page)).document).toEqual(branch.document);
        await sameRegion(page, mounted);
    });

    test("a captured color drag and repeated keyboard updates each create one undo step, and Escape cancels", async ({ page }) => {
        const initial = await snapshot(page);
        const plane = page.getByTestId("color-plane");
        await plane.scrollIntoViewIfNeeded();
        const box = await plane.boundingBox();
        if (!box) throw new Error("color plane has no visible bounds");
        await page.mouse.move(box.x + box.width * .25, box.y + box.height * .3);
        await page.mouse.down();
        for (const fraction of [.35, .45, .55]) await page.mouse.move(box.x + box.width * fraction, box.y + box.height * .4);
        let pending = await snapshot(page);
        expect(pending.history.gesture).toBe(true);
        expect(pending.history.undo).toBe(initial.history.undo);
        await page.mouse.up();
        const dragged = await drawn(page);
        expect(dragged.history.gesture).toBe(false);
        expect(dragged.history.undo).toBe(initial.history.undo + 1);
        expect(dragged.document.styles.dark.primary).toBe(initial.document.styles.dark.primary);
        await page.getByTestId("theme-undo").click();
        expect((await drawn(page)).document).toEqual(initial.document);
        await plane.focus();
        for (let i = 0; i < 3; i++) await page.keyboard.down("ArrowLeft");
        pending = await snapshot(page);
        expect(pending.history.gesture).toBe(true);
        expect(pending.history.undo).toBe(initial.history.undo);
        await page.keyboard.up("ArrowLeft");
        const keyed = await drawn(page);
        expect(keyed.history.undo).toBe(initial.history.undo + 1);
        await page.getByTestId("theme-undo").click();
        const origin = await drawn(page);
        await plane.focus();
        await page.keyboard.down("ArrowLeft");
        await page.keyboard.press("Escape");
        await page.keyboard.up("ArrowLeft");
        const cancelled = await drawn(page);
        expect(cancelled.document).toEqual(origin.document);
        expect(cancelled.history).toEqual(origin.history);
    });

    test("batch HSL always uses its origin, commits once, and cancellation preserves the redo branch", async ({ page }) => {
        const mounted = await identity(page);
        await page.getByTestId("editor-controls").click();
        await select(page, page.getByTestId("hsl-scope"), "hsl-scope", "Both modes");
        const origin = await snapshot(page);
        await page.getByTestId("hsl-hue").fill("45");
        await page.getByTestId("hsl-saturation").fill("1.25");
        await page.getByTestId("hsl-lightness").fill("0.8");
        const shifted = await snapshot(page);
        expect(shifted.history.gesture).toBe(true);
        expect(shifted.history.undo).toBe(origin.history.undo);
        expect(shifted.document.styles.light).not.toEqual(origin.document.styles.light);
        expect(shifted.document.styles.dark).not.toEqual(origin.document.styles.dark);
        await page.getByTestId("hsl-hue").fill("90");
        await page.getByTestId("hsl-hue").fill("45");
        expect((await snapshot(page)).document).toEqual(shifted.document);
        const beforeCommit = await snapshot(page);
        await page.getByTestId("hsl-apply").click();
        const committed = await drawn(page);
        expect(committed.document).toEqual(shifted.document);
        expect(committed.document_revision).toBe(beforeCommit.document_revision);
        expect(committed.resolve_count).toBe(beforeCommit.resolve_count);
        expect(committed.history.undo).toBe(origin.history.undo + 1);
        expect(committed.history.gesture).toBe(false);
        await expect(page.getByTestId("hsl-hue")).toHaveValue("0");
        await expect(page.getByTestId("hsl-saturation")).toHaveValue("1");
        await expect(page.getByTestId("hsl-lightness")).toHaveValue("1");
        await page.getByTestId("theme-undo").click();
        const undo = await drawn(page);
        expect(undo.document).toEqual(origin.document);
        await page.getByTestId("hsl-hue").fill("-45");
        await page.getByTestId("hsl-hue").press("Escape");
        const cancelled = await drawn(page);
        expect(cancelled.document).toEqual(undo.document);
        expect(cancelled.history).toEqual(undo.history);
        await page.getByTestId("theme-redo").click();
        expect((await drawn(page)).document).toEqual(committed.document);
        await sameRegion(page, mounted);
    });

    test("HSL invalid and identity drafts do not resolve, and current-mode adjustment keeps gray colors finite", async ({ page }) => {
        await page.getByTestId("mode-dark").click();
        await color(page, "primary", "#808080");
        await drawn(page);
        await page.getByTestId("editor-controls").click();
        await select(page, page.getByTestId("hsl-scope"), "hsl-scope", "Current mode");
        const origin = await snapshot(page);
        for (const value of ["-", "181", "NaN"]) {
            await page.getByTestId("hsl-hue").fill(value);
            await expect(page.getByTestId("hsl-hue")).toHaveAttribute("aria-invalid", "true");
            unchangedCore(origin, await snapshot(page));
        }
        await page.getByTestId("hsl-hue").fill("0");
        await page.getByTestId("hsl-reset").click();
        await twoFrames(page);
        unchangedCore(origin, await snapshot(page));
        await page.getByTestId("hsl-hue").fill("180");
        await page.getByTestId("hsl-saturation").fill("0");
        await page.getByTestId("hsl-lightness").fill("0.2");
        await page.getByTestId("hsl-apply").click();
        const adjusted = await drawn(page);
        expect(adjusted.document.styles.light).toEqual(origin.document.styles.light);
        expect(adjusted.document.styles.dark).not.toEqual(origin.document.styles.dark);
        for (const entry of adjusted.samples.filter((entry: any) => entry.id.startsWith("color."))) {
            expect(entry.rgba.every((channel: number) => Number.isFinite(channel))).toBe(true);
        }
        const gray = sample(adjusted, "color.primary").rgba;
        expect(gray[0]).toBeCloseTo(gray[1], 8);
        expect(gray[1]).toBeCloseTo(gray[2], 8);
    });

    test("leaving the batch panel commits once and a reset-to-identity gesture preserves author strings", async ({ page }) => {
        await page.getByTestId("editor-controls").click();
        await select(page, page.getByTestId("hsl-scope"), "hsl-scope", "Both modes");
        const origin = await snapshot(page);
        await page.getByTestId("hsl-hue").fill("30");
        await page.getByTestId("editor-colors").click();
        const committed = await drawn(page);
        expect(committed.history.undo).toBe(origin.history.undo + 1);
        expect(committed.history.gesture).toBe(false);
        await page.getByTestId("editor-controls").click();
        await expect(page.getByTestId("hsl-hue")).toHaveValue("0");
        await page.getByTestId("theme-undo").click();
        const undo = await drawn(page);
        await page.getByTestId("hsl-saturation").fill("1.4");
        await page.getByTestId("hsl-reset").click();
        expect((await snapshot(page)).document).toEqual(undo.document);
        await page.getByTestId("hsl-apply").click();
        const noChange = await drawn(page);
        expect(noChange.document).toEqual(undo.document);
        expect(noChange.history).toEqual(undo.history);
    });

    test("diagnostics preserve wide-gamut and unknown font authors and report unknown translucent backing", async ({ page }) => {
        const shell = await page.getByTestId("editor-shell").evaluate(node => ({ color: getComputedStyle(node).color, background: getComputedStyle(node).backgroundColor }));
        await color(page, "primary", "oklch(0.7 0.4 30)");
        await color(page, "background", "#00000080");
        await color(page, "foreground", "rgb(255 255 255 / .5)");
        await page.getByTestId("editor-controls").click();
        const stack = '"Unbundled Local Font", sans-serif';
        await field(page, "font-sans", stack);
        const state = await drawn(page);
        expect(state.document.styles.light.primary).toBe("oklch(0.7 0.4 30)");
        expect(state.document.styles.light["font-sans"]).toBe(stack);
        expect(state.document.styles.dark["font-sans"]).toBe(stack);
        expect(sample(state, "font.sans.latin").font).toBe("IBM Plex Sans");
        await page.locator(".studio-diagnostics > summary").click();
        await expect(page.getByTestId("gamut-diagnostics")).toContainText("primary");
        await expect(page.getByTestId("diagnostic-font-sans")).toContainText("Unbundled Local Font");
        await expect(page.getByTestId("diagnostic-font-sans")).toContainText("bundled fallback");
        await expect(page.getByTestId("contrast-foreground-on-background")).toContainText("Unknown · actual backing required");
        const after = await page.getByTestId("editor-shell").evaluate(node => ({ color: getComputedStyle(node).color, background: getComputedStyle(node).backgroundColor }));
        expect(after).toEqual(shell);
    });

    test("Inspector suppresses sample actions and chooses actual DOM, GPU and keyboard tokens", async ({ page }) => {
        const mounted = await identity(page);
        const origin = await snapshot(page);
        await page.getByTestId("inspector-toggle").click();
        await expect(page.getByTestId("inspector-panel")).toBeVisible();
        const cards = page.getByTestId("dom-preview").getByTestId("scene-cards");
        const card = cards.getByTestId("cards-form");
        await card.scrollIntoViewIfNeeded();
        await card.hover({ position: { x: 8, y: 8 } });
        await expect(page.getByTestId("inspector-outline")).toBeVisible();
        await expect(page.getByTestId("inspector-outline")).toContainText("card-foreground");
        await expect.poll(async () => {
            const cardBounds = await card.boundingBox(), outline = await page.getByTestId("inspector-outline").boundingBox();
            return !!cardBounds && !!outline && outline.y >= Math.max(0, cardBounds.y) - 2;
        }).toBe(true);
        const cardBounds = await card.boundingBox();
        const outline = await page.getByTestId("inspector-outline").boundingBox();
        if (!cardBounds || !outline) throw new Error("the inspected DOM sample has no visible bounds");
        expect(outline.width).toBeGreaterThan(0);
        expect(outline.height).toBeGreaterThan(0);
        expect(outline.width).toBeLessThanOrEqual(cardBounds.width + 2);
        expect(outline.height).toBeLessThanOrEqual(cardBounds.height + 2);
        expect(outline.x).toBeGreaterThanOrEqual(Math.max(0, cardBounds.x) - 2);
        expect(outline.y).toBeGreaterThanOrEqual(Math.max(0, cardBounds.y) - 2);
        const viewport = page.viewportSize();
        if (!viewport) throw new Error("the inspector check requires a fixed viewport");
        expect(outline.x + outline.width).toBeLessThanOrEqual(viewport.width + 2);
        expect(outline.y + outline.height).toBeLessThanOrEqual(viewport.height + 2);
        await cards.getByTestId("cards-submit").hover();
        await expect(page.getByTestId("inspector-outline")).toBeVisible();
        await expect(page.getByTestId("inspector-outline")).toContainText("primary");
        await cards.getByTestId("cards-submit").click();
        await expect(cards.getByTestId("cards-error")).toBeEmpty();
        await expect(page.getByTestId("color-token-primary")).toBeFocused();
        await page.getByTestId("theme-gpu").scrollIntoViewIfNeeded();
        const state = await drawn(page);
        const picked = sample(state, "color.chart-3");
        expect(picked.visible_rect).toHaveLength(4);
        const [x, y, width, height] = picked.visible_rect;
        const scale = await page.getByTestId("theme-gpu").evaluate(node => ({ width: node.getBoundingClientRect().width / node.clientWidth, height: node.getBoundingClientRect().height / node.clientHeight }));
        await page.getByTestId("theme-gpu").click({ position: { x: (x + width / 2) * scale.width, y: (y + height / 2) * scale.height } });
        await expect(page.getByTestId("color-token-chart-3")).toHaveAttribute("aria-pressed", "true");
        await expect(page.getByTestId("color-token-chart-3")).toBeFocused();
        await expect(page.getByTestId("inspector-outline")).toContainText("chart-3");
        await page.getByTestId("inspector-panel").locator("details > summary").click();
        await page.getByTestId("inspect-token-font-sans").focus();
        await page.keyboard.press("Enter");
        await expect(page.getByTestId("editor-controls")).toHaveAttribute("aria-selected", "true");
        await expect(page.getByTestId("edit-font-sans")).toBeFocused();
        await page.keyboard.press("Escape");
        await expect(page.getByTestId("inspector-panel")).toBeHidden();
        await expect(page.getByTestId("inspector-outline")).toBeHidden();
        unchangedCore(origin, await snapshot(page));
        await sameRegion(page, mounted);
    });

    test("editor resizing works with keyboard bounds and pointer capture without rebuilding the GPU region", async ({ page }) => {
        const mounted = await identity(page);
        const origin = await snapshot(page);
        const separator = page.getByTestId("editor-separator");
        await separator.focus();
        await page.keyboard.press("Home");
        await expect(separator).toHaveAttribute("aria-valuenow", "260");
        await expect.poll(async () => page.getByTestId("studio-editor").evaluate(node => node.getBoundingClientRect().width)).toBe(260);
        await page.keyboard.press("ArrowRight");
        await expect(separator).toHaveAttribute("aria-valuenow", "270");
        await page.keyboard.press("End");
        await expect(separator).toHaveAttribute("aria-valuenow", "520");
        await page.keyboard.press("ArrowRight");
        await expect(separator).toHaveAttribute("aria-valuenow", "520");
        const box = await separator.boundingBox();
        if (!box) throw new Error("editor separator has no visible bounds");
        await page.mouse.move(box.x + box.width / 2, box.y + Math.min(box.height / 2, 200));
        await page.mouse.down();
        await page.mouse.move(box.x + box.width / 2 - 120, box.y + Math.min(box.height / 2, 200), { steps: 5 });
        await page.mouse.up();
        await expect(separator).toHaveAttribute("aria-valuenow", "400");
        await expect.poll(async () => page.getByTestId("studio-editor").evaluate(node => node.getBoundingClientRect().width)).toBe(400);
        await drawn(page);
        unchangedCore(origin, await snapshot(page));
        await sameRegion(page, mounted);
    });
});
