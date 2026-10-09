# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: utility-gpu.spec.ts >> GPU per-side spacing, size, min/max and column alignment place boxes where the DOM does
- Location: tests/browser/utility-gpu.spec.ts:804:5

# Error details

```
Error: expect(received).toBe(expected) // Object.is equality

Expected: "rejected"
Received: "applied"
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
        - paragraph [ref=e41]: "Button: applied · applied 2 · drawn 2 · 0 clicks"
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
  738 |   expect(wrapped.button.text_rows).toHaveLength(2);
  739 |   sameLines((measured.button as any).dom, (measured.button as any).gpu);
  740 | 
  741 |   const wide = await apply(page, ['w-[220px]', 'h-[72px]', 'p-[7px]', 'bg-[#123456]', 'text-[#ffffff]']);
  742 |   const single = await drawn(page, wide);
  743 |   measured.single = {dom: await domLines(page, 'scope-a-sample'), gpu: gpuLines(single.button)};
  744 |   expect(single.button.text_rows).toHaveLength(1);
  745 |   sameLines((measured.single as any).dom, (measured.single as any).gpu);
  746 |   await page.getByTestId('utility-gpu').screenshot({path: info.outputPath('wrapped-button.png')});
  747 | 
  748 |   const tracked = await apply(page, ['w-[220px]', 'h-[72px]'], [[], ["font-['Liberation_Mono']", 'text-[20px]', 'leading-[28px]', 'tracking-[0.05em]', 'w-[100px]'], []]);
  749 |   const text = await drawn(page, tracked, 'text');
  750 |   measured.text = {dom: await domLines(page, 'scope-a-text'), gpu: gpuLines(text.text)};
  751 |   expect(text.text.status).toBe('applied');
  752 |   expect(text.text.text_rows).toHaveLength(2);
  753 |   sameLines((measured.text as any).dom, (measured.text as any).gpu);
  754 |   const gpuText = gpuLines(text.text);
  755 |   expect(gpuText[1].centre - gpuText[0].centre).toBeCloseTo(28, 5);
  756 |   const domHeight = await page.locator('#scope-a-text').evaluate(element => element.getBoundingClientRect().height);
  757 |   expect(text.text.geometry[3]).toBeCloseTo(domHeight, 5);
  758 |   await info.attach('DOM and GPU line boxes', {body: JSON.stringify(measured, null, 2), contentType: 'application/json'});
  759 |   await page.getByTestId('utility-gpu').screenshot({path: info.outputPath('wrapped-text.png')});
  760 |   await page.locator('#scope-a-frame').screenshot({path: info.outputPath('wrapped-text-dom.png')});
  761 |   expect(errors).toEqual([]); expect(failed).toEqual([]);
  762 | });
  763 | 
  764 | test('GPU ring offset, ring and shadow layers paint the same bands as the DOM', async ({page}, info) => {
  765 |   const errors: string[] = [], failed: string[] = [];
  766 |   page.on('pageerror', error => errors.push(error.message));
  767 |   page.on('requestfailed', request => failed.push(request.url()));
  768 |   await page.setViewportSize({width: 1280, height: 1000});
  769 |   await ready(page);
  770 |   const revision = await apply(page, ['w-[120px]', 'h-[48px]', 'bg-[#123456]', 'ring-[3px]', 'ring-[#abcdef]',
  771 |     'ring-offset-[4px]', 'ring-offset-[#fedcba]', 'shadow-[0_12px_0_0_#234567]']);
  772 |   const state = await drawn(page, revision);
  773 |   expect(state.button.status).toBe('applied');
  774 |   const gpu = PNG.sync.read(await page.getByTestId('utility-gpu').screenshot());
  775 |   const sample = (await page.locator('#scope-a-sample').boundingBox())!;
  776 |   const dom = PNG.sync.read(await page.screenshot({clip: {x: sample.x - 20, y: sample.y - 20, width: sample.width + 40, height: sample.height + 40}}));
  777 |   const [x, y, w, h] = state.button.geometry;
  778 |   // Offsets from the border box at its horizontal centre, away from corners.
  779 |   const bands: [string, number, number[]][] = [
  780 |     ['face', 24, [18, 52, 86]],
  781 |     ['offset above', -2, [254, 220, 186]],
  782 |     ['ring above', -5.5, [171, 205, 239]],
  783 |     ['clear above', -10, [255, 255, 255]],
  784 |     ['offset below', h + 2, [254, 220, 186]],
  785 |     ['ring below', h + 5.5, [171, 205, 239]],
  786 |     ['shadow below', h + 9.5, [35, 69, 103]],
  787 |     ['clear below', h + 14, [255, 255, 255]],
  788 |   ];
  789 |   const measured = bands.map(([name, offset, expected]) => {
  790 |     const fromGpu = pixel(gpu, x + w / 2 + 1, y + offset).slice(0, 3);
  791 |     const fromDom = pixel(dom, 20 + sample.width / 2 + 1, 20 + offset).slice(0, 3);
  792 |     expected.forEach((value, index) => {
  793 |       expect(Math.abs(fromGpu[index] - value), `GPU ${name} ${fromGpu}`).toBeLessThanOrEqual(3);
  794 |       expect(Math.abs(fromDom[index] - value), `DOM ${name} ${fromDom}`).toBeLessThanOrEqual(3);
  795 |     });
  796 |     return {name, offset, gpu: fromGpu, dom: fromDom};
  797 |   });
  798 |   await info.attach('Shadow band pixels', {body: JSON.stringify(measured, null, 2), contentType: 'application/json'});
  799 |   await page.getByTestId('utility-gpu').screenshot({path: info.outputPath('shadow-bands-gpu.png')});
  800 |   await page.screenshot({path: info.outputPath('shadow-bands-dom.png'), clip: {x: sample.x - 20, y: sample.y - 20, width: sample.width + 40, height: sample.height + 40}});
  801 |   expect(errors).toEqual([]); expect(failed).toEqual([]);
  802 | });
  803 | 
  804 | test('GPU per-side spacing, size, min/max and column alignment place boxes where the DOM does', async ({page}, info) => {
  805 |   await page.setViewportSize({width: 1280, height: 1000});
  806 |   await ready(page);
  807 |   const container = ['w-[300px]', 'h-[240px]', 'flex-col', 'items-center', 'justify-end', 'pt-[10px]', 'pr-[20px]', 'pb-[30px]', 'pl-[40px]', 'gap-[6px]'];
  808 |   const revision = await apply(page, ['size-[64px]', 'bg-[#123456]'], [container, ['px-[3px]', 'py-[5px]'], ['w-[50px]', 'max-w-[30px]', 'h-[4px]', 'min-h-[10px]']]);
  809 |   const state = await drawn(page, revision);
  810 |   for (const slot of ['container', 'button', 'text', 'swatch']) expect(state[slot].status, slot).toBe('applied');
  811 |   expect(state.container.computed.padding).toEqual([10, 20, 30, 40]);
  812 |   expect(state.text.computed.padding).toEqual([5, 3, 5, 3]);
  813 |   const boxes = await page.evaluate(() => {
  814 |     const frame = document.getElementById('scope-a-frame')!.getBoundingClientRect();
  815 |     return Object.fromEntries(['sample', 'text', 'swatch'].map(id => {
  816 |       const box = document.getElementById(`scope-a-${id}`)!.getBoundingClientRect();
  817 |       return [id, [box.left - frame.left, box.top - frame.top, box.width, box.height]];
  818 |     }));
  819 |   });
  820 |   const [cx, cy] = state.container.geometry;
  821 |   const measured = {dom: boxes, gpu: {} as Record<string, number[]>};
  822 |   for (const [slot, id] of [['button', 'sample'], ['text', 'text'], ['swatch', 'swatch']]) {
  823 |     const [x, y, w, h] = state[slot].geometry;
  824 |     measured.gpu[id] = [x - cx, y - cy, w, h];
  825 |     measured.gpu[id].forEach((value, index) => expect(Math.abs(value - boxes[id][index]), `${slot}[${index}] ${measured.gpu[id]} vs ${boxes[id]}`).toBeLessThanOrEqual(1));
  826 |   }
  827 |   expect(measured.gpu.sample.slice(2)).toEqual([64, 64]);
  828 |   expect(measured.gpu.swatch.slice(2)).toEqual([30, 10]);
  829 |   // justify-end: the last box ends at the bottom padding edge.
  830 |   expect(measured.gpu.swatch[1] + measured.gpu.swatch[3]).toBeCloseTo(240 - 30, 5);
  831 |   await info.attach('Container geometry', {body: JSON.stringify(measured, null, 2), contentType: 'application/json'});
  832 |   await page.getByTestId('utility-gpu').screenshot({path: info.outputPath('column-geometry-gpu.png')});
  833 |   await page.locator('#scope-a-frame').screenshot({path: info.outputPath('column-geometry-dom.png')});
  834 | 
  835 |   const weighted = await apply(page, ['size-[64px]', 'bg-[#123456]'], [container, ['font-semibold'], []]);
  836 |   await expect.poll(async () => (await report(page)).text.attempted_revision).toBe(weighted);
  837 |   const rejected = await report(page);
> 838 |   expect(rejected.text.status).toBe('rejected');
      |                                ^ Error: expect(received).toBe(expected) // Object.is equality
  839 |   expect(rejected.text.diagnostics.some((item: any) => item.property === 'font-weight')).toBe(true);
  840 |   expect(rejected.text.applied_revision).toBe(revision);
  841 |   expect(rejected.text.computed).toEqual(state.text.computed);
  842 |   await expect(page.locator('#scope-a-text')).toHaveCSS('font-weight', '600');
  843 | });
  844 | 
  845 | test('GPU restores its latest committed programs after a real WebGL context loss', async ({page}) => {
  846 |   await ready(page);
  847 |   const first = await apply(page, ['w-[137px]', 'bg-[#123456]']);
  848 |   await drawn(page, first);
  849 |   await page.getByTestId('utility-gpu').evaluate((canvas: HTMLCanvasElement) => {
  850 |     const gl = canvas.getContext('webgl2')!;
  851 |     const extension = gl.getExtension('WEBGL_lose_context')!;
  852 |     (window as any).__utility_loss = extension; extension.loseContext();
  853 |   });
  854 |   await expect(page.locator('#gpu-status')).toHaveText('Restoring GPU…');
  855 |   const next = await page.evaluate(async () => (await (window as any).__utility_lab.gpuApply('scope-a', [[], ['w-[179px]', 'bg-[#abcdef]'], [], []])).revision);
  856 |   await page.evaluate(() => { (window as any).__utility_loss.restoreContext(); delete (window as any).__utility_loss; });
  857 |   await expect(page.locator('#gpu-status')).toHaveText('GPU ready');
  858 |   const state = await drawn(page, next);
  859 |   expect(state.button.computed.width.value).toBe(179);
  860 |   expect(state.button.computed.background.slice(0, 3)).toEqual([171/255, 205/255, 239/255]);
  861 |   expect(await page.getByTestId('utility-gpu').count()).toBe(1);
  862 | });
  863 | 
  864 | test('terminal GPU initialization failure explains recovery while DOM editing remains usable', async ({page}, info) => {
  865 |   await page.addInitScript(() => {
  866 |     const original = HTMLCanvasElement.prototype.getContext;
  867 |     HTMLCanvasElement.prototype.getContext = function(type: any, ...args: any[]) {
  868 |       if (type === 'webgl2') return null;
  869 |       return original.apply(this, [type, ...args] as any);
  870 |     } as any;
  871 |   });
  872 |   await page.goto('./');
  873 |   await expect(page.locator('#status')).toHaveAttribute('data-status', 'ready');
  874 |   await expect(page.locator('#gpu-status')).toContainText('GPU unavailable: no WebGL2 context');
  875 |   await expect(page.locator('#gpu-status')).toContainText('Copy your class inputs, then reload');
  876 |   await page.locator('#classes').fill('w-[179px] bg-[#abcdef]');
  877 |   await page.locator('#apply').click();
  878 |   await expect(page.locator('#status')).toContainText('DOM styles installed.');
  879 |   await expect(page.locator('#scope-a-sample')).toHaveCSS('width', '179px');
  880 |   await page.screenshot({path: info.outputPath('gpu-terminal-failure.png'), fullPage: true});
  881 | });
  882 | 
  883 | test('all non-button GPU boundaries are visible and the desktop comparison frames align', async ({page}, info) => {
  884 |   await ready(page);
  885 |   await apply(page, ['w-[137px]', 'bg-[#123456]'], [['opacity-50'], ['bg-[#abcdef]', 'opacity-50'], ['grid']]);
  886 |   await expect(page.locator('#gpu-results')).toContainText('Container: rejected');
  887 |   await expect(page.locator('#gpu-results')).toContainText('Text: rejected');
  888 |   await expect(page.locator('#gpu-results')).toContainText('Color block: rejected');
  889 |   await expect(page.locator('#gpu-warning')).toContainText('opacity-50 (opacity)');
  890 |   await expect(page.locator('#gpu-warning')).toContainText('grid (display)');
  891 |   await expect(page.locator('#gpu-warning')).toContainText('Remove the class');
  892 |   const dom = (await page.locator('#scope-a.preview').boundingBox())!;
  893 |   const gpu = (await page.getByTestId('utility-gpu').boundingBox())!;
  894 |   expect(Math.abs(dom.y - gpu.y)).toBeLessThanOrEqual(1);
  895 |   await page.screenshot({path: info.outputPath('gpu-boundaries.png'), fullPage: true});
  896 |   await apply(page, ['w-[137px]', 'bg-[#123456]'], [['opacity-50'], ['bg-[#abcdef]', 'opacity-50'], ['grid']], true);
  897 |   await expect(page.locator('#gpu-results')).toContainText('Container: partial');
  898 |   await expect(page.locator('#gpu-results')).toContainText('Text: partial');
  899 |   await expect(page.locator('#gpu-results')).toContainText('Color block: partial');
  900 | });
  901 | 
  902 | test('live ThemeScope and ThemeBoundary update colors, dark conditions and host rem without recompiling', async ({page}, info) => {
  903 |   await page.setViewportSize({width: 1000, height: 900});
  904 |   await ready(page);
  905 |   const baseline = (await report(page)).button.computed;
  906 |   const revision = await apply(page, ['w-[5rem]', 'bg-(--primary)', 'text-(--primary-foreground)', 'dark:border-2', 'dark:border-[#123456]', 'min-[60rem]:h-[73px]'], [['gap-[1rem]'], ['text-(--foreground)'], ['bg-(--primary)']]);
  907 |   await drawn(page, revision);
  908 |   await expect(page.locator('#scope-a-sample')).toHaveCSS('width', '80px');
  909 |   expect((await report(page)).button.computed.height.value).toBe(73);
  910 |   const installed = await page.evaluate(() => (window as any).__utility_lab.report('scope-a'));
  911 |   const sibling = await page.locator('#scope-b-sample').evaluate(node => getComputedStyle(node).backgroundColor);
  912 |   await page.emulateMedia({colorScheme: 'dark'});
  913 |   await page.evaluate(() => {
  914 |     document.documentElement.style.fontSize = '20px';
  915 |     (window as any).__utility_lab.theme('scope-a', true, 20, {primary: '#abcdef'});
  916 |   });
  917 |   await expect(page.locator('#scope-a')).toHaveAttribute('data-theme', 'dark');
  918 |   await expect(page.getByTestId('utility-theme-boundary')).toHaveCSS('--primary', 'rgb(171 205 239)');
  919 |   await expect(page.locator('#scope-a-sample')).toHaveCSS('width', '100px');
  920 |   await expect(page.locator('#scope-a-sample')).toHaveCSS('background-color', 'rgb(171, 205, 239)');
  921 |   await expect.poll(async () => (await report(page)).button.computed.background).toEqual([171 / 255, 205 / 255, 239 / 255, 1]);
  922 |   await expect.poll(async () => (await report(page)).button.computed.width.value).toBe(100);
  923 |   expect((await report(page)).button.computed.height.value).toBe(73);
  924 |   expect((await report(page)).button.computed.border_widths).toEqual([2, 2, 2, 2]);
  925 |   expect((await report(page)).container.computed.gap).toEqual([20, 20]);
  926 |   await expect(page.locator('#scope-b-sample')).toHaveCSS('background-color', sibling);
  927 |   expect(await page.evaluate(() => (window as any).__utility_lab.report('scope-a'))).toEqual(installed);
  928 |   const retained = (await report(page)).button.computed;
  929 |   const rejected = await page.evaluate(() => {
  930 |     try { (window as any).__utility_lab.theme('scope-a', false, 16, {radius: 'invalid'}); return false; }
  931 |     catch { return true; }
  932 |   });
  933 |   expect(rejected).toBe(true);
  934 |   expect((await report(page)).button.computed).toEqual(retained);
  935 |   const actual = PNG.sync.read(await page.getByTestId('utility-gpu').screenshot());
  936 |   const bounds = (await report(page)).button.geometry;
  937 |   expect(pixel(actual, bounds[0] + 5, bounds[1] + 5)).toEqual([171, 205, 239, 255]);
  938 |   await info.attach('Theme actual drawn report', {body: JSON.stringify(await report(page), null, 2), contentType: 'application/json'});
```