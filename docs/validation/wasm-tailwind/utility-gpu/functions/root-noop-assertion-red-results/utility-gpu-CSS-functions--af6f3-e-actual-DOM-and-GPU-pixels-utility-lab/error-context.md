# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: utility-gpu.spec.ts >> CSS functions resolve runtime candidates once and drive actual DOM and GPU pixels
- Location: tests/browser/utility-gpu.spec.ts:67:5

# Error details

```
Error: expect(received).toBe(expected) // Object.is equality

Expected: 0
Received: 8
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
      - generic [ref=e28]:
        - button "Live utility sample" [ref=e29]
        - generic [ref=e30]: Utility text
    - region [ref=e32]:
      - heading "GPU preview" [level=2] [ref=e33]
      - paragraph [ref=e34]: Supported properties update after compilation. Unsupported properties keep the previous appearance.
      - generic [ref=e36]:
        - status [ref=e38]: GPU ready
        - paragraph [ref=e39]: "Button: applied · applied 1 · drawn 1 · 0 clicks"
        - paragraph [ref=e40]: "Container: applied · Text: applied · Color block: applied"
        - group [ref=e41]:
          - generic "GPU values and diagnostics" [ref=e42] [cursor=pointer]
  - region [ref=e43]:
    - heading "Independent scope" [level=2] [ref=e44]
    - paragraph [ref=e45]: Uses its own compiler and stylesheet.
    - generic [ref=e48]:
      - button "Live utility sample" [ref=e49]
      - generic [ref=e50]: Utility text
  - group [ref=e52]:
    - generic "Generated CSS and diagnostics" [ref=e53] [cursor=pointer]
  - generic [ref=e54]: Outside every utility scope
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
  45  |   const pixels = await page.getByTestId('utility-gpu').screenshot();
  46  |   await info.attach('GPU canvas pixels', {body: pixels, contentType: 'image/png'});
  47  |   await info.attach('GPU actual drawn report', {body: JSON.stringify(state, null, 2), contentType: 'application/json'});
  48  |   const png = PNG.sync.read(pixels);
  49  |   const [x, y, w, h] = state.button.geometry;
  50  |   expect(pixel(png, x + 8, y + 8).slice(0, 3)).toEqual([18, 52, 86]);
  51  |   const border = pixel(png, x + w / 2 + 1, y + 1);
  52  |   expect(border.slice(0, 3)).toEqual([171, 205, 239]);
  53  |   let whiteInk = 0;
  54  |   for (let py = y + 12; py < y + h - 12; py++) for (let px = x + 8; px < x + w - 8; px++) {
  55  |     if (pixel(png, px, py).slice(0, 3).every(value => value > 220)) whiteInk++;
  56  |   }
  57  |   expect(whiteInk).toBeGreaterThan(15);
  58  |   expect(errors).toEqual([]); expect(failed).toEqual([]);
  59  |   await page.screenshot({path: info.outputPath('desktop.png'), fullPage: true});
  60  |   await page.setViewportSize({width: 390, height: 844});
  61  |   await page.getByTestId('utility-gpu').scrollIntoViewIfNeeded();
  62  |   await page.evaluate(async () => { await document.fonts.ready; await new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve))); });
  63  |   expect(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth)).toBe(true);
  64  |   await page.screenshot({path: info.outputPath('mobile.png'), fullPage: true});
  65  | });
  66  | 
  67  | test('CSS functions resolve runtime candidates once and drive actual DOM and GPU pixels', async ({page}, info) => {
  68  |   const errors: string[] = [], failed: string[] = [];
  69  |   page.on('pageerror', error => errors.push(error.message));
  70  |   page.on('requestfailed', request => failed.push(request.url()));
  71  |   await ready(page);
  72  |   const classes = ['w-[--theme(--missing,128px)]', 'h-[theme(spacing.12)]', 'p-[theme(spacing.2)]',
  73  |     'bg-[color:--alpha(#123456_/_1)]', 'text-[color:--alpha(#ffffff_/_1)]',
  74  |     'rounded-[--theme(--missing,8px)]', 'border-[length:--theme(--missing,2px)]',
  75  |     'border-[color:--alpha(#abcdef_/_1)]'];
  76  |   const revision = await apply(page, classes);
  77  |   const state = await drawn(page, revision);
  78  |   expect(state.button.status).toBe('applied');
  79  |   expect(state.button.geometry.slice(2)).toEqual([128, 48]);
  80  |   expect(state.button.computed.padding).toEqual([8, 8, 8, 8]);
  81  |   expect(state.button.computed.border_widths).toEqual([2, 2, 2, 2]);
  82  |   const sample = page.locator('#scope-a-sample');
  83  |   await expect(sample).toHaveCSS('width', '128px');
  84  |   await expect(sample).toHaveCSS('height', '48px');
  85  |   await expect(sample).toHaveCSS('padding-top', '8px');
  86  |   await expect(sample).toHaveCSS('background-color', 'rgb(18, 52, 86)');
  87  |   const png = PNG.sync.read(await page.getByTestId('utility-gpu').screenshot());
  88  |   const [x, y, width] = state.button.geometry;
  89  |   expect(pixel(png, x + 8, y + 8).slice(0, 3)).toEqual([18, 52, 86]);
  90  |   expect(pixel(png, x + width / 2 + 1, y + 1).slice(0, 3)).toEqual([171, 205, 239]);
  91  |   const repeated = await page.evaluate(async classes => (window as any).__utility_lab.ensure('scope-a', classes), classes);
  92  |   expect(repeated.revision).toBe(revision);
> 93  |   expect(repeated.metrics.parsed_candidates).toBe(0);
      |                                              ^ Error: expect(received).toBe(expected) // Object.is equality
  94  |   expect(repeated.metrics.generated_candidates).toBe(0);
  95  |   expect(errors).toEqual([]); expect(failed).toEqual([]);
  96  |   await info.attach('CSS functions drawn GPU report', {body: JSON.stringify(state, null, 2), contentType: 'application/json'});
  97  |   await page.screenshot({path: info.outputPath('gpu-css-functions.png'), fullPage: true});
  98  | });
  99  | 
  100 | test('strict GPU rejection retains the drawn style, partial reports skipped properties and deleting classes restores defaults', async ({page}) => {
  101 |   await ready(page);
  102 |   const defaults = (await report(page)).button.computed;
  103 |   const first = await apply(page, ['w-[137px]', 'bg-[#123456]']);
  104 |   const before = await drawn(page, first);
  105 |   const rejected = await apply(page, ['w-[179px]', 'bg-[#abcdef]', 'opacity-50']);
  106 |   const retained = await report(page);
  107 |   expect(retained.button.status).toBe('rejected');
  108 |   expect(retained.button.applied_revision).toBe(first);
  109 |   expect(retained.button.drawn_revision).toBe(first);
  110 |   expect(retained.button.computed).toEqual(before.button.computed);
  111 |   expect(retained.button.diagnostics.some((item: any) => item.property === 'opacity')).toBe(true);
  112 |   await apply(page, ['w-[179px]', 'bg-[#abcdef]', 'opacity-50'], [[], [], []], true);
  113 |   const partial = await drawn(page, rejected);
  114 |   expect(partial.button.status).toBe('partial');
  115 |   expect(partial.button.computed.width.value).toBe(179);
  116 |   expect(partial.button.computed.opacity).toBe(1);
  117 |   const restored = await apply(page, []);
  118 |   const reset = await drawn(page, restored);
  119 |   expect(reset.button.status).toBe('applied');
  120 |   expect(reset.button.computed).toEqual(defaults);
  121 |   await expect(page.locator('#scope-a-sample')).toHaveCSS('width', '170px');
  122 |   await expect(page.locator('#scope-a-sample')).toHaveCSS('height', '36px');
  123 | });
  124 | 
  125 | test('GPU responds to pointer, keyboard focus-visible and disabled input without recompiling', async ({page}) => {
  126 |   await ready(page);
  127 |   const revision = await apply(page, ['w-[137px]', 'h-[72px]', 'bg-[#123456]', 'hover:bg-[#abcdef]', 'focus-visible:bg-[#fedcba]', 'active:bg-[#234567]', 'disabled:bg-[#999999]']);
  128 |   let state = await drawn(page, revision);
  129 |   const canvas = page.getByTestId('utility-gpu');
  130 |   const bounds = (await canvas.boundingBox())!;
  131 |   const [x, y, w, h] = state.button.geometry;
  132 |   await page.mouse.move(bounds.x + x + w / 2 + 1, bounds.y + y + h / 2 + 1);
  133 |   await expect.poll(async () => (await report(page)).button.state.hover).toBe(true);
  134 |   expect((await report(page)).button.computed.background.slice(0, 3)).toEqual([171/255, 205/255, 239/255]);
  135 |   await page.mouse.move(0, 0);
  136 |   await expect.poll(async () => (await report(page)).button.state.hover).toBe(false);
  137 |   await page.locator('#scope-a-sample').focus();
  138 |   await expect.poll(async () => (await report(page)).button.state.focus_visible).toBe(true);
  139 |   expect((await report(page)).button.computed.background.slice(0, 3)).toEqual([254/255, 220/255, 186/255]);
  140 |   await page.keyboard.down('Space');
  141 |   await expect.poll(async () => (await report(page)).button.state.active).toBe(true);
  142 |   await page.keyboard.up('Space');
  143 |   await expect(page.locator('#gpu-revision')).toContainText('1 clicks');
  144 |   await apply(page, ['w-[137px]', 'h-[72px]', 'bg-[#123456]', 'disabled:bg-[#999999]'], [[], [], []], false, true);
  145 |   await expect.poll(async () => (await report(page)).button.state.disabled).toBe(true);
  146 |   expect((await report(page)).button.state.active).toBe(false);
  147 |   await page.keyboard.press('Space');
  148 |   await expect(page.locator('#gpu-revision')).toContainText('1 clicks');
  149 |   expect((await report(page)).button.applied_revision).toBe(revision);
  150 | });
  151 | 
  152 | test('GPU media uses the browser viewport, and alpha color blocks match DOM pixels', async ({page}) => {
  153 |   await page.setViewportSize({width: 1000, height: 1000});
  154 |   await ready(page);
  155 |   const revision = await apply(page, ['w-[89px]', 'min-[768px]:w-[149px]', 'bg-[#123456]'], [[], [], ['bg-[#c82850]/50', 'opacity-50']]);
  156 |   let state = await drawn(page, revision);
  157 |   expect(state.button.geometry[2]).toBe(149);
  158 |   await expect(page.locator('#scope-a-sample')).toHaveCSS('width', '149px');
  159 |   const png = PNG.sync.read(await page.getByTestId('utility-gpu').screenshot());
  160 |   const [x, y, w, h] = state.swatch.geometry;
  161 |   const gpu = pixel(png, x + w / 2 + 1, y + h / 2 + 1);
  162 |   const domPng = PNG.sync.read(await page.locator('#scope-a-swatch').screenshot());
  163 |   const dom = pixel(domPng, domPng.width / 2, domPng.height / 2);
  164 |   gpu.slice(0, 3).forEach((value, index) => expect(Math.abs(value - dom[index]), `alpha ${gpu}, ${dom}`).toBeLessThanOrEqual(2));
  165 |   await page.setViewportSize({width: 390, height: 844});
  166 |   await page.getByTestId('utility-gpu').scrollIntoViewIfNeeded();
  167 |   await expect.poll(async () => (await report(page)).button.geometry[2]).toBe(89);
  168 |   await expect(page.locator('#scope-a-sample')).toHaveCSS('width', '89px');
  169 |   expect((await report(page)).button.applied_revision).toBe(revision);
  170 | });
  171 | 
  172 | test('GPU applies text resources, row layout, shadows and rings from compiler bindings', async ({page}) => {
  173 |   await page.setViewportSize({width: 1280, height: 1000});
  174 |   await ready(page);
  175 |   const revision = await apply(page, ['w-[100px]', 'h-[60px]', 'bg-[#123456]', 'ring-[3px]', 'ring-[#abcdef]'],
  176 |     [['flex', 'flex-row', 'gap-[11px]', 'p-[9px]', 'items-center'], ["font-['Liberation_Mono']", 'text-[20px]', 'leading-[28px]', 'tracking-[0.05em]'], []]);
  177 |   const state = await drawn(page, revision, 'text');
  178 |   expect(state.text.status).toBe('applied');
  179 |   expect(state.text.computed.font.family).toBe('Liberation Mono');
  180 |   expect(state.text.computed.font_size_px).toBe(20);
  181 |   expect(state.text.computed.line_height.value).toBe(28);
  182 |   expect(state.text.computed.letter_spacing_em).toBe(.05);
  183 |   expect(state.container.computed.gap).toEqual([11, 11]);
  184 |   expect(state.container.computed.flow).toBe('row');
  185 |   expect(state.text.geometry[0] - state.button.geometry[0] - state.button.geometry[2]).toBeCloseTo(11);
  186 |   expect(state.swatch.geometry[0] - state.text.geometry[0] - state.text.geometry[2]).toBeCloseTo(11);
  187 |   expect(state.button.computed.shadows.some((shadow: any) => shadow.spread === 3)).toBe(true);
  188 |   await expect(page.locator('#scope-a-text')).toHaveCSS('font-family', '"Liberation Mono"');
  189 |   await expect(page.locator('#scope-a-text')).toHaveCSS('font-size', '20px');
  190 |   const png = PNG.sync.read(await page.getByTestId('utility-gpu').screenshot());
  191 |   const [x, y, w] = state.button.geometry;
  192 |   const ring = pixel(png, x + w / 2 + 1, y - 1);
  193 |   [171, 205, 239].forEach((value, index) => expect(Math.abs(ring[index] - value)).toBeLessThanOrEqual(3));
```