import { CDPSession, expect, Page, test } from "@playwright/test";
import { waitForQuiet } from "./support";

/// M6 · what B0 costs when nobody is doing anything.
///
/// Three states, and the difference between them is the whole point: idle is
/// a page nobody is touching, hidden is a page nobody can see, and restored
/// is the moment a person comes back and has to find their own state rather
/// than a minute of replayed input.
///
/// Every figure is taken inside the page. A round trip through the harness
/// costs more than some of the limits here.

const SIXTY_SECONDS = 60_000;

async function openB0(page: Page) {
    await page.goto("./");
    await expect(page.getByTestId("status")).toHaveAttribute("data-status", "ready", {
        timeout: 120_000,
    });
    await expect
        .poll(async () => (await page.evaluate(() => window.__fusion_basic.b0_state())).region, {
            timeout: 60_000,
        })
        .toBe("ready");
    await waitForQuiet(page);
}

const stats = (page: Page) => page.evaluate(() => window.__fusion_basic.stats());

/// Milliseconds of main-thread task time the browser has run for this page.
///
/// `Performance.getMetrics` is the browser's own accounting, which is the
/// only one that sees work the page did not ask for - a poll, a timer, a
/// rasterisation.
async function taskSeconds(cdp: CDPSession): Promise<number> {
    const { metrics } = await cdp.send("Performance.getMetrics");
    const task = metrics.find((metric) => metric.name === "TaskDuration");
    expect(task, "TaskDuration").toBeDefined();
    return task!.value;
}

/// The share of a stretch of wall-clock time the page spent on the main
/// thread, as a percentage.
async function cpuShare(cdp: CDPSession, ms: number, wait: () => Promise<void>): Promise<number> {
    const before = await taskSeconds(cdp);
    await wait();
    const after = await taskSeconds(cdp);
    return (((after - before) * 1000) / ms) * 100;
}

/// Sleeps inside the page, so the wait is the page's own idle time rather
/// than the harness's.
const idle = (page: Page, ms: number) =>
    page.evaluate((ms) => new Promise((resolve) => setTimeout(resolve, ms)), ms);

test.describe("M6 · B0 at rest", () => {
    test("sixty seconds of nothing presents at most one frame", async ({ page }) => {
        test.setTimeout(300_000);
        await openB0(page);
        const cdp = await page.context().newCDPSession(page);
        await cdp.send("Performance.enable");

        const before = await stats(page);
        const busy = await cpuShare(cdp, SIXTY_SECONDS, () => idle(page, SIXTY_SECONDS));
        const after = await stats(page);

        // A blank page in the same browser, measured the same way: what is
        // being asked is what the application costs, not what the browser
        // costs.
        const blank = await page.context().newPage();
        await blank.goto("about:blank");
        const blankCdp = await blank.context().newCDPSession(blank);
        await blankCdp.send("Performance.enable");
        const baseline = await cpuShare(blankCdp, SIXTY_SECONDS, () => idle(blank, SIXTY_SECONDS));
        await blank.close();

        console.log(
            `idle 60 s: frames ${before.frames} -> ${after.frames}, ` +
                `pumps ${before.pumps} -> ${after.pumps}, ` +
                `cpu ${busy.toFixed(3)}% against a blank page's ${baseline.toFixed(3)}%`
        );

        expect(after.frames - before.frames).toBeLessThanOrEqual(1);
        expect(busy - baseline).toBeLessThanOrEqual(1);
        // And it is still a live region, not a stopped one.
        expect(after.regions).toBe(before.regions);
        expect(after.errors).toBe(before.errors);
    });

    test("hidden, it stops within a second and stays stopped for a minute", async ({ page }) => {
        test.setTimeout(300_000);
        await openB0(page);

        const measured = await page.evaluate(async () => {
            const api = window.__fusion_basic;
            const canvas = document.querySelector('[data-testid="b0-gpu"]') as HTMLElement;
            const host = canvas.parentElement as HTMLElement;
            const frames = () => api.stats().frames;
            const at_hide = frames();
            host.hidden = true;
            await new Promise((resolve) => setTimeout(resolve, 1_000));
            const after_a_second = frames();
            await new Promise((resolve) => setTimeout(resolve, 59_000));
            const after_a_minute = frames();

            // Back on screen: the first frame after it, timed in the page.
            const restored_at = performance.now();
            host.hidden = false;
            let took = -1;
            const deadline = restored_at + 5_000;
            for (;;) {
                if (frames() > after_a_minute) {
                    took = performance.now() - restored_at;
                    break;
                }
                if (performance.now() > deadline) {
                    break;
                }
                await new Promise((resolve) => requestAnimationFrame(resolve));
            }
            return { at_hide, after_a_second, after_a_minute, took, count: api.b0_state().count };
        });

        console.log(
            `hidden: frames ${measured.at_hide} -> ${measured.after_a_second} after a second, ` +
                `${measured.after_a_minute} after a minute; back in ${measured.took.toFixed(1)} ms`
        );

        // A region that has just been hidden may still finish the frame it
        // was in; what it may not do is go on presenting.
        expect(measured.after_a_minute).toBe(measured.after_a_second);
        expect(measured.took).toBeGreaterThanOrEqual(0);
        expect(measured.took).toBeLessThan(500);

        // And it answers a real pointer again, which is the only proof that
        // "presented" meant the region and not the page.
        const region = page.getByTestId("b0-gpu");
        await region.scrollIntoViewIfNeeded();
        const control = (await page.evaluate(() => window.__fusion_basic.b0_state())).controls.find(
            (entry) => entry.name === "bump_first"
        )!;
        const canvas = (await region.boundingBox())!;
        await page.mouse.click(
            canvas.x + control.x + control.width / 2,
            canvas.y + control.y + control.height / 2
        );
        await expect
            .poll(async () => (await page.evaluate(() => window.__fusion_basic.b0_state())).count)
            .toBe(measured.count + 1);
    });

    test("a document nobody is looking at stops too", async ({ page }) => {
        test.setTimeout(300_000);
        await openB0(page);

        // The other kind of hidden: the document itself rather than the
        // element. Two ways to ask for it - a second tab in front of this
        // one, and Chrome's own lifecycle state - because whether either
        // works is a property of the browser the harness drives, not of the
        // application. If neither hides the document, the state is recorded
        // as untested rather than asserted against a page that stayed
        // visible.
        const other = await page.context().newPage();
        await other.goto("about:blank");
        await other.bringToFront();
        let hidden = await page.evaluate(() => document.hidden);
        const cdp = await page.context().newCDPSession(page);
        if (!hidden) {
            try {
                await cdp.send("Page.enable");
                await cdp.send("Page.setWebLifecycleState", { state: "frozen" });
                hidden = await page.evaluate(() => document.hidden);
            } catch (error) {
                console.log(`document.hidden: the lifecycle state was refused (${error})`);
            }
        }
        if (!hidden) {
            console.log("document.hidden: UNTESTED - this browser stayed visible");
            await other.close();
            test.skip();
            return;
        }

        const before = await stats(page);
        await page.waitForTimeout(5_000);
        const during = await stats(page);
        console.log(`document hidden: frames ${before.frames} -> ${during.frames}`);
        // A hidden document gets no animation frames from the browser, so a
        // region in it presents nothing at all.
        expect(during.frames - before.frames).toBeLessThanOrEqual(1);

        await cdp.send("Page.setWebLifecycleState", { state: "active" }).catch(() => {});
        await page.bringToFront();
        await other.close();
        expect(await page.evaluate(() => document.hidden)).toBe(false);
        // Back in front, it answers again.
        await page.evaluate(() =>
            (document.querySelector('[data-testid="b0-bump"]') as HTMLElement).click()
        );
        await expect
            .poll(async () => (await page.evaluate(() => window.__fusion_basic.b0_state())).count)
            .toBeGreaterThan(0);
    });

    test("a hundred rounds of hiding and restoring repeat no action", async ({ page }) => {
        test.setTimeout(600_000);
        await openB0(page);

        const outcome = await page.evaluate(async () => {
            const api = window.__fusion_basic;
            const canvas = document.querySelector('[data-testid="b0-gpu"]') as HTMLElement;
            const host = canvas.parentElement as HTMLElement;
            const button = document.querySelector('[data-testid="b0-bump"]') as HTMLElement;
            const before = api.b0_state();
            const rect = canvas.getBoundingClientRect();
            const control = before.controls.find((entry) => entry.name === "bump_first")!;
            const x = rect.left + control.x + control.width / 2;
            const y = rect.top + control.y + control.height / 2;
            for (let round = 0; round < 100; round++) {
                host.hidden = true;
                await new Promise((resolve) => requestAnimationFrame(resolve));
                // Aimed at the region while nothing of it is on screen. What
                // must not happen is this arriving later, when the region is
                // back: an action nobody can see being taken is an action
                // nobody asked for.
                for (const type of ["pointerdown", "pointerup"]) {
                    canvas.dispatchEvent(
                        new PointerEvent(type, {
                            clientX: x,
                            clientY: y,
                            bubbles: true,
                            isPrimary: true,
                            button: 0,
                        })
                    );
                }
                host.hidden = false;
                await new Promise((resolve) => requestAnimationFrame(resolve));
                // One business action per round, from the half that is never
                // hidden: the count is how many arrived.
                button.click();
            }
            await new Promise((resolve) => setTimeout(resolve, 500));
            return {
                before: before.count,
                after: api.b0_state().count,
                stats: api.stats(),
                errors: api.errors().length,
            };
        });

        console.log(
            `hide/restore 100: count ${outcome.before} -> ${outcome.after}, ` +
                `regions ${outcome.stats.regions}, errors ${outcome.errors}`
        );
        // A hundred rounds, a hundred actions: none lost, none repeated.
        expect(outcome.after - outcome.before).toBe(100);
        expect(outcome.stats.regions).toBe(1);
        expect(outcome.errors).toBe(0);

        await expect
            .poll(async () => (await page.evaluate(() => window.__fusion_basic.b0_state())).region)
            .toBe("ready");
    });
});
