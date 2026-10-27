import type { ConsoleMessage, Locator, Page, Request, Response } from "@playwright/test";
import { test as base, expect, waitForReady } from "./support";

type CspWindow = Window & {
    __buttonFixtureCsp?: { violations: string[]; listener: EventListener };
};

const test = base.extend<{ buttons: Locator }>({
    buttons: async ({ page }, use) => {
        await waitForReady(page);
        const viewport = page.viewportSize();
        const root = page.getByTestId("catalog-component-fixture");
        const previousStyle = await root.evaluate(element => (element as HTMLElement).style.cssText);
        const regions = await page.evaluate(() => window.__component_catalog.live_regions());
        const failures: string[] = [];
        const onError = (error: Error) => failures.push(error.message);
        const onConsole = (message: ConsoleMessage) => {
            if (message.type() === "error") failures.push(message.text());
        };
        const onRequest = (request: Request) => failures.push(request.url());
        const onResponse = (response: Response) => {
            if (response.status() >= 400) failures.push(`${response.status()} ${response.url()}`);
        };
        page.on("pageerror", onError);
        page.on("console", onConsole);
        page.on("requestfailed", onRequest);
        page.on("response", onResponse);
        await page.evaluate(() => {
            const state = { violations: [] as string[], listener: null as unknown as EventListener };
            state.listener = event => state.violations.push((event as SecurityPolicyViolationEvent).violatedDirective);
            (window as CspWindow).__buttonFixtureCsp = state;
            document.addEventListener("securitypolicyviolation", state.listener);
            window.__component_catalog.mount_button_fixtures();
        });
        await expect(root.getByTestId("button-default")).toBeVisible();
        expect(await page.evaluate(() => window.__component_catalog.live_regions())).toBe(regions);
        try {
            await use(root);
        } finally {
            const violations = await page.evaluate(() => {
                window.__component_catalog.dispose_button_fixtures();
                const state = (window as CspWindow).__buttonFixtureCsp;
                if (state) document.removeEventListener("securitypolicyviolation", state.listener);
                delete (window as CspWindow).__buttonFixtureCsp;
                return state?.violations ?? [];
            });
            page.off("pageerror", onError);
            page.off("console", onConsole);
            page.off("requestfailed", onRequest);
            page.off("response", onResponse);
            if (viewport) await page.setViewportSize(viewport);
            await expect(root).toBeEmpty();
            expect(await root.evaluate(element => (element as HTMLElement).style.cssText)).toBe(previousStyle);
            expect(await page.evaluate(() => window.__component_catalog.live_regions())).toBe(regions);
            expect(violations).toEqual([]);
            expect(failures).toEqual([]);
        }
    },
});

const colors = (node: Locator) => node.evaluate(element => {
    const style = getComputedStyle(element);
    return [style.backgroundColor, style.color];
});

const height = (node: Locator) => node.evaluate(element => element.getBoundingClientRect().height);

async function flush(page: Page) {
    await page.evaluate(() => new Promise<void>(resolve => requestAnimationFrame(() => requestAnimationFrame(() => resolve()))));
}

async function syntheticClick(node: Locator, cancelable: boolean) {
    return node.evaluate((element, cancelable) => {
        const button = element as HTMLButtonElement;
        // Model a caller's native attribute/property removing disabled after
        // rendering. The component must still reject activation and submit.
        button.disabled = false;
        const dispatched = button.dispatchEvent(new MouseEvent("click", { bubbles: true, cancelable }));
        return { dispatched, disabled: button.disabled };
    }, cancelable);
}

test("all button variants and sizes render with scoped focus, hover and icon semantics", async ({ page, buttons }, info) => {
    await page.setViewportSize({ width: 1280, height: 900 });
    const variants = ["classic", "solid", "soft", "surface", "outline", "ghost-offset", "ghost"];
    for (const variant of variants) {
        for (let size = 1; size <= 4; size += 1) {
            const button = buttons.getByTestId(`button-${variant}-${size}`);
            await expect(button).toHaveAttribute("data-variant", variant);
            await expect(button).toHaveAttribute("type", "button");
            await expect(button).toHaveCSS("font-size", `${[12, 14, 16, 18][size - 1]}px`);
            expect(await height(button)).toBeCloseTo((variant === "ghost" ? [24, 28, 36, 42] : [24, 32, 40, 48])[size - 1], 1);
            await expect(button).not.toHaveAttribute("aria-controls");
            await expect(button).not.toHaveAttribute("aria-expanded");
        }
    }
    await expect(buttons.getByTestId("button-default")).toHaveAttribute("data-variant", "solid");
    expect(await height(buttons.getByTestId("button-default"))).toBe(32);
    const icon = buttons.getByTestId("button-icon");
    await expect(icon).toHaveAccessibleName("Close preview");
    await expect(icon).toHaveCSS("width", "32px");
    await expect(icon.locator(".rustify-button-content")).toHaveAttribute("aria-hidden", "true");
    await expect(icon.getByRole("img")).toHaveCount(0);
    await icon.click();
    await expect(buttons.getByTestId("button-counts")).toHaveAttribute("data-icon-request", "1");

    const action = buttons.getByTestId("button-solid-2");
    await action.focus();
    await page.keyboard.press("Tab");
    await page.keyboard.press("Shift+Tab");
    await expect(action).toBeFocused();
    await expect(action).toHaveCSS("outline-style", "solid");
    await expect(action).toHaveCSS("outline-width", "2px");
    await expect(action).toHaveCSS("outline-offset", "2px");
    const beforeHover = await colors(action);
    await action.hover();
    await expect.poll(() => colors(action)).not.toEqual(beforeHover);
    await action.click();
    await expect(buttons.getByTestId("button-counts")).toHaveAttribute("data-matrix-request", "1");

    const disclosure = buttons.getByTestId("button-disclosure");
    await expect(disclosure).toHaveAttribute("aria-controls", "button-details");
    await expect(disclosure).toHaveAttribute("aria-expanded", "false");
    await disclosure.click();
    await expect(disclosure).toHaveAttribute("aria-expanded", "true");
    await expect(buttons.locator("#button-details")).toBeVisible();
    await page.mouse.move(0, 0);
    const screenshot = process.env.RUSTIFY_UI_EVIDENCE_OUT ? `${process.env.RUSTIFY_UI_EVIDENCE_OUT}/buttons-desktop.png` : "docs/validation/dom-components-p1/buttons-desktop.png";
    await buttons.screenshot({ path: screenshot, animations: "disabled" });
    await info.attach("DOM buttons desktop", { path: screenshot, contentType: "image/png" });
});

test("component colors, semantic overrides, radius and scaling remain local", async ({ buttons }) => {
    const primary = buttons.getByTestId("button-default");
    const primaryToken = buttons.getByTestId("button-primary-token");
    expect(await colors(primary)).toEqual(await colors(primaryToken));
    expect(await colors(buttons.getByTestId("button-semantic-accent"))).toEqual(await colors(buttons.getByTestId("button-accent-token")));
    expect(await colors(buttons.getByTestId("button-semantic-destructive"))).toEqual(await colors(buttons.getByTestId("button-destructive-token")));
    await expect(buttons.getByTestId("button-colors").locator('[data-name="Button"]')).toHaveCount(27);
    const blue = buttons.getByTestId("button-color-blue");
    const localBefore = await colors(blue);
    const highContrast = buttons.getByTestId("button-high-contrast");
    const contrastBefore = await colors(highContrast);
    expect(contrastBefore).not.toEqual(localBefore);
    await expect(highContrast).toHaveCSS("background-color", "rgb(17, 50, 100)");
    await buttons.getByTestId("button-override-primary").click();
    await expect(primary).toHaveCSS("background-color", "rgb(17, 34, 51)");
    await expect(primary).toHaveCSS("color", "rgb(255, 255, 255)");
    expect(await colors(primary)).toEqual(await colors(primaryToken));
    expect(await colors(blue)).toEqual(localBefore);
    expect(await colors(buttons.getByTestId("button-semantic-accent"))).toEqual(await colors(buttons.getByTestId("button-accent-token")));

    for (const [radius, pixels] of [["none", 0], ["small", 3], ["medium", 4], ["large", 6], ["full", 9999]] as const) {
        await expect(buttons.getByTestId(`button-radius-${radius}`)).toHaveCSS("border-top-left-radius", `${pixels}px`);
    }
    const override = buttons.getByTestId("button-class-override");
    await expect(override).toHaveCSS("height", "52px");
    await expect(override).toHaveCSS("padding-left", "22px");
    await expect(override).toHaveCSS("border-radius", "0px");
    expect((await colors(override))[0]).toBe((await colors(buttons.getByTestId("button-destructive-token")))[0]);
    await buttons.getByTestId("button-scale-110").click();
    await expect.poll(() => height(primary)).toBeCloseTo(35.2, 1);
    await expect(buttons.getByTestId("button-radius-medium")).toHaveCSS("border-top-left-radius", "4.4px");
    await expect(buttons.getByTestId("button-radius-full")).toHaveCSS("border-top-left-radius", "9999px");
    await expect(override).toHaveCSS("height", "52px");
    await expect(override).toHaveCSS("padding-left", "22px");
    await buttons.getByTestId("button-theme-full").click();
    await expect(primary).toHaveCSS("border-top-left-radius", "9999px");
    await expect(buttons.getByTestId("button-radius-none")).toHaveCSS("border-top-left-radius", "0px");
    await buttons.getByTestId("button-mode-dark").click();
    await expect(buttons).toHaveAttribute("data-theme", "dark");
    await expect(highContrast).toHaveCSS("background-color", "rgb(194, 230, 255)");
    expect(await colors(blue)).toEqual(localBefore);
    expect(await colors(primary)).toEqual(await colors(primaryToken));
    await buttons.getByTestId("button-mode-light").click();
    await expect.poll(() => colors(highContrast)).toEqual(contrastBefore);
    await expect.poll(() => colors(blue)).toEqual(localBefore);
});

test("button size changes only at the six Themes viewport breakpoints", async ({ page, buttons }, info) => {
    const action = buttons.getByTestId("button-responsive");
    for (const [width, expected] of [[390, 24], [519, 24], [520, 32], [767, 32], [768, 40], [1023, 40], [1024, 48], [1279, 48], [1280, 24], [1639, 24], [1640, 32], [1800, 32]]) {
        await page.setViewportSize({ width, height: 900 });
        await expect.poll(() => height(action), `${width}px viewport`).toBe(expected);
    }
    await page.setViewportSize({ width: 390, height: 900 });
    await expect(action).toHaveCSS("height", "24px");
    const screenshot = process.env.RUSTIFY_UI_EVIDENCE_OUT ? `${process.env.RUSTIFY_UI_EVIDENCE_OUT}/buttons-mobile.png` : "docs/validation/dom-components-p1/buttons-mobile.png";
    await buttons.screenshot({ path: screenshot, animations: "disabled" });
    await info.attach("DOM buttons mobile", { path: screenshot, contentType: "image/png" });
});

test("loading blocks click and native submit or reset while unavailable still delivers requests", async ({ page, buttons }) => {
    const counts = buttons.getByTestId("button-counts");
    const submit = buttons.getByTestId("button-native-submit");
    const reset = buttons.getByTestId("button-native-reset");
    const draft = buttons.getByRole("textbox", { name: "Draft title" });
    await expect(submit).toHaveAttribute("name", "intent");
    await expect(submit).toHaveAttribute("value", "save");
    await buttons.getByTestId("button-native-default").click();
    await expect(counts).toHaveAttribute("data-default-request", "1");
    await expect(counts).toHaveAttribute("data-submit", "0");
    await submit.click();
    await expect(counts).toHaveAttribute("data-submit-request", "1");
    await expect(counts).toHaveAttribute("data-submit", "1");
    await draft.fill("Changed");
    await reset.click();
    await expect(counts).toHaveAttribute("data-reset-request", "1");
    await expect(counts).toHaveAttribute("data-reset", "1");
    await expect(draft).toHaveValue("Initial");

    const unavailable = buttons.getByTestId("button-native-unavailable");
    await expect(unavailable).toHaveAttribute("aria-disabled", "true");
    await expect.poll(() => unavailable.evaluate(element => (element as HTMLButtonElement).disabled)).toBe(false);
    await expect(unavailable).not.toHaveAttribute("disabled");
    await unavailable.focus();
    await expect(unavailable).toBeFocused();
    await page.keyboard.press("Enter");
    await expect(counts).toHaveAttribute("data-unavailable-request", "1");
    await expect(counts).toHaveAttribute("data-submit", "2");
    const disabled = buttons.getByTestId("button-native-disabled");
    await expect(disabled).toBeDisabled();
    await disabled.focus();
    await expect(disabled).not.toBeFocused();
    await disabled.dispatchEvent("click");
    await expect(counts).toHaveAttribute("data-disabled-request", "0");

    const width = await submit.evaluate(element => element.getBoundingClientRect().width);
    await buttons.getByTestId("button-toggle-loading").click();
    await expect(submit).toBeDisabled();
    await expect(submit).toHaveAttribute("aria-busy", "true");
    await expect(submit).toHaveAccessibleName("Save changes");
    await expect(submit).toHaveText("Save changes");
    expect(await submit.evaluate(element => element.getBoundingClientRect().width)).toBeCloseTo(width, 2);
    await expect(submit.locator(".rustify-button-content")).toHaveCSS("opacity", "0");
    await expect(submit.locator(".rustify-button-content")).not.toHaveAttribute("aria-hidden", "true");
    await expect(submit.locator(".rustify-button-spinner")).toHaveAttribute("aria-hidden", "true");
    await expect(submit.getByRole("status")).toHaveCount(0);
    for (const cancelable of [true, false]) {
        expect(await syntheticClick(submit, cancelable)).toEqual({ dispatched: !cancelable, disabled: true });
        expect(await syntheticClick(reset, cancelable)).toEqual({ dispatched: !cancelable, disabled: true });
    }
    await flush(page);
    await expect(counts).toHaveAttribute("data-submit-request", "1");
    await expect(counts).toHaveAttribute("data-submit", "2");
    await expect(counts).toHaveAttribute("data-reset-request", "1");
    await expect(counts).toHaveAttribute("data-reset", "1");
    await buttons.getByTestId("button-toggle-loading").click();
    await expect(submit).toBeEnabled();
    await expect(submit).not.toHaveAttribute("aria-busy");
    await expect(submit.locator(".rustify-button-spinner")).toHaveCount(0);
    await submit.click();
    await expect(counts).toHaveAttribute("data-submit-request", "2");
    await expect(counts).toHaveAttribute("data-submit", "3");
    await draft.fill("Changed again");
    await reset.click();
    await expect(draft).toHaveValue("Initial");
    await expect(counts).toHaveAttribute("data-reset-request", "2");
    await expect(counts).toHaveAttribute("data-reset", "2");
});

test("ThemePanel requests stay controlled, preserve overrides and reset only their two-mode group", async ({ page, buttons }) => {
    const panel = buttons.getByTestId("button-theme-panel");
    const counts = buttons.getByTestId("button-counts");
    const brand = panel.getByTestId("button-theme-panel-accent_color");
    const brandStatus = panel.locator('[data-theme-config="accent_color"] [data-name="ThemeOverrideCount"]');
    const panelStatus = panel.locator('[data-theme-config="panel_background"] [data-name="ThemeOverrideCount"]');
    const radiusStatus = panel.locator('[data-theme-config="radius"] [data-name="ThemeOverrideCount"]');
    await expect(panel.getByRole("combobox")).toHaveCount(6);
    await expect(brand).toHaveAccessibleName("Brand color");
    await expect(brand).toHaveText("Indigo");
    await expect(panel).toContainText("Overrides remain in place when configuration changes.");
    await buttons.getByTestId("button-panel-seed").click();
    await expect(brandStatus).toHaveAttribute("aria-label", "Overridden · Light 2 · Dark 1");
    await expect(panelStatus).toHaveAttribute("aria-label", "Overridden · Light 1 · Dark 1");
    await expect(radiusStatus).toHaveAttribute("aria-label", "Overridden · Light 1 · Dark 1");
    const before = await counts.evaluate(element => [
        element.getAttribute("data-override-light"), element.getAttribute("data-override-dark"),
        element.getAttribute("data-independent-light"), element.getAttribute("data-independent-dark"),
    ]);
    await buttons.getByTestId("button-panel-toggle-accept").click();
    await expect(counts).toHaveAttribute("data-panel-accepting", "false");
    await brand.click();
    await page.getByTestId("button-theme-panel-accent_color-list").getByRole("option", { name: "Tomato", exact: true }).click();
    await expect(counts).toHaveAttribute("data-panel-request", "1");
    await expect(counts).toHaveAttribute("data-panel-patch-fields", "1");
    await expect(counts).toHaveAttribute("data-panel-last-patch", "accent_color");
    await expect(brand).toHaveText("Indigo");

    await buttons.getByTestId("button-panel-toggle-accept").click();
    await brand.click();
    await page.getByTestId("button-theme-panel-accent_color-list").getByRole("option", { name: "Tomato", exact: true }).click();
    await expect(brand).toHaveText("Tomato");
    await expect(counts).toHaveAttribute("data-panel-request", "2");
    await expect(counts).toHaveAttribute("data-panel-patch-fields", "1");
    await expect(brandStatus).toHaveAttribute("aria-label", "Overridden · Light 2 · Dark 1");
    await expect(buttons.getByTestId("button-default")).toHaveCSS("background-color", "rgb(17, 34, 51)");
    expect(await counts.evaluate(element => [
        element.getAttribute("data-override-light"), element.getAttribute("data-override-dark"),
        element.getAttribute("data-independent-light"), element.getAttribute("data-independent-dark"),
    ])).toEqual(before);
    const gray = panel.getByTestId("button-theme-panel-gray_color");
    await gray.click();
    await page.getByTestId("button-theme-panel-gray_color-list").getByRole("option", { name: "Custom", exact: true }).click();
    await expect(gray).toHaveText("Custom");
    await expect(brand).toHaveText("Tomato");
    await expect(counts).toHaveAttribute("data-panel-request", "3");
    await expect(counts).toHaveAttribute("data-panel-patch-fields", "1");
    await expect(counts).toHaveAttribute("data-panel-last-patch", "gray_color");

    await buttons.getByTestId("button-panel-chinese").click();
    await expect(panel).toHaveAttribute("lang", "zh-CN");
    await expect(panel.getByRole("heading", { name: "主题基础配置" })).toBeVisible();
    await expect(gray).toHaveText("自定义");
    await expect(brand).toHaveAccessibleName("品牌色族");
    await expect(panel).toContainText("其他组和独立精调保持不变。");
    await panel.getByRole("combobox", { name: "外观", exact: true }).click();
    await expect(page.getByTestId("button-theme-panel-appearance-list").getByRole("option", { name: "深色", exact: true })).toBeVisible();
    await page.keyboard.press("Escape");
    await panel.getByRole("combobox", { name: "圆角", exact: true }).click();
    await expect(page.getByTestId("button-theme-panel-radius-list").getByRole("option", { name: "全圆角", exact: true })).toBeVisible();
    await page.keyboard.press("Escape");
    const reset = panel.getByTestId("button-theme-panel-accent_color-reset");
    await expect(reset).toHaveText("使用配置值 · 两模式");
    await reset.click();
    await expect(counts).toHaveAttribute("data-panel-reset", "1");
    await expect(counts).toHaveAttribute("data-panel-last-reset", "accent_color");
    await expect(counts).toHaveAttribute("data-panel-request", "3");
    await expect(brandStatus).toHaveAttribute("aria-label", "无覆盖 · 浅色 0 · 深色 0");
    await expect(reset).toBeDisabled();
    await expect(panelStatus).toHaveAttribute("aria-label", "已覆盖 · 浅色 1 · 深色 1");
    await expect(radiusStatus).toHaveAttribute("aria-label", "已覆盖 · 浅色 1 · 深色 1");
    await expect(counts).toHaveAttribute("data-override-light", "accent,card,radius");
    await expect(counts).toHaveAttribute("data-override-dark", "accent,popover,radius");
    await expect(counts).toHaveAttribute("data-independent-light", before[2]!);
    await expect(counts).toHaveAttribute("data-independent-dark", before[3]!);
    await expect(buttons.getByTestId("button-default")).not.toHaveCSS("background-color", "rgb(17, 34, 51)");
});
