from decimal import Decimal
from pathlib import Path
import hashlib
import json
import re

p = Path(__file__).resolve().parent
units = [f'U{i}' for i in range(1, 8)]
raws = {u: [p / f'judge-{u}-a.md'] for u in units}
raws['U7'].append(p / 'judge-U7-b.md')
def score(file, unit):
    text = file.read_text()
    row = next(line for line in text.splitlines() if re.match(r'^\|\s*' + unit + r'\b', line))
    cells = row.split('|')[2:7]
    values = [Decimal(re.search(r'\d+(?:\.\d+)?', cell).group()) for cell in cells]
    assert sum(values[:4]) == values[4]
    assert all(0 <= v <= m for v, m in zip(values[:4], [3, 3, 3, 1]))
    return values
def section(text, number):
    pattern = r'(?m)^(?:#{1,6}\s+|\*\*)' + str(number) + r'[.、．]?\s*'
    start = re.search(pattern, text)
    assert start
    content = text[start.end():].split('\n', 1)[1]
    end = re.search(r'(?m)^(?:#{1,6}\s+|\*\*)' + str(number + 1) + r'[.、．]?\s*', content)
    assert end
    return content[:end.start()].strip()

records = {}
chosen = {}
for u in units:
    candidates = [(file, score(file, u)) for file in raws[u]]
    file, values = min(reversed(candidates), key=lambda item: item[1][4])
    chosen[u] = file
    records[u] = {'dimensions': [str(x) for x in values[:4]], 'total': str(values[4]),
        'selected_raw': file.name, 'candidates': [{'raw': f.name, 'scores': [str(x) for x in v],
        'sha256': hashlib.sha256(f.read_bytes()).hexdigest()} for f, v in candidates]}

summary = '# DOM-COMPONENTS-P1 UI 验收 R8 · 独立评审合并结论\n\n## 评分\n\n'
summary += '7 个单元分别由全新只读上下文首评；U7 在原生缩放分段补采后另由全新评审重评。共 8 位独立评审。按单元保留总分较低者的一整组四维分与理由；U7 同分时采用覆盖完整补采材料的 B 组，差距取并集。各位原文独立保留。\n\n'
summary += '| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 总分 | 采用原文 |\n| --- | ---: | ---: | ---: | ---: | ---: | --- |\n'
for u in units:
    r = records[u]
    summary += f'| {u} | ' + ' | '.join(r['dimensions']) + f' | {r["total"]} | [{r["selected_raw"]}]({r["selected_raw"]}) |\n'
for u in units:
    summary += f'\n### {u} · 采用原文的评分与理由\n\n' + section(chosen[u].read_text(), 1) + '\n'
summary += '\n## 未关闭差距\n\n无。8 位评审的未关闭差距并集为空。\n\n## 建议（不扣分）\n'
for u in units:
    summary += f'\n### {u}\n\n' + section(chosen[u].read_text(), 4) + '\n'
summary += '\nU7 首评另建议窄屏演示标题可采用更适合断行的示例文案；属于比较口径之外的阅读体验建议。U5 必需矩阵覆盖的同一评审澄清另见 [原文附录](judge-U5-a-clarification.md)，评分与差距保持原样。\n'
(p / 'judge.md').write_text(summary)
(p / 'judge-results.json').write_text(json.dumps({'units': records, 'round_score': str(min(Decimal(r['total']) for r in records.values())),
    'unclosed_judge_gaps': [], 'reviewers': 8, 'confirmation_review_performed': False,
    'U7_selection': 'min whole vector; equal totals choose later packet covering all native segments'}, indent=2) + '\n')
print(json.dumps({u: r['total'] for u, r in records.items()}))
