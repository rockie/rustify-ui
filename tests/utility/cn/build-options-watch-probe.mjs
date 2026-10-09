// Isolated recording for the non-reproduced full-run native watcher timeout.
import {createCnBuild,nodeHost} from '../../../target/utility/cn/cn-node.mjs'
import fs from 'node:fs'
import {join} from 'node:path'
import {tmpdir} from 'node:os'
const limits={maxDepth:128,maxNodes:1000000,maxUtf16Units:10000000,maxTokens:1000000,cacheBytes:10000000}
const cwd=fs.mkdtempSync(join(tmpdir(),'cn-watch-getters-')),events=[]
fs.mkdirSync(join(cwd,'src'));fs.writeFileSync(join(cwd,'src/index.ts'),'p-2')
const native=nodeHost(),host={...native,watch:(dir,callback)=>native.watch(dir,(event,path)=>{events.push([event,path]);callback(event,path)})}
const bridge=await createCnBuild(limits,{host})
try {
 await bridge.session.with_cn({}, {cwd})('phase-development-server',{})
 const out=join(cwd,'cn-tables.mjs'),before=fs.readFileSync(out,'utf8')
 fs.writeFileSync(join(cwd,'src/index.ts'),'p-2 m-4')
 for(let i=0;i<100&&fs.readFileSync(out,'utf8')===before;i++)await new Promise(r=>setTimeout(r,20))
 const changed=fs.readFileSync(out,'utf8')!==before
 console.log(JSON.stringify({events,changed},null,2))
 if(!changed)process.exitCode=1
} finally {bridge.close();fs.rmSync(cwd,{recursive:true,force:true})}
