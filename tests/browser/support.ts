import { expect, Locator, Page } from "@playwright/test";
import { PNG } from "pngjs";

export interface Anchor {
    id: number;
    x: number;
    y: number;
    width: number;
    height: number;
}

export const geometry = (page: Page) => page.evaluate(() => window.__fusion_basic.geometry());

/// Mounts the geometry fixture and waits until its region has reported the
/// anchors it drew.
export async function mountGeometry(page: Page) {
    await waitForReady(page);
    await page.evaluate(() => window.__fusion_basic.mount_geometry("geometry"));
    await expect.poll(async () => (await geometry(page)).state).toBe("ready");
    await expect.poll(async () => (await geometry(page)).anchors.length).toBe(20);
}

/// Where an anchor of the region sits in the viewport right now.
export async function anchorRect(page: Page, anchor: Anchor) {
    const origin = await page.evaluate(() => {
        const canvas = document.querySelector('[data-testid="geometry-gpu"]') as HTMLElement;
        const box = canvas.getBoundingClientRect();
        return { left: box.left, top: box.top };
    });
    return {
        x: origin.left + anchor.x,
        y: origin.top + anchor.y,
        width: anchor.width,
        height: anchor.height,
    };
}

export async function waitForReady(page: Page) {
    await page.goto("./");
    await expect(page.getByTestId("status")).toHaveAttribute("data-status", "ready", { timeout: 60_000 });
}

export interface Pixels {
    width: number;
    height: number;
    data: Buffer;
}

export async function capture(locator: Locator): Promise<Pixels> {
    const png = PNG.sync.read(await locator.screenshot({ animations: "disabled" }));
    return { width: png.width, height: png.height, data: png.data };
}

/// Number of pixels whose RGB differs by more than `threshold` between two captures
/// of the same size.
export function differingPixels(a: Pixels, b: Pixels, threshold = 24): number {
    expect(a.width).toBe(b.width);
    expect(a.height).toBe(b.height);
    let count = 0;
    for (let i = 0; i < a.data.length; i += 4) {
        const delta =
            Math.abs(a.data[i] - b.data[i]) +
            Math.abs(a.data[i + 1] - b.data[i + 1]) +
            Math.abs(a.data[i + 2] - b.data[i + 2]);
        if (delta > threshold) {
            count += 1;
        }
    }
    return count;
}

/// Pixels that are clearly brighter than the region background.
export function litPixels(pixels: Pixels, minLuma = 96): number {
    let count = 0;
    for (let i = 0; i < pixels.data.length; i += 4) {
        const luma = 0.299 * pixels.data[i] + 0.587 * pixels.data[i + 1] + 0.114 * pixels.data[i + 2];
        if (luma > minLuma) {
            count += 1;
        }
    }
    return count;
}

/// Waits until two consecutive captures of the region are identical, i.e. the
/// GPU has finished presenting whatever was requested.
export async function settle(locator: Locator, attempts = 20): Promise<Pixels> {
    let previous = await capture(locator);
    for (let i = 0; i < attempts; i++) {
        await locator.page().waitForTimeout(100);
        const next = await capture(locator);
        if (differingPixels(previous, next) === 0) {
            return next;
        }
        previous = next;
    }
    return previous;
}

declare global {
    interface Window {
        __property_workbench: {
            mount(): number;
            dispose(): boolean;
            live_regions(): number;
            snapshot(): {
                count: number;
                position: number;
                selected: number | null;
                name: string | null;
                color: string;
                first_colors: string;
                first_ids: string;
                accepted: number;
                refused: number;
                hovered: number | null;
                hovers: number;
            };
            inject_duplicate_id(): boolean;
            close_on_next_action(): void;
            stats(): {
                regions: number;
                timers: number;
                animation_frames: number;
                tasks: number;
                errors: number;
                memory: number;
                pumps: number;
            };
            hooks: {
                regions: Map<number, { new_from_wasm(ptr: number): unknown }>;
                defer(callback: () => void): void;
                runtime: { errors: string[]; enter_fatal(error: unknown): void };
            };
        };
        __fusion_basic: {
            mount(container_id: string): number;
            dispose(container_id: string): boolean;
            live_regions(): number;
            errors(): string[];
            stats(): {
                regions: number;
                timers: number;
                animation_frames: number;
                tasks: number;
                errors: number;
                memory: number;
                pumps: number;
            };
            region_states(): Record<string, string>;
            mount_geometry(container_id: string): number;
            geometry(): {
                anchors: { id: number; x: number; y: number; width: number; height: number }[];
                hits: number;
                last_hit: { anchor: number; x: number; y: number } | null;
                state: string;
                menu: number | null;
                dialog: boolean;
                anchored: boolean;
                commands: number;
            };
            hooks: { runtime: { errors: string[]; enter_fatal(error: unknown): void } };
        };
    }
}
