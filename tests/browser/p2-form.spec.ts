import { expect, Page, test } from "@playwright/test";

import { sharedPage } from "./support";

/// M2 V5: a form of twenty visible properties, and the three questions it
/// exists to answer - which validation answer is still current, whether a
/// submit that arrived during one is still the submit that was asked for, and
/// how many saves twenty clicks are.
///
/// The workbench holds the values; the SDK holds the bookkeeping. So the
/// assertions here read the application's own snapshot for what was saved and
/// the accessibility tree for what a person is told.

const snapshot = (page: Page) => page.evaluate(() => window.__property_workbench.snapshot());
const checks = (page: Page) => page.evaluate(() => window.__property_workbench.form_checks());
const resolveCheck = (page: Page, index: number, ok: boolean) =>
    page.evaluate(([index, ok]) => window.__property_workbench.resolve_check(index, ok), [
        index,
        ok,
    ] as const);
const resolveSave = (page: Page, ok: boolean) =>
    page.evaluate((ok) => window.__property_workbench.resolve_save(ok), ok);

/// Types into a field of the details form and lets the form hear about it.
async function type(page: Page, field: string, value: string) {
    await page.getByTestId(field).fill(value);
}

test.describe("M2 V5: a form that says what is wrong and saves once", () => {
    test.describe.configure({ mode: "serial" });
    const shared = sharedPage();

    test("twenty visible properties, each with a label pointing at its control", async () => {
        const page = shared.page;
        expect((await snapshot(page)).form).toMatchObject({ fields: 20 });

        // The fifteen the form draws are labelled and wired; the other five
        // are the live controls above them.
        const fields = page.locator('[data-testid^="field-"]');
        await expect(fields).toHaveCount(15);
        const wiring = await page.evaluate(() =>
            Array.from(document.querySelectorAll('[data-testid^="field-"]')).map((field) => {
                const label = field.querySelector("label")!;
                const control = field.querySelector(
                    "input, textarea, [role=combobox], [role=switch]"
                )!;
                return {
                    field: (field as HTMLElement).dataset.testid,
                    names: label.getAttribute("for") === control.id,
                    described: control.getAttribute("aria-describedby"),
                };
            })
        );
        for (const row of wiring) {
            expect(row.names, `${row.field}`).toBe(true);
            // Nothing points at an error that is not on the page.
            expect(row.described, `${row.field}`).toBeNull();
        }
    });

    test("a rule that fails names its field, describes it, and takes the keyboard", async () => {
        const page = shared.page;
        await type(page, "email", "not-an-address");
        await page.getByTestId("save-details").click();

        expect((await snapshot(page)).form).toMatchObject({
            asked: "blocked",
            first_error: "email",
            saves: 0,
        });
        const error = page.getByTestId("error-email");
        await expect(error).toHaveText("an address needs a name and a host");
        const field = page.getByTestId("email");
        await expect(field).toHaveAttribute("aria-invalid", "true");
        await expect(field).toHaveAttribute("aria-describedby", (await error.getAttribute("id"))!);
        // The keyboard is on the thing that is wrong, without being asked.
        await expect(field).toBeFocused();

        await type(page, "email", "owner-0001@example.com");
        await expect(error).toBeHidden();
        await expect(field).not.toHaveAttribute("aria-invalid", "true");
    });

    test("a rule across two fields belongs to neither and says which pair", async () => {
        const page = shared.page;
        await type(page, "end", "2025-01-01");
        await page.getByTestId("save-details").click();
        await expect(page.getByTestId("error-end")).toHaveText("the end is before the start");
        expect((await snapshot(page)).form).toMatchObject({ saves: 0 });

        // And one whose requirement depends on another field's value.
        await type(page, "end", "2026-12-31");
        await page.getByTestId("priority").click();
        await page.getByTestId("option-high").click();
        await page.getByTestId("save-details").click();
        await expect(page.getByTestId("error-reviewer")).toHaveText(
            "a high priority needs a reviewer"
        );

        await type(page, "reviewer", "a reviewer");
        await page.getByTestId("save-details").click();
        expect((await snapshot(page)).form).toMatchObject({ asked: "save", saves: 1 });
        expect(await resolveSave(page, true)).toBe(true);
    });

    test("twenty checks answered backwards leave the latest one showing", async () => {
        const page = shared.page;
        // Twenty edits, each starting a check. Only the last one is about the
        // value on screen; the other nineteen are about values that are gone.
        for (let round = 0; round < 20; round += 1) {
            await type(page, "reference", `REF-${1000 + round}`);
        }
        expect(await checks(page)).toHaveLength(20);

        // The newest answers first, and it is the one that counts.
        expect(await resolveCheck(page, 19, false)).toBe(true);
        await expect(page.getByTestId("error-reference")).toHaveText("no such reference");
        expect((await snapshot(page)).form).toMatchObject({
            errors: 1,
            first_error: "reference",
        });

        // Then the nineteen stale ones, each with the opposite answer. Not one
        // of them may overwrite what is on screen: they describe values that
        // no longer exist.
        for (let round = 18; round >= 0; round -= 1) {
            expect(await resolveCheck(page, round, true)).toBe(true);
            expect((await snapshot(page)).form, `stale answer ${round}`).toMatchObject({
                errors: 1,
                first_error: "reference",
            });
        }
        expect((await snapshot(page)).form).toMatchObject({ checks: 0 });
        await expect(page.getByTestId("error-reference")).toHaveText("no such reference");

        // Leave it usable for what runs next.
        await type(page, "reference", "REF-1999");
        expect(await resolveCheck(page, 0, true)).toBe(true);
        expect((await snapshot(page)).form).toMatchObject({ errors: 0 });
    });

    test("asking while a check runs waits, and then saves exactly once", async () => {
        const page = shared.page;
        const before = (await snapshot(page)).form.saves;
        await type(page, "reference", "REF-2001");
        expect(await checks(page)).toHaveLength(1);

        // The button already says no, because a check is running - and it
        // says so in a way that leaves it reachable, so the click still gets
        // to `submit()`, which is the authority.
        await expect(page.getByTestId("save-details")).toHaveAttribute("aria-disabled", "true");
        await page.getByTestId("save-details").click({ force: true });
        expect((await snapshot(page)).form).toMatchObject({
            asked: "waiting",
            submitting: false,
            saves: before,
        });

        expect(await resolveCheck(page, 0, true)).toBe(true);
        expect((await snapshot(page)).form).toMatchObject({
            submitting: true,
            saves: before + 1,
        });
        expect(await resolveSave(page, true)).toBe(true);
        expect((await snapshot(page)).form).toMatchObject({ submitting: false, dirty: false });
    });

    test("a check that fails turns a waiting submit into a block, not a save", async () => {
        const page = shared.page;
        const before = (await snapshot(page)).form.saves;
        await type(page, "reference", "REF-2002");
        await page.getByTestId("save-details").click({ force: true });
        expect((await snapshot(page)).form).toMatchObject({ asked: "waiting" });

        expect(await resolveCheck(page, 0, false)).toBe(true);
        expect((await snapshot(page)).form).toMatchObject({
            saves: before,
            first_error: "reference",
        });
        await expect(page.getByTestId("error-reference")).toHaveText("no such reference");
        await expect(page.getByTestId("reference")).toBeFocused();

        // And asking twenty times more never saves: the answer was about the
        // value, and the value has not changed.
        for (let round = 0; round < 20; round += 1) {
            await page.getByTestId("save-details").click({ force: true });
        }
        expect((await snapshot(page)).form).toMatchObject({ saves: before });

        await type(page, "reference", "REF-2003");
        expect(await resolveCheck(page, 0, true)).toBe(true);
    });

    test("changing a field while a submit waits voids the submit", async () => {
        const page = shared.page;
        const before = (await snapshot(page)).form.saves;
        await type(page, "reference", "REF-2004");
        await page.getByTestId("save-details").click({ force: true });
        expect((await snapshot(page)).form).toMatchObject({ asked: "waiting" });

        // The user carried on. What they asked to save is not what is on
        // screen any more.
        await type(page, "location", "shelf 9");
        expect(await resolveCheck(page, 0, true)).toBe(true);
        expect((await snapshot(page)).form).toMatchObject({ saves: before });
    });

    test("twenty clicks while a save runs are one save", async () => {
        const page = shared.page;
        const before = (await snapshot(page)).form.saves;
        await type(page, "location", "shelf 4");
        await page.getByTestId("save-details").click();
        expect((await snapshot(page)).form).toMatchObject({ submitting: true });

        for (let round = 0; round < 20; round += 1) {
            await page.getByTestId("save-details").click({ force: true });
        }
        expect((await snapshot(page)).form).toMatchObject({
            saves: before + 1,
            asked: "busy",
        });
        // And the button said so as well as the state machine.
        await expect(page.getByTestId("save-details")).toHaveAttribute("aria-disabled", "true");

        expect(await resolveSave(page, true)).toBe(true);
        expect((await snapshot(page)).form).toMatchObject({ submitting: false });
    });

    test("a save that fails keeps the values and says why", async () => {
        const page = shared.page;
        await type(page, "location", "shelf 12");
        await page.getByTestId("save-details").click();
        expect(await resolveSave(page, false)).toBe(true);

        expect((await snapshot(page)).form).toMatchObject({
            dirty: true,
            failure: "the server refused the details",
        });
        await expect(page.getByTestId("details-status")).toHaveText(
            "the server refused the details"
        );
        await expect(page.getByTestId("details-status")).toHaveAttribute("data-state", "failed");
        // Nothing was taken away: the value the user typed is still there.
        await expect(page.getByTestId("location")).toHaveValue("shelf 12");

        // And it can be asked again.
        await page.getByTestId("save-details").click();
        expect((await snapshot(page)).form).toMatchObject({ asked: "save", failure: null });
        expect(await resolveSave(page, true)).toBe(true);
        expect((await snapshot(page)).form).toMatchObject({ dirty: false });
    });
});

