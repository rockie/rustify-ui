from pathlib import Path
import json,hashlib
p=Path(__file__).resolve().parent;root=p.parents[3];q=p/'judge-inputs';q.mkdir(exist_ok=True)
s=(root/'.agents/skills/review-ui/references/judge.md').read_text();template=s.split('```text\n',1)[1].split('\n```',1)[0]
target=root/'docs/plan/DOM-COMPONENTS-P1.ui-target.r7.md';runs=json.loads((p/'capture-log.json').read_text())['runs'];records=[r for run in runs for r in run['records']]
for i in range(1,8):
 u=f'U{i}';capture=p/f'capture-{u}.md';prior=q/f'previous-judge-{u}.md';priorlist=q/f'r7-images-{u}.txt';images=[p/r['file'] for r in records if r['unit']==u]
 prompt=template.replace('{路径}',str(target)).replace('{目录}',str(p)).replace('{capture.md 路径}',str(capture)).replace('{各单元的目录与 judge.md 路径}',f'{prior}；上一轮原图只读 {priorlist} 列出的 PNG')
 prompt=prompt.replace('差分图（可选）：'+str(p),'差分图（可选）：无')
 prefix=f'''本任务只评 {u}，不要给其他单元打分。本轮 R8。你是全新上下文独立评审，继承宿主模型与 effort；只读、不改文件、不派生。不要读取其他技能、仓库规则或源码。完整结论直接返回主 agent。

严格白名单（不沿来源链接打开其他材料）：
- 冻结目标：{target}。其中只有目标 PNG 附件可以沿链接打开；完整变更记录须读。
- 本轮本单元采集：{capture}。
- 本轮本单元实际操作：{p/f'{u}-actions.log'}。
- 本轮本单元原图（逐张实际开图；无法看清须说出，不凭尺寸统计判断）：
'''+ '\n'.join(str(x) for x in images)+f'''
- 上一轮独立结论：{prior}。
- 上一轮原图白名单：{priorlist}；先对照未关闭差距所需旧图，其余按需。

同目录机器结果、其他评审、本轮报告、修复记录、代码、diff 与提交说明均不在白名单。没有门槛分，按下述标尺独立给分。每个维度理由、旧差距状态、失分对应的可执行差距须完整。

'''
 (q/f'task-{u}.md').write_text(prefix+prompt+'\n\n末尾附实际开图工具与实际打开图片的绝对路径清单。只列文件或读取尺寸不算开图。\n')
 files=[target,capture,p/f'{u}-actions.log',prior,priorlist,*images]
 (q/f'packet-{u}.json').write_text(json.dumps({'unit':u,'mode':'manual-whitelist','files':{str(x):hashlib.sha256(x.read_bytes()).hexdigest() for x in files}},ensure_ascii=False,indent=2)+'\n')
print('Prepared seven unit-only packets.')
