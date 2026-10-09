# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: utility-gpu.spec.ts >> GPU percentages use the parent content box and follow its real resize
- Location: tests/browser/utility-gpu.spec.ts:31:5

# Error details

```
Error: expect(locator).toHaveCSS(expected) failed

Locator:  locator('#scope-a-sample')
Expected: "27.6px"
Received: "27.5938px"
Timeout:  15000ms

Call log:
  - Expect "toHaveCSS" locator('#scope-a-sample') with timeout 15000ms
  - waiting for locator('#scope-a-sample')
    34 × locator resolved to <div tabindex="0" role="button" id="scope-a-sample" class="sample w-[50%] h-[50%] p-[10%] min-w-[100px] max-w-[120px] bg-[#123456]">Live utility sample</div>
       - unexpected value "27.5938px"

```

```yaml
- button "Live utility sample"
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
  31  | test('GPU percentages use the parent content box and follow its real resize', async ({page}, info) => {
  32  |   const errors: string[] = [], failed: string[] = [];
  33  |   page.on('pageerror', error => errors.push(error.message));
  34  |   page.on('requestfailed', request => failed.push(request.url()));
  35  |   await page.setViewportSize({width: 1000, height: 900});
  36  |   await ready(page);
  37  |   const container = ['w-[300px]', 'h-[220px]', 'p-[10px]', 'border-2', 'flex-col', 'items-start', 'justify-start', 'gap-[0px]'];
  38  |   const button = ['w-[50%]', 'h-[50%]', 'p-[10%]', 'min-w-[100px]', 'max-w-[120px]', 'bg-[#123456]'];
  39  |   const compilation = await page.evaluate(async (classes) => (window as any).__utility_lab.ensure('scope-a', classes), [...container, ...button, 'w-[60px]', 'h-[20px]', 'w-[69px]', 'h-[69px]']);
  40  |   await info.attach('Parent geometry compiler input', {body: JSON.stringify(compilation, null, 2), contentType: 'application/json'});
  41  |   expect(compilation.unsupported).toEqual([]);
  42  |   expect(compilation.rejected.filter((item: any) => [...container, ...button].includes(item.candidate))).toEqual([]);
  43  |   const revision = await apply(page, button, [container, ['w-[60px]', 'h-[20px]'], ['w-[69px]', 'h-[69px]']]);
  44  |   const fixed = await drawn(page, revision);
  45  |   expect(fixed.container.geometry.slice(2)).toEqual([300, 220]);
  46  |   expect(fixed.button.status).toBe('applied');
  47  |   expect(fixed.button.geometry.slice(2)).toEqual([120, 98]);
  48  |   fixed.button.computed.padding.forEach((value: number) => expect(value).toBeCloseTo(27.6, 5));
  49  |   const sample = page.locator('#scope-a-sample');
  50  |   await expect(sample).toHaveCSS('width', '120px');
  51  |   await expect(sample).toHaveCSS('height', '98px');
> 52  |   await expect(sample).toHaveCSS('padding-top', '27.6px');
      |                        ^ Error: expect(locator).toHaveCSS(expected) failed
  53  |   await expect(sample).toHaveCSS('padding-left', '27.6px');
  54  |   const fixedImage = PNG.sync.read(await page.getByTestId('utility-gpu').screenshot());
  55  |   expect(pixel(fixedImage, fixed.button.geometry[0] + 4, fixed.button.geometry[1] + 4).slice(0, 3)).toEqual([18, 52, 86]);
  56  | 
  57  |   const responsive = await apply(page, ['w-[50%]', 'h-[50px]', 'p-[0px]'], [
  58  |     container.filter(value => value !== 'w-[300px]').concat('w-[50%]'),
  59  |     ['w-[20px]', 'h-[20px]'], ['w-[40px]', 'h-[40px]'],
  60  |   ]);
  61  |   const before = await drawn(page, responsive);
  62  |   const compiled = await page.evaluate(() => (window as any).__utility_lab.report('scope-a'));
  63  |   const relative = (state: any) => {
  64  |     expect(state.button.status).toBe('applied');
  65  |     expect(state.button.geometry[2]).toBeCloseTo((state.container.geometry[2] - 24) / 2, 5);
  66  |     expect(state.button.geometry[3]).toBe(50);
  67  |   };
  68  |   relative(before);
  69  |   await page.setViewportSize({width: 800, height: 900});
  70  |   await expect.poll(async () => (await report(page)).container.geometry[2]).not.toBe(before.container.geometry[2]);
  71  |   const resized = await report(page);
  72  |   relative(resized);
  73  |   expect(resized.button.drawn_revision).toBe(responsive);
  74  |   const after = await page.evaluate(() => (window as any).__utility_lab.report('scope-a'));
  75  |   expect(after.revision).toBe(compiled.revision);
  76  |   expect(after.metrics.parsed_candidates).toBe(compiled.metrics.parsed_candidates);
  77  |   expect(errors).toEqual([]); expect(failed).toEqual([]);
  78  |   await info.attach('Measured parent geometry', {body: JSON.stringify({fixed, before, resized}, null, 2), contentType: 'application/json'});
  79  |   await page.screenshot({path: info.outputPath('parent-content.png'), fullPage: true});
  80  | });
  81  | 
  82  | test('inline and path sources with media theme options drive scoped DOM and GPU', async ({page}, info) => {
  83  |   const errors: string[] = [], failed: string[] = [], violations: string[] = [];
  84  |   page.on('console', message => { if (message.type() === 'error') errors.push(`${message.text()} ${message.location().url}`); });
  85  |   page.on('pageerror', error => errors.push(error.message));
  86  |   page.on('requestfailed', request => failed.push(request.url()));
  87  |   page.on('response', response => { if (response.status() >= 400) failed.push(`${response.status()} ${response.url()}`); });
  88  |   await ready(page);
  89  |   await page.exposeFunction('recordSourceViolation', (directive: string) => violations.push(directive));
  90  |   await page.evaluate(() => {
  91  |     document.addEventListener('securitypolicyviolation', event => (window as any).recordSourceViolation(event.violatedDirective));
  92  |     const api = (window as any).__utility_lab;
  93  |     api.dispose('scope-a');
  94  |     api.mount('scope-a', 10000, '@source "./virtual/**/*.rs"; @source not "./virtual/private/**"; @media theme(inline) prefix(tw) important source(none) { @theme { --spacing: 7px; --color-brand: #123456; } @source inline("tw:w-[{137,159}px] tw:h-[72px] tw:bg-brand tw:p-1"); @source not inline("tw:w-[159px]"); @tailwind utilities; }');
  95  |   });
  96  |   await expect(page.locator('#gpu-status')).toHaveText('GPU ready');
  97  |   const revision = await apply(page, ['tw:w-[137px]', 'tw:h-[72px]', 'tw:bg-brand', 'tw:p-1']);
  98  |   const state = await drawn(page, revision);
  99  |   expect(state.button.status).toBe('applied');
  100 |   expect(state.button.geometry.slice(2)).toEqual([137, 72]);
  101 |   expect(state.button.computed.padding).toEqual([7, 7, 7, 7]);
  102 |   const sample = page.locator('#scope-a-sample');
  103 |   await expect(sample).toHaveCSS('width', '137px');
  104 |   await expect(sample).toHaveCSS('height', '72px');
  105 |   await expect(sample).toHaveCSS('padding-top', '7px');
  106 |   await expect(sample).toHaveCSS('background-color', 'rgb(18, 52, 86)');
  107 |   const result = await page.evaluate(async () => {
  108 |     const api = (window as any).__utility_lab;
  109 |     const before = api.report('scope-a');
  110 |     const rejected = await api.ensure('scope-a', ['tw:w-[159px]']);
  111 |     return {before, rejected, otherWidth: getComputedStyle(document.getElementById('scope-b-sample')!).width};
  112 |   });
  113 |   expect(result.before.css).toContain('width: 137px !important');
  114 |   expect(result.before.css).not.toContain('width: 159px');
  115 |   expect(result.before.css).not.toContain('@source');
  116 |   expect(result.before.css).not.toContain('theme(inline)');
  117 |   expect(result.rejected.rejected.some((item: any) => item.candidate === 'tw:w-[159px]')).toBe(true);
  118 |   expect(result.rejected.revision).toBe(revision);
  119 |   expect(result.otherWidth).toBe('170px');
  120 |   const png = PNG.sync.read(await page.getByTestId('utility-gpu').screenshot());
  121 |   const [x, y] = state.button.geometry;
  122 |   expect(pixel(png, x + 8, y + 8).slice(0, 3)).toEqual([18, 52, 86]);
  123 |   expect(errors).toEqual([]); expect(failed).toEqual([]); expect(violations).toEqual([]);
  124 |   await info.attach('Source expansion and theme option values', {body: JSON.stringify({state, ...result}, null, 2), contentType: 'application/json'});
  125 |   await page.screenshot({path: info.outputPath('source-options.png'), fullPage: true});
  126 | });
  127 | 
  128 | test('applied utility dependencies drive DOM and GPU and survive an installation failure', async ({page}, info) => {
  129 |   const errors: string[] = [], failed: string[] = [], violations: string[] = [];
  130 |   page.on('pageerror', error => errors.push(error.message));
  131 |   page.on('requestfailed', request => failed.push(request.url()));
  132 |   await ready(page);
  133 |   await page.exposeFunction('recordApplyViolation', (directive: string) => violations.push(directive));
  134 |   await page.evaluate(() => {
  135 |     document.addEventListener('securitypolicyviolation', event => (window as any).recordApplyViolation(event.violatedDirective));
  136 |     const api = (window as any).__utility_lab;
  137 |     api.dispose('scope-a');
  138 |     api.mount('scope-a', 10000, '@utility base-card { @apply w-[137px] h-[72px] bg-[#123456] text-[#ffffff]; } @utility applied-card { @apply base-card p-[7px] border-2 border-[#abcdef]; } @utility next-card { @apply w-[179px] h-[72px] bg-[#abcdef]; } @tailwind utilities;');
  139 |   });
  140 |   await expect(page.locator('#gpu-status')).toHaveText('GPU ready');
  141 |   const first = await apply(page, ['applied-card']);
  142 |   const before = await drawn(page, first);
  143 |   expect(before.button.status).toBe('applied');
  144 |   expect(before.button.geometry.slice(2)).toEqual([137, 72]);
  145 |   expect(before.button.computed.padding).toEqual([7, 7, 7, 7]);
  146 |   expect(before.button.computed.border_widths).toEqual([2, 2, 2, 2]);
  147 |   const sample = page.locator('#scope-a-sample');
  148 |   await expect(sample).toHaveCSS('width', '137px');
  149 |   await expect(sample).toHaveCSS('background-color', 'rgb(18, 52, 86)');
  150 |   const png = PNG.sync.read(await page.getByTestId('utility-gpu').screenshot());
  151 |   const [x, y, width] = before.button.geometry;
  152 |   expect(pixel(png, x + 8, y + 8).slice(0, 3)).toEqual([18, 52, 86]);
```