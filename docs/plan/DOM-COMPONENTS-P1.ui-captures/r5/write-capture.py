from pathlib import Path
import json, hashlib, re
from collections import Counter
p=Path(__file__).resolve().parent
root=p.parents[3]
data=json.loads((p/'capture-log.json').read_text())
records=[r for run in data['runs'] for r in run['records']]
version=json.loads((p/'tested-version.json').read_text())
products=json.loads((p/'products.json').read_text())
fp=(p/'target-files.txt').read_text().split('fingerprint ')[-1].strip()
intro=(f'- 被测版本：HEAD `{version["head"]}`，产品路径相对 HEAD 无 diff、无未跟踪文件。\n'
 f'- 构建：本轮从当前源码重新构建，Studio `{products["theme-studio"]["build_id"]}`，catalog `{products["component-catalog"]["build_id"]}`；Rust 1.99.0、mbx 1.15.0、Tailwind 4.1.13。\n'
 f'- 目标指纹：`manual-sha256-normative-sections-v1:{fp}`，沿用已有人工循环算法，与 R4 相同。本轮不是重建基线。\n'
 '- Playwright 1.63.0 / Chromium 153.0.8010.12 headless；locale en-US、colorScheme light、reducedMotion reduce；默认 DPR 1，焦点特写为 DPR 2。\n'
 '- 每帧等待 document.fonts.ready、networkidle 及连续 500ms 无超过 100ms 的帧。每流程新 browser context，空 localStorage（U4 种子与环境干预见逐图表）。未拦截网络请求，未改变产品 UI 代码。\n'
 '- 页面截图为可视区域；U6 分段为元素截图；U7 整段为夹具根向外取整并四周留 8px 的页面裁切；焦点特写为控件四周 24px 的页面裁切。原始 PNG 未做合成、修饰或覆盖。\n'
 '- 地址：Studio http://127.0.0.1:4181/；catalog http://127.0.0.1:4176/；均由本轮复制的 release 产物通过 xtask serve 提供 strict CSP。catalog 单元经页面既定 mount_button_fixtures / mount_layout_fixtures 入口进入。\n')
header='| 单元 | 状态 | 原始截图 | 实际 URL | CSS 视口 | DPR | 主题 | 截图方式 | 拍摄时间 UTC | 焦点 | 数据/环境干预 |\n| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |\n'
def table(rs):
 return header+'\n'.join(f'| {r["unit"]} | {r["state"]} | [{r["file"]}]({r["file"]}) | {r["url"]} | {r["viewport"]} | {r["dpr"]} | {r["theme"]} | {r["kind"]} | {r["time"]} | {r["focused"] or "—"} | {r["interventions"]} |' for r in rs)+'\n'
for u in sorted(set(r['unit'] for r in records)):
 rs=[r for r in records if r['unit']==u]
 (p/f'capture-{u}.md').write_text(f'# UI 验收 R5 · {u} 采集清单\n\n'+intro+f'\n- 本单元共 {len(rs)} 张本轮截图。操作记录：[{u}-actions.log]({u}-actions.log)。\n\n'+table(rs))
 acts=[a for run in data['runs'] for a in run['actions'].get(u,[])]
 (p/f'{u}-actions.log').write_text('\n'.join(acts)+'\n')
 old=p.parent/'r4'
 (p/'judge-inputs'/f'r4-images-{u}.txt').write_text('\n'.join(str(f.resolve()) for f in sorted(old.glob(f'{u}-*.png')))+'\n')
failed=[f for run in data['runs'] for f in run['failures']]
counts=Counter(r['unit'] for r in records)
(p/'capture.md').write_text('# DOM-COMPONENTS-P1 UI 验收 R5 · 采集清单\n\n'+intro+'\n## 覆盖与出处\n\n'
+'- 模式：已有纯 Markdown 历史，无 managed 索引；继续人工 R5，未 import、未重置历史。\n'
+'- '+', '.join(f'{u}: {n}' for u,n in sorted(counts.items()))+f'；合计 {len(records)} 张、{len(data["runs"])} 次采集运行。\n'
+f'- 脚本报告失败：{json.dumps(failed,ensure_ascii=False)}。必需矩阵在全量采集结束后核对。\n'
+'- 原始操作、时间与干预逐帧见下表及 U<n>-actions.log；完整采集轨迹为 capture-log.json，脚本为 capture-runner.cjs。\n'
+'- 为避免本地 target 回收影响服务，本轮将构建完成的两份产品复制到 products.json 记录的临时目录；服务 PID 与停止方式见 services.txt。\n'
+'- 初次构建因本地 makepad 原生缓存中存在 Rust 1.99.1 的 rlib 而 E0514；已保留原目录，使用 mise 固定工具重新构建成功，见 build-theme-studio-final.log、build-component-catalog.log、cache-workaround.json。\n'
+'- R5 增拍 U2-search-row，以完整呈现跨单元颜色搜索行。U1-zoom200-source 首拍之后沿用 R4 的稳定后补采，复核采用 -a2 与 -end，原图保留。\n\n'
+'## 逐图登记\n\n'+table(records))
print(json.dumps({'shots':len(records),'units':counts,'failures':failed},ensure_ascii=False))
