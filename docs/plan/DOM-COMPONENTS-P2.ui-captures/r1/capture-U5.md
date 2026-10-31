# DOM-COMPONENTS-P2 R1 · 实拍采集

- HEAD: 64bb17281567a646b312a4b7050219fe5a7f0c55；main；dirty fingerprint: 19479199a276dd54c403765cc1f9af125254e0e60c038875cda3942d39364c1c。产品源码无未提交差异，原有未提交内容是仓库指引、历史评审与状态台账。
- 目标指纹：phoenix-ui-target-v1/manual-u64be:43a74974da555b476c3c8ce9a27112cba19b79d6aa75a96a5a156a6d6f402c29；目标文件 SHA-256 3703966418fcf4a31531799167d5b4874a4dba8a076a15e339456b91db881c89。
- 指纹规则：UTF-8 LF; heading sections 单元 then 规范要点; each includes heading and one trailing LF; domain phoenix-ui-target-v1 NUL; u64be byte length + bytes; no linked local attachments in these sections. Manual implementation, not engine-verified. 使用固定人工编码；无 engine 比对，不与未知算法跨轮比较。
- 工具：Playwright 1.63.0 + 本机 Google Chrome；headless，DPR1，locale en-US、Asia/Shanghai；截图等待字体、animations disabled。系统原生 validationMessage 实际为中文；未替换验证文案。
- 工作树当前构建：Studio build_id dc7fb113f1267f33；raw/consumer 本轮 build-dom；catalog/workbench 本轮 release。均由仓内 mbx 命令生成，浏览器通过 HTTP 加载；未拦截网络、未注入替代 UI。
- 状态通过实际选择/点击/按键/鼠标操作到达；为取景仅移动真实滚动位置。无截图拼接/修图，未使用旧图。
- 首轮没有上一轮图片/评分。未执行 OS IME、VoiceOver、真机触控或浏览器 UI 200% 缩放；按计划不归入本期必需格子。

## 范围 U5

本单元主证据如下；其它单元本评审不评分。

### U5 · trigger-surface

- 图：[U5-trigger-surface-1440-light.png](U5-trigger-surface-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:45:22.123Z；1440×1000，DPR 1，light。
- 实际操作：Trigger surface，size=1
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "SUMMARY",
    "id": null,
    "role": null,
    "text": "States and parts"
  },
  "visibleText": "Component\nCheckbox\nSwitch\nToggle\nRadioGroup\nCheckboxGroup\nToggleGroup\nRadioCards\nCheckboxCards\nSlider\nRangeSlider\nTextField\nTextArea\nNumberField\nColorField\nSelect\nPresentation\nThemes\nPrimitives\nValue requests\nAccept\nRefuse\nAccept later\nSize\n1\n2\n3\nVariant\nsurface\nclassic\nsoft\nghost-offset\nghost\nLocal colour\nInherit\nBlue\nRed\nGray\nState\nNormal\nDisabled\nRead only\nInvalid\nRadius\nInherit\n0\nFull\nList variant\nsolid\nsoft\nRTL\nHigh contrast\nCancel selection\n\nInteract with the example. Requests are separate from accepted values. Number and colour fields keep a draft until input completes.\n\nEmail\nEmail\nActivity feed\nUnavailable\nRequest count\n0\nLast request\n—\nAccepted value\n\"a\"\nCommit / cancel event\n—\nAccept pending value"
}
```

### U5 · trigger-classic

- 图：[U5-trigger-classic-1440-light.png](U5-trigger-classic-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:45:22.455Z；1440×1000，DPR 1，light。
- 实际操作：Trigger classic，size=2
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "SUMMARY",
    "id": null,
    "role": null,
    "text": "States and parts"
  },
  "visibleText": "Component\nCheckbox\nSwitch\nToggle\nRadioGroup\nCheckboxGroup\nToggleGroup\nRadioCards\nCheckboxCards\nSlider\nRangeSlider\nTextField\nTextArea\nNumberField\nColorField\nSelect\nPresentation\nThemes\nPrimitives\nValue requests\nAccept\nRefuse\nAccept later\nSize\n1\n2\n3\nVariant\nsurface\nclassic\nsoft\nghost-offset\nghost\nLocal colour\nInherit\nBlue\nRed\nGray\nState\nNormal\nDisabled\nRead only\nInvalid\nRadius\nInherit\n0\nFull\nList variant\nsolid\nsoft\nRTL\nHigh contrast\nCancel selection\n\nInteract with the example. Requests are separate from accepted values. Number and colour fields keep a draft until input completes.\n\nEmail\nEmail\nActivity feed\nUnavailable\nRequest count\n0\nLast request\n—\nAccepted value\n\"a\"\nCommit / cancel event\n—\nAccept pending value"
}
```

### U5 · trigger-soft

- 图：[U5-trigger-soft-1440-light.png](U5-trigger-soft-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:45:22.787Z；1440×1000，DPR 1，light。
- 实际操作：Trigger soft，size=3
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "SUMMARY",
    "id": null,
    "role": null,
    "text": "States and parts"
  },
  "visibleText": "Component\nCheckbox\nSwitch\nToggle\nRadioGroup\nCheckboxGroup\nToggleGroup\nRadioCards\nCheckboxCards\nSlider\nRangeSlider\nTextField\nTextArea\nNumberField\nColorField\nSelect\nPresentation\nThemes\nPrimitives\nValue requests\nAccept\nRefuse\nAccept later\nSize\n1\n2\n3\nVariant\nsurface\nclassic\nsoft\nghost-offset\nghost\nLocal colour\nInherit\nBlue\nRed\nGray\nState\nNormal\nDisabled\nRead only\nInvalid\nRadius\nInherit\n0\nFull\nList variant\nsolid\nsoft\nRTL\nHigh contrast\nCancel selection\n\nInteract with the example. Requests are separate from accepted values. Number and colour fields keep a draft until input completes.\n\nEmail\nEmail\nActivity feed\nUnavailable\nRequest count\n0\nLast request\n—\nAccepted value\n\"a\"\nCommit / cancel event\n—\nAccept pending value"
}
```

### U5 · trigger-ghost-offset

- 图：[U5-trigger-ghost-offset-1440-light.png](U5-trigger-ghost-offset-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:45:23.098Z；1440×1000，DPR 1，light。
- 实际操作：Trigger ghost-offset，size=1
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "SUMMARY",
    "id": null,
    "role": null,
    "text": "States and parts"
  },
  "visibleText": "Component\nCheckbox\nSwitch\nToggle\nRadioGroup\nCheckboxGroup\nToggleGroup\nRadioCards\nCheckboxCards\nSlider\nRangeSlider\nTextField\nTextArea\nNumberField\nColorField\nSelect\nPresentation\nThemes\nPrimitives\nValue requests\nAccept\nRefuse\nAccept later\nSize\n1\n2\n3\nVariant\nsurface\nclassic\nsoft\nghost-offset\nghost\nLocal colour\nInherit\nBlue\nRed\nGray\nState\nNormal\nDisabled\nRead only\nInvalid\nRadius\nInherit\n0\nFull\nList variant\nsolid\nsoft\nRTL\nHigh contrast\nCancel selection\n\nInteract with the example. Requests are separate from accepted values. Number and colour fields keep a draft until input completes.\n\nEmail\nEmail\nActivity feed\nUnavailable\nRequest count\n0\nLast request\n—\nAccepted value\n\"a\"\nCommit / cancel event\n—\nAccept pending value"
}
```

### U5 · trigger-ghost

- 图：[U5-trigger-ghost-1440-light.png](U5-trigger-ghost-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:45:23.405Z；1440×1000，DPR 1，light。
- 实际操作：Trigger ghost，size=2
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "SUMMARY",
    "id": null,
    "role": null,
    "text": "States and parts"
  },
  "visibleText": "Component\nCheckbox\nSwitch\nToggle\nRadioGroup\nCheckboxGroup\nToggleGroup\nRadioCards\nCheckboxCards\nSlider\nRangeSlider\nTextField\nTextArea\nNumberField\nColorField\nSelect\nPresentation\nThemes\nPrimitives\nValue requests\nAccept\nRefuse\nAccept later\nSize\n1\n2\n3\nVariant\nsurface\nclassic\nsoft\nghost-offset\nghost\nLocal colour\nInherit\nBlue\nRed\nGray\nState\nNormal\nDisabled\nRead only\nInvalid\nRadius\nInherit\n0\nFull\nList variant\nsolid\nsoft\nRTL\nHigh contrast\nCancel selection\n\nInteract with the example. Requests are separate from accepted values. Number and colour fields keep a draft until input completes.\n\nEmail\nEmail\nActivity feed\nUnavailable\nRequest count\n0\nLast request\n—\nAccepted value\n\"a\"\nCommit / cancel event\n—\nAccept pending value"
}
```

### U5 · open-solid

- 图：[U5-open-solid-1440-light.png](U5-open-solid-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:45:23.790Z；1440×1000，DPR 1，light。
- 实际操作：ArrowDown打开Select，保留Trigger焦点
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "BUTTON",
    "id": "parts-control",
    "role": "combobox",
    "text": "Email"
  },
  "visibleText": "Email\nActivity feed\nUnavailable"
}
```

### U5 · keyboard-selected

- 图：[U5-keyboard-selected-1440-light.png](U5-keyboard-selected-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:45:24.046Z；1440×1000，DPR 1，light。
- 实际操作：键盘字符a定位并Enter选择；记录关闭后焦点
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "BUTTON",
    "id": "parts-control",
    "role": "combobox",
    "text": "Activity feed"
  },
  "visibleText": "Component\nCheckbox\nSwitch\nToggle\nRadioGroup\nCheckboxGroup\nToggleGroup\nRadioCards\nCheckboxCards\nSlider\nRangeSlider\nTextField\nTextArea\nNumberField\nColorField\nSelect\nPresentation\nThemes\nPrimitives\nValue requests\nAccept\nRefuse\nAccept later\nSize\n1\n2\n3\nVariant\nsurface\nclassic\nsoft\nghost-offset\nghost\nLocal colour\nInherit\nBlue\nRed\nGray\nState\nNormal\nDisabled\nRead only\nInvalid\nRadius\nInherit\n0\nFull\nList variant\nsolid\nsoft\nRTL\nHigh contrast\nCancel selection\n\nInteract with the example. Requests are separate from accepted values. Number and colour fields keep a draft until input completes.\n\nActivity feed\nEmail\nActivity feed\nUnavailable\nRequest count\n1\nLast request\n\"b\"\nAccepted value\n\"b\"\nCommit / cancel event\nselect:b\nAccept pending value"
}
```

### U5 · cancel-selection

- 图：[U5-cancel-selection-1440-light.png](U5-cancel-selection-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:45:24.382Z；1440×1000，DPR 1，light。
- 实际操作：取消选择已开启，点击Activity feed
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "BUTTON",
    "id": "parts-control",
    "role": "combobox",
    "text": "Activity feed"
  },
  "visibleText": "Component\nCheckbox\nSwitch\nToggle\nRadioGroup\nCheckboxGroup\nToggleGroup\nRadioCards\nCheckboxCards\nSlider\nRangeSlider\nTextField\nTextArea\nNumberField\nColorField\nSelect\nPresentation\nThemes\nPrimitives\nValue requests\nAccept\nRefuse\nAccept later\nSize\n1\n2\n3\nVariant\nsurface\nclassic\nsoft\nghost-offset\nghost\nLocal colour\nInherit\nBlue\nRed\nGray\nState\nNormal\nDisabled\nRead only\nInvalid\nRadius\nInherit\n0\nFull\nList variant\nsolid\nsoft\nRTL\nHigh contrast\nCancel selection\n\nInteract with the example. Requests are separate from accepted values. Number and colour fields keep a draft until input completes.\n\nActivity feed\nEmail\nActivity feed\nUnavailable\nRequest count\n1\nLast request\n\"b\"\nAccepted value\n\"b\"\nCommit / cancel event\nselect:b\nAccept pending value"
}
```

### U5 · content-soft

- 图：[U5-content-soft-1440-light.png](U5-content-soft-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:45:24.728Z；1440×1000，DPR 1，light。
- 实际操作：Content改soft后打开
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "BUTTON",
    "id": "parts-control",
    "role": "combobox",
    "text": "Activity feed"
  }
}
```

### U5 · raw-empty

- 图：[U5-raw-empty-1440-light.png](U5-raw-empty-1440-light.png)
- 出处：http://127.0.0.1:4190/；2026-10-31T06:46:46.305Z；1440×1000，DPR 1，light。
- 实际操作：Empty entries then open
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "BUTTON",
    "id": "raw-select-trigger",
    "role": "combobox",
    "text": "Choose fruitOpen"
  },
  "accepted": "b"
}
```

### U5 · raw-long-end

- 图：[U5-raw-long-end-1440-light.png](U5-raw-long-end-1440-light.png)
- 出处：http://127.0.0.1:4190/；2026-10-31T06:46:46.592Z；1440×1000，DPR 1，light。
- 实际操作：150 entries; End to last item
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "BUTTON",
    "id": "raw-select-trigger",
    "role": "combobox",
    "text": "Choose fruitOpen"
  },
  "active": "rui-select-item-211"
}
```

### U5 · mobile-open-control-a2

- 图：[U5-mobile-open-control-a2-390-dark.png](U5-mobile-open-control-a2-390-dark.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:47:36.290Z；390×844，DPR 1，dark。
- 实际操作：Supplement: actual mobile RTL popup; no scroll after opening
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "BUTTON",
    "id": "parts-control",
    "role": "combobox",
    "text": "Email"
  }
}
```

