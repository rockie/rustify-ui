from pathlib import Path
import json,uuid,hashlib,datetime,re,subprocess
root=Path.cwd();base=root/'.xgent-ai/sdlc';p=root/'docs/plan/DOM-COMPONENTS-P1.ui-captures/r6';report=root/'docs/plan/DOM-COMPONENTS-P1.ui-review.r6.md';old='2cc80246-5d61-4370-bbd7-462827ecbce3';plan='37a2c7b4-7052-4f85-96e9-135b6f7667a1';review=str(uuid.uuid4());ev_review=str(uuid.uuid4());ev_disp=str(uuid.uuid4());ev_old=str(uuid.uuid4());now=datetime.datetime.now(datetime.timezone(datetime.timedelta(hours=8))).isoformat(timespec='seconds');head=subprocess.check_output(['git','rev-parse','HEAD']).decode().strip();branch=subprocess.check_output(['git','branch','--show-current']).decode().strip();baseline={'head':head,'branch':branch,'worktree':'dirty'}
assert not (base/'status/review'/f'{review}.md').exists()
with report.open('a') as f:f.write(f'\n## 状态登记\n\n本轮独立评审 ID：`{review}`，见 [R6 评审状态](../../.xgent-ai/sdlc/status/review/{review}.md)。按仓内 v3 协议登记为 `review_type: other`、`review-recorded` / `review-not-required`；UI-216 仍为未关闭的可选 P2，没有伪记已解决。R5 的六条修复已由本轮复核关闭，UI-115 按用户接受保留；只续接其处置状态，不改 R5 原判定或评分。Ready、7/7、合并与发布状态均未改变。原报告、截图及台账未提交、未推送。\n')
def src(rel,anchor):return {'path':rel,'revision':'sha256:'+hashlib.sha256((root/rel).read_bytes()).hexdigest(),'anchor':anchor}
sources=[src(str(report.relative_to(root)),'验收判定、评分、上轮差距复核、差距清单、机器检查'),src('docs/plan/DOM-COMPONENTS-P1.ui-target.r6.md','单元、规范要点；manual-sha256-normative-sections-v1:6c8b16d50cf78224c314e9c6a8a6c3416ea54069319ba4425751bf4d50d159c4'),src('docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/judge.md','七单元独立评分、UI-216'),src('docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/machine-focus-confirm.json','UI-215 四触发器真实键盘焦点')]
relations=[{'type':'reviews','target':plan,'scope':'P1 Studio 与 DOM 组件；U1–U7 全量'},{'type':'rechecks','target':old,'scope':'R5 六条作者修复与 UI-115 已确认目标变更'}]
def event(event_id,object_id,facet,scope,state,source_state,predecessors,ss,body,title,artifact,rels):
 data={'schema_version':3,'event_id':event_id,'object_id':object_id,'kind':'review','review_type':'other','title':title,'artifact':artifact,'facet':facet,'scope':scope,'state':state,'source_state':source_state,'evidence_status':'confirmed','observed_at':now,'predecessors':predecessors,'baseline':baseline,'sources':ss,'relations':rels}
 fm='---\n'+'\n'.join(k+': '+json.dumps(v,ensure_ascii=False) for k,v in data.items())+'\n---\n\n'+body+'\n';dest=base/'events/review'/object_id/f'{event_id}.md';dest.parent.mkdir(exist_ok=True);assert not dest.exists();dest.write_text(fm)
artifact=str(report.relative_to(root));title='DOM-COMPONENTS-P1 UI 验收评审 R6'
event(ev_review,review,'review','UI-R6','review-recorded','通过验收；9.7/10（U7），门槛8',[],sources,f'# UI 验收 R6\n\n- 事实与证据：见[完整报告](../../../../../{artifact})。被测 {head}，目标指纹6c8b16d50cf7，U1–U7全量；154张当前原图、12张后置机器原图、7份独立结论。\n- 校准依据：最低单元9.7，未关闭P0/P1为0；UI-215真实键盘实测关闭；新增UI-216/P2为可选打磨。六条修复关闭，UI-115按已确认目标不再适用，不记为已修复。\n- 限制：仅冻结矩阵与本机headless Chromium；其他未验范围留报告。没有产品代码或计划改动，未提交/推送；不登记合并、发布或产品终验。\n- 下一步：用户选择是否打磨UI-216；若结束UI打磨，先对本循环UI修复做review-code。',title,artifact,relations)
event(ev_disp,review,'disposition','UI-R6','review-not-required','通过验收；UI-216 为可选打磨',[],sources[:1],f'# 本轮无需强制 apply\n\n- 报告首行判定通过验收，按仓内状态模型登记review-not-required。UI-216仍未关闭，仅为可选P2，未伪造已修复或运行apply。\n- 原始分数、差距与后续选择见[报告](../../../../../{artifact})。本轮没有执行修复或代码复审。',title,artifact,relations)
event(ev_old,old,'disposition','UI-R5','review-applied','六条修复经R6独立复核已解决；UI-115接受偏差、不再适用',['167f2e54-ab63-41d3-864e-3cb085cdc77e'],sources[:2],f'# R5 修复经 R6 独立复核\n\n- [R6报告](../../../../../{artifact}#上轮差距复核)逐条复核UI-215、UI-214、UI-213、UI-203、UI-76、UI-210，全部已解决；UI-115沿用用户接受范围变更，未冒充修复。\n- 本scope各条均有最终处置与独立复核，登记review-applied；R5原判定需要修改、原分数9.6保持。\n- 新增UI-216属于R6，不合并进R5的原差距。Ready与7/7不变，未合并或发布。','DOM-COMPONENTS-P1 UI R5 · R6独立复核处置','docs/plan/DOM-COMPONENTS-P1.ui-review.r5.md',relations[:1])
leaf=base/'events/review'/review
(leaf/'index.md').write_text(f'# DOM-COMPONENTS-P1 UI R6 · 事件索引\n\n仅列本轮评审与处置事件，报告与证据保留原位；index不是事件。\n\n| 事件 ID | facet / scope | 文件 |\n| --- | --- | --- |\n'+''.join(f'| {eid} | {facet} / UI-R6 | [事件]({eid}.md) |\n' for eid,facet in sorted([(ev_review,'review'),(ev_disp,'disposition')]))+'\n遵循[目录索引协议](../../../protocol.md#目录索引维护)。\n')
state=f'''---
object_id: "{review}"
kind: "review"
review_type: "other"
artifact: "{artifact}"
archived: false
observed_at: "{now}"
baseline: {json.dumps(baseline,ensure_ascii=False)}
included_events: {json.dumps([ev_review,ev_disp])}
relations: {json.dumps(relations,ensure_ascii=False)}
---

# {title}

来源：[完整报告](../../../../{artifact})（sha256:`{sources[0]['revision'][7:]}`）、[截图与独立原文](../../../../docs/plan/DOM-COMPONENTS-P1.ui-captures/r6/)、[前轮R5]({old}.md)、[计划](../plan/{plan}.md)。

| 维度 / 范围 | 固定状态码 | 来源自述 | 证据质量 | 下一步 |
| --- | --- | --- | --- | --- |
| review / UI-R6 | review-recorded | 通过验收；9.7/10（U7），门槛8 | confirmed | [评审事件](../../events/review/{review}/{ev_review}.md) |
| disposition / UI-R6 | review-not-required | 无强制apply；UI-216/P2可选 | confirmed | 用户决定是否继续打磨；否则先review-code |

- UI验收评审登记为other。被测HEAD `{head}`，产品路径无diff；Studio dff085ce38bbff64、catalog 32b52b66f04701ad。
- 目标指纹 `manual-sha256-normative-sections-v1:6c8b16d50cf78224c314e9c6a8a6c3416ea54069319ba4425751bf4d50d159c4`；UI-115用户接受后重建基线。U1–U6均10，U7为9.7；154张矩阵原图、12张机器原图、7位独立评审。
- R5六条修复全部已解决，UI-115接受偏差、不再适用；新发现UI-216/P2仍未关闭，不阻止本轮通过。P0/P1为0。
- 当前源码与目标结束前指纹未变；临时服务已停止。完整范围、扫描对比度接受口径、矩阵外未知与人工未验见报告。不改变计划Ready/7/7、不登记合并或发布。
- 报告、截图与台账未提交/未推送，只声明当前工作树可见。历史代码R3不自动覆盖后续UI修复，后续由用户发起review-code。

## 已纳入事件

- [{ev_review}](../../events/review/{review}/{ev_review}.md) · review/UI-R6
- [{ev_disp}](../../events/review/{review}/{ev_disp}.md) · disposition/UI-R6
'''
(base/'status/review'/f'{review}.md').write_text(state)
# Read each mutable projection immediately before the surgical update.
f=base/'status/review'/f'{old}.md';text=f.read_text();text=re.sub(r'^observed_at: .*$',f'observed_at: "{now}"',text,count=1,flags=re.M);m=re.search(r'^included_events: (\[.*\])$',text,re.M);ids=json.loads(m.group(1));ids.append(ev_old);text=text[:m.start(1)]+json.dumps(ids)+text[m.end(1):]
oldrow='| disposition / UI-R5 | review-partially-applied | 6 条已修改待评审；UI-115接受偏差 | confirmed | [作者处置事件](../../events/review/'+old+'/167f2e54-ab63-41d3-864e-3cb085cdc77e.md)；下一步独立 UI 复审 |';assert oldrow in text
newrow=f'| disposition / UI-R5 | review-applied | 6 条经 R6 独立复核已解决；UI-115接受偏差 | confirmed | [R6复核事件](../../events/review/{old}/{ev_old}.md)；R5原评分与判定保留 |';text=text.replace(oldrow,newrow);text+=f'\n## R6 独立复核\n\n[R6评审]({review}.md)已完成：六条修复全部已解决，UI-115按用户接受不再适用；R5处置review-applied。R5原判定及9.6分不改。R6新UI-216/P2为可选打磨，不写入旧报告。\n\n- [{ev_old}](../../events/review/{old}/{ev_old}.md) · disposition/UI-R5\n';f.write_text(text)
f=base/'events/review'/old/'index.md';text=f.read_text();row=f'| {ev_old} | disposition / UI-R5 · R6独立复核 | [事件]({ev_old}.md) |\n';text=text.replace('\n遵循[目录索引协议]', '\n'+row+'\n遵循[目录索引协议]');f.write_text(text)
def index_add(file,row,key):
 text=file.read_text();assert key not in text
 lines=text.splitlines(keepends=True);pos=next((i for i,l in enumerate(lines) if l.startswith('| ') and re.match(r'\| [0-9a-f]{8}-',l) and l.split('|')[1].strip()>key),None)
 if pos is None:pos=next(i for i,l in enumerate(lines) if l.startswith('维护约定'))
 lines.insert(pos,row+'\n');file.write_text(''.join(lines))
index_add(base/'status/review/index.md',f'| {review} | [DOM-COMPONENTS-P1 UI 验收评审 R6](../../../../{artifact}) | [对象]({review}.md) | 活动 |',review)
index_add(base/'events/review/index.md',f'| {review} | DOM-COMPONENTS-P1 UI 验收评审 R6 事件 | [入口]({review}/index.md) |',review)
f=base/'status/plan'/f'{plan}.md';text=f.read_text();text=re.sub(r'^observed_at: .*$',f'observed_at: "{now}"',text,count=1,flags=re.M)
oldnext='6 条已修改待评审，UI-115保留用户接受偏差；下一步独立 UI 复审，整期未重跑及人工边界保留';assert oldnext in text;text=text.replace(oldnext,f'六条已由[R6](../review/{review}.md)独立复核关闭，UI-115保留用户接受偏差；R6通过9.7，UI-216/P2可选；后续代码复审，整期未重跑及人工边界保留')
text+=f'\n## UI 验收 R6（当前）\n\n[独立评审对象](../review/{review}.md)与[完整报告](../../../../{artifact})：通过验收，最低U7为9.7、门槛8，其余六单元10；P0/P1为0，UI-216/P2可选打磨。R5六条修复已关闭，UI-115按用户接受不再适用。原计划恢复快照仍指向发起UI复审，本轮按review-ui只读边界未改原文；本页记录已经实际完成的独立复审，不改变Ready与7/7。\n\n若继续打磨，由用户运行 `$apply-ui-review {artifact}`；若结束UI打磨，先 `$review-code docs/plan/DOM-COMPONENTS-P1.md` 复审本循环UI修改。原资料与台账未提交、未推送；不另造计划完成/产品终验/发布事件。\n';f.write_text(text)
f=base/'index.md';text=f.read_text();start=text.index('- [DOM-COMPONENTS-P1]');end=text.index('\n\n- [DOM-COMPONENTS PRD]',start);block=text[start:end];oldnext='下一步 `$review-ui docs/plan/DOM-COMPONENTS-P1.md`。';assert block.count(oldnext)==1
new=f'[UI 验收 R6](status/review/{review}.md)（重建基线）已通过，最低U7为9.7、门槛8，其余六单元10；R5六条修复已解决，UI-115按用户接受不再适用，P0/P1为0，新增UI-216/P2可选打磨。154张矩阵与12张机器原图、七份独立结论见报告。若结束UI打磨，下一步 `$review-code docs/plan/DOM-COMPONENTS-P1.md`；若继续，用户可发起apply-ui-review。Ready与7/7保持，未登记合并、发布或产品终验。'
block=block.replace(oldnext,new);text=text[:start]+block+text[end:];f.write_text(text)
registration={'review_id':review,'review_events':[ev_review,ev_disp],'r5_disposition_event':ev_old,'observed_at':now,'report_sha256':sources[0]['revision'],'new_state':'review-recorded / review-not-required','r5_disposition':'review-applied'};(p/'sdlc-registration.json').write_text(json.dumps(registration,ensure_ascii=False,indent=2));print(json.dumps(registration,ensure_ascii=False,indent=2))
