const events = Application('System Events');
const app = events.applicationProcesses.whose({unixId:45249})[0];
app.frontmost = true;
function find(node, role) {
  if (node.role() === role) return node;
  if (node.role() === 'AXWebArea') return null;
  for (const child of node.uiElements()) { const found=find(child,role); if (found) return found; }
  return null;
}
function button() {
  const win=app.windows.whose({name:'Setup | Fixture Docs'})[0];
  return find(win,'AXPopover').uiElements().filter(x=>x.role()==='AXPopUpButton')[0];
}
const before=button().value();
events.click(button());
events.keystroke('200%');
events.keyCode(36);
delay(.3);
const after=button().value();
events.keyCode(53);
if (after !== '200%') throw new Error('Unexpected page zoom: '+after);
JSON.stringify({before,after,method:'Actual Safari website page-zoom popup',pid:45249});
