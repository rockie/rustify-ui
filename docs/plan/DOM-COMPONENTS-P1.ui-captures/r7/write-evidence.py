from pathlib import Path
from collections import Counter
import json, hashlib, struct
p=Path(__file__).resolve().parent
j=json.loads((p/'capture-log.json').read_text());rs=[r for run in j['runs'] for r in run['records']]
v=json.loads((p/'tested-version.json').read_text());products=json.loads((p/'products.json').read_text());t=json.loads((p/'target.json').read_text())
versions=', '.join(sorted(set(run['chromium'] for run in j['runs'])))
intro=f'''- 被测版本：HEAD `{v['head']}`；产品路径 diff 与未跟踪文件见 tested-version.json；初采两项均为空。产品源码 SHA-256 `{v['product_fingerprint_sha256']}`。
- 本轮从当前源码重新构建，HTTP manifest 与本地逐字节一致：Studio `{products['theme-studio']['build_id']}`，catalog `{products['component-catalog']['build_id']}`。
- 目标指纹：`{t['algorithm']}:{t['fingerprint']}`，与 R6 相同，不重建基线。冻结目标仍为 `DOM-COMPONENTS-P1.ui-target.r6.md`；本轮没有修改期望。
- 人工循环：既有 Markdown R1–R6，无 managed 索引；本轮 R7，未 import 或 begin managed 循环。
- 工具：Playwright 1.63.0 / Chromium {versions} headless；en-US、colorScheme light、reducedMotion reduce。默认 DPR 1；焦点特写 DPR 2。
- 条件：逐帧等待字体、networkidle 与连续 500ms 无 >100ms 长帧；每流程新 context，空存储。U4 旧库种子、配额、无锁与同源第二页逐图列明；没有网络拦截。200% 使用目标规定的 720×900 与根字号 200%。
- 真实入口：Studio http://127.0.0.1:4181/；catalog http://127.0.0.1:4176/。strict CSP 的 xtask serve；catalog 按既定入口调用 mount_button_fixtures / mount_layout_fixtures。深色 Link 用真实 Studio DOM 预览中同 SDK 的 Edit theme 链接，未改产品 DOM/样式。
- 截图：viewport 为可视区；element 为原生元素截图；U7 整段为向外取整并留 8px 的原生页面裁切；DPR2 焦点为控件周围 24px 原生裁切。原图未合成、修饰、覆盖。采集轨迹见各单元 actions.log。
'''
head='| 单元 | 状态 | 原图 | 实际URL | CSS视口 | DPR | 主题 | 方式 | 拍摄UTC | 焦点 | 环境干预 |\n| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |\n'
def table(rows):
 return head+'\n'.join(f'| {r["unit"]} | {r["state"]} | [{r["file"]}]({r["file"]}) | {r["url"]} | {r["viewport"]} | {r["dpr"]} | {r["theme"]} | {r["kind"]} | {r["time"]} | {r["focused"] or "—"} | {r["interventions"]} |' for r in rows)+'\n'
for u in [f'U{i}' for i in range(1,8)]:
 rows=[r for r in rs if r['unit']==u]
 (p/f'capture-{u}.md').write_text(f'# UI 验收 R7 · {u} 采集清单\n\n'+intro+f'\n共 {len(rows)} 张。操作记录：[{u}-actions.log]({u}-actions.log)。\n\n'+table(rows))
 (p/f'{u}-actions.log').write_text('\n'.join(a for run in j['runs'] for a in run['actions'].get(u,[]))+'\n')
failures=[f for run in j['runs'] for f in run['failures']];counts=Counter(r['unit'] for r in rs)
(p/'capture.md').write_text('# DOM-COMPONENTS-P1 UI 验收 R7 · 采集清单\n\n'+intro+'\n## 覆盖与出处\n\n'+f'- 共 {len(rs)} 张：'+', '.join(f'{u} {n}' for u,n in sorted(counts.items()))+'。\n- 采集尝试失败：'+json.dumps(failures,ensure_ascii=False)+'。失败与补采对应关系另行登记，不用相近状态替代。\n- 服务及停止方式见 services.json，构建见 products.json；逐图完整记录如下。\n\n'+table(rs))
images=[]
for r in rs:
 f=p/r['file'];b=f.read_bytes();assert b[:8]==b'\x89PNG\r\n\x1a\n';w,h=struct.unpack('>II',b[16:24]);images.append({'file':r['file'],'width':w,'height':h,'bytes':len(b),'sha256':hashlib.sha256(b).hexdigest()})
assert len({r['file'] for r in rs})==len(rs)
(p/'image-integrity.json').write_text(json.dumps(images,indent=2)+'\n')
print(json.dumps({'images':len(rs),'counts':dict(counts),'failures':failures},ensure_ascii=False))
