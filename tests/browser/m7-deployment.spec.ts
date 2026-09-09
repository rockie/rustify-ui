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
        // The page mounts its own two scopes as it boots, exactly as it does
        // at the root: a sub-path is a deployment detail, not an API.
        const region = page.getByTestId("scope-a-gpu-1");
        await settle(region);
        expect(await page.evaluate(() => window.__fusion_basic.live_regions())).toBe(4);

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
    for (const [name, spec] of [
        ["missing", `missing:${FONT}`],
        ["corrupt", `corrupt:${FONT}`],
        ["truncated", `truncated:${FONT}`],
    ] as const) {
        test(`a ${name} font costs glyphs, not the application`, async ({ page }) => {
            const failures: string[] = [];
            page.on("pageerror", (error) => failures.push(String(error)));
            await fault(page, spec);
            await page.goto("./");

            // The page's own half is untouched by an asset a region wanted.
            await expect(page.getByRole("heading", { level: 1 })).toHaveCount(1);
            // And the runtime still starts. A font is not a precondition for
            // an application: what it costs is the text a region can draw.
            await expect(page.getByTestId("status")).toHaveAttribute("data-status", "ready", {
                timeout: 60_000,
            });
            expect(await page.evaluate(() => window.__fusion_basic.live_regions())).toBe(4);

            // Both halves still move one value together.
            await page.getByTestId("scope-a-dom-increment").click();
            await expect(page.getByTestId("scope-a-dom-count")).toHaveText("1");
            await settle(page.getByTestId("scope-a-gpu-1"));
            expect(failures).toEqual([]);
            expect(await page.evaluate(() => window.__fusion_basic.errors())).toEqual([]);
        });
    }
});
