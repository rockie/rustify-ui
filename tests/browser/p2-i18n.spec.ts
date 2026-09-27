import { expect, Page } from "@playwright/test";

import { rounds } from "../tier";
import { test } from "./support";

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
    test.beforeEach(async ({ page }) => openSamples(page));

    test("every message the SDK ships is on the page, in both languages", async ({ page }) => {
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

    test("twenty switches leave the page saying the same thing it started with", async ({ page }) => {
        const before = await page.getByTestId("message-retry").innerText();
        const number = await page.getByTestId("format-number").innerText();
        const switches = 2 * rounds(10);
        for (let n = 0; n < switches; n += 1) {
            await page.getByTestId("toggle-locale").click();
        }
        // An even number, so the page is back where it started - and back
        // exactly, which is what says nothing accumulated on the way.
        await expect.poll(async () => (await snapshot(page)).locale).toBe("en");
        expect(await page.getByTestId("message-retry").innerText()).toBe(before);
        expect(await page.getByTestId("format-number").innerText()).toBe(number);
    });

    test("numbers and dates are the browser's, in the reader's language", async ({ page }) => {
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
    test.beforeEach(async ({ page }) => openSamples(page));

    test("the browser drew all twenty, and none of them is empty", async ({ page }) => {
        for (const id of SAMPLE_IDS) {
            const sample = page.getByTestId(`sample-${id}`);
            await expect(sample, id).toBeVisible();
            expect((await sample.innerText()).trim(), id).not.toBe("");
        }
    });

    test("a right-to-left sample says so, rather than leaving it to be guessed", async ({ page }) => {
        for (const id of ["ar-plain", "ar-digits", "ar-latin", "he-plain"]) {
            await expect(page.getByTestId(`sample-${id}`), id).toHaveAttribute("dir", "rtl");
        }
        // And one that starts with a digit is still left to right, which
        // `dir="auto"` would have got wrong.
        await expect(page.getByTestId("sample-zh-latin")).toHaveAttribute("dir", "ltr");
    });

    test("every sample actually put marks on the screen", async ({ page }) => {
        // A reviewer looks for three things: direction, order, and whether
        // anything came out as a box or a blank. The first two need eyes and a
        // reference rendering; the third has a measurable half - a line that
        // rendered nothing at all has no width, and no amount of looking at a
        // screenshot is needed to say so.
        const widths = await page.evaluate((ids) => {
            const measure = (id: string) => {
                const element = document.querySelector(
                    `[data-testid="sample-${id}"]`
                ) as HTMLElement;
                const range = document.createRange();
                range.selectNodeContents(element);
                return {
                    id,
                    text: element.textContent ?? "",
                    width: range.getBoundingClientRect().width,
                };
            };
            return ids.map(measure);
        }, SAMPLE_IDS);

        const blank = widths.filter((sample) => sample.width < 1);
        expect(blank.map((sample) => sample.id), "a sample that drew nothing").toEqual([]);

        // Width cannot be checked against character count in general, and the
        // samples are the reason why: five combining marks on one letter are
        // six characters and one narrow glyph, and a family emoji is seven
        // code points and one. Getting those *right* makes them narrow. So the
        // proportional check is made only where proportionality holds - a
        // Latin pangram with nothing stacked or joined in it.
        const pangram = widths.find((sample) => sample.id === "en-plain")!;
        expect(pangram.width, "a plain Latin line is as wide as its text").toBeGreaterThan(100);

        // And the one that should be narrow is narrow, which is the same fact
        // from the other side: it means the marks combined instead of being
        // laid out one after another.
        const stacked = widths.find((sample) => sample.id === "combining-stack")!;
        expect(stacked.width, "five marks stacked onto one letter").toBeLessThan(
            pangram.width / 4
        );
    });

    test("the families the samples need are the ones the browser loaded", async ({ page }) => {
        // `document.fonts.check` answers for the text it is given: if the page
        // is drawing Chinese with a font that has no Chinese in it, this is
        // where it shows, rather than in a screenshot somebody has to squint
        // at. The DOM half falls back to the system stack, which on this gate
        // does have these scripts - so what this catches is the page having
        // asked for something that never arrived.
        const ready = await page.evaluate(async () => {
            await document.fonts.ready;
            return {
                latin: document.fonts.check("15px system-ui", "The quick brown fox"),
                chinese: document.fonts.check("15px system-ui", "属性工作台"),
                emoji: document.fonts.check("15px system-ui", "👨‍👩‍👧‍👦"),
            };
        });
        expect(ready.latin).toBe(true);
        expect(ready.chinese).toBe(true);
        expect(ready.emoji).toBe(true);
    });

    test("the region drew all twenty as well", async ({ page }) => {
        await expect
            .poll(async () => (await page.getByTestId("samples-drawn").innerText()).trim())
            .toBe(`${SAMPLE_IDS.length} / ${SAMPLE_IDS.length}`);
    });

    test("a font that never arrives costs glyphs, and says so", async ({ page }) => {
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
