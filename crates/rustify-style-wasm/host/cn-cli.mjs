#!/usr/bin/env node
import { createCnBuild } from './cn-node.mjs'
// Bounded caller policy. These values are SDK limits, not upstream business thresholds.
const defaults={maxDepth:128,maxNodes:1000000,maxUtf16Units:100000000,maxTokens:1000000,cacheBytes:16000000}
try {
  const limits=process.env.RUSTIFY_CN_LIMITS?JSON.parse(process.env.RUSTIFY_CN_LIMITS):defaults
  const bridge=await createCnBuild(limits)
  try {
    const result=await bridge.session.run_cli(process.argv.slice(2))
    process.stdout.write(result.stdout)
    process.stderr.write(result.stderr)
    process.exitCode=result.exitCode
  } finally {bridge.close()}
} catch(error){process.stderr.write(`cn: ${error?.message??String(error)}\n`);process.exitCode=1}
