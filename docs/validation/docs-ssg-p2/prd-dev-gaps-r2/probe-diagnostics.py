"""Replay the first coverage report's embedded-diagnostic counterexamples on the
installed C' and probe a default literal that contains a backtick.

python3.12 probe-diagnostics.py --cli <installed rustify-docs> --sha <sha256> --work <empty dir>

Run in the gaps plan's 9.2 shell (python3 -> 3.12, process-local Git remap,
RUSTIFY_DOCS_CARGO=mbx). Writes diagnostics.json next to this script.
"""

import argparse, hashlib, http.client, json, os, signal, subprocess, sys, threading, time
from pathlib import Path

ROOT = Path(__file__).resolve().parents[4]
OUT = Path(__file__).resolve().parent
args = argparse.ArgumentParser()
for name in ["--cli", "--sha", "--work"]:
    args.add_argument(name, required=True)
args = args.parse_args()
work = Path(args.work)
assert work.is_dir() and not any(work.iterdir()), "--work must be an empty directory"
assert hashlib.sha256(Path(args.cli).read_bytes()).hexdigest() == args.sha
report = {"cli": args.cli, "binary_sha256": args.sha, "cases": {}}
HEADER = "---\ntitle: Diagnostic probe\ndescription: Isolated error cases.\n---\n\n"


def save():
    (OUT / "diagnostics.json").write_text(json.dumps(report, indent=2, ensure_ascii=False) + "\n")


def run(name, command, cwd, expect=None):
    result = subprocess.run(command, cwd=cwd, capture_output=True, text=True)
    (OUT / f"{name}.stdout").write_text(result.stdout)
    (OUT / f"{name}.stderr").write_text(result.stderr)
    if expect is not None and result.returncode != expect:
        report["failure"] = f"{name}: exit {result.returncode}, expected {expect}"
        save()
        sys.exit(report["failure"] + "\n" + result.stderr[-3000:])
    return result


def tree(root):
    return {str(p.relative_to(root)): hashlib.sha256(p.read_bytes()).hexdigest() for p in sorted(root.rglob("*")) if p.is_file()}


def errors(diagnostics, file="content/gaps.md"):
    return [{k: d[k] for k in ["rule", "line", "column", "message", "suggestion"]} for d in diagnostics if d["severity"] == "error" and d["file"] == file]


def refuse(name, site, config, page):
    """check JSON, check text and build on one refused page; dist must stay."""
    (site / "content/gaps.md").write_text(page)
    before = tree(site / "dist") if (site / "dist").exists() else None
    checked = json.loads(run(f"{name}-check", [args.cli, "check", "--config", config, "--format", "json"], site, expect=1).stdout)
    text = run(f"{name}-check-text", [args.cli, "check", "--config", config], site, expect=1).stderr
    built = json.loads(run(f"{name}-build", [args.cli, "build", "--config", config, "--format", "json"], site, expect=1).stdout)
    found = errors(checked["diagnostics"])
    case = {
        "errors": found,
        "build_equals_check": errors(built["diagnostics"]) == found,
        "text_has_each_help": all(f"  --> content/gaps.md:{e['line']}:{e['column']}\n  = help: {e['suggestion']}\n" in text for e in found),
        "null_suggestions": sum(e["suggestion"] is None for e in found),
        "dist_unchanged": before is None or tree(site / "dist") == before,
    }
    report["cases"][name] = case
    save()
    return case


def warm(site, config, name):
    result = run(name, [args.cli, "check", "--config", config, "--format", "json"], site)
    if result.returncode != 0:
        rules = [d["rule"] for d in json.loads(result.stdout)["diagnostics"] if d["severity"] == "error"]
        assert rules == ["rust-generation-changed"], rules
        report.setdefault("generation_changed_once", []).append(name)
        run(name + "-2", [args.cli, "check", "--config", config, "--format", "json"], site, expect=0)


# 1. Content site without Rust: the original builtin counterexample.
content = work / "content-site"
run("content-new", [args.cli, "new", str(content)], work, expect=0)
config = str(content / "rustify-docs.toml")
run("content-build-good", [args.cli, "build", "--config", config, "--format", "json"], content, expect=0)
refuse("content", content, config, HEADER + "```rustify-component\nname: MissingCounter\n```\n\n"
       "```rustify-component\nname: Steps\nprops:\n  items: wrong-type\n```\n")

# 2. The components fixture projected onto C': the original warm counterexamples.
site = work / "components-site"
prepared = run("component-prepare", ["python3", str(ROOT / "scripts/docs-pages-prepare.py"), "--project",
                                     str(ROOT / "tests/docs-ssg/fixtures/components/Cargo.toml"), "--cli", args.cli, "--out", str(site)], ROOT, expect=0)
report["prepare"] = json.loads(prepared.stdout)
site = Path(report["prepare"]["project"])
config = str(site / "rustify-docs.toml")
lock = (site / "Cargo.lock").read_bytes()
warm(site, config, "component-warm")
run("component-build-good", [args.cli, "build", "--config", config, "--format", "json"], site, expect=0)
refuse("component-names", site, config, HEADER + "```rustify-component\nname: MissingCounter\n```\n\n"
       "```rustify-component\nname: Counter\nprops:\n  unexpected: 1\n```\n")
refuse("component-type", site, config, HEADER + "```rustify-component\nname: Counter\nprops:\n  initial: wrong-type\n```\n")
report["lock_unchanged_after_original_cases"] = (site / "Cargo.lock").read_bytes() == lock
save()

# 3. A default literal with a backtick: check/build text versus the dev panel.
registry = (site / "src/lib.rs").read_text()
plain = 'label: String = "Counter".to_string() }'
tick = 'label: String = "a`b".to_string() }'
assert registry.count(plain) == 1
(site / "src/lib.rs").write_text(registry.replace(plain, tick))
(site / "content/gaps.md").unlink()
warm(site, config, "tick-warm")
run("tick-build-good", [args.cli, "build", "--config", config, "--format", "json"], site, expect=0)
tick_page = HEADER + "```rustify-component\nname: Counter\nprops:\n  unexpected: 1\n```\n"
case = refuse("tick", site, config, tick_page)
cli_suggestion = case["errors"][0]["suggestion"]
assert '"a`b".to_string()' in cli_suggestion, cli_suggestion

(site / "content/gaps.md").unlink()
dev = subprocess.Popen([args.cli, "dev", "--config", config, "--port", "0"], cwd=site, stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                       text=True, start_new_session=True)
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
        event = []
        for raw in connection.getresponse():
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
        raise SystemExit(f"dev state not reached; last {states[-1:]}")

    first = until(lambda s: not s["building"] and s["ok"])
    (site / "content/gaps.md").write_text(tick_page)
    failed = until(lambda s: s["revision"] > first["revision"] and not s["building"] and not s["ok"])
    dev_errors = errors(failed["diagnostics"])
    browser = subprocess.run(["node", str(OUT / "tick-browser.cjs"), url, cli_suggestion, str(OUT / "tick-panel")],
                             cwd=ROOT, capture_output=True, text=True)
    (OUT / "tick-browser.log").write_text(browser.stdout + browser.stderr)
    panel = json.loads((OUT / "tick-panel.json").read_text())
    report["tick_dev"] = {
        "sse_suggestion_equals_cli": [e["suggestion"] for e in dev_errors] == [cli_suggestion],
        "terminal_has_suggestion": None,
        "panel": panel,
        "browser_exit": browser.returncode,
    }
    (site / "content/gaps.md").unlink()
    until(lambda s: s["revision"] > failed["revision"] and not s["building"] and s["ok"])
finally:
    os.killpg(dev.pid, signal.SIGTERM)
    dev.wait()
    (OUT / "tick-dev.stderr").write_text("".join(log))
report["tick_dev"]["terminal_has_suggestion"] = cli_suggestion in "".join(log)
(site / "src/lib.rs").write_text(registry)
report["lock_unchanged"] = (site / "Cargo.lock").read_bytes() == lock
report["finished"] = True
save()
print(json.dumps(report, indent=2, ensure_ascii=False))
