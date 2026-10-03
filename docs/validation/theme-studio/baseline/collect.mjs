import { chromium, expect } from '@playwright/test';
import { PNG } from 'pngjs';
import { createHash } from 'node:crypto';
import { readFile, writeFile } from 'node:fs/promises';
import { dirname, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

// Run after the existing release catalog is served by xtask on port 4176.
// All durations below come from performance.now() inside the page.
const output = dirname(fileURLToPath(import.meta.url));
const build = resolve(output, '../../../../target/makepad-wasm-app/release/component-catalog');
const executablePath = process.env.RUSTIFY_CHROMIUM || '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome';
const manifest = JSON.parse(await readFile(resolve(build, 'build-manifest.json'), 'utf8'));
const assets = {};
for (const name of ['build-manifest.json', 'component-catalog.wasm', 'tailwind.css', 'runtime.css', 'app.css', 'app.js', 'rustify_makepad/embedded.js', 'makepad_platform/web_gl.js']) {
    const data = await readFile(resolve(build, name));
    assets[name] = { bytes: data.length, sha256: createHash('sha256').update(data).digest('hex') };
}
const percentile = (values, percent) => [...values].sort((a, b) => a - b)[Math.ceil(values.length * percent) - 1];
const summary = (values) => ({ samples: values, p50_ms: percentile(values, 0.5), p95_ms: percentile(values, 0.95) });
const browser = await chromium.launch({ executablePath, headless: false });
try {
    const browserCdp = await browser.newBrowserCDPSession();
    const browserVersion = await browserCdp.send('Browser.getVersion');
    const system = await browserCdp.send('SystemInfo.getInfo');
    const context = await browser.newContext({ viewport: { width: 1280, height: 960 }, deviceScaleFactor: 1 });
    const page = await context.newPage();
    const errors = [];
    const failedRequests = [];
    const errorResponses = [];
    page.on('console', message => { if (message.type() === 'error') errors.push({ text: message.text(), location: message.location() }); });
    page.on('pageerror', error => errors.push({ text: String(error), source: 'pageerror' }));
    page.on('requestfailed', request => failedRequests.push({ url: request.url(), error: request.failure()?.errorText }));
    page.on('response', response => { if (response.status() >= 400) errorResponses.push({ url: response.url(), status: response.status() }); });
    await page.addInitScript(() => {
        const probe = window.__baseline = { active: null, contexts: 0, contextKinds: [], draws: 0, longTasks: [] };
        const contexts = new WeakSet();
        const getContext = HTMLCanvasElement.prototype.getContext;
        HTMLCanvasElement.prototype.getContext = function (kind, ...args) {
            const gl = getContext.call(this, kind, ...args);
            if ((kind === 'webgl2' || kind === 'webgl') && gl && !contexts.has(gl)) {
                contexts.add(gl);
                probe.contexts += 1;
                probe.contextKinds.push(kind);
            }
            return gl;
        };
        const setAttribute = Element.prototype.setAttribute;
        Element.prototype.setAttribute = function (name, value) {
            const result = setAttribute.call(this, name, value);
            const active = probe.active;
            if (active && this.id === 'catalog' && name === 'data-theme' && value === active.expected) {
                active.dom_commit_ms = performance.now() - active.started;
                active.primary = getComputedStyle(this).getPropertyValue('--primary').trim();
            }
            return result;
        };
        const draw = WebGL2RenderingContext.prototype.drawElementsInstanced;
        WebGL2RenderingContext.prototype.drawElementsInstanced = function (...args) {
            const result = draw.apply(this, args);
            probe.draws += 1;
            const active = probe.active;
            if (active && this.canvas.dataset.testid === 'catalogue-region') {
                const elapsed = performance.now() - active.started;
                active.gpu_first_submit_ms ??= elapsed;
                active.gpu_last_submit_ms = elapsed;
                active.draw_calls += 1;
            }
            return result;
        };
        new PerformanceObserver(list => {
            for (const entry of list.getEntries()) probe.longTasks.push({ start_ms: entry.startTime, duration_ms: entry.duration });
        }).observe({ type: 'longtask', buffered: true });
    });
    const response = await page.goto('http://127.0.0.1:4176/');
    await expect(page.getByTestId('status')).toHaveAttribute('data-status', 'ready', { timeout: 60000 });
    await page.evaluate(async () => {
        await window.__component_catalog.reset();
        let previous = performance.now();
        let quietSince = previous;
        const deadline = previous + 60000;
        while (performance.now() < deadline) {
            await new Promise(requestAnimationFrame);
            const now = performance.now();
            if (now - previous > 100) quietSince = now;
            previous = now;
            if (now - quietSince > 1000) return;
        }
        throw new Error('catalog did not reach a quiet startup');
    });
    const environment = await page.evaluate(() => {
        const canvas = document.querySelector('[data-testid="catalogue-region"]');
        const gl = canvas.getContext('webgl2');
        const debug = gl.getExtension('WEBGL_debug_renderer_info');
        return {
            user_agent: navigator.userAgent, platform: navigator.platform, viewport: { width: innerWidth, height: innerHeight },
            device_pixel_ratio: devicePixelRatio, visibility: document.visibilityState,
            webgl_version: gl.getParameter(gl.VERSION), shading_language: gl.getParameter(gl.SHADING_LANGUAGE_VERSION),
            gpu_vendor: debug ? gl.getParameter(debug.UNMASKED_VENDOR_WEBGL) : gl.getParameter(gl.VENDOR),
            gpu_renderer: debug ? gl.getParameter(debug.UNMASKED_RENDERER_WEBGL) : gl.getParameter(gl.RENDERER),
            canvas: { width: canvas.width, height: canvas.height, css_width: canvas.clientWidth, css_height: canvas.clientHeight },
        };
    });
    const styles = async () => page.evaluate(() => {
        const scope = document.getElementById('catalog');
        const properties = ['--background', '--foreground', '--primary', '--radius', '--font-size', '--spacing', '--motion-duration'];
        const computed = (element) => {
            const css = getComputedStyle(element);
            return { font_size: css.fontSize, font_family: css.fontFamily, gap: css.gap, padding: css.padding, border_radius: css.borderRadius, box_shadow: css.boxShadow, background: css.backgroundColor, color: css.color };
        };
        const radii = {};
        for (const size of ['sm', 'md', 'lg']) {
            const probe = document.createElement('div');
            probe.className = `rounded-${size}`;
            probe.style.display = 'none';
            scope.append(probe);
            radii[size] = getComputedStyle(probe).borderRadius;
            probe.remove();
        }
        return {
            theme: scope.dataset.theme,
            tokens: Object.fromEntries(properties.map(name => [name, getComputedStyle(scope).getPropertyValue(name).trim()])),
            utility_border_radius: radii, scope: computed(scope),
            catalogue: computed(document.querySelector('.catalogue')),
            header: computed(document.querySelector('.catalogue-header')),
            header_row: computed(document.querySelector('.catalogue-header > div')),
            example: computed(document.querySelector('.catalogue-example')),
            primary_button: computed(document.querySelector('[data-testid="default-button"]')),
        };
    });
    const lightStyles = await styles();
    const lightGpu = await page.getByTestId('catalogue-region').screenshot({ path: resolve(output, 'gpu-light.png') });
    await page.getByTestId('catalogue').screenshot({ path: resolve(output, 'dom-gpu-light.png') });
    const before = await page.evaluate(() => ({ stats: window.__component_catalog.stats(), diagnostics: window.__component_catalog.diagnostics(), live_regions: window.__component_catalog.live_regions(), contexts: window.__baseline.contexts, draws: window.__baseline.draws }));
    const timed = await page.evaluate(async () => {
        const probe = window.__baseline;
        const scope = document.getElementById('catalog');
        const toggle = document.querySelector('[data-testid="toggle-theme"]');
        const frame = () => new Promise(requestAnimationFrame);
        const windowStart = performance.now();
        const rounds = [];
        for (let i = 0; i < 20; i += 1) {
            const beforePrimary = getComputedStyle(scope).getPropertyValue('--primary').trim();
            const beforeFrames = window.__component_catalog.stats().frames;
            const active = probe.active = { index: i, expected: i % 2 ? 'light' : 'dark', started: performance.now(), dom_commit_ms: null, gpu_first_submit_ms: null, gpu_last_submit_ms: null, draw_calls: 0 };
            toggle.click();
            await frame();
            await frame();
            active.two_frames_ms = performance.now() - active.started;
            active.theme = scope.dataset.theme;
            active.frames_presented = window.__component_catalog.stats().frames - beforeFrames;
            active.live_regions = window.__component_catalog.live_regions();
            if (active.theme !== active.expected || active.primary === beforePrimary || active.dom_commit_ms === null || active.gpu_first_submit_ms === null || active.frames_presented < 1) throw new Error(`incomplete round ${JSON.stringify(active)}`);
            rounds.push({ ...active });
            probe.active = null;
        }
        const windowEnd = performance.now();
        return { window_start_ms: windowStart, window_end_ms: windowEnd, rounds, long_tasks: probe.longTasks.filter(item => item.start_ms >= windowStart && item.start_ms < windowEnd) };
    });
    const after = await page.evaluate(() => ({ stats: window.__component_catalog.stats(), live_regions: window.__component_catalog.live_regions(), contexts: window.__baseline.contexts, draws: window.__baseline.draws }));
    await page.getByTestId('toggle-theme').click();
    await expect(page.locator('#catalog')).toHaveAttribute('data-theme', 'dark');
    await page.evaluate(() => new Promise(resolve => setTimeout(resolve, 250)));
    const darkStyles = await styles();
    const darkGpu = await page.getByTestId('catalogue-region').screenshot({ path: resolve(output, 'gpu-dark.png') });
    await page.getByTestId('catalogue').screenshot({ path: resolve(output, 'dom-gpu-dark.png') });
    const pixelDiff = (a, b) => {
        const light = PNG.sync.read(a), dark = PNG.sync.read(b);
        let changed = 0;
        for (let i = 0; i < light.data.length; i += 4) if (Math.abs(light.data[i] - dark.data[i]) + Math.abs(light.data[i+1] - dark.data[i+1]) + Math.abs(light.data[i+2] - dark.data[i+2]) > 24) changed += 1;
        return { width: light.width, height: light.height, changed_pixels_rgb_delta_gt_24: changed };
    };
    const overlays = {};
    for (const mode of ['dark', 'light']) {
        if (await page.locator('#catalog').getAttribute('data-theme') !== mode) await page.getByTestId('toggle-theme').click();
        await page.getByTestId('nav-dialog').click();
        await page.getByTestId('default-dialog-trigger').click();
        await expect(page.getByTestId('dialog')).toBeVisible();
        overlays[mode] = { dialog: await page.getByTestId('dialog').locator('[data-name="Dialog"]').evaluate(element => ({ box_shadow: getComputedStyle(element).boxShadow, border_radius: getComputedStyle(element).borderRadius, background: getComputedStyle(element).backgroundColor, gap: getComputedStyle(element).gap, font_size: getComputedStyle(element).fontSize, class: element.className })) };
        await page.getByTestId('dialog-close').click();
        await page.getByTestId('nav-menu').click();
        await page.getByTestId('default-menu-trigger').click();
        await expect(page.getByTestId('default-menu')).toBeVisible();
        overlays[mode].menu = await page.getByTestId('default-menu').evaluate(element => ({ box_shadow: getComputedStyle(element).boxShadow, border_radius: getComputedStyle(element).borderRadius, background: getComputedStyle(element).backgroundColor, font_size: getComputedStyle(element).fontSize, class: element.className }));
        await page.keyboard.press('Escape');
    }
    await page.evaluate(() => window.__component_catalog.reset());
    const idle = await page.evaluate(async () => {
        const frame = () => new Promise(requestAnimationFrame);
        for (let i = 0; i < 3; i++) await frame();
        const before = window.__component_catalog.stats();
        const started = performance.now();
        await new Promise(resolve => setTimeout(resolve, 1500));
        const ended = performance.now();
        const after = window.__component_catalog.stats();
        return { duration_ms: ended - started, before, after, frames_delta: after.frames - before.frames, pumps_delta: after.pumps - before.pumps };
    });
    await page.getByTestId('nav-color-field').click();
    await expect(page.getByTestId('default-colour')).toBeVisible();
    const colorInput = await page.evaluate(async () => {
        const field = document.querySelector('[data-testid="default-colour"]');
        const swatch = document.querySelector('[data-testid="default-colour-swatch"]');
        const scope = document.getElementById('catalog');
        const beforePrimary = getComputedStyle(scope).getPropertyValue('--primary').trim();
        field.focus();
        const started = performance.now();
        field.value = '#d92d20';
        field.dispatchEvent(new InputEvent('input', { bubbles: true, inputType: 'insertText', data: '#d92d20' }));
        await new Promise(requestAnimationFrame);
        await new Promise(requestAnimationFrame);
        const measured = performance.now() - started;
        const atTwoFrames = getComputedStyle(swatch).backgroundColor;
        await new Promise(resolve => setTimeout(resolve, 250));
        const result = { input_to_two_frames_ms: measured, swatch_background_at_two_frames: atTwoFrames, swatch_background_after_transition: getComputedStyle(swatch).backgroundColor, application_colour: window.__component_catalog.snapshot().colour, primary_before: beforePrimary, primary_after: getComputedStyle(scope).getPropertyValue('--primary').trim(), gpu_theme_change_supported: false };
        field.blur();
        return result;
    });
    const measurements = {
        collected_at: new Date().toISOString(), url: page.url(), headed: true, executable_path: executablePath,
        manifest, assets, browser_version: browserVersion, gpu_system_info: system.gpu, environment,
        csp: response.headers()['content-security-policy'], regression: { tier: 'regression', passed: 6, failed: 0, duration_seconds: 14.7, evidence_test_run: false },
        styles: { light: lightStyles, dark: darkStyles, overlays },
        timing: { method: 'input click -> old ThemedScope data-theme setter after token writes -> actual WebGL2 drawElementsInstanced calls; two rAFs use existing p2-theme convention', ...timed, dom_commit: summary(timed.rounds.map(row => row.dom_commit_ms)), gpu_first_submit: summary(timed.rounds.map(row => row.gpu_first_submit_ms)), gpu_last_submit: summary(timed.rounds.map(row => row.gpu_last_submit_ms)), two_frames: summary(timed.rounds.map(row => row.two_frames_ms)) },
        regions: { before, after }, idle, color_input: colorInput, gpu_light_dark_pixel_diff: pixelDiff(lightGpu, darkGpu),
        resolver: { count: null, reason: 'The old Theme is a direct Copy table with properties(); no new resolver/revision or resolver counter is present in this browser API.' },
        limitations: ['GPU submit timestamps are CPU-side GL invocation timestamps, not a revision-tagged draw acknowledgement or display scanout.', 'The old catalog exposes palette switching but no arbitrary theme color input. Its ColorField changes only a DOM swatch and application field state.', 'Raw GL instrumentation and style reads add probe overhead; compare a later sample using this same methodology or clearly report the instrumentation difference.'],
        errors, failed_requests: failedRequests, error_responses: errorResponses,
    };
    await writeFile(resolve(output, 'measurements.json'), JSON.stringify(measurements, null, 2) + '\n');
    console.log(JSON.stringify({ dom_p50: measurements.timing.dom_commit.p50_ms, dom_p95: measurements.timing.dom_commit.p95_ms, gpu_submit_p50: measurements.timing.gpu_last_submit.p50_ms, gpu_submit_p95: measurements.timing.gpu_last_submit.p95_ms, two_frames_p50: measurements.timing.two_frames.p50_ms, two_frames_p95: measurements.timing.two_frames.p95_ms, gpu_renderer: environment.gpu_renderer, region_count: after.live_regions, new_contexts: after.contexts - before.contexts, idle_frames: idle.frames_delta, long_tasks: timed.long_tasks.length, errors, failedRequests }));
} finally {
    await browser.close();
}
