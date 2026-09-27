import { expect, Page } from "@playwright/test";
import { B3, LOCATORS } from "./loads";
import * as twin from "./dataset";
import { isShared, test, waitForQuiet } from "./support";
import { pick, rounds } from "../tier";

/// M4 · the scene: ten thousand objects, and the pointer over them.
///
/// What is checked here is that the picture and the arithmetic agree. The
/// region draws a window of the scene and reports what it drew; the marquee,
/// the pick and the camera are worked out a second time in `dataset.ts` from
/// the layout in the plan, and the two have to say the same thing.

const api = (page: Page) => page.evaluate(() => window.__data_workbench.snapshot());

/// How many of the chosen objects the page lists. The count beside the list
/// is of all of them.
const LISTED = 50;

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

/// Opens the scene and waits until the region has drawn: until then its pane
/// is nothing, and a camera clamped against nothing is clamped to zero.
///
/// Then until it has finished starting: its first draws build a font atlas,
/// which on a software rasteriser holds the main thread for seconds, and a
/// check that counts frames or waits on a click inside that window is
/// measuring the start-up rather than the scene.
async function openScene(page: Page) {
    await page.setViewportSize({ width: 1440, height: 1200 });
    await arrive(page, "/scene");
    await expect(page.getByTestId("status")).toHaveAttribute("data-status", "ready", {
        timeout: 120_000,
    });
    await expect(page.getByTestId("scene-view")).toHaveCount(1);
    await expect
        .poll(async () => (await api(page)).scene.drawn, { timeout: 60_000 })
        .toBeGreaterThan(0);
    await waitForQuiet(page);
}

test.describe("M4 · what a frame shows", () => {
    test("a camera change is presented in the frame it was made in", async ({ page }) => {
        await openScene(page);
        const run = await page.evaluate(
            ({ step, extent }) =>
                new Promise<{ drives: number; presentations: number }>((resolve) => {
                    const api = window.__data_workbench;
                    let drives = 0;
                    let presentations = 0;
                    let seen = api.stats().frames;
                    let x = 0;
                    let y = 0;
                    const started = performance.now();
                    const tick = (now: number) => {
                        x = x + step[0] > extent[0] ? 0 : x + step[0];
                        y = y + step[1] > extent[1] ? 0 : y + step[1];
                        api.look_at(x, y);
                        drives += 1;
                        // Read after the drive and inside the same callback:
                        // a region that drew in this frame has counted its
                        // presentation before the callback returns.
                        const frames = api.stats().frames;
                        if (frames !== seen) {
                            seen = frames;
                            presentations += 1;
                        }
                        if (now - started >= 3_000) {
                            resolve({ drives, presentations });
                            return;
                        }
                        requestAnimationFrame(tick);
                    };
                    requestAnimationFrame(tick);
                }),
            { step: B3.panPerFrame, extent: B3.extent }
        );
        console.log(`  ${run.presentations} presentations for ${run.drives} drives`);
        // A software rasteriser draws this scene in about seventy
        // milliseconds, so three seconds of it is tens of frames rather than a
        // hundred and eighty. How fast it can go is A-3's question and M8's
        // gate; what this one is about is the ratio. The floor only makes the
        // ratio a sample worth having, so it scales with the tier like any
        // other repeat count: a slower rasteriser still gives a regression
        // run a few frames to check.
        expect(run.drives).toBeGreaterThan(rounds(30));
        // The clause NFR-1 is written with: a frame that was driven is a frame
        // that was drawn. One drive may still be in flight when the run ends.
        expect(run.presentations).toBeGreaterThanOrEqual(run.drives - 1);
    });
});

/// Where the scene's pane is on the screen, and what it is looking at.
///
/// Both are needed to put the pointer on an object: the objects are at fixed
/// places in the scene, and the camera is what decides where on the screen
/// that is.
async function pane(page: Page) {
    const box = (await page.getByTestId("scene-gpu").boundingBox())!;
    const snapshot = await api(page);
    return {
        box,
        camera: snapshot.scene.camera as [number, number],
        size: snapshot.scene.pane as [number, number],
    };
}

type Pane = Awaited<ReturnType<typeof pane>>;

/// A point of the scene, as a point of the screen.
function onScreen(at: Pane, x: number, y: number) {
    return { x: at.box.x + x - at.camera[0], y: at.box.y + y - at.camera[1] };
}

/// Scattered points of the scene, the same ones every run.
///
/// A stride into a span looks scattered and is not: a hundred and thirty seven
/// into a span of one thousand and ninety six visits eight columns, and the
/// two layers of the scene are a hundred and thirty apart. So the points come
/// from a generator instead, seeded the same way each time.
function scatter(seed: number): () => number {
    let state = seed >>> 0;
    return () => {
        state ^= state << 13;
        state >>>= 0;
        state ^= state >>> 17;
        state ^= state << 5;
        state >>>= 0;
        return state;
    };
}

/// The same, measured now. Used in the loops that change the document as they
/// go, where a pane measured before the first round is not where the pane is
/// by the tenth.
async function pointAt(page: Page, x: number, y: number) {
    return onScreen(await pane(page), x, y);
}

/// Points the camera somewhere and waits for the region to draw from there.
async function lookAt(page: Page, x: number, y: number) {
    await page.evaluate(([x, y]) => window.__data_workbench.look_at(x, y), [x, y] as const);
    await expect.poll(async () => (await api(page)).scene.camera).toEqual([x, y]);
}

test.describe("M4 · the pointer on ten thousand objects", () => {
    test("twenty boxes take what a second implementation says they take", async ({ page }) => {
        await openScene(page);
        await lookAt(page, 800, 400);
        const at = await pane(page);
        for (let round = 0; round < rounds(20); round++) {
            // Starting on an object is what makes it a box rather than a pan,
            // so every one of these starts in the middle of one - and in the
            // part of the scene this camera can see.
            const index = (11 + (round % 6)) * 100 + 7 + (round % 4);
            const object = twin.sceneRect(index);
            const from = {
                x: object.x + twin.SCENE.width / 2,
                y: object.y + twin.SCENE.height / 2,
            };
            const to = {
                x: from.x + 30 + ((round * 53) % 300),
                y: from.y + 20 + ((round * 31) % 240),
            };
            const start = await pointAt(page, from.x, from.y);
            const end = await pointAt(page, to.x, to.y);
            await page.mouse.move(start.x, start.y);
            await page.mouse.down();
            await page.mouse.move((start.x + end.x) / 2, (start.y + end.y) / 2);
            await page.mouse.move(end.x, end.y);
            await page.mouse.up();
            await expect.poll(async () => (await api(page)).scene.marquees).toBe(round + 1);
            const expected = twin.sceneWithin(from.x, from.y, to.x - from.x, to.y - from.y);
            const chosen = await page.evaluate(() => window.__data_workbench.scene_chosen());
            expect(chosen, `box ${round} from (${from.x}, ${from.y})`).toEqual(expected);
        }
    });

    test("a box with shift held adds to what was already chosen", async ({ page }) => {
        await openScene(page);
        const at = await pane(page);
        const first = twin.sceneRect(0);
        const second = twin.sceneRect(304);
        const drag = async (index: number, additive: boolean) => {
            const rect = twin.sceneRect(index);
            const from = await pointAt(page, rect.x + 2, rect.y + 2);
            const to = await pointAt(
                page,
                rect.x + twin.SCENE.width - 2,
                rect.y + twin.SCENE.height - 2
            );
            if (additive) {
                await page.keyboard.down("Shift");
            }
            await page.mouse.move(from.x, from.y);
            await page.mouse.down();
            await page.mouse.move(to.x, to.y);
            await page.mouse.up();
            if (additive) {
                await page.keyboard.up("Shift");
            }
        };
        await drag(0, false);
        await expect.poll(async () => (await api(page)).scene.chosen).toBe(1);
        expect(await page.evaluate(() => window.__data_workbench.scene_chosen())).toEqual([0]);
        await drag(304, true);
        expect(first).not.toEqual(second);
        await expect.poll(async () => (await api(page)).scene.chosen).toBe(2);
        expect(await page.evaluate(() => window.__data_workbench.scene_chosen())).toEqual([0, 304]);
    });

    test("a hundred steps of the pointer move the camera by a hundred steps", async ({ page }) => {
        await openScene(page);
        await lookAt(page, 1_400, 800);
        const at = await pane(page);
        // Empty space. Between the bottom of one row of objects and the top of
        // the next there is a band that nothing reaches: the shifted layer
        // stops four pixels above it, and the columns it shifts into are a
        // thousand pixels to the left of here.
        const empty = { x: at.camera[0] + 100, y: Math.floor(at.camera[1] / 40) * 40 + 32 };
        expect(
            twin.scenePick(empty.x, empty.y),
            "the pan has to start on nothing"
        ).toBeNull();
        const gap = onScreen(at, empty.x, empty.y);
        await page.mouse.move(gap.x, gap.y);
        await page.mouse.down();
        const acceptedBefore = (await api(page)).scene.accepted;
        // What the region was actually given, counted beside it. The browser
        // may coalesce pointer moves, so a hundred dispatched is not a hundred
        // delivered - and what the frame gate compares is drives against
        // answers, not dispatches against answers. Counted from here, because
        // the move that brought the pointer to the starting point is not a
        // step of the drag.
        await page.evaluate(() => {
            const canvas = document.querySelector(
                '[data-testid="scene-gpu"]'
            ) as HTMLCanvasElement & { moves?: number };
            canvas.moves = 0;
            canvas.addEventListener("pointermove", () => {
                canvas.moves = (canvas.moves ?? 0) + 1;
            });
        });
        const STEP = [7, 3];
        const steps = rounds(100);
        let x = gap.x;
        let y = gap.y;
        for (let step = 0; step < steps; step++) {
            x += STEP[0];
            y += STEP[1];
            await page.mouse.move(x, y);
        }
        await page.mouse.up();
        // Dragging the scene one way moves the camera the other.
        const expected = twin.sceneClamp(
            at.camera[0] - STEP[0] * steps,
            at.camera[1] - STEP[1] * steps,
            at.size[0],
            at.size[1]
        );
        await expect.poll(async () => (await api(page)).scene.camera).toEqual(expected);
        // Every move that arrived was answered. This is the count the frame
        // gate compares its own drives against: a region that answered half of
        // them has not kept up, whatever its frame interval says.
        const moves = await page.evaluate(
            () =>
                (
                    document.querySelector(
                        '[data-testid="scene-gpu"]'
                    ) as HTMLCanvasElement & { moves?: number }
                ).moves ?? 0
        );
        expect(moves).toBeGreaterThan(0);
        expect((await api(page)).scene.accepted - acceptedBefore).toBe(moves);
    });

    test("ten wheels in one task move the camera by ten wheels", async ({ page }) => {
        await openScene(page);
        await lookAt(page, 1_000, 800);
        const wheel = (times: number) =>
            page.evaluate((times) => {
                const canvas = document.querySelector('[data-testid="scene-gpu"]')!;
                const box = canvas.getBoundingClientRect();
                // Dispatched in one turn, with nothing awaited between them:
                // what is being checked is that a burst the browser never gave
                // the application a chance to answer still adds up.
                for (let i = 0; i < times; i++) {
                    canvas.dispatchEvent(
                        new WheelEvent("wheel", {
                            deltaX: 11,
                            deltaY: 5,
                            deltaMode: 0,
                            clientX: box.x + box.width / 2,
                            clientY: box.y + box.height / 2,
                            bubbles: true,
                            cancelable: true,
                        })
                    );
                }
            }, times);
        const before = (await api(page)).scene.camera;
        const acceptedBefore = (await api(page)).scene.accepted;
        await wheel(1);
        await expect.poll(async () => (await api(page)).scene.camera).not.toEqual(before);
        const once = (await api(page)).scene.camera;
        const step = [once[0] - before[0], once[1] - before[1]];
        expect(step[0]).toBeGreaterThan(0);
        await wheel(10);
        const expected = [once[0] + step[0] * 10, once[1] + step[1] * 10];
        await expect.poll(async () => (await api(page)).scene.camera).toEqual(expected);
        // Eleven wheels, eleven answers: a burst that never gave the browser a
        // turn was still delivered one at a time rather than collapsed into
        // its last value.
        expect((await api(page)).scene.accepted - acceptedBefore).toBe(11);
    });

    test("a hundred clicks pick what is under them, upper layer first", async ({ page }) => {
        await openScene(page);
        await lookAt(page, 800, 400);
        const at = await pane(page);
        const points: { x: number; y: number; index: number }[] = [];
        const random = scatter(0x2545_f491);
        while (points.length < 100) {
            const x = at.camera[0] + 10 + (random() % Math.floor(at.size[0] - 20));
            const y = at.camera[1] + 10 + (random() % Math.floor(at.size[1] - 20));
            const index = twin.scenePick(x, y);
            // A click on nothing is a pan that went nowhere, not a pick.
            if (index !== null) {
                points.push({ x, y, index });
            }
        }
        // Fewer of them stand for all of them: the first on the upper layer
        // and the first two on the lower.
        const clicked = pick(points, [
            points.find((point) => twin.isOverlay(point.index))!,
            ...points.filter((point) => !twin.isOverlay(point.index)).slice(0, 2),
        ]);
        // The scene has two layers, and a check that never lands on the upper
        // one has not checked the rule that decides between them.
        expect(clicked.filter((point) => twin.isOverlay(point.index)).length).toBeGreaterThan(0);
        for (const [round, point] of clicked.entries()) {
            const screen = await pointAt(page, point.x, point.y);
            await page.mouse.click(screen.x, screen.y);
            await expect
                .poll(async () => (await api(page)).scene.picks, {
                    message: `click ${round} at (${point.x}, ${point.y})`,
                })
                .toBe(round + 1);
            const snapshot = await api(page);
            expect(snapshot.scene.editing, `click ${round}`).toBe(point.index);
            expect(await page.evaluate(() => window.__data_workbench.scene_chosen())).toEqual([
                point.index,
            ]);
        }
    });

    test("the pointer reports what it is over, and only the latest of it", async ({ page }) => {
        await openScene(page);
        const at = await pane(page);
        const over = [0, 1, 2, 100, 101];
        for (const index of over) {
            const rect = twin.sceneRect(index);
            const screen = onScreen(at, rect.x + 10, rect.y + 10);
            await page.mouse.move(screen.x, screen.y);
        }
        await expect.poll(async () => (await api(page)).scene.hover).toBe(over[over.length - 1]);
        await expect(page.getByTestId("scene-hover")).toHaveText(
            twin.sceneLabel(over[over.length - 1])
        );
        // The gap between two rows is over nothing, and nothing is a value.
        const gapY = 70;
        expect(twin.scenePick(300, gapY)).toBeNull();
        const gap = onScreen(at, 300, gapY);
        await page.mouse.move(gap.x, gap.y);
        await expect(page.getByTestId("scene-hover")).toHaveText("nothing");
    });
});

test.describe("M4 · reaching an object without the pointer", () => {
    test("the first, the middle and the last object are findable and renameable", async ({
        page,
    }) => {
        await openScene(page);
        for (const [round, label] of B3.targets.entries()) {
            const index = Number(label.slice(3));
            await page.getByTestId("scene-find").fill(label);
            await page.getByTestId("scene-find").press("Enter");
            await expect.poll(async () => (await api(page)).scene.editing).toBe(index);
            await expect(page.getByTestId("scene-detail-object")).toHaveText(label);
            // The camera went there, and the object is inside what it can see.
            const snapshot = await api(page);
            const visible = twin.sceneVisible(
                snapshot.scene.camera[0],
                snapshot.scene.camera[1],
                snapshot.scene.pane[0],
                snapshot.scene.pane[1]
            );
            expect(snapshot.scene.drawn).toBe(visible);
            const rect = twin.sceneRect(index);
            expect(rect.x).toBeGreaterThanOrEqual(snapshot.scene.camera[0]);
            expect(rect.x + twin.SCENE.width).toBeLessThanOrEqual(
                snapshot.scene.camera[0] + snapshot.scene.pane[0]
            );
            const renamed = `FOUND-${index}`;
            await page.getByTestId("scene-detail-label").fill(renamed);
            await page.getByTestId("scene-detail-submit").click();
            await expect.poll(async () => (await api(page)).scene.renames).toBe(round + 1);
            expect(
                await page.evaluate((id) => window.__data_workbench.scene_label(id), index)
            ).toBe(renamed);
            await expect(page.getByTestId("scene-detail-object")).toHaveText(renamed);
        }
    });

    test("finding an object and pointing at it reach the same object", async ({ page }) => {
        await openScene(page);
        const index = 5_000;
        await page.getByTestId("scene-find").fill(twin.sceneLabel(index));
        await page.getByTestId("scene-find").press("Enter");
        await expect.poll(async () => (await api(page)).scene.editing).toBe(index);
        await page.getByTestId("scene-detail-label").fill("by the query");
        await page.getByTestId("scene-detail-submit").click();
        await expect.poll(async () => (await api(page)).scene.renames).toBe(1);

        // The same object, reached the other way: the camera is already on it,
        // so the pointer only has to land on it.
        const at = await pane(page);
        const rect = twin.sceneRect(index);
        const screen = onScreen(at, rect.x + 10, rect.y + 10);
        expect(twin.scenePick(rect.x + 10, rect.y + 10)).toBe(index);
        await page.mouse.click(screen.x, screen.y);
        await expect.poll(async () => (await api(page)).scene.picks).toBe(1);
        expect((await api(page)).scene.editing).toBe(index);
        await expect(page.getByTestId("scene-detail-label")).toHaveValue("by the query");
        await page.getByTestId("scene-detail-label").fill("by the pointer");
        await page.getByTestId("scene-detail-submit").click();
        await expect.poll(async () => (await api(page)).scene.renames).toBe(2);
        expect(await page.evaluate((id) => window.__data_workbench.scene_label(id), index)).toBe(
            "by the pointer"
        );
    });

    test("a hundred each way, none lost and none counted twice", async ({ page }) => {
        await openScene(page);
        // DOM to GPU: a hundred queries, each one moving the camera and the
        // mark on the object it found.
        const queries = rounds(100);
        for (let round = 0; round < queries; round++) {
            const index = round * 97;
            await page.getByTestId("scene-find").fill(twin.sceneLabel(index));
            await page.getByTestId("scene-find").press("Enter");
            await expect
                .poll(async () => (await api(page)).scene.editing, { message: `query ${round}` })
                .toBe(index);
        }
        expect((await api(page)).scene.asked).toBe(queries);

        // GPU to DOM: a hundred clicks, each one changing the list and the
        // count the document shows.
        await lookAt(page, 800, 400);
        const at = await pane(page);
        const picked: number[] = [];
        // More than the fifty the list shows, however few rounds there are:
        // what is not listed still has to be counted, and it takes more than
        // fifty to say.
        const clicks = Math.max(rounds(100), LISTED + 1);
        const random = scatter(0x1b87_3593);
        while (picked.length < clicks) {
            const x = at.camera[0] + 10 + (random() % Math.floor(at.size[0] - 20));
            const y = at.camera[1] + 10 + (random() % Math.floor(at.size[1] - 20));
            const index = twin.scenePick(x, y);
            if (index === null || picked.includes(index)) {
                continue;
            }
            picked.push(index);
            const screen = await pointAt(page, x, y);
            await page.keyboard.down("Shift");
            await page.mouse.click(screen.x, screen.y);
            await page.keyboard.up("Shift");
            await expect
                .poll(async () => (await api(page)).scene.picks, {
                    message: `click ${picked.length}`,
                })
                .toBe(picked.length);
        }
        expect((await api(page)).scene.chosen).toBe(clicks);
        expect(await page.evaluate(() => window.__data_workbench.scene_chosen())).toEqual(
            [...picked].sort((a, b) => a - b)
        );
        await expect(page.getByTestId("scene-selected-count")).toHaveText(`selected ${clicks}`);
        // Fifty of them are listed and the count says all of them: what is not
        // shown is still said. Which fifty matters too - a list that shows the
        // wrong objects is worse than one that shows fewer.
        const listed = await page.evaluate(() =>
            Array.from(
                document.querySelectorAll('[data-testid="scene-selected"] li'),
                (item) => Number(item.getAttribute("data-object"))
            )
        );
        expect(listed).toEqual([...picked].sort((a, b) => a - b).slice(0, LISTED));
        await page.getByTestId("scene-clear").click();
        await expect(page.getByTestId("scene-selected-count")).toHaveText("selected 0");
    });
});

test.describe("M4 · twenty names, thirty times over", () => {
    // Fourteen of the twenty are the table's, and p3-table checks those the
    // same way; what is left here is the scene's six.
    test("every one of the twenty is findable by role and name", async ({ page }) => {
        expect(LOCATORS).toHaveLength(20);
        const check = async (round: number) => {
            for (const entry of LOCATORS.filter((entry) => entry.side === "scene")) {
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
        };
        await openScene(page);
        // Two of the six are the object being edited, so one is open: a name
        // nothing is showing is not a name that is missing.
        await page.getByTestId("scene-find").fill(twin.sceneLabel(0));
        await page.getByTestId("scene-find").press("Enter");
        await expect.poll(async () => (await api(page)).scene.editing).toBe(0);
        for (let round = 0; round < rounds(30); round++) {
            await check(round);
            // Something that redraws the scene between rounds, so what is
            // counted is the name surviving rather than the same document
            // standing still.
            await lookAt(page, (round % 10) * 100, (round % 7) * 80);
        }
    });
});
