#!/bin/sh
set -eu
text_tests_dir=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
export RUSTIFY_TEXT_MANIFEST_DIR="$text_tests_dir/../../makepad/draw"
exec mbx test --manifest-path "$text_tests_dir/Cargo.toml" "$@"
