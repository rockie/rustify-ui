#!/bin/sh
set -eu

export_fixture_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
export_fixture_root=$(CDPATH= cd -- "$export_fixture_dir/../.." && pwd)
export_fixture_site="$export_fixture_dir/target/site-root"
export_fixture_base="/"
export_fixture_css_only=0
while [ "$#" -gt 0 ]; do
  case "$1" in
    --out) export_fixture_site=$2; shift 2 ;;
    --base) export_fixture_base=$2; shift 2 ;;
    --css-only) export_fixture_css_only=1; shift ;;
    *) echo "Usage: sh tests/theme-export/build.sh [--css-only] [--out directory] [--base /tools/demo/]" >&2; exit 2 ;;
  esac
done
case "$export_fixture_base" in /*/) ;; /) ;; *) echo "Use an absolute base with a trailing slash." >&2; exit 2 ;; esac
mkdir -p "$export_fixture_site"
export_fixture_site=$(CDPATH= cd -- "$export_fixture_site" && pwd)
export_fixture_tw_version=$(tailwindcss --help)
case "$export_fixture_tw_version" in *"tailwindcss v4.1.13"*) ;; *) echo "Tailwind 4.1.13 is required; use the repository mise toolchain." >&2; exit 1 ;; esac

cd "$export_fixture_dir"
mbx build --quiet --locked --offline --manifest-path "$export_fixture_dir/Cargo.toml" -p theme-export-generator --bin theme-export-serve
mbx run --quiet --locked --offline --manifest-path "$export_fixture_dir/Cargo.toml" -p theme-export-generator --bin theme-export-generator -- generate "$export_fixture_site"
for export_fixture_profile in standard scoped; do
  for export_fixture_format in hex rgb hsl oklch; do
    tailwindcss --cwd "$export_fixture_dir" -i "$export_fixture_site/inputs/$export_fixture_profile-$export_fixture_format.css" -o "$export_fixture_site/$export_fixture_profile-$export_fixture_format.css"
  done
done
tailwindcss --cwd "$export_fixture_dir" -i "$export_fixture_site/inputs/runtime-theme.css" -o "$export_fixture_site/runtime-theme.css"
if [ "$export_fixture_css_only" -eq 0 ]; then
  RUSTFLAGS="${RUSTFLAGS-} --cfg=web_sys_unstable_apis" mbx run --quiet --locked --offline --release --manifest-path "$export_fixture_root/makepad/tools/cargo_makepad/Cargo.toml" -- wasm build --locked --offline --release -p theme_export_consumer
  mbx run --quiet --locked --offline --manifest-path "$export_fixture_dir/Cargo.toml" -p theme-export-generator --bin theme-export-generator -- package "$export_fixture_site" "$export_fixture_dir/target/makepad-wasm-app/release/theme_export_consumer" "$export_fixture_base"
fi
echo "Static directory: $export_fixture_site"
echo "Deployment base: $export_fixture_base"
