from pathlib import Path
import json, hashlib, uuid, datetime, re

root = Path.cwd()
p = root / 'docs/plan/DOM-COMPONENTS-P1.ui-captures/r6'
sdlc = root / '.xgent-ai/sdlc'
r6 = '4096c476-5ee3-44a5-a195-09a5183542ca'
r5 = '2cc80246-5d61-4370-bbd7-462827ecbce3'
plan = '37a2c7b4-7052-4f85-96e9-135b6f7667a1'
head = 'd7ff86ce2111991f0a4e0b0ea3e131cb4a6bea03'
base = {'head': head, 'branch': 'main', 'worktree': 'dirty'}
now = datetime.datetime.now(datetime.timezone(datetime.timedelta(hours=8))).isoformat(timespec='seconds')
report = 'docs/plan/DOM-COMPONENTS-P1.ui-review.r6.md'
snapshot = 'docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/revisions/report-before-matrix-supplement.md'

def sha(path):
    return 'sha256:' + hashlib.sha256((root / path).read_bytes()).hexdigest()

def source(path, anchor):
    return {'path': path, 'revision': sha(path), 'anchor': anchor}

assert sha(snapshot) == 'sha256:85585f488ef1ebfd4aae566ae1cab1794894e0891c3eca80c6d58345367655a1'
assert '**需要修改**' in (root / report).read_text()
assert '9.5/10（U1）' in (root / report).read_text()
ids = {'review_correction': str(uuid.uuid4()), 'disposition_correction': str(uuid.uuid4()), 'r5_source_correction': str(uuid.uuid4())}
receipt = p / 'sdlc-correction.json'
assert not receipt.exists()
original_events = list((sdlc / 'events/review' / r6).glob('*.md')) + [sdlc / 'events/review' / r5 / '6b8201a4-fd97-46c4-9270-bb4bd92e7d25.md']
original_hashes = {str(f.relative_to(root)): sha(str(f.relative_to(root))) for f in original_events if f.name != 'index.md'}
receipt.write_text(json.dumps({'ids': ids, 'observed_at': now, 'report_sha256': sha(report), 'initial_report_snapshot': {'path': snapshot, 'sha256': sha(snapshot)}, 'immutable_original_events': original_hashes}, ensure_ascii=False, indent=2) + '\n')
relations = [{'type': 'reviews', 'target': plan, 'scope': 'P1 Studio 与 DOM 组件；U1–U7 全量'}, {'type': 'rechecks', 'target': r5, 'scope': 'R5 六条作者修复与 UI-115 已确认目标变更'}]
sources = [source(report, '验收判定、评分、上轮差距复核、UI-216/UI-217、采集与覆盖'), source('docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/judge.md', '14位独立评审合并；最低U1 9.5；差距并集'), source('docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/capture.md', '交付前矩阵补采与范围更正；236张原图、15张机器图'), source('docs/plan/DOM-COMPONENTS-P1.ui-target.r6.md', '冻结目标；规范指纹6c8b16d50cf7不变'), source(snapshot, '初版9.7/通过的完整原始内容；后续补采更正，不再代表当前全量结论'), source('docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/revisions/judge-before-matrix-supplement.md', '初版独立合并原文；对应旧事件的judge来源hash'), source('docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/machine-pink-menu-confirm.json', '390浅色和1440深色实测Plum在滚动视窗之外；768首测见另存文件')]

def write_event(object_id, event_id, facet, scope, state, source_state, predecessors, title, body, event_sources=sources, event_relations=relations):
    fields = {'schema_version': 3, 'event_id': event_id, 'object_id': object_id, 'kind': 'review', 'review_type': 'other', 'title': title, 'artifact': report if object_id == r6 else 'docs/plan/DOM-COMPONENTS-P1.ui-review.r5.md', 'facet': facet, 'scope': scope, 'state': state, 'source_state': source_state, 'evidence_status': 'confirmed', 'observed_at': now, 'predecessors': predecessors, 'baseline': base, 'sources': event_sources, 'relations': event_relations}
    text = '---\n' + '\n'.join(k + ': ' + json.dumps(v, ensure_ascii=False) for k, v in fields.items()) + '\n---\n\n' + body + '\n'
    f = sdlc / 'events/review' / object_id / f'{event_id}.md'
    with f.open('x') as out:
        out.write(text)

write_event(r6, ids['review_correction'], 'review', 'UI-R6', 'review-recorded', '需要修改；9.5/10（U1），门槛8；P1 UI-217、P2 UI-216', ['eac1086e-aaec-4eea-b1ac-c6021ce9c4d5'], 'DOM-COMPONENTS-P1 UI R6 · 矩阵补证与判定更正', '''# 同轮补证与判定更正

- 首批154张覆盖步骤但遗漏部分窄屏/深色组合，初版暂存为通过9.7。交付前回读矩阵发现后补82张，当前236张独立原图、15张后置机器原图、14位全新独立评审；全部原图与原始结论保留。
- 最低整组分变为U1的9.5。U1补评发现UI-217/P1：Pink起点展开菜单并按方向键，活动Plum不在列表可视区，三个组合实测scrollTop=0。UI-216/P2按差距并集保留；其余五单元10、U7为9.7。当前需要修改，不能沿用初版通过。
- 初版报告可取回快照：[原报告](../../../../../docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/revisions/report-before-matrix-supplement.md)。其hash与旧事件来源完全一致；初版judge也另存快照。旧事件均未覆盖，本事件明确接续并更正其范围与结论。
- [最终报告](../../../../../docs/plan/DOM-COMPONENTS-P1.ui-review.r6.md)和原图逐一可达。目标期望未改，仍是UI-115接受后的6c8b16d50cf7基线；R5六条修复关闭、UI-115不再适用保持。
- 本机headless Chromium的冻结UI矩阵结论；完整参数/人工辅助技术等未验见报告。产品代码、原计划与历史轮报告未改，服务已停，无提交/推送，不登记合并、发布或终验。''')
write_event(r6, ids['disposition_correction'], 'disposition', 'UI-R6', 'review-pending', 'UI-217/P1与UI-216/P2待处理；本轮未执行apply', ['4fff6d8e-db53-488e-b0db-eb8717e1df16'], 'DOM-COMPONENTS-P1 UI R6 · 处置校准为待处理', '''# 当前需要处理

- 同轮补证后的[最终报告](../../../../../docs/plan/DOM-COMPONENTS-P1.ui-review.r6.md)需要修改；UI-217/P1阻止验收，UI-216/P2也仍成立。尚无这两项处置记录，按状态模型登记review-pending。
- 原review-not-required基于缺少部分矩阵的暂存报告，已由本事件接续更正；原报告快照及旧事件保留，不假装已执行修复。
- 下一步由用户发起 `$apply-ui-review docs/plan/DOM-COMPONENTS-P1.ui-review.r6.md`，修复后复审。本轮不自行开始apply或review-code。''')
write_event(r5, ids['r5_source_correction'], 'disposition', 'UI-R5', 'review-applied', 'R6补证后六条旧修复仍已解决；UI-115接受偏差；新增UI-216/UI-217归R6', ['6b8201a4-fd97-46c4-9270-bb4bd92e7d25'], 'DOM-COMPONENTS-P1 UI R5 · 复核来源版本续接', '''# R5处置保持，复核来源续接

- R6交付前补采更正了整轮判定；[最终报告的上轮复核](../../../../../docs/plan/DOM-COMPONENTS-P1.ui-review.r6.md#上轮差距复核)仍逐条确认UI-215、UI-214、UI-213、UI-203、UI-76、UI-210已解决，UI-115按用户接受不再适用。R5本scope仍为review-applied，R5原9.6及需要修改的原判定不改。
- R6新增UI-217/P1、UI-216/P2由R6对象待处理，不倒写为R5原问题。当前R6判需要修改，不再建议因初版通过而结束UI打磨。
- 前序事件使用的初版报告内容保留于[精确快照](../../../../../docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/revisions/report-before-matrix-supplement.md)，本事件引用最终来源版本。没有追加代码修改、原计划进度或合并发布事实。''', event_relations=[{'type': 'reviews', 'target': plan, 'scope': 'R5六条修复的R6最终复核'}, {'type': 'depends-on', 'target': r6, 'scope': 'R6最终上轮差距复核'}])

all_r6 = ['eac1086e-aaec-4eea-b1ac-c6021ce9c4d5', '4fff6d8e-db53-488e-b0db-eb8717e1df16', ids['review_correction'], ids['disposition_correction']]
fields = {'object_id': r6, 'kind': 'review', 'review_type': 'other', 'artifact': report, 'archived': False, 'observed_at': now, 'baseline': base, 'included_events': all_r6, 'relations': relations}
view = '---\n' + '\n'.join(k + ': ' + json.dumps(v, ensure_ascii=False) for k, v in fields.items()) + '\n---\n\n'
view += f'''# DOM-COMPONENTS-P1 UI 验收评审 R6

来源：[最终报告](../../../../{report})（{sha(report)}）、[全部截图与独立原文](../../../../docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/)、[R5](../review/{r5}.md)、[计划](../plan/{plan}.md)。

| 维度 / 范围 | 固定状态码 | 来源自述 | 证据质量 | 下一步 |
| --- | --- | --- | --- | --- |
| review / UI-R6 | review-recorded | 需要修改；9.5/10（U1），门槛8 | confirmed | [最终补证事件](../../events/review/{r6}/{ids['review_correction']}.md) |
| disposition / UI-R6 | review-pending | UI-217/P1、UI-216/P2待处理 | confirmed | apply-ui-review后复审；本轮未执行修复 |

- UI验收登记为other。被测HEAD `{head}`，产品diff/untracked为空；Studio dff085ce38bbff64、catalog 32b52b66f04701ad。
- UI-115接受后重建基线，规范指纹6c8b16d50cf7不变；U1为9.5，U7为9.7，其余五单元10。236张矩阵原图、15张机器原图、14位全新独立评审；取最低整组分和差距并集。
- 新增UI-217/P1阻止通过：Pink键盘展开后活动项在列表可视区外；UI-216/P2保留。R5六条修复关闭、UI-115不再适用保持；R5历史评分与原判定不改。
- 首批154张遗漏部分矩阵而暂存的通过9.7已更正；[旧报告精确快照](../../../../{snapshot})与[旧judge快照](../../../../docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/revisions/judge-before-matrix-supplement.md)均可取回。旧事件保留，新事件明确接续，不按时间覆盖事实。
- 临时服务已停止；完整未知、扫描接受与辅助技术边界见报告。Ready/7/7不变，不登记合并、发布或产品终验。资料未提交/推送，声明范围仅当前工作树。

## 已纳入事件

'''
for event_id in all_r6:
    view += f'- [{event_id}](../../events/review/{r6}/{event_id}.md)\n'
(sdlc / 'status/review' / f'{r6}.md').write_text(view)

f = sdlc / 'status/review' / f'{r5}.md'
text = f.read_text()
old_events = json.loads(re.search(r'^included_events: (.+)$', text, re.M).group(1))
text = re.sub(r'^observed_at: .+$', 'observed_at: ' + json.dumps(now), text, count=1, flags=re.M)
text = re.sub(r'^included_events: .+$', 'included_events: ' + json.dumps(old_events + [ids['r5_source_correction']]), text, count=1, flags=re.M)
text = text.replace(f'/6b8201a4-fd97-46c4-9270-bb4bd92e7d25.md)；R5原评分与判定保留', f'/{ids["r5_source_correction"]}.md)；R5原评分与判定保留')
old = 'R6新UI-216/P2为可选打磨，不写入旧报告。'
assert old in text
text = text.replace(old, 'R6最终补证判需要修改9.5，新增UI-217/P1及UI-216/P2由R6对象待处理，不写入旧报告；六条旧修复的关闭不受新增问题影响。')
text += f'\n- [最终来源版本续接](../../events/review/{r5}/{ids["r5_source_correction"]}.md) · disposition/UI-R5；初版来源精确快照见R6补证事件。\n'
f.write_text(text)

for object_id, label in [(r6, 'DOM-COMPONENTS-P1 UI R6'), (r5, 'DOM-COMPONENTS-P1 UI R5')]:
    directory = sdlc / 'events/review' / object_id
    lines = [f'# {label} · 事件索引', '', '本目录保存该轮评审及处置事实，报告与证据保留原位；index不是事件。', '', '| 事件 ID | facet / scope | 文件 |', '| --- | --- | --- |']
    for event in sorted(directory.glob('*.md')):
        if event.name == 'index.md':
            continue
        data = event.read_text()
        event_id = json.loads(re.search(r'^event_id: (.+)$', data, re.M).group(1))
        facet = json.loads(re.search(r'^facet: (.+)$', data, re.M).group(1))
        scope = json.loads(re.search(r'^scope: (.+)$', data, re.M).group(1))
        lines.append(f'| {event_id} | {facet} / {scope} | [事件]({event.name}) |')
    lines += ['', '遵循[目录索引协议](../../../protocol.md#目录索引维护)。', '']
    (directory / 'index.md').write_text('\n'.join(lines))

f = sdlc / 'status/plan' / f'{plan}.md'
text = f.read_text()
old = 'R6通过9.7，UI-216/P2可选；后续代码复审'
assert old in text
text = text.replace(old, 'R6最终需要修改9.5，新增UI-217/P1与UI-216/P2待处理；后续UI修复复审')
start = text.index('## UI 验收 R6（当前）')
text = text[:start] + f'''## UI 验收 R6（当前）

[独立评审对象](../review/{r6}.md)与[最终报告](../../../../{report})：需要修改，最低U1为9.5、门槛8；U7为9.7，其余五单元10。P1为新增UI-217（已选品牌色后活动项在列表可视区之外），另有UI-216/P2。R5六条旧修复已关闭，UI-115按用户接受不再适用。初版通过9.7因遗漏部分矩阵而补采更正，原报告快照及旧事件保留；最终236张矩阵图、15张机器图、14位全新独立评审。

原计划恢复快照仍指向发起UI复审，本轮按review-ui只读边界未改原文；本页只记录实际完成的独立复审，不改变Ready与7/7。下一步 `$apply-ui-review docs/plan/DOM-COMPONENTS-P1.ui-review.r6.md`，修复后再review-ui。原资料与台账未提交、未推送；不另造计划完成/产品终验/发布事件。
'''
text = re.sub(r'^observed_at: .+$', 'observed_at: ' + json.dumps(now), text, count=1, flags=re.M)
f.write_text(text)

f = sdlc / 'index.md'
text = f.read_text()
start = text.index(f'[UI 验收 R6](status/review/{r6}.md)')
end = text.index('Ready与7/7保持', start)
text = text[:start] + f'[UI 验收 R6](status/review/{r6}.md)（重建基线）最终需要修改，最低U1为9.5、门槛8；U7为9.7，其余五单元10。R5六条修复已解决，UI-115按用户接受不再适用；新增UI-217/P1（已选品牌色后键盘活动项不可见）阻止通过，另有UI-216/P2。首批遗漏矩阵的暂存通过9.7已用补证事件更正，旧报告快照和事件保留；最终236张矩阵与15张机器原图、14位独立评审。下一步 `$apply-ui-review docs/plan/DOM-COMPONENTS-P1.ui-review.r6.md`，修后复审。' + text[end:]
text = text.replace('均已修改待评审，UI-215/P1是否关闭交独立复审', '六条已由R6独立复核关闭，UI-215/P1已解决', 1)
f.write_text(text)
for path, checksum in original_hashes.items():
    assert sha(path) == checksum, path
print(json.dumps({'ids': ids, 'report_sha256': sha(report), 'original_events_unchanged': True}, ensure_ascii=False))
