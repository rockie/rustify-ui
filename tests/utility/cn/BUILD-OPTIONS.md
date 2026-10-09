# cn build options getter contract

The facade keeps the original options object and reads each named field at its
original stage. It does not materialize all options before CSS discovery or
config loading. Sixteen additional production Node/WASM scenarios compare
actual Proxy/getter reads with the locked upstream build, Vite and Next source.
The [recorded 26 trace pairs](evidence/build-options.json) contain both original
and actual sequences; all pairs are equal.

## Observable stages

| Entry | Original property reads preserved by the facade |
| --- | --- |
| `build` | `cwd`, `out`, CSS selection, config selection/import, then after its await the scan options, then safelist, then a second `full` read for subsetting |
| `createBuilder` | Only `cwd` and content selection at construction; each `run` later reads the original object using the build stages |
| `builder.changed` | Known CSS short-circuits option reads; otherwise `css`, then `full \|\| tokens`; unmatched/output paths and missing files retain the last report |
| Vite `configResolved` | Real spread enumeration and own enumerable property reads, then builder creation with the copied object |
| Vite `buildStart` | If unresolved, builder creation and build start happen before returning the async hook's Promise; original getter throws reject it |
| Next factory | Builder creation and `run` start synchronously before its first await; development watches read `cwd` again after the build |
| CLI | Executable options are an internally created plain object; the SDK summary reads only `full` and `out` at the original print stage, after native gzip |

Successful build branches read `cwd` and `out` once. CSS selection reads `css`
once for `false`, twice for default discovery, or three times for an explicit
file. Config selection reads `config` for its condition and again for the
selected module. In a scan, tokens and safelist use separate condition, path
and error-label reads. Content selection reads `content` for optional length
and again when the selected array is used. The two `full` reads can return
different values; a config transform can change the value before either read.
An earlier exception or a taken branch prevents later reads.

A full build never reads tokens, content or safelist. A known theme edit does
not read `css` before rebuilding. A truthy incremental `full` prevents reading
`tokens`. These branches are tested with getters that throw if reached.
Original thrown objects retain identity. Vite and Next also support a bounded
getter that reenters the same hook/factory: immutable Rust closures and short
state borrows preserve that native JS behavior.

## Evidence

The first real production differential had 40 passing prior scenarios and 14
failing options scenarios; [the original RED log](build-host-options-red.log)
shows the unconditional snapshot reads and early throws. The async hook throw
case also exposed an unhandled reference rejection after its assertion failed;
the test now attaches a rejection observer while retaining the assertion.

A stricter pre-await trace then exposed the [Next start-time RED](build-host-options-next-start-red.log).
A finite options getter exposed the [Next closure reentry RED](build-host-options-reentry-red.log).
The final packaged facade passes all 56 host scenarios, including these two
closures and the original 40 host cases. It also passes the separate 947 fixed
and 56 added runtime cases. See [production evidence](BUILD-HOST.md).

Core native APIs remain intact. New preparation, source-only scanning and
source-change steps share the existing Rust algorithms; the facade places
native JS reads around those steps. No JSON conversion or JS build algorithm
implements options handling. The fixed reference inventory and fixtures remain
unchanged.

One full run initially timed out waiting two seconds for the native recursive
watcher to update a nested-file edit. The original full log was overwritten on
replay; the tool conversation retains that observation. An
[isolated native event recording](build-host-options-watch-probe.log) then
recorded the file callback and changed output, and two full runs on the same
unchanged bundle passed. The cause is unproven; this is not evidence of a
registration race. The machine record preserves the observation and checks.
