import { expect, Page, Route, test } from "@playwright/test";
import { ChildProcess, execFileSync, spawn } from "node:child_process";
import { existsSync, mkdirSync, mkdtempSync, readFileSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { ClientRequest, get } from "node:http";
import { tmpdir } from "node:os";
import path from "node:path";

const repo = process.cwd();
const buildTimeout = 600_000;
const base = "/rustify-ui/";
type Timing = { profile: string; target: string | null; success: boolean; flags: string[] };
type State = {
    revision: number; sequence: number; building: boolean; good: number | null; ok: boolean;
    compile: { host: Timing; wasm: Timing; completed_at_ms: number } | null;
    diagnostics: { severity: string; rule: string; file: string | null; message: string }[];
};
type Audit = {
    requests: string[]; failures: string[]; expectedStreamClosures: string[];
    responses: { url: string; status: number; type: string }[];
    console: { url: string; text: string }[]; pageErrors: string[]; csp: string[];
};
type Original = { document: HTMLElement; roots: Element[]; nodes: Node[][] };

declare global {
    interface Window {
        __dev_i18n_original?: Original;
        __dev_i18n_csp(value: string): void;
    }
}

class Site {
    readonly dir = mkdtempSync(path.join(tmpdir(), "rustify-docs-dev-i18n-"));
    readonly root = path.join(this.dir, "site");
    readonly lock: Buffer;

    constructor(readonly bin: string) {
        const prepared = execFileSync("python3", [
            path.join(repo, "tests/docs-ssg/i18n-fixture.py"),
            "--out", this.root, "--cli", bin, "--base", base,
        ], { cwd: repo, encoding: "utf8" }).trim();
        expect(prepared).toBe(realpathSync(this.root));
        this.lock = readFileSync(path.join(this.root, "Cargo.lock"));
    }

    read(file: string) { return readFileSync(path.join(this.root, file), "utf8"); }
    write(file: string, text: string) {
        const target = path.join(this.root, file);
        mkdirSync(path.dirname(target), { recursive: true });
        writeFileSync(target, text);
    }
    remove() { rmSync(this.dir, { recursive: true, force: true }); }
}

class Dev {
    log = "";
    readonly states: State[] = [];
    readonly streamErrors: string[] = [];
    private events?: ClientRequest;
    private stopping = false;

    private constructor(readonly child: ChildProcess, readonly url: string) {}

    static async start(site: Site): Promise<Dev> {
        const child = spawn(site.bin, ["dev", "--port", "0"], {
            cwd: site.root, detached: true, env: { ...process.env, RUSTIFY_DOCS_CARGO: "mbx" },
        });
        let log = "";
        child.stderr!.on("data", chunk => { log += chunk; });
        const url = await new Promise<string>((resolve, reject) => {
            let out = "";
            child.stdout!.on("data", chunk => {
                out += chunk;
                const match = /^dev: (\S+)\n/m.exec(out);
                if (match) resolve(match[1]);
            });
            child.on("error", reject);
            child.on("exit", code => reject(new Error(`dev exited ${code}: ${out}${log}`)));
        });
        const server = new Dev(child, url);
        server.log = log;
        child.stderr!.on("data", chunk => { server.log += chunk; });
        const events = new URL("/__rustify-docs/events", url);
        events.searchParams.set("path", new URL("fr/locale/", url).pathname);
        events.searchParams.set("good", "0");
        server.events = get(events, response => {
            if (response.statusCode !== 200) server.streamErrors.push(`SSE response ${response.statusCode}`);
            response.setEncoding("utf8");
            let pending = "";
            response.on("data", chunk => {
                pending += chunk;
                let end: number;
                while ((end = pending.indexOf("\n\n")) !== -1) {
                    const message = pending.slice(0, end);
                    pending = pending.slice(end + 2);
                    if (message.startsWith("event: state\ndata: ")) {
                        server.states.push(JSON.parse(message.slice("event: state\ndata: ".length)));
                    }
                }
            });
            response.on("error", error => { if (!server.stopping) server.streamErrors.push(String(error)); });
        });
        server.events.on("error", error => { if (!server.stopping) server.streamErrors.push(String(error)); });
        return server;
    }

    async settled(after = -1, ok = true): Promise<State> {
        await expect.poll(() => {
            const state = this.states.at(-1);
            return !!state && state.revision > after && !state.building && state.ok === ok;
        }, { timeout: buildTimeout }).toBe(true);
        return this.states.at(-1)!;
    }

    async stop() {
        this.stopping = true;
        this.events?.destroy();
        if (this.child.exitCode !== null || this.child.signalCode !== null) return;
        const exited = new Promise(resolve => this.child.once("exit", resolve));
        process.kill(-this.child.pid!, "SIGTERM");
        await exited;
    }
}

const islands = (page: Page) => page.locator("[data-rustify-island]");
const devScript = (page: Page) => page.locator('script[src*="/__rustify-docs/dev.js"]');
const copies = (page: Page) => page.locator(".rd-demo-source [data-rd-copy]");

async function captureOriginals(page: Page) {
    await page.evaluate(() => {
        const roots = [...document.querySelectorAll("fieldset[data-rd-island-controls]")].map(field => field.firstElementChild!);
        window.__dev_i18n_original = {
            document: document.documentElement,
            roots,
            nodes: roots.map(root => {
                const nodes: Node[] = [root];
                const walker = document.createTreeWalker(root, NodeFilter.SHOW_ALL);
                while (walker.nextNode()) nodes.push(walker.currentNode);
                return nodes;
            }),
        };
    });
}

async function retained(page: Page) {
    expect(await page.evaluate(() => {
        const original = window.__dev_i18n_original!;
        const fields = [...document.querySelectorAll("fieldset[data-rd-island-controls]")];
        return document.documentElement === original.document && original.roots.every((root, index) =>
            fields[index]?.firstElementChild === root && original.nodes[index].every(node => node.isConnected && root.contains(node)));
    })).toBe(true);
}

async function ready(page: Page) {
    await expect(islands(page)).toHaveCount(2);
    await expect(page.locator('[data-rustify-island][data-rd-island-state="ready"]')).toHaveCount(2, { timeout: buildTimeout });
    const metadata = [];
    for (const island of await islands(page).all()) {
        const item = JSON.parse((await island.getAttribute("data-rustify-island-metadata"))!);
        metadata.push(item);
        expect(item.environment.lang).toBe("fr");
        expect(item.environment.base).toBe(base);
        const controls = island.locator("fieldset[data-rd-island-controls]");
        await expect(controls).toHaveJSProperty("disabled", false);
        await expect(controls).toHaveJSProperty("inert", false);
        await expect(controls.locator(":scope > [data-rustify-scope]")).toHaveAttribute("data-rustify-mounted", item.seed);
    }
    expect(new Set(metadata.map(item => item.key)).size).toBe(2);
    expect(new Set(metadata.map(item => item.seed)).size).toBe(2);
}

async function openAttached(page: Page, url: string, status: number, beforeAttach: () => Promise<void>) {
    let release!: () => void;
    const released = new Promise<void>(resolve => { release = resolve; });
    let requested = false;
    const pending: Promise<void>[] = [];
    const pattern = "**/assets/islands/**/app_bg.wasm";
    const handler = (route: Route) => {
        requested = true;
        const handled = released.then(() => route.continue());
        pending.push(handled);
        return handled;
    };
    await page.route(pattern, handler);
    try {
        const response = await page.goto(url, { waitUntil: "commit" });
        expect(response?.status()).toBe(status);
        expect(response?.headers()["content-security-policy"]).toContain("'wasm-unsafe-eval'");
        await expect.poll(() => requested).toBe(true);
        await expect(page.locator('[data-rustify-island][data-rd-island-state="loading"]')).toHaveCount(2);
        for (const controls of await page.locator("fieldset[data-rd-island-controls]").all()) {
            await expect(controls).toHaveJSProperty("disabled", false);
            await expect(controls).toHaveJSProperty("inert", true);
            await expect(controls).toHaveAttribute("aria-busy", "true");
            const gated = await controls.locator("button").first().evaluate(button => {
                let delivered = false;
                const receive = () => { delivered = true; };
                button.addEventListener("click", receive);
                const accepted = button.dispatchEvent(new MouseEvent("click", { bubbles: true, cancelable: true }));
                button.removeEventListener("click", receive);
                return { accepted, delivered };
            });
            expect(gated).toEqual({ accepted: false, delivered: false });
        }
        await beforeAttach();
        await captureOriginals(page);
        release();
        await ready(page);
        await retained(page);
    } finally {
        release();
        try { await Promise.all(pending); } finally { await page.unroute(pattern, handler); }
    }
}

async function exercise(page: Page, initial = 7) {
    await expect(page.getByTestId("counter-value")).toHaveText(String(initial));
    await page.getByTestId("counter-increment").click();
    await expect(page.getByTestId("counter-value")).toHaveText(String(initial + 1));
    const trigger = page.getByTestId("theme-probe-dialog-trigger");
    await trigger.click();
    const dialog = page.getByRole("dialog", { name: "Theme probe dialog", exact: true });
    await expect(dialog).toBeVisible();
    await dialog.getByRole("button", { name: "close", exact: true }).click();
    await expect(dialog).toHaveCount(0);
    await expect(trigger).toBeFocused();
}

test("French dev pages update messages and translations, retain the last good locale through errors, then recover", async ({ page }, info) => {
    test.setTimeout(900_000);
    expect(process.env.RUSTIFY_DOCS_BIN, "Run with the installed, paired CLI in RUSTIFY_DOCS_BIN").toBeTruthy();
    const site = new Site(path.resolve(repo, process.env.RUSTIFY_DOCS_BIN!));
    let server: Dev | undefined;
    const audit: Audit = { requests: [], failures: [], expectedStreamClosures: [], responses: [], console: [], pageErrors: [], csp: [] };
    await page.exposeBinding("__dev_i18n_csp", (_, value: string) => audit.csp.push(value));
    await page.addInitScript(() => {
        document.addEventListener("securitypolicyviolation", event => window.__dev_i18n_csp(`${event.violatedDirective} ${event.blockedURI}`));
    });
    page.on("request", request => audit.requests.push(request.url()));
    page.on("requestfailed", request => {
        const message = `${request.url()} ${request.failure()?.errorText}`;
        if (new URL(request.url()).pathname === "/__rustify-docs/events" && request.failure()?.errorText === "net::ERR_ABORTED") audit.expectedStreamClosures.push(message);
        else audit.failures.push(message);
    });
    page.on("response", response => audit.responses.push({ url: response.url(), status: response.status(), type: response.request().resourceType() }));
    page.on("console", message => { if (message.type() === "error") audit.console.push({ url: message.location().url, text: message.text() }); });
    page.on("pageerror", error => audit.pageErrors.push(String(error)));
    try {
        server = await Dev.start(site);
        let state = await server.settled();
        const compiled = state.compile;
        expect(compiled).not.toBeNull();
        expect(compiled!.host).toMatchObject({ profile: "docs-host", target: null, success: true });
        expect(compiled!.wasm).toMatchObject({ profile: "docs-wasm", target: "wasm32-unknown-unknown", success: true });
        expect(compiled!.host.flags).toContain("--locked");
        expect(compiled!.wasm.flags).toContain("--locked");
        const localeUrl = new URL("fr/locale/", server.url).href;
        const missingUrl = new URL("fr/missing/a/b/", server.url).href;
        await openAttached(page, localeUrl, 200, async () => {
            await expect(page.locator("html")).toHaveAttribute("lang", "fr");
            await expect(page.locator(".rd-prose")).toHaveAttribute("lang", "en");
            await expect(page.locator(".rd-untranslated")).toHaveAttribute("lang", "fr");
            await expect(page.locator("[data-locale-page]")).toHaveAttribute("data-context-route", "/locale/");
            await expect(page.locator("[data-locale-page]")).toHaveAttribute("data-context-lang", "fr");
            await expect(page.locator("[data-locale-page]")).toContainText("English locale page");
            await expect(page.getByRole("tab", { name: "Fonction", exact: true }).first()).toHaveAttribute("lang", "fr");
            expect(await page.getByRole("tab", { name: "Full file", exact: true }).first().evaluate(button => button.closest("[lang]")?.getAttribute("lang"))).toBe("en");
            await expect(copies(page).first()).toHaveText("Copier");
            await expect(copies(page).first()).toHaveAttribute("lang", "fr");
        });
        await exercise(page);
        await info.attach("dev-i18n-fallback.png", { body: await page.screenshot(), contentType: "image/png" });

        const messages = site.read("locales/fr.toml")
            .replace('copy = "Copier"', 'copy = "Copier le code"')
            + 'error_one = "{n} erreur"\nerror_other = "{n} erreurs"\nhide = "Masquer les problèmes"\n';
        site.write("locales/fr.toml", messages);
        state = await server.settled(state.revision);
        expect(state.compile).toEqual(compiled);
        await expect(copies(page).first()).toHaveText("Copier le code");
        await expect(copies(page).first()).toHaveAttribute("lang", "fr");
        await expect.poll(async () => JSON.parse((await devScript(page).getAttribute("data-label-langs"))!).hide).toBe("fr");
        expect(JSON.parse((await devScript(page).getAttribute("data-label-langs"))!).last_good).toBe("en");
        await ready(page);
        await exercise(page);

        site.write("content/fr/locale.md", '---\ntitle: Page locale française\ndescription: Une vraie traduction avec les mêmes exemples.\nlang: fr\n---\n# Page locale française\n\nUn texte français réel.\n\n```rustify-demo\nname: CounterDemo\n```\n\n```rustify-demo\nname: ThemeProbeDemo\n```\n');
        state = await server.settled(state.revision);
        expect(state.compile).toEqual(compiled);
        await expect(page.getByRole("heading", { name: "Page locale française", level: 1 })).toBeVisible();
        await expect(page.locator(".rd-prose")).toHaveAttribute("lang", "fr");
        await expect(page.locator(".rd-untranslated")).toHaveCount(0);
        await expect(page.locator('.rd-language-switch a[hreflang="fr"] .rd-language-fallback')).toHaveCount(0);
        await ready(page);
        await exercise(page);
        await info.attach("dev-i18n-translation.png", { body: await page.screenshot(), contentType: "image/png" });

        rmSync(path.join(site.root, "content/fr/locale.md"));
        state = await server.settled(state.revision);
        expect(state.compile).toEqual(compiled);
        await expect(page.locator(".rd-untranslated")).toBeVisible();
        await expect(page.locator(".rd-prose")).toHaveAttribute("lang", "en");
        await expect(page.locator("[data-locale-page]")).toContainText("English locale page");
        await ready(page);
        await exercise(page);
        await captureOriginals(page);
        const lastGood = await page.request.get(localeUrl);
        expect(lastGood.status()).toBe(200);
        const lastGoodBody = await lastGood.body();
        const config = site.read("rustify-docs.toml");
        expect(config).toContain('prefix = "fr"');
        site.write("locales/fr.toml", "copy = [\n");
        site.write("rustify-docs.toml", config.replace('prefix = "fr"', 'prefix = "french"'));
        const failed = await server.settled(state.revision, false);
        expect(failed.good).toBe(state.good);
        expect(failed.compile).toEqual(compiled);
        expect(failed.diagnostics.some(issue => issue.rule === "i18n-messages" && issue.file === "locales/fr.toml")).toBe(true);
        const panel = page.locator(".rd-dev-panel");
        await expect(panel).toBeVisible();
        await expect(panel.locator(".rd-dev-count-error")).toHaveAttribute("lang", "fr");
        await expect(panel.locator(".rd-dev-count-error")).toContainText("erreur");
        await expect(page.getByRole("button", { name: "Masquer les problèmes", exact: true })).toHaveAttribute("lang", "fr");
        await expect(page.locator(".rd-dev-list")).toHaveAttribute("lang", "fr");
        await expect(panel.locator(".rd-dev-state > span[lang=en]")).toHaveText("showing the last good version");
        await expect(panel.locator(".rd-dev-revision")).toHaveAttribute("lang", "en");
        await retained(page);
        const retainedResponse = await page.request.get(localeUrl);
        expect(retainedResponse.status()).toBe(200);
        expect(await retainedResponse.body()).toEqual(lastGoodBody);
        await exercise(page, 8);
        await info.attach("dev-i18n-last-good.png", { body: await page.screenshot(), contentType: "image/png" });

        await openAttached(page, missingUrl, 404, async () => {
            await expect(page.locator("html")).toHaveAttribute("lang", "fr");
            await expect(page).toHaveTitle(/^Page introuvable \|/);
            for (const slot of ["before_content", "not_found"]) {
                const wrapper = page.locator(`[data-slot="${slot}"]`);
                await expect(wrapper).toHaveCount(1);
                await expect(wrapper).toHaveAttribute("data-context-route", "/404/");
                await expect(wrapper).toHaveAttribute("data-context-lang", "fr");
            }
            await expect(page.locator('.rd-available-languages a[hreflang="fr"]')).toHaveAttribute("href", `${base}fr/`);
        });
        await exercise(page);
        await expect(page.locator(".rd-dev-list")).toHaveAttribute("lang", "fr");

        site.write("rustify-docs.toml", config);
        site.write("locales/fr.toml", messages.replace('copy = "Copier le code"', 'copy = "Copier après correction"'));
        state = await server.settled(failed.revision);
        expect(state.compile).toEqual(compiled);
        await expect(copies(page).first()).toHaveText("Copier après correction");
        await expect(page.locator(".rd-dev")).toHaveAttribute("data-revision", String(state.revision));
        await expect(page.locator('.rd-dev-item[data-severity="error"]')).toHaveCount(0);
        await expect(page.locator("html")).toHaveAttribute("lang", "fr");
        await ready(page);
        await exercise(page);
        await info.attach("dev-i18n-recovered.png", { body: await page.screenshot(), contentType: "image/png" });

        expect(audit.csp).toEqual([]);
        expect(audit.failures).toEqual([]);
        expect(audit.pageErrors).toEqual([]);
        expect(audit.responses.filter(response => response.status >= 400 && !(response.type === "document" && response.status === 404 && response.url === missingUrl))).toEqual([]);
        expect(audit.console.filter(message => !(message.url === missingUrl && message.text === "Failed to load resource: the server responded with a status of 404 (Not Found)"))).toEqual([]);
        expect(audit.requests.filter(url => new URL(url).origin !== new URL(server!.url).origin || /makepad/i.test(url))).toEqual([]);
        expect(server.streamErrors).toEqual([]);
        expect(readFileSync(path.join(site.root, "Cargo.lock"))).toEqual(site.lock);
        expect(existsSync(path.join(site.root, "dist"))).toBe(false);
    } finally {
        try {
            await info.attach("dev-i18n-browser-audit.json", { body: JSON.stringify(audit, null, 2), contentType: "application/json" });
            await info.attach("dev-i18n-states.json", { body: JSON.stringify(server?.states || [], null, 2), contentType: "application/json" });
            await info.attach("dev-i18n-cli.log", { body: server?.log || "", contentType: "text/plain" });
        } finally {
            try { await server?.stop(); } finally { site.remove(); }
        }
    }
});
