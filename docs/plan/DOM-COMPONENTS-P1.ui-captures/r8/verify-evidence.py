"""Verify this manual review's captured evidence; does not score the UI."""
import hashlib
import json
import re
import struct
import subprocess
import sys
from collections import Counter
from datetime import datetime, timezone
from pathlib import Path
from urllib.request import urlopen

p = Path(__file__).resolve().parent
root = p.parents[3]


def read(name):
    return json.loads((p / name).read_text())


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def write(name, value):
    (p / name).write_text(json.dumps(value, ensure_ascii=False, indent=2) + "\n")


v = read("tested-version.json")
head = subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=root).decode().strip()
assert head == v["head"]
files = sorted(subprocess.check_output(["git", "ls-files", "-z", "--", *v["product_paths"]], cwd=root).decode().rstrip("\0").split("\0"))
h = hashlib.sha256()
for rel in files:
    h.update(rel.encode() + b"\0" + hashlib.sha256((root / rel).read_bytes()).digest())
assert h.hexdigest() == v["product_fingerprint_sha256"]
assert not subprocess.check_output(["git", "diff", "HEAD", "--", *v["product_paths"]], cwd=root)
assert not subprocess.check_output(["git", "ls-files", "--others", "--exclude-standard", "--", *v["product_paths"]], cwd=root)
unchanged = {rel: sha(root / rel) == digest for rel, digest in read("immutable-before.json").items()}
assert all(unchanged.values()), unchanged
target_output = subprocess.check_output([sys.executable, str(p.parent / "r5/target-fingerprint.py"), str(root / read("target.json")["frozen_snapshot"]), str(root)], cwd=root).decode()
assert target_output.rstrip().endswith(read("target.json")["fingerprint"])
(p / "target-fingerprint-final.txt").write_text(target_output)
packets = []
for a in read("judge-dispatch.json")["agents"]:
    name = a.get("packet_file", "packet-" + a["unit"] + ".json")
    task = a.get("task_file", "task-" + a["unit"] + ".md")
    packet_path = p / "judge-inputs" / name
    assert sha(packet_path) == a["packet_sha256"]
    assert sha(p / "judge-inputs" / task) == a["task_sha256"]
    packet = json.loads(packet_path.read_text())
    assert all(sha(Path(rel)) == digest for rel, digest in packet["files"].items())
    packets.append({"packet": name, "files_verified": len(packet["files"]), "hashes_match": True})
records = [r for run in read("capture-log.json")["runs"] for r in run["records"]]
assert len(records) == len({r["file"] for r in records}) == 340
images = read("image-integrity.json")
assert {r["file"] for r in records} == {r["file"] for r in images}
for r in images:
    f = p / r["file"]
    assert sha(f) == r["sha256"]
    assert struct.unpack(">II", f.read_bytes()[16:24]) == (r["width"], r["height"])
for unit, data in read("judge-results.json")["units"].items():
    for candidate in data["candidates"]:
        assert sha(p / candidate["raw"]) == candidate["sha256"]
assert not read("capture-resolutions.json")["unresolved_required_frames"]
http = {}
for name, product in read("products.json").items():
    data = urlopen(product["url"] + "build-manifest.json", timeout=10).read()
    assert hashlib.sha256(data).hexdigest() == product["manifest_sha256"]
    http[name] = {"build_id": json.loads(data)["build_id"], "manifest_sha256": hashlib.sha256(data).hexdigest()}
    assert http[name]["build_id"] == product["build_id"]
for product in read("recovery-products.json")["products"].values():
    assert all(sha(Path(product["directory"]) / rel) == digest for rel, digest in product["file_sha256"].items())
write("version-final-check.json", {"at": datetime.now(timezone.utc).isoformat(), "head": head, "product_fingerprint_sha256": h.hexdigest(), "product_files": len(files), "product_diff_and_untracked_empty": True, "immutable_files": unchanged, "target_fingerprint_matches": True, "packets": packets, "native_frames_verified": len(images), "by_unit": dict(Counter(r["unit"] for r in records)), "http_products": http, "stable_product_file_hashes_match": True, "required_missing": []})


def rgb(s):
    return [float(x) for x in re.findall(r"[\d.]+", s)]


c = read("machine-confirmation.json")
comparisons = []
for row in c["contrast"]:
    if row.get("testid", "").startswith("button-color-"):
        a = rgb(row["a9"])
        expected = [round(x * a[3] + 255 * (1 - a[3])) for x in a[:3]] if len(a) == 4 else a
        actual = rgb(row["computedBackground"])
        delta = max(abs(x - y) for x, y in zip(expected, actual))
        foreground_matches = row["computedColor"] == row["contrast"]
        assert delta <= 1 and foreground_matches
        comparisons.append({"color": row["color"], "computed_background": actual, "a9_over_white": expected, "maximum_channel_delta": delta, "foreground_matches_contrast": foreground_matches})
assert len(comparisons) == 16
menus = [{"viewport": m["viewport"], "theme": m["theme"], "steps": [{"step": s["step"], "expanded": s["expanded"], "list_visible": bool(s["lists"]) and all(x["visible"] for x in s["lists"])} for s in m["states"][1:]]} for m in c["menus"]]
assert all(s["expanded"] == "true" and s["list_visible"] for m in menus for s in m["steps"])
assert all(not x[k] for x in c["health"] for k in ["pageErrors", "consoleErrors", "failed", "csp"])
write("machine-confirmation-summary.json", {"menus": menus, "solid_color_pairs": comparisons, "comparison_method": "Computed a9 alpha-composited over the white fixture; at most one RGB channel unit from rounded CSS alpha serialization. Foreground equals computed contrast variable. This checks current runtime pairs, not every palette value.", "other_contrast_nodes": [{k: row.get(k) for k in ["testid", "text", "summary", "computedColor", "computedBackground"]} for row in c["contrast"] if not row.get("testid", "").startswith("button-color-")], "health_pages": len(c["health"]), "health_errors": 0})
mc = {r["file"]: r for r in c["records"]}
extra = read("machine-extra.json")
for r in extra["inspector"]:
    mc[r["file"]] = {"file": r["file"], "viewport": r["viewport"], "theme": r["mode"], "url": "http://127.0.0.1:4181/", "run_started_at": extra["at"], "time_note": "Per-frame UTC was not recorded; filesystem mtime below is not a shutter timestamp."}
mc["MC-root200-preview-720-light.png"] = {"file": "MC-root200-preview-720-light.png", "viewport": "720x900", "theme": "light", "url": "http://127.0.0.1:4181/", "interventions": "root font-size 200%", "run_started_at": read("machine-check.json")["at"], "time_note": "Per-frame UTC was not recorded; filesystem mtime below is not a shutter timestamp."}
assert len(mc) == len(list(p.glob("MC-*.png"))) == 13
for name, r in mc.items():
    r.update(sha256=sha(p / name), dimensions=struct.unpack(">II", (p / name).read_bytes()[16:24]), filesystem_mtime_utc=datetime.fromtimestamp((p / name).stat().st_mtime, timezone.utc).isoformat(), tool="Playwright 1.63.0 / Chromium 153.0.8010.12 headless", dpr=1)
write("machine-captures.json", {"purpose": "Machine-check captures after independent judges; excluded from scores", "frames": list(mc.values())})
print(json.dumps({"version_and_immutable_checks": "passed", "packets": len(packets), "scoring_frames": len(images), "machine_frames": len(mc), "solid_pairs": len(comparisons), "settled_menu_steps": 9}))
