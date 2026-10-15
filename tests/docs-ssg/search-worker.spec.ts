import { expect, Page, test } from "@playwright/test";

type Section = { anchor: string | null; path: string[]; text: string };
type Entry = { route: string; title: string; source_lang: string; is_fallback: boolean; nav_order: number; sections: Section[] };
const entry = (route: string, title: string, text: string, nav_order = 0): Entry => ({ route, title, source_lang: "en", is_fallback: false, nav_order, sections: [{ anchor: null, path: [title], text }] });

async function start(page: Page, pages?: Entry[]) {
    const base = String(test.info().project.use.baseURL);
    const response = await page.request.get(new URL("rustify-docs-manifest.json", base).href);
    const manifest = await response.json();
    const worker = manifest.files.find((file: { path: string }) => /^assets\/search-worker\.[a-f0-9]+\.js$/.test(file.path));
    expect(worker, "the actual CLI publishes a fingerprinted production Worker").toBeTruthy();
    const indexPath = pages ? "assets/search.test.json" : manifest.files.find((file: { path: string }) => /^assets\/search\.en\.[a-f0-9]+\.json$/.test(file.path)).path;
    const indexUrl = new URL(indexPath, base).href;
    if (pages) await page.context().route(indexUrl, route => route.fulfill({ json: { schema_version: 1, ui_lang: "zh", pages } }));
    await page.goto(base);
    await page.evaluate(({ worker, indexUrl, base }) => {
        const state = { messages: [] as any[], worker: new Worker(new URL(worker, base), { type: "module" }) };
        (window as any).searchWorkerTest = state;
        state.worker.onmessage = event => state.messages.push(event.data);
        state.worker.postMessage({ type: "init", instance: 7, index_url: indexUrl });
    }, { worker: worker.path, indexUrl, base });
    await expect.poll(() => page.evaluate(() => (window as any).searchWorkerTest.messages)).toContainEqual({ type: "ready", instance: 7 });
}

async function query(page: Page, text: string, id = 1, offset = 0) {
    await page.evaluate(({ text, id, offset }) => {
        (window as any).searchWorkerTest.worker.postMessage({ type: "query", instance: 7, id, query: text, offset });
    }, { text, id, offset });
    await expect.poll(() => page.evaluate(id => (window as any).searchWorkerTest.messages.find((message: any) => message.type === "results" && message.id === id), id)).toBeTruthy();
    return page.evaluate(id => (window as any).searchWorkerTest.messages.find((message: any) => message.type === "results" && message.id === id), id);
}

test("matching uses whole Chinese words, English word starts and UTF-16 offsets", async ({ page }) => {
    const texts = ["使用ThemeScope", "ThemeScope主题", "Theme_Scope2", "foo_ThemeScope", "_ThemeScope", "2ThemeScope", "Theme2Scope", "😀ThemeScope", "主题切换", "主题切换器"];
    await start(page, texts.map((text, i) => entry(`/case-${i}/`, "Cases", text, i)));
    let id = 0;
    const routes = async (text: string) => (await query(page, text, ++id)).groups.map((group: any) => group.route).sort();
    expect(await routes("ThemeScope")).toEqual(["/case-0/", "/case-1/", "/case-7/"]);
    expect(await routes("theme")).toEqual(["/case-0/", "/case-1/", "/case-2/", "/case-6/", "/case-7/"]);
    expect(await routes("heme")).toEqual([]);
    expect(await routes("theme_s")).toEqual(["/case-2/"]);
    expect(await routes("Scope")).toEqual([]);
    expect(await routes("切换")).toContain("/case-8/");
    expect(await routes("换")).toEqual([]);
    const emoji = (await query(page, "ThemeScope", ++id)).groups.find((group: any) => group.route === "/case-7/");
    expect(emoji.snippet).toBe("😀ThemeScope");
    expect(emoji.highlights).toEqual([[2, 12]]);
});

test("matching ranks titles then headings ahead of repeated body matches", async ({ page }) => {
    const pages = [entry("/body/", "Other", "ThemeScope ".repeat(200)), entry("/title/", "使用ThemeScope", "Unrelated", 9), entry("/heading/", "Guide", "Unrelated", 2)];
    pages[2].sections.push({ anchor: "setup", path: ["Guide", "ThemeScope setup"], text: "Use it" });
    await start(page, pages);
    const result = await query(page, "theme");
    expect(result.groups.map((group: any) => group.route)).toEqual(["/title/", "/heading/", "/body/"]);
    expect(result.groups[0].snippet).toContain("ThemeScope");
    expect(result.groups[0].highlights).toEqual([[2, 7]]);
    expect(result.groups[1].anchor).toBe("setup");
});

test("matching requires all words in one section and groups pages with stable pagination", async ({ page }) => {
    const pages = Array.from({ length: 100 }, (_, i) => entry(`/page-${String(i).padStart(3, "0")}/`, "Guide", "ThemeScope supports 主题切换", i));
    pages[0].sections.push({ anchor: "extra", path: ["Guide", "Extra"], text: "ThemeScope 主题切换" });
    pages.push({ ...entry("/split/", "Guide", "ThemeScope"), sections: [{ anchor: null, path: ["Guide"], text: "ThemeScope" }, { anchor: "other", path: ["Guide", "Other"], text: "主题切换" }] });
    await start(page, pages);
    const first = await query(page, "theme 切换");
    expect(first.total).toBe(100);
    expect(first.groups).toHaveLength(20);
    expect(first.has_more).toBe(true);
    expect(first.groups[0].paths).toHaveLength(2);
    const next = await query(page, "theme 切换", 2, 20);
    expect(next.groups[0].route).toBe("/page-020/");
    const last = await query(page, "theme 切换", 3, 80);
    expect(last.groups).toHaveLength(20);
    expect(last.has_more).toBe(false);
    const empty = await query(page, "  ", 4);
    expect(empty.total).toBe(0);
    expect(empty.groups).toEqual([]);
});

test("matching retains request identities and serves warm queries without network", async ({ page }) => {
    await start(page, [entry("/first/", "First", "ThemeScope"), entry("/second/", "Second", "切换")]);
    const requests: string[] = [];
    page.context().on("request", request => requests.push(request.url()));
    await page.evaluate(() => {
        const worker = (window as any).searchWorkerTest.worker;
        worker.postMessage({ type: "query", instance: 7, id: 13, query: "theme", offset: 0 });
        worker.postMessage({ type: "query", instance: 7, id: 14, query: "切换", offset: 0 });
    });
    await expect.poll(() => page.evaluate(() => (window as any).searchWorkerTest.messages.filter((message: any) => message.type === "results").length)).toBe(2);
    const messages = await page.evaluate(() => (window as any).searchWorkerTest.messages.filter((message: any) => message.type === "results"));
    expect(messages.map((message: any) => [message.instance, message.id, message.groups[0].route])).toEqual([[7, 13, "/first/"], [7, 14, "/second/"]]);
    expect(requests).toEqual([]);
});

test("matching scans the real 100-page guide corpus with one registered page", async ({ page }) => {
    const manifest = await (await page.request.get(urlFor("rustify-docs-manifest.json"))).json();
    for (const lang of ["en", "zh"]) {
        const path = manifest.files.find((file: { path: string }) => new RegExp(`^assets/search\\.${lang}\\.[a-f0-9]+\\.json$`).test(file.path)).path;
        const data = await (await page.request.get(urlFor(path))).json();
        expect(data.pages).toHaveLength(100);
        if (lang === "zh") {
            expect(data.pages.filter((page: Entry) => page.is_fallback)).toHaveLength(50);
            expect(data.pages.filter((page: Entry) => !page.is_fallback)).toHaveLength(50);
        }
    }
    await start(page);
    expect((await query(page, "CustomOnlyToken", 1)).groups[0].route).toBe("/");
    expect((await query(page, "FallbackOnlyToken", 2)).total).toBe(1);
    const timings = await page.evaluate(async () => {
        const state = (window as any).searchWorkerTest;
        const values = [];
        for (let id = 100; id < 105; id++) {
            const start = performance.now();
            await new Promise<void>(resolve => {
                const receive = (event: MessageEvent) => {
                    if (event.data.id === id) { state.worker.removeEventListener("message", receive); resolve(); }
                };
                state.worker.addEventListener("message", receive);
                state.worker.postMessage({ type: "query", instance: 7, id, query: "theme", offset: 0 });
            });
            values.push(performance.now() - start);
        }
        return values;
    });
    await test.info().attach("100-page-worker-timings.json", { body: JSON.stringify(timings), contentType: "application/json" });
    expect(timings.sort((a, b) => a - b)[2]).toBeLessThan(100);
});

function urlFor(path: string) { return new URL(path, String(test.info().project.use.baseURL)).href; }

test("recovery refuses nonlocal indexes, failed HTTP and invalid schemas", async ({ page }) => {
    await start(page, [entry("/valid/", "Valid", "ThemeScope")]);
    const requests: string[] = [];
    page.context().on("request", request => requests.push(request.url()));
    const invalid = urlFor("assets/search.invalid.json");
    await page.context().route(invalid, route => route.fulfill({ status: 404, contentType: "text/html", body: "<h1>Not found</h1>" }));
    const schema = urlFor("assets/search.schema.json");
    await page.context().route(schema, route => route.fulfill({ json: { schema_version: 1, ui_lang: "en", pages: [{ ...entry("/../outside/", "Invalid", "Text") }] } }));
    for (const [instance, index_url, code] of [[8, "https://invalid.example/search.json", "load"], [9, new URL("/outside/search.json", baseFor()).href, "load"], [10, invalid, "load"], [11, schema, "schema"]] as const) {
        await page.evaluate(({ instance, index_url }) => (window as any).searchWorkerTest.worker.postMessage({ type: "init", instance, index_url }), { instance, index_url });
        await expect.poll(() => page.evaluate(() => (window as any).searchWorkerTest.messages)).toContainEqual({ type: "error", instance, code });
    }
    expect(requests).toEqual([invalid, schema]);
});

test("recovery reports unsupported word boundaries and invalid queries explicitly", async ({ page }) => {
    await start(page, [entry("/valid/", "Valid", "ThemeScope")]);
    await page.evaluate(() => (window as any).searchWorkerTest.worker.postMessage({ type: "query", instance: 7, id: 3, query: "theme", offset: -1 }));
    await expect.poll(() => page.evaluate(() => (window as any).searchWorkerTest.messages)).toContainEqual({ type: "error", instance: 7, id: 3, code: "query" });
    const manifest = await (await page.request.get(urlFor("rustify-docs-manifest.json"))).json();
    const path = manifest.files.find((file: { path: string }) => /^assets\/search-worker\./.test(file.path)).path;
    const worker = urlFor(path) + "?unsupported";
    await page.context().route(worker, async route => {
        const response = await route.fetch();
        await route.fulfill({ response, body: "Intl.Segmenter = undefined;\n" + await response.text() });
    });
    await page.evaluate(worker => {
        const state = (window as any).searchWorkerTest;
        state.worker.terminate();
        state.worker = new Worker(worker, { type: "module" });
        state.worker.onmessage = (event: MessageEvent) => state.messages.push(event.data);
        state.worker.postMessage({ type: "init", instance: 8, index_url: location.href });
    }, worker);
    await expect.poll(() => page.evaluate(() => (window as any).searchWorkerTest.messages)).toContainEqual({ type: "error", instance: 8, code: "unsupported" });
});

function baseFor() { return String(test.info().project.use.baseURL); }
