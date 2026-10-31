# DOM-COMPONENTS-P2 R1 · 实拍采集

- HEAD: 64bb17281567a646b312a4b7050219fe5a7f0c55；main；dirty fingerprint: 19479199a276dd54c403765cc1f9af125254e0e60c038875cda3942d39364c1c。产品源码无未提交差异，原有未提交内容是仓库指引、历史评审与状态台账。
- 目标指纹：phoenix-ui-target-v1/manual-u64be:43a74974da555b476c3c8ce9a27112cba19b79d6aa75a96a5a156a6d6f402c29；目标文件 SHA-256 3703966418fcf4a31531799167d5b4874a4dba8a076a15e339456b91db881c89。
- 指纹规则：UTF-8 LF; heading sections 单元 then 规范要点; each includes heading and one trailing LF; domain phoenix-ui-target-v1 NUL; u64be byte length + bytes; no linked local attachments in these sections. Manual implementation, not engine-verified. 使用固定人工编码；无 engine 比对，不与未知算法跨轮比较。
- 工具：Playwright 1.63.0 + 本机 Google Chrome；headless，DPR1，locale en-US、Asia/Shanghai；截图等待字体、animations disabled。系统原生 validationMessage 实际为中文；未替换验证文案。
- 工作树当前构建：Studio build_id dc7fb113f1267f33；raw/consumer 本轮 build-dom；catalog/workbench 本轮 release。均由仓内 mbx 命令生成，浏览器通过 HTTP 加载；未拦截网络、未注入替代 UI。
- 状态通过实际选择/点击/按键/鼠标操作到达；为取景仅移动真实滚动位置。无截图拼接/修图，未使用旧图。
- 首轮没有上一轮图片/评分。未执行 OS IME、VoiceOver、真机触控或浏览器 UI 200% 缩放；按计划不归入本期必需格子。

## 矩阵及补采

131 张原始实拍；112 张主证据。全部文件保留。首采 U1 顶部/底部与错误帧有部分目标区域在视口外，已追加 a2/a3 同状态补采；U2/U5 390px首图只显示配置区，已追加实际控件帧。首采图仍属真实截图，但不单独用于证明其画面之外状态。

## U1

主证据 22 张；原始图 39 张。

补采前原图，主证据见 a2/a3：

### U1 · default-top

- 图：[U1-default-top-1440-light.png](U1-default-top-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:42:46.665Z；1440×1000，DPR 1，light。
- 实际操作：进入 DOM → Cards；默认表单，滚动到标题
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

补采前原图，主证据见 a2/a3：

### U1 · default-bottom

- 图：[U1-default-bottom-1440-light.png](U1-default-bottom-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:42:46.883Z；1440×1000，DPR 1，light。
- 实际操作：滚动到保存/表单数据区域
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

补采前原图，主证据见 a2/a3：

### U1 · default-top

- 图：[U1-default-top-1440-dark.png](U1-default-top-1440-dark.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:42:47.240Z；1440×1000，DPR 1，dark。
- 实际操作：进入 DOM → Cards；默认表单，滚动到标题
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

补采前原图，主证据见 a2/a3：

### U1 · default-bottom

- 图：[U1-default-bottom-1440-dark.png](U1-default-bottom-1440-dark.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:42:47.450Z；1440×1000，DPR 1，dark。
- 实际操作：滚动到保存/表单数据区域
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

补采前原图，主证据见 a2/a3：

### U1 · default-top

- 图：[U1-default-top-768-light.png](U1-default-top-768-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:42:47.826Z；768×1000，DPR 1，light。
- 实际操作：进入 DOM → Cards；默认表单，滚动到标题
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

补采前原图，主证据见 a2/a3：

### U1 · default-bottom

- 图：[U1-default-bottom-768-light.png](U1-default-bottom-768-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:42:48.045Z；768×1000，DPR 1，light。
- 实际操作：滚动到保存/表单数据区域
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

补采前原图，主证据见 a2/a3：

### U1 · default-top

- 图：[U1-default-top-768-dark.png](U1-default-top-768-dark.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:42:48.442Z；768×1000，DPR 1，dark。
- 实际操作：进入 DOM → Cards；默认表单，滚动到标题
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

补采前原图，主证据见 a2/a3：

### U1 · default-bottom

- 图：[U1-default-bottom-768-dark.png](U1-default-bottom-768-dark.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:42:48.665Z；768×1000，DPR 1，dark。
- 实际操作：滚动到保存/表单数据区域
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

补采前原图，主证据见 a2/a3：

### U1 · default-top

- 图：[U1-default-top-390-light.png](U1-default-top-390-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:42:49.085Z；390×844，DPR 1，light。
- 实际操作：进入 DOM → Cards；默认表单，滚动到标题
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

补采前原图，主证据见 a2/a3：

### U1 · default-bottom

- 图：[U1-default-bottom-390-light.png](U1-default-bottom-390-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:42:49.301Z；390×844，DPR 1，light。
- 实际操作：滚动到保存/表单数据区域
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

补采前原图，主证据见 a2/a3：

### U1 · default-top

- 图：[U1-default-top-390-dark.png](U1-default-top-390-dark.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:42:49.668Z；390×844，DPR 1，dark。
- 实际操作：进入 DOM → Cards；默认表单，滚动到标题
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

补采前原图，主证据见 a2/a3：

### U1 · default-bottom

- 图：[U1-default-bottom-390-dark.png](U1-default-bottom-390-dark.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:42:49.884Z；390×844，DPR 1，dark。
- 实际操作：滚动到保存/表单数据区域
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

补采前原图，主证据见 a2/a3：

### U1 · constraint-focus

- 图：[U1-constraint-focus-1440-light.png](U1-constraint-focus-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:42:50.391Z；1440×1000，DPR 1，light。
- 实际操作：键盘 Enter 提交空表单；记录错误与实际焦点
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

补采前原图，主证据见 a2/a3：

### U1 · rule-focus

- 图：[U1-rule-focus-1440-light.png](U1-rule-focus-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:42:50.692Z；1440×1000，DPR 1，light。
- 实际操作：名称填 A 后 Enter 提交
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

补采前原图，主证据见 a2/a3：

### U1 · application-error

- 图：[U1-application-error-1440-light.png](U1-application-error-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:42:51.686Z；1440×1000，DPR 1，light。
- 实际操作：启用 Fail next save 后再次键盘提交
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

补采前原图，主证据见 a2/a3：

### U1 · retry-success

- 图：[U1-retry-success-1440-light.png](U1-retry-success-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:42:51.936Z；1440×1000，DPR 1，light。
- 实际操作：修改名称后再次 Enter 提交
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

补采前原图，主证据见 a2/a3：

### U1 · reset-keyboard

- 图：[U1-reset-keyboard-1440-light.png](U1-reset-keyboard-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:42:52.289Z；1440×1000，DPR 1，light。
- 实际操作：点击 Reset；通过 Tab/Shift+Tab 返回名称输入
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "INPUT",
    "id": "sample-name",
    "role": null,
    "text": ""
  },
  "value": ""
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

## U2

主证据 28 张；原始图 29 张。

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

补采前原图，主证据见 a2/a3：

### U2 · mobile-cards

- 图：[U2-mobile-cards-390-dark.png](U2-mobile-cards-390-dark.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:44:48.348Z；390×844，DPR 1，dark。
- 实际操作：390px下查看RadioCards状态工具
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "BUTTON",
    "id": "mobile-preview",
    "role": null,
    "text": "Preview"
  },
  "visibleText": "Component\nCheckbox\nSwitch\nToggle\nRadioGroup\nCheckboxGroup\nToggleGroup\nRadioCards\nCheckboxCards\nSlider\nRangeSlider\nTextField\nTextArea\nNumberField\nColorField\nSelect\nPresentation\nThemes\nPrimitives\nValue requests\nAccept\nRefuse\nAccept later\nSize\n1\n2\n3\nVariant\nsurface\nclassic\nLocal colour\nInherit\nBlue\nRed\nGray\nState\nNormal\nDisabled\nRead only\nInvalid\nRTL\nHigh contrast\n\nInteract with the example. Requests are separate from accepted values. Number and colour fields keep a draft until input completes.\n\nEmail\nActivity feed\nRequest count\n0\nLast request\n—\nAccepted value\n\"a\"\nCommit / cancel event\n—\nAccept pending value"
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

## U3

主证据 25 张；原始图 25 张。

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

## U4

主证据 12 张；原始图 12 张。

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

## U5

主证据 12 张；原始图 13 张。

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

补采前原图，主证据见 a2/a3：

### U5 · mobile-dark-rtl-open

- 图：[U5-mobile-dark-rtl-open-390-dark.png](U5-mobile-dark-rtl-open-390-dark.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:45:25.277Z；390×844，DPR 1，dark。
- 实际操作：390px深色RTL打开Select
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

## U6

主证据 13 张；原始图 13 张。

### U6 · compare-dom-only

- 图：[U6-compare-dom-only-1440-light.png](U6-compare-dom-only-1440-light.png)
- 出处：http://127.0.0.1:4181/；2026-10-31T06:45:25.889Z；1440×1000，DPR 1，light。
- 实际操作：切换compare查看新增表单仅DOM说明和既有GPU预览
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "BUTTON",
    "id": "renderer-compare",
    "role": null,
    "text": "DOM + GPU"
  }
}
```

### U6 · raw-default

- 图：[U6-raw-default-1440-light.png](U6-raw-default-1440-light.png)
- 出处：http://127.0.0.1:4190/；2026-10-31T06:46:44.501Z；1440×1000，DPR 1，light。
- 实际操作：Independent unstyled consumer
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "BODY",
    "id": null,
    "role": null,
    "text": "\n    Unstyled DOM partsChange writing directionCancel the default action00Toggle"
  }
}
```

### U6 · raw-cancel-portal

- 图：[U6-raw-cancel-portal-1440-light.png](U6-raw-cancel-portal-1440-light.png)
- 出处：http://127.0.0.1:4190/；2026-10-31T06:46:44.806Z；1440×1000，DPR 1，light。
- 实际操作：Cancel default action; open scope Portal
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "BUTTON",
    "id": "portal-toggle",
    "role": null,
    "text": "Toggle portal"
  },
  "childCalls": "1",
  "requests": "0"
}
```

### U6 · raw-refused

- 图：[U6-raw-refused-1440-light.png](U6-raw-refused-1440-light.png)
- 出处：http://127.0.0.1:4190/；2026-10-31T06:46:45.068Z；1440×1000，DPR 1，light。
- 实际操作：Refuse B; accepted values and native form
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "INPUT",
    "id": "choice-one-b",
    "role": null,
    "text": ""
  },
  "formData": [
    [
      "peer",
      "a"
    ],
    [
      "features",
      "b"
    ],
    [
      "format",
      "bold"
    ]
  ]
}
```

### U6 · raw-mobile

- 图：[U6-raw-mobile-390-light.png](U6-raw-mobile-390-light.png)
- 出处：http://127.0.0.1:4190/；2026-10-31T06:46:45.279Z；390×844，DPR 1，light。
- 实际操作：390px unstyled consumer
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "INPUT",
    "id": "choice-one-b",
    "role": null,
    "text": ""
  }
}
```

### U6 · raw-form-blocked

- 图：[U6-raw-form-blocked-1440-light.png](U6-raw-form-blocked-1440-light.png)
- 出处：http://127.0.0.1:4190/；2026-10-31T06:47:32.181Z；1440×1000，DPR 1，light。
- 实际操作：Empty required field then submit
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "INPUT",
    "id": "field-text",
    "role": null,
    "text": ""
  },
  "asked": "Blocked { first_error: \"text\" }"
}
```

### U6 · raw-form-busy

- 图：[U6-raw-form-busy-1440-light.png](U6-raw-form-busy-1440-light.png)
- 出处：http://127.0.0.1:4190/；2026-10-31T06:47:32.437Z；1440×1000，DPR 1，light。
- 实际操作：Valid field then submit
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "BUTTON",
    "id": "field-submit",
    "role": null,
    "text": "Save"
  },
  "asked": "Blocked { first_error: \"text\" }|Save",
  "saves": "1",
  "data": [
    [
      "title",
      "Valid native title"
    ],
    [
      "notes",
      "notes"
    ],
    [
      "number",
      "12"
    ],
    [
      "color",
      "#112233"
    ]
  ]
}
```

### U6 · raw-form-failed

- 图：[U6-raw-form-failed-1440-light.png](U6-raw-form-failed-1440-light.png)
- 出处：http://127.0.0.1:4190/；2026-10-31T06:47:32.685Z；1440×1000，DPR 1，light。
- 实际操作：Fail the pending native form save
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "BUTTON",
    "id": "field-failed",
    "role": null,
    "text": "Fail save"
  }
}
```

### U6 · raw-form-retry

- 图：[U6-raw-form-retry-1440-light.png](U6-raw-form-retry-1440-light.png)
- 出处：http://127.0.0.1:4190/；2026-10-31T06:47:32.955Z；1440×1000，DPR 1，light。
- 实际操作：Retry then complete pending save
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "BUTTON",
    "id": "field-saved",
    "role": null,
    "text": "Complete save"
  },
  "asked": "Blocked { first_error: \"text\" }|Save|Save",
  "saves": "2"
}
```

### U6 · catalog-slider

- 图：[U6-catalog-slider-1440-light.png](U6-catalog-slider-1440-light.png)
- 出处：http://127.0.0.1:4176/；2026-10-31T06:47:37.857Z；1440×1000，DPR 1，light。
- 实际操作：Catalog slider category
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "BUTTON",
    "id": "nav-slider",
    "role": null,
    "text": "slider"
  }
}
```

### U6 · catalog-number

- 图：[U6-catalog-number-1440-light.png](U6-catalog-number-1440-light.png)
- 出处：http://127.0.0.1:4176/；2026-10-31T06:47:38.103Z；1440×1000，DPR 1，light。
- 实际操作：Catalog NumberField category
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "BUTTON",
    "id": "nav-number-field",
    "role": null,
    "text": "number field"
  }
}
```

### U6 · workbench-error

- 图：[U6-workbench-error-1440-light.png](U6-workbench-error-1440-light.png)
- 出处：http://127.0.0.1:4174/objects/1；2026-10-31T06:47:39.505Z；1440×1000，DPR 1，light。
- 实际操作：Workbench invalid email then Save details
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "INPUT",
    "id": "email",
    "role": null,
    "text": ""
  }
}
```

### U6 · workbench-corrected

- 图：[U6-workbench-corrected-1440-light.png](U6-workbench-corrected-1440-light.png)
- 出处：http://127.0.0.1:4174/objects/1；2026-10-31T06:47:39.702Z；1440×1000，DPR 1，light。
- 实际操作：Corrected email clears error
- 页面读取/焦点记录：

```json
{
  "focus": {
    "tag": "INPUT",
    "id": "email",
    "role": null,
    "text": ""
  }
}
```


## 后续补采（独立重评）

- [U1-keyboard-group-a4-1440-light.png](U1-keyboard-group-a4-1440-light.png) · 2026-10-31T06:57:18.766Z · http://127.0.0.1:4181/ · 1440×1000 DPR1 light · Focus Daily then ArrowRight to Weekly
- [U1-keyboard-region-open-a4-1440-light.png](U1-keyboard-region-open-a4-1440-light.png) · 2026-10-31T06:57:19.034Z · http://127.0.0.1:4181/ · 1440×1000 DPR1 light · ArrowDown opens grouped region list
- [U1-keyboard-region-selected-a4-1440-light.png](U1-keyboard-region-selected-a4-1440-light.png) · 2026-10-31T06:57:19.311Z · http://127.0.0.1:4181/ · 1440×1000 DPR1 light · Type e then Enter selects East Asia and returns focus
- [U1-draft-native-before-a4-1440-light.png](U1-draft-native-before-a4-1440-light.png) · 2026-10-31T06:57:19.596Z · http://127.0.0.1:4181/ · 1440×1000 DPR1 light · Focused incomplete quantity remains local Draft
- [U1-draft-native-after-a4-1440-light.png](U1-draft-native-after-a4-1440-light.png) · 2026-10-31T06:57:19.884Z · http://127.0.0.1:4181/ · 1440×1000 DPR1 light · Quantity completed and form saved via keyboard
- [U2-unchecked-keyboard-a4-1440-light.png](U2-unchecked-keyboard-a4-1440-light.png) · 2026-10-31T06:57:20.382Z · http://127.0.0.1:4181/ · 1440×1000 DPR1 light · Two Space presses from mixed to checked to unchecked
- [U2-readonly-keyboard-a4-1440-light.png](U2-readonly-keyboard-a4-1440-light.png) · 2026-10-31T06:57:20.730Z · http://127.0.0.1:4181/ · 1440×1000 DPR1 light · Readonly RadioGroup ArrowRight moves focus without requesting value
- [U2-disabled-attempt-a4-1440-light.png](U2-disabled-attempt-a4-1440-light.png) · 2026-10-31T06:57:21.073Z · http://127.0.0.1:4181/ · 1440×1000 DPR1 light · Physical click at disabled Checkbox
- [U3-native-undo-a4-1440-light.png](U3-native-undo-a4-1440-light.png) · 2026-10-31T06:57:21.487Z · http://127.0.0.1:4181/ · 1440×1000 DPR1 light · Type text then native Meta+Z
- [U3-native-selection-a4-1440-light.png](U3-native-selection-a4-1440-light.png) · 2026-10-31T06:57:21.788Z · http://127.0.0.1:4181/ · 1440×1000 DPR1 light · Home and Shift+ArrowRight twice
- [U3-native-selection-a5-1440-light.png](U3-native-selection-a5-1440-light.png) · 2026-10-31T07:00:49.488Z · http://127.0.0.1:4181/ · 1440×1000 DPR1 light · macOS caret start uses Meta+ArrowLeft, then Shift+ArrowRight twice

补采逐项页面读数与焦点见 supplement-U1.md / supplement-U2.md / supplement-U3.md；U3 selection-a4 未得到选区，a5 使用 macOS 正确组合键后得到0–2选区，均保留。

## 主 agent 机器检查实拍与轮次收拢

- 原始PNG最终145张：初采131张（其中主证据112张），独立补采11张，机器检查3张。评审实际输入共123个不同PNG；机器图未给独立评审。
- 原始评分与补采重评见各judge-*.raw.md；合并见[judge.md](judge.md)。
- 完整未采格子、原因与补证方法见[coverage.md](coverage.md)。局部扫描不替代完整矩阵。
- 冻结R1目标保留在[ui-target.r1.md](../../DOM-COMPONENTS-P2.ui-target.r1.md)，hash及指纹保持原记录。用户澄清后的[当前目标](../../DOM-COMPONENTS-P2.ui-target.md)仅供下轮重建基线：文件SHA-256 `3badcee12fcd6363901e6fe0be0f87541e7300562e928caa93365b8da859002d`，同人工算法目标指纹 `c83c6339be779c05baf3a1c91dc24d47ba7bd122b975b35759649fb4342525af`。

- [M-select-names-1440-light.png](M-select-names-1440-light.png) · http://127.0.0.1:4190/fields.html · 1440×1000、DPR1、light · 文件mtime UTC 2026-10-31T07:08:48.109937+00:00 · Playwright+Chrome实拍、动画禁用、无拦截 · 真实页面滚动至Select responsive/utility及Submit fruit。
- [M-default-contrast-1440-light.png](M-default-contrast-1440-light.png) · http://127.0.0.1:4181/ · 1440×1000、DPR1、light · 文件mtime UTC 2026-10-31T07:08:50.158625+00:00 · Playwright+Chrome实拍、动画禁用、无拦截 · DOM→Cards，滚动至完整表单保存操作。
- [M-soft-switch-1440-light.png](M-soft-switch-1440-light.png) · http://127.0.0.1:4181/ · 1440×1000、DPR1、light · 文件mtime UTC 2026-10-31T07:08:50.472397+00:00 · Playwright+Chrome实拍、动画禁用、无拦截 · DOM→Cards→States and parts，实际选择Switch与soft。

页面实测及console汇总见[machine-checks.md](machine-checks.md)；截图全量SHA-256见[capture-hashes.md](capture-hashes.md)。
