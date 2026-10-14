import { expect, Page } from "@playwright/test";

// What the reduced-motion contract names, relative to an island's SDK scope.
// The five child elements carry their own transition utility and no
// `data-name`, so a rule that only reaches `[data-name]` misses them.
export const motionTargets = {
    "Switch thumb": '[role="switch"] > span',
    "Checkbox box": 'input[type="checkbox"] + span',
    "Radio dot": 'input[type="radio"] + span',
    "Progress fill": '[role="progressbar"] > div',
    "ColorField input": '[data-name="ColorField"] > input',
    Tab: '[data-name="Tab"]',
    Button: '[data-name="Button"]',
    "Spinner svg": '[data-name="Spinner"] > svg',
} as const;
export type MotionTarget = keyof typeof motionTargets;

export type MotionSample = {
    label: string;
    /** Each scope's own motion token, which `[data-name]` roots follow. */
    tokens: string[];
    present: Record<string, number>;
    /** Computed values for the named targets, first match in each scope. */
    computed: { target: string; duration: string; delay: string; animation: string }[];
    /** Every element in a scope whose computed transition is not instant. */
    moving: { node: string; duration: string; delay: string }[];
    /** Finite CSS transitions in a scope at the moment of sampling. */
    transitions: { node: string; property: string; duration: number | string | undefined; state: string }[];
    /** Every `transitionrun` in a scope since the page started. */
    runs: { node: string; property: string; at: number }[];
    spinners: { name: string; running: number }[];
};

declare global {
    interface Window { __gaps_motion?: { node: string; property: string; at: number }[] }
}

/** Records every transition a scope creates, before the page's own scripts run. */
export async function installMotionProbe(page: Page) {
    await page.addInitScript(() => {
        const runs: { node: string; property: string; at: number }[] = [];
        window.__gaps_motion = runs;
        const describe = (node: Element) => {
            const owner = node.closest("[data-name]")?.getAttribute("data-name") ?? "scope";
            const name = node.getAttribute("data-name");
            return `${node.tagName.toLowerCase()}${name ? `[data-name=${name}]` : `.${[...node.classList].slice(0, 3).join(".")}`} in ${owner}`;
        };
        document.addEventListener("transitionrun", event => {
            const node = event.target as Element;
            if (node instanceof Element && node.closest("[data-rustify-scope]")) {
                runs.push({ node: describe(node), property: (event as TransitionEvent).propertyName, at: Math.round(performance.now()) });
            }
        }, true);
    });
}

/** Reads styles and animations at once: no waiting, retry or transition end before it. */
export async function sampleMotion(page: Page, label: string): Promise<MotionSample> {
    return page.evaluate(([label, targets]) => {
        const describe = (node: Element) => {
            const owner = node.closest("[data-name]")?.getAttribute("data-name") ?? "scope";
            const name = node.getAttribute("data-name");
            return `${node.tagName.toLowerCase()}${name ? `[data-name=${name}]` : `.${[...node.classList].slice(0, 3).join(".")}`} in ${owner}`;
        };
        const scopes = [...document.querySelectorAll("[data-rustify-scope]")];
        const inScope = (node: Element | null | undefined) => !!node && scopes.some(scope => scope === node || scope.contains(node));
        const instant = (value: string) => value.split(",").every(part => parseFloat(part) === 0);
        const animations = document.getAnimations();
        const moving: MotionSample["moving"] = [];
        for (const scope of scopes) {
            for (const node of [scope, ...scope.querySelectorAll("*")]) {
                const style = getComputedStyle(node);
                if (!instant(style.transitionDuration) || !instant(style.transitionDelay)) {
                    moving.push({ node: describe(node), duration: style.transitionDuration, delay: style.transitionDelay });
                }
            }
        }
        const computed: MotionSample["computed"] = [];
        const present: Record<string, number> = {};
        for (const [target, selector] of Object.entries(targets)) {
            present[target] = 0;
            for (const scope of scopes) {
                const nodes = scope.querySelectorAll(selector);
                present[target] += nodes.length;
                if (nodes[0]) {
                    const style = getComputedStyle(nodes[0]);
                    computed.push({ target, duration: style.transitionDuration, delay: style.transitionDelay, animation: style.animationName });
                }
            }
        }
        const transitions = animations
            .filter((animation): animation is CSSTransition => animation instanceof CSSTransition)
            .filter(animation => inScope((animation.effect as KeyframeEffect | null)?.target as Element | null))
            .filter(animation => Number.isFinite(animation.effect?.getTiming().iterations))
            .map(animation => ({
                node: describe((animation.effect as KeyframeEffect).target as Element),
                property: animation.transitionProperty,
                duration: animation.effect?.getTiming().duration,
                state: animation.playState,
            }));
        const spinners = [...document.querySelectorAll('[data-rustify-scope] [data-name="Spinner"] > svg')].map(svg => ({
            name: getComputedStyle(svg).animationName,
            running: animations.filter(animation => animation instanceof CSSAnimation && animation.animationName === "rui-spin"
                && (animation.effect as KeyframeEffect | null)?.target === svg && animation.playState === "running").length,
        }));
        const tokens = scopes.map(scope => getComputedStyle(scope).getPropertyValue("--motion-duration").trim());
        return { label, tokens, present, computed, moving, transitions, runs: [...(window.__gaps_motion ?? [])], spinners };
    }, [label, motionTargets] as const);
}

/** Under `reduce`: nothing in a scope transitions, and no spinner turns. */
export function expectStill(sample: MotionSample) {
    expect(sample.moving, `${sample.label}: computed transition duration/delay`).toEqual([]);
    expect(sample.transitions, `${sample.label}: CSSTransition in a scope`).toEqual([]);
    expect(sample.runs, `${sample.label}: transitionrun in a scope`).toEqual([]);
    for (const spinner of sample.spinners) expect(spinner, `${sample.label}: spinner`).toEqual({ name: "none", running: 0 });
}

export async function attachMotion(page: Page, info: { attach(name: string, options: { body: string; contentType: string }): Promise<void> }, name: string, samples: MotionSample[]) {
    await info.attach(name, { body: JSON.stringify({ url: page.url(), samples }, null, 2), contentType: "application/json" });
}

// Children that carry a Tailwind transition utility and Tailwind's default
// duration; the `[data-name]` roots follow the scope's motion token instead.
const utilityChildren: string[] = ["Switch thumb", "Checkbox box", "Radio dot", "Progress fill", "ColorField input"];

export const seconds = (value: string) => value.trim().endsWith("ms") ? parseFloat(value) / 1000 : parseFloat(value);

/** Without a preference: the SDK's own durations, unchanged by the reduce rule. */
export function expectNormal(sample: MotionSample) {
    expect(new Set(sample.tokens).size, `${sample.label}: one motion token`).toBe(1);
    const token = seconds(sample.tokens[0]);
    for (const entry of sample.computed) {
        if (entry.target === "Spinner svg") {
            expect(entry.animation, `${sample.label}: ${entry.target}`).toBe("rui-spin");
            continue;
        }
        const expected = utilityChildren.includes(entry.target) ? 0.15 : token;
        expect(entry.duration.split(",").map(seconds), `${sample.label}: ${entry.target} duration`).toEqual(entry.duration.split(",").map(() => expected));
    }
}
