#!/bin/bash
# M2 exit evidence: the V-15 matrix with the docs packages on their own, V-1's
# docs graphs, V-14's risk gate on reference site B, the CLI install
# rehearsal, and V-3's "affected" check: every example's release product built
# at M1's commit and in this tree, byte for byte.
set -u
M=/Users/rockie/Documents/gh-rockie/rustify-ui
E=$M/docs/validation/docs-ssg-p1/m2
BASE_COMMIT=179c5ec
# Same path length as this tree, so a path a product embeds can be swapped
# for this tree's without moving a byte.
BASE_TREE=/Users/rockie/Documents/gh-rockie/rustify-u1
cd "$M" || exit 1
mkdir -p "$E/host" "$E/products"
status() { echo "$1 status=$2" | tee -a "$E/summary.txt"; }
: > "$E/summary.txt"
date '+started %F %T %z' >> "$E/summary.txt"

cargo fmt --all -- --check > "$E/host/fmt.log" 2>&1; status fmt $?
(cd tests/dom-only && cargo fmt --all -- --check) > "$E/host/fmt-dom-only.log" 2>&1; status fmt-dom-only $?
EXCLUDE="--exclude rustify-docs --exclude rustify-docs-cli"
mbx test --locked --workspace $EXCLUDE --lib --bins > "$E/host/workspace-test.log" 2>&1; status workspace-test $?
mbx clippy --locked --workspace $EXCLUDE --all-targets -- -D warnings > "$E/host/workspace-clippy.log" 2>&1; status workspace-clippy $?
for p in rustify-ui rustify-components; do
  mbx test --locked -p $p --lib > "$E/host/$p-default-test.log" 2>&1; status $p-default-test $?
  mbx test --locked -p $p --features gpu --lib > "$E/host/$p-gpu-test.log" 2>&1; status $p-gpu-test $?
  mbx test --locked -p $p --no-default-features --features ssr --lib > "$E/host/$p-ssr-test.log" 2>&1; status $p-ssr-test $?
  mbx clippy --locked -p $p --all-targets -- -D warnings > "$E/host/$p-default-clippy.log" 2>&1; status $p-default-clippy $?
  mbx clippy --locked -p $p --features gpu --all-targets -- -D warnings > "$E/host/$p-gpu-clippy.log" 2>&1; status $p-gpu-clippy $?
  mbx clippy --locked -p $p --no-default-features --features ssr --all-targets -- -D warnings > "$E/host/$p-ssr-clippy.log" 2>&1; status $p-ssr-clippy $?
done
for p in rustify-docs rustify-docs-cli; do
  mbx test --locked -p $p > "$E/host/$p-test.log" 2>&1; status $p-test $?
  mbx clippy --locked -p $p --all-targets -- -D warnings > "$E/host/$p-clippy.log" 2>&1; status $p-clippy $?
done
mbx build --locked -p rustify-docs-cli --profile docs-cli > "$E/host/docs-cli-build.log" 2>&1; status docs-cli-build $?
target/docs-cli/rustify-docs --build-info > "$E/host/docs-cli-build-info.txt" 2>&1; status docs-cli-build-info $?
mbx test --locked -p rustify-ui -p rustify-components --features rustify-components/utility --lib > "$E/host/sdk-utility-test.log" 2>&1; status sdk-utility-test $?
mbx test --locked --manifest-path tests/theme-export/Cargo.toml --workspace --bins > "$E/host/theme-export-test.log" 2>&1; status theme-export-test $?
mbx xtask sources verify > "$E/host/sources.log" 2>&1; status sources $?
mbx xtask css --check > "$E/host/css.log" 2>&1; status css $?
mbx xtask catalog --write docs/components.md --check > "$E/host/catalog.log" 2>&1; status catalog $?

mbx xtask docs-verify --group features > "$E/features.log" 2>&1; status features $?
mbx xtask docs-verify --group performance --report "$E/performance.json" > "$E/performance.log" 2>&1; status performance $?
mbx xtask docs-verify --group toolchain > "$E/toolchain.log" 2>&1; status toolchain $?

# V-3: the examples' release products at M1's commit and here.
if [ ! -d "$BASE_TREE" ]; then
  git worktree add --detach "$BASE_TREE" "$BASE_COMMIT" > "$E/products/worktree.log" 2>&1
fi
EXAMPLES="fusion-basic property-workbench component-catalog data-workbench vellum theme-studio utility-lab"
hash_products() {
  local root=$1 out=$2
  local copy=$E/products/.copy
  rm -rf "$copy"; mkdir -p "$copy"
  : > "$out"
  for ex in $EXAMPLES; do
    cp -R "$root/target/makepad-wasm-app/release/$ex" "$copy/$ex"
  done
  # The worktree's own path, wherever a product carries it, becomes this tree's.
  find "$copy" -type f -exec perl -0777 -pi -e 's#/gh-rockie/rustify-u1/#/gh-rockie/rustify-ui/#g' {} \;
  (cd "$copy" && find . -type f ! -name build-manifest.json -exec shasum -a 256 {} \; | sed "s|  ./|  |" | sort -k2) > "$out"
  rm -rf "$copy"
}
build_all() {
  local root=$1 log=$2
  local s=0
  for ex in $EXAMPLES; do
    (cd "$root" && mbx xtask build-web --example "$ex" --release) >> "$log" 2>&1 || s=1
  done
  return $s
}
build_all "$BASE_TREE" "$E/products/build-$BASE_COMMIT.log"; status "products-build-$BASE_COMMIT" $?
hash_products "$BASE_TREE" "$E/products/$BASE_COMMIT.sha256"
build_all "$M" "$E/products/build-candidate.log"; status products-build-candidate $?
hash_products "$M" "$E/products/candidate.sha256"
diff "$E/products/$BASE_COMMIT.sha256" "$E/products/candidate.sha256" > "$E/products/diff.txt"; status products-identical $?
wc -l < "$E/products/candidate.sha256" | sed 's/^/product files compared: /' >> "$E/summary.txt"
date '+done %F %T %z' >> "$E/summary.txt"
