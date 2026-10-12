"""Run from any directory: reproduce.py BASE_BINARY TARGET_BINARY OUTPUT_DIR."""
import hashlib
import json
from pathlib import Path
import subprocess
import sys

base, target, root = [Path(value).resolve() for value in sys.argv[1:]]
cases = {
    "svg-template-pair": '<svg><template id="svg-holder"><circle id="circle" r="10"></circle></template></svg>\n\n[Circle](#circle)',
    "svg-template-selfclose": '<svg><template/></svg>\n\n## Visible\n\n[Visible](#visible)',
    "html-template-control": '<template id="holder"><span id="ghost">Hidden</span></template>\n\n## Visible\n\n[Visible](#visible) [Holder](#holder)',
    "html-template-ghost": '<template id="holder"><span id="ghost">Hidden</span></template>\n\n[Ghost](#ghost)',
    "nested-svg-template": '<template id="holder"><svg><template/></svg></template>\n\n## Visible\n\n[Visible](#visible)',
}
results = []
for label, html in cases.items():
    source = root / "candidate-fixtures" / label
    (source / "content").mkdir(parents=True)
    (source / "static").mkdir()
    config = source / "rustify-docs.toml"
    config.write_text('schema_version=1\n[site]\nname="Review"\nurl="https://example.net/"\nbase="/"\n[[nav.groups]]\nlabel="Pages"\nitems=["index.md"]\n')
    (source / "content/index.md").write_text('---\ndescription: Review case\n---\n# Review\n\n' + html + '\n')
    item = {"case": label, "input": html}
    for version, binary, out in [("baseline", base, "baseline-dist"), ("target", target, "dist")]:
        done = subprocess.run([str(binary), "build", "--config", str(config), "--out", str(source / out), "--format", "json"], capture_output=True, text=True)
        result = {"exit": done.returncode, "report": json.loads(done.stdout), "stderr": done.stderr}
        if version == "target":
            item.update({"exit": done.returncode, "stdout": result["report"], "stderr": done.stderr})
        else:
            item[version] = result
    results.append(item)
data = {"base": "1f0fc5ed21d7937a1297528ad38cdefe6b95f721", "target": "540462a895a48385761f8db6e3271a714957d818", "cases": results,
        "binary_hashes": {version: hashlib.sha256(binary.read_bytes()).hexdigest() for version, binary in [("base", base), ("target", target)]}}
(root / "candidate-results.json").write_text(json.dumps(data, indent=2) + '\n')
print(json.dumps([{"case": r["case"], "base_exit": r["baseline"]["exit"], "target_exit": r["exit"]} for r in results]))
