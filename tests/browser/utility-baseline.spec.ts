import { test, expect } from "@playwright/test";

test("Rust compiler Worker installs a prepared scope under strict CSP", async ({ page }, testInfo) => {
    const errors: string[] = [];
    page.on("pageerror", error => errors.push(error.message));
    page.on("requestfailed", request => errors.push(`${request.url()}: ${request.failure()?.errorText}`));
    await page.goto("./");
    const report = await page.evaluate(async () => (window as any).utilityProbe.compile());
    expect(report.ready.scanner_enabled).toBe(false);
    expect(report.ready.shared_memory).toBe(false);
    expect(report.prepared.revision).toBe(1);
    expect(report.uncommittedRevision).toBe(0);
    expect(report.committed.revision).toBe(1);
    expect(report.installed.width).toBe("137px");
    expect(report.installed.background).toBe("rgb(18, 52, 86)");
    expect(report.outsideWidth).not.toBe("137px");
    expect(report.boundaryWidth).not.toBe("137px");
    expect(report.repeated.changed).toBe(false);
    expect(report.repeated.metrics.parsed_candidates).toBe(0);
    expect(report.repeated.metrics.generated_candidates).toBe(0);
    expect(report.cleanedWidth).toBe(report.beforeWidth);
    expect(report.violations).toEqual([]);
    expect(errors).toEqual([]);
    await testInfo.attach("Rust Worker stylesheet", { body: JSON.stringify(report, null, 2), contentType: "application/json" });
});

test("scanner-free module Worker roundtrips complete class names through Rust", async ({ page }) => {
    const errors: string[] = [];
    page.on("pageerror", error => errors.push(error.message));
    page.on("requestfailed", request => errors.push(`${request.url()}: ${request.failure()?.errorText}`));
    const response = await page.goto("./");
    const csp = response!.headers()["content-security-policy"];
    expect(csp).toContain("style-src 'self'");
    expect(csp).not.toContain("'unsafe-inline'");
    expect(csp).not.toContain("'unsafe-eval'");
    const report = await page.evaluate(async () => (window as any).utilityProbe.worker());
    expect(report.candidates).toEqual(["w-[137px]", "bg-[#123456]", "hover:p-7"]);
    expect(report.scannerEnabled).toBe(false);
    expect(report.sharedMemory).toBe(false);
    expect(report.protocolVersion).toBe(1);
    expect(errors).toEqual([]);
});

test("strict CSP accepts staged scoped sheets, nesting and registered properties", async ({ page }, testInfo) => {
    const errors: string[] = [];
    page.on("pageerror", error => errors.push(error.message));
    page.on("requestfailed", request => errors.push(`${request.url()}: ${request.failure()?.errorText}`));
    await page.goto("./");
    const report = await page.evaluate(async () => (window as any).utilityProbe.css());
    expect(report.before.width).not.toBe("137px");
    expect(report.staged.width).toBe(report.before.width);
    expect(report.installed.width).toBe("137px");
    expect(report.installed.background).toBe("rgb(18, 52, 86)");
    expect(report.installed.nestedColor).toBe("rgb(21, 112, 239)");
    expect(report.installed.propertyWidth).toBe("9px");
    expect(report.outside.width).not.toBe("137px");
    expect(report.boundary.width).not.toBe("137px");
    expect(report.cleaned.width).toBe(report.before.width);
    expect(report.sheetCountAfter).toBe(report.sheetCountBefore);
    expect(report.violations).toEqual([]);
    expect(errors).toEqual([]);
    await testInfo.attach("CSSOM probe", { body: JSON.stringify(report, null, 2), contentType: "application/json" });
});

test("extractor WASM returns the same fixed memory fixture as native", async ({ page }) => {
    const errors: string[] = [];
    page.on("pageerror", error => errors.push(error.message));
    page.on("requestfailed", request => errors.push(`${request.url()}: ${request.failure()?.errorText}`));
    await page.goto("./");
    const report = await page.evaluate(async () => (window as any).utilityProbe.scanner());
    expect(report.actual).toEqual(report.expected);
    expect(report.sharedMemory).toBe(false);
    expect(errors).toEqual([]);
});

test("strict CSP installs official Tailwind dependencies and nested pseudo selectors", async ({ page }, testInfo) => {
    const errors: string[] = [];
    page.on("pageerror", error => errors.push(error.message));
    page.on("requestfailed", request => errors.push(`${request.url()}: ${request.failure()?.errorText}`));
    await page.goto("./");
    const report = await page.evaluate(async () => (window as any).utilityProbe.official());
    expect(report.width).toBe("137px");
    expect(report.background).toBe("rgb(18, 52, 86)");
    expect(report.beforeContent).toBe('"probe"');
    expect(report.boxShadow).toContain("3px");
    expect(report.outsideWidth).not.toBe("137px");
    expect(report.cleanedWidth).not.toBe("137px");
    expect(report.violations).toEqual([]);
    expect(errors).toEqual([]);
    await testInfo.attach("Official stylesheet probe", { body: JSON.stringify(report, null, 2), contentType: "application/json" });
});

test("strict CSP probe fixtures remain readable at desktop and mobile widths", async ({ page }, testInfo) => {
    await page.goto("./");
    await page.evaluate(async () => {
        const sheet = new CSSStyleSheet();
        sheet.replaceSync(await fetch("./official.css").then(response => response.text()));
        document.adoptedStyleSheets = [...document.adoptedStyleSheets, sheet];
        document.querySelector("#sample")!.className = 'w-[137px] bg-[#123456] ring-[3px] before:content-["probe"]';
    });
    for (const [name, width, height] of [["desktop", 1280, 900], ["mobile", 390, 844]] as const) {
        await page.setViewportSize({ width, height });
        expect(await page.evaluate(() => document.documentElement.scrollWidth)).toBeLessThanOrEqual(width);
        await expect(page.getByRole("heading", { name: "Style integration probes" })).toBeVisible();
        await testInfo.attach(name, { body: await page.screenshot({ fullPage: true }), contentType: "image/png" });
    }
});

test("cn WASM facade answers every fixed runtime case from the page realm under strict CSP", async ({ page }, testInfo) => {
    const errors: string[] = [];
    page.on("pageerror", error => errors.push(error.message));
    page.on("requestfailed", request => errors.push(`${request.url()}: ${request.failure()?.errorText}`));
    await page.goto("./");
    const report = await page.evaluate(async () => (window as any).utilityProbe.cn());
    await testInfo.attach("cn browser crossing", { body: JSON.stringify(report, null, 2), contentType: "application/json" });
    expect(report.fixedCases).toBe(947);
    expect(report.failures).toEqual([]);
    expect(report.passed).toBe(947);
    expect(report.callbackExceptionIdentity).toBe(true);
    expect(report.sharedMemory).toBe(false);
    expect(report.violations).toEqual([]);
    expect(errors).toEqual([]);
});
