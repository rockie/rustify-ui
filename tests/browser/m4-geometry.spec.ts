import { expect, Page } from "@playwright/test";
import { PNG } from "pngjs";
import { Anchor, anchorRect, geometry, mountGeometry, test } from "./support";
import { EVIDENCE, pick } from "../tier";

/// The three CSS viewports and three zoom levels R09 names. Zoom is the device
/// scale factor: what browser zoom actually changes for a page is how many
/// device pixels a CSS pixel is worth, not the CSS box.
const VIEWPORTS = [
    { width: 1024, height: 768 },
    { width: 1440, height: 900 },
    { width: 1920, height: 1080 },
];
const ZOOMS = [1, 1.25, 2];
const SIZES = VIEWPORTS.flatMap((viewport) => ZOOMS.map((zoom) => ({ viewport, zoom })));

/// The two of those nine that stand for the rest on every change: the
/// smallest viewport at 100%, and a fractional zoom, where a CSS pixel is not
/// a whole number of device pixels and every rounding shows.
const REPRESENTATIVE = SIZES.filter(
    ({ viewport, zoom }) => (viewport.width === 1024 && zoom === 1) || (viewport.width === 1440 && zoom === 1.25)
);

/// Scrolls the anchor into the middle of the inner clip box, and the clip box
/// into the middle of the page, then reports where the region's own origin
/// ended up.
async function showAnchor(page: Page, anchor: Anchor) {
    return page.evaluate((anchor) => {
        const inner = document.querySelector('[data-testid="geometry-inner"]') as HTMLElement;
        const canvas = document.querySelector('[data-testid="geometry-gpu"]') as HTMLElement;
        const clamp = (value: number, max: number) => Math.max(0, Math.min(value, max));
        inner.scrollLeft = clamp(
            anchor.x + anchor.width / 2 - inner.clientWidth / 2,
            inner.scrollWidth - inner.clientWidth
        );
        inner.scrollTop = clamp(
            anchor.y + anchor.height / 2 - inner.clientHeight / 2,
            inner.scrollHeight - inner.clientHeight
        );
        inner.scrollIntoView({ block: "center", inline: "center" });
        const box = canvas.getBoundingClientRect();
        return { left: box.left, top: box.top };
    }, anchor);
}

/// Centre of mass of the marker pixels inside a window centred on `point`, in
/// CSS pixels relative to that point.
async function markerOffset(page: Page, point: { x: number; y: number }, window = 12) {
    const shot = await page.screenshot({
        scale: "css",
        clip: { x: point.x - window / 2, y: point.y - window / 2, width: window, height: window },
    });
    const png = PNG.sync.read(shot);
    let sumX = 0;
    let sumY = 0;
    let lit = 0;
    for (let y = 0; y < png.height; y++) {
        for (let x = 0; x < png.width; x++) {
            const i = (y * png.width + x) * 4;
            const luma = 0.299 * png.data[i] + 0.587 * png.data[i + 1] + 0.114 * png.data[i + 2];
            if (luma > 180) {
                // Pixel centres, so a marker centred in the window has its
                // centroid at the window's centre.
                sumX += x + 0.5;
                sumY += y + 0.5;
                lit += 1;
            }
        }
    }
    if (lit === 0) {
        return null;
    }
    const scale = window / png.width;
    return { dx: (sumX / lit) * scale - window / 2, dy: (sumY / lit) * scale - window / 2 };
}

/// Waits until the anchors the region reports describe the size its canvas has
/// now, so a check never compares against the layout of a previous size.
async function currentAnchors(page: Page) {
    await expect
        .poll(async () => {
            const anchors = (await geometry(page)).anchors;
            if (anchors.length !== 20) {
                return false;
            }
            const box = await page.evaluate(() => {
                const canvas = document.querySelector('[data-testid="geometry-gpu"]') as HTMLElement;
                return { width: canvas.clientWidth, height: canvas.clientHeight };
            });
            const right = Math.max(...anchors.map((a) => a.x + a.width));
            const bottom = Math.max(...anchors.map((a) => a.y + a.height));
            return Math.abs(right - box.width) <= 4 && Math.abs(bottom - box.height) <= 4;
        }, { timeout: 10_000 })
        .toBe(true);
    return (await geometry(page)).anchors;
}

async function checkAnchors(page: Page, label: string) {
    const anchors = await currentAnchors(page);
    expect(anchors.length).toBe(20);
    for (const anchor of anchors) {
        const origin = await showAnchor(page, anchor);
        const centre = {
            x: origin.left + anchor.x + anchor.width / 2,
            y: origin.top + anchor.y + anchor.height / 2,
        };

        // Where the region says the anchor is, is where the picture puts it.
        const offset = await markerOffset(page, centre);
        expect(offset, `${label} anchor ${anchor.id}: no marker on screen`).not.toBeNull();
        expect(Math.abs(offset!.dx), `${label} anchor ${anchor.id} display x`).toBeLessThanOrEqual(1);
        expect(Math.abs(offset!.dy), `${label} anchor ${anchor.id} display y`).toBeLessThanOrEqual(1);

        // And where a real click at that point lands.
        const before = (await geometry(page)).hits;
        await page.mouse.click(centre.x, centre.y);
        await expect
            .poll(async () => (await geometry(page)).hits, { timeout: 5_000 })
            .toBe(before + 1);
        const hit = (await geometry(page)).last_hit!;
        expect(hit.anchor, `${label} anchor ${anchor.id} hit`).toBe(anchor.id);
        expect(
            Math.abs(hit.x - (anchor.x + anchor.width / 2)),
            `${label} anchor ${anchor.id} hit x`
        ).toBeLessThanOrEqual(1);
        expect(
            Math.abs(hit.y - (anchor.y + anchor.height / 2)),
            `${label} anchor ${anchor.id} hit y`
        ).toBeLessThanOrEqual(1);
    }
}

/// R10: a DOM menu opened from a GPU anchor lands on that anchor, in every
/// size and zoom R09 names.
async function checkAnchoredMenu(page: Page, label: string) {
    await page.getByTestId("arm-menu").click();
    const anchor = (await geometry(page)).anchors[7];
    // The last check left the region scrolled wherever anchor 19 was.
    await showAnchor(page, anchor);
    const rect = await anchorRect(page, anchor);
    await page.mouse.click(rect.x + rect.width / 2, rect.y + rect.height / 2);
    await expect.poll(async () => (await geometry(page)).menu).toBe(7);

    const expected = await anchorRect(page, anchor);
    const menu = await page.evaluate(() => {
        const element = document.querySelector('[data-testid="geometry-menu"]')!;
        const box = element.getBoundingClientRect();
        return { x: box.left, y: box.top };
    });
    expect(Math.abs(menu.x - expected.x), `${label} menu left`).toBeLessThanOrEqual(1);
    expect(Math.abs(menu.y - (expected.y + expected.height)), `${label} menu top`).toBeLessThanOrEqual(1);

    await page.keyboard.press("Escape");
    await expect.poll(async () => (await geometry(page)).menu).toBeNull();
}

// A zoom other than 100% is a device scale factor, which a page cannot
// change on itself, so those sizes run on a page of their own; the shared page
// takes the viewport of a 100% one.
// The other seven are the same check at more sizes: they run with the
// measurements rather than on every change, so no size goes unchecked.
for (const size of SIZES) {
    const { viewport, zoom } = size;
    const label = `${viewport.width}x${viewport.height} at ${zoom * 100}%`;
    const tag = REPRESENTATIVE.includes(size) ? [] : [EVIDENCE];
    test.describe(`M4 V4: one geometry for display and for hits, ${label}`, { tag }, () => {
        test.use({ viewport, deviceScaleFactor: zoom });

        test("twenty anchors agree with the picture and with the pointer", async ({ page }) => {
            test.setTimeout(300_000);
            await mountGeometry(page);
            await checkAnchors(page, label);
            await checkAnchoredMenu(page, label);
        });
    });
}

test.describe("M4 V4: the region follows a change of resolution", () => {
    test("the backing store matches the device pixel ratio after zooming", async ({ page }) => {
        test.setTimeout(300_000);
        const cdp = await page.context().newCDPSession(page);
        await mountGeometry(page);

        const backing = () =>
            page.evaluate(() => {
                const canvas = document.querySelector('[data-testid="geometry-gpu"]') as HTMLCanvasElement;
                return {
                    dpr: window.devicePixelRatio,
                    css_width: canvas.clientWidth,
                    css_height: canvas.clientHeight,
                    width: canvas.width,
                    height: canvas.height,
                };
            });

        // Zooming a page shrinks the CSS viewport and raises the ratio; the
        // canvas keeps its CSS size either way, so the region's only signal
        // that it must redraw at a new resolution is the resolution itself.
        const base = { width: 1280, height: 720 };
        for (const zoom of [1.25, 2, 1, 3]) {
            await cdp.send("Emulation.setDeviceMetricsOverride", {
                width: Math.round(base.width / zoom),
                height: Math.round(base.height / zoom),
                deviceScaleFactor: zoom,
                mobile: false,
            });
            await expect
                .poll(
                    async () => {
                        const state = await backing();
                        return state.width;
                    },
                    { timeout: 5_000 }
                )
                .toBe(Math.round(400 * zoom));
            const state = await backing();
            expect(state.dpr, `device pixel ratio at ${zoom}`).toBeCloseTo(zoom, 5);
            expect(state.css_width).toBe(400);
            expect(state.css_height).toBe(240);
            expect(state.height).toBe(Math.round(240 * zoom));
        }
        await cdp.send("Emulation.clearDeviceMetricsOverride");
        await cdp.detach();
    });
});

test.describe("M4 V4: the clip is the same for the picture and for the pointer", () => {
    test("a hundred rounds of resizing and scrolling keep the region inside its clip", async ({
        page,
    }) => {
        test.setTimeout(600_000);
        await mountGeometry(page);
        const before = (await geometry(page)).hits;
        let clicksOutside = 0;
        let leakChecks = 0;

        // Each round is a size and a pair of offsets rather than a repeat, so
        // a short run takes rounds that reach past both clip edges and fall on
        // a look: the second is under the bottom edge, the other two past
        // the right one.
        const played = pick(
            Array.from({ length: 100 }, (_, round) => round),
            [2, 10, 20]
        );
        for (const round of played) {
            // A size the region has to re-lay out for, and scroll offsets that
            // move it under its clip box on both axes.
            const outside = await page.evaluate((round) => {
                const canvas = document.querySelector('[data-testid="geometry-gpu"]') as HTMLElement;
                const inner = document.querySelector('[data-testid="geometry-inner"]') as HTMLElement;
                const outer = document.querySelector('[data-testid="geometry-outer"]') as HTMLElement;
                canvas.style.width = `${300 + (round % 7) * 20}px`;
                canvas.style.height = `${180 + (round % 5) * 20}px`;
                const span = (element: HTMLElement, axis: "Width" | "Height") =>
                    Math.max(0, element[`scroll${axis}`] - element[`client${axis}`]);
                // Kept small on purpose: the region has to stay under the clip
                // edge for the round to be able to say anything about it.
                inner.scrollLeft = Math.min((round * 7) % 40, span(inner, "Width"));
                inner.scrollTop = Math.min((round * 5) % 40, span(inner, "Height"));
                outer.scrollTop = (round * 11) % (span(outer, "Height") + 1);
                inner.scrollIntoView({ block: "center", inline: "center" });

                const canvas_box = canvas.getBoundingClientRect();
                const clip = inner.getBoundingClientRect();
                const overlap = (a0: number, a1: number, b0: number, b1: number) => {
                    const lo = Math.max(a0, b0);
                    const hi = Math.min(a1, b1);
                    return hi - lo > 16 ? (lo + hi) / 2 : null;
                };
                // Past the right edge if the region reaches there, otherwise
                // past the bottom edge.
                if (canvas_box.right > clip.right + 8) {
                    const y = overlap(canvas_box.top, canvas_box.bottom, clip.top, clip.bottom);
                    if (y !== null) {
                        return {
                            x: clip.right + 4,
                            y,
                            strip: { x: clip.right + 2, y: y - 16, width: 6, height: 32 },
                        };
                    }
                }
                if (canvas_box.bottom > clip.bottom + 8) {
                    const x = overlap(canvas_box.left, canvas_box.right, clip.left, clip.right);
                    if (x !== null) {
                        return {
                            x,
                            y: clip.bottom + 4,
                            strip: { x: x - 16, y: clip.bottom + 2, width: 32, height: 6 },
                        };
                    }
                }
                return null;
            }, round);

            if (outside === null) {
                continue;
            }
            clicksOutside += 1;
            await page.mouse.click(outside.x, outside.y);

            // Every tenth round, also look: nothing the region draws may show
            // up past the clip edge.
            if (round % 10 === 0) {
                const shot = await page.screenshot({ scale: "css", clip: outside.strip });
                const png = PNG.sync.read(shot);
                let dark = 0;
                for (let i = 0; i < png.data.length; i += 4) {
                    const luma =
                        0.299 * png.data[i] + 0.587 * png.data[i + 1] + 0.114 * png.data[i + 2];
                    if (luma < 96) {
                        dark += 1;
                    }
                }
                expect(dark, `round ${round}: region pixels past the clip edge`).toBe(0);
                leakChecks += 1;
            }
        }

        // A fifth of the rounds clicked past the edge, and a third of the
        // looks found something to look at: at full length, more than twenty
        // clicks and three looks.
        expect(clicksOutside).toBeGreaterThan(Math.floor(played.length / 5));
        expect(leakChecks).toBeGreaterThan(Math.floor(played.filter((round) => round % 10 === 0).length / 3));
        // Not one of those clicks reached the region.
        expect((await geometry(page)).hits).toBe(before);

        // And the region still works where it is visible.
        await page.evaluate(() => {
            const canvas = document.querySelector('[data-testid="geometry-gpu"]') as HTMLElement;
            canvas.style.width = "400px";
            canvas.style.height = "240px";
        });
        await checkAnchors(page, "after a hundred rounds");
    });
});

test.describe("M4 V4: a region with no area", () => {
    test("suspends, and comes back at the size it returns to", async ({ page }) => {
        test.setTimeout(300_000);
        await mountGeometry(page);

        await page.evaluate(() => {
            const canvas = document.querySelector('[data-testid="geometry-gpu"]') as HTMLElement;
            canvas.style.display = "none";
        });
        await expect.poll(async () => (await geometry(page)).state).toBe("suspended");

        // The layout it comes back to is not the one it left: a frame drawn at
        // the old size would be visibly wrong.
        const recovery = await page.evaluate(async () => {
            const canvas = document.querySelector('[data-testid="geometry-gpu"]') as HTMLCanvasElement;
            canvas.style.width = "320px";
            canvas.style.height = "200px";
            const start = performance.now();
            canvas.style.display = "";
            let frames = 0;
            await new Promise<void>((resolve) => {
                const tick = () => {
                    frames += 1;
                    const wanted = Math.round(canvas.clientWidth * window.devicePixelRatio);
                    if (canvas.clientWidth > 0 && canvas.width === wanted) {
                        resolve();
                        return;
                    }
                    if (performance.now() - start > 2_000) {
                        resolve();
                        return;
                    }
                    requestAnimationFrame(tick);
                };
                requestAnimationFrame(tick);
            });
            return { ms: performance.now() - start, frames, width: canvas.width, css: canvas.clientWidth };
        });

        expect(recovery.css).toBe(320);
        expect(recovery.ms).toBeLessThanOrEqual(500);
        // The frame that already shows the new size is the first correct one,
        // so anything before it is a frame at the old size.
        expect(recovery.frames - 1).toBeLessThanOrEqual(2);

        await expect.poll(async () => (await geometry(page)).state).toBe("ready");
        await checkAnchors(page, "after coming back from no area");
    });
});
