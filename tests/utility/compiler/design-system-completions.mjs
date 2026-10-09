// Capture fixed upstream suggestion definitions, before theme namespace expansion.
// Production class completions resolve these definitions in Rust; this is build-time tooling.
import assert from 'node:assert/strict'
import { createHash } from 'node:crypto'
import { createRequire } from 'node:module'
import { cp, mkdtemp, readFile, rm, writeFile } from 'node:fs/promises'
import { tmpdir } from 'node:os'
import { dirname, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'

const directory = dirname(fileURLToPath(import.meta.url))
const root = resolve(directory, '../../..')
const source = resolve(root, 'ref/tailwindcss/packages/tailwindcss/src')
const destination = resolve(root, 'crates/rustify-style/src/compiler/public_api/design_system/completions.json')
const require = createRequire(resolve(directory, 'oracle/package.json'))
const { createJiti } = require('jiti')
const temporary = await mkdtemp(resolve(tmpdir(), 'rustify-design-completions-'))
try {
  await cp(source, resolve(temporary, 'src'), { recursive: true })
  const packageBytes = await readFile(resolve(source, '../package.json'), 'utf8')
  assert.equal(JSON.parse(packageBytes).version, '4.1.13')
  await writeFile(resolve(temporary, 'package.json'), packageBytes)
  const original = await readFile(resolve(source, 'utilities.ts'), 'utf8')
  let instrumented = original.replace('let utilities = new Utilities()', 'let utilities = new Utilities(); let captured = new Map()')
  instrumented = instrumented.replace('function suggest(classRoot: string, defns: () => SuggestionDefinition[]) {', 'function suggest(classRoot: string, defns: () => SuggestionDefinition[]) { captured.set(classRoot, defns)')
  instrumented = instrumented.replace('return utilities\n}', 'utilities.__definitions = () => Array.from(captured, ([name, fn]) => [name, fn()]); return utilities\n}')
  assert.notEqual(instrumented, original)
  await writeFile(resolve(temporary, 'src/utilities.ts'), instrumented)
  const jiti = createJiti(import.meta.url, { moduleCache: false, fsCache: false })
  const { createUtilities } = await jiti.import(resolve(temporary, 'src/utilities.ts'))
  const { Theme } = await jiti.import(resolve(temporary, 'src/theme.ts'))
  const theme = new Theme()
  const utilities = createUtilities(theme)
  const definitions = utilities.__definitions()
  const flags = ['--spacing', '--shadow', '--inset-shadow', '--text-shadow']
  for (const flag of flags) {
    theme.add(flag, '1px')
    const probe = utilities.__definitions()
    theme.add(flag, 'initial')
    for (let i = 0; i < definitions.length; i++) {
      assert.equal(definitions[i][0], probe[i][0])
      for (let j = 0; j < definitions[i][1].length; j++) {
        const base = definitions[i][1][j]
        const changed = probe[i][1][j]
        if (JSON.stringify(base.values) !== JSON.stringify(changed.values)) {
          assert.deepEqual(base.values, [])
          base.valuesWhen = { key: flag, values: changed.values }
        }
        if (base.hasDefaultValue !== changed.hasDefaultValue) {
          assert.equal(base.hasDefaultValue, false)
          assert.equal(changed.hasDefaultValue, true)
          base.defaultWhen = flag
        }
      }
    }
  }
  const output = {
    version: '4.1.13', source: 'packages/tailwindcss/src/utilities.ts',
    sourceSha256: createHash('sha256').update(original).digest('hex'),
    definitions,
  }
  const bytes = JSON.stringify(output, null, 2) + '\n'
  if (process.argv.includes('--write')) await writeFile(destination, bytes)
  else assert.equal(await readFile(destination, 'utf8'), bytes)
  console.log(JSON.stringify({ definitions: definitions.length, sha256: createHash('sha256').update(bytes).digest('hex'), sourceSha256: output.sourceSha256 }))
} finally {
  await rm(temporary, { recursive: true, force: true })
}
