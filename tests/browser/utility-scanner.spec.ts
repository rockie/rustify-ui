import { test, expect } from '@playwright/test';
import { PNG } from 'pngjs';

async function ready(page: any) {
    await page.goto('./');
    await expect(page.locator('#status')).toHaveAttribute('data-status', 'ready');
    const manifest = await page.request.get('utility/build-manifest.json');
    expect((await manifest.json()).scanner_enabled).toBe(true);
}

test('raw source scanning installs CSS and variables, then direct ensure takes the no-op path', async ({page}, info) => {
    const errors: string[] = [], failed: string[] = [];
    page.on('pageerror', error => errors.push(error.message));
    page.on('console', message => { if (message.type() === 'error') errors.push(message.text()); });
    page.on('requestfailed', request => failed.push(request.url()));
    await ready(page);
    const result = await page.evaluate(async () => {
        const api = (window as any).__utility_lab;
        const violations: string[] = [];
        document.addEventListener('securitypolicyviolation', event => violations.push(event.violatedDirective));
        api.dispose('scope-a');
        api.mount('scope-a', 10000, '@theme { --color-probe:#123456; } @tailwind utilities;');
        const first = await api.scanContent('scope-a', [{content:'<div class="w-[147px] bg-[#abcdef]">var(--color-probe)</div>', extension:'html'}]);
        const sample = document.getElementById('scope-a-sample')!;
        sample.className = 'sample w-[147px] bg-[#abcdef]';
        const style = getComputedStyle(sample);
        const width = style.width, color = style.backgroundColor;
        const variable = getComputedStyle(document.getElementById('scope-a')!).getPropertyValue('--color-probe').trim();
        const repeated = await api.ensure('scope-a', ['w-[147px]', 'bg-[#abcdef]', '--color-probe']);
        const scannedAgain = await api.scanContent('scope-a', [{content:'w-[147px] bg-[#abcdef] --color-probe', extension:'html'}]);
        return {first, repeated, scannedAgain, width, color, variable, violations,
            other: getComputedStyle(document.getElementById('scope-b-sample')!).width};
    });
    expect(result.width).toBe('147px'); expect(result.color).toBe('rgb(171, 205, 239)');
    expect(result.variable).toBe('#123456'); expect(result.other).toBe('170px');
    expect(result.first.admitted).toContain('w-[147px]');
    expect(result.repeated.revision).toBe(result.first.revision);
    expect(result.scannedAgain.revision).toBe(result.first.revision);
    expect(result.scannedAgain.metrics.parsed_candidates).toBe(0);
    expect(result.violations).toEqual([]); expect(errors).toEqual([]); expect(failed).toEqual([]);
    await info.attach('Source scanner results', {body: JSON.stringify(result, null, 2), contentType:'application/json'});
});

test('virtual sources follow CSS paths and exclusions, retaining compiled rules after file removal', async ({page}, info) => {
    await ready(page);
    const result = await page.evaluate(async () => {
        const api = (window as any).__utility_lab;
        api.dispose('scope-a');
        api.mount('scope-a', 10000, '@tailwind utilities source(none); @source "./src/**/*.html"; @source not "./src/excluded.html";');
        const source = {root:'/', base:'/', entries:[
            {kind:'file', path:'/src/probe.html', content:'w-[153px] bg-[#123456]', revision:1},
            {kind:'file', path:'/src/excluded.html', content:'w-[199px]', revision:1},
            {kind:'file', path:'/other/probe.html', content:'w-[201px]', revision:1},
        ]};
        const first = await api.scanSources('scope-a', source);
        const sample = document.getElementById('scope-a-sample')!;
        sample.className = 'sample w-[153px] bg-[#123456]';
        const width = getComputedStyle(sample).width;
        let missing;
        try {await api.scanSources('scope-a', {...source,entries:[...source.entries,{kind:'unavailable',path:'/src/missing.html'}]});}
        catch (error) {missing = JSON.parse(String(error));}
        const retained = api.report('scope-a');
        const removed = await api.scanSources('scope-a', {...source, entries:[]});
        return {first, missing, retained, removed, width, after: getComputedStyle(sample).width};
    });
    expect(result.first.admitted).toContain('w-[153px]');
    expect(result.first.admitted).not.toContain('w-[199px]');
    expect(result.first.admitted).not.toContain('w-[201px]');
    expect(result.width).toBe('153px'); expect(result.after).toBe('153px');
    expect(result.missing.kind).toBe('scan');
    expect(result.missing.message).toContain('MissingSourceInformation');
    expect(result.retained.revision).toBe(result.first.revision);
    expect(result.removed.revision).toBe(result.first.revision);
    await info.attach('Virtual source results', {body:JSON.stringify(result,null,2),contentType:'application/json'});
});

test('scanned candidate programs draw the same dimensions and color in the GPU preview', async ({page}, info) => {
    await ready(page);
    await page.getByTestId('utility-gpu').scrollIntoViewIfNeeded();
    await expect(page.locator('#gpu-status')).toHaveText('GPU ready');
    const result = await page.evaluate(async () => {
        const api = (window as any).__utility_lab;
        const classes = ['w-[147px]', 'h-[72px]', 'bg-[#234567]', 'p-[7px]'];
        const scanned = await api.scanContent('scope-a', [{content:`<div class="${classes.join(' ')}"></div>`,extension:'html'}]);
        const applied = await api.gpuApply('scope-a',[[],classes,[],[]]);
        document.getElementById('scope-a-sample')!.className = `sample ${classes.join(' ')}`;
        return {scanned,applied};
    });
    expect(result.applied.revision).toBe(result.scanned.revision);
    await expect.poll(async () => page.evaluate(() => (window as any).__utility_lab.gpuReport('scope-a').button.drawn_revision)).toBe(result.applied.revision);
    const gpu = await page.evaluate(() => (window as any).__utility_lab.gpuReport('scope-a'));
    expect(gpu.button.status).toBe('applied');
    expect(gpu.button.geometry.slice(2)).toEqual([147,72]);
    expect(gpu.button.computed.padding).toEqual([7,7,7,7]);
    await expect(page.locator('#scope-a-sample')).toHaveCSS('width','147px');
    await expect(page.locator('#scope-a-sample')).toHaveCSS('background-color','rgb(35, 69, 103)');
    const png = PNG.sync.read(await page.getByTestId('utility-gpu').screenshot());
    const [x,y] = gpu.button.geometry;
    const offset = (Math.floor(y+8)*png.width + Math.floor(x+8))*4;
    expect(Array.from(png.data.subarray(offset,offset+4))).toEqual([35,69,103,255]);
    await info.attach('Scanned GPU program',{body:JSON.stringify(gpu,null,2),contentType:'application/json'});
});

test('scan install failures and input quotas preserve the snapshot; lost ack replays scanned variables', async ({page}) => {
    await ready(page);
    const result = await page.evaluate(async () => {
        const api = (window as any).__utility_lab;
        api.dispose('scope-a');
        api.mount('scope-a', 1000, '@theme { --color-probe:#123456; } @tailwind utilities;');
        const first = await api.scanContent('scope-a', [{content:'w-[147px] --color-probe',extension:'html'}]);
        const replace = CSSStyleSheet.prototype.replaceSync;
        CSSStyleSheet.prototype.replaceSync = function() {throw new DOMException('scan install failure','SyntaxError');};
        let install, quota, ack;
        try {await api.scanContent('scope-a', [{content:'w-[179px]',extension:'html'}]);}
        catch (error) {install = JSON.parse(String(error));}
        finally {CSSStyleSheet.prototype.replaceSync = replace;}
        const retained = api.report('scope-a');
        try {await api.scanContent('scope-a', [{content:'a'.repeat(300000),extension:'html'}]);}
        catch (error) {quota = JSON.parse(String(error));}
        const afterQuota = api.report('scope-a');
        const post = Worker.prototype.postMessage;
        Worker.prototype.postMessage = function(message:any,...args:any[]) {
            if (message.operation === 'ack') return;
            return post.call(this,message,...args as [any]);
        };
        try {await api.scanContent('scope-a', [{content:'w-[181px]',extension:'html'}]);}
        catch (error) {ack = JSON.parse(String(error));}
        finally {Worker.prototype.postMessage = post;}
        const installed = api.report('scope-a');
        const replay = await api.retry('scope-a');
        const sample = document.getElementById('scope-a-sample')!;
        sample.className = 'sample w-[181px]';
        return {first,install,quota,ack,retained,afterQuota,installed,replay,
            width:getComputedStyle(sample).width,
            variable:getComputedStyle(document.getElementById('scope-a')!).getPropertyValue('--color-probe').trim()};
    });
    expect(result.install.kind).toBe('install');
    expect(result.retained.revision).toBe(result.first.revision);
    expect(result.retained.admitted).not.toContain('w-[179px]');
    expect(result.quota.kind).toBe('compile'); expect(result.quota.error.kind).toBe('limit');
    expect(result.afterQuota.revision).toBe(result.first.revision);
    expect(result.ack.kind).toBe('timeout');
    expect(result.installed.admitted).toContain('w-[181px]');
    expect(result.replay.admitted).toContain('w-[181px]');
    expect(result.replay.admitted).not.toContain('w-[179px]');
    expect(result.width).toBe('181px'); expect(result.variable).toBe('#123456');
});

test('source panel works by keyboard, reports empty input, and fits desktop and mobile', async ({page}, info) => {
    await ready(page);
    await page.locator('#source-panel summary').click();
    await page.locator('#source').fill('<div class="w-[161px] bg-[#234567]"></div>');
    await page.locator('#scan').focus(); await page.keyboard.press('Enter');
    await expect(page.locator('#source-status')).toHaveAttribute('data-status','ready');
    await expect(page.locator('#source-result')).toContainText('w-[161px]');
    await page.locator('#source').fill(''); await page.locator('#scan').click();
    await expect(page.locator('#source-status')).toHaveText('Enter source text to scan.');
    await info.attach('Source panel desktop',{body:await page.screenshot({fullPage:true}),contentType:'image/png'});
    await page.setViewportSize({width:390,height:844});
    expect(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth)).toBe(true);
    await info.attach('Source panel mobile',{body:await page.screenshot({fullPage:true}),contentType:'image/png'});
});
