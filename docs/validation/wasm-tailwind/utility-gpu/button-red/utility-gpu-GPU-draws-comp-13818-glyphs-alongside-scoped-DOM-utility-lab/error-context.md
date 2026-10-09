# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: utility-gpu.spec.ts >> GPU draws compiled colors, geometry and real font glyphs alongside scoped DOM
- Location: tests/browser/utility-gpu.spec.ts:31:5

# Error details

```
Error: expect(received).toEqual(expected) // deep equality

- Expected  - 2
+ Received  + 2

  Array [
    171,
-   205,
-   239,
+   191,
+   211,
  ]
```

# Page snapshot

```yaml
- main [ref=e2]:
  - generic [ref=e3]:
    - heading "Utility Lab" [level=1] [ref=e4]
    - paragraph [ref=e5]: Apply complete class names to a live, isolated preview.
  - generic [ref=e6]:
    - generic [ref=e7]: Classes
    - textbox "Classes" [ref=e8]: w-[137px] bg-[#123456] h-[72px] text-[#ffffff] rounded-[8px]
    - generic [ref=e9]:
      - button "Apply classes" [ref=e10] [cursor=pointer]
      - button "Restart compiler" [ref=e11] [cursor=pointer]
      - button "Copy CSS" [ref=e12] [cursor=pointer]
    - generic [ref=e13]:
      - generic [ref=e14]:
        - checkbox "Apply supported GPU properties only" [ref=e15]
        - text: Apply supported GPU properties only
      - generic [ref=e16]:
        - checkbox "Disable sample button" [ref=e17]
        - text: Disable sample button
    - group [ref=e18]:
      - generic "Container, text and color block classes" [ref=e19] [cursor=pointer]
    - status [ref=e20]: Ready. Apply classes to update the preview.
  - generic [ref=e21]:
    - region [ref=e22]:
      - heading "DOM preview" [level=2] [ref=e23]
      - paragraph [ref=e24]: Updates after the generated stylesheet is installed.
      - generic [ref=e27]:
        - button "Live utility sample" [ref=e28]
        - generic [ref=e29]: Utility text
    - region [ref=e31]:
      - heading "GPU preview" [level=2] [ref=e32]
      - paragraph [ref=e33]: Supported properties update after compilation. Unsupported properties keep the previous appearance.
      - generic [ref=e35]:
        - status [ref=e37]: GPU ready
        - paragraph [ref=e38]: "Button: applied · applied 1 · drawn 1 · 0 clicks"
        - paragraph [ref=e39]: "Container: applied · Text: applied · Color block: applied"
        - group [ref=e40]:
          - generic "GPU values and diagnostics" [ref=e41] [cursor=pointer]
  - region [ref=e42]:
    - heading "Independent scope" [level=2] [ref=e43]
    - paragraph [ref=e44]: Uses its own compiler and stylesheet.
    - generic [ref=e47]:
      - button "Live utility sample" [ref=e48]
      - generic [ref=e49]: Utility text
  - group [ref=e51]:
    - generic "Generated CSS and diagnostics" [ref=e52] [cursor=pointer]
  - generic [ref=e53]: Outside every utility scope
```

# Test source

```ts
  1   | import {test, expect, type Page} from '@playwright/test';
  2   | import {PNG} from 'pngjs';
  3   | 
  4   | const report = (page: Page) => page.evaluate(() => (window as any).__utility_lab.gpuReport('scope-a'));
  5   | async function ready(page: Page) {
  6   |   await page.goto('./');
  7   |   await expect(page.locator('#status')).toHaveAttribute('data-status', 'ready');
  8   |   await page.getByTestId('utility-gpu').scrollIntoViewIfNeeded();
  9   |   await expect(page.locator('#gpu-status')).toHaveText('GPU ready');
  10  |   await expect.poll(async () => (await report(page))?.button.drawn_revision).toBe(0);
  11  | }
  12  | async function apply(page: Page, button: string[], others: string[][] = [[], [], []], partial = false, disabled = false) {
  13  |   const revision = await page.evaluate(async ({button, others, partial, disabled}) => {
  14  |     const api = (window as any).__utility_lab;
  15  |     const result = await api.gpuApply('scope-a', [others[0], button, others[1], others[2]], partial, disabled);
  16  |     document.getElementById('scope-a-sample')!.className = `sample ${button.join(' ')}`;
  17  |     for (const [id, base, values] of [['scope-a-frame', 'preview-content', others[0]], ['scope-a-text', 'sample-text', others[1]], ['scope-a-swatch', 'sample-swatch', others[2]]] as const) {
  18  |       document.getElementById(id)!.className = `${base} ${values.join(' ')}`;
  19  |     }
  20  |     return result.revision;
  21  |   }, {button, others, partial, disabled});
  22  |   await expect.poll(async () => (await report(page))?.button.attempted_revision).toBe(revision);
  23  |   return revision;
  24  | }
  25  | async function drawn(page: Page, revision: number, slot = 'button') {
  26  |   await expect.poll(async () => (await report(page))?.[slot].drawn_revision).toBe(revision);
  27  |   return report(page);
  28  | }
  29  | const pixel = (png: PNG, x: number, y: number) => Array.from(png.data.subarray((Math.floor(y) * png.width + Math.floor(x)) * 4, (Math.floor(y) * png.width + Math.floor(x)) * 4 + 4));
  30  | 
  31  | test('GPU draws compiled colors, geometry and real font glyphs alongside scoped DOM', async ({page}, info) => {
  32  |   const errors: string[] = [], failed: string[] = [];
  33  |   page.on('pageerror', error => errors.push(error.message));
  34  |   page.on('requestfailed', request => failed.push(request.url()));
  35  |   await ready(page);
  36  |   const revision = await apply(page, ['w-[137px]', 'h-[72px]', 'bg-[#123456]', 'text-[#ffffff]', 'rounded-[8px]', 'p-[7px]', 'border-2', 'border-[#abcdef]']);
  37  |   const state = await drawn(page, revision);
  38  |   expect(state.button.status).toBe('applied');
  39  |   expect(state.button.computed.padding).toEqual([7, 7, 7, 7]);
  40  |   expect(state.button.computed.border_widths).toEqual([2, 2, 2, 2]);
  41  |   expect(state.button.geometry.slice(2)).toEqual([137, 72]);
  42  |   expect(state.button.computed.font.resource_path).toContain('IBMPlexSans');
  43  |   await expect(page.locator('#scope-a-sample')).toHaveCSS('width', '137px');
  44  |   await expect(page.locator('#scope-a-sample')).toHaveCSS('padding-top', '7px');
  45  |   const png = PNG.sync.read(await page.getByTestId('utility-gpu').screenshot());
  46  |   const [x, y, w, h] = state.button.geometry;
  47  |   expect(pixel(png, x + 8, y + 8).slice(0, 3)).toEqual([18, 52, 86]);
  48  |   const border = pixel(png, x + w / 2 + 1, y + 1);
> 49  |   expect(border.slice(0, 3)).toEqual([171, 205, 239]);
      |                              ^ Error: expect(received).toEqual(expected) // deep equality
  50  |   let whiteInk = 0;
  51  |   for (let py = y + 12; py < y + h - 12; py++) for (let px = x + 8; px < x + w - 8; px++) {
  52  |     if (pixel(png, px, py).slice(0, 3).every(value => value > 220)) whiteInk++;
  53  |   }
  54  |   expect(whiteInk).toBeGreaterThan(15);
  55  |   expect(errors).toEqual([]); expect(failed).toEqual([]);
  56  |   await info.attach('GPU actual drawn report', {body: JSON.stringify(state, null, 2), contentType: 'application/json'});
  57  |   await page.screenshot({path: info.outputPath('desktop.png'), fullPage: true});
  58  |   await page.setViewportSize({width: 390, height: 844});
  59  |   await page.getByTestId('utility-gpu').scrollIntoViewIfNeeded();
  60  |   await page.evaluate(async () => { await document.fonts.ready; await new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve))); });
  61  |   expect(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth)).toBe(true);
  62  |   await page.screenshot({path: info.outputPath('mobile.png'), fullPage: true});
  63  | });
  64  | 
  65  | test('strict GPU rejection retains the drawn style, partial reports skipped properties and deleting classes restores defaults', async ({page}) => {
  66  |   await ready(page);
  67  |   const defaults = (await report(page)).button.computed;
  68  |   const first = await apply(page, ['w-[137px]', 'bg-[#123456]']);
  69  |   const before = await drawn(page, first);
  70  |   const rejected = await apply(page, ['w-[179px]', 'bg-[#abcdef]', 'opacity-50']);
  71  |   const retained = await report(page);
  72  |   expect(retained.button.status).toBe('rejected');
  73  |   expect(retained.button.applied_revision).toBe(first);
  74  |   expect(retained.button.drawn_revision).toBe(first);
  75  |   expect(retained.button.computed).toEqual(before.button.computed);
  76  |   expect(retained.button.diagnostics.some((item: any) => item.property === 'opacity')).toBe(true);
  77  |   await apply(page, ['w-[179px]', 'bg-[#abcdef]', 'opacity-50'], [[], [], []], true);
  78  |   const partial = await drawn(page, rejected);
  79  |   expect(partial.button.status).toBe('partial');
  80  |   expect(partial.button.computed.width.value).toBe(179);
  81  |   expect(partial.button.computed.opacity).toBe(1);
  82  |   const restored = await apply(page, []);
  83  |   const reset = await drawn(page, restored);
  84  |   expect(reset.button.status).toBe('applied');
  85  |   expect(reset.button.computed).toEqual(defaults);
  86  |   await expect(page.locator('#scope-a-sample')).toHaveCSS('width', '170px');
  87  |   await expect(page.locator('#scope-a-sample')).toHaveCSS('height', '36px');
  88  | });
  89  | 
  90  | test('GPU responds to pointer, keyboard focus-visible and disabled input without recompiling', async ({page}) => {
  91  |   await ready(page);
  92  |   const revision = await apply(page, ['w-[137px]', 'h-[72px]', 'bg-[#123456]', 'hover:bg-[#abcdef]', 'focus-visible:bg-[#fedcba]', 'active:bg-[#234567]', 'disabled:bg-[#999999]']);
  93  |   let state = await drawn(page, revision);
  94  |   const canvas = page.getByTestId('utility-gpu');
  95  |   const bounds = (await canvas.boundingBox())!;
  96  |   const [x, y, w, h] = state.button.geometry;
  97  |   await page.mouse.move(bounds.x + x + w / 2 + 1, bounds.y + y + h / 2 + 1);
  98  |   await expect.poll(async () => (await report(page)).button.state.hover).toBe(true);
  99  |   expect((await report(page)).button.computed.background.slice(0, 3)).toEqual([171/255, 205/255, 239/255]);
  100 |   await page.mouse.move(0, 0);
  101 |   await expect.poll(async () => (await report(page)).button.state.hover).toBe(false);
  102 |   await page.locator('#scope-a-sample').focus();
  103 |   await expect.poll(async () => (await report(page)).button.state.focus_visible).toBe(true);
  104 |   expect((await report(page)).button.computed.background.slice(0, 3)).toEqual([254/255, 220/255, 186/255]);
  105 |   await page.keyboard.down('Space');
  106 |   await expect.poll(async () => (await report(page)).button.state.active).toBe(true);
  107 |   await page.keyboard.up('Space');
  108 |   await expect(page.locator('#gpu-revision')).toContainText('1 clicks');
  109 |   await apply(page, ['w-[137px]', 'h-[72px]', 'bg-[#123456]', 'disabled:bg-[#999999]'], [[], [], []], false, true);
  110 |   await expect.poll(async () => (await report(page)).button.state.disabled).toBe(true);
  111 |   expect((await report(page)).button.state.active).toBe(false);
  112 |   await page.keyboard.press('Space');
  113 |   await expect(page.locator('#gpu-revision')).toContainText('1 clicks');
  114 |   expect((await report(page)).button.applied_revision).toBe(revision);
  115 | });
  116 | 
  117 | test('GPU media uses the browser viewport, and alpha color blocks match DOM pixels', async ({page}) => {
  118 |   await page.setViewportSize({width: 1000, height: 1000});
  119 |   await ready(page);
  120 |   const revision = await apply(page, ['w-[89px]', 'min-[768px]:w-[149px]', 'bg-[#123456]'], [[], [], ['bg-[#c82850]/50', 'opacity-50']]);
  121 |   let state = await drawn(page, revision);
  122 |   expect(state.button.geometry[2]).toBe(149);
  123 |   await expect(page.locator('#scope-a-sample')).toHaveCSS('width', '149px');
  124 |   const png = PNG.sync.read(await page.getByTestId('utility-gpu').screenshot());
  125 |   const [x, y, w, h] = state.swatch.geometry;
  126 |   const gpu = pixel(png, x + w / 2 + 1, y + h / 2 + 1);
  127 |   const domPng = PNG.sync.read(await page.locator('#scope-a-swatch').screenshot());
  128 |   const dom = pixel(domPng, domPng.width / 2, domPng.height / 2);
  129 |   gpu.slice(0, 3).forEach((value, index) => expect(Math.abs(value - dom[index]), `alpha ${gpu}, ${dom}`).toBeLessThanOrEqual(2));
  130 |   await page.setViewportSize({width: 390, height: 844});
  131 |   await page.getByTestId('utility-gpu').scrollIntoViewIfNeeded();
  132 |   await expect.poll(async () => (await report(page)).button.geometry[2]).toBe(89);
  133 |   await expect(page.locator('#scope-a-sample')).toHaveCSS('width', '89px');
  134 |   expect((await report(page)).button.applied_revision).toBe(revision);
  135 | });
  136 | 
  137 | test('GPU applies text resources, row layout, shadows and rings from compiler bindings', async ({page}) => {
  138 |   await page.setViewportSize({width: 1280, height: 1000});
  139 |   await ready(page);
  140 |   const revision = await apply(page, ['w-[100px]', 'h-[60px]', 'bg-[#123456]', 'ring-[3px]', 'ring-[#abcdef]'],
  141 |     [['flex', 'flex-row', 'gap-[11px]', 'p-[9px]', 'items-center'], ["font-['Liberation_Mono']", 'text-[20px]', 'leading-[28px]', 'tracking-[0.05em]'], []]);
  142 |   const state = await drawn(page, revision, 'text');
  143 |   expect(state.text.status).toBe('applied');
  144 |   expect(state.text.computed.font.family).toBe('Liberation Mono');
  145 |   expect(state.text.computed.font_size_px).toBe(20);
  146 |   expect(state.text.computed.line_height.value).toBe(28);
  147 |   expect(state.text.computed.letter_spacing_em).toBe(.05);
  148 |   expect(state.container.computed.gap).toEqual([11, 11]);
  149 |   expect(state.container.computed.flow).toBe('row');
```