function run(argv){
  const events=Application('System Events'),app=events.applicationProcesses.whose({unixId:45249})[0];
  app.frontmost=true;
  function find(node,role){if(node.role()===role)return node;if(node.role()==='AXWebArea')return null;for(const child of node.uiElements()){const value=find(child,role);if(value)return value;}return null;}
  function win(){return app.windows().find(w=>/Fixture Docs|Home semantics/.test(w.name()));}
  events.perform(win().actions.byName('AXRaise'));
  events.click(app.menuBars[0].menuBarItems.byName('Safari浏览器').menus[0].menuItems.byName('127.0.0.1设置…'));
  delay(.2);
  function button(){return find(win(),'AXPopover').uiElements().filter(x=>x.role()==='AXPopUpButton')[0];}
  const before=button().value();
  events.click(button());events.keystroke(argv[0]);events.keyCode(36);delay(.2);
  const after=button().value();events.keyCode(53);
  if(after!==argv[0])throw new Error('Unexpected zoom '+after);
  return JSON.stringify({before,after});
}
