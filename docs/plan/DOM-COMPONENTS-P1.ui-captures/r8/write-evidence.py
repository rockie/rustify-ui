from pathlib import Path
from collections import Counter
import json, hashlib, struct
p=Path(__file__).resolve().parent
j=json.loads((p/'capture-log.json').read_text());rs=[r for run in j['runs'] for r in run['records']]
v=json.loads((p/'tested-version.json').read_text());products=json.loads((p/'products.json').read_text());t=json.loads((p/'target.json').read_text())
versions=', '.join(sorted(set(run['chromium'] for run in j['runs'])))
intro=f'''- 被测版本：HEAD `{v['head']}`；产品路径 diff 与未跟踪文件见 tested-version.json；初采两项均为空。产品源码 SHA-256 `{v['product_fingerprint_sha256']}`。
- 本轮从当前源码重新构建，HTTP manifest 与本地逐字节一致：Studio `{products['theme-studio']['build_id']}`，catalog `{products['component-catalog']['build_id']}`。
- 目标指纹：`{t['algorithm']}:{t['fingerprint']}`，与 R7 相同，不重建基线。冻结目标仍为 `DOM-COMPONENTS-P1.ui-target.r7.md`；本轮没有修改期望。
- 人工循环：既有 Markdown R1–R7，无 managed 索引；本轮 R8，未 import 或 begin managed 循环。
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
 recovered=u in ('U3','U4','U7') and any(r['file'].endswith('-a2.png') for r in rows)
 recovery_note=('\n- 补采环境：初次对应流程因构建输出目录缺失而收到 HTTP 404，未拍到的格子登记为 attempt 1 缺证；本轮从同一源码重建，两个 build_id 与初采一致，逐文件校验后复制到独立临时目录，仍通过仓库 `mbx xtask serve --dir ... --csp strict` 在相同 URL 服务。文件名 `-a2` 为这些格子的第二次采集；视口、DPR、语言、主题、字体等待及操作步骤保持原条件。原图和失败轨迹保留。\n' if recovered else '')
 suffix='-b' if u=='U7' and any(r['state'].startswith('zoomdetail-') for r in rows) else ''
 action_file=f'{u}-actions{suffix}.log'
 zoom_note=('\n- 缩放分段补采：保持相同 95%／110% 状态、1280／390 视口及实际夹具；原生 page screenshot clip 高度最多 900 CSS px，分段之间保留重叠。原始长图保留；没有拼接、修图或改变 UI 代码。\n' if suffix else '')
 (p/f'capture-{u}{suffix}.md').write_text(f'# UI 验收 R8 · {u} 采集清单\n\n'+intro+recovery_note+zoom_note+f'\n共 {len(rows)} 张。操作记录：[{action_file}]({action_file})。\n\n'+table(rows))
 (p/action_file).write_text('\n'.join(a for run in j['runs'] for a in run['actions'].get(u,[]))+'\n')
failures=[f for run in j['runs'] for f in run['failures']];counts=Counter(r['unit'] for r in rs)
(p/'capture.md').write_text('# DOM-COMPONENTS-P1 UI 验收 R8 · 采集清单\n\n'+intro+'\n## 覆盖与出处\n\n'+f'- 共 {len(rs)} 张：'+', '.join(f'{u} {n}' for u,n in sorted(counts.items()))+'。\n- 采集尝试失败：'+json.dumps(failures,ensure_ascii=False)+'。失败与补采对应关系另行登记，不用相近状态替代。\n- 服务及停止方式见 services.json，构建见 products.json；逐图完整记录如下。\n\n'+table(rs))
images=[]
for r in rs:
 f=p/r['file'];b=f.read_bytes();assert b[:8]==b'\x89PNG\r\n\x1a\n';w,h=struct.unpack('>II',b[16:24]);images.append({'file':r['file'],'width':w,'height':h,'bytes':len(b),'sha256':hashlib.sha256(b).hexdigest()})
assert len({r['file'] for r in rs})==len(rs)
(p/'image-integrity.json').write_text(json.dumps(images,indent=2)+'\n')
resolution=p/'capture-resolutions.json'
if resolution.exists():
 d=json.loads(resolution.read_text())
 note='\n## 补采与环境恢复\n\n- 初次流程有 5 次就绪超时，影响 13 帧；当时本地构建输出目录缺失，两个运行中的静态服务均返回 HTTP 404。删除原因未查明，未归因为产品 UI 缺陷。\n- 同一源码重新构建后，两项 build_id 与初采一致。逐文件验证的独立副本使用仓库现有 `xtask serve --dir`、相同 URL 和 strict CSP；见 [恢复产物身份](recovery-products.json) 与 [服务记录](services.json)。\n- 所有 13 帧以 `-a2` 补齐，原图和失败日志没有覆盖；当前未采到的必需格子为 0。详见 [逐帧对应与补采结果](capture-resolutions.json)。\n\n| 初次缺证帧 | 本轮第二次采集 | 结果 |\n| --- | --- | --- |\n'
 note+='\n'.join(f'| {r["expected_original_name"]} | [{r["attempt_2_file"]}]({r["attempt_2_file"]}) | 已采集 |' for r in d['frames'])+'\n'
 with (p/'capture.md').open('a') as f:f.write(note)
print(json.dumps({'images':len(rs),'counts':dict(counts),'failed_attempts':len(failures),'recovered_frames':d['recovered_frames'] if resolution.exists() else 0},ensure_ascii=False))
