import json
import pathlib
import subprocess
import sys
from datetime import datetime, timezone

root = pathlib.Path(__file__).resolve().parents[4]
evidence = pathlib.Path(__file__).resolve().parent
commands = [
    ("fmt", ["cargo", "fmt", "--all", "--", "--check"]),
    ("library-test", ["mbx", "test", "--locked", "-p", "rustify-docs"]),
    ("library-clippy", ["mbx", "clippy", "--locked", "-p", "rustify-docs", "--all-targets", "--", "-D", "warnings"]),
    ("cli-clippy", ["mbx", "clippy", "--locked", "-p", "rustify-docs-cli", "--all-targets", "--", "-D", "warnings"]),
]
for package in ["rustify-ui", "rustify-components"]:
    for mode, flags in [("default", []), ("gpu", ["--features", "gpu"]), ("ssr", ["--no-default-features", "--features", "ssr"])]:
        for check in ["test", "clippy"]:
            options = ["--lib"] if check == "test" else ["--all-targets"]
            tail = [] if check == "test" else ["--", "-D", "warnings"]
            commands.append((f"{package}-{mode}-{check}", ["mbx", check, "--locked", "-p", package, *flags, *options, *tail]))
commands.extend([
    ("csr-workspace-test", ["mbx", "test", "--locked", "--workspace", "--exclude", "rustify-docs", "--exclude", "rustify-docs-cli", "--lib", "--bins"]),
    ("csr-workspace-clippy", ["mbx", "clippy", "--locked", "--workspace", "--exclude", "rustify-docs", "--exclude", "rustify-docs-cli", "--all-targets", "--", "-D", "warnings"]),
    ("sources", ["mbx", "xtask", "sources", "verify"]),
    ("css", ["mbx", "xtask", "css", "--check"]),
    ("catalog", ["mbx", "xtask", "catalog", "--write", "docs/components.md", "--check"]),
    ("features", ["mbx", "xtask", "docs-verify", "--group", "features"]),
])
results = []
for name, args in commands:
    print(f"Running {name}", flush=True)
    started = datetime.now(timezone.utc).isoformat()
    with (evidence / f"{name}.log").open("w") as log:
        completed = subprocess.run(args, cwd=root, stdout=log, stderr=subprocess.STDOUT)
    results.append({"name": name, "command": args, "started_at": started, "exit_code": completed.returncode})
    (evidence / "matrix.json").write_text(json.dumps(results, indent=2) + "\n")
    print(f"{name}: exit {completed.returncode}", flush=True)
    if completed.returncode:
        sys.exit(completed.returncode)
