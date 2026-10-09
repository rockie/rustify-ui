import assert from 'node:assert/strict'
import { createHash } from 'node:crypto'
import { readdir, readFile, writeFile } from 'node:fs/promises'
import { createRequire } from 'node:module'
import { dirname, relative, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'

const here = dirname(fileURLToPath(import.meta.url))
const root = resolve(here, '../../..')
const oracleRequire = createRequire(resolve(here, 'oracle/package.json'))
const ts = oracleRequire('typescript')
const { createJiti } = oracleRequire('jiti')
const published = oracleRequire('tailwindcss')
const commit = '1334c99db8fd26c8ea065375dd9259800863f072'
const sha256 = value => createHash('sha256').update(value).digest('hex')

async function filesUnder(path) {
  const result = []
  for (const entry of await readdir(path, { withFileTypes: true })) {
    const item = resolve(path, entry.name)
    if (entry.isDirectory()) result.push(...await filesUnder(item))
    else result.push(item)
  }
  return result.sort()
}

function registry(design) {
  return {
    utilities: ['static', 'functional'].flatMap(kind => design.utilities.keys(kind).map(name => ({ name, kind, handlers: design.utilities.get(name).filter(item => item.kind === kind).length }))).sort((a, b) => a.name.localeCompare(b.name, 'en') || a.kind.localeCompare(b.kind, 'en')),
    variants: Array.from(design.variants.entries(), ([name, item]) => ({ name, kind: item.kind, order: item.order, compounds: item.compounds, compoundsWith: item.compoundsWith })).sort((a, b) => a.order - b.order || a.name.localeCompare(b.name, 'en')),
  }
}

export async function generateInventory(sourceRoot) {
  const packagePath = resolve(sourceRoot, 'packages/tailwindcss')
  const packageFile = JSON.parse(await readFile(resolve(packagePath, 'package.json'), 'utf8'))
  assert.equal(packageFile.version, '4.1.13')
  const css = await readFile(resolve(packagePath, 'theme.css'), 'utf8')
  const jiti = createJiti(import.meta.url, { moduleCache: false, fsCache: false })
  const sourceCompiler = await jiti.import(resolve(packagePath, 'src/index.ts'))
  const sourceRegistry = registry(await sourceCompiler.__unstable__loadDesignSystem(css))
  assert.deepEqual(sourceRegistry, registry(await published.__unstable__loadDesignSystem(css)), 'local source registry differs from the pinned published oracle')
  const classify = source => /(?:^|\/)plugin\.[ct]s$|\/compat\/(?:config(?:\/|\.)|plugin-|apply-config|apply-keyframes|container-config|screens-config|dark-mode|container\.)/.test(source) ? 'js_plugin_and_config_execution_out_of_scope' : source.includes('/compat/apply-compat-hooks.') ? 'mixed_css_and_excluded_js' : 'in_scope'
  const sources = []
  const tests = []
  const exports = []
  const registrationSites = []
  const directives = new Map()
  for (const path of await filesUnder(resolve(packagePath, 'src'))) {
    const content = await readFile(path, 'utf8')
    const source = relative(sourceRoot, path)
    sources.push({ path: source, sha256: sha256(content), bytes: Buffer.byteLength(content), kind: /\.test\./.test(path) ? 'test' : /\.bench\./.test(path) ? 'benchmark' : path.endsWith('.snap') ? 'snapshot' : 'module', status: 'pending' })
    if (!/\.[cm]?tsx?$/.test(path)) continue
    const ast = ts.createSourceFile(path, content, ts.ScriptTarget.Latest, true)
    const locator = node => ({ path: source, line: ast.getLineAndCharacterOfPosition(node.getStart(ast)).line + 1, sha256: sha256(node.getText(ast)) })
    function visit(node) {
      if (node.modifiers?.some(modifier => modifier.kind === ts.SyntaxKind.ExportKeyword)) {
        exports.push({ ...locator(node), declaration: node.name?.getText(ast) ?? node.getText(ast).split('\n')[0], scope: node.name?.getText(ast) === 'Config' && source.endsWith('/src/index.ts') ? 'js_config_type_out_of_scope' : classify(source), status: 'pending' })
      } else if (ts.isExportDeclaration(node)) {
        exports.push({ ...locator(node), declaration: node.getText(ast), scope: classify(source), status: 'pending' })
      }
      if (ts.isCallExpression(node)) {
        let expression = node.expression
        while (ts.isCallExpression(expression) || ts.isPropertyAccessExpression(expression)) expression = expression.expression
        const base = expression.getText(ast)
        if (/\.test\./.test(path) && ['test', 'it', 'describe'].includes(base)) {
          const argument = node.arguments[0]
          const name = argument && (ts.isStringLiteralLike(argument) ? argument.text : argument.getText(ast))
          tests.push({ ...locator(node), id: `${source}:${ast.getLineAndCharacterOfPosition(node.getStart(ast)).line + 1}:${node.getStart(ast)}-${node.end}`, role: base === 'describe' ? 'suite' : ts.isPropertyAccessExpression(node.expression) && ['each', 'for'].includes(node.expression.name.text) ? 'parameterization' : 'test', name: name ?? null, call: node.expression.getText(ast), containsJsPluginOrConfig: /@(?:plugin|config)|loadModule/.test(node.getText(ast)), scope: /@(?:plugin|config)|loadModule/.test(node.getText(ast)) && classify(source) === 'in_scope' ? 'mixed_css_and_excluded_js' : classify(source), status: 'pending', rustFixture: null })
        }
        const call = node.expression.getText(ast)
        if (/^(?:staticUtility|functionalUtility|spacingUtility|colorUtility|staticVariant)$/.test(call) || /(?:utilities|variants)\.(?:static|functional|compound)$/.test(call)) registrationSites.push({ ...locator(node), call, keyExpression: node.arguments[0]?.getText(ast) ?? null, status: 'pending' })
      }
      if (ts.isStringLiteralLike(node) && /^@(?:tailwind|utility|source|variant|custom-variant|theme|import|reference|apply|plugin|config)$/.test(node.text) && !/\.(?:test|bench)\./.test(path)) {
        if (!directives.has(node.text)) directives.set(node.text, [])
        directives.get(node.text).push(locator(node))
      }
      ts.forEachChild(node, visit)
    }
    visit(ast)
  }
  const locate = name => registrationSites.filter(site => site.keyExpression === JSON.stringify(name) || site.keyExpression === `'${name}'`)
  const data = {
    schemaVersion: 1,
    baseline: { project: 'tailwindcss', version: '4.1.13', commit, license: 'MIT', sourcePathAtCapture: 'ref/tailwindcss', sourceDigest: sha256(sources.map(item => `${item.path}\0${item.sha256}\n`).join('')), registryInput: 'packages/tailwindcss/theme.css', registryInputSha256: sha256(css), registryValidation: 'executed source registry equals tailwindcss@4.1.13 published registry' },
    coverage: { meaning: 'Complete enumerated source files, exports, syntactic test calls, registration sites and executed built-in registry; no Rust implementation or conformance pass is implied. Parameterized/dynamic tests require execution and fixture conversion.', sourceFiles: sources.length, testCalls: tests.filter(item => item.role === 'test').length, staticUtilityRoots: sourceRegistry.utilities.filter(item => item.kind === 'static').length, functionalUtilityRoots: sourceRegistry.utilities.filter(item => item.kind === 'functional').length, variantRoots: sourceRegistry.variants.length, implementationStatus: 'pending', conformanceStatus: 'pending' },
    excluded: [ { feature: 'execute JavaScript plugins', directives: ['@plugin'], status: 'out_of_scope', requiredDiagnostic: 'UnsupportedFeature' }, { feature: 'execute JavaScript configuration', directives: ['@config'], status: 'out_of_scope', requiredDiagnostic: 'UnsupportedFeature' }, { feature: 'Tailwind v3', status: 'out_of_scope' } ],
    publicPackageExports: Object.entries(packageFile.exports).map(([name, target]) => ({ name, target, scope: name === './plugin' || name === './plugin.js' ? 'js_plugin_execution_out_of_scope' : 'in_scope', status: 'pending' })),
    publicDataHelperSemantics: [
      { name: 'colors', scope: 'in_scope', behavior: 'fixed color token object; Rust data equivalent without JavaScript execution', source: 'packages/tailwindcss/src/compat/colors.ts', status: 'pending' },
      { name: 'defaultTheme', scope: 'in_scope', behavior: 'fixed default values plus built-in theme/number/fraction resolution functions; equivalent Rust data and resolvers, no user JavaScript configuration execution', source: 'packages/tailwindcss/src/compat/default-theme.ts', status: 'pending' },
      { name: 'flattenColorPalette', scope: 'in_scope', behavior: 'recursive nested color flattening, DEFAULT key elision, __CSS_VALUES__ precedence respecting ThemeOptions.DEFAULT', source: 'packages/tailwindcss/src/compat/flatten-color-palette.ts', status: 'pending' },
    ],
    compileOptions: ['base', 'from', 'polyfills', 'loadStylesheet', 'loadModule'].map(name => ({ name, locator: { path: 'packages/tailwindcss/src/index.ts', symbol: 'CompileOptions' }, scope: name === 'loadModule' ? 'js_plugin_and_config_execution_out_of_scope' : 'in_scope', status: 'pending' })),
    compilerMechanisms: ['CSS parser and value parser', 'AST walk, serialization and optimization', 'candidate parse and print', 'arbitrary properties and arbitrary selectors', 'negative, fractions, alpha and important', 'prefix handling', 'theme inline/reference/default/static and namespace reset', 'theme token dependency pruning and keyframes', 'property registration and polyfill controls', 'virtual stylesheet imports, reference imports, paths, cycles and errors', 'CSS utility static and functional value resolution', 'custom variants, slots and nested variant application', 'apply dependency topological order and circular errors', 'source selection, inline and not-inline brace expansion, source(none)', 'CSS functions --alpha, --spacing, --theme and theme', 'variant rank, property order, declaration count and class name sorting', 'invalid candidate rejection and exception behavior', 'additive build cache and used-variable candidates', 'source map and source location options'].map(name => ({ name, scope: 'in_scope', status: 'pending', rustFixture: null })),
    directives: [...directives].sort(([a], [b]) => a.localeCompare(b, 'en')).map(([name, locators]) => ({ name, locators, scope: ['@plugin', '@config'].includes(name) ? 'out_of_scope' : 'in_scope', status: ['@plugin', '@config'].includes(name) ? 'requires_unsupported_diagnostic' : 'pending', rustFixture: null })),
    utilities: sourceRegistry.utilities.map(item => ({ ...item, locators: locate(item.name).length ? locate(item.name) : [{ path: 'packages/tailwindcss/src/utilities.ts', symbol: 'createUtilities', detail: 'computed registration; resolve against registrationSites during fixture conversion' }], scope: 'in_scope', status: 'pending', rustFixture: null })),
    variants: sourceRegistry.variants.map(item => ({ ...item, locators: locate(item.name).length ? locate(item.name) : [{ path: 'packages/tailwindcss/src/variants.ts', symbol: 'createVariants', detail: 'computed registration; includes theme-derived breakpoint roots' }], scope: 'in_scope', status: 'pending', rustFixture: null })),
    assets: await Promise.all(['package.json', 'index.css', 'preflight.css', 'theme.css', 'utilities.css'].map(async name => { const bytes = await readFile(resolve(packagePath, name)); return { path: `packages/tailwindcss/${name}`, sha256: sha256(bytes), bytes: bytes.length, status: 'pending' } })),
    registrationSites, exports, sources, tests,
  }
  return data
}

export async function checkCompilerInventory() {
  const inventory = JSON.parse(await readFile(resolve(here, 'inventory.json'), 'utf8'))
  const css = await readFile(oracleRequire.resolve('tailwindcss/theme.css'), 'utf8')
  assert.equal(sha256(css), inventory.baseline.registryInputSha256)
  const actual = registry(await published.__unstable__loadDesignSystem(css))
  assert.deepEqual(actual.utilities, inventory.utilities.map(({ name, kind, handlers }) => ({ name, kind, handlers })))
  assert.deepEqual(actual.variants, inventory.variants.map(({ name, kind, order, compounds, compoundsWith }) => ({ name, kind, order, compounds, compoundsWith })))
  assert.equal(new Set(inventory.tests.map(item => item.id)).size, inventory.tests.length, 'test IDs must be unique')
  assert.equal(sha256(inventory.sources.map(item => `${item.path}\0${item.sha256}\n`).join('')), inventory.baseline.sourceDigest)
  assert.equal(inventory.coverage.testCalls, inventory.tests.filter(item => item.role === 'test').length)
  assert.equal(inventory.coverage.sourceFiles, inventory.sources.length)
  return { utilities: actual.utilities.length, variants: actual.variants.length, tests: inventory.coverage.testCalls, conformanceStatus: inventory.coverage.conformanceStatus }
}

if (process.argv[1] && resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  if (process.argv.includes('--check-registry')) {
    console.log(JSON.stringify(await checkCompilerInventory()))
    process.exit(0)
  }
  const arg = process.argv.indexOf('--source')
  const data = await generateInventory(arg >= 0 ? resolve(process.argv[arg + 1]) : resolve(root, 'ref/tailwindcss'))
  if (process.argv.includes('--check')) assert.deepEqual(data, JSON.parse(await readFile(resolve(here, 'inventory.json'), 'utf8')))
  else await writeFile(resolve(here, 'inventory.json'), `${JSON.stringify(data, null, 2)}\n`)
  console.log(JSON.stringify(data.coverage))
}
