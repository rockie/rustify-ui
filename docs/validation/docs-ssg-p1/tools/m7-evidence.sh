#!/bin/bash
# M7 exit evidence: the minimal toolchain, security and recovery gate, and
# the commit point's V-15. V-1 (features), V-5/V-6 (artifacts: new/build and
# what a build publishes), V-10 (reliability: real process kills, a full
# disk, refused permissions, damaged outputs, two writers, outputs and files
# that must not be touched, author HTML, leaks; external links), V-11
# (toolchain: install from a commit, the five commands in an author's
# environment, offline under a no-network sandbox), V-7's N7/AC-4 in the
# browser, V-9 again after the CLI changed, and the full V-15 matrix.
set -u
M=/Users/rockie/Documents/gh-rockie/rustify-ui
E=$M/docs/validation/docs-ssg-p1/m7
cd "$M" || exit 1
mkdir -p "$E/host" "$E/browser"
status() { echo "$1 status=$2" | tee -a "$E/summary.txt"; }
: > "$E/summary.txt"
date '+started %F %T %z' >> "$E/summary.txt"

# V-15: formatting, sources, generated files.
cargo fmt --all -- --check > "$E/host/fmt.log" 2>&1; status fmt $?
mbx xtask sources verify > "$E/host/sources.log" 2>&1; status sources-verify $?
mbx xtask css --check > "$E/host/css.log" 2>&1; status css-check $?
mbx xtask catalog --write docs/components.md --check > "$E/host/catalog.log" 2>&1; status catalog-check $?

# V-15: the §9.2 matrix, each cell on its own.
mbx test --locked --workspace --exclude rustify-docs --exclude rustify-docs-cli --lib --bins > "$E/host/workspace-test.log" 2>&1; status workspace-test $?
mbx clippy --locked --workspace --exclude rustify-docs --exclude rustify-docs-cli --all-targets -- -D warnings > "$E/host/workspace-clippy.log" 2>&1; status workspace-clippy $?
for p in rustify-ui rustify-components; do
  mbx test --locked -p $p --lib > "$E/host/$p-default-test.log" 2>&1; status $p-default-test $?
  mbx clippy --locked -p $p --all-targets -- -D warnings > "$E/host/$p-default-clippy.log" 2>&1; status $p-default-clippy $?
  mbx test --locked -p $p --features gpu --lib > "$E/host/$p-gpu-test.log" 2>&1; status $p-gpu-test $?
  mbx clippy --locked -p $p --features gpu --all-targets -- -D warnings > "$E/host/$p-gpu-clippy.log" 2>&1; status $p-gpu-clippy $?
  mbx test --locked -p $p --no-default-features --features ssr --lib > "$E/host/$p-ssr-test.log" 2>&1; status $p-ssr-test $?
  mbx clippy --locked -p $p --no-default-features --features ssr --all-targets -- -D warnings > "$E/host/$p-ssr-clippy.log" 2>&1; status $p-ssr-clippy $?
done
for p in rustify-docs rustify-docs-cli; do
  mbx test --locked -p $p > "$E/host/$p-test.log" 2>&1; status $p-test $?
  mbx clippy --locked -p $p --all-targets -- -D warnings > "$E/host/$p-clippy.log" 2>&1; status $p-clippy $?
done
grep -E "^test result|Running" "$E/host/rustify-docs-test.log" "$E/host/rustify-docs-cli-test.log" > "$E/host/test-counts.txt"
mbx build --locked -p rustify-docs-cli --profile docs-cli > "$E/host/docs-cli-build.log" 2>&1; status docs-cli-build $?
target/docs-cli/rustify-docs --build-info > "$E/host/build-info.txt" 2>&1; status build-info $?

# V-1, V-5/V-6, V-10, V-11.
for group in features artifacts reliability toolchain; do
  mbx xtask docs-verify --group $group > "$E/$group.log" 2>&1; status $group $?
done

# V-7 (N7/AC-4) and V-9 in the browser, one project per command.
for project in docs-root docs-meta docs-dev; do
  RUSTIFY_DOCS_BIN=target/docs-cli/rustify-docs npx playwright test -c playwright.docs.config.ts --project=$project > "$E/browser/$project.log" 2>&1; status "browser-$project" $?
done
date '+done %F %T %z' >> "$E/summary.txt"
