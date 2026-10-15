"""Probe the installed C5 CLI on isolated content and component sites."""

from pathlib import Path
import hashlib
import json
import os
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[4]
OUT = Path(__file__).resolve().parent
IDENTITY = json.loads((ROOT / "target/docs-ssg/m6-installed-c.json").read_text())
CLI = Path(IDENTITY["binary"])
ENV = dict(os.environ, CARGO_NET_OFFLINE="true", RUSTIFY_DOCS_CARGO="mbx")
ENV.update(IDENTITY["transport_environment"])
summary = {"cli": IDENTITY["candidate_sha"], "framework": IDENTITY["framework_revision"], "cases": []}


def run(command, name):
    result = subprocess.run(command, cwd=ROOT, env=ENV, capture_output=True, text=True)
    (OUT / f"{name}.stdout").write_text(result.stdout)
    (OUT / f"{name}.stderr").write_text(result.stderr)
    return result


assert hashlib.sha256(CLI.read_bytes()).hexdigest() == IDENTITY["binary_sha256"]
with tempfile.TemporaryDirectory(prefix="rustify-prd-gaps-") as scratch:
    content = Path(scratch) / "content-site"
    assert run([str(CLI), "new", str(content)], "content-new").returncode == 0
    (content / "content/gaps.md").write_text(
        "---\ntitle: Diagnostic probe\ndescription: Isolated error cases.\n---\n\n"
        "```rustify-component\nname: MissingCounter\n```\n\n"
        "```rustify-component\nname: Steps\nprops:\n  items: wrong-type\n```\n"
    )
    result = run([str(CLI), "check", "--config", str(content / "rustify-docs.toml"), "--format", "json"], "content-diagnostics")
    data = json.loads(result.stdout)
    assert result.returncode == 1
    summary["cases"].append({"case": "content", "exit_code": result.returncode, "diagnostics": data["diagnostics"]})
    component = Path(scratch) / "components-site"
    prepared = run([
        "python3.12", "scripts/docs-pages-prepare.py", "--project",
        str(ROOT / "tests/docs-ssg/fixtures/components/Cargo.toml"),
        "--cli", str(CLI), "--out", str(component),
    ], "component-prepare")
    assert prepared.returncode == 0, prepared.stderr
    (component / "content/gaps.md").write_text(
        "---\ntitle: Diagnostic probe\ndescription: Isolated error cases.\n---\n\n"
        "```rustify-component\nname: MissingCounter\n```\n\n"
        "```rustify-component\nname: Counter\nprops:\n  initial: wrong-type\n```\n\n"
        "```rustify-component\nname: Counter\nprops:\n  unexpected: 1\n```\n"
    )
    lock_before = (component / "Cargo.lock").read_bytes()
    result = run([str(CLI), "check", "--config", str(component / "rustify-docs.toml"), "--format", "json"], "component-diagnostics")
    data = json.loads(result.stdout)
    assert result.returncode == 1
    assert (component / "Cargo.lock").read_bytes() == lock_before
    summary["cases"].append({"case": "components", "exit_code": result.returncode, "lock_unchanged": True, "diagnostics": data["diagnostics"]})
(OUT / "diagnostic-probe.json").write_text(json.dumps(summary, ensure_ascii=False, indent=2) + "\n")
print(json.dumps(summary, ensure_ascii=False, indent=2))
