const events=Application('System Events'),app=events.applicationProcesses.whose({unixId:45249})[0];
app.frontmost=true;delay(.4);events.keyCode(53);delay(.2);
function win(){return app.windows().find(w=>/Fixture Docs|Home semantics/.test(w.name()));}
function prompt(node,depth){if(node.role()==='AXTextArea'&&node.name()==='控制台提示')return node;if(depth)for(const child of node.uiElements()){const q=prompt(child,depth-1);if(q)return q;}return null;}
function logs(node,depth,values){if(node.role()==='AXStaticText')values.push(node.name());if(depth)for(const child of node.uiElements())logs(child,depth-1,values);}
function consoleGroup(node,depth){if(node.role()==='AXGroup'&&node.name()==='控制台')return node;if(depth)for(const child of node.uiElements()){const q=consoleGroup(child,depth-1);if(q)return q;}return null;}
function address(node,depth){if(node.role()==='AXTextField'&&node.description()==='智能搜索栏')return node;if(depth)for(const child of node.uiElements()){const q=address(child,depth-1);if(q)return q;}return null;}
const results=[];
for(const url of ['http://127.0.0.1:52252/','http://127.0.0.1:53233/rustify-ui/','http://127.0.0.1:53327/','http://127.0.0.1:53478/rustify-ui/']){
  events.perform(win().actions.byName('AXRaise'));
  events.click(address(win(),6));events.keystroke('a',{using:'command down'});events.keyCode(51);address(win(),6).value=url;events.keyCode(36);delay(.8);
  const box=prompt(win(),12);if(!box)throw new Error('No console prompt');
  events.click(box);events.keyCode(53);events.keystroke('a',{using:'command down'});events.keyCode(51);
  box.value='JSON.stringify({url:location.href,reduce:matchMedia("(prefers-reduced-motion: reduce)").matches,scroll:getComputedStyle(document.documentElement).scrollBehavior})';
  events.keyCode(36);delay(.3);
  const values=[];logs(consoleGroup(win(),12),12,values);
  results.push({url,consoleFragments:values});
}
JSON.stringify(results);
