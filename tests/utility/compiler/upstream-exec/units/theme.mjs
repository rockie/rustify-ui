// `Theme` records its edits as an `@theme` stylesheet for the Rust design system.
export * from 'rustify-foreign:../../../../../ref/tailwindcss/packages/tailwindcss/src/theme.ts'

const flags = [
  [1, 'inline'],
  [2, 'reference'],
  [4, 'default'],
  [8, 'static'],
]
const block = (options, body) =>
  `@theme ${flags.filter(([flag]) => options & flag).map(([, name]) => name).join(' ')} { ${body} }`

export class Theme {
  prefix = null
  edits = []
  add(key, value, options = 0) {
    this.edits.push(block(options, `${key}: ${value};`))
  }
  clearNamespace(namespace, options = 0) {
    this.edits.push(block(options, `${namespace}-*: initial;`))
  }
  get css() {
    return this.edits.join('\n')
  }
}
