#!/bin/bash
# M8 exit evidence, the parts a machine can run: V-14's formal measurements
# on reference site B under the frozen conditions (§9.3) — build and page
# weight, mobile LCP/CLS, saves visible in the browser (5 each, median), a
# short round and the 1,000-save run on one `dev` — the §9.4 UAT chains in
# the browser (new→dev→pages, saves and a mistake in one session), V-2's
# DOM-only regression, V-7–V-9/V-13 projects, V-10/V-11 again, and V-15 for
# the commit point. Real Safari, phones and stable Firefox are a person's
# walkthrough and are recorded separately.
set -u
M=/Users/rockie/Documents/gh-rockie/rustify-ui
E=$M/docs/validation/docs-ssg-p1/m8
cd "$M" || exit 1
mkdir -p "$E/host" "$E/browser"
status() { echo "$1 status=$2" | tee -a "$E/summary.txt"; }
: > "$E/summary.txt"
date '+started %F %T %z' >> "$E/summary.txt"

# V-15 for the commit point.
cargo fmt --all -- --check > "$E/host/fmt.log" 2>&1; status fmt $?
mbx xtask sources verify > "$E/host/sources.log" 2>&1; status sources-verify $?
mbx test --locked --workspace --exclude rustify-docs --exclude rustify-docs-cli --lib --bins > "$E/host/workspace-test.log" 2>&1; status workspace-test $?
mbx clippy --locked --workspace --exclude rustify-docs --exclude rustify-docs-cli --all-targets -- -D warnings > "$E/host/workspace-clippy.log" 2>&1; status workspace-clippy $?
for p in rustify-ui rustify-components; do
  for mode in default gpu ssr; do
    case $mode in
      default) flags="" ;;
      gpu) flags="--features gpu" ;;
      ssr) flags="--no-default-features --features ssr" ;;
    esac
    mbx test --locked -p $p $flags --lib > "$E/host/$p-$mode-test.log" 2>&1; status $p-$mode-test $?
    mbx clippy --locked -p $p $flags --all-targets -- -D warnings > "$E/host/$p-$mode-clippy.log" 2>&1; status $p-$mode-clippy $?
  done
done
for p in rustify-docs rustify-docs-cli; do
  mbx test --locked -p $p > "$E/host/$p-test.log" 2>&1; status $p-test $?
  mbx clippy --locked -p $p --all-targets -- -D warnings > "$E/host/$p-clippy.log" 2>&1; status $p-clippy $?
done
mbx build --locked -p rustify-docs-cli --profile docs-cli > "$E/host/docs-cli-build.log" 2>&1; status docs-cli-build $?
target/docs-cli/rustify-docs --build-info > "$E/host/build-info.txt" 2>&1; status build-info $?

# V-1, V-5/V-6, V-10, V-11 (the last installs from a commit and walks an
# author's environment, offline).
for group in features artifacts reliability toolchain; do
  mbx xtask docs-verify --group $group > "$E/$group.log" 2>&1; status $group $?
done
INSTALLED=${TMPDIR:-/tmp}/rustify-docs-verify/cli-install/bin/rustify-docs
INSTALLED=$(ls "$INSTALLED" 2>/dev/null || echo "$M/target/docs-cli/rustify-docs")
echo "binary for the dev chains: $INSTALLED" >> "$E/summary.txt"

# V-14: B, its build, page weight and the risk-gate timings; then the
# largest home, content and 404 pages on a phone under Slow 4G, five cold
# loads each.
mbx xtask docs-verify --group performance --report "$E/performance.json" > "$E/performance.log" 2>&1; status performance $?
target/docs-cli/rustify-docs preview --dir target/docs-ssg/perf/build-0 --port 4198 > "$E/host/preview-b.log" 2>&1 &
PREVIEW=$!
sleep 1
CONTENT=$(python3 -c "import json;print('/'+json.load(open('$E/performance.json'))['sizes']['largest_content']['path'].removesuffix('index.html'))")
node tests/docs-ssg/harness/measure-load.mjs --origin http://127.0.0.1:4198 --page / --page "$CONTENT" --page /no/such/page/ --samples 5 --out "$E/load.json" > "$E/load.log" 2>&1; status load $?
kill $PREVIEW

# V-14: saves on B in one `dev` (the installed binary when there is one):
# body, configuration and logo five times each, 20 in a row, then 1,000.
node tests/docs-ssg/harness/measure-save.mjs --bin "$INSTALLED" --site target/docs-ssg/perf/b --page "$CONTENT" --samples 5 --short 20 --long 1000 --out "$E/save.json" > "$E/save.log" 2>&1; status save-1000 $?

# The browser projects, one at a time: V-2, V-7/V-8/V-13 at the root and
# header-less, V-9 and the UAT-1/UAT-2 chain with the same binary.
npx playwright test -c playwright.docs.config.ts --project=dom-only > "$E/browser/dom-only.log" 2>&1; status browser-dom-only $?
for project in docs-root docs-meta; do
  RUSTIFY_DOCS_BIN=target/docs-cli/rustify-docs npx playwright test -c playwright.docs.config.ts --project=$project > "$E/browser/$project.log" 2>&1; status "browser-$project" $?
done
RUSTIFY_DOCS_BIN="$INSTALLED" npx playwright test -c playwright.docs.config.ts --project=docs-dev > "$E/browser/docs-dev.log" 2>&1; status browser-docs-dev $?

# UAT-10 offline: the three known errors, as JSON, with no network at all.
sandbox-exec -p '(version 1)(allow default)(deny network*)' target/docs-cli/rustify-docs check --config tests/docs-ssg/fixtures/three-errors/rustify-docs.toml --format json > "$E/host/uat10.json" 2> "$E/host/uat10.err"
code=$?
python3 -c "
import json,sys
d=json.load(open('$E/host/uat10.json'))
errors=[x for x in d['diagnostics'] if x['severity']=='error']
ok = $code==1 and len(errors)==3 and all(x['file'] and x['line'] and x['column'] and x['suggestion'] for x in errors)
print('uat10 errors', len(errors), [x['rule'] for x in errors]); sys.exit(0 if ok else 1)
" >> "$E/summary.txt"; status uat10-offline $?
date '+done %F %T %z' >> "$E/summary.txt"
