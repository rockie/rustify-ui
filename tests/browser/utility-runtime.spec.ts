import { test, expect } from '@playwright/test';
import { createRequire } from 'node:module';
import { resolve } from 'node:path';

async function ready(page: any) {
    await page.goto('./');
    await expect(page.locator('#status')).toHaveAttribute('data-status', 'ready');
}

test('manifest seed preserves static utility order on first ensure, replacement and retry', async ({page}, info) => {
    const errors: string[] = [], failed: string[] = [], csp: string[] = [];
    page.on('pageerror', error => errors.push(error.message));
    page.on('console', message => { if (message.type() === 'error') errors.push(message.text()); });
    page.on('requestfailed', request => failed.push(`${request.url()}: ${request.failure()?.errorText}`));
    await page.addInitScript(() => {
        (window as any).__seedCsp = [];
        document.addEventListener('securitypolicyviolation', event => (window as any).__seedCsp.push(event.violatedDirective));
    });
    const requireOracle = createRequire(resolve('tests/utility/compiler/oracle/package.json'));
    expect(requireOracle('tailwindcss/package.json').version).toBe('4.1.13');
    const oracle = requireOracle('tailwindcss');
    const official: string[] = [];
    for (const spacing of [4,8]) {
        const compiler = await oracle.compile(`@theme inline { --spacing: ${spacing}px; } @tailwind utilities;`);
        official.push(compiler.build(['p-2','p-4']));
    }
    await ready(page);
    const result = await page.evaluate(async official => {
        const api = (window as any).__utility_lab;
        const manifest = await fetch('./utility/build-manifest.json').then(response => response.json());
        const response = await fetch(`./utility/${manifest.static_candidates}`);
        const bytes = await response.arrayBuffer();
        const digest = [...new Uint8Array(await crypto.subtle.digest('SHA-256', bytes))].map(byte => byte.toString(16).padStart(2,'0')).join('');
        const seed = JSON.parse(new TextDecoder().decode(bytes));
        api.dispose('scope-b');
        api.mount('scope-b', 10000, '@theme inline { --spacing: 4px; } @tailwind utilities;');
        const sample = document.getElementById('scope-b-sample')!;
        const referenceRoot = document.createElement('div'); referenceRoot.id = 'seed-reference';
        const reference = document.createElement('div'); reference.className = 'p-2 p-4'; referenceRoot.append(reference);
        document.querySelector('main')!.append(referenceRoot);
        const sheet = new CSSStyleSheet();
        const installReference = (css: string) => { sheet.replaceSync(`@scope (#seed-reference) { ${css} }`); };
        installReference(official[0]);
        document.adoptedStyleSheets = [...document.adoptedStyleSheets, sheet];
        const first = await api.ensure('scope-b', ['p-2']);
        // Read synchronously before Observer can supply p-4: its rule must come from the seed.
        sample.className = 'p-2 p-4';
        const padding = getComputedStyle(sample).padding;
        const officialPadding = getComputedStyle(reference).padding;
        const next = await api.replace('scope-b', '@theme inline { --spacing: 8px; } @tailwind utilities;');
        const replacedPadding = getComputedStyle(sample).padding;
        installReference(official[1]);
        const officialReplacedPadding = getComputedStyle(reference).padding;
        await api.flush('scope-b');
        const replay = await api.retry('scope-b');
        const retryPadding = getComputedStyle(sample).padding;
        document.adoptedStyleSheets = document.adoptedStyleSheets.filter(value => value !== sheet); referenceRoot.remove();
        return {seed, digest, expectedDigest:manifest.files_sha256[manifest.static_candidates], first, padding,
            next, replacedPadding, replay, retryPadding, officialPadding, officialReplacedPadding};
    }, official);
    expect(result.seed).toContain('p-4');
    expect(result.seed).toContain('focus-visible:ring-[3px]');
    expect(result.seed).not.toContain('group');
    expect(result.digest).toBe(result.expectedDigest);
    expect(result.first.admitted).toContain('p-4');
    expect(result.padding).toBe('16px');
    expect(result.padding).toBe(result.officialPadding);
    expect(result.replacedPadding).toBe('32px');
    expect(result.retryPadding).toBe('32px');
    expect(result.replacedPadding).toBe(result.officialReplacedPadding);
    expect(result.next.css.indexOf('.p-2')).toBeLessThan(result.next.css.indexOf('.p-4'));
    expect(result.replay.admitted).toContain('p-4');
    expect(result.replay.revision).toBeGreaterThan(result.next.revision);
    csp.push(...await page.evaluate(() => (window as any).__seedCsp));
    expect(errors).toEqual([]); expect(failed).toEqual([]); expect(csp).toEqual([]);
    await info.attach('Static candidate seed and installed cascade', {body: JSON.stringify(result,null,2),contentType:'application/json'});
});

test('a modified seed is rejected before installation and explicit retry keeps committed styles', async ({page}) => {
    await ready(page);
    await page.evaluate(async () => { await (window as any).__utility_lab.ensure('scope-b',['w-[137px]']); });
    await page.route('**/utility/static-candidates.json?sha256=*', route => route.fulfill({contentType:'application/json',body:'[]'}));
    const rejected = await page.evaluate(async () => {
        const api = (window as any).__utility_lab;
        const before = api.report('scope-b');
        let failure;
        try { await api.retry('scope-b'); } catch (error) { failure = JSON.parse(String(error)); }
        const sample = document.getElementById('scope-b-sample')!; sample.className='w-[137px]';
        return {failure,before:before.revision,retained:api.report('scope-b').revision,width:getComputedStyle(sample).width};
    });
    expect(rejected.failure.kind).toBe('protocol');
    expect(rejected.failure.message).toContain('asset digest mismatch: static-candidates.json');
    expect(rejected.retained).toBe(rejected.before); expect(rejected.width).toBe('137px');
    await page.unroute('**/utility/static-candidates.json?sha256=*');
    const replayed = await page.evaluate(async () => (window as any).__utility_lab.retry('scope-b'));
    expect(replayed.admitted).toContain('w-[137px]');
});

test('seed count and transfer budgets reject init without retaining a half initialized session', async ({page}, info) => {
    await ready(page);
    const result = await page.evaluate(async () => {
        const manifest = await fetch('./utility/build-manifest.json').then(response => response.json());
        const worker = new Worker(new URL('./utility/utility-worker.js', document.baseURI), {type:'module'});
        let request = 0;
        const limits = {input_bytes:65536,asset_bytes:65536,candidate_count:1024,ast_depth:32,css_bytes:262144,queued_bytes:65536,cache_bytes:1048576};
        const init = (override: object) => new Promise<any>((resolve, reject) => {
            worker.onmessage = ({data}) => resolve(data);
            worker.onerror = event => reject(new Error(event.message));
            worker.postMessage({protocol_version:1,runtime_id:'seed-budget',epoch:1,request_id:++request,operation:'init',payload:{build_id:manifest.build_id,
                config:JSON.stringify({input:{css:'@tailwind utilities;',entry:'memory:input.css'},assets:[],limits:{...limits,...override}})}});
        });
        try {
            const count = await init({candidate_count:1});
            const transfer = await init({queued_bytes:1024});
            const recovered = await init({});
            return {count,transfer,recovered};
        } finally {worker.terminate();}
    });
    expect(result.count.operation).toBe('error');
    expect(result.count.payload).toEqual({kind:'compile',error:{kind:'limit',resource:'candidate_count',limit:1}});
    expect(result.transfer.operation).toBe('error');
    expect(result.transfer.payload).toEqual({kind:'compile',error:{kind:'limit',resource:'queued_bytes',limit:1024}});
    expect(result.recovered.operation).toBe('ready');
    expect(result.recovered.payload.static_candidates).toContain('p-4');
    await info.attach('Seed init budget boundaries and recovery', {body:JSON.stringify(result,null,2),contentType:'application/json'});
});

test('real DOM components use scope cn on raw fragments and react to caller and size changes', async ({page}, info) => {
    const errors: string[] = [], failed: string[] = [], csp: string[] = [];
    page.on('pageerror', error => errors.push(error.message));
    page.on('console', message => { if (message.type() === 'error') errors.push(message.text()); });
    page.on('requestfailed', request => failed.push(`${request.url()}: ${request.failure()?.errorText}`));
    page.on('response', response => { if (response.status() >= 400) failed.push(`${response.status()} ${response.url()}`); });
    await ready(page);
    await expect.poll(() => page.evaluate(() => (window as any).__utility_lab.gpuReport('scope-a')?.button.drawn_revision)).toBe(0);
    const initial = await page.evaluate(async () => {
        document.addEventListener('securitypolicyviolation', event => (window as any).__cn_csp.push(event.violatedDirective));
        (window as any).__cn_csp = [];
        const api = (window as any).__utility_lab.componentBaseline;
        for (const [id, custom] of [['cn-default', false], ['cn-custom', true]] as const) {
            const root = document.createElement('div'); root.id = id; document.querySelector('main')!.append(root);
            api.mount(id, custom);
            await api.ensure(id, ['p-2', 'p-4', 'p-8', 'bg-[#123456]']);
        }
        return ['cn-default', 'cn-custom'].map(id => {
            const card = document.querySelector(`[data-testid="${id}-card"]`)!;
            return {classes: card.className, padding: getComputedStyle(card).padding, background: getComputedStyle(card).backgroundColor};
        });
    });
    expect(initial).toEqual([
        {classes: 'bg-[#123456] p-4', padding: '16px', background: 'rgb(18, 52, 86)'},
        {classes: 'p-2 bg-[#123456] p-4', padding: '32px', background: 'rgb(18, 52, 86)'},
    ]);
    const button = page.getByTestId('cn-custom-button');
    await expect(button).toHaveClass(/h-9/);
    await expect(button).toHaveClass(/p-4/);
    await page.evaluate(() => (window as any).__utility_lab.componentBaseline.update('cn-custom', 'p-8', true));
    await expect(button).toHaveClass(/h-8/);
    await expect(button).toHaveClass(/p-8/);
    await expect(button).not.toHaveClass(/h-9|p-4/);
    await page.evaluate(() => (window as any).__utility_lab.componentBaseline.update('cn-custom', '', false));
    await expect(button).toHaveClass(/h-9/);
    await expect(button).toHaveClass(/px-4/);
    await expect(button).toHaveClass(/py-2/);
    await expect(button).not.toHaveClass(/p-8/);
    await info.attach('Scope cn classes and computed styles', {body: JSON.stringify(initial, null, 2), contentType: 'application/json'});
    await page.getByTestId('cn-custom-card').screenshot({path: info.outputPath('cn-component.png')});
    csp.push(...await page.evaluate(() => (window as any).__cn_csp));
    await page.evaluate(() => {
        for (const id of ['cn-default', 'cn-custom']) {
            (window as any).__utility_lab.componentBaseline.dispose(id);
            const root = document.getElementById(id)!;
            if (root.hasAttribute('data-rustify-style-owner')) throw new Error('component scope ownership leaked');
            root.remove();
        }
    });
    expect(errors).toEqual([]); expect(failed).toEqual([]); expect(csp).toEqual([]);
});

test('parser recovery preserves authored important values and valid compound variants', async ({page}, info) => {
    const errors: string[] = [], failed: string[] = [];
    page.on('pageerror', error => errors.push(error.message));
    page.on('console', message => { if (message.type() === 'error') errors.push(message.text()); });
    page.on('requestfailed', request => failed.push(`${request.url()}: ${request.failure()?.errorText}`));
    await ready(page);
    await page.getByTestId('utility-gpu').scrollIntoViewIfNeeded();
    await expect.poll(() => page.evaluate(() => (window as any).__utility_lab.gpuReport('scope-a')?.button.drawn_revision)).toBe(0);
    const result = await page.evaluate(async () => {
        const api = (window as any).__utility_lab;
        api.dispose('scope-a');
        api.mount('scope-a', 10000, '@tailwind utilities; .sample { width:137px!important ignored; background-color:#123456; }');
        const classes = ['[_]:block', 'data-foo[bar]:block', 'group-data-[state=open]:w-[159px]!'];
        const report = await api.ensure('scope-a', classes);
        const frame = document.getElementById('scope-a-frame')!;
        frame.classList.add('group');
        const sample = document.getElementById('scope-a-sample')!;
        sample.className = `sample ${classes.join(' ')}`;
        frame.setAttribute('data-state', 'closed');
        const closed = getComputedStyle(sample).width;
        frame.setAttribute('data-state', 'open');
        const open = getComputedStyle(sample).width;
        frame.setAttribute('data-state', 'closed');
        return {report, closed, open, restored: getComputedStyle(sample).width,
            background: getComputedStyle(sample).backgroundColor,
            otherWidth: getComputedStyle(document.getElementById('scope-b-sample')!).width};
    });
    expect(result.report.unsupported).toEqual([]);
    expect(result.report.rejected.filter((item: any) => ['[_]:block', 'data-foo[bar]:block'].includes(item.candidate))
        .map((item: any) => item.candidate).sort()).toEqual(['[_]:block', 'data-foo[bar]:block']);
    expect(result.report.admitted.filter((candidate: string) => ['[_]:block', 'data-foo[bar]:block', 'group-data-[state=open]:w-[159px]!'].includes(candidate))).toEqual(['group-data-[state=open]:w-[159px]!']);
    expect(result.report.css).not.toContain('ignored');
    expect(result.closed).toBe('137px');
    expect(result.open).toBe('159px');
    expect(result.restored).toBe('137px');
    expect(result.background).toBe('rgb(18, 52, 86)');
    expect(result.otherWidth).toBe('170px');
    expect(errors).toEqual([]); expect(failed).toEqual([]);
    await info.attach('Parser browser values', {body: JSON.stringify(result, null, 2), contentType: 'application/json'});
});

test('structural spacing, container and line clamp keep their actual browser semantics', async ({page}, info) => {
    await ready(page);
    const result = await page.evaluate(async () => {
        const api = (window as any).__utility_lab;
        const classes = ['border-separate', 'border-spacing-[12px]', 'flex', 'space-x-[9px]',
            'space-x-reverse', 'line-clamp-2', '@container/probe'];
        const report = await api.ensure('scope-a', classes);
        const root = document.getElementById('scope-a')!;
        const table = document.createElement('table');
        table.className = 'border-separate border-spacing-[12px]';
        table.innerHTML = '<tbody><tr><td>one</td><td>two</td></tr></tbody>';
        const siblings = document.createElement('div');
        siblings.className = 'flex space-x-[9px] space-x-reverse';
        siblings.innerHTML = '<span>one</span><span>two</span><span>three</span>';
        const container = document.createElement('div');
        container.className = '@container/probe';
        const clamp = document.createElement('p');
        clamp.className = 'line-clamp-2';
        clamp.textContent = 'Browser utility checks';
        container.append(clamp);
        root.append(table, siblings, container);
        const margins = Array.from(siblings.children).map(child => {
            const style = getComputedStyle(child);
            return [style.marginInlineStart, style.marginInlineEnd];
        });
        const containerStyle = getComputedStyle(container), clampStyle = getComputedStyle(clamp);
        return {revision: report.revision, pending: report.unsupported, spacing: getComputedStyle(table).borderSpacing,
            margins, containerType: containerStyle.containerType, containerName: containerStyle.containerName,
            lineClamp: clampStyle.webkitLineClamp, overflow: clampStyle.overflow, css: report.css};
    });
    expect(result.pending).toEqual([]);
    expect(result.spacing).toBe('12px');
    expect(result.margins).toEqual([['9px', '0px'], ['9px', '0px'], ['0px', '0px']]);
    expect(result.containerType).toBe('inline-size');
    expect(result.containerName).toBe('probe');
    expect(result.lineClamp).toBe('2');
    expect(result.overflow).toBe('hidden');
    expect(result.css).not.toContain('--tw-sort');
    await info.attach('Structural browser values', {body: JSON.stringify(result, null, 2), contentType: 'application/json'});
});

test('SDK ensure installs scanner-free CSS, coalesces requests and isolates independent scopes', async ({ page }, testInfo) => {
    const errors: string[] = [];
    const failed: string[] = [];
    const paths: string[] = [];
    page.on('pageerror', error => errors.push(error.message));
    page.on('requestfailed', request => failed.push(request.url()));
    page.on('request', request => paths.push(request.url()));
    await ready(page);
    const result = await page.evaluate(async () => {
        const api = (window as any).__utility_lab;
        const violations: string[] = [];
        document.addEventListener('securitypolicyviolation', event => violations.push(event.violatedDirective));
        const baseline = await api.flush('scope-a');
        const first = await Promise.all([
            api.ensure('scope-a', ['w-[137px]']),
            api.ensure('scope-a', ['bg-[#123456]', 'w-[137px]']),
        ]);
        const sample = document.getElementById('scope-a-sample')!;
        sample.className = 'sample w-[137px] bg-[#123456]';
        const style = getComputedStyle(sample);
        const width = style.width;
        const color = style.backgroundColor;
        const initial = api.report('scope-a');
        const repeated = await api.ensure('scope-a', ['w-[137px]']);
        const other = await api.ensure('scope-b', ['w-[93px]', 'bg-[#abcdef]']);
        const second = document.getElementById('scope-b-sample')!;
        second.className = 'sample w-[93px] bg-[#abcdef]';
        const outside = document.getElementById('outside')!;
        const outsideColor = getComputedStyle(outside).backgroundColor;
        const otherColor = getComputedStyle(second).backgroundColor;
        const sheetCount = document.adoptedStyleSheets.length;
        const css = api.report('scope-a').css;
        api.dispose('scope-a');
        return { baselineRevision:baseline.revision, first: first.map((report: any) => report.revision), revision: initial.revision,
            repeatedRevision: repeated.revision, width, color, otherColor, outsideColor,
            otherRevision: other.revision, sheetCount, remainingSheets: document.adoptedStyleSheets.length,
            remainingColor: getComputedStyle(second).backgroundColor, violations, css };
    });
    expect(result.first).toEqual([result.baselineRevision + 1, result.baselineRevision + 1]);
    expect(result.width).toBe('137px');
    expect(result.color).toBe('rgb(18, 52, 86)');
    expect(result.otherColor).toBe('rgb(171, 205, 239)');
    expect(result.outsideColor).toBe('rgba(0, 0, 0, 0)');
    expect(result.repeatedRevision).toBe(result.revision);
    expect(result.remainingSheets).toBe(result.sheetCount - 1);
    expect(result.remainingColor).toBe(result.otherColor);
    expect(result.violations).toEqual([]);
    expect(errors).toEqual([]);
    expect(failed).toEqual([]);
    expect(paths.some(path => path.includes('/extractor/'))).toBe(false);
    await testInfo.attach('SDK committed stylesheet', { body: JSON.stringify(result, null, 2), contentType: 'application/json' });
});

test('controls apply classes and fit desktop and mobile', async ({ page }, testInfo) => {
    await ready(page);
    const baseline = await page.evaluate(async () => (window as any).__utility_lab.flush('scope-a'));
    await page.locator('#classes').fill('w-[137px] bg-[#123456] h-[72px] text-[#ffffff] rounded-[8px]');
    await page.locator('#apply').click();
    await expect(page.locator('#status')).toContainText(`DOM styles installed. Revision ${baseline.revision + 1}.`);
    await expect(page.locator('#scope-a-sample')).toHaveCSS('width', '137px');
    await expect(page.locator('#scope-a-sample')).toHaveCSS('background-color', 'rgb(18, 52, 86)');
    await expect(page.locator('#scope-a-sample')).toHaveCSS('color', 'rgb(255, 255, 255)');
    await page.screenshot({ path: testInfo.outputPath('desktop.png'), fullPage: true });
    await page.setViewportSize({ width: 390, height: 844 });
    await page.evaluate(async () => {
        await document.fonts.ready;
        await new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve)));
    });
    expect(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth)).toBe(true);
    await page.screenshot({ path: testInfo.outputPath('mobile.png'), fullPage: true });
});

test('base, override and conditional classes merge through the scope cn before both previews apply them', async ({ page }, testInfo) => {
    const errors: string[] = [];
    page.on('pageerror', error => errors.push(error.message));
    await ready(page);
    const merged = page.locator('#merged');
    await expect(merged).toHaveText('w-[137px] bg-[#123456] h-[72px] text-[#ffffff] rounded-[8px]');
    await page.locator('#override-classes').fill('w-[150px] bg-[#abcdef]');
    await expect(merged).toHaveText('h-[72px] text-[#ffffff] rounded-[8px] w-[150px] bg-[#abcdef]');
    await page.locator('#conditional-classes').fill('p-[7px] w-[163px]');
    await expect(merged).toHaveText('h-[72px] text-[#ffffff] rounded-[8px] w-[150px] bg-[#abcdef]');
    await page.locator('#conditional-enabled').check();
    await expect(merged).toHaveText('h-[72px] text-[#ffffff] rounded-[8px] bg-[#abcdef] p-[7px] w-[163px]');
    expect(await page.evaluate(() => (window as any).__utility_lab.merge('scope-a', ['p-2 p-4']))).toBe('p-4');
    await page.locator('#apply').click();
    await expect(page.locator('#status')).toContainText('DOM styles installed.');
    const sample = page.locator('#scope-a-sample');
    await expect(sample).toHaveClass('sample h-[72px] text-[#ffffff] rounded-[8px] bg-[#abcdef] p-[7px] w-[163px]');
    await expect(sample).toHaveCSS('width', '163px');
    await expect(sample).toHaveCSS('background-color', 'rgb(171, 205, 239)');
    await expect.poll(async () => (await page.evaluate(() => (window as any).__utility_lab.gpuReport('scope-a')))?.button.geometry?.[2]).toBe(163);
    const gpu = await page.evaluate(() => (window as any).__utility_lab.gpuReport('scope-a'));
    expect(gpu.button.computed.padding).toEqual([7, 7, 7, 7]);
    await page.locator('#merged').scrollIntoViewIfNeeded();
    await page.screenshot({ path: testInfo.outputPath('merge-desktop.png') });
    await page.setViewportSize({ width: 390, height: 844 });
    await page.evaluate(async () => {
        window.scrollTo(0, 0);
        await document.fonts.ready;
        await new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve)));
    });
    expect(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth)).toBe(true);
    await page.screenshot({ path: testInfo.outputPath('merge-mobile.png'), fullPage: true });
    expect(errors).toEqual([]);
});

test('switching language re-renders labels and live statuses without touching inputs or styles', async ({ page }, testInfo) => {
    const errors: string[] = [];
    page.on('pageerror', error => errors.push(error.message));
    await ready(page);
    await expect(page.locator('html')).toHaveAttribute('lang', 'en');
    await expect(page.locator('#gpu-status')).toHaveText('GPU ready');
    await page.locator('#override-classes').fill('w-[150px]');
    await page.locator('#apply').click();
    await expect(page.locator('#status')).toContainText('DOM styles installed.');
    const revision = (await page.evaluate(() => (window as any).__utility_lab.report('scope-a'))).revision;
    await page.locator('#language').selectOption('zh');
    await expect(page.locator('html')).toHaveAttribute('lang', 'zh-CN');
    await expect(page.locator('label[for="classes"]')).toHaveText('类名');
    await expect(page.locator('#apply')).toHaveText('应用类名');
    await expect(page.locator('#status')).toHaveText(`DOM 样式已安装。修订 ${revision}。GPU 结果见下方。`);
    await expect(page.locator('#gpu-status')).toHaveText('GPU 就绪');
    await expect(page.locator('#gpu-revision')).toContainText('按钮：已应用');
    await expect(page.locator('#source-result')).toHaveAttribute('aria-label', '已编译的源码候选');
    await expect(page.locator('#override-classes')).toHaveValue('w-[150px]');
    await expect(page.locator('#scope-a-sample')).toHaveCSS('width', '150px');
    expect((await page.evaluate(() => (window as any).__utility_lab.report('scope-a'))).revision).toBe(revision);
    await page.screenshot({ path: testInfo.outputPath('chinese-desktop.png') });
    await page.setViewportSize({ width: 390, height: 844 });
    await page.evaluate(async () => {
        window.scrollTo(0, 0);
        await document.fonts.ready;
        await new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve)));
    });
    expect(await page.evaluate(() => document.documentElement.scrollWidth <= innerWidth)).toBe(true);
    await page.screenshot({ path: testInfo.outputPath('chinese-mobile.png'), fullPage: true });
    await page.locator('#language').selectOption('en');
    await expect(page.locator('html')).toHaveAttribute('lang', 'en');
    await expect(page.locator('#status')).toHaveText(`DOM styles installed. Revision ${revision}. See GPU results below.`);
    await expect(page.locator('#gpu-status')).toHaveText('GPU ready');
    expect(errors).toEqual([]);
});

test.describe('a reader who prefers Chinese', () => {
    test.use({ locale: 'zh-CN' });
    test('starts in Chinese', async ({ page }) => {
        await page.goto('./');
        await expect(page.locator('#status')).toHaveAttribute('data-status', 'ready');
        await expect(page.locator('html')).toHaveAttribute('lang', 'zh-CN');
        await expect(page.locator('#language')).toHaveValue('zh');
        await expect(page.locator('#status')).toHaveText('就绪。应用类名以更新预览。');
        await expect(page.locator('#source-status')).toHaveText('此构建未启用源码扫描。使用 --utility-scanner 构建以启用。');
        await expect(page.locator('#gpu-status')).toHaveText('GPU 就绪');
    });
});

test('nested owners stop outer rules and disposal removes the owned stylesheet', async ({ page }) => {
    await ready(page);
    const result = await page.evaluate(async () => {
        const api = (window as any).__utility_lab;
        api.mount('scope-a-nested');
        const nested = document.getElementById('scope-a-nested-sample')!;
        const baseline = getComputedStyle(nested).backgroundColor;
        await api.ensure('scope-a', ['w-[137px]', 'bg-[#123456]']);
        nested.className = 'sample w-[137px] bg-[#123456]';
        const unstyled = getComputedStyle(nested).backgroundColor;
        await api.ensure('scope-a-nested', ['bg-[#abcdef]']);
        nested.className = 'sample bg-[#abcdef]';
        const styled = getComputedStyle(nested).backgroundColor;
        api.dispose('scope-a-nested');
        return { baseline, unstyled, styled, owner: document.getElementById('scope-a-nested')!.hasAttribute('data-rustify-style-owner') };
    });
    expect(result.unstyled).toBe(result.baseline);
    expect(result.unstyled).not.toBe('rgb(18, 52, 86)');
    expect(result.styled).toBe('rgb(171, 205, 239)');
    expect(result.owner).toBe(false);
});

test('classList changes and newly added subtrees compile through the bounded observer without scanner', async ({ page }) => {
    await ready(page);
    await page.evaluate(async () => {
        const api = (window as any).__utility_lab;
        await api.ensure('scope-a', []);
        document.getElementById('scope-a-sample')!.classList.add('w-[151px]');
    });
    await expect(page.locator('#scope-a-sample')).toHaveCSS('width', '151px');
    await page.evaluate(() => {
        const child = document.createElement('div');
        child.id = 'observer-child';
        child.className = 'bg-[#abcdef] h-[37px]';
        document.getElementById('scope-a-sample')!.append(child);
    });
    await expect(page.locator('#observer-child')).toHaveCSS('background-color', 'rgb(171, 205, 239)');
    await expect(page.locator('#observer-child')).toHaveCSS('height', '37px');
    const unchanged = await page.evaluate(async () => {
        const api = (window as any).__utility_lab;
        const before = api.report('scope-a').revision;
        const sample = document.getElementById('scope-a-sample')!;
        sample.classList.remove('w-[151px]'); sample.classList.add('w-[151px]');
        await new Promise(resolve => setTimeout(resolve, 50));
        return { before, after: api.report('scope-a').revision };
    });
    expect(unchanged.after).toBe(unchanged.before);
});

test('scoped relation variants and arbitrary selectors retain conditional matching', async ({ page }, testInfo) => {
    await ready(page);
    const result = await page.evaluate(async () => {
        const api = (window as any).__utility_lab;
        const classes = ['w-[89px]', 'h-[11px]', 'group-data-[state=open]:bg-[#123456]',
            'peer-data-[state=open]:h-[37px]', '[&>span]:w-[63px]', 'p-[7px]'];
        const report = await api.ensure('scope-a', classes);
        const root = document.getElementById('scope-a-sample')!;
        root.className = 'group p-[7px]';
        const flag = document.createElement('div');
        flag.className = 'peer';
        const child = document.createElement('div');
        child.className = 'w-[89px] h-[11px] group-data-[state=open]:bg-[#123456] peer-data-[state=open]:h-[37px] [&>span]:w-[63px]';
        const span = document.createElement('span');
        span.style.display = 'block';
        child.append(span);
        root.append(flag, child);
        const before = { color: getComputedStyle(child).backgroundColor, height: getComputedStyle(child).height };
        root.dataset.state = 'open';
        flag.dataset.state = 'open';
        const after = { color: getComputedStyle(child).backgroundColor, height: getComputedStyle(child).height };
        const other = document.getElementById('scope-b-sample')!;
        other.className = child.className;
        const outside = document.getElementById('outside')!;
        outside.className = child.className;
        const isolated = [other, outside].map(element => getComputedStyle(element).backgroundColor);
        root.dataset.state = 'closed';
        flag.dataset.state = 'closed';
        return { before, after, isolated, padding: getComputedStyle(root).padding,
            spanWidth: getComputedStyle(span).width, restoredHeight: getComputedStyle(child).height,
            unsupported: report.unsupported, css: report.css };
    });
    expect(result.before).toEqual({ color: 'rgba(0, 0, 0, 0)', height: '11px' });
    expect(result.after).toEqual({ color: 'rgb(18, 52, 86)', height: '37px' });
    expect(result.isolated).toEqual(['rgba(0, 0, 0, 0)', 'rgba(0, 0, 0, 0)']);
    expect(result.padding).toBe('7px');
    expect(result.spanWidth).toBe('63px');
    expect(result.restoredHeight).toBe('11px');
    expect(result.unsupported).toEqual([]);
    await testInfo.attach('scoped relation variant results', { body: JSON.stringify(result, null, 2), contentType: 'application/json' });
});

test('arbitrary responsive conditions follow the browser viewport at both sizes', async ({ page }, testInfo) => {
    await ready(page);
    await page.setViewportSize({ width: 1000, height: 844 });
    const report = await page.evaluate(async () => {
        const api = (window as any).__utility_lab;
        const classes = ['w-[89px]', 'h-[11px]', 'min-[600px]:w-[149px]', 'max-[600px]:h-[47px]'];
        const report = await api.ensure('scope-a', classes);
        document.getElementById('scope-a-sample')!.className = classes.join(' ');
        return report;
    });
    await expect(page.locator('#scope-a-sample')).toHaveCSS('width', '149px');
    await expect(page.locator('#scope-a-sample')).toHaveCSS('height', '11px');
    await page.setViewportSize({ width: 390, height: 844 });
    await expect(page.locator('#scope-a-sample')).toHaveCSS('width', '89px');
    await expect(page.locator('#scope-a-sample')).toHaveCSS('height', '47px');
    expect(report.unsupported).toEqual([]);
    await testInfo.attach('responsive prepared report', { body: JSON.stringify(report, null, 2), contentType: 'application/json' });
});

test('color alpha and scoped placeholder selectors retain their browser semantics', async ({ page }, testInfo) => {
    await ready(page);
    const result = await page.evaluate(async () => {
        const api = (window as any).__utility_lab;
        const classes = ['bg-[#123456]/50', 'text-[#abcdef]/[.35]', 'border-[#fedcba]/25', 'placeholder-[#123456]/50'];
        const report = await api.ensure('scope-a', classes);
        const sample = document.getElementById('scope-a-sample')!;
        sample.className = classes.slice(0, 3).join(' ');
        const input = document.createElement('input');
        input.placeholder = 'Color sample';
        input.className = classes[3];
        sample.append(input);
        const outside = document.createElement('input');
        outside.placeholder = 'Outside scope';
        outside.className = classes[3];
        document.getElementById('outside')!.append(outside);
        const canvas = document.createElement('canvas');
        canvas.width = canvas.height = 1;
        const context = canvas.getContext('2d')!;
        function pixel(color: string) {
            context.clearRect(0, 0, 1, 1);
            context.fillStyle = color;
            context.fillRect(0, 0, 1, 1);
            return Array.from(context.getImageData(0, 0, 1, 1).data);
        }
        const style = getComputedStyle(sample);
        return { background: pixel(style.backgroundColor), text: pixel(style.color), border: pixel(style.borderTopColor),
            placeholder: pixel(getComputedStyle(input, '::placeholder').color),
            outside: pixel(getComputedStyle(outside, '::placeholder').color),
            unsupported: report.unsupported, css: report.css };
    });
    for (const [actual, expected] of [[result.background, [18, 52, 86, 128]],
        [result.text, [171, 205, 239, 89]], [result.border, [254, 220, 186, 64]],
        [result.placeholder, [18, 52, 86, 128]]] as const) {
        actual.forEach((value, index) => expect(Math.abs(value - expected[index])).toBeLessThanOrEqual(2));
    }
    expect(result.outside).not.toEqual(result.placeholder);
    expect(result.unsupported).toEqual([]);
    await testInfo.attach('color alpha and placeholder results', { body: JSON.stringify(result, null, 2), contentType: 'application/json' });
});

test('external variable inputs install live theme dependencies without utility selectors', async ({page}, info) => {
    const errors: string[] = [], failed: string[] = [];
    page.on('pageerror', error => errors.push(error.message));
    page.on('console', message => { if (message.type() === 'error') errors.push(message.text()); });
    page.on('requestfailed', request => failed.push(request.url()));
    await ready(page);
    await page.waitForLoadState('networkidle');
    const result = await page.evaluate(async () => {
        const api = (window as any).__utility_lab;
        api.dispose('scope-a');
        api.mount('scope-a', 10000, '@theme { --color-brand: #123456; --color-alias: var(--color-brand); } @tailwind utilities;');
        const value = (scope: string) => getComputedStyle(document.getElementById(`${scope}-sample`)!).getPropertyValue('--color-alias').trim();
        await api.ensure('scope-a', []);
        const before = value('scope-a');
        const first = await api.ensure('scope-a', ['--color-alias']);
        const installed = value('scope-a');
        const repeated = await api.ensure('scope-a', ['--color-alias']);
        return {before,installed,other:value('scope-b'),first,repeated};
    });
    expect(result.before).toBe('');
    expect(result.installed).toBe('#123456');
    expect(result.other).toBe('');
    expect(result.first.admitted.filter((candidate: string) => candidate.startsWith('--'))).toEqual([]);
    expect(result.first.rejected.filter((item: any) => item.candidate.startsWith('--'))).toEqual([]);
    expect(result.first.unsupported).toEqual([]);
    expect(result.first.css).toContain('--color-alias: var(--color-brand)');
    expect(result.repeated.revision).toBe(result.first.revision);
    expect(errors).toEqual([]); expect(failed).toEqual([]);
    await info.attach('External variable browser values', {body:JSON.stringify(result,null,2),contentType:'application/json'});
});

test('compiler-only Worker diagnoses disabled scanning and keeps its session usable', async ({page}, info) => {
    await ready(page);
    await expect(page.locator('#scan')).toBeDisabled();
    const sdk = await page.evaluate(async () => {
        const api = (window as any).__utility_lab;
        const before = await api.ensure('scope-a', ['w-[137px]']);
        const failures = [];
        for (const scan of [
            () => api.scanContent('scope-a', [{content:'w-[159px]',extension:'html'}]),
            () => api.scanSources('scope-a', {root:'/',base:'/',entries:[]}),
        ]) {
            try {await scan();} catch (error) {failures.push(JSON.parse(String(error)));}
        }
        const retained = api.report('scope-a');
        const next = await api.ensure('scope-a',['w-[179px]']);
        return {before,failures,retained,next};
    });
    expect(sdk.failures).toEqual(Array(2).fill({kind:'compile',error:{kind:'unsupported_feature',feature:'source scanning'}}));
    expect(sdk.retained.revision).toBe(sdk.before.revision);
    expect(sdk.next.admitted).toContain('w-[179px]');
    const result = await page.evaluate(async () => {
        const manifestUrl = new URL('./utility/build-manifest.json', location.href);
        const manifest = await (await fetch(manifestUrl)).json();
        const worker = new Worker(new URL('./utility-worker.js', manifestUrl), {type:'module'});
        let request = 0;
        const send = (operation: string, payload: unknown) => new Promise<any>((resolve,reject) => {
            const requestId = ++request;
            const timer = setTimeout(() => reject(new Error('utility probe timeout')),10000);
            worker.onmessage = ({data}) => { clearTimeout(timer); resolve(data); };
            worker.onerror = event => { clearTimeout(timer); reject(new Error(event.message)); };
            worker.postMessage({protocol_version:1,runtime_id:'compiler-only-probe',epoch:1,request_id:requestId,operation,payload});
        });
        const limits = {input_bytes:65536,asset_bytes:65536,candidate_count:1024,ast_depth:32,css_bytes:262144,queued_bytes:65536,cache_bytes:1048576};
        try {
            const ready = await send('init',{build_id:manifest.build_id,config:JSON.stringify({input:{css:'@tailwind utilities;',entry:'/app/input.css'},assets:[],limits,owner:null})});
            const content = await send('scan_content',JSON.stringify([{content:'w-[137px]',extension:'html'}]));
            const sources = await send('scan_sources',JSON.stringify({root:'/',base:'/app',entries:[]}));
            const prepared = await send('add_candidates',JSON.stringify(['w-[137px]']));
            const committed = await send('ack',{token:prepared.payload.token});
            return {ready,content,sources,prepared,committed};
        } finally {worker.terminate();}
    });
    expect(result.ready.operation).toBe('ready');
    expect(result.ready.payload.scanner_enabled).toBe(false);
    for (const response of [result.content,result.sources]) {
        expect(response.operation).toBe('error');
        expect(response.payload).toEqual({kind:'compile',error:{kind:'unsupported_feature',feature:'source scanning'}});
    }
    expect(result.prepared.operation).toBe('prepared');
    expect(result.committed.operation).toBe('committed');
    expect(result.committed.payload.css).toContain('width: 137px');
    await info.attach('Compiler-only Worker boundary', {body:JSON.stringify(result,null,2),contentType:'application/json'});
});

test('non-scalar compiler output keeps its UTF-16 units across Worker and SDK installation', async ({page}, info) => {
    await ready(page);
    const result = await page.evaluate(async () => {
        const api = (window as any).__utility_lab;
        api.dispose('scope-a');
        api.mount('scope-a',10000,'@theme static { --custom-\\d800: 101px; } @tailwind utilities;');
        const report = await api.ensure('scope-a',[]);
        const units = Array.from({length:report.css.length},(_,index) => report.css.charCodeAt(index));
        const repeated = await api.ensure('scope-a',[]);
        const replayed = await api.retry('scope-a');
        return {units, report, repeated, replayed, sheets:document.adoptedStyleSheets.length};
    });
    expect(result.units).toContain(0xd800);
    expect(result.units).not.toContain(0xfffd);
    expect(result.report.css).toContain(':scope');
    expect(result.repeated.revision).toBe(result.report.revision);
    expect(result.replayed.css).toBe(result.report.css);
    expect(result.sheets).toBeLessThanOrEqual(2);
    await info.attach('UTF-16 installed report',{body:JSON.stringify(result,null,2),contentType:'application/json'});
});

test('configuration replacement commits raw component classes and CSS together and rolls failures back', async ({page}, info) => {
    const errors: string[] = [], failed: string[] = [];
    page.on('pageerror', error => errors.push(error.message));
    page.on('console', message => { if (message.type() === 'error') errors.push(message.text()); });
    page.on('requestfailed', request => failed.push(`${request.url()}: ${request.failure()?.errorText}`));
    await ready(page);
    const result = await page.evaluate(async () => {
        const api = (window as any).__utility_lab.componentBaseline;
        const id = 'cn-transaction';
        const root = document.createElement('div'); root.id = id; document.querySelector('main')!.append(root);
        api.mount(id);
        await api.ensure(id, ['p-4', 'bg-[#123456]']);
        const card = document.querySelector(`[data-testid="${id}-card"]`)!;
        const read = () => ({classes: card.className, padding: getComputedStyle(card).padding, snapshot: api.report(id)});
        const before = read();
        const beforeOtherPadding = getComputedStyle(document.getElementById('scope-b-sample')!).padding;
        const css = '@theme { --spacing: 0.5rem; } @tailwind utilities;';
        await api.replace(id, css, true);
        const installed = read();
        const original = CSSStyleSheet.prototype.replaceSync;
        CSSStyleSheet.prototype.replaceSync = function () { throw new Error('injected configuration install failure'); };
        let installation;
        try { await api.replace(id, '@theme { --spacing: 1rem; } @tailwind utilities;', false); }
        catch (error) { installation = JSON.parse(String(error)); }
        finally { CSSStyleSheet.prototype.replaceSync = original; }
        const afterInstallationFailure = read();
        let compiler, callback;
        try { await api.replace(id, '@theme { .nested { color: red; } } @tailwind utilities;', false); }
        catch (error) { compiler = JSON.parse(String(error)); }
        try { await api.replace(id, css, false, true); }
        catch (error) { callback = JSON.parse(String(error)); }
        const afterFailures = read();
        await api.replace(id, '@theme { --spacing: 0.25rem; } @tailwind utilities;', false);
        const restored = read();
        return {before, installed, installation, afterInstallationFailure, compiler, callback, afterFailures, restored, beforeOtherPadding,
            otherPadding: getComputedStyle(document.getElementById('scope-b-sample')!).padding};
    });
    expect(result.before.classes).toBe('bg-[#123456] p-4');
    expect(result.before.padding).toBe('16px');
    expect(result.installed.classes).toBe('p-2 bg-[#123456] p-4');
    expect(result.installed.padding).toBe('32px');
    expect(result.installed.snapshot.epoch).toBe(result.installed.snapshot.cn_epoch);
    expect(result.installed.snapshot.report.revision).toBeGreaterThan(result.before.snapshot.report.revision);
    expect(result.installed.snapshot.report.admitted).toContain('p-2');
    expect(result.installation.kind).toBe('install');
    expect(result.afterInstallationFailure).toEqual(result.installed);
    expect(result.compiler.kind).toBe('compile');
    expect(result.callback.message).toContain('fixture callback failed');
    expect(result.afterFailures).toEqual(result.installed);
    expect(result.restored.classes).toBe('bg-[#123456] p-4');
    expect(result.restored.padding).toBe('16px');
    expect(result.restored.snapshot.epoch).toBe(result.restored.snapshot.cn_epoch);
    expect(result.otherPadding).toBe(result.beforeOtherPadding);
    await info.attach('configuration transaction', {body: JSON.stringify(result, null, 2), contentType: 'application/json'});
    await page.evaluate(() => (window as any).__utility_lab.componentBaseline.dispose('cn-transaction'));
    expect(errors).toEqual([]); expect(failed).toEqual([]);
});

test('configuration ack loss replays the installed cn and compiler configuration', async ({page}, info) => {
    await ready(page);
    const result = await page.evaluate(async () => {
        const api = (window as any).__utility_lab.componentBaseline;
        const id = 'cn-ack';
        const root = document.createElement('div'); root.id = id; document.querySelector('main')!.append(root);
        api.mount(id);
        await api.ensure(id, ['p-4', 'bg-[#123456]']);
        const owner = root.dataset.rustifyStyleOwner;
        const original = Worker.prototype.postMessage;
        Worker.prototype.postMessage = function (message: any, ...rest: any[]) {
            if (message.runtime_id === owner && message.operation === 'ack') return;
            return original.call(this, message, ...rest);
        };
        const css = '@theme { --spacing: 0.5rem; } @tailwind utilities;';
        let failure;
        try { await api.replace(id, css, true); }
        catch (error) { failure = JSON.parse(String(error)); }
        finally { Worker.prototype.postMessage = original; }
        const installed = api.report(id);
        const replayed = await api.retry(id);
        const card = document.querySelector(`[data-testid="${id}-card"]`)!;
        return {failure, installed, replayed, classes: card.className, padding: getComputedStyle(card).padding};
    });
    expect(result.failure.kind).toBe('timeout');
    expect(result.installed.config.input.css).toContain('0.5rem');
    expect(result.replayed.config).toEqual(result.installed.config);
    expect(result.replayed.epoch).toBeGreaterThan(result.installed.epoch);
    expect(result.replayed.cn_epoch).toBe(result.replayed.epoch);
    expect(result.classes).toBe('p-2 bg-[#123456] p-4');
    expect(result.padding).toBe('32px');
    await info.attach('configuration ack recovery', {body: JSON.stringify(result, null, 2), contentType: 'application/json'});
    await page.evaluate(() => (window as any).__utility_lab.componentBaseline.dispose('cn-ack'));
});

test('rapid configuration edits supersede old work and retain same epoch additions', async ({page}) => {
    await ready(page);
    const result = await page.evaluate(async () => {
        const api = (window as any).__utility_lab.componentBaseline;
        const id = 'cn-rapid';
        const root = document.createElement('div'); root.id = id; document.querySelector('main')!.append(root);
        api.mount(id);
        await api.ensure(id, ['p-4', 'bg-[#123456]']);
        const first = api.replace(id, '@theme { --spacing: 1rem; } @tailwind utilities;', false)
            .then(() => 'unexpected success', (error: any) => JSON.parse(String(error)).kind);
        const second = api.replace(id, '@theme { --spacing: 0.5rem; } @tailwind utilities;', true);
        const sameEpoch = api.ensure(id, ['w-[183px]']);
        const flushed = api.flush(id);
        const stale = await first;
        const latest = await second;
        await sameEpoch;
        const barrier = await flushed;
        const unchanged = await api.flush(id);
        const card = document.querySelector(`[data-testid="${id}-card"]`)!;
        const probe = document.createElement('div'); probe.className = 'w-[183px]'; root.append(probe);
        return {stale, latest, barrier, unchanged, final: api.report(id), width: getComputedStyle(probe).width, padding: getComputedStyle(card).padding};
    });
    expect(result.stale).toBe('superseded');
    expect(result.latest.epoch).toBe(result.latest.cn_epoch);
    expect(result.final.report.admitted).toContain('w-[183px]');
    expect(result.barrier.epoch).toBe(result.final.epoch);
    expect(result.barrier.report.revision).toBe(result.final.report.revision);
    expect(result.barrier.report.css).toBe(result.final.report.css);
    expect(result.unchanged.report.revision).toBe(result.barrier.report.revision);
    expect(result.unchanged.report.css).toBe(result.barrier.report.css);
    expect(result.width).toBe('183px');
    expect(result.padding).toBe('32px');
    await page.evaluate(() => (window as any).__utility_lab.componentBaseline.dispose('cn-rapid'));
});

test('scoped keyframes follow theme animation variables, aliases and fallbacks in each owner', async ({page}, info) => {
    const errors: string[] = [];
    page.on('pageerror', error => errors.push(error.message));
    page.on('console', message => { if (message.type() === 'error') errors.push(message.text()); });
    await ready(page);
    const result = await page.evaluate(async () => {
        const api = (window as any).__utility_lab;
        const read = async (id: string, opacity: string) => {
            const root = document.createElement('div'); root.id = id; document.querySelector('main')!.append(root);
            api.mount(id, 10000, `@theme { --animate-card: card 1s linear infinite; --animate-alias: var(--animate-card); @keyframes card { to { opacity: ${opacity}; } } } @tailwind utilities; .fallback { animation: var(--undefined, card 2s linear infinite); }`);
            const frame = document.getElementById(`${id}-frame`)!;
            const samples = ['animate-card', 'animate-alias', 'fallback'].map(className => {
                const element = document.createElement('div'); element.className = className; frame.append(element); return element;
            });
            const report = await api.ensure(id, ['animate-card', 'animate-alias']);
            return {owner: root.dataset.rustifyStyleOwner, css: report.css, samples: samples.map(element => {
                const animations = element.getAnimations() as CSSAnimation[];
                const frames = animations[0] ? (animations[0].effect as KeyframeEffect).getKeyframes() : [];
                return {name: getComputedStyle(element).animationName, count: animations.length, opacity: frames.at(-1)?.opacity};
            })};
        };
        const alpha = await read('animation-alpha', '0');
        const beta = await read('animation-beta', '0.5');
        api.dispose('animation-alpha'); api.dispose('animation-beta');
        return {alpha, beta};
    });
    for (const [scope, opacity] of [[result.alpha, '0'], [result.beta, '0.5']] as const) {
        expect(scope.css).toContain(`--animate-card: rustify-${scope.owner}-card 1s linear infinite`);
        for (const sample of scope.samples) {
            expect(sample.name).toBe(`rustify-${scope.owner}-card`);
            expect(sample.count).toBe(1);
            expect(sample.opacity).toBe(opacity);
        }
    }
    expect(result.alpha.owner).not.toBe(result.beta.owner);
    await info.attach('scoped animation variables', {body: JSON.stringify(result, null, 2), contentType: 'application/json'});
    expect(errors).toEqual([]);
});

test('a keyframes name that is also a timing keyword keeps the timing variable intact', async ({page}, info) => {
    const errors: string[] = [];
    page.on('pageerror', error => errors.push(error.message));
    page.on('console', message => { if (message.type() === 'error') errors.push(message.text()); });
    await ready(page);
    const result = await page.evaluate(async () => {
        const api = (window as any).__utility_lab;
        const id = 'animation-keyword';
        const root = document.createElement('div'); root.id = id; document.querySelector('main')!.append(root);
        api.mount(id, 10000, ':root { --ease: linear; } @keyframes card { to { opacity: 0; } } @keyframes linear { to { opacity: 0.5; } } .main-motion { animation: card 1s var(--ease) infinite; } .other-motion { animation-name: linear; animation-duration: 2s; } @tailwind utilities;');
        const frame = document.getElementById(`${id}-frame`)!;
        const samples = ['main-motion', 'other-motion'].map(className => {
            const element = document.createElement('div'); element.className = className; frame.append(element); return element;
        });
        const report = await api.ensure(id, []);
        const owner = root.dataset.rustifyStyleOwner;
        const read = samples.map(element => {
            const animations = element.getAnimations() as CSSAnimation[];
            const frames = animations[0] ? (animations[0].effect as KeyframeEffect).getKeyframes() : [];
            const style = getComputedStyle(element);
            return {name: style.animationName, timing: style.animationTimingFunction, count: animations.length, opacity: frames.at(-1)?.opacity};
        });
        api.dispose(id);
        return {owner, css: report.css, main: read[0], other: read[1]};
    });
    expect(result.css).toContain('--ease: linear');
    expect(result.main).toEqual({name: `rustify-${result.owner}-card`, timing: 'linear', count: 1, opacity: '0'});
    expect(result.other).toEqual({name: `rustify-${result.owner}-linear`, timing: 'ease', count: 1, opacity: '0.5'});
    await info.attach('keyword-named keyframes', {body: JSON.stringify(result, null, 2), contentType: 'application/json'});
    expect(errors).toEqual([]);
});

test('keyframes names keep their slot after substitution, math times and numeric iterations', async ({page}, info) => {
    const errors: string[] = [];
    page.on('pageerror', error => errors.push(error.message));
    page.on('console', message => { if (message.type() === 'error') errors.push(message.text()); });
    await ready(page);
    const cases = [
        {id: 'var-before', css: ':root { --x: linear; } @keyframes ease { to { opacity: 0.3; } } .motion { animation: var(--x) 10s ease infinite; }'},
        {id: 'var-name', css: ':root { --name: linear; } @keyframes linear { to { opacity: 0.3; } } .motion { animation: 10s ease var(--name) infinite; }'},
        {id: 'var-fallback', css: ':root { --x: ease; } @keyframes linear { to { opacity: 0.3; } } .motion { animation: var(--x, linear) 10s linear infinite; }'},
        {id: 'time-calc', css: '@keyframes card { to { opacity: 0.3; } } .motion { animation: calc(10s) linear card infinite; }'},
        {id: 'time-min', css: '@keyframes card { to { opacity: 0.3; } } .motion { animation: min(10s, 12s) linear card infinite; }'},
        {id: 'iteration', css: '@keyframes infinite { to { opacity: 0.3; } } .motion { animation: 2 10s infinite linear; }'},
        {id: 'iteration-fraction', css: '@keyframes infinite { to { opacity: 0.3; } } .motion { animation: 0.5 10s infinite linear; }'},
    ];
    const result = await page.evaluate(async cases => {
        const api = (window as any).__utility_lab;
        const read = (element: Element) => {
            const animations = element.getAnimations() as CSSAnimation[];
            const frames = animations[0] ? (animations[0].effect as KeyframeEffect).getKeyframes() : [];
            const style = getComputedStyle(element);
            return {name: style.animationName, timing: style.animationTimingFunction, duration: style.animationDuration,
                iteration: style.animationIterationCount, count: animations.length, opacity: frames.at(-1)?.opacity};
        };
        const samples = (parent: Element) => ['motion', 'other'].map(className => {
            const element = document.createElement('div'); element.className = className; parent.append(element); return element;
        });
        const rows = [];
        for (const {id, css} of cases) {
            const scope = `slot-${id}`;
            const root = document.createElement('div'); root.id = scope; document.querySelector('main')!.append(root);
            api.mount(scope, 10000, `${css} @tailwind utilities;`);
            const scoped = samples(document.getElementById(`${scope}-frame`)!);
            const report = await api.ensure(scope, []);
            const owner = root.dataset.rustifyStyleOwner;
            const actual = scoped.map(read);
            api.dispose(scope); root.remove();
            const reference = document.createElement('div'); reference.id = `${scope}-reference`; document.querySelector('main')!.append(reference);
            const plain = samples(reference);
            const sheet = new CSSStyleSheet(); sheet.replaceSync(css.replaceAll(':root', `#${reference.id}`));
            document.adoptedStyleSheets = [...document.adoptedStyleSheets, sheet];
            const expected = plain.map(read);
            document.adoptedStyleSheets = document.adoptedStyleSheets.filter(known => known !== sheet); reference.remove();
            rows.push({id, owner, css: report.css, actual, expected});
        }
        return rows;
    }, cases);
    for (const row of result) {
        row.actual.forEach((actual, index) => {
            const expected = row.expected[index];
            if (expected.name === 'none') { expect(actual, `${row.id}[${index}]`).toEqual(expected); return; }
            expect(expected.count, `${row.id}[${index}] reference`).toBe(1);
            expect(actual, `${row.id}[${index}]`).toEqual({...expected, name: `rustify-${row.owner}-${expected.name}`});
        });
    }
    await info.attach('keyframes slots after substitution', {body: JSON.stringify(result, null, 2), contentType: 'application/json'});
    expect(errors).toEqual([]);
});

test('a public property carrying a keyframes name follows overrides, and one with conflicting uses is refused', async ({page}, info) => {
    const errors: string[] = [];
    page.on('pageerror', error => errors.push(error.message));
    page.on('console', message => { if (message.type() === 'error') errors.push(message.text()); });
    await ready(page);
    const result = await page.evaluate(async () => {
        const api = (window as any).__utility_lab;
        const id = 'animation-public';
        const root = document.createElement('div'); root.id = id; document.querySelector('main')!.append(root);
        api.mount(id, 10000, ':root { --v: linear; --a: var(--v); } @keyframes linear { to { opacity: 0.5; } } .motion { animation-name: var(--v); animation-duration: 10s; } .alias { animation-name: var(--a); animation-duration: 10s; } @tailwind utilities;');
        const frame = document.getElementById(`${id}-frame`)!;
        const [motion, alias, other] = ['motion', 'alias', 'motion'].map(className => {
            const element = document.createElement('div'); element.className = className; frame.append(element); return element;
        });
        const report = await api.ensure(id, []);
        const owner = root.dataset.rustifyStyleOwner;
        const read = (element: HTMLElement) => {
            const animations = element.getAnimations() as CSSAnimation[];
            const frames = animations[0] ? (animations[0].effect as KeyframeEffect).getKeyframes() : [];
            return {variable: getComputedStyle(element).getPropertyValue('--v').trim(), name: getComputedStyle(element).animationName,
                count: animations.length, opacity: frames.at(-1)?.opacity};
        };
        const before = [motion, alias, other].map(read);
        other.style.setProperty('--v', 'none');
        const element = [motion, other].map(read);
        root.style.setProperty('--v', 'none');
        const inherited = [motion, alias].map(read);
        root.style.removeProperty('--v');
        const fail = async (css: string) => {
            try { await api.replace(id, css); return null; } catch (error) { return JSON.parse(String(error)); }
        };
        const shared = await fail(':root { --ease: linear; } @keyframes card { to { opacity: 0; } } @keyframes linear { to { opacity: 0.5; } } .motion { animation: card 10s var(--ease) infinite; } .alias { animation-name: var(--ease); animation-duration: 10s; } @tailwind utilities;');
        const plain = await fail(':root { --v: linear; } @keyframes linear { to { opacity: 0.5; } } .motion { animation-name: var(--v); animation-duration: 10s; } .alias { transition-timing-function: var(--v); } @tailwind utilities;');
        const after = api.report(id);
        const kept = [motion, alias].map(read);
        api.dispose(id);
        return {owner, css: report.css, revision: report.revision, before, element, inherited, shared, plain,
            afterRevision: after.revision, afterCss: after.css, kept};
    });
    const named = {variable: `rustify-${result.owner}-linear`, name: `rustify-${result.owner}-linear`, count: 1, opacity: '0.5'};
    const stopped = {variable: 'none', name: 'none', count: 0, opacity: undefined};
    expect(result.css).toContain(`--v: rustify-${result.owner}-linear`);
    expect(result.css).not.toContain('-k1-');
    expect(result.before).toEqual([named, named, named]);
    expect(result.element).toEqual([named, stopped]);
    expect(result.inherited).toEqual([stopped, stopped]);
    expect(result.shared).toEqual({kind: 'compile', error: {kind: 'unsupported_feature',
        feature: 'custom property `--ease` used both as a keyframes name and with another meaning'}});
    expect(result.plain).toEqual({kind: 'compile', error: {kind: 'unsupported_feature',
        feature: 'custom property `--v` used both as a keyframes name and with another meaning'}});
    expect(result.afterRevision).toBe(result.revision);
    expect(result.afterCss).toBe(result.css);
    expect(result.kept).toEqual([named, named]);
    await info.attach('public keyframes properties', {body: JSON.stringify(result, null, 2), contentType: 'application/json'});
    expect(errors).toEqual([]);
});

test('rebuild releases inactive candidates and keeps the seed, DOM and named classes', async ({page}, info) => {
    const errors: string[] = [];
    page.on('pageerror', error => errors.push(error.message));
    page.on('console', message => { if (message.type() === 'error') errors.push(message.text()); });
    await ready(page);
    const result = await page.evaluate(async () => {
        const api = (window as any).__utility_lab;
        const id = 'rebuild-quota';
        const limits = {input_bytes: 65536, asset_bytes: 65536, candidate_count: 1024, ast_depth: 32,
            css_bytes: 262144, queued_bytes: 65536, cache_bytes: 1048576};
        const root = document.createElement('div'); root.id = id; document.querySelector('main')!.append(root);
        api.mount(id, 10000, undefined, limits);
        // The seed, the mounted DOM and nothing else: the smallest set a rebuild can keep.
        const seeded = (await api.rebuild(id, [])).admitted;
        const baseline = api.retained(id);
        api.dispose(id); root.replaceChildren();
        api.mount(id, 10000, undefined, {...limits, candidate_count: baseline + 2});
        await api.flush(id);
        const failure = async (run: () => Promise<unknown>) => {
            try { await run(); return null; } catch (error) { return JSON.parse(String(error)); }
        };
        const frame = document.getElementById(`${id}-frame`)!;
        const shown = document.createElement('div'); shown.className = 'h-[23px]'; frame.append(shown);
        const stale = document.createElement('div'); frame.append(stale);
        await api.ensure(id, ['w-[12345px]', 'h-[23px]']);
        stale.className = 'w-[12345px]';
        const staleWidth = getComputedStyle(stale).width;
        stale.className = '';
        const full = await failure(() => api.ensure(id, ['w-[12346px]']));
        const installed = api.report(id);
        const oversized = await failure(() => api.rebuild(id, ['w-[1px]', 'w-[2px]', 'w-[3px]']));
        const afterFailure = {report: api.report(id), retained: api.retained(id)};
        const rebuilt = await api.rebuild(id, []);
        const released = {retained: api.retained(id), admitted: rebuilt.admitted};
        await api.ensure(id, ['w-[12346px]']);
        const named = await api.rebuild(id, ['w-[12346px]']);
        stale.className = 'w-[12345px]';
        const releasedWidth = getComputedStyle(stale).width;
        const probe = document.createElement('div'); probe.className = 'w-[12346px]'; frame.append(probe);
        const output = {baseline, seeded, full, installed, oversized, afterFailure, released, staleWidth, releasedWidth,
            named: {retained: api.retained(id), admitted: named.admitted, revision: named.revision},
            shownHeight: getComputedStyle(shown).height, namedWidth: getComputedStyle(probe).width};
        api.dispose(id);
        return output;
    });
    expect(result.staleWidth).toBe('12345px');
    expect(result.full.error.resource).toBe('candidate_count');
    expect(result.oversized.error.resource).toBe('candidate_count');
    expect(result.afterFailure.report).toEqual(result.installed);
    expect(result.afterFailure.retained).toBe(result.baseline + 2);
    expect(result.released.retained).toBe(result.baseline + 1);
    expect(result.released.admitted).not.toContain('w-[12345px]');
    expect(result.released.admitted).toContain('h-[23px]');
    for (const seeded of result.seeded) expect(result.released.admitted).toContain(seeded);
    expect(result.named.retained).toBe(result.baseline + 2);
    expect(result.named.admitted).toContain('w-[12346px]');
    expect(result.named.revision).toBeGreaterThan(result.installed.revision);
    expect(result.releasedWidth).not.toBe('12345px');
    expect(result.shownHeight).toBe('23px');
    expect(result.namedWidth).toBe('12346px');
    await info.attach('active rebuild', {body: JSON.stringify(result, null, 2), contentType: 'application/json'});
    expect(errors).toEqual([]);
});
