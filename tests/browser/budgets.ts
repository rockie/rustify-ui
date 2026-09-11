/// The performance budgets, in one place.
///
/// Until 2026-09-11 these figures were targets to compare against: the PRD's
/// numbers were not an approved budget (A-3), so `docs/reports/p2/performance.md`
/// printed measurements beside them and nothing failed. A-3 is now closed - the
/// user approved R29 and R30 as gates - so the numbers below are assertions,
/// and `p2-budget.spec.ts` is where they are asserted.
///
/// Every figure here is B1's. R29 also carries B0 figures (2.5 s, 3 MiB, 1 s)
/// and they are *not* the ones to gate this build with: B1 is the load P2
/// delivers, and a B0 budget applied to a B1 build would be a different claim.
export const R29 = {
    /// AC1: cold start, first interactive, p95 over thirty loads.
    cold_p95_ms: 5_000,
    /// AC2: what a first load puts on the wire, compressed, in bytes.
    first_load_bytes: 8 * 1024 * 1024,
    /// AC3: hot start, resources already cached, p95 over thirty loads.
    hot_p95_ms: 2_000,
    /// Both start-up figures are p95 over this many loads per browser/machine
    /// combination. Thirty is the PRD's number, not a sample size chosen here.
    rounds: 30,
} as const;

export const R30 = {
    /// AC1: input to presentation for cross-region selection and property
    /// editing.
    latency_p95_ms: 50,
    latency_p99_ms: 100,
    /// At least this many actions per combination, with no errored action.
    actions: 1_000,
} as const;

/// What these gates do **not** cover, stated here because a gate that is read
/// as covering more than it measures is worse than no gate:
///
/// - **One machine, one browser.** R29 asks for each performance machine and
///   fully supported browser to pass on its own. This suite runs Chrome on the
///   development machine under SwiftShader, a software rasteriser. A pass here
///   is a pass on that combination and says nothing about any other.
/// - **B0, B2 and B3.** P2 delivers B1. R29's B0 column and R30's AC2/AC3
///   (scrolling, panning, sorting over the large loads) belong to loads this
///   phase does not build, and are reported as not measured rather than passed.
/// - **A throttled link.** The cold-start gate runs on the loopback, so it
///   measures what the machine costs rather than what a link costs. The
///   throttled figures stay in `m8-network.spec.ts` as observations; they are
///   served uncompressed there and so are not a reading of AC2.

/// The p-th percentile of `samples`, nearest-rank, which is the definition
/// that keeps a p95 of thirty samples an actual sample rather than an
/// interpolation between two of them.
export function percentile(samples: number[], p: number): number {
    if (samples.length === 0) {
        throw new Error("no samples");
    }
    const sorted = [...samples].sort((a, b) => a - b);
    const rank = Math.ceil((p / 100) * sorted.length);
    return sorted[Math.min(sorted.length - 1, Math.max(1, rank) - 1)];
}
