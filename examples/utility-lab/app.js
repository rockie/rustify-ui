import { boot, show_fatal, release_container } from './loader.js';
// The page's own words. The SDK ships its messages in both languages; an
// application ships the rest, chosen the same way: the first of the reader's
// languages this page speaks, by language subtag, else English.
const messages = {
  en: {
    subtitle: 'Apply complete class names to a live, isolated preview.', language: 'Language',
    classes: 'Classes', override: 'Override classes', conditional: 'Conditional classes',
    includeConditional: 'Include conditional classes', merged: 'Merged classes',
    apply: 'Apply classes', retry: 'Restart compiler', export: 'Copy CSS',
    partial: 'Apply supported GPU properties only', disable: 'Disable sample button',
    nodeInputs: 'Container, text and color block classes', containerClasses: 'Container classes',
    textClasses: 'Text classes', swatchClasses: 'Color block classes',
    scanSummary: 'Scan source text', scanIntro: 'Generate rules from HTML source, then apply complete preview classes above.',
    htmlSource: 'HTML source', scan: 'Scan source', sourceResult: 'Compiled source candidates',
    domPreview: 'DOM preview', domIntro: 'Updates after the generated stylesheet is installed.',
    gpuPreview: 'GPU preview', gpuIntro: 'Supported properties update after compilation. Unsupported properties keep the previous appearance.',
    isolation: 'Independent scope', isolationIntro: 'Uses its own compiler and stylesheet.',
    cssSummary: 'Generated CSS and diagnostics', outside: 'Outside every utility scope',
    reportEmpty: 'Apply classes to generate a stylesheet.',
    starting: 'Starting preview…', ready: 'Ready. Apply classes to update the preview.', compiling: 'Compiling classes…',
    installed: 'DOM styles installed. Revision {0}. See GPU results below.', needSupport: '{0} classes need compiler support.',
    applyFailed: 'Could not apply classes: {0}. Restart the compiler to retry.',
    restarted: 'Compiler restarted; installed styles restored.', restartFailed: 'Could not restart compiler: {0}',
    mergeFailed: 'Could not merge classes: {0}. The last applied preview is unchanged.',
    copied: 'CSS copied.', clipboard: 'Clipboard unavailable. Select the generated CSS to copy it.',
    scannerOn: 'Enter HTML source to generate utility rules.',
    scannerOff: 'Source scanning is disabled in this build. Build with --utility-scanner to enable it.',
    scannerChecking: 'Checking scanner availability…', sourceEmpty: 'Enter source text to scan.', scanning: 'Scanning source…',
    sourceInstalled: 'Source rules installed. Revision {0}. Apply preview classes above.',
    scanFailed: 'Could not scan source: {0}. Previous styles are retained.',
  },
  zh: {
    subtitle: '将完整类名应用到实时、隔离的预览。', language: '语言',
    classes: '类名', override: '覆盖类名', conditional: '条件类名',
    includeConditional: '包含条件类名', merged: '合并后的类名',
    apply: '应用类名', retry: '重启编译器', export: '复制 CSS',
    partial: '仅应用 GPU 支持的属性', disable: '禁用示例按钮',
    nodeInputs: '容器、文字和色块类名', containerClasses: '容器类名',
    textClasses: '文字类名', swatchClasses: '色块类名',
    scanSummary: '扫描源码文本', scanIntro: '从 HTML 源码生成规则，然后在上方应用完整的预览类名。',
    htmlSource: 'HTML 源码', scan: '扫描源码', sourceResult: '已编译的源码候选',
    domPreview: 'DOM 预览', domIntro: '生成的样式表安装后更新。',
    gpuPreview: 'GPU 预览', gpuIntro: '编译后更新支持的属性；不支持的属性保留之前的外观。',
    isolation: '独立作用域', isolationIntro: '使用自己的编译器和样式表。',
    cssSummary: '生成的 CSS 与诊断', outside: '位于所有 utility 作用域之外',
    reportEmpty: '应用类名以生成样式表。',
    starting: '正在启动预览…', ready: '就绪。应用类名以更新预览。', compiling: '正在编译类名…',
    installed: 'DOM 样式已安装。修订 {0}。GPU 结果见下方。', needSupport: '{0} 个类名需要编译器支持。',
    applyFailed: '无法应用类名：{0}。请重启编译器后重试。',
    restarted: '编译器已重启，已恢复已安装的样式。', restartFailed: '无法重启编译器：{0}',
    mergeFailed: '无法合并类名：{0}。上次应用的预览保持不变。',
    copied: '已复制 CSS。', clipboard: '剪贴板不可用。请选中生成的 CSS 手动复制。',
    scannerOn: '输入 HTML 源码以生成 utility 规则。',
    scannerOff: '此构建未启用源码扫描。使用 --utility-scanner 构建以启用。',
    scannerChecking: '正在检查扫描器是否可用…', sourceEmpty: '请输入要扫描的源码文本。', scanning: '正在扫描源码…',
    sourceInstalled: '源码规则已安装。修订 {0}。请在上方应用预览类名。',
    scanFailed: '无法扫描源码：{0}。之前的样式保持不变。',
  },
};
let language = navigator.languages.map(tag => tag.split(/[-_]/)[0].toLowerCase()).find(tag => tag in messages) || 'en';
const t = (key, ...args) => messages[language][key].replace(/\{(\d)\}/g, (_, index) => args[index]);
// Messages on screen, re-rendered when the language changes. Raw output such
// as generated CSS is never translated.
const shown = new Map();
function say(element, state, key, ...args) {
  if (state) element.dataset.status = state;
  shown.set(element, [key, args]);
  element.textContent = t(key, ...args);
}
function raw(element, text) { shown.delete(element); element.textContent = text; }
function localize() {
  document.documentElement.lang = language === 'zh' ? 'zh-CN' : 'en';
  for (const element of document.querySelectorAll('[data-i18n]')) element.textContent = t(element.dataset.i18n);
  for (const element of document.querySelectorAll('[data-i18n-label]')) element.setAttribute('aria-label', t(element.dataset.i18nLabel));
  for (const [element, [key, args]] of shown) element.textContent = t(key, ...args);
  document.getElementById('language').value = language;
  window.__utility_lab?.locale(language);
}
const status = document.getElementById('status');
say(status, null, 'starting');
say(document.getElementById('source-status'), null, 'scannerChecking');
say(document.getElementById('report'), null, 'reportEmpty');
localize();
document.getElementById('language').addEventListener('change', event => { language = event.target.value; localize(); });
let failed = false;
function fatal(error) {
  failed = true;
  delete window.__utility_lab;
  for (const id of ['apply', 'retry', 'export', 'scan']) document.getElementById(id).disabled = true;
  for (const id of ['scope-a', 'scope-b']) release_container(document.getElementById(id));
  status.dataset.status = 'error';
  // The loader's notice owns the status from here; a language switch must not
  // put an earlier message back over it.
  shown.delete(status);
  show_fatal(status, error);
}
async function publish(handle) {
  const app = handle.app;
  app.utility_lab_identify(handle.instance, handle.build);
  const api = {
    mount(id, timeout = 10000, css, limits) { app.utility_lab_mount(id, handle.base, timeout, css, limits && JSON.stringify(limits)); },
    async ensure(id, classes) { return JSON.parse(await app.utility_lab_ensure(id, JSON.stringify(classes))); },
    async replace(id, css) { return JSON.parse(await app.utility_lab_replace(id, css)); },
    async rebuild(id, active = []) { return JSON.parse(await app.utility_lab_rebuild(id, JSON.stringify(active))); },
    async flush(id) { return JSON.parse(await app.utility_lab_flush(id)); },
    async scanContent(id, content) { return JSON.parse(await app.utility_lab_scan_content(id, JSON.stringify(content))); },
    async scanSources(id, sources) { return JSON.parse(await app.utility_lab_scan_sources(id, JSON.stringify(sources))); },
    report(id) { return JSON.parse(app.utility_lab_report(id)); },
    retained(id) { return app.utility_lab_retained(id); },
    merge(id, fragments) { return app.utility_lab_merge(id, JSON.stringify(fragments)); },
    locale(tag) { app.utility_lab_locale(tag); },
    async gpuApply(id, classes, partial = false, disabled = false) { return JSON.parse(await app.utility_lab_gpu_apply(id, JSON.stringify(classes), partial, disabled)); },
    theme(id, dark = false, rem = 16, patch = {}) { app.utility_lab_theme(id, dark, rem, JSON.stringify(patch)); },
    gpuReport(id) { return JSON.parse(app.utility_lab_gpu_report(id)); },
    buttonBaseline: {
      mount(id, themed = true) { app.utility_lab_button_baseline_mount(id, handle.base, themed); },
      update(phase, dark = false, rem = 16, palette = false, disabled = false) { return app.utility_lab_button_baseline_update(phase, dark, rem, palette, disabled); },
      report() { return JSON.parse(app.utility_lab_button_baseline_report()); },
      dispose() { app.utility_lab_button_baseline_dispose(); },
    },
    componentBaseline: {
      mount(id, disjoint = false) { app.utility_lab_component_mount(id, handle.base, disjoint); },
      ensure(id, classes) { return app.utility_lab_component_ensure(id, JSON.stringify(classes)); },
      async replace(id, css, disjoint = false, failCallback = false) { return JSON.parse(await app.utility_lab_component_replace(id, css, disjoint, failCallback)); },
      async flush(id) { return JSON.parse(await app.utility_lab_component_flush(id)); },
      async retry(id) { return JSON.parse(await app.utility_lab_component_retry(id)); },
      report(id) { return JSON.parse(app.utility_lab_component_report(id)); },
      update(id, caller, small = false) { app.utility_lab_component_update(id, caller, small); },
      dispose(id) { app.utility_lab_component_dispose(id); },
    },
    keyboard(id, focused, active = false, clicked = false) { app.utility_lab_gpu_keyboard(id, focused, active, clicked); },
    async retry(id) { return JSON.parse(await app.utility_lab_retry(id)); },
    dispose(id) { return app.utility_lab_dispose(id); },
    trap() { app.utility_lab_trap(); },
    hooks: handle.hooks,
    async second(id) { const next = await boot({ wasm_url: new URL('./utility-lab.wasm', import.meta.url) }); next.app.utility_lab_identify(next.instance, next.build); next.app.utility_lab_mount(id, next.base, 10000, undefined, undefined); return next; },
  };
  const manifestResponse = await fetch(new URL('./utility/build-manifest.json', import.meta.url));
  if (!manifestResponse.ok) throw new Error(`Utility manifest: ${manifestResponse.status}`);
  const manifest = await manifestResponse.json();
  document.getElementById('scan').disabled = !manifest.scanner_enabled;
  say(document.getElementById('source-status'), null, manifest.scanner_enabled ? 'scannerOn' : 'scannerOff');
  api.mount('scope-a'); api.mount('scope-b');
  const sample = document.getElementById('scope-a-sample');
  sample.addEventListener('focus', () => api.keyboard('scope-a', true));
  sample.addEventListener('blur', () => api.keyboard('scope-a', false));
  sample.addEventListener('keydown', event => {
    if ([' ', 'Enter'].includes(event.key)) { event.preventDefault(); api.keyboard('scope-a', true, true); }
  });
  sample.addEventListener('keyup', event => {
    if ([' ', 'Enter'].includes(event.key)) { event.preventDefault(); api.keyboard('scope-a', true, false, true); }
  });
  sample.addEventListener('click', () => api.keyboard('scope-a', false, false, true));
  window.addEventListener('resize', () => {
    if (!failed) app.utility_lab_gpu_viewport('scope-a', innerWidth, innerHeight);
  });
  window.__utility_lab = api;
  localize();
  showMerged();
  for (const id of ['apply', 'retry', 'export']) document.getElementById(id).disabled = false;
  say(status, 'ready', 'ready');
}
// Base, override and conditional fragments go through the scope's committed cn
// once; the merged list is what the compiler and both previews receive.
function mergedClasses() {
  const fragments = [document.getElementById('classes').value, document.getElementById('override-classes').value];
  if (document.getElementById('conditional-enabled').checked) fragments.push(document.getElementById('conditional-classes').value);
  return window.__utility_lab.merge('scope-a', fragments);
}
function showMerged() {
  const output = document.getElementById('merged');
  const note = document.getElementById('merged-status');
  if (!window.__utility_lab) return;
  try { output.value = mergedClasses(); raw(note, ''); delete note.dataset.status; }
  catch (error) { say(note, 'error', 'mergeFailed', error); }
}
for (const id of ['classes', 'override-classes', 'conditional-classes', 'conditional-enabled']) {
  document.getElementById(id).addEventListener('input', showMerged);
}
document.getElementById('controls').addEventListener('submit', async event => {
  event.preventDefault();
  const button = document.getElementById('apply');
  button.disabled = true; say(status, null, 'compiling');
  try {
    const classes = mergedClasses().split(/\s+/).filter(Boolean);
    showMerged();
    const more = ['container-classes', 'text-classes', 'swatch-classes'].map(id => document.getElementById(id).value.split(/\s+/).filter(Boolean));
    const disabled = document.getElementById('disabled').checked;
    const report = await window.__utility_lab.gpuApply('scope-a', [more[0], classes, more[1], more[2]], document.getElementById('partial').checked, disabled);
    document.getElementById('scope-a-sample').className = `sample ${classes.join(' ')}`;
    for (const [id, base, values] of [['scope-a-frame', 'preview-content', more[0]], ['scope-a-text', 'sample-text', more[1]], ['scope-a-swatch', 'sample-swatch', more[2]]]) {
      document.getElementById(id).className = `${base} ${values.join(' ')}`;
    }
    document.getElementById('scope-a-sample').setAttribute('aria-disabled', String(disabled));
    raw(document.getElementById('report'), report.css + '\n' + JSON.stringify({ rejected: report.rejected, unsupported: report.unsupported }, null, 2));
    if (report.unsupported.length) say(status, 'error', 'needSupport', report.unsupported.length);
    else say(status, 'ready', 'installed', report.revision);
  } catch (error) { say(status, 'error', 'applyFailed', error); }
  finally { button.disabled = failed; }
});
document.getElementById('retry').addEventListener('click', async () => {
  try { await window.__utility_lab.retry('scope-a'); say(status, 'ready', 'restarted'); }
  catch (error) { say(status, 'error', 'restartFailed', error); }
});
document.getElementById('source-controls').addEventListener('submit', async event => {
  event.preventDefault();
  const source = document.getElementById('source').value;
  const sourceStatus = document.getElementById('source-status');
  if (!source.trim()) { say(sourceStatus, 'empty', 'sourceEmpty'); return; }
  const button = document.getElementById('scan');
  button.disabled = true; say(sourceStatus, 'loading', 'scanning');
  try {
    const report = await window.__utility_lab.scanContent('scope-a', [{content: source, extension: 'html'}]);
    raw(document.getElementById('source-result'), JSON.stringify({ admitted: report.admitted, rejected: report.rejected, unsupported: report.unsupported }, null, 2));
    raw(document.getElementById('report'), report.css);
    if (report.unsupported.length) say(sourceStatus, 'error', 'needSupport', report.unsupported.length);
    else say(sourceStatus, 'ready', 'sourceInstalled', report.revision);
  } catch (error) { say(sourceStatus, 'error', 'scanFailed', error); }
  finally { button.disabled = failed; }
});
document.getElementById('export').addEventListener('click', async () => {
  try { await navigator.clipboard.writeText(window.__utility_lab.report('scope-a')?.css || ''); say(status, null, 'copied'); }
  catch { say(status, 'error', 'clipboard'); }
});
boot({ wasm_url: new URL('./utility-lab.wasm', import.meta.url), on_fatal: fatal }).then(publish)
  .catch(error => { status.dataset.status = 'error'; show_fatal(status, error); });
