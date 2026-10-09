# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: utility-gpu.spec.ts >> clearing utility ownership restores the latest palette without another baseline setter
- Location: ../../../../../../../Users/rockie/Documents/gh-rockie/rustify-ui/tests/browser/utility-gpu.spec.ts:155:7

# Error details

```
Error: expect(received).toEqual(expected) // deep equality

- Expected  - 1
+ Received  + 1

  Array [
-   18,
    52,
    86,
+   255,
    255,
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
  76  |   page.on('requestfailed', request => failed.push(request.url()));
  77  |   page.on('response', response => { if (response.status() >= 400) failed.push(`${response.status()} ${response.url()}`); });
  78  |   await ready(page);
  79  |   await page.evaluate(() => {
  80  |     const container = document.createElement('div');
  81  |     container.id = 'button-baseline-root';
  82  |     document.querySelector('main')!.append(container);
  83  |     const sheet = new CSSStyleSheet();
  84  |     sheet.replaceSync('#button-baseline-root canvas { display:block; width:300px; height:240px; }');
  85  |     document.adoptedStyleSheets = [...document.adoptedStyleSheets, sheet];
  86  |     (window as any).__utility_lab.buttonBaseline.mount(container.id);
  87  |   });
  88  |   const canvas = page.getByTestId('button-baseline');
  89  |   await canvas.scrollIntoViewIfNeeded();
  90  |   await page.mouse.move(0, 0);
  91  |   const baselineReport = () => page.evaluate(() => (window as any).__utility_lab.buttonBaseline.report());
  92  |   await expect.poll(async () => (await baselineReport())?.buttons?.[1]?.geometry?.[2]).toBe(220);
  93  |   const compare = async (phase: number, generation = 0) => {
  94  |     await expect.poll(async () => (await baselineReport())?.generation).toBe(generation);
  95  |     await expect.poll(async () => (await baselineReport())?.phase).toBe(phase);
  96  |     const state = await baselineReport();
  97  |     expect(state.buttons.map((button: any) => button.utility)).toEqual([null, null]);
  98  |     expect(state.buttons[0].geometry.slice(2)).toEqual(state.buttons[1].geometry.slice(2));
  99  |     // Both controls use the same fonts/shaders in one canvas. Compare every
  100 |     // pixel, including text, corner coverage, border and disabled alpha.
  101 |     const capture = async () => {
  102 |       const image = PNG.sync.read(await canvas.screenshot());
  103 |       return state.buttons.map((button: any) => {
  104 |         const [x, y, width, height] = button.geometry;
  105 |         const pixels: number[] = [];
  106 |         for (let row = 0; row < height; row++) for (let col = 0; col < width; col++) pixels.push(...pixel(image, x + col, y + row));
  107 |         return pixels;
  108 |       });
  109 |     };
  110 |     await expect.poll(async () => { const [ordinary, cleared] = await capture(); return ordinary.filter((value: number, index: number) => value !== cleared[index]).length; }).toBe(0);
  111 |     return state;
  112 |   };
  113 |   const initial = await compare(0);
  114 |   let clearedGeneration = 0;
  115 |   for (const [dark, rem, palette, disabled] of [[false, 16, false, false], [true, 20, false, false], [false, 16, true, false], [true, 20, false, true]] as const) {
  116 |     await page.evaluate(({dark, rem, palette, disabled}) => (window as any).__utility_lab.buttonBaseline.update(1, dark, rem, palette, disabled), {dark, rem, palette, disabled});
  117 |     await expect.poll(async () => (await baselineReport())?.buttons?.[1]?.utility?.drawn_revision).toBe(1);
  118 |     expect((await baselineReport()).buttons[0].utility).toBeNull();
  119 |     clearedGeneration = await page.evaluate(({dark, rem, palette, disabled}) => (window as any).__utility_lab.buttonBaseline.update(2, dark, rem, palette, disabled), {dark, rem, palette, disabled});
  120 |     await compare(2, clearedGeneration);
  121 |   }
  122 |   clearedGeneration = await page.evaluate(() => (window as any).__utility_lab.buttonBaseline.update(2));
  123 |   const restored = await compare(2, clearedGeneration);
  124 |   const box = (await canvas.boundingBox())!;
  125 |   const focusedPixels = async (geometry: number[]) => {
  126 |     const image = PNG.sync.read(await canvas.screenshot());
  127 |     const [x, y, width, height] = geometry;
  128 |     const pixels: number[] = [];
  129 |     for (let row = -4; row < height + 4; row++) for (let col = -4; col < width + 4; col++) pixels.push(...pixel(image, x + col, y + row));
  130 |     return pixels;
  131 |   };
  132 |   let ordinaryFocused: number[] = [];
  133 |   for (const [index, button] of restored.buttons.entries()) {
  134 |     const [x, y, width, height] = button.geometry;
  135 |     await page.mouse.click(box.x + x + width / 2, box.y + y + height / 2);
  136 |     await page.mouse.move(0, 0);
  137 |     await expect.poll(async () => (await baselineReport()).clicks[index]).toBe(1);
  138 |     if (index === 0) ordinaryFocused = await focusedPixels(button.geometry);
  139 |     else await expect.poll(async () => {
  140 |       const clearedFocused = await focusedPixels(button.geometry);
  141 |       return ordinaryFocused.filter((value, index) => value !== clearedFocused[index]).length;
  142 |     }).toBe(0);
  143 |   }
  144 |   expect(errors).toEqual([]); expect(failed).toEqual([]);
  145 |   await info.attach('Ordinary and cleared Button reports', {body: JSON.stringify({initial, restored, clicked: await baselineReport()}, null, 2), contentType: 'application/json'});
  146 |   await canvas.screenshot({path: info.outputPath('button-none.png')});
  147 |   await page.evaluate(() => (window as any).__utility_lab.buttonBaseline.dispose());
  148 |   await expect(canvas).toHaveCount(0);
  149 | });
  150 | 
  151 | for (const {name, dark, rem, palette, height, color} of [
  152 |   {name: 'theme', dark: true, rem: 20, palette: false, height: 45, color: [83, 177, 253, 255]},
  153 |   {name: 'palette', dark: false, rem: 16, palette: true, height: 36, color: [18, 52, 86, 255]},
  154 | ]) {
  155 |   test(`clearing utility ownership restores the latest ${name} without another baseline setter`, async ({page}, info) => {
  156 |     const fixture = await mountButtonBaseline(page);
  157 |     const initial = await fixture.capture();
  158 |     const appliedGeneration = await page.evaluate(() => (window as any).__utility_lab.buttonBaseline.update(1));
  159 |     await fixture.wait(appliedGeneration, 1);
  160 |     await expect.poll(async () => (await fixture.read()).buttons[1].utility?.drawn_revision).toBe(1);
  161 |     const applied = await fixture.capture();
  162 |     expect(applied.state.buttons[1].utility.status).toBe('Applied');
  163 |     expect(applied.state.buttons[1].geometry[3]).toBe(64);
  164 |     const [ax, ay, aw, ah] = applied.state.buttons[1].geometry;
  165 |     expect(pixel(applied.image, ax + aw / 2, ay + ah - 8)).toEqual([254, 220, 186, 255]);
  166 | 
  167 |     // Refresh the ordinary baseline while the same utility still owns the Button.
  168 |     const heldGeneration = await page.evaluate(({dark, rem, palette}) => (window as any).__utility_lab.buttonBaseline.update(3, dark, rem, palette), {dark, rem, palette});
  169 |     await fixture.wait(heldGeneration, 3);
  170 |     const held = await fixture.capture();
  171 |     expect(held.state.buttons[1].utility).toEqual({status: 'Applied', drawn_revision: 1});
  172 |     expect(held.state.buttons[0].geometry[3]).toBe(height);
  173 |     expect(held.state.baseline.font_size).toBe(21);
  174 |     expect(held.state.baseline.tracking).toBe(.04);
  175 |     const [hx, hy, hw, hh] = held.state.buttons[0].geometry;
> 176 |     expect(pixel(held.image, hx + hw / 2, hy + hh - 8)).toEqual(color);
      |                                                         ^ Error: expect(received).toEqual(expected) // deep equality
  177 |     expect(held.pixels[0]).not.toEqual(initial.pixels[0]);
  178 | 
  179 |     const clearedGeneration = await page.evaluate(() => (window as any).__utility_lab.buttonBaseline.update(4));
  180 |     await fixture.wait(clearedGeneration, 4);
  181 |     const clearedState = await fixture.read();
  182 |     expect(clearedState.baseline).toEqual(held.state.baseline);
  183 |     expect(clearedState.buttons.map((button: any) => button.utility)).toEqual([null, null]);
  184 |     expect(clearedState.buttons[0].geometry.slice(2)).toEqual(clearedState.buttons[1].geometry.slice(2));
  185 |     await expect.poll(async () => {
  186 |       const {pixels} = await fixture.capture();
  187 |       return pixels[0].filter((value: number, index: number) => value !== pixels[1][index]).length;
  188 |     }).toBe(0);
  189 |     const cleared = await fixture.capture();
  190 |     expect(cleared.pixels[0]).toEqual(held.pixels[0]);
  191 | 
  192 |     const updatedGeneration = await page.evaluate(() => (window as any).__utility_lab.buttonBaseline.update(0));
  193 |     await fixture.wait(updatedGeneration, 0);
  194 |     const updatedState = await fixture.read();
  195 |     expect(updatedState.buttons.map((button: any) => button.utility)).toEqual([null, null]);
  196 |     expect(updatedState.buttons[0].geometry.slice(2)).toEqual(updatedState.buttons[1].geometry.slice(2));
  197 |     await expect.poll(async () => {
  198 |       const {pixels} = await fixture.capture();
  199 |       return pixels[0].filter((value: number, index: number) => value !== pixels[1][index]).length;
  200 |     }).toBe(0);
  201 |     const updated = await fixture.capture();
  202 |     expect(updated.pixels[0]).not.toEqual(cleared.pixels[0]);
  203 |     expect(updated.state.baseline.font_size).toBe(15);
  204 |     await info.attach(`Button ${name} baseline lifecycle`, {body: JSON.stringify({initial: initial.state, applied: applied.state, held: held.state, cleared: cleared.state, updated: updated.state}), contentType: 'application/json'});
  205 |     await fixture.canvas.screenshot({path: info.outputPath(`button-none-live-${name}.png`)});
  206 |     expect(fixture.errors).toEqual([]); expect(fixture.failed).toEqual([]);
  207 |     await page.evaluate(() => (window as any).__utility_lab.buttonBaseline.dispose());
  208 |     await expect(fixture.canvas).toHaveCount(0);
  209 |   });
  210 | }
  211 | 
  212 | test('clearing utility ownership restores untouched Script Button defaults', async ({page}, info) => {
  213 |   const fixture = await mountButtonBaseline(page, false);
  214 |   await expect.poll(async () => {
  215 |     const {state, image} = await fixture.capture();
  216 |     const [x, y, width, height] = state.buttons[0].geometry;
  217 |     let glyphPixels = 0;
  218 |     for (let row = 8; row < height - 8; row++) for (let col = 16; col < width - 16; col++) {
  219 |       const color = pixel(image, x + col, y + row);
  220 |       if (color.slice(0, 3).every(value => value < 100)) glyphPixels++;
  221 |     }
  222 |     return glyphPixels;
  223 |   }).toBeGreaterThan(0);
  224 |   const initial = await fixture.capture();
  225 |   expect(initial.state.baseline.themed).toBe(false);
  226 |   expect(initial.state.buttons.map((button: any) => button.utility)).toEqual([null, null]);
  227 |   expect(initial.pixels[0]).toEqual(initial.pixels[1]);
  228 | 
  229 |   const appliedGeneration = await page.evaluate(() => (window as any).__utility_lab.buttonBaseline.update(1));
  230 |   await fixture.wait(appliedGeneration, 1);
  231 |   await expect.poll(async () => (await fixture.read()).buttons[1].utility?.drawn_revision).toBe(1);
  232 |   const applied = await fixture.capture();
  233 |   expect(applied.state.buttons[1].utility.status).toBe('Applied');
  234 |   expect(applied.state.buttons[1].geometry[3]).toBe(64);
  235 |   const [x, y, width, height] = applied.state.buttons[1].geometry;
  236 |   expect(pixel(applied.image, x + width / 2, y + height - 8)).toEqual([254, 220, 186, 255]);
  237 | 
  238 |   const clearedGeneration = await page.evaluate(() => (window as any).__utility_lab.buttonBaseline.update(4));
  239 |   await fixture.wait(clearedGeneration, 4);
  240 |   const clearedState = await fixture.read();
  241 |   expect(clearedState.baseline).toEqual(initial.state.baseline);
  242 |   expect(clearedState.buttons.map((button: any) => button.utility)).toEqual([null, null]);
  243 |   expect(clearedState.buttons[0].geometry.slice(2)).toEqual(clearedState.buttons[1].geometry.slice(2));
  244 |   await expect.poll(async () => {
  245 |     const {pixels} = await fixture.capture();
  246 |     return pixels[0].filter((value: number, index: number) => value !== pixels[1][index]).length;
  247 |   }).toBe(0);
  248 |   const cleared = await fixture.capture();
  249 |   expect(cleared.pixels[0]).toEqual(initial.pixels[0]);
  250 |   expect(cleared.pixels[1]).toEqual(initial.pixels[0]);
  251 |   await info.attach('Unthemed Script Button lifecycle', {body: JSON.stringify({initial: initial.state, applied: applied.state, cleared: cleared.state}), contentType: 'application/json'});
  252 |   await fixture.canvas.screenshot({path: info.outputPath('button-none-script.png')});
  253 |   expect(fixture.errors).toEqual([]); expect(fixture.failed).toEqual([]);
  254 |   await page.evaluate(() => (window as any).__utility_lab.buttonBaseline.dispose());
  255 |   await expect(fixture.canvas).toHaveCount(0);
  256 | });
  257 | 
  258 | test('GPU percentages use the parent content box and follow its real resize', async ({page}, info) => {
  259 |   const errors: string[] = [], failed: string[] = [];
  260 |   page.on('pageerror', error => errors.push(error.message));
  261 |   page.on('requestfailed', request => failed.push(request.url()));
  262 |   await page.setViewportSize({width: 1000, height: 900});
  263 |   await ready(page);
  264 |   const container = ['w-[300px]', 'h-[220px]', 'p-[10px]', 'border-2', 'flex-col', 'items-start', 'justify-start', 'gap-[0px]'];
  265 |   const button = ['w-[50%]', 'h-[50%]', 'p-[10%]', 'min-w-[100px]', 'max-w-[120px]', 'bg-[#123456]'];
  266 |   const compilation = await page.evaluate(async (classes) => (window as any).__utility_lab.ensure('scope-a', classes), [...container, ...button, 'w-[60px]', 'h-[20px]', 'w-[69px]', 'h-[69px]']);
  267 |   await info.attach('Parent geometry compiler input', {body: JSON.stringify(compilation, null, 2), contentType: 'application/json'});
  268 |   expect(compilation.unsupported).toEqual([]);
  269 |   expect(compilation.rejected.filter((item: any) => [...container, ...button].includes(item.candidate))).toEqual([]);
  270 |   const revision = await apply(page, button, [container, ['w-[60px]', 'h-[20px]'], ['w-[69px]', 'h-[69px]']]);
  271 |   const fixed = await drawn(page, revision);
  272 |   expect(fixed.container.geometry.slice(2)).toEqual([300, 220]);
  273 |   expect(fixed.button.status).toBe('applied');
  274 |   expect(fixed.button.geometry.slice(2)).toEqual([120, 98]);
  275 |   fixed.button.computed.padding.forEach((value: number) => expect(value).toBeCloseTo(27.6, 5));
  276 |   const sample = page.locator('#scope-a-sample');
```