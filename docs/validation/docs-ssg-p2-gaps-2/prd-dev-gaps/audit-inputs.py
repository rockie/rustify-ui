"""Check source equivalence and the actual committed CLI/library pair."""
import hashlib
import json
from pathlib import Path
import re
import subprocess
import tomllib

out = Path(__file__).resolve().parent
repo = out.parents[3]
C = "b31e28b7f7dd949f7dc89ebc7b5f64380adbb63d"
R = "6942ec0d23812ecda61492d9db5c0046f481edfc"
previous_R = "e20daf7f07179d2bd24700c345f577fb9c8dbc15"


def git(*args):
    return subprocess.check_output(["git", *args], cwd=repo)


def source(rev, path):
    return git("show", rev + ":" + path)


def lock(rev, path):
    return tomllib.loads(source(rev, path).decode())["package"]


expected = ["Cargo.lock", "crates/rustify-docs-cli/Cargo.toml", "crates/rustify-docs-cli/assets/components-Cargo.lock", "crates/rustify-docs-cli/assets/framework-revision.txt", "docs/docs-ssg.md"]
files = git("diff-tree", "--no-commit-id", "--name-only", "-r", C).decode().splitlines()
assert files == expected
assert source(C, "crates/rustify-docs-cli/assets/framework-revision.txt").decode().strip() == R
assert tomllib.loads(source(C, "crates/rustify-docs-cli/Cargo.toml").decode())["package"]["version"] == "0.1.19"
template_path = "crates/rustify-docs-cli/assets/components-Cargo.lock"
old, new = lock(previous_R, template_path), lock(C, template_path)
changed = [(a, b) for a, b in zip(old, new) if a != b]
assert len(old) == len(new) == 193 and len(changed) == 6
for a, b in changed:
    assert {k: v for k, v in a.items() if k != "source"} == {k: v for k, v in b.items() if k != "source"}
    assert b["source"].endswith("?rev=" + R + "#" + R)
root_changes = [(a, b) for a, b in zip(lock(previous_R, "Cargo.lock"), lock(C, "Cargo.lock")) if a != b]
assert len(root_changes) == 1 and root_changes[0][1]["name"] == "rustify-docs-cli"
assert {k: v for k, v in root_changes[0][0].items() if k != "version"} == {k: v for k, v in root_changes[0][1].items() if k != "version"}
paths = git("ls-files", "crates/rustify-docs/src", "crates/rustify-docs/assets", "crates/rustify-docs-cli/src", "crates/rustify-docs-cli/tests/diagnostic_text.rs", "crates/rustify-docs-syntax/src", "tests/docs-ssg/dev-components.spec.ts").decode().splitlines()
drift = [p for p in paths if (repo / p).read_bytes() != source(C, p)]
assert not drift
producer_changes = [p for p in paths if ("/src/" in p or "/assets/" in p) and (repo / p).read_bytes() != source(previous_R, p)]
assert producer_changes == ["crates/rustify-docs/src/frontmatter.rs"]
head_product_diff = git("diff", "--name-only", C, "HEAD", "--", "crates", "site", "Cargo.toml", "Cargo.lock", "tests/docs-ssg/dev-components.spec.ts").decode().splitlines()
assert not head_product_diff
versions = {p: hashlib.sha256((repo / p).read_bytes()).hexdigest() for p in ["docs/PRD-DOCS-SSG.md", "docs/plan/DOCS-SSG-P2.md", "docs/plan/DOCS-SSG-P2-gaps.md", "docs/plan/DOCS-SSG-P2-gaps-2.md", "docs/plan/DOCS-SSG-P2.prd-dev-gaps.md", "docs/plan/DOCS-SSG-P2.prd-dev-gaps.r2.md", "docs/plan/DOCS-SSG-P2-gaps-2.records/M1.md", "docs/plan/DOCS-SSG-P2-gaps-2.records/M2.md", "docs/plan/DOCS-SSG-P2-gaps-2.code-review.md", "docs/plan/DOCS-SSG-P2-gaps-2.code-review.r2.md"]}
history = []
base = repo / "docs/validation/docs-ssg-p2-gaps-2/apply-r1"
for row in json.loads((base / "m1-checks.json").read_text()):
    log = base / (row["name"] + ".log")
    results = re.findall(r"test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored", log.read_text()) if log.exists() else []
    history.append({"name": row["name"], "exit": row["exit"], "raw_results": results})
audit = {"C": C, "R": R, "five_files": files, "template_records": len(new), "updated_sources": [b["name"] for a, b in changed], "unchanged_template_records": len(new) - len(changed), "root_lock_changes": root_changes, "current_source_files_equal_C": len(paths), "source_drift": drift, "source_paths": paths, "versions": versions, "historical_checks": history, "r_double_prime_to_current_producer_changes": producer_changes, "head_to_C_product_diff": head_product_diff}
(out / "input-audit.json").write_text(json.dumps(audit, ensure_ascii=False, indent=2) + "\n")
print(json.dumps({k: v for k, v in audit.items() if k not in {"source_paths", "versions", "historical_checks", "root_lock_changes"}}, ensure_ascii=False))
