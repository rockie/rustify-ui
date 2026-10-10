#!/bin/bash
# Pre-change regression baseline for docs/plan/DOCS-SSG-P1.md (V-3), at 91e81b8.
set -u
B=/Users/rockie/Documents/gh-rockie/rustify-ui-docs-ssg-baseline-91e81b8
M=/Users/rockie/Documents/gh-rockie/rustify-ui
OUT=$M/docs/validation/docs-ssg-p1/baseline
LOG=$B/.baseline-logs
mkdir -p "$OUT" "$LOG"
cd "$B" || exit 1
ln -sfn "$M/node_modules" node_modules
ln -sfn "$M/ref" ref
ln -sfn "$M/tests/utility/compiler/oracle/node_modules" tests/utility/compiler/oracle/node_modules

step() { echo "=== $(date '+%F %T %z') $*" | tee -a "$LOG/steps.log"; }
run() { local name=$1; shift; step "start $name"; "$@" > "$LOG/$name.log" 2>&1; local s=$?; step "end $name status=$s"; return $s; }
pw() {
  local p=$1
  step "start pw $p"
  npx playwright test --project="$p" > "$LOG/pw-$p.log" 2>&1
  local s=$?
  cp test-results/browser.json "$OUT/$p.json" 2>/dev/null
  step "end pw $p status=$s"
}

if [ "${SKIP_BUILD:-0}" != 1 ]; then
  run host-theme-export mbx test --locked --manifest-path tests/theme-export/Cargo.toml --workspace --bins
  for ex in fusion-basic property-workbench component-catalog data-workbench vellum theme-studio utility-lab; do
    run "build-$ex" mbx xtask build-web --example "$ex" --release
  done
  for ex in fusion-basic property-workbench theme-studio; do
    run "build-$ex-demo" mbx xtask build-web --example "$ex" --release --base /tools/demo/
  done
  run build-utility-lab-deep mbx xtask build-web --example utility-lab --release --base /tools/utility-lab/
  run fetch-theme-export mbx fetch --locked --manifest-path tests/theme-export/Cargo.toml
  run build-theme-export-root sh tests/theme-export/build.sh
  run build-theme-export-sub sh tests/theme-export/build.sh --base /tools/theme-export/ --out tests/theme-export/target/site-subpath
fi

for p in ${PROJECTS:-component-catalog data-workbench deployment workbench-deep theme-studio theme-studio-deep theme-export theme-export-deep utility-lab utility-lab-deep}; do
  pw "$p"
done
if [ "${SKIP_SCANNER:-0}" != 1 ]; then
  run build-utility-lab-scanner mbx xtask build-web --example utility-lab --release --utility-scanner
  run build-utility-lab-scanner-deep mbx xtask build-web --example utility-lab --release --base /tools/utility-lab/ --utility-scanner
  pw utility-lab-scanner
  pw utility-lab-scanner-deep
fi
if [ "${SKIP_VELLUM:-0}" != 1 ]; then
  step "start pw vellum"
  npx playwright test -c tests/vellum/playwright.config.ts > "$LOG/pw-vellum.log" 2>&1
  s=$?
  cp test-results/vellum/suite-results.json "$OUT/vellum.json" 2>/dev/null
  step "end pw vellum status=$s"
fi
for p in ${LONG_PROJECTS:-property-workbench fusion-basic}; do
  pw "$p"
done
step "baseline done"
