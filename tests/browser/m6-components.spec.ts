import { expect, Page, test } from "@playwright/test";
import { CATALOG_SIZE, capture, differingPixels, settle, waitForReady } from "./support";

const snapshot = (page: Page) => page.evaluate(() => window.__property_workbench.snapshot());

const styleOf = (page: Page, selector: string) =>
    page.evaluate((selector) => {
        const element = document.querySelector(selector) as HTMLElement;
        const style = getComputedStyle(element);
        return {
            color: style.color,
            background: style.backgroundColor,
            fontSize: style.fontSize,
            gap: style.rowGap,
            // What the element resolves the tokens to, override included.
            foreground: style.getPropertyValue("--foreground").trim(),
            spacing: style.getPropertyValue("--spacing").trim(),
        };
    }, selector);

test.describe("M6 V7: every category, and no blank support cell", () => {
    test("the catalogue answers for every category it names", async ({ page }) => {
        await waitForReady(page);
        const rows = page.locator('[data-testid="catalogue"] tbody tr');
        await expect(rows).toHaveCount(CATALOG_SIZE);

        const cells = await page.evaluate(() =>
            Array.from(document.querySelectorAll('[data-testid="catalogue"] tbody tr')).map(
                (row) => ({
                    category: row.querySelector("th")!.textContent,
                    region: (row as HTMLElement).dataset.region,
                    values: Array.from(row.querySelectorAll("td")).map((cell) => ({
                        support: (cell as HTMLElement).dataset.support,
                        text: cell.textContent?.trim() ?? "",
                    })),
                })
            )
        );

        for (const row of cells) {
            // Three presentation columns and the six capability classes.
            expect(row.values, `${row.category}`).toHaveLength(9);
            for (const value of row.values) {
                expect(["yes", "partial", "no"], `${row.category}`).toContain(value.support);
                expect(value.text, `${row.category}`).not.toBe("");
            }
            // The presentation columns say the level; the six classes say why.
            for (const value of row.values.slice(0, 3)) {
                expect(value.text).toBe(value.support);
            }
        }

        // Every category has a DOM component, which is where a control is
        // named, focused and read - including the ones a region draws.
        for (const row of cells) {
            expect(row.values[0].support, `${row.category}`).toBe("yes");
        }

        const drawn = cells.filter((row) => row.region === "true").map((row) => row.category);
        expect(drawn).toEqual([
            "button",
            "label",
            "icon",
            "text field",
            "text area",
            "checkbox",
            "radio",
            "switch",
            "select",
            "slider",
            "progress",
            "loading",
            "tabs",
            "scroll area",
        ]);
        // The ones with no GPU half say where they are drawn instead: a cell
        // saying only "no" would read as "not usable with a region", which is
        // the opposite of true for every one of them.
        const elsewhere = cells.filter((row) => row.region !== "true");
        expect(elsewhere.map((row) => row.category)).toEqual([
            "link",
            "tooltip",
            "menu",
            "dialog",
            "data table",
            "tree",
        ]);
        for (const row of elsewhere) {
            expect(row.values[1].support, `${row.category}`).toBe("no");
            expect(row.values[8].text, `${row.category}`).toContain("region");
        }
    });
});

test.describe("M6 V7: a value the application refuses", () => {
    test("the field goes back to the value in force and says why", async ({ page }) => {
        await waitForReady(page);
        const name = page.getByRole("textbox", { name: "name" });
        await name.fill("a name that is fine");
        expect(await snapshot(page)).toMatchObject({ name: "a name that is fine", refusals: 0 });

        // An empty name is refused, and the control does not keep the
        // keystrokes that changed nothing.
        await name.fill("");
        await expect(page.getByTestId("rejected-value")).toContainText("a name cannot be empty");
        await expect(name).toHaveValue("a name that is fine");
        expect(await snapshot(page)).toMatchObject({ name: "a name that is fine", refusals: 1 });

        // So is one another object already has.
        await name.fill("object-0002");
        await expect(page.getByTestId("rejected-value")).toContainText(
            "object 2 already has that name"
        );
        await expect(name).toHaveValue("a name that is fine");

        // And an accepted one clears the notice.
        await name.fill("accepted after all");
        await expect(page.getByTestId("rejected-value")).toHaveCount(0);
        expect(await snapshot(page)).toMatchObject({ name: "accepted after all", refusals: 2 });
    });

    test("a read-only field is reachable and unchanged by typing", async ({ page }) => {
        await waitForReady(page);
        const id = page.getByRole("textbox", { name: "object id" });
        await expect(id).toHaveValue("1");
        await id.focus();
        // It takes focus - it is a control, not a picture of one.
        expect(await page.evaluate(() => document.activeElement?.getAttribute("data-testid"))).toBe(
            "object-id"
        );
        await page.keyboard.type("999");
        await expect(id).toHaveValue("1");
        expect(await snapshot(page)).toMatchObject({ selected: 1, refusals: 0 });
    });

    test("locking an object makes its name read-only in both halves", async ({ page }) => {
        await waitForReady(page);
        const name = page.getByRole("textbox", { name: "name" });
        await page.getByRole("checkbox", { name: "locked" }).check();
        expect(await snapshot(page)).toMatchObject({ locked: true });
        await expect(name).toHaveAttribute("aria-readonly", "true");

        // The browser's own read-only control refuses the keystrokes, so the
        // application is never asked and the value cannot move.
        await name.focus();
        await page.keyboard.type("no");
        expect(await snapshot(page)).toMatchObject({ name: "object-0001" });

        // The GPU path is refused by the application, which says why.
        await page.getByRole("checkbox", { name: "locked" }).uncheck();
        expect(await snapshot(page)).toMatchObject({ locked: false });
        await name.fill("editable again");
        expect(await snapshot(page)).toMatchObject({ name: "editable again" });
    });

    test("a disabled control neither changes a value nor takes the keyboard", async ({ page }) => {
        await waitForReady(page);
        await page.getByRole("button", { name: "delete selected" }).click();
        expect(await snapshot(page)).toMatchObject({ selected: null });
        const name = page.getByRole("textbox", { name: "name" });
        await expect(name).toBeDisabled();
        await expect(page.getByRole("checkbox", { name: "locked" })).toBeDisabled();
        await expect(page.getByRole("slider", { name: "size" })).toBeDisabled();
        const before = await snapshot(page);
        await page.keyboard.press("Tab");
        await page.keyboard.type("nothing");
        expect(await snapshot(page)).toMatchObject({ count: before.count, selected: null });
    });
});

test.describe("M6 V7: one value, both halves, one range", () => {
    test("the DOM slider and the region show the object's own value", async ({ page }) => {
        await waitForReady(page);
        const region = page.getByTestId("workbench-gpu");
        const before = await settle(region);
        const size = page.getByRole("slider", { name: "size" });
        await expect(size).toHaveValue("50");

        // The browser's own control, driven the way a user drives it.
        await size.focus();
        await page.keyboard.press("ArrowRight");
        expect(await snapshot(page)).toMatchObject({ size: 55 });
        await expect(size).toHaveValue("55");
        // And the region drew it.
        await expect
            .poll(async () => differingPixels(before, await capture(region)), { timeout: 5_000 })
            .toBeGreaterThan(20);

        // The value belongs to the object, not to the control.
        await page.getByRole("button", { name: "next" }).click();
        await expect(size).toHaveValue("50");
        await page.getByRole("button", { name: "previous" }).click();
        await expect(size).toHaveValue("55");
    });

    test("the region's own slider asks for a value on the same steps", async ({ page }) => {
        await waitForReady(page);
        const region = page.getByTestId("workbench-gpu");
        await settle(region);
        const box = (await region.boundingBox())!;
        const controls = (await snapshot(page)).controls!;

        // Where the region says it drew its track, not where a second copy of
        // its layout guesses.
        const track = controls.size;
        const at = (fraction: number) => ({
            x: box.x + track.x + track.width * fraction,
            y: box.y + track.y + track.height * 0.5,
        });

        // 0.47 of the range is 47, which is not one of the steps of five.
        const spot = at(0.47);
        await page.mouse.click(spot.x, spot.y);
        await expect.poll(async () => (await snapshot(page)).size).toBe(45);
        await expect(page.getByRole("slider", { name: "size" })).toHaveValue("45");

        // Both ends are reachable, and neither goes past the range.
        await page.mouse.click(at(0).x, at(0).y);
        await expect.poll(async () => (await snapshot(page)).size).toBe(0);
        await page.mouse.click(at(1).x, at(1).y);
        await expect.poll(async () => (await snapshot(page)).size).toBe(100);
    });

    test("the region's own checkbox asks, and the application answers", async ({ page }) => {
        await waitForReady(page);
        const region = page.getByTestId("workbench-gpu");
        await settle(region);
        const box = (await region.boundingBox())!;
        const controls = (await snapshot(page)).controls!;
        const centre = {
            x: box.x + controls.locked.x + controls.locked.width * 0.5,
            y: box.y + controls.locked.y + controls.locked.height * 0.5,
        };

        expect(await snapshot(page)).toMatchObject({ locked: false });
        await page.mouse.click(centre.x, centre.y);
        await expect.poll(async () => (await snapshot(page)).locked).toBe(true);
        // The DOM checkbox bound to the same value moved with it.
        await expect(page.getByRole("checkbox", { name: "locked" })).toBeChecked();

        await page.mouse.click(centre.x, centre.y);
        await expect.poll(async () => (await snapshot(page)).locked).toBe(false);
        await expect(page.getByRole("checkbox", { name: "locked" })).not.toBeChecked();
    });
});

test.describe("M6 V7: a failure that says what to do about it", () => {
    test("the retry the application defined asks the same question again", async ({ page }) => {
        await waitForReady(page);
        const details = page.getByTestId("details");
        const retry = page.getByTestId("details-retry");

        // No retry while there is nothing to retry.
        await page.evaluate(() => window.__property_workbench.start_load(40, "the details"));
        await expect(details).toHaveAttribute("data-state", "ready");
        await expect(retry).toHaveCount(0);

        await page.evaluate(() => window.__property_workbench.start_load(40, "error"));
        await expect(details).toHaveAttribute("data-state", "error");
        await expect(details).toContainText("try again");
        await expect(retry).toHaveCount(1);

        await retry.click();
        // The retry is a new request: it goes through loading to an answer.
        await expect(details).toHaveAttribute("data-state", "ready");
        expect(await snapshot(page)).toMatchObject({
            details: "ready",
            details_value: "the details",
        });
        await expect(retry).toHaveCount(0);
    });
});

test.describe("M6 V7: a local theme override", () => {
    test("twenty rounds change one area and nothing around it", async ({ page }) => {
        test.setTimeout(300_000);
        await waitForReady(page);
        const panelBefore = await styleOf(page, ".workbench .panel");
        const scopeBefore = await styleOf(page, "[data-rustify-scope]");
        const hostBefore = await styleOf(page, "body");
        const region = page.getByTestId("workbench-gpu");
        const regionBefore = await settle(region);

        for (let round = 0; round < 20; round++) {
            await page.getByRole("button", { name: "emphasise the summary" }).click();
            const on = round % 2 === 0;
            const emphasis = await styleOf(page, "[data-testid='emphasis']");
            if (on) {
                expect(emphasis.foreground).toBe("#b42318");
                expect(emphasis.spacing).toBe("2px");
                expect(emphasis.fontSize).toBe("12px");
            } else {
                // Back to what it inherits: the scope's own values.
                expect(emphasis.foreground).toBe(scopeBefore.foreground);
                expect(emphasis.spacing).toBe(scopeBefore.spacing);
                expect(emphasis.fontSize).toBe(panelBefore.fontSize);
            }
            // Nothing outside the override moved, in either state.
            expect(await styleOf(page, ".workbench .panel")).toEqual(panelBefore);
            expect(await styleOf(page, "[data-rustify-scope]")).toEqual(scopeBefore);
            expect(await styleOf(page, "body")).toEqual(hostBefore);
        }
        // Twenty rounds end where they started, and the region never saw one.
        expect(await styleOf(page, "[data-testid='emphasis']")).toMatchObject({
            foreground: scopeBefore.foreground,
        });
        expect(differingPixels(regionBefore, await capture(region))).toBe(0);
    });

    test("another scope on the page keeps its own theme", async ({ page }) => {
        test.setTimeout(300_000);
        await waitForReady(page);
        const first = "[data-rustify-scope='workbench']";
        const second = "[data-rustify-scope='second-workbench']";
        const handle = await page.evaluate(() =>
            window.__property_workbench.mount_into("second-workbench")
        );
        expect(handle).toBeGreaterThan(0);
        await expect(page.locator(second)).toHaveCount(1);

        const firstBefore = await styleOf(page, first);
        // The second scope switches to the dark theme; the first is untouched.
        await page.locator(`${second} [data-testid="toggle-theme"]`).click();
        await expect(page.locator(second)).toHaveAttribute("data-theme", "dark");
        await expect(page.locator(first)).toHaveAttribute("data-theme", "light");
        expect(await styleOf(page, first)).toEqual(firstBefore);

        // And an override inside the second scope reaches neither the first
        // scope nor the document.
        await page.locator(`${second} [data-testid="toggle-emphasis"]`).click();
        expect(
            await page.evaluate(
                (first) =>
                    getComputedStyle(
                        document.querySelector(`${first} [data-testid="emphasis"]`) as HTMLElement
                    ).getPropertyValue("--foreground").trim(),
                first
            )
        ).toBe(firstBefore.foreground);
        expect(
            await page.evaluate(() => document.documentElement.getAttribute("data-theme"))
        ).toBeNull();

        await page.evaluate((id) =>
            window.__property_workbench.dispose_handle(id, "second-workbench"), handle);
        await expect(page.locator(second)).toHaveCount(0);
        // The first scope's page-level seams survived the second one's teardown.
        expect(await page.evaluate(() => window.__property_workbench.lookup_object(3))).toBe("found");
        expect(await styleOf(page, first)).toEqual(firstBefore);
    });
});

test.describe("M6 V7 / R07: a third-party component rebuilt twenty times", () => {
    test("each rebuild leaves one component and one subscription", async ({ page }) => {
        test.setTimeout(600_000);
        const failures: string[] = [];
        page.on("pageerror", (error) => failures.push(String(error)));
        await waitForReady(page);

        const stats = () => page.evaluate(() => window.__property_workbench.third_party());
        // The first render, not a rebuild: the panel around this component has
        // grown a twenty-field form since this was written, and the default
        // five seconds is now a near miss rather than a margin. The rebuilds
        // below keep the default, because that is what the test is about.
        const started = Date.now();
        await expect.poll(async () => (await stats()).targets, { timeout: 30_000 }).toBe(1);
        console.log(`third-party first render: ${Date.now() - started} ms`);
        expect(await stats()).toMatchObject({ live: 1, created: 1, destroyed: 0 });

        for (let round = 0; round < 20; round++) {
            // Focus stays where the user put it across a rebuild.
            await page.getByRole("textbox", { name: "go to object" }).focus();
            await page.evaluate(() => window.__property_workbench.set_third_party(false));
            await expect.poll(async () => (await stats()).targets).toBe(0);
            expect(await stats()).toMatchObject({ live: 0, destroyed: round + 1 });
            await page.evaluate(() => window.__property_workbench.set_third_party(true));
            await expect.poll(async () => (await stats()).targets).toBe(1);
            expect(await stats()).toMatchObject({ live: 1, created: round + 2 });
            expect(
                await page.evaluate(() => document.activeElement?.getAttribute("data-testid"))
            ).toBe("find-object-id");
        }

        // One live component means one callback for one change: a rebuild that
        // left a subscription behind would report two.
        const before = (await snapshot(page)).third_party_updates;
        expect(await page.evaluate(() => window.__property_workbench.nudge_third_party(35))).toBe(1);
        await expect.poll(async () => (await snapshot(page)).size).toBe(35);
        expect((await snapshot(page)).third_party_updates).toBe(before + 1);
        // And the value it reported is the application's now, so the SDK's own
        // slider shows it too.
        await expect(page.getByRole("slider", { name: "size" })).toHaveValue("35");
        expect(failures).toEqual([]);
    });
});

test.describe("M6: select, edit, confirm", () => {
    test("the main path from a pointer on the region to a committed value", async ({ page }) => {
        await waitForReady(page);
        const region = page.getByTestId("workbench-gpu");
        await settle(region);
        const box = (await region.boundingBox())!;

        // Select: a real click on the region picks an object.
        await page.mouse.click(box.x + box.width * 0.5, box.y + box.height * 0.7);
        const picked = (await snapshot(page)).selected!;
        expect(picked).not.toBe(1);

        // Edit: the region hands its name rectangle to a real text control.
        await page.mouse.click(box.x + 250, box.y + box.height * 0.06);
        await expect.poll(async () => (await snapshot(page)).editing).toBe(true);
        const field = page.getByTestId("gpu-name-edit");
        await field.fill("confirmed on the region");

        // Confirm: the value reaches the application and the session ends.
        await field.press("Enter");
        await expect.poll(async () => (await snapshot(page)).editing).toBe(false);
        expect(await snapshot(page)).toMatchObject({
            selected: picked,
            name: "confirmed on the region",
        });
        // And the panel bound to the same value shows it.
        await expect(page.getByRole("textbox", { name: "name" })).toHaveValue(
            "confirmed on the region"
        );
    });

    test("a confirmed value the application refuses leaves the old one in both halves", async ({
        page,
    }) => {
        await waitForReady(page);
        const region = page.getByTestId("workbench-gpu");
        await settle(region);
        const box = (await region.boundingBox())!;
        await page.mouse.click(box.x + 250, box.y + box.height * 0.06);
        await expect.poll(async () => (await snapshot(page)).editing).toBe(true);

        const field = page.getByTestId("gpu-name-edit");
        await field.fill("object-0002");
        await field.press("Enter");
        await expect.poll(async () => (await snapshot(page)).editing).toBe(false);
        await expect(page.getByTestId("rejected-value")).toContainText(
            "object 2 already has that name"
        );
        expect(await snapshot(page)).toMatchObject({ name: "object-0001" });
        await expect(page.getByRole("textbox", { name: "name" })).toHaveValue("object-0001");
    });
});

test.describe("M6 V6 again: the theme is values, not a different set of controls", () => {
    test("thirty switches leave every control where a name can find it", async ({ page }) => {
        test.setTimeout(600_000);
        await waitForReady(page);
        const named: [string, string][] = [
            ["textbox", "name"],
            ["textbox", "object id"],
            ["textbox", "go to object"],
            ["checkbox", "locked"],
            ["slider", "size"],
            ["button", "previous"],
            ["button", "next"],
            ["button", "delete selected"],
            ["button", "switch theme"],
            ["status", "find result"],
        ];
        for (let round = 0; round < 30; round++) {
            await page.getByRole("button", { name: "switch theme" }).click();
            await expect
                .poll(async () => (await snapshot(page)).theme)
                .toBe(round % 2 === 0 ? "dark" : "light");
            for (const [role, name] of named) {
                await expect(
                    page.getByRole(role as "button", { name, exact: true }),
                    `${name} after switch ${round}`
                ).toHaveCount(1);
            }
        }
        // Thirty switches end where they started, and the region still answers
        // a pointer at the same place.
        expect(await snapshot(page)).toMatchObject({ theme: "light" });
        const region = page.getByTestId("workbench-gpu");
        await settle(region);
        const box = (await region.boundingBox())!;
        await page.mouse.click(box.x + box.width * 0.5, box.y + box.height * 0.7);
        const picked = (await snapshot(page)).selected!;
        await page.getByRole("textbox", { name: "go to object" }).fill(String(picked));
        await page.getByRole("button", { name: "go", exact: true }).click();
        await expect(page.getByRole("status", { name: "find result" })).toHaveText(
            `selected object ${picked}`
        );
    });
});
