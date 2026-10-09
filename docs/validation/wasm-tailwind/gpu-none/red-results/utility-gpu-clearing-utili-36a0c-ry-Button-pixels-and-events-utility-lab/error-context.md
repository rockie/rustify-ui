# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: utility-gpu.spec.ts >> clearing utility ownership restores the ordinary Button pixels and events
- Location: tests/browser/utility-gpu.spec.ts:31:5

# Error details

```
Error: expect(received).toBe(expected) // Object.is equality

Expected: 0
Received: 11604

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
  22  |   await expect.poll(async () => (await report(page))?.button.attempted_revision).toBe(revision);
  23  |   return revision;
  24  | }
  25  | async function drawn(page: Page, revision: number, slot = 'button') {
  26  |   await expect.poll(async () => (await report(page))?.[slot].drawn_revision).toBe(revision);
  27  |   return report(page);
  28  | }
  29  | const pixel = (png: PNG, x: number, y: number) => Array.from(png.data.subarray((Math.floor(y) * png.width + Math.floor(x)) * 4, (Math.floor(y) * png.width + Math.floor(x)) * 4 + 4));
  30  | 
  31  | test('clearing utility ownership restores the ordinary Button pixels and events', async ({page}, info) => {
  32  |   const errors: string[] = [], failed: string[] = [];
  33  |   page.on('pageerror', error => errors.push(error.message));
  34  |   page.on('console', message => { if (message.type() === 'error') errors.push(message.text()); });
  35  |   page.on('requestfailed', request => failed.push(request.url()));
  36  |   page.on('response', response => { if (response.status() >= 400) failed.push(`${response.status()} ${response.url()}`); });
  37  |   await ready(page);
  38  |   await page.evaluate(() => {
  39  |     const container = document.createElement('div');
  40  |     container.id = 'button-baseline-root';
  41  |     document.querySelector('main')!.append(container);
  42  |     const sheet = new CSSStyleSheet();
  43  |     sheet.replaceSync('#button-baseline-root canvas { display:block; width:300px; height:240px; }');
  44  |     document.adoptedStyleSheets = [...document.adoptedStyleSheets, sheet];
  45  |     (window as any).__utility_lab.buttonBaseline.mount(container.id);
  46  |   });
  47  |   const canvas = page.getByTestId('button-baseline');
  48  |   await canvas.scrollIntoViewIfNeeded();
  49  |   await page.mouse.move(0, 0);
  50  |   const baselineReport = () => page.evaluate(() => (window as any).__utility_lab.buttonBaseline.report());
  51  |   await expect.poll(async () => (await baselineReport())?.buttons?.[1]?.geometry?.[2]).toBe(220);
  52  |   const compare = async (phase: number) => {
  53  |     await expect.poll(async () => (await baselineReport())?.phase).toBe(phase);
  54  |     const state = await baselineReport();
  55  |     expect(state.buttons.map((button: any) => button.utility)).toEqual([null, null]);
  56  |     expect(state.buttons[0].geometry.slice(2)).toEqual(state.buttons[1].geometry.slice(2));
  57  |     // Both controls use the same fonts/shaders in one canvas. Compare every
  58  |     // pixel, including text, corner coverage, border and disabled alpha.
  59  |     const capture = async () => {
  60  |       const image = PNG.sync.read(await canvas.screenshot());
  61  |       return state.buttons.map((button: any) => {
  62  |         const [x, y, width, height] = button.geometry;
  63  |         const pixels: number[] = [];
  64  |         for (let row = 0; row < height; row++) for (let col = 0; col < width; col++) pixels.push(...pixel(image, x + col, y + row));
  65  |         return pixels;
  66  |       });
  67  |     };
> 68  |     await expect.poll(async () => { const [ordinary, cleared] = await capture(); return ordinary.filter((value: number, index: number) => value !== cleared[index]).length; }).toBe(0);
      |                                                                                                                                                                                ^ Error: expect(received).toBe(expected) // Object.is equality
  69  |     return state;
  70  |   };
  71  |   const initial = await compare(0);
  72  |   for (const [dark, rem, palette, disabled] of [[false, 16, false, false], [true, 20, false, false], [false, 16, true, false], [true, 20, false, true]] as const) {
  73  |     await page.evaluate(({dark, rem, palette, disabled}) => (window as any).__utility_lab.buttonBaseline.update(1, dark, rem, palette, disabled), {dark, rem, palette, disabled});
  74  |     await expect.poll(async () => (await baselineReport())?.buttons?.[1]?.utility?.drawn_revision).toBe(1);
  75  |     expect((await baselineReport()).buttons[0].utility).toBeNull();
  76  |     await page.evaluate(({dark, rem, palette, disabled}) => (window as any).__utility_lab.buttonBaseline.update(2, dark, rem, palette, disabled), {dark, rem, palette, disabled});
  77  |     await compare(2);
  78  |   }
  79  |   await page.evaluate(() => (window as any).__utility_lab.buttonBaseline.update(2));
  80  |   const restored = await compare(2);
  81  |   const box = (await canvas.boundingBox())!;
  82  |   for (const [index, button] of restored.buttons.entries()) {
  83  |     const [x, y, width, height] = button.geometry;
  84  |     await page.mouse.click(box.x + x + width / 2, box.y + y + height / 2);
  85  |     await page.mouse.move(0, 0);
  86  |     await expect.poll(async () => (await baselineReport()).clicks[index]).toBe(1);
  87  |   }
  88  |   await compare(2);
  89  |   expect(errors).toEqual([]); expect(failed).toEqual([]);
  90  |   await info.attach('Ordinary and cleared Button reports', {body: JSON.stringify({initial, restored, clicked: await baselineReport()}, null, 2), contentType: 'application/json'});
  91  |   await canvas.screenshot({path: info.outputPath('button-none.png')});
  92  |   await page.evaluate(() => (window as any).__utility_lab.buttonBaseline.dispose());
  93  |   await expect(canvas).toHaveCount(0);
  94  | });
  95  | 
  96  | test('GPU percentages use the parent content box and follow its real resize', async ({page}, info) => {
  97  |   const errors: string[] = [], failed: string[] = [];
  98  |   page.on('pageerror', error => errors.push(error.message));
  99  |   page.on('requestfailed', request => failed.push(request.url()));
  100 |   await page.setViewportSize({width: 1000, height: 900});
  101 |   await ready(page);
  102 |   const container = ['w-[300px]', 'h-[220px]', 'p-[10px]', 'border-2', 'flex-col', 'items-start', 'justify-start', 'gap-[0px]'];
  103 |   const button = ['w-[50%]', 'h-[50%]', 'p-[10%]', 'min-w-[100px]', 'max-w-[120px]', 'bg-[#123456]'];
  104 |   const compilation = await page.evaluate(async (classes) => (window as any).__utility_lab.ensure('scope-a', classes), [...container, ...button, 'w-[60px]', 'h-[20px]', 'w-[69px]', 'h-[69px]']);
  105 |   await info.attach('Parent geometry compiler input', {body: JSON.stringify(compilation, null, 2), contentType: 'application/json'});
  106 |   expect(compilation.unsupported).toEqual([]);
  107 |   expect(compilation.rejected.filter((item: any) => [...container, ...button].includes(item.candidate))).toEqual([]);
  108 |   const revision = await apply(page, button, [container, ['w-[60px]', 'h-[20px]'], ['w-[69px]', 'h-[69px]']]);
  109 |   const fixed = await drawn(page, revision);
  110 |   expect(fixed.container.geometry.slice(2)).toEqual([300, 220]);
  111 |   expect(fixed.button.status).toBe('applied');
  112 |   expect(fixed.button.geometry.slice(2)).toEqual([120, 98]);
  113 |   fixed.button.computed.padding.forEach((value: number) => expect(value).toBeCloseTo(27.6, 5));
  114 |   const sample = page.locator('#scope-a-sample');
  115 |   await expect(sample).toHaveCSS('width', '120px');
  116 |   await expect(sample).toHaveCSS('height', '98px');
  117 |   const domPadding = await sample.evaluate(element => {
  118 |     const style = getComputedStyle(element);
  119 |     return [parseFloat(style.paddingTop), parseFloat(style.paddingLeft)];
  120 |   });
  121 |   domPadding.forEach(value => expect(Math.abs(value - 27.6)).toBeLessThan(1 / 64));
  122 |   const fixedImage = PNG.sync.read(await page.getByTestId('utility-gpu').screenshot());
  123 |   expect(pixel(fixedImage, fixed.button.geometry[0] + 4, fixed.button.geometry[1] + 4).slice(0, 3)).toEqual([18, 52, 86]);
  124 | 
  125 |   const responsive = await apply(page, ['w-[50%]', 'h-[50px]', 'p-[0px]'], [
  126 |     container.filter(value => value !== 'w-[300px]').concat('w-[50%]'),
  127 |     ['w-[20px]', 'h-[20px]'], ['w-[40px]', 'h-[40px]'],
  128 |   ]);
  129 |   const before = await drawn(page, responsive);
  130 |   const compiled = await page.evaluate(() => (window as any).__utility_lab.report('scope-a'));
  131 |   const relative = (state: any) => {
  132 |     expect(state.button.status).toBe('applied');
  133 |     expect(state.button.geometry[2]).toBeCloseTo((state.container.geometry[2] - 24) / 2, 5);
  134 |     expect(state.button.geometry[3]).toBe(50);
  135 |   };
  136 |   relative(before);
  137 |   await page.setViewportSize({width: 800, height: 900});
  138 |   await expect.poll(async () => (await report(page)).container.geometry[2]).not.toBe(before.container.geometry[2]);
  139 |   const resized = await report(page);
  140 |   relative(resized);
  141 |   expect(resized.button.drawn_revision).toBe(responsive);
  142 |   const after = await page.evaluate(() => (window as any).__utility_lab.report('scope-a'));
  143 |   expect(after.revision).toBe(compiled.revision);
  144 |   expect(after.metrics.parsed_candidates).toBe(compiled.metrics.parsed_candidates);
  145 |   expect(errors).toEqual([]); expect(failed).toEqual([]);
  146 |   await info.attach('Measured parent geometry', {body: JSON.stringify({fixed, before, resized}, null, 2), contentType: 'application/json'});
  147 |   await page.screenshot({path: info.outputPath('parent-content.png'), fullPage: true});
  148 | });
  149 | 
  150 | test('inline and path sources with media theme options drive scoped DOM and GPU', async ({page}, info) => {
  151 |   const errors: string[] = [], failed: string[] = [], violations: string[] = [];
  152 |   page.on('console', message => { if (message.type() === 'error') errors.push(`${message.text()} ${message.location().url}`); });
  153 |   page.on('pageerror', error => errors.push(error.message));
  154 |   page.on('requestfailed', request => failed.push(request.url()));
  155 |   page.on('response', response => { if (response.status() >= 400) failed.push(`${response.status()} ${response.url()}`); });
  156 |   await ready(page);
  157 |   await page.exposeFunction('recordSourceViolation', (directive: string) => violations.push(directive));
  158 |   await page.evaluate(() => {
  159 |     document.addEventListener('securitypolicyviolation', event => (window as any).recordSourceViolation(event.violatedDirective));
  160 |     const api = (window as any).__utility_lab;
  161 |     api.dispose('scope-a');
  162 |     api.mount('scope-a', 10000, '@source "./virtual/**/*.rs"; @source not "./virtual/private/**"; @media theme(inline) prefix(tw) important source(none) { @theme { --spacing: 7px; --color-brand: #123456; } @source inline("tw:w-[{137,159}px] tw:h-[72px] tw:bg-brand tw:p-1"); @source not inline("tw:w-[159px]"); @tailwind utilities; }');
  163 |   });
  164 |   await expect(page.locator('#gpu-status')).toHaveText('GPU ready');
  165 |   const revision = await apply(page, ['tw:w-[137px]', 'tw:h-[72px]', 'tw:bg-brand', 'tw:p-1']);
  166 |   const state = await drawn(page, revision);
  167 |   expect(state.button.status).toBe('applied');
  168 |   expect(state.button.geometry.slice(2)).toEqual([137, 72]);
```