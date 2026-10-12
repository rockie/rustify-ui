const e = Application('System Events');
const safari = e.applicationProcesses.byName('Safari');
const settings = e.applicationProcesses.byName('System Settings');
const report = {observedAt: new Date().toISOString(), query: null};
function find(n, role, name, depth) {
  if (n.role() === role && n.name() === name) return n;
  if (depth) for (const c of n.uiElements()) {
    const result = find(c, role, name, depth - 1);
    if (result) return result;
  }
  return null;
}
function focus(w) {
  safari.frontmost = true;
  e.perform(w.actions.byName('AXRaise'));
  delay(.2);
  if (!safari.frontmost()) throw new Error('Safari did not become frontmost');
}
function preferences() {
  e.click(safari.menuBars[0].menuBarItems.byName('Safari浏览器').menus[0].menuItems.byName('设置…'));
  delay(.2);
  const w = safari.windows().find(w => /^(通用|高级|开发者)$/.test(w.name()));
  if (!w) throw new Error('No owned Safari preferences window');
  e.click(find(w, 'AXButton', '高级', 6));
  delay(.2);
  return safari.windows.whose({name: '高级'})[0];
}
function closePreferences(w) {
  e.click(find(w, 'AXButton', '通用', 6));
  delay(.2);
  const general = safari.windows.whose({name: '通用'})[0];
  e.click(general.buttons().find(b => b.description() === '关闭按钮'));
}
const motion = find(settings.windows.whose({name: '动态效果'})[0], 'AXCheckBox', '减弱动态效果', 12);
report.motionBefore = motion.value();
let developerBefore = null;
let inspectorOpened = false;
try {
  const native = Application('Safari');
  const browserWindow = native.windows.byId(30864);
  const expectedUrl = 'http://127.0.0.1:53327/';
  browserWindow.currentTab.url = expectedUrl;
  delay(.8);
  if (browserWindow.currentTab.url() !== expectedUrl) throw new Error('Native Safari URL did not match');
  report.nativeUrl = browserWindow.currentTab.url();
  const w = safari.windows.whose({name: 'Home | Home semantics'})[0];
  focus(w);
  const pref = preferences();
  const developer = find(pref, 'AXCheckBox', '显示网页开发者功能', 8);
  if (!developer) throw new Error('Developer checkbox is unavailable');
  developerBefore = developer.value();
  report.developerBefore = developerBefore;
  if (developerBefore === 0) e.click(developer);
  closePreferences(pref);
  if (report.motionBefore === 0) e.click(motion);
  focus(w);
  e.keystroke('c', {using: ['command down', 'option down']}); inspectorOpened = true; delay(.5);
  const box = find(w, 'AXTextArea', '控制台提示', 14);
  if (!box) throw new Error('No console prompt');
  report.query = '"RUSTIFY_MOTION:" + btoa(JSON.stringify({url:location.href,reduce:matchMedia("(prefers-reduced-motion: reduce)").matches,scroll:getComputedStyle(document.documentElement).scrollBehavior}))';
  e.click(box); e.keyCode(53); e.keystroke('a', {using: 'command down'}); e.keyCode(51);
  box.value = report.query; e.keyCode(36); delay(.3);
  const console = find(w, 'AXGroup', '控制台', 14);
  report.fragments = [];
  function collect(n, depth) {
    if (n.role() === 'AXStaticText') report.fragments.push(n.name());
    if (depth) for (const c of n.uiElements()) collect(c, depth - 1);
  }
  if (console) collect(console, 12);
} catch (error) {
  report.error = String(error);
} finally {
  if (motion.value() !== report.motionBefore) e.click(motion);
  report.motionAfter = motion.value();
  try {
    const w = safari.windows().find(w => w.name() === 'Home | Home semantics');
    if (inspectorOpened && w) { focus(w); e.keystroke('c', {using: ['command down', 'option down']}); delay(.2); }
    if (developerBefore !== null) {
      const pref = preferences();
      const developer = find(pref, 'AXCheckBox', '显示网页开发者功能', 8);
      if (developer.value() !== developerBefore) e.click(developer);
      report.developerAfter = developer.value();
      closePreferences(pref);
    }
  } catch (error) { report.cleanupError = String(error); }
}
JSON.stringify(report);
