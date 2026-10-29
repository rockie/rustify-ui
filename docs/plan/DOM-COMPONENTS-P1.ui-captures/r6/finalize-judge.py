from pathlib import Path
from decimal import Decimal
import json, re, hashlib

p = Path(__file__).resolve().parent
scores = {f'U{i}': ['3.0', '3.0', '3.0', '1.0'] for i in range(1, 8)}
scores['U1'] = ['3.0', '3.0', '2.5', '1.0']
scores['U7'] = ['3.0', '2.7', '3.0', '1.0']
chosen = {'U1': 'c', 'U2': 'c', 'U3': 'a', 'U4': 'a', 'U5': 'b', 'U6': 'b', 'U7': 'a'}
attempts = {'U1': ['a', 'b', 'c'], 'U2': ['a', 'b', 'c'], 'U3': ['a'], 'U4': ['a'], 'U5': ['a', 'b'], 'U6': ['a', 'b'], 'U7': ['a', 'b']}
attempt_totals = {'U1': ['10', '9.6', '9.5'], 'U2': ['10', '10', '10'], 'U3': ['10'], 'U4': ['10'], 'U5': ['10', '10'], 'U6': ['10', '10'], 'U7': ['9.7', '10']}
raw = {u: (p / f'judge-{u}-{a}.md').read_text() for u, a in chosen.items()}
lines = ['# DOM-COMPONENTS-P1 UI 验收 R6 · 独立评审结论（最终合并）', '', '## 评分', '',
         '七个单元首审各一位全新只读评审；补采后 U1/U2/U5/U6/U7 各由另一位全新评审读取该单元完整材料。再补 U1 中文窄屏预览与 U2 窄屏低对比预览后，U1/U2 各由第三位全新评审完整复核。每次均只给冻结目标、本轮实拍和前轮 R5 独立材料。U5 同一评审的轨迹续接只补开图清单，不重评。', '',
         '每个单元取总分最低评审的一整组四维分及理由；同分时选覆盖完整材料的后评审。全部未关闭差距取并集，不因另一个评审未发现而删除。', '',
         '| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 单元分 | 各组总分 a/b/c | 采用原文 |',
         '| --- | --- | --- | --- | --- | --- | --- | --- |']
for u, dimensions in scores.items():
    total = str(sum(map(Decimal, dimensions)))
    assert Decimal(total) == min(map(Decimal, attempt_totals[u]))
    lines.append(f'| {u} | ' + ' | '.join(dimensions) + f' | {total} | {" / ".join(attempt_totals[u])} | [judge-{u}-{chosen[u]}.md](judge-{u}-{chosen[u]}.md) |')
for u, text in raw.items():
    start = re.search(r'(?m)^(?:#{1,3} )?(?:\*\*)?1\. 评分(?:表)?', text)
    end = re.search(r'(?m)^(?:#{1,3} )?(?:\*\*)?2\. 上一轮', text)
    assert start and end, u
    lines.extend(['', f'### {u} · 四维理由（采用评审原文）', '', text[start.start():end.start()].strip()])

u1_gap = raw['U1'].split('**新-1 · ', 1)[1].split('\n4. 建议', 1)[0]
u1_heading, u1_body = u1_gap.split('\n', 1)
u1_gap = u1_heading.removesuffix('**') + '\n' + u1_body
u7_gap = raw['U7'].split('### 新-2 · ', 1)[1].split('\n## 4.', 1)[0]
lines.extend(['', '## 未关闭差距', '', '### UI-217 · ' + u1_gap.strip(), '',
              '映射：U1-b 与 U1-c 的「新-1」均为 Pink 起点展开菜单后的活动项不可见；同一单元、同一三张图、同一交互偏差，合并为 UI-217。采用较低整组评分 U1-c 的可挽回 0.5；U1-b 的原始 0.4 保留在独立原文。', '',
              '### UI-216 · ' + u7_gap.strip(), '',
              '映射：U7-a「新-2」→ UI-216；U7-b 没有另列此项，但按差距并集保留 U7-a 的发现和 9.7 整组分。R5 的 Kbd「新-1」是 UI-214，本轮已解决，不复用编号。', '',
              '## 建议（不扣分）', '',
              '- U2：Neutral 影响清单可按 Light / Dark 分节；现有范围已经完整。',
              '- U4：长迁移 JSON 审阅可固定标题栏；目标未要求。',
              '- U1：可增加键盘激活模式、分隔条拖拽的记录；未执行结果按原文保留未知。',
              '- 其他未拍参数组合、操作结果及矩阵外断点，保持各原文的证据边界，不由满分外推。'])
(p / 'judge.md').write_text('\n'.join(lines) + '\n')

old = json.loads((p / 'revisions/judge-results-before-matrix-supplement.json').read_text())
old['scores'] = {u: {'dimensions': d, 'total': str(sum(map(Decimal, d))), 'source': f'judge-{u}-{chosen[u]}.md', 'all_attempt_totals': attempt_totals[u]} for u, d in scores.items()}
old['minimum_unit'] = 'U1'
old['minimum_score'] = '9.5'
old['new_id_mappings'].append({'unit': 'U1', 'raw': 'U1-b 新-1; U1-c 新-1', 'id': 'UI-217', 'basis': 'Same Pink-origin keyboard menu active option missing in the same three current frames; distinct from export/import selector UI-215'})
old['raw_sha256'] = {f'{u}-{a}': hashlib.sha256((p / f'judge-{u}-{a}.md').read_bytes()).hexdigest() for u, aa in attempts.items() for a in aa}
old['merge_rule'] = 'Minimum whole four-dimension group per unit, union of every reviewer gap; no dimension mixing; same-score latest complete evidence group.'
(p / 'judge-results.json').write_text(json.dumps(old, ensure_ascii=False, indent=2) + '\n')

checks = {}
all_opened = set()
for u, aa in attempts.items():
    for a in aa:
        task = p / 'judge-inputs' / (f'task-{u}.md' if a == 'a' else f'task-{u}-{a}.md')
        expected = set(re.findall(r'/Users/[^\n`]+/r6/' + u + r'-[^\n`]+\.png', task.read_text()))
        text = (p / f'judge-{u}-{a}.md').read_text()
        if u == 'U5' and a == 'a':
            text += '\n' + (p / 'judge-U5-trace-supplement.md').read_text()
        opened = set(re.findall(r'/Users/[^\n`]+\.png', text))
        missing = sorted(expected - opened)
        assert expected and not missing, (u, a, missing)
        checks[f'{u}-{a}'] = {'packet_current_images': len(expected), 'opened_current_images': len(expected & opened), 'missing': missing}
        all_opened |= expected & opened
expected_all = {str(x.resolve()) for x in p.glob('U[1-7]-*.png')}
assert all_opened == expected_all, sorted(expected_all - all_opened)
(p / 'judge-evidence-check.json').write_text(json.dumps({'attempts': checks, 'unique_current_images': len(all_opened), 'expected_current_images': len(expected_all), 'missing': []}, ensure_ascii=False, indent=2) + '\n')

dispatch = json.loads((p / 'revisions/judge-dispatch-before-matrix-supplement.json').read_text())
dispatch['supplement_agents'] = {f'{u}-{a}': f'/root/ui_r6_{u.lower()}_{a}' for u, aa in attempts.items() for a in aa if a != 'a'}
dispatch['supplement_reason'] = 'Required viewport/theme cells were omitted by initial script; final U1/U2 preview frames added after blind review evidence limits. No new product changes or score dispute.'
dispatch['all_raw_sha256'] = old['raw_sha256']
dispatch['packet_hashes'] = {f'{u}-{a}': hashlib.sha256((p / 'judge-inputs' / (f'task-{u}.md' if a == 'a' else f'task-{u}-{a}.md')).read_bytes()).hexdigest() for u, aa in attempts.items() for a in aa}
dispatch['confirmation_review'] = 'No threshold-near confirmation: final verdict requires modification because UI-217 is P1; every score is also above 8.5.'
dispatch['fresh_reviewers'] = 14
(p / 'judge-dispatch.json').write_text(json.dumps(dispatch, ensure_ascii=False, indent=2) + '\n')
print(json.dumps({'scores': old['scores'], 'unique_opened': len(all_opened), 'fresh_reviewers': 14}, ensure_ascii=False))
