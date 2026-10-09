# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: utility-gpu.spec.ts >> applied utility dependencies drive DOM and GPU and survive an installation failure
- Location: tests/browser/utility-gpu.spec.ts:31:5

# Error details

```
Error: page.evaluate: CSS syntax at 0: Cannot apply unknown utility class `text-white`. Are you using CSS modules or similar and missing `@reference`? https://tailwindcss.com/docs/functions-and-directives#reference-directive
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
        - paragraph [ref=e39]: "Button: applied · applied 0 · drawn 0 · 0 clicks"
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
> 13  |   const revision = await page.evaluate(async ({button, others, partial, disabled}) => {
      |                               ^ Error: page.evaluate: CSS syntax at 0: Cannot apply unknown utility class `text-white`. Are you using CSS modules or similar and missing `@reference`? https://tailwindcss.com/docs/functions-and-directives#reference-directive
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
  31  | test('applied utility dependencies drive DOM and GPU and survive an installation failure', async ({page}, info) => {
  32  |   const errors: string[] = [], failed: string[] = [], violations: string[] = [];
  33  |   page.on('pageerror', error => errors.push(error.message));
  34  |   page.on('requestfailed', request => failed.push(request.url()));
  35  |   await ready(page);
  36  |   await page.exposeFunction('recordApplyViolation', (directive: string) => violations.push(directive));
  37  |   await page.evaluate(() => {
  38  |     document.addEventListener('securitypolicyviolation', event => (window as any).recordApplyViolation(event.violatedDirective));
  39  |     const api = (window as any).__utility_lab;
  40  |     api.dispose('scope-a');
  41  |     api.mount('scope-a', 10000, '@utility base-card { @apply w-[137px] h-[72px] bg-[#123456] text-white; } @utility applied-card { @apply base-card p-[7px] border-2 border-[#abcdef]; } @utility next-card { @apply w-[179px] h-[72px] bg-[#abcdef]; } @tailwind utilities;');
  42  |   });
  43  |   await expect(page.locator('#gpu-status')).toHaveText('GPU ready');
  44  |   const first = await apply(page, ['applied-card']);
  45  |   const before = await drawn(page, first);
  46  |   expect(before.button.status).toBe('applied');
  47  |   expect(before.button.geometry.slice(2)).toEqual([137, 72]);
  48  |   expect(before.button.computed.padding).toEqual([7, 7, 7, 7]);
  49  |   expect(before.button.computed.border_widths).toEqual([2, 2, 2, 2]);
  50  |   const sample = page.locator('#scope-a-sample');
  51  |   await expect(sample).toHaveCSS('width', '137px');
  52  |   await expect(sample).toHaveCSS('background-color', 'rgb(18, 52, 86)');
  53  |   const png = PNG.sync.read(await page.getByTestId('utility-gpu').screenshot());
  54  |   const [x, y, width] = before.button.geometry;
  55  |   expect(pixel(png, x + 8, y + 8).slice(0, 3)).toEqual([18, 52, 86]);
  56  |   expect(pixel(png, x + width / 2 + 1, y + 1).slice(0, 3)).toEqual([171, 205, 239]);
  57  |   const failure = await page.evaluate(async () => {
  58  |     const original = CSSStyleSheet.prototype.replaceSync;
  59  |     CSSStyleSheet.prototype.replaceSync = function() { throw new DOMException('injected @apply install failure', 'SyntaxError'); };
  60  |     try { await (window as any).__utility_lab.gpuApply('scope-a', [[], ['next-card'], [], []]); }
  61  |     catch (error) { return JSON.parse(String(error)); }
  62  |     finally { CSSStyleSheet.prototype.replaceSync = original; }
  63  |   });
  64  |   expect(failure.kind).toBe('install');
  65  |   const retained = await report(page);
  66  |   expect(retained.button.applied_revision).toBe(first);
  67  |   expect(retained.button.drawn_revision).toBe(first);
  68  |   expect(retained.button.computed).toEqual(before.button.computed);
  69  |   await expect(sample).toHaveCSS('width', '137px');
  70  |   const next = await apply(page, ['next-card']);
  71  |   const recovered = await drawn(page, next);
  72  |   expect(next).toBe(first + 1);
  73  |   expect(recovered.button.geometry.slice(2)).toEqual([179, 72]);
  74  |   await expect(sample).toHaveCSS('width', '179px');
  75  |   await expect(sample).toHaveCSS('background-color', 'rgb(171, 205, 239)');
  76  |   expect(errors).toEqual([]); expect(failed).toEqual([]); expect(violations).toEqual([]);
  77  |   await info.attach('Applied dependencies and recovery', {body: JSON.stringify({before, retained, recovered}, null, 2), contentType: 'application/json'});
  78  |   await page.screenshot({path: info.outputPath('applied-utility.png'), fullPage: true});
  79  | });
  80  | 
  81  | test('GPU draws compiled colors, geometry and real font glyphs alongside scoped DOM', async ({page}, info) => {
  82  |   const errors: string[] = [], failed: string[] = [];
  83  |   page.on('pageerror', error => errors.push(error.message));
  84  |   page.on('requestfailed', request => failed.push(request.url()));
  85  |   await ready(page);
  86  |   const revision = await apply(page, ['w-[137px]', 'h-[72px]', 'bg-[#123456]', 'text-[#ffffff]', 'rounded-[8px]', 'p-[7px]', 'border-2', 'border-[#abcdef]']);
  87  |   const state = await drawn(page, revision);
  88  |   expect(state.button.status).toBe('applied');
  89  |   expect(state.button.computed.padding).toEqual([7, 7, 7, 7]);
  90  |   expect(state.button.computed.border_widths).toEqual([2, 2, 2, 2]);
  91  |   expect(state.button.geometry.slice(2)).toEqual([137, 72]);
  92  |   expect(state.button.computed.font.resource_path).toContain('IBMPlexSans');
  93  |   await expect(page.locator('#scope-a-sample')).toHaveCSS('width', '137px');
  94  |   await expect(page.locator('#scope-a-sample')).toHaveCSS('padding-top', '7px');
  95  |   const pixels = await page.getByTestId('utility-gpu').screenshot();
  96  |   await info.attach('GPU canvas pixels', {body: pixels, contentType: 'image/png'});
  97  |   await info.attach('GPU actual drawn report', {body: JSON.stringify(state, null, 2), contentType: 'application/json'});
  98  |   const png = PNG.sync.read(pixels);
  99  |   const [x, y, w, h] = state.button.geometry;
  100 |   expect(pixel(png, x + 8, y + 8).slice(0, 3)).toEqual([18, 52, 86]);
  101 |   const border = pixel(png, x + w / 2 + 1, y + 1);
  102 |   expect(border.slice(0, 3)).toEqual([171, 205, 239]);
  103 |   let whiteInk = 0;
  104 |   for (let py = y + 12; py < y + h - 12; py++) for (let px = x + 8; px < x + w - 8; px++) {
  105 |     if (pixel(png, px, py).slice(0, 3).every(value => value > 220)) whiteInk++;
  106 |   }
  107 |   expect(whiteInk).toBeGreaterThan(15);
  108 |   expect(errors).toEqual([]); expect(failed).toEqual([]);
  109 |   await page.screenshot({path: info.outputPath('desktop.png'), fullPage: true});
  110 |   await page.setViewportSize({width: 390, height: 844});
  111 |   await page.getByTestId('utility-gpu').scrollIntoViewIfNeeded();
  112 |   await page.evaluate(async () => { await document.fonts.ready; await new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve))); });
  113 |   expect(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth)).toBe(true);
```