import {test, expect} from '@playwright/test';

test.use({javaScriptEnabled: false});

for (const profile of ['index.html', 'p3.html']) {
  test(`${profile}: script-free scopes, explicit modes, nested roots and fallback`, async ({page}, info) => {
    const requests: string[] = [], failures: string[] = [], errors: string[] = [];
    page.on('request', request => requests.push(request.url()));
    page.on('requestfailed', request => failures.push(request.url()));
    page.on('response', response => { if (response.status() >= 400) failures.push(`${response.status()} ${response.url()}`); });
    page.on('pageerror', error => errors.push(error.message));
    page.on('console', message => { if (message.type() === 'error') errors.push(message.text()); });
    const cdp = await page.context().newCDPSession(page);
    await cdp.send('Emulation.setEmulatedMedia', {features: [{name: 'color-gamut', value: 'srgb'}]});
    await page.goto(`./${profile}`);
    const light = page.locator('main > .palettes > [data-rustify-colors-mode="light"]');
    const dark = page.locator('main > .palettes > [data-rustify-colors-mode="dark"]');
    await expect(light.locator(':scope > .solid')).toHaveCSS('background-color', 'rgb(62, 99, 221)');
    await expect(dark.locator(':scope > .solid')).toHaveCSS('background-color', 'rgb(62, 99, 221)');
    await expect(light).toHaveCSS('background-color', 'rgb(253, 253, 254)');
    await expect(dark).toHaveCSS('background-color', 'rgb(17, 19, 31)');
    await expect(light.locator('.nested')).toHaveCSS('background-color', 'rgb(17, 19, 31)');
    await expect(light.locator('.nested')).toHaveCSS('color', 'rgb(214, 225, 255)');
    await expect(page.locator('#host-sentinel')).toHaveCSS('--indigo-9', '');
    await expect(page.locator('html')).toHaveCSS('--indigo-9', '');
    await expect(page.locator('body')).toHaveCSS('background-color', 'rgb(255, 255, 255)');
    await expect(page.locator('script')).toHaveCount(0);
    expect(requests.some(url => /\.(?:js|wasm)(?:\?|$)/.test(url))).toBe(false);
    if (profile === 'index.html') expect(requests.some(url => url.includes('-p3.css'))).toBe(false);
    if (profile === 'p3.html') {
      expect(requests.some(url => url.endsWith('/indigo-light-p3.css'))).toBe(true);
      expect(requests.some(url => url.endsWith('/indigo-light.css'))).toBe(true);
      await cdp.send('Emulation.setEmulatedMedia', {features: [{name: 'color-gamut', value: 'p3'}]});
      await expect(light).toHaveCSS('--indigo-9', 'color(display-p3 0.276 0.384 0.837)');
    }
    expect(failures).toEqual([]);
    expect(errors).toEqual([]);
    for (const width of [1280, 390]) {
      await page.setViewportSize({width, height: 900});
      await expect(page.locator('body')).toHaveJSProperty('scrollWidth', width);
      await page.locator('h1').scrollIntoViewIfNeeded();
      await page.screenshot({path: info.outputPath(`${profile}-${width}.png`)});
    }
  });
}
