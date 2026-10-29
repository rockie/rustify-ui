"""Restore identical products outside the disposable mbx target and retry missing cells."""
from pathlib import Path
import datetime
import hashlib
import json
import shutil
import subprocess
import time
import urllib.request

out = Path(__file__).resolve().parent
repo = out.parents[3]
stable = Path(json.loads((out / "stable-preview.json").read_text())["directory"])
expected = json.loads((out / "products.json").read_text())
http = urllib.request.build_opener(urllib.request.ProxyHandler({}))
proof = {"at": datetime.datetime.now(datetime.timezone.utc).isoformat(), "products": {}}

def protect(name):
    source = repo / "target/makepad-wasm-app/release" / name
    manifest = json.loads((source / "build-manifest.json").read_text())
    assert manifest["build_id"] == expected[name]["build_id"], (name, manifest["build_id"])
    destination = stable / name
    shutil.copytree(source, destination)
    files = {}
    for original in sorted(source.rglob("*")):
        if original.is_file():
            relative = original.relative_to(source)
            digest = hashlib.sha256(original.read_bytes()).hexdigest()
            assert digest == hashlib.sha256((destination / relative).read_bytes()).hexdigest()
            files[str(relative)] = digest
    proof["products"][name] = {"build_id": manifest["build_id"], "directory": str(destination),
                               "matches_initial_build_id": True, "file_sha256": files}
    (out / "recovery-products.json").write_text(json.dumps(proof, indent=2) + "\n")
    print("protected " + name, flush=True)

protect("theme-studio")
with (out / "build-catalog-recovery.log").open("w") as log:
    subprocess.run(["mbx", "xtask", "build-web", "--example", "component-catalog", "--release"],
                   cwd=repo, stdout=log, stderr=subprocess.STDOUT, check=True)
protect("component-catalog")

services = json.loads((out / "services.json").read_text())
for old in services["services"]:
    old["status"] = "stopped for recovery"
for name, port in [("theme-studio", 4181), ("component-catalog", 4176)]:
    command = ["mbx", "xtask", "serve", "--dir", str(stable / name), "--port", str(port), "--csp", "strict"]
    with (out / ("serve-" + name + "-recovery.log")).open("w") as log:
        child = subprocess.Popen(command, cwd=repo, stdout=log, stderr=subprocess.STDOUT, start_new_session=True)
    services["services"].append({"example": name, "port": port, "launcher_pid": child.pid,
        "process_group": child.pid, "command": command, "status": "starting",
        "stop": "SIGINT this task-owned process group, then verify its recorded listener PID is absent"})
    (out / "services.json").write_text(json.dumps(services, indent=2) + "\n")
    for attempt in range(75):
        try:
            remote = http.open(f"http://127.0.0.1:{port}/build-manifest.json", timeout=2).read()
            assert remote == (stable / name / "build-manifest.json").read_bytes()
            break
        except (OSError, AssertionError):
            if child.poll() is not None:
                raise RuntimeError("review server exited: " + name)
            time.sleep(0.2)
    else:
        raise RuntimeError("review server did not start: " + name)
    services["services"][-1]["listener_pids"] = subprocess.check_output(
        ["lsof", "-nP", f"-iTCP:{port}", "-sTCP:LISTEN", "-t"], text=True).split()
    services["services"][-1]["status"] = "running; identical manifest confirmed"
    (out / "services.json").write_text(json.dumps(services, indent=2) + "\n")
    print("serving stable " + name, flush=True)

executions = []
for script in ["capture-u4-conflict-a2.cjs", "capture-detail-supplement-a2.cjs", "capture-u3-supplement-a2.cjs"]:
    with (out / (script + ".log")).open("w") as log:
        result = subprocess.run(["node", str(out / script)], cwd=repo, stdout=log, stderr=subprocess.STDOUT)
    executions.append({"script": script, "exit_code": result.returncode,
                       "finished": datetime.datetime.now(datetime.timezone.utc).isoformat()})
    (out / "recovery-executions.json").write_text(json.dumps(executions, indent=2) + "\n")
    print(json.dumps(executions[-1]), flush=True)
    assert result.returncode == 0
    last = json.loads((out / "capture-log.json").read_text())["runs"][-1]
    assert not last["failures"], last["failures"]
