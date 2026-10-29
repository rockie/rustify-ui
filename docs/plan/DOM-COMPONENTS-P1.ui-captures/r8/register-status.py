# -*- coding: utf-8 -*-
"""One-time v3 ledger registration for the completed R8 report."""
import hashlib
import json
import re
from pathlib import Path

p = Path(__file__).resolve().parent
root = p.parents[3]
s = root / ".xgent-ai/sdlc"
r = json.loads((p / "registration.json").read_text())
rid = r["review_id"]
old = "8abc8d80-fa38-4e09-b066-929980601837"
plan = "37a2c7b4-7052-4f85-96e9-135b6f7667a1"
report = root / "docs/plan/DOM-COMPONENTS-P1.ui-review.r8.md"
report_rel = report.relative_to(root).as_posix()
capture_rel = p.relative_to(root).as_posix()


def sha(f):
    return hashlib.sha256(f.read_bytes()).hexdigest()


def replace(text, a, b):
    assert text.count(a) == 1, (a, text.count(a))
    return text.replace(a, b, 1)


def source(f, anchor):
    return {"path": f.relative_to(root).as_posix(), "revision": "sha256:" + sha(f), "anchor": anchor}


def front(data):
    return "---\n" + "".join(k + ": " + json.dumps(v, ensure_ascii=False) + "\n" for k, v in data.items()) + "---\n\n"


def create(f, text):
    assert not f.exists(), str(f)
    f.parent.mkdir(parents=True, exist_ok=True)
    f.write_text(text, encoding="utf-8")


for rel, digest in r["before"].items():
    assert sha(root / rel) == digest, rel
text = report.read_text()
assert "## 状态登记" not in text
text += f"""

## 状态登记

按仓内协议 v3 / state-model v2 登记独立 UI 验收评审 ID `{rid}`，`review_type: other`，关联 R7 `{old}`。见[本轮状态页](../../.xgent-ai/sdlc/status/review/{rid}.md)。本轮为 `review-recorded` / `review-not-required`，表示报告已记录且无需 apply；R7 的 UI-218 处置已接续独立复核关闭，原 R7 分数与结论保留。

仅同步受影响的评审状态、计划状态导航与目录索引；原计划 Ready、实施 7/7 及实施事件集合不变，未登记产品终验、合并或发布。报告、证据和本次台账回写留在未提交工作树，未推送；下一步由用户选择是否结束打磨并启动代码复审。
"""
report.write_text(text, encoding="utf-8")
sources = [source(report, "验收判定、评分、上轮差距复核、机器检查与采集覆盖"), source(p / "judge.md", "8位全新评审；七单元均10，未关闭差距并集为空"), source(p / "capture.md", "340张本轮原图、13帧恢复补采、62张缩放分段"), source(root / "docs/plan/DOM-COMPONENTS-P1.ui-target.r7.md", "冻结规范与13张附件；目标指纹6c8b16d50cf7"), source(p / "version-final-check.json", "被测HEAD、源码指纹、产物身份和只读文件核对")]
relations = [{"type": "reviews", "target": plan, "scope": "P1 Studio 与 DOM 组件；U1–U7 全量"}, {"type": "rechecks", "target": old, "scope": "UI-R7 的 UI-218 独立复核；同一目标全量回归"}]
old_status = s / f"status/review/{old}.md"
old_text = old_status.read_text()
old_relations = json.loads(re.search(r"^relations: (.+)$", old_text, re.M)[1])


def event(eid, obj, facet, state, original, title, predecessors, body, src=sources, rels=relations):
    data = {"schema_version": 3, "event_id": eid, "object_id": obj, "kind": "review", "review_type": "other", "title": title, "artifact": report_rel if obj == rid else "docs/plan/DOM-COMPONENTS-P1.ui-review.r7.md", "facet": facet, "scope": "UI-R8" if obj == rid else "UI-R7", "state": state, "source_state": original, "evidence_status": "confirmed", "observed_at": r["observed_at"], "predecessors": predecessors, "baseline": r["baseline"], "sources": src, "relations": rels}
    create(s / f"events/review/{obj}/{eid}.md", front(data) + "# " + title + "\n\n" + body + "\n")


event(r["review_event"], rid, "review", "review-recorded", "通过验收；10/10；门槛8；UI-218已解决；未关闭差距0", "DOM-COMPONENTS-P1 UI 验收评审 R8", [], f"""- [完整报告](../../../../../{report_rel})判通过验收，七单元均10/10，门槛8；UI-218由U5独立评审确认已解决，无未关闭差距、停滞或待裁定。
- 被测HEAD `{r['baseline']['head']}`；产品路径diff及未跟踪为空，源码指纹 `e492abef49d9ff1bd4ebb891788621965bcce02dd86f99d53de37417e76c7ab2`。两项release当前重建，HTTP manifest及恢复副本逐文件一致。
- 沿用人工算法和目标指纹 `6c8b16d50cf78224c314e9c6a8a6c3416ea54069319ba4425751bf4d50d159c4`，无managed接管或重建基线。340张原图、8位全新评审，U7补采后重评；另13张机器图不参与评分。初次404的13帧已全部补齐，失败日志保留。
- 按state-model记录review-recorded；不表示代码复审、整期UAT、WCAG认证、合并或发布。参数组合、axe incomplete、人工与其他浏览器未验范围见报告；临时服务及自有副本已清理。
- 本轮无产品修改或提交；计划原文、目标、历史只读。若结束打磨，下一步 `$review-code docs/plan/DOM-COMPONENTS-P1.md`。""")
event(r["disposition_event"], rid, "disposition", "review-not-required", "通过验收；无剩余差距，无需apply", "UI R8 · 无需强制修复", [], f"""- [完整报告](../../../../../{report_rel})已落盘并回读，冻结全量矩阵经独立评审通过，无需处理的差距或待裁定事项。
- 按state-model v2的通过优先规则取review-not-required，表示无需apply，不伪造运行过修复。建议及未验范围保留。
- Ready和7/7保持，不登记产品终验、合并或发布。后续代码复审由用户另行发起。""")
closing_sources = [source(report, "上轮差距复核 / UI-218已解决"), source(p / "judge-U5-a.md", "固定蓝色与真实未标注说明的独立对照"), source(p / "machine-extra.json", "三组合蓝色实线、拾取拦截和Escape退出"), source(root / "docs/plan/DOM-COMPONENTS-P1.records/M7.md", "UI验收修复 / R7；原处置已修改待评审")]
event(r["prior_disposition_event"], old, "disposition", "review-not-required", "原R7通过；可选UI-218已由R8独立复核解决", "UI R7 · UI-218 获 R8 独立复核关闭", ["d967cf00-b1ee-4964-acbb-0cc230091813"], f"""- [R8复核](../../../../../{report_rel}#上轮差距复核)与[U5原文](../../../../../{capture_rel}/judge-U5-a.md)确认UI-218已解决：固定蓝色拾取框及真实未标注说明同时保留。三组合真实页面实测确认蓝色实线、拾取拦截和Esc退出。
- 接续UI-R7原处置；原R7通过9.8和历史stale边界不改写，当前CSS由独立R8覆盖。按通过优先规则保持review-not-required，另明确其唯一可选P2已关闭。
- M7原“已修改待评审”保持只读。Ready、7/7及实施事件集合不变；未产生代码复审、产品终验、合并或发布事实。""", closing_sources, old_relations)
status_data = {"object_id": rid, "kind": "review", "review_type": "other", "artifact": report_rel, "archived": False, "observed_at": r["observed_at"], "baseline": r["baseline"], "included_events": [r["review_event"], r["disposition_event"]], "relations": relations}
create(s / f"status/review/{rid}.md", front(status_data) + f"""# DOM-COMPONENTS-P1 UI 验收评审 R8

来源：[报告](../../../../{report_rel})（sha256:{sha(report)}）、[实拍](../../../../{capture_rel}/capture.md)、[独立结论](../../../../{capture_rel}/judge.md)、[版本核对](../../../../{capture_rel}/version-final-check.json)。

| 维度 / 范围 | 固定状态码 | 来源自述 | 证据质量 | 下一步 |
| --- | --- | --- | --- | --- |
| review / UI-R8 | review-recorded | 通过验收，10/10，门槛8 | confirmed | 无剩余UI差距；结束打磨后review-code |
| disposition / UI-R8 | review-not-required | 无需apply | confirmed | 不伪造修复或产品终验 |

- 被测HEAD `{r['baseline']['head']}`，产品路径无未提交改动；源码指纹e492abef49d9…，完整算法与文件集合见版本证据。非产品工作树已有修改保留。
- 同一冻结目标6c8b16d50cf7…；七单元均10，UI-218独立复核已解决；未关闭P0/P1为0/0、其余差距0，无停滞、待裁定或必需缺证。
- 340张真实原图、8位全新评审；U7补采后另派评审，两位均10，取整组分与差距并集。另13张机器图不计评分。初次404的13帧已全部补齐，环境噪声及未验边界见报告。
- UI结论不等于代码复审、整期UAT、WCAG认证、合并或发布。Ready、7/7和原实施事件保持；源产物与台账未提交、未推送，自有临时服务已停止。
- 下一步：是否继续打磨由用户选择；结束时 `$review-code docs/plan/DOM-COMPONENTS-P1.md`。

## 已纳入事件

- [{r['review_event']}](../../events/review/{rid}/{r['review_event']}.md)
- [{r['disposition_event']}](../../events/review/{rid}/{r['disposition_event']}.md)
""")
# Preserve prior event files, source reports, plan and implementation event set.
old_text = replace(old_text, 'observed_at: "2026-10-29T17:10:34+08:00"', "observed_at: " + json.dumps(r["observed_at"]))
included = json.loads(re.search(r"^included_events: (.+)$", old_text, re.M)[1]) + [r["prior_disposition_event"]]
old_text = re.sub(r"^included_events: .+$", "included_events: " + json.dumps(included), old_text, count=1, flags=re.M)
old_text = replace(old_text, "；独立复审 |", f"；当前版本见[R8]({rid}.md) |")
line = next(x for x in old_text.splitlines() if x.startswith("| disposition / UI-R7 |"))
old_text = replace(old_text, line, f"| disposition / UI-R7 | review-not-required | 可选UI-218已由R8独立复核解决 | confirmed | [独立关闭](../../events/review/{old}/{r['prior_disposition_event']}.md) |")
old_text = replace(old_text, "处置只记已修改待评审，不评分、不自行关闭。", f"作者当时仅记已修改待评审；本次[R8独立复核]({rid}.md)确认解决，未改原分数。")
old_text = replace(old_text, "- 下一步：`$review-ui docs/plan/DOM-COMPONENTS-P1.md`；独立UI复审后仍须review-code本循环UI修复。整期/参数/人工验证边界见原报告与本次检查。", "- 下一步：R8已完成独立UI复审；若结束打磨，`$review-code docs/plan/DOM-COMPONENTS-P1.md`。整期/参数/人工边界见原报告与R8。")
old_text += f"\n- [{r['prior_disposition_event']}](../../events/review/{old}/{r['prior_disposition_event']}.md)\n"
old_status.write_text(old_text)
plan_status = s / f"status/plan/{plan}.md"
text = plan_status.read_text()
text = replace(text, 'observed_at: "2026-10-29T17:10:34+08:00"', "observed_at: " + json.dumps(r["observed_at"]))
line = next(x for x in text.splitlines() if x.startswith("| implementation / plan |"))
text = replace(text, line, f"| implementation / plan | dev-plan-completed | 7/7；UI R8独立验收通过，UI-218已解决 | confirmed | [R8对象](../review/{rid}.md)；10/10、无剩余UI差距；结束打磨后review-code。原M7、Ready和实施事件集合保持，整期及人工边界不变 |")
text = replace(text, "## UI 验收 R7（当前）", "## UI 验收 R7（历史；R8已复核）")
text += f"""

## UI 验收 R8（当前）

[独立对象](../review/{rid}.md)与[完整报告](../../../../{report_rel})：通过验收，七单元均10/10、门槛8，UI-218独立复核已解决；未关闭差距0，无停滞或待裁定。目标指纹6c8b16d50cf7…，被测HEAD b9d5d356，产品指纹e492abef49d9…；340张原图、13张机器图、8位全新评审，U7补采后另派评审并取较低整组分与差距并集。

原R7及作者修复历史保留，UI-R7处置已接续独立关闭。Ready、7/7及实施事件集合保持，原计划只读；未登记产品终验、合并或发布，整期/参数/人工未验边界见报告。若结束打磨，下一步 `$review-code docs/plan/DOM-COMPONENTS-P1.md`；本轮不自行启动。新报告、证据和台账回写未提交、未推送。
"""
plan_status.write_text(text)
for f, row in [(s / "status/review/index.md", f"| {rid} | [DOM-COMPONENTS-P1 UI 验收评审 R8](../../../../{report_rel}) | [对象]({rid}.md) | 活动 |"), (s / "events/review/index.md", f"| {rid} | DOM-COMPONENTS-P1 UI 验收评审 R8 事件 | [入口]({rid}/index.md) |")]:
    text = f.read_text()
    anchor = next(x for x in text.splitlines() if x.startswith("| 53350840-"))
    f.write_text(replace(text, anchor, row + "\n" + anchor))
rows = sorted([(r["review_event"], "review"), (r["disposition_event"], "disposition")])
create(s / f"events/review/{rid}/index.md", "# DOM-COMPONENTS-P1 UI R8 · 事件索引\n\n本目录保存本轮UI验收事实；原报告与截图保留原位，index不是事件。\n\n| 事件 ID | facet / scope | 文件 |\n| --- | --- | --- |\n" + "".join(f"| {eid} | {facet} / UI-R8 | [事件]({eid}.md) |\n" for eid, facet in rows) + "\n遵循[目录索引协议](../../../protocol.md#目录索引维护)。\n")
f = s / f"events/review/{old}/index.md"
text = f.read_text()
anchor = "\n遵循[目录索引协议]"
f.write_text(replace(text, anchor, f"| {r['prior_disposition_event']} | disposition / UI-R7 | [事件]({r['prior_disposition_event']}.md) |\n" + anchor))
f = s / "index.md"
text = f.read_text()
a = "UI-218已修改待评审，不自行关闭。下一步独立review-ui，之后仍需review-code；原R7通过不覆盖此后CSS变更。"
b = f"作者当时将UI-218记为已修改待评审。[UI 验收 R8](status/review/{rid}.md)现已独立确认解决，七单元均10/10、门槛8，未关闭差距0；340张原图、13张机器图、8位全新评审，U7补采重评，目标指纹6c8b16d50cf7未变。R8覆盖当前CSS，原R7结论保留；若结束打磨，下一步review-code本循环UI修复。"
f.write_text(replace(text, a, b))
r["report_sha256"] = "sha256:" + sha(report)
r["written_files"] = [report_rel, f".xgent-ai/sdlc/status/review/{rid}.md", f".xgent-ai/sdlc/events/review/{rid}/{r['review_event']}.md", f".xgent-ai/sdlc/events/review/{rid}/{r['disposition_event']}.md", f".xgent-ai/sdlc/events/review/{rid}/index.md", f".xgent-ai/sdlc/events/review/{old}/{r['prior_disposition_event']}.md", *r["before"].keys()]
(p / "registration.json").write_text(json.dumps(r, ensure_ascii=False, indent=2) + "\n")
print(json.dumps({"review_id": rid, "new_events": 3, "report_sha256": r["report_sha256"]}))
