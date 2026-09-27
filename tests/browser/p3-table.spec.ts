import { expect, Locator, Page } from "@playwright/test";
import { LOCATORS } from "./loads";
import * as twin from "./dataset";
import { isShared, test, waitForQuiet } from "./support";
import { rounds } from "../tier";

/// M2 · the table, the groups over it and the row being edited.
///
/// What is checked here is identity and reach: that a hundred thousand rows
/// can all be got to, that the ones on screen are a window rather than the
/// whole thing, and that a selection is of things rather than of positions -
/// so deleting a row deselects that row and nothing else.

const api = (page: Page) => page.evaluate(() => window.__data_workbench.snapshot());

/// The part of the example's handle that puts a shared page back.
type Resettable = { reset(path?: string): Promise<void> };

/// Brings the page to `path` as a first load of it would: a load of its own
/// for a page of its own, and the example's reset for the shared one, which
/// is already loaded and has its regions running.
async function arrive(page: Page, path: string) {
    if (isShared(page)) {
        await page.evaluate(
            (path) => (window.__data_workbench as unknown as Resettable).reset(path),
            path
        );
    } else {
        await page.goto(`.${path}`);
    }
}

async function openTable(page: Page) {
    await page.setViewportSize({ width: 1440, height: 1200 });
    await arrive(page, "/table");
    await expect(page.getByTestId("status")).toHaveAttribute("data-status", "ready", {
        timeout: 120_000,
    });
    await expect(page.getByTestId("table-view")).toHaveCount(1);
    await expect(page.getByTestId("table")).toHaveAttribute(
        "aria-rowcount",
        String(twin.ROWS + 1)
    );
}

/// The row elements that are showing a row, and the row each one is showing.
async function drawnRows(page: Page): Promise<{ index: number; id: number }[]> {
    return page.evaluate(() =>
        Array.from(document.querySelectorAll('[data-testid="table"] [role="row"]'))
            .filter((row) => !row.classList.contains("hidden") && row.hasAttribute("data-row-id"))
            .map((row) => ({
                index: Number(row.getAttribute("aria-rowindex")) - 1,
                id: Number(row.getAttribute("data-row-id")),
            }))
    );
}

/// Sends the table to a row through the page's own control, the way a person
/// would, and waits until it is showing.
async function gotoRow(page: Page, row: number) {
    await page.getByTestId("table-goto").fill(String(row));
    await page.getByTestId("table-goto").press("Enter");
    await expect
        .poll(async () => (await drawnRows(page)).some((drawn) => drawn.index === row))
        .toBe(true);
}

/// A cell by the two indices a person and a screen reader read it by. Not by
/// position among the cells that happen to be drawn: the table windows across
/// as well as down, so the tenth cell in the document is not column ten.
function cellOf(page: Page, row: number, column: number): Locator {
    return page.locator(
        `[data-testid="table"] [role="row"][aria-rowindex="${row + 1}"] [role="gridcell"][aria-colindex="${column + 1}"]`
    );
}

test.describe("M2 · a hundred thousand rows", () => {
    test("the first, the middle and the last row are all reachable", async ({ page }) => {
        await openTable(page);
        for (const row of [1, 50_000, twin.ROWS]) {
            await gotoRow(page, row);
            const drawn = (await drawnRows(page)).find((entry) => entry.index === row)!;
            // The row number a person reads and the identity the application
            // holds are both right, and they are about the same row.
            expect(drawn.id).toBe(twin.rowId(row - 1));
            await expect(cellOf(page, row, 0)).toHaveText(twin.cell(row - 1, 0).trimEnd());
        }
    });

    test("what is in the document is a window, not the table", async ({ page }) => {
        await openTable(page);
        await gotoRow(page, 50_000);
        const drawn = await drawnRows(page);
        // Sixty visible rows and ten of overscan either side; the pool does
        // not grow with the table.
        expect(drawn.length).toBeLessThanOrEqual(60 + 20);
        expect(drawn.length).toBeGreaterThan(20);
        const cells = await page.locator('[data-testid="table"] [role="gridcell"]').count();
        expect(cells).toBeLessThanOrEqual(drawn.length * (12 + 2 * 2));
    });

    test("a hundred sampled cells say what the second implementation says", async ({ page }) => {
        await openTable(page);
        const sampled = await page.evaluate(() => {
            const api = window.__data_workbench;
            const out: { row: number; column: number; text: string; id: number }[] = [];
            for (let i = 0; i < 100; i++) {
                const row = (i * 977) % 100_000;
                const column = (i * 3) % 20;
                out.push({ row, column, text: api.cell(row, column), id: api.row_id(row) });
            }
            return out;
        });
        for (const entry of sampled) {
            expect(entry.text).toBe(twin.cell(entry.row, entry.column));
            expect(entry.id).toBe(twin.rowId(entry.row));
        }
    });

    test("the keyboard walks the grid and reaches both ends of it", async ({ page }) => {
        await openTable(page);
        await cellOf(page, 1, 0).click();
        await expect(cellOf(page, 1, 0)).toBeFocused();
        await page.keyboard.press("ArrowDown");
        await page.keyboard.press("ArrowRight");
        await expect(cellOf(page, 2, 1)).toBeFocused();
        await page.keyboard.press("End");
        await expect(cellOf(page, 2, 19)).toBeFocused();
        // Ctrl+End is the last cell of the last row, which is a row the table
        // has to scroll to before it can put the keyboard on it.
        await page.keyboard.press("ControlOrMeta+End");
        await expect
            .poll(async () => (await drawnRows(page)).some((drawn) => drawn.index === twin.ROWS))
            .toBe(true);
        await expect(cellOf(page, twin.ROWS, 19)).toBeFocused();
        await page.keyboard.press("ControlOrMeta+Home");
        await expect
            .poll(async () => (await drawnRows(page)).some((drawn) => drawn.index === 1))
            .toBe(true);
        await expect(cellOf(page, 1, 0)).toBeFocused();
    });

    test("a row opened from the keyboard can be edited and reads back the same", async ({ page }) => {
        await openTable(page);
        await gotoRow(page, 50_000);
        await cellOf(page, 50_000, 0).click();
        await page.keyboard.press("Enter");
        await expect(page.getByTestId("table-detail-row")).toHaveText("row 50000");
        const field = page.getByTestId("table-detail-2");
        await expect(field).toHaveValue(twin.cell(49_999, 2));
        await field.fill("EDITED");
        await page.getByTestId("table-detail-submit").click();
        await expect.poll(async () => (await api(page)).table.saves).toBe(1);
        // Away and back, so what is read is the sample rather than the draft
        // still sitting in the control.
        await gotoRow(page, 1);
        await gotoRow(page, 50_000);
        await expect(cellOf(page, 50_000, 2)).toHaveText("EDITED");
        expect((await api(page)).version).toBeGreaterThan(0);
    });

    test("a selection is of rows, and deleting one deselects that one only", async ({ page }) => {
        await openTable(page);
        const chosen = await page.evaluate(() => window.__data_workbench.select_rows(0, 10));
        expect(chosen).toEqual([1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
        await expect.poll(async () => (await api(page)).table.selected).toBe(10);

        // Ten more rows inserted at the top: every selected identity survives,
        // and none of the new rows is selected. The keyboard goes to the top
        // row rather than clicking it, because a click on a row is how a
        // person selects one and this is about where the insert happens.
        await gotoRow(page, 1);
        await cellOf(page, 1, 0).focus();
        await page.getByTestId("table-insert").click();
        await expect.poll(async () => (await api(page)).rows).toBe(twin.ROWS + 10);
        expect(await page.evaluate(() => window.__data_workbench.selected())).toEqual(chosen);

        // Now delete ten rows from the top - which are the ten just inserted -
        // and the selection is untouched.
        await page.getByTestId("table-delete").click();
        await expect.poll(async () => (await api(page)).rows).toBe(twin.ROWS);
        expect(await page.evaluate(() => window.__data_workbench.selected())).toEqual(chosen);

        // And now ten that *are* selected. Three of them stay selected only if
        // the selection is of positions; it is of identities, so all ten go.
        await page.getByTestId("table-delete").click();
        await expect.poll(async () => (await api(page)).rows).toBe(twin.ROWS - 10);
        expect(await page.evaluate(() => window.__data_workbench.selected())).toEqual([]);
        await expect.poll(async () => (await api(page)).table.selected).toBe(0);
    });

    test("deleting three of ten selected leaves seven selected", async ({ page }) => {
        await openTable(page);
        const chosen = await page.evaluate(() =>
            window.__data_workbench.select_rows(500, 10)
        );
        expect(chosen).toHaveLength(10);

        // Ten rows go, of which three were selected: rows 508, 509 and 510 by
        // the numbers a person reads. A selection of positions would end up
        // with ten again, shifted onto rows that were never chosen.
        await gotoRow(page, 508);
        await cellOf(page, 508, 0).focus();
        await page.evaluate(() => window.__data_workbench.open_row(507));
        await page.getByTestId("table-delete").click();
        await expect.poll(async () => (await api(page)).rows).toBe(twin.ROWS - 10);
        expect(await page.evaluate(() => window.__data_workbench.selected())).toEqual(
            chosen.slice(0, 7)
        );
        let state = (await api(page)).table;
        expect(state.selected).toBe(7);
        // Nothing filters yet, so every surviving identity is in view: a
        // deleted identity is gone, not hidden.
        expect(state.hidden).toBe(0);

        // And the other seven. Their rows have moved up under them, so this
        // deletes from where the first of them now is.
        await gotoRow(page, 501);
        await cellOf(page, 501, 0).focus();
        await page.getByTestId("table-delete").click();
        await expect.poll(async () => (await api(page)).rows).toBe(twin.ROWS - 20);
        expect(await page.evaluate(() => window.__data_workbench.selected())).toEqual([]);
        state = (await api(page)).table;
        expect(state.selected).toBe(0);
        expect(state.hidden).toBe(0);
    });

    test("the tree opens, closes and walks from the keyboard", async ({ page }) => {
        await openTable(page);
        const tree = page.getByTestId("table-tree");
        await expect(tree).toHaveAttribute("role", "tree");
        // The first group starts open, so its children are in the document and
        // the others' are not.
        await expect(page.getByTestId("table-tree-g0")).toHaveAttribute("aria-expanded", "true");
        await expect(page.getByTestId("table-tree-g0-0")).toHaveCount(1);
        await expect(page.getByTestId("table-tree-g1-0")).toHaveCount(0);

        await page.getByTestId("table-tree-g0").focus();
        await page.keyboard.press("ArrowLeft");
        await expect(page.getByTestId("table-tree-g0")).toHaveAttribute("aria-expanded", "false");
        await expect(page.getByTestId("table-tree-g0-0")).toHaveCount(0);
        await page.keyboard.press("ArrowRight");
        await expect(page.getByTestId("table-tree-g0")).toHaveAttribute("aria-expanded", "true");
        await page.keyboard.press("ArrowRight");
        await expect(page.getByTestId("table-tree-g0-0")).toBeFocused();
        await page.keyboard.press("ArrowLeft");
        await expect(page.getByTestId("table-tree-g0")).toBeFocused();

        // Choosing one is an event the application hears; what it filters is
        // the next milestone's.
        await page.keyboard.press("ArrowDown");
        await page.keyboard.press(" ");
        await expect.poll(async () => (await api(page)).table.group).toBe("g0-0");
        await expect(page.getByTestId("table-tree-g0-0")).toHaveAttribute("aria-selected", "true");
    });

    test("the strip sends the table where it was clicked", async ({ page }) => {
        await openTable(page);
        await page.evaluate(() => window.__data_workbench.select_rows(0, 200));
        const strip = page.getByTestId("table-strip");
        await expect(strip).toHaveCount(1);
        const box = (await strip.boundingBox())!;
        // Spread over the whole strip however many there are.
        const clicks = rounds(20);
        let jumps = 0;
        for (let click = 0; click < clicks; click++) {
            const across = (click + 0.5) / clicks;
            await page.mouse.click(box.x + box.width * across, box.y + box.height / 2);
            jumps += 1;
            await expect.poll(async () => (await api(page)).table.jumps).toBe(jumps);
            const asked = Math.floor(across * twin.ROWS);
            const drawn = await drawnRows(page);
            // Somewhere near where it was clicked: a bucket is a hundred rows
            // wide, and the window is sixty.
            expect(
                drawn.some((row) => Math.abs(row.index - 1 - asked) < 200),
                `click ${click} asked for about ${asked}`
            ).toBe(true);
        }
    });

    test("text that looks like markup stays a value", async ({ page }) => {
        await openTable(page);
        await page.evaluate(() => window.__data_workbench.open_row(7));
        const field = page.getByTestId("table-detail-3");
        await field.fill("<script>x</script>");
        await page.getByTestId("table-detail-submit").click();
        await expect.poll(async () => (await api(page)).table.saves).toBe(1);
        await gotoRow(page, 8);
        // A cell is sixteen characters, so what is stored is the front of it;
        // what matters is that it is text.
        await expect(cellOf(page, 8, 3)).toHaveText("<script>x</scrip");
        expect(
            await page.evaluate(
                () => document.querySelectorAll('[data-testid="table"] script').length
            )
        ).toBe(0);
    });

    test("the table's fourteen named things are findable, thirty times over", async ({ page }) => {
        await openTable(page);
        // Two of them are fields of the row being edited, so a row is open: a
        // name nothing is showing is not a name that is missing.
        await page.evaluate(() => window.__data_workbench.open_row(0));
        const expected = LOCATORS.filter((entry) => entry.side === "table");
        expect(expected).toHaveLength(14);
        // Each one is reachable by role and name - which is what a person
        // using a screen reader actually does - and it is the same element
        // that carries the test id. An intersection rather than a reading of
        // the role attribute, because a spinbutton is a number input and says
        // so without an attribute. Thirty rounds, because a name that is right
        // once and wrong after a redraw is worse than one never right at all.
        const total = rounds(30);
        // Six redraws over the thirty, and one after every round of fewer:
        // what is counted is the name surviving a redraw either way.
        const redrawEvery = Math.max(1, Math.round(total / 6));
        for (let round = 0; round < total; round++) {
            for (const entry of expected) {
                const found = page.getByTestId(entry.testId);
                await expect(found, `${entry.testId} round ${round}`).toHaveCount(1);
                const role = entry.role as Parameters<Page["getByRole"]>[0];
                const named = entry.name
                    ? page.getByRole(role, { name: entry.name, exact: true })
                    : page.getByRole(role);
                await expect(
                    found.and(named),
                    `${entry.testId} by role and name, round ${round}`
                ).toHaveCount(1);
            }
            // Something that redraws the table between rounds, so what is being
            // counted is the name surviving rather than the same DOM standing
            // still.
            if (round % redrawEvery === 0) {
                await gotoRow(page, 1 + round * 1_000);
            }
        }
    });
});

test.describe("review · table identity and window", () => {
    for (const mode of ["sorted", "filtered"] as const) {
        test(`deleting ${mode} rows removes the displayed identities`, async ({ page }) => {
            await openTable(page);
            if (mode === "sorted") {
                await page.getByTestId("table-column-0").getByRole("button").click();
            } else {
                await page.getByTestId("table-filter").fill("QQ");
                await page.getByTestId("table-filter").press("Enter");
            }
            await expect.poll(async () => (await api(page)).table.job.ended).toBe("done");
            const before = await page.evaluate(() => {
                const api = window.__data_workbench;
                return api.view(0, 30).map((row) => api.row_id(row));
            });
            await page.locator('[data-row-id]').first().getByRole("gridcell").first().focus();
            await page.evaluate(() => window.__data_workbench.select_rows(0, 12));
            await page.getByTestId("table-delete").click();
            const after = await page.evaluate(() => {
                const api = window.__data_workbench;
                return api.view(0, 20).map((row) => api.row_id(row));
            });
            expect(after).toEqual(before.slice(10));
            expect(await page.evaluate(() => window.__data_workbench.selected())).toEqual(
                before.slice(10, 12).sort((a, b) => a - b)
            );
        });
    }

    test("an open draft stays with its identity across insert and delete", async ({ page }) => {
        await openTable(page);
        const row = page.locator('[data-row-id="20"]');
        await row.getByRole("gridcell").first().focus();
        await page.keyboard.press("Enter");
        await page.getByTestId("table-detail-0").fill("DRAFT");
        await page.getByTestId("table-insert").click();
        await expect(page.getByTestId("table-detail-0")).toHaveValue("DRAFT");
        await page.getByTestId("table-detail-submit").click();
        expect(await page.evaluate(() => window.__data_workbench.cell(29, 0).trim())).toBe("DRAFT");
        await page.getByTestId("table-delete").click();
        await expect(page.getByTestId("table-detail-row")).toHaveText("row 20");
        await page.getByTestId("table-detail-0").fill("SAVED AGAIN");
        await page.getByTestId("table-detail-submit").click();
        expect(await page.evaluate(() => window.__data_workbench.cell(19, 0).trim())).toBe("SAVED AGAIN");
        await page.getByTestId("table-delete").click();
        await expect(page.getByTestId("table-detail-row")).toHaveText("no row open");
        await expect(page.getByTestId("table-detail-0")).toHaveCount(0);
    });

    test("scrolling a focused slot keeps selection and the tab stop on the focused row", async ({ page }) => {
        await openTable(page);
        await page.locator('[data-row-id="1"]').getByRole("gridcell").first().focus();
        await page.getByTestId("table-scroller").evaluate((element) => element.scrollTo(1800, 1_000_000));
        await expect.poll(async () => Number(await page.locator('[data-row-id]').first().getAttribute("data-row-id"))).toBeGreaterThan(1);
        await expect(page.locator('[role="gridcell"][tabindex="0"]')).toHaveCount(1);
        const focusedId = await page.evaluate(() => Number(document.activeElement?.closest('[data-row-id]')?.getAttribute("data-row-id")));
        expect(focusedId).toBeGreaterThan(1);
        await page.keyboard.press(" ");
        expect(await page.evaluate(() => window.__data_workbench.selected())).toEqual([focusedId]);
        await expect(page.locator('[role="gridcell"][tabindex="0"]')).toBeFocused();
    });

    test("resizing the scroller covers its enlarged viewport without scrolling", async ({ page }) => {
        await page.setViewportSize({ width: 1100, height: 600 });
        await arrive(page, "/table");
        await expect(page.getByTestId("status")).toHaveAttribute("data-status", "ready");
        const count = await page.locator('[data-row-id]').count();
        await page.setViewportSize({ width: 1440, height: 1200 });
        await expect.poll(() => page.locator('[data-row-id]').count()).toBeGreaterThan(count);
        // Change only the element's layout too; a window resize listener cannot see this.
        await page.getByTestId("table-scroller").evaluate((element) => element.style.maxHeight = "240px");
        await expect.poll(() => page.locator('[data-row-id]').count()).toBeLessThan(count);
        await page.getByTestId("table-scroller").evaluate((element) => element.style.maxHeight = "");
        await expect.poll(async () => page.getByTestId("table-scroller").evaluate((element) => {
            const bottom = Math.max(...Array.from(element.querySelectorAll('[data-row-id]')).map((row) => row.getBoundingClientRect().bottom));
            return bottom >= element.getBoundingClientRect().bottom;
        })).toBe(true);
    });

    test("ARIA row indices include the header and stay within the total", async ({ page }) => {
        await openTable(page);
        await expect(page.getByTestId("table")).toHaveAttribute("aria-rowcount", "100001");
        await expect(page.getByTestId("table").getByRole("row").first()).toHaveAttribute("aria-rowindex", "1");
        await expect(page.locator('[data-row-id="1"]')).toHaveAttribute("aria-rowindex", "2");
        await gotoRow(page, 100000);
        await expect(page.locator('[data-row-id="100000"]')).toHaveAttribute("aria-rowindex", "100001");
    });
});

test.describe(() => {
    // A script that has to run before the page does, and an instance that is
    // killed at the end: both need a page of their own.
    test.use({ fresh: true });

    test("review · size observations stop on unmount and fatal", async ({ page }) => {
        await page.addInitScript(() => {
            const NativeObserver = window.ResizeObserver;
            const tableObservers = new Set<ResizeObserver>();
            Object.assign(window, { __tableObservers: tableObservers });
            window.ResizeObserver = class extends NativeObserver {
                observe(element: Element, options?: ResizeObserverOptions) {
                    if (element.getAttribute("data-testid") === "table-scroller") tableObservers.add(this);
                    super.observe(element, options);
                }
                disconnect() {
                    tableObservers.delete(this);
                    super.disconnect();
                }
            };
        });
        await openTable(page);
        const observations = () => page.evaluate(() => (window as unknown as { __tableObservers: Set<ResizeObserver> }).__tableObservers.size);
        await expect.poll(observations).toBe(1);
        await page.evaluate(() => window.__data_workbench.dispose());
        await expect.poll(observations).toBe(0);
        await page.evaluate(() => window.__data_workbench.mount());
        await expect.poll(observations).toBe(1);
        await page.evaluate(() => window.__data_workbench.hooks.runtime.enter_fatal(new Error("observer cleanup")));
        await expect.poll(observations).toBe(0);
    });
});

test("review · resizing at the end preserves the focused cell", async ({ page }) => {
    await page.setViewportSize({ width: 1100, height: 600 });
    await arrive(page, "/table");
    await expect(page.getByTestId("status")).toHaveAttribute("data-status", "ready");
    await page.locator('[data-row-id="1"]').getByRole("gridcell").first().focus();
    await page.keyboard.press("ControlOrMeta+End");
    const last = page.locator('[data-row-id="100000"] [aria-colindex="20"]');
    await expect(last).toBeFocused();
    await page.setViewportSize({ width: 1440, height: 1200 });
    await expect(last).toBeFocused();
    await expect(page.locator('[role="gridcell"][tabindex="0"]')).toBeFocused();
    await page.setViewportSize({ width: 1100, height: 600 });
    await expect(last).toBeFocused();
    await expect(page.locator('[role="gridcell"][tabindex="0"]')).toBeFocused();
});

/// What a first load of the page fixes, read the same way after a load and
/// after a reset: the application's own account of itself, the sample, the
/// address, and what the document shows and holds.
async function firstLoadState(page: Page) {
    return page.evaluate(() => {
        const api = window.__data_workbench;
        const snapshot = api.snapshot() as unknown as Record<string, unknown>;
        // How long making the sample took: a timing, which two first loads do
        // not agree on either.
        delete snapshot.generated_ms;
        const scroller = document.querySelector('[data-testid="table-scroller"]');
        return {
            snapshot,
            hash: api.dataset_hash(),
            url: location.pathname + location.search + location.hash,
            view: api.view(0, 30),
            selected: api.selected(),
            chosen: api.scene_chosen(),
            regions: api.live_regions(),
            errors: [...api.errors()],
            focus: document.activeElement?.tagName ?? null,
            scrolled: [window.scrollX, window.scrollY],
            scroller: scroller && [scroller.scrollTop, scroller.scrollLeft, scroller.getAttribute("style")],
            inputs: Array.from(
                document.querySelectorAll<HTMLInputElement>("#workbench input[data-testid]"),
                (input) => [input.dataset.testid, input.value]
            ),
            expanded: Array.from(
                document.querySelectorAll('#workbench [aria-expanded="true"]'),
                (node) => node.getAttribute("data-testid")
            ),
            sorted: Array.from(document.querySelectorAll("#workbench [aria-sort]"), (node) =>
                node.getAttribute("aria-sort")
            ),
            rows: Array.from(
                document.querySelectorAll('[data-testid="table"] [data-row-id]'),
                (row) => row.getAttribute("data-row-id")
            ).slice(0, 5),
            open: document.querySelector(
                '[data-testid="table-detail-row"], [data-testid="scene-detail-object"]'
            )?.textContent,
            listed: document.querySelectorAll('[data-testid="scene-selected"] li').length,
            moves:
                (document.querySelector('[data-testid="scene-gpu"]') as { moves?: number } | null)
                    ?.moves ?? null,
            scratch: document.getElementById("scratch")?.childElementCount,
        };
    });
}

test.describe(() => {
    // What a reset is compared against is a first load, so this one loads.
    test.use({ fresh: true });

    test("a reset leaves the page as a first load of the same address does", async ({ page }) => {
        // Two loads and three regions started, on a software rasteriser.
        test.setTimeout(240_000);
        const reset = (path?: string) =>
            page.evaluate(
                (path) => (window.__data_workbench as unknown as Resettable).reset(path),
                path
            );
        const drawn = () =>
            expect
                .poll(async () => (await api(page)).scene.drawn, { timeout: 60_000 })
                .toBeGreaterThan(0);
        const jobDone = () =>
            expect
                .poll(async () => (await api(page)).table.job.running, { timeout: 30_000 })
                .toBe(false);
        const ready = async () => {
            await expect(page.getByTestId("status")).toHaveAttribute("data-status", "ready", {
                timeout: 120_000,
            });
            await waitForQuiet(page);
        };
        await page.setViewportSize({ width: 1440, height: 1200 });

        // The two first loads: the scene's, and the root's, which is the
        // table's and which the page then stays on.
        await page.goto("./scene");
        await ready();
        await drawn();
        await waitForQuiet(page);
        const scene = await firstLoadState(page);
        await page.goto("./");
        await ready();
        const root = await firstLoadState(page);
        expect(root.url).toBe("/table");
        expect(root.snapshot).toMatchObject({ rows: twin.ROWS, version: 0 });

        /// Everything the table lets a person change, and a few things a check
        /// leaves behind on the page, ending with a job still running.
        const disturbTable = async () => {
            await page.getByTestId("table-column-3").getByRole("button").click();
            await jobDone();
            await page.getByTestId("table-filter").fill("QQ");
            await page.getByTestId("table-filter").press("Enter");
            await jobDone();
            await page.evaluate(() => window.__data_workbench.select_rows(0, 5));
            await page.evaluate(() => window.__data_workbench.open_row(7));
            await page.getByTestId("table-detail-2").fill("EDITED");
            await page.getByTestId("table-detail-submit").click();
            await expect.poll(async () => (await api(page)).table.saves).toBe(1);
            await page.locator("[data-row-id]").first().getByRole("gridcell").first().focus();
            await page.getByTestId("table-insert").click();
            await page.getByTestId("table-delete").click();
            await page.getByTestId("table-delete").click();
            await page.getByTestId("table-tree-g0").focus();
            await page.keyboard.press("ArrowLeft");
            await page.getByTestId("table-goto").fill("500");
            await page.getByTestId("table-goto").press("Enter");
            await page.getByTestId("table-find").fill("NOT THERE");
            await page.getByTestId("table-scroller").evaluate((element) => {
                (element as HTMLElement).style.maxHeight = "240px";
            });
            await page.evaluate(() => {
                window.__data_workbench.hooks.runtime.errors.push("left by a check");
                document.getElementById("scratch")!.append(document.createElement("div"));
                window.__data_workbench.sort(5, true);
            });
            const changed = await firstLoadState(page);
            expect(changed.hash).not.toBe(root.hash);
            expect(changed.snapshot).not.toEqual(root.snapshot);
        };
        const disturbScene = async () => {
            await drawn();
            await waitForQuiet(page);
            await page.evaluate(() => window.__data_workbench.look_at(900, 700));
            await expect.poll(async () => (await api(page)).scene.camera).toEqual([900, 700]);
            // Found by name, which moves the camera onto it, and then pointed
            // at, which chooses it.
            const index = 5_000;
            await page.getByTestId("scene-find").fill(twin.sceneLabel(index));
            await page.getByTestId("scene-find").press("Enter");
            await expect.poll(async () => (await api(page)).scene.editing).toBe(index);
            const box = (await page.getByTestId("scene-gpu").boundingBox())!;
            const camera = (await api(page)).scene.camera;
            const rect = twin.sceneRect(index);
            await page.mouse.click(box.x + rect.x + 10 - camera[0], box.y + rect.y + 10 - camera[1]);
            await expect.poll(async () => (await api(page)).scene.picks).toBe(1);
            expect(await page.evaluate(() => window.__data_workbench.scene_chosen())).toEqual([index]);
            await page.getByTestId("scene-detail-label").fill("RENAMED");
            await page.getByTestId("scene-detail-submit").click();
            await expect.poll(async () => (await api(page)).scene.renames).toBe(1);
            await page.getByTestId("scene-gpu").evaluate((canvas) => {
                (canvas as HTMLCanvasElement & { moves?: number }).moves = 3;
            });
        };

        // The table, then the scene, then the default reset: the root again.
        await disturbTable();
        await page.getByTestId("go-scene").click();
        await disturbScene();
        await reset();
        await waitForQuiet(page);
        expect(await firstLoadState(page)).toEqual(root);

        // The table again, reset in place, where its region stays.
        await disturbTable();
        await reset("/table");
        await waitForQuiet(page);
        expect(await firstLoadState(page)).toEqual(root);

        // From the table to the scene, which is how every scene check starts.
        await reset("/scene");
        await waitForQuiet(page);
        expect(await firstLoadState(page)).toEqual(scene);

        // And the scene in place, where its region stays.
        await disturbScene();
        await reset("/scene");
        await waitForQuiet(page);
        expect(await firstLoadState(page)).toEqual(scene);
    });
});
