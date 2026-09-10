import { expect, Page, test } from "@playwright/test";

import { sharedPage } from "./support";

/// M2 V3: one theme, adopted by both halves, in every state a control has -
/// and a scope that asks for less movement gets less movement.
///
/// The timings here are taken inside the page. A round trip through the test
/// process costs more than the work being measured, so a 200 ms budget checked
/// from out here would be measuring Playwright.

const snapshot = (page: Page) => page.evaluate(() => window.__component_catalog.snapshot());

const scopeVariable = (page: Page, name: string) =>
    page.evaluate((name) => {
        const scope = document.querySelector("[data-rustify-scope]") as HTMLElement;
        return getComputedStyle(scope).getPropertyValue(name).trim();
    }, name);

test.describe("M2 V3: a theme both halves adopt", () => {
    test.describe.configure({ mode: "serial" });
    const shared = sharedPage();

    test("twenty switches, and every one of them lands inside the budget", async () => {
        const page = shared.page;
        const measured = await page.evaluate(async () => {
            const toggle = document.querySelector(
                '[data-testid="toggle-theme"]'
            ) as HTMLButtonElement;
            const scope = document.querySelector("[data-rustify-scope]") as HTMLElement;
            const frame = () => new Promise((resolve) => requestAnimationFrame(resolve));
            const rounds: { adopted: number; theme: string; primary: string }[] = [];
            for (let round = 0; round < 20; round += 1) {
                const before = getComputedStyle(scope).getPropertyValue("--primary").trim();
                const started = performance.now();
                toggle.click();
                // Two frames: one for the write, one for the style to be in
                // force when it is read.
                await frame();
                await frame();
                rounds.push({
                    adopted: performance.now() - started,
                    theme: scope.dataset.theme ?? "",
                    primary: getComputedStyle(scope).getPropertyValue("--primary").trim(),
                });
                if (rounds[round].primary === before) {
                    throw new Error(`round ${round} did not change --primary`);
                }
            }
            return rounds;
        });

        expect(measured).toHaveLength(20);
        // Twenty switches from light is twenty alternations back to light.
        expect(measured.map((round) => round.theme)).toEqual(
            Array.from({ length: 20 }, (_, index) => (index % 2 === 0 ? "dark" : "light"))
        );
        for (const [index, round] of measured.entries()) {
            expect(round.adopted, `round ${index}`).toBeLessThan(200);
        }
        expect(await snapshot(page)).toMatchObject({ theme: "light" });
    });

    test("the region's pixels move with the panel, not after it", async () => {
        const page = shared.page;
        await expect.poll(async () => (await snapshot(page)).region, { timeout: 30_000 }).toBe(
            "ready"
        );
        const region = page.getByTestId("catalogue-region");
        await region.scrollIntoViewIfNeeded();
        const light = await region.screenshot();

        await page.getByTestId("toggle-theme").click();
        expect(await scopeVariable(page, "--background")).toBe("#0c111d");
        await expect
            .poll(async () => (await region.screenshot()).equals(light), { timeout: 15_000 })
            .toBe(false);

        await page.getByTestId("toggle-theme").click();
        expect(await scopeVariable(page, "--background")).toBe("#f9fafb");
    });

    test("a control's four states are four different things to look at", async () => {
        const page = shared.page;
        await page.getByTestId("nav-button").click();
        const live = page.getByTestId("default-button");
        const disabled = page.getByTestId("disabled-button");

        const face = (testId: string) =>
            page.evaluate((testId) => {
                const element = document.querySelector(`[data-testid="${testId}"]`)!;
                const computed = getComputedStyle(element);
                return {
                    background: computed.backgroundColor,
                    opacity: computed.opacity,
                    outline: computed.outlineStyle,
                };
            }, testId);

        const resting = await face("default-button");
        await live.hover();
        const hovered = await face("default-button");
        expect(hovered.background).not.toBe(resting.background);

        // Tabbed into, not focused programmatically: `:focus-visible` is what
        // the ring is bound to, and a script moving focus does not match it -
        // which is the whole point of the pseudo-class.
        await page.getByTestId("nav-status").focus();
        await page.keyboard.press("Tab");
        const focused = await page.evaluate(() => {
            const element = document.querySelector('[data-testid="default-button"]')!;
            return {
                active: document.activeElement === element,
                ring: getComputedStyle(element).boxShadow,
            };
        });
        expect(focused.active).toBe(true);
        // The focus ring is a shadow drawn from the scope's own ring token.
        expect(focused.ring).not.toBe("none");

        const off = await face("disabled-button");
        expect(Number(off.opacity)).toBeLessThan(Number(resting.opacity));
        await expect(disabled).toBeDisabled();
    });

    test("less motion is less motion, in the stylesheet and in the region", async () => {
        const page = shared.page;
        await page.getByTestId("nav-loading").click();
        expect(await scopeVariable(page, "--motion-duration")).toBe("150ms");
        const spinning = await page.evaluate(() => {
            const svg = document.querySelector('[data-testid="default-spinner"] svg')!;
            return getComputedStyle(svg).animationDuration;
        });
        expect(spinning).toBe("1.2s");

        await page.getByTestId("toggle-motion").click();
        expect(await snapshot(page)).toMatchObject({ reduce_motion: true });
        expect(await scopeVariable(page, "--motion-duration")).toBe("0ms");
        // One token, and every transition and animation in the scope follows
        // it: the spinner stops, and so does everything that eases.
        expect(
            await page.evaluate(() => {
                const svg = document.querySelector('[data-testid="default-spinner"] svg')!;
                return getComputedStyle(svg).animationDuration;
            })
        ).toBe("0s");
        expect(
            await page.evaluate(() => {
                const button = document.querySelector('[data-testid="toggle-theme"]')!;
                return getComputedStyle(button).transitionDuration;
            })
        ).toBe("0s");

        await page.getByTestId("toggle-motion").click();
        expect(await snapshot(page)).toMatchObject({ reduce_motion: false });
    });

    test("the region's spinner turns, and stops when the scope asks it to", async () => {
        const page = shared.page;
        await page.getByTestId("nav-loading").click();
        await expect.poll(async () => (await snapshot(page)).control, { timeout: 15_000 }).not.toBeNull();
        const region = page.getByTestId("catalogue-region");
        await region.scrollIntoViewIfNeeded();

        // Turning: two frames a moment apart are different pictures.
        await expect
            .poll(
                async () => {
                    const first = await region.screenshot();
                    const second = await region.screenshot();
                    return first.equals(second);
                },
                { timeout: 15_000 }
            )
            .toBe(false);

        // Still: the arc is still drawn - it is what says "busy" - it just
        // does not move, and the region stops asking for frames.
        await page.getByTestId("toggle-motion").click();
        expect(await snapshot(page)).toMatchObject({ reduce_motion: true });
        await expect
            .poll(
                async () => {
                    const first = await region.screenshot();
                    const second = await region.screenshot();
                    return first.equals(second);
                },
                { timeout: 15_000 }
            )
            .toBe(true);
        await page.getByTestId("toggle-motion").click();
    });

    test("the host page is not part of the scope's theme", async () => {
        const page = shared.page;
        const before = await page.evaluate(() => ({
            theme: document.documentElement.dataset.theme ?? null,
            body: getComputedStyle(document.body).backgroundColor,
            host: getComputedStyle(document.querySelector('[data-testid="host-button"]')!)
                .backgroundColor,
        }));
        await page.getByTestId("toggle-theme").click();
        expect(await snapshot(page)).toMatchObject({ theme: "dark" });
        expect(
            await page.evaluate(() => ({
                theme: document.documentElement.dataset.theme ?? null,
                body: getComputedStyle(document.body).backgroundColor,
                host: getComputedStyle(document.querySelector('[data-testid="host-button"]')!)
                    .backgroundColor,
            }))
        ).toEqual(before);
        await page.getByTestId("toggle-theme").click();
    });
});
