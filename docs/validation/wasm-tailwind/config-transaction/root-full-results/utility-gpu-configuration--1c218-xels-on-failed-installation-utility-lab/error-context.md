# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: utility-gpu.spec.ts >> configuration replacement publishes new DOM and GPU programs and retains pixels on failed installation
- Location: tests/browser/utility-gpu.spec.ts:646:5

# Error details

```
Error: expect(locator).toHaveCSS(expected) failed

Locator:  locator('#scope-a-sample')
Expected: "16px"
Received: "32px"
Timeout:  15000ms

Call log:
  - Expect "toHaveCSS" locator('#scope-a-sample') with timeout 15000ms
  - waiting for locator('#scope-a-sample')
    34 × locator resolved to <div tabindex="0" role="button" id="scope-a-sample" class="sample w-[121px] h-[61px] bg-brand text-white p-4">Live utility sample</div>
       - unexpected value "32px"

```

```yaml
- button "Live utility sample"
```

# Test source

```ts
  556 |   await page.addInitScript(() => {
  557 |     const original = HTMLCanvasElement.prototype.getContext;
  558 |     HTMLCanvasElement.prototype.getContext = function(type: any, ...args: any[]) {
  559 |       if (type === 'webgl2') return null;
  560 |       return original.apply(this, [type, ...args] as any);
  561 |     } as any;
  562 |   });
  563 |   await page.goto('./');
  564 |   await expect(page.locator('#status')).toHaveAttribute('data-status', 'ready');
  565 |   await expect(page.locator('#gpu-status')).toContainText('GPU unavailable: no WebGL2 context');
  566 |   await expect(page.locator('#gpu-status')).toContainText('Copy your class inputs, then reload');
  567 |   await page.locator('#classes').fill('w-[179px] bg-[#abcdef]');
  568 |   await page.locator('#apply').click();
  569 |   await expect(page.locator('#status')).toContainText('DOM styles installed.');
  570 |   await expect(page.locator('#scope-a-sample')).toHaveCSS('width', '179px');
  571 |   await page.screenshot({path: info.outputPath('gpu-terminal-failure.png'), fullPage: true});
  572 | });
  573 | 
  574 | test('all non-button GPU boundaries are visible and the desktop comparison frames align', async ({page}, info) => {
  575 |   await ready(page);
  576 |   await apply(page, ['w-[137px]', 'bg-[#123456]'], [['opacity-50'], ['bg-[#abcdef]', 'opacity-50'], ['grid']]);
  577 |   await expect(page.locator('#gpu-results')).toContainText('Container: rejected');
  578 |   await expect(page.locator('#gpu-results')).toContainText('Text: rejected');
  579 |   await expect(page.locator('#gpu-results')).toContainText('Color block: rejected');
  580 |   await expect(page.locator('#gpu-warning')).toContainText('opacity-50 (opacity)');
  581 |   await expect(page.locator('#gpu-warning')).toContainText('grid (display)');
  582 |   await expect(page.locator('#gpu-warning')).toContainText('Remove the class');
  583 |   const dom = (await page.locator('#scope-a.preview').boundingBox())!;
  584 |   const gpu = (await page.getByTestId('utility-gpu').boundingBox())!;
  585 |   expect(Math.abs(dom.y - gpu.y)).toBeLessThanOrEqual(1);
  586 |   await page.screenshot({path: info.outputPath('gpu-boundaries.png'), fullPage: true});
  587 |   await apply(page, ['w-[137px]', 'bg-[#123456]'], [['opacity-50'], ['bg-[#abcdef]', 'opacity-50'], ['grid']], true);
  588 |   await expect(page.locator('#gpu-results')).toContainText('Container: partial');
  589 |   await expect(page.locator('#gpu-results')).toContainText('Text: partial');
  590 |   await expect(page.locator('#gpu-results')).toContainText('Color block: partial');
  591 | });
  592 | 
  593 | test('live ThemeScope and ThemeBoundary update colors, dark conditions and host rem without recompiling', async ({page}, info) => {
  594 |   await page.setViewportSize({width: 1000, height: 900});
  595 |   await ready(page);
  596 |   const baseline = (await report(page)).button.computed;
  597 |   const revision = await apply(page, ['w-[5rem]', 'bg-(--primary)', 'text-(--primary-foreground)', 'dark:border-2', 'dark:border-[#123456]', 'min-[60rem]:h-[73px]'], [['gap-[1rem]'], ['text-(--foreground)'], ['bg-(--primary)']]);
  598 |   await drawn(page, revision);
  599 |   await expect(page.locator('#scope-a-sample')).toHaveCSS('width', '80px');
  600 |   expect((await report(page)).button.computed.height.value).toBe(73);
  601 |   const installed = await page.evaluate(() => (window as any).__utility_lab.report('scope-a'));
  602 |   const sibling = await page.locator('#scope-b-sample').evaluate(node => getComputedStyle(node).backgroundColor);
  603 |   await page.emulateMedia({colorScheme: 'dark'});
  604 |   await page.evaluate(() => {
  605 |     document.documentElement.style.fontSize = '20px';
  606 |     (window as any).__utility_lab.theme('scope-a', true, 20, {primary: '#abcdef'});
  607 |   });
  608 |   await expect(page.locator('#scope-a')).toHaveAttribute('data-theme', 'dark');
  609 |   await expect(page.getByTestId('utility-theme-boundary')).toHaveCSS('--primary', 'rgb(171 205 239)');
  610 |   await expect(page.locator('#scope-a-sample')).toHaveCSS('width', '100px');
  611 |   await expect(page.locator('#scope-a-sample')).toHaveCSS('background-color', 'rgb(171, 205, 239)');
  612 |   await expect.poll(async () => (await report(page)).button.computed.background).toEqual([171 / 255, 205 / 255, 239 / 255, 1]);
  613 |   await expect.poll(async () => (await report(page)).button.computed.width.value).toBe(100);
  614 |   expect((await report(page)).button.computed.height.value).toBe(73);
  615 |   expect((await report(page)).button.computed.border_widths).toEqual([2, 2, 2, 2]);
  616 |   expect((await report(page)).container.computed.gap).toEqual([20, 20]);
  617 |   await expect(page.locator('#scope-b-sample')).toHaveCSS('background-color', sibling);
  618 |   expect(await page.evaluate(() => (window as any).__utility_lab.report('scope-a'))).toEqual(installed);
  619 |   const retained = (await report(page)).button.computed;
  620 |   const rejected = await page.evaluate(() => {
  621 |     try { (window as any).__utility_lab.theme('scope-a', false, 16, {radius: 'invalid'}); return false; }
  622 |     catch { return true; }
  623 |   });
  624 |   expect(rejected).toBe(true);
  625 |   expect((await report(page)).button.computed).toEqual(retained);
  626 |   const actual = PNG.sync.read(await page.getByTestId('utility-gpu').screenshot());
  627 |   const bounds = (await report(page)).button.geometry;
  628 |   expect(pixel(actual, bounds[0] + 5, bounds[1] + 5)).toEqual([171, 205, 239, 255]);
  629 |   await info.attach('Theme actual drawn report', {body: JSON.stringify(await report(page), null, 2), contentType: 'application/json'});
  630 |   await page.screenshot({path: info.outputPath('gpu-theme-boundary.png'), fullPage: true});
  631 |   await page.evaluate(() => (window as any).__utility_lab.theme('scope-a', true, 20));
  632 |   await expect(page.getByTestId('utility-theme-boundary')).not.toHaveAttribute('data-rustify-theme-runtime');
  633 |   await expect.poll(async () => (await report(page)).button.computed.background).not.toEqual(retained.background);
  634 |   await page.emulateMedia({colorScheme: 'light'});
  635 |   await page.evaluate(() => {
  636 |     document.documentElement.style.fontSize = '';
  637 |     (window as any).__utility_lab.theme('scope-a', false, 16);
  638 |   });
  639 |   await expect(page.locator('#scope-a')).toHaveAttribute('data-theme', 'light');
  640 |   const restored = await apply(page, []);
  641 |   await drawn(page, restored);
  642 |   expect((await report(page)).button.computed.background).toEqual(baseline.background);
  643 |   expect((await report(page)).button.computed.padding).toEqual(baseline.padding);
  644 | });
  645 | 
  646 | test('configuration replacement publishes new DOM and GPU programs and retains pixels on failed installation', async ({page}, info) => {
  647 |   const errors: string[] = [], failed: string[] = [];
  648 |   page.on('pageerror', error => errors.push(error.message));
  649 |   page.on('requestfailed', request => failed.push(request.url()));
  650 |   await ready(page);
  651 |   const classes = ['w-[121px]', 'h-[61px]', 'bg-brand', 'text-white', 'p-4'];
  652 |   const css = (color: string, spacing: string) => `@theme { --color-brand: ${color}; --color-white: #ffffff; --spacing: ${spacing}rem; } @tailwind utilities;`;
  653 |   await page.evaluate(css => (window as any).__utility_lab.replace('scope-a', css), css('#123456', '0.25'));
  654 |   const old = await drawn(page, await apply(page, classes));
  655 |   await expect(page.locator('#scope-a-sample')).toHaveCSS('background-color', 'rgb(18, 52, 86)');
> 656 |   await expect(page.locator('#scope-a-sample')).toHaveCSS('padding', '16px');
      |                                                 ^ Error: expect(locator).toHaveCSS(expected) failed
  657 |   const changed = await page.evaluate(css => (window as any).__utility_lab.replace('scope-a', css), css('#abcdef', '0.5'));
  658 |   expect(changed.revision).toBeGreaterThan(old.button.drawn_revision);
  659 |   const next = await drawn(page, await apply(page, classes));
  660 |   expect(next.button.drawn_revision).toBe(changed.revision);
  661 |   await expect(page.locator('#scope-a-sample')).toHaveCSS('background-color', 'rgb(171, 205, 239)');
  662 |   await expect(page.locator('#scope-a-sample')).toHaveCSS('padding', '32px');
  663 |   const canvas = page.getByTestId('utility-gpu');
  664 |   const rgba = () => canvas.screenshot().then(buffer => {
  665 |     const [x, y] = next.button.geometry;
  666 |     return pixel(PNG.sync.read(buffer), x + 8, y + 8).slice(0, 3);
  667 |   });
  668 |   await expect.poll(rgba).toEqual([171, 205, 239]);
  669 |   const rejected = await page.evaluate(async css => {
  670 |     const api = (window as any).__utility_lab;
  671 |     const original = CSSStyleSheet.prototype.replaceSync;
  672 |     CSSStyleSheet.prototype.replaceSync = function () { throw new Error('injected GPU configuration install failure'); };
  673 |     let failure;
  674 |     try { await api.replace('scope-a', css); }
  675 |     catch (error) { failure = JSON.parse(String(error)); }
  676 |     finally { CSSStyleSheet.prototype.replaceSync = original; }
  677 |     return {failure, revision: api.report('scope-a').revision};
  678 |   }, css('#fedcba', '1'));
  679 |   expect(rejected.failure.kind).toBe('install');
  680 |   expect(rejected.revision).toBe(changed.revision);
  681 |   expect((await report(page)).button.drawn_revision).toBe(changed.revision);
  682 |   await expect.poll(rgba).toEqual([171, 205, 239]);
  683 |   await expect(page.locator('#scope-a-sample')).toHaveCSS('background-color', 'rgb(171, 205, 239)');
  684 |   expect(errors).toEqual([]); expect(failed).toEqual([]);
  685 |   await info.attach('Configuration DOM/GPU commit and rollback', {body: JSON.stringify({old, next, rejected}, null, 2), contentType: 'application/json'});
  686 |   await page.screenshot({path: info.outputPath('configuration-gpu.png'), fullPage: true});
  687 | });
  688 | 
```