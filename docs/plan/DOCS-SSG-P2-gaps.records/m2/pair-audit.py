"""Audits a prepared CLI C against its library R before C is committed.

pair-audit.py --cli <rustify-docs> --r <R> --previous-r <R5> --work <empty dir> --out <json>

Run from the repository root in the plan's 9.2 shell (Python 3.12, the
process-local Git remap, RUSTIFY_DOCS_CARGO=mbx). It checks:
- the template lock changes only the six framework packages' Git source to R;
- the root lock changes only the CLI's own version;
- a `new --template components` project, with that lock, fetches with
  --locked, then resolves and checks both targets --locked --offline without
  Makepad, and `cargo update --workspace --offline` changes nothing.
"""

import argparse, hashlib, json, subprocess, sys, tomllib
from pathlib import Path

FRAMEWORKS = ["rustify-components", "rustify-docs", "rustify-docs-macros", "rustify-docs-syntax", "rustify-style", "rustify-ui"]
URL = "git+https://github.com/rockie/rustify-ui?rev={0}#{0}"

args = argparse.ArgumentParser()
for name in ["--cli", "--r", "--previous-r", "--work", "--out"]:
    args.add_argument(name, required=True)
args = args.parse_args()
report = {"R": args.r, "previous_R": args.previous_r, "checks": {}, "commands": []}


def git(*command):
    return subprocess.run(["git", *command], check=True, capture_output=True, text=True).stdout


def packages(text):
    return {(package["name"], package["version"]): package for package in tomllib.loads(text)["package"]}


# Template lock: six framework sources move from the previous R to R, nothing else.
old = packages(git("show", f"HEAD:crates/rustify-docs-cli/assets/components-Cargo.lock"))
new_text = Path("crates/rustify-docs-cli/assets/components-Cargo.lock").read_text()
new = packages(new_text)
assert old.keys() == new.keys(), "the template lock's package set changed"
moved, third_party = [], 0
for key, before in old.items():
    after = new[key]
    if key[0] in FRAMEWORKS:
        assert before["source"] == URL.format(args.previous_r) and after["source"] == URL.format(args.r), key
        assert {k: v for k, v in before.items() if k != "source"} == {k: v for k, v in after.items() if k != "source"}, key
        moved.append(key[0])
    else:
        assert before == after, key
        third_party += 1
assert sorted(moved) == FRAMEWORKS, moved
report["checks"]["template_lock"] = {
    "packages": len(new), "framework_sources_moved": sorted(moved), "third_party_unchanged": third_party,
    "bytes": len(new_text.encode()), "sha256": hashlib.sha256(new_text.encode()).hexdigest(),
}

# Root lock: only the CLI's own version.
old_root, new_root = packages(git("show", "HEAD:Cargo.lock")), packages(Path("Cargo.lock").read_text())
version = tomllib.loads(Path("crates/rustify-docs-cli/Cargo.toml").read_text())["package"]["version"]
removed, added = sorted(set(old_root) - set(new_root)), sorted(set(new_root) - set(old_root))
assert [name for name, _ in removed] == ["rustify-docs-cli"] and added == [("rustify-docs-cli", version)], (removed, added)
assert all(old_root[key] == new_root[key] for key in old_root.keys() & new_root.keys())
assert old_root[removed[0]] | {"version": version} == new_root[added[0]]
report["checks"]["root_lock"] = {"cli": [removed[0][1], version], "other_packages_unchanged": len(old_root) - 1}
assert Path("crates/rustify-docs-cli/assets/framework-revision.txt").read_text().strip() == args.r

# A real new project from the prepared CLI, with the same lock.
work = Path(args.work)
assert work.is_dir() and not any(work.iterdir()), "--work must be an empty directory"
site = work / "docs-site"


def run(name, command, cwd=None):
    result = subprocess.run(command, cwd=cwd, capture_output=True, text=True)
    (work / f"{name}.log").write_text(result.stdout + result.stderr)
    report["commands"].append({"name": name, "command": command, "exit": result.returncode})
    if result.returncode != 0:
        Path(args.out).write_text(json.dumps(report, indent=2) + "\n")
        sys.exit(f"{name} failed:\n{result.stderr[-4000:]}")
    return result.stdout


run("build-info", [args.cli, "--build-info"])
run("new", [args.cli, "new", str(site), "--template", "components"])
manifest = str(site / "Cargo.toml")
lock = (site / "Cargo.lock").read_bytes()
generated = packages(lock.decode())
root = [key for key, package in generated.items() if "source" not in package]
assert len(root) == 1, root
assert {key: package for key, package in generated.items() if key not in root} == \
    {key: package for key, package in new.items() if "source" in package}, "new wrote a different lock"
assert f'rev = "{args.r}"' in (site / "Cargo.toml").read_text()

run("fetch", ["cargo", "fetch", "--locked", "--manifest-path", manifest])
graphs = {}
for target, features, extra in [("host", "ssr", []), ("wasm", "hydrate", ["--filter-platform", "wasm32-unknown-unknown"])]:
    metadata = json.loads(run(f"{target}-metadata", ["mbx", "metadata", "--locked", "--offline", "--format-version", "1",
                                                     "--manifest-path", manifest, "--no-default-features", "--features", features, *extra]))
    names = sorted({package["name"] for package in metadata["packages"]})
    makepad = [name for name in names if "makepad" in name.lower()]
    assert not makepad, makepad
    frameworks = {package["name"]: package["source"] for package in metadata["packages"] if package["name"] in FRAMEWORKS}
    graphs[target] = {"packages": len(names), "makepad": makepad, "framework_sources": frameworks}
    assert set(frameworks.values()) <= {URL.format(args.r)}, frameworks
run("host-check", ["mbx", "check", "--locked", "--offline", "--manifest-path", manifest, "--no-default-features",
                   "--features", "ssr", "--profile", "docs-host", "--bin", "docs-render"])
run("wasm-check", ["mbx", "check", "--locked", "--offline", "--manifest-path", manifest, "--no-default-features",
                   "--features", "hydrate", "--profile", "docs-wasm", "--lib", "--target", "wasm32-unknown-unknown"])
run("update-workspace", ["cargo", "update", "--workspace", "--offline", "--manifest-path", manifest])
assert (site / "Cargo.lock").read_bytes() == lock, "a --locked/--offline step or the workspace update changed the lock"
report["checks"]["template_project"] = {"graphs": graphs, "lock_unchanged": True, "root_package": root[0][0],
                                         "lock_sha256": hashlib.sha256(lock).hexdigest()}
report["passed"] = True
Path(args.out).write_text(json.dumps(report, indent=2) + "\n")
print(json.dumps({"passed": True, "template_lock": report["checks"]["template_lock"], "root_lock": report["checks"]["root_lock"],
                  "graphs": {t: {"packages": g["packages"], "makepad": g["makepad"]} for t, g in graphs.items()}}, indent=2))
