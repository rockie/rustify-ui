from pathlib import Path
from decimal import Decimal
import json,re,hashlib
p=Path(__file__).resolve().parent
scores={f'U{i}':['3.0','3.0','3.0','1.0'] for i in range(1,7)};scores['U7']=['3.0','2.7','3.0','1.0']
raw={u:(p/f'judge-{u}-a.md').read_text() for u in scores}
lines=['# DOM-COMPONENTS-P1 UI 验收 R6 · 独立评审结论（合并）','',
'## 评分','',
'七个单元分别由一位全新、只读上下文评审。原文独立保留；没有确认评审或补采重评。U5 追加的开图轨迹确认仅补齐清单，不重评、不改分。','',
'| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 单元分 | 原文 |','| --- | --- | --- | --- | --- | --- | --- |']
for u,v in scores.items():lines.append(f'| {u} | '+' | '.join(v)+f' | {sum(map(Decimal,v))} | [judge-{u}-a.md](judge-{u}-a.md) |')
for u,text in raw.items():
 start=re.search(r'(?m)^(?:## )?(?:\*\*)?1\. 评分(?:表)?',text);assert start,u
 end=re.search(r'(?m)^(?:## )?(?:\*\*)?2\. 上一轮',text);assert end,u
 lines.extend(['',f'### {u} · 四维理由（评审原文）','',text[start.start():end.start()].strip()])
text=raw['U7'];gap=text.split('### 新-2 · ',1)[1].split('\n## 4.',1)[0];gap='### UI-216 · '+gap
lines.extend(['','## 未关闭差距','',gap.strip(),'',
'新差距编号映射：U7 原文「新-2」→ UI-216；R5 的 U7「新-1」是 UI-214，本轮判已解决。两者不是同一问题，不复用编号。','',
'## 建议（不扣分）','',
'- U2：Neutral 影响清单可按 Light / Dark 分成更易扫描的小节；现有内容已列明范围。',
'- U4：长迁移 JSON 审阅可固定标题栏；目标未要求，现有操作仍可达。',
'- U1：可增加键盘确认品牌、键盘激活模式与拖拽分隔条记录。',
'- 其他矩阵外状态与未操作结果仍按各评审原文的「看不出的格子」保留为未知，不外推通过。'])
f=p/'judge.md';assert not f.exists();f.write_text('\n'.join(lines)+'\n')
old={'UI-115':'不再适用','UI-213':'已解决','UI-203':'已解决','UI-76':'已解决','UI-210':'已解决','UI-214':'已解决'}
results={'scores':{u:{'dimensions':v,'total':str(sum(map(Decimal,v))),'source':f'judge-{u}-a.md'} for u,v in scores.items()},'previous_review_gaps':old,'new_id_mappings':[{'unit':'U7','raw':'新-2','id':'UI-216','basis':'R6 Code ghost foreground discrepancy, distinct from R5 Kbd new-1/UI-214'}],'old_id_mappings':[{'unit':'U3','raw':'新-1（R5）','id':'UI-213','basis':'R5 merged judge explicitly maps this sticky-header remainder gap to UI-213'},{'unit':'U7','raw':'R5 新-1','id':'UI-214','basis':'R5 merged judge explicitly maps Kbd radius gap to UI-214'}],'raw_sha256':{u:hashlib.sha256((p/f'judge-{u}-a.md').read_bytes()).hexdigest() for u in scores},'minimum_unit':'U7','minimum_score':'9.7'}
(p/'judge-results.json').write_text(json.dumps(results,ensure_ascii=False,indent=2))
checks={}
for u in scores:
 text=raw[u]
 if u=='U5':text+='\n'+(p/'judge-U5-trace-supplement.md').read_text()
 opened=set(re.findall(r'/Users/[^\n`]+\.png',text));expected={str(x.resolve()) for x in p.glob(f'{u}-*.png')};checks[u]={'current_images':len(opened & expected),'expected':len(expected),'missing':sorted(expected-opened)}
assert all(not c['missing'] for c in checks.values()),checks
(p/'judge-evidence-check.json').write_text(json.dumps(checks,ensure_ascii=False,indent=2))
print(json.dumps({'scores':results['scores'],'evidence':checks},ensure_ascii=False))
