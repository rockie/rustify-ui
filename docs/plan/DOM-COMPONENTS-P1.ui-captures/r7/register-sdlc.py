from pathlib import Path
from datetime import datetime
from zoneinfo import ZoneInfo
import hashlib,json,re,uuid,subprocess
root=Path.cwd();p=root/'docs/plan/DOM-COMPONENTS-P1.ui-captures/r7';base=root/'.xgent-ai/sdlc';intent=json.loads((p/'sdlc-intent.json').read_text());rid=intent['review_id'];old=intent['prior_review_id'];plan='37a2c7b4-7052-4f85-96e9-135b6f7667a1';artifact='docs/plan/DOM-COMPONENTS-P1.ui-review.r7.md'
for key in ['review_event','disposition_event','r6_closure_event']:
 intent.setdefault(key,str(uuid.uuid4()))
intent.setdefault('observed_at',datetime.now(ZoneInfo('Asia/Shanghai')).isoformat());(p/'sdlc-intent.json').write_text(json.dumps(intent,indent=2)+'\n');now=intent['observed_at']
head=subprocess.check_output(['git','rev-parse','HEAD'],text=True).strip();branch=subprocess.check_output(['git','branch','--show-current'],text=True).strip() or 'detached';baseline={'head':head,'branch':branch,'worktree':'dirty' if subprocess.check_output(['git','status','--porcelain']) else 'clean'}
sha=lambda f:hashlib.sha256(f.read_bytes()).hexdigest()
def readfield(t,key):
 m=re.search(r'^'+re.escape(key)+r': (.+)$',t,re.M);assert m,key;return json.loads(m.group(1))
old_dir=base/'events/review'/old;old_events={f.stem:f for f in old_dir.glob('*.md') if re.fullmatch(r'[0-9a-f-]{36}',f.stem)};old_status=base/'status/review'/f'{old}.md';old_text=old_status.read_text();assert set(readfield(old_text,'included_events'))==set(old_events)
immutable={str(f.relative_to(root)):sha(f) for f in old_events.values()};immutable.update({path:sha(root/path) for path in ['docs/plan/DOM-COMPONENTS-P1.md','docs/plan/DOM-COMPONENTS-P1.records/M7.md','docs/plan/DOM-COMPONENTS-P1.ui-review.r6.md']});(p/'sdlc-immutable-before.json').write_text(json.dumps(immutable,indent=2)+'\n')
assert readfield((old_events['5b8ab8f7-df0c-4bf7-9011-39dcb62ad7f0']).read_text(),'facet')=='disposition'
relations=[{'type':'reviews','target':plan,'scope':'P1 Studio 与 DOM 组件；U1–U7 全量'},{'type':'rechecks','target':old,'scope':'UI-R6 两条 UI-217、UI-216 独立复核'}]
def source(path,anchor):return {'path':path,'revision':'sha256:'+sha(root/path),'anchor':anchor}
sources=[source(artifact,'验收判定、评分、UI-218、上轮差距复核、机器检查与覆盖边界'),source('docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/judge.md','10位独立评审；最低U5 9.8；UI-216/UI-217已解决；UI-218/P2'),source('docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/capture.md','278张真实原图与逐图出处；被测版本b218075a'),source('docs/plan/DOM-COMPONENTS-P1.ui-target.r7.md','规范指纹6c8b16d50cf7，与R6相同；本轮仅增加采集')]
def front(data):return '---\n'+'\n'.join(k+': '+json.dumps(v,ensure_ascii=False) for k,v in data.items())+'\n---\n\n'
def put_new(path,text):
 if path.exists():assert path.read_text()==text,('refuse immutable overwrite',str(path))
 else:path.parent.mkdir(parents=True,exist_ok=True);path.write_text(text)
def event(obj,eid,facet,scope,state,source_state,pred,title,body,src=sources,rels=relations,art=artifact):
 data={'schema_version':3,'event_id':eid,'object_id':obj,'kind':'review','review_type':'other','title':title,'artifact':art,'facet':facet,'scope':scope,'state':state,'source_state':source_state,'evidence_status':'confirmed','observed_at':now,'predecessors':pred,'baseline':baseline,'sources':src,'relations':rels};f=base/'events/review'/obj/f'{eid}.md';put_new(f,front(data)+body+'\n');return f
review_event=event(rid,intent['review_event'],'review','UI-R7','review-recorded','通过验收；9.8/10（U5），门槛8；P0/P1为0，P2为1',[],'DOM-COMPONENTS-P1 UI 验收评审 R7',f'''# UI 验收评审 R7：冻结矩阵通过

- [完整报告](../../../../../{artifact})：7单元、278张本轮原图、14张后置机器原图、10位全新只读独立评审；最低U5为9.8。门槛8，无P0/P1；UI-216/UI-217已解决，新识别UI-218/P2。
- 被测HEAD `{head}`，产品diff/untracked为空、源码指纹见[版本记录](../../../../../docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/tested-version.json)；规范指纹6c8b16d50cf7，与R6相同，不重建基线。
- 按other登记UI验收，不登记合并、发布或整期产品终验；完整参数/断点、未操作行为及人工边界见报告。临时服务已停止。
- 下一步由用户选择是否打磨UI-218；继续打磨用apply-ui-review，否则先review-code本循环UI修复代码。本轮未修产品、未改计划、未提交或推送。''')
disposition_event=event(rid,intent['disposition_event'],'disposition','UI-R7','review-not-required','通过验收；无需强制apply；UI-218仍为可选P2',[],'DOM-COMPONENTS-P1 UI R7 · 通过后的可选打磨',f'''# 无强制 apply，保留可选差距

- [报告判定](../../../../../{artifact}#验收判定)为通过验收。按state-model v2“通过优先”的处置顺序登记review-not-required；没有伪造执行过apply。
- UI-218/P2保持未关闭：未标注拾取框仍为灰色虚线。用户尚未选择打磨，也没有修复或整体豁免，不将其记为已解决。
- Ready与7/7、旧代码评审基线保持；当前UI修复代码仍需独立review-code，不以UI通过替代。''')
old_relations=readfield(old_text,'relations');closure=event(old,intent['r6_closure_event'],'disposition','UI-R6','review-applied','UI-217、UI-216两条经R7独立复核已解决',['5b8ab8f7-df0c-4bf7-9011-39dcb62ad7f0'],'DOM-COMPONENTS-P1 UI R6 · R7独立复核关闭',f'''# R6 两条修复获得独立复核

- [R7报告](../../../../../{artifact}#上轮差距复核)分别确认UI-217/P1、UI-216/P2已解决；[M7作者处置](../../../../../docs/plan/DOM-COMPONENTS-P1.records/M7.md#ui-验收修复)原“已修改待评审”留存，不覆盖原文。
- UI-217由U1独立评审及真实菜单边界测量复核；UI-216由两位U7评审对照两视口、目标和R6原图复核。
- UI-R6处置接续为review-applied；R6原9.5/需要修改不改写。R7新识别UI-218归R7，不并入R6原差距。
- Ready/7/7不变，没有新增产品终验、合并、发布或代码复审通过事实。''',src=sources+[source('docs/plan/DOM-COMPONENTS-P1.records/M7.md','UI验收修复 / R6；两条已修改待评审')],rels=old_relations,art='docs/plan/DOM-COMPONENTS-P1.ui-review.r6.md')
# New current-state page, distinct from the immutable events.
data={'object_id':rid,'kind':'review','review_type':'other','artifact':artifact,'archived':False,'observed_at':now,'baseline':baseline,'included_events':[intent['review_event'],intent['disposition_event']],'relations':relations}
new_state=front(data)+f'''# DOM-COMPONENTS-P1 UI 验收评审 R7

来源：[完整报告](../../../../{artifact})（sha256:{sha(root/artifact)}）、[实拍与独立原文](../../../../docs/plan/DOM-COMPONENTS-P1.ui-captures/r7/)、[R6](../review/{old}.md)、[计划](../plan/{plan}.md)。

| 维度 / 范围 | 固定状态码 | 来源自述 | 证据质量 | 下一步 |
| --- | --- | --- | --- | --- |
| review / UI-R7 | review-recorded | 通过验收；9.8/10，门槛8 | confirmed | [评审事件](../../events/review/{rid}/{intent['review_event']}.md) |
| disposition / UI-R7 | review-not-required | 无须强制apply；保留UI-218/P2可选打磨 | confirmed | [处置校准](../../events/review/{rid}/{intent['disposition_event']}.md)；用户选打磨或代码复审 |

- 被测HEAD `{head}`；目标指纹6c8b16d50cf7与R6相同。278张真实原图、14张机器图、10位全新独立评审；U1/U2/U3/U4/U6/U7均10，U5为9.8。
- UI-216/UI-217独立复核已解决。UI-218仍开放P2，不作修复或豁免；按状态模型通过优先记review-not-required，意为没有强制apply，不是差距被抹除。
- 完整参数组合、所有断点、未操作行为、人工辅助技术与整期V1–V10保留报告边界；未记产品终验、合并或发布。
- 原计划Ready/7/7与历史报告保持。若继续打磨：`$apply-ui-review docs/plan/DOM-COMPONENTS-P1.ui-review.r7.md`；否则先`$review-code docs/plan/DOM-COMPONENTS-P1.md`。本轮没有执行上述后续命令。
- 报告、截图与台账未提交或推送；仅当前工作树可见。

## 已纳入事件

- [{intent['review_event']}](../../events/review/{rid}/{intent['review_event']}.md) · review/UI-R7
- [{intent['disposition_event']}](../../events/review/{rid}/{intent['disposition_event']}.md) · disposition/UI-R7
'''
put_new(base/'status/review'/f'{rid}.md',new_state)
# R6 view retains its original review verdict and historical author record.
t=old_status.read_text();ids=readfield(t,'included_events');ids.append(intent['r6_closure_event']);t=re.sub(r'^observed_at: .+$','observed_at: '+json.dumps(now),t,count=1,flags=re.M);t=re.sub(r'^included_events: .+$','included_events: '+json.dumps(ids),t,count=1,flags=re.M)
old_row='| disposition / UI-R6 | review-applied | UI-217、UI-216 两条已修改待评审 | confirmed | [作者处置事件](../../events/review/4096c476-5ee3-44a5-a195-09a5183542ca/5b8ab8f7-df0c-4bf7-9011-39dcb62ad7f0.md)；下一步独立 UI 复审 |';new_row=f'| disposition / UI-R6 | review-applied | UI-217、UI-216 经R7独立复核已解决 | confirmed | [独立复核事件](../../events/review/{old}/{intent["r6_closure_event"]}.md)；[R7](../review/{rid}.md) |';assert old_row in t;t=t.replace(old_row,new_row).replace('## 当前作者处置','## 作者处置（R7前）');t+=f'\n## R7 独立复核（当前）\n\n[R7报告](../../../../{artifact})确认两条已解决，原R6评审判定与分数保留；[复核事件](../../events/review/{old}/{intent["r6_closure_event"]}.md)接续作者处置。R7通过9.8，另有可选UI-218/P2；当前后续为用户决定是否打磨，否则review-code本循环UI修复。\n\n- [{intent["r6_closure_event"]}](../../events/review/{old}/{intent["r6_closure_event"]}.md) · disposition/UI-R6\n';old_status.write_text(t)
# The plan source is read-only; refresh only its derived status and current route.
f=base/'status/plan'/f'{plan}.md';t=f.read_text();t=re.sub(r'^observed_at: .+$','observed_at: '+json.dumps(now),t,count=1,flags=re.M);t=re.sub(r'当前计划SHA-256 `[0-9a-f]{64}`','当前计划SHA-256 `'+sha(root/'docs/plan/DOM-COMPONENTS-P1.md')+'`',t,count=1)
needle='；下一步独立 UI 复审，整期及人工未验边界不变 |';assert needle in t;t=t.replace(needle,f'；[R7](../review/{rid}.md)已独立确认两条解决；UI修复代码待review-code，整期及人工未验边界不变 |',1).replace('## UI 验收 R6（当前）','## UI 验收 R6（历史；R7已复核）')
t+=f'''\n## UI 验收 R7（当前）

[独立对象](../review/{rid}.md)与[完整报告](../../../../{artifact})：通过验收，最低U5为9.8/10，门槛8，P0/P1为0；UI-216、UI-217经独立复核已解决，新增UI-218/P2为可选打磨。278张原图、14张机器图、10位全新评审；目标指纹6c8b16d50cf7未变。

原计划仍写R6等待UI复审；本页依据新证据接续，计划原文不改。Ready、7/7与实施事件集合不变，不重复造完成或产品终验事件；旧代码R3不能覆盖后续UI修复。用户若继续打磨用apply-ui-review R7报告，否则先review-code原计划。台账与本轮原始证据未提交、未推送。
''';f.write_text(t)
# Maintain only the changed type and leaf indices, preserving unrelated rows.
def insert_row(path,key,row):
 text=path.read_text();assert not any(line.startswith('| '+key+' |') for line in text.splitlines());lines=text.splitlines();pos=[i for i,l in enumerate(lines) if re.match(r'^\| [0-9a-f-]{36} \|',l)];assert pos
 candidates=[i for i in pos if lines[i].split('|')[1].strip()<key];at=(max(candidates)+1) if candidates else min(pos);lines.insert(at,row);path.write_text('\n'.join(lines)+'\n')
insert_row(base/'status/review/index.md',rid,f'| {rid} | [DOM-COMPONENTS-P1 UI 验收评审 R7](../../../../{artifact}) | [对象]({rid}.md) | 活动 |')
insert_row(base/'events/review/index.md',rid,f'| {rid} | DOM-COMPONENTS-P1 UI 验收评审 R7 事件 | [入口]({rid}/index.md) |')
leaf=f'''# DOM-COMPONENTS-P1 UI R7 · 事件索引

本目录保存本轮UI验收与处置事实；报告、目标和截图保留原位，index不是事件。

| 事件 ID | facet / scope | 文件 |
| --- | --- | --- |
'''
for eid,facet in sorted([(intent['review_event'],'review'),(intent['disposition_event'],'disposition')]):leaf+=f'| {eid} | {facet} / UI-R7 | [事件]({eid}.md) |\n'
leaf+='\n遵循[目录索引协议](../../../protocol.md#目录索引维护)。\n';put_new(base/'events/review'/rid/'index.md',leaf)
insert_row(old_dir/'index.md',intent['r6_closure_event'],f'| {intent["r6_closure_event"]} | disposition / UI-R6 | [事件]({intent["r6_closure_event"]}.md) |')
# Refresh the actual focus, retaining the original history on the same line.
f=base/'index.md';text=f.read_text();lines=text.splitlines();found=[i for i,l in enumerate(lines) if l.startswith('- [DOM-COMPONENTS-P1]')];assert len(found)==1;i=found[0];row=lines[i];row=row.replace('当前 UI 修改仍待独立复核','当前 UI 修改的代码复审仍待续接')
needle='UI-217 / UI-216 两条已修改待评审，290 项组件测试、32 个不同浏览器用例及 scoped 检查通过。下一步 `$review-ui docs/plan/DOM-COMPONENTS-P1.md`。';assert needle in row;row=row.replace(needle,f'UI-217 / UI-216 已由 [UI 验收 R7](status/review/{rid}.md)独立确认解决；R7通过9.8/10（U5），P0/P1为0，新增UI-218/P2可选打磨，278张原图、14张机器图、10位全新评审。原290项组件测试、32个浏览器自检保留各自基线。下一步由用户选择继续打磨或review-code本循环UI修复；UI通过不替代代码复审。');lines[i]=row;f.write_text('\n'.join(lines)+'\n')
for path,digest in immutable.items():assert sha(root/path)==digest,('immutable source changed',path)
registration={'review_id':rid,'review_events':[intent['review_event'],intent['disposition_event']],'r6_closure_event':intent['r6_closure_event'],'observed_at':now,'baseline':baseline,'report_sha256':'sha256:'+sha(root/artifact),'review_state':'review-recorded','disposition_state':'review-not-required','r6_disposition':'review-applied','open_optional_gap':'UI-218/P2','plan_state_unchanged':True,'original_plan_and_records_unchanged':True};(p/'sdlc-registration.json').write_text(json.dumps(registration,ensure_ascii=False,indent=2)+'\n');print(json.dumps(registration,ensure_ascii=False,indent=2))
