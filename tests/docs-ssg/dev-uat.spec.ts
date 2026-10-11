import { expect, Page, test } from "@playwright/test";
import { ChildProcess, execFileSync, spawn } from "node:child_process";
import { copyFileSync, mkdirSync, mkdtempSync, readFileSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";
import { performance } from "node:perf_hooks";

/// The author's path through `rustify-docs` in one session, as the plan's
/// P1 UAT chains it (§9.4): with only Rust and the CLI on PATH, `new` then
/// `dev` per the generated README, and the home page and two pages open in
/// a browser within 60 s (UAT-1); then, in the same `dev`, a paragraph, an
/// image and an included source file each show within a second at the
/// reader's scroll position, and a broken link is reported, hidden, saved
/// again and fixed (UAT-2). Times run from a write (or `new`) returning to
/// the page's own record of painting, on the shared wall clock.

const repo = process.cwd();
const bin = path.resolve(repo, process.env.RUSTIFY_DOCS_BIN || "target/docs-cli/rustify-docs");
const now = () => performance.timeOrigin + performance.now();
const LIMIT_MS = 1000;

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

/// An author's machine: an empty home, and on PATH only Rust's compiler,
/// the CLI and the system's own tools.
function author(dir: string): NodeJS.ProcessEnv {
    const home = path.join(dir, "home");
    const tools = path.join(dir, "tools");
    mkdirSync(home);
    mkdirSync(tools);
    const sysroot = execFileSync("rustc", ["--print", "sysroot"], { encoding: "utf8" }).trim();
    symlinkSync(path.join(sysroot, "bin", "rustc"), path.join(tools, "rustc"));
    symlinkSync(bin, path.join(tools, "rustify-docs"));
    return { PATH: `${tools}:/usr/bin:/bin`, HOME: home, TMPDIR: tmpdir() };
}

/// Records, in every document, when it painted after its content loaded.
/// Plain script text: a transpiled function's nested callbacks can carry
/// helpers that do not exist in the page.
async function witness(page: Page): Promise<void> {
    await page.addInitScript({
        content:
            "window.__painted = null; document.addEventListener('DOMContentLoaded', function () { requestAnimationFrame(function () { requestAnimationFrame(function () { window.__painted = performance.timeOrigin + performance.now(); }); }); });",
    });
}

/// Waits, across reloads, until `check` holds and the document has painted;
/// returns when it painted.
async function shown(page: Page, check: () => boolean): Promise<number> {
    const start = Date.now();
    for (;;) {
        try {
            // No reload follows the one that showed the change, so both
            // reads are of the same document.
            if (await page.evaluate(check)) {
                const painted = await page.evaluate(() => (window as unknown as { __painted: number | null }).__painted);
                if (painted) return painted;
            }
        } catch {
            // Between documents.
        }
        if (Date.now() - start > 15_000) throw new Error(`never shown: ${check}`);
        await new Promise((resolve) => setTimeout(resolve, 5));
    }
}

let dir: string;
let child: ChildProcess | null = null;

test.beforeEach(() => {
    dir = mkdtempSync(path.join(tmpdir(), "rustify-docs-uat-"));
});

test.afterEach(() => {
    child?.kill();
    child = null;
    rmSync(dir, { recursive: true, force: true });
});

test("UAT-1 and UAT-2: new, dev, three pages within a minute; then saves, a mistake and its fix in one session", async ({ browser }) => {
    test.setTimeout(120_000);
    const env = author(dir);
    const site = path.join(dir, "my-docs");

    // UAT-1: the generated README's commands, nothing compiled.
    const started = now();
    execFileSync("rustify-docs", ["new", site], { env, cwd: dir });
    child = spawn("rustify-docs", ["dev"], { env, cwd: site });
    const url = await new Promise<string>((resolve, reject) => {
        let out = "";
        child!.stdout!.on("data", (chunk) => {
            out += chunk;
            const match = /^dev: (\S+)\n/m.exec(out);
            if (match) resolve(match[1]);
        });
        child!.on("exit", (code) => reject(new Error(`dev exited ${code}`)));
    });
    const context = await browser.newContext({ viewport: { width: 1280, height: 480 } });
    const page = await context.newPage();
    await witness(page);
    await page.goto(url);
    const home = await shown(page, () => document.querySelector("h1") !== null);
    const toHome = home - started;
    test.info().annotations.push({ type: "new-to-home-ms", description: String(Math.round(toHome)) });
    expect(toHome, "new to the home page in the browser").toBeLessThan(60_000);
    await page.locator(".rd-action-primary").click();
    await expect(page.getByRole("heading", { level: 1, name: "Getting started" })).toBeVisible();
    await page.locator(".rd-sidebar-wide").getByRole("link", { name: "Writing pages" }).click();
    await expect(page.getByRole("heading", { level: 1 })).toBeVisible();
    expect(new URL(page.url()).pathname).toBe("/guide/writing-pages/");

    // UAT-2: on Getting started, scrolled down, in the same session.
    await page.goto(`${url}guide/getting-started/`);
    await shown(page, () => true);
    await page.evaluate(() => window.scrollTo(0, 300));
    await expect.poll(() => page.evaluate(() => window.scrollY)).toBe(300);
    const file = path.join(site, "content/guide/getting-started.md");
    const timings: Record<string, number> = {};
    const save = async (name: string, write: () => void, check: () => boolean) => {
        write();
        const saved = now();
        const painted = await shown(page, check);
        timings[name] = Math.round(painted - saved);
        expect(timings[name], `${name} shown within a second`).toBeLessThanOrEqual(LIMIT_MS);
        await expect.poll(() => page.evaluate(() => window.scrollY), { message: `${name} kept the scroll` }).toBe(300);
    };
    const edit = (from: string, to: string) => {
        const text = readFileSync(file, "utf8");
        expect(text).toContain(from);
        writeFileSync(file, text.replace(from, to));
    };

    await save(
        "paragraph",
        () => edit("Saving a file updates the page you are reading.", "Saving a file updates this very page."),
        () => document.body.innerText.includes("updates this very page"),
    );
    await save(
        "image",
        () => {
            copyFileSync(path.join(site, "static/logo.svg"), path.join(site, "content/guide/mark.svg"));
            edit("Next: [write a page]", "![The mark](mark.svg)\n\nNext: [write a page]");
        },
        () => {
            const img = document.querySelector<HTMLImageElement>('img[alt="The mark"]');
            return !!img && img.complete && img.naturalWidth > 0;
        },
    );
    mkdirSync(path.join(site, "src"));
    const source = path.join(site, "src/example.rs");
    await save(
        "include",
        () => {
            writeFileSync(source, "// rustify-region:main\nfn main() {\n    println!(\"first\");\n}\n// rustify-endregion:main\n");
            edit("Next: [write a page]", '<!-- rustify:include path="src/example.rs" region="main" lang="rust" -->\n\nNext: [write a page]');
        },
        () => document.body.innerText.includes('println!("first")'),
    );
    await save(
        "included source",
        () => writeFileSync(source, readFileSync(source, "utf8").replace("first", "second")),
        () => document.body.innerText.includes('println!("second")'),
    );
    test.info().annotations.push({ type: "save-ms", description: JSON.stringify(timings) });

    // A broken link: reported where it is, hidden, saved again, fixed.
    edit("Next: [write a page](writing-pages.md)", "Next: [write a page](writing-page.md)");
    const panel = page.getByRole("dialog", { name: /^1 error · 1 warning · showing the last good version$/ });
    await expect(panel).toBeVisible();
    await expect(panel).toContainText("content/guide/getting-started.md");
    await expect(panel).toContainText("broken-link");
    await panel.getByRole("button", { name: "Hide problems" }).click();
    const marker = page.getByRole("button", { name: /^1 error · 1 warning · showing the last good version$/ });
    await expect(marker).toBeVisible();
    const revision = Number(await page.locator(".rd-dev").getAttribute("data-revision"));
    writeFileSync(file, `${readFileSync(file, "utf8")}\n`);
    await expect.poll(async () => Number(await page.locator(".rd-dev").getAttribute("data-revision"))).toBeGreaterThan(revision);
    await expect(marker).toBeVisible();
    await expect(panel).toBeHidden();
    edit("(writing-page.md)", "(writing-pages.md)");
    await expect(marker).toBeHidden();
    await expect(panel).toBeHidden();
    // What is left is the new site's one warning (no `site.url` yet), which
    // waits behind its marker.
    await expect(page.getByRole("button", { name: "1 warning" })).toBeVisible();
    await context.close();
});
