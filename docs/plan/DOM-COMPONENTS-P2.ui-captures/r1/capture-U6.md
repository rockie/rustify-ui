# DOM-COMPONENTS-P2 R1 · 实拍采集

- HEAD: 64bb17281567a646b312a4b7050219fe5a7f0c55；main；dirty fingerprint: 19479199a276dd54c403765cc1f9af125254e0e60c038875cda3942d39364c1c。产品源码无未提交差异，原有未提交内容是仓库指引、历史评审与状态台账。
- 目标指纹：phoenix-ui-target-v1/manual-u64be:43a74974da555b476c3c8ce9a27112cba19b79d6aa75a96a5a156a6d6f402c29；目标文件 SHA-256 3703966418fcf4a31531799167d5b4874a4dba8a076a15e339456b91db881c89。
- 指纹规则：UTF-8 LF; heading sections 单元 then 规范要点; each includes heading and one trailing LF; domain phoenix-ui-target-v1 NUL; u64be byte length + bytes; no linked local attachments in these sections. Manual implementation, not engine-verified. 使用固定人工编码；无 engine 比对，不与未知算法跨轮比较。
- 工具：Playwright 1.63.0 + 本机 Google Chrome；headless，DPR1，locale en-US、Asia/Shanghai；截图等待字体、animations disabled。系统原生 validationMessage 实际为中文；未替换验证文案。
- 工作树当前构建：Studio build_id dc7fb113f1267f33；raw/consumer 本轮 build-dom；catalog/workbench 本轮 release。均由仓内 mbx 命令生成，浏览器通过 HTTP 加载；未拦截网络、未注入替代 UI。
- 状态通过实际选择/点击/按键/鼠标操作到达；为取景仅移动真实滚动位置。无截图拼接/修图，未使用旧图。
- 首轮没有上一轮图片/评分。未执行 OS IME、VoiceOver、真机触控或浏览器 UI 200% 缩放；按计划不归入本期必需格子。

## 范围 U6

本单元主证据如下；其它单元本评审不评分。

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

