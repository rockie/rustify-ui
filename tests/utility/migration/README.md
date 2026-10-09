# Component class migration snapshots

Historical class output fixtures for [Rustify UI](../../../README.md), captured before replacing the component merger. Future tests read [fixtures.json](fixtures.json) and require no old package or derive implementation.

The capture ran against a freshly built `rustify-components` library after:

```sh
mbx test --locked -p rustify-components --lib macros::
mbx build --locked -p rustify-components --lib
```

The six macro tests passed. A temporary Rust binary called the actual component class APIs and `macros::merge`; its source/output digests and the compiler-artifact library path are recorded in the fixture. The temporary capture program is not part of the repository or future test dependencies.

The 125 outputs cover 56 Button variant/size combinations with default and caller classes, 55 component class constants, seven merge contracts and seven generated API cases. The fixture records original class fragments, caller class, default enum arms, source hashes, API syntax and current call sites. Builder snapshots are historical reference only; unused generated builder methods do not add migration requirements. Required component names, used macro forms, enum defaults and class conversion methods remain recorded.

Check fixture integrity without a Cargo build:

```sh
node tests/utility/migration/check.mjs
```

Before changing component sources, the capture source hashes can also be checked:

```sh
node tests/utility/migration/check.mjs --check-capture-sources
```

The source check is deliberately optional because the migration changes these files. The default integrity check reconstructs the captured output digest and checks unique IDs and the recorded case count; it does not prove the new implementation matches them.

The fixed cn reference remains the merge oracle. `reference.mjs --check` verifies [cn-expected.json](cn-expected.json), and `mbx test -p rustify-components --lib class_migration` checks all 125 inputs against the new Rust engine, including actual Button variant/size values and retained raw fragments. The 74 changes preserve ring color classes that the old engine discarded when a ring width followed them; no tokens are removed by these changes. The original captures remain historical evidence.

The rebuilt component catalog passed 24 Chrome catalog/theme/legacy-theme checks; see [M2 evidence](../../../docs/validation/wasm-tailwind/M2.md). These static checks do not validate dynamic cn configuration epochs, which remain pending.
