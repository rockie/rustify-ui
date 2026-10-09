# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: utility-gpu.spec.ts >> clearing utility ownership restores untouched Script Button defaults
- Location: ../../../../../../../Users/rockie/Documents/gh-rockie/rustify-ui/tests/browser/utility-gpu.spec.ts:212:5

# Error details

```
Error: expect(received).toEqual(expected) // deep equality

- Expected  - 8
+ Received  + 8

@@ -15797,23 +15797,23 @@
    255,
    77,
    87,
    100,
    255,
-   145,
+   144,
    151,
    160,
    255,
-   145,
+   144,
    151,
    160,
    255,
-   145,
+   144,
    151,
    160,
    255,
-   145,
+   144,
    151,
    160,
    255,
    31,
    43,
@@ -16297,19 +16297,19 @@
    255,
    110,
    118,
    129,
    255,
-   145,
+   144,
    151,
    160,
    255,
-   145,
+   144,
    151,
    160,
    255,
-   145,
+   144,
    151,
    160,
    255,
    160,
    165,
@@ -27015,11 +27015,11 @@
    224,
    230,
    255,
    209,
    214,
-   222,
+   221,
    255,
    233,
    235,
    239,
    255,
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
  176 |     expect(pixel(held.image, hx + hw / 2, hy + hh - 8)).toEqual(color);
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
> 227 |   expect(initial.pixels[0]).toEqual(initial.pixels[1]);
      |                             ^ Error: expect(received).toEqual(expected) // deep equality
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
  277 |   await expect(sample).toHaveCSS('width', '120px');
  278 |   await expect(sample).toHaveCSS('height', '98px');
  279 |   const domPadding = await sample.evaluate(element => {
  280 |     const style = getComputedStyle(element);
  281 |     return [parseFloat(style.paddingTop), parseFloat(style.paddingLeft)];
  282 |   });
  283 |   domPadding.forEach(value => expect(Math.abs(value - 27.6)).toBeLessThan(1 / 64));
  284 |   const fixedImage = PNG.sync.read(await page.getByTestId('utility-gpu').screenshot());
  285 |   expect(pixel(fixedImage, fixed.button.geometry[0] + 4, fixed.button.geometry[1] + 4).slice(0, 3)).toEqual([18, 52, 86]);
  286 | 
  287 |   const responsive = await apply(page, ['w-[50%]', 'h-[50px]', 'p-[0px]'], [
  288 |     container.filter(value => value !== 'w-[300px]').concat('w-[50%]'),
  289 |     ['w-[20px]', 'h-[20px]'], ['w-[40px]', 'h-[40px]'],
  290 |   ]);
  291 |   const before = await drawn(page, responsive);
  292 |   const compiled = await page.evaluate(() => (window as any).__utility_lab.report('scope-a'));
  293 |   const relative = (state: any) => {
  294 |     expect(state.button.status).toBe('applied');
  295 |     expect(state.button.geometry[2]).toBeCloseTo((state.container.geometry[2] - 24) / 2, 5);
  296 |     expect(state.button.geometry[3]).toBe(50);
  297 |   };
  298 |   relative(before);
  299 |   await page.setViewportSize({width: 800, height: 900});
  300 |   await expect.poll(async () => (await report(page)).container.geometry[2]).not.toBe(before.container.geometry[2]);
  301 |   const resized = await report(page);
  302 |   relative(resized);
  303 |   expect(resized.button.drawn_revision).toBe(responsive);
  304 |   const after = await page.evaluate(() => (window as any).__utility_lab.report('scope-a'));
  305 |   expect(after.revision).toBe(compiled.revision);
  306 |   expect(after.metrics.parsed_candidates).toBe(compiled.metrics.parsed_candidates);
  307 |   expect(errors).toEqual([]); expect(failed).toEqual([]);
  308 |   await info.attach('Measured parent geometry', {body: JSON.stringify({fixed, before, resized}, null, 2), contentType: 'application/json'});
  309 |   await page.screenshot({path: info.outputPath('parent-content.png'), fullPage: true});
  310 | });
  311 | 
  312 | test('inline and path sources with media theme options drive scoped DOM and GPU', async ({page}, info) => {
  313 |   const errors: string[] = [], failed: string[] = [], violations: string[] = [];
  314 |   page.on('console', message => { if (message.type() === 'error') errors.push(`${message.text()} ${message.location().url}`); });
  315 |   page.on('pageerror', error => errors.push(error.message));
  316 |   page.on('requestfailed', request => failed.push(request.url()));
  317 |   page.on('response', response => { if (response.status() >= 400) failed.push(`${response.status()} ${response.url()}`); });
  318 |   await ready(page);
  319 |   await page.exposeFunction('recordSourceViolation', (directive: string) => violations.push(directive));
  320 |   await page.evaluate(() => {
  321 |     document.addEventListener('securitypolicyviolation', event => (window as any).recordSourceViolation(event.violatedDirective));
  322 |     const api = (window as any).__utility_lab;
  323 |     api.dispose('scope-a');
  324 |     api.mount('scope-a', 10000, '@source "./virtual/**/*.rs"; @source not "./virtual/private/**"; @media theme(inline) prefix(tw) important source(none) { @theme { --spacing: 7px; --color-brand: #123456; } @source inline("tw:w-[{137,159}px] tw:h-[72px] tw:bg-brand tw:p-1"); @source not inline("tw:w-[159px]"); @tailwind utilities; }');
  325 |   });
  326 |   await expect(page.locator('#gpu-status')).toHaveText('GPU ready');
  327 |   const revision = await apply(page, ['tw:w-[137px]', 'tw:h-[72px]', 'tw:bg-brand', 'tw:p-1']);
```