// Checks upstream tests that build a design system through the internal
// `buildDesignSystem(theme)`, which skips `applyCompatibilityHooks`. Rust
// registers the legacy utilities in every public entry, so its class list
// must equal upstream's frozen snapshot plus exactly those utilities, in order.
import assert from 'node:assert/strict'
import { readFileSync } from 'node:fs'
import path from 'node:path'
import { fileURLToPath } from 'node:url'
import { configuration, facade } from './facade.mjs'

const here = path.dirname(fileURLToPath(import.meta.url))
const snapshots = path.join(here, '../../../../ref/tailwindcss/packages/tailwindcss/src/__snapshots__/intellisense.test.ts.snap')
const block = readFileSync(snapshots, 'utf8').split('exports[`getClassList 1`] = `')[1].split('`;')[0]
const expected = [...block.matchAll(/^  "(.*)",$/gm)].map(match => match[1].replace(/\\\\/g, '\\'))

// The theme `loadDesignSystem()` in intellisense.test.ts adds.
const theme = [
  ['--spacing', '0.25rem'], ['--colors-red-500', 'red'], ['--colors-blue-500', 'blue'],
  ['--breakpoint-sm', '640px'], ['--aspect-video', '16 / 9'], ['--font-sans', 'sans-serif'],
  ['--font-weight-superbold', '900'], ['--text-xs', '0.75rem'], ['--text-xs--line-height', '1rem'],
  ['--perspective-dramatic', '100px'], ['--perspective-normal', '500px'], ['--opacity-background', '0.3'],
  ['--drop-shadow-sm', '0 1px 1px rgb(0 0 0 / 0.05)'], ['--inset-shadow-sm', 'inset 0 1px 1px rgb(0 0 0 / 0.05)'],
  ['--font-weight-bold', '700'], ['--container-md', '768px'], ['--container-lg', '1024px'],
]
// `registerLegacyUtilities` and the legacy static roots it adds.
const legacy = [
  'bg-gradient-to-b', 'bg-gradient-to-bl', 'bg-gradient-to-br', 'bg-gradient-to-l', 'bg-gradient-to-r',
  'bg-gradient-to-t', 'bg-gradient-to-tl', 'bg-gradient-to-tr', 'bg-left-bottom', 'bg-left-top',
  'bg-right-bottom', 'bg-right-top', 'decoration-clone', 'decoration-slice', 'object-left-bottom',
  'object-left-top', 'object-right-bottom', 'object-right-top', 'order-none', 'overflow-ellipsis',
]

const system = new facade.DesignSystem(theme.map(([key, value]) => `@theme { ${key}: ${value}; }`).join('\n'), configuration())
const actual = system.getClassList().flatMap(([name, meta]) => [name, ...meta.modifiers.map(modifier => `${name}/${modifier}`)])
assert.deepEqual(actual.filter(name => !legacy.includes(name)), expected, 'class list beyond the legacy utilities')
assert.deepEqual(actual.filter(name => legacy.includes(name)).sort(), [...legacy].sort(), 'legacy utilities')
console.log(`getClassList: ${expected.length} upstream classes in order, plus ${legacy.length} legacy utilities`)
