import { expect, Page, test } from "@playwright/test";

import { sharedPage } from "./support";

/// M6 V8: carrying an object from the DOM into the region, and back out.
///
/// Everything here is counted rather than looked at. A drop is a fact the
/// application records - one per delivered drag, never two - so the check is
/// the count, and the count is read from inside the page. A drag measured
/// through the harness would be measuring the harness: the fifty pointer
/// events of one drag cost more to dispatch from Playwright than the whole
/// thing costs to run, so each sequence is dispatched inside one `evaluate`.

const snapshot = (page: Page) => page.evaluate(() => window.__property_workbench.snapshot());
const drag = async (page: Page) => (await snapshot(page)).drag;

/// Where each of the things a drag deals with is, in viewport pixels.
///
/// The group list's rectangle comes from the region, which drew it. Every time
/// something in this project guessed at geometry the region already knew, it
/// broke the first time a panel changed width.
async function places(page: Page) {
    const controls = (await snapshot(page)).controls;
    expect(controls, "the region has reported where it drew").not.toBeNull();
    return page.evaluate(
        ({ groups, row }) => {
            const box = (selector: string) => {
                const element = document.querySelector(selector) as HTMLElement;
                const rect = element.getBoundingClientRect();
                return { x: rect.left + rect.width / 2, y: rect.top + rect.height / 2, rect };
            };
            const canvas = box('[data-testid="workbench-gpu"]');
            const list = {
                left: canvas.rect.left + groups.x,
                top: canvas.rect.top + groups.y,
                width: groups.width,
                height: groups.height,
            };
            return {
                bin: box('[data-testid="ungrouped-bin"]'),
                canvas,
                list,
                // The middle of the first row.
                group: { x: list.left + list.width / 2, y: list.top + row / 2 },
                // The middle of the second, for a drag that has to land
                // somewhere else.
                group2: { x: list.left + list.width / 2, y: list.top + row * 1.5 },
                // Far from every target: the middle of the properties heading.
                nowhere: box(".panel h2"),
            };
        },
        { groups: controls!.groups, row: controls!.row }
    );
}

interface Step {
    x: number;
    y: number;
}

/// One whole drag, dispatched inside the page.
///
/// `steps` are where the pointer goes between the press and the release; the
/// last one is where it is released. `escape` presses Escape instead of
/// releasing, which is how a drag is abandoned.
async function dragObject(
    page: Page,
    object: number,
    steps: Step[],
    options: { escape?: boolean } = {}
) {
    await page.evaluate(
        ({ object, steps, escape }) => {
            const row = document.querySelector(`[data-testid="object-${object}"]`) as HTMLElement;
            const box = row.getBoundingClientRect();
            const from = { x: box.left + box.width / 2, y: box.top + box.height / 2 };
            const send = (type: string, at: { x: number; y: number }) =>
                row.dispatchEvent(
                    new PointerEvent(type, {
                        bubbles: true,
                        clientX: at.x,
                        clientY: at.y,
                        pointerId: 1,
                        isPrimary: true,
                        button: 0,
                        buttons: type === "pointerup" ? 0 : 1,
                    })
                );
            send("pointerdown", from);
            for (const step of steps) {
                send("pointermove", step);
            }
            const last = steps[steps.length - 1] ?? from;
            if (escape) {
                window.dispatchEvent(new KeyboardEvent("keydown", { key: "Escape", bubbles: true }));
            } else {
                send("pointerup", last);
            }
        },
        { object, steps, escape: options.escape === true }
    );
}

/// The region answers on its next pump, so a drop that crossed the boundary
/// lands a frame or two after the release. Waiting on the count is waiting on
/// the thing under test rather than on a duration.
async function drops(page: Page) {
    return (await drag(page)).drops;
}

test.describe("M6 V8: an object carried from one half of the page to the other", () => {
    test.describe.configure({ mode: "serial" });
    const shared = sharedPage();

    test("a drag into a group in the region delivers exactly one drop", async () => {
        const page = shared.page;
        const before = await drag(page);
        const at = await places(page);
        await dragObject(page, 1, [
            { x: at.bin.x, y: at.bin.y },
            { x: at.group.x, y: at.group.y },
            { x: at.group.x + 1, y: at.group.y },
        ]);
        await expect.poll(() => drops(page)).toBe(before.drops + 1);
        const after = await drag(page);
        expect(after.grouped).toBe(before.grouped + 1);
        expect(after.dragging).toBe(false);
        // And the drag let go of everything it was showing.
        expect(after.target).toBeNull();
    });

    test("a drag onto a DOM target delivers one drop and takes the group away", async () => {
        const page = shared.page;
        const before = await drag(page);
        const at = await places(page);
        await dragObject(page, 1, [{ x: at.bin.x, y: at.bin.y }]);
        await expect.poll(() => drops(page)).toBe(before.drops + 1);
        expect((await drag(page)).grouped).toBe(before.grouped - 1);
    });

    test("a hundred drags into the region deliver exactly a hundred drops", async () => {
        const page = shared.page;
        const before = await drag(page);
        const at = await places(page);
        for (let n = 0; n < 100; n += 1) {
            await dragObject(page, 1, [
                { x: at.canvas.x, y: at.canvas.y },
                { x: at.group.x, y: at.group.y + (n % 3) },
            ]);
            await expect.poll(() => drops(page)).toBe(before.drops + n + 1);
        }
        const after = await drag(page);
        expect(after.drops).toBe(before.drops + 100);
        // A hundred drops of the same object into the same list is one object
        // in one group, not a hundred: a drop is a move, not an addition.
        expect(after.grouped).toBe(before.grouped);
        expect(after.cancels).toBe(before.cancels);
    });

    test("releasing over nothing delivers nothing, twenty times", async () => {
        const page = shared.page;
        const before = await drag(page);
        const at = await places(page);
        for (let n = 0; n < 20; n += 1) {
            await dragObject(page, 2, [
                { x: at.group.x, y: at.group.y },
                { x: at.nowhere.x, y: at.nowhere.y },
            ]);
        }
        const after = await drag(page);
        expect(after.drops).toBe(before.drops);
        expect(after.grouped).toBe(before.grouped);
        expect(after.dragging).toBe(false);
    });

    test("escape abandons a drag over a target that had already agreed", async () => {
        const page = shared.page;
        const before = await drag(page);
        const at = await places(page);
        for (let n = 0; n < 20; n += 1) {
            await dragObject(page, 3, [{ x: at.group.x, y: at.group.y }], { escape: true });
        }
        const after = await drag(page);
        expect(after.cancels).toBe(before.cancels + 20);
        expect(after.drops).toBe(before.drops);
        expect(after.grouped).toBe(before.grouped);
    });

    test("a target that refuses takes nothing, twenty times", async () => {
        const page = shared.page;
        // The bin refuses a locked object. Locking is a business rule, so the
        // refusal is the application's, not the control's.
        await page.getByTestId("object-4").click();
        await page.getByTestId("locked-input").click();
        await expect.poll(async () => (await snapshot(page)).locked).toBe(true);

        const before = await drag(page);
        const at = await places(page);
        for (let n = 0; n < 20; n += 1) {
            await dragObject(page, 4, [{ x: at.bin.x, y: at.bin.y }]);
        }
        const after = await drag(page);
        expect(after.drops).toBe(before.drops);
        // And it never claimed it would take it.
        expect(after.target).toBeNull();

        await page.getByTestId("locked-input").click();
        await expect.poll(async () => (await snapshot(page)).locked).toBe(false);
    });

    test("a release the region has not answered yet still lands, twenty times", async () => {
        const page = shared.page;
        const before = await drag(page);
        const at = await places(page);
        for (let n = 0; n < 20; n += 1) {
            // Moved onto the group and released in the same turn: the region
            // has not been pumped in between, so the release is decided by an
            // answer that has not arrived yet.
            await dragObject(page, 5, [
                { x: at.canvas.x, y: at.canvas.y },
                { x: at.group.x, y: at.group.y },
            ]);
            await expect.poll(() => drops(page)).toBe(before.drops + n + 1);
        }
        expect((await drag(page)).drops).toBe(before.drops + 20);
    });

    test("a drag that never moved is a click, and selects", async () => {
        const page = shared.page;
        const before = await drag(page);
        await page.getByTestId("object-7").click();
        await expect.poll(async () => (await snapshot(page)).selected).toBe(7);
        expect((await drag(page)).drops).toBe(before.drops);
    });
});

test.describe("M6 V8: whose wheel event it is", () => {
    test.describe.configure({ mode: "serial" });
    const shared = sharedPage();

    /// Turns the region's scroll into a number the page can read: how far down
    /// the group list is, and how far down it can go.
    const scroll = async (page: Page) => {
        const state = await drag(page);
        return { at: state.scroll, max: state.scroll_max };
    };

    /// A wheel over the group list, and whether the region kept it.
    async function wheelOverGroups(page: Page, deltaY: number): Promise<boolean> {
        const at = (await places(page)).list;
        return page.evaluate(
            ({ at, deltaY }) => {
                const canvas = document.querySelector(
                    '[data-testid="workbench-gpu"]'
                ) as HTMLCanvasElement;
                const event = new WheelEvent("wheel", {
                    bubbles: true,
                    cancelable: true,
                    clientX: at.left + at.width / 2,
                    clientY: at.top + at.height / 2,
                    deltaY,
                    deltaMode: 0,
                });
                canvas.dispatchEvent(event);
                // Cancelled means the region consumed it; not cancelled means
                // the browser is free to scroll whatever contains the region,
                // which is what handing it over looks like from here.
                return event.defaultPrevented;
            },
            { at, deltaY }
        );
    }

    test("the region has more groups than room, so it can reach an end", async () => {
        const page = shared.page;
        await expect.poll(async () => (await scroll(page)).max).toBeGreaterThan(0);
    });

    test("twenty wheels scroll the region until it runs out, and no further", async () => {
        const page = shared.page;
        const limit = (await scroll(page)).max;
        for (let n = 0; n < 20; n += 1) {
            await wheelOverGroups(page, 40);
        }
        await expect.poll(async () => (await scroll(page)).at).toBe(limit);
    });

    test("at the end, the page is given the wheel instead of the region", async () => {
        const page = shared.page;
        expect((await drag(page)).propagates).toBe(true);
        // At the end already, from the test before. The report the host reads
        // arrives from the region's next draw, so the handover is polled for
        // rather than assumed to be in place the same millisecond.
        await expect.poll(() => wheelOverGroups(page, 40)).toBe(false);
    });

    test("a region told to keep them keeps them, even at the end", async () => {
        const page = shared.page;
        await page.getByTestId("wheel-propagates").click();
        await expect.poll(async () => (await drag(page)).propagates).toBe(false);
        // The policy only reaches the host from a draw, so the region has to
        // have drawn once with it before the next wheel is judged.
        await expect.poll(() => wheelOverGroups(page, 40)).toBe(true);

        await page.getByTestId("wheel-propagates").click();
        await expect.poll(async () => (await drag(page)).propagates).toBe(true);
    });

    test("scrolling back up reaches the top and stops there", async () => {
        const page = shared.page;
        for (let n = 0; n < 30; n += 1) {
            await wheelOverGroups(page, -40);
        }
        await expect.poll(async () => (await scroll(page)).at).toBe(0);
    });

    test("the DOM list says the same thing with overscroll-behavior", async () => {
        const page = shared.page;
        const behaviour = () =>
            page.evaluate(
                () =>
                    getComputedStyle(
                        document.querySelector('[data-testid="objects-list"]') as HTMLElement
                    ).overscrollBehaviorY
            );
        expect(await behaviour()).toBe("auto");
        await page.getByTestId("wheel-propagates").click();
        await expect.poll(behaviour).toBe("contain");
        await page.getByTestId("wheel-propagates").click();
        await expect.poll(behaviour).toBe("auto");
    });
});
