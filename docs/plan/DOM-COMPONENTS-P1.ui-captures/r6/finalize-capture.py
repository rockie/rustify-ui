from pathlib import Path
import json, hashlib, struct
from collections import Counter

p = Path(__file__).resolve().parent
q = p / 'revisions'
q.mkdir(exist_ok=True)
for name in ['judge.md', 'judge-results.json', 'judge-evidence-check.json', 'judge-dispatch.json', 'capture.md', 'capture-qa.json', 'final-verification.json', 'capture-health-summary.json']:
    dest = q / (name.replace('.md', '-before-matrix-supplement.md') if name.endswith('.md') else name.replace('.json', '-before-matrix-supplement.json'))
    if not dest.exists():
        dest.write_bytes((p / name).read_bytes())
runs = json.loads((p / 'capture-log.json').read_text())['runs']
records = [r for run in runs for r in run['records']]
assert len(records) == 236
assert len({r['file'] for r in records}) == 236
old = {r['file']: r['sha256'] for r in json.loads((p / 'image-integrity.json').read_text())}
images = []
for r in records:
    data = (p / r['file']).read_bytes()
    width, height = struct.unpack('>II', data[16:24])
    sha = hashlib.sha256(data).hexdigest()
    if r['file'] in old:
        assert sha == old[r['file']]
    images.append({'file': r['file'], 'width': width, 'height': height, 'bytes': len(data), 'sha256': sha})
(p / 'image-integrity-final.json').write_text(json.dumps(images, indent=2) + '\n')
extra = [r for run in runs[2:] for r in run['records']]
heading = '\n## 交付前矩阵补采与范围更正\n\n'
intro = '''首批 154 张覆盖了全部步骤，但遗漏了部分状态 × 视口 × 主题组合。先前“无必需格子缺失”的范围判断在交付前回读矩阵时更正；初版清单、评分和报告保留在 revisions/，不覆盖任何原图。追加 82 张后共 **236 张**：'''
intro += ', '.join(f'{u} {n}' for u, n in sorted(Counter(r['unit'] for r in records).items()))
intro += '''。本节及各单元 -b 清单为完整补采索引，旧 -a 清单保持原样。

- U1：补 768/390 中文与四个键盘焦点帧，1440 深色默认/中文/键盘帧。
- U2：补 390 的来源、单项/分组解除、展开影响清单首尾、字段错误与低对比。
- U5：补 1440 深色和 390 浅色的嵌套相反边界、菜单、对话框、拾取、DOM+GPU；窄屏拾取后按真实导航返回 Preview 再继续。
- U6：补 1280 深色、390 浅色的 loading/键盘/110%/full/中文状态和 Link 悬停/焦点。catalog 的布局夹具没有深色入口；深色 Link 使用同 SDK 的 Studio DOM 预览中 Edit theme 真实链接，在 1280×900 实拍 normal/hover/keyboard-focus。只改变采集入口，不改组件样式或目标，不把这一链接的结果外推到所有参数组合。1440 的同链接帧作为额外证据保留。
- U7：补 390 的 110%/95% 整体、Heading/Kbd 分段及 invalid 整体/诊断。
- 独立补评后再追加 U2 390 低对比预览诊断，以及 U1 390 中文预览页，各 1 张；U1/U2 再由新的独立评审读取本单元完整材料复核。中文与 200% 是两个独立状态，不假定所有状态彼此做组合；额外中文 200% 未采，不外推通过。
- 两次补采脚本超时保留：中文界面中仍用英文预设名定位、窄屏拾取后未回 Preview 而点隐藏按钮。分别通过先选预设再切中文、真实返回 Preview 补齐；它们是采集脚本问题，不登记产品故障。最终 followup 13 帧无失败，见 matrix-followup-log.json。
- 补采后的 U1/U2/U5/U6/U7 各交给新的独立评审，只给目标、本轮完整原图和上一轮 R5 评审材料；不提供本轮初评分、报告或机器检查。

'''
head = '| 单元 | 状态 | 原始截图 | URL | CSS视口 | DPR | 主题 | 截图方式 | 拍摄UTC | 焦点 | 数据/环境干预 |\n| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |\n'
rows = '\n'.join(f'| {r["unit"]} | {r["state"]} | [{r["file"]}]({r["file"]}) | {r["url"]} | {r["viewport"]} | {r["dpr"]} | {r["theme"]} | {r["kind"]} | {r["time"]} | {r["focused"] or "—"} | {r["interventions"]} |' for r in extra) + '\n'
capture = p / 'capture.md'
text = capture.read_text().split(heading, 1)[0]
capture.write_text(text + heading + intro + head + rows)
qa = json.loads((p / 'capture-qa.json').read_text())
qa.update(source_images=236, primary_visual_method='20 derived contact sheets inspect 234 source frames for authenticity; last two original frames opened directly; key originals opened across U1–U7. Judges use original PNGs only.', contact_sheets=[f'qa/contact-{n:02}.png' for n in range(1, 21)], supplement_frames=82, supplement_contact_sheets_opened=[f'qa/contact-{n:02}.png' for n in range(13, 21)], additional_originals_opened=['U2-matrix3-low-contrast-preview-390-light.png', 'U1-matrix3-zh-preview-390-light.png'])
(p / 'capture-qa.json').write_text(json.dumps(qa, ensure_ascii=False, indent=2) + '\n')
print(json.dumps({'images': len(records), 'supplement': len(extra), 'counts': Counter(r['unit'] for r in records), 'original_154_hashes_unchanged': True}, ensure_ascii=False))
