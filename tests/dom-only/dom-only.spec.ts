import { expect, Page, test } from "@playwright/test";

/// A Rustify UI application with its default features, as an outside
/// consumer builds it: DOM only, no Makepad, booted by the SDK's own DOM
/// loader. The same lifecycle contract as the GPU host's (`p3-instances`,
/// `p3-restart`): one instance's trap reaches nothing of another's, and what
/// the dead one had registered on the page - listeners, timers, tasks, size
/// observers, the address bar's mark - is gone before the page is told.

declare global {
    interface Window {
        __dom_consumer: {
            instances: Record<number, Instance>;
            released: number[];
            boot_instance(container_id?: string, url_owner?: boolean): Promise<number>;
        };
        __dom_instances: Record<number, Instance>;
        __csp: string[];
        __booting?: Promise<number>;
    }
}

interface Instance {
    instance: number;
    restarts: number;
    restart_limit: number;
    glue_url: string;
    fatal(): string | null;
    stats(): { tasks: number; timers: number; observers: number; errors: number; aborted: boolean };
    aborted(): boolean;
    state(): { count: number; keys: number; resizes: number; dialog: boolean } | null;
    diagnostics(): { runtime: number; entries: { kind: string }[] };
    clipboard_available(): boolean;
    tooltip(open: boolean): boolean;
    hold_release(): void;
    trap(): void;
    trap_deferred(): void;
    simulate_trap(message?: string): void;
    restart(): Promise<number | null>;
}

const status = (page: Page) => page.getByTestId("status");

/// Records every policy violation from before the page's own scripts run.
async function watchPolicy(page: Page) {
    await page.addInitScript(() => {
        window.__csp = [];
        document.addEventListener("securitypolicyviolation", (event) => {
            window.__csp.push(`${event.violatedDirective} ${event.blockedURI}`);
        });
    });
}

async function open(page: Page) {
    await watchPolicy(page);
    await page.goto("./");
    await expect(status(page)).toHaveAttribute("data-status", "ready");
}

/// The listeners on `window`, by event type, as DevTools sees them.
async function windowListeners(page: Page): Promise<Record<string, number>> {
    const cdp = await page.context().newCDPSession(page);
    const { result } = (await cdp.send("Runtime.evaluate", { expression: "window" })) as {
        result: { objectId: string };
    };
    const { listeners } = (await cdp.send("DOMDebugger.getEventListeners", {
        objectId: result.objectId,
    })) as { listeners: { type: string }[] };
    await cdp.detach();
    const counts: Record<string, number> = {};
    for (const listener of listeners) {
        counts[listener.type] = (counts[listener.type] ?? 0) + 1;
    }
    return counts;
}

/// Boots a second instance into `instance-two`, holding the pending promise on
/// the page so DevTools does not collect it before it settles.
async function bootSecond(page: Page): Promise<number> {
    return page.evaluate(async () => {
        window.__booting = window.__dom_consumer.boot_instance("instance-two", false);
        try {
            return await window.__booting;
        } finally {
            delete window.__booting;
        }
    });
}

/// A hundred clicks on one scope's own button, and how far its count moved.
async function liveActions(page: Page, scope: string): Promise<number> {
    return page.evaluate(async (scope) => {
        const button = document.querySelector(`[data-testid="${scope}-increment"]`);
        if (!(button instanceof HTMLElement)) {
            return -1;
        }
        const count = () => Number(document.querySelector(`[data-testid="${scope}-count"]`)?.textContent ?? "0");
        const before = count();
        for (let i = 0; i < 100; i++) {
            button.click();
        }
        await new Promise((resolve) => requestAnimationFrame(resolve));
        return count() - before;
    }, scope);
}

const instance = (page: Page, number: number) => ({
    stats: () => page.evaluate((n) => window.__dom_instances[n].stats(), number),
    state: () => page.evaluate((n) => window.__dom_instances[n].state(), number),
    fatal: () => page.evaluate((n) => window.__dom_instances[n].fatal(), number),
});

/// The four ways into a trap: three real ones, each leaving the module by a
/// different door, and the page's own simulation, which has to arrive at the
/// same place.
const TRAPS: { name: string; fire: (page: Page) => Promise<unknown> }[] = [
    {
        name: "an export the page called",
        fire: (page) => page.evaluate(() => window.__dom_instances[1].trap()).catch(() => {}),
    },
    {
        name: "a DOM event handler",
        fire: (page) => page.getByTestId("instance-one-trap").click(),
    },
    {
        name: "a task the host was running",
        fire: (page) => page.evaluate(() => window.__dom_instances[1].trap_deferred()),
    },
    {
        name: "the page's own simulation",
        fire: (page) => page.evaluate(() => window.__dom_instances[1].simulate_trap()),
    },
];

test.describe("a DOM application starts without Makepad", () => {
    test("the page loads the DOM loader, the glue and the SDK runtime, and nothing of Makepad", async ({ page }) => {
        const requested: string[] = [];
        page.on("request", (request) => requested.push(new URL(request.url()).pathname));
        await open(page);
        expect(requested.some((path) => /makepad|embedded\.js|bindgen\.js|message_bridge/.test(path))).toBe(false);
        expect(requested).toContain("/rustify-dom-loader.js");
        expect(requested).toContain("/pkg/app.js");
        expect(requested).toContain("/pkg/app_bg.wasm");
        expect(requested.some((path) => path.endsWith("/web/dom-runtime.js"))).toBe(true);
        expect(await page.evaluate(() => window.__csp)).toEqual([]);
        // Built the way the recipe says, with web-sys's unstable APIs.
        expect(await page.evaluate(() => window.__dom_instances[1].clipboard_available())).toBe(true);
    });

    test("toast, dialog, size observer and page listener work in one scope", async ({ page }) => {
        await open(page);
        const one = instance(page, 1);

        // A toast hides itself on a timer the host owns.
        await page.getByTestId("instance-one-toast-show").click();
        await expect(page.getByTestId("instance-one-toaster-toast")).toBeVisible();
        expect((await one.stats()).timers).toBeGreaterThan(0);
        await expect(page.getByTestId("instance-one-toaster-toast")).toHaveCount(0);
        await expect.poll(async () => (await one.stats()).timers).toBe(0);

        // The dialog stands on the overlay stack: focus moves in, Escape
        // closes it, and focus comes back to what opened it.
        const opener = page.getByTestId("instance-one-dialog-open");
        await opener.click();
        await expect(page.getByRole("dialog")).toBeVisible();
        expect((await one.state())?.dialog).toBe(true);
        await page.keyboard.press("Escape");
        await expect(page.getByRole("dialog")).toHaveCount(0);
        await expect(opener).toBeFocused();

        // The size observer the host owns hears the box change size.
        const before = (await one.state())!.resizes;
        await page.getByTestId("instance-one-box").evaluate((box: HTMLElement) => {
            box.style.width = "300px";
        });
        await expect.poll(async () => (await one.state())!.resizes).toBeGreaterThan(before);
        expect((await one.stats()).observers).toBe(1);

        // A layer anchored to an element follows the anchor when only the
        // anchor changes size: no scroll, no window resize, no change in the
        // layer itself.
        // The pointer is parked away from it: hovering opens and closes a
        // tooltip, and a layer opened again is placed afresh. The page is at
        // its top, so a grown element cannot move it by scroll anchoring - a
        // scroll repositions every layer on its own. Neither is what is being
        // checked.
        await page.mouse.move(0, 0);
        await page.evaluate(() => window.scrollTo(0, 0));
        await page.evaluate(() => window.__dom_instances[1].tooltip(true));
        const tip = page.getByTestId("instance-one-tooltip");
        await expect(tip).toBeVisible();
        await tip.evaluate((content: HTMLElement) => {
            (content.closest(".rustify-layer") as HTMLElement & { __kept?: boolean }).__kept = true;
        });
        const anchor = page.getByTestId("instance-one-tooltip-anchor");
        const below = async () => {
            const [a, t] = await Promise.all([anchor.boundingBox(), tip.boundingBox()]);
            return Math.round(t!.y - (a!.y + a!.height));
        };
        const gap = await below();
        const top = (await tip.boundingBox())!.y;
        await anchor.evaluate((button: HTMLElement) => {
            button.style.height = "96px";
        });
        // Promptly: a layer that catches up only when something else happens
        // to reposition it is not following its anchor.
        await expect.poll(async () => (await tip.boundingBox())!.y, { timeout: 2_000 }).toBeGreaterThan(top + 40);
        expect(await below()).toBe(gap);
        // The same layer moved; it was not closed and opened again.
        expect(
            await tip.evaluate(
                (content: HTMLElement) => (content.closest(".rustify-layer") as HTMLElement & { __kept?: boolean }).__kept
            )
        ).toBe(true);
        expect((await one.stats()).observers).toBe(3);
        await page.evaluate(() => window.__dom_instances[1].tooltip(false));
        await expect(tip).toHaveCount(0);

        // And the listener on the window hears keys.
        const keys = (await one.state())!.keys;
        await page.keyboard.press("a");
        await expect.poll(async () => (await one.state())!.keys).toBe(keys + 1);
    });

    test("a scope is not mounted before the runtime has started", async ({ page }) => {
        await open(page);
        const refused = await page.evaluate(async () => {
            // The same wasm, initialised by hand without the loader.
            const glue = await import("./pkg/app.js?instance=77");
            await glue.default({ module_or_path: "./pkg/app_bg.wasm" });
            const container = document.createElement("div");
            container.id = "unbooted";
            document.body.append(container);
            try {
                glue.dom_consumer_mount("unbooted", false);
                return "mounted";
            } catch (error) {
                return String(error);
            } finally {
                container.remove();
            }
        });
        expect(refused).toContain("RuntimeNotStarted");
    });

    test("the runtime refuses hooks of another protocol, and a second start", async ({ page }) => {
        await open(page);
        const answers = await page.evaluate(async () => {
            const glue = await import("./pkg/app.js?instance=78");
            await glue.default({ module_or_path: "./pkg/app_bg.wasm" });
            const answers: string[] = [];
            const wrong = glue.rustify_dom_create_host(78);
            wrong.protocol = 99;
            try {
                glue.rustify_runtime_boot(wrong);
                answers.push("started");
            } catch (error) {
                answers.push(String(error));
            }
            wrong.runtime.release();
            const right = glue.rustify_dom_create_host(78);
            glue.rustify_runtime_boot(right);
            answers.push("started");
            try {
                glue.rustify_runtime_boot(glue.rustify_dom_create_host(78));
                answers.push("started twice");
            } catch (error) {
                answers.push(String(error));
            }
            right.runtime.release();
            return answers;
        });
        expect(answers[0]).toContain("protocol 99");
        expect(answers[1]).toBe("started");
        expect(answers[2]).toContain("already started");
    });

    test("a loader of another protocol refuses to start and leaves nothing behind", async ({ page }) => {
        await page.route("**/rustify-dom-loader.js", async (route) => {
            const response = await route.fetch();
            const body = await response.text();
            expect(body).toContain("const PROTOCOL = 1;");
            await route.fulfill({ response, body: body.replace("const PROTOCOL = 1;", "const PROTOCOL = 2;") });
        });
        await watchPolicy(page);
        await page.goto("./");
        await expect(status(page)).toHaveAttribute("data-status", "startup-failed");
        await expect(page.getByRole("alert")).toContainText("BuildContractMismatch");
        await expect(page.getByRole("alert")).toContainText("protocol 2");
        // Nothing was mounted and nothing listens for an instance that never
        // started.
        expect(await page.locator("[data-rustify-scope]").count()).toBe(0);
        const listeners = await windowListeners(page);
        expect(listeners.error ?? 0).toBe(0);
        expect(listeners.unhandledrejection ?? 0).toBe(0);
    });

    test("a host created for another protocol is released, not started", async ({ page }) => {
        await page.route("**/web/dom-runtime.js", async (route) => {
            const response = await route.fetch();
            const body = await response.text();
            await route.fulfill({
                response,
                body: body.replace("export function create_dom_host(instance, protocol) {", "export function create_dom_host(instance, protocol) {\n    protocol = protocol + 1;"),
            });
        });
        await page.goto("./");
        await expect(status(page)).toHaveAttribute("data-status", "startup-failed");
        await expect(page.getByRole("alert")).toContainText("BuildContractMismatch");
        const listeners = await windowListeners(page);
        expect(listeners.error ?? 0).toBe(0);
        expect(listeners.unhandledrejection ?? 0).toBe(0);
    });
});

test.describe("one instance fails, the other does not", () => {
    for (const trap of TRAPS) {
        test(`a trap through ${trap.name} reaches its own instance only`, async ({ page }) => {
            await open(page);
            expect(await bootSecond(page)).toBe(2);
            const one = instance(page, 1);
            const two = instance(page, 2);
            await page.evaluate(() => {
                window.__dom_instances[1].hold_release();
                window.__dom_instances[2].hold_release();
            });
            // Something pending in each: a toast's timer, a size observer, a
            // listener on the window; and the first one owns the address bar.
            await page.getByTestId("instance-one-toast-show").click();
            expect((await one.stats()).timers).toBeGreaterThan(0);
            expect((await one.stats()).observers).toBe(1);
            expect(await page.evaluate(() => document.documentElement.getAttribute("data-rustify-url-owner"))).toBe(
                "1:instance-one"
            );
            const before = await windowListeners(page);

            await trap.fire(page);
            await expect(status(page)).toHaveAttribute("data-status", "fatal");
            const notice = page.getByRole("alert");
            await expect(notice).toContainText("RuntimeFatal");
            await expect(notice).toContainText("unsaved in-memory state is lost");
            await expect(page.getByTestId("fatal-restart")).toHaveCount(1);

            // Everything the dead instance had on the page is gone: its
            // timers, tasks and observers, its abort releases, its mark on the
            // address bar, and its listeners - its scope's keydown and the
            // loader's own error and rejection listeners.
            expect(await one.stats()).toMatchObject({ tasks: 0, timers: 0, observers: 0, aborted: true });
            expect(await page.evaluate(() => window.__dom_consumer.released)).toEqual([1]);
            expect(await page.evaluate(() => document.documentElement.getAttribute("data-rustify-url-owner"))).toBeNull();
            const after = await windowListeners(page);
            expect(after.keydown ?? 0).toBe((before.keydown ?? 0) - 1);
            expect(after.error ?? 0).toBe((before.error ?? 0) - 1);
            expect(after.unhandledrejection ?? 0).toBe((before.unhandledrejection ?? 0) - 1);
            await expect(page.getByTestId("instance-one")).toBeEmpty();

            // The other one took every one of a hundred actions, and its
            // timer, observer and listener all still work.
            expect(await liveActions(page, "instance-two")).toBe(100);
            expect(await two.fatal()).toBeNull();
            expect((await two.stats()).aborted).toBe(false);
            await page.getByTestId("instance-two-toast-show").click();
            await expect(page.getByTestId("instance-two-toaster-toast")).toBeVisible();
            await expect(page.getByTestId("instance-two-toaster-toast")).toHaveCount(0);
            const resizes = (await two.state())!.resizes;
            await page.getByTestId("instance-two-box").evaluate((box: HTMLElement) => {
                box.style.width = "320px";
            });
            await expect.poll(async () => (await two.state())!.resizes).toBeGreaterThan(resizes);
            const keys = (await two.state())!.keys;
            await page.keyboard.press("b");
            await expect.poll(async () => (await two.state())!.keys).toBe(keys + 1);
            // Its record is its own.
            const record = await page.evaluate(() => window.__dom_instances[2].diagnostics());
            expect(record.runtime).toBe(2);
        });
    }

    test("two instances start under the release policy with nothing refused", async ({ page }) => {
        await open(page);
        expect(await bootSecond(page)).toBe(2);
        expect(await page.evaluate(() => window.__csp)).toEqual([]);
        const glue = await page.evaluate(() => [window.__dom_instances[1].glue_url, window.__dom_instances[2].glue_url]);
        expect(glue[0]).toMatch(/\/pkg\/app\.js$/);
        expect(glue[1]).toMatch(/\/pkg\/app\.js\?instance=2$/);
        expect(await liveActions(page, "instance-two")).toBe(100);
        expect(await liveActions(page, "instance-one")).toBe(100);
    });

    test("a slot may be restarted three times and then only reloading is left", async ({ page }) => {
        await open(page);
        for (let round = 1; round <= 3; round++) {
            const showing = await page.evaluate(() => Math.max(...Object.keys(window.__dom_instances).map(Number)));
            await page.evaluate((n) => window.__dom_instances[n].simulate_trap(), showing);
            await expect(status(page)).toHaveAttribute("data-status", "fatal");
            await page.getByTestId("fatal-restart").click();
            await expect(status(page)).toHaveAttribute("data-status", "ready");
            // The restarted instance owns the address bar again and works.
            const next = showing + 1;
            expect(await page.evaluate(() => document.documentElement.getAttribute("data-rustify-url-owner"))).toBe(
                `${next}:instance-one`
            );
            expect(await liveActions(page, "instance-one")).toBe(100);
            expect(await page.evaluate((n) => window.__dom_instances[n].restarts, next)).toBe(round);
        }
        await page.evaluate(() => window.__dom_instances[4].simulate_trap());
        await expect(status(page)).toHaveAttribute("data-status", "fatal");
        await expect(page.getByRole("alert")).toContainText("restarted as often as it can be");
        await expect(page.getByTestId("fatal-restart")).toHaveCount(0);
        await expect(page.getByTestId("fatal-reload")).toHaveCount(1);
        expect(await page.evaluate(() => window.__dom_instances[4].restart())).toBeNull();
    });
});
