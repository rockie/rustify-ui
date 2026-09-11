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

/// R29's B0 column: the smallest complete application, not the workbench.
///
/// A different load and therefore a different budget. What makes it worth its
/// own figures is the first load: B0 is where three compressed megabytes has
/// to be enough, and B1 already spends 2.77 of them on the module alone.
export const R29_B0 = {
    cold_p95_ms: 2_500,
    first_load_bytes: 3 * 1024 * 1024,
    hot_p95_ms: 1_000,
    rounds: 30,
} as const;

/// R30 AC2 and AC3 over the large loads: scrolling B2, panning and selecting
/// in B3, and putting a hundred thousand rows in order.
export const R30_LARGE = {
    /// AC2: the interval between two **effective presentations** - two frames
    /// whose content actually differs. Not the interval between animation
    /// frame callbacks: those keep arriving at 60 Hz while the picture stands
    /// still, so a budget measured on them passes a frozen screen.
    frame_p95_ms: 20,
    /// The share of intervals that may exceed this.
    frame_slow_ms: 50,
    frame_slow_share: 0.01,
    /// Each run is this long after a warm-up that is not measured, and every
    /// one of `runs` has to pass on its own rather than the best of them.
    seconds: 60,
    warmup_seconds: 10,
    runs: 5,
    /// Every drive has to be accepted, and almost every drive has to produce a
    /// presentation: a region that keeps up with two thirds of the input is
    /// not a region that is keeping up.
    effective_frame_share: 0.99,
    /// AC3: a single-column sort and a text filter over the whole sample, end
    /// to end, and the feedback a cancel gives.
    job_p95_ms: 500,
    cancel_p95_ms: 100,
    job_rounds: 20,
} as const;

/// R32 over the large loads. The CPU figure is committed wasm memory plus the
/// JavaScript heap; the GPU figure is a ledger the host keeps, not a reading
/// from the driver.
export const R32 = {
    b0_cpu_bytes: 128 * 1024 * 1024,
    b2_cpu_bytes: 384 * 1024 * 1024,
    b0_gpu_bytes: 128 * 1024 * 1024,
    b2_gpu_bytes: 256 * 1024 * 1024,
    /// B4: what eighty rounds of mounting and unmounting may add, and what the
    /// last forty of them may add per round.
    growth_bytes: 8 * 1024 * 1024,
    growth_bytes_per_round: 64 * 1024,
} as const;

/// What these gates do **not** cover, stated here because a gate that is read
/// as covering more than it measures is worse than no gate:
///
/// - **One machine, one browser.** R29 asks for each performance machine and
///   fully supported browser to pass on its own. This suite runs Chrome on the
///   development machine under SwiftShader, a software rasteriser. A pass here
///   is a pass on that combination and says nothing about any other.
/// - **Where each one runs.** The B1 and B0 gates and B2's frame interval run
///   headless. B3's frame interval runs **only on headed Chrome**, and CI does
///   not run it. M1's second probe measured the same scene under both: p95
///   50.10 ms under headless SwiftShader against 17.60 ms on headed Chrome
///   152. SwiftShader is a software rasteriser, and R30's figures are about a
///   machine with a GPU, so a failure under it would say nothing about the
///   budget and a pass would say nothing about the machine.
/// - **A throttled link.** The cold-start gate runs on the loopback, so it
///   measures what the machine costs rather than what a link costs. The
///   throttled figures stay in `m8-network.spec.ts` as observations; they are
///   served uncompressed there and so are not a reading of AC2.
/// - **Nothing above is asserted yet.** The figures for B0, B2 and B3 are
///   written down here as of M1 so that the probes have something to be
///   measured against; the projects that fail a build for missing them are
///   built in M8. Until then a report prints the measurement beside the
///   figure and says "not gated".

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
