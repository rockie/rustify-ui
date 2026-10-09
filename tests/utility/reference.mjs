// Development conformance entry. No oracle code is copied into production assets.
import { spawnSync } from 'node:child_process'

const [flag, group] = process.argv.slice(2)
if (flag !== '--check' || !['cn', 'scanner', 'compiler-smoke', 'compiler-full'].includes(group)) {
  throw new Error('usage: node tests/utility/reference.mjs --check cn|scanner|compiler-smoke|compiler-full')
}
function run(program, args) {
  const result = spawnSync(program, args, { stdio: 'inherit' })
  if (result.error) throw result.error
  if (result.status !== 0) process.exit(result.status || 1)
}
if (group === 'compiler-smoke') {
  run('mbx', ['xtask', 'utility-conformance', '--check', '--group', group])
} else if (group === 'scanner') {
  run('python3', ['tests/utility/scanner/reference.py'])
  run('mbx', ['test', '--locked', '-p', 'rustify-style', '--no-default-features', '--features', 'scanner-fs', '--lib'])
  for (const example of ['scanner-probe', 'scanner-source-probe']) {
    run('mbx', ['build', '--locked', '-p', 'rustify-style', '--no-default-features', '--features', 'scanner', '--example', example, '--release', '--target', 'wasm32-unknown-unknown'])
  }
  run('node', ['tests/utility/scanner/run-wasm.mjs'])
  run('node', ['tests/utility/scanner/run-source-wasm.mjs'])
} else if (group === 'cn') {
  run('node', ['tests/utility/cn/reference.mjs', '--check'])
  run('mbx', ['test', '--locked', '-p', 'rustify-style', '--lib', 'cn::'])
  run('mbx', ['xtask', 'utility-conformance', '--check', '--group', 'cn-native'])
  run('mbx', ['xtask', 'utility-build', '--group', 'cn'])
  run('node', ['tests/utility/cn/run-build-host.mjs'])
} else {
  // Public API and family fixtures, then the upstream test files themselves.
  run('mbx', ['xtask', 'utility-conformance', '--check', '--group', 'compiler-full'])
}
