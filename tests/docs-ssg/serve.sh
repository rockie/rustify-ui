#!/bin/sh
# Builds the content fixture for one docs browser project and serves it:
#   serve.sh root <port>     base /, `rustify-docs preview` (CSP as a header)
#   serve.sh subpath <port>  base /rustify-ui/, `rustify-docs preview`
#   serve.sh meta <port>     base /rustify-ui/, a plain static host with no
#                            response headers of its own (the policy only in
#                            each page's <meta>) and no rewrite rules
# The CLI is the binary `--profile docs-cli` builds, as installed, rebuilt
# here when its sources changed; RUSTIFY_DOCS_BIN points at another.
set -eu
if [ "$#" -eq 2 ]; then
  source=content
  variant=$1
  port=$2
else
  source=$1
  variant=$2
  port=$3
fi
root=$(cd "$(dirname "$0")/../.." && pwd)
if [ -n "${RUSTIFY_DOCS_BIN:-}" ]; then
  bin=$RUSTIFY_DOCS_BIN
else
  # Brought up to date every time, so a run never tests a stale binary.
  (cd "$root" && CARGO_TARGET_DIR="$root/target" mbx build --locked --profile docs-cli -p rustify-docs-cli >&2)
  bin=$root/target/docs-cli/rustify-docs
fi
case $variant in
  root) base=/ ;;
  subpath | meta) base=/rustify-ui/ ;;
  *) echo "serve.sh: unknown variant $variant" >&2; exit 2 ;;
esac
work=$root/target/docs-ssg/http/$source/$variant
rm -rf "$work"
mkdir -p "$work"
case $source in
  content)
    cp -R "$root/tests/docs-ssg/fixtures/content" "$work/site"
    perl -0pi -e "s#url = \"https://example.com/docs/\"\nbase = \"/docs/\"#url = \"https://docs.example.net$base\"\nbase = \"$base\"#" "$work/site/rustify-docs.toml"
    rg -q "base = \"$base\"" "$work/site/rustify-docs.toml"
    "$bin" build --config "$work/site/rustify-docs.toml" --out "$work/dist" >&2
    ;;
  components)
    fixture=$root/tests/docs-ssg/fixtures/components
    # Keep the author package at its original path. Its relative Cargo
    # dependencies and source excerpts must match the files compiled.
    config=$(mktemp "$fixture/.rustify-docs-browser.XXXXXX")
    trap 'rm -f "$config"' EXIT
    perl -0pe "s#url = \"https://example.com/docs/\"\nbase = \"/docs/\"#url = \"https://docs.example.net$base\"\nbase = \"$base\"#" "$fixture/rustify-docs.toml" > "$config"
    RUSTIFY_DOCS_CARGO=mbx "$bin" build --config "$config" --out "$work/dist" >&2
    rm -f "$config"
    trap - EXIT
    ;;
  *) echo "serve.sh: unknown source $source" >&2; exit 2 ;;
esac
if [ "$variant" = meta ]; then
  exec node "$root/tests/docs-ssg/static-host.mjs" "$work/dist" "$port" "$base"
fi
exec "$bin" preview --dir "$work/dist" --port "$port"
