import { expect, Locator, Page, test } from "@playwright/test";

type PublicState = Record<string, unknown>;
type Diagnostic = { kind: string; scope?: string };
type RuntimeStats = { tasks: number; timers: number; observers: number; errors: number; aborted: boolean };
type Refusal = { error: string | null; before: string; after: string; sameNodes: boolean };
type Peer = {
    state(): PublicState | null;
    stats(): RuntimeStats;
    dispose(): boolean;
    trap(): void;
    fatal(): string | null;
};

declare global {
    interface Window {
        __hydrate_csp: string[];
        __hydration: {
            mode: string;
            scopes: string[];
            capture(scope: string): { html: string[]; rawHtml: string[]; nodes: number; elements: number };
            identity(scope: string): { root: boolean; nodes: number; retained: number; ids: boolean };
            content(scope: string): string[];
            rawContent(scope: string): string[];
            start(scope: string): number;
            startAll(): Record<string, string[]>;
            attempt(scope: string): { handle: number | null; error: string | null };
            state(scope: string): PublicState | null;
            diagnostics(): { entries: Diagnostic[] };
            stats(): RuntimeStats;
            dispose(scope: string): boolean;
            asynchronous(scope: string): boolean;
            reject(kind: string): Refusal;
            fatal(): string | null;
            trap(): void;
            peers: Record<string, Peer>;
            bootPeer(slot: string, seed: string): Promise<number>;
        };
    }
}

const scopes = ["catalog-a", "catalog-b", "catalog-c"];
const root = (page: Page, scope: string) => page.locator(`[data-harness-root="${scope}"] > div`);
const control = (page: Page, scope: string, name: string) => page.getByTestId(`${scope}-${name}`);

async function open(page: Page, mode: "hydrate" | "csr" | "preboot" = "hydrate") {
    const errors: string[] = [];
    page.on("pageerror", (error) => errors.push(String(error)));
    page.on("console", (message) => { if (message.type() === "error") errors.push(message.text()); });
    page.on("requestfailed", (request) => errors.push(`${request.method()} ${request.url()}: ${request.failure()?.errorText}`));
    await page.addInitScript(() => {
        window.__hydrate_csp = [];
        document.addEventListener("securitypolicyviolation", (event) => {
            window.__hydrate_csp.push(`${event.violatedDirective} ${event.blockedURI}`);
        });
    });
    await page.goto(mode === "hydrate" ? "./" : `./${mode}.html`);
    await expect(page.getByTestId("status")).toHaveAttribute("data-status", "runtime-ready");
    return errors;
}

async function settle(page: Page) {
    await page.evaluate(() => new Promise<void>((resolve) => requestAnimationFrame(() => requestAnimationFrame(() => resolve()))));
}

async function attach(page: Page) {
    const immediate = await page.evaluate(() => window.__hydration.startAll());
    await expect(page.getByTestId("status")).toHaveAttribute("data-status", "attached");
    await settle(page);
    return immediate;
}

async function publicState(page: Page, scope: string) {
    return page.evaluate((scope) => {
        const state = window.__hydration.state(scope);
        if (state === null) return null;
        // Observer deliveries and the cleanup listener are lifecycle probes,
        // separate from the component's deterministic application state.
        const { keys, resizes, ...display } = state;
        return display;
    }, scope);
}

async function initialControls(page: Page, scope: string) {
    await expect(root(page, scope).locator("[data-category]")).toHaveCount(24);
    await expect(control(page, scope, "count")).toHaveText("7");
    await expect(control(page, scope, "text")).toHaveValue("Résumé <&>");
    await expect(control(page, scope, "text-readonly")).toBeDisabled();
    await expect(control(page, scope, "text-readonly")).toHaveJSProperty("readOnly", true);
    await expect(control(page, scope, "area")).toHaveValue("\nRésumé </textarea> & <line>\nCRLF\nCR\nlast");
    await expect(control(page, scope, "checkbox")).toBeChecked();
    await expect(control(page, scope, "radio-north")).toBeChecked();
    await expect(control(page, scope, "radio-blocked")).toBeDisabled();
    await expect(control(page, scope, "radio-south")).not.toBeChecked();
    await expect(control(page, scope, "switch")).toHaveAttribute("aria-checked", "false");
    await expect(control(page, scope, "select")).toHaveText("Red");
    await expect(control(page, scope, "select")).toHaveAttribute("aria-expanded", "false");
    await expect(control(page, scope, "slider")).toHaveValue("42.5");
    await expect(control(page, scope, "progress")).toHaveAttribute("aria-valuenow", "34");
    await expect(control(page, scope, "loading")).toHaveAttribute("role", "status");
    await expect(control(page, scope, "loading")).toHaveAttribute("aria-label", "Loading fixture");
    await expect(control(page, scope, "icon")).toHaveAttribute("data-glyph", "close");
    await expect(control(page, scope, "icon").locator("path")).toHaveCount(1);
    await expect(control(page, scope, "number")).toHaveValue("-3.25");
    await expect(control(page, scope, "color")).toHaveValue("#1a2b3c80");
    await expect(control(page, scope, "toggle-italic")).toHaveAttribute("aria-pressed", "true");
    await expect(control(page, scope, "toggle-underline")).toBeDisabled();
    await expect(control(page, scope, "tabs").getByRole("tab", { name: "Detail", exact: true })).toHaveAttribute("aria-selected", "true");
    await expect(control(page, scope, "tabs").getByRole("tabpanel")).toHaveText("Detail panel");
    await expect(control(page, scope, "tree-alpha")).toHaveAttribute("aria-selected", "true");
    await expect(control(page, scope, "table")).toHaveAttribute("aria-rowcount", "5");
    await expect(control(page, scope, "table")).toHaveAttribute("aria-colcount", "3");
    await expect(control(page, scope, "table-row-1")).toHaveAttribute("aria-selected", "true");
    await expect(control(page, scope, "static-toast")).toContainText("Deterministic status");
    await expect(root(page, scope).locator("[style]")).toHaveCount(0);
    await expect(root(page, scope).locator("[role=dialog], [role=tooltip], [role=menu], [role=listbox]")).toHaveCount(0);
}

async function nativeColor(page: Page, color: string) {
    return page.evaluate((color) => {
        const reference = document.createElement("span");
        reference.style.backgroundColor = color;
        document.body.append(reference);
        const canonical = getComputedStyle(reference).backgroundColor;
        reference.remove();
        return canonical;
    }, color);
}

async function geometry(page: Page, scope: string) {
    await expect(control(page, scope, "table-row-0")).toHaveCSS("height", "31.5px");
    await expect(control(page, scope, "table-row-1")).toHaveCSS("transform", "matrix(1, 0, 0, 1, 0, 31.5)");
    await expect(control(page, scope, "table").getByRole("columnheader").first()).toHaveCSS("width", "137.25px");
    await expect(control(page, scope, "tree-root")).toHaveCSS("padding-left", "8px");
    await expect(control(page, scope, "tree-alpha")).toHaveCSS("padding-left", "24px");
    const color = await nativeColor(page, "#1a2b3c80");
    await expect(control(page, scope, "color-swatch")).toHaveCSS("background-color", color);
    await expect(control(page, scope, "splitter").getByRole("separator")).toHaveCSS("left", "143.5px");
    const progress = await control(page, scope, "progress").evaluate((element) => {
        const fill = element.firstElementChild as HTMLElement;
        return fill.getBoundingClientRect().width / element.getBoundingClientRect().width * 100;
    });
    expect(progress).toBeCloseTo(34, 1);
    return { color: { authored: "#1a2b3c80", native: color }, progress };
}

async function assertReferences(page: Page) {
    const faults = await page.evaluate(() => {
        const roots = [...document.querySelectorAll("[data-rustify-scope]")];
        const ids = new Set<string>();
        const faults: string[] = [];
        for (const root of roots) {
            for (const element of root.querySelectorAll("[id]")) {
                if (ids.has(element.id)) faults.push(`duplicate ${element.id}`);
                ids.add(element.id);
            }
            for (const element of root.querySelectorAll("[for], [aria-labelledby], [aria-describedby], [aria-controls]")) {
                for (const attribute of ["for", "aria-labelledby", "aria-describedby", "aria-controls"]) {
                    for (const id of (element.getAttribute(attribute) ?? "").split(/\s+/).filter(Boolean)) {
                        const target = document.getElementById(id);
                        // A closed Select keeps the identity of its future listbox.
                        if (attribute === "aria-controls" && element.getAttribute("role") === "combobox" && element.getAttribute("aria-expanded") === "false") continue;
                        if (!target || !root.contains(target)) faults.push(`${attribute}=${id} crosses or misses its scope`);
                    }
                }
            }
        }
        return faults;
    });
    expect(faults).toEqual([]);
}

async function exercise(page: Page, scope: string) {
    const states: { input: string; state: PublicState | null }[] = [];
    const record = async (input: string) => { states.push({ input, state: await publicState(page, scope) }); };
    await control(page, scope, "button").click();
    await expect(control(page, scope, "count")).toHaveText("8");
    await control(page, scope, "button-disabled").dispatchEvent("click");
    await expect(control(page, scope, "count")).toHaveText("8");
    await record("button");
    await control(page, scope, "label").click();
    await expect(control(page, scope, "label-control")).toBeFocused();
    await record("label");
    const address = page.url();
    await control(page, scope, "link").click();
    // Document islands are guest scopes, so the SDK keeps their navigation
    // in memory and preserves the host page's address.
    expect(page.url()).toBe(address);
    await record("link");
    await control(page, scope, "text").fill("Edited <&> text");
    await expect(control(page, scope, "text")).toHaveValue("Edited <&> text");
    await record("text field");
    await control(page, scope, "area").fill("Edited\nmultiline text");
    await record("text area");
    await control(page, scope, "checkbox").uncheck();
    await record("checkbox");
    await control(page, scope, "radio-south").check();
    await record("radio");
    await control(page, scope, "switch").click();
    await expect(control(page, scope, "switch")).toHaveAttribute("aria-checked", "true");
    await record("switch");
    await control(page, scope, "select").click();
    await expect(root(page, scope).getByRole("listbox")).toBeVisible();
    await expect(root(page, scope).getByRole("listbox")).toHaveAttribute("id", (await control(page, scope, "select").getAttribute("aria-controls"))!);
    await assertReferences(page);
    await root(page, scope).getByRole("option", { name: "Blue", exact: true }).click();
    await expect(control(page, scope, "select")).toHaveText("Blue");
    await expect(root(page, scope).getByRole("listbox")).toHaveCount(0);
    await record("select");
    await control(page, scope, "slider").focus();
    await control(page, scope, "slider").press("ArrowRight");
    await expect(control(page, scope, "slider")).toHaveValue("45");
    await expect(control(page, scope, "progress")).toHaveAttribute("aria-valuenow", "36");
    await record("slider and progress");
    await control(page, scope, "tooltip-trigger").hover();
    await expect(root(page, scope).getByRole("tooltip")).toHaveText("Help for this control");
    await page.mouse.move(0, 0);
    await expect(root(page, scope).getByRole("tooltip")).toHaveCount(0);
    await record("tooltip");
    await control(page, scope, "menu-trigger").click();
    await expect(root(page, scope).getByRole("menu")).toBeVisible();
    await root(page, scope).getByRole("menuitem", { name: "Copy", exact: true }).click();
    await expect(root(page, scope).getByRole("menu")).toHaveCount(0);
    await record("menu");
    await control(page, scope, "dialog-trigger").click();
    await expect(root(page, scope).getByRole("dialog")).toBeVisible();
    await expect(root(page, scope).locator("[data-rustify-content]")).toHaveAttribute("inert", "");
    await expect(control(page, scope, "dialog-close")).toHaveAttribute("aria-label", "close");
    await control(page, scope, "dialog-close").click();
    await expect(root(page, scope).getByRole("dialog")).toHaveCount(0);
    await expect(control(page, scope, "dialog-trigger")).toBeFocused();
    await expect(root(page, scope).locator("[data-rustify-content]")).not.toHaveAttribute("inert", "");
    await record("dialog");
    await control(page, scope, "tabs").getByRole("tab", { name: "Summary", exact: true }).click();
    await expect(control(page, scope, "tabs").getByRole("tabpanel")).toHaveText("Summary panel");
    await record("tabs");
    await control(page, scope, "scroll").evaluate((element) => { element.scrollTop = 24; });
    await expect(control(page, scope, "scroll")).toHaveJSProperty("scrollTop", 24);
    await record("scroll area");
    await control(page, scope, "table-row-0").getByRole("gridcell").first().focus();
    await control(page, scope, "table-row-0").getByRole("gridcell").first().press("Space");
    await expect(control(page, scope, "table-row-0")).toHaveAttribute("aria-selected", "true");
    await control(page, scope, "table-row-0").getByRole("gridcell").first().press("Enter");
    await control(page, scope, "table").getByRole("columnheader").nth(1).getByRole("button").click();
    await expect(control(page, scope, "table").getByRole("columnheader").nth(1)).toHaveAttribute("aria-sort", "descending");
    await record("data table");
    await control(page, scope, "tree-beta").click();
    await expect(control(page, scope, "tree-beta")).toHaveAttribute("aria-selected", "true");
    await control(page, scope, "tree-beta").press("ArrowLeft");
    await expect(control(page, scope, "tree-root")).toBeFocused();
    await control(page, scope, "tree-root").press("ArrowLeft");
    await expect(control(page, scope, "tree-root")).toHaveAttribute("aria-expanded", "false");
    await record("tree");
    await control(page, scope, "static-toast-dismiss").click();
    await control(page, scope, "toast-trigger").click();
    await expect(control(page, scope, "toaster-toast")).toHaveText("Runtime status");
    await control(page, scope, "toaster-toast-dismiss").click();
    await expect(control(page, scope, "toaster-toast")).toHaveCount(0);
    await record("toast");
    await control(page, scope, "number").fill("6.75");
    await control(page, scope, "number").press("Enter");
    await expect(control(page, scope, "number")).toHaveValue("6.75");
    await record("number field");
    await control(page, scope, "color").fill("#abc123ff");
    await control(page, scope, "color").press("Enter");
    await expect(control(page, scope, "color")).toHaveValue("#abc123ff");
    await expect(control(page, scope, "color-swatch")).toHaveCSS("background-color", "rgb(171, 193, 35)");
    await record("colour field");
    await control(page, scope, "toggle-bold").click();
    await expect(control(page, scope, "toggle-bold")).toHaveAttribute("aria-pressed", "true");
    await record("toggle group");
    await control(page, scope, "splitter").getByRole("separator").press("ArrowRight");
    await expect(control(page, scope, "splitter").getByRole("separator")).toHaveCSS("left", "151px");
    await record("splitter");
    await expect(root(page, scope).locator("[style]")).toHaveCount(0);
    return states;
}

async function windowListeners(page: Page) {
    const cdp = await page.context().newCDPSession(page);
    const { result } = await cdp.send("Runtime.evaluate", { expression: "window" });
    const { listeners } = await cdp.send("DOMDebugger.getEventListeners", { objectId: result.objectId! });
    await cdp.detach();
    return listeners.filter((listener) => listener.type === "harness-key").length;
}

async function widthRules(fill: Locator) {
    return fill.evaluate((element) => [...document.adoptedStyleSheets]
        .flatMap((sheet) => [...sheet.cssRules])
        .filter((rule): rule is CSSStyleRule => rule instanceof CSSStyleRule)
        .filter((rule) => rule.style.width && element.matches(rule.selectorText))
        .map((rule) => rule.style.width));
}

test("static catalog remains useful without JavaScript", async ({ browser, baseURL }, testInfo) => {
    const context = await browser.newContext({ javaScriptEnabled: false, baseURL });
    const page = await context.newPage();
    try {
        await page.goto("./");
        const catalog = await (await page.request.get("./catalog.json")).json() as string[];
        expect(catalog).toHaveLength(24);
        const geometryCSS = await (await page.request.get("./geometry.css")).text();
        expect(geometryCSS).toContain("background-color:#1a2b3c80;");
        const observations = [];
        for (const scope of scopes) {
            const rendered = await root(page, scope).locator("[data-category]").evaluateAll((elements) => elements.map((element) => element.getAttribute("data-category")));
            expect(rendered).toEqual(catalog);
            await initialControls(page, scope);
            observations.push({ scope, ...await geometry(page, scope) });
            await expect(root(page, scope)).toHaveAttribute("data-rustify-scope", scope);
            await expect(root(page, scope)).toHaveAttribute("data-rustify-ssr", "1");
            await expect(root(page, scope)).toHaveAttribute("data-rustify-seed", scope);
            await expect(root(page, scope)).not.toHaveAttribute("data-rustify-mounted");
        }
        await assertReferences(page);
        await testInfo.attach("static-geometry.json", { body: JSON.stringify(observations, null, 2), contentType: "application/json" });
    } finally {
        await context.close();
    }
});

test("all catalog nodes, IDs, ARIA and geometry survive and match independent CSR", async ({ page, context }, testInfo) => {
    const errors = await open(page);
    const before = await page.evaluate((scopes) => scopes.map((scope) => window.__hydration.capture(scope)), scopes);
    const immediate = await attach(page);
    const csr = await context.newPage();
    const csrErrors = await open(csr, "csr");
    try {
        const csrImmediate = await attach(csr);
        const hydrated = await page.evaluate((scopes) => Object.fromEntries(scopes.map((scope) => [scope, window.__hydration.content(scope)])), scopes);
        const mounted = await csr.evaluate((scopes) => Object.fromEntries(scopes.map((scope) => [scope, window.__hydration.content(scope)])), scopes);
        const rawSettled = await page.evaluate((scopes) => Object.fromEntries(scopes.map((scope) => [scope, window.__hydration.rawContent(scope)])), scopes);
        const rawCsrSettled = await csr.evaluate((scopes) => Object.fromEntries(scopes.map((scope) => [scope, window.__hydration.rawContent(scope)])), scopes);
        const identities = await page.evaluate((scopes) => scopes.map((scope) => window.__hydration.identity(scope)), scopes);
        await testInfo.attach("scope-dom.json", {
            body: JSON.stringify({ before, immediate, rawSettled, settled: hydrated, csrImmediate, rawCsrSettled, csrSettled: mounted, identities }, null, 2),
            contentType: "application/json",
        });
        for (const [index, scope] of scopes.entries()) {
            const identity = identities[index];
            expect(identity.root).toBe(true);
            expect(identity.ids).toBe(true);
            expect(identity.nodes).toBeGreaterThan(200);
            expect(identity.retained).toBe(identity.nodes);
            expect(immediate[scope]).toEqual(before[index].rawHtml);
            expect(hydrated[scope]).toEqual(mounted[scope]);
            expect(await publicState(page, scope)).toEqual(await publicState(csr, scope));
            await initialControls(page, scope);
            await initialControls(csr, scope);
            await geometry(page, scope);
            await geometry(csr, scope);
        }
        await assertReferences(page);
        await assertReferences(csr);
        await page.screenshot({ path: testInfo.outputPath("hydrated-catalog.png"), fullPage: true });
        expect(errors).toEqual([]);
        expect(csrErrors).toEqual([]);
        expect(await page.evaluate(() => window.__hydrate_csp)).toEqual([]);
        expect(await csr.evaluate(() => window.__hydrate_csp)).toEqual([]);
    } finally {
        await csr.close();
    }
});

test("the same component inputs produce the same public state in hydrate and CSR", async ({ page, context }, testInfo) => {
    const errors = await open(page);
    await attach(page);
    const csr = await context.newPage();
    const csrErrors = await open(csr, "csr");
    try {
        await attach(csr);
        const hydrated = await exercise(page, "catalog-a");
        const mounted = await exercise(csr, "catalog-a");
        expect(hydrated).toEqual(mounted);
        for (const scope of scopes.slice(1)) {
            expect(await publicState(page, scope)).toEqual(await publicState(csr, scope));
            await expect(control(page, scope, "count")).toHaveText("7");
        }
        await assertReferences(page);
        await assertReferences(csr);
        await testInfo.attach("input-state.json", { body: JSON.stringify(hydrated, null, 2), contentType: "application/json" });
        expect(errors).toEqual([]);
        expect(csrErrors).toEqual([]);
        expect(await page.evaluate(() => window.__hydrate_csp)).toEqual([]);
        expect(await csr.evaluate(() => window.__hydrate_csp)).toEqual([]);
    } finally {
        await csr.close();
    }
});

test("duplicate, malformed, detached and mismatched roots are refused without mutation", async ({ page }) => {
    const errors = await open(page);
    for (const [kind, reason] of [
        ["marker", "InvalidHydrationRoot"], ["content", "InvalidHydrationRoot"],
        ["overlay", "InvalidHydrationRoot"], ["extra", "InvalidHydrationRoot"],
        ["scope", "HydrationIdentityMismatch"], ["seed", "HydrationIdentityMismatch"],
        ["base", "HydrationIdentityMismatch"], ["detached", "InvalidContainer"],
    ]) {
        const refusal = await page.evaluate((kind) => window.__hydration.reject(kind), kind);
        expect(refusal.error).toContain(reason);
        expect(refusal.after).toBe(refusal.before);
        expect(refusal.sameNodes).toBe(true);
    }
    await attach(page);
    const before = await page.evaluate(() => window.__hydration.content("catalog-a"));
    const duplicate = await page.evaluate(() => window.__hydration.attempt("catalog-a"));
    expect(duplicate.error).toContain("OccupiedContainer");
    expect(await page.evaluate(() => window.__hydration.content("catalog-a"))).toEqual(before);
    const diagnostics = await page.evaluate(() => window.__hydration.diagnostics());
    for (const reason of ["InvalidHydrationRoot", "HydrationIdentityMismatch", "InvalidContainer", "OccupiedContainer"]) {
        expect(diagnostics.entries.some((entry) => entry.kind === reason && entry.scope)).toBe(true);
    }
    await control(page, "catalog-a", "button").click();
    await expect(control(page, "catalog-a", "count")).toHaveText("8");
    expect(errors).toEqual([]);
});

test("hydrate before runtime startup preserves all static nodes", async ({ page }) => {
    const errors = await open(page, "preboot");
    const refusal = await page.evaluate(() => window.__hydration.reject("runtime"));
    expect(refusal.error).toContain("RuntimeNotStarted");
    expect(refusal.after).toBe(refusal.before);
    expect(refusal.sameNodes).toBe(true);
    const diagnostics = await page.evaluate(() => window.__hydration.diagnostics());
    expect(diagnostics.entries.some((entry) => entry.kind === "RuntimeNotStarted" && entry.scope === "rejected")).toBe(true);
    expect(errors).toEqual([]);
});

test("structural comparison detects a deliberate wrong HTML element", async ({ page, context }, testInfo) => {
    await open(page);
    const csr = await context.newPage();
    await open(csr, "csr");
    try {
        await attach(csr);
        await control(page, "catalog-a", "button").evaluate((button) => {
            const wrong = document.createElement("div");
            for (const attribute of button.attributes) wrong.setAttribute(attribute.name, attribute.value);
            wrong.append(...button.childNodes);
            button.replaceWith(wrong);
        });
        const wrong = await page.evaluate(() => window.__hydration.content("catalog-a"));
        const correct = await csr.evaluate(() => window.__hydration.content("catalog-a"));
        expect(wrong).not.toEqual(correct);
        const outcome = await page.evaluate(() => window.__hydration.attempt("catalog-a"));
        if (outcome.error === null) {
            const attached = await page.evaluate(() => window.__hydration.content("catalog-a"));
            expect(attached).not.toEqual(correct);
            await expect(control(page, "catalog-a", "button")).toHaveJSProperty("tagName", "DIV");
        }
        await testInfo.attach("wrong-element.json", { body: JSON.stringify({ detected: true, outcome }, null, 2), contentType: "application/json" });
    } finally {
        await csr.close();
    }
});

test("asynchronous browser work starts after deterministic attachment", async ({ page, context }) => {
    const errors = await open(page);
    await expect(control(page, "catalog-a", "asynchronous")).toHaveText("idle");
    await attach(page);
    const csr = await context.newPage();
    const csrErrors = await open(csr, "csr");
    try {
        await attach(csr);
        expect(await page.evaluate(() => window.__hydration.asynchronous("catalog-a"))).toBe(true);
        expect(await csr.evaluate(() => window.__hydration.asynchronous("catalog-a"))).toBe(true);
        await expect(control(page, "catalog-a", "asynchronous")).toHaveText("complete");
        await expect(control(csr, "catalog-a", "asynchronous")).toHaveText("complete");
        expect(await publicState(page, "catalog-a")).toEqual(await publicState(csr, "catalog-a"));
        for (const scope of scopes.slice(1)) await expect(control(page, scope, "asynchronous")).toHaveText("idle");
        expect(errors).toEqual([]);
        expect(csrErrors).toEqual([]);
    } finally {
        await csr.close();
    }
});

test("disposing one hydrated scope releases its Owner and keeps peers interactive", async ({ page }) => {
    const errors = await open(page);
    await attach(page);
    expect(await windowListeners(page)).toBe(3);
    const before = await page.evaluate(() => window.__hydration.stats());
    await control(page, "catalog-a", "dialog-trigger").click();
    await expect(root(page, "catalog-a").getByRole("dialog")).toBeVisible();
    expect(await page.evaluate(() => window.__hydration.dispose("catalog-a"))).toBe(true);
    await settle(page);
    await expect(root(page, "catalog-a").locator("[data-rustify-content], [data-rustify-overlay]")).toHaveCount(0);
    expect(await page.evaluate(() => window.__hydration.state("catalog-a"))).toBeNull();
    expect(await windowListeners(page)).toBe(2);
    expect((await page.evaluate(() => window.__hydration.stats())).observers).toBeLessThan(before.observers);
    await control(page, "catalog-b", "button").click();
    await expect(control(page, "catalog-b", "count")).toHaveText("8");
    await expect(control(page, "catalog-c", "count")).toHaveText("7");
    expect(await page.evaluate(() => window.__hydration.dispose("catalog-a"))).toBe(false);
    expect(errors).toEqual([]);
});

test("independent CSR runtimes isolate identical geometry classes and their cleanup", async ({ page }, testInfo) => {
    await open(page);
    await attach(page);
    const instances = await page.evaluate(async () => {
        // This adversarial case intentionally reuses the seed in two isolated
        // runtimes to force matching geometry class names. Normal document
        // scopes above retain their distinct seeds and unique IDs.
        const first = await window.__hydration.bootPeer("first", "shared-seed");
        const second = await window.__hydration.bootPeer("second", "shared-seed");
        return [first, second];
    });
    expect(instances[0]).not.toBe(instances[1]);
    await settle(page);
    const first = page.locator('[data-harness-peer="first"] > div');
    const second = page.locator('[data-harness-peer="second"] > div');
    const progress = (scope: typeof first) => scope.getByTestId("shared-seed-progress");
    const ratio = async (element: typeof first) => element.evaluate((element) => element.firstElementChild!.getBoundingClientRect().width / element.getBoundingClientRect().width * 100);
    expect(await progress(first).locator("div").getAttribute("class")).toBe(await progress(second).locator("div").getAttribute("class"));
    const markers = [await first.getAttribute("data-rustify-geometry-scope"), await second.getAttribute("data-rustify-geometry-scope")];
    expect(markers[0]).toBeTruthy();
    expect(markers[1]).toBeTruthy();
    expect(markers[0]).not.toBe(markers[1]);
    const transition = await progress(first).locator("div").evaluate((element) => {
        const style = getComputedStyle(element);
        return { property: style.transitionProperty, duration: style.transitionDuration };
    });
    expect(transition).toEqual({ property: "width", duration: "0.15s" });
    const measurements: number[] = [];
    await first.getByTestId("shared-seed-slider").focus();
    await first.getByTestId("shared-seed-slider").press("ArrowRight");
    await expect(progress(first)).toHaveAttribute("aria-valuenow", "36");
    await expect(progress(second)).toHaveAttribute("aria-valuenow", "34");
    await expect.poll(() => widthRules(progress(first).locator("div"))).toEqual(["36%"]);
    await expect.poll(async () => {
        const width = await ratio(progress(first));
        measurements.push(width);
        return width;
    }).toBeCloseTo(36, 1);
    expect(await ratio(progress(second))).toBeCloseTo(34, 1);
    await first.getByTestId("shared-seed-color").fill("#ff0000ff");
    await first.getByTestId("shared-seed-color").press("Enter");
    await expect(first.getByTestId("shared-seed-color-swatch")).toHaveCSS("background-color", "rgb(255, 0, 0)");
    const initialColor = await nativeColor(page, "#1a2b3c80");
    await expect(second.getByTestId("shared-seed-color-swatch")).toHaveCSS("background-color", initialColor);
    await page.evaluate(() => window.__hydration.peers.first.trap());
    await expect.poll(() => page.evaluate(() => window.__hydration.peers.first.fatal())).not.toBeNull();
    const failed = await page.evaluate(() => window.__hydration.peers.first.stats());
    expect(failed).toMatchObject({ tasks: 0, timers: 0, observers: 0, aborted: true });
    expect((await page.evaluate(() => window.__hydration.peers.second.stats())).aborted).toBe(false);
    expect((await page.evaluate(() => window.__hydration.stats())).aborted).toBe(false);
    await second.getByTestId("shared-seed-slider").focus();
    await second.getByTestId("shared-seed-slider").press("ArrowRight");
    await expect(progress(second)).toHaveAttribute("aria-valuenow", "36");
    await expect.poll(() => widthRules(progress(second).locator("div"))).toEqual(["36%"]);
    await expect.poll(() => ratio(progress(second))).toBeCloseTo(36, 1);
    expect(await ratio(control(page, "catalog-a", "progress"))).toBeCloseTo(34, 1);
    expect(await page.evaluate(() => window.__hydration.peers.second.dispose())).toBe(true);
    await expect(second.locator("[data-rustify-content], [data-rustify-overlay]")).toHaveCount(0);
    await control(page, "catalog-a", "button").click();
    await expect(control(page, "catalog-a", "count")).toHaveText("8");
    expect(await page.evaluate(() => window.__hydrate_csp)).toEqual([]);
    await testInfo.attach("independent-geometry.json", { body: JSON.stringify({ instances, markers, transition, measurements, color: { authored: "#1a2b3c80", native: initialColor }, failed }, null, 2), contentType: "application/json" });
});

test("a trap aborts shared listeners, tasks and overlays and restores static scopes", async ({ page }, testInfo) => {
    await open(page);
    const before = await page.evaluate((scopes) => scopes.map((scope) => window.__hydration.content(scope)), scopes);
    await attach(page);
    await control(page, "catalog-b", "menu-trigger").click();
    await expect(root(page, "catalog-b").getByRole("menu")).toBeVisible();
    await control(page, "catalog-a", "toast-trigger").click();
    await control(page, "catalog-a", "dialog-trigger").click();
    await expect(root(page, "catalog-a").getByRole("dialog")).toBeVisible();
    await expect(root(page, "catalog-b").getByRole("menu")).toBeVisible();
    const active = await page.evaluate(() => window.__hydration.stats());
    expect(active.timers).toBeGreaterThan(0);
    expect(active.observers).toBeGreaterThan(0);
    await page.evaluate(() => window.__hydration.trap());
    await expect(page.getByTestId("status")).toHaveAttribute("data-status", "failed");
    expect(await windowListeners(page)).toBe(0);
    const stats = await page.evaluate(() => window.__hydration.stats());
    expect(stats).toMatchObject({ tasks: 0, timers: 0, observers: 0, aborted: true });
    await expect(page.getByRole("dialog")).toHaveCount(0);
    await expect(page.getByRole("menu")).toHaveCount(0);
    await expect(page.locator("[data-rustify-content][inert]")).toHaveCount(0);
    for (const [index, scope] of scopes.entries()) {
        expect(await page.evaluate((scope) => window.__hydration.content(scope), scope)).toEqual(before[index]);
        await expect(root(page, scope)).not.toHaveAttribute("data-rustify-mounted");
    }
    expect(await page.evaluate(() => [...document.adoptedStyleSheets].every((sheet) => sheet.cssRules.length === 0))).toBe(true);
    await testInfo.attach("fatal-runtime.json", { body: JSON.stringify({ active, stats, fatal: await page.evaluate(() => window.__hydration.fatal()) }, null, 2), contentType: "application/json" });
});
