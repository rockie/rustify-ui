"""Check component errors after a successful generation of the same site."""

from pathlib import Path
import json
import os
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[4]
OUT = Path(__file__).resolve().parent
STATE = json.loads((ROOT / "target/docs-ssg/m6-installed-c.json").read_text())
CLI = STATE["binary"]
ENV = dict(os.environ, CARGO_NET_OFFLINE="true", RUSTIFY_DOCS_CARGO="mbx")
ENV.update(STATE["transport_environment"])
results = []


def check(site, name):
    command = [CLI, "check", "--config", str(site / "rustify-docs.toml"), "--format", "json"]
    result = subprocess.run(command, cwd=ROOT, env=ENV, capture_output=True, text=True)
    (OUT / f"component-{name}.json").write_text(result.stdout)
    (OUT / f"component-{name}.stderr").write_text(result.stderr)
    report = json.loads(result.stdout)
    results.append({"name": name, "exit_code": result.returncode, "summary": report["summary"], "errors": [d for d in report["diagnostics"] if d["severity"] == "error"]})
    return result.returncode, report


with tempfile.TemporaryDirectory(prefix="rustify-prd-gaps-warm-") as scratch:
    site = Path(scratch) / "components-site"
    prepared = subprocess.run(["python3.12", "scripts/docs-pages-prepare.py", "--project", str(ROOT / "tests/docs-ssg/fixtures/components/Cargo.toml"), "--cli", CLI, "--out", str(site)], cwd=ROOT, env=ENV, capture_output=True, text=True)
    assert prepared.returncode == 0, prepared.stderr
    status, report = check(site, "warm-1")
    if status != 0:
        assert all(d["rule"] == "rust-generation-changed" for d in results[-1]["errors"])
        status, report = check(site, "warm-2")
    assert status == 0, report
    lock = (site / "Cargo.lock").read_bytes()
    header = "---\ntitle: Diagnostic probe\ndescription: Isolated error cases.\n---\n\n"
    (site / "content/gaps.md").write_text(header + "```rustify-component\nname: MissingCounter\n```\n\n```rustify-component\nname: Counter\nprops:\n  unexpected: 1\n```\n")
    status, report = check(site, "names")
    assert status == 1 and {d["rule"] for d in results[-1]["errors"]} == {"unregistered-component", "unknown-property"}
    (site / "content/gaps.md").write_text(header + "```rustify-component\nname: Counter\nprops:\n  initial: wrong-type\n```\n")
    status, report = check(site, "type")
    assert status == 1 and [d["rule"] for d in results[-1]["errors"]] == ["invalid_props"]
    assert (site / "Cargo.lock").read_bytes() == lock
(OUT / "component-probe.json").write_text(json.dumps({"C": STATE["candidate_sha"], "R": STATE["framework_revision"], "lock_unchanged": True, "results": results}, ensure_ascii=False, indent=2) + "\n")
print(json.dumps(results, ensure_ascii=False, indent=2))
