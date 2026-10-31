# DOM-COMPONENTS-P2 R1 · 实拍采集

- HEAD: 64bb17281567a646b312a4b7050219fe5a7f0c55；main；dirty fingerprint: 19479199a276dd54c403765cc1f9af125254e0e60c038875cda3942d39364c1c。产品源码无未提交差异，原有未提交内容是仓库指引、历史评审与状态台账。
- 目标指纹：phoenix-ui-target-v1/manual-u64be:43a74974da555b476c3c8ce9a27112cba19b79d6aa75a96a5a156a6d6f402c29；目标文件 SHA-256 3703966418fcf4a31531799167d5b4874a4dba8a076a15e339456b91db881c89。
- 指纹规则：UTF-8 LF; heading sections 单元 then 规范要点; each includes heading and one trailing LF; domain phoenix-ui-target-v1 NUL; u64be byte length + bytes; no linked local attachments in these sections. Manual implementation, not engine-verified. 使用固定人工编码；无 engine 比对，不与未知算法跨轮比较。
- 工具：Playwright 1.63.0 + 本机 Google Chrome；headless，DPR1，locale en-US、Asia/Shanghai；截图等待字体、animations disabled。系统原生 validationMessage 实际为中文；未替换验证文案。
- 工作树当前构建：Studio build_id dc7fb113f1267f33；raw/consumer 本轮 build-dom；catalog/workbench 本轮 release。均由仓内 mbx 命令生成，浏览器通过 HTTP 加载；未拦截网络、未注入替代 UI。
- 状态通过实际选择/点击/按键/鼠标操作到达；为取景仅移动真实滚动位置。无截图拼接/修图，未使用旧图。
- 首轮没有上一轮图片/评分。未执行 OS IME、VoiceOver、真机触控或浏览器 UI 200% 缩放；按计划不归入本期必需格子。

## 范围 U1

本单元主证据如下；其它单元本评审不评分。

### U1 · saved-data

- 图：[U1-saved-data-1440-light.png](U1-saved-data-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:42:51.336Z；1440×1000，DPR 1，light。
- 实际操作：完成名称、选择与 Draft 后 Enter 保存，展开原生 FormData
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "SUMMARY",
    "id": null,
    "role": null,
    "text": "Submitted form data"
  },
  "status": "Saved in this preview only · 1",
  "data": "[\n  [\n    \"name\",\n    \"Avery\"\n  ],\n  [\n    \"description\",\n    \"\"\n  ],\n  [\n    \"topics\",\n    \"updates\"\n  ],\n  [\n    \"topics\",\n    \"guides\"\n  ],\n  [\n    \"cadence\",\n    \"weekly\"\n  ],\n  [\n    \"plan\",\n    \"team\"\n  ],\n  [\n    \"channels\",\n    \"mail\"\n  ],\n  [\n    \"channels\",\n    \"feed\"\n  ],\n  [\n    \"strength\",\n    \"50\"\n  ],\n  [\n    \"window\",\n    \"20\"\n  ],\n  [\n    \"window\",\n    \"80\"\n  ],\n  [\n    \"amount\",\n    \"17.5\"\n  ],\n  [\n    \"color\",\n    \"#336699\"\n  ],\n  [\n    \"region\",\n    \"east\"\n  ],\n  [\n    \"consent\",\n    \"on\"\n  ]\n]"
}
```

### U1 · default-heading-a2

- 图：[U1-default-heading-a2-1440-light.png](U1-default-heading-a2-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:43:51.557Z；1440×1000，DPR 1，light。
- 实际操作：补采：将表单标题放在固定顶部栏下方
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "BUTTON",
    "id": "mode-light",
    "role": null,
    "text": "Light"
  }
}
```

### U1 · default-heading-a2

- 图：[U1-default-heading-a2-1440-dark.png](U1-default-heading-a2-1440-dark.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:43:51.887Z；1440×1000，DPR 1，dark。
- 实际操作：补采：将表单标题放在固定顶部栏下方
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "BUTTON",
    "id": "mode-dark",
    "role": null,
    "text": "Dark"
  }
}
```

### U1 · default-heading-a2

- 图：[U1-default-heading-a2-768-light.png](U1-default-heading-a2-768-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:43:52.309Z；768×1000，DPR 1，light。
- 实际操作：补采：将表单标题放在固定顶部栏下方
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "BUTTON",
    "id": "mode-light",
    "role": null,
    "text": "Light"
  }
}
```

### U1 · default-heading-a2

- 图：[U1-default-heading-a2-768-dark.png](U1-default-heading-a2-768-dark.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:43:52.713Z；768×1000，DPR 1，dark。
- 实际操作：补采：将表单标题放在固定顶部栏下方
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "BUTTON",
    "id": "mode-dark",
    "role": null,
    "text": "Dark"
  }
}
```

### U1 · default-heading-a2

- 图：[U1-default-heading-a2-390-light.png](U1-default-heading-a2-390-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:43:53.166Z；390×844，DPR 1，light。
- 实际操作：补采：将表单标题放在固定顶部栏下方
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "BUTTON",
    "id": "mode-light",
    "role": null,
    "text": "Light"
  }
}
```

### U1 · default-heading-a2

- 图：[U1-default-heading-a2-390-dark.png](U1-default-heading-a2-390-dark.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:43:53.531Z；390×844，DPR 1，dark。
- 实际操作：补采：将表单标题放在固定顶部栏下方
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "BUTTON",
    "id": "mode-dark",
    "role": null,
    "text": "Dark"
  }
}
```

### U1 · zh-long

- 图：[U1-zh-long-390-dark.png](U1-zh-long-390-dark.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:43:54.081Z；390×844，DPR 1，dark。
- 实际操作：桌面切中文后切390px深色；长作者输入保留
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "BUTTON",
    "id": "mode-dark",
    "role": null,
    "text": "深色"
  }
}
```

### U1 · default-middle-a3

- 图：[U1-default-middle-a3-768-light.png](U1-default-middle-a3-768-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:46:39.068Z；768×1000，DPR 1，light。
- 实际操作：Supplement: middle controls
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "BUTTON",
    "id": "mode-light",
    "role": null,
    "text": "Light"
  }
}
```

### U1 · default-actions-a3

- 图：[U1-default-actions-a3-768-light.png](U1-default-actions-a3-768-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:46:39.299Z；768×1000，DPR 1，light。
- 实际操作：Supplement: fields, save and status
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "BUTTON",
    "id": "mode-light",
    "role": null,
    "text": "Light"
  }
}
```

### U1 · default-middle-a3

- 图：[U1-default-middle-a3-768-dark.png](U1-default-middle-a3-768-dark.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:46:39.678Z；768×1000，DPR 1，dark。
- 实际操作：Supplement: middle controls
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "BUTTON",
    "id": "mode-dark",
    "role": null,
    "text": "Dark"
  }
}
```

### U1 · default-actions-a3

- 图：[U1-default-actions-a3-768-dark.png](U1-default-actions-a3-768-dark.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:46:39.924Z；768×1000，DPR 1，dark。
- 实际操作：Supplement: fields, save and status
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "BUTTON",
    "id": "mode-dark",
    "role": null,
    "text": "Dark"
  }
}
```

### U1 · default-middle-a3

- 图：[U1-default-middle-a3-390-light.png](U1-default-middle-a3-390-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:46:40.369Z；390×844，DPR 1，light。
- 实际操作：Supplement: middle controls
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "BUTTON",
    "id": "mode-light",
    "role": null,
    "text": "Light"
  }
}
```

### U1 · default-actions-a3

- 图：[U1-default-actions-a3-390-light.png](U1-default-actions-a3-390-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:46:40.587Z；390×844，DPR 1，light。
- 实际操作：Supplement: fields, save and status
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "BUTTON",
    "id": "mode-light",
    "role": null,
    "text": "Light"
  }
}
```

### U1 · default-middle-a3

- 图：[U1-default-middle-a3-390-dark.png](U1-default-middle-a3-390-dark.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:46:40.933Z；390×844，DPR 1，dark。
- 实际操作：Supplement: middle controls
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "BUTTON",
    "id": "mode-dark",
    "role": null,
    "text": "Dark"
  }
}
```

### U1 · default-actions-a3

- 图：[U1-default-actions-a3-390-dark.png](U1-default-actions-a3-390-dark.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:46:41.151Z；390×844，DPR 1，dark。
- 实际操作：Supplement: fields, save and status
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "BUTTON",
    "id": "mode-dark",
    "role": null,
    "text": "Dark"
  }
}
```

### U1 · constraint-focus-a3

- 图：[U1-constraint-focus-a3-1440-light.png](U1-constraint-focus-a3-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:46:41.593Z；1440×1000，DPR 1，light。
- 实际操作：Empty form Enter: error and focus
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "INPUT",
    "id": "sample-name",
    "role": null,
    "text": ""
  },
  "error": "请填写此字段。"
}
```

### U1 · rule-focus-a3

- 图：[U1-rule-focus-a3-1440-light.png](U1-rule-focus-a3-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:46:41.856Z；1440×1000，DPR 1，light。
- 实际操作：A then Enter: rule error and focus
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "INPUT",
    "id": "sample-name",
    "role": null,
    "text": ""
  },
  "error": "Use at least two characters."
}
```

### U1 · saved-status-a3

- 图：[U1-saved-status-a3-1440-light.png](U1-saved-status-a3-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:46:42.371Z；1440×1000，DPR 1，light。
- 实际操作：Local save status
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "BUTTON",
    "id": "sample-save",
    "role": null,
    "text": "Save locally"
  },
  "status": "Saving locally…"
}
```

### U1 · application-error-a3

- 图：[U1-application-error-a3-1440-light.png](U1-application-error-a3-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:46:42.674Z；1440×1000，DPR 1，light。
- 实际操作：Application error and field focus
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "INPUT",
    "id": "sample-name",
    "role": null,
    "text": ""
  },
  "error": "This demo name is unavailable. Edit it and try again."
}
```

### U1 · retry-status-a3

- 图：[U1-retry-status-a3-1440-light.png](U1-retry-status-a3-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:46:42.954Z；1440×1000，DPR 1，light。
- 实际操作：Retry after changing name
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "BUTTON",
    "id": "sample-save",
    "role": null,
    "text": "Save locally"
  },
  "status": "Saving locally…"
}
```

### U1 · reset-focus-a3

- 图：[U1-reset-focus-a3-1440-light.png](U1-reset-focus-a3-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:46:43.238Z；1440×1000，DPR 1，light。
- 实际操作：Reset then Tab and Shift+Tab to name
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "INPUT",
    "id": "sample-name",
    "role": null,
    "text": ""
  }
}
```

