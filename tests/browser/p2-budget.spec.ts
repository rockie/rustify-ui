import { CDPSession, expect, Page, test } from "@playwright/test";
import { percentile, R29, R30 } from "./budgets";
import { EVIDENCE } from "../tier";

/// R29 and R30, asserted rather than reported.
///
/// A-3 is closed: the PRD's B1 figures are an approved budget, so this file
/// fails when the build misses one. What each gate covers - and the three
/// things none of them cover - is in `budgets.ts`; read that before quoting a
/// pass from here.
///
/// Every figure is taken **inside the page**, and on this machine that is not
/// a nicety: a navigation costs about 10.9 seconds of wall clock here whatever
/// the page does, against a start-up of about 160 ms. Timed from the Playwright
/// process, this application's cold start was once reported as 9,916 ms. The
/// clock below starts at the navigation (`performance.now()` is relative to
/// `timeOrigin`) and stops in the page, on the frame the application reports
/// itself usable.

declare global {
    interface Window {
        __start_marks: {
            /// When the page's own script finished mounting the application.
            dom: number | null;
            /// When the region behind B1's main view reported itself ready.
            /// This is the one the gate uses: a workbench whose view has not
            /// arrived is not yet a workbench.
            region: number | null;
            /// Set when the page reported a failed start. Such a load is not a
            /// slow sample, it is a failure, and R29 AC3 asks for those to be
            /// counted apart rather than averaged in.
            failed: boolean;
        };
    }
}

/// Installed before any page script on every navigation, so the marks belong
/// to the load rather than to whatever the harness did afterwards.
const CLOCK = () => {
    window.__start_marks = { dom: null, region: null, failed: false };
    const poll = () => {
        const marks = window.__start_marks;
        const status = document.querySelector('[data-testid="status"]');
        const state = status?.getAttribute("data-status");
        if (state === "failed") {
            marks.failed = true;
            return;
        }
        if (marks.dom === null && state === "ready") {
            marks.dom = performance.now();
        }
        if (marks.dom !== null) {
            let region: string | undefined;
            try {
                region = window.__property_workbench?.snapshot().region;
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
    /// When the wasm module's bytes had all arrived, so that "waiting for the
    /// module" and "everything after it" can be told apart. A failing budget
    /// that cannot say which of the two it failed on is a number, not a
    /// finding.
    wasm_arrived: number;
    /// Bytes of the module that crossed the socket. Zero means the cache
    /// answered, which is the difference between a hot start and a cold one.
    transferred: number;
}

/// One load, with the cache in whatever state the caller set, waited out in
/// the page. Returns `null` for a start that failed.
async function load(page: Page): Promise<Start | null> {
    await page.goto("./", { waitUntil: "commit" });
    const marks = await page
        .waitForFunction(
            () => (window.__start_marks.region !== null || window.__start_marks.failed ? window.__start_marks : null),
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
            .find((entry) => entry.name.endsWith(".wasm")) as PerformanceResourceTiming | undefined;
        if (!entry) {
            return null;
        }
        return {
            responseEnd: entry.responseEnd,
            fetchStart: entry.fetchStart,
            requestStart: entry.requestStart,
            responseStart: entry.responseStart,
            connectStart: entry.connectStart,
            connectEnd: entry.connectEnd,
            transfer: entry.transferSize,
            encoded: entry.encodedBodySize,
        };
    });
    // A load that took seconds says where they went, in the browser's own
    // accounting. Without this the only thing a slow sample proves is that
    // something was slow.
    if (marks.region > 1_000 && wasm !== null) {
        const everything = await page.evaluate(() => {
            const nav = performance.getEntriesByType(
                "navigation"
            )[0] as PerformanceNavigationTiming;
            const lines = [
                `document: start ${nav.startTime.toFixed(0)} connect ${nav.connectStart.toFixed(0)}-` +
                    `${nav.connectEnd.toFixed(0)} request ${nav.requestStart.toFixed(0)} response ` +
                    `${nav.responseStart.toFixed(0)}-${nav.responseEnd.toFixed(0)} domInteractive ` +
                    `${nav.domInteractive.toFixed(0)}`,
            ];
            for (const entry of performance.getEntriesByType(
                "resource"
            ) as PerformanceResourceTiming[]) {
                lines.push(
                    `${new URL(entry.name).pathname}: fetchStart ${entry.fetchStart.toFixed(0)} ` +
                        `connect ${entry.connectStart.toFixed(0)}-${entry.connectEnd.toFixed(0)} ` +
                        `request ${entry.requestStart.toFixed(0)} response ` +
                        `${entry.responseStart.toFixed(0)}-${entry.responseEnd.toFixed(0)} ` +
                        `transferred ${entry.transferSize}`
                );
            }
            return lines;
        });
        console.log(`  slow load ${marks.region.toFixed(0)} ms:`);
        for (const line of everything) {
            console.log(`    ${line}`);
        }
    }
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
        `${kind}: ${samples.length} loads, p50 ${percentile(samples, 50).toFixed(0)} ms, ` +
            `p95 ${p95.toFixed(0)} ms, worst ${Math.max(...samples).toFixed(0)} ms, ` +
            `${failures} failed start(s)`
    );
    console.log(
        `  ${kind}, every load: ${[...samples].map((s) => s.toFixed(0)).join(",")}`
    );
    return p95;
}

/// Where a start-up went: waiting for the module's bytes, and everything
/// after them - compiling it, booting the runtime, building the view and the
/// region.
function split(kind: string, starts: Start[]) {
    const arrival = starts.map((start) => start.wasm_arrived);
    const after = starts.map((start) => start.region - start.wasm_arrived);
    console.log(
        `  ${kind}: module arrived by p50 ${percentile(arrival, 50).toFixed(0)} ms / ` +
            `p95 ${percentile(arrival, 95).toFixed(0)} ms; everything after it ` +
            `p50 ${percentile(after, 50).toFixed(0)} ms / p95 ${percentile(after, 95).toFixed(0)} ms`
    );
}

test.describe("R29: start-up and what a deployment sends", { tag: EVIDENCE }, () => {
    test(`AC1: ${R29.rounds} cold starts, p95 under ${R29.cold_p95_ms} ms`, async ({ browser }) => {
        test.setTimeout(3_600_000);
        // A context of its own for every load. Turning the HTTP cache off is
        // not enough to make a start cold: a compiled module is kept in a code
        // cache that the HTTP setting does not touch, and thirty loads in one
        // context measured that cache after the second of them. A context is
        // the boundary the browser actually resets.
        const samples: number[] = [];
        const dom: number[] = [];
        const starts: Start[] = [];
        let failures = 0;
        for (let round = 0; round < R29.rounds; round++) {
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
            `cold start, application mounted: p95 ${percentile(dom, 95).toFixed(0)} ms ` +
                `(the region's first frame follows it)`
        );
        const p95 = report("cold start, usable", samples, failures);
        split("cold start", starts);
        // A failed start is not a fast one. R29 AC3 says so about hot starts;
        // it is no less true here, so they are counted and none is tolerated.
        expect(failures).toBe(0);
        expect(samples).toHaveLength(R29.rounds);
        expect(p95).toBeLessThanOrEqual(R29.cold_p95_ms);
    });

    test(`AC3: ${R29.rounds} hot starts, p95 under ${R29.hot_p95_ms} ms`, async ({ browser }) => {
        test.setTimeout(3_600_000);
        // One context, so the cache carries from load to load - that is what
        // makes these hot - but a *page* of its own for each of them, closed
        // before the next one opens.
        //
        // Not tidiness: discarding this page costs the browser about ten
        // seconds on this machine, and when the next document's clock starts
        // before the previous one has finished dying, those seconds land in
        // the new page's timeline. Reloading in place measured that: three to
        // six loads in thirty came back at 10.5 s, and their own resource
        // timings showed the document received in 1 ms and nothing parsed for
        // the next ten seconds. It is not the application's teardown -
        // `dispose()` measured 7 to 17 ms in the page - and it is not the
        // module: the same stall appears before the module is even asked for.
        // The figure is in `docs/reports/p2/performance.md`; what is gated
        // here is the start-up of a page, not the death of its predecessor.
        const context = await browser.newContext();
        const samples: number[] = [];
        const starts: Start[] = [];
        let failures = 0;
        // Three loads to warm, not one: the browser keeps the module's bytes
        // after the first and its compiled form after the second, and "hot"
        // means both.
        for (let warm = 0; warm < 3; warm++) {
            const page = await context.newPage();
            await page.addInitScript(CLOCK);
            expect(await load(page)).not.toBeNull();
            await page.close();
        }

        for (let round = 0; round < R29.rounds; round++) {
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
        // A failed start is reported apart from the fast ones rather than
        // averaged in with them - R29 AC3 asks for exactly that.
        expect(failures).toBe(0);
        expect(samples).toHaveLength(R29.rounds);
        expect(p95).toBeLessThanOrEqual(R29.hot_p95_ms);
        // How much of the module the cache actually saved, recorded rather
        // than asserted. The server offers a validator and answers
        // `If-None-Match` with a 304 (checked in `xtask`'s own tests), and the
        // smaller files are served from the cache - but this browser profile
        // will not hold an eleven-megabyte entry, so the module is fetched
        // again every time. On a loopback that costs about 26 ms and the
        // figure above is still close to a hot start; on a real link it would
        // not be, and a reader of this number needs to know which of the two
        // they are looking at.
        const biggest = Math.max(...starts.map((start) => start.transferred));
        console.log(`  hot start: the module cost at most ${biggest} bytes on the wire`);
    });

    test(`AC2: a first load sends under ${R29.first_load_bytes} compressed bytes`, async ({
        page,
    }) => {
        test.setTimeout(600_000);
        await page.addInitScript(CLOCK);
        await cache(page, true);
        expect(await load(page)).not.toBeNull();

        // The budget is the *compressed* transfer, which is what a static host
        // sends and what R29 AC2 counts. This preview server sends no
        // `Content-Encoding`, so the compressed size is computed here, from the
        // very bytes the browser fetched, with the browser's own gzip. Reading
        // the directory instead would count files a first load never asks for
        // - the two CJK and emoji faces alone are 29 MB of them.
        const measured = await page.evaluate(async () => {
            const navigation = performance.getEntriesByType(
                "navigation"
            )[0] as PerformanceNavigationTiming;
            const resources = performance.getEntriesByType(
                "resource"
            ) as PerformanceResourceTiming[];
            const fetched = [navigation, ...resources].map((entry) => ({
                name: entry.name,
                transfer: entry.transferSize,
                encoded: entry.encodedBodySize,
            }));

            const gzipped = async (url: string) => {
                const response = await fetch(url, { cache: "no-store" });
                const body = await response.arrayBuffer();
                const stream = new Blob([body])
                    .stream()
                    .pipeThrough(new CompressionStream("gzip"));
                return (await new Response(stream).arrayBuffer()).byteLength;
            };

            const files: { path: string; wire: number; gzip: number }[] = [];
            for (const entry of fetched) {
                files.push({
                    path: new URL(entry.name).pathname,
                    wire: entry.transfer,
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
            `first load: ${named.length} files, ${gzip} bytes gzipped, ${wire} bytes uncompressed`
        );
        for (const [category, bucket] of [...totals].sort(([a], [b]) => a.localeCompare(b))) {
            console.log(
                `  ${category}: ${bucket.gzip} gzipped, ${bucket.wire} uncompressed, ` +
                    `${bucket.count} file(s)`
            );
        }

        // "漏计文件数为 0": every file the browser fetched landed in a named
        // category, so the categories account for the total rather than for
        // whatever happened to match.
        const uncounted = named.filter((file) => file.category === undefined);
        if (uncounted.length > 0) {
            console.log(`uncounted: ${uncounted.map((file) => file.path).join(", ")}`);
        }
        expect(uncounted).toEqual([]);
        expect(named.length).toBeGreaterThan(0);
        for (const file of named) {
            expect(file.gzip).toBeGreaterThan(0);
        }
        expect(gzip).toBeLessThanOrEqual(R29.first_load_bytes);
    });
});

test.describe("R30: latency at B1's density", { tag: EVIDENCE }, () => {
    test(`AC1: ${R30.actions} actions, p95 under ${R30.latency_p95_ms} ms`, async ({ page }) => {
        test.setTimeout(1_800_000);
        await page.addInitScript(CLOCK);
        expect(await load(page)).not.toBeNull();

        // Both directions across the boundary, alternating: a click the region
        // receives whose result the DOM has to show, and an edit the DOM
        // receives whose result the region has to draw. One of them alone
        // would measure half of what R30 asks about.
        //
        // An action is done when its result has been *presented*: the value is
        // the application's and a frame has been rendered since. Stopping the
        // clock when the value lands would leave out the frame the user waits
        // for; waiting for two frames would measure the display's clock.
        const outcome = await page.evaluate(
            async ({ actions, next }) => {
                const api = window.__property_workbench;
                const canvas = document.querySelector(
                    '[data-testid="workbench-gpu"]'
                ) as HTMLElement;
                const field = document.querySelector(
                    '[data-testid="name-input"]'
                ) as HTMLInputElement;
                const shown = () =>
                    document.querySelector('[data-testid="selected-id"]')?.textContent ?? "";
                const frame = () => new Promise((resolve) => requestAnimationFrame(resolve));
                const box = canvas.getBoundingClientRect();
                const at = { x: box.left + next.dx, y: box.top + box.height * next.fy };
                const click = () => {
                    for (const type of ["pointerdown", "pointerup"]) {
                        canvas.dispatchEvent(
                            new PointerEvent(type, {
                                bubbles: true,
                                cancelable: true,
                                clientX: at.x,
                                clientY: at.y,
                                pointerId: 1,
                                pointerType: "mouse",
                                button: 0,
                                buttons: type === "pointerdown" ? 1 : 0,
                            })
                        );
                    }
                };

                const before = api.snapshot();
                const taken: number[] = [];
                const selection: number[] = [];
                const edits: number[] = [];
                let stalled = -1;
                for (let round = 0; round < actions; round++) {
                    const across_the_boundary = round % 2 === 0;
                    const started = performance.now();
                    let waited = 0;
                    if (across_the_boundary) {
                        // The region's own "next" control, clicked where the
                        // region drew it: the input starts on the GPU side and
                        // the answer has to appear in the DOM.
                        const was = shown();
                        click();
                        while (shown() === was) {
                            await frame();
                            waited += 1;
                            if (waited > 180) {
                                stalled = round;
                                break;
                            }
                        }
                    } else {
                        const wanted = `n-${round}`;
                        field.value = wanted;
                        field.dispatchEvent(new Event("input", { bubbles: true }));
                        while (api.snapshot().name !== wanted) {
                            await frame();
                            waited += 1;
                            if (waited > 180) {
                                stalled = round;
                                break;
                            }
                        }
                    }
                    if (stalled >= 0) {
                        break;
                    }
                    // The frame that presents it.
                    await frame();
                    const cost = performance.now() - started;
                    taken.push(cost);
                    (across_the_boundary ? selection : edits).push(cost);
                }
                const after = api.snapshot();
                return {
                    taken,
                    selection,
                    edits,
                    stalled,
                    refused: after.refused - before.refused,
                    refusals: after.refusals - before.refusals,
                    errors: api.stats().errors,
                    region: after.region,
                };
            },
            { actions: R30.actions, next: { dx: 100, fy: 0.06 } }
        );

        const p95 = percentile(outcome.taken, 95);
        const p99 = percentile(outcome.taken, 99);
        console.log(
            `B1 latency: ${outcome.taken.length} actions, p50 ${percentile(outcome.taken, 50).toFixed(1)} ms, ` +
                `p95 ${p95.toFixed(1)} ms, p99 ${p99.toFixed(1)} ms, ` +
                `worst ${Math.max(...outcome.taken).toFixed(1)} ms`
        );
        console.log(
            `  region click to DOM: p95 ${percentile(outcome.selection, 95).toFixed(1)} ms; ` +
                `DOM edit to region: p95 ${percentile(outcome.edits, 95).toFixed(1)} ms`
        );

        // "错误动作数为 0": every action was taken, none was refused, and the
        // run left no error behind.
        expect(outcome.stalled).toBe(-1);
        expect(outcome.taken).toHaveLength(R30.actions);
        expect(outcome.refused).toBe(0);
        expect(outcome.refusals).toBe(0);
        expect(outcome.errors).toBe(0);
        expect(outcome.region).toBe("ready");

        expect(p95).toBeLessThanOrEqual(R30.latency_p95_ms);
        expect(p99).toBeLessThanOrEqual(R30.latency_p99_ms);
    });
});
