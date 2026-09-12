import { expect, Page, test } from "@playwright/test";

/// M5 · two application instances on one page, and what one of them dying
/// does to the other.
///
/// An instance is a wasm instance: its own linear memory, its own runtime, its
/// own diagnostics. A trap takes every scope in the instance it happened in
/// and must reach nothing else - not the other instance's regions, not its
/// actions, and not the page's address bar, which is neither instance's to
/// keep once its owner has gone.
///
/// Health is measured by what still works. A dead instance's closures can be
/// finalised later and complain, and an uncaught error from one of those says
/// nothing about the instance beside it - so nothing here judges by the
/// absence of console noise.

const status = (page: Page) => page.getByTestId("status");

async function open(page: Page) {
    await page.goto("./");
    await expect(status(page)).toHaveAttribute("data-status", "ready", { timeout: 120_000 });
}

/// Records every policy violation from before the page's own scripts run. Two
/// instances means a second module evaluated under a query string, which is
/// exactly the kind of thing a strict policy refuses quietly.
async function watchPolicy(page: Page) {
    await page.addInitScript(() => {
        (window as unknown as { __csp: string[] }).__csp = [];
        document.addEventListener("securitypolicyviolation", (event) => {
            (window as unknown as { __csp: string[] }).__csp.push(
                `${event.violatedDirective} ${event.blockedURI}`
            );
        });
    });
}

const policyViolations = (page: Page) =>
    page.evaluate(() => (window as unknown as { __csp: string[] }).__csp);

/// A hundred clicks on one scope's own button, and how many of them the count
/// went up by. The scope is named after the container it was mounted into.
async function liveActions(page: Page, container: string): Promise<number> {
    return page.evaluate(async (container) => {
        const button = document.querySelector(`[data-testid="${container}-dom-increment"]`);
        if (!(button instanceof HTMLElement)) {
            return -1;
        }
        const count = () =>
            Number(
                document.querySelector(`[data-testid="${container}-dom-count"]`)?.textContent ?? "0"
            );
        const before = count();
        for (let i = 0; i < 100; i++) {
            button.click();
        }
        await new Promise((resolve) => requestAnimationFrame(resolve));
        return count() - before;
    }, container);
}

/// The mark on the page that says who owns the address bar.
const urlOwner = (page: Page) =>
    page.evaluate(() => document.documentElement.getAttribute("data-rustify-url-owner"));

/// The four ways into a trap. Three of them are real - a panic under
/// `panic = "abort"` is an `unreachable` in wasm - and each leaves the module
/// by a different door. The fourth is the page simulating one, which has to
/// arrive at the same place or the other three are being checked against a
/// door nobody else uses.
const TRAPS: { name: string; fire: (page: Page) => Promise<unknown> }[] = [
    {
        name: "an export the page called",
        // The error is let through rather than swallowed in the page: what is
        // being checked is that the loader saw it first. Catching it here, in
        // the test process, is not the page catching it.
        fire: (page) => page.evaluate(() => window.__fusion_basic.trap()).catch(() => {}),
    },
    {
        name: "a DOM event handler",
        fire: async (page) => {
            await page.evaluate(() => window.__fusion_basic.mount_trap("geometry"));
            await page.getByTestId("trap-in-handler").click();
        },
    },
    {
        name: "a task the host was running",
        fire: (page) => page.evaluate(() => window.__fusion_basic.trap_deferred()),
    },
    {
        name: "the page's own simulation",
        fire: (page) => page.evaluate(() => window.__fusion_basic.simulate_trap()),
    },
];

test.describe("M5 · one instance fails, the other does not", () => {
    for (const trap of TRAPS) {
        test(`a trap through ${trap.name} reaches its own instance only`, async ({ page }) => {
            await open(page);
            // The second instance gets the counter fixture: an instance that
            // is going to be asked whether it still works needs something to
            // work.
            const second = await page.evaluate(() =>
                window.__fusion_basic.boot_instance("instance-two", "mount")
            );
            expect(second).toBe(2);

            await trap.fire(page);
            await expect(status(page)).toHaveAttribute("data-status", "fatal", { timeout: 30_000 });

            // The same notice, whichever door it came through: what was lost,
            // and the one thing that can still be done about it.
            const notice = page.getByRole("alert");
            await expect(notice).toContainText("RuntimeFatal");
            await expect(notice).toContainText("unsaved in-memory state is lost");
            await expect(page.getByTestId("fatal-restart")).toHaveCount(1);
            // Nothing the page still holds can call into the module that
            // trapped.
            expect(await page.evaluate(() => "__fusion_basic" in window)).toBe(false);

            // And the instance beside it took every one of a hundred actions.
            expect(await liveActions(page, "instance-two")).toBe(100);
            const alive = await page.evaluate(() => ({
                fatal: window.__fusion_instances[2].fatal(),
                regions: window.__fusion_instances[2].live_regions(),
            }));
            expect(alive.fatal).toBeNull();
            expect(alive.regions).toBeGreaterThan(0);
        });
    }

    test("a dead instance's diagnostics are its own", async ({ page }) => {
        await open(page);
        await page.evaluate(() => window.__fusion_basic.boot_instance("instance-two", "mount"));
        // Read before the trap: an export called on a dead instance throws
        // `InstanceDead` rather than reaching it, which is the boundary doing
        // its job - so a dead instance's record is not something to go asking
        // it for afterwards.
        const first = await page.evaluate(() => window.__fusion_instances[1].diagnostics());
        expect(first.runtime).toBe(1);

        await page.evaluate(() => window.__fusion_basic.simulate_trap());
        await expect(status(page)).toHaveAttribute("data-status", "fatal", { timeout: 30_000 });

        // The instance beside it names itself in its own record and has none
        // of the dead one's entries.
        const second = await page.evaluate(() => window.__fusion_instances[2].diagnostics());
        expect(second.runtime).toBe(2);
        expect(
            second.entries.filter((entry: { kind: string }) => entry.kind === "RuntimeFatal")
        ).toHaveLength(0);
        // And it is still the one the page can reach through its own door.
        expect(await page.evaluate(() => window.__fusion_instances[2].fatal())).toBeNull();
    });

    test("two instances start under the release policy with nothing refused", async ({ page }) => {
        await watchPolicy(page);
        await open(page);
        const second = await page.evaluate(() =>
            window.__fusion_basic.boot_instance("instance-two", "mount")
        );
        expect(second).toBe(2);
        // One wasm module, two instances of it: the second glue is a second
        // module record under a query string, and a strict `script-src 'self'`
        // has to allow that.
        expect(await policyViolations(page)).toEqual([]);
        expect(await liveActions(page, "instance-two")).toBe(100);
    });
});

/// How many listeners of each kind are on `window` right now.
///
/// Read through the debugger rather than by counting registrations: what is
/// being checked is that a trap actually took them off, and a count the page
/// keeps itself would only say what the page believed.
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

test.describe("M5 · the address bar after its owner dies", () => {
    test("a restarted instance owns the URL again and is the only one answering", async ({
        page,
    }) => {
        await open(page);
        const baseline = await windowListeners(page);

        await page.evaluate(() => window.__fusion_basic.mount_owner("route-owner"));
        await expect
            .poll(async () => await urlOwner(page), { message: "the first owner", timeout: 15_000 })
            .toBe("1:route-owner");
        const owning = await windowListeners(page);
        expect(owning.popstate ?? 0).toBeGreaterThan(baseline.popstate ?? 0);

        await page.evaluate(() => window.__fusion_basic.trap()).catch(() => {});
        await expect(status(page)).toHaveAttribute("data-status", "fatal", { timeout: 30_000 });
        // A trap runs no destructor, so nothing Rust owns took these off: the
        // host aborting the signal every one of them was registered with is
        // what did, and the count says whether it worked.
        await expect
            .poll(async () => (await windowListeners(page)).popstate ?? 0, {
                message: "the dead instance's listeners",
                timeout: 15_000,
            })
            .toBe(baseline.popstate ?? 0);
        // The loader's own attribution listener goes with them: the instance
        // that had it is the one that died, and it was the only instance on
        // this page.
        expect((await windowListeners(page)).error ?? 0).toBe(0);
        // And the mark is off the page, so the address bar is free for
        // whoever comes next.
        expect(await urlOwner(page)).toBeNull();

        // The notice's own restart entry, used the way a person would.
        await page.getByTestId("fatal-restart").click();
        await expect(status(page)).toHaveAttribute("data-status", "ready", { timeout: 60_000 });
        const restarted = await page.evaluate(() => window.__fusion_basic.instance);
        expect(restarted).toBeGreaterThan(1);
        await expect
            .poll(async () => await urlOwner(page), {
                message: "the replacement owner",
                timeout: 15_000,
            })
            .toBe(`${restarted}:route-owner`);
        // The replacement put its own listeners back where the dead one's
        // had been.
        const after = await windowListeners(page);
        expect(after.error ?? 0).toBe(baseline.error ?? 0);
        expect(after.popstate ?? 0).toBe(owning.popstate ?? 0);

        // The new instance routes, and it is the only thing answering. Five
        // moves, then five back and five forward: a `popstate` that nobody
        // answers leaves the address bar and the view disagreeing, and a
        // `popstate` that two instances answer is worse.
        const visited: string[] = [];
        for (let step = 0; step < 5; step++) {
            const target = step % 2 === 0 ? "two" : "one";
            await page.getByTestId(`route-owner-${target}`).click();
            visited.push(`/${target}`);
            await expect
                .poll(async () => (await routes(page)).owner, {
                    message: `moved to ${target}`,
                    timeout: 10_000,
                })
                .toBe(`/${target}`);
        }
        for (let step = 4; step >= 1; step--) {
            await page.goBack();
            await expect
                .poll(async () => (await routes(page)).owner, {
                    message: `back to ${step}`,
                    timeout: 10_000,
                })
                .toBe(visited[step - 1]);
        }
        for (let step = 1; step < 5; step++) {
            await page.goForward();
            await expect
                .poll(async () => (await routes(page)).owner, {
                    message: `forward to ${step}`,
                    timeout: 10_000,
                })
                .toBe(visited[step]);
        }
    });

    test("a second owner is refused and the first one is untouched", async ({ page }) => {
        await open(page);
        await page.evaluate(() => window.__fusion_basic.mount_owner("route-owner"));
        await expect
            .poll(async () => await urlOwner(page), { message: "the first owner", timeout: 15_000 })
            .toBe("1:route-owner");

        // A second instance asking for the same address bar. One page, one
        // owner - and the page-level mark is what makes that true across two
        // wasm instances, each of which has its own everything else.
        const refused = await page.evaluate(async () => {
            try {
                await window.__fusion_basic.boot_instance("instance-two", "mount_owner");
                return null;
            } catch (error) {
                return String(error);
            }
        });
        expect(refused).toContain("another scope owns this page's URL");
        expect(await urlOwner(page)).toBe("1:route-owner");
        const record = await page.evaluate(() => window.__fusion_instances[2].diagnostics());
        expect(
            record.entries.filter((entry: { kind: string }) => entry.kind === "UrlOwnerConflict")
        ).toHaveLength(1);
    });

    test("a slot may be restarted three times and then only reloading is left", async ({ page }) => {
        await open(page);
        for (let round = 1; round <= 3; round++) {
            await page.evaluate(() => window.__fusion_basic.simulate_trap());
            await expect(status(page)).toHaveAttribute("data-status", "fatal", { timeout: 30_000 });
            await expect(page.getByTestId("fatal-restart"), `restart ${round}`).toHaveCount(1);
            await page.getByTestId("fatal-restart").click();
            await expect(status(page)).toHaveAttribute("data-status", "ready", { timeout: 60_000 });
        }
        // The fourth failure has nothing left to offer: every restart of this
        // slot left a linear memory behind that the document will hold until
        // it is unloaded, which is why there is a limit at all.
        await page.evaluate(() => window.__fusion_basic.simulate_trap());
        await expect(status(page)).toHaveAttribute("data-status", "fatal", { timeout: 30_000 });
        await expect(page.getByTestId("fatal-restart")).toHaveCount(0);
        await expect(page.getByRole("alert")).toContainText("Reload the page");
    });
});

/// What each routing scope says its own location is, by scope name.
const routes = (page: Page) =>
    page.evaluate(() => window.__fusion_basic.routes()).then((all: string) =>
        Object.fromEntries(
            all
                .split(";")
                .filter(Boolean)
                .map((part) => part.split("=") as [string, string])
        )
    );
