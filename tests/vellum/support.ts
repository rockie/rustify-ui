import { expect, test as base, type ConsoleMessage, type Locator, type Page } from "@playwright/test";
import { PNG } from "pngjs";
import { EVIDENCE } from "../tier";

export { expect };

export interface VellumNode {
    id: string;
    type: string;
    name: string;
    parentId: string | null;
    x: number;
    y: number;
    w: number;
    h: number;
    rotation: number;
    [key: string]: any;
}

declare global {
    interface Window {
        __vellumCsp: string[];
        __vellum: {
            reset(): Promise<void>;
            stats(): { frames: number; gpu_bytes: number; [key: string]: any };
            errors(): any[];
            diagnostics(): any;
            live_regions(): number;
            rasterStats(): { bytes: number; entries: number; hits: number; misses: number; evictions: number; lastResolution: number; lastProjectMs: number; lastGenerated: number; totalGenerated: number; pending: number; deferred: number; lastBudgetMs: number };
        };
        vellum: {
            ready: boolean;
            doc: {
                nodes: VellumNode[];
                page: { id: string; name: string };
                data: { name: string; pageId: string; pages: { id: string; name: string; nodes: VellumNode[] }[] };
                get(id: string): VellumNode;
                world(id: string): { matrix: number[]; box: { x: number; y: number; w: number; h: number } };
                serialize(): string;
            };
            state: { camera: { x: number; y: number; zoom: number }; selection: Set<string>; [key: string]: any };
            renderer: { backend: string; visibleCount: number; instanceCount: number; drawCalls: number; cpuMs: number; gpuBytes: number; width: number; height: number; [key: string]: any };
            actions: Record<string, (...args: any[]) => any>;
            select(ids: string[]): void;
            fit(ids?: string[]): void;
            zoomAt(factor: number, x?: number, y?: number): void;
            setProperty(prop: string, value: any): void;
            createAtCenter(kind: string, props?: Record<string, any>): VellumNode;
            transaction(label: string, patches: { id: string; prop: string; value: any }[]): void;
            render(): void;
            textLayout(id: string): { displayText: string; fontSpec: string; lines: string[]; lineHeight: number; height: number; baseline: number; widths: number[] };
            importFont(input: { name: string; dataUrl: string }): Promise<{ family: string }>;
            loadStoredFonts(): Promise<void>;
            fontReady(): Promise<{ families: string[]; pending: number; errors: string[] }>;
            setAsset(input: { id: string; source: string }): Promise<void>;
        };
    }
}

function recordCsp() {
    window.__vellumCsp = [];
    document.addEventListener("securitypolicyviolation", event => {
        window.__vellumCsp.push(`${event.violatedDirective}: ${event.blockedURI}`);
    });
}

function isError(message: ConsoleMessage): boolean {
    return message.type() === "error" || /content security policy|violates.*directive/i.test(message.text());
}

/// What a shared page logged while it was made ready for a test - its load or
/// its reset - and how many CSP violations it had recorded before that.
/// `browserErrors` holds the test to it.
interface Setup {
    errors: string[];
    console: string[];
    csp: number;
}

interface Slot {
    page: Page | null;
    /// The test options the page's context was made with.
    options: string | null;
    /// Why the next test has to load the page again.
    lost: string;
}

const shared = new WeakSet<Page>();
const setups = new WeakMap<Page, Setup>();
/// Shared pages a test has already called `waitForReady` on.
const readied = new WeakSet<Page>();

/// Whether this page is the worker's shared one rather than the test's own.
export function isShared(page: Page): boolean {
    return shared.has(page);
}

/// Whether the shared page can carry the next test: the instance is alive,
/// its one region is up, it has recorded no error and it says it is ready.
async function healthy(page: Page): Promise<boolean> {
    if (page.isClosed()) return false;
    return page.evaluate(() => typeof window.__vellum?.reset === "function"
        && document.getElementById("status")?.dataset.status === "ready"
        && window.__vellum.live_regions() === 1
        && window.__vellum.errors().length === 0).catch(() => false);
}

async function discard(slot: Slot, why: string) {
    const page = slot.page;
    slot.page = null;
    slot.options = null;
    slot.lost = why;
    await page?.context().close();
}

/// `test`, with one page per worker instead of one per test.
///
/// A test runs on the page the previous test used, put back by the editor's
/// own `reset()`: storage wiped and the editor mounted again, which is a
/// fraction of a page load. It gets a page of its own - Playwright's, in a new
/// context - when it asks with `test.use({ fresh: true })`, when it sets
/// options the shared context was not made with, when it is evidence (those
/// measure from a load, as they always have), or when the previous test
/// failed or left the page broken.
///
/// A test needs a fresh page when it reloads or navigates, installs an init
/// script, stubs a prototype it does not put back, or makes the instance trap.
export const test = base.extend<{ fresh: boolean; browserErrors: string[] }, { slot: Slot }>({
    fresh: [false, { option: true }],
    slot: [async ({}, use) => {
        const slot: Slot = { page: null, options: null, lost: "the first test on this worker" };
        await use(slot);
        await discard(slot, "the worker is done");
    }, { scope: "worker" }],
    page: async ({ page, fresh, slot, browser, viewport, deviceScaleFactor, isMobile, hasTouch, locale, colorScheme, reducedMotion, forcedColors, userAgent }, use, info) => {
        const options = JSON.stringify({ viewport, deviceScaleFactor, isMobile, hasTouch, locale, colorScheme, reducedMotion, forcedColors, userAgent });
        if (fresh || info.tags.includes(EVIDENCE) || (slot.options !== null && slot.options !== options)) {
            await use(page);
            return;
        }

        let setup: Setup = { errors: [], console: [], csp: 0 };
        const onError = (error: Error) => setup.errors.push(error.message);
        const onConsole = (message: ConsoleMessage) => {
            setup.console.push(`${message.type()}: ${message.text()}`);
            if (isError(message)) setup.errors.push(message.text());
        };
        let app = slot.page;
        if (app !== null && await healthy(app)) {
            app.on("pageerror", onError);
            app.on("console", onConsole);
            setup.csp = await app.evaluate(() => window.__vellumCsp.length);
            const failure = await app.evaluate(async () => {
                let timer: ReturnType<typeof setTimeout> | undefined;
                const late = new Promise<string>(resolve => { timer = setTimeout(() => resolve("did not finish within 30 s"), 30_000); });
                try {
                    return await Promise.race([window.__vellum.reset().then(() => null), late]);
                } catch (error) {
                    return String(error);
                } finally {
                    clearTimeout(timer);
                }
            });
            app.off("pageerror", onError);
            app.off("console", onConsole);
            if (failure !== null) {
                // The test still gets a page as a first load has it.
                await discard(slot, `reset failed: ${failure}`);
                app = null;
            }
        } else if (app !== null) {
            await discard(slot, "the page was no longer healthy");
            app = null;
        }
        if (app === null) {
            // The report says which tests paid for a load, and why.
            info.annotations.push({ type: "page load", description: slot.lost });
            setup = { errors: [], console: [], csp: 0 };
            // The test's own options fill in what is not given here, so the
            // context is the one Playwright would have made for it.
            const context = await browser.newContext();
            await context.addInitScript(recordCsp);
            app = await context.newPage();
            slot.page = app;
            slot.options = options;
            shared.add(app);
            app.on("pageerror", onError);
            app.on("console", onConsole);
            await app.goto("./");
        }
        await settle(app);
        app.off("pageerror", onError);
        app.off("console", onConsole);
        setups.set(app, setup);
        readied.delete(app);

        const listening = new Map(app.eventNames().map(name => [name, app!.listeners(name).slice()]));
        await use(app);

        for (const name of app.eventNames()) {
            const before = listening.get(name) ?? [];
            for (const listener of app.listeners(name)) {
                if (!before.includes(listener)) app.removeListener(name, listener as (...args: unknown[]) => void);
            }
        }
        if (info.status !== info.expectedStatus) {
            await discard(slot, "the previous test failed");
            return;
        }
        if (!(await healthy(app))) {
            await discard(slot, "the previous test left the page unhealthy");
            return;
        }
        await app.unrouteAll({ behavior: "ignoreErrors" });
        await app.emulateMedia({ media: null, colorScheme: colorScheme ?? null, reducedMotion: reducedMotion ?? null, forcedColors: forcedColors ?? null });
        await app.mouse.move(0, 0);
        if (viewport) await app.setViewportSize(viewport);
    },
    browserErrors: [async ({ page }, use, info) => {
        const setup = setups.get(page);
        setups.delete(page);
        const errors: string[] = [...setup?.errors ?? []];
        const consoleMessages: string[] = [...setup?.console ?? []];
        page.on("pageerror", error => errors.push(error.message));
        page.on("console", message => {
            consoleMessages.push(`${message.type()}: ${message.text()}`);
            if (isError(message)) errors.push(message.text());
        });
        // A shared page has had this since its context was made.
        if (!isShared(page)) await page.addInitScript(recordCsp);
        await use(errors);
        await info.attach("browser-console", { body: consoleMessages.join("\n"), contentType: "text/plain" });
        expect(errors, "Uncaught errors and CSP console messages").toEqual([]);
        if (!page.isClosed()) expect(await page.evaluate(start => (window.__vellumCsp ?? []).slice(start), setup?.csp ?? 0)).toEqual([]);
    }, { auto: true }],
});

/// Loads the editor and waits until it has drawn its scene. On the shared
/// page the fixture has already done both, from a reset instead of a load;
/// anything that needs an actual load there - another address, the no-GPU
/// mode, a second call to reload - has to ask for a fresh page.
export async function waitForReady(page: Page, url = "./", gpu = true) {
    if (isShared(page)) {
        if (url !== "./" || !gpu || readied.has(page)) {
            throw new Error(`waitForReady(page, "${url}", ${gpu}) needs a page load: mark the test with test.use({ fresh: true })`);
        }
        readied.add(page);
        return;
    }
    await page.goto(url);
    await settle(page, gpu);
}

async function settle(page: Page, gpu = true) {
    await expect(page.locator("#status")).toHaveAttribute("data-status", "ready", { timeout: 60_000 });
    await page.waitForFunction(() => window.vellum?.ready);
    if (gpu) {
        await expect.poll(() => page.evaluate(() => window.vellum.renderer.backend), { timeout: 60_000 }).toBe("Makepad WebGL2");
        await expect.poll(() => page.evaluate(() => window.vellum.renderer.instanceCount), { timeout: 60_000 }).toBeGreaterThan(150);
    }
    await waitForQuiet(page);
}

export async function waitForQuiet(page: Page, quietMs = 500, timeoutMs = 60_000) {
    await page.evaluate(([quietMs, timeoutMs]) => new Promise<void>(resolve => {
        const deadline = performance.now() + timeoutMs;
        let previous = performance.now();
        let since = previous;
        const tick = () => {
            const now = performance.now();
            if (now - previous > 100) since = now;
            previous = now;
            if (now - since >= quietMs || now > deadline) resolve();
            else requestAnimationFrame(tick);
        };
        requestAnimationFrame(tick);
    }), [quietMs, timeoutMs]);
}

export async function present(page: Page) {
    await page.evaluate(async () => {
        const frame = () => new Promise<void>(resolve => requestAnimationFrame(() => resolve()));
        await frame(); await frame(); await frame();
        const started = performance.now();
        while (window.__vellum?.rasterStats?.().pending > 0) {
            if (performance.now() - started > 30_000) throw new Error("Raster refinement did not finish");
            await frame();
        }
        await frame(); await frame();
    });
}

export interface Pixels { width: number; height: number; data: Buffer }

export async function capture(locator: Locator): Promise<Pixels> {
    const png = PNG.sync.read(await locator.screenshot({ animations: "disabled" }));
    return { width: png.width, height: png.height, data: png.data };
}

export function differingPixels(a: Pixels, b: Pixels, threshold = 24): number {
    expect(a.width).toBe(b.width);
    expect(a.height).toBe(b.height);
    let count = 0;
    for (let i = 0; i < a.data.length; i += 4) {
        const delta = Math.abs(a.data[i] - b.data[i]) + Math.abs(a.data[i + 1] - b.data[i + 1]) + Math.abs(a.data[i + 2] - b.data[i + 2]);
        if (delta > threshold) count++;
    }
    return count;
}

export function rgb(pixels: Pixels, x: number, y: number): number[] {
    const start = (Math.round(y) * pixels.width + Math.round(x)) * 4;
    return [...pixels.data.subarray(start, start + 3)];
}
