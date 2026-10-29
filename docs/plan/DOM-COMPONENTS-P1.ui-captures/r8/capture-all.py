"""Replay the existing real-browser matrix, serially, into this round only."""
from pathlib import Path
import datetime
import json
import subprocess

out = Path(__file__).resolve().parent
scripts = [
    "capture-runner.cjs", "matrix-supplement.cjs", "matrix-followup.cjs",
    "matrix-preview.cjs", "matrix-zh-preview.cjs", "matrix-complete.cjs",
    "capture-u4-supplement.cjs", "capture-detail-supplement.cjs",
    "capture-u3-supplement.cjs",
]
executions = []
for name in scripts:
    started = datetime.datetime.now(datetime.timezone.utc).isoformat()
    with (out / (name + ".log")).open("w") as log:
        result = subprocess.run(["node", str(out / name)], stdout=log, stderr=subprocess.STDOUT)
    executions.append({"script": name, "started": started, "exit_code": result.returncode})
    (out / "capture-executions.json").write_text(json.dumps(executions, indent=2) + "\n")
    print(json.dumps(executions[-1]), flush=True)
    if result.returncode:
        raise SystemExit(result.returncode)
