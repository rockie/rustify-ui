// Node owns IO and module evaluation. Business rules execute in the Rust facade.
import { readFileSync, writeFileSync, mkdirSync, readdirSync, realpathSync, statSync, watch } from 'node:fs'
import { resolve, dirname, relative, sep } from 'node:path'
import { pathToFileURL } from 'node:url'
import { gzipSync } from 'node:zlib'
const kind=entry=>entry.isSymbolicLink()?2:entry.isDirectory()?1:entry.isFile()?0:3
export function nodeHost(){
  const watchers=new Set()
  return {
    cwd:()=>process.cwd(),
    resolve:(base,path)=>resolve(base,path),
    relative:(base,path)=>relative(base,path).split(sep).join("/"),
    dirname:path=>dirname(path),
    readText:path=>readFileSync(path,'utf8'),
    writeText:(path,source)=>{mkdirSync(dirname(path),{recursive:true});writeFileSync(path,source)},
    realpath:path=>realpathSync(path),
    kind:path=>kind(statSync(path)),
    children:path=>readdirSync(path,{withFileTypes:true}).map(entry=>[entry.name,kind(entry)]),
    importModule:path=>import(pathToFileURL(path).href),
    gzipSize:source=>gzipSync(source,{level:9}).length,
    watch:(directory,callback)=>{const watcher=watch(directory,{recursive:true,persistent:false},callback);watchers.add(watcher);watcher.on('close',()=>watchers.delete(watcher));return watcher},
    reportError:message=>console.error(message),
    close:()=>{for(const watcher of watchers)watcher.close();watchers.clear()},
  }
}
export async function createCnBuild(limits,{module=new URL('./style.js',import.meta.url),binary=new URL('./style_bg.wasm',import.meta.url),host=nodeHost()}={}){
  const wasm=await import(module.href??module)
  await wasm.default({module_or_path:readFileSync(binary)})
  const session=new wasm.CnBuildSession(host,limits)
  return {session,host,close(){host.close?.();session.free()}}
}
