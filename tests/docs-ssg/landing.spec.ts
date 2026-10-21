import AxeBuilder from "@axe-core/playwright";
import { BrowserContext, expect, Page, test } from "@playwright/test";
import { ChildProcess, execFileSync, spawn } from "node:child_process";
import { cpSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";
import { axeTags } from "./a11y";

// The landing fixture built by the CLI under test and served at the
// project's base, by `preview` or, under docs-meta, by a host that sends no
// headers, beside a local stand-in for the third parties its `ads` profile
// lists (`landing-third-party.mjs`). Tests tagged @policy are the security
// gate: what the profile lets the page load, and what it does not.

const STRICT =
    "default-src 'none'; script-src 'self' 'wasm-unsafe-eval'; style-src 'self'; img-src 'self' data:; font-src 'self'; connect-src 'self'; media-src 'self'; worker-src 'self'; object-src 'none'; base-uri 'none'; form-action 'none'; frame-ancestors 'none'";

type Entry = {
    kind: string;
    event?: string;
    page?: string;
    form?: Record<string, string>;
    query?: Record<string, string>;
};
type Violation = { blocked: string; directive: string };

let root: string;
let site: ChildProcess;
let thirdParty: ChildProcess;
let url: string;
let base: string;
let meta: boolean;
let listed: string;
let blocked: string;
let policies: Record<string, string>;

/** Starts `command` and resolves with the first match of `pattern` in its output. */
function started(command: string, args: string[], pattern: RegExp): Promise<[ChildProcess, RegExpExecArray]> {
    const child = spawn(command, args);
    return new Promise((resolve, reject) => {
        let output = "";
        const read = (chunk: Buffer) => {
            output += chunk;
            const match = pattern.exec(output);
            if (match) resolve([child, match]);
        };
        child.stdout!.on("data", read);
        child.stderr!.on("data", (chunk) => (output += chunk));
        child.on("exit", (code) => reject(new Error(`${command} exited ${code}: ${output}`)));
    });
}

async function stop(child: ChildProcess | undefined) {
    if (child && child.exitCode === null && child.signalCode === null) {
        const exited = new Promise((resolve) => child.once("exit", resolve));
        child.kill();
        await exited;
    }
}

test.beforeAll(async () => {
    const project = test.info().project.name;
    base = project === "docs-root" ? "/" : "/rustify-ui/";
    meta = project === "docs-meta";
    let ports: RegExpExecArray;
    [thirdParty, ports] = await started("node", ["tests/docs-ssg/landing-third-party.mjs", "0", "0", base], /third-party: (\d+) (\d+)/);
    listed = `http://127.0.0.1:${ports[1]}`;
    blocked = `http://127.0.0.1:${ports[2]}`;
    root = mkdtempSync(path.join(tmpdir(), "rustify-docs-landing-"));
    cpSync("tests/docs-ssg/fixtures/landing", root, { recursive: true });
    for (const file of ["rustify-docs.toml", "content/lp.md"]) {
        const target = path.join(root, file);
        const text = readFileSync(target, "utf8");
        expect(text).toContain("http://127.0.0.1:4211");
        writeFileSync(
            target,
            text
                .replaceAll("http://127.0.0.1:4211", listed)
                .replace('url = "https://example.com/docs/"\nbase = "/docs/"', `url = "https://docs.example.net${base}"\nbase = "${base}"`),
        );
    }
    expect(readFileSync(path.join(root, "rustify-docs.toml"), "utf8")).toContain(`base = "${base}"`);
    const bin = path.resolve(process.env.RUSTIFY_DOCS_BIN || "target/docs-cli/rustify-docs");
    execFileSync(bin, ["build", "--config", path.join(root, "rustify-docs.toml")]);
    const dist = path.join(root, "dist");
    policies = JSON.parse(readFileSync(path.join(dist, "rustify-docs-manifest.json"), "utf8")).page_policies;
    let address: RegExpExecArray;
    [site, address] = meta
        ? await started("node", ["tests/docs-ssg/static-host.mjs", dist, "0", base], /^static host: (\S+)/m)
        : await started(bin, ["preview", "--dir", dist, "--port", "0"], /^preview: (\S+)/m);
    url = address[1];
});

test.afterAll(async () => {
    await stop(site);
    await stop(thirdParty);
    if (root) rmSync(root, { recursive: true, force: true });
});

test.beforeEach(async ({ request }) => {
    expect((await request.post(`${listed}/reset`)).status()).toBe(204);
});

async function log(page: Page): Promise<Entry[]> {
    return (await page.request.get(`${listed}/log`)).json();
}

/** A context whose pages record every policy violation they report. */
async function recording(context: BrowserContext) {
    await context.addInitScript(() => {
        const found: { blocked: string; directive: string }[] = [];
        (window as any).__violations = found;
        document.addEventListener("securitypolicyviolation", (event) =>
            found.push({ blocked: event.blockedURI, directive: event.effectiveDirective }));
    });
}

const violations = (page: Page): Promise<Violation[]> => page.evaluate(() => (window as any).__violations);

/** The response's header policy, as the host sends it. */
function expectHeader(header: string | undefined, policy: string) {
    if (meta) expect(header).toBeUndefined();
    else expect(header).toBe(policy);
}

test("a landing page loads what its profile lists and nothing else @policy", async ({ browser }) => {
    const context = await browser.newContext();
    await recording(context);
    const page = await context.newPage();
    const response = await page.goto(`${url}lp/`);
    expectHeader(response!.headers()["content-security-policy"], policies["lp/index.html"]);
    expect(policies["lp/index.html"]).toContain(`script-src 'self' 'wasm-unsafe-eval' ${listed};`);
    expect(policies["lp/index.html"]).toContain(`form-action 'self' ${listed};`);
    // The profile's script reports the visit every way it was given.
    await expect.poll(async () =>
        (await log(page)).filter((entry) => entry.event === "page_view" && entry.page === `${base}lp/`).map((entry) => entry.kind).sort(),
    ).toEqual(["beacon", "fetch", "pixel"]);
    // What it then tries to load from elsewhere is blocked: its script and
    // its frame. (A blocked frame is reported by its origin alone, and each
    // of the header and the meta policy may report it.)
    await expect.poll(async () => {
        const found = await violations(page);
        const kinds = found
            .filter((v) => v.blocked.startsWith(blocked))
            .map((v) => (v.directive.startsWith("script-src") ? "script" : v.directive));
        return [...new Set(kinds)].sort();
    }).toEqual(["frame-src", "script"]);
    const found = await violations(page);
    expect(found.filter((v) => v.blocked.startsWith(listed))).toEqual([]);
    expect(found.every((v) => v.blocked.startsWith(blocked)), JSON.stringify(found)).toBe(true);
    expect((await log(page)).filter((entry) => entry.kind.startsWith("blocked-"))).toEqual([]);
    await context.close();
});

test("an ordinary page loads no third party and keeps the strict policy @policy", async ({ browser }) => {
    const context = await browser.newContext();
    await recording(context);
    const page = await context.newPage();
    const requests: string[] = [];
    page.on("request", (request) => requests.push(request.url()));
    for (const route of ["guide/", "", "offer/"]) {
        const response = await page.goto(`${url}${route}`);
        expectHeader(response!.headers()["content-security-policy"], STRICT);
        await page.waitForLoadState("networkidle");
    }
    const origin = new URL(url).origin;
    expect(requests.filter((request) => !request.startsWith(`${origin}/`) && !request.startsWith("data:"))).toEqual([]);
    expect(await log(page)).toEqual([]);
    expect(await violations(page)).toEqual([]);
    expect(policies["offer/index.html"]).toBeUndefined();
    await context.close();
});

test("carried parameters reach only marked elements, literally, without storage @policy", async ({ browser }) => {
    const context = await browser.newContext();
    const page = await context.newPage();
    const attributes = () => page.evaluate(() => ({
        fields: Object.fromEntries([...document.querySelectorAll<HTMLInputElement>("input[type=hidden][data-rd-carry]")].map((input) => [input.name, input.value])),
        links: [...document.querySelectorAll("a[data-rd-carry]")].map((link) => link.getAttribute("href")!),
        frame: document.querySelector("iframe[data-rd-carry]")!.getAttribute("src")!,
    }));

    // Without parameters nothing changes.
    await page.goto(`${url}lp/`);
    const plain = await attributes();
    expect(plain).toEqual({
        fields: { gclid: "", utm_source: "", utm_campaign: "" },
        links: ["#lead", `${base}guide/`, "https://example.org/signup"],
        frame: `${listed}/embed`,
    });

    // Only the listed parameters an address has, into every marked element.
    await page.goto(`${url}lp/?gclid=g1&utm_source=s&x=1`);
    const carried = await attributes();
    expect(carried.fields).toEqual({ gclid: "g1", utm_source: "s", utm_campaign: "" });
    // An anchor on the page keeps the page's own address, parameters included.
    expect(carried.links[0]).toBe("#lead");
    for (const target of [...carried.links.slice(1), carried.frame]) {
        const parsed = new URL(target, page.url());
        expect(parsed.searchParams.get("gclid"), target).toBe("g1");
        expect(parsed.searchParams.get("utm_source"), target).toBe("s");
        expect(parsed.searchParams.has("x"), target).toBe(false);
        expect(parsed.searchParams.has("utm_campaign"), target).toBe(false);
    }

    // A value is copied as text, never as markup.
    const hostile = "\"><script>window.__carried = 1</script><img src=x onerror=\"window.__carried = 2\">";
    await page.goto(`${url}lp/?gclid=${encodeURIComponent(hostile)}`);
    const literal = await attributes();
    expect(literal.fields.gclid).toBe(hostile);
    expect(new URL(literal.links[1], page.url()).searchParams.get("gclid")).toBe(hostile);
    expect(await page.evaluate(() => (window as any).__carried)).toBeUndefined();
    expect(await page.locator("img[src=x]").count()).toBe(0);

    expect(await page.evaluate(() => [localStorage.length, sessionStorage.length, document.cookie])).toEqual([0, 0, ""]);
    expect((await context.cookies()).length).toBe(0);
    await context.close();
});

test("the frame is embedded with the carried parameters", async ({ browser, request }) => {
    // The session history entries of a fresh page once its frame has loaded.
    const entries = async (address: string) => {
        const context = await browser.newContext();
        const page = await context.newPage();
        await page.goto(address);
        // A lazy frame loads as it nears the viewport.
        await page.locator("iframe[data-rd-carry]").scrollIntoViewIfNeeded();
        await expect(page.frameLocator("iframe[data-rd-carry]").getByRole("button", { name: "Sign up" })).toBeVisible();
        const length = await page.evaluate(() => window.history.length);
        await context.close();
        return length;
    };
    const plain = await entries(`${url}lp/`);
    const carried = await entries(`${url}lp/?gclid=g1&utm_source=s&x=1`);
    const embeds = async () => ((await (await request.get(`${listed}/log`)).json()) as Entry[]).filter((entry) => entry.kind === "embed").map((entry) => entry.query);
    await expect.poll(embeds).toContainEqual({ gclid: "g1", utm_source: "s" });
    // Carrying into the frame adds no entry: Back still leaves the page.
    expect(carried).toBe(plain);
});

for (const script of [true, false]) {
    test(`the lead form submits and returns to the thanks page with script ${script}`, async ({ browser }) => {
        const context = await browser.newContext({ javaScriptEnabled: script, reducedMotion: "reduce" });
        const page = await context.newPage();
        await page.goto(`${url}lp/?gclid=g1&utm_source=s`);
        await page.getByLabel("Work email").fill("lead@example.com");
        await page.getByRole("button", { name: "Request a demo" }).click();
        await page.waitForURL(`${url}thanks/`);
        await expect(page.getByText("We received your request and will be in touch.")).toBeVisible();
        await expect(page.getByRole("heading", { level: 1, name: "Thanks" })).toBeVisible();
        const leads = (await log(page)).filter((entry) => entry.kind === "lead");
        expect(leads.length).toBe(1);
        expect(leads[0].form!.email).toBe("lead@example.com");
        if (script) {
            expect(leads[0].form).toMatchObject({ gclid: "g1", utm_source: "s", utm_campaign: "" });
            await expect.poll(async () =>
                (await log(page)).some((entry) => entry.event === "conversion" && entry.page === `${base}thanks/`),
            ).toBe(true);
        } else {
            // Without script the form still submits; only attribution and
            // measurement are missing.
            expect(leads[0].form).toMatchObject({ gclid: "", utm_source: "", utm_campaign: "" });
            await page.waitForTimeout(1_000);
            const reported = (await log(page)).filter((entry) => ["fetch", "beacon", "pixel", "collect"].includes(entry.kind));
            expect(reported).toEqual([]);
        }
        await context.close();
    });
}

test("the landing layout has one way on, fits every width and passes axe", async ({ browser }) => {
    const context = await browser.newContext();
    const page = await context.newPage();
    for (const width of [1280, 768, 375, 320]) {
        await page.setViewportSize({ width, height: 800 });
        for (const route of ["lp/", "thanks/", "offer/"]) {
            await page.goto(`${url}${route}`);
            await expect(page.getByRole("heading", { level: 1 })).toHaveCount(1);
            expect(await page.locator("html").evaluate((node) => node.scrollWidth)).toBeLessThanOrEqual(width);
            for (const absent of ["[data-rd-search-open]", "dialog[data-rd-search]", ".rd-header-nav", ".rd-sidebar", "[data-rd-mode-switch]"]) {
                await expect(page.locator(absent)).toHaveCount(0);
            }
            await expect(page.getByRole("banner").getByRole("link")).toHaveCount(0);
            // The `ads` profile lists a footer link; `plain` lists none.
            await expect(page.getByRole("contentinfo")).toHaveCount(route === "offer/" ? 0 : 1);
            if (width === 1280 || width === 320) {
                const results = await new AxeBuilder({ page }).withTags(axeTags).exclude("iframe[title=blocked]").analyze();
                expect(results.violations.map((violation) => `${route} ${width}: ${violation.id}`)).toEqual([]);
            }
        }
    }
    await page.goto(`${url}lp/`);
    await page.getByRole("contentinfo").getByRole("link", { name: "Privacy" }).click();
    await page.waitForURL(`${url}privacy/`);
    await expect(page.getByRole("heading", { level: 1, name: "Privacy" })).toBeVisible();
    await context.close();
});
