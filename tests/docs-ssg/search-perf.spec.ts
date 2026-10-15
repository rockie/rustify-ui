import { expect, Page, test } from "@playwright/test";
import { mkdirSync, writeFileSync } from "node:fs";
import path from "node:path";
import { EVIDENCE } from "../tier";
import { binary } from "./a11y";

const queries = [
    { name: "chinese", query: "主题切换" },
    { name: "english", query: "ThemeScope" },
    { name: "prefix", query: "t" },
    { name: "empty", query: "DefinitelyNoSuchSearchTermP4" },
    { name: "fallback", query: "FallbackOnlyToken" },
    { name: "custom", query: "CustomOnlyToken" },
] as const;
const median = (values: number[]) => [...values].sort((a, b) => a - b)[2];
const base = () => String(test.info().project.use.baseURL);

async function instrument(page: Page) {
    await page.addInitScript(() => {
        const Native = Worker;
        window.Worker = class extends Native {
            postMessage(message: any) {
                const pending = (window as any).searchPerformance?.pending;
                if (pending && message.type === "query" && pending.query === message.query) {
                    pending.id = message.id;
                    pending.instance = message.instance;
                }
                super.postMessage(message);
            }
        } as typeof Worker;
    });
}

async function prepare(page: Page, route: string, lang: string) {
    const response = await page.goto(new URL(route, base()).href);
    expect(response?.status()).toBe(route.includes("__search_missing__") ? 404 : 200);
    await expect(page.locator("html")).toHaveAttribute("lang", lang);
    await expect(page.locator("[data-rd-search-open]")).toBeEnabled();
    await page.evaluate(() => {
        const dialog = document.querySelector<HTMLDialogElement>("[data-rd-search]")!;
        const input = dialog.querySelector<HTMLInputElement>("[data-rd-search-input]")!;
        const button = document.querySelector<HTMLButtonElement>("[data-rd-search-open]")!;
        const state: any = { records: [], pending: null, sequence: 0, opened: 0, focusMs: null };
        (window as any).searchPerformance = state;
        input.addEventListener("focus", () => { state.focusMs = performance.now() - state.opened; });
        input.addEventListener("input", () => {
            state.pending = { sequence: ++state.sequence, started: performance.now(), query: input.value,
                focusImmediate: document.activeElement === input, focusMs: state.focusMs };
        }, true);
        new MutationObserver(() => {
            const pending = state.pending;
            if (!pending || pending.id === undefined || pending.painting
                || dialog.dataset.rdSearchResultId !== String(pending.id)) return;
            pending.painting = true;
            requestAnimationFrame(() => requestAnimationFrame(() => {
                if (state.pending !== pending || dialog.dataset.rdSearchResultId !== String(pending.id)) return;
                const options = [...dialog.querySelectorAll<HTMLAnchorElement>("[role=option]")];
                state.records.push({ ...pending, elapsed: performance.now() - pending.started,
                    resultId: dialog.dataset.rdSearchResultId, state: dialog.dataset.state,
                    count: dialog.querySelector("[data-rd-search-count]")!.textContent,
                    options: options.map(option => ({ href: option.href, id: option.id, title: option.querySelector(".rd-search-title")!.textContent, fallback: option.querySelector(".rd-search-source")?.textContent })) });
            }));
        }).observe(dialog, { attributes: true, attributeFilter: ["data-rd-search-result-id"] });
        state.submit = (query: string, open: boolean) => {
            if (open) { state.opened = performance.now(); button.click(); }
            input.value = query;
            input.dispatchEvent(new InputEvent("input", { bubbles: true, inputType: "insertText", data: query }));
            return state.sequence;
        };
    });
}

async function measure(page: Page, query: string, open: boolean) {
    const sequence = await page.evaluate(({ query, open }) => (window as any).searchPerformance.submit(query, open), { query, open });
    await expect.poll(() => page.evaluate(sequence => (window as any).searchPerformance.records.find((record: any) => record.sequence === sequence), sequence), { timeout: 15_000 }).toBeTruthy();
    return page.evaluate(sequence => (window as any).searchPerformance.records.find((record: any) => record.sequence === sequence), sequence);
}

function visible(record: any, name: string, lang: string) {
    expect(record.focusImmediate).toBe(true);
    expect(record.resultId).toBe(String(record.id));
    if (name === "empty" || name === "chinese" && lang === "en") {
        expect(record.state).toBe("empty");
        expect(record.options).toHaveLength(0);
        expect(record.count).toMatch(/^0\D/);
    } else {
        expect(record.state).toBe("results");
        const first = record.options[0];
        expect(first.id).toBe(`rd-search-option-${record.instance}-${record.id}-0`);
        if (name === "english") expect(first.title).toContain("ThemeScope");
        if (name === "prefix") expect(first.title).toMatch(lang === "en" ? /^Theme(?:Scope| switching)/ : /^使用ThemeScope$/);
        if (name === "chinese") expect(first.title).toBe("主题切换");
        if (name === "custom") expect(first.title).toBe("Custom search home");
        if (name === "fallback") {
            expect(new URL(first.href).pathname).toContain(`${lang === "zh" ? "zh/" : ""}page-0501/`);
            if (lang === "zh") expect(first.fallback).toContain("en");
        }
    }
}

for (const lang of ["en", "zh"] as const) for (const entrance of ["largest", "404"] as const) {
    test(`${EVIDENCE} ${lang} ${entrance}: 1000-page cold and warm search paints`, async ({ browser, request }) => {
        test.setTimeout(360_000);
        const manifest = await (await request.get(new URL("rustify-docs-manifest.json", base()).href)).json();
        const largest = manifest.files.filter((file: any) => file.path.endsWith("index.html") && (lang === "zh" ? file.path.startsWith("zh/") : !file.path.startsWith("zh/")))
            .sort((a: any, b: any) => b.bytes - a.bytes || a.path.localeCompare(b.path))[0];
        const route = entrance === "404" ? `${lang === "zh" ? "zh/" : ""}__search_missing__/deep/` : largest.path.slice(0, -"index.html".length);
        const metrics = queries.map(query => ({ ...query, cold: [] as any[], warm: [] as any[] }));
        for (const metric of metrics) {
            for (let sample = 0; sample < 5; sample++) {
                const context = await browser.newContext({ serviceWorkers: "block" });
                try {
                    const page = await context.newPage();
                    const network = await context.newCDPSession(page);
                    await network.send("Network.enable");
                    await network.send("Network.setCacheDisabled", { cacheDisabled: true });
                    await instrument(page);
                    await prepare(page, route, lang);
                    const record = await measure(page, metric.query, true);
                    visible(record, metric.name, lang);
                    expect(record.focusMs).toBeLessThan(50);
                    metric.cold.push(record);
                } finally { await context.close(); }
            }
        }
        const warmContext = await browser.newContext({ serviceWorkers: "block" });
        const requests: string[] = [];
        try {
            const page = await warmContext.newPage();
            await instrument(page);
            await prepare(page, route, lang);
            await measure(page, "ThemeScope", true);
            warmContext.on("request", request => requests.push(request.url()));
            for (let sample = 0; sample < 5; sample++) for (const metric of metrics) {
                const record = await measure(page, metric.query, false);
                visible(record, metric.name, lang);
                metric.warm.push(record);
            }
        } finally { await warmContext.close(); }
        expect(requests).toEqual([]);
        const indexPath = manifest.files.find((file: any) => new RegExp(`^assets/search\\.${lang}\\.[a-f0-9]+\\.json$`).test(file.path)).path;
        const index = await (await request.get(new URL(indexPath, base()).href)).json();
        expect(index.pages).toHaveLength(1000);
        expect(index.pages.filter((page: any) => page.is_fallback)).toHaveLength(lang === "zh" ? 500 : 0);
        const report = { schema_version: 1, lang, entrance, route, browser: browser.version(), binary: binary(),
            build_id: manifest.build_id, index: { path: indexPath, pages: index.pages.length, fallback: index.pages.filter((page: any) => page.is_fallback).length },
            contract: "input event performance.now through matching request id DOM update and two requestAnimationFrame callbacks; fresh contexts and disabled HTTP cache; no throttling",
            warm_requests: requests, metrics };
        const body = JSON.stringify(report, null, 2);
        await test.info().attach("search-performance.json", { body, contentType: "application/json" });
        const directory = process.env.RUSTIFY_SEARCH_REPORT_DIR || test.info().outputPath("metrics");
        mkdirSync(directory, { recursive: true });
        writeFileSync(path.join(directory, `${lang}-${entrance}.json`), body + "\n");
        for (const metric of metrics) {
            expect(metric.cold).toHaveLength(5);
            expect(metric.warm).toHaveLength(5);
            expect(median(metric.cold.map(record => record.elapsed)), `${metric.name} cold median`).toBeLessThanOrEqual(1000);
            expect(median(metric.warm.map(record => record.elapsed)), `${metric.name} warm median`).toBeLessThanOrEqual(100);
        }
    });
}
