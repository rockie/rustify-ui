import { expect, test } from "@playwright/test";
import { B4, B4_ACTIONS } from "./loads";
import { settle, waitForQuiet } from "./support";

/// M7 · B4 for as long as it takes: two instances, four regions, ten actions
/// a second, and every one of them counted out and counted back in.
///
/// The load is the PRD's B4 and the rotation is the one frozen in `loads.ts`:
/// a DOM action and a GPU action in each of two wasm instances, in turn. What
/// it answers is whether a run that lasts loses an action, takes one twice,
/// or grows while it does - none of which a short run can answer, because all
/// three look like nothing for the first minute.
///
/// The duration is a parameter: two minutes is the useful default for a
/// change, two hours is the acceptance. `RUSTIFY_ENDURANCE_MINUTES=120` takes
/// the long one, and the report says which was run.
const MINUTES = Number(process.env.RUSTIFY_ENDURANCE_MINUTES ?? "2");
const RATE_HZ = B4.actionsPerSecond;

/// Where each instance's scope is mounted, in the rotation's own order.
const CONTAINERS: string[] = ["scope-a", "instance-two"];

test.describe("M7 V7: B4, for as long as it takes", () => {
    test(`${MINUTES} minutes at ${RATE_HZ} actions a second across two instances`, async ({
        page,
    }) => {
        test.setTimeout(MINUTES * 60_000 + 900_000);
        const failures: string[] = [];
        page.on("pageerror", (error) => failures.push(String(error)));

        await page.goto("./");
        await expect(page.getByTestId("status")).toHaveAttribute("data-status", "ready", {
            timeout: 120_000,
        });
        // Two instances, booted once and alive for the whole run: a restart
        // would leave a linear memory behind and make the curve below a
        // reading of restarts rather than of a steady state.
        expect(await page.evaluate(() => window.__fusion_basic.boot_second_instance())).toBe(
            B4.instances
        );
        await page.evaluate(() => {
            // B4 is two instances of two regions each. The B0 scope the page
            // mounts at load is a different load, and leaving it up would put
            // a third region and its own memory into every figure below.
            window.__fusion_instances[1].dispose("b0");
            window.__fusion_instances[1].mount("scope-a");
            window.__fusion_instances[2].mount("instance-two");
        });
        for (const container of CONTAINERS) {
            for (const slot of [1, 2]) {
                await settle(page.getByTestId(`${container}-gpu-${slot}`));
            }
        }
        await waitForQuiet(page);
        expect(await page.evaluate(() => window.__fusion_instances[1].live_regions())).toBe(
            B4.regionsPerInstance
        );
        expect(await page.evaluate(() => window.__fusion_instances[2].live_regions())).toBe(
            B4.regionsPerInstance
        );

        // The working set before the run. Every label the counter regions
        // draw is a string they have not drawn before - the count is in it -
        // and the shaper and layouter hold four thousand entries each, so a
        // run that starts on empty caches measures them filling.
        //
        // Five thousand rounds is measured, not guessed: with a warm-up of
        // 2,200 the memory of a ten-minute run climbed for its first six
        // minutes and was then flat for the rest, which is a cache reaching
        // its capacity and starting to evict. Walking past that point first,
        // as fast as the frames allow, is what makes the run below a reading
        // of a steady state at any duration - and what makes the two-minute
        // default worth running at all.
        const warm = await page.evaluate(
            async ({ containers, rounds }) => {
                // Where each region drew its "+1", measured here rather than
                // passed in: a screenshot taken on the way in scrolls the
                // page, and a spot measured before that lands somewhere else.
                const spots = containers.map((container) => {
                    const box = document
                        .querySelector(`[data-testid="${container}-gpu-2"]`)!
                        .getBoundingClientRect();
                    return { x: box.left + box.width / 2, y: box.top + box.height / 2 + 24 };
                });
                const before = [1, 2].map(
                    (number) => window.__fusion_instances[number].stats().memory
                );
                const click = (spot: { x: number; y: number }, canvas: Element) => {
                    for (const type of ["pointerdown", "pointerup"]) {
                        canvas.dispatchEvent(
                            new PointerEvent(type, {
                                bubbles: true,
                                cancelable: true,
                                clientX: spot.x,
                                clientY: spot.y,
                                pointerId: 1,
                                pointerType: "mouse",
                                button: 0,
                                buttons: type === "pointerdown" ? 1 : 0,
                            })
                        );
                    }
                };
                for (let round = 0; round < rounds; round++) {
                    for (const [index, container] of containers.entries()) {
                        const canvas = document.querySelector(
                            `[data-testid="${container}-gpu-2"]`
                        )!;
                        click(spots[index], canvas);
                    }
                    await new Promise((resolve) => requestAnimationFrame(resolve));
                }
                await new Promise((resolve) => setTimeout(resolve, 1_000));
                return {
                    rounds,
                    before,
                    after: [1, 2].map(
                        (number) => window.__fusion_instances[number].stats().memory
                    ),
                };
            },
            { containers: CONTAINERS, rounds: 5_000 }
        );
        console.log(
            `B4 warm-up: ${warm.rounds} rounds, memory ${warm.before.join(" + ")} -> ` +
                warm.after.join(" + ")
        );

        // Paced by a timer inside the page: a round trip through the harness
        // per action would set the rate rather than measure it, and at ten a
        // second for two hours it would never finish.
        const result = await page.evaluate(
            async ({ minutes, rate, rotation, containers }) => {
                const spots = containers.map((container) => {
                    const box = document
                        .querySelector(`[data-testid="${container}-gpu-2"]`)!
                        .getBoundingClientRect();
                    return { x: box.left + box.width / 2, y: box.top + box.height / 2 + 24 };
                });
                const count = (container: string) =>
                    Number(
                        document.querySelector(`[data-testid="${container}-dom-count"]`)
                            ?.textContent ?? "-1"
                    );
                const button = (container: string) =>
                    document.querySelector(`[data-testid="${container}-dom-increment"]`) as
                        | HTMLElement
                        | null;
                const canvas = (container: string) =>
                    document.querySelector(`[data-testid="${container}-gpu-2"]`) as Element | null;
                const click = (spot: { x: number; y: number }, target: Element) => {
                    for (const type of ["pointerdown", "pointerup"]) {
                        target.dispatchEvent(
                            new PointerEvent(type, {
                                bubbles: true,
                                cancelable: true,
                                clientX: spot.x,
                                clientY: spot.y,
                                pointerId: 1,
                                pointerType: "mouse",
                                button: 0,
                                buttons: type === "pointerdown" ? 1 : 0,
                            })
                        );
                    }
                };

                const started = performance.now();
                const deadline = started + minutes * 60_000;
                const interval = 1_000 / rate;
                const before = containers.map(count);
                const sent = containers.map(() => 0);
                const memory: number[][] = [[], []];
                const held: number[] = [];
                const latency: number[] = [];
                let step = 0;
                let next = started;
                let missed = -1;

                while (performance.now() < deadline) {
                    const action = rotation[step % rotation.length];
                    const index = action.instance - 1;
                    const container = containers[index];
                    const was = count(container);
                    const at = performance.now();
                    if (action.action === "dom-increment") {
                        button(container)?.click();
                    } else {
                        click(spots[index], canvas(container)!);
                    }
                    sent[index] += 1;
                    step += 1;

                    // Every tenth action is timed to its own arrival, so the
                    // tail can be compared with the head without timing all
                    // seventy-two thousand of them.
                    if (step % 10 === 0) {
                        let waited = 0;
                        while (count(container) === was && waited < 120) {
                            await new Promise((resolve) => requestAnimationFrame(resolve));
                            waited += 1;
                        }
                        if (waited >= 120 && missed < 0) {
                            missed = step;
                        }
                        latency.push(performance.now() - at);
                    }
                    if (step % (rate * 10) === 0) {
                        const stats = [1, 2].map((number) =>
                            window.__fusion_instances[number].stats()
                        );
                        memory[0].push(stats[0].memory);
                        memory[1].push(stats[1].memory);
                        held.push(
                            stats.reduce(
                                (sum, one) => sum + one.timers + one.animation_frames + one.tasks,
                                0
                            )
                        );
                    }

                    next += interval;
                    const wait = next - performance.now();
                    await new Promise((resolve) => setTimeout(resolve, Math.max(0, wait)));
                }

                // A moment for the last actions to be delivered before they
                // are counted.
                await new Promise((resolve) => setTimeout(resolve, 1_000));
                const stats = [1, 2].map((number) => window.__fusion_instances[number].stats());
                return {
                    sent,
                    accepted: containers.map((container, index) => count(container) - before[index]),
                    elapsed_ms: performance.now() - started,
                    memory,
                    held,
                    latency,
                    missed,
                    errors: stats.map((one) => one.errors),
                    regions: stats.map((one) => one.regions),
                    fatal: [1, 2].map((number) => window.__fusion_instances[number].fatal()),
                };
            },
            {
                minutes: MINUTES,
                rate: RATE_HZ,
                rotation: B4_ACTIONS as unknown as {
                    instance: number;
                    region: number;
                    action: string;
                }[],
                containers: CONTAINERS,
            }
        );

        const sent = result.sent.reduce((a, b) => a + b, 0);
        const accepted = result.accepted.reduce((a, b) => a + b, 0);
        const rate = sent / (result.elapsed_ms / 1_000);
        const percentile = (samples: number[], p: number) => {
            const sorted = [...samples].sort((a, b) => a - b);
            return sorted[Math.min(sorted.length - 1, Math.ceil((p / 100) * sorted.length) - 1)];
        };
        const head = result.latency.slice(0, Math.floor(result.latency.length / 10));
        const tailLatency = result.latency.slice(-Math.floor(result.latency.length / 10));
        console.log(
            `B4 endurance: ${MINUTES} min, sent ${sent} at ${rate.toFixed(2)}/s ` +
                `(${result.sent.join(" + ")}), accepted ${accepted} (${result.accepted.join(" + ")}), ` +
                `errors ${result.errors.join(" + ")}, regions ${result.regions.join(" + ")}`
        );
        console.log(
            `B4 latency: head p95 ${percentile(head, 95).toFixed(1)} ms, ` +
                `tail p95 ${percentile(tailLatency, 95).toFixed(1)} ms, ` +
                `worst ${Math.max(...result.latency).toFixed(1)} ms, ${result.latency.length} sampled`
        );
        for (const [index, curve] of result.memory.entries()) {
            console.log(
                `B4 memory, instance ${index + 1}: ${curve[0]} -> ${curve[curve.length - 1]} ` +
                    `(${curve[curve.length - 1] - curve[0]} bytes); curve ${curve.join(",")}`
            );
        }

        // The rate asked for, near enough to describe.
        expect(rate).toBeGreaterThan(RATE_HZ * 0.8);
        // Every action arrived exactly once, in both instances: a lost one
        // and a duplicated one are the same arithmetic from opposite sides.
        expect(result.missed, "an action that never arrived").toBe(-1);
        expect(result.accepted).toEqual(result.sent);
        expect(accepted).toBe(sent);
        // Nothing crashed and nothing was left ticking.
        expect(result.fatal).toEqual([null, null]);
        expect(result.errors).toEqual([0, 0]);
        expect(result.regions).toEqual([B4.regionsPerInstance, B4.regionsPerInstance]);
        expect(result.held[result.held.length - 1]).toBeLessThanOrEqual(result.held[0] + 1);

        // Flat at the end. Linear memory never shrinks, so the question is
        // not how much was allocated but whether allocation is still going on
        // once the caches the warm-up paid for are full: a steady state that
        // keeps growing is a leak.
        for (const [index, curve] of result.memory.entries()) {
            const tail = curve.slice(Math.floor(curve.length * 0.75));
            expect(tail.length, `instance ${index + 1}`).toBeGreaterThan(1);
            expect(tail[tail.length - 1] - tail[0], `instance ${index + 1}`).toBeLessThan(
                tail[0] * 0.01
            );
        }

        // And both halves of both instances still work afterwards.
        for (const container of CONTAINERS) {
            const before = Number(await page.getByTestId(`${container}-dom-count`).textContent());
            await page.getByTestId(`${container}-dom-increment`).click();
            await expect(page.getByTestId(`${container}-dom-count`)).toHaveText(
                String(before + 1)
            );
        }
        expect(failures).toEqual([]);
    });
});
