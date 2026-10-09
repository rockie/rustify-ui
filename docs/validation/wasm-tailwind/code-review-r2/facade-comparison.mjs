import {readFileSync,writeFileSync} from 'node:fs';
import {pathToFileURL} from 'node:url';
const css=':root { --ease: linear; } @keyframes card { to { opacity: 0; } } @keyframes linear { to { opacity: 0.5; } } .main-motion { animation: card 1s var(--ease) infinite; } .other-motion { animation-name: linear; animation-duration: 2s; } @tailwind utilities;';
const results=[];
for(const [dir,owner] of [[process.cwd()+'/docs/validation/wasm-tailwind/code-review-r2/old-facade','old'],[process.cwd()+'/target/makepad-wasm-app/release/utility-lab/utility/compiler','new']]) {
 const m=await import(pathToFileURL(dir+'/style.js'));m.initSync({module:readFileSync(dir+'/style_bg.wasm')});
 const session=new m.CompilerSession(JSON.stringify({input:{css,entry:'memory:input.css'},assets:[],owner,limits:{input_bytes:65536,asset_bytes:65536,candidate_count:1024,ast_depth:32,css_bytes:262144,queued_bytes:65536,cache_bytes:1048576}}));
 const prepared=JSON.parse(session.prepare('[]'));results.push({dir,owner,css:prepared.report.css});session.discard(prepared.token);session.free();
}
writeFileSync(process.argv[2],JSON.stringify({input:css,results},null,2)+'\n');
console.log(results.map(r=>({owner:r.owner,ease:r.css.match(/--ease: ([^;]+);/)?.[1]})));
