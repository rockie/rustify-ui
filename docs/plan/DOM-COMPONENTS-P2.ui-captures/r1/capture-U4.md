# DOM-COMPONENTS-P2 R1 · 实拍采集

- HEAD: 64bb17281567a646b312a4b7050219fe5a7f0c55；main；dirty fingerprint: 19479199a276dd54c403765cc1f9af125254e0e60c038875cda3942d39364c1c。产品源码无未提交差异，原有未提交内容是仓库指引、历史评审与状态台账。
- 目标指纹：phoenix-ui-target-v1/manual-u64be:43a74974da555b476c3c8ce9a27112cba19b79d6aa75a96a5a156a6d6f402c29；目标文件 SHA-256 3703966418fcf4a31531799167d5b4874a4dba8a076a15e339456b91db881c89。
- 指纹规则：UTF-8 LF; heading sections 单元 then 规范要点; each includes heading and one trailing LF; domain phoenix-ui-target-v1 NUL; u64be byte length + bytes; no linked local attachments in these sections. Manual implementation, not engine-verified. 使用固定人工编码；无 engine 比对，不与未知算法跨轮比较。
- 工具：Playwright 1.63.0 + 本机 Google Chrome；headless，DPR1，locale en-US、Asia/Shanghai；截图等待字体、animations disabled。系统原生 validationMessage 实际为中文；未替换验证文案。
- 工作树当前构建：Studio build_id dc7fb113f1267f33；raw/consumer 本轮 build-dom；catalog/workbench 本轮 release。均由仓内 mbx 命令生成，浏览器通过 HTTP 加载；未拦截网络、未注入替代 UI。
- 状态通过实际选择/点击/按键/鼠标操作到达；为取景仅移动真实滚动位置。无截图拼接/修图，未使用旧图。
- 首轮没有上一轮图片/评分。未执行 OS IME、VoiceOver、真机触控或浏览器 UI 200% 缩放；按计划不归入本期必需格子。

## 范围 U4

本单元主证据如下；其它单元本评审不评分。

### U4 · Slider-s1-classic

- 图：[U4-Slider-s1-classic-1440-light.png](U4-Slider-s1-classic-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:44:43.767Z；1440×1000，DPR 1，light。
- 实际操作：组件 Slider，size=1，variant=classic
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "SUMMARY",
    "id": null,
    "role": null,
    "text": "States and parts"
  },
  "visibleText": "Component\nCheckbox\nSwitch\nToggle\nRadioGroup\nCheckboxGroup\nToggleGroup\nRadioCards\nCheckboxCards\nSlider\nRangeSlider\nTextField\nTextArea\nNumberField\nColorField\nSelect\nPresentation\nThemes\nPrimitives\nValue requests\nAccept\nRefuse\nAccept later\nSize\n1\n2\n3\nVariant\nsurface\nclassic\nsoft\nLocal colour\nInherit\nBlue\nRed\nGray\nState\nNormal\nDisabled\nRead only\nInvalid\nRadius\nInherit\n0\nFull\nRTL\nHigh contrast\n\nInteract with the example. Requests are separate from accepted values. Number and colour fields keep a draft until input completes.\n\nRequest count\n0\nLast request\n—\nAccepted value\n50\nCommit / cancel event\n—\nAccept pending value"
}
```

### U4 · Slider-s2-surface

- 图：[U4-Slider-s2-surface-1440-light.png](U4-Slider-s2-surface-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:44:44.032Z；1440×1000，DPR 1，light。
- 实际操作：组件 Slider，size=2，variant=surface
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "SUMMARY",
    "id": null,
    "role": null,
    "text": "States and parts"
  },
  "visibleText": "Component\nCheckbox\nSwitch\nToggle\nRadioGroup\nCheckboxGroup\nToggleGroup\nRadioCards\nCheckboxCards\nSlider\nRangeSlider\nTextField\nTextArea\nNumberField\nColorField\nSelect\nPresentation\nThemes\nPrimitives\nValue requests\nAccept\nRefuse\nAccept later\nSize\n1\n2\n3\nVariant\nsurface\nclassic\nsoft\nLocal colour\nInherit\nBlue\nRed\nGray\nState\nNormal\nDisabled\nRead only\nInvalid\nRadius\nInherit\n0\nFull\nRTL\nHigh contrast\n\nInteract with the example. Requests are separate from accepted values. Number and colour fields keep a draft until input completes.\n\nRequest count\n0\nLast request\n—\nAccepted value\n50\nCommit / cancel event\n—\nAccept pending value"
}
```

### U4 · Slider-s3-soft

- 图：[U4-Slider-s3-soft-1440-light.png](U4-Slider-s3-soft-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:44:44.297Z；1440×1000，DPR 1，light。
- 实际操作：组件 Slider，size=3，variant=soft
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "SUMMARY",
    "id": null,
    "role": null,
    "text": "States and parts"
  },
  "visibleText": "Component\nCheckbox\nSwitch\nToggle\nRadioGroup\nCheckboxGroup\nToggleGroup\nRadioCards\nCheckboxCards\nSlider\nRangeSlider\nTextField\nTextArea\nNumberField\nColorField\nSelect\nPresentation\nThemes\nPrimitives\nValue requests\nAccept\nRefuse\nAccept later\nSize\n1\n2\n3\nVariant\nsurface\nclassic\nsoft\nLocal colour\nInherit\nBlue\nRed\nGray\nState\nNormal\nDisabled\nRead only\nInvalid\nRadius\nInherit\n0\nFull\nRTL\nHigh contrast\n\nInteract with the example. Requests are separate from accepted values. Number and colour fields keep a draft until input completes.\n\nRequest count\n0\nLast request\n—\nAccepted value\n50\nCommit / cancel event\n—\nAccept pending value"
}
```

### U4 · RangeSlider-s1-classic

- 图：[U4-RangeSlider-s1-classic-1440-light.png](U4-RangeSlider-s1-classic-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:44:44.664Z；1440×1000，DPR 1，light。
- 实际操作：组件 RangeSlider，size=1，variant=classic
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "SUMMARY",
    "id": null,
    "role": null,
    "text": "States and parts"
  },
  "visibleText": "Component\nCheckbox\nSwitch\nToggle\nRadioGroup\nCheckboxGroup\nToggleGroup\nRadioCards\nCheckboxCards\nSlider\nRangeSlider\nTextField\nTextArea\nNumberField\nColorField\nSelect\nPresentation\nThemes\nPrimitives\nValue requests\nAccept\nRefuse\nAccept later\nSize\n1\n2\n3\nVariant\nsurface\nclassic\nsoft\nLocal colour\nInherit\nBlue\nRed\nGray\nState\nNormal\nDisabled\nRead only\nInvalid\nRadius\nInherit\n0\nFull\nRTL\nHigh contrast\n\nInteract with the example. Requests are separate from accepted values. Number and colour fields keep a draft until input completes.\n\nRequest count\n0\nLast request\n—\nAccepted value\nstart=20, end=80\nCommit / cancel event\n—\nAccept pending value"
}
```

### U4 · RangeSlider-s2-surface

- 图：[U4-RangeSlider-s2-surface-1440-light.png](U4-RangeSlider-s2-surface-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:44:44.913Z；1440×1000，DPR 1，light。
- 实际操作：组件 RangeSlider，size=2，variant=surface
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "SUMMARY",
    "id": null,
    "role": null,
    "text": "States and parts"
  },
  "visibleText": "Component\nCheckbox\nSwitch\nToggle\nRadioGroup\nCheckboxGroup\nToggleGroup\nRadioCards\nCheckboxCards\nSlider\nRangeSlider\nTextField\nTextArea\nNumberField\nColorField\nSelect\nPresentation\nThemes\nPrimitives\nValue requests\nAccept\nRefuse\nAccept later\nSize\n1\n2\n3\nVariant\nsurface\nclassic\nsoft\nLocal colour\nInherit\nBlue\nRed\nGray\nState\nNormal\nDisabled\nRead only\nInvalid\nRadius\nInherit\n0\nFull\nRTL\nHigh contrast\n\nInteract with the example. Requests are separate from accepted values. Number and colour fields keep a draft until input completes.\n\nRequest count\n0\nLast request\n—\nAccepted value\nstart=20, end=80\nCommit / cancel event\n—\nAccept pending value"
}
```

### U4 · RangeSlider-s3-soft

- 图：[U4-RangeSlider-s3-soft-1440-light.png](U4-RangeSlider-s3-soft-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:44:45.181Z；1440×1000，DPR 1，light。
- 实际操作：组件 RangeSlider，size=3，variant=soft
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "SUMMARY",
    "id": null,
    "role": null,
    "text": "States and parts"
  },
  "visibleText": "Component\nCheckbox\nSwitch\nToggle\nRadioGroup\nCheckboxGroup\nToggleGroup\nRadioCards\nCheckboxCards\nSlider\nRangeSlider\nTextField\nTextArea\nNumberField\nColorField\nSelect\nPresentation\nThemes\nPrimitives\nValue requests\nAccept\nRefuse\nAccept later\nSize\n1\n2\n3\nVariant\nsurface\nclassic\nsoft\nLocal colour\nInherit\nBlue\nRed\nGray\nState\nNormal\nDisabled\nRead only\nInvalid\nRadius\nInherit\n0\nFull\nRTL\nHigh contrast\n\nInteract with the example. Requests are separate from accepted values. Number and colour fields keep a draft until input completes.\n\nRequest count\n0\nLast request\n—\nAccepted value\nstart=20, end=80\nCommit / cancel event\n—\nAccept pending value"
}
```

### U4 · range-refused

- 图：[U4-range-refused-1440-light.png](U4-range-refused-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:44:50.651Z；1440×1000，DPR 1，light。
- 实际操作：拒绝策略；首滑块按 End，显示请求与接受值
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "BUTTON",
    "id": "parts-start",
    "role": "slider",
    "text": ""
  },
  "visibleText": "Component\nCheckbox\nSwitch\nToggle\nRadioGroup\nCheckboxGroup\nToggleGroup\nRadioCards\nCheckboxCards\nSlider\nRangeSlider\nTextField\nTextArea\nNumberField\nColorField\nSelect\nPresentation\nThemes\nPrimitives\nValue requests\nAccept\nRefuse\nAccept later\nSize\n1\n2\n3\nVariant\nsurface\nclassic\nsoft\nLocal colour\nInherit\nBlue\nRed\nGray\nState\nNormal\nDisabled\nRead only\nInvalid\nRadius\nInherit\n0\nFull\nRTL\nHigh contrast\n\nInteract with the example. Requests are separate from accepted values. Number and colour fields keep a draft until input completes.\n\nRequest count\n1\nLast request\nend=80, start=100\nAccepted value\nstart=20, end=80\nCommit / cancel event\ncommit\nAccept pending value"
}
```

### U4 · range-crossed-focus

- 图：[U4-range-crossed-focus-1440-light.png](U4-range-crossed-focus-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:44:50.897Z；1440×1000，DPR 1，light。
- 实际操作：接受策略；首滑块 End 跨越后 Tab 到下一固定身份滑块
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "BUTTON",
    "id": "parts-end",
    "role": "slider",
    "text": ""
  },
  "visibleText": "Component\nCheckbox\nSwitch\nToggle\nRadioGroup\nCheckboxGroup\nToggleGroup\nRadioCards\nCheckboxCards\nSlider\nRangeSlider\nTextField\nTextArea\nNumberField\nColorField\nSelect\nPresentation\nThemes\nPrimitives\nValue requests\nAccept\nRefuse\nAccept later\nSize\n1\n2\n3\nVariant\nsurface\nclassic\nsoft\nLocal colour\nInherit\nBlue\nRed\nGray\nState\nNormal\nDisabled\nRead only\nInvalid\nRadius\nInherit\n0\nFull\nRTL\nHigh contrast\n\nInteract with the example. Requests are separate from accepted values. Number and colour fields keep a draft until input completes.\n\nRequest count\n2\nLast request\nend=80, start=100\nAccepted value\nend=80, start=100\nCommit / cancel event\ncommit\nAccept pending value"
}
```

### U4 · dark-rtl

- 图：[U4-dark-rtl-1440-dark.png](U4-dark-rtl-1440-dark.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:44:51.281Z；1440×1000，DPR 1，dark。
- 实际操作：深色RTL多滑块
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "BUTTON",
    "id": "mode-dark",
    "role": null,
    "text": "Dark"
  },
  "visibleText": "Component\nCheckbox\nSwitch\nToggle\nRadioGroup\nCheckboxGroup\nToggleGroup\nRadioCards\nCheckboxCards\nSlider\nRangeSlider\nTextField\nTextArea\nNumberField\nColorField\nSelect\nPresentation\nThemes\nPrimitives\nValue requests\nAccept\nRefuse\nAccept later\nSize\n1\n2\n3\nVariant\nsurface\nclassic\nsoft\nLocal colour\nInherit\nBlue\nRed\nGray\nState\nNormal\nDisabled\nRead only\nInvalid\nRadius\nInherit\n0\nFull\nRTL\nHigh contrast\n\nInteract with the example. Requests are separate from accepted values. Number and colour fields keep a draft until input completes.\n\nRequest count\n2\nLast request\nend=80, start=100\nAccepted value\nend=80, start=100\nCommit / cancel event\ncommit\nAccept pending value"
}
```

### U4 · raw-vertical

- 图：[U4-raw-vertical-1440-light.png](U4-raw-vertical-1440-light.png)
- 出处：http://127.0.0.1:4190/；2026-10-31T06:46:45.521Z；1440×1000，DPR 1，light。
- 实际操作：Vertical thumb ArrowUp
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "BUTTON",
    "id": "slider-vertical-thumb",
    "role": "slider",
    "text": ""
  },
  "request": "vertical=36"
}
```

### U4 · raw-inverted

- 图：[U4-raw-inverted-1440-light.png](U4-raw-inverted-1440-light.png)
- 出处：http://127.0.0.1:4190/；2026-10-31T06:46:45.734Z；1440×1000，DPR 1，light。
- 实际操作：Inverted vertical thumb ArrowUp
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "BUTTON",
    "id": "slider-inverted-thumb",
    "role": "slider",
    "text": ""
  },
  "request": "vertical=34"
}
```

### U4 · raw-cancel

- 图：[U4-raw-cancel-1440-light.png](U4-raw-cancel-1440-light.png)
- 出处：http://127.0.0.1:4190/；2026-10-31T06:46:46.021Z；1440×1000，DPR 1，light。
- 实际操作：Mouse drag then Escape cancel
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "BUTTON",
    "id": "slider-left",
    "role": "slider",
    "text": ""
  },
  "commits": "0",
  "cancels": "1"
}
```

