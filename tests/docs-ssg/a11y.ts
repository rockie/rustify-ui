import AxeBuilder from "@axe-core/playwright";
import { expect, Page } from "@playwright/test";
import { execFileSync } from "node:child_process";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import path from "node:path";

export const axeTags = ["wcag2a", "wcag2aa", "wcag21a", "wcag21aa", "wcag22aa"];

type Result = Awaited<ReturnType<AxeBuilder["analyze"]>>;

export type Scan = {
    route: string;
    state: string;
    width: number;
    colorScheme: string;
    motion: string;
    buildId: string | null;
    violations: Result["violations"];
    incomplete: Result["incomplete"];
};

let binaryRecord: { path: string; sha256: string; buildInfo: string } | undefined;

/** The CLI that built the served site, as the servers resolve it. */
export function binary() {
    if (!binaryRecord) {
        const file = path.resolve(process.env.RUSTIFY_DOCS_BIN || "target/docs-cli/rustify-docs");
        binaryRecord = {
            path: file,
            sha256: createHash("sha256").update(readFileSync(file)).digest("hex"),
            buildInfo: execFileSync(file, ["--build-info"], { encoding: "utf8" }),
        };
    }
    return binaryRecord;
}

/**
 * Waits for the state the reader rests in: fonts loaded and the finite
 * transitions and animations running now finished. The infinite spinner is
 * excluded. Only for the normal motion preference; under `reduce` nothing
 * may need waiting out.
 */
export async function atRest(page: Page) {
    await page.evaluate(async () => {
        await document.fonts.ready;
        const finite = document.getAnimations().filter(animation =>
            (animation instanceof CSSTransition || animation instanceof CSSAnimation)
            && Number.isFinite(animation.effect?.getComputedTiming().endTime));
        await Promise.all(finite.map(animation => animation.finished.catch(() => undefined)));
        await new Promise<void>(resolve => requestAnimationFrame(() => requestAnimationFrame(() => resolve())));
    });
}

export async function scan(page: Page, route: string, state: string): Promise<Scan> {
    const { violations, incomplete } = await new AxeBuilder({ page }).withTags(axeTags).analyze();
    const viewport = page.viewportSize()!;
    const facts = await page.evaluate(() => ({
        colorScheme: matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light",
        motion: matchMedia("(prefers-reduced-motion: reduce)").matches ? "reduce" : "no-preference",
    }));
    return { route, state, width: viewport.width, colorScheme: facts.colorScheme, motion: facts.motion, buildId: null, violations, incomplete };
}

/** The manifest's build id of the site under `root`. */
export async function buildId(page: Page, root: string) {
    const response = await page.request.get(new URL("rustify-docs-manifest.json", root).href);
    expect(response.status()).toBe(200);
    return (await response.json()).build_id as string;
}

export function serious(scans: Scan[]) {
    return scans.flatMap(scan => scan.violations
        .filter(violation => violation.impact === "serious" || violation.impact === "critical")
        .map(violation => ({ route: scan.route, state: scan.state, width: scan.width, colorScheme: scan.colorScheme, id: violation.id, targets: violation.nodes.map(node => node.target) })));
}

export async function attachScans(info: { attach(name: string, options: { body: string; contentType: string }): Promise<void> }, name: string, scans: Scan[], id: string) {
    const record = { binary: binary(), buildId: id, tags: axeTags, scans: scans.map(scan => ({ ...scan, buildId: id })) };
    await info.attach(name, { body: JSON.stringify(record, null, 2), contentType: "application/json" });
}
