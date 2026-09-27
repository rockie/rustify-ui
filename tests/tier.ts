// Which of the two jobs a browser run is doing.
//
// A regression run asks whether behaviour is still correct, and has to be
// cheap enough to run on every change. An evidence run asks whether a number
// still meets its budget or has drifted over time - percentiles, bytes,
// memory tails, frame rates, long runs, idle CPU - or needs an outside
// reference or a headed browser. Those are the same specs at a different
// strength, so one file carries both and this module decides which part of it
// a run executes:
//
//   regression  (the default) every test not tagged `@evidence`; loops cut
//               to a few rounds and matrices to a representative subset
//   evidence    only the `@evidence` tests, at their full round counts
//   all         everything at full strength, which is what a release
//               verification runs
//
// A new test belongs to evidence when what it asserts is a measurement rather
// than an outcome. Tag it, or its whole describe block, with `EVIDENCE`.

export type Tier = "regression" | "evidence" | "all";

export const EVIDENCE = "@evidence";

function parse(value: string | undefined): Tier {
    if (value === undefined || value === "" || value === "regression") {
        return "regression";
    }
    if (value === "evidence" || value === "all") {
        return value;
    }
    // A typo here would otherwise run the wrong tier and report it green.
    throw new Error(`RUSTIFY_TIER=${value}: use regression, evidence or all`);
}

export const TIER: Tier = parse(process.env.RUSTIFY_TIER);

/// How many times a loop that repeats a behaviour goes round. A deterministic
/// behaviour that holds three times holds; the long counts are there to catch
/// races and leaks, and those belong to the evidence run.
export function rounds(n: number): number {
    return TIER === "regression" ? Math.min(n, 3) : n;
}

/// A parameter matrix, or the part of it that stands for the whole.
export function pick<T>(all: readonly T[], representative: readonly T[]): readonly T[] {
    return TIER === "regression" ? representative : all;
}

/// The filter a config applies so that a run executes only its tier.
export function tierFilter(): { grep?: RegExp; grepInvert?: RegExp } {
    const tagged = new RegExp(EVIDENCE);
    switch (TIER) {
        case "regression":
            return { grepInvert: tagged };
        case "evidence":
            return { grep: tagged };
        case "all":
            return {};
    }
}
