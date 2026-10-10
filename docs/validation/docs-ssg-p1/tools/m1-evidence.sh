#!/bin/bash
# Final M1 evidence: V-15 host matrix, V-1 in and out of the repository, V-2 on both builds.
set -u
M=/Users/rockie/Documents/gh-rockie/rustify-ui
E=$M/docs/validation/docs-ssg-p1/m1
cd "$M" || exit 1
mkdir -p "$E/host"
status() { echo "$1 status=$2" | tee -a "$E/summary.txt"; }
: > "$E/summary.txt"
cargo fmt --all -- --check > "$E/host/fmt.log" 2>&1; status fmt $?
(cd tests/dom-only && cargo fmt --all -- --check) > "$E/host/fmt-dom-only.log" 2>&1; status fmt-dom-only $?
mbx test --locked --workspace --lib --bins > "$E/host/workspace-test.log" 2>&1; status workspace-test $?
mbx clippy --locked --workspace --all-targets -- -D warnings > "$E/host/workspace-clippy.log" 2>&1; status workspace-clippy $?
for p in rustify-ui rustify-components; do
  mbx test --locked -p $p --lib > "$E/host/$p-default-test.log" 2>&1; status $p-default-test $?
  mbx test --locked -p $p --features gpu --lib > "$E/host/$p-gpu-test.log" 2>&1; status $p-gpu-test $?
  mbx test --locked -p $p --no-default-features --features ssr --lib > "$E/host/$p-ssr-test.log" 2>&1; status $p-ssr-test $?
  mbx clippy --locked -p $p --all-targets -- -D warnings > "$E/host/$p-default-clippy.log" 2>&1; status $p-default-clippy $?
  mbx clippy --locked -p $p --features gpu --all-targets -- -D warnings > "$E/host/$p-gpu-clippy.log" 2>&1; status $p-gpu-clippy $?
  mbx clippy --locked -p $p --no-default-features --features ssr --all-targets -- -D warnings > "$E/host/$p-ssr-clippy.log" 2>&1; status $p-ssr-clippy $?
done
mbx test --locked -p rustify-ui -p rustify-components --features rustify-components/utility --lib > "$E/host/sdk-utility-test.log" 2>&1; status sdk-utility-test $?
mbx test --locked --manifest-path tests/theme-export/Cargo.toml --workspace --bins > "$E/host/theme-export-test.log" 2>&1; status theme-export-test $?
mbx xtask sources verify > "$E/host/sources.log" 2>&1; status sources $?
mbx xtask css --check > "$E/host/css.log" 2>&1; status css $?
mbx xtask catalog --write docs/components.md --check > "$E/host/catalog.log" 2>&1; status catalog $?
mbx xtask docs-verify --group features > "$E/features.log" 2>&1; status features $?
npx playwright test -c playwright.docs.config.ts --project=dom-only > "$E/dom-only.log" 2>&1; status dom-only $?
gzip -9 -c test-results/docs/results.json > "$E/dom-only.json.gz"
mbx xtask docs-verify --group features --external > "$E/features-external.log" 2>&1; status features-external $?
EXT=$(sed -n 's/^features (outside the repository): the default build is DOM only; the product is //p' "$E/features-external.log")
DOM_ONLY_DIR="$EXT" npx playwright test -c playwright.docs.config.ts --project=dom-only > "$E/dom-only-external.log" 2>&1; status dom-only-external $?
gzip -9 -c test-results/docs/results.json > "$E/dom-only-external.json.gz"
echo done >> "$E/summary.txt"
