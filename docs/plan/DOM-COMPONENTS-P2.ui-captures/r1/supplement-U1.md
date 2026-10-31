# R1 补采 · U1

目标使用冻结包 DOM-COMPONENTS-P2.ui-target.r1.md；版本/视口/字体条件与初采相同。本次仅追加实拍，不修改产品。

## keyboard-group-a4

- 图片：[U1-keyboard-group-a4-1440-light.png](U1-keyboard-group-a4-1440-light.png)
- http://127.0.0.1:4181/；2026-10-31T06:57:18.766Z；1440×1000，DPR1，light
- 操作：Focus Daily then ArrowRight to Weekly

```json
{
  "unit": "U1",
  "state": "keyboard-group-a4",
  "file": "U1-keyboard-group-a4-1440-light.png",
  "url": "http://127.0.0.1:4181/",
  "viewport": {
    "width": 1440,
    "height": 1000
  },
  "dpr": 1,
  "theme": "light",
  "at": "2026-10-31T06:57:18.766Z",
  "action": "Focus Daily then ArrowRight to Weekly",
  "focus": {
    "tag": "INPUT",
    "id": "sample-weekly",
    "role": null,
    "text": ""
  }
}
```

## keyboard-region-open-a4

- 图片：[U1-keyboard-region-open-a4-1440-light.png](U1-keyboard-region-open-a4-1440-light.png)
- http://127.0.0.1:4181/；2026-10-31T06:57:19.034Z；1440×1000，DPR1，light
- 操作：ArrowDown opens grouped region list

```json
{
  "unit": "U1",
  "state": "keyboard-region-open-a4",
  "file": "U1-keyboard-region-open-a4-1440-light.png",
  "url": "http://127.0.0.1:4181/",
  "viewport": {
    "width": 1440,
    "height": 1000
  },
  "dpr": 1,
  "theme": "light",
  "at": "2026-10-31T06:57:19.034Z",
  "action": "ArrowDown opens grouped region list",
  "focus": {
    "tag": "BUTTON",
    "id": "sample-region",
    "role": "combobox",
    "text": "Choose a region"
  }
}
```

## keyboard-region-selected-a4

- 图片：[U1-keyboard-region-selected-a4-1440-light.png](U1-keyboard-region-selected-a4-1440-light.png)
- http://127.0.0.1:4181/；2026-10-31T06:57:19.311Z；1440×1000，DPR1，light
- 操作：Type e then Enter selects East Asia and returns focus

```json
{
  "unit": "U1",
  "state": "keyboard-region-selected-a4",
  "file": "U1-keyboard-region-selected-a4-1440-light.png",
  "url": "http://127.0.0.1:4181/",
  "viewport": {
    "width": 1440,
    "height": 1000
  },
  "dpr": 1,
  "theme": "light",
  "at": "2026-10-31T06:57:19.311Z",
  "action": "Type e then Enter selects East Asia and returns focus",
  "focus": {
    "tag": "BUTTON",
    "id": "sample-region",
    "role": "combobox",
    "text": "East Asia"
  }
}
```

## draft-native-before-a4

- 图片：[U1-draft-native-before-a4-1440-light.png](U1-draft-native-before-a4-1440-light.png)
- http://127.0.0.1:4181/；2026-10-31T06:57:19.596Z；1440×1000，DPR1，light
- 操作：Focused incomplete quantity remains local Draft

```json
{
  "unit": "U1",
  "state": "draft-native-before-a4",
  "file": "U1-draft-native-before-a4-1440-light.png",
  "url": "http://127.0.0.1:4181/",
  "viewport": {
    "width": 1440,
    "height": 1000
  },
  "dpr": 1,
  "theme": "light",
  "at": "2026-10-31T06:57:19.596Z",
  "action": "Focused incomplete quantity remains local Draft",
  "focus": {
    "tag": "INPUT",
    "id": "sample-amount",
    "role": "spinbutton",
    "text": ""
  },
  "formData": [
    [
      "name",
      "Keyboard Avery"
    ],
    [
      "description",
      ""
    ],
    [
      "topics",
      "updates"
    ],
    [
      "cadence",
      "weekly"
    ],
    [
      "plan",
      "personal"
    ],
    [
      "channels",
      "mail"
    ],
    [
      "strength",
      "50"
    ],
    [
      "window",
      "20"
    ],
    [
      "window",
      "80"
    ],
    [
      "amount",
      "12"
    ],
    [
      "color",
      "#336699"
    ],
    [
      "region",
      "east"
    ],
    [
      "consent",
      "on"
    ]
  ]
}
```

## draft-native-after-a4

- 图片：[U1-draft-native-after-a4-1440-light.png](U1-draft-native-after-a4-1440-light.png)
- http://127.0.0.1:4181/；2026-10-31T06:57:19.884Z；1440×1000，DPR1，light
- 操作：Quantity completed and form saved via keyboard

```json
{
  "unit": "U1",
  "state": "draft-native-after-a4",
  "file": "U1-draft-native-after-a4-1440-light.png",
  "url": "http://127.0.0.1:4181/",
  "viewport": {
    "width": 1440,
    "height": 1000
  },
  "dpr": 1,
  "theme": "light",
  "at": "2026-10-31T06:57:19.884Z",
  "action": "Quantity completed and form saved via keyboard",
  "focus": {
    "tag": "BUTTON",
    "id": "sample-save",
    "role": null,
    "text": "Save locally"
  },
  "formData": [
    [
      "name",
      "Keyboard Avery"
    ],
    [
      "description",
      ""
    ],
    [
      "topics",
      "updates"
    ],
    [
      "cadence",
      "weekly"
    ],
    [
      "plan",
      "personal"
    ],
    [
      "channels",
      "mail"
    ],
    [
      "strength",
      "50"
    ],
    [
      "window",
      "20"
    ],
    [
      "window",
      "80"
    ],
    [
      "amount",
      "17.5"
    ],
    [
      "color",
      "#336699"
    ],
    [
      "region",
      "east"
    ],
    [
      "consent",
      "on"
    ]
  ],
  "status": "Saved in this preview only · 1"
}
```

