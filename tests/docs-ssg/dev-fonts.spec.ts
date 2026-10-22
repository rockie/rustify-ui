import { expect, test } from "@playwright/test";
import { ChildProcess, execFileSync, spawn } from "node:child_process";
import { createHash } from "node:crypto";
import { copyFileSync, mkdirSync, mkdtempSync, readFileSync, rmSync, unlinkSync, writeFileSync } from "node:fs";
import { ClientRequest, get } from "node:http";
import { tmpdir } from "node:os";
import path from "node:path";

const repo = process.cwd();
let bin: string;
type State = { revision: number; good: number | null; building: boolean; ok: boolean; diagnostics: { rule: string }[] };

test.beforeAll(() => {
    bin = process.env.RUSTIFY_DOCS_BIN || path.join(repo, "target/docs-cli/rustify-docs");
    if (!process.env.RUSTIFY_DOCS_BIN) {
        execFileSync("mbx", ["build", "--locked", "--profile", "docs-cli", "-p", "rustify-docs-cli"], { cwd: repo, stdio: "inherit" });
    }
});

class Dev {
    readonly states: State[] = [];
    private events?: ClientRequest;
    private log = "";

    private constructor(readonly child: ChildProcess, readonly url: string) {}

    static async start(directory: string): Promise<Dev> {
        const child = spawn(bin, ["dev", "--port", "0"], { cwd: directory, detached: true, env: { ...process.env } });
        let log = "";
        child.stderr!.on("data", chunk => { log += chunk; });
        const url = await new Promise<string>((resolve, reject) => {
            const timer = setTimeout(() => reject(new Error(`dev startup timed out: ${log}`)), 60_000);
            let output = "";
            child.stdout!.on("data", chunk => {
                output += chunk;
                const match = /^dev: (\S+)\n/m.exec(output);
                if (match) { clearTimeout(timer); resolve(match[1]); }
            });
            child.once("error", error => { clearTimeout(timer); reject(error); });
            child.once("exit", code => { clearTimeout(timer); reject(new Error(`dev exited ${code}: ${log}`)); });
        }).catch(error => { if (child.pid) process.kill(-child.pid, "SIGTERM"); throw error; });
        const dev = new Dev(child, url);
        child.stderr!.on("data", chunk => { dev.log += chunk; });
        const events = new URL("/__rustify-docs/events", url);
        events.searchParams.set("path", new URL(url).pathname);
        events.searchParams.set("good", "0");
        dev.events = get(events, response => {
            response.setEncoding("utf8");
            response.on("error", error => { dev.log += String(error); });
            let pending = "";
            response.on("data", chunk => {
                pending += chunk;
                let end: number;
                while ((end = pending.indexOf("\n\n")) !== -1) {
                    const message = pending.slice(0, end);
                    pending = pending.slice(end + 2);
                    if (message.startsWith("event: state\ndata: ")) dev.states.push(JSON.parse(message.slice("event: state\ndata: ".length)));
                }
            });
        });
        dev.events.on("error", error => { dev.log += String(error); });
        return dev;
    }

    async settled(after: number, ok: boolean): Promise<State> {
        await expect.poll(() => {
            const state = this.states.at(-1);
            return !!state && state.revision > after && !state.building && state.ok === ok && (!ok || state.good !== null);
        }, { timeout: 120_000, message: this.log }).toBe(true);
        return this.states.at(-1)!;
    }

    async stop() {
        this.events?.destroy();
        if (this.child.exitCode !== null || this.child.signalCode !== null) return;
        const exited = new Promise(resolve => this.child.once("exit", resolve));
        process.kill(-this.child.pid!, "SIGTERM");
        await exited;
    }
}

test("local font edits, deletion and recovery replace or retain complete dev generations", async ({ page, request }, info) => {
    test.setTimeout(240_000);
    const directory = mkdtempSync(path.join(tmpdir(), "rustify-docs-dev-fonts-"));
    let dev: Dev | undefined;
    const errors: string[] = [];
    page.on("console", message => { if (message.type() === "error") errors.push(message.text()); });
    page.on("pageerror", error => errors.push(String(error)));
    page.on("response", response => {
        if (response.status() >= 400 && response.request().resourceType() !== "document") errors.push(`${response.status()} ${response.url()}`);
    });
    try {
        mkdirSync(path.join(directory, "content"));
        mkdirSync(path.join(directory, "fonts"));
        copyFileSync(path.join(repo, "tests/docs-ssg/fixtures/fonts/author-text.ttf"), path.join(directory, "fonts/author.ttf"));
        const document = JSON.parse(readFileSync(path.join(repo, "crates/rustify-docs-cli/templates/theme.example.json"), "utf8"));
        for (const mode of ["light", "dark"]) document.styles[mode]["font-sans"] = "Author Text, sans-serif";
        writeFileSync(path.join(directory, "theme.json"), JSON.stringify(document));
        writeFileSync(path.join(directory, "rustify-docs.toml"), "schema_version=1\n[site]\nname='Dev fonts'\nbase='/fonts/'\n[theme]\ndocument='theme.json'\n[[theme.fonts]]\nfamily='Author Text'\nsrc='fonts/author.ttf'\n");
        writeFileSync(path.join(directory, "content/index.md"), "---\ndescription: Font recovery\n---\n# Local font recovery\n\nAuthor-owned font text.\n");
        dev = await Dev.start(directory);
        const server = dev;
        const initial = await server.settled(-1, true);
        const publication = async () => {
            const response = await request.get(server.url);
            expect(response.status()).toBe(200);
            const html = await response.text();
            const styles = [...html.matchAll(/<link\b[^>]*href="([^"]+\.css)"/g)].map(match => new URL(match[1], server.url).href);
            const css = (await Promise.all(styles.map(async url => (await request.get(url)).text()))).join("\n");
            const font = css.match(/assets\/fonts\/[a-f0-9]{64}\.ttf/)?.[0];
            expect(font).toBeTruthy();
            return { styles, font: new URL(font!, server.url).href };
        };
        await page.goto(server.url);
        expect(await page.evaluate(async () => (await document.fonts.load('16px "Author Text"')).length)).toBe(1);
        const before = await publication();
        const original = readFileSync(path.join(directory, "fonts/author.ttf"));
        // Zero padding changes the input digest without changing any SFNT table.
        const changed = Buffer.concat([original, Buffer.alloc(4)]);
        writeFileSync(path.join(directory, "fonts/author.ttf"), changed);
        const edited = await server.settled(initial.revision, true);
        const after = await publication();
        expect(after.font).not.toBe(before.font);
        expect(after.font).toContain(createHash("sha256").update(changed).digest("hex"));
        const bytes = await request.get(after.font);
        expect(bytes.headers()["content-type"].split(";")[0]).toBe("font/ttf");
        expect((await bytes.body()).equals(changed)).toBe(true);
        await page.reload();
        expect(await page.evaluate(async () => (await document.fonts.load('16px "Author Text"')).length)).toBe(1);
        unlinkSync(path.join(directory, "fonts/author.ttf"));
        const failed = await server.settled(edited.revision, false);
        expect(failed.good).toBe(edited.good);
        expect(failed.diagnostics.some(issue => issue.rule === "theme-font")).toBe(true);
        expect(await publication()).toEqual(after);
        expect((await (await request.get(after.font)).body()).equals(changed)).toBe(true);
        writeFileSync(path.join(directory, "fonts/author.ttf"), original);
        await server.settled(failed.revision, true);
        expect(await publication()).toEqual(before);
        await page.reload();
        expect(await page.evaluate(async () => (await document.fonts.load('16px "Author Text"')).length)).toBe(1);
        expect(errors).toEqual([]);
        await info.attach("font-generations.json", { body: JSON.stringify(server.states, null, 2), contentType: "application/json" });
        await info.attach("font-recovered.png", { body: await page.screenshot(), contentType: "image/png" });
    } finally {
        await dev?.stop();
        rmSync(directory, { recursive: true, force: true });
    }
});
