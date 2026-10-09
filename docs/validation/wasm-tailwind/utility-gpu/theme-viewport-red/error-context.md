# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: utility-gpu.spec.ts >> live ThemeScope and ThemeBoundary update colors, dark conditions and host rem without recompiling
- Location: tests/browser/utility-gpu.spec.ts:220:5

# Error details

```
Error: expect(received).toBe(expected) // Object.is equality

Expected: 73
Received: 45
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
  141 |   await ready(page);
  142 |   const revision = await apply(page, ['w-[100px]', 'h-[60px]', 'bg-[#123456]', 'ring-[3px]', 'ring-[#abcdef]'],
  143 |     [['flex', 'flex-row', 'gap-[11px]', 'p-[9px]', 'items-center'], ["font-['Liberation_Mono']", 'text-[20px]', 'leading-[28px]', 'tracking-[0.05em]'], []]);
  144 |   const state = await drawn(page, revision, 'text');
  145 |   expect(state.text.status).toBe('applied');
  146 |   expect(state.text.computed.font.family).toBe('Liberation Mono');
  147 |   expect(state.text.computed.font_size_px).toBe(20);
  148 |   expect(state.text.computed.line_height.value).toBe(28);
  149 |   expect(state.text.computed.letter_spacing_em).toBe(.05);
  150 |   expect(state.container.computed.gap).toEqual([11, 11]);
  151 |   expect(state.container.computed.flow).toBe('row');
  152 |   expect(state.text.geometry[0] - state.button.geometry[0] - state.button.geometry[2]).toBeCloseTo(11);
  153 |   expect(state.swatch.geometry[0] - state.text.geometry[0] - state.text.geometry[2]).toBeCloseTo(11);
  154 |   expect(state.button.computed.shadows.some((shadow: any) => shadow.spread === 3)).toBe(true);
  155 |   await expect(page.locator('#scope-a-text')).toHaveCSS('font-family', '"Liberation Mono"');
  156 |   await expect(page.locator('#scope-a-text')).toHaveCSS('font-size', '20px');
  157 |   const png = PNG.sync.read(await page.getByTestId('utility-gpu').screenshot());
  158 |   const [x, y, w] = state.button.geometry;
  159 |   const ring = pixel(png, x + w / 2 + 1, y - 1);
  160 |   [171, 205, 239].forEach((value, index) => expect(Math.abs(ring[index] - value)).toBeLessThanOrEqual(3));
  161 | });
  162 | 
  163 | test('GPU restores its latest committed programs after a real WebGL context loss', async ({page}) => {
  164 |   await ready(page);
  165 |   const first = await apply(page, ['w-[137px]', 'bg-[#123456]']);
  166 |   await drawn(page, first);
  167 |   await page.getByTestId('utility-gpu').evaluate((canvas: HTMLCanvasElement) => {
  168 |     const gl = canvas.getContext('webgl2')!;
  169 |     const extension = gl.getExtension('WEBGL_lose_context')!;
  170 |     (window as any).__utility_loss = extension; extension.loseContext();
  171 |   });
  172 |   await expect(page.locator('#gpu-status')).toHaveText('Restoring GPU…');
  173 |   const next = await page.evaluate(async () => (await (window as any).__utility_lab.gpuApply('scope-a', [[], ['w-[179px]', 'bg-[#abcdef]'], [], []])).revision);
  174 |   await page.evaluate(() => { (window as any).__utility_loss.restoreContext(); delete (window as any).__utility_loss; });
  175 |   await expect(page.locator('#gpu-status')).toHaveText('GPU ready');
  176 |   const state = await drawn(page, next);
  177 |   expect(state.button.computed.width.value).toBe(179);
  178 |   expect(state.button.computed.background.slice(0, 3)).toEqual([171/255, 205/255, 239/255]);
  179 |   expect(await page.getByTestId('utility-gpu').count()).toBe(1);
  180 | });
  181 | 
  182 | test('terminal GPU initialization failure explains recovery while DOM editing remains usable', async ({page}, info) => {
  183 |   await page.addInitScript(() => {
  184 |     const original = HTMLCanvasElement.prototype.getContext;
  185 |     HTMLCanvasElement.prototype.getContext = function(type: any, ...args: any[]) {
  186 |       if (type === 'webgl2') return null;
  187 |       return original.apply(this, [type, ...args] as any);
  188 |     } as any;
  189 |   });
  190 |   await page.goto('./');
  191 |   await expect(page.locator('#status')).toHaveAttribute('data-status', 'ready');
  192 |   await expect(page.locator('#gpu-status')).toContainText('GPU unavailable: no WebGL2 context');
  193 |   await expect(page.locator('#gpu-status')).toContainText('Copy your class inputs, then reload');
  194 |   await page.locator('#classes').fill('w-[179px] bg-[#abcdef]');
  195 |   await page.locator('#apply').click();
  196 |   await expect(page.locator('#status')).toContainText('DOM styles installed.');
  197 |   await expect(page.locator('#scope-a-sample')).toHaveCSS('width', '179px');
  198 |   await page.screenshot({path: info.outputPath('gpu-terminal-failure.png'), fullPage: true});
  199 | });
  200 | 
  201 | test('all non-button GPU boundaries are visible and the desktop comparison frames align', async ({page}, info) => {
  202 |   await ready(page);
  203 |   await apply(page, ['w-[137px]', 'bg-[#123456]'], [['opacity-50'], ['bg-[#abcdef]', 'opacity-50'], ['grid']]);
  204 |   await expect(page.locator('#gpu-results')).toContainText('Container: rejected');
  205 |   await expect(page.locator('#gpu-results')).toContainText('Text: rejected');
  206 |   await expect(page.locator('#gpu-results')).toContainText('Color block: rejected');
  207 |   await expect(page.locator('#gpu-warning')).toContainText('opacity-50 (opacity)');
  208 |   await expect(page.locator('#gpu-warning')).toContainText('grid (display)');
  209 |   await expect(page.locator('#gpu-warning')).toContainText('Remove the class');
  210 |   const dom = (await page.locator('#scope-a.preview').boundingBox())!;
  211 |   const gpu = (await page.getByTestId('utility-gpu').boundingBox())!;
  212 |   expect(Math.abs(dom.y - gpu.y)).toBeLessThanOrEqual(1);
  213 |   await page.screenshot({path: info.outputPath('gpu-boundaries.png'), fullPage: true});
  214 |   await apply(page, ['w-[137px]', 'bg-[#123456]'], [['opacity-50'], ['bg-[#abcdef]', 'opacity-50'], ['grid']], true);
  215 |   await expect(page.locator('#gpu-results')).toContainText('Container: partial');
  216 |   await expect(page.locator('#gpu-results')).toContainText('Text: partial');
  217 |   await expect(page.locator('#gpu-results')).toContainText('Color block: partial');
  218 | });
  219 | 
  220 | test('live ThemeScope and ThemeBoundary update colors, dark conditions and host rem without recompiling', async ({page}, info) => {
  221 |   await page.setViewportSize({width: 1000, height: 900});
  222 |   await ready(page);
  223 |   const baseline = (await report(page)).button.computed;
  224 |   const revision = await apply(page, ['w-[5rem]', 'bg-(--primary)', 'text-(--primary-foreground)', 'dark:border-2', 'dark:border-[#123456]', 'min-[60rem]:h-[73px]'], [['gap-[1rem]'], ['text-(--foreground)'], ['bg-(--primary)']]);
  225 |   await drawn(page, revision);
  226 |   await expect(page.locator('#scope-a-sample')).toHaveCSS('width', '80px');
  227 |   expect((await report(page)).button.computed.height.value).toBe(73);
  228 |   const installed = await page.evaluate(() => (window as any).__utility_lab.report('scope-a'));
  229 |   const sibling = await page.locator('#scope-b-sample').evaluate(node => getComputedStyle(node).backgroundColor);
  230 |   await page.emulateMedia({colorScheme: 'dark'});
  231 |   await page.evaluate(() => {
  232 |     document.documentElement.style.fontSize = '20px';
  233 |     (window as any).__utility_lab.theme('scope-a', true, 20, {primary: '#abcdef'});
  234 |   });
  235 |   await expect(page.locator('#scope-a')).toHaveAttribute('data-theme', 'dark');
  236 |   await expect(page.getByTestId('utility-theme-boundary')).toHaveCSS('--primary', 'rgb(171 205 239)');
  237 |   await expect(page.locator('#scope-a-sample')).toHaveCSS('width', '100px');
  238 |   await expect(page.locator('#scope-a-sample')).toHaveCSS('background-color', 'rgb(171, 205, 239)');
  239 |   await expect.poll(async () => (await report(page)).button.computed.background).toEqual([171 / 255, 205 / 255, 239 / 255, 1]);
  240 |   await expect.poll(async () => (await report(page)).button.computed.width.value).toBe(100);
> 241 |   expect((await report(page)).button.computed.height.value).toBe(73);
      |                                                             ^ Error: expect(received).toBe(expected) // Object.is equality
  242 |   expect((await report(page)).button.computed.border_widths).toEqual([2, 2, 2, 2]);
  243 |   expect((await report(page)).container.computed.gap).toEqual([20, 20]);
  244 |   await expect(page.locator('#scope-b-sample')).toHaveCSS('background-color', sibling);
  245 |   expect(await page.evaluate(() => (window as any).__utility_lab.report('scope-a'))).toEqual(installed);
  246 |   const retained = (await report(page)).button.computed;
  247 |   const rejected = await page.evaluate(() => {
  248 |     try { (window as any).__utility_lab.theme('scope-a', false, 16, {radius: 'invalid'}); return false; }
  249 |     catch { return true; }
  250 |   });
  251 |   expect(rejected).toBe(true);
  252 |   expect((await report(page)).button.computed).toEqual(retained);
  253 |   await page.screenshot({path: info.outputPath('gpu-theme-boundary.png'), fullPage: true});
  254 |   await page.evaluate(() => (window as any).__utility_lab.theme('scope-a', true, 20));
  255 |   await expect(page.getByTestId('utility-theme-boundary')).not.toHaveAttribute('data-rustify-theme-runtime');
  256 |   await expect.poll(async () => (await report(page)).button.computed.background).not.toEqual(retained.background);
  257 |   await page.emulateMedia({colorScheme: 'light'});
  258 |   await page.evaluate(() => {
  259 |     document.documentElement.style.fontSize = '';
  260 |     (window as any).__utility_lab.theme('scope-a', false, 16);
  261 |   });
  262 |   await expect(page.locator('#scope-a')).toHaveAttribute('data-theme', 'light');
  263 |   const restored = await apply(page, []);
  264 |   await drawn(page, restored);
  265 |   expect((await report(page)).button.computed.background).toEqual(baseline.background);
  266 |   expect((await report(page)).button.computed.padding).toEqual(baseline.padding);
  267 | });
  268 | 
```