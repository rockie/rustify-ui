#!/bin/sh
set -eu
cd "$(dirname "$0")/../.."
RUSTIFY_COMPONENTS_SSR_OUT="$PWD/tests/dom-components-ssr/target/site"
export RUSTIFY_COMPONENTS_SSR_OUT
mbx test --locked -p rustify-components --no-default-features --features ssr --test theme_layout
