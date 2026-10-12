"""Build the fixed production inputs for real-browser acceptance."""

import hashlib
import json
import shutil
import subprocess
from pathlib import Path

repo = Path(__file__).resolve().parents[4]
binary = repo / "target/docs-ssg/gaps/rustify-docs-candidate"
work = repo / "target/docs-ssg/gaps/matrix"
entries = []
for name, fixture, base in [
    ("full-root", "content", "/"),
    ("full-subpath", "content", "/rustify-ui/"),
    ("home-root", "home", "/"),
    ("home-subpath", "home", "/rustify-ui/"),
]:
    site = work / name / "site"
    out = work / name / "dist"
    if site.exists():
        raise SystemExit(f"{site} already exists; reuse the recorded build")
    shutil.copytree(repo / "tests/docs-ssg/fixtures" / fixture, site)
    config = site / "rustify-docs.toml"
    config.write_text(config.read_text().replace("/docs/", base))
    subprocess.run([str(binary), "build", "--config", str(config), "--out", str(out)], check=True)
    manifest = out / "rustify-docs-manifest.json"
    data = json.loads(manifest.read_text())
    entries.append({"name": name, "out": str(out.relative_to(repo)), "base": base,
                    "build_id": data["build_id"], "cli_version": data["cli_version"],
                    "manifest_sha256": hashlib.sha256(manifest.read_bytes()).hexdigest()})
report = {"binary_sha256": hashlib.sha256(binary.read_bytes()).hexdigest(), "sites": entries}
(Path(__file__).parent / "matrix-artifacts.json").write_text(json.dumps(report, indent=2) + "\n")
print(json.dumps(report, indent=2))
