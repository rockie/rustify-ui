// Runs the fixed Tailwind test files from ref/tailwindcss. With
// RUSTIFY_UPSTREAM_IMPL=rust the package entry resolves to the Rust/WASM
// facade; `official` keeps the upstream TypeScript as a harness control.
import { readFileSync } from 'node:fs'
import path from 'node:path'
import { fileURLToPath } from 'node:url'
import { defineConfig } from 'vitest/config'

const here = path.dirname(fileURLToPath(import.meta.url))
const repo = path.resolve(here, '../../../..')
const pkg = path.join(repo, 'ref/tailwindcss/packages/tailwindcss')
const src = path.join(pkg, 'src')
const impl = process.env.RUSTIFY_UPSTREAM_IMPL ?? 'rust'
if (!['rust', 'official'].includes(impl)) throw new Error(`unknown RUSTIFY_UPSTREAM_IMPL ${impl}`)

// The test harness's own dependencies are pinned in this package, never
// resolved from the repository root or the reference tree.
const bare = new Set(['lightningcss', 'magic-string', '@jridgewell/remapping', 'dedent', 'source-map-js'])
const entry = path.join(src, 'index.ts')
const shim = path.join(here, 'rust-index.mjs')
// Upstream internal modules a test file imports directly, backed by Rust.
const units = new Map(
  [
    'css-parser',
    'value-parser',
    'utils/segment',
    'utils/decode-arbitrary-value',
    'utils/brace-expansion',
    'utils/compare',
    'utils/escape',
    'utils/replace-shadow-colors',
    'source-maps/line-table',
    'utils/to-key-path',
    'utilities',
    'ast',
    'source-maps/source-map',
    'variants',
    'theme',
    'design-system',
  ].map(name => [path.join(src, `${name}.ts`), path.join(here, 'units', `${path.basename(name)}.mjs`)]),
)
const shims = new Set([shim, ...units.values()])

// Any other upstream module a test file imports runs as TypeScript. Its
// exports are proxied so a call marks the running test as mixed evidence.
// Upstream's harness only post-processes results and is exempt: `test-utils`
// optimizes printed CSS, and `DefaultMap` groups source-map annotations.
const harness = [path.join(src, 'test-utils') + path.sep, path.join(src, 'utils/default-map.ts')]
const FOREIGN = '\0rustify-foreign:'
const facadeModule = path.join(here, 'facade.mjs')
function exportNames(source) {
  let names = new Set()
  for (let match of source.matchAll(/^export\s+(?:declare\s+)?(?:async\s+)?(?:function\*?|const\s+enum|enum|const|let|var|class)\s+([A-Za-z_$][\w$]*)/gm)) names.add(match[1])
  for (let match of source.matchAll(/^export\s*\{([^}]*)\}/gm)) {
    for (let part of match[1].split(',')) {
      let name = part.trim().replace(/^type\s+/, '').split(/\s+as\s+/).pop()?.trim()
      if (name && !/^type\b/.test(part.trim())) names.add(name)
    }
  }
  if (/^export\s+default\b/m.test(source)) names.add('default')
  return [...names]
}

export default defineConfig({
  root: pkg,
  cacheDir: path.join(here, 'node_modules/.vite'),
  plugins: [
    {
      name: 'rustify-upstream-exec',
      enforce: 'pre',
      async resolveId(id, importer, options) {
        if (bare.has(id)) return this.resolve(id, path.join(here, 'package.json'), { ...options, skipSelf: true })
        // A unit shim re-exports the rest of its upstream module as foreign.
        if (id.startsWith('rustify-foreign:')) return FOREIGN + path.resolve(path.dirname(importer), id.slice('rustify-foreign:'.length))
        if (impl !== 'rust' || !importer || shims.has(importer)) return null
        if (!/^\.\.?(\/|$)/.test(id)) return null
        let resolved = path.resolve(path.dirname(importer), id)
        if (resolved === src || resolved === path.join(src, 'index') || resolved === entry) return shim
        // Compat suites exercise JavaScript plugin/config helpers; only the
        // package entry is redirected for them.
        if (!importer.endsWith('.test.ts') || importer.startsWith(path.join(src, 'compat') + path.sep)) return null
        let file = resolved.endsWith('.ts') ? resolved : `${resolved}.ts`
        if (units.has(file)) return units.get(file)
        if (!file.startsWith(src + path.sep) || harness.some(prefix => file.startsWith(prefix))) return null
        return FOREIGN + file
      },
      load(id) {
        if (!id.startsWith(FOREIGN)) return null
        let file = id.slice(FOREIGN.length)
        let names = exportNames(readFileSync(file, 'utf8'))
        return [
          `import * as upstream from ${JSON.stringify(file)}`,
          `import { foreign } from ${JSON.stringify(facadeModule)}`,
          `const wrap = value => typeof value !== 'function' ? value : new Proxy(value, {`,
          `  apply(target, self, args) { foreign(); return Reflect.apply(target, self, args) },`,
          `  construct(target, args, next) { foreign(); return Reflect.construct(target, args, next) },`,
          `})`,
          `const missing = Object.keys(upstream).filter(name => !${JSON.stringify(names)}.includes(name))`,
          `if (missing.length) throw new Error('unproxied upstream exports in ${path.relative(src, file)}: ' + missing.join(', '))`,
          ...names.map(name => (name === 'default' ? `export default wrap(upstream.default)` : `export const ${name} = wrap(upstream.${name})`)),
        ].join('\n')
      },
    },
  ],
  test: {
    root: pkg,
    snapshotSerializers: [path.join(src, 'test-utils/custom-serializer.ts')],
    include: (process.env.RUSTIFY_UPSTREAM_FILES ?? 'src/**/*.test.ts').split(','),
    exclude: ['**/*.spec.?(c|m)[jt]s?(x)', 'integrations/**/*', '**/node_modules/**'],
    setupFiles: [path.join(here, 'record.mjs')],
    includeTaskLocation: true,
    update: false,
    pool: 'forks',
    testTimeout: 60_000,
  },
})
