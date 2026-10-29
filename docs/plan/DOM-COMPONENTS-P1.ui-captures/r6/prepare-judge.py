from pathlib import Path
import re,json,hashlib
root=Path.cwd();p=root/'docs/plan/DOM-COMPONENTS-P1.ui-captures/r6';src=(root/'.agents/skills/review-ui/references/judge.md').read_text()
template=src.split('```text\n',1)[1].split('\n```',1)[0]
target=root/'docs/plan/DOM-COMPONENTS-P1.ui-target.r6.md'
for i in range(1,8):
 u=f'U{i}';capture=p/f'capture-{u}.md';prior=p/'judge-inputs'/f'r5-judge-{u}.md';priorimages=p/'judge-inputs'/f'r5-images-{u}.txt';currentimages=sorted(p.glob(f'{u}-*.png'))
 prompt=template.replace('{路径}',str(target)).replace('{目录}',str(p)).replace('{capture.md 路径}',str(capture)).replace('{各单元的目录与 judge.md 路径}',f'{prior}；上一轮截图只读 {priorimages} 列出的文件').replace('差分图（可选）：'+str(p),'差分图（可选）：无')
 prefix=f'''本任务只评 {u}，不要给其他单元打分。本轮是 R6；目标包已记录用户对 UI-115 的范围接受，完整变更记录须读取。你继承宿主模型与 effort，不需要读取任何其他技能或仓库规则，不派生，不改文件。将完整结论直接返回给主 agent。

严格白名单：
- 任务说明：本文件。
- 冻结目标包：{target}，只沿其中目标 PNG 链接读取附件，不沿来源链接打开其他文件。
- 本轮本单元清单：{capture}。
- 本轮本单元实际操作：{p/f'{u}-actions.log'}。
- 本轮原图：仅下列 {len(currentimages)} 个绝对路径：
'''+ '\n'.join(str(x) for x in currentimages)+f'''
- 上一轮本单元独立评审原文：{prior}。
- 上一轮原图白名单：{priorimages} 中的 PNG。优先对照未关闭差距所需的旧图；其余按需比对。

只打开上述材料，不读同目录其他 JSON、日志、代码、完整报告、机器检查或修复记录。先读清单与目标，再实际调用 view_image 打开当前原图与所需目标/旧图；纯文件尺寸与像素统计不能代替开图。勿凭未操作、未拍的状态补造通过或失败。

'''
 (p/'judge-inputs'/f'task-{u}.md').write_text(prefix+prompt+'\n\n请在末尾附上实际使用的开图工具和实际打开的图片绝对路径清单；不要把仅列目录或读取尺寸算作开图。\n')
 files=[target,capture,p/f'{u}-actions.log',prior,priorimages,*currentimages]
 packet={'unit':u,'mode':'manual-whitelist','files':{str(f):hashlib.sha256(f.read_bytes()).hexdigest() for f in files}}
 (p/'judge-inputs'/f'packet-{u}.json').write_text(json.dumps(packet,ensure_ascii=False,indent=2))
print('Prepared 7 independent-unit prompts from the verbatim judge template.')
