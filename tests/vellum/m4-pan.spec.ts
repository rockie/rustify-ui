import { mkdir, writeFile } from "node:fs/promises";
import path from "node:path";
import { expect, test, waitForReady } from "./support";
import { EVIDENCE, GPU_ONLY } from "../tier";

// A file of its own because it draws on the machine's GPU rather than the
// SwiftShader every other Vellum test is pinned to, and launch options are
// per worker. Under SwiftShader a pan frame's p95 is 130-140 ms against a
// budget of 50, on a build from before a change and on one from after it
// alike, so it runs by hand, headed, on a machine with a GPU.
test.use({ launchOptions: { args: [], executablePath: process.env.RUSTIFY_CHROMIUM || undefined } });
test.skip(({ headless }) => headless, GPU_ONLY);

test("pan presentation latency is measured from input to an SDK frame in the page", { tag: EVIDENCE }, async ({ page }, info) => {
    await waitForReady(page);
    const box = (await page.locator("#overlay").boundingBox())!;
    await page.mouse.move(box.x + 500, box.y + 350);
    await page.mouse.down({ button: "middle" });
    const samples = await page.evaluate(async () => {
        const overlay = document.getElementById("overlay")!, box = overlay.getBoundingClientRect();
        const frame = () => new Promise<void>(resolve => requestAnimationFrame(() => resolve()));
        const event = (type: string, x: number, y: number, buttons: number) => overlay.dispatchEvent(new PointerEvent(type, { bubbles: true, pointerId: 1, pointerType: "mouse", button: 1, buttons, clientX: box.x + x, clientY: box.y + y }));
        const durations: number[] = [];
        for (let index = 0; index < 30; index++) {
            const before = window.__vellum.stats().frames;
            const start = performance.now();
            event("pointermove", 500 + (index + 1) * 2, 350 + (index + 1), 4);
            while (window.__vellum.stats().frames <= before) {
                if (performance.now() - start > 2000) throw new Error("Pan did not produce an SDK frame");
                await frame();
            }
            durations.push(performance.now() - start);
            await frame();
        }
        return durations;
    });
    await page.mouse.up({ button: "middle" });
    const sorted = [...samples].sort((a, b) => a - b);
    const p95 = sorted[Math.ceil(sorted.length * 0.95) - 1];
    const result = { samples, p95, unit: "ms", criterion: "input dispatch to SDK presentation frame count observed in requestAnimationFrame" };
    console.log(`Pan presentation: ${JSON.stringify(result)}`);
    const directory = path.resolve(__dirname, "../../test-results/vellum");
    await mkdir(directory, { recursive: true });
    await writeFile(path.join(directory, "m4-pan.json"), JSON.stringify(result, null, 2));
    await info.attach("pan-presentation", { body: JSON.stringify(result), contentType: "application/json" });
    expect(p95, "ADR-2 requires a camera ownership reassessment above 50 ms").toBeLessThanOrEqual(50);
});
