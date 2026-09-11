import { expect, Page, test } from "@playwright/test";
import * as twin from "./dataset";

/// M3 · the jobs: sorting, filtering, finding, and stopping.
///
/// A hundred thousand rows cannot be put in order between two frames, and
/// there is no thread to move the work to. So it is sliced, and what is
/// checked here is everything that being sliced makes possible to get wrong:
/// an answer that arrives after the question changed, a job that reads a row
/// that has moved, two jobs both delivering, and a cancel that leaves half a
/// view behind.
///
/// Every order is compared against one worked out in `dataset.ts` from the
/// definition rather than against what the application produced.

const api = (page: Page) => page.evaluate(() => window.__data_workbench.snapshot());

async function openTable(page: Page) {
    await page.setViewportSize({ width: 1440, height: 1200 });
    await page.goto("./table");
    await expect(page.getByTestId("status")).toHaveAttribute("data-status", "ready", {
        timeout: 120_000,
    });
    await expect(page.getByTestId("table-view")).toHaveCount(1);
}

/// Waits until no job is running, and says how the last one ended.
async function settled(page: Page): Promise<string | null> {
    await expect.poll(async () => (await api(page)).table.job.running, {
        timeout: 30_000,
    }).toBe(false);
    return (await api(page)).table.job.ended;
}

/// The view, in the terms the second implementation uses: sample positions, in
/// the order the table is showing them.
const view = (page: Page, from: number, count: number) =>
    page.evaluate(
        ([from, count]) => window.__data_workbench.view(from, count),
        [from, count] as const
    );

/// The whole view is a hundred thousand numbers; a comparison does not need
/// them in one message, and the ends and the middle are where an order goes
/// wrong.
async function sameOrder(page: Page, expected: number[], what: string) {
    const shown = (await api(page)).table.shown;
    expect(shown, `${what}: how many rows`).toBe(expected.length);
    for (const from of [0, Math.floor(expected.length / 2), Math.max(0, expected.length - 500)]) {
        const window = await view(page, from, 500);
        expect(window, `${what}: rows ${from}..`).toEqual(
            expected.slice(from, from + window.length)
        );
    }
}

test.describe("M3 · a hundred thousand rows, in order", () => {
    test("three columns, up and down, are the order a second implementation gives", async ({
        page,
    }) => {
        await openTable(page);
        for (const [column, ascending] of [
            [0, true],
            [7, true],
            [7, false],
            [19, true],
        ] as const) {
            await page.evaluate(
                ([column, ascending]) => window.__data_workbench.sort(column, ascending),
                [column, ascending] as const
            );
            expect(await settled(page)).toBe("done");
            await sameOrder(
                page,
                twin.sorted(column, ascending),
                `column ${column} ${ascending ? "up" : "down"}`
            );
            const state = (await api(page)).table;
            expect(state.sorted).toBe(`${column}:${ascending ? "asc" : "desc"}`);
            expect(state.stale, "a view just built is not stale").toBe(false);
        }
    });

    test("a heading sorts the column it names, and says which way", async ({ page }) => {
        await openTable(page);
        const heading = page.getByTestId("table-column-2").getByRole("button");
        await heading.click();
        expect(await settled(page)).toBe("done");
        await expect(page.getByTestId("table-column-2")).toHaveAttribute("aria-sort", "ascending");
        await sameOrder(page, twin.sorted(2, true), "column 2 by its heading");

        // The same heading again turns it round, which is what every other
        // grid does.
        await heading.click();
        expect(await settled(page)).toBe("done");
        await expect(page.getByTestId("table-column-2")).toHaveAttribute("aria-sort", "descending");
        await sameOrder(page, twin.sorted(2, false), "column 2 turned round");

        // And a column nobody sorted says so rather than saying nothing.
        await expect(page.getByTestId("table-column-3")).toHaveAttribute("aria-sort", "none");
    });

    test("five filters keep the rows a second implementation keeps", async ({ page }) => {
        await openTable(page);
        for (const needle of ["QQ", "ZZZ", "A0A", "7", "QZXQZX"]) {
            await page.getByTestId("table-filter").fill(needle);
            await page.getByTestId("table-filter").press("Enter");
            expect(await settled(page)).toBe("done");
            const expected = twin.filtered(needle);
            await sameOrder(page, expected, `filter ${needle}`);
            if (expected.length === 0) {
                await expect(page.getByTestId("table-empty")).toHaveCount(1);
            }
        }
    });

    test("a group in the tree is a filter over where the rows are", async ({ page }) => {
        await openTable(page);
        await page.getByTestId("table-tree-g0-3").click();
        expect(await settled(page)).toBe("done");
        await sameOrder(page, twin.grouped(0, 3), "group 1.4");
        expect((await api(page)).table.group).toBe("g0-3");

        // A second group replaces the first rather than narrowing it: the tree
        // is for getting about, not for building a query.
        await page.getByTestId("table-tree-g0-7").click();
        expect(await settled(page)).toBe("done");
        await sameOrder(page, twin.grouped(0, 7), "group 1.8");
    });

    test("find reaches the first, the middle and the last sample", async ({ page }) => {
        await openTable(page);
        // Three needles from three places in the sample, each of them the
        // sixteen characters of one cell, so each has exactly one row.
        const targets = [0, 50_000, twin.ROWS - 1].map((row) => ({
            row,
            needle: twin.cell(row, 5),
        }));
        for (const { row, needle } of targets) {
            await page.getByTestId("table-find").fill(needle);
            await page.getByTestId("table-find").press("Enter");
            expect(await settled(page)).toBe("done");
            const at = twin.foundIn(
                Array.from({ length: twin.ROWS }, (_, index) => index),
                needle
            );
            expect(at, `${needle} is somewhere`).toBeGreaterThanOrEqual(0);
            expect(at, `${needle} is row ${row}`).toBe(row);
            // The table went there: the row it found is drawn, and the
            // keyboard is on it.
            const drawn = await page.evaluate(() =>
                Array.from(
                    document.querySelectorAll('[data-testid="table"] [role="row"][data-row-id]')
                ).map((element) => Number(element.getAttribute("aria-rowindex")))
            );
            expect(drawn, `${needle} is on screen`).toContain(at + 1);
        }
    });

    test("a find inside a sorted view answers where the sorted view has it", async ({ page }) => {
        await openTable(page);
        await page.evaluate(() => window.__data_workbench.sort(3, true));
        expect(await settled(page)).toBe("done");
        const needle = twin.cell(12_345, 5);
        await page.getByTestId("table-find").fill(needle);
        await page.getByTestId("table-find").press("Enter");
        expect(await settled(page)).toBe("done");
        const at = twin.foundIn(twin.sorted(3, true), needle);
        const drawn = await page.evaluate(() =>
            Array.from(
                document.querySelectorAll('[data-testid="table"] [role="row"][data-row-id]')
            ).map((element) => Number(element.getAttribute("aria-rowindex")))
        );
        expect(drawn).toContain(at + 1);
    });
});

test.describe("M3 · stopping a job", () => {
    test("twenty cancels each answer inside a tenth of a second", async ({ page }) => {
        await openTable(page);
        const times: number[] = [];
        for (let round = 0; round < 20; round++) {
            const ms = await page.evaluate(async (column) => {
                const api = window.__data_workbench;
                api.sort(column, true);
                // Timed inside the page, from the click to the frame that
                // shows it: a round trip out to the test process costs more
                // than the budget being measured.
                const started = performance.now();
                api.cancel_job();
                await new Promise((resolve) => requestAnimationFrame(resolve));
                return performance.now() - started;
            }, round % 20);
            times.push(ms);
            expect((await api(page)).table.job.ended).toBe("cancelled");
        }
        times.sort((a, b) => a - b);
        const p95 = times[Math.min(times.length - 1, Math.floor(times.length * 0.95))];
        console.log(`  cancel to shown: p50 ${times[10].toFixed(1)} ms, p95 ${p95.toFixed(1)} ms`);
        expect(p95).toBeLessThan(100);
    });

    test("a cancelled job leaves the view it was going to replace", async ({ page }) => {
        await openTable(page);
        const before = await view(page, 0, 200);
        await page.evaluate(() => {
            window.__data_workbench.sort(4, true);
            window.__data_workbench.cancel_job();
        });
        expect(await settled(page)).toBe("cancelled");
        expect(await view(page, 0, 200)).toEqual(before);
        expect((await api(page)).table.shown).toBe(twin.ROWS);
    });

    test("only the newest of several jobs takes effect", async ({ page }) => {
        await openTable(page);
        // Four sorts in one turn of the browser: the first three never get a
        // slice, and the order that arrives is the last one's.
        await page.evaluate(() => {
            const api = window.__data_workbench;
            api.sort(1, true);
            api.sort(2, true);
            api.sort(3, true);
            api.sort(6, false);
        });
        expect(await settled(page)).toBe("done");
        await sameOrder(page, twin.sorted(6, false), "the last sort asked for");
        expect((await api(page)).table.sorted).toBe("6:desc");
    });
});

test.describe("M3 · a write while a job is running", () => {
    /// Starts a sort, waits until it has actually done a slice, and then does
    /// the write - from inside the page, because a write driven from out here
    /// arrives whenever the round trip gets round to it, and what is under
    /// test is a write that lands in the middle of a job.
    ///
    /// Returns whether the job was still running when the write landed. The
    /// checks assert it: a write that arrived after the job finished would be
    /// a different thing entirely, and it would pass for the wrong reason.
    async function sortThen(page: Page, write: "insert" | "delete" | "edit"): Promise<boolean> {
        return page.evaluate(async (write) => {
            const api = window.__data_workbench;
            const press = (id: string) =>
                (document.querySelector(`[data-testid="${id}"]`) as HTMLElement).click();
            api.sort(9, true);
            await new Promise<void>((resolve) => {
                const wait = () => {
                    const job = api.snapshot().table.job;
                    if (!job.running || job.done > 0) {
                        resolve();
                    } else {
                        setTimeout(wait, 0);
                    }
                };
                setTimeout(wait, 0);
            });
            const running = api.snapshot().table.job.running;
            if (write === "edit") {
                // Typed through the element rather than through the keyboard:
                // the keyboard path is p3-table's, and this is about when the
                // write lands rather than about how it was made.
                const field = document.querySelector(
                    '[data-testid="table-detail-9"]'
                ) as HTMLInputElement;
                field.value = "0000000000000000";
                field.dispatchEvent(new Event("input", { bubbles: true }));
                press("table-detail-submit");
            } else {
                press(`table-${write}`);
            }
            return running;
        }, write);
    }

    test("an insert makes the job stale, and it is asked again", async ({ page }) => {
        await openTable(page);
        expect(await sortThen(page, "insert")).toBe(true);
        expect(await settled(page)).toBe("done");
        // Stale, then asked again with the version it has now - and the answer
        // that arrives is an order over the rows that are there now.
        expect((await api(page)).rows).toBe(twin.ROWS + 10);
        expect((await api(page)).table.shown).toBe(twin.ROWS + 10);
        expect((await api(page)).table.stale).toBe(false);
        const entries = await page.evaluate(() =>
            window.__data_workbench
                .diagnostics()
                .entries.filter((entry) => entry.kind === "JobCancelled")
        );
        expect(entries.length, "the dropped answer is recorded").toBeGreaterThan(0);
        expect(entries.every((entry) => entry.severity === "info")).toBe(true);
    });

    test("a delete makes the job stale, and nothing reads a row that has gone", async ({
        page,
    }) => {
        await openTable(page);
        const errors: string[] = [];
        page.on("pageerror", (error) => errors.push(String(error)));
        expect(await sortThen(page, "delete")).toBe(true);
        expect(await settled(page)).toBe("done");
        expect((await api(page)).rows).toBe(twin.ROWS - 10);
        expect((await api(page)).table.shown).toBe(twin.ROWS - 10);
        expect(errors, "a row read after it moved would trap here").toEqual([]);
    });

    test("an edit makes the job stale too, because a sort reads cell values", async ({ page }) => {
        await openTable(page);
        await page.evaluate(() => window.__data_workbench.open_row(4));
        expect(await sortThen(page, "edit")).toBe(true);
        expect(await settled(page)).toBe("done");
        expect((await api(page)).table.stale).toBe(false);
        // And the re-run sorted the value that was written, not the one that
        // was there when the first job started. A cell of zeros is the
        // smallest key the sample's alphabet has - digits come before letters
        // in byte order - so the edited row is first.
        expect(await view(page, 0, 1)).toEqual([4]);
    });

    test("a view built before a write is marked stale until a job rebuilds it", async ({
        page,
    }) => {
        await openTable(page);
        await page.evaluate(() => window.__data_workbench.sort(11, true));
        expect(await settled(page)).toBe("done");
        await expect(page.getByTestId("table-stale")).toHaveCount(0);

        // A write with nothing running: there is no job to make stale, so the
        // view stands and says that it is behind.
        await page.getByTestId("table-insert").click();
        await expect(page.getByTestId("table-stale")).toHaveCount(1);
        expect((await api(page)).table.stale).toBe(true);
        // The new rows are in the view, at the end, where they can be seen.
        expect((await api(page)).table.shown).toBe(twin.ROWS + 10);

        await page.evaluate(() => window.__data_workbench.sort(11, true));
        expect(await settled(page)).toBe("done");
        await expect(page.getByTestId("table-stale")).toHaveCount(0);
    });
});

test.describe("M3 · what a sort costs", () => {
    test("twenty sorts of the whole sample, timed in the page", async ({ page }) => {
        await openTable(page);
        const runs: { ms: number; slices: number }[] = [];
        for (let round = 0; round < 20; round++) {
            const run = await page.evaluate(async (column) => {
                const api = window.__data_workbench;
                const started = performance.now();
                api.sort(column, true);
                await new Promise<void>((resolve) => {
                    const wait = () => {
                        if (api.snapshot().table.job.running) {
                            requestAnimationFrame(wait);
                        } else {
                            resolve();
                        }
                    };
                    requestAnimationFrame(wait);
                });
                return {
                    ms: performance.now() - started,
                    slices: api.snapshot().table.job.slices,
                };
            }, round % 20);
            runs.push(run);
        }
        const times = runs.map((run) => run.ms).sort((a, b) => a - b);
        const slices = runs.map((run) => run.slices).sort((a, b) => a - b);
        const p95 = times[Math.floor(times.length * 0.95)];
        // Measured, not yet a gate: the gate is M8's, on its own project.
        console.log(
            `  sort of ${twin.ROWS} rows: p50 ${times[10].toFixed(0)} ms, ` +
                `p95 ${p95.toFixed(0)} ms, worst ${times[times.length - 1].toFixed(0)} ms; ` +
                `slices ${slices[0]}-${slices[slices.length - 1]}, median ${slices[10]}`
        );
        // Every one of them was sliced: a job that took one slice is a job
        // that held the frame for as long as it took.
        expect(slices[0]).toBeGreaterThan(1);
        expect(times.every((ms) => ms > 0)).toBe(true);
    });
});
