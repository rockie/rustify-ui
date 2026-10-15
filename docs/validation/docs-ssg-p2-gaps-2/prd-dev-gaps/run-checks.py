"""Reproduce this coverage review without changing product inputs."""
import datetime
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import time
import uuid
from zoneinfo import ZoneInfo

out = Path(__file__).resolve().parent
repo = out.parents[3]


def git(*args):
    return subprocess.check_output(["git", *args], cwd=repo)


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def save(name, data):
    (out / name).write_text(json.dumps(data, ensure_ascii=False, indent=2) + "\n")


status = git("status", "--porcelain=v1", "--untracked-files=all").decode()
paths = git("ls-files", "-z", "--cached", "--others", "--exclude-standard").decode().split("\0")
hashes = {p: digest(repo / p) for p in paths if p and (repo / p).is_file() and not p.startswith(str(out.relative_to(repo)) + "/")}
save("baseline.json", {
    "review_id": str(uuid.uuid4()),
    "observed_at": datetime.datetime.now(ZoneInfo("Asia/Shanghai")).isoformat(),
    "head": git("rev-parse", "HEAD").decode().strip(),
    "branch": git("branch", "--show-current").decode().strip(),
    "status": status,
    "hashes": hashes,
    "index_diff_sha256": hashlib.sha256(git("diff", "--cached", "--binary")).hexdigest(),
    "tracked_diff_sha256": hashlib.sha256(git("diff", "--binary")).hexdigest(),
})
installed = json.loads((repo / "docs/validation/docs-ssg-p2-gaps-2/apply-r1/installed-after-browser.json").read_text())
entry = Path(installed["entry"])


def identity():
    return {"entry": str(entry), "sha256": digest(entry), "build_info": subprocess.check_output([str(entry), "--build-info"], text=True)}


before = identity()
assert all(before[key] == installed[key] for key in before), (before, installed)
save("installed-before.json", before)
shim = Path(tempfile.mkdtemp(prefix="rustify-gaps2-coverage-tools-"))
(shim / "python3").symlink_to(sys.executable)
env = dict(os.environ, PATH=str(shim) + os.pathsep + os.environ["PATH"],
           GIT_CONFIG_COUNT="1", GIT_CONFIG_KEY_0="url.file://" + str(repo) + "/.insteadOf",
           GIT_CONFIG_VALUE_0="https://github.com/rockie/rustify-ui",
           CARGO_NET_GIT_FETCH_WITH_CLI="true", RUSTIFY_DOCS_CARGO="mbx")
save("environment.json", {"python": sys.version, "node": subprocess.check_output(["node", "--version"], text=True).strip(),
                          "rust": subprocess.check_output(["rustc", "--version"], text=True).strip(),
                          "mbx": subprocess.check_output(["mbx", "--version"], text=True).strip(),
                          "process_environment": {k: env[k] for k in ["GIT_CONFIG_COUNT", "GIT_CONFIG_KEY_0", "GIT_CONFIG_VALUE_0", "CARGO_NET_GIT_FETCH_WITH_CLI", "RUSTIFY_DOCS_CARGO"]}})
results = []
commands = [("cli-diagnostic-text", ["mbx", "test", "--locked", "-p", "rustify-docs-cli", "--test", "diagnostic_text"]),
            ("browser", ["npx", "--no-install", "playwright", "test", "-c", "playwright.docs.config.ts", "--project=docs-dev",
                         "dev-components.spec.ts", "--grep", "gaps embedded diagnostics|gaps-2 code spans", "--workers=1", "--reporter=json",
                         "--output=" + str(out / "browser-artifacts")])]
try:
    for name, command in commands:
        if name == "browser":
            env["RUSTIFY_DOCS_BIN"] = str(entry)
            env["RUSTIFY_ARTIFACT_DIR"] = str(out / "browser")
        start = time.monotonic()
        with (out / (name + (".json" if name == "browser" else ".log"))).open("w") as log, (out / (name + ".stderr")).open("w") as err:
            result = subprocess.run(command, cwd=repo, env=env, stdout=log, stderr=err)
        row = {"name": name, "command": command, "exit": result.returncode, "seconds": round(time.monotonic() - start, 2)}
        results.append(row)
        save("checks.json", results)
        print(json.dumps(row), flush=True)
        if result.returncode:
            sys.exit(result.returncode)
finally:
    save("installed-after.json", identity())
    changed = [p for p, old in hashes.items() if not (repo / p).is_file() or digest(repo / p) != old]
    save("verification-after-tests.json", {"head": git("rev-parse", "HEAD").decode().strip(), "changed_inputs": changed,
                                         "installed_identity_unchanged": identity() == before,
                                         "index_diff_sha256": hashlib.sha256(git("diff", "--cached", "--binary")).hexdigest(),
                                         "tracked_diff_sha256": hashlib.sha256(git("diff", "--binary")).hexdigest()})
    shutil.rmtree(shim)
