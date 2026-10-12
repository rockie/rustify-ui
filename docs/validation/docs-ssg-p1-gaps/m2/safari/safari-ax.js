const events = Application('System Events');
const app = events.applicationProcesses.whose({unixId: 45249})[0];
const win = app.windows[0];
if (!/Fixture Docs|Home semantics/.test(win.name())) throw new Error('The active window is outside this validation');
function prop(node, key) { try { return node[key](); } catch (_) { return null; } }
function area(node, depth) {
  if (prop(node, 'role') === 'AXWebArea') return node;
  if (!depth) return null;
  for (const child of node.uiElements()) { const found = area(child, depth - 1); if (found) return found; }
  return null;
}
const page = area(win, 8);
if (!page) throw new Error('No web area found');
let count = 0;
function tree(node, depth) {
  if (++count > 140) return null;
  const value = {role:prop(node,'role'),name:prop(node,'name'),description:prop(node,'description'),value:prop(node,'value'),position:prop(node,'position'),size:prop(node,'size')};
  if (depth) value.children = node.uiElements().map(child=>tree(child,depth-1)).filter(Boolean);
  return value;
}
JSON.stringify({window:win.name(),webArea:tree(page,5)});
