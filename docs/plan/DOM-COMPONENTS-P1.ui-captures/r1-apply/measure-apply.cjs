// Runtime measurements accompanying the repair screenshots; no scoring.
const { chromium } = require('@playwright/test');
const fs = require('node:fs');
const path = require('node:path');
const outDir = process.env.UI_APPLY_OUT || __dirname;
const style = node => {
    if (!node) return null;
    const c = getComputedStyle(node);
    return {
        rect: node.getBoundingClientRect().toJSON(), color: c.color,
        background: c.backgroundColor, weight: c.fontWeight, font: c.fontFamily,
        size: c.fontSize, lineHeight: c.lineHeight, fontStyle: c.fontStyle,
        borderLeft: c.borderLeftWidth, padding: c.padding,
        outline: c.outline, outlineOffset: c.outlineOffset, opacity: c.opacity,
    };
};
async function open(browser, url, width = 1280, height = 900) {
    const page = await browser.newPage({viewport: {width, height}, reducedMotion: 'reduce'});
    await page.goto(url);
    await page.locator('#status[data-status=ready]').waitFor({state:'attached', timeout:90000});
    await page.evaluate(() => document.fonts.ready);
    return page;
}
(async () => {
    const browser = await chromium.launch();
    const catalog = await open(browser, 'http://127.0.0.1:4176/');
    await catalog.evaluate(() => window.__component_catalog.mount_layout_fixtures());
    await catalog.getByTestId('type-code').waitFor();
    const result = {at: new Date().toISOString(), chromium: browser.version()};
    result.typography = {
        defaults: Object.fromEntries(await Promise.all(['type-heading','type-default-heading','type-strong','type-quote','type-code','type-kbd','type-blockquote'].map(async id => [id, await catalog.getByTestId(id).evaluate(style)]))),
        blockquoteSizes: await Promise.all((await catalog.locator('[data-testid=type-blockquote-matrix] blockquote').all()).map(node => node.evaluate(style))),
        explicitSoftCode: await catalog.locator('[data-testid=type-code-matrix] code[data-variant=soft][data-color=indigo]').first().evaluate(style),
    };
    await catalog.close();
    const studio = await open(browser, 'http://127.0.0.1:4181/',1440,900);
    result.builds = {studio: await studio.evaluate(async () => (await (await fetch('./build-manifest.json')).json()).build_id)};
    await studio.getByTestId('renderer-compare').click();
    await studio.getByTestId('studio-config-accent_color').click();
    result.brandSelect = {trigger: await studio.getByTestId('studio-config-accent_color').boundingBox(), list: await studio.getByTestId('studio-config-accent_color-list').boundingBox()};
    await studio.keyboard.press('Escape');
    result.preview = {
        authored: await studio.locator('[data-testid=studio-preview] .studio-authored-preview').evaluate(style),
        gpu: await studio.getByTestId('gpu-board').evaluate(style),
        hostPosition: await studio.locator('.host-probe').evaluate(node => getComputedStyle(node).position),
    };
    await studio.getByTestId('mode-dark').click();
    result.darkPreview = await studio.locator('[data-testid=studio-preview] .studio-authored-preview').evaluate(style);
    await studio.close();
    fs.writeFileSync(path.join(outDir,'measurements.json'),JSON.stringify(result,null,2));
    await browser.close();
    console.log(JSON.stringify({written:path.join(outDir,'measurements.json'), blockquotes:result.typography.blockquoteSizes.map(s=>({size:s.size,border:s.borderLeft})), codeDefault:result.typography.defaults['type-code'].color, codeExplicit:result.typography.explicitSoftCode.color}));
})();
