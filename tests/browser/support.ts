import { Browser, BrowserContext, expect, Locator, Page, test as base, TestInfo } from "@playwright/test";

export { expect };
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
    // A shared page is already loaded and reset; loading it again would pay
    // for the region start-up the page is shared to avoid.
    if (!isShared(page)) {
        await page.goto("./");
    }
    await expect(page.getByTestId("status")).toHaveAttribute("data-status", "ready", { timeout: 60_000 });
    // The fusion-basic page shows B0 and nothing else: a load has to be the
    // load it says it is, so every other fixture is mounted by whoever needs
    // it. The two counter scopes are what the phase-one checks were written
    // for, and this is where they come from now. Every example's checks come
    // through here, and only one of them has those scopes to mount.
    //
    // Only the missing ones: a shared page's reset keeps a healthy counter
    // scope mounted, because mounting it again would start its two regions
    // again - seconds each on a software rasteriser.
    const mounted = await page.evaluate(() => {
        const fusion = window.__fusion_basic;
        if (fusion === undefined) {
            return false;
        }
        let mounted = false;
        for (const scope of ["scope-a", "scope-b"]) {
            if (!document.getElementById(scope)?.hasAttribute("data-rustify-scope")) {
                fusion.mount(scope);
                mounted = true;
            }
        }
        return mounted;
    });
    // The fixture waited for quiet after its reset; only a region started
    // here has anything left to settle.
    if (mounted || !isShared(page)) {
        await waitForQuiet(page);
    }
}

/// Waits until the page stops blocking its own animation frames.
///
/// "Ready" is the application saying it has mounted, and a region's start-up
/// runs on after it: it presents once, and presents again when the font it
/// asked for has arrived and its atlas has been built. On a software
/// rasteriser that second one blocks the main thread for about six seconds. A
/// test that starts measuring inside that window is measuring the font atlas,
/// and one that clicks inside it waits out its own timeout for the click to
/// take effect - which is how a check that has nothing to do with start-up
/// fails, and only sometimes, depending on what ran before it.
///
/// Quiet is the absence of a late frame rather than the absence of frames: a
/// region that is animating is not busy, and one that is rasterising is.
export async function waitForQuiet(page: Page, quietMs = 500, timeoutMs = 60_000) {
    await page.evaluate(
        ([quietMs, timeoutMs]) =>
            new Promise<void>((resolve) => {
                const deadline = performance.now() + timeoutMs;
                let previous = performance.now();
                let since = previous;
                const tick = () => {
                    const now = performance.now();
                    if (now - previous > 100) {
                        since = now;
                    }
                    previous = now;
                    if (now - since >= quietMs || now > deadline) {
                        // Past the deadline the page is busy with something
                        // that is not start-up, and whatever the test came to
                        // check is a better thing to fail on than this.
                        resolve();
                        return;
                    }
                    requestAnimationFrame(tick);
                };
                requestAnimationFrame(tick);
            }),
        [quietMs, timeoutMs]
    );
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
/// Every category in the catalogue. R18 names eighteen; the table and the tree
/// that large data needed are two more. Written here so a test that counts
/// them says what the number means rather than repeating a literal.
export const CATALOG_SIZE = 20;

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
            reset(): Promise<void>;
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
                    name: { x: number; y: number; width: number; height: number };
                    notes: { x: number; y: number; width: number; height: number };
                    groups: { x: number; y: number; width: number; height: number };
                    row: number;
                } | null;
                region: string;
                form: {
                    fields: number;
                    errors: number;
                    first_error: string | null;
                    can_submit: boolean;
                    submitting: boolean;
                    dirty: boolean;
                    checks: number;
                    saves: number;
                    asked: string;
                    failure: string | null;
                };
                path: string;
                guarded: boolean;
                workspace: {
                    panels: number[];
                    tabs: number;
                    tab: string;
                    palette: boolean;
                    menu: boolean;
                };
                drag: {
                    drops: number;
                    cancels: number;
                    grouped: number;
                    dragging: boolean;
                    target: string | null;
                    selected_group: number | null;
                    propagates: boolean;
                    scroll: number;
                    scroll_max: number;
                };
                transfer: {
                    imports: number;
                    exports: number;
                    status: string;
                    bytes: number;
                    copies: number;
                    pastes: number;
                    clipboard: string;
                    by_hand: boolean;
                    can_copy: boolean;
                };
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
                frames: number;
            };
            hooks: {
                regions: Map<number, { new_from_wasm(ptr: number): unknown }>;
                defer(callback: () => void): void;
                runtime: { errors: string[]; enter_fatal(error: unknown): void };
            };
        };
        __component_catalog: {
            reset(): Promise<void>;
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
                frames: number;
            };
            hooks: { runtime: { errors: string[]; enter_fatal(error: unknown): void } };
        };
        /// Every instance running on the fusion-basic page, by its number.
        __fusion_instances: Record<number, Window["__fusion_basic"]>;
        __data_workbench: {
            /// Puts the page where a first load of `path` (default `/`) puts it.
            reset(path?: string): Promise<void>;
            hooks: { runtime: { errors: string[]; enter_fatal(error: unknown): void } };
            instance: number;
            mount(container_id?: string): number;
            dispose(): boolean;
            snapshot(): {
                path: string;
                rows: number;
                version: number;
                generated_ms: number;
                scene: {
                    camera: [number, number];
                    pane: [number, number];
                    drawn: number;
                    asked: number;
                    reported: number;
                };
                table: {
                    selected: number;
                    hidden: number;
                    editing: number | null;
                    group: string | null;
                    jumps: number;
                    saves: number;
                    window_version: number;
                    /// Rows the view is showing, which is not the sample's
                    /// count once something has been filtered.
                    shown: number;
                    /// Whether the sample has been written to since the view
                    /// was built.
                    stale: boolean;
                    /// "3:asc", "3:desc", or nothing.
                    sorted: string | null;
                    job: {
                        running: boolean;
                        done: number;
                        total: number;
                        slices: number;
                        /// "done", "stale" or "cancelled".
                        ended: string | null;
                    };
                };
            };
            diagnostics(): {
                runtime: number;
                build: string;
                count: number;
                entries: { kind: string; severity: string; detail: string }[];
            };
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
                frames: number;
            };
            dataset_hash(): string;
            row_id(row: number): number;
            cell(row: number, column: number): string;
            look_at(x: number, y: number): boolean;
            freeze_scene(on: boolean): boolean;
            scene_visible(): number;
            window_version(): number;
            selected(): number[];
            select_rows(from: number, count: number): number[];
            open_row(row: number): boolean;
            view(from: number, count: number): number[];
            sort(column: number, ascending?: boolean): boolean;
            cancel_job(): boolean;
            sort_probe(column: number, ascending?: boolean): boolean;
            sort_probe_state(): { running: boolean; slices: number; ms: number; rows: number };
            sort_probe_order(from: number, count: number): number[];
        };
        __fusion_basic: {
            reset(): Promise<void>;
            instance: number;
            wasm: { exports: { memory: WebAssembly.Memory } };
            restarts: number;
            restart_limit: number;
            mount(container_id: string): number;
            b0(container_id: string): number;
            b0_state(): {
                count: number;
                title: string;
                subtitle: string;
                tag: string;
                notes: string;
                visible: boolean;
                locked: boolean;
                compact: boolean;
                busy: boolean;
                choice: number;
                size: number;
                weight: number;
                palette: string;
                density: string;
                tab: number;
                region: string;
                controls: { name: string; x: number; y: number; width: number; height: number }[];
            };
            b0_controls(): string[];
            live_components(): number;
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
                frames: number;
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
            mount_trap(container_id: string): number;
            trap(): void;
            fatal(): string | null;
            boot_instance(container_id: string, fixture?: string): Promise<number>;
            boot_second_instance(container_id?: string): Promise<number>;
            restart(container_id: string): Promise<number | null>;
            trap_deferred(): void;
            simulate_trap(message?: string): void;
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


/// The window handle each project's example publishes, and so the one whose
/// `reset()` puts a shared page back. A project that is not here runs every
/// test on a fresh page.
const HANDLES: Record<string, string> = {
    "fusion-basic": "__fusion_basic",
    "property-workbench": "__property_workbench",
    "component-catalog": "__component_catalog",
    "data-workbench": "__data_workbench",
};

const shared = new WeakSet<Page>();

/// Whether this page is the worker's shared one rather than the test's own.
export function isShared(page: Page): boolean {
    return shared.has(page);
}

interface Slot {
    context: BrowserContext | null;
    page: Page | null;
    viewport: { width: number; height: number } | null;
}

/// The project options that make a context rather than a page.
const CONTEXT_ONLY = [
    "userAgent",
    "javaScriptEnabled",
    "bypassCSP",
    "offline",
    "permissions",
    "geolocation",
    "extraHTTPHeaders",
    "storageState",
    "httpCredentials",
    "ignoreHTTPSErrors",
    "acceptDownloads",
    "serviceWorkers",
    "timezoneId",
    "forcedColors",
    "screen",
    "baseURL",
] as const;

async function open(
    browser: Browser,
    slot: Slot,
    info: TestInfo,
    fixed: Record<string, unknown>,
    viewport: { width: number; height: number } | null
): Promise<Page> {
    const use = info.project.use as Record<string, unknown>;
    const options: Record<string, unknown> = { ...fixed, viewport };
    for (const key of CONTEXT_ONLY) {
        if (use[key] !== undefined) {
            options[key] = use[key];
        }
    }
    slot.context = await browser.newContext(options);
    slot.page = await slot.context.newPage();
    slot.viewport = viewport;
    shared.add(slot.page);
    await slot.page.goto("./");
    await expect(slot.page.getByTestId("status")).toHaveAttribute("data-status", "ready", { timeout: 60_000 });
    return slot.page;
}

async function discard(slot: Slot) {
    const context = slot.context;
    slot.context = null;
    slot.page = null;
    slot.viewport = null;
    await context?.close();
}

/// Whether the shared page can carry the next test: the instance is alive,
/// its handle is published and the page still says it is ready.
async function healthy(page: Page, handle: string): Promise<boolean> {
    if (page.isClosed()) {
        return false;
    }
    return page
        .evaluate(
            (handle) =>
                (window as unknown as Record<string, unknown>)[handle] !== undefined &&
                document.querySelector('[data-testid="status"]')?.getAttribute("data-status") === "ready",
            handle
        )
        .catch(() => false);
}

/// `test`, with one page per worker and project instead of one per test.
///
/// Loading the page is not what a test costs; starting its region is. Every
/// new WebGL context has the software rasteriser compile the region's
/// shaders, which takes several seconds and happens again on a remount. So a
/// regression test runs on the page the previous test used, put back by the
/// example's own `reset()`, and pays for a start-up only when it asks for one
/// with `test.use({ fresh: true })`, when it sets a context option a live page
/// cannot take (a device scale factor, a locale), or when the previous test
/// left the page broken or failed on it.
///
/// A fresh page is Playwright's own: a new context per test. Tests that touch
/// what a context owns - `context.addInitScript`, permissions, a second page
/// in the same context - need one.
export const test = base.extend<{ fresh: boolean }, { slot: Slot }>({
    fresh: [false, { option: true }],
    slot: [
        async ({}, use) => {
            const slot: Slot = { context: null, page: null, viewport: null };
            await use(slot);
            await discard(slot);
        },
        { scope: "worker" },
    ],
    page: async (
        { page, fresh, slot, browser, viewport, deviceScaleFactor, isMobile, hasTouch, locale, colorScheme, reducedMotion },
        use,
        info
    ) => {
        const handle = HANDLES[info.project.name];
        const project = info.project.use as Record<string, unknown>;
        // The shared context is always the project's own, whichever test
        // happens to open it; a test that asks for a different one gets its
        // own. These defaults are Playwright's for the same options.
        const projectFixed = {
            deviceScaleFactor: project.deviceScaleFactor,
            isMobile: project.isMobile ?? false,
            hasTouch: project.hasTouch ?? false,
            locale: project.locale ?? "en-US",
            colorScheme: project.colorScheme ?? "light",
            reducedMotion: project.reducedMotion ?? "no-preference",
        };
        const fixed = { deviceScaleFactor, isMobile, hasTouch, locale, colorScheme, reducedMotion };
        const projectViewport = (project.viewport ?? { width: 1280, height: 720 }) as { width: number; height: number };
        const differs = JSON.stringify(fixed) !== JSON.stringify(projectFixed);
        if (fresh || handle === undefined || differs) {
            await use(page);
            return;
        }

        let app = slot.page;
        if (app === null || !(await healthy(app, handle))) {
            await discard(slot);
            app = await open(browser, slot, info, projectFixed, projectViewport);
        } else {
            await app.evaluate(async (handle) => {
                const example = (window as unknown as Record<string, { reset(): Promise<void> }>)[handle];
                await example.reset();
            }, handle);
        }
        if (viewport && (viewport.width !== projectViewport.width || viewport.height !== projectViewport.height)) {
            await app.setViewportSize(viewport);
        }
        await waitForQuiet(app);

        const listening = new Map(app.eventNames().map((name) => [name, app!.listeners(name).slice()]));
        await use(app);

        for (const name of app.eventNames()) {
            const before = listening.get(name) ?? [];
            for (const listener of app.listeners(name)) {
                if (!before.includes(listener)) {
                    app.removeListener(name, listener as (...args: unknown[]) => void);
                }
            }
        }
        const passed = info.status === info.expectedStatus;
        if (!passed || app.isClosed() || !(await healthy(app, handle))) {
            await discard(slot);
            return;
        }
        await app.unrouteAll({ behavior: "ignoreErrors" });
        await app.emulateMedia({ media: null, colorScheme, reducedMotion, forcedColors: null });
        await app.mouse.move(0, 0);
        await app.setViewportSize(projectViewport);
    },
});
