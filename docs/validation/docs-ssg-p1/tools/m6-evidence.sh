#!/bin/bash
# M6 exit evidence: `rustify-docs dev`. V-9 in the browser (`docs-dev`, the
# docs-cli binary, panic recovery included) and as a process (the CLI's dev
# tests), V-14's saves on reference site B (body, configuration, the logo:
# five each, median; then a short round of saves one after another), and the
# docs packages' own suites, the CLI graph's gate (V-1) and the CLI profile.
set -u
M=/Users/rockie/Documents/gh-rockie/rustify-ui
E=$M/docs/validation/docs-ssg-p1/m6
cd "$M" || exit 1
mkdir -p "$E/host" "$E/browser"
status() { echo "$1 status=$2" | tee -a "$E/summary.txt"; }
: > "$E/summary.txt"
date '+started %F %T %z' >> "$E/summary.txt"

cargo fmt --all -- --check > "$E/host/fmt.log" 2>&1; status fmt $?
for p in rustify-docs rustify-docs-cli; do
  mbx test --locked -p $p > "$E/host/$p-test.log" 2>&1; status $p-test $?
  mbx clippy --locked -p $p --all-targets -- -D warnings > "$E/host/$p-clippy.log" 2>&1; status $p-clippy $?
done
grep -E "^test result|Running" "$E/host/rustify-docs-test.log" "$E/host/rustify-docs-cli-test.log" > "$E/host/test-counts.txt"
mbx build --locked -p rustify-docs-cli --profile docs-cli > "$E/host/docs-cli-build.log" 2>&1; status docs-cli-build $?
target/docs-cli/rustify-docs --build-info > "$E/host/build-info.txt" 2>&1; status build-info $?
mbx xtask docs-verify --group features > "$E/features.log" 2>&1; status features $?

# A build that aborts on a panic refuses `dev` before it watches or listens,
# and says how to install one that does not.
P=$M/target/m6-abort-probe
CARGO_PROFILE_DEV_PANIC=abort CARGO_TARGET_DIR=$P mbx build --locked -p rustify-docs-cli > "$E/host/abort-build.log" 2>&1
W=$(mktemp -d)
cp -R tests/docs-ssg/fixtures/content "$W/site"
(cd "$W/site" && perl -e 'alarm 30; exec @ARGV' "$P/debug/rustify-docs" dev --port 0) > "$E/host/abort-dev.log" 2>&1
code=$?
rm -rf "$W"
grep -q -- "--profile docs-cli rustify-docs-cli" "$E/host/abort-dev.log" && [ $code -eq 1 ]; status abort-refuses-dev $?
rm -rf "$P"

# V-9: dev in the browser, with the binary just built.
RUSTIFY_DOCS_BIN=target/docs-cli/rustify-docs npx playwright test -c playwright.docs.config.ts --project=docs-dev > "$E/browser/docs-dev.log" 2>&1; status browser-docs-dev $?

# V-14: saves on reference site B, written afresh from its frozen profile.
B=$M/target/docs-ssg/m6/b
rm -rf "$B"
mbx xtask docs-corpus --out "$B" > "$E/host/corpus.log" 2>&1; status corpus $?
node tests/docs-ssg/harness/measure-save.mjs --bin target/docs-cli/rustify-docs --site "$B" --page /zh/g04/page-024/ --samples 5 --short 20 --out "$E/save.json" > "$E/save.log" 2>&1; status save $?
date '+done %F %T %z' >> "$E/summary.txt"
