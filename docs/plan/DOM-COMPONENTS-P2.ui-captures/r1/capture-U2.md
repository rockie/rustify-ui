# DOM-COMPONENTS-P2 R1 · 实拍采集

- HEAD: 64bb17281567a646b312a4b7050219fe5a7f0c55；main；dirty fingerprint: 19479199a276dd54c403765cc1f9af125254e0e60c038875cda3942d39364c1c。产品源码无未提交差异，原有未提交内容是仓库指引、历史评审与状态台账。
- 目标指纹：phoenix-ui-target-v1/manual-u64be:43a74974da555b476c3c8ce9a27112cba19b79d6aa75a96a5a156a6d6f402c29；目标文件 SHA-256 3703966418fcf4a31531799167d5b4874a4dba8a076a15e339456b91db881c89。
- 指纹规则：UTF-8 LF; heading sections 单元 then 规范要点; each includes heading and one trailing LF; domain phoenix-ui-target-v1 NUL; u64be byte length + bytes; no linked local attachments in these sections. Manual implementation, not engine-verified. 使用固定人工编码；无 engine 比对，不与未知算法跨轮比较。
- 工具：Playwright 1.63.0 + 本机 Google Chrome；headless，DPR1，locale en-US、Asia/Shanghai；截图等待字体、animations disabled。系统原生 validationMessage 实际为中文；未替换验证文案。
- 工作树当前构建：Studio build_id dc7fb113f1267f33；raw/consumer 本轮 build-dom；catalog/workbench 本轮 release。均由仓内 mbx 命令生成，浏览器通过 HTTP 加载；未拦截网络、未注入替代 UI。
- 状态通过实际选择/点击/按键/鼠标操作到达；为取景仅移动真实滚动位置。无截图拼接/修图，未使用旧图。
- 首轮没有上一轮图片/评分。未执行 OS IME、VoiceOver、真机触控或浏览器 UI 200% 缩放；按计划不归入本期必需格子。

## 范围 U2

本单元主证据如下；其它单元本评审不评分。

### U2 · Checkbox-s1-classic

- 图：[U2-Checkbox-s1-classic-1440-light.png](U2-Checkbox-s1-classic-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:44:33.967Z；1440×1000，DPR 1，light。
- 实际操作：组件 Checkbox，size=1，variant=classic
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "SUMMARY",
    "id": null,
    "role": null,
    "text": "States and parts"
  },
  "visibleText": "Component\nCheckbox\nSwitch\nToggle\nRadioGroup\nCheckboxGroup\nToggleGroup\nRadioCards\nCheckboxCards\nSlider\nRangeSlider\nTextField\nTextArea\nNumberField\nColorField\nSelect\nPresentation\nThemes\nPrimitives\nValue requests\nAccept\nRefuse\nAccept later\nSize\n1\n2\n3\nVariant\nsurface\nclassic\nsoft\nLocal colour\nInherit\nBlue\nRed\nGray\nState\nNormal\nDisabled\nRead only\nInvalid\nRTL\nHigh contrast\nSet indeterminate\n\nInteract with the example. Requests are separate from accepted values. Number and colour fields keep a draft until input completes.\n\nRequest count\n0\nLast request\n—\nAccepted value\nIndeterminate\nCommit / cancel event\n—\nAccept pending value"
}
```

### U2 · Checkbox-s2-surface

- 图：[U2-Checkbox-s2-surface-1440-light.png](U2-Checkbox-s2-surface-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:44:34.254Z；1440×1000，DPR 1，light。
- 实际操作：组件 Checkbox，size=2，variant=surface
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "SUMMARY",
    "id": null,
    "role": null,
    "text": "States and parts"
  },
  "visibleText": "Component\nCheckbox\nSwitch\nToggle\nRadioGroup\nCheckboxGroup\nToggleGroup\nRadioCards\nCheckboxCards\nSlider\nRangeSlider\nTextField\nTextArea\nNumberField\nColorField\nSelect\nPresentation\nThemes\nPrimitives\nValue requests\nAccept\nRefuse\nAccept later\nSize\n1\n2\n3\nVariant\nsurface\nclassic\nsoft\nLocal colour\nInherit\nBlue\nRed\nGray\nState\nNormal\nDisabled\nRead only\nInvalid\nRTL\nHigh contrast\nSet indeterminate\n\nInteract with the example. Requests are separate from accepted values. Number and colour fields keep a draft until input completes.\n\nRequest count\n0\nLast request\n—\nAccepted value\nIndeterminate\nCommit / cancel event\n—\nAccept pending value"
}
```

### U2 · Checkbox-s3-soft

- 图：[U2-Checkbox-s3-soft-1440-light.png](U2-Checkbox-s3-soft-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:44:34.566Z；1440×1000，DPR 1，light。
- 实际操作：组件 Checkbox，size=3，variant=soft
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "SUMMARY",
    "id": null,
    "role": null,
    "text": "States and parts"
  },
  "visibleText": "Component\nCheckbox\nSwitch\nToggle\nRadioGroup\nCheckboxGroup\nToggleGroup\nRadioCards\nCheckboxCards\nSlider\nRangeSlider\nTextField\nTextArea\nNumberField\nColorField\nSelect\nPresentation\nThemes\nPrimitives\nValue requests\nAccept\nRefuse\nAccept later\nSize\n1\n2\n3\nVariant\nsurface\nclassic\nsoft\nLocal colour\nInherit\nBlue\nRed\nGray\nState\nNormal\nDisabled\nRead only\nInvalid\nRTL\nHigh contrast\nSet indeterminate\n\nInteract with the example. Requests are separate from accepted values. Number and colour fields keep a draft until input completes.\n\nRequest count\n0\nLast request\n—\nAccepted value\nIndeterminate\nCommit / cancel event\n—\nAccept pending value"
}
```

### U2 · Switch-s1-classic

- 图：[U2-Switch-s1-classic-1440-light.png](U2-Switch-s1-classic-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:44:34.953Z；1440×1000，DPR 1，light。
- 实际操作：组件 Switch，size=1，variant=classic
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "SUMMARY",
    "id": null,
    "role": null,
    "text": "States and parts"
  },
  "visibleText": "Component\nCheckbox\nSwitch\nToggle\nRadioGroup\nCheckboxGroup\nToggleGroup\nRadioCards\nCheckboxCards\nSlider\nRangeSlider\nTextField\nTextArea\nNumberField\nColorField\nSelect\nPresentation\nThemes\nPrimitives\nValue requests\nAccept\nRefuse\nAccept later\nSize\n1\n2\n3\nVariant\nsurface\nclassic\nsoft\nLocal colour\nInherit\nBlue\nRed\nGray\nState\nNormal\nDisabled\nRead only\nInvalid\nRadius\nInherit\n0\nFull\nRTL\nHigh contrast\n\nInteract with the example. Requests are separate from accepted values. Number and colour fields keep a draft until input completes.\n\nRequest count\n0\nLast request\n—\nAccepted value\nfalse\nCommit / cancel event\n—\nAccept pending value"
}
```

### U2 · Switch-s2-surface

- 图：[U2-Switch-s2-surface-1440-light.png](U2-Switch-s2-surface-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:44:35.222Z；1440×1000，DPR 1，light。
- 实际操作：组件 Switch，size=2，variant=surface
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "SUMMARY",
    "id": null,
    "role": null,
    "text": "States and parts"
  },
  "visibleText": "Component\nCheckbox\nSwitch\nToggle\nRadioGroup\nCheckboxGroup\nToggleGroup\nRadioCards\nCheckboxCards\nSlider\nRangeSlider\nTextField\nTextArea\nNumberField\nColorField\nSelect\nPresentation\nThemes\nPrimitives\nValue requests\nAccept\nRefuse\nAccept later\nSize\n1\n2\n3\nVariant\nsurface\nclassic\nsoft\nLocal colour\nInherit\nBlue\nRed\nGray\nState\nNormal\nDisabled\nRead only\nInvalid\nRadius\nInherit\n0\nFull\nRTL\nHigh contrast\n\nInteract with the example. Requests are separate from accepted values. Number and colour fields keep a draft until input completes.\n\nRequest count\n0\nLast request\n—\nAccepted value\nfalse\nCommit / cancel event\n—\nAccept pending value"
}
```

### U2 · Switch-s3-soft

- 图：[U2-Switch-s3-soft-1440-light.png](U2-Switch-s3-soft-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:44:35.501Z；1440×1000，DPR 1，light。
- 实际操作：组件 Switch，size=3，variant=soft
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "SUMMARY",
    "id": null,
    "role": null,
    "text": "States and parts"
  },
  "visibleText": "Component\nCheckbox\nSwitch\nToggle\nRadioGroup\nCheckboxGroup\nToggleGroup\nRadioCards\nCheckboxCards\nSlider\nRangeSlider\nTextField\nTextArea\nNumberField\nColorField\nSelect\nPresentation\nThemes\nPrimitives\nValue requests\nAccept\nRefuse\nAccept later\nSize\n1\n2\n3\nVariant\nsurface\nclassic\nsoft\nLocal colour\nInherit\nBlue\nRed\nGray\nState\nNormal\nDisabled\nRead only\nInvalid\nRadius\nInherit\n0\nFull\nRTL\nHigh contrast\n\nInteract with the example. Requests are separate from accepted values. Number and colour fields keep a draft until input completes.\n\nRequest count\n0\nLast request\n—\nAccepted value\nfalse\nCommit / cancel event\n—\nAccept pending value"
}
```

### U2 · RadioGroup-s1-classic

- 图：[U2-RadioGroup-s1-classic-1440-light.png](U2-RadioGroup-s1-classic-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:44:35.885Z；1440×1000，DPR 1，light。
- 实际操作：组件 RadioGroup，size=1，variant=classic
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "SUMMARY",
    "id": null,
    "role": null,
    "text": "States and parts"
  },
  "visibleText": "Component\nCheckbox\nSwitch\nToggle\nRadioGroup\nCheckboxGroup\nToggleGroup\nRadioCards\nCheckboxCards\nSlider\nRangeSlider\nTextField\nTextArea\nNumberField\nColorField\nSelect\nPresentation\nThemes\nPrimitives\nValue requests\nAccept\nRefuse\nAccept later\nSize\n1\n2\n3\nVariant\nsurface\nclassic\nsoft\nLocal colour\nInherit\nBlue\nRed\nGray\nState\nNormal\nDisabled\nRead only\nInvalid\nRTL\nHigh contrast\n\nInteract with the example. Requests are separate from accepted values. Number and colour fields keep a draft until input completes.\n\nEmail\nActivity feed\nRequest count\n0\nLast request\n—\nAccepted value\n\"a\"\nCommit / cancel event\n—\nAccept pending value"
}
```

### U2 · RadioGroup-s2-surface

- 图：[U2-RadioGroup-s2-surface-1440-light.png](U2-RadioGroup-s2-surface-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:44:36.171Z；1440×1000，DPR 1，light。
- 实际操作：组件 RadioGroup，size=2，variant=surface
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "SUMMARY",
    "id": null,
    "role": null,
    "text": "States and parts"
  },
  "visibleText": "Component\nCheckbox\nSwitch\nToggle\nRadioGroup\nCheckboxGroup\nToggleGroup\nRadioCards\nCheckboxCards\nSlider\nRangeSlider\nTextField\nTextArea\nNumberField\nColorField\nSelect\nPresentation\nThemes\nPrimitives\nValue requests\nAccept\nRefuse\nAccept later\nSize\n1\n2\n3\nVariant\nsurface\nclassic\nsoft\nLocal colour\nInherit\nBlue\nRed\nGray\nState\nNormal\nDisabled\nRead only\nInvalid\nRTL\nHigh contrast\n\nInteract with the example. Requests are separate from accepted values. Number and colour fields keep a draft until input completes.\n\nEmail\nActivity feed\nRequest count\n0\nLast request\n—\nAccepted value\n\"a\"\nCommit / cancel event\n—\nAccept pending value"
}
```

### U2 · RadioGroup-s3-soft

- 图：[U2-RadioGroup-s3-soft-1440-light.png](U2-RadioGroup-s3-soft-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:44:36.479Z；1440×1000，DPR 1，light。
- 实际操作：组件 RadioGroup，size=3，variant=soft
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "SUMMARY",
    "id": null,
    "role": null,
    "text": "States and parts"
  },
  "visibleText": "Component\nCheckbox\nSwitch\nToggle\nRadioGroup\nCheckboxGroup\nToggleGroup\nRadioCards\nCheckboxCards\nSlider\nRangeSlider\nTextField\nTextArea\nNumberField\nColorField\nSelect\nPresentation\nThemes\nPrimitives\nValue requests\nAccept\nRefuse\nAccept later\nSize\n1\n2\n3\nVariant\nsurface\nclassic\nsoft\nLocal colour\nInherit\nBlue\nRed\nGray\nState\nNormal\nDisabled\nRead only\nInvalid\nRTL\nHigh contrast\n\nInteract with the example. Requests are separate from accepted values. Number and colour fields keep a draft until input completes.\n\nEmail\nActivity feed\nRequest count\n0\nLast request\n—\nAccepted value\n\"a\"\nCommit / cancel event\n—\nAccept pending value"
}
```

### U2 · CheckboxGroup-s1-classic

- 图：[U2-CheckboxGroup-s1-classic-1440-light.png](U2-CheckboxGroup-s1-classic-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:44:36.884Z；1440×1000，DPR 1，light。
- 实际操作：组件 CheckboxGroup，size=1，variant=classic
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "SUMMARY",
    "id": null,
    "role": null,
    "text": "States and parts"
  },
  "visibleText": "Component\nCheckbox\nSwitch\nToggle\nRadioGroup\nCheckboxGroup\nToggleGroup\nRadioCards\nCheckboxCards\nSlider\nRangeSlider\nTextField\nTextArea\nNumberField\nColorField\nSelect\nPresentation\nThemes\nPrimitives\nValue requests\nAccept\nRefuse\nAccept later\nSize\n1\n2\n3\nVariant\nsurface\nclassic\nsoft\nLocal colour\nInherit\nBlue\nRed\nGray\nState\nNormal\nDisabled\nRead only\nInvalid\nRTL\nHigh contrast\n\nInteract with the example. Requests are separate from accepted values. Number and colour fields keep a draft until input completes.\n\nEmail\nActivity feed\nRequest count\n0\nLast request\n—\nAccepted value\n[\"a\"]\nCommit / cancel event\n—\nAccept pending value"
}
```

### U2 · CheckboxGroup-s2-surface

- 图：[U2-CheckboxGroup-s2-surface-1440-light.png](U2-CheckboxGroup-s2-surface-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:44:37.166Z；1440×1000，DPR 1，light。
- 实际操作：组件 CheckboxGroup，size=2，variant=surface
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "SUMMARY",
    "id": null,
    "role": null,
    "text": "States and parts"
  },
  "visibleText": "Component\nCheckbox\nSwitch\nToggle\nRadioGroup\nCheckboxGroup\nToggleGroup\nRadioCards\nCheckboxCards\nSlider\nRangeSlider\nTextField\nTextArea\nNumberField\nColorField\nSelect\nPresentation\nThemes\nPrimitives\nValue requests\nAccept\nRefuse\nAccept later\nSize\n1\n2\n3\nVariant\nsurface\nclassic\nsoft\nLocal colour\nInherit\nBlue\nRed\nGray\nState\nNormal\nDisabled\nRead only\nInvalid\nRTL\nHigh contrast\n\nInteract with the example. Requests are separate from accepted values. Number and colour fields keep a draft until input completes.\n\nEmail\nActivity feed\nRequest count\n0\nLast request\n—\nAccepted value\n[\"a\"]\nCommit / cancel event\n—\nAccept pending value"
}
```

### U2 · CheckboxGroup-s3-soft

- 图：[U2-CheckboxGroup-s3-soft-1440-light.png](U2-CheckboxGroup-s3-soft-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:44:37.433Z；1440×1000，DPR 1，light。
- 实际操作：组件 CheckboxGroup，size=3，variant=soft
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "SUMMARY",
    "id": null,
    "role": null,
    "text": "States and parts"
  },
  "visibleText": "Component\nCheckbox\nSwitch\nToggle\nRadioGroup\nCheckboxGroup\nToggleGroup\nRadioCards\nCheckboxCards\nSlider\nRangeSlider\nTextField\nTextArea\nNumberField\nColorField\nSelect\nPresentation\nThemes\nPrimitives\nValue requests\nAccept\nRefuse\nAccept later\nSize\n1\n2\n3\nVariant\nsurface\nclassic\nsoft\nLocal colour\nInherit\nBlue\nRed\nGray\nState\nNormal\nDisabled\nRead only\nInvalid\nRTL\nHigh contrast\n\nInteract with the example. Requests are separate from accepted values. Number and colour fields keep a draft until input completes.\n\nEmail\nActivity feed\nRequest count\n0\nLast request\n—\nAccepted value\n[\"a\"]\nCommit / cancel event\n—\nAccept pending value"
}
```

### U2 · RadioCards-s1-classic

- 图：[U2-RadioCards-s1-classic-1440-light.png](U2-RadioCards-s1-classic-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:44:37.800Z；1440×1000，DPR 1，light。
- 实际操作：组件 RadioCards，size=1，variant=classic
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "SUMMARY",
    "id": null,
    "role": null,
    "text": "States and parts"
  },
  "visibleText": "Component\nCheckbox\nSwitch\nToggle\nRadioGroup\nCheckboxGroup\nToggleGroup\nRadioCards\nCheckboxCards\nSlider\nRangeSlider\nTextField\nTextArea\nNumberField\nColorField\nSelect\nPresentation\nThemes\nPrimitives\nValue requests\nAccept\nRefuse\nAccept later\nSize\n1\n2\n3\nVariant\nsurface\nclassic\nLocal colour\nInherit\nBlue\nRed\nGray\nState\nNormal\nDisabled\nRead only\nInvalid\nRTL\nHigh contrast\n\nInteract with the example. Requests are separate from accepted values. Number and colour fields keep a draft until input completes.\n\nEmail\nActivity feed\nRequest count\n0\nLast request\n—\nAccepted value\n\"a\"\nCommit / cancel event\n—\nAccept pending value"
}
```

### U2 · RadioCards-s2-surface

- 图：[U2-RadioCards-s2-surface-1440-light.png](U2-RadioCards-s2-surface-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:44:38.084Z；1440×1000，DPR 1，light。
- 实际操作：组件 RadioCards，size=2，variant=surface
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "SUMMARY",
    "id": null,
    "role": null,
    "text": "States and parts"
  },
  "visibleText": "Component\nCheckbox\nSwitch\nToggle\nRadioGroup\nCheckboxGroup\nToggleGroup\nRadioCards\nCheckboxCards\nSlider\nRangeSlider\nTextField\nTextArea\nNumberField\nColorField\nSelect\nPresentation\nThemes\nPrimitives\nValue requests\nAccept\nRefuse\nAccept later\nSize\n1\n2\n3\nVariant\nsurface\nclassic\nLocal colour\nInherit\nBlue\nRed\nGray\nState\nNormal\nDisabled\nRead only\nInvalid\nRTL\nHigh contrast\n\nInteract with the example. Requests are separate from accepted values. Number and colour fields keep a draft until input completes.\n\nEmail\nActivity feed\nRequest count\n0\nLast request\n—\nAccepted value\n\"a\"\nCommit / cancel event\n—\nAccept pending value"
}
```

### U2 · RadioCards-s3-classic

- 图：[U2-RadioCards-s3-classic-1440-light.png](U2-RadioCards-s3-classic-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:44:38.366Z；1440×1000，DPR 1，light。
- 实际操作：组件 RadioCards，size=3，variant=classic
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "SUMMARY",
    "id": null,
    "role": null,
    "text": "States and parts"
  },
  "visibleText": "Component\nCheckbox\nSwitch\nToggle\nRadioGroup\nCheckboxGroup\nToggleGroup\nRadioCards\nCheckboxCards\nSlider\nRangeSlider\nTextField\nTextArea\nNumberField\nColorField\nSelect\nPresentation\nThemes\nPrimitives\nValue requests\nAccept\nRefuse\nAccept later\nSize\n1\n2\n3\nVariant\nsurface\nclassic\nLocal colour\nInherit\nBlue\nRed\nGray\nState\nNormal\nDisabled\nRead only\nInvalid\nRTL\nHigh contrast\n\nInteract with the example. Requests are separate from accepted values. Number and colour fields keep a draft until input completes.\n\nEmail\nActivity feed\nRequest count\n0\nLast request\n—\nAccepted value\n\"a\"\nCommit / cancel event\n—\nAccept pending value"
}
```

### U2 · CheckboxCards-s1-classic

- 图：[U2-CheckboxCards-s1-classic-1440-light.png](U2-CheckboxCards-s1-classic-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:44:38.716Z；1440×1000，DPR 1，light。
- 实际操作：组件 CheckboxCards，size=1，variant=classic
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "SUMMARY",
    "id": null,
    "role": null,
    "text": "States and parts"
  },
  "visibleText": "Component\nCheckbox\nSwitch\nToggle\nRadioGroup\nCheckboxGroup\nToggleGroup\nRadioCards\nCheckboxCards\nSlider\nRangeSlider\nTextField\nTextArea\nNumberField\nColorField\nSelect\nPresentation\nThemes\nPrimitives\nValue requests\nAccept\nRefuse\nAccept later\nSize\n1\n2\n3\nVariant\nsurface\nclassic\nLocal colour\nInherit\nBlue\nRed\nGray\nState\nNormal\nDisabled\nRead only\nInvalid\nRTL\nHigh contrast\n\nInteract with the example. Requests are separate from accepted values. Number and colour fields keep a draft until input completes.\n\nEmail\nActivity feed\nRequest count\n0\nLast request\n—\nAccepted value\n[\"a\"]\nCommit / cancel event\n—\nAccept pending value"
}
```

### U2 · CheckboxCards-s2-surface

- 图：[U2-CheckboxCards-s2-surface-1440-light.png](U2-CheckboxCards-s2-surface-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:44:38.984Z；1440×1000，DPR 1，light。
- 实际操作：组件 CheckboxCards，size=2，variant=surface
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "SUMMARY",
    "id": null,
    "role": null,
    "text": "States and parts"
  },
  "visibleText": "Component\nCheckbox\nSwitch\nToggle\nRadioGroup\nCheckboxGroup\nToggleGroup\nRadioCards\nCheckboxCards\nSlider\nRangeSlider\nTextField\nTextArea\nNumberField\nColorField\nSelect\nPresentation\nThemes\nPrimitives\nValue requests\nAccept\nRefuse\nAccept later\nSize\n1\n2\n3\nVariant\nsurface\nclassic\nLocal colour\nInherit\nBlue\nRed\nGray\nState\nNormal\nDisabled\nRead only\nInvalid\nRTL\nHigh contrast\n\nInteract with the example. Requests are separate from accepted values. Number and colour fields keep a draft until input completes.\n\nEmail\nActivity feed\nRequest count\n0\nLast request\n—\nAccepted value\n[\"a\"]\nCommit / cancel event\n—\nAccept pending value"
}
```

### U2 · CheckboxCards-s3-classic

- 图：[U2-CheckboxCards-s3-classic-1440-light.png](U2-CheckboxCards-s3-classic-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:44:39.246Z；1440×1000，DPR 1，light。
- 实际操作：组件 CheckboxCards，size=3，variant=classic
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "SUMMARY",
    "id": null,
    "role": null,
    "text": "States and parts"
  },
  "visibleText": "Component\nCheckbox\nSwitch\nToggle\nRadioGroup\nCheckboxGroup\nToggleGroup\nRadioCards\nCheckboxCards\nSlider\nRangeSlider\nTextField\nTextArea\nNumberField\nColorField\nSelect\nPresentation\nThemes\nPrimitives\nValue requests\nAccept\nRefuse\nAccept later\nSize\n1\n2\n3\nVariant\nsurface\nclassic\nLocal colour\nInherit\nBlue\nRed\nGray\nState\nNormal\nDisabled\nRead only\nInvalid\nRTL\nHigh contrast\n\nInteract with the example. Requests are separate from accepted values. Number and colour fields keep a draft until input completes.\n\nEmail\nActivity feed\nRequest count\n0\nLast request\n—\nAccepted value\n[\"a\"]\nCommit / cancel event\n—\nAccept pending value"
}
```

### U2 · Toggle-default

- 图：[U2-Toggle-default-1440-light.png](U2-Toggle-default-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:44:39.498Z；1440×1000，DPR 1，light。
- 实际操作：组件 Toggle 默认
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "SUMMARY",
    "id": null,
    "role": null,
    "text": "States and parts"
  },
  "visibleText": "Component\nCheckbox\nSwitch\nToggle\nRadioGroup\nCheckboxGroup\nToggleGroup\nRadioCards\nCheckboxCards\nSlider\nRangeSlider\nTextField\nTextArea\nNumberField\nColorField\nSelect\nPresentation\nThemes\nPrimitives\nValue requests\nAccept\nRefuse\nAccept later\nState\nNormal\nDisabled\nRead only\nInvalid\nRTL\n\nInteract with the example. Requests are separate from accepted values. Number and colour fields keep a draft until input completes.\n\nEmail\nRequest count\n0\nLast request\n—\nAccepted value\nfalse\nCommit / cancel event\n—\nAccept pending value"
}
```

### U2 · ToggleGroup-default

- 图：[U2-ToggleGroup-default-1440-light.png](U2-ToggleGroup-default-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:44:39.715Z；1440×1000，DPR 1，light。
- 实际操作：组件 ToggleGroup 默认
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "SUMMARY",
    "id": null,
    "role": null,
    "text": "States and parts"
  },
  "visibleText": "Component\nCheckbox\nSwitch\nToggle\nRadioGroup\nCheckboxGroup\nToggleGroup\nRadioCards\nCheckboxCards\nSlider\nRangeSlider\nTextField\nTextArea\nNumberField\nColorField\nSelect\nPresentation\nThemes\nPrimitives\nValue requests\nAccept\nRefuse\nAccept later\nState\nNormal\nDisabled\nRead only\nInvalid\nRTL\n\nInteract with the example. Requests are separate from accepted values. Number and colour fields keep a draft until input completes.\n\nEmail\nActivity feed\nRequest count\n0\nLast request\n—\nAccepted value\n[\"a\"]\nCommit / cancel event\n—\nAccept pending value"
}
```

### U2 · Checkbox-disabled

- 图：[U2-Checkbox-disabled-1440-light.png](U2-Checkbox-disabled-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:44:45.452Z；1440×1000，DPR 1，light。
- 实际操作：选择 Checkbox 的 disabled 状态
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "SUMMARY",
    "id": null,
    "role": null,
    "text": "States and parts"
  },
  "visibleText": "Component\nCheckbox\nSwitch\nToggle\nRadioGroup\nCheckboxGroup\nToggleGroup\nRadioCards\nCheckboxCards\nSlider\nRangeSlider\nTextField\nTextArea\nNumberField\nColorField\nSelect\nPresentation\nThemes\nPrimitives\nValue requests\nAccept\nRefuse\nAccept later\nSize\n1\n2\n3\nVariant\nsurface\nclassic\nsoft\nLocal colour\nInherit\nBlue\nRed\nGray\nState\nNormal\nDisabled\nRead only\nInvalid\nRTL\nHigh contrast\nSet indeterminate\n\nInteract with the example. Requests are separate from accepted values. Number and colour fields keep a draft until input completes.\n\nRequest count\n0\nLast request\n—\nAccepted value\nIndeterminate\nCommit / cancel event\n—\nAccept pending value"
}
```

### U2 · RadioGroup-readonly

- 图：[U2-RadioGroup-readonly-1440-light.png](U2-RadioGroup-readonly-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:44:45.732Z；1440×1000，DPR 1，light。
- 实际操作：选择 RadioGroup 的 readonly 状态
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "SUMMARY",
    "id": null,
    "role": null,
    "text": "States and parts"
  },
  "visibleText": "Component\nCheckbox\nSwitch\nToggle\nRadioGroup\nCheckboxGroup\nToggleGroup\nRadioCards\nCheckboxCards\nSlider\nRangeSlider\nTextField\nTextArea\nNumberField\nColorField\nSelect\nPresentation\nThemes\nPrimitives\nValue requests\nAccept\nRefuse\nAccept later\nSize\n1\n2\n3\nVariant\nsurface\nclassic\nsoft\nLocal colour\nInherit\nBlue\nRed\nGray\nState\nNormal\nDisabled\nRead only\nInvalid\nRTL\nHigh contrast\n\nInteract with the example. Requests are separate from accepted values. Number and colour fields keep a draft until input completes.\n\nEmail\nActivity feed\nRequest count\n0\nLast request\n—\nAccepted value\n\"a\"\nCommit / cancel event\n—\nAccept pending value"
}
```

### U2 · CheckboxCards-invalid

- 图：[U2-CheckboxCards-invalid-1440-light.png](U2-CheckboxCards-invalid-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:44:46.000Z；1440×1000，DPR 1，light。
- 实际操作：选择 CheckboxCards 的 invalid 状态
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "SUMMARY",
    "id": null,
    "role": null,
    "text": "States and parts"
  },
  "visibleText": "Component\nCheckbox\nSwitch\nToggle\nRadioGroup\nCheckboxGroup\nToggleGroup\nRadioCards\nCheckboxCards\nSlider\nRangeSlider\nTextField\nTextArea\nNumberField\nColorField\nSelect\nPresentation\nThemes\nPrimitives\nValue requests\nAccept\nRefuse\nAccept later\nSize\n1\n2\n3\nVariant\nsurface\nclassic\nLocal colour\nInherit\nBlue\nRed\nGray\nState\nNormal\nDisabled\nRead only\nInvalid\nRTL\nHigh contrast\n\nInteract with the example. Requests are separate from accepted values. Number and colour fields keep a draft until input completes.\n\nEmail\nActivity feed\nRequest count\n0\nLast request\n—\nAccepted value\n[\"a\"]\nCommit / cancel event\n—\nAccept pending value"
}
```

### U2 · refused

- 图：[U2-refused-1440-light.png](U2-refused-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:44:47.113Z；1440×1000，DPR 1，light。
- 实际操作：Checkbox 选择拒绝策略后点击
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "INPUT",
    "id": "parts-control",
    "role": null,
    "text": ""
  },
  "visibleText": "Component\nCheckbox\nSwitch\nToggle\nRadioGroup\nCheckboxGroup\nToggleGroup\nRadioCards\nCheckboxCards\nSlider\nRangeSlider\nTextField\nTextArea\nNumberField\nColorField\nSelect\nPresentation\nThemes\nPrimitives\nValue requests\nAccept\nRefuse\nAccept later\nSize\n1\n2\n3\nVariant\nsurface\nclassic\nsoft\nLocal colour\nInherit\nBlue\nRed\nGray\nState\nNormal\nDisabled\nRead only\nInvalid\nRTL\nHigh contrast\nSet indeterminate\n\nInteract with the example. Requests are separate from accepted values. Number and colour fields keep a draft until input completes.\n\nRequest count\n1\nLast request\nChecked\nAccepted value\nIndeterminate\nCommit / cancel event\n—\nAccept pending value"
}
```

### U2 · delayed

- 图：[U2-delayed-1440-light.png](U2-delayed-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:44:47.363Z；1440×1000，DPR 1，light。
- 实际操作：改延迟策略后点击，尚未接受
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "INPUT",
    "id": "parts-control",
    "role": null,
    "text": ""
  },
  "visibleText": "Component\nCheckbox\nSwitch\nToggle\nRadioGroup\nCheckboxGroup\nToggleGroup\nRadioCards\nCheckboxCards\nSlider\nRangeSlider\nTextField\nTextArea\nNumberField\nColorField\nSelect\nPresentation\nThemes\nPrimitives\nValue requests\nAccept\nRefuse\nAccept later\nSize\n1\n2\n3\nVariant\nsurface\nclassic\nsoft\nLocal colour\nInherit\nBlue\nRed\nGray\nState\nNormal\nDisabled\nRead only\nInvalid\nRTL\nHigh contrast\nSet indeterminate\n\nInteract with the example. Requests are separate from accepted values. Number and colour fields keep a draft until input completes.\n\nRequest count\n2\nLast request\nChecked\nAccepted value\nIndeterminate\nCommit / cancel event\n—\nAccept pending value"
}
```

### U2 · applied

- 图：[U2-applied-1440-light.png](U2-applied-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:44:47.597Z；1440×1000，DPR 1，light。
- 实际操作：点击接受延迟值
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "BODY",
    "id": null,
    "role": null,
    "text": "\n  \n    EditPreviewRustify LightMatches reference documentSaved on this devicePr"
  },
  "visibleText": "Component\nCheckbox\nSwitch\nToggle\nRadioGroup\nCheckboxGroup\nToggleGroup\nRadioCards\nCheckboxCards\nSlider\nRangeSlider\nTextField\nTextArea\nNumberField\nColorField\nSelect\nPresentation\nThemes\nPrimitives\nValue requests\nAccept\nRefuse\nAccept later\nSize\n1\n2\n3\nVariant\nsurface\nclassic\nsoft\nLocal colour\nInherit\nBlue\nRed\nGray\nState\nNormal\nDisabled\nRead only\nInvalid\nRTL\nHigh contrast\nSet indeterminate\n\nInteract with the example. Requests are separate from accepted values. Number and colour fields keep a draft until input completes.\n\nRequest count\n2\nLast request\nChecked\nAccepted value\nChecked\nCommit / cancel event\n—\nAccept pending value"
}
```

### U2 · dark-local-red-highcontrast

- 图：[U2-dark-local-red-highcontrast-1440-dark.png](U2-dark-local-red-highcontrast-1440-dark.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:44:48.017Z；1440×1000，DPR 1，dark。
- 实际操作：深色、局部红色、high contrast
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "BUTTON",
    "id": "mode-dark",
    "role": null,
    "text": "Dark"
  },
  "visibleText": "Component\nCheckbox\nSwitch\nToggle\nRadioGroup\nCheckboxGroup\nToggleGroup\nRadioCards\nCheckboxCards\nSlider\nRangeSlider\nTextField\nTextArea\nNumberField\nColorField\nSelect\nPresentation\nThemes\nPrimitives\nValue requests\nAccept\nRefuse\nAccept later\nSize\n1\n2\n3\nVariant\nsurface\nclassic\nsoft\nLocal colour\nInherit\nBlue\nRed\nGray\nState\nNormal\nDisabled\nRead only\nInvalid\nRTL\nHigh contrast\nSet indeterminate\n\nInteract with the example. Requests are separate from accepted values. Number and colour fields keep a draft until input completes.\n\nRequest count\n2\nLast request\nChecked\nAccepted value\nChecked\nCommit / cancel event\n—\nAccept pending value"
}
```

### U2 · mobile-cards-control-a2

- 图：[U2-mobile-cards-control-a2-390-dark.png](U2-mobile-cards-control-a2-390-dark.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:47:35.790Z；390×844，DPR 1，dark。
- 实际操作：Supplement: mobile cards and request/accepted outputs
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "BUTTON",
    "id": "mobile-preview",
    "role": null,
    "text": "Preview"
  }
}
```

