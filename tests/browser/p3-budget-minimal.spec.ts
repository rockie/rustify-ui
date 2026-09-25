import { CDPSession, expect, Page, test } from "@playwright/test";
import { percentile, R29_B0 } from "./budgets";
import { EVIDENCE } from "../tier";

/// R29's B0 column, asserted.
///
/// The same three gates `p2-budget.spec.ts` applies to B1, applied to the load
/// they were actually written for. R29 gives B0 its own figures because B0 is
/// the smallest complete application and the first load is where that shows:
/// three compressed megabytes is the whole budget, and B1 spends 2.77 of them
/// on its module alone. A build that passes B1's 8 MiB says nothing about it.
///
/// Every figure is taken **inside the page**. A navigation costs about 10.9
/// seconds of wall clock on this machine whatever the page does, against a
/// start-up of about 200 ms; timed from the Playwright process this
/// application's cold start would be reported as ten seconds of harness.

declare global {
    interface Window {
        __b0_marks: {
            /// When the page's own script finished mounting B0.
            dom: number | null;
            /// When B0's region reported itself ready. This is the one the
            /// gate uses: the twenty GPU controls are half of the load, and an
            /// application showing only the other half is not started.
            region: number | null;
            failed: boolean;
        };
    }
}

/// Installed before any page script on every navigation, so the marks belong
/// to the load and not to whatever the harness did afterwards.
const CLOCK = () => {
    window.__b0_marks = { dom: null, region: null, failed: false };
    const poll = () => {
        const marks = window.__b0_marks;
        const state = document
            .querySelector('[data-testid="status"]')
            ?.getAttribute("data-status");
        if (state === "failed" || state === "fatal") {
            marks.failed = true;
            return;
        }
        if (marks.dom === null && state === "ready") {
            marks.dom = performance.now();
        }
        if (marks.dom !== null) {
            let region: string | undefined;
            try {
                region = window.__fusion_basic?.b0_state().region;
            } catch {
                region = undefined;
            }
            if (region === "ready") {
                marks.region = performance.now();
                return;
            }
        }
        requestAnimationFrame(poll);
    };
    requestAnimationFrame(poll);
};

interface Start {
    dom: number;
    region: number;
    /// When the module's bytes had all arrived, so that "waiting for the
    /// module" and "everything after it" can be told apart. A failing budget
    /// that cannot say which of the two it missed is a number, not a finding.
    wasm_arrived: number;
    transferred: number;
}

async function load(page: Page): Promise<Start | null> {
    await page.goto("./", { waitUntil: "commit" });
    const marks = await page
        .waitForFunction(
            () =>
                window.__b0_marks.region !== null || window.__b0_marks.failed
                    ? window.__b0_marks
                    : null,
            null,
            { timeout: 300_000 }
        )
        .then((handle) => handle.jsonValue());
    if (marks.failed || marks.region === null || marks.dom === null) {
        return null;
    }
    const wasm = await page.evaluate(() => {
        const entry = performance
            .getEntriesByType("resource")
            .find((entry) => entry.name.endsWith(".wasm")) as
            | PerformanceResourceTiming
            | undefined;
        return entry ? { responseEnd: entry.responseEnd, transfer: entry.transferSize } : null;
    });
    return {
        dom: marks.dom,
        region: marks.region,
        wasm_arrived: wasm?.responseEnd ?? 0,
        transferred: wasm?.transfer ?? 0,
    };
}

async function cache(page: Page, disabled: boolean): Promise<CDPSession> {
    const cdp = await page.context().newCDPSession(page);
    await cdp.send("Network.enable");
    await cdp.send("Network.setCacheDisabled", { cacheDisabled: disabled });
    return cdp;
}

function report(kind: string, samples: number[], failures: number) {
    const p95 = percentile(samples, 95);
    console.log(
        `B0 ${kind}: ${samples.length} loads, p50 ${percentile(samples, 50).toFixed(0)} ms, ` +
            `p95 ${p95.toFixed(0)} ms, worst ${Math.max(...samples).toFixed(0)} ms, ` +
            `${failures} failed start(s)`
    );
    console.log(`  B0 ${kind}, every load: ${samples.map((s) => s.toFixed(0)).join(",")}`);
    return p95;
}

function split(kind: string, starts: Start[]) {
    const arrival = starts.map((start) => start.wasm_arrived);
    const after = starts.map((start) => start.region - start.wasm_arrived);
    console.log(
        `  B0 ${kind}: module arrived by p50 ${percentile(arrival, 50).toFixed(0)} ms / ` +
            `p95 ${percentile(arrival, 95).toFixed(0)} ms; everything after it ` +
            `p50 ${percentile(after, 50).toFixed(0)} ms / p95 ${percentile(after, 95).toFixed(0)} ms`
    );
}

test.describe("R29 B0: the smallest complete application, started", { tag: EVIDENCE }, () => {
    test(`AC1: ${R29_B0.rounds} cold starts, p95 under ${R29_B0.cold_p95_ms} ms`, async ({
        browser,
    }) => {
        test.setTimeout(3_600_000);
        // A context per load. Disabling the HTTP cache is not enough to make a
        // start cold: the compiled module lives in a code cache the HTTP
        // setting does not reach, and thirty loads in one context would
        // measure that cache from the second one on.
        const samples: number[] = [];
        const dom: number[] = [];
        const starts: Start[] = [];
        let failures = 0;
        for (let round = 0; round < R29_B0.rounds; round++) {
            const context = await browser.newContext();
            const page = await context.newPage();
            await page.addInitScript(CLOCK);
            await cache(page, true);
            const start = await load(page);
            if (start === null) {
                failures += 1;
            } else {
                samples.push(start.region);
                dom.push(start.dom);
                starts.push(start);
            }
            await context.close();
        }

        console.log(
            `B0 cold start, application mounted: p95 ${percentile(dom, 95).toFixed(0)} ms ` +
                `(the region's first frame follows it)`
        );
        const p95 = report("cold start, usable", samples, failures);
        split("cold start", starts);
        // A failed start is not a fast one.
        expect(failures).toBe(0);
        expect(samples).toHaveLength(R29_B0.rounds);
        expect(p95).toBeLessThanOrEqual(R29_B0.cold_p95_ms);
    });

    test(`AC3: ${R29_B0.rounds} hot starts, p95 under ${R29_B0.hot_p95_ms} ms`, async ({
        browser,
    }) => {
        test.setTimeout(3_600_000);
        // One context so the caches carry across loads - that is what makes
        // these hot - but a page of its own for each, closed before the next
        // opens. Reloading in place puts the previous document's death, which
        // costs about ten seconds here, inside the next document's timeline.
        const context = await browser.newContext();
        const samples: number[] = [];
        const starts: Start[] = [];
        let failures = 0;
        // Three to warm, not one: the browser keeps the module's bytes after
        // the first load and its compiled form after the second, and "hot"
        // means both.
        for (let warm = 0; warm < 3; warm++) {
            const page = await context.newPage();
            await page.addInitScript(CLOCK);
            expect(await load(page)).not.toBeNull();
            await page.close();
        }
        for (let round = 0; round < R29_B0.rounds; round++) {
            const page = await context.newPage();
            await page.addInitScript(CLOCK);
            const start = await load(page);
            if (start === null) {
                failures += 1;
            } else {
                samples.push(start.region);
                starts.push(start);
            }
            await page.close();
        }
        await context.close();

        const p95 = report("hot start, usable", samples, failures);
        split("hot start", starts);
        expect(failures).toBe(0);
        expect(samples).toHaveLength(R29_B0.rounds);
        expect(p95).toBeLessThanOrEqual(R29_B0.hot_p95_ms);
        const biggest = Math.max(...starts.map((start) => start.transferred));
        console.log(`  B0 hot start: the module cost at most ${biggest} bytes on the wire`);
    });

    test(`AC2: a first load sends under ${R29_B0.first_load_bytes} compressed bytes`, async ({
        page,
    }) => {
        test.setTimeout(600_000);
        await page.addInitScript(CLOCK);
        await cache(page, true);
        expect(await load(page)).not.toBeNull();

        // The budget is the *compressed* transfer, which is what a static host
        // sends and what AC2 counts. This preview server sends no
        // `Content-Encoding`, so the compressed size is computed here from the
        // bytes the browser actually fetched, with the browser's own gzip.
        // Reading the build directory instead would count files a first load
        // never asks for - the CJK and emoji faces alone are 29 MB of them.
        const measured = await page.evaluate(async () => {
            const navigation = performance.getEntriesByType(
                "navigation"
            )[0] as PerformanceNavigationTiming;
            const resources = performance.getEntriesByType(
                "resource"
            ) as PerformanceResourceTiming[];
            const gzipped = async (url: string) => {
                const response = await fetch(url, { cache: "no-store" });
                const body = await response.arrayBuffer();
                const stream = new Blob([body])
                    .stream()
                    .pipeThrough(new CompressionStream("gzip"));
                return (await new Response(stream).arrayBuffer()).byteLength;
            };
            const files: { path: string; wire: number; gzip: number }[] = [];
            for (const entry of [navigation, ...resources]) {
                files.push({
                    path: new URL(entry.name).pathname,
                    wire: entry.transferSize,
                    gzip: await gzipped(entry.name),
                });
            }
            return files;
        });

        const CATEGORIES: [string, RegExp][] = [
            ["wasm", /\.wasm$/],
            ["js", /\.m?js$/],
            ["css", /\.css$/],
            ["fonts", /\.(ttf|otf|woff2?)$/],
            ["images", /\.(png|jpe?g|svg|webp|gif)$/],
            ["data", /\.json$/],
            ["document", /(\/|\.html)$/],
        ];
        const named = measured.map((file) => ({
            ...file,
            category: CATEGORIES.find(([, pattern]) => pattern.test(file.path))?.[0],
        }));
        const totals = new Map<string, { gzip: number; wire: number; count: number }>();
        for (const file of named) {
            const bucket = totals.get(file.category ?? "uncounted") ?? {
                gzip: 0,
                wire: 0,
                count: 0,
            };
            bucket.gzip += file.gzip;
            bucket.wire += file.wire;
            bucket.count += 1;
            totals.set(file.category ?? "uncounted", bucket);
        }
        const gzip = named.reduce((sum, file) => sum + file.gzip, 0);
        const wire = named.reduce((sum, file) => sum + file.wire, 0);
        console.log(
            `B0 first load: ${named.length} files, ${gzip} bytes gzipped, ` +
                `${wire} bytes uncompressed`
        );
        for (const [category, bucket] of [...totals].sort(([a], [b]) => a.localeCompare(b))) {
            console.log(
                `  ${category}: ${bucket.gzip} gzipped, ${bucket.wire} uncompressed, ` +
                    `${bucket.count} file(s)`
            );
        }

        // Every file the browser fetched landed in a named category, so the
        // categories account for the total rather than for whatever happened
        // to match: R29 AC2 asks for a miscount of zero.
        const uncounted = named.filter((file) => file.category === undefined);
        if (uncounted.length > 0) {
            console.log(`uncounted: ${uncounted.map((file) => file.path).join(", ")}`);
        }
        expect(uncounted).toEqual([]);
        expect(named.length).toBeGreaterThan(0);
        for (const file of named) {
            expect(file.gzip).toBeGreaterThan(0);
        }
        // B0 takes one Latin face on the first screen; a build that started
        // pulling the CJK faces at load would blow this budget by ten times,
        // which is the failure this figure is really guarding against.
        expect(gzip).toBeLessThanOrEqual(R29_B0.first_load_bytes);
    });
});
