from pathlib import Path
import json,sys,re,hashlib
root=Path.cwd();p=root/'docs/plan/DOM-COMPONENTS-P1.ui-captures/r6';data=json.loads((p/'capture-log.json').read_text());rs=[r for run in data['runs'] for r in run['records']]
for u in sys.argv[1:]:
 rows=[r for r in rs if r['unit']==u];cap=p/f'capture-{u}-c.md';actions=p/f'{u}-actions-c.log'
 intro=(p/f'capture-{u}.md').read_text().split('\n- 共 ',1)[0];intro=intro.replace('采集清单','补采后完整采集清单',1)
 lines=[intro,'',f'- 本单元当前共 {len(rows)} 张原图，包含首次与补充实拍；本次补齐原脚本遗漏的视口/主题组合。所有原始帧保留，未改界面代码或把旧图当作新图。','- U6 深色 Link 由实际 Studio DOM 预览进入，拍 1280×900；catalog 布局夹具没有外观操作。仅改采集入口，不改组件或目标期望。' if u=='U6' else '',f'- 实际操作：[记录]({actions.name})。','','| 状态 | 原图 | URL | CSS视口 | DPR | 主题 | UTC | 截图方式 | 干预 |','| --- | --- | --- | --- | --- | --- | --- | --- | --- |']
 for r in rows:lines.append(f'| {r["state"]} | [{r["file"]}]({r["file"]}) | {r["url"]} | {r["viewport"]} | {r["dpr"]} | {r["theme"]} | {r["time"]} | {r["kind"]} | {r["interventions"]} |')
 assert not cap.exists();cap.write_text('\n'.join(lines)+'\n');actions.write_text('\n'.join(a for run in data['runs'] for a in run['actions'].get(u,[]))+'\n')
 task=(p/'judge-inputs'/f'task-{u}.md').read_text();start=task.index('- 本轮原图：');end=task.index('- 上一轮本单元独立评审原文：',start);task=task[:start]+f'- 本轮原图：仅下列 {len(rows)} 个绝对路径：\n'+'\n'.join(str((p/r['file']).resolve()) for r in rows)+'\n'+task[end:];task=task.replace(str(p/f'capture-{u}.md'),str(cap)).replace(str(p/f'{u}-actions.log'),str(actions));task='这是本轮补采后的全新独立复核；不知道首位评审的本轮分数，也不读取本轮 judge 或报告。只依据完整当前材料给本单元整组四维分。不要修改或新建文件。\n\n'+task
 (p/'judge-inputs'/f'task-{u}-c.md').write_text(task)
 print(json.dumps({'unit':u,'shots':len(rows),'task':str(p/'judge-inputs'/f'task-{u}-c.md')},ensure_ascii=False))
