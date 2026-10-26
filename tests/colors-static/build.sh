#!/bin/sh
set -eu
cd "$(dirname "$0")/../.."
colors_site=tests/colors-static/target/site
mkdir -p "$colors_site/colors"
cp crates/rustify-colors/css/*.css "$colors_site/colors/"
cp tests/colors-static/index.html tests/colors-static/fixture.css "$colors_site/"
# The enhancement is an explicit alternate page, not a runtime appearance switch.
sed 's@colors/\([^"]*\)\.css@colors/\1-p3.css@g' tests/colors-static/index.html > "$colors_site/p3.html"
