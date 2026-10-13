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
if [ "$source" = official ] && [ -f "$work/site/.git" ]; then
  git -C "$root" worktree remove --force "$work/site"
fi
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
    # Empty-R development keeps the original exact paths. A paired C uses
    # a copied consumer with only its three Git dependencies and lock projected.
    fixture=$(python3 "$root/scripts/docs-pages-prepare.py" --project "$fixture/Cargo.toml" \
      --cli "$bin" --out "$work/site" | python3 -c 'import json,sys; print(json.load(sys.stdin)["project"])')
    config=$(mktemp "$fixture/.rustify-docs-browser.XXXXXX")
    trap 'rm -f "$config"' EXIT
    perl -0pe "s#url = \"https://example.com/docs/\"\nbase = \"/docs/\"#url = \"https://docs.example.net$base\"\nbase = \"$base\"#" "$fixture/rustify-docs.toml" > "$config"
    RUSTIFY_DOCS_CARGO=mbx "$bin" build --config "$config" --out "$work/dist" >&2
    rm -f "$config"
    trap - EXIT
    ;;
  official)
    # A paired CLI consumes detached C sources and genuine history; only the
    # manifest dependencies and lock differ. Empty-R development stays in place.
    official=$root
    pair=$(python3 - "$root/scripts/docs-pages-prepare.py" "$bin" <<'PY'
from pathlib import Path
import json, runpy, sys
candidate, framework, _ = runpy.run_path(sys.argv[1])['cli_pair'](Path(sys.argv[2]))
print(json.dumps({'candidate': candidate, 'framework': framework}))
PY
)
    framework=$(printf '%s' "$pair" | python3 -c 'import json,sys; print(json.load(sys.stdin)["framework"])')
    if [ -n "$framework" ]; then
      candidate=$(printf '%s' "$pair" | python3 -c 'import json,sys; print(json.load(sys.stdin)["candidate"])')
      python3 "$root/scripts/docs-pages-prepare.py" --candidate-sha "$candidate" \
        --cli "$bin" --out "$work/site" >&2
      official=$work/site
    fi
    config=$(mktemp "$official/.rustify-docs-browser.XXXXXX")
    trap 'rm -f "$config"' EXIT
    python3 - "$official/rustify-docs.toml" "$config" "$base" <<'PY'
from pathlib import Path
import sys
source, destination, base = sys.argv[1:]
text = Path(source).read_text()
old_url = 'url = "https://rockie.github.io/rustify-ui/"'
old_base = 'base = "/rustify-ui/"'
assert text.count(old_url) == text.count(old_base) == 1
text = text.replace(old_url, 'url = "https://rockie.github.io' + base + '"')
text = text.replace(old_base, 'base = "' + base + '"')
Path(destination).write_text(text)
PY
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
