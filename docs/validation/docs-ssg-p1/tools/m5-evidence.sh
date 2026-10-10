#!/bin/bash
# M5 exit evidence: the default reading theme. V-4's computed-style slice,
# V-7 and V-8 in the browser (preview at the root and under a base, a
# header-less static host), V-12's engine smoke (Playwright WebKit and
# Firefox; real Safari and phones are a person's walkthrough, recorded in
# m5/walkthrough.md), V-13's local checks, V-14's LCP/CLS and page weight on
# reference site B, and the docs packages' own suites.
set -u
M=/Users/rockie/Documents/gh-rockie/rustify-ui
E=$M/docs/validation/docs-ssg-p1/m5
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
mbx xtask docs-verify --group features > "$E/features.log" 2>&1; status features $?

# V-14: B's risk gate and real builds, then LCP/CLS on its largest home,
# content and 404 pages under the frozen conditions (§9.3).
mbx xtask docs-verify --group performance --report "$E/performance.json" > "$E/performance.log" 2>&1; status performance $?
target/docs-cli/rustify-docs preview --dir target/docs-ssg/perf/build-0 --port 4198 > "$E/host/preview-b.log" 2>&1 &
PREVIEW=$!
sleep 1
CONTENT=$(python3 -c "import json;print('/'+json.load(open('$E/performance.json'))['sizes']['largest_content']['path'].removesuffix('index.html'))")
node tests/docs-ssg/harness/measure-load.mjs --origin http://127.0.0.1:4198 --page / --page "$CONTENT" --page /no/such/page/ --samples 5 --out "$E/load.json" > "$E/load.log" 2>&1; status load $?
kill $PREVIEW

# V-7, V-8, V-12 (engines), V-13, V-4 computed style: one project per command.
for project in docs-root docs-subpath docs-meta docs-webkit docs-firefox; do
  npx playwright test -c playwright.docs.config.ts --project=$project > "$E/browser/$project.log" 2>&1; status "browser-$project" $?
done
date '+done %F %T %z' >> "$E/summary.txt"
