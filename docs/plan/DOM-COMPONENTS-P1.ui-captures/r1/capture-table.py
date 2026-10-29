# Prints the per-image table and per-unit action logs for capture.md from
# capture-log.json (latest record per file wins; earlier failed runs stay in the log).
import json, sys, pathlib
here = pathlib.Path(__file__).parent
log = json.loads((here / "capture-log.json").read_text())
latest = {}
for run in log["runs"]:
    for r in run["records"]:
        latest[r["file"]] = r
order = sorted(latest.values(), key=lambda r: (r["unit"], r["time"]))
print("| 单元 | 状态 | 图像 | 实际URL | CSS视口 / DPR | 主题 | 截图方式 | 拍摄时间（UTC） | 干预 |")
print("| --- | --- | --- | --- | --- | --- | --- | --- | --- |")
for r in order:
    print(f"| {r['unit']} | {r['state']} | [{r['file']}]({r['file']}) | {r['url']} | {r['viewport']} / {r['dpr']} | {r['theme']} | {r['kind']} | {r['time']} | {r['interventions']} |")
if "--actions" in sys.argv:
    for run in log["runs"]:
        for unit, lines in run["actions"].items():
            (here / f"{unit}-actions.log").open("a").write("\n".join(lines) + "\n")
