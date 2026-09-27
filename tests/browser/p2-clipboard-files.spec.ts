import { expect, Page } from "@playwright/test";

import { test, waitForReady } from "./support";

/// M6 V9: files in, files out, and the clipboard when it says no.
///
/// The three answers an import has - read it, refused it, nobody chose
/// anything - are checked separately, because only the first may change the
/// application. A refusal that quietly imported half a file would pass a test
/// that only looked at the message.

const snapshot = (page: Page) => page.evaluate(() => window.__property_workbench.snapshot());
const transfer = async (page: Page) => (await snapshot(page)).transfer;

/// Hands the picker a file, as though somebody had chosen it.
async function choose(page: Page, name: string, body: Buffer | string, mime = "text/plain") {
    await page.getByTestId("import-file").setInputFiles({
        name,
        mimeType: mime,
        buffer: typeof body === "string" ? Buffer.from(body, "utf8") : body,
    });
}

/// Clicks an export and returns exactly what the browser saved.
async function download(page: Page, testId: string): Promise<Buffer> {
    const started = page.waitForEvent("download");
    await page.getByTestId(testId).click();
    const saved = await started;
    const stream = await saved.createReadStream();
    const chunks: Buffer[] = [];
    for await (const chunk of stream) {
        chunks.push(chunk as Buffer);
    }
    return Buffer.concat(chunks);
}

test.describe("M6 V9: exporting, and reading back exactly what was written", () => {
    test("a strict page can still start a download", async ({ page }) => {
        // `default-src 'none'` governs what the page may load, not what it may
        // hand the user to save. If it did, the export would be the feature
        // that the security policy silently removed, so it is checked rather
        // than assumed.
        const saved = await download(page, "export-text");
        expect(saved.byteLength).toBeGreaterThan(0);
        const state = await transfer(page);
        expect(state.exports).toBe(1);
        // What the browser saved is the length the application meant to write.
        expect(saved.byteLength).toBe(state.bytes);
    });

    test("text out, in, and out again is the same bytes", async ({ page }) => {
        const first = await download(page, "export-text");
        await choose(page, "objects.txt", first);
        await expect.poll(async () => (await transfer(page)).imports).toBeGreaterThan(0);
        expect((await transfer(page)).status).toContain("imported 1000 objects");
        const second = await download(page, "export-text");
        expect(second.equals(first)).toBe(true);
    });

    test("binary out, in, and out again is the same bytes", async ({ page }) => {
        const before = (await transfer(page)).imports;
        const first = await download(page, "export-binary");
        // Seventeen bytes an object: an id, a group, a flag and a size.
        expect(first.byteLength).toBe(1000 * 17);
        await choose(page, "objects.bin", first, "application/octet-stream");
        await expect.poll(async () => (await transfer(page)).imports).toBe(before + 1);
        const second = await download(page, "export-binary");
        expect(second.equals(first)).toBe(true);
    });

    test("what was dropped on the region survives a round trip", async ({ page }) => {
        // Put an object in a group, write everything out, take it out of the
        // group, read the file back: the group is where the file said.
        await page.getByTestId("object-11").click();
        await expect.poll(async () => (await snapshot(page)).selected).toBe(11);
        await page.evaluate(() => window.__property_workbench.snapshot());

        const before = await download(page, "export-text");
        const line = before.toString("utf8").split("\n")[10];
        expect(line.startsWith("11\t")).toBe(true);
        const moved = before
            .toString("utf8")
            .replace(line, line.split("\t").map((f, i) => (i === 2 ? "4" : f)).join("\t"));
        await choose(page, "objects.txt", moved);
        await expect.poll(async () => (await snapshot(page)).drag.selected_group).toBe(4);
    });
});

test.describe("M6 V9: the three answers an import has", () => {
    test("a file within the limits is read", async ({ page }) => {
        await choose(page, "objects.txt", "1\tfirst\t2\t0\t35\n2\tsecond\t0\t1\t10\n");
        await expect.poll(async () => (await transfer(page)).imports).toBe(1);
        expect((await transfer(page)).status).toBe("imported 2 objects");
        const state = await snapshot(page);
        expect(state.count).toBe(1000);
    });

    test("a file over the limit is refused before it is read", async ({ page }) => {
        const before = await transfer(page);
        // Bigger than the quarter of a megabyte the application declared.
        const huge = "1\tname\t0\t0\t5\n".repeat(30_000);
        await choose(page, "objects.txt", huge);
        await expect.poll(async () => (await transfer(page)).status).toContain("the limit is");
        expect((await transfer(page)).imports).toBe(before.imports);
    });

    test("a kind we do not read is refused, and says what we do read", async ({ page }) => {
        const before = await transfer(page);
        await choose(page, "objects.exe", "1\tname\t0\t0\t5\n");
        await expect.poll(async () => (await transfer(page)).status).toContain("we read .txt, .bin");
        expect((await transfer(page)).imports).toBe(before.imports);
    });

    test("a file of the right kind that goes wrong changes nothing", async ({ page }) => {
        const before = await snapshot(page);
        await choose(page, "objects.txt", "1\tfirst\tnope\t0\t35\n");
        await expect.poll(async () => (await transfer(page)).status).toContain("is not readable");
        expect((await transfer(page)).imports).toBe(before.transfer.imports);
        // The first line of that file was perfectly good, and it was still not
        // applied: a half-imported file describes a state no file describes.
        expect((await snapshot(page)).name).toBe(before.name);
    });

    test("choosing nothing is not an error and imports nothing", async ({ page }) => {
        const before = await transfer(page);
        await page.getByTestId("import-file").setInputFiles([]);
        await expect.poll(async () => (await transfer(page)).status).toBe("nothing chosen");
        expect((await transfer(page)).imports).toBe(before.imports);
    });

    test("a file dropped on the zone is read the same way", async ({ page }) => {
        const before = await transfer(page);
        await page.evaluate(() => {
            const zone = document.querySelector('[data-testid="import-drop"]') as HTMLElement;
            const transfer = new DataTransfer();
            transfer.items.add(
                new File(["3\tdropped\t5\t0\t20\n"], "objects.txt", { type: "text/plain" })
            );
            zone.dispatchEvent(new DragEvent("dragenter", { bubbles: true, dataTransfer: transfer }));
            zone.dispatchEvent(new DragEvent("drop", { bubbles: true, dataTransfer: transfer }));
        });
        await expect.poll(async () => (await transfer(page)).imports).toBe(before.imports + 1);
        expect((await transfer(page)).status).toBe("imported 1 objects");
        // And the zone stopped saying a file was over it.
        await expect(page.getByTestId("import-drop")).toHaveAttribute("data-state", "idle");
    });
});

test.describe("M6 V9: the clipboard, and what happens when it refuses", () => {
    // These two make their own contexts - one granted the clipboard, one whose
    // clipboard refuses - so they load their own pages.

    /// Ten thousand characters of the two things a Latin-only path gets wrong.
    ///
    /// Counted in code points, not in the UTF-16 units JavaScript calls
    /// `length`: an emoji is one character and two of those units, so slicing
    /// by `length` would both cut a pair in half and give 7,778 characters
    /// where ten thousand were asked for.
    const LONG = (() => {
        const unit = [..."属性工作台🙂🌍"];
        const out: string[] = [];
        while (out.length < 10_000) {
            out.push(...unit);
        }
        return out.slice(0, 10_000).join("");
    })();

    test("ten thousand Chinese characters and emoji make the round trip", async ({
        browser,
    }, info) => {
        const context = await browser.newContext({ baseURL: info.project.use.baseURL });
        await context.grantPermissions(["clipboard-read", "clipboard-write"]);
        const page = await context.newPage();
        await waitForReady(page);

        await page.getByTestId("object-1").click();
        await expect.poll(async () => (await snapshot(page)).selected).toBe(1);
        await page.getByTestId("notes-input").fill(LONG);
        await expect.poll(async () => (await snapshot(page)).notes).toBe(LONG);

        expect((await transfer(page)).can_copy).toBe(true);
        await page.getByTestId("copy-notes").click();
        await expect.poll(async () => (await transfer(page)).clipboard).toBe("copied");

        await page.getByTestId("notes-input").fill("");
        await expect.poll(async () => (await snapshot(page)).notes).toBe("");
        await page.getByTestId("paste-notes").click();
        await expect
            .poll(async () => (await transfer(page)).clipboard)
            .toContain("pasted 10000 characters");
        expect((await snapshot(page)).notes).toBe(LONG);
        await context.close();
    });

    test("a refusal says so and selects the text instead", async ({ browser }, info) => {
        const context = await browser.newContext({ baseURL: info.project.use.baseURL });
        // A browser that says no. What is under test is the application's
        // answer to a refusal, and a refusal has to be certain to be tested.
        await context.addInitScript(() => {
            const refuse = () => Promise.reject(new DOMException("denied", "NotAllowedError"));
            Object.defineProperty(navigator, "clipboard", {
                configurable: true,
                value: { writeText: refuse, readText: refuse },
            });
        });
        const page = await context.newPage();
        await waitForReady(page);

        await page.getByTestId("object-1").click();
        await expect.poll(async () => (await snapshot(page)).selected).toBe(1);
        await page.getByTestId("copy-notes").click();
        await expect
            .poll(async () => (await transfer(page)).clipboard)
            .toContain("the browser refused the clipboard");
        // Never a success it did not have.
        expect((await transfer(page)).copies).toBe(0);
        // And the path that needs no permission: the text, selected, in a
        // control the user can press the copy key in.
        expect((await transfer(page)).by_hand).toBe(true);
        const selected = await page.evaluate(() => {
            const field = document.querySelector(
                '[data-testid="notes-input"]'
            ) as HTMLTextAreaElement;
            return {
                focused: document.activeElement === field,
                length: field.selectionEnd - field.selectionStart,
                total: field.value.length,
            };
        });
        expect(selected.focused).toBe(true);
        expect(selected.length).toBe(selected.total);
        expect(selected.total).toBeGreaterThan(0);

        // Pasting is refused the same way, and offers the keyboard instead.
        const notes = (await snapshot(page)).notes;
        await page.getByTestId("paste-notes").click();
        await expect
            .poll(async () => (await transfer(page)).clipboard)
            .toContain("paste into the notes with the keyboard");
        expect((await transfer(page)).pastes).toBe(0);
        expect((await snapshot(page)).notes).toBe(notes);
        await context.close();
    });
});
