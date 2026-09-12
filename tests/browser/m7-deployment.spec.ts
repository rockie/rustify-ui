import { expect, Page, test } from "@playwright/test";
import { settle, waitForReady } from "./support";

/// Sets what the server does wrong from now on. The page is reloaded after,
/// so the fault applies to a whole document rather than to whatever happened
/// to be in flight.
async function fault(page: Page, spec: string) {
    const response = await page.request.get(`./__fault/${spec}`);
    expect(response.ok()).toBe(true);
}

test.afterEach(async ({ page }) => {
    await fault(page, "none");
});

test.describe("M7 V9: the same build, served under a sub-path", () => {
    test("boots, draws, and leaves the page it was embedded in alone", async ({ page }) => {
        await waitForReady(page);
        expect(page.url()).toContain("/tools/demo/");
        // The page mounts B0 as it boots and the helper adds the two counter
        // scopes, exactly as at the root: a sub-path is a deployment detail,
        // not an API. Five regions - B0's one and two each.
        const region = page.getByTestId("scope-a-gpu-1");
        await settle(region);
        expect(await page.evaluate(() => window.__fusion_basic.live_regions())).toBe(5);

        // The host page around the scope is the page's, not the SDK's: its
        // own headings and links are still there and still work.
        await expect(page.getByRole("heading", { level: 1 })).toHaveCount(1);
        expect(
            await page.evaluate(() => document.documentElement.getAttribute("data-theme"))
        ).toBeNull();
    });

    test("asks for nothing but its own build", async ({ page }) => {
        const seen: string[] = [];
        page.on("request", (request) => seen.push(request.url()));
        await waitForReady(page);
        await settle(page.getByTestId("scope-a-gpu-1"));

        expect(seen.length).toBeGreaterThan(3);
        for (const url of seen) {
            // Same origin, and under the sub-path it was deployed at: a
            // deployment that reached anywhere else would be reaching for
            // something nobody deployed.
            expect(url, url).toMatch(/^http:\/\/127\.0\.0\.1:4175\/tools\/demo\//);
        }
        // The policy the deployment runs under is the strict one, and the
        // page never asked it to be relaxed.
        const head = await page.request.head("./");
        const policy = head.headers()["content-security-policy"] ?? "";
        expect(policy).toContain("script-src 'self' 'wasm-unsafe-eval'");
        expect(policy).not.toContain("unsafe-inline");
        expect(policy).not.toContain("unsafe-eval;");
    });
});

test.describe("M7 V9: assets from two different builds", () => {
    test("a bridge that claims another build stops the start and says so", async ({ page }) => {
        await fault(page, "stale-bridge");
        await page.goto("./");
        const status = page.getByTestId("status");
        await expect(status).toHaveAttribute("data-status", "failed", { timeout: 60_000 });
        await expect(page.getByRole("alert")).toContainText("BuildContractMismatch");
        await expect(page.getByRole("alert")).toContainText("redeploy matching assets");
        // Nothing started, so nothing has to be cleaned up.
        expect(await page.evaluate(() => "__fusion_basic" in window)).toBe(false);
    });
});

const FONT = "makepad_widgets/resources/IBMPlexSans-Text.ttf";

test.describe("M7 V8: an asset that does not arrive, or arrives broken", () => {
    // The three classes do not fail in the same place. A 404 fails the load,
    // which the runtime hears about. Damaged bytes arrive with a 200: the load
    // succeeds and the failure is in decoding them, which is inside the fork's
    // font code and is not reported. Both are survivable; only one is visible,
    // and the test says which.
    for (const [name, spec, reported] of [
        ["missing", `missing:${FONT}`, true],
        ["corrupt", `corrupt:${FONT}`, false],
        ["truncated", `truncated:${FONT}`, false],
    ] as const) {
        test(`a ${name} font costs glyphs, not the application`, async ({ page }) => {
            const failures: string[] = [];
            page.on("pageerror", (error) => failures.push(String(error)));
            await fault(page, spec);
            // Loaded through the helper, which is also where the two counter
            // scopes this check drives come from now that the page's own
            // mount is B0. The runtime still starts, which is the point: a
            // font is not a precondition for an application, and what a
            // missing one costs is the text a region can draw.
            await waitForReady(page);

            // The page's own half is untouched by an asset a region wanted.
            await expect(page.getByRole("heading", { level: 1 })).toHaveCount(1);
            expect(await page.evaluate(() => window.__fusion_basic.live_regions())).toBe(5);

            // Both halves still move one value together.
            await page.getByTestId("scope-a-dom-increment").click();
            await expect(page.getByTestId("scope-a-dom-count")).toHaveText("1");
            await settle(page.getByTestId("scope-a-gpu-1"));
            expect(failures).toEqual([]);
            expect(await page.evaluate(() => window.__fusion_basic.errors())).toEqual([]);

            const log = await page.evaluate(() => window.__fusion_basic.diagnostics());
            const assets = log.entries.filter((entry) => entry.kind === "AssetLoadFailed");
            if (reported) {
                // The file that did not arrive is named, with what to do.
                expect(assets.length).toBeGreaterThan(0);
                expect(assets[0].asset).toContain("IBMPlexSans-Text.ttf");
                expect(assets[0].suggestion).toContain("deployed beside the build");
            } else {
                // Nothing reports it, and this records that rather than
                // implying the class is covered.
                expect(assets).toHaveLength(0);
            }
        });
    }
});
