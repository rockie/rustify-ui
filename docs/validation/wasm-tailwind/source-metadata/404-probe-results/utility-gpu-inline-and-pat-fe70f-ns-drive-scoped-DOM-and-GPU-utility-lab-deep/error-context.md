# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: utility-gpu.spec.ts >> inline and path sources with media theme options drive scoped DOM and GPU
- Location: tests/browser/utility-gpu.spec.ts:31:5

# Error details

```
Error: expect(received).toEqual(expected) // deep equality

- Expected  - 1
+ Received  + 3

- Array []
+ Array [
+   "Failed to load resource: the server responded with a status of 404 (Not Found) http://127.0.0.1:4187/favicon.ico",
+ ]
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
  31  | test('inline and path sources with media theme options drive scoped DOM and GPU', async ({page}, info) => {
  32  |   const errors: string[] = [], failed: string[] = [], violations: string[] = [];
  33  |   page.on('console', message => { if (message.type() === 'error') errors.push(`${message.text()} ${message.location().url}`); });
  34  |   page.on('pageerror', error => errors.push(error.message));
  35  |   page.on('requestfailed', request => failed.push(request.url()));
  36  |   page.on('response', response => { if (response.status() >= 400) failed.push(`${response.status()} ${response.url()}`); });
  37  |   await ready(page);
  38  |   await page.exposeFunction('recordSourceViolation', (directive: string) => violations.push(directive));
  39  |   await page.evaluate(() => {
  40  |     document.addEventListener('securitypolicyviolation', event => (window as any).recordSourceViolation(event.violatedDirective));
  41  |     const api = (window as any).__utility_lab;
  42  |     api.dispose('scope-a');
  43  |     api.mount('scope-a', 10000, '@source "./virtual/**/*.rs"; @source not "./virtual/private/**"; @media theme(inline) prefix(tw) important source(none) { @theme { --spacing: 7px; --color-brand: #123456; } @source inline("tw:w-[{137,159}px] tw:h-[72px] tw:bg-brand tw:p-1"); @source not inline("tw:w-[159px]"); @tailwind utilities; }');
  44  |   });
  45  |   await expect(page.locator('#gpu-status')).toHaveText('GPU ready');
  46  |   const revision = await apply(page, ['tw:w-[137px]', 'tw:h-[72px]', 'tw:bg-brand', 'tw:p-1']);
  47  |   const state = await drawn(page, revision);
  48  |   expect(state.button.status).toBe('applied');
  49  |   expect(state.button.geometry.slice(2)).toEqual([137, 72]);
  50  |   expect(state.button.computed.padding).toEqual([7, 7, 7, 7]);
  51  |   const sample = page.locator('#scope-a-sample');
  52  |   await expect(sample).toHaveCSS('width', '137px');
  53  |   await expect(sample).toHaveCSS('height', '72px');
  54  |   await expect(sample).toHaveCSS('padding-top', '7px');
  55  |   await expect(sample).toHaveCSS('background-color', 'rgb(18, 52, 86)');
  56  |   const result = await page.evaluate(async () => {
  57  |     const api = (window as any).__utility_lab;
  58  |     const before = api.report('scope-a');
  59  |     const rejected = await api.ensure('scope-a', ['tw:w-[159px]']);
  60  |     return {before, rejected, otherWidth: getComputedStyle(document.getElementById('scope-b-sample')!).width};
  61  |   });
  62  |   expect(result.before.css).toContain('width: 137px !important');
  63  |   expect(result.before.css).not.toContain('width: 159px');
  64  |   expect(result.before.css).not.toContain('@source');
  65  |   expect(result.before.css).not.toContain('theme(inline)');
  66  |   expect(result.rejected.rejected.some((item: any) => item.candidate === 'tw:w-[159px]')).toBe(true);
  67  |   expect(result.rejected.revision).toBe(revision);
  68  |   expect(result.otherWidth).toBe('170px');
  69  |   const png = PNG.sync.read(await page.getByTestId('utility-gpu').screenshot());
  70  |   const [x, y] = state.button.geometry;
  71  |   expect(pixel(png, x + 8, y + 8).slice(0, 3)).toEqual([18, 52, 86]);
> 72  |   expect(errors).toEqual([]); expect(failed).toEqual([]); expect(violations).toEqual([]);
      |                  ^ Error: expect(received).toEqual(expected) // deep equality
  73  |   await info.attach('Source expansion and theme option values', {body: JSON.stringify({state, ...result}, null, 2), contentType: 'application/json'});
  74  |   await page.screenshot({path: info.outputPath('source-options.png'), fullPage: true});
  75  | });
  76  | 
  77  | test('applied utility dependencies drive DOM and GPU and survive an installation failure', async ({page}, info) => {
  78  |   const errors: string[] = [], failed: string[] = [], violations: string[] = [];
  79  |   page.on('pageerror', error => errors.push(error.message));
  80  |   page.on('requestfailed', request => failed.push(request.url()));
  81  |   await ready(page);
  82  |   await page.exposeFunction('recordApplyViolation', (directive: string) => violations.push(directive));
  83  |   await page.evaluate(() => {
  84  |     document.addEventListener('securitypolicyviolation', event => (window as any).recordApplyViolation(event.violatedDirective));
  85  |     const api = (window as any).__utility_lab;
  86  |     api.dispose('scope-a');
  87  |     api.mount('scope-a', 10000, '@utility base-card { @apply w-[137px] h-[72px] bg-[#123456] text-[#ffffff]; } @utility applied-card { @apply base-card p-[7px] border-2 border-[#abcdef]; } @utility next-card { @apply w-[179px] h-[72px] bg-[#abcdef]; } @tailwind utilities;');
  88  |   });
  89  |   await expect(page.locator('#gpu-status')).toHaveText('GPU ready');
  90  |   const first = await apply(page, ['applied-card']);
  91  |   const before = await drawn(page, first);
  92  |   expect(before.button.status).toBe('applied');
  93  |   expect(before.button.geometry.slice(2)).toEqual([137, 72]);
  94  |   expect(before.button.computed.padding).toEqual([7, 7, 7, 7]);
  95  |   expect(before.button.computed.border_widths).toEqual([2, 2, 2, 2]);
  96  |   const sample = page.locator('#scope-a-sample');
  97  |   await expect(sample).toHaveCSS('width', '137px');
  98  |   await expect(sample).toHaveCSS('background-color', 'rgb(18, 52, 86)');
  99  |   const png = PNG.sync.read(await page.getByTestId('utility-gpu').screenshot());
  100 |   const [x, y, width] = before.button.geometry;
  101 |   expect(pixel(png, x + 8, y + 8).slice(0, 3)).toEqual([18, 52, 86]);
  102 |   expect(pixel(png, x + width / 2 + 1, y + 1).slice(0, 3)).toEqual([171, 205, 239]);
  103 |   const failure = await page.evaluate(async () => {
  104 |     const original = CSSStyleSheet.prototype.replaceSync;
  105 |     CSSStyleSheet.prototype.replaceSync = function() { throw new DOMException('injected @apply install failure', 'SyntaxError'); };
  106 |     try { await (window as any).__utility_lab.gpuApply('scope-a', [[], ['next-card'], [], []]); }
  107 |     catch (error) { return String(error); }
  108 |     finally { CSSStyleSheet.prototype.replaceSync = original; }
  109 |   });
  110 |   expect(failure).toContain('injected @apply install failure');
  111 |   const retained = await report(page);
  112 |   expect(retained.button.applied_revision).toBe(first);
  113 |   expect(retained.button.drawn_revision).toBe(first);
  114 |   expect(retained.button.computed).toEqual(before.button.computed);
  115 |   await expect(sample).toHaveCSS('width', '137px');
  116 |   const next = await apply(page, ['next-card']);
  117 |   const recovered = await drawn(page, next);
  118 |   expect(next).toBe(first + 1);
  119 |   expect(recovered.button.geometry.slice(2)).toEqual([179, 72]);
  120 |   await expect(sample).toHaveCSS('width', '179px');
  121 |   await expect(sample).toHaveCSS('background-color', 'rgb(171, 205, 239)');
  122 |   expect(errors).toEqual([]); expect(failed).toEqual([]); expect(violations).toEqual([]);
  123 |   await info.attach('Applied dependencies and recovery', {body: JSON.stringify({before, retained, recovered}, null, 2), contentType: 'application/json'});
  124 |   await page.screenshot({path: info.outputPath('applied-utility.png'), fullPage: true});
  125 | });
  126 | 
  127 | test('GPU draws compiled colors, geometry and real font glyphs alongside scoped DOM', async ({page}, info) => {
  128 |   const errors: string[] = [], failed: string[] = [];
  129 |   page.on('pageerror', error => errors.push(error.message));
  130 |   page.on('requestfailed', request => failed.push(request.url()));
  131 |   await ready(page);
  132 |   const revision = await apply(page, ['w-[137px]', 'h-[72px]', 'bg-[#123456]', 'text-[#ffffff]', 'rounded-[8px]', 'p-[7px]', 'border-2', 'border-[#abcdef]']);
  133 |   const state = await drawn(page, revision);
  134 |   expect(state.button.status).toBe('applied');
  135 |   expect(state.button.computed.padding).toEqual([7, 7, 7, 7]);
  136 |   expect(state.button.computed.border_widths).toEqual([2, 2, 2, 2]);
  137 |   expect(state.button.geometry.slice(2)).toEqual([137, 72]);
  138 |   expect(state.button.computed.font.resource_path).toContain('IBMPlexSans');
  139 |   await expect(page.locator('#scope-a-sample')).toHaveCSS('width', '137px');
  140 |   await expect(page.locator('#scope-a-sample')).toHaveCSS('padding-top', '7px');
  141 |   const pixels = await page.getByTestId('utility-gpu').screenshot();
  142 |   await info.attach('GPU canvas pixels', {body: pixels, contentType: 'image/png'});
  143 |   await info.attach('GPU actual drawn report', {body: JSON.stringify(state, null, 2), contentType: 'application/json'});
  144 |   const png = PNG.sync.read(pixels);
  145 |   const [x, y, w, h] = state.button.geometry;
  146 |   expect(pixel(png, x + 8, y + 8).slice(0, 3)).toEqual([18, 52, 86]);
  147 |   const border = pixel(png, x + w / 2 + 1, y + 1);
  148 |   expect(border.slice(0, 3)).toEqual([171, 205, 239]);
  149 |   let whiteInk = 0;
  150 |   for (let py = y + 12; py < y + h - 12; py++) for (let px = x + 8; px < x + w - 8; px++) {
  151 |     if (pixel(png, px, py).slice(0, 3).every(value => value > 220)) whiteInk++;
  152 |   }
  153 |   expect(whiteInk).toBeGreaterThan(15);
  154 |   expect(errors).toEqual([]); expect(failed).toEqual([]);
  155 |   await page.screenshot({path: info.outputPath('desktop.png'), fullPage: true});
  156 |   await page.setViewportSize({width: 390, height: 844});
  157 |   await page.getByTestId('utility-gpu').scrollIntoViewIfNeeded();
  158 |   await page.evaluate(async () => { await document.fonts.ready; await new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve))); });
  159 |   expect(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth)).toBe(true);
  160 |   await page.screenshot({path: info.outputPath('mobile.png'), fullPage: true});
  161 | });
  162 | 
  163 | test('CSS functions resolve runtime candidates once and drive actual DOM and GPU pixels', async ({page}, info) => {
  164 |   const errors: string[] = [], failed: string[] = [];
  165 |   page.on('pageerror', error => errors.push(error.message));
  166 |   page.on('requestfailed', request => failed.push(request.url()));
  167 |   await ready(page);
  168 |   const classes = ['w-[--theme(--missing,128px)]', 'h-[theme(spacing.12)]', 'p-[theme(spacing.2)]',
  169 |     'bg-[color:--alpha(#123456_/_1)]', 'text-[color:--alpha(#ffffff_/_1)]',
  170 |     'rounded-[--theme(--missing,8px)]', 'border-[length:--theme(--missing,2px)]',
  171 |     'border-[color:--alpha(#abcdef_/_1)]'];
  172 |   const revision = await apply(page, classes);
```