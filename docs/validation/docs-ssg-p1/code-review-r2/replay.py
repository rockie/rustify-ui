"""Build the saved, synthetic review fixtures with target and optional base binaries."""
import argparse
import json
from pathlib import Path
import shutil
import subprocess
import tempfile

parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument("--binary", default="target/docs-cli/rustify-docs")
parser.add_argument("--baseline")
parser.add_argument("--work")
args = parser.parse_args()
source = Path(__file__).resolve().parent / "fixtures"
work = Path(args.work or tempfile.mkdtemp(prefix="docs-review-r2-replay-")).resolve()
work.mkdir(parents=True, exist_ok=True)
if any(work.iterdir()):
    parser.error("--work must be empty")
shutil.copytree(source, work / "boundaries")
versions = [("target", Path(args.binary).resolve(), "dist")]
if args.baseline:
    versions.append(("baseline", Path(args.baseline).resolve(), "baseline-dist"))
results = []
for root in sorted((work / "boundaries").iterdir()):
    for version, binary, out in versions:
        command = [str(binary), "build", "--config", str(root / "rustify-docs.toml"),
                   "--out", str(root / out), "--format", "json"]
        done = subprocess.run(command, capture_output=True, text=True)
        report = json.loads(done.stdout) if done.stdout else None
        page = root / out / "index.html"
        html = page.read_text() if page.exists() else ""
        item = {"case": root.name, "version": version, "command": command,
                "exit": done.returncode, "report": report, "stderr": done.stderr}
        if root.name == "unused-footnote":
            item.update(linked='href="#fnref-b"' in html, target_exists='id="fnref-b"' in html)
        results.append(item)
        print(json.dumps({"case": root.name, "version": version, "exit": done.returncode,
                          "rules": [d["rule"] for d in report["diagnostics"]
                                    if d["severity"] == "error"] if report else None}))
(work / "replay-results.json").write_text(json.dumps(results, indent=2) + "\n")
print(json.dumps({"work": str(work)}))
