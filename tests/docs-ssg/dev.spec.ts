import AxeBuilder from "@axe-core/playwright";
import { expect, Page, test } from "@playwright/test";
import { ChildProcess, execFileSync, spawn } from "node:child_process";
import { cpSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { createServer } from "node:net";
import { tmpdir } from "node:os";
import path from "node:path";

/// `rustify-docs dev` with a browser open on it: a save reloads the pages it
/// changed in place and no others; a failed build leaves the last good page
/// readable under a corner panel that lists the problems, hides to a marker
/// and clears itself on the fix; nothing built yet is a page that says so; a
/// restarted dev brings open pages up to date; a panic in a build is one
/// more problem. Each test runs its own `dev` on a copy of the content
/// fixture, with the binary `--profile docs-cli` builds (as installed).

const repo = process.cwd();
const fixture = path.join(repo, "tests/docs-ssg/fixtures/content");
const bin = path.resolve(repo, process.env.RUSTIFY_DOCS_BIN || "target/docs-cli/rustify-docs");

test.beforeAll(() => {
    if (!process.env.RUSTIFY_DOCS_BIN) {
        test.setTimeout(600_000);
        execFileSync("mbx", ["build", "--locked", "--profile", "docs-cli", "-p", "rustify-docs-cli"], {
            cwd: repo,
            stdio: "inherit",
            env: { ...process.env, CARGO_TARGET_DIR: path.join(repo, "target") },
        });
    }
});

/// A copy of the content fixture, removed when the test ends.
class Site {
    readonly dir = mkdtempSync(path.join(tmpdir(), "rustify-docs-dev-"));
    readonly root = path.join(this.dir, "site");

    constructor() {
        cpSync(fixture, this.root, { recursive: true });
    }

    read(file: string): string {
        return readFileSync(path.join(this.root, file), "utf8");
    }

    write(file: string, text: string): void {
        writeFileSync(path.join(this.root, file), text);
    }

    /// Replaces `from` with `to`, asserting it was there.
    edit(file: string, from: string, to: string): void {
        const text = this.read(file);
        expect(text, `${from} in ${file}`).toContain(from);
        this.write(file, text.replace(from, to));
    }

    remove(): void {
        rmSync(this.dir, { recursive: true, force: true });
    }
}

/// A running `dev`.
class Dev {
    log = "";

    private constructor(
        readonly child: ChildProcess,
        readonly url: string,
    ) {}

    static start(site: Site, args: string[] = ["--port", "0"], env: Record<string, string> = {}): Promise<Dev> {
        const child = spawn(bin, ["dev", ...args], { cwd: site.root, env: { ...process.env, ...env } });
        let log = "";
        child.stderr!.on("data", (chunk) => (log += chunk));
        return new Promise((resolve, reject) => {
            let out = "";
            child.stdout!.on("data", (chunk) => {
                out += chunk;
                const match = /^dev: (\S+)\n/m.exec(out);
                if (match) {
                    const dev = new Dev(child, match[1]);
                    child.stderr!.on("data", (more) => (dev.log += more));
                    dev.log = log;
                    resolve(dev);
                }
            });
            child.on("exit", (code) => reject(new Error(`dev exited ${code}: ${out}${log}`)));
        });
    }

    async stop(): Promise<void> {
        if (this.child.exitCode !== null || this.child.signalCode !== null) return;
        const exited = new Promise((resolve) => this.child.on("exit", resolve));
        this.child.kill();
        await exited;
    }
}

async function freePort(): Promise<number> {
    return new Promise((resolve) => {
        const server = createServer();
        server.listen(0, "127.0.0.1", () => {
            const port = (server.address() as { port: number }).port;
            server.close(() => resolve(port));
        });
    });
}

let site: Site;
const running: Dev[] = [];

async function dev(args?: string[], env?: Record<string, string>): Promise<Dev> {
    const started = await Dev.start(site, args, env);
    running.push(started);
    return started;
}

test.beforeEach(() => {
    site = new Site();
});

test.afterEach(async () => {
    while (running.length) await running.pop()!.stop();
    site.remove();
});

/// The build result the page's panel last took in.
async function seen(page: Page): Promise<number> {
    return Number(await page.locator(".rd-dev").getAttribute("data-revision"));
}

async function nextResult(page: Page, after: number): Promise<void> {
    await expect.poll(() => seen(page)).toBeGreaterThan(after);
}

/// Marks the document, so a reload shows as the mark's absence.
async function mark(page: Page): Promise<void> {
    await page.evaluate(() => ((window as unknown as { __kept: boolean }).__kept = true));
}

async function kept(page: Page): Promise<boolean> {
    return page.evaluate(() => (window as unknown as { __kept?: boolean }).__kept === true);
}

const BROKEN = "[Broken](missing.md)";

test("a new site is readable at once: the home page and its two pages", async ({ page }) => {
    const target = path.join(site.dir, "new-site");
    execFileSync(bin, ["new", target]);
    const child = spawn(bin, ["dev", "--port", "0"], { cwd: target });
    const url = await new Promise<string>((resolve, reject) => {
        let out = "";
        child.stdout!.on("data", (chunk) => {
            out += chunk;
            const match = /^dev: (\S+)\n/m.exec(out);
            if (match) resolve(match[1]);
        });
        child.on("exit", (code) => reject(new Error(`dev exited ${code}`)));
    });
    try {
        await page.goto(url);
        await expect(page.getByRole("heading", { level: 1 })).toBeVisible();
        await page.locator(".rd-action-primary").click();
        await expect(page.getByRole("heading", { level: 1 })).toBeVisible();
        const first = page.url();
        const sidebar = page.locator(".rd-sidebar-wide a:not([aria-current])").first();
        await sidebar.click();
        await expect(page).not.toHaveURL(first);
        await expect(page.getByRole("heading", { level: 1 })).toBeVisible();
        // The template has no `site.url` yet: a warning, which waits behind
        // a marker rather than opening the panel.
        const marker = page.getByRole("button", { name: "1 warning" });
        await expect(marker).toBeVisible();
        await expect(page.getByRole("dialog")).toBeHidden();
        await marker.click();
        await expect(page.getByRole("dialog")).toContainText("rustify-docs.toml");
    } finally {
        child.kill();
    }
});

test("a save reloads the page it changed in place and leaves the others", async ({ context }) => {
    const server = await dev();
    const setup = await context.newPage();
    await setup.goto(`${server.url}guide/setup/`);
    await setup.getByRole("tab", { name: "Config" }).first().click();
    await setup.evaluate(() => window.scrollTo(0, 600));
    await expect.poll(() => setup.evaluate(() => window.scrollY)).toBe(600);
    await mark(setup);
    const api = await context.newPage();
    await api.goto(`${server.url}reference/api/`);
    await expect(api.locator(".rd-dev")).toHaveAttribute("data-revision", /\d+/);
    await mark(api);
    const before = await seen(api);

    site.edit("content/guide/setup.md", "Once installed, the same choice applies here:", "Once installed, the same choice still applies here:");
    await expect(setup.getByText("the same choice still applies here")).toBeVisible();
    expect(await kept(setup), "the changed page reloaded").toBe(false);
    await expect.poll(() => setup.evaluate(() => window.scrollY)).toBe(600);
    await expect(setup.getByRole("tab", { name: "Config" }).first()).toHaveAttribute("aria-selected", "true");
    await nextResult(api, before);
    expect(await kept(api), "an unchanged page stayed").toBe(true);

    // The configuration changes every page.
    site.edit("rustify-docs.toml", "Fixture Docs", "Fixture Handbook");
    await expect(api.locator(".rd-brand")).toContainText("Fixture Handbook");
    expect(await kept(api)).toBe(false);
});

test("a failed build keeps the page readable under a panel that hides to a marker and clears on the fix", async ({ page }) => {
    const server = await dev();
    await page.goto(`${server.url}guide/setup/`);
    await mark(page);
    const before = await seen(page);
    site.edit("content/guide/setup.md", "Run `rustify-docs --version`", `${BROKEN} Run \`rustify-docs --version\``);

    const panel = page.getByRole("dialog", { name: "1 error · showing the last good version" });
    await expect(panel).toBeVisible();
    await expect(panel).toContainText("content/guide/setup.md");
    await expect(panel.locator(".rd-dev-location")).toHaveText(/^\d+:\d+$/);
    await expect(panel).toContainText("broken-link");
    expect(await kept(page), "the last good page stays").toBe(true);
    await expect(page.locator("article").getByText("Broken", { exact: true })).toHaveCount(0);
    expect(await page.evaluate(() => document.activeElement === document.body), "focus stays where it was").toBe(true);
    await expect(page.getByRole("status").filter({ hasText: "1 error" })).toHaveCount(1);

    await panel.getByRole("button", { name: "Hide problems" }).click();
    await expect(panel).toBeHidden();
    const marker = page.getByRole("button", { name: "1 error · showing the last good version" });
    await expect(marker).toBeVisible();
    await expect(marker).toBeFocused();
    await expect(marker).toHaveAttribute("aria-expanded", "false");

    // The same mistake saved again: a new result, the marker stays.
    const failed = await seen(page);
    site.write("content/guide/setup.md", `${site.read("content/guide/setup.md")}\n`);
    await nextResult(page, failed);
    await expect(marker).toBeVisible();
    await expect(panel).toBeHidden();

    await page.keyboard.press("Enter");
    await expect(panel).toBeVisible();
    await expect(panel.getByRole("button", { name: "Hide problems" })).toBeFocused();
    await page.keyboard.press("Escape");
    await expect(marker).toBeFocused();

    site.edit("content/guide/setup.md", `${BROKEN} `, "");
    await expect(marker).toBeHidden();
    await expect(panel).toBeHidden();
    // Focus was on the marker: it continues from the page's content.
    await expect(page.locator("#content")).toBeFocused();
    await expect(page.getByRole("status").filter({ hasText: "Problems fixed" })).toHaveCount(1);
    expect(await kept(page), "the fixed page was the last good one").toBe(true);
    expect(before).toBeGreaterThan(0);
});

test("while nothing has built, every page says so, and becomes the site once it builds", async ({ page }) => {
    site.edit("content/guide/setup.md", "Run `rustify-docs --version`", `${BROKEN} Run \`rustify-docs --version\``);
    const server = await dev();
    const response = await page.goto(`${server.url}guide/setup/`);
    expect(response?.status()).toBe(500);
    await expect(page.getByRole("heading", { level: 1 })).toHaveText("The site does not build yet");
    // The problems are the page: listed in its column, with nothing to hide them.
    const list = page.getByRole("region", { name: "1 error · nothing has built yet" });
    await expect(list).toBeVisible();
    await expect(page.locator("main .rd-dev")).toBeVisible();
    await expect(list.getByRole("button")).toHaveCount(0);
    await expect(list.locator(".rd-dev-code")).toHaveText("content/guide/missing.md");
    await list.locator(".rd-dev-list").focus();
    await page.keyboard.press("Escape");
    await expect(list).toBeVisible();
    site.edit("content/guide/setup.md", `${BROKEN} `, "");
    await expect(page.getByRole("heading", { level: 1 })).toHaveText("Setup");
    await expect(page.locator(".rd-dev")).toBeHidden();
});

test("a restarted dev brings an open page up to date", async ({ page }) => {
    const port = String(await freePort());
    const first = await dev(["--port", port]);
    await page.goto(`${first.url}guide/setup/`);
    await expect(page.locator(".rd-dev")).toHaveAttribute("data-revision", /\d+/);
    await first.stop();
    site.edit("content/guide/setup.md", "Once installed, the same choice applies here:", "After a restart, the same choice applies here:");
    await dev(["--port", port]);
    await expect(page.getByText("After a restart")).toBeVisible({ timeout: 15_000 });
});

test("a panic in a build is one more problem, and the next build is served", async ({ page }) => {
    const server = await dev(["--port", "0"], { RUSTIFY_DOCS_FAULT: "panic:PANIC-HERE" });
    await page.goto(`${server.url}guide/setup/`);
    await mark(page);
    site.edit("content/guide/setup.md", "Once installed,", "PANIC-HERE Once installed,");
    const panel = page.getByRole("dialog", { name: "1 error · showing the last good version" });
    await expect(panel).toContainText("internal");
    await expect(panel).toContainText("stopped inside rustify-docs");
    await expect(panel.locator(".rd-dev-suggestion")).toContainText("save again");
    expect(await kept(page)).toBe(true);
    site.edit("content/guide/setup.md", "PANIC-HERE Once installed,", "Recovered. Once installed,");
    await expect(page.getByText("Recovered. Once installed")).toBeVisible();
    await expect(page.locator(".rd-dev")).toBeHidden();
    expect(server.log).toContain("stopped inside rustify-docs");
});

test("the panel speaks the page's language", async ({ page }) => {
    const server = await dev();
    await page.goto(`${server.url}reference/api/`);
    site.edit("content/reference/api.md", "返回[指南](../guide/index.md)。", `返回[指南](../guide/index.md)。${BROKEN}`);
    const panel = page.getByRole("dialog", { name: "1 个错误 · 显示上一个可用版本" });
    await expect(panel).toBeVisible();
    await expect(page.locator(".rd-dev")).toHaveAttribute("lang", "zh-CN");
    await expect(panel.getByRole("button", { name: "收起问题" })).toBeVisible();
});

test("on a phone the panel is a sheet along the bottom and nothing scrolls sideways", async ({ browser }) => {
    const server = await dev();
    for (const width of [375, 320]) {
        const context = await browser.newContext({ viewport: { width, height: 640 }, reducedMotion: "reduce" });
        const page = await context.newPage();
        await page.goto(`${server.url}guide/setup/`);
        if (width === 375) {
            site.edit("content/guide/setup.md", "Run `rustify-docs --version`", `${BROKEN} Run \`rustify-docs --version\``);
        }
        const panel = page.getByRole("dialog");
        await expect(panel).toBeVisible();
        const box = (await panel.boundingBox())!;
        expect(box.x).toBe(0);
        expect(box.width).toBe(width);
        expect(box.y + box.height).toBeCloseTo(640, 0);
        expect(box.height).toBeLessThanOrEqual(320);
        expect(await page.evaluate(() => getComputedStyle(document.querySelector(".rd-dev-panel")!).animationName)).toBe("none");
        expect(await page.evaluate(() => document.documentElement.scrollWidth - window.innerWidth)).toBeLessThanOrEqual(0);
        await context.close();
    }
});

test("axe finds no serious or critical problem with the panel open or hidden, in both modes", async ({ browser }) => {
    const server = await dev();
    site.edit("content/guide/setup.md", "Run `rustify-docs --version`", `${BROKEN} Run \`rustify-docs --version\``);
    const found: string[] = [];
    for (const scheme of ["light", "dark"] as const) {
        const context = await browser.newContext({ colorScheme: scheme });
        const page = await context.newPage();
        await page.goto(`${server.url}guide/setup/`);
        const panel = page.getByRole("dialog");
        await expect(panel).toBeVisible();
        for (const state of ["open", "hidden"]) {
            if (state === "hidden") await panel.getByRole("button").click();
            // Colours are measured once the panel's entrance has finished.
            await page.waitForFunction(() => document.getAnimations().every((animation) => animation.playState === "finished"));
            const { violations } = await new AxeBuilder({ page })
                .include(".rd-dev")
                .withTags(["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"])
                .analyze();
            for (const violation of violations) {
                for (const node of violation.nodes) {
                    const detail = (node.any[0]?.message || node.failureSummary || "").replace(/\s+/g, " ");
                    found.push(`${scheme} ${state} ${violation.impact} ${violation.id}: ${node.target.join(" ")}: ${detail}`);
                }
            }
        }
        await context.close();
    }
    await test.info().attach("axe-violations.txt", { body: found.join("\n") || "none", contentType: "text/plain" });
    expect(found.filter((line) => / (serious|critical) /.test(line))).toEqual([]);
});
