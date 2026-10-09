// Oracle only: route the fixed package's absent dist module to its original TS.
import './upstream.mjs'
import { registerHooks } from 'node:module'
import { pathToFileURL } from 'node:url'
import { upstreamRoot } from './upstream.mjs'
registerHooks({resolve(specifier,context,next){
 if(specifier==='../dist/build.js'&&context.parentURL===pathToFileURL(`${upstreamRoot}/packages/cn/bin/cn.mjs`).href)return next(pathToFileURL(`${upstreamRoot}/packages/cn/src/build.ts`).href,context)
 return next(specifier,context)
}})
