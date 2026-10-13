"""Diagnostic only: compare fresh immutable copies and an already run helper."""
import hashlib
import json
import pathlib
import shutil
import subprocess
import sys
import tempfile
import time

helper = pathlib.Path(sys.argv[1]).resolve()
site = pathlib.Path(sys.argv[2]).resolve()
report = pathlib.Path(sys.argv[3])
request = json.dumps({"protocol_version": 1, "generation": "startup-diagnostic", "operation": {"type": "describe"}}).encode()


def measure(path, kind, sample):
    started = time.perf_counter_ns()
    process = subprocess.Popen([str(path)], cwd=site, stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
    spawned = time.perf_counter_ns()
    process.stdin.write(request)
    process.stdin.close()
    first = process.stdout.read(1)
    first_byte = time.perf_counter_ns()
    response = first + process.stdout.read()
    stderr = process.stderr.read()
    status = process.wait()
    exited = time.perf_counter_ns()
    value = json.loads(response)
    parsed = time.perf_counter_ns()
    assert status == 0 and not stderr, (status, stderr)
    assert value["generation"] == "startup-diagnostic"
    return {"kind": kind, "sample": sample, "spawn_ms": (spawned-started)/1e6, "first_byte_ms": (first_byte-spawned)/1e6, "read_exit_ms": (exited-first_byte)/1e6, "parse_ms": (parsed-exited)/1e6, "total_ms": (parsed-started)/1e6, "response_bytes": len(response), "response_sha256": hashlib.sha256(response).hexdigest()}


rows = []
with tempfile.TemporaryDirectory(prefix="rustify-helper-startup-") as directory:
    for sample in range(5):
        fresh = pathlib.Path(directory) / f"docs-render-{sample}"
        shutil.copy2(helper, fresh)
        rows.append(measure(fresh, "fresh-copy-first-run", sample))
        rows.append(measure(fresh, "same-copy-second-run", sample))
    rows.append(measure(helper, "existing-helper", 0))
result = {"diagnostic_only": True, "helper": str(helper), "helper_bytes": helper.stat().st_size, "helper_sha256": hashlib.sha256(helper.read_bytes()).hexdigest(), "rows": rows}
report.write_text(json.dumps(result, indent=2) + "\n")
for row in rows:
    print(json.dumps({k: row[k] for k in ("kind", "sample", "spawn_ms", "first_byte_ms", "read_exit_ms", "parse_ms", "total_ms", "response_bytes")}))
