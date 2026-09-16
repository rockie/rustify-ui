import { expect, test as base, type Locator, type Page } from "@playwright/test";
import { PNG } from "pngjs";

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

export const test = base.extend<{ browserErrors: string[] }>({
    browserErrors: [async ({ page }, use, info) => {
        const errors: string[] = [];
        const consoleMessages: string[] = [];
        page.on("pageerror", error => errors.push(error.message));
        page.on("console", message => {
            consoleMessages.push(`${message.type()}: ${message.text()}`);
            if (message.type() === "error" || /content security policy|violates.*directive/i.test(message.text())) errors.push(message.text());
        });
        await page.addInitScript(() => {
            window.__vellumCsp = [];
            document.addEventListener("securitypolicyviolation", event => {
                window.__vellumCsp.push(`${event.violatedDirective}: ${event.blockedURI}`);
            });
        });
        await use(errors);
        await info.attach("browser-console", { body: consoleMessages.join("\n"), contentType: "text/plain" });
        expect(errors, "Uncaught errors and CSP console messages").toEqual([]);
        if (!page.isClosed()) expect(await page.evaluate(() => window.__vellumCsp ?? [])).toEqual([]);
    }, { auto: true }],
});

export async function waitForReady(page: Page, url = "./", gpu = true) {
    await page.goto(url);
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
