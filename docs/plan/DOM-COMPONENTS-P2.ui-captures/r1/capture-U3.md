# DOM-COMPONENTS-P2 R1 · 实拍采集

- HEAD: 64bb17281567a646b312a4b7050219fe5a7f0c55；main；dirty fingerprint: 19479199a276dd54c403765cc1f9af125254e0e60c038875cda3942d39364c1c。产品源码无未提交差异，原有未提交内容是仓库指引、历史评审与状态台账。
- 目标指纹：phoenix-ui-target-v1/manual-u64be:43a74974da555b476c3c8ce9a27112cba19b79d6aa75a96a5a156a6d6f402c29；目标文件 SHA-256 3703966418fcf4a31531799167d5b4874a4dba8a076a15e339456b91db881c89。
- 指纹规则：UTF-8 LF; heading sections 单元 then 规范要点; each includes heading and one trailing LF; domain phoenix-ui-target-v1 NUL; u64be byte length + bytes; no linked local attachments in these sections. Manual implementation, not engine-verified. 使用固定人工编码；无 engine 比对，不与未知算法跨轮比较。
- 工具：Playwright 1.63.0 + 本机 Google Chrome；headless，DPR1，locale en-US、Asia/Shanghai；截图等待字体、animations disabled。系统原生 validationMessage 实际为中文；未替换验证文案。
- 工作树当前构建：Studio build_id dc7fb113f1267f33；raw/consumer 本轮 build-dom；catalog/workbench 本轮 release。均由仓内 mbx 命令生成，浏览器通过 HTTP 加载；未拦截网络、未注入替代 UI。
- 状态通过实际选择/点击/按键/鼠标操作到达；为取景仅移动真实滚动位置。无截图拼接/修图，未使用旧图。
- 首轮没有上一轮图片/评分。未执行 OS IME、VoiceOver、真机触控或浏览器 UI 200% 缩放；按计划不归入本期必需格子。

## 范围 U3

本单元主证据如下；其它单元本评审不评分。

### U3 · TextField-s1-classic

- 图：[U3-TextField-s1-classic-1440-light.png](U3-TextField-s1-classic-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:44:40.063Z；1440×1000，DPR 1，light。
- 实际操作：组件 TextField，size=1，variant=classic
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "SUMMARY",
    "id": null,
    "role": null,
    "text": "States and parts"
  },
  "visibleText": "Component\nCheckbox\nSwitch\nToggle\nRadioGroup\nCheckboxGroup\nToggleGroup\nRadioCards\nCheckboxCards\nSlider\nRangeSlider\nTextField\nTextArea\nNumberField\nColorField\nSelect\nPresentation\nThemes\nPrimitives\nValue requests\nAccept\nRefuse\nAccept later\nSize\n1\n2\n3\nVariant\nsurface\nclassic\nsoft\nLocal colour\nInherit\nBlue\nRed\nGray\nState\nNormal\nDisabled\nRead only\nInvalid\nRadius\nInherit\n0\nFull\nRTL\n\nInteract with the example. Requests are separate from accepted values. Number and colour fields keep a draft until input completes.\n\nRequest count\n0\nLast request\n—\nAccepted value\n\"Example\"\nCommit / cancel event\n—\nAccept pending value"
}
```

### U3 · TextField-s2-surface

- 图：[U3-TextField-s2-surface-1440-light.png](U3-TextField-s2-surface-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:44:40.314Z；1440×1000，DPR 1，light。
- 实际操作：组件 TextField，size=2，variant=surface
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "SUMMARY",
    "id": null,
    "role": null,
    "text": "States and parts"
  },
  "visibleText": "Component\nCheckbox\nSwitch\nToggle\nRadioGroup\nCheckboxGroup\nToggleGroup\nRadioCards\nCheckboxCards\nSlider\nRangeSlider\nTextField\nTextArea\nNumberField\nColorField\nSelect\nPresentation\nThemes\nPrimitives\nValue requests\nAccept\nRefuse\nAccept later\nSize\n1\n2\n3\nVariant\nsurface\nclassic\nsoft\nLocal colour\nInherit\nBlue\nRed\nGray\nState\nNormal\nDisabled\nRead only\nInvalid\nRadius\nInherit\n0\nFull\nRTL\n\nInteract with the example. Requests are separate from accepted values. Number and colour fields keep a draft until input completes.\n\nRequest count\n0\nLast request\n—\nAccepted value\n\"Example\"\nCommit / cancel event\n—\nAccept pending value"
}
```

### U3 · TextField-s3-soft

- 图：[U3-TextField-s3-soft-1440-light.png](U3-TextField-s3-soft-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:44:40.582Z；1440×1000，DPR 1，light。
- 实际操作：组件 TextField，size=3，variant=soft
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "SUMMARY",
    "id": null,
    "role": null,
    "text": "States and parts"
  },
  "visibleText": "Component\nCheckbox\nSwitch\nToggle\nRadioGroup\nCheckboxGroup\nToggleGroup\nRadioCards\nCheckboxCards\nSlider\nRangeSlider\nTextField\nTextArea\nNumberField\nColorField\nSelect\nPresentation\nThemes\nPrimitives\nValue requests\nAccept\nRefuse\nAccept later\nSize\n1\n2\n3\nVariant\nsurface\nclassic\nsoft\nLocal colour\nInherit\nBlue\nRed\nGray\nState\nNormal\nDisabled\nRead only\nInvalid\nRadius\nInherit\n0\nFull\nRTL\n\nInteract with the example. Requests are separate from accepted values. Number and colour fields keep a draft until input completes.\n\nRequest count\n0\nLast request\n—\nAccepted value\n\"Example\"\nCommit / cancel event\n—\nAccept pending value"
}
```

### U3 · TextArea-s1-classic

- 图：[U3-TextArea-s1-classic-1440-light.png](U3-TextArea-s1-classic-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:44:40.997Z；1440×1000，DPR 1，light。
- 实际操作：组件 TextArea，size=1，variant=classic
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "SUMMARY",
    "id": null,
    "role": null,
    "text": "States and parts"
  },
  "visibleText": "Component\nCheckbox\nSwitch\nToggle\nRadioGroup\nCheckboxGroup\nToggleGroup\nRadioCards\nCheckboxCards\nSlider\nRangeSlider\nTextField\nTextArea\nNumberField\nColorField\nSelect\nPresentation\nThemes\nPrimitives\nValue requests\nAccept\nRefuse\nAccept later\nSize\n1\n2\n3\nVariant\nsurface\nclassic\nsoft\nLocal colour\nInherit\nBlue\nRed\nGray\nState\nNormal\nDisabled\nRead only\nInvalid\nRadius\nInherit\n0\nFull\nRTL\n\nInteract with the example. Requests are separate from accepted values. Number and colour fields keep a draft until input completes.\n\nRequest count\n0\nLast request\n—\nAccepted value\n\"Example\"\nCommit / cancel event\n—\nAccept pending value"
}
```

### U3 · TextArea-s2-surface

- 图：[U3-TextArea-s2-surface-1440-light.png](U3-TextArea-s2-surface-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:44:41.265Z；1440×1000，DPR 1，light。
- 实际操作：组件 TextArea，size=2，variant=surface
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "SUMMARY",
    "id": null,
    "role": null,
    "text": "States and parts"
  },
  "visibleText": "Component\nCheckbox\nSwitch\nToggle\nRadioGroup\nCheckboxGroup\nToggleGroup\nRadioCards\nCheckboxCards\nSlider\nRangeSlider\nTextField\nTextArea\nNumberField\nColorField\nSelect\nPresentation\nThemes\nPrimitives\nValue requests\nAccept\nRefuse\nAccept later\nSize\n1\n2\n3\nVariant\nsurface\nclassic\nsoft\nLocal colour\nInherit\nBlue\nRed\nGray\nState\nNormal\nDisabled\nRead only\nInvalid\nRadius\nInherit\n0\nFull\nRTL\n\nInteract with the example. Requests are separate from accepted values. Number and colour fields keep a draft until input completes.\n\nRequest count\n0\nLast request\n—\nAccepted value\n\"Example\"\nCommit / cancel event\n—\nAccept pending value"
}
```

### U3 · TextArea-s3-soft

- 图：[U3-TextArea-s3-soft-1440-light.png](U3-TextArea-s3-soft-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:44:41.530Z；1440×1000，DPR 1，light。
- 实际操作：组件 TextArea，size=3，variant=soft
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "SUMMARY",
    "id": null,
    "role": null,
    "text": "States and parts"
  },
  "visibleText": "Component\nCheckbox\nSwitch\nToggle\nRadioGroup\nCheckboxGroup\nToggleGroup\nRadioCards\nCheckboxCards\nSlider\nRangeSlider\nTextField\nTextArea\nNumberField\nColorField\nSelect\nPresentation\nThemes\nPrimitives\nValue requests\nAccept\nRefuse\nAccept later\nSize\n1\n2\n3\nVariant\nsurface\nclassic\nsoft\nLocal colour\nInherit\nBlue\nRed\nGray\nState\nNormal\nDisabled\nRead only\nInvalid\nRadius\nInherit\n0\nFull\nRTL\n\nInteract with the example. Requests are separate from accepted values. Number and colour fields keep a draft until input completes.\n\nRequest count\n0\nLast request\n—\nAccepted value\n\"Example\"\nCommit / cancel event\n—\nAccept pending value"
}
```

### U3 · NumberField-s1-classic

- 图：[U3-NumberField-s1-classic-1440-light.png](U3-NumberField-s1-classic-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:44:41.899Z；1440×1000，DPR 1，light。
- 实际操作：组件 NumberField，size=1，variant=classic
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "SUMMARY",
    "id": null,
    "role": null,
    "text": "States and parts"
  },
  "visibleText": "Component\nCheckbox\nSwitch\nToggle\nRadioGroup\nCheckboxGroup\nToggleGroup\nRadioCards\nCheckboxCards\nSlider\nRangeSlider\nTextField\nTextArea\nNumberField\nColorField\nSelect\nPresentation\nThemes\nPrimitives\nValue requests\nAccept\nRefuse\nAccept later\nSize\n1\n2\n3\nVariant\nsurface\nclassic\nsoft\nLocal colour\nInherit\nBlue\nRed\nGray\nState\nNormal\nDisabled\nRead only\nInvalid\nRadius\nInherit\n0\nFull\nRTL\n\nInteract with the example. Requests are separate from accepted values. Number and colour fields keep a draft until input completes.\n\nRequest count\n0\nLast request\n—\nAccepted value\n12.5\nCommit / cancel event\n—\nAccept pending value"
}
```

### U3 · NumberField-s2-surface

- 图：[U3-NumberField-s2-surface-1440-light.png](U3-NumberField-s2-surface-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:44:42.185Z；1440×1000，DPR 1，light。
- 实际操作：组件 NumberField，size=2，variant=surface
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "SUMMARY",
    "id": null,
    "role": null,
    "text": "States and parts"
  },
  "visibleText": "Component\nCheckbox\nSwitch\nToggle\nRadioGroup\nCheckboxGroup\nToggleGroup\nRadioCards\nCheckboxCards\nSlider\nRangeSlider\nTextField\nTextArea\nNumberField\nColorField\nSelect\nPresentation\nThemes\nPrimitives\nValue requests\nAccept\nRefuse\nAccept later\nSize\n1\n2\n3\nVariant\nsurface\nclassic\nsoft\nLocal colour\nInherit\nBlue\nRed\nGray\nState\nNormal\nDisabled\nRead only\nInvalid\nRadius\nInherit\n0\nFull\nRTL\n\nInteract with the example. Requests are separate from accepted values. Number and colour fields keep a draft until input completes.\n\nRequest count\n0\nLast request\n—\nAccepted value\n12.5\nCommit / cancel event\n—\nAccept pending value"
}
```

### U3 · NumberField-s3-soft

- 图：[U3-NumberField-s3-soft-1440-light.png](U3-NumberField-s3-soft-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:44:42.466Z；1440×1000，DPR 1，light。
- 实际操作：组件 NumberField，size=3，variant=soft
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "SUMMARY",
    "id": null,
    "role": null,
    "text": "States and parts"
  },
  "visibleText": "Component\nCheckbox\nSwitch\nToggle\nRadioGroup\nCheckboxGroup\nToggleGroup\nRadioCards\nCheckboxCards\nSlider\nRangeSlider\nTextField\nTextArea\nNumberField\nColorField\nSelect\nPresentation\nThemes\nPrimitives\nValue requests\nAccept\nRefuse\nAccept later\nSize\n1\n2\n3\nVariant\nsurface\nclassic\nsoft\nLocal colour\nInherit\nBlue\nRed\nGray\nState\nNormal\nDisabled\nRead only\nInvalid\nRadius\nInherit\n0\nFull\nRTL\n\nInteract with the example. Requests are separate from accepted values. Number and colour fields keep a draft until input completes.\n\nRequest count\n0\nLast request\n—\nAccepted value\n12.5\nCommit / cancel event\n—\nAccept pending value"
}
```

### U3 · ColorField-s1-classic

- 图：[U3-ColorField-s1-classic-1440-light.png](U3-ColorField-s1-classic-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:44:42.831Z；1440×1000，DPR 1，light。
- 实际操作：组件 ColorField，size=1，variant=classic
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "SUMMARY",
    "id": null,
    "role": null,
    "text": "States and parts"
  },
  "visibleText": "Component\nCheckbox\nSwitch\nToggle\nRadioGroup\nCheckboxGroup\nToggleGroup\nRadioCards\nCheckboxCards\nSlider\nRangeSlider\nTextField\nTextArea\nNumberField\nColorField\nSelect\nPresentation\nThemes\nPrimitives\nValue requests\nAccept\nRefuse\nAccept later\nSize\n1\n2\n3\nVariant\nsurface\nclassic\nsoft\nLocal colour\nInherit\nBlue\nRed\nGray\nState\nNormal\nDisabled\nRead only\nInvalid\nRadius\nInherit\n0\nFull\nRTL\n\nInteract with the example. Requests are separate from accepted values. Number and colour fields keep a draft until input completes.\n\nRequest count\n0\nLast request\n—\nAccepted value\n\"#336699\"\nCommit / cancel event\n—\nAccept pending value"
}
```

### U3 · ColorField-s2-surface

- 图：[U3-ColorField-s2-surface-1440-light.png](U3-ColorField-s2-surface-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:44:43.123Z；1440×1000，DPR 1，light。
- 实际操作：组件 ColorField，size=2，variant=surface
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "SUMMARY",
    "id": null,
    "role": null,
    "text": "States and parts"
  },
  "visibleText": "Component\nCheckbox\nSwitch\nToggle\nRadioGroup\nCheckboxGroup\nToggleGroup\nRadioCards\nCheckboxCards\nSlider\nRangeSlider\nTextField\nTextArea\nNumberField\nColorField\nSelect\nPresentation\nThemes\nPrimitives\nValue requests\nAccept\nRefuse\nAccept later\nSize\n1\n2\n3\nVariant\nsurface\nclassic\nsoft\nLocal colour\nInherit\nBlue\nRed\nGray\nState\nNormal\nDisabled\nRead only\nInvalid\nRadius\nInherit\n0\nFull\nRTL\n\nInteract with the example. Requests are separate from accepted values. Number and colour fields keep a draft until input completes.\n\nRequest count\n0\nLast request\n—\nAccepted value\n\"#336699\"\nCommit / cancel event\n—\nAccept pending value"
}
```

### U3 · ColorField-s3-soft

- 图：[U3-ColorField-s3-soft-1440-light.png](U3-ColorField-s3-soft-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:44:43.416Z；1440×1000，DPR 1，light。
- 实际操作：组件 ColorField，size=3，variant=soft
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "SUMMARY",
    "id": null,
    "role": null,
    "text": "States and parts"
  },
  "visibleText": "Component\nCheckbox\nSwitch\nToggle\nRadioGroup\nCheckboxGroup\nToggleGroup\nRadioCards\nCheckboxCards\nSlider\nRangeSlider\nTextField\nTextArea\nNumberField\nColorField\nSelect\nPresentation\nThemes\nPrimitives\nValue requests\nAccept\nRefuse\nAccept later\nSize\n1\n2\n3\nVariant\nsurface\nclassic\nsoft\nLocal colour\nInherit\nBlue\nRed\nGray\nState\nNormal\nDisabled\nRead only\nInvalid\nRadius\nInherit\n0\nFull\nRTL\n\nInteract with the example. Requests are separate from accepted values. Number and colour fields keep a draft until input completes.\n\nRequest count\n0\nLast request\n—\nAccepted value\n\"#336699\"\nCommit / cancel event\n—\nAccept pending value"
}
```

### U3 · TextField-disabled

- 图：[U3-TextField-disabled-1440-light.png](U3-TextField-disabled-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:44:46.263Z；1440×1000，DPR 1，light。
- 实际操作：选择 TextField 的 disabled 状态
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "SUMMARY",
    "id": null,
    "role": null,
    "text": "States and parts"
  },
  "visibleText": "Component\nCheckbox\nSwitch\nToggle\nRadioGroup\nCheckboxGroup\nToggleGroup\nRadioCards\nCheckboxCards\nSlider\nRangeSlider\nTextField\nTextArea\nNumberField\nColorField\nSelect\nPresentation\nThemes\nPrimitives\nValue requests\nAccept\nRefuse\nAccept later\nSize\n1\n2\n3\nVariant\nsurface\nclassic\nsoft\nLocal colour\nInherit\nBlue\nRed\nGray\nState\nNormal\nDisabled\nRead only\nInvalid\nRadius\nInherit\n0\nFull\nRTL\n\nInteract with the example. Requests are separate from accepted values. Number and colour fields keep a draft until input completes.\n\nRequest count\n0\nLast request\n—\nAccepted value\n\"Example\"\nCommit / cancel event\n—\nAccept pending value"
}
```

### U3 · TextArea-readonly

- 图：[U3-TextArea-readonly-1440-light.png](U3-TextArea-readonly-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:44:46.534Z；1440×1000，DPR 1，light。
- 实际操作：选择 TextArea 的 readonly 状态
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "SUMMARY",
    "id": null,
    "role": null,
    "text": "States and parts"
  },
  "visibleText": "Component\nCheckbox\nSwitch\nToggle\nRadioGroup\nCheckboxGroup\nToggleGroup\nRadioCards\nCheckboxCards\nSlider\nRangeSlider\nTextField\nTextArea\nNumberField\nColorField\nSelect\nPresentation\nThemes\nPrimitives\nValue requests\nAccept\nRefuse\nAccept later\nSize\n1\n2\n3\nVariant\nsurface\nclassic\nsoft\nLocal colour\nInherit\nBlue\nRed\nGray\nState\nNormal\nDisabled\nRead only\nInvalid\nRadius\nInherit\n0\nFull\nRTL\n\nInteract with the example. Requests are separate from accepted values. Number and colour fields keep a draft until input completes.\n\nRequest count\n0\nLast request\n—\nAccepted value\n\"Example\"\nCommit / cancel event\n—\nAccept pending value"
}
```

### U3 · ColorField-invalid

- 图：[U3-ColorField-invalid-1440-light.png](U3-ColorField-invalid-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:44:46.800Z；1440×1000，DPR 1，light。
- 实际操作：选择 ColorField 的 invalid 状态
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "SUMMARY",
    "id": null,
    "role": null,
    "text": "States and parts"
  },
  "visibleText": "Component\nCheckbox\nSwitch\nToggle\nRadioGroup\nCheckboxGroup\nToggleGroup\nRadioCards\nCheckboxCards\nSlider\nRangeSlider\nTextField\nTextArea\nNumberField\nColorField\nSelect\nPresentation\nThemes\nPrimitives\nValue requests\nAccept\nRefuse\nAccept later\nSize\n1\n2\n3\nVariant\nsurface\nclassic\nsoft\nLocal colour\nInherit\nBlue\nRed\nGray\nState\nNormal\nDisabled\nRead only\nInvalid\nRadius\nInherit\n0\nFull\nRTL\n\nInteract with the example. Requests are separate from accepted values. Number and colour fields keep a draft until input completes.\n\nRequest count\n0\nLast request\n—\nAccepted value\n\"#336699\"\nCommit / cancel event\n—\nAccept pending value"
}
```

### U3 · TextField-keyboard-focus

- 图：[U3-TextField-keyboard-focus-1440-light.png](U3-TextField-keyboard-focus-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:44:48.823Z；1440×1000，DPR 1，light。
- 实际操作：Tab/Shift+Tab 到 TextField
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "INPUT",
    "id": "parts-control",
    "role": null,
    "text": ""
  },
  "visibleText": "Component\nCheckbox\nSwitch\nToggle\nRadioGroup\nCheckboxGroup\nToggleGroup\nRadioCards\nCheckboxCards\nSlider\nRangeSlider\nTextField\nTextArea\nNumberField\nColorField\nSelect\nPresentation\nThemes\nPrimitives\nValue requests\nAccept\nRefuse\nAccept later\nSize\n1\n2\n3\nVariant\nsurface\nclassic\nsoft\nLocal colour\nInherit\nBlue\nRed\nGray\nState\nNormal\nDisabled\nRead only\nInvalid\nRadius\nInherit\n0\nFull\nRTL\n\nInteract with the example. Requests are separate from accepted values. Number and colour fields keep a draft until input completes.\n\nRequest count\n0\nLast request\n—\nAccepted value\n\"Example\"\nCommit / cancel event\n—\nAccept pending value"
}
```

### U3 · NumberField-draft

- 图：[U3-NumberField-draft-1440-light.png](U3-NumberField-draft-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:44:49.100Z；1440×1000，DPR 1，light。
- 实际操作：聚焦编辑未完成 Draft：-
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "INPUT",
    "id": "parts-control",
    "role": "spinbutton",
    "text": ""
  },
  "visibleText": "Component\nCheckbox\nSwitch\nToggle\nRadioGroup\nCheckboxGroup\nToggleGroup\nRadioCards\nCheckboxCards\nSlider\nRangeSlider\nTextField\nTextArea\nNumberField\nColorField\nSelect\nPresentation\nThemes\nPrimitives\nValue requests\nAccept\nRefuse\nAccept later\nSize\n1\n2\n3\nVariant\nsurface\nclassic\nsoft\nLocal colour\nInherit\nBlue\nRed\nGray\nState\nNormal\nDisabled\nRead only\nInvalid\nRadius\nInherit\n0\nFull\nRTL\n\nInteract with the example. Requests are separate from accepted values. Number and colour fields keep a draft until input completes.\n\nRequest count\n0\nLast request\n—\nAccepted value\n12.5\nCommit / cancel event\n—\nAccept pending value"
}
```

### U3 · NumberField-cancel

- 图：[U3-NumberField-cancel-1440-light.png](U3-NumberField-cancel-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:44:49.332Z；1440×1000，DPR 1，light。
- 实际操作：Escape 取消 Draft
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "INPUT",
    "id": "parts-control",
    "role": "spinbutton",
    "text": ""
  },
  "visibleText": "Component\nCheckbox\nSwitch\nToggle\nRadioGroup\nCheckboxGroup\nToggleGroup\nRadioCards\nCheckboxCards\nSlider\nRangeSlider\nTextField\nTextArea\nNumberField\nColorField\nSelect\nPresentation\nThemes\nPrimitives\nValue requests\nAccept\nRefuse\nAccept later\nSize\n1\n2\n3\nVariant\nsurface\nclassic\nsoft\nLocal colour\nInherit\nBlue\nRed\nGray\nState\nNormal\nDisabled\nRead only\nInvalid\nRadius\nInherit\n0\nFull\nRTL\n\nInteract with the example. Requests are separate from accepted values. Number and colour fields keep a draft until input completes.\n\nRequest count\n0\nLast request\n—\nAccepted value\n12.5\nCommit / cancel event\n—\nAccept pending value"
}
```

### U3 · ColorField-draft

- 图：[U3-ColorField-draft-1440-light.png](U3-ColorField-draft-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:44:49.635Z；1440×1000，DPR 1，light。
- 实际操作：聚焦编辑未完成 Draft：#1f
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "INPUT",
    "id": "parts-control",
    "role": null,
    "text": ""
  },
  "visibleText": "Component\nCheckbox\nSwitch\nToggle\nRadioGroup\nCheckboxGroup\nToggleGroup\nRadioCards\nCheckboxCards\nSlider\nRangeSlider\nTextField\nTextArea\nNumberField\nColorField\nSelect\nPresentation\nThemes\nPrimitives\nValue requests\nAccept\nRefuse\nAccept later\nSize\n1\n2\n3\nVariant\nsurface\nclassic\nsoft\nLocal colour\nInherit\nBlue\nRed\nGray\nState\nNormal\nDisabled\nRead only\nInvalid\nRadius\nInherit\n0\nFull\nRTL\n\nInteract with the example. Requests are separate from accepted values. Number and colour fields keep a draft until input completes.\n\nRequest count\n0\nLast request\n—\nAccepted value\n\"#336699\"\nCommit / cancel event\n—\nAccept pending value"
}
```

### U3 · ColorField-cancel

- 图：[U3-ColorField-cancel-1440-light.png](U3-ColorField-cancel-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:44:49.868Z；1440×1000，DPR 1，light。
- 实际操作：Escape 取消 Draft
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "INPUT",
    "id": "parts-control",
    "role": null,
    "text": ""
  },
  "visibleText": "Component\nCheckbox\nSwitch\nToggle\nRadioGroup\nCheckboxGroup\nToggleGroup\nRadioCards\nCheckboxCards\nSlider\nRangeSlider\nTextField\nTextArea\nNumberField\nColorField\nSelect\nPresentation\nThemes\nPrimitives\nValue requests\nAccept\nRefuse\nAccept later\nSize\n1\n2\n3\nVariant\nsurface\nclassic\nsoft\nLocal colour\nInherit\nBlue\nRed\nGray\nState\nNormal\nDisabled\nRead only\nInvalid\nRadius\nInherit\n0\nFull\nRTL\n\nInteract with the example. Requests are separate from accepted values. Number and colour fields keep a draft until input completes.\n\nRequest count\n0\nLast request\n—\nAccepted value\n\"#336699\"\nCommit / cancel event\n—\nAccept pending value"
}
```

### U3 · dark-field

- 图：[U3-dark-field-1440-dark.png](U3-dark-field-1440-dark.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:44:50.257Z；1440×1000，DPR 1，dark。
- 实际操作：深色字段；沿用局部红色/high contrast 设置
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "BUTTON",
    "id": "mode-dark",
    "role": null,
    "text": "Dark"
  },
  "visibleText": "Component\nCheckbox\nSwitch\nToggle\nRadioGroup\nCheckboxGroup\nToggleGroup\nRadioCards\nCheckboxCards\nSlider\nRangeSlider\nTextField\nTextArea\nNumberField\nColorField\nSelect\nPresentation\nThemes\nPrimitives\nValue requests\nAccept\nRefuse\nAccept later\nSize\n1\n2\n3\nVariant\nsurface\nclassic\nsoft\nLocal colour\nInherit\nBlue\nRed\nGray\nState\nNormal\nDisabled\nRead only\nInvalid\nRadius\nInherit\n0\nFull\nRTL\n\nInteract with the example. Requests are separate from accepted values. Number and colour fields keep a draft until input completes.\n\nRequest count\n0\nLast request\n—\nAccepted value\n\"#336699\"\nCommit / cancel event\n—\nAccept pending value"
}
```

### U3 · consumer-fields-and-slots

- 图：[U3-consumer-fields-and-slots-1440-light.png](U3-consumer-fields-and-slots-1440-light.png)
- 出处：http://127.0.0.1:4190/fields.html；2026-10-31T06:48:02.696Z；1440×1000，DPR 1，light。
- 实际操作：Themed independent consumer: all field sizes/variants and composed slots
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "BODY",
    "id": null,
    "role": null,
    "text": "Native field samplesChange directionToggle disabledToggle read onlyComplete save"
  }
}
```

### U3 · consumer-slots-rtl

- 图：[U3-consumer-slots-rtl-1440-light.png](U3-consumer-slots-rtl-1440-light.png)
- 出处：http://127.0.0.1:4190/fields.html；2026-10-31T06:48:02.969Z；1440×1000，DPR 1，light。
- 实际操作：RTL composed slots and utility overrides
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "BUTTON",
    "id": "skin-direction",
    "role": null,
    "text": "Change direction"
  }
}
```

### U3 · consumer-slots-mobile

- 图：[U3-consumer-slots-mobile-390-light.png](U3-consumer-slots-mobile-390-light.png)
- 出处：http://127.0.0.1:4190/fields.html；2026-10-31T06:48:03.192Z；390×844，DPR 1，light。
- 实际操作：390px responsive slots
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "BUTTON",
    "id": "skin-direction",
    "role": null,
    "text": "Change direction"
  }
}
```

### U3 · consumer-slots-dark

- 图：[U3-consumer-slots-dark-1440-dark.png](U3-consumer-slots-dark-1440-dark.png)
- 出处：http://127.0.0.1:4190/fields.html；2026-10-31T06:48:03.485Z；1440×1000，DPR 1，dark。
- 实际操作：Dark themed fields and slots
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "BUTTON",
    "id": "skin-theme",
    "role": null,
    "text": "Change colour mode"
  }
}
```

