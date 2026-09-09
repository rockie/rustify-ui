import { expect, Page, test } from "@playwright/test";

/// What a deployment of this build actually sends, and what it costs when the
/// link is not a loopback.
///
/// The size report counts bytes on disk; a browser fetches some of them, and
/// over a real connection the time that takes is the start-up figure a user
/// gets. Both numbers are taken from the browser: the resource timings are the
/// bytes that crossed the socket, and the clock starts at the navigation.

const CATEGORIES = [
    ["wasm", /\.wasm$/],
    ["js", /\.m?js$/],
    ["css", /\.css$/],
    ["fonts", /\.(ttf|otf|woff2?)$/],
    ["images", /\.(png|jpe?g|svg|webp|gif)$/],
] as const;

interface Transfer {
    name: string;
    transfer: number;
    encoded: number;
    decoded: number;
}

function byCategory(resources: Transfer[]) {
    const totals: Record<string, { transfer: number; encoded: number; count: number }> = {};
    for (const resource of resources) {
        const path = new URL(resource.name).pathname;
        const category = CATEGORIES.find(([, pattern]) => pattern.test(path))?.[0] ?? "data";
        const bucket = (totals[category] ??= { transfer: 0, encoded: 0, count: 0 });
        bucket.transfer += resource.transfer;
        bucket.encoded += resource.encoded;
        bucket.count += 1;
    }
    return totals;
}

/// Everything the page fetched, as the browser accounts for it. `transferSize`
/// includes the response headers and is zero for a cached entry, which is why
/// the cache is disabled before the load these numbers describe.
const transfers = (page: Page) =>
    page.evaluate(() =>
        performance.getEntriesByType("resource").map((entry) => {
            const resource = entry as PerformanceResourceTiming;
            return {
                name: resource.name,
                transfer: resource.transferSize,
                encoded: resource.encodedBodySize,
                decoded: resource.decodedBodySize,
            };
        })
    );

/// One session for the whole test. Network emulation belongs to the session
/// that set it, so a second session attached to the same page leaves it
/// ambiguous which one the browser is honouring - which is exactly how a
/// throttled load came back at loopback speed once.
async function session(page: Page) {
    const cdp = await page.context().newCDPSession(page);
    await cdp.send("Network.enable");
    await cdp.send("Network.setCacheDisabled", { cacheDisabled: true });
    return cdp;
}

async function coldLoad(
    page: Page,
    cdp: Awaited<ReturnType<typeof session>>,
    conditions?: { throughput: number; latency: number }
) {
    await cdp.send("Network.emulateNetworkConditions", {
        offline: false,
        latency: conditions?.latency ?? 0,
        downloadThroughput: conditions ? conditions.throughput : -1,
        uploadThroughput: conditions ? conditions.throughput : -1,
    });
    const started = Date.now();
    await page.goto("./");
    await expect(page.getByTestId("status")).toHaveAttribute("data-status", "ready", {
        timeout: 600_000,
    });
    return { ready_ms: Date.now() - started };
}

test.describe("M8 V11: what a deployment sends", () => {
    test("a first load with an empty cache, counted by the browser", async ({ page }) => {
        test.setTimeout(600_000);
        const { ready_ms } = await coldLoad(page, await session(page));

        const resources = await transfers(page);
        const totals = byCategory(resources);
        const total = resources.reduce((sum, resource) => sum + resource.transfer, 0);
        const lines = Object.entries(totals)
            .sort(([a], [b]) => a.localeCompare(b))
            .map(([name, bucket]) => `${name} ${bucket.transfer} (${bucket.count} files)`);
        console.log(`first load: ready in ${ready_ms} ms, ${total} bytes over the wire`);
        console.log(`first load by category: ${lines.join(" | ")}`);

        // Every byte counted crossed the socket: with the cache disabled a
        // served response reports its transfer size, and a zero here would
        // mean the figure describes a cache rather than a deployment.
        expect(resources.length).toBeGreaterThan(0);
        for (const resource of resources) {
            expect(resource.transfer).toBeGreaterThan(0);
        }
        // The server sends no Content-Encoding, so the wire bytes are the file
        // bytes plus headers. That is what the report says, rather than
        // implying a compressing host.
        for (const resource of resources) {
            expect(resource.transfer).toBeGreaterThanOrEqual(resource.encoded);
            expect(resource.encoded).toBe(resource.decoded);
        }
        expect(totals.wasm?.transfer ?? 0).toBeGreaterThan(1_000_000);
    });

    test("the same load over links that are not a loopback", async ({ page }) => {
        test.setTimeout(900_000);
        // Fixed profiles rather than named presets: naming the numbers is what
        // makes the result reproducible. Twelve is an ordinary broadband link
        // and three is a poor one; two points are what show whether this build
        // starts on bandwidth or on something else.
        const results: { mbit: number; ready_ms: number; bytes: number; wasm: number }[] = [];
        const cdp = await session(page);
        for (const mbit of [12, 3]) {
            const { ready_ms } = await coldLoad(page, cdp, {
                throughput: (mbit * 1_000_000) / 8,
                latency: 40,
            });
            const resources = await transfers(page);
            const bytes = resources.reduce((sum, resource) => sum + resource.transfer, 0);
            const wasm = byCategory(resources).wasm?.transfer ?? 0;
            results.push({ mbit, ready_ms, bytes, wasm });
            console.log(
                `throttled load: ${mbit} Mbit/s, 40 ms latency: ready in ${ready_ms} ms, ${bytes} bytes`
            );
            await expect
                .poll(
                    async () =>
                        (await page.evaluate(() => window.__property_workbench.snapshot())).region
                )
                .toBe("ready");
        }

        // The same build arrives over both: the wasm is the bulk of a first
        // load and its byte count is exact. The total is not asserted, because
        // a region fetches its font when it first draws - a moment that falls
        // on either side of `ready` depending on how slow the link is.
        expect(results[1].wasm).toBe(results[0].wasm);
        // A link four times slower makes a visibly slower start, which is what
        // says the emulation was in force and the figure is the link's rather
        // than the machine's. No budget is claimed by either number.
        expect(results[1].ready_ms).toBeGreaterThan(results[0].ready_ms * 1.5);
    });

    test("what the first Chinese glyph costs a deployment", async ({ page }) => {
        test.setTimeout(600_000);
        await coldLoad(page, await session(page));
        const before = await transfers(page);
        const fonts = (list: Transfer[]) => list.filter((r) => /\.ttf$/.test(new URL(r.name).pathname));
        // One Latin font on a first load: the rest of the shipped fonts are
        // fetched when something has to be drawn with them.
        expect(fonts(before).length).toBe(1);

        // A name in Chinese, drawn by the region rather than by a DOM control:
        // the browser's own control uses the page's font stack, so only the
        // region's drawing makes the runtime fetch a CJK face.
        await page.getByRole("textbox", { name: "name" }).fill("属性工作台");
        await expect
            .poll(async () => (await page.evaluate(() => window.__property_workbench.snapshot())).name)
            .toBe("属性工作台");

        await expect
            .poll(async () => fonts(await transfers(page)).length, { timeout: 120_000 })
            .toBeGreaterThan(1);
        const arrived = fonts(await transfers(page));
        const added = arrived.filter(
            (resource) => !before.some((seen) => seen.name === resource.name)
        );
        const bytes = added.reduce((sum, resource) => sum + resource.transfer, 0);
        console.log(
            `first CJK glyph: ${added.length} font file(s), ${bytes} bytes: ` +
                added.map((r) => `${new URL(r.name).pathname} ${r.transfer}`).join(", ")
        );

        // The cost of the first Chinese character is a fetch of a whole font,
        // and it is far larger than everything a first load transferred. That
        // is a deployment fact, not a defect, and the report states it.
        expect(added.length).toBeGreaterThan(0);
        expect(bytes).toBeGreaterThan(1_000_000);
    });
});
