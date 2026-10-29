from pathlib import Path
from decimal import Decimal
import json,re,hashlib
p=Path(__file__).resolve().parent
scores={
'U1':['3.0','3.0','3.0','1.0'],
'U2':['3.00','3.00','2.95','1.00'],
'U3':['2.95','3.00','3.00','0.95'],
'U4':['3.00','3.00','3.00','0.95'],
'U5':['2.95','3.00','3.00','1.00'],
'U6':['3.0','2.99','3.0','1.0'],
'U7':['3.0','2.6','3.0','1.0']}
selected={u:f'judge-{u}-a.md' for u in scores};selected['U2']='judge-U2-b.md'
old={'UI-02':'已解决','UI-193':'已解决','UI-194':'已解决','UI-172':'已解决','UI-195':'已解决','UI-196':'已解决','UI-18':'已解决','UI-115':'仍存在','UI-197':'已解决','UI-160':'已解决','UI-128':'已解决','UI-198':'已解决','UI-176':'已解决','UI-199':'已解决','UI-200':'已解决','UI-25':'已解决','UI-201':'已解决','UI-203':'部分解决','UI-202':'已解决','UI-204':'已解决','UI-205':'已解决','UI-56':'已解决','UI-206':'已解决','UI-207':'已解决','UI-76':'回归','UI-208':'已解决','UI-209':'已解决','UI-210':'部分解决','UI-189':'已解决','UI-211':'已解决'}
mapping={'U3 新-1':{'id':'UI-213','basis':'U3 reviewer new sticky-header residual-row gap; no prior allocated gap matches.'},'U7 新-1':{'id':'UI-214','basis':'U7 reviewer newly discovered Kbd large-size corner radius gap; not a regression.'}}
score_ranges={'U1':('1. 评分表','2. 上一轮'),'U2':('**1. 评分**','**2. 上一轮'),'U3':('**1. 评分表**','**2. 上一轮'),'U4':('1. 评分表','2. 上一轮'),'U5':('## 1. 评分','## 2. 上一轮'),'U6':('## 评分表','## 上一轮'),'U7':('## 1. 评分表','## 2. 上一轮')}
gap_ranges={'U2':('**3. 未关闭差距**','**4. 建议'),'U3':('**3. 差距清单**','**4. 建议'),'U4':('3. 差距清单','4. 建议'),'U5':('## 3. 未关闭差距','## 4. 建议'),'U6':('## 差距清单','## 建议'),'U7':('## 3. 差距清单','## 4. 建议')}
text='# DOM-COMPONENTS-P1 UI 验收 R5 · 独立评审结论（合并）\n\n'
text+='来源为 8 份独立原文（每单元一位，U2 因评审提出缺证，补采后另派一位）。原文各自保存、不覆盖；模型继承宿主默认、fork_turns=none、评审不读代码或修复记录，不再派生。U7 首次容量异常后原模型重试获得有效结论。\n\n'
text+='- 原文：'+ '、'.join(f'[{f.name}]({f.name})' for f in sorted(p.glob('judge-U*-*.md')))+'。\n'
text+='- U2 首采 A 与补采后 B 都为 9.95，采用 B 的整组四维分与理由。A 的 UI-172 待核实源于入口未展开；补采新增两张原图后，B 逐条核实并判已解决。这是追加材料后的缺证复核，不是同批材料的确认分歧；A 原文与原 11 张清单保持不变。UI-115 的两份结论均完整保留。\n'
text+='- UI-76 原文为“部分解决，横向回归”，合并状态按更严格的“回归”记录，保留纵向已改善的说明。\n'
text+='- 新差距编号：U3 新-1 → UI-213；U7 新-1 → UI-214。\n\n## 评分\n\n| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 单元分 | 采用原文 |\n| --- | --- | --- | --- | --- | --- | --- |\n'
for u,vals in scores.items():
 total=sum(map(Decimal,vals));text+='| '+u+' | '+' | '.join(vals)+f' | {total} | [{selected[u]}]({selected[u]}) |\n'
text+='\n## 各单元评分与理由原文\n\n'
for u,fn in selected.items():
 raw=(p/fn).read_text();start,end=score_ranges[u];part=raw.split(start,1)[1].split(end,1)[0].strip();text+=f'### {u}\n\n'+part+'\n\n'
text+='## 上轮差距复核\n\n| 编号 | 本轮独立评审结论 |\n| --- | --- |\n'
for id,status in old.items():text+=f'| {id} | {status} |\n'
text+='\n逐条依据保留在对应单元的评审原文复核表。UI-172 采用补采后的 B 结论；UI-76 的横向回归与纵向部分改善均保留。\n\n## 全部未关闭差距（独立评审来源）\n\n'
for u in ['U7','U3','U2','U4','U5','U6']:
 raw=(p/selected[u]).read_text();a,b=gap_ranges[u];body=raw.split(a,1)[1].split(b,1)[0].strip()
 if u=='U3':body=body.replace('新-1','UI-213')
 if u=='U7':body=body.replace('新-1','UI-214')
 text+=f'### {u}\n\n'+body+'\n\n'
text+='## 建议（不扣分）\n\n'
text+='- U1：可增加键盘确认选择、模式激活与分隔条拖拽记录。\n'
text+='- U2：展开后的 Neutral 清单可将 Light/Dark 分成更清楚的小节，当前清单已完整列出范围。\n'
text+='- U4：长迁移内容审阅时可固定标题栏，目标未要求，不扣分。\n'
text+='- U3、U5、U6、U7：没有额外产品设计建议。未拍摄的额外行为均按各原文“看不出的格子”保留为未知，不外推通过或失败。\n'
(p/'judge.md').write_text(text)
data={'scores':{u:{'dimensions':v,'total':str(sum(map(Decimal,v))),'source':selected[u]} for u,v in scores.items()},'previous_review_gaps':old,'new_id_mappings':mapping,'raw_sha256':{f.name:hashlib.sha256(f.read_bytes()).hexdigest() for f in sorted(p.glob('judge-U*-*.md'))}}
(p/'judge-results.json').write_text(json.dumps(data,ensure_ascii=False,indent=2)+'\n')
print('Saved merged judge with',len(old),'old gap results; scores:',{u:str(sum(map(Decimal,v))) for u,v in scores.items()})
