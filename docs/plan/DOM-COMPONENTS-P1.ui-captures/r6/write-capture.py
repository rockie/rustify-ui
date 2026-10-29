from pathlib import Path
from collections import Counter
import json,hashlib,struct
p=Path(__file__).resolve().parent
j=json.loads((p/'capture-log.json').read_text());records=[r for run in j['runs'] for r in run['records']]
v=json.loads((p/'tested-version.json').read_text());products=json.loads((p/'products.json').read_text());fp=(p/'target-files.txt').read_text().split('fingerprint ')[-1].strip()
chromium=', '.join(sorted(set(r['chromium'] for r in j['runs'])))
intro=f'''- 被测版本：HEAD `{v['head']}`，产品路径无未提交 diff / 未跟踪文件；源码指纹 `{v['product_fingerprint_sha256']}`。
- 当前源码重新构建并经 HTTP 核对：Studio `{products['theme-studio']['build_id']}`，catalog `{products['component-catalog']['build_id']}`；Rust 1.99.0、mbx 1.15.0、Tailwind 4.1.13。
- 目标指纹：`manual-sha256-normative-sections-v1:{fp}`。沿用人工循环既有算法。相对 R5 `567117725f56` 已改变：用户接受 UI-115 的目标变更，本轮重建基线。冻结快照是 `DOM-COMPONENTS-P1.ui-target.r6.md`。
- 工具：Playwright 1.63.0 / Chromium {chromium} headless；locale en-US、colorScheme light、reducedMotion reduce；DPR 默认 1，焦点特写为 2。
- 条件：每帧等待字体、networkidle、连续 500ms 无 >100ms 长帧。每流程新 context；默认空存储。U4 种子、故障和同源第二标签干预逐图记录；无网络请求拦截。
- 地址：Studio http://127.0.0.1:4181/；catalog http://127.0.0.1:4176/。二者由本轮 release 副本通过 xtask serve 的 strict CSP 提供。catalog 由既定 mount_button_fixtures / mount_layout_fixtures 入口进入。
- 原图：viewport 为可视区域；element 为真实元素截图；U7 整段为根边界向外取整加 8px 的原生页面裁切；DPR2 特写为控件周围 24px 原生裁切。未合成、修饰或覆盖原图。200% 按目标规定的 720×900 + 根字号 200% 方法。
'''
head='| 单元 | 状态 | 原始截图 | URL | CSS视口 | DPR | 主题 | 截图方式 | 拍摄UTC | 焦点 | 数据/环境干预 |\n| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |\n'
def table(rs):
 return head+'\n'.join(f'| {r["unit"]} | {r["state"]} | [{r["file"]}]({r["file"]}) | {r["url"]} | {r["viewport"]} | {r["dpr"]} | {r["theme"]} | {r["kind"]} | {r["time"]} | {r["focused"] or "—"} | {r["interventions"]} |' for r in rs)+'\n'
for u in [f'U{i}' for i in range(1,8)]:
 rs=[r for r in records if r['unit']==u]
 (p/f'capture-{u}.md').write_text(f'# UI 验收 R6 · {u} 采集清单\n\n'+intro+f'\n- 共 {len(rs)} 张。实际操作见 [{u}-actions.log]({u}-actions.log)。\n\n'+table(rs))
 (p/f'{u}-actions.log').write_text('\n'.join(a for run in j['runs'] for a in run['actions'].get(u,[]))+'\n')
failures=[f for run in j['runs'] for f in run['failures']];counts=Counter(r['unit'] for r in records)
(p/'capture.md').write_text('# DOM-COMPONENTS-P1 UI 验收 R6 · 采集清单\n\n'+intro+'\n## 覆盖与出处\n\n- 旧 Markdown 循环，无 managed 索引。继续人工 R6；未 import 或 begin 新循环。\n- '+', '.join(f'{u}: {n}' for u,n in sorted(counts.items()))+f'；共 {len(records)} 张。\n- 历史采集尝试失败：'+json.dumps(failures,ensure_ascii=False)+'。旧定位名 import-kind 不存在，已改用真实 import-format 入口在 FOCUS6B 补齐 4 张；原图未覆盖，当前无必需格子缺失，见 capture-resolutions.json。\n- 每帧出处见下表与 capture-log.json；U1-source 的稳定后 a2 帧与条件性 end 帧用于核对，首图保留。服务与停止方式见 services.json，产品路径见 products.json。\n\n'+table(records))
integrity=[]
for r in records:
 f=p/r['file'];b=f.read_bytes();assert b[:8]==b'\x89PNG\r\n\x1a\n';w,h=struct.unpack('>II',b[16:24]);integrity.append({'file':r['file'],'width':w,'height':h,'bytes':len(b),'sha256':hashlib.sha256(b).hexdigest()})
(p/'image-integrity.json').write_text(json.dumps(integrity,indent=2))
print(json.dumps({'shots':len(records),'counts':counts,'failures':failures},ensure_ascii=False))
