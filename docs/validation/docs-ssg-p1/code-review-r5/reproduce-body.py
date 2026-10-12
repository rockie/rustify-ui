"""Reproduce document/fragment anchor disagreement using isolated projects."""
import hashlib
import json
import subprocess
import sys
import tempfile
from pathlib import Path

before, after, output = sys.argv[1:]
workspace = Path(tempfile.mkdtemp(prefix="rustify-review-r5-body-"))
body = '---\ndescription: Body anchor\n---\n# Body anchor\n\n<body id="target">\n\nText\n\n</body>\n'
links = '\n[Target](#target) [Other target](other.md#target)\n'


def tree(root):
    return {str(p.relative_to(root)): hashlib.sha256(p.read_bytes()).hexdigest()
            for p in sorted(root.rglob("*")) if p.is_file()}


def run(binary, command, site, *extra):
    args = [binary, command, "--config", str(site / "rustify-docs.toml"),
            "--format", "json", *extra]
    result = subprocess.run(args, capture_output=True, text=True)
    return {"command": args, "exit": result.returncode,
            "report": json.loads(result.stdout), "stderr": result.stderr}


results = []
for base in ["/", "/docs/"]:
    site = workspace / ("root" if base == "/" else "subpath")
    (site / "content").mkdir(parents=True)
    (site / "rustify-docs.toml").write_text(
        f'schema_version=1\n[site]\nname="Body anchors"\nurl="https://example.net{base}"\nbase="{base}"\n'
        '[[nav.groups]]\nlabel="Pages"\nitems=["index.md","other.md"]\n')
    (site / "content/index.md").write_text(body)
    (site / "content/other.md").write_text(body)
    assert run(after, "build", site)["exit"] == 0
    good = tree(site / "dist")
    (site / "content/index.md").write_text(body + links)
    base_check = run(before, "check", site)
    base_build = run(before, "build", site, "--out", str(site / "before-dist"))
    target_check = run(after, "check", site)
    target_build = run(after, "build", site)
    assert base_check["exit"] == base_build["exit"] == 0
    assert target_check["exit"] == target_build["exit"] == 1
    assert target_check["report"]["summary"]["errors"] == 2
    assert target_build["report"]["summary"]["errors"] == 2
    assert tree(site / "dist") == good
    results.append({"base": base, "site": str(site), "base_check": base_check,
                    "base_build": base_build, "target_check": target_check,
                    "target_build": target_build, "previous_output_preserved": True})
Path(output).write_text(json.dumps({"workspace": str(workspace), "cases": results}, indent=2) + "\n")
print(json.dumps({"cases": len(results), "workspace": str(workspace), "errors_per_target": 2}))
