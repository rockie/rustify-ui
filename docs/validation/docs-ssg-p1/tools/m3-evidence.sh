#!/bin/bash
# M3 exit evidence: V-4's content group (the docs library's unit and
# end-to-end tests, the CLI's integration tests, `check` on both fixtures with
# the binary built the way it is installed), V-1's docs graphs with M3's new
# dependencies, V-14's risk gate re-run on reference site B (now required to
# analyse clean), format, sources, and the CLI graph's licences against M2's.
set -u
M=/Users/rockie/Documents/gh-rockie/rustify-ui
E=$M/docs/validation/docs-ssg-p1/m3
cd "$M" || exit 1
mkdir -p "$E/host" "$E/check"
status() { echo "$1 status=$2" | tee -a "$E/summary.txt"; }
: > "$E/summary.txt"
date '+started %F %T %z' >> "$E/summary.txt"

cargo fmt --all -- --check > "$E/host/fmt.log" 2>&1; status fmt $?
for p in rustify-docs rustify-docs-cli; do
  mbx test --locked -p $p > "$E/host/$p-test.log" 2>&1; status $p-test $?
  mbx clippy --locked -p $p --all-targets -- -D warnings > "$E/host/$p-clippy.log" 2>&1; status $p-clippy $?
done
grep -E "^test result|Running" "$E/host/rustify-docs-test.log" "$E/host/rustify-docs-cli-test.log" > "$E/host/test-counts.txt"
mbx test --locked -p xtask --bins > "$E/host/xtask-test.log" 2>&1; status xtask-test $?
mbx clippy --locked -p xtask --all-targets -- -D warnings > "$E/host/xtask-clippy.log" 2>&1; status xtask-clippy $?

# `check` from the binary as it is installed (panic=unwind, thin LTO).
mbx build --locked -p rustify-docs-cli --profile docs-cli > "$E/host/docs-cli-build.log" 2>&1; status docs-cli-build $?
BIN=target/docs-cli/rustify-docs
F=tests/docs-ssg/fixtures
$BIN check --config $F/three-errors/rustify-docs.toml --format json > "$E/check/three-errors.json" 2> "$E/check/three-errors.stderr"
echo "exit $?" > "$E/check/three-errors.exit"
$BIN check --config $F/three-errors/rustify-docs.toml > /dev/null 2> "$E/check/three-errors.txt"
$BIN check --config $F/content/rustify-docs.toml --format json --deny-warnings > "$E/check/content.json" 2> "$E/check/content.stderr"
echo "exit $?" > "$E/check/content.exit"
python3 - "$E/check" <<'PY' >> "$E/summary.txt"
import json, sys
d = sys.argv[1]
three = json.load(open(f"{d}/three-errors.json"))
found = [(x["rule"], x["file"], x["line"], x["column"], bool(x["suggestion"])) for x in three["diagnostics"]]
ok = (open(f"{d}/three-errors.exit").read().strip() == "exit 1" and three["success"] is False
      and found == [("broken-anchor", "content/index.md", 8, 5, True),
                    ("missing-image", "content/index.md", 10, 1, True),
                    ("inline-script", "content/index.md", 12, 1, True)])
print(f"check-three-errors status={0 if ok else 1} {found}")
content = json.load(open(f"{d}/content.json"))
ok = (open(f"{d}/content.exit").read().strip() == "exit 0" and content["success"] is True
      and content["diagnostics"] == [] and content["summary"]["pages"] == 4)
print(f"check-content status={0 if ok else 1} {content['summary']}")
PY

mbx xtask docs-verify --group features > "$E/features.log" 2>&1; status features $?
mbx xtask docs-verify --group performance --report "$E/performance.json" > "$E/performance.log" 2>&1; status performance $?
mbx xtask sources verify > "$E/host/sources.log" 2>&1; status sources $?

# Licences: the CLI's graph alone, each package marked when M2's list lacked it.
mbx tree --locked -p rustify-docs-cli --edges normal,build --target all --prefix none --format '{p}|{l}' > "$E/host/tree-licenses.txt" 2>/dev/null
python3 - "$E/host/tree-licenses.txt" docs/validation/docs-ssg-p1/m2/licenses.json "$E/licenses.json" <<'PY' >> "$E/summary.txt"
import json, re, sys
tree, m2_path, out = sys.argv[1:]
m2 = {(p["name"], p["version"]) for p in json.load(open(m2_path))["packages"]}
packages = {}
for line in open(tree):
    line = line.strip().removesuffix(" (*)")
    if not line or "|" not in line:
        continue
    spec, license = line.rsplit("|", 1)
    m = re.match(r"(\S+) v(\S+)(?: \((.+)\))?", spec)
    name, version, source = m.group(1), m.group(2), m.group(3) or "crates.io"
    if source.startswith("/"):
        source = "path"
    packages[(name, version)] = {"name": name, "version": version, "license": license or None,
                                 "source": source, "new_since_m2": (name, version) not in m2}
listed = sorted(packages.values(), key=lambda p: (p["name"], p["version"]))
json.dump({"package": "rustify-docs-cli",
           "edges": "normal and build, every target, resolved for this package alone (cargo tree -p)",
           "packages": listed}, open(out, "w"), indent=1, ensure_ascii=False)
new = [f'{p["name"]} {p["version"]} ({p["license"]})' for p in listed if p["new_since_m2"]]
print(f"licenses packages={len(listed)} new_since_m2={len(new)}: " + "; ".join(new))
PY
date '+done %F %T %z' >> "$E/summary.txt"
