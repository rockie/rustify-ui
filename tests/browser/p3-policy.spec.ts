import { expect, test } from "@playwright/test";

/// M5 · every example starts under the release policy with nothing refused.
///
/// Two of the four already answer this inside a larger check - the workbench
/// with two instances, the data example on its skeleton - and these are the
/// other two. A refused stylesheet or a blocked module does not stop a page
/// looking like it started, so the only way to know is to listen for the
/// refusal, from before the page's own scripts run.

test("the page boots under the release policy with nothing refused", async ({ page }) => {
    await page.addInitScript(() => {
        (window as unknown as { __csp: string[] }).__csp = [];
        document.addEventListener("securitypolicyviolation", (event) => {
            (window as unknown as { __csp: string[] }).__csp.push(
                `${event.violatedDirective} ${event.blockedURI}`
            );
        });
    });
    await page.goto("./");
    await expect(page.getByTestId("status")).toHaveAttribute("data-status", "ready", {
        timeout: 120_000,
    });
    const violations = await page.evaluate(
        () => (window as unknown as { __csp: string[] }).__csp
    );
    expect(violations).toEqual([]);
});
