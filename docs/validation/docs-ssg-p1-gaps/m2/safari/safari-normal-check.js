const events=Application('System Events');
const app=events.applicationProcesses.whose({unixId:45249})[0];
function prop(node,key){try{return node[key]();}catch(_){return null;}}
function find(node,role,depth){
  if(node.role()===role)return node;
  if(!depth)return null;
  for(const child of node.uiElements()){const value=find(child,role,depth-1);if(value)return value;}
  return null;
}
function taskWindow(){
  for(const win of app.windows()){if(/Fixture Docs|Home semantics/.test(win.name()))return win;}
  throw new Error('No task window');
}
app.frontmost=true;
function go(url){
  events.perform(taskWindow().actions.byName('AXRaise'));
  events.keystroke('l',{using:'command down'});
  events.keystroke(url);
  events.keyCode(36);
  delay(.8);
}
function snapshot(){
  const win=taskWindow(),page=find(win,'AXWebArea',8),nodes=[];
  function walk(node,depth){
    nodes.push({role:prop(node,'role'),name:prop(node,'name'),value:prop(node,'value'),position:prop(node,'position'),size:prop(node,'size')});
    if(depth&&nodes.length<200)for(const child of node.uiElements())walk(child,depth-1);
  }
  walk(page,7);
  events.keystroke('l',{using:'command down'});
  const toolbar=find(win,'AXToolbar',3);
  const url=find(toolbar,'AXTextField',4).value();
  events.keyCode(53);
  return {window:win.name(),url,windowSize:win.size(),pagePosition:page.position(),pageSize:page.size(),h1:nodes.filter(n=>n.role==='AXHeading'&&n.value===1),header:nodes.find(n=>n.role==='AXGroup'),footerLinks:nodes.filter(n=>n.name==='Footer links'),theme:nodes.filter(n=>['System','Light','Dark'].includes(n.name)),copy:nodes.filter(n=>n.role==='AXButton'&&n.name==='Copy'),nodes};
}
const urls=[
 'http://127.0.0.1:52252/guide/setup/#check-the-version',
 'http://127.0.0.1:53233/rustify-ui/guide/setup/#check-the-version',
 'http://127.0.0.1:53327/#home',
 'http://127.0.0.1:53327/explicit/#%E5%85%A5%E5%8F%A3',
 'http://127.0.0.1:53327/raw/#%3C%2Fh1%3E',
 'http://127.0.0.1:53327/empty/',
 'http://127.0.0.1:53478/rustify-ui/#home',
 'http://127.0.0.1:53478/rustify-ui/explicit/#%E5%85%A5%E5%8F%A3',
 'http://127.0.0.1:53478/rustify-ui/raw/#%3C%2Fh1%3E',
 'http://127.0.0.1:53478/rustify-ui/empty/'
];
function run(argv){
  const results=[];
  for(const width of (argv[0]==='nojs'?[1280,640]:[1280])){
    taskWindow().size=[width,800];
    for(const url of urls){
      go(url);const value=snapshot();
      if(value.h1.length!==1)throw new Error('Unexpected headings at '+url);
      if(argv[0]==='nojs'&&(value.theme.length||value.copy.length))throw new Error('Scripts still active at '+url);
      results.push({requested:url,requestedWidth:width,script:argv[0]||'zoom',actual:value});
      if(url.endsWith('/empty/')){
        const page=find(taskWindow(),'AXWebArea',8);
        function footer(n){if(n.role()==='AXLink'&&n.name()==='Back to the home page')return n;for(const child of n.uiElements()){const q=footer(child);if(q)return q;}return null;}
        events.keyCode(125,{using:'command down'});delay(.2);
        events.perform(footer(page).actions.byName('AXPress'));delay(.5);
        results.push({footerFrom:url,actual:snapshot()});
      }
    }
  }
  return JSON.stringify(results);
}
