import { mkdir, writeFile } from "node:fs/promises";
import path from "node:path";
import { PNG } from "pngjs";
import { differingPixels, expect, present, test, waitForReady } from "./support";
import { hasReference, openTwin, prepareVisual } from "./twin";

test("starter pages are measured against the same-browser Canvas reference", async ({ page, context }, info) => {
    test.skip(!hasReference, "The optional Vellum source reference is absent.");
    test.setTimeout(300_000);
    await waitForReady(page);
    const twin = await context.newPage();
    await openTwin(twin);
    const results: { page: number; theme: string; differing: number; total: number; ratio: number }[] = [];
    const maxRatio = 0.02;
    const cases = [0, 1, 2].flatMap(index => (["light", "dark"] as const).map(theme => ({ index, theme })));
    for (const { index, theme } of cases) {
        await prepareVisual(page, index, theme);
        await prepareVisual(twin, index, theme);
        const actual = await page.screenshot({ path: info.outputPath(`actual-${index}-${theme}.png`), animations: "disabled" });
        const reference = await twin.screenshot({ path: info.outputPath(`reference-${index}-${theme}.png`), animations: "disabled" });
        const a = PNG.sync.read(actual), b = PNG.sync.read(reference);
        const differing = differingPixels(a, b);
        const total = a.width * a.height;
        const result = { page: index, theme, differing, total, ratio: differing / total };
        results.push(result);
        console.log(`Vellum visual: ${JSON.stringify(result)}`);
        expect(result.ratio).toBeLessThanOrEqual(maxRatio);
    }
    const directory = path.resolve(__dirname, "../../test-results/vellum");
    await mkdir(directory, { recursive: true });
    await writeFile(path.join(directory, "visual.json"), JSON.stringify({ maxRatio, results }, null, 2));
    await twin.close();
});

test("open shell menus and dialogs remain within the approved visual gate", async ({ page, context }, info) => {
    test.skip(!hasReference, "The optional Vellum source reference is absent.");
    await waitForReady(page);
    const twin = await context.newPage();
    await openTwin(twin);
    const results: { surface: string; differing: number; total: number; ratio: number }[] = [];
    for (const surface of ["main-menu", "help-dialog"]) {
        for (const target of [page, twin]) {
            await prepareVisual(target, 0, "light");
            if (surface === "main-menu") await target.getByRole("button", { name: "Vellum menu", exact: true }).click();
            else await target.getByRole("button", { name: "Keyboard shortcuts", exact: true }).click();
            await present(target);
            await target.evaluate(() => {
                for (const id of ["engine-label", "performance"]) {
                    const element = document.getElementById(id);
                    if (element) element.textContent = "Renderer";
                }
            });
        }
        const actual = PNG.sync.read(await page.screenshot({ path: info.outputPath(`actual-${surface}.png`) }));
        const reference = PNG.sync.read(await twin.screenshot({ path: info.outputPath(`reference-${surface}.png`) }));
        const differing = differingPixels(actual, reference), total = actual.width * actual.height;
        const result = { surface, differing, total, ratio: differing / total };
        results.push(result);
        console.log(`Vellum shell visual: ${JSON.stringify(result)}`);
        expect(result.ratio).toBeLessThanOrEqual(0.02);
        if (surface === "main-menu") {
            for (const target of [page, twin]) await target.keyboard.press("Escape");
        } else {
            for (const target of [page, twin]) await target.getByRole("button", { name: "Close dialog" }).click();
        }
    }
    const directory = path.resolve(__dirname, "../../test-results/vellum");
    await mkdir(directory, { recursive: true });
    await writeFile(path.join(directory, "shell-visual.json"), JSON.stringify({ maxRatio: 0.02, results }, null, 2));
    await twin.close();
});
