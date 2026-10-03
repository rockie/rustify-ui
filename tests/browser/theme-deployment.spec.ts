import { test, expect, allowRegionStart, REGION_START_MS } from "./support";
import type { Page, TestInfo } from "@playwright/test";
import { createHash } from "node:crypto";

test.use({ fresh: true });

const packagedFonts = [
    ["IBM Plex Sans", "IBMPlexSans-Text.ttf"],
    ["Noto Sans", "NotoSans-Regular.ttf"],
    ["Rustify WenKai", "LXGWWenKaiRegular.ttf"],
    ["Noto Serif", "NotoSerif-Regular.ttf"],
    ["Liberation Mono", "LiberationMono-Regular.ttf"],
    ["JetBrains Mono", "jetbrains_mono_variable.ttf"],
    ["Noto Color Emoji", "NotoColorEmoji.ttf"],
] as const;

function deployment(info: TestInfo) {
    const url = new URL(info.project.use.baseURL as string);
    return { url, base: url.pathname };
}

async function ready(page: Page) {
    await expect(page.getByTestId("status")).toHaveAttribute("data-status", "ready", { timeout: REGION_START_MS });
    await page.getByTestId("renderer-compare").click();
    await page.getByTestId("theme-gpu").scrollIntoViewIfNeeded();
    await expect.poll(() => page.evaluate(() => {
        const state = window.__theme_studio.snapshot();
        return state.gpu_state === "Ready" && state.drawn_revision === state.revision && state.samples.length > 50;
    }), { timeout: REGION_START_MS }).toBe(true);
}

test("V8 root/subpath product: current manifest, real CSS/fonts/wasm and deep-link resource ownership", async ({ page }, info) => {
    test.setTimeout(240_000); allowRegionStart(info);
    const { url, base } = deployment(info);
    const requests: string[] = [], failures: string[] = [], errors: string[] = [];
    const responses = new Map<string, { status: number; bytes: number; mime: string }>();
    page.on("request", request => requests.push(request.url()));
    page.on("requestfailed", request => failures.push(`${request.url()} ${request.failure()?.errorText}`));
    page.on("pageerror", error => errors.push(error.message));
    const consoleErrors: string[] = [];
    page.on("console", message => { if (message.type() === "error") consoleErrors.push(message.text()); });
    page.on("response", response => {
        // A deep-link reload may revalidate cached bytes. Keep the previously
        // measured representation when the server returns bodyless 304.
        if (response.status() === 304 && responses.has(response.url())) return;
        const headers = response.headers();
        responses.set(response.url(), { status: response.status(), bytes: Number(headers["content-length"]), mime: headers["content-type"] || "" });
        if (response.status() >= 400) failures.push(`${response.status()} ${response.url()}`);
    });
    await page.addInitScript(() => {
        (window as any).__themeDeploymentCsp = [];
        document.addEventListener("securitypolicyviolation", event => (window as any).__themeDeploymentCsp.push({ directive: event.violatedDirective, blocked: event.blockedURI }));
    });

    const manifestResponse = await page.request.get(new URL("build-manifest.json", url).href);
    expect(manifestResponse.ok()).toBe(true);
    const manifest = await manifestResponse.json();
    expect(manifest).toMatchObject({ example: "theme-studio", profile: "release", base });
    expect(manifest.build_id).toMatch(/^[0-9a-f]{16}$/);
    const wasmResponse = await page.request.get(new URL("theme-studio.wasm", url).href);
    expect(wasmResponse.ok()).toBe(true); expect(wasmResponse.headers()["content-type"]).toContain("application/wasm");
    const wasm = await wasmResponse.body();
    expect(wasm.length).toBe(manifest.files["theme-studio.wasm"]);
    expect(createHash("sha256").update(wasm).digest("hex").slice(0, 16)).toBe(manifest.build_id);
    expect(manifest.size_report.wasm).toBe(wasm.length);

    const routes = base === "/" ? ["./"] : ["./", "objects/42"];
    for (const route of routes) {
        if (route !== "./") allowRegionStart(info);
        const response = await page.goto(new URL(route, url).href);
        expect(response?.status()).toBe(200);
        const csp = response!.headers()["content-security-policy"];
        expect(csp).toContain("'wasm-unsafe-eval'");
        const scripts = csp.split(";").find((directive: string) => directive.trim().startsWith("script-src"));
        expect(scripts).toContain("'self'"); expect(scripts).not.toContain("'unsafe-inline'");
        await ready(page);
        expect(await page.evaluate(() => (window as any).makepad_resource_base)).toBe(base);
        expect((await page.evaluate(() => window.__theme_studio.diagnostics())).build).toBe(manifest.schema_hash);
        const styles = await page.evaluate(() => [...document.querySelectorAll<HTMLLinkElement>('link[rel="stylesheet"]')].map(link => ({ url: link.href, loaded: link.sheet !== null })));
        for (const file of ["runtime.css", "theme-fonts.css", "tailwind.css", "app.css"]) {
            expect(styles).toContainEqual({ url: new URL(file, url).href, loaded: true });
            expect(manifest.files[file], file).toBeGreaterThan(0);
            expect(responses.get(new URL(file, url).href)?.mime).toContain("text/css");
        }
        // This is a computed utility, so an HTTP 200 carrying index.html in
        // place of Tailwind cannot masquerade as a working stylesheet.
        expect(await page.getByTestId("studio-preview").getByTestId("radius-md").evaluate(node => getComputedStyle(node).paddingLeft)).toBe("16px");
        const faces = await page.evaluate(async (catalogue) => {
            const results = [];
            for (const [family] of catalogue) {
                const faces = await document.fonts.load(`16px "${family}"`, family === "Noto Color Emoji" ? "😀" : "Offline 中文");
                results.push({ family, faces: faces.map(face => ({ family: face.family, status: face.status })), checked: document.fonts.check(`16px "${family}"`) });
            }
            return results;
        }, packagedFonts.map(([family, file]) => [family, file]));
        for (const { family, faces: loaded, checked } of faces) {
            expect(loaded.map(face => ({ ...face, family: face.family.replace(/^["']|["']$/g, "") })), family).toEqual([{ family, status: "loaded" }]); expect(checked, family).toBe(true);
        }
        for (const [, file] of packagedFonts) {
            const path = `makepad_widgets/resources/${file}`, address = new URL(path, url).href;
            expect(requests, file).toContain(address);
            expect(responses.get(address), file).toMatchObject({ status: 200, bytes: manifest.files[path] });
            expect(responses.get(address)!.mime, file).toMatch(/font|octet-stream/);
        }
        expect(requests).toContain(new URL("app.js", url).href);
        expect(requests).toContain(new URL("theme-studio.wasm", url).href);
        expect(await page.evaluate(() => (window as any).__themeDeploymentCsp)).toEqual([]);
        expect((await page.evaluate(() => window.__theme_studio.snapshot())).region_count).toBe(1);
        await info.attach(route === "./" ? "deployment-home" : "deployment-deep-link", {
            body: JSON.stringify({ page: page.url(), manifest, fonts: faces, resources: [...responses] }, null, 2), contentType: "application/json",
        });
    }
    // Reject escaped roots and external services. Navigation routes themselves
    // remain under the configured base, including the SPA deep link above.
    expect(requests.filter(address => {
        const resource = new URL(address); return resource.protocol !== "data:" && resource.protocol !== "blob:" && (resource.origin !== url.origin || !resource.pathname.startsWith(base));
    })).toEqual([]);
    expect(failures).toEqual([]); expect(errors).toEqual([]); expect(consoleErrors).toEqual([]);
    for (const [address, response] of responses) {
        const resource = new URL(address), path = resource.pathname.slice(base.length);
        if (manifest.files[path] !== undefined && /\.(css|js|wasm|ttf)$/.test(path)) {
            expect(response.bytes, path).toBe(manifest.files[path]);
        }
    }
});

test("V8 a missing deployed wasm gives a usable reload action and recovers under the same base", async ({ page }, info) => {
    const { url, base } = deployment(info), pattern = "**/theme-studio.wasm";
    const pageErrors: string[] = []; page.on("pageerror", error => pageErrors.push(error.message));
    await page.route(pattern, route => route.fulfill({ status: 404, contentType: "text/plain", body: "V8 missing wasm" }));
    try {
        await page.goto(url.href);
        await expect(page.getByTestId("status")).toHaveAttribute("data-status", "failed");
        await expect(page.getByTestId("fatal-text")).toContainText(/WebAssembly|compile|MIME|fetch/i);
        await expect(page.getByTestId("fatal-reload")).toBeVisible();
        expect(await page.evaluate(() => typeof window.__theme_studio)).toBe("undefined");
        await page.unroute(pattern); allowRegionStart(info); await page.getByTestId("fatal-reload").click();
        await ready(page);
        expect(await page.evaluate(() => (window as any).makepad_resource_base)).toBe(base);
        expect(pageErrors).toEqual([]);
        await info.attach("deployment-recovered", { body: JSON.stringify({ page: page.url(), state: await page.evaluate(() => window.__theme_studio.snapshot()) }, null, 2), contentType: "application/json" });
    } finally { await page.unroute(pattern); }
});
