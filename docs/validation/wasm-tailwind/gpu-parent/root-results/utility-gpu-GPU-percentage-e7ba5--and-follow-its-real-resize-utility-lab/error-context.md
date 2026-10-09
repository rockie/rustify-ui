# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: utility-gpu.spec.ts >> GPU percentages use the parent content box and follow its real resize
- Location: tests/browser/utility-gpu.spec.ts:31:5

# Error details

```
Error: expect(received).toBe(expected) // Object.is equality

Expected: 1
Received: 0

Call Log:
- Timeout 15000ms exceeded while waiting on the predicate
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
  - group [ref=e21]:
    - generic "Scan source text" [ref=e22] [cursor=pointer]
  - generic [ref=e23]:
    - region [ref=e24]:
      - heading "DOM preview" [level=2] [ref=e25]
      - paragraph [ref=e26]: Updates after the generated stylesheet is installed.
      - generic [ref=e30]:
        - button "Live utility sample" [ref=e31]
        - generic [ref=e32]: Utility text
    - region [ref=e34]:
      - heading "GPU preview" [level=2] [ref=e35]
      - paragraph [ref=e36]: Supported properties update after compilation. Unsupported properties keep the previous appearance.
      - generic [ref=e38]:
        - status [ref=e40]: GPU ready
        - paragraph [ref=e41]: "Button: applied · applied 0 · drawn 0 · 0 clicks"
        - paragraph [ref=e42]: "Container: applied · Text: applied · Color block: applied"
        - group [ref=e43]:
          - generic "GPU values and diagnostics" [ref=e44] [cursor=pointer]
  - region [ref=e45]:
    - heading "Independent scope" [level=2] [ref=e46]
    - paragraph [ref=e47]: Uses its own compiler and stylesheet.
    - generic [ref=e50]:
      - button "Live utility sample" [ref=e51]
      - generic [ref=e52]: Utility text
  - group [ref=e54]:
    - generic "Generated CSS and diagnostics" [ref=e55] [cursor=pointer]
  - generic [ref=e56]: Outside every utility scope
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
> 22  |   await expect.poll(async () => (await report(page))?.button.attempted_revision).toBe(revision);
      |                                                                                  ^ Error: expect(received).toBe(expected) // Object.is equality
  23  |   return revision;
  24  | }
  25  | async function drawn(page: Page, revision: number, slot = 'button') {
  26  |   await expect.poll(async () => (await report(page))?.[slot].drawn_revision).toBe(revision);
  27  |   return report(page);
  28  | }
  29  | const pixel = (png: PNG, x: number, y: number) => Array.from(png.data.subarray((Math.floor(y) * png.width + Math.floor(x)) * 4, (Math.floor(y) * png.width + Math.floor(x)) * 4 + 4));
  30  | 
  31  | test('GPU percentages use the parent content box and follow its real resize', async ({page}, info) => {
  32  |   const errors: string[] = [], failed: string[] = [];
  33  |   page.on('pageerror', error => errors.push(error.message));
  34  |   page.on('requestfailed', request => failed.push(request.url()));
  35  |   await page.setViewportSize({width: 1000, height: 900});
  36  |   await ready(page);
  37  |   const container = ['w-[300px]', 'h-[220px]', 'p-[10px]', 'border-2', 'flex-col', 'items-start', 'justify-start', 'gap-0'];
  38  |   const button = ['w-[50%]', 'h-[50%]', 'p-[10%]', 'min-w-[100px]', 'max-w-[120px]', 'bg-[#123456]'];
  39  |   const revision = await apply(page, button, [container, ['w-[60px]', 'h-[20px]'], ['w-[69px]', 'h-[69px]']]);
  40  |   const fixed = await drawn(page, revision);
  41  |   expect(fixed.container.geometry.slice(2)).toEqual([300, 220]);
  42  |   expect(fixed.button.status).toBe('applied');
  43  |   expect(fixed.button.geometry.slice(2)).toEqual([120, 98]);
  44  |   fixed.button.computed.padding.forEach((value: number) => expect(value).toBeCloseTo(27.6, 5));
  45  |   const sample = page.locator('#scope-a-sample');
  46  |   await expect(sample).toHaveCSS('width', '120px');
  47  |   await expect(sample).toHaveCSS('height', '98px');
  48  |   await expect(sample).toHaveCSS('padding-top', '27.6px');
  49  |   await expect(sample).toHaveCSS('padding-left', '27.6px');
  50  |   const fixedImage = PNG.sync.read(await page.getByTestId('utility-gpu').screenshot());
  51  |   expect(pixel(fixedImage, fixed.button.geometry[0] + 4, fixed.button.geometry[1] + 4).slice(0, 3)).toEqual([18, 52, 86]);
  52  | 
  53  |   const responsive = await apply(page, ['w-[50%]', 'h-[50px]', 'p-0'], [
  54  |     container.filter(value => value !== 'w-[300px]').concat('w-[50%]'),
  55  |     ['w-[20px]', 'h-[20px]'], ['w-[40px]', 'h-[40px]'],
  56  |   ]);
  57  |   const before = await drawn(page, responsive);
  58  |   const compiled = await page.evaluate(() => (window as any).__utility_lab.report('scope-a'));
  59  |   const relative = (state: any) => {
  60  |     expect(state.button.status).toBe('applied');
  61  |     expect(state.button.geometry[2]).toBeCloseTo((state.container.geometry[2] - 24) / 2, 5);
  62  |     expect(state.button.geometry[3]).toBe(50);
  63  |   };
  64  |   relative(before);
  65  |   await page.setViewportSize({width: 800, height: 900});
  66  |   await expect.poll(async () => (await report(page)).container.geometry[2]).not.toBe(before.container.geometry[2]);
  67  |   const resized = await report(page);
  68  |   relative(resized);
  69  |   expect(resized.button.drawn_revision).toBe(responsive);
  70  |   const after = await page.evaluate(() => (window as any).__utility_lab.report('scope-a'));
  71  |   expect(after.revision).toBe(compiled.revision);
  72  |   expect(after.metrics.parsed_candidates).toBe(compiled.metrics.parsed_candidates);
  73  |   expect(errors).toEqual([]); expect(failed).toEqual([]);
  74  |   await info.attach('Measured parent geometry', {body: JSON.stringify({fixed, before, resized}, null, 2), contentType: 'application/json'});
  75  |   await page.screenshot({path: info.outputPath('parent-content.png'), fullPage: true});
  76  | });
  77  | 
  78  | test('inline and path sources with media theme options drive scoped DOM and GPU', async ({page}, info) => {
  79  |   const errors: string[] = [], failed: string[] = [], violations: string[] = [];
  80  |   page.on('console', message => { if (message.type() === 'error') errors.push(`${message.text()} ${message.location().url}`); });
  81  |   page.on('pageerror', error => errors.push(error.message));
  82  |   page.on('requestfailed', request => failed.push(request.url()));
  83  |   page.on('response', response => { if (response.status() >= 400) failed.push(`${response.status()} ${response.url()}`); });
  84  |   await ready(page);
  85  |   await page.exposeFunction('recordSourceViolation', (directive: string) => violations.push(directive));
  86  |   await page.evaluate(() => {
  87  |     document.addEventListener('securitypolicyviolation', event => (window as any).recordSourceViolation(event.violatedDirective));
  88  |     const api = (window as any).__utility_lab;
  89  |     api.dispose('scope-a');
  90  |     api.mount('scope-a', 10000, '@source "./virtual/**/*.rs"; @source not "./virtual/private/**"; @media theme(inline) prefix(tw) important source(none) { @theme { --spacing: 7px; --color-brand: #123456; } @source inline("tw:w-[{137,159}px] tw:h-[72px] tw:bg-brand tw:p-1"); @source not inline("tw:w-[159px]"); @tailwind utilities; }');
  91  |   });
  92  |   await expect(page.locator('#gpu-status')).toHaveText('GPU ready');
  93  |   const revision = await apply(page, ['tw:w-[137px]', 'tw:h-[72px]', 'tw:bg-brand', 'tw:p-1']);
  94  |   const state = await drawn(page, revision);
  95  |   expect(state.button.status).toBe('applied');
  96  |   expect(state.button.geometry.slice(2)).toEqual([137, 72]);
  97  |   expect(state.button.computed.padding).toEqual([7, 7, 7, 7]);
  98  |   const sample = page.locator('#scope-a-sample');
  99  |   await expect(sample).toHaveCSS('width', '137px');
  100 |   await expect(sample).toHaveCSS('height', '72px');
  101 |   await expect(sample).toHaveCSS('padding-top', '7px');
  102 |   await expect(sample).toHaveCSS('background-color', 'rgb(18, 52, 86)');
  103 |   const result = await page.evaluate(async () => {
  104 |     const api = (window as any).__utility_lab;
  105 |     const before = api.report('scope-a');
  106 |     const rejected = await api.ensure('scope-a', ['tw:w-[159px]']);
  107 |     return {before, rejected, otherWidth: getComputedStyle(document.getElementById('scope-b-sample')!).width};
  108 |   });
  109 |   expect(result.before.css).toContain('width: 137px !important');
  110 |   expect(result.before.css).not.toContain('width: 159px');
  111 |   expect(result.before.css).not.toContain('@source');
  112 |   expect(result.before.css).not.toContain('theme(inline)');
  113 |   expect(result.rejected.rejected.some((item: any) => item.candidate === 'tw:w-[159px]')).toBe(true);
  114 |   expect(result.rejected.revision).toBe(revision);
  115 |   expect(result.otherWidth).toBe('170px');
  116 |   const png = PNG.sync.read(await page.getByTestId('utility-gpu').screenshot());
  117 |   const [x, y] = state.button.geometry;
  118 |   expect(pixel(png, x + 8, y + 8).slice(0, 3)).toEqual([18, 52, 86]);
  119 |   expect(errors).toEqual([]); expect(failed).toEqual([]); expect(violations).toEqual([]);
  120 |   await info.attach('Source expansion and theme option values', {body: JSON.stringify({state, ...result}, null, 2), contentType: 'application/json'});
  121 |   await page.screenshot({path: info.outputPath('source-options.png'), fullPage: true});
  122 | });
```