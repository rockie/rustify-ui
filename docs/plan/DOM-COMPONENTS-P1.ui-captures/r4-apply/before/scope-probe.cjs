const { chromium } = require("@playwright/test");
const fs = require("node:fs");
const path = require("node:path");

const OUT = __dirname;
const STUDIO = "http://127.0.0.1:4181/";
const tid = (page, id) => page.getByTestId(id);

async function select(page, id, label) {
    await tid(page, id).click();
    await tid(page, `${id}-list`).getByRole("option", { name: label, exact: true }).click();
}

async function quiet(page) {
    await page.evaluate(() => document.fonts.ready);
    await page.waitForLoadState("networkidle").catch(() => {});
    await page.waitForTimeout(600);
}

async function shot(page, name) {
    await quiet(page);
    await page.screenshot({ path: path.join(OUT, name), animations: "disabled" });
}

(async () => {
    const browser = await chromium.launch({ headless: true });
    const context = await browser.newContext({ viewport: { width: 1440, height: 900 }, deviceScaleFactor: 1, reducedMotion: "reduce", colorScheme: "light", locale: "en-US" });
    const page = await context.newPage();
    const health = { pageErrors: [], consoleErrors: [], failed: [], csp: [] };
    page.on("pageerror", error => health.pageErrors.push(error.message));
    page.on("console", message => { if (message.type() === "error") health.consoleErrors.push(message.text()); });
    page.on("requestfailed", request => health.failed.push(`${request.url()} ${request.failure()?.errorText}`));
    page.on("response", response => { if (response.status() >= 400) health.failed.push(`${response.status()} ${response.url()}`); });
    await page.addInitScript(() => {
        window.__reviewCsp = [];
        document.addEventListener("securitypolicyviolation", event => window.__reviewCsp.push(event.violatedDirective));
    });
    await page.goto(STUDIO);
    await tid(page, "status").and(page.locator("[data-status=ready]")).waitFor({ state: "attached", timeout: 90000 });
    await quiet(page);
    await tid(page, "preset-search").fill("Graphite");
    await select(page, "preset-select", "Built-in · Graphite");
    await select(page, "studio-config-accent_color", "Pink");
    await select(page, "studio-config-gray_color", "Auto");

    const group = tid(page, "group-override-tokens");
    await group.locator("summary").click();
    await group.evaluate(node => node.scrollIntoView({ block: "center" }));
    await shot(page, "APPLY-group-tokens-1440-light.png");
    const groupEvidence = await group.evaluate(node => ({ open: node.open, text: node.innerText, rect: node.getBoundingClientRect().toJSON() }));

    await tid(page, "editor-colors").click();
    await tid(page, "color-search").fill("");
    await page.locator(".ce-search").evaluate(node => node.scrollIntoView({ block: "center" }));
    await shot(page, "APPLY-search-row-1440-light.png");
    const search = await page.locator(".ce-search").evaluate(node => {
        const input = node.querySelector("[data-testid=color-search]");
        const clear = node.querySelector("[data-testid=color-search-clear]");
        const rect = element => ({ ...element.getBoundingClientRect().toJSON(), height: element.getBoundingClientRect().height });
        return { input: rect(input), clear: rect(clear) };
    });

    await tid(page, "color-token-primary").click();
    const reset = tid(page, "color-reset-primary");
    await reset.focus();
    await page.keyboard.press("Shift+Tab");
    await page.keyboard.press("Tab");
    await reset.evaluate(node => node.scrollIntoView({ block: "center" }));
    await shot(page, "APPLY-reset-focus-1440-light.png");
    const resetEvidence = await reset.evaluate(node => {
        const style = getComputedStyle(node);
        return {
            focused: document.activeElement === node,
            ariaLabel: node.getAttribute("aria-label"),
            title: node.getAttribute("title"),
            ariaDisabled: node.getAttribute("aria-disabled"),
            disabled: node.disabled,
            outline: `${style.outlineWidth} ${style.outlineStyle} ${style.outlineColor}`,
            visibleMode: node.closest(".ce-token-choice")?.querySelector(".ce-token-mode")?.textContent,
            sourceClear: document.querySelector("[data-testid=source-clear]")?.textContent,
        };
    });
    health.csp = await page.evaluate(() => window.__reviewCsp || []);
    const out = { group: groupEvidence, search, reset: resetEvidence, health };
    fs.writeFileSync(path.join(OUT, "scope-probe.json"), JSON.stringify(out, null, 2));
    await browser.close();
    console.log(JSON.stringify(out, null, 2));
})().catch(error => { console.error(error); process.exit(1); });
