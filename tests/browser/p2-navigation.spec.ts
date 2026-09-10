import { expect, Page, test } from "@playwright/test";

import { sharedPage } from "./support";

/// M4's first task: find out what the browser actually does, before writing a
/// router that assumes it.
///
/// A-5 is the assumption the plan wants tested - that a guard can put the user
/// back where they were by computing a displacement from a sequence number it
/// wrote into `history.state`, and calling `history.go` with it. The review
/// already showed the naive version fails: from `/d`, `go(-3)` lands on `/a`,
/// and `go(+1)` returns to `/b` rather than `/d`. These probes settle the rest
/// of it, and they stay in the suite because they are the evidence for why the
/// router is built the way it is.

/// Pushes `count` entries, each stamped with a sequence number the way the
/// router will stamp them.
async function stack(page: Page, urls: string[]) {
    await page.evaluate((urls) => {
        const w = window as unknown as { __probe: unknown[]; __probe_listening?: boolean };
        w.__probe = [];
        // One listener for the page, not one per call: these tests share a
        // page, and a second listener would report every event twice.
        if (!w.__probe_listening) {
            w.__probe_listening = true;
            addEventListener("popstate", () => {
                w.__probe.push({
                    url: location.pathname + location.search,
                    index:
                        (history.state as { rustify?: { index?: number } } | null)?.rustify
                            ?.index ?? null,
                });
            });
        }
        history.replaceState({ rustify: { index: 0 } }, "", location.pathname);
        urls.forEach((url, offset) => {
            history.pushState({ rustify: { index: offset + 1 } }, "", url);
        });
    }, urls);
}

const seen = (page: Page) =>
    page.evaluate(() => (window as unknown as { __probe: unknown[] }).__probe);

const here = (page: Page) =>
    page.evaluate(() => ({
        url: location.pathname + location.search,
        index: (history.state as { rustify?: { index?: number } } | null)?.rustify?.index ?? null,
    }));

test.describe("M4 A-5: what the browser does with history, measured", () => {
    test.describe.configure({ mode: "serial" });
    const shared = sharedPage();

    test("one go() crosses several entries, and reports the entry it landed on", async () => {
        const page = shared.page;
        await stack(page, ["?a", "?b", "?c", "?d"]);
        expect(await here(page)).toMatchObject({ url: "/?d", index: 4 });

        await page.evaluate(() => history.go(-3));
        await expect.poll(async () => (await seen(page)).length).toBe(1);
        // Three entries back in one move, and the state that came with it is
        // the state that entry was pushed with.
        expect(await here(page)).toMatchObject({ url: "/?a", index: 1 });
        expect(await seen(page)).toEqual([{ url: "/?a", index: 1 }]);
    });

    test("the way back is one go() of the difference, not several of one", async () => {
        const page = shared.page;
        await stack(page, ["?a", "?b", "?c", "?d"]);
        await page.evaluate(() => history.go(-3));
        await expect.poll(async () => (await seen(page)).length).toBe(1);

        // What the review found: stepping forward one at a time recovers one
        // step, and a guard built on it would leave the user two entries from
        // where they were.
        await page.evaluate(() => history.go(1));
        await expect.poll(async () => (await seen(page)).length).toBe(2);
        expect(await here(page)).toMatchObject({ url: "/?b", index: 2 });

        // The difference of the sequence numbers, in one move, is what works.
        const current = await here(page);
        await page.evaluate((delta) => history.go(delta), 4 - (current.index as number));
        await expect.poll(async () => (await seen(page)).length).toBe(3);
        expect(await here(page)).toMatchObject({ url: "/?d", index: 4 });
    });

    test("the same URL twice is two entries, and the sequence number tells them apart", async () => {
        const page = shared.page;
        // A list, a detail, and back to the same list - the ordinary shape of
        // a repeated URL, and the reason a URL cannot be an identity.
        await stack(page, ["?list", "?item", "?list"]);
        expect(await here(page)).toMatchObject({ url: "/?list", index: 3 });

        await page.evaluate(() => history.go(-2));
        await expect.poll(async () => (await seen(page)).length).toBe(1);
        const landed = await here(page);
        expect(landed.url).toBe("/?list");
        // Same URL, different entry: only the number says which.
        expect(landed.index).toBe(1);
    });

    test("a back pressed during a recovery arrives after it, and is its own event", async () => {
        const page = shared.page;
        await stack(page, ["?a", "?b", "?c", "?d"]);
        await page.evaluate(() => history.go(-3));
        await expect.poll(async () => (await seen(page)).length).toBe(1);

        // The guard asks to go forward three, and the user presses back in the
        // same breath. Both are queued; what a router has to survive is that
        // its own recovery is not the last word.
        await page.evaluate(() => {
            history.go(3);
            history.back();
        });
        await expect.poll(async () => (await seen(page)).length, { timeout: 10_000 }).toBeGreaterThanOrEqual(2);
        const trail = (await seen(page)) as { url: string; index: number | null }[];
        const landed = await here(page);
        // Whatever order they land in, every event carries the entry it is
        // about - so a router judges by the number it reads, never by
        // counting how many moves it asked for.
        for (const step of trail) {
            expect(step.index).not.toBeNull();
        }
        expect(landed.index).toBe(trail[trail.length - 1].index);
    });

    test("an entry the router did not push has no sequence number", async () => {
        const page = shared.page;
        await stack(page, ["?a"]);
        // Something else on the page pushed one: a host script, an older
        // build, a browser restoring a session.
        await page.evaluate(() => history.pushState({ theirs: true }, "", "?host"));
        expect(await here(page)).toMatchObject({ url: "/?host", index: null });

        await page.evaluate(() => history.back());
        await expect.poll(async () => (await seen(page)).length).toBe(1);
        // Back on ours, and the number is there again - so "no number" is a
        // property of the entry, not a state the router falls into.
        expect(await here(page)).toMatchObject({ url: "/?a", index: 1 });
    });
});
