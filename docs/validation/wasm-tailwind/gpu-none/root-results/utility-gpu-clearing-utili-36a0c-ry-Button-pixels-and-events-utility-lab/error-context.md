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
Received: 60

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
  52  |   const compare = async (phase: number, generation = 0) => {
  53  |     await expect.poll(async () => (await baselineReport())?.generation).toBe(generation);
  54  |     await expect.poll(async () => (await baselineReport())?.phase).toBe(phase);
  55  |     const state = await baselineReport();
  56  |     expect(state.buttons.map((button: any) => button.utility)).toEqual([null, null]);
  57  |     expect(state.buttons[0].geometry.slice(2)).toEqual(state.buttons[1].geometry.slice(2));
  58  |     // Both controls use the same fonts/shaders in one canvas. Compare every
  59  |     // pixel, including text, corner coverage, border and disabled alpha.
  60  |     const capture = async () => {
  61  |       const image = PNG.sync.read(await canvas.screenshot());
  62  |       return state.buttons.map((button: any) => {
  63  |         const [x, y, width, height] = button.geometry;
  64  |         const pixels: number[] = [];
  65  |         for (let row = 0; row < height; row++) for (let col = 0; col < width; col++) pixels.push(...pixel(image, x + col, y + row));
  66  |         return pixels;
  67  |       });
  68  |     };
> 69  |     await expect.poll(async () => { const [ordinary, cleared] = await capture(); return ordinary.filter((value: number, index: number) => value !== cleared[index]).length; }).toBe(0);
      |                                                                                                                                                                                ^ Error: expect(received).toBe(expected) // Object.is equality
  70  |     return state;
  71  |   };
  72  |   const initial = await compare(0);
  73  |   let clearedGeneration = 0;
  74  |   for (const [dark, rem, palette, disabled] of [[false, 16, false, false], [true, 20, false, false], [false, 16, true, false], [true, 20, false, true]] as const) {
  75  |     await page.evaluate(({dark, rem, palette, disabled}) => (window as any).__utility_lab.buttonBaseline.update(1, dark, rem, palette, disabled), {dark, rem, palette, disabled});
  76  |     await expect.poll(async () => (await baselineReport())?.buttons?.[1]?.utility?.drawn_revision).toBe(1);
  77  |     expect((await baselineReport()).buttons[0].utility).toBeNull();
  78  |     clearedGeneration = await page.evaluate(({dark, rem, palette, disabled}) => (window as any).__utility_lab.buttonBaseline.update(2, dark, rem, palette, disabled), {dark, rem, palette, disabled});
  79  |     await compare(2, clearedGeneration);
  80  |   }
  81  |   clearedGeneration = await page.evaluate(() => (window as any).__utility_lab.buttonBaseline.update(2));
  82  |   const restored = await compare(2, clearedGeneration);
  83  |   const box = (await canvas.boundingBox())!;
  84  |   for (const [index, button] of restored.buttons.entries()) {
  85  |     const [x, y, width, height] = button.geometry;
  86  |     await page.mouse.click(box.x + x + width / 2, box.y + y + height / 2);
  87  |     await page.mouse.move(0, 0);
  88  |     await expect.poll(async () => (await baselineReport()).clicks[index]).toBe(1);
  89  |   }
  90  |   await compare(2, clearedGeneration);
  91  |   expect(errors).toEqual([]); expect(failed).toEqual([]);
  92  |   await info.attach('Ordinary and cleared Button reports', {body: JSON.stringify({initial, restored, clicked: await baselineReport()}, null, 2), contentType: 'application/json'});
  93  |   await canvas.screenshot({path: info.outputPath('button-none.png')});
  94  |   await page.evaluate(() => (window as any).__utility_lab.buttonBaseline.dispose());
  95  |   await expect(canvas).toHaveCount(0);
  96  | });
  97  | 
  98  | test('GPU percentages use the parent content box and follow its real resize', async ({page}, info) => {
  99  |   const errors: string[] = [], failed: string[] = [];
  100 |   page.on('pageerror', error => errors.push(error.message));
  101 |   page.on('requestfailed', request => failed.push(request.url()));
  102 |   await page.setViewportSize({width: 1000, height: 900});
  103 |   await ready(page);
  104 |   const container = ['w-[300px]', 'h-[220px]', 'p-[10px]', 'border-2', 'flex-col', 'items-start', 'justify-start', 'gap-[0px]'];
  105 |   const button = ['w-[50%]', 'h-[50%]', 'p-[10%]', 'min-w-[100px]', 'max-w-[120px]', 'bg-[#123456]'];
  106 |   const compilation = await page.evaluate(async (classes) => (window as any).__utility_lab.ensure('scope-a', classes), [...container, ...button, 'w-[60px]', 'h-[20px]', 'w-[69px]', 'h-[69px]']);
  107 |   await info.attach('Parent geometry compiler input', {body: JSON.stringify(compilation, null, 2), contentType: 'application/json'});
  108 |   expect(compilation.unsupported).toEqual([]);
  109 |   expect(compilation.rejected.filter((item: any) => [...container, ...button].includes(item.candidate))).toEqual([]);
  110 |   const revision = await apply(page, button, [container, ['w-[60px]', 'h-[20px]'], ['w-[69px]', 'h-[69px]']]);
  111 |   const fixed = await drawn(page, revision);
  112 |   expect(fixed.container.geometry.slice(2)).toEqual([300, 220]);
  113 |   expect(fixed.button.status).toBe('applied');
  114 |   expect(fixed.button.geometry.slice(2)).toEqual([120, 98]);
  115 |   fixed.button.computed.padding.forEach((value: number) => expect(value).toBeCloseTo(27.6, 5));
  116 |   const sample = page.locator('#scope-a-sample');
  117 |   await expect(sample).toHaveCSS('width', '120px');
  118 |   await expect(sample).toHaveCSS('height', '98px');
  119 |   const domPadding = await sample.evaluate(element => {
  120 |     const style = getComputedStyle(element);
  121 |     return [parseFloat(style.paddingTop), parseFloat(style.paddingLeft)];
  122 |   });
  123 |   domPadding.forEach(value => expect(Math.abs(value - 27.6)).toBeLessThan(1 / 64));
  124 |   const fixedImage = PNG.sync.read(await page.getByTestId('utility-gpu').screenshot());
  125 |   expect(pixel(fixedImage, fixed.button.geometry[0] + 4, fixed.button.geometry[1] + 4).slice(0, 3)).toEqual([18, 52, 86]);
  126 | 
  127 |   const responsive = await apply(page, ['w-[50%]', 'h-[50px]', 'p-[0px]'], [
  128 |     container.filter(value => value !== 'w-[300px]').concat('w-[50%]'),
  129 |     ['w-[20px]', 'h-[20px]'], ['w-[40px]', 'h-[40px]'],
  130 |   ]);
  131 |   const before = await drawn(page, responsive);
  132 |   const compiled = await page.evaluate(() => (window as any).__utility_lab.report('scope-a'));
  133 |   const relative = (state: any) => {
  134 |     expect(state.button.status).toBe('applied');
  135 |     expect(state.button.geometry[2]).toBeCloseTo((state.container.geometry[2] - 24) / 2, 5);
  136 |     expect(state.button.geometry[3]).toBe(50);
  137 |   };
  138 |   relative(before);
  139 |   await page.setViewportSize({width: 800, height: 900});
  140 |   await expect.poll(async () => (await report(page)).container.geometry[2]).not.toBe(before.container.geometry[2]);
  141 |   const resized = await report(page);
  142 |   relative(resized);
  143 |   expect(resized.button.drawn_revision).toBe(responsive);
  144 |   const after = await page.evaluate(() => (window as any).__utility_lab.report('scope-a'));
  145 |   expect(after.revision).toBe(compiled.revision);
  146 |   expect(after.metrics.parsed_candidates).toBe(compiled.metrics.parsed_candidates);
  147 |   expect(errors).toEqual([]); expect(failed).toEqual([]);
  148 |   await info.attach('Measured parent geometry', {body: JSON.stringify({fixed, before, resized}, null, 2), contentType: 'application/json'});
  149 |   await page.screenshot({path: info.outputPath('parent-content.png'), fullPage: true});
  150 | });
  151 | 
  152 | test('inline and path sources with media theme options drive scoped DOM and GPU', async ({page}, info) => {
  153 |   const errors: string[] = [], failed: string[] = [], violations: string[] = [];
  154 |   page.on('console', message => { if (message.type() === 'error') errors.push(`${message.text()} ${message.location().url}`); });
  155 |   page.on('pageerror', error => errors.push(error.message));
  156 |   page.on('requestfailed', request => failed.push(request.url()));
  157 |   page.on('response', response => { if (response.status() >= 400) failed.push(`${response.status()} ${response.url()}`); });
  158 |   await ready(page);
  159 |   await page.exposeFunction('recordSourceViolation', (directive: string) => violations.push(directive));
  160 |   await page.evaluate(() => {
  161 |     document.addEventListener('securitypolicyviolation', event => (window as any).recordSourceViolation(event.violatedDirective));
  162 |     const api = (window as any).__utility_lab;
  163 |     api.dispose('scope-a');
  164 |     api.mount('scope-a', 10000, '@source "./virtual/**/*.rs"; @source not "./virtual/private/**"; @media theme(inline) prefix(tw) important source(none) { @theme { --spacing: 7px; --color-brand: #123456; } @source inline("tw:w-[{137,159}px] tw:h-[72px] tw:bg-brand tw:p-1"); @source not inline("tw:w-[159px]"); @tailwind utilities; }');
  165 |   });
  166 |   await expect(page.locator('#gpu-status')).toHaveText('GPU ready');
  167 |   const revision = await apply(page, ['tw:w-[137px]', 'tw:h-[72px]', 'tw:bg-brand', 'tw:p-1']);
  168 |   const state = await drawn(page, revision);
  169 |   expect(state.button.status).toBe('applied');
```