import { Browser, expect, Locator, Page, test } from "@playwright/test";
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
                editing: boolean;
                invalidated: number;
                notes: string | null;
                theme: string;
                details: string;
                details_value: string | null;
                locked: boolean;
                size: number;
                refusals: number;
                refusal: string | null;
                third_party: boolean;
                third_party_updates: number;
                controls: {
                    locked: { x: number; y: number; width: number; height: number };
                    size: { x: number; y: number; width: number; height: number };
                } | null;
                region: string;
            };
            mount_into(container_id: string): number;
            mount_over(container_id: string): string;
            dispose_handle(id: number, container_id: string): boolean;
            set_third_party(present: boolean): boolean;
            nudge_third_party(value: number): number;
            third_party(): {
                created: number;
                destroyed: number;
                live: number;
                targets: number;
                handles: number;
            };
            diagnostics(): {
                runtime: number;
                build: string;
                count: number;
                bytes: number;
                dropped: number;
                recording: boolean;
                suppressed: number;
                max_entries: number;
                max_bytes: number;
                entries: {
                    kind: string;
                    at_ms: number;
                    scope: string | null;
                    region: number | null;
                    asset: string | null;
                    detail: string;
                    suggestion: string;
                }[];
            };
            set_diagnostics(on: boolean): boolean;
            lose_context(test_id: string): boolean;
            restore_context(test_id: string): boolean;
            inject_duplicate_id(): boolean;
            close_on_next_action(): void;
            start_load(delay_ms: number, outcome: string): boolean;
            lookup_object(id: number): string;
            await_object(id: number, timeout_ms?: number): Promise<string>;
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
        __component_catalog: {
            mount(): number;
            dispose(): boolean;
            mount_second(): number;
            dispose_second(): boolean;
            live_regions(): number;
            snapshot(): {
                path: string;
                theme: string;
                locale: string;
                categories: number;
                button: { x: number; y: number; width: number; height: number } | null;
                region: string;
            };
            diagnostics(): {
                runtime: number;
                build: string;
                count: number;
                dropped: number;
                recording: boolean;
                suppressed: number;
                entries: {
                    kind: string;
                    at_ms: number;
                    scope: string | null;
                    region: number | null;
                    asset: string | null;
                    detail: string;
                    suggestion: string;
                }[];
            };
            stats(): {
                regions: number;
                timers: number;
                animation_frames: number;
                tasks: number;
                errors: number;
                memory: number;
                pumps: number;
            };
            hooks: { runtime: { errors: string[]; enter_fatal(error: unknown): void } };
        };
        __fusion_basic: {
            mount(container_id: string): number;
            mount_owner(container_id: string): number;
            mount_guest(container_id: string): number;
            routes(): string;
            set_guard(on: boolean): boolean;
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
            diagnostics(): {
                runtime: number;
                build: string;
                count: number;
                dropped: number;
                entries: {
                    kind: string;
                    at_ms: number;
                    scope: string | null;
                    region: number | null;
                    asset: string | null;
                    detail: string;
                    suggestion: string;
                }[];
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
                selected: number | null;
                colors: string;
                applied: number;
                refusal: string | null;
            };
            hooks: { runtime: { errors: string[]; enter_fatal(error: unknown): void } };
        };
    }
}


/// One page for a whole `describe`, instead of one per test.
///
/// A cold catalogue page downloads and compiles an eleven megabyte module and
/// boots its region: about seven seconds, which for a short check is nearly
/// all of it. Sharing the page makes the suite roughly as long as the work in
/// it rather than as long as the number of tests.
///
/// The cost is that the tests in the block are no longer independent, so the
/// block has to run serially and each test has to leave the page as it found
/// it - or read a delta rather than an absolute. Use it where the checks are
/// reads and small reversible interactions; use a fresh `page` where a test
/// changes something it cannot put back.
export function sharedPage(setup?: (page: Page) => Promise<void>): { page: Page } {
    // Filled by `beforeAll`; the tests in the block run after it.
    const holder = { page: null as unknown as Page };
    test.beforeAll(async ({ browser }: { browser: Browser }, info) => {
        const context = await browser.newContext({ baseURL: info.project.use.baseURL });
        holder.page = await context.newPage();
        await waitForReady(holder.page);
        await setup?.(holder.page);
    });
    test.afterAll(async () => {
        await holder.page?.context().close();
    });
    return holder;
}
