import { expect, Page, test } from "@playwright/test";

import { sharedPage } from "./support";

/// M7 V10: two languages, the browser's own formatting, and twenty texts.
///
/// The direction and legibility of the samples is a person's judgement against
/// a reference rendering (A-4); what is checked here is everything a machine
/// can check about them - that there are twenty, that both halves drew all
/// twenty, that none is empty, and that a right-to-left sample says so on the
/// element rather than leaving the browser to guess from its first character.

const snapshot = (page: Page) => page.evaluate(() => window.__component_catalog.snapshot());

/// The twenty ids, in the order the page draws them.
const SAMPLE_IDS = [
    "en-plain",
    "en-punctuation",
    "zh-common",
    "zh-punctuation",
    "zh-latin",
    "zh-traditional",
    "zh-vertical-forms",
    "ar-plain",
    "ar-digits",
    "ar-latin",
    "he-plain",
    "combining-acute",
    "combining-stack",
    "combining-devanagari",
    "combining-thai",
    "emoji-family",
    "emoji-skin-tone",
    "emoji-flags",
    "emoji-in-text",
    "mixed-everything",
];

async function openSamples(page: Page) {
    await page.getByTestId("nav-samples").click();
    await expect.poll(async () => (await snapshot(page)).path).toBe("/samples");
    await expect(page.getByTestId("samples-page")).toBeVisible();
}

test.describe("M7 V10: the SDK's own words follow the language", () => {
    test.describe.configure({ mode: "serial" });
    const shared = sharedPage(openSamples);

    test("every message the SDK ships is on the page, in both languages", async () => {
        const page = shared.page;
        const messages = page.getByTestId("sdk-messages").locator("li");
        const count = await messages.count();
        expect(count).toBeGreaterThan(0);

        const english = await messages.allInnerTexts();
        await page.getByTestId("toggle-locale").click();
        await expect.poll(async () => (await snapshot(page)).locale).toBe("zh-CN");
        const chinese = await messages.allInnerTexts();

        expect(chinese).toHaveLength(count);
        for (const [index, text] of chinese.entries()) {
            expect(text.trim(), `message ${index}`).not.toBe("");
            // Every one of them is actually translated, rather than left in
            // the other language - the mistake a catalogue makes silently.
            expect(text, `message ${index}`).not.toBe(english[index]);
        }

        await page.getByTestId("toggle-locale").click();
        await expect.poll(async () => (await snapshot(page)).locale).toBe("en");
    });

    test("twenty switches leave the page saying the same thing it started with", async () => {
        const page = shared.page;
        const before = await page.getByTestId("message-retry").innerText();
        const number = await page.getByTestId("format-number").innerText();
        for (let n = 0; n < 20; n += 1) {
            await page.getByTestId("toggle-locale").click();
        }
        // Twenty is even, so the page is back where it started - and back
        // exactly, which is what says nothing accumulated on the way.
        await expect.poll(async () => (await snapshot(page)).locale).toBe("en");
        expect(await page.getByTestId("message-retry").innerText()).toBe(before);
        expect(await page.getByTestId("format-number").innerText()).toBe(number);
    });

    test("numbers and dates are the browser's, in the reader's language", async () => {
        const page = shared.page;
        const number = () => page.getByTestId("format-number").innerText();
        const date = () => page.getByTestId("format-date").innerText();

        const englishNumber = await number();
        const englishDate = await date();
        // The application asked for a currency; the language decides how one
        // is written.
        expect(englishNumber).toContain("1,234,567.89");
        expect(englishDate).toMatch(/2026/);

        await page.getByTestId("toggle-locale").click();
        await expect.poll(async () => (await snapshot(page)).locale).toBe("zh-CN");
        const chineseDate = await date();
        expect(chineseDate).not.toBe(englishDate);
        expect(chineseDate).toContain("2026");

        await page.getByTestId("toggle-locale").click();
        await expect.poll(async () => (await snapshot(page)).locale).toBe("en");
    });
});

test.describe("M7 V10: twenty texts, drawn twice", () => {
    test.describe.configure({ mode: "serial" });
    const shared = sharedPage(openSamples);

    test("the browser drew all twenty, and none of them is empty", async () => {
        const page = shared.page;
        for (const id of SAMPLE_IDS) {
            const sample = page.getByTestId(`sample-${id}`);
            await expect(sample, id).toBeVisible();
            expect((await sample.innerText()).trim(), id).not.toBe("");
        }
    });

    test("a right-to-left sample says so, rather than leaving it to be guessed", async () => {
        const page = shared.page;
        for (const id of ["ar-plain", "ar-digits", "ar-latin", "he-plain"]) {
            await expect(page.getByTestId(`sample-${id}`), id).toHaveAttribute("dir", "rtl");
        }
        // And one that starts with a digit is still left to right, which
        // `dir="auto"` would have got wrong.
        await expect(page.getByTestId("sample-zh-latin")).toHaveAttribute("dir", "ltr");
    });

    test("the region drew all twenty as well", async () => {
        const page = shared.page;
        await expect
            .poll(async () => (await page.getByTestId("samples-drawn").innerText()).trim())
            .toBe(`${SAMPLE_IDS.length} / ${SAMPLE_IDS.length}`);
    });

    test("a font that never arrives costs glyphs, and says so", async () => {
        const page = shared.page;
        const drawn = () => page.getByTestId("samples-drawn").innerText();
        const marked = async () =>
            Number((await page.getByTestId("samples-marked").innerText()).trim());
        const all = `${SAMPLE_IDS.length} / ${SAMPLE_IDS.length}`;

        // The SDK's own words for it, and they say whose fault it is: a row of
        // boxes on its own tells a reader nothing.
        expect(await page.getByTestId("message-missing-glyph").innerText()).toContain("□");
        expect(await marked()).toBe(0);

        await page.getByTestId("block-font").click();
        // The page took the request first; then the region says what it drew.
        await expect.poll(async () => page.getByTestId("samples-blocked").innerText()).toBe("true");
        // …and the projection into the region carries it.
        await expect.poll(async () => page.getByTestId("samples-asked").innerText()).toBe("true");
        // Sixteen of the twenty need more than Latin. The count comes from the
        // region, which is the only thing that knows what it drew.
        await expect.poll(marked).toBeGreaterThan(0);
        // And it is still twenty lines: the samples that are pure Latin are
        // unaffected, and nothing was dropped.
        expect((await drawn()).trim()).toBe(all);

        // When the font arrives, the region is back to the samples themselves.
        await page.getByTestId("block-font").click();
        await expect.poll(marked).toBe(0);
        expect((await drawn()).trim()).toBe(all);
    });
});
