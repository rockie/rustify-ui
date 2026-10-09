import {readFileSync,writeFileSync} from 'node:fs';
import {pathToFileURL} from 'node:url';
const dir=process.cwd()+'/target/makepad-wasm-app/release/utility-lab/utility/compiler';
const m=await import(pathToFileURL(dir+'/style.js'));m.initSync({module:readFileSync(dir+'/style_bg.wasm')});
const cases=JSON.parse(readFileSync('docs/validation/wasm-tailwind/code-review-r3/animation-probe.json')).cases;
const results=[];
for(const c of cases){
 const outputs=[];
 for(const owner of [null,'review-r3']){
  const session=new m.CompilerSession(JSON.stringify({input:{css:c.css,entry:'memory:input.css'},assets:[],owner,limits:{input_bytes:65536,asset_bytes:65536,candidate_count:1024,ast_depth:32,css_bytes:262144,queued_bytes:65536,cache_bytes:1048576}}));
  const prepared=JSON.parse(session.prepare('[]'));outputs.push({owner,css:prepared.report.css});session.discard(prepared.token);session.free();
 }
 results.push({id:c.id,input:c.css,outputs});
}
writeFileSync('docs/validation/wasm-tailwind/code-review-r3/facade-probe.json',JSON.stringify({dir,results},null,2)+'\n');
console.log(JSON.stringify(results.map(r=>({id:r.id,css:r.outputs.map(o=>({owner:o.owner,lines:o.css.split('\n').filter(l=>l.includes('animation:')||l.includes('animation-name:')||l.includes('@keyframes'))}))})),null,2));
