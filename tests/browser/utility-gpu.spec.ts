import {test, expect, type Page} from '@playwright/test';
import {PNG} from 'pngjs';

const report = (page: Page) => page.evaluate(() => (window as any).__utility_lab.gpuReport('scope-a'));
async function ready(page: Page) {
  await page.goto('./');
  await expect(page.locator('#status')).toHaveAttribute('data-status', 'ready');
  await page.getByTestId('utility-gpu').scrollIntoViewIfNeeded();
  await expect(page.locator('#gpu-status')).toHaveText('GPU ready');
  await expect.poll(async () => (await report(page))?.button.drawn_revision).toBe(0);
}
async function apply(page: Page, button: string[], others: string[][] = [[], [], []], partial = false, disabled = false) {
  const revision = await page.evaluate(async ({button, others, partial, disabled}) => {
    const api = (window as any).__utility_lab;
    const result = await api.gpuApply('scope-a', [others[0], button, others[1], others[2]], partial, disabled);
    document.getElementById('scope-a-sample')!.className = `sample ${button.join(' ')}`;
    for (const [id, base, values] of [['scope-a-frame', 'preview-content', others[0]], ['scope-a-text', 'sample-text', others[1]], ['scope-a-swatch', 'sample-swatch', others[2]]] as const) {
      document.getElementById(id)!.className = `${base} ${values.join(' ')}`;
    }
    return result.revision;
  }, {button, others, partial, disabled});
  await expect.poll(async () => (await report(page))?.button.attempted_revision).toBe(revision);
  return revision;
}
async function drawn(page: Page, revision: number, slot = 'button') {
  await expect.poll(async () => (await report(page))?.[slot].drawn_revision).toBe(revision);
  return report(page);
}
const pixel = (png: PNG, x: number, y: number) => Array.from(png.data.subarray((Math.floor(y) * png.width + Math.floor(x)) * 4, (Math.floor(y) * png.width + Math.floor(x)) * 4 + 4));

async function mountButtonBaseline(page: Page, themed = true) {
  const errors: string[] = [], failed: string[] = [];
  page.on('pageerror', error => errors.push(error.message));
  page.on('console', message => { if (message.type() === 'error') errors.push(message.text()); });
  page.on('requestfailed', request => failed.push(request.url()));
  page.on('response', response => { if (response.status() >= 400) failed.push(`${response.status()} ${response.url()}`); });
  await ready(page);
  await page.evaluate(themed => {
    const container = document.createElement('div');
    container.id = 'button-baseline-root';
    document.querySelector('main')!.append(container);
    const sheet = new CSSStyleSheet();
    sheet.replaceSync('#button-baseline-root canvas { display:block; width:300px; height:240px; }');
    document.adoptedStyleSheets = [...document.adoptedStyleSheets, sheet];
    (window as any).__utility_lab.buttonBaseline.mount(container.id, themed);
  }, themed);
  const canvas = page.getByTestId('button-baseline');
  await canvas.scrollIntoViewIfNeeded();
  await page.mouse.move(0, 0);
  const read = () => page.evaluate(() => (window as any).__utility_lab.buttonBaseline.report());
  const wait = async (generation: number, phase: number) => {
    await expect.poll(async () => {
      const state = await read();
      return {generation: state?.generation, phase: state?.phase};
    }).toEqual({generation, phase});
  };
  const capture = async () => {
    const state = await read();
    const image = PNG.sync.read(await canvas.screenshot());
    const pixels = state.buttons.map((button: any) => {
      const [x, y, width, height] = button.geometry;
      const values: number[] = [];
      for (let row = 0; row < height; row++) for (let col = 0; col < width; col++) values.push(...pixel(image, x + col, y + row));
      return values;
    });
    return {state, image, pixels};
  };
  await wait(0, 0);
  return {canvas, read, wait, capture, errors, failed};
}

test('clearing utility ownership restores the ordinary Button pixels and events', async ({page}, info) => {
  const errors: string[] = [], failed: string[] = [];
  page.on('pageerror', error => errors.push(error.message));
  page.on('console', message => { if (message.type() === 'error') errors.push(message.text()); });
  page.on('requestfailed', request => failed.push(request.url()));
  page.on('response', response => { if (response.status() >= 400) failed.push(`${response.status()} ${response.url()}`); });
  await ready(page);
  await page.evaluate(() => {
    const container = document.createElement('div');
    container.id = 'button-baseline-root';
    document.querySelector('main')!.append(container);
    const sheet = new CSSStyleSheet();
    sheet.replaceSync('#button-baseline-root canvas { display:block; width:300px; height:240px; }');
    document.adoptedStyleSheets = [...document.adoptedStyleSheets, sheet];
    (window as any).__utility_lab.buttonBaseline.mount(container.id);
  });
  const canvas = page.getByTestId('button-baseline');
  await canvas.scrollIntoViewIfNeeded();
  await page.mouse.move(0, 0);
  const baselineReport = () => page.evaluate(() => (window as any).__utility_lab.buttonBaseline.report());
  await expect.poll(async () => (await baselineReport())?.buttons?.[1]?.geometry?.[2]).toBe(220);
  const compare = async (phase: number, generation = 0) => {
    await expect.poll(async () => (await baselineReport())?.generation).toBe(generation);
    await expect.poll(async () => (await baselineReport())?.phase).toBe(phase);
    const state = await baselineReport();
    expect(state.buttons.map((button: any) => button.utility)).toEqual([null, null]);
    expect(state.buttons[0].geometry.slice(2)).toEqual(state.buttons[1].geometry.slice(2));
    // Both controls use the same fonts/shaders in one canvas. Compare every
    // pixel, including text, corner coverage, border and disabled alpha.
    const capture = async () => {
      const image = PNG.sync.read(await canvas.screenshot());
      return state.buttons.map((button: any) => {
        const [x, y, width, height] = button.geometry;
        const pixels: number[] = [];
        for (let row = 0; row < height; row++) for (let col = 0; col < width; col++) pixels.push(...pixel(image, x + col, y + row));
        return pixels;
      });
    };
    await expect.poll(async () => { const [ordinary, cleared] = await capture(); return ordinary.filter((value: number, index: number) => value !== cleared[index]).length; }).toBe(0);
    return state;
  };
  const initial = await compare(0);
  let clearedGeneration = 0;
  for (const [dark, rem, palette, disabled] of [[false, 16, false, false], [true, 20, false, false], [false, 16, true, false], [true, 20, false, true]] as const) {
    await page.evaluate(({dark, rem, palette, disabled}) => (window as any).__utility_lab.buttonBaseline.update(1, dark, rem, palette, disabled), {dark, rem, palette, disabled});
    await expect.poll(async () => (await baselineReport())?.buttons?.[1]?.utility?.drawn_revision).toBe(1);
    expect((await baselineReport()).buttons[0].utility).toBeNull();
    clearedGeneration = await page.evaluate(({dark, rem, palette, disabled}) => (window as any).__utility_lab.buttonBaseline.update(2, dark, rem, palette, disabled), {dark, rem, palette, disabled});
    await compare(2, clearedGeneration);
  }
  clearedGeneration = await page.evaluate(() => (window as any).__utility_lab.buttonBaseline.update(2));
  const restored = await compare(2, clearedGeneration);
  const box = (await canvas.boundingBox())!;
  const focusedPixels = async (geometry: number[]) => {
    const image = PNG.sync.read(await canvas.screenshot());
    const [x, y, width, height] = geometry;
    const pixels: number[] = [];
    for (let row = -4; row < height + 4; row++) for (let col = -4; col < width + 4; col++) pixels.push(...pixel(image, x + col, y + row));
    return pixels;
  };
  let ordinaryFocused: number[] = [];
  for (const [index, button] of restored.buttons.entries()) {
    const [x, y, width, height] = button.geometry;
    await page.mouse.click(box.x + x + width / 2, box.y + y + height / 2);
    await page.mouse.move(0, 0);
    await expect.poll(async () => (await baselineReport()).clicks[index]).toBe(1);
    if (index === 0) ordinaryFocused = await focusedPixels(button.geometry);
    else await expect.poll(async () => {
      const clearedFocused = await focusedPixels(button.geometry);
      return ordinaryFocused.filter((value, index) => value !== clearedFocused[index]).length;
    }).toBe(0);
  }
  expect(errors).toEqual([]); expect(failed).toEqual([]);
  await info.attach('Ordinary and cleared Button reports', {body: JSON.stringify({initial, restored, clicked: await baselineReport()}, null, 2), contentType: 'application/json'});
  await canvas.screenshot({path: info.outputPath('button-none.png')});
  await page.evaluate(() => (window as any).__utility_lab.buttonBaseline.dispose());
  await expect(canvas).toHaveCount(0);
});

for (const {name, dark, rem, palette, height, color} of [
  {name: 'theme', dark: true, rem: 20, palette: false, height: 45, color: [83, 177, 253, 255]},
  {name: 'palette', dark: false, rem: 16, palette: true, height: 36, color: [18, 52, 86, 255]},
]) {
  test(`clearing utility ownership restores the latest ${name} without another baseline setter`, async ({page}, info) => {
    const fixture = await mountButtonBaseline(page);
    const initial = await fixture.capture();
    const appliedGeneration = await page.evaluate(() => (window as any).__utility_lab.buttonBaseline.update(1));
    await fixture.wait(appliedGeneration, 1);
    await expect.poll(async () => (await fixture.read()).buttons[1].utility?.drawn_revision).toBe(1);
    const applied = await fixture.capture();
    expect(applied.state.buttons[1].utility.status).toBe('Applied');
    expect(applied.state.buttons[1].geometry[3]).toBe(64);
    const [ax, ay, aw, ah] = applied.state.buttons[1].geometry;
    expect(pixel(applied.image, ax + aw / 2, ay + ah - 8)).toEqual([254, 220, 186, 255]);

    // Refresh the ordinary baseline while the same utility still owns the Button.
    const heldGeneration = await page.evaluate(({dark, rem, palette}) => (window as any).__utility_lab.buttonBaseline.update(3, dark, rem, palette), {dark, rem, palette});
    await fixture.wait(heldGeneration, 3);
    const held = await fixture.capture();
    expect(held.state.buttons[1].utility).toEqual({status: 'Applied', drawn_revision: 1});
    expect(held.state.buttons[0].geometry[3]).toBe(height);
    expect(held.state.baseline.font_size).toBe(21);
    expect(held.state.baseline.tracking).toBe(.04);
    const [hx, hy, hw, hh] = held.state.buttons[0].geometry;
    expect(pixel(held.image, hx + hw / 2, hy + hh - 8)).toEqual(color);
    expect(held.pixels[0]).not.toEqual(initial.pixels[0]);

    const clearedGeneration = await page.evaluate(() => (window as any).__utility_lab.buttonBaseline.update(4));
    await fixture.wait(clearedGeneration, 4);
    const clearedState = await fixture.read();
    expect(clearedState.baseline).toEqual(held.state.baseline);
    expect(clearedState.buttons.map((button: any) => button.utility)).toEqual([null, null]);
    expect(clearedState.buttons[0].geometry.slice(2)).toEqual(clearedState.buttons[1].geometry.slice(2));
    await expect.poll(async () => {
      const {pixels} = await fixture.capture();
      return pixels[0].filter((value: number, index: number) => value !== pixels[1][index]).length;
    }).toBe(0);
    const cleared = await fixture.capture();
    expect(cleared.pixels[0]).toEqual(held.pixels[0]);

    const updatedGeneration = await page.evaluate(() => (window as any).__utility_lab.buttonBaseline.update(0));
    await fixture.wait(updatedGeneration, 0);
    const updatedState = await fixture.read();
    expect(updatedState.buttons.map((button: any) => button.utility)).toEqual([null, null]);
    expect(updatedState.buttons[0].geometry.slice(2)).toEqual(updatedState.buttons[1].geometry.slice(2));
    await expect.poll(async () => {
      const {pixels} = await fixture.capture();
      return pixels[0].filter((value: number, index: number) => value !== pixels[1][index]).length;
    }).toBe(0);
    const updated = await fixture.capture();
    expect(updated.pixels[0]).not.toEqual(cleared.pixels[0]);
    expect(updated.state.baseline.font_size).toBe(15);
    await info.attach(`Button ${name} baseline lifecycle`, {body: JSON.stringify({initial: initial.state, applied: applied.state, held: held.state, cleared: cleared.state, updated: updated.state}), contentType: 'application/json'});
    await fixture.canvas.screenshot({path: info.outputPath(`button-none-live-${name}.png`)});
    expect(fixture.errors).toEqual([]); expect(fixture.failed).toEqual([]);
    await page.evaluate(() => (window as any).__utility_lab.buttonBaseline.dispose());
    await expect(fixture.canvas).toHaveCount(0);
  });
}

test('clearing utility ownership restores untouched Script Button defaults', async ({page}, info) => {
  const fixture = await mountButtonBaseline(page, false);
  await expect.poll(async () => {
    const {state, image} = await fixture.capture();
    const [x, y, width, height] = state.buttons[0].geometry;
    let glyphPixels = 0;
    for (let row = 8; row < height - 8; row++) for (let col = 16; col < width - 16; col++) {
      const color = pixel(image, x + col, y + row);
      if (color.slice(0, 3).every(value => value < 100)) glyphPixels++;
    }
    return glyphPixels;
  }).toBeGreaterThan(0);
  const initial = await fixture.capture();
  expect(initial.state.baseline.themed).toBe(false);
  expect(initial.state.buttons.map((button: any) => button.utility)).toEqual([null, null]);
  // The two positions already differ by one channel unit before ownership.
  expect(Math.max(...initial.pixels[0].map((value: number, index: number) => Math.abs(value - initial.pixels[1][index])))).toBeLessThanOrEqual(1);

  const appliedGeneration = await page.evaluate(() => (window as any).__utility_lab.buttonBaseline.update(1));
  await fixture.wait(appliedGeneration, 1);
  await expect.poll(async () => (await fixture.read()).buttons[1].utility?.drawn_revision).toBe(1);
  const applied = await fixture.capture();
  expect(applied.state.buttons[1].utility.status).toBe('Applied');
  expect(applied.state.buttons[1].geometry[3]).toBe(64);
  const [x, y, width, height] = applied.state.buttons[1].geometry;
  expect(pixel(applied.image, x + width / 2, y + height - 8)).toEqual([254, 220, 186, 255]);

  const clearedGeneration = await page.evaluate(() => (window as any).__utility_lab.buttonBaseline.update(4));
  await fixture.wait(clearedGeneration, 4);
  const clearedState = await fixture.read();
  expect(clearedState.baseline).toEqual(initial.state.baseline);
  expect(clearedState.buttons.map((button: any) => button.utility)).toEqual([null, null]);
  expect(clearedState.buttons[0].geometry.slice(2)).toEqual(clearedState.buttons[1].geometry.slice(2));
  await expect.poll(async () => {
    const {pixels} = await fixture.capture();
    return pixels[1].filter((value: number, index: number) => value !== initial.pixels[1][index]).length;
  }).toBe(0);
  const cleared = await fixture.capture();
  expect(cleared.pixels[0]).toEqual(initial.pixels[0]);
  expect(cleared.pixels[1]).toEqual(initial.pixels[1]);
  await info.attach('Unthemed Script Button lifecycle', {body: JSON.stringify({initial: initial.state, applied: applied.state, cleared: cleared.state}), contentType: 'application/json'});
  await fixture.canvas.screenshot({path: info.outputPath('button-none-script.png')});
  expect(fixture.errors).toEqual([]); expect(fixture.failed).toEqual([]);
  await page.evaluate(() => (window as any).__utility_lab.buttonBaseline.dispose());
  await expect(fixture.canvas).toHaveCount(0);
});

test('GPU percentages use the parent content box and follow its real resize', async ({page}, info) => {
  const errors: string[] = [], failed: string[] = [];
  page.on('pageerror', error => errors.push(error.message));
  page.on('requestfailed', request => failed.push(request.url()));
  await page.setViewportSize({width: 1000, height: 900});
  await ready(page);
  const container = ['w-[300px]', 'h-[220px]', 'p-[10px]', 'border-2', 'flex-col', 'items-start', 'justify-start', 'gap-[0px]'];
  const button = ['w-[50%]', 'h-[50%]', 'p-[10%]', 'min-w-[100px]', 'max-w-[120px]', 'bg-[#123456]'];
  const compilation = await page.evaluate(async (classes) => (window as any).__utility_lab.ensure('scope-a', classes), [...container, ...button, 'w-[60px]', 'h-[20px]', 'w-[69px]', 'h-[69px]']);
  await info.attach('Parent geometry compiler input', {body: JSON.stringify(compilation, null, 2), contentType: 'application/json'});
  expect(compilation.unsupported).toEqual([]);
  expect(compilation.rejected.filter((item: any) => [...container, ...button].includes(item.candidate))).toEqual([]);
  const revision = await apply(page, button, [container, ['w-[60px]', 'h-[20px]'], ['w-[69px]', 'h-[69px]']]);
  const fixed = await drawn(page, revision);
  expect(fixed.container.geometry.slice(2)).toEqual([300, 220]);
  expect(fixed.button.status).toBe('applied');
  expect(fixed.button.geometry.slice(2)).toEqual([120, 98]);
  fixed.button.computed.padding.forEach((value: number) => expect(value).toBeCloseTo(27.6, 5));
  const sample = page.locator('#scope-a-sample');
  await expect(sample).toHaveCSS('width', '120px');
  await expect(sample).toHaveCSS('height', '98px');
  const domPadding = await sample.evaluate(element => {
    const style = getComputedStyle(element);
    return [parseFloat(style.paddingTop), parseFloat(style.paddingLeft)];
  });
  domPadding.forEach(value => expect(Math.abs(value - 27.6)).toBeLessThan(1 / 64));
  const fixedImage = PNG.sync.read(await page.getByTestId('utility-gpu').screenshot());
  expect(pixel(fixedImage, fixed.button.geometry[0] + 4, fixed.button.geometry[1] + 4).slice(0, 3)).toEqual([18, 52, 86]);

  const responsive = await apply(page, ['w-[50%]', 'h-[50px]', 'p-[0px]'], [
    container.filter(value => value !== 'w-[300px]').concat('w-[50%]'),
    ['w-[20px]', 'h-[20px]'], ['w-[40px]', 'h-[40px]'],
  ]);
  const before = await drawn(page, responsive);
  const compiled = await page.evaluate(() => (window as any).__utility_lab.report('scope-a'));
  const relative = (state: any) => {
    expect(state.button.status).toBe('applied');
    expect(state.button.geometry[2]).toBeCloseTo((state.container.geometry[2] - 24) / 2, 5);
    expect(state.button.geometry[3]).toBe(50);
  };
  relative(before);
  await page.setViewportSize({width: 800, height: 900});
  await expect.poll(async () => (await report(page)).container.geometry[2]).not.toBe(before.container.geometry[2]);
  const resized = await report(page);
  relative(resized);
  expect(resized.button.drawn_revision).toBe(responsive);
  const after = await page.evaluate(() => (window as any).__utility_lab.report('scope-a'));
  expect(after.revision).toBe(compiled.revision);
  expect(after.metrics.parsed_candidates).toBe(compiled.metrics.parsed_candidates);
  expect(errors).toEqual([]); expect(failed).toEqual([]);
  await info.attach('Measured parent geometry', {body: JSON.stringify({fixed, before, resized}, null, 2), contentType: 'application/json'});
  await page.screenshot({path: info.outputPath('parent-content.png'), fullPage: true});
});

test('inline and path sources with media theme options drive scoped DOM and GPU', async ({page}, info) => {
  const errors: string[] = [], failed: string[] = [], violations: string[] = [];
  page.on('console', message => { if (message.type() === 'error') errors.push(`${message.text()} ${message.location().url}`); });
  page.on('pageerror', error => errors.push(error.message));
  page.on('requestfailed', request => failed.push(request.url()));
  page.on('response', response => { if (response.status() >= 400) failed.push(`${response.status()} ${response.url()}`); });
  await ready(page);
  await page.exposeFunction('recordSourceViolation', (directive: string) => violations.push(directive));
  await page.evaluate(() => {
    document.addEventListener('securitypolicyviolation', event => (window as any).recordSourceViolation(event.violatedDirective));
    const api = (window as any).__utility_lab;
    api.dispose('scope-a');
    api.mount('scope-a', 10000, '@source "./virtual/**/*.rs"; @source not "./virtual/private/**"; @media theme(inline) prefix(tw) important source(none) { @theme { --spacing: 7px; --color-brand: #123456; } @source inline("tw:w-[{137,159}px] tw:h-[72px] tw:bg-brand tw:p-1"); @source not inline("tw:w-[159px]"); @tailwind utilities; }');
  });
  await expect(page.locator('#gpu-status')).toHaveText('GPU ready');
  const revision = await apply(page, ['tw:w-[137px]', 'tw:h-[72px]', 'tw:bg-brand', 'tw:p-1']);
  const state = await drawn(page, revision);
  expect(state.button.status).toBe('applied');
  expect(state.button.geometry.slice(2)).toEqual([137, 72]);
  expect(state.button.computed.padding).toEqual([7, 7, 7, 7]);
  const sample = page.locator('#scope-a-sample');
  await expect(sample).toHaveCSS('width', '137px');
  await expect(sample).toHaveCSS('height', '72px');
  await expect(sample).toHaveCSS('padding-top', '7px');
  await expect(sample).toHaveCSS('background-color', 'rgb(18, 52, 86)');
  const result = await page.evaluate(async () => {
    const api = (window as any).__utility_lab;
    const before = api.report('scope-a');
    const rejected = await api.ensure('scope-a', ['tw:w-[159px]']);
    return {before, rejected, otherWidth: getComputedStyle(document.getElementById('scope-b-sample')!).width};
  });
  expect(result.before.css).toContain('width: 137px !important');
  expect(result.before.css).not.toContain('width: 159px');
  expect(result.before.css).not.toContain('@source');
  expect(result.before.css).not.toContain('theme(inline)');
  expect(result.rejected.rejected.some((item: any) => item.candidate === 'tw:w-[159px]')).toBe(true);
  expect(result.rejected.revision).toBe(revision);
  expect(result.otherWidth).toBe('170px');
  const png = PNG.sync.read(await page.getByTestId('utility-gpu').screenshot());
  const [x, y] = state.button.geometry;
  expect(pixel(png, x + 8, y + 8).slice(0, 3)).toEqual([18, 52, 86]);
  expect(errors).toEqual([]); expect(failed).toEqual([]); expect(violations).toEqual([]);
  await info.attach('Source expansion and theme option values', {body: JSON.stringify({state, ...result}, null, 2), contentType: 'application/json'});
  await page.screenshot({path: info.outputPath('source-options.png'), fullPage: true});
});

test('applied utility dependencies drive DOM and GPU and survive an installation failure', async ({page}, info) => {
  const errors: string[] = [], failed: string[] = [], violations: string[] = [];
  page.on('pageerror', error => errors.push(error.message));
  page.on('requestfailed', request => failed.push(request.url()));
  await ready(page);
  await page.exposeFunction('recordApplyViolation', (directive: string) => violations.push(directive));
  await page.evaluate(() => {
    document.addEventListener('securitypolicyviolation', event => (window as any).recordApplyViolation(event.violatedDirective));
    const api = (window as any).__utility_lab;
    api.dispose('scope-a');
    api.mount('scope-a', 10000, '@utility base-card { @apply w-[137px] h-[72px] bg-[#123456] text-[#ffffff]; } @utility applied-card { @apply base-card p-[7px] border-2 border-[#abcdef]; } @utility next-card { @apply w-[179px] h-[72px] bg-[#abcdef]; } @tailwind utilities;');
  });
  await expect(page.locator('#gpu-status')).toHaveText('GPU ready');
  const first = await apply(page, ['applied-card']);
  const before = await drawn(page, first);
  expect(before.button.status).toBe('applied');
  expect(before.button.geometry.slice(2)).toEqual([137, 72]);
  expect(before.button.computed.padding).toEqual([7, 7, 7, 7]);
  expect(before.button.computed.border_widths).toEqual([2, 2, 2, 2]);
  const sample = page.locator('#scope-a-sample');
  await expect(sample).toHaveCSS('width', '137px');
  await expect(sample).toHaveCSS('background-color', 'rgb(18, 52, 86)');
  const png = PNG.sync.read(await page.getByTestId('utility-gpu').screenshot());
  const [x, y, width] = before.button.geometry;
  expect(pixel(png, x + 8, y + 8).slice(0, 3)).toEqual([18, 52, 86]);
  expect(pixel(png, x + width / 2 + 1, y + 1).slice(0, 3)).toEqual([171, 205, 239]);
  const failure = await page.evaluate(async () => {
    const original = CSSStyleSheet.prototype.replaceSync;
    CSSStyleSheet.prototype.replaceSync = function() { throw new DOMException('injected @apply install failure', 'SyntaxError'); };
    try { await (window as any).__utility_lab.gpuApply('scope-a', [[], ['next-card'], [], []]); }
    catch (error) { return String(error); }
    finally { CSSStyleSheet.prototype.replaceSync = original; }
  });
  expect(failure).toContain('injected @apply install failure');
  const retained = await report(page);
  expect(retained.button.applied_revision).toBe(first);
  expect(retained.button.drawn_revision).toBe(first);
  expect(retained.button.computed).toEqual(before.button.computed);
  await expect(sample).toHaveCSS('width', '137px');
  const next = await apply(page, ['next-card']);
  const recovered = await drawn(page, next);
  expect(next).toBe(first + 1);
  expect(recovered.button.geometry.slice(2)).toEqual([179, 72]);
  await expect(sample).toHaveCSS('width', '179px');
  await expect(sample).toHaveCSS('background-color', 'rgb(171, 205, 239)');
  expect(errors).toEqual([]); expect(failed).toEqual([]); expect(violations).toEqual([]);
  await info.attach('Applied dependencies and recovery', {body: JSON.stringify({before, retained, recovered}, null, 2), contentType: 'application/json'});
  await page.screenshot({path: info.outputPath('applied-utility.png'), fullPage: true});
});

test('GPU draws compiled colors, geometry and real font glyphs alongside scoped DOM', async ({page}, info) => {
  const errors: string[] = [], failed: string[] = [];
  page.on('pageerror', error => errors.push(error.message));
  page.on('requestfailed', request => failed.push(request.url()));
  await ready(page);
  const revision = await apply(page, ['w-[137px]', 'h-[72px]', 'bg-[#123456]', 'text-[#ffffff]', 'rounded-[8px]', 'p-[7px]', 'border-2', 'border-[#abcdef]']);
  const state = await drawn(page, revision);
  expect(state.button.status).toBe('applied');
  expect(state.button.computed.padding).toEqual([7, 7, 7, 7]);
  expect(state.button.computed.border_widths).toEqual([2, 2, 2, 2]);
  expect(state.button.geometry.slice(2)).toEqual([137, 72]);
  expect(state.button.computed.font.resource_path).toContain('IBMPlexSans');
  await expect(page.locator('#scope-a-sample')).toHaveCSS('width', '137px');
  await expect(page.locator('#scope-a-sample')).toHaveCSS('padding-top', '7px');
  const pixels = await page.getByTestId('utility-gpu').screenshot();
  await info.attach('GPU canvas pixels', {body: pixels, contentType: 'image/png'});
  await info.attach('GPU actual drawn report', {body: JSON.stringify(state, null, 2), contentType: 'application/json'});
  const png = PNG.sync.read(pixels);
  const [x, y, w, h] = state.button.geometry;
  expect(pixel(png, x + 8, y + 8).slice(0, 3)).toEqual([18, 52, 86]);
  const border = pixel(png, x + w / 2 + 1, y + 1);
  expect(border.slice(0, 3)).toEqual([171, 205, 239]);
  let whiteInk = 0;
  for (let py = y + 12; py < y + h - 12; py++) for (let px = x + 8; px < x + w - 8; px++) {
    if (pixel(png, px, py).slice(0, 3).every(value => value > 220)) whiteInk++;
  }
  expect(whiteInk).toBeGreaterThan(15);
  expect(errors).toEqual([]); expect(failed).toEqual([]);
  await page.screenshot({path: info.outputPath('desktop.png'), fullPage: true});
  await page.setViewportSize({width: 390, height: 844});
  await page.getByTestId('utility-gpu').scrollIntoViewIfNeeded();
  await page.evaluate(async () => { await document.fonts.ready; await new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve))); });
  expect(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth)).toBe(true);
  await page.screenshot({path: info.outputPath('mobile.png'), fullPage: true});
});

test('CSS functions resolve runtime candidates once and drive actual DOM and GPU pixels', async ({page}, info) => {
  const errors: string[] = [], failed: string[] = [];
  page.on('pageerror', error => errors.push(error.message));
  page.on('requestfailed', request => failed.push(request.url()));
  await ready(page);
  const classes = ['w-[--theme(--missing,128px)]', 'h-[theme(spacing.12)]', 'p-[theme(spacing.2)]',
    'bg-[color:--alpha(#123456_/_1)]', 'text-[color:--alpha(#ffffff_/_1)]',
    'rounded-[--theme(--missing,8px)]', 'border-[length:--theme(--missing,2px)]',
    'border-[color:--alpha(#abcdef_/_1)]'];
  const revision = await apply(page, classes);
  const state = await drawn(page, revision);
  expect(state.button.status).toBe('applied');
  expect(state.button.geometry.slice(2)).toEqual([128, 48]);
  expect(state.button.computed.padding).toEqual([8, 8, 8, 8]);
  expect(state.button.computed.border_widths).toEqual([2, 2, 2, 2]);
  const sample = page.locator('#scope-a-sample');
  await expect(sample).toHaveCSS('width', '128px');
  await expect(sample).toHaveCSS('height', '48px');
  await expect(sample).toHaveCSS('padding-top', '8px');
  await expect(sample).toHaveCSS('background-color', 'rgb(18, 52, 86)');
  const png = PNG.sync.read(await page.getByTestId('utility-gpu').screenshot());
  const [x, y, width] = state.button.geometry;
  expect(pixel(png, x + 8, y + 8).slice(0, 3)).toEqual([18, 52, 86]);
  expect(pixel(png, x + width / 2 + 1, y + 1).slice(0, 3)).toEqual([171, 205, 239]);
  const repeated = await page.evaluate(async classes => {
    const original = Worker.prototype.postMessage;
    let prepares = 0;
    Worker.prototype.postMessage = function(this: Worker, message: any, options?: any) {
      if (message.operation === 'prepare') prepares++;
      return original.call(this, message, options);
    };
    try { return {report: await (window as any).__utility_lab.ensure('scope-a', classes), prepares}; }
    finally { Worker.prototype.postMessage = original; }
  }, classes);
  expect(repeated.report.revision).toBe(revision);
  expect(repeated.prepares).toBe(0);
  expect(errors).toEqual([]); expect(failed).toEqual([]);
  await info.attach('CSS functions drawn GPU report', {body: JSON.stringify(state, null, 2), contentType: 'application/json'});
  await page.screenshot({path: info.outputPath('gpu-css-functions.png'), fullPage: true});
});

test('CSS-native utilities drive scoped DOM and GPU while nested branches retain the drawn style', async ({page}, info) => {
  const errors: string[] = [], failed: string[] = [];
  page.on('pageerror', error => errors.push(error.message));
  page.on('requestfailed', request => failed.push(request.url()));
  await ready(page);
  await page.evaluate(() => {
    const api = (window as any).__utility_lab;
    api.dispose('scope-a');
    api.mount('scope-a', 10000, '@theme { --size-card:137px; --color-card:#123456; } @utility fixture-* { width:--value(--size,[length]); height:72px; background-color:--modifier(--color,[color]); color:white; } @utility nested-card { &:hover { border:2px solid #abcdef; } } @tailwind utilities;');
  });
  await expect(page.locator('#gpu-status')).toHaveText('GPU ready');
  const first = await apply(page, ['fixture-card/card']);
  const before = await drawn(page, first);
  expect(before.button.status).toBe('applied');
  expect(before.button.geometry.slice(2)).toEqual([137, 72]);
  const sample = page.locator('#scope-a-sample');
  await expect(sample).toHaveCSS('width', '137px');
  await expect(sample).toHaveCSS('background-color', 'rgb(18, 52, 86)');
  const png = PNG.sync.read(await page.getByTestId('utility-gpu').screenshot());
  const [x, y] = before.button.geometry;
  expect(pixel(png, x + 8, y + 8).slice(0, 3)).toEqual([18, 52, 86]);
  await page.screenshot({path: info.outputPath('custom-utility.png'), fullPage: true});
  const next = await apply(page, ['fixture-card/card', 'nested-card']);
  await sample.hover();
  await expect(sample).toHaveCSS('border-top-color', 'rgb(171, 205, 239)');
  const after = await report(page);
  expect(after.button.status).toBe('rejected');
  expect(after.button.attempted_revision).toBe(next);
  expect(after.button.applied_revision).toBe(first);
  expect(after.button.drawn_revision).toBe(first);
  expect(after.button.computed).toEqual(before.button.computed);
  expect(errors).toEqual([]); expect(failed).toEqual([]);
  await info.attach('Custom utility DOM and retained GPU report', {body: JSON.stringify({before, after}, null, 2), contentType: 'application/json'});
});

test('authored custom variants apply DOM hover and preserve the last drawn GPU style on rejection', async ({page}, info) => {
  const errors: string[] = [], failed: string[] = [];
  page.on('pageerror', error => errors.push(error.message));
  page.on('requestfailed', request => failed.push(request.url()));
  await ready(page);
  await page.evaluate(() => {
    const api = (window as any).__utility_lab;
    api.dispose('scope-a');
    api.mount('scope-a', 10000, '@custom-variant intent (&:hover); @utility application-card { width:137px; height:72px; background-color:#123456; @variant intent { border:2px solid #abcdef; } } @tailwind utilities;');
  });
  await expect(page.locator('#gpu-status')).toHaveText('GPU ready');
  const first = await apply(page, ['w-[137px]', 'h-[72px]', 'bg-[#123456]']);
  const before = await drawn(page, first);
  const next = await apply(page, ['application-card']);
  const sample = page.locator('#scope-a-sample');
  await expect(sample).toHaveCSS('width', '137px');
  await expect(sample).toHaveCSS('height', '72px');
  await expect(sample).toHaveCSS('background-color', 'rgb(18, 52, 86)');
  await sample.hover();
  await expect(sample).toHaveCSS('border-top-color', 'rgb(171, 205, 239)');
  await expect(sample).toHaveCSS('border-top-width', '2px');
  const after = await report(page);
  expect(after.button.status).toBe('rejected');
  expect(after.button.attempted_revision).toBe(next);
  expect(after.button.applied_revision).toBe(first);
  expect(after.button.drawn_revision).toBe(first);
  expect(after.button.computed).toEqual(before.button.computed);
  const png = PNG.sync.read(await page.getByTestId('utility-gpu').screenshot());
  const [x, y] = after.button.geometry;
  expect(pixel(png, x + 8, y + 8).slice(0, 3)).toEqual([18, 52, 86]);
  await page.mouse.move(0, 0);
  await expect(sample).not.toHaveCSS('border-top-color', 'rgb(171, 205, 239)');
  expect(errors).toEqual([]); expect(failed).toEqual([]);
  await info.attach('Custom variant DOM and retained GPU report', {body: JSON.stringify({before, after}, null, 2), contentType: 'application/json'});
  await page.screenshot({path: info.outputPath('custom-variant.png'), fullPage: true});
});

test('arbitrary properties with modifiers drive actual DOM and GPU pixels', async ({page}, info) => {
  await ready(page);
  const revision = await apply(page, ['[width:137px]/[1]', '[height:72px]/[1]', '[background-color:#123456]/100', '[color:#ffffff]/100']);
  const state = await drawn(page, revision);
  expect(state.button.status).toBe('applied');
  expect(state.button.geometry.slice(2)).toEqual([137, 72]);
  await expect(page.locator('#scope-a-sample')).toHaveCSS('width', '137px');
  await expect(page.locator('#scope-a-sample')).toHaveCSS('background-color', 'rgb(18, 52, 86)');
  const png = PNG.sync.read(await page.getByTestId('utility-gpu').screenshot());
  const [x, y] = state.button.geometry;
  expect(pixel(png, x + 8, y + 8).slice(0, 3)).toEqual([18, 52, 86]);
  await page.screenshot({path: info.outputPath('arbitrary-property.png'), fullPage: true});
  await info.attach('Arbitrary property actual GPU report', {body: JSON.stringify(state, null, 2), contentType: 'application/json'});
});

test('mask CSS applies to DOM while GPU rejects its unsupported property and retains its drawn style', async ({page}, info) => {
  await ready(page);
  const first = await apply(page, ['w-[137px]', 'bg-[#123456]']);
  const before = await drawn(page, first);
  const next = await apply(page, ['w-[137px]', 'bg-[#123456]', 'mask-[linear-gradient(#123456,#abcdef)]']);
  await expect(page.locator('#scope-a-sample')).toHaveCSS('mask-image', 'linear-gradient(rgb(18, 52, 86), rgb(171, 205, 239))');
  const after = await report(page);
  expect(after.button.attempted_revision).toBe(next);
  expect(after.button.status).toBe('rejected');
  expect(after.button.applied_revision).toBe(first);
  expect(after.button.drawn_revision).toBe(first);
  expect(after.button.computed).toEqual(before.button.computed);
  expect(after.button.diagnostics.some((item: any) => item.property === 'mask-image')).toBe(true);
  await info.attach('Mask CSS with retained GPU report', {body: JSON.stringify(after, null, 2), contentType: 'application/json'});
});

test('strict GPU rejection retains the drawn style, partial reports skipped properties and deleting classes restores defaults', async ({page}) => {
  await ready(page);
  const defaults = (await report(page)).button.computed;
  const first = await apply(page, ['w-[137px]', 'bg-[#123456]']);
  const before = await drawn(page, first);
  const rejected = await apply(page, ['w-[179px]', 'bg-[#abcdef]', 'opacity-50']);
  const retained = await report(page);
  expect(retained.button.status).toBe('rejected');
  expect(retained.button.applied_revision).toBe(first);
  expect(retained.button.drawn_revision).toBe(first);
  expect(retained.button.computed).toEqual(before.button.computed);
  expect(retained.button.diagnostics.some((item: any) => item.property === 'opacity')).toBe(true);
  await apply(page, ['w-[179px]', 'bg-[#abcdef]', 'opacity-50'], [[], [], []], true);
  const partial = await drawn(page, rejected);
  expect(partial.button.status).toBe('partial');
  expect(partial.button.computed.width.value).toBe(179);
  expect(partial.button.computed.opacity).toBe(1);
  const restored = await apply(page, []);
  const reset = await drawn(page, restored);
  expect(reset.button.status).toBe('applied');
  expect(reset.button.computed).toEqual(defaults);
  await expect(page.locator('#scope-a-sample')).toHaveCSS('width', '170px');
  await expect(page.locator('#scope-a-sample')).toHaveCSS('height', '36px');
});

test('GPU responds to pointer, keyboard focus-visible and disabled input without recompiling', async ({page}) => {
  await ready(page);
  const revision = await apply(page, ['w-[137px]', 'h-[72px]', 'bg-[#123456]', 'hover:bg-[#abcdef]', 'focus-visible:bg-[#fedcba]', 'active:bg-[#234567]', 'disabled:bg-[#999999]']);
  let state = await drawn(page, revision);
  const canvas = page.getByTestId('utility-gpu');
  const bounds = (await canvas.boundingBox())!;
  const [x, y, w, h] = state.button.geometry;
  await page.mouse.move(bounds.x + x + w / 2 + 1, bounds.y + y + h / 2 + 1);
  await expect.poll(async () => (await report(page)).button.state.hover).toBe(true);
  expect((await report(page)).button.computed.background.slice(0, 3)).toEqual([171/255, 205/255, 239/255]);
  await page.mouse.move(0, 0);
  await expect.poll(async () => (await report(page)).button.state.hover).toBe(false);
  await page.locator('#scope-a-sample').focus();
  await expect.poll(async () => (await report(page)).button.state.focus_visible).toBe(true);
  expect((await report(page)).button.computed.background.slice(0, 3)).toEqual([254/255, 220/255, 186/255]);
  await page.keyboard.down('Space');
  await expect.poll(async () => (await report(page)).button.state.active).toBe(true);
  await page.keyboard.up('Space');
  await expect(page.locator('#gpu-revision')).toContainText('1 clicks');
  await apply(page, ['w-[137px]', 'h-[72px]', 'bg-[#123456]', 'disabled:bg-[#999999]'], [[], [], []], false, true);
  await expect.poll(async () => (await report(page)).button.state.disabled).toBe(true);
  expect((await report(page)).button.state.active).toBe(false);
  await page.keyboard.press('Space');
  await expect(page.locator('#gpu-revision')).toContainText('1 clicks');
  expect((await report(page)).button.applied_revision).toBe(revision);
});

test('GPU media uses the browser viewport, and alpha color blocks match DOM pixels', async ({page}) => {
  await page.setViewportSize({width: 1000, height: 1000});
  await ready(page);
  const revision = await apply(page, ['w-[89px]', 'min-[768px]:w-[149px]', 'bg-[#123456]'], [[], [], ['bg-[#c82850]/50', 'opacity-50']]);
  let state = await drawn(page, revision);
  expect(state.button.geometry[2]).toBe(149);
  await expect(page.locator('#scope-a-sample')).toHaveCSS('width', '149px');
  const png = PNG.sync.read(await page.getByTestId('utility-gpu').screenshot());
  const [x, y, w, h] = state.swatch.geometry;
  const gpu = pixel(png, x + w / 2 + 1, y + h / 2 + 1);
  const domPng = PNG.sync.read(await page.locator('#scope-a-swatch').screenshot());
  const dom = pixel(domPng, domPng.width / 2, domPng.height / 2);
  gpu.slice(0, 3).forEach((value, index) => expect(Math.abs(value - dom[index]), `alpha ${gpu}, ${dom}`).toBeLessThanOrEqual(2));
  await page.setViewportSize({width: 390, height: 844});
  await page.getByTestId('utility-gpu').scrollIntoViewIfNeeded();
  await expect.poll(async () => (await report(page)).button.geometry[2]).toBe(89);
  await expect(page.locator('#scope-a-sample')).toHaveCSS('width', '89px');
  expect((await report(page)).button.applied_revision).toBe(revision);
});

test('GPU applies text resources, row layout, shadows and rings from compiler bindings', async ({page}) => {
  await page.setViewportSize({width: 1280, height: 1000});
  await ready(page);
  const revision = await apply(page, ['w-[100px]', 'h-[60px]', 'bg-[#123456]', 'ring-[3px]', 'ring-[#abcdef]'],
    [['flex', 'flex-row', 'gap-[11px]', 'p-[9px]', 'items-center'], ["font-['Liberation_Mono']", 'text-[20px]', 'leading-[28px]', 'tracking-[0.05em]'], []]);
  const state = await drawn(page, revision, 'text');
  expect(state.text.status).toBe('applied');
  expect(state.text.computed.font.family).toBe('Liberation Mono');
  expect(state.text.computed.font_size_px).toBe(20);
  expect(state.text.computed.line_height.value).toBe(28);
  expect(state.text.computed.letter_spacing_em).toBe(.05);
  expect(state.container.computed.gap).toEqual([11, 11]);
  expect(state.container.computed.flow).toBe('row');
  expect(state.text.geometry[0] - state.button.geometry[0] - state.button.geometry[2]).toBeCloseTo(11);
  expect(state.swatch.geometry[0] - state.text.geometry[0] - state.text.geometry[2]).toBeCloseTo(11);
  expect(state.button.computed.shadows.some((shadow: any) => shadow.spread === 3)).toBe(true);
  await expect(page.locator('#scope-a-text')).toHaveCSS('font-family', '"Liberation Mono"');
  await expect(page.locator('#scope-a-text')).toHaveCSS('font-size', '20px');
  const png = PNG.sync.read(await page.getByTestId('utility-gpu').screenshot());
  const [x, y, w] = state.button.geometry;
  const ring = pixel(png, x + w / 2 + 1, y - 1);
  [171, 205, 239].forEach((value, index) => expect(Math.abs(ring[index] - value)).toBeLessThanOrEqual(3));
});

// The lines of an element's own text: words, inked extent and vertical centre
// relative to its border box. Character boxes span the font's content area,
// which CSS centres in the line box exactly as the GPU centres its glyphs.
const domLines = (page: Page, id: string) => page.evaluate(async id => {
  const element = document.getElementById(id)!;
  // The faces swap in when first used; measure the element's own face.
  await document.fonts.load(getComputedStyle(element).font);
  await new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve)));
  const box = element.getBoundingClientRect();
  const lines: {text: string, left: number, right: number, top: number, bottom: number}[] = [];
  const range = document.createRange();
  const walker = document.createTreeWalker(element, NodeFilter.SHOW_TEXT);
  for (let node = walker.nextNode(); node; node = walker.nextNode()) {
    const text = node.textContent!;
    for (let index = 0; index < text.length; index++) {
      const last = lines[lines.length - 1];
      if (/\s/.test(text[index])) { if (last) last.text += ' '; continue; }
      range.setStart(node, index);
      range.setEnd(node, index + 1);
      const rect = range.getBoundingClientRect();
      if (last && Math.abs(rect.top - last.top) < 1) {
        last.text += text[index];
        last.right = Math.max(last.right, rect.right);
      } else {
        lines.push({text: text[index], left: rect.left, right: rect.right, top: rect.top, bottom: rect.bottom});
      }
    }
  }
  return lines.map(line => ({text: line.text.trim().replace(/\s+/g, ' '), x: line.left - box.left,
    width: line.right - line.left, centre: (line.top + line.bottom) / 2 - box.top}));
}, id);
const gpuLines = (node: any) => node.text_rows.map((row: any) => ({text: row.text, x: row.x - node.geometry[0],
  width: row.width, centre: row.top + row.height / 2 - node.geometry[1]}));
function sameLines(dom: any[], gpu: any[]) {
  expect(gpu.map(line => line.text)).toEqual(dom.map(line => line.text));
  dom.forEach((line, index) => {
    expect(Math.abs(gpu[index].x - line.x)).toBeLessThanOrEqual(1);
    expect(Math.abs(gpu[index].width - line.width)).toBeLessThanOrEqual(1.5);
    expect(Math.abs(gpu[index].centre - line.centre)).toBeLessThanOrEqual(1);
  });
}

test('GPU text wraps, leads and tracks on the same lines as the DOM', async ({page}, info) => {
  const errors: string[] = [], failed: string[] = [];
  page.on('pageerror', error => errors.push(error.message));
  page.on('requestfailed', request => failed.push(request.url()));
  await page.setViewportSize({width: 1280, height: 1000});
  await ready(page);
  const measured: Record<string, unknown> = {};
  const narrow = await apply(page, ['w-[137px]', 'h-[72px]', 'p-[7px]', 'bg-[#123456]', 'text-[#ffffff]']);
  const wrapped = await drawn(page, narrow);
  measured.button = {dom: await domLines(page, 'scope-a-sample'), gpu: gpuLines(wrapped.button)};
  expect(wrapped.button.status).toBe('applied');
  expect(wrapped.button.text_rows).toHaveLength(2);
  sameLines((measured.button as any).dom, (measured.button as any).gpu);

  const wide = await apply(page, ['w-[220px]', 'h-[72px]', 'p-[7px]', 'bg-[#123456]', 'text-[#ffffff]']);
  const single = await drawn(page, wide);
  measured.single = {dom: await domLines(page, 'scope-a-sample'), gpu: gpuLines(single.button)};
  expect(single.button.text_rows).toHaveLength(1);
  sameLines((measured.single as any).dom, (measured.single as any).gpu);
  await page.getByTestId('utility-gpu').screenshot({path: info.outputPath('wrapped-button.png')});

  const tracked = await apply(page, ['w-[220px]', 'h-[72px]'], [[], ["font-['Liberation_Mono']", 'text-[20px]', 'leading-[28px]', 'tracking-[0.05em]', 'w-[100px]'], []]);
  const text = await drawn(page, tracked, 'text');
  measured.text = {dom: await domLines(page, 'scope-a-text'), gpu: gpuLines(text.text)};
  expect(text.text.status).toBe('applied');
  expect(text.text.text_rows).toHaveLength(2);
  sameLines((measured.text as any).dom, (measured.text as any).gpu);
  const gpuText = gpuLines(text.text);
  expect(gpuText[1].centre - gpuText[0].centre).toBeCloseTo(28, 5);
  const domHeight = await page.locator('#scope-a-text').evaluate(element => element.getBoundingClientRect().height);
  expect(text.text.geometry[3]).toBeCloseTo(domHeight, 5);
  await info.attach('DOM and GPU line boxes', {body: JSON.stringify(measured, null, 2), contentType: 'application/json'});
  await page.getByTestId('utility-gpu').screenshot({path: info.outputPath('wrapped-text.png')});
  await page.locator('#scope-a-frame').screenshot({path: info.outputPath('wrapped-text-dom.png')});
  expect(errors).toEqual([]); expect(failed).toEqual([]);
});

test('GPU ring offset, ring and shadow layers paint the same bands as the DOM', async ({page}, info) => {
  const errors: string[] = [], failed: string[] = [];
  page.on('pageerror', error => errors.push(error.message));
  page.on('requestfailed', request => failed.push(request.url()));
  await page.setViewportSize({width: 1280, height: 1000});
  await ready(page);
  const revision = await apply(page, ['w-[120px]', 'h-[48px]', 'bg-[#123456]', 'ring-[3px]', 'ring-[#abcdef]',
    'ring-offset-[4px]', 'ring-offset-[#fedcba]', 'shadow-[0_12px_0_0_#234567]']);
  const state = await drawn(page, revision);
  expect(state.button.status).toBe('applied');
  const gpu = PNG.sync.read(await page.getByTestId('utility-gpu').screenshot());
  const sample = (await page.locator('#scope-a-sample').boundingBox())!;
  const dom = PNG.sync.read(await page.screenshot({clip: {x: sample.x - 20, y: sample.y - 20, width: sample.width + 40, height: sample.height + 40}}));
  const [x, y, w, h] = state.button.geometry;
  // Offsets from the border box at its horizontal centre, away from corners.
  const bands: [string, number, number[]][] = [
    ['face', 24, [18, 52, 86]],
    ['offset above', -2, [254, 220, 186]],
    ['ring above', -5.5, [171, 205, 239]],
    ['clear above', -10, [255, 255, 255]],
    ['offset below', h + 2, [254, 220, 186]],
    ['ring below', h + 5.5, [171, 205, 239]],
    ['shadow below', h + 9.5, [35, 69, 103]],
    ['clear below', h + 14, [255, 255, 255]],
  ];
  const measured = bands.map(([name, offset, expected]) => {
    const fromGpu = pixel(gpu, x + w / 2 + 1, y + offset).slice(0, 3);
    const fromDom = pixel(dom, 20 + sample.width / 2 + 1, 20 + offset).slice(0, 3);
    expected.forEach((value, index) => {
      expect(Math.abs(fromGpu[index] - value), `GPU ${name} ${fromGpu}`).toBeLessThanOrEqual(3);
      expect(Math.abs(fromDom[index] - value), `DOM ${name} ${fromDom}`).toBeLessThanOrEqual(3);
    });
    return {name, offset, gpu: fromGpu, dom: fromDom};
  });
  await info.attach('Shadow band pixels', {body: JSON.stringify(measured, null, 2), contentType: 'application/json'});
  await page.getByTestId('utility-gpu').screenshot({path: info.outputPath('shadow-bands-gpu.png')});
  await page.screenshot({path: info.outputPath('shadow-bands-dom.png'), clip: {x: sample.x - 20, y: sample.y - 20, width: sample.width + 40, height: sample.height + 40}});
  expect(errors).toEqual([]); expect(failed).toEqual([]);
});

test('GPU per-side spacing, size, min/max and column alignment place boxes where the DOM does', async ({page}, info) => {
  await page.setViewportSize({width: 1280, height: 1000});
  await ready(page);
  const container = ['w-[300px]', 'h-[240px]', 'flex-col', 'items-center', 'justify-end', 'pt-[10px]', 'pr-[20px]', 'pb-[30px]', 'pl-[40px]', 'gap-[6px]'];
  const revision = await apply(page, ['size-[64px]', 'bg-[#123456]'], [container, ['px-[3px]', 'py-[5px]'], ['w-[50px]', 'max-w-[30px]', 'h-[4px]', 'min-h-[10px]']]);
  const state = await drawn(page, revision);
  for (const slot of ['container', 'button', 'text', 'swatch']) expect(state[slot].status, slot).toBe('applied');
  expect(state.container.computed.padding).toEqual([10, 20, 30, 40]);
  expect(state.text.computed.padding).toEqual([5, 3, 5, 3]);
  const boxes = await page.evaluate(() => {
    const frame = document.getElementById('scope-a-frame')!.getBoundingClientRect();
    return Object.fromEntries(['sample', 'text', 'swatch'].map(id => {
      const box = document.getElementById(`scope-a-${id}`)!.getBoundingClientRect();
      return [id, [box.left - frame.left, box.top - frame.top, box.width, box.height]];
    }));
  });
  const [cx, cy] = state.container.geometry;
  const measured = {dom: boxes, gpu: {} as Record<string, number[]>};
  for (const [slot, id] of [['button', 'sample'], ['text', 'text'], ['swatch', 'swatch']]) {
    const [x, y, w, h] = state[slot].geometry;
    measured.gpu[id] = [x - cx, y - cy, w, h];
    measured.gpu[id].forEach((value, index) => expect(Math.abs(value - boxes[id][index]), `${slot}[${index}] ${measured.gpu[id]} vs ${boxes[id]}`).toBeLessThanOrEqual(1));
  }
  expect(measured.gpu.sample.slice(2)).toEqual([64, 64]);
  expect(measured.gpu.swatch.slice(2)).toEqual([30, 10]);
  // justify-end: the last box ends at the bottom padding edge.
  expect(measured.gpu.swatch[1] + measured.gpu.swatch[3]).toBeCloseTo(240 - 30, 5);
  await info.attach('Container geometry', {body: JSON.stringify(measured, null, 2), contentType: 'application/json'});
  await page.getByTestId('utility-gpu').screenshot({path: info.outputPath('column-geometry-gpu.png')});
  await page.locator('#scope-a-frame').screenshot({path: info.outputPath('column-geometry-dom.png')});

  // Only weight 400 faces are packaged for GPU text; a heavier weight is a
  // visible diagnostic rather than synthetic bold.
  const weighted = await apply(page, ['size-[64px]', 'bg-[#123456]'], [container, ['font-[600]'], []]);
  await expect.poll(async () => (await report(page)).text.attempted_revision).toBe(weighted);
  const rejected = await report(page);
  expect(rejected.text.status).toBe('rejected');
  expect(rejected.text.diagnostics.some((item: any) => item.property === 'font-weight')).toBe(true);
  expect(rejected.text.applied_revision).toBe(revision);
  expect(rejected.text.computed).toEqual(state.text.computed);
  await expect(page.locator('#scope-a-text')).toHaveCSS('font-weight', '600');
});

test('GPU restores its latest committed programs after a real WebGL context loss', async ({page}) => {
  await ready(page);
  const first = await apply(page, ['w-[137px]', 'bg-[#123456]']);
  await drawn(page, first);
  await page.getByTestId('utility-gpu').evaluate((canvas: HTMLCanvasElement) => {
    const gl = canvas.getContext('webgl2')!;
    const extension = gl.getExtension('WEBGL_lose_context')!;
    (window as any).__utility_loss = extension; extension.loseContext();
  });
  await expect(page.locator('#gpu-status')).toHaveText('Restoring GPU…');
  const next = await page.evaluate(async () => (await (window as any).__utility_lab.gpuApply('scope-a', [[], ['w-[179px]', 'bg-[#abcdef]'], [], []])).revision);
  await page.evaluate(() => { (window as any).__utility_loss.restoreContext(); delete (window as any).__utility_loss; });
  await expect(page.locator('#gpu-status')).toHaveText('GPU ready');
  const state = await drawn(page, next);
  expect(state.button.computed.width.value).toBe(179);
  expect(state.button.computed.background.slice(0, 3)).toEqual([171/255, 205/255, 239/255]);
  expect(await page.getByTestId('utility-gpu').count()).toBe(1);
});

test('terminal GPU initialization failure explains recovery while DOM editing remains usable', async ({page}, info) => {
  await page.addInitScript(() => {
    const original = HTMLCanvasElement.prototype.getContext;
    HTMLCanvasElement.prototype.getContext = function(type: any, ...args: any[]) {
      if (type === 'webgl2') return null;
      return original.apply(this, [type, ...args] as any);
    } as any;
  });
  await page.goto('./');
  await expect(page.locator('#status')).toHaveAttribute('data-status', 'ready');
  await expect(page.locator('#gpu-status')).toContainText('GPU unavailable: no WebGL2 context');
  await expect(page.locator('#gpu-status')).toContainText('Copy your class inputs, then reload');
  await page.locator('#classes').fill('w-[179px] bg-[#abcdef]');
  await page.locator('#apply').click();
  await expect(page.locator('#status')).toContainText('DOM styles installed.');
  await expect(page.locator('#scope-a-sample')).toHaveCSS('width', '179px');
  await page.screenshot({path: info.outputPath('gpu-terminal-failure.png'), fullPage: true});
});

test('all non-button GPU boundaries are visible and the desktop comparison frames align', async ({page}, info) => {
  await ready(page);
  await apply(page, ['w-[137px]', 'bg-[#123456]'], [['opacity-50'], ['bg-[#abcdef]', 'opacity-50'], ['grid']]);
  await expect(page.locator('#gpu-results')).toContainText('Container: rejected');
  await expect(page.locator('#gpu-results')).toContainText('Text: rejected');
  await expect(page.locator('#gpu-results')).toContainText('Color block: rejected');
  await expect(page.locator('#gpu-warning')).toContainText('opacity-50 (opacity)');
  await expect(page.locator('#gpu-warning')).toContainText('grid (display)');
  await expect(page.locator('#gpu-warning')).toContainText('Remove the class');
  const dom = (await page.locator('#scope-a.preview').boundingBox())!;
  const gpu = (await page.getByTestId('utility-gpu').boundingBox())!;
  expect(Math.abs(dom.y - gpu.y)).toBeLessThanOrEqual(1);
  await page.screenshot({path: info.outputPath('gpu-boundaries.png'), fullPage: true});
  await apply(page, ['w-[137px]', 'bg-[#123456]'], [['opacity-50'], ['bg-[#abcdef]', 'opacity-50'], ['grid']], true);
  await expect(page.locator('#gpu-results')).toContainText('Container: partial');
  await expect(page.locator('#gpu-results')).toContainText('Text: partial');
  await expect(page.locator('#gpu-results')).toContainText('Color block: partial');
});

test('live ThemeScope and ThemeBoundary update colors, dark conditions and host rem without recompiling', async ({page}, info) => {
  await page.setViewportSize({width: 1000, height: 900});
  await ready(page);
  const baseline = (await report(page)).button.computed;
  const revision = await apply(page, ['w-[5rem]', 'bg-(--primary)', 'text-(--primary-foreground)', 'dark:border-2', 'dark:border-[#123456]', 'min-[60rem]:h-[73px]'], [['gap-[1rem]'], ['text-(--foreground)'], ['bg-(--primary)']]);
  await drawn(page, revision);
  await expect(page.locator('#scope-a-sample')).toHaveCSS('width', '80px');
  expect((await report(page)).button.computed.height.value).toBe(73);
  const installed = await page.evaluate(() => (window as any).__utility_lab.report('scope-a'));
  const sibling = await page.locator('#scope-b-sample').evaluate(node => getComputedStyle(node).backgroundColor);
  await page.emulateMedia({colorScheme: 'dark'});
  await page.evaluate(() => {
    document.documentElement.style.fontSize = '20px';
    (window as any).__utility_lab.theme('scope-a', true, 20, {primary: '#abcdef'});
  });
  await expect(page.locator('#scope-a')).toHaveAttribute('data-theme', 'dark');
  await expect(page.getByTestId('utility-theme-boundary')).toHaveCSS('--primary', 'rgb(171 205 239)');
  await expect(page.locator('#scope-a-sample')).toHaveCSS('width', '100px');
  await expect(page.locator('#scope-a-sample')).toHaveCSS('background-color', 'rgb(171, 205, 239)');
  await expect.poll(async () => (await report(page)).button.computed.background).toEqual([171 / 255, 205 / 255, 239 / 255, 1]);
  await expect.poll(async () => (await report(page)).button.computed.width.value).toBe(100);
  expect((await report(page)).button.computed.height.value).toBe(73);
  expect((await report(page)).button.computed.border_widths).toEqual([2, 2, 2, 2]);
  expect((await report(page)).container.computed.gap).toEqual([20, 20]);
  await expect(page.locator('#scope-b-sample')).toHaveCSS('background-color', sibling);
  expect(await page.evaluate(() => (window as any).__utility_lab.report('scope-a'))).toEqual(installed);
  const retained = (await report(page)).button.computed;
  const rejected = await page.evaluate(() => {
    try { (window as any).__utility_lab.theme('scope-a', false, 16, {radius: 'invalid'}); return false; }
    catch { return true; }
  });
  expect(rejected).toBe(true);
  expect((await report(page)).button.computed).toEqual(retained);
  const actual = PNG.sync.read(await page.getByTestId('utility-gpu').screenshot());
  const bounds = (await report(page)).button.geometry;
  expect(pixel(actual, bounds[0] + 5, bounds[1] + 5)).toEqual([171, 205, 239, 255]);
  await info.attach('Theme actual drawn report', {body: JSON.stringify(await report(page), null, 2), contentType: 'application/json'});
  await page.screenshot({path: info.outputPath('gpu-theme-boundary.png'), fullPage: true});
  await page.evaluate(() => (window as any).__utility_lab.theme('scope-a', true, 20));
  await expect(page.getByTestId('utility-theme-boundary')).not.toHaveAttribute('data-rustify-theme-runtime');
  await expect.poll(async () => (await report(page)).button.computed.background).not.toEqual(retained.background);
  await page.emulateMedia({colorScheme: 'light'});
  await page.evaluate(() => {
    document.documentElement.style.fontSize = '';
    (window as any).__utility_lab.theme('scope-a', false, 16);
  });
  await expect(page.locator('#scope-a')).toHaveAttribute('data-theme', 'light');
  const restored = await apply(page, []);
  await drawn(page, restored);
  expect((await report(page)).button.computed.background).toEqual(baseline.background);
  expect((await report(page)).button.computed.padding).toEqual(baseline.padding);
});

test('configuration replacement publishes new DOM and GPU programs and retains pixels on failed installation', async ({page}, info) => {
  const errors: string[] = [], failed: string[] = [];
  page.on('pageerror', error => errors.push(error.message));
  page.on('requestfailed', request => failed.push(request.url()));
  await ready(page);
  const classes = ['w-[121px]', 'h-[61px]', 'bg-brand', 'text-white', 'p-4'];
  const css = (color: string, spacing: string) => `@theme inline { --color-brand: ${color}; --color-white: #ffffff; --spacing: ${spacing}rem; } @tailwind utilities;`;
  await page.evaluate(css => (window as any).__utility_lab.replace('scope-a', css), css('#123456', '0.25'));
  const old = await drawn(page, await apply(page, classes));
  await expect(page.locator('#scope-a-sample')).toHaveCSS('background-color', 'rgb(18, 52, 86)');
  await expect(page.locator('#scope-a-sample')).toHaveCSS('padding', '16px');
  const changed = await page.evaluate(css => (window as any).__utility_lab.replace('scope-a', css), css('#abcdef', '0.5'));
  expect(changed.revision).toBeGreaterThan(old.button.drawn_revision);
  const next = await drawn(page, await apply(page, classes));
  expect(next.button.drawn_revision).toBe(changed.revision);
  await expect(page.locator('#scope-a-sample')).toHaveCSS('background-color', 'rgb(171, 205, 239)');
  await expect(page.locator('#scope-a-sample')).toHaveCSS('padding', '32px');
  const canvas = page.getByTestId('utility-gpu');
  const rgba = () => canvas.screenshot().then(buffer => {
    const [x, y] = next.button.geometry;
    return pixel(PNG.sync.read(buffer), x + 8, y + 8).slice(0, 3);
  });
  await expect.poll(rgba).toEqual([171, 205, 239]);
  const rejected = await page.evaluate(async css => {
    const api = (window as any).__utility_lab;
    const original = CSSStyleSheet.prototype.replaceSync;
    CSSStyleSheet.prototype.replaceSync = function () { throw new Error('injected GPU configuration install failure'); };
    let failure;
    try { await api.replace('scope-a', css); }
    catch (error) { failure = JSON.parse(String(error)); }
    finally { CSSStyleSheet.prototype.replaceSync = original; }
    return {failure, revision: api.report('scope-a').revision};
  }, css('#fedcba', '1'));
  expect(rejected.failure.kind).toBe('install');
  expect(rejected.revision).toBe(changed.revision);
  expect((await report(page)).button.drawn_revision).toBe(changed.revision);
  await expect.poll(rgba).toEqual([171, 205, 239]);
  await expect(page.locator('#scope-a-sample')).toHaveCSS('background-color', 'rgb(171, 205, 239)');
  expect(errors).toEqual([]); expect(failed).toEqual([]);
  await info.attach('Configuration DOM/GPU commit and rollback', {body: JSON.stringify({old, next, rejected}, null, 2), contentType: 'application/json'});
  await page.screenshot({path: info.outputPath('configuration-gpu.png'), fullPage: true});
});
