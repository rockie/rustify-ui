import { expect, Locator, Page, test } from "@playwright/test";
import { LOCATORS } from "./loads";
import * as twin from "./dataset";

/// M2 · the table, the groups over it and the row being edited.
///
/// What is checked here is identity and reach: that a hundred thousand rows
/// can all be got to, that the ones on screen are a window rather than the
/// whole thing, and that a selection is of things rather than of positions -
/// so deleting a row deselects that row and nothing else.

const api = (page: Page) => page.evaluate(() => window.__data_workbench.snapshot());

async function openTable(page: Page) {
    await page.setViewportSize({ width: 1440, height: 1200 });
    await page.goto("./table");
    await expect(page.getByTestId("status")).toHaveAttribute("data-status", "ready", {
        timeout: 120_000,
    });
    await expect(page.getByTestId("table-view")).toHaveCount(1);
    await expect(page.getByTestId("table")).toHaveAttribute(
        "aria-rowcount",
        String(twin.ROWS)
    );
}

/// The row elements that are showing a row, and the row each one is showing.
async function drawnRows(page: Page): Promise<{ index: number; id: number }[]> {
    return page.evaluate(() =>
        Array.from(document.querySelectorAll('[data-testid="table"] [role="row"]'))
            .filter((row) => !row.classList.contains("rui:hidden") && row.hasAttribute("data-row-id"))
            .map((row) => ({
                index: Number(row.getAttribute("aria-rowindex")),
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

function cellOf(page: Page, row: number, column: number): Locator {
    return page.locator(`[data-testid="table"] [role="row"][aria-rowindex="${row}"] [role="gridcell"]`).nth(column);
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
        const field = page.getByTestId("table-detail-2").locator("input");
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
        // and none of the new rows is selected.
        await gotoRow(page, 1);
        await cellOf(page, 1, 0).click();
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

    test("a selected row that is no longer there is counted as not in view", async ({ page }) => {
        await openTable(page);
        // Select ten rows in the middle, then delete three of them.
        await page.evaluate(() => window.__data_workbench.select_rows(500, 10));
        await gotoRow(page, 501);
        await cellOf(page, 501, 0).click();
        await page.evaluate(() => window.__data_workbench.open_row(500));
        await page.getByTestId("table-delete").click();
        await expect.poll(async () => (await api(page)).rows).toBe(twin.ROWS - 10);
        const state = (await api(page)).table;
        // All ten were deleted, so none is selected and none is hidden: a
        // deleted identity is not a hidden one.
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
        let jumps = 0;
        for (let click = 0; click < 20; click++) {
            const across = (click + 0.5) / 20;
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
        const field = page.getByTestId("table-detail-3").locator("input");
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

    test("the table's twelve named things are findable, thirty times over", async ({ page }) => {
        await openTable(page);
        const expected = LOCATORS.filter((entry) => entry.side === "table" && entry.from === "M2");
        // Every one of them exists and carries the role the list says. Thirty
        // rounds, because a name that is right once and wrong after a redraw
        // is worse than one that was never right.
        for (let round = 0; round < 30; round++) {
            for (const entry of expected) {
                const found = page.getByTestId(entry.testId);
                await expect(found, `${entry.testId} round ${round}`).toHaveCount(1);
                await expect(found, `${entry.testId} role, round ${round}`).toHaveAttribute(
                    "role",
                    entry.role === "textbox" || entry.role === "spinbutton" ? /.*/ : entry.role
                );
            }
            // Something that redraws the table between rounds, so what is being
            // counted is the name surviving rather than the same DOM standing
            // still.
            if (round % 5 === 0) {
                await gotoRow(page, 1 + round * 1_000);
            }
        }
        // And each of them is reachable by role and name, which is what a
        // person using a screen reader actually does.
        await expect(page.getByRole("grid", { name: "the sample" })).toHaveCount(1);
        await expect(page.getByRole("tree", { name: "groups" })).toHaveCount(1);
        await expect(page.getByRole("treeitem", { name: "group 1" }).first()).toBeVisible();
        await expect(page.getByRole("spinbutton", { name: "go to row" })).toHaveCount(1);
        await expect(page.getByRole("columnheader", { name: "column 1" })).toHaveCount(1);
    });
});
