from pathlib import Path
from decimal import Decimal
import re,json,hashlib
p=Path(__file__).resolve().parent
chosen={'U1':'a','U2':'a','U3':'b','U4':'b','U5':'a','U6':'a','U7':'b'}
raws=sorted(p.glob('judge-U?-?.md'));assert len(raws)==10
scores={u:['3','3','3','1'] for u in chosen};scores['U5']=['3','2.8','3','1']
headers={'U1':('### 1. 评分表','### 2.'),'U2':('## 1. 评分表','## 2.'),'U3':('## 1. 评分表','## 2.'),'U4':('1. **评分表**','2. **'),'U5':('## 1. 评分','## 2.'),'U6':('**1. 评分**','**2.'),'U7':('## 1. 评分表','## 2.')}
s='# DOM-COMPONENTS-P1 UI 验收 R7 · 独立评审结论\n\n## 评分\n\n7 个单元分别由全新只读上下文首评；U3、U4、U7 补采后分别再由全新评审读取完整本单元材料。共 10 位独立评审。单元取总分较低的一整组四维分与理由；同分时取覆盖补采材料的后评审；未关闭差距取并集。\n\n| 单元 | 布局 | 视觉 | 交互与状态 | 细节 | 总分 | 首评/补评 | 采用原文 |\n| --- | --- | --- | --- | --- | --- | --- | --- |\n'
results={'scores':{},'old_gap_reviews':{'UI-216':{'unit':'U7','status':'已解决','sources':['judge-U7-a.md','judge-U7-b.md']},'UI-217':{'unit':'U1','status':'已解决','sources':['judge-U1-a.md']}},'new_id_mappings':[{'unit':'U5','raw':'新-1','id':'UI-218','basis':'未标注部件拾取框为灰色虚线；此前未作为差距编号登记，本轮U5独立评审新识别。'}],'raw_sha256':{x.name:hashlib.sha256(x.read_bytes()).hexdigest() for x in raws}}
for u,a in chosen.items():
 dims=scores[u];total=sum(map(Decimal,dims));src=f'judge-{u}-{a}.md';attempt='10 / 10' if u in ['U3','U4','U7'] else str(total)
 s+='| '+u+' | '+' | '.join(dims)+f' | {total} | {attempt} | [{src}]({src}) |\n'
 results['scores'][u]={'dimensions':dims,'total':str(total),'source':src}
for u,a in chosen.items():
 raw=(p/f'judge-{u}-{a}.md').read_text();start,end=headers[u];part=raw.split(start,1)[1].split(end,1)[0].strip();s+=f'\n### {u} · 四维理由（采用评审原文）\n\n'+part+'\n'
raw=(p/'judge-U5-a.md').read_text();gap=raw.split('### 新-1',1)[1].split('## 4.',1)[0].strip()
s+='\n## 未关闭差距\n\n### UI-218 '+gap+'\n\n映射：U5-a「新-1」→ UI-218；按同一位置、同一未标注拾取框及固定蓝色要求登记。\n'
s+='\n## 建议（不扣分）\n\n- U1：拍摄 Enter 后的额外展开帧前先等待列表可见；已有方向键展开帧支持本轮活动项判断。\n- U2：无新增设计建议。\n- U3、U4：无新增产品修改建议；未执行行为保留在各独立原文的证据边界中。\n- U5：可补固定内容宽度、浮层拾取和真实键盘操作帧。\n- U6：可补 IconButton、Link 与各变体附加列的完整参数组合及 ThemePanel 非零覆盖状态；本轮不外推这些组合。\n- U7：可补 95% 独立分段、更多缩放排版、Reset 点击与其他断点边界。\n'
(p/'judge.md').write_text(s);results['minimum_unit']='U5';results['minimum_score']='9.8';results['open_gaps']=[{'id':'UI-218','unit':'U5','severity':'P2','recoverable':'0.2','raw':'judge-U5-a.md'}];(p/'judge-results.json').write_text(json.dumps(results,ensure_ascii=False,indent=2)+'\n')
print(json.dumps({'scores':{u:v['total'] for u,v in results['scores'].items()},'raw_judgments':len(raws),'old_gap_reviews':results['old_gap_reviews'],'open_gaps':results['open_gaps']},ensure_ascii=False))
