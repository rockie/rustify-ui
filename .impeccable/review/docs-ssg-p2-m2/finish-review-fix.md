## verdict

1. **resolved — mobile status reservation:** the re-opened `mobile.png` and `client-only-375.png` show compact 72px ready rows instead of 128px blank bands; `no-script-375.png` retains the explanation in the same footprint. The new light/dark loading and failure captures at 375 show complete recovery text and Reload buttons with the source starting at the same position. The 200% captures keep wrapped recovery text unobscured and reserve its intrinsic height through loading/failure. `docs.css` retains the shared 4.5rem minimum and invisible recovery-item grid reservation, with the mobile 8rem override removed. `visual-capture-r2.json` reports 72px rows for the eight main captures; `islands-r6/results.json` records 20 expected tests passing with zero skipped/unexpected/flaky. The inspected responsive test asserts exact status-height and source-position equality through loading, ready and failure. All 20 current packet captures pass evidence check 0.

Regressions introduced by the fix batch: none visible in the supplied captures.

## remaining

clear. Ship covers the scored fix, not the whole surface. The full review's stated scope and verification limits remain.

disposition: ship
