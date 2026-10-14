"""An author's new components project with an installed CLI, end to end.

author-acceptance.py --cli <installed rustify-docs> --r <R> --work <empty dir> --out <json>

Run in the plan's 9.2 shell. The project pins R and keeps its lock; check
and build succeed; embedded refusals carry the same repairs at the same
places in check, build and dev, keep the previous output and the last good
dev page, and clear after the fix.
"""

import argparse, hashlib, http.client, json, os, signal, subprocess, sys, threading, time, tomllib
from pathlib import Path

args = argparse.ArgumentParser()
for name in ["--cli", "--r", "--work", "--out"]:
    args.add_argument(name, required=True)
args = args.parse_args()
work = Path(args.work)
assert work.is_dir() and not any(work.iterdir()), "--work must be an empty directory"
site = work / "author-site"
report = {"cli": args.cli, "R": args.r, "steps": []}
REFERENCE = "https://github.com/rockie/rustify-ui/blob/main/docs/docs-ssg.md#static-builtins"


def save():
    Path(args.out).write_text(json.dumps(report, indent=2, ensure_ascii=False) + "\n")


def fail(message):
    report["failure"] = message
    save()
    sys.exit(message)


def run(name, command, expect=0, cwd=None):
    result = subprocess.run(command, cwd=cwd or site, capture_output=True, text=True)
    (work / f"{name}.stdout").write_text(result.stdout)
    (work / f"{name}.stderr").write_text(result.stderr)
    report["steps"].append({"name": name, "command": command, "exit": result.returncode})
    if result.returncode != expect:
        fail(f"{name}: exit {result.returncode}, expected {expect}\n{result.stderr[-3000:]}")
    return result


def tree(root):
    return {str(path.relative_to(root)): hashlib.sha256(path.read_bytes()).hexdigest() for path in sorted(root.rglob("*")) if path.is_file()}


def embedded(diagnostics, file):
    return sorted((d["rule"], d["line"], d["column"], d["suggestion"]) for d in diagnostics if d["severity"] == "error" and d["file"] == file)


run("build-info", [args.cli, "--build-info"], cwd=work)
run("new", [args.cli, "new", str(site), "--template", "components"], cwd=work)
manifest = (site / "Cargo.toml").read_text()
pins = [line.strip() for line in manifest.splitlines() if "github.com/rockie/rustify-ui" in line]
assert len(pins) == 3 and all(f'rev = "{args.r}"' in line for line in pins), pins
lock = (site / "Cargo.lock").read_bytes()
inputs = {path: (site / path).read_bytes() for path in ["Cargo.toml", "Cargo.lock", "src/lib.rs", "src/demos.rs"]}
report["pins"] = pins
report["lock_sha256"] = hashlib.sha256(lock).hexdigest()

run("fetch", ["cargo", "fetch", "--locked"])
graphs = {}
for target, features, extra in [("host", "ssr", []), ("wasm", "hydrate", ["--filter-platform", "wasm32-unknown-unknown"])]:
    metadata = json.loads(run(f"{target}-metadata", ["mbx", "metadata", "--locked", "--offline", "--format-version", "1",
                                                     "--no-default-features", "--features", features, *extra]).stdout)
    names = {package["name"] for package in metadata["packages"]}
    sources = {package["source"] for package in metadata["packages"] if package["name"] in ("rustify-docs", "rustify-ui", "rustify-components")}
    assert not [name for name in names if "makepad" in name.lower()], names
    assert sources == {f"git+https://github.com/rockie/rustify-ui?rev={args.r}#{args.r}"}, sources
    graphs[target] = {"packages": len(names), "makepad": 0, "framework_source": sorted(sources)}
report["graphs"] = graphs

# The first development generation under mbx can discard itself once
# (plan 12); it is recorded, and the second check must succeed.
result = subprocess.run([args.cli, "check", "--format", "json"], cwd=site, capture_output=True, text=True)
report["steps"].append({"name": "check-1", "exit": result.returncode})
if result.returncode != 0:
    errors = [d["rule"] for d in json.loads(result.stdout)["diagnostics"] if d["severity"] == "error"]
    if errors != ["rust-generation-changed"]:
        fail(f"first check: {errors}")
    report["first_check_generation_changed"] = True
    run("check-2", [args.cli, "check", "--format", "json"])
build = json.loads(run("build-good", [args.cli, "build", "--format", "json"]).stdout)
good = tree(site / "dist")
report["good"] = {"pages": build["summary"]["pages"], "files": len(good)}

names = "Use a component from `CardGrid`, `Counter`, `Steps`, `Tabs`; for an author component, enable `[rust]` and register its name in `site!`."
fields = "`initial` (`i32`; default expression `2`), `label` (`String`; default expression `\"Counter\".to_string()`)"
cold = "\n".join([
    "---", "title: Gaps", "description: Refused embeds", "---", "# Gaps", "",
    "```rustify-component", "name: MissingWidget", "```", "",
    "```rustify-component", "name: Counter", "props:", "  unexpected: 1", "```", "",
    "```rustify-demo", "name: MissingDemo", "```", "",
    "```rustify-component", "name: Steps", "```", "",
    "```rustify-component", "name: Counter", "props: {initial: 1, initial: 2}", "```", "",
])
cold_expected = sorted([
    ("unregistered-component", 7, 1, names),
    ("unknown-property", 14, 3, f"Use one of {fields} for `Counter`; rename or remove `unexpected`."),
    ("unregistered-demo", 17, 1, "Use a demo from `CounterDemo`; register demos in `site!`."),
    ("embed-props", 21, 1, f"Add or correct `props` using the declared fields `items` (`Vec<StepSpec>`; required); see {REFERENCE} for its object fields."),
    ("embed-payload", 27, 8, "Keep each YAML mapping key once; remove or rename the duplicate key."),
])
typed = "\r\n".join([
    "---", "title: Gaps", "description: 中文与 CRLF", "---", "# Gaps", "",
    "中文前缀：初始值写错了。", "",
    "```rustify-component", "name: Counter", "props: {label: 中文标签, initial: 错误}", "```", "",
])
typed_expected = [("invalid_props", 11, 22, "Write `initial: 0` for declared type `i32` (default expression `2`). Reported props path: `initial`.")]
cases = {}
for label, text, expected in [("cold", cold, cold_expected), ("typed", typed, typed_expected)]:
    (site / "content/gaps.md").write_bytes(text.encode())
    built = json.loads(run(f"build-{label}", [args.cli, "build", "--format", "json"], expect=1).stdout)
    checked = json.loads(run(f"check-{label}", [args.cli, "check", "--format", "json"], expect=1).stdout)
    text_check = run(f"check-{label}-text", [args.cli, "check"], expect=1).stderr
    found = embedded(built["diagnostics"], "content/gaps.md")
    if found != expected:
        fail(f"{label}: {json.dumps(found, ensure_ascii=False)}")
    if embedded(checked["diagnostics"], "content/gaps.md") != found:
        fail(f"{label}: check differs from build")
    for rule, line, column, suggestion in found:
        if f"  --> content/gaps.md:{line}:{column}\n  = help: {suggestion}\n" not in text_check:
            fail(f"{label}: text check lacks {rule} at {line}:{column}")
    if tree(site / "dist") != good:
        fail(f"{label}: a refused build changed dist")
    cases[label] = {"refusals": len(found), "dist_unchanged": True, "check_equals_build": True}
report["cases"] = cases

# dev: the same refusal over the last good page, then the fix clears it.
(site / "content/gaps.md").unlink()
dev = subprocess.Popen([args.cli, "dev", "--port", "0"], cwd=site, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True, start_new_session=True)
log = []
threading.Thread(target=lambda: log.extend(dev.stderr), daemon=True).start()
try:
    line = dev.stdout.readline()
    assert line.startswith("dev: "), line
    url = line.split(" ", 1)[1].strip()
    host, port = url.split("//", 1)[1].split("/", 1)[0].split(":")
    states = []

    def follow():
        connection = http.client.HTTPConnection(host, int(port))
        connection.request("GET", "/__rustify-docs/events?path=/&good=0")
        response = connection.getresponse()
        event = []
        for raw in response:
            text = raw.decode().rstrip("\n")
            if text == "":
                if event and event[0] == "event: state":
                    states.append(json.loads(event[1][len("data: "):]))
                event = []
            else:
                event.append(text)

    threading.Thread(target=follow, daemon=True).start()

    def until(wanted, seconds=900):
        started = time.time()
        while time.time() - started < seconds:
            for state in list(states):
                if wanted(state):
                    return state
            time.sleep(0.2)
        fail(f"dev state not reached; last {states[-1:]}")

    first = until(lambda s: not s["building"] and s["ok"])
    page = lambda: http.client.HTTPConnection(host, int(port))
    connection = page(); connection.request("GET", "/"); good_page = connection.getresponse().read()
    (site / "content/gaps.md").write_bytes(typed.encode())
    failed = until(lambda s: s["revision"] > first["revision"] and not s["building"] and not s["ok"])
    if embedded(failed["diagnostics"], "content/gaps.md") != typed_expected:
        fail(f"dev: {embedded(failed['diagnostics'], 'content/gaps.md')}")
    connection = page(); connection.request("GET", "/"); served = connection.getresponse()
    assert served.status == 200 and served.read() == good_page, "dev must keep serving the last good page"
    (site / "content/gaps.md").unlink()
    fixed = until(lambda s: s["revision"] > failed["revision"] and not s["building"] and s["ok"])
    assert fixed["errors"] == 0 and not embedded(fixed["diagnostics"], "content/gaps.md")
    report["dev"] = {"url": url, "first_revision": first["revision"], "failed_revision": failed["revision"], "fixed_revision": fixed["revision"],
                     "repair": failed["diagnostics"] and embedded(failed["diagnostics"], "content/gaps.md"), "last_good_served": True}
finally:
    os.killpg(dev.pid, signal.SIGTERM)
    dev.wait()
    (work / "dev.stderr").write_text("".join(log))

unchanged = {path: (site / path).read_bytes() == data for path, data in inputs.items()}
assert all(unchanged.values()), unchanged
report["inputs_unchanged"] = unchanged
report["passed"] = True
save()
print(json.dumps({k: report[k] for k in ["graphs", "good", "cases", "dev", "inputs_unchanged"]}, indent=2, ensure_ascii=False))
