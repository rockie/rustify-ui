import { expect, Page, test } from "@playwright/test";

async function fatal(page: Page, example: string) {
    await page.evaluate((example) => {
        const api = window[`__${example.replaceAll("-", "_")}`];
        if (example === "fusion-basic") api.simulate_trap();
        else api.hooks.runtime.enter_fatal(new Error("restart regression"));
    }, example);
    await expect(page.getByTestId("status")).toHaveAttribute("data-status", "fatal");
}

const ready = (page: Page) => expect(page.getByTestId("status")).toHaveAttribute("data-status", "ready", { timeout: 30000 });

test("review · mounted instances restart with matching SDK identity and release URL ownership", async ({ page }, info) => {
    const example = info.project.name;
    await page.goto("./");
    await ready(page);
    const container = example === "component-catalog" ? "catalog" : example === "fusion-basic" ? "b0" : "workbench";
    if (example === "component-catalog") {
        await page.evaluate(() => window.__component_catalog.mount_second());
    }
    for (let instance = 2; instance <= 3; instance++) {
        await fatal(page, example);
        await expect(page.locator(`#${container}`)).not.toHaveAttribute("data-rustify-scope");
        expect(await page.evaluate(() => document.documentElement.getAttribute("data-rustify-url-owner"))).toBeNull();
        if (example === "component-catalog") {
            await expect(page.locator("#catalog-second")).not.toHaveAttribute("data-rustify-scope");
        }
        await page.getByTestId("fatal-restart").click();
        await ready(page);
        await expect(page.locator(`#${container}`)).toHaveAttribute("data-rustify-scope");
        const runtime = await page.evaluate((example) => window[`__${example.replaceAll("-", "_")}`].diagnostics().runtime, example);
        expect(runtime).toBe(instance);
        if (example.endsWith("workbench")) {
            expect(await page.evaluate(() => document.documentElement.getAttribute("data-rustify-url-owner"))).toBe(`${instance}:workbench`);
        }
    }
    await fatal(page, example);
    expect(await page.evaluate(() => document.documentElement.getAttribute("data-rustify-url-owner"))).toBeNull();
});

test("review · a failed restart load shows an error and a working reload action", async ({ page }, info) => {
    const errors: string[] = [];
    page.on("pageerror", (error) => errors.push(error.message));
    await page.goto("./");
    await ready(page);
    await fatal(page, info.project.name);
    errors.length = 0;
    await page.route("**/bindgen.js?instance=*", (route) => route.abort("failed"));
    await page.getByTestId("fatal-restart").click();
    await expect(page.getByTestId("status")).toHaveAttribute("data-status", "failed");
    await expect(page.getByTestId("fatal-text")).toContainText(/module|fetch|load/i);
    await expect(page.getByRole("button", { name: /reload/i })).toBeVisible();
    expect(errors).toEqual([]);
    await page.unroute("**/bindgen.js?instance=*");
    await page.getByRole("button", { name: /reload/i }).click();
    await ready(page);
});
